use crate::error::Error;
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
            webdriver.goto(&url).await?;

            //
            println!("sleep 5s");
            const JS: &'static str = r#"
            // const [date, callback] = arguments;
            alert("hi");
            "#;
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
                    alert("jquery is already in use!");
               }else{

                   loadScript('https://ajax.googleapis.com/ajax/libs/jquery/1.7.1/jquery.js');
                //    loadScript('https://ajax.googleapis.com/ajax/libs/jqueryui/1.8.16/jquery-ui.js');
                alert(" loaded jquery!");
               }
            "#;
            // .Replace("\r\n", ""); // 看stackoverflow上面的评论 说是有的驱动只能执行行代码 多行有问题？🤨
            let r = webdriver.execute(injectjQuery2, vec![]).await?;

            sleep(Duration::from_millis(15000)).await; // time to load jQuery library
            webdriver.source().await?
        };


        let mut index = 0;
        let document = Document::from(html.as_str());
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
            // println!("title:{} ", node.text());
            // *[@id="form"]//*[@type="text"]
            // let selector = format!("//ul[@class=\"painting-list\"]/li[{}]/a", index);
            // let selector = format!("//ul[@id=\"list1\" and @class='shmu-tuwen-list']//li[{}]/a", index);
            let selector = format!(
                "//ul[@id=\"list1\" and @class='shmu-tuwen-list']/li[{}]/div[@class='show']//a",
                index
            );

            let _ = {
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
            };
           
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
        Ok((items, Vec::new()))
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

   async fn _tmp(&self){
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
