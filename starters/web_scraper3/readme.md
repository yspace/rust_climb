[作者们的讨论！](https://bytemeta.vip/repo/jonhoo/fantoccini/issues/145?page=1)
[更早的：](https://www.reddit.com/r/rust/comments/evlc49/thirtyfour_a_new_selenium_library_for_rust_for/) 

## 先决步骤

安装 mozilla出品的[geckodriver](https://github.com/mozilla/geckodriver)
惊喜的是可以用cargo安装 之后直接运行
> cargo install geckodriver && geckodriver

进程都可以指定端口 不指定就用默认的了


也可以安装 WebDriver compatible 就是兼容WebDriver的进程 Selenium

chrome 的好像有更多扩展插件

`测试 用chromedriver速度更快` 很奇怪的现象 而且手动关闭后不需要重启driver 二用geckodriver 就需要手动重启 不然session建立就会有问题 就是重用性不如chromedriver好
> chromedriver --port=4444


更多依赖看examplers：
https://github.com/jonhoo/fantoccini/tree/main/examples

## dom 操作
网页抓取后 可以把文本的html页面变为dom结构 这样可以做jquery类似的操作 可选库也挺多
有国人开发的visdom 挺好用的jquery风格！[visdom](https://github.com/fefit/visdom)
此库adobe好像用了！


## 存db 

爬网页 总归是要落地的 需要的信息可以存储在文件或者db中 方便后续查看 提取有用的东西

这里使用最简单的sqlite 不过想方便的使用它可不那么简单 因为需要考虑异步运行时 sqlx稍显笨重
这里使用了一个最近才开始的库 发现挺轻便好用：[tokio-rusqlite](https://github.com/programatik29/tokio-rusqlite/tree/master/examples) 配合 rusqlite 库一起使用 如果还想支持更人性化的查询构造 还可以考虑sea-query

### 锁db
~~~shell
mv my—data.db temp.db
cp temp.db my-data.db
~~~

## 下载文件
https://stackoverflow.com/questions/46937319/how-to-use-chrome-webdriver-in-selenium-to-download-files-in-python

~~~python

options = webdriver.ChromeOptions()
options.add_experimental_option("prefs", {
  "download.default_directory": r"C:\Users\xxx\downloads\Test",
  "download.prompt_for_download": False,
  "download.directory_upgrade": True,
  "safebrowsing.enabled": True
})
driver = webdriver.Chrome(chrome_options=options)
~~~

~~~java

public class DownloadChromeFile {
       public static void main(String[] args) {
       System.setProperty("webdriver.chrome.driver","./chromedriver.exe");
       String downloadFilepath = "c:\\download";

       HashMap<String, Object> chromePrefs = new HashMap<String, Object>();
       chromePrefs.put("profile.default_content_settings.popups", 0);
       chromePrefs.put("download.default_directory", downloadFilepath);
       ChromeOptions options = new ChromeOptions();
       HashMap<String, Object> chromeOptionsMap = new HashMap<String, Object>();
       options.setExperimentalOption("prefs", chromePrefs);
       options.addArguments("--test-type");
       options.addArguments("--disable-extensions"); //to disable browser extension popup

       DesiredCapabilities cap = DesiredCapabilities.chrome();
       cap.setCapability(ChromeOptions.CAPABILITY, chromeOptionsMap);
       cap.setCapability(CapabilityType.ACCEPT_SSL_CERTS, true); // Bydefault it will accepts all popups.
       cap.setCapability(ChromeOptions.CAPABILITY, options);
       driver = new ChromeDriver(cap);  
                driver.get("Your Application Url");
                driver.findElement(By.xpath("Export Button xpath")).click();
        }
} 
~~~

## 运行
~~~shell

cargo run -p web_scraper3 --bin download_files

~~~

## 后记：

- 加快下载速度
  网络上满足条件后 就是分任务 任务表可以切分下
  [Split a very large SQL table to multiple smaller tables](https://dba.stackexchange.com/questions/66552/split-a-very-large-sql-table-to-multiple-smaller-tables)

~~~sql
-- If you already have a column by which you can "partition" the table, then you can do something like this:

SELECT * 
 INTO dbo.newtable_0001_to_2000 
 FROM dbo.existingtable 
 WHERE column >= 1 AND column <= 2000;

SELECT * 
 INTO dbo.newtable_2001_to_4000 
 FROM dbo.existingtable 
 WHERE column >= 2001 AND column <= 4000;

...

-- Just note that this will give you a predictable number of rows, but won't control the size of the rows, so depending on the data in each "partition" these tables will not likely be of equal sizes.
~~~

临时表？
~~~sql
CREATE TEMPORARY TABLE some_backup(field1 TEXT, field2 REAL)
INSERT INTO some_backup SELECT field1, field2 FROM orig_table
~~~

sqlite 不支持 select into语法 可以这样搞

~~~sql
create table project2 as select * from projects where id>999 and id < 2000
~~~

这种任务分割后 查询和更新需要针对不同的表进行 比较麻烦

还有一种 就是还是针对同一张表
一个任务取奇数 另一个取偶数   这种思想推广开也可以是取模那种 每个任务只取自己相同间隔的数 1+n*3 （比如首任务编号1 第二个就是 1+1*3 = 4 第三次取就是 1+2*3=7 ）等差数列形式进行
这样可以开3 个任务进程 首任务号 分别是1 2 3     然后通用的下次任务id就是 n*3+ first_number

对于表id非连续情形   或许这样有点不均匀 不过也可以搞没有自己的号就跳过本次跳到下个号去直到找到就行 任务分配可能不均匀 有的多点有的少点

也可以两头干 你增序取 我倒序取 回合一处就完了 修桥那样 你那头我这头汇合一处整个桥就成了