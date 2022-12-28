[作者们的讨论！](https://bytemeta.vip/repo/jonhoo/fantoccini/issues/145?page=1)
[更早的：](https://www.reddit.com/r/rust/comments/evlc49/thirtyfour_a_new_selenium_library_for_rust_for/) 

## 先决步骤

安装 mozilla出品的[geckodriver](https://github.com/mozilla/geckodriver)
惊喜的是可以用cargo安装 之后直接运行
> cargo install geckodriver && geckodriver

进程都可以指定端口 不指定就用默认的了


也可以安装 WebDriver compatible 就是兼容WebDriver的进程 Selenium

chrome 的好像有更多扩展插件

chromdriver 版本的选择需要跟本机所装的chrome浏览器版本对应 先查看本机浏览器版本再选择对应的driver下载 运行

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

### rust下载
参考
https://patshaughnessy.net/2020/1/20/downloading-100000-files-using-async-rust
https://crates.io/crates/downloader
https://github.com/hunger/downloader

https://github.com/mattgathu/duma
https://github.com/smoqadam/rust-youtube-downloader
https://github.com/lecepin/douyin-downloader
https://github.com/lecepin/douyin-downloader/blob/master/src-tauri/src/command.rs
https://github.com/lzdyes/douyin-downloader/blob/main/src-tauri/src/command.rs


### webdriver 下载
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
[How to handle custom download location in selenium — Chrome](https://medium.com/geekculture/how-to-handle-custom-download-location-in-selenium-chrome-78daaacd79ab)

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


## xpath

xpath 感觉其定位查找功能比jquery 应该更丰富 

### 通过父子兄弟 爷爷关系来查找 


//span[@class="bg"]/../div

    先通过/..找到 span 的父节点，再通过父节点找到 div。


### 使用谓语定位

谓语是 Xpath 中用于描述元素位置。主要有数字下标、最后一个子元素last()、元素下标函数position()。

~~~xpath
# 查找最后一个子元素，选取 form 下的最后一个 span

//form[@id="form"]/span[last()] 

//form[@id="form"]/span[last()-1]  倒数第一个

//form[@id="form"]/span[position()=2] 第二个span

//form[@id="form"]/span[position()>2] 下标大于 2 的 span

~~~
### 使用逻辑运算符

~~~xpath
//*[@name='wd' and @class='s_ipt'] 查找 name 属性为 wd 并且 class 属性为 s_ipt 的任意元素

//*[@name='wd' or @class='s_ipt'] 

# 使用|，同时查找多个路径，取或
//form[@id="form"]//span | //form[@id="form"]//input 
~~~

### 使用文本定位

text()   string()

text()：当前元素节点包含的文本内容；

表达式//div[text()="文本"]，能找到：

<div>文本</div> 不能找到：

<div><span>文本</span></div>

表达式//div[string()="文本"]上述两种情况都能找到。

### 使用部分匹配函数
1. contains
2. starts-with
3. ends-with
  //div[ends-with(@id, 'require')] 选取 id 属性以 require 结尾的 div 元素
//div[ends-with(string(), '支付')] 选取内部文本以“支付”结尾的 div 元素

作者：猫与测试
链接：https://www.jianshu.com/p/6a0dbb4e246a
来源：简书
著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

### 节点轴选择

轴可定义相对于当前节点的节点集。
语法：轴名称::节点[谓语]

常用的轴：

    ancestor：选取当前节点的所有先辈节点（父、祖父等）。
    ancestor-or-self：选取当前节点的所有先辈节点（父、祖父等）以及当前节点本身。
    attribute：选取当前节点的所有属性。
    self：选取当前节点。
    child：选取当前节点的所有子节点。
    parent：选取当前节点的父节点。
    descendant：选取当前节点的所有后代节点（子、孙等）。
    descendant-or-self：选取当前节点的所有后代节点（子、孙等）以及当前节点本身。

示例：

    //li[@data="one"]/ancestor::div：选取属性data="one"的li节点的所有div祖先节点。
    //li[@data="one"]/ancestor::*：选取属性data="one"的li标签的所有祖先节点。
    //div[@id="testid"]/attribute::*：选取id="testid"的div节点的所有属性值。
    //div[@id]/self::div[@data-h]/attribute::*：选取含id属性和data-h属性的div标签的所有属性值
    //div[@id="testid"]/child::*：选取id="testid"的div节点的所有子节点。
    //li[@data="one"]/parent::ol/li[last()]/text()：选取属性data="one"的li节点的父节点ol，其最后一个li子节点的文本值。
    注意：由于每个元素节点只有唯一的一个父节点，所以“parent::父节点”等价于“parent::*” 。


~~~python
from lxml import etree

text = '''
<div>
    <ul>
         <li class="item-0"><a href="link1.html"><span>first item</span></a></li>
         <li class="item-1"><a href="link2.html">second item</a></li>
         <li class="item-inactive"><a href="link3.html">third item</a></li>
         <li class="item-1"><a href="link4.html">fourth item</a></li>
         <li class="item-0"><a href="link5.html">fifth item</a>
     </ul>
 </div>
'''
html = etree.HTML(text)
result = html.xpath('//li[1]/ancestor::*')
print(result)
result = html.xpath('//li[1]/ancestor::div')
print(result)
result = html.xpath('//li[1]/attribute::*')
print(result)
result = html.xpath('//li[1]/child::a[@href="link1.html"]')
print(result)
result = html.xpath('//li[1]/descendant::span')
print(result)
result = html.xpath('//li[1]/following::*[2]')
print(result)
result = html.xpath('//li[1]/following-sibling::*')
print(result)

~~~
第一次选择我们调用了 ancestor 轴，可以获取所有祖先节点，其后需要跟两个冒号，然后是节点的选择器，这里我们直接使用了 *，表示匹配所有节点，因此返回结果是第一个 li 节点的所有祖先节点，包括 html，body，div，ul。

第二次选择我们又加了限定条件，这次在冒号后面加了 div，这样得到的结果就只有 div 这个祖先节点了。

第三次选择我们调用了 attribute 轴，可以获取所有属性值，其后跟的选择器还是 *，这代表获取节点的所有属性，返回值就是 li 节点的所有属性值。

第四次选择我们调用了 child 轴，可以获取所有直接子节点，在这里我们又加了限定条件选取 href 属性为 link1.html 的 a 节点。

第五次选择我们调用了 descendant 轴，可以获取所有子孙节点，这里我们又加了限定条件获取 span 节点，所以返回的就是只包含 span 节点而没有 a 节点。

第六次选择我们调用了 following 轴，可以获取当前节点之后的所有节点，这里我们虽然使用的是 * 匹配，但又加了索引选择，所以只获取了第二个后续节点。

第七次选择我们调用了 following-sibling 轴，可以获取当前节点之后的所有同级节点，这里我们使用的是 * 匹配，所以获取了所有后续同级节点

作者：IT派森
链接：https://www.jianshu.com/p/15f39d8b395a
来源：简书
著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。


## 杂
- [infer](https://github.com/bojand/infer)
此库可以快速检测出文件类型 通过魔术字节 头部的一些特征编码 判断文件类型 

- [Xpath 详解](https://www.jianshu.com/p/6a0dbb4e246a)
- [xpath详解](https://blog.csdn.net/baidu_32542573/article/details/79675420)
有以上两篇 基本可以覆盖xpath常用场景了！

- [xpath_syntax](http://www.w3school.com.cn/xpath/xpath_syntax.asp)
https://www.w3school.com.cn/xpath/xpath_functions.asp

- [Python爬虫教程（从入门到精通）](http://c.biancheng.net/python_spider/)
- [【Python】Python3网络爬虫实战-28、解析库的使用：XPath](https://www.jianshu.com/p/15f39d8b395a)


## 程序结构
app
config
如刘备打江山  必须先有山头地盘 后有配置粮草（config ） 然后还需要有大将 光一个main 有点太宽泛 光杆司令的感觉

args 程序命令行解析逻辑 对命令行的映射解析
common｜utils ｜helpers 此是边角料 通用方法 或者结构 或是对其他库的扩展功能

error[s] 错误处理 

