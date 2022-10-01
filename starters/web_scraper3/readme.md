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


## 下载文件

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