## 参考
https://github.com/gregstoll/rust-scraping

https://www.gkbrk.com/wiki/rust_web_scraping/


- [ crawl and scrape web pages in rust ](https://github.com/mattsse/voyager)
这是个全站爬虫！


[Minimal HTTP download manager ](https://github.com/agourlay/dlm)
这个下载器写的不错 精简小巧 可以看看


[Web Scraping with Rust](https://www.scrapingbee.com/blog/web-scraping-rust/c)

[Building a crawler in Rust: Scraping and Parsing HTML](https://kerkour.com/rust-crawler-scraping-and-parsing-html)
Black Hat Rust! 
源码样本在这里 [black-hat-crawler](https://github.com/skerkour/black-hat-rust/tree/main/ch_05/crawler)

[一次简单的 rust 爬虫开发技术调研](https://zhuanlan.zhihu.com/p/516033159)

### 依赖
- 下载库 reqwest
   技巧：

~~~rust
const ORIGIN: &str = "https://movie.douban.com";
const REFERER: &str = "https://movie.douban.com/";
const UA: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.131 Safari/537.36";
// const DEFAULT_LIMIT: usize = 3;
// const CACHE_SIZE: usize = 100;

 let mut headers = HeaderMap::new();
        headers.insert("Origin", HeaderValue::from_static(ORIGIN));
        headers.insert("Referer", HeaderValue::from_static(REFERER));

        let client = reqwest::Client::builder()
            .user_agent(UA)
            .default_headers(headers)
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();
        
~~~

相似库：同步库 https://github.com/algesten/ureq 这个支持代理 支持socks cookie 连接池

- html文档解析库
  可以把下载下来的html文档变为  HTML DOM 对象 方便提取操作。
  当网页下载下来后就需要解析出想要的内容 html文档是非良构的 可以用字符串查找｜正则查找得到想要的内容 但最好用的如jquery 或者xml风格的api那样操作 
  visdom 这个库看起来不错 国人搞的 adobe用来解析xml文件了！可以类似jquery那样修改文档对象 移除添加对象！（go中 有个对等的goquery）

  有个doubanapi： https://github.com/cxfksword/douban-api-rs/blob/master/src/bookapi.rs 好像用来扒特定信息  

  库scraper  
  可以使用css 选择器来提取需要的内容
  ~~~rust
   let document = scraper::Html::parse_document(&response);
   let title_selector = scraper::Selector::parse("h3.lister-item-header>a").unwrap();

       let titles = document.select(&title_selector).map(|x| x.inner_html());
    titles
        .zip(1..101)
        .for_each(|(item, number)| println!("{}. {}", number, item));

  ~~~

- 库 crabquery

JQuery like HTML query library 

-  库： https://github.com/utkarshkukreti/select.rs 
没试过 感觉有jquery风格 

其他库：
https://github.com/y21/tl


### json 解析

针对api请求一般都返回json数据 同样也需要提取特定部分感兴趣的信息

json  转 map后使用 jsonpath 筛选

## 更高级的数据抓取 

主要针对反扒网站 需要集成webdriver 例子看 web_scraper2 和 web_scraper3

可以使用thirtyfour 和 fantoccini 这俩都基于tokio

对于后续信息提取 可以使用驱动的选择器提取 也可以转换为html文本 发给rust端再提取

~~~rust
.inner_html().await?.as_str());
~~~