
高级爬虫 主要是用
https://github.com/stevepryde/thirtyfour

配合chromedriver

先运行driver：
> ./bins/chromedriver

注意此执行文件 对应不同版本的chrome 浏览器版本
https://chromedriver.chromium.org/downloads 
下载对应的版本（打开自己的chorme浏览器 菜单的关于部分查看版本号 menu > Help > About Google Chrome） 
对应的平台（win linux mac mac_m1 ...）


### 参考
- [Using Selenium with Rust](https://dev.to/stevepryde/using-selenium-with-rust-aca)
- [一次简单的 rust 爬虫开发技术调研](https://zhuanlan.zhihu.com/p/516033159)
- [Building a crawler in Rust: Scraping Javascript Single Page Applications (SPA) with a headless browser](https://kerkour.com/rust-crawler-javascript-single-page-application-headless-browser)