use crate::{error::Error, utils};
use async_trait::async_trait;
use fantoccini::{Client, ClientBuilder, Locator};
use select::{
    document::Document,
    predicate::{Class, Name, Predicate},
};

use visdom::types::BoxDynError;
use visdom::Vis;

use serde_json::json;
use tokio::{
    sync::Mutex,
    time::{sleep, Duration},
};

use chrono::prelude::*;

const LOAD_MORE: &str = "__LOAD_MORE__";

pub struct ShanghaiMusSpider {
    webdriver_client: Mutex<Client>,
}

impl ShanghaiMusSpider {
    pub async fn new() -> Result<Self, Error> {
        println!("[ShanghaiMusSpider::new]");
        let mut caps = serde_json::map::Map::new();
        // let chrome_opts = serde_json::json!({ "args": ["--headless", "--disable-gpu"] });
        // caps.insert("goog:chromeOptions".to_string(), chrome_opts);
        let webdriver_client = ClientBuilder::rustls()
            // .capabilities(caps)
            .connect("http://localhost:4444")
            .await?;
        println!("[ShanghaiMusSpider::new ok!]");
        Ok(Self {
            webdriver_client: Mutex::new(webdriver_client),
        })
    }
}

#[derive(Debug, Clone)]
pub struct QuotesItem {
    quote: String,
    author: String,
}

const MORE_ITEMS: &str = "more_items";

#[async_trait]
impl super::Spider for ShanghaiMusSpider {
    type Item = QuotesItem;

    fn name(&self) -> String {
        String::from("quotes")
    }

    fn start_urls(&self) -> Vec<String> {
        vec!["https://www.shanghaimuseum.net/mu/frontend/pg/collection/antique?antiqueType1Code=CP_HIGH_CLASS_TYPE_3".to_string()]
    }

    async fn scrape(&self, url: String) -> Result<(Vec<Self::Item>, Vec<String>), Error> {
        println!("[begin scrape:] {url:?}");

        let mut items = Vec::new();

        let html = {
            let webdriver = self.webdriver_client.lock().await;

            // if(url == LOAD_MORE.to_string()){
            if (url.starts_with(LOAD_MORE)) {
                println!("load more  !");

                const JS: &'static str = r#"
                // const [date, callback] = arguments;
                // consle.log("clear the li list!");
                 var $li_list = $('ul#list1 li');
                 console.log($li_list.size());
                 $li_list.empty().remove();

                console.log('load more!!');
                var $loadMoreLink = $('ul#list1 .layui-flow-more a');
                console.log('[load more]',$loadMoreLink.size());
                if($loadMoreLink.size()>0){
                    $loadMoreLink.click();
                }

                "#;

                let r = webdriver.execute(JS, vec![]).await?;
                println!("sleep");
                sleep(Duration::from_millis(15000)).await;
            } else {
                webdriver.goto(&url).await?;
            }

            //
            println!("sleep 5s");
            const JS: &'static str = r#"
            // const [date, callback] = arguments;
            alert("hi");
            "#;
            // TODO: 可以找个好的jquery的cdn
            // let r =  webdriver.execute(JS, vec![]).await?;
            // webdriver.execute_async(JS, vec![serde_json::to_value(elem)?]);
            // let r = webdriver.execute_async(JS, vec![json!("from rust")]).await?;
            // @see https://sqa.stackexchange.com/questions/2921/webdriver-can-i-inject-a-jquery-script-for-a-page-that-isnt-using-jquery
            const injectjQuery2: &'static str = r#"
               function loadScript(scriptUrl) 
               {
                   var head =  document.getElementsByTagName('head')[0];
                   var script = document.createElement('script');
                   script.type = 'text/javascript';
                   script.src = scriptUrl;
                   head.appendChild(script);
               }
               if (typeof jQuery != 'undefined') {
               //    if($){
                    console.log("jquery is already in use!");
               }else{

                   loadScript('https://ajax.googleapis.com/ajax/libs/jquery/1.7.1/jquery.js');
                //    loadScript('https://ajax.googleapis.com/ajax/libs/jqueryui/1.8.16/jquery-ui.js');
                console.log(" loaded jquery!");
               }
            "#;
            // .Replace("\r\n", ""); // 看stackoverflow上面的评论 说是有的驱动只能执行行代码 多行有问题？🤨
            let r = webdriver.execute(injectjQuery2, vec![]).await?;

            sleep(Duration::from_millis(15000)).await; // time to load jQuery library
            webdriver.source().await?
        };

        let mut load_more_suffix = String::new();

        // TODO  针对翻页情形， 翻页之后再次计算下内容区域是否跟上次相同
        let count = {
            let mut index = 0;
            let document = Document::from(html.as_str());

            let mut target_content = document.find(
                Class("shmu-tuwen-list")
            );
            let target_content = target_content.next().unwrap();
            let target_content = target_content.html();
            // println!("[content:] {}", utils::md5(target_content));
            load_more_suffix = utils::md5(target_content);

            if url.ends_with(&load_more_suffix) {
                return Ok((items, vec![]));
            }


            for node in document.find(
                Class("shmu-tuwen-list").descendant(
                    Name("li")
                        .descendant(Class("show"))
                        .descendant(Class("shmu-box"))
                        .descendant(Name("a")),
                ),
            ) {
                index += 1;

                // for node in document.find(Class("shmu-tuwen-list").descendant(Name("a"))) {
                // println!("{} ({:?})", node.text(), node.attr("href").unwrap());
                println!("link ({:?})", node.attr("href").unwrap());
                println!("--");
            }
            index
        };

        // let count = 1; // FIXME:  手动更改测试用
        for index in 1..count {
            // *[@id="form"]//*[@type="text"]
            // let selector = format!("//ul[@class=\"painting-list\"]/li[{}]/a", index);
            // let selector = format!("//ul[@id=\"list1\" and @class='shmu-tuwen-list']//li[{}]/a", index);
            let selector = format!(
                "//ul[@id=\"list1\" and @class='shmu-tuwen-list']/li[{}]/div[@class='show']//a",
                index
            );
            // NOTE：⚠️ 注意await 前面出现的变量可能变为async块的依赖 所以要求可 `Send`
            let client = self.webdriver_client.lock().await;
            // Click the img.
            let el_link = client
                .wait()
                //r#"a[href="/learn/get-started"]"#,
                .for_element(Locator::XPath(selector.as_str()))
                .await?;
            el_link.click().await?;
            // 等下 不然一会就退回了

            client.wait().for_element(Locator::Css("body")).await?;

            // TODO 睡随机数
            sleep(Duration::from_millis(5000)).await; //
            // TODO： 处理fancybox

            let sub_items = {
                // let webdriver = self.webdriver_client.lock().await;
                const JS: &'static str = r#"
            // const [date, callback] = arguments;
           // var callback = arguments[arguments.length - 1];
           var callback = arguments[0];
           // ----------------------------------------------------------


           // ----------------------------------------------------------
           setTimeout(function(){
                callback("test_async_exec");
}          , 2000);
            "#;
                // https://api.flutter.dev/flutter/package-webdriver_async_io/WebDriver/executeAsync.html

                let js_result = client
                    .execute_async(
                        JS,
                        vec![],
                    )
                    .await?;
                //     .as
                //     .expect("should be integer variant");
                //
                // assert_eq!(2, count);
                println!("js callback result is : {}",js_result);

            };

            let mut windows = client.windows().await?;
            let new_window = windows.remove(windows.len() - 1);
            client.switch_to_window(new_window).await;

            client.close_window().await;
            // 看看 跳没
            client.switch_to_window(windows.remove(windows.len() - 1)).await;

            // client.back().await?;
        }

        // let quotes = document.find(Class("quote"));
        // for quote in quotes {
        //     let mut spans = quote.find(Name("span"));
        //     let quote_span = spans.next().unwrap();
        //     let quote_str = quote_span.text().trim().to_string();

        //     let author = spans
        //         .next()
        //         .unwrap()
        //         .find(Class("author"))
        //         .next()
        //         .unwrap()
        //         .text()
        //         .trim()
        //         .to_string();

        //     items.push(QuotesItem {
        //         quote: quote_str,
        //         author,
        //     });
        // }

        // let next_pages_link = document
        //     .find(
        //         Class("pager")
        //             .descendant(Class("next"))
        //             .descendant(Name("a")),
        //     )
        //     .filter_map(|n| n.attr("href"))
        //     .map(|url| self.normalize_url(url))
        //     .collect::<Vec<String>>();

        //     println!("[items:] {items:?}");
        // Ok((items, Vec::new()))
        println!("[end scrap current page]");

        // TODO： 如何决定是否是最后一页？ 可以用上一页翻页表格区的内容做特征对比 把ts时间戳部分替换为内容区域文本hash-code即可

        let now = Utc::now();
        // let ts: i64 = now.timestamp();
        // let load_more_ = format!("{}_{}",LOAD_MORE,ts);
        let load_more_ = format!("{}_{}", LOAD_MORE, load_more_suffix);

        Ok((items, vec![load_more_]))
        // Ok((items, vec![LOAD_MORE.to_string()]))
    }

    async fn process(&self, item: Self::Item) -> Result<(), Error> {
        println!("{}", item.quote);
        println!("by {}\n", item.author);
        Ok(())
    }
}

impl ShanghaiMusSpider {
    fn normalize_url(&self, url: &str) -> String {
        let url = url.trim();

        if url.starts_with("/") {
            return format!("https://quotes.toscrape.com{}", url);
        }

        return url.to_string();
    }

    async fn _tmp(&self) {
        let client = self.webdriver_client.lock().await;
        // Click the img.
        // let el_link = client
        //     .wait()
        //     //r#"a[href="/learn/get-started"]"#,
        //     .for_element(Locator::XPath(selector.as_str()))
        //     .await?;
        // el_link.click().await?;
        // // 等下 不然一会就退回了

        // client.wait().for_element(Locator::Css("body")).await?;

        // sleep(Duration::from_millis(15000)).await; // time to load jQuery library
        //    // 看看 跳没
        // client.back().await?;
    }
}

pub fn run() {}
