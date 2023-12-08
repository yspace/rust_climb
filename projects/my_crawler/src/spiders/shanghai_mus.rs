use crate::{error::Error, utils};
use async_trait::async_trait;
use fantoccini::{Client, ClientBuilder, Locator};
use fantoccini::actions::{
    Actions, InputSource, KeyAction, KeyActions,  NullActions,

};
use fantoccini::actions::{MOUSE_BUTTON_LEFT, MouseActions, PointerAction};

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


            let mut windows = client.windows().await?;
            let new_window = windows.remove(windows.len() - 1);
            // 切换到新tab窗口去
            client.switch_to_window(new_window).await;

            // TODO： 处理fancybox
            let sub_items = {
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

                // loadScript('https://ajax.googleapis.com/ajax/libs/jquery/1.7.1/jquery.js');
                //    loadScript('https://ajax.googleapis.com/ajax/libs/jqueryui/1.8.16/jquery-ui.js');
                   loadScript('https://cdn.bootcss.com/jquery/2.1.1/jquery.min.js');
                console.log(" loaded jquery!");
               }
            "#;
                // .Replace("\r\n", ""); // 看stackoverflow上面的评论 说是有的驱动只能执行行代码 多行有问题？🤨
                let r = client.execute(injectjQuery2, vec![]).await?;

                // let webdriver = self.webdriver_client.lock().await;
                const JS: &'static str = r#"


            // const [date, callback] = arguments;
           // var callback = arguments[arguments.length - 1];
           var callback = arguments[0];
           // ----------------------------------------------------------

           function waitForElement(selector, callback) {
              var intervalId = setInterval(function () {
                if ($(selector).length > 0) {
                  clearInterval(intervalId);
                  callback();
                }
              }, 100);
            }

            function waitForCondition(fn, callback) {
              var intervalId = setInterval(function () {
                if (fn() == true) {
                  clearInterval(intervalId);
                  callback();
                }
              }, 3000);
            }


           var $sliderItems =   $(".slick-list .shmu-slider-item","\#slider2");
            var result = [];
            $("body").append("<div id='rust_result'>");
            var $resultHelper = $("\#rust_result");
            $resultHelper.data("state", { total: $sliderItems.length, current: 0 });

            // FIXME: 这里直接返回小列表的数目
             callback($sliderItems.length);


            // waitForCondition(function(){
            //
            // var $sliderItemLinks =   $(".slick-list a.shmu-thumbnail");
            //
            // $sliderItemLinks.each(function(index, $item){
            //         if(index>0) return ;
            //
            //         $item.click();
            //         $slideCurrentActive =  $(".slick-current" ,"\#slider2");
            //
            //         var $slickCurrentLarger =  $(".slick-current" ,"\#slider1");
            //
            //         $slickCurrentLarger.click(function(){
            //
            //              setTimeout(function(){
            //              // 会由多个实例 如果不点击关闭按钮！
            //                 var $fancyBoxViewPort = $(".fancybox__container:last");
            //                 var $largeImage = $fancyBoxViewPort.find("img");
            //                 result.push($largeImage.attr("src"));
            //                 // result.push($largeImage.length);
            //                 // 更新当前进度
            //                 var state = $resultHelper.data("state");
            //                 state.current = state.current + 1;
            //                  $resultHelper.data("state", state);
            //
            //             }          , 2000);
            //
            //         });
            //         $slickCurrentLarger.click();
            //
            //          setTimeout(function(){
            //                 $item.remove();
            //             }          , 4000);
            //
            //  });
            //
            //  // 查看当前进度
            //     var state = $resultHelper.data("state");
            //
            //     console.log(state.total, '|' , state.current);
            //
            //     // return false ;
            //     return state.current >= state.total ;
            // },function(){
            //   callback(result);
            // });


           // ----------------------------------------------------------
//            setTimeout(function(){
//                 // callback($sliderItems.length);
//                 callback(result);
// }          , 12000);
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
                let slider_items_count = js_result.as_u64().unwrap();
                println!("js callback result is : {}", slider_items_count);

                for idx in 0..slider_items_count{

                    // By.xpath("img[title='到百度首页']:nth-child(1)")
                    // let selector = format!(r#"//div[@id="slider2"]//div[@class="slick-track"]/div[{}]"# ,idx);
                    let selector = format!(r#"//div[@id="slider2"]//div[@class="slick-track"]/div[position()={}]"# ,idx+1);
                    println!("xpath: {selector}");
                    let elem = client.wait().for_element(Locator::XPath(selector.as_str())).await?;

                    // client.execute(
                    //     "arguments[0].scrollIntoView(true);",
                    //     vec![serde_json::to_value(elem).unwrap()],
                    // )
                    //     .await?;
                    const JS: &'static str = r#"
           var callback = arguments[1];
           // ----------------------------------------------------------
           var result = 'any-thing';
            var $item = $(arguments[0]);
            $item.click();

           // ----------------------------------------------------------
           setTimeout(function(){
                // callback($sliderItems.length);
                callback(result);
}          , 2000);
            "#;
                    let js_result = client
                        .execute_async(
                            JS,
                            vec![serde_json::to_value(elem).unwrap()],
                        )
                        .await?;
                    println!("js callback result is : {}", js_result);

                    const JS2: &'static str = r#"
          var callback = arguments[arguments.length - 1];
           // ----------------------------------------------------------
           var result = 'step2';

           result = $(".slick-current" ,"\#slider1").length;

           // $(".slick-active a","\#slider1").click();
          $(".slick-current" ,"\#slider1").find("a").click();

           // ----------------------------------------------------------
           setTimeout(function(){
                callback(result);
}          , 2000);
            "#;
                    let js_result = client
                        .execute_async(
                            JS2,
                            vec![ ],
                        )
                        .await?;
                    println!("js callback result is : {}", js_result);

                    let elem = client.find(Locator::Id("slider1")).await?;
                    let rect = elem.rectangle().await?;
                    println!("{:?}", rect);
                    let elem_center_x = rect.0 + (rect.2 / 2.0);
                    let elem_center_y = rect.1 + (rect.3 / 2.0);

                    // Test mouse MoveBy.
                    let mouse_actions = MouseActions::new("mouse".to_string())
                        // Move to a position at a known offset from the button.
                        .then(PointerAction::MoveTo {
                            duration: None,
                            // x: 0,
                            x: elem_center_x as i64,
                            y: elem_center_y as i64 - 100,
                        })
                        // Now move by relative offset so that the cursor is now over the button.
                        // .then(PointerAction::MoveBy {
                        //     duration: None,
                        //     x: elem_center_x as i64,
                        //     y: 100,
                        // })
                        // Press left mouse button down.
                        .then(PointerAction::Down {
                            button: MOUSE_BUTTON_LEFT,
                        })
                        // Release left mouse button.
                        .then(PointerAction::Up {
                            button: MOUSE_BUTTON_LEFT,
                        });
                    let actions = Actions::from(mouse_actions);
                    client.perform_actions(actions).await?;
                    sleep(Duration::from_millis(5000)).await;

                    const JS3: &'static str = r#"
          var callback = arguments[arguments.length - 1];
           // ----------------------------------------------------------
           var result = 'step3';

           var $fancyBox = $(".fancybox__container:last");

            $fancyBox.find("button.fancybox__button--close").click();

           // ----------------------------------------------------------
           setTimeout(function(){
                callback(result);
}          , 2000);
            "#;
                    let js_result = client
                        .execute_async(
                            JS3,
                            vec![ ],
                        )
                        .await?;


                    // Click the "Get Started" button.
                    // let element = client
                    //     .wait()
                    //     .for_element(Locator::Css(
                    //         r#"\#slider1"#,
                    //     ))
                    //     .await?;
                    // element.click().await?;


                    // Test mouse down/up.
                    // let mouse_actions = MouseActions::new("mouse".to_string())
                    //     .then(PointerAction::MoveToElement {
                    //         element: elem,
                    //         duration: None,
                    //         x: 0,
                    //         y: 0,
                    //     })
                    //     .then(PointerAction::Down {
                    //         button: MOUSE_BUTTON_LEFT,
                    //     })
                    //     .then(PointerAction::Up {
                    //         button: MOUSE_BUTTON_LEFT,
                    //     });
                    //
                    // client.perform_actions(mouse_actions).await?;

                    // elem.click().await?;

                    // sleep(Duration::from_millis(5000)).await;
                    println!("index: {idx}");
                    sleep(Duration::from_millis(6000)).await;
                }


            };

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
    fn normalize_image_url(&self, url: &str) -> String {
        let url = url.trim();


        return format!("https://www.shanghaimuseum.net/mu/{}", url);
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
