use std::{path::PathBuf, borrow::Borrow};
use std::ops::Deref;
use fantoccini::{ClientBuilder, Locator};
use fantoccini::actions::{Actions, InputSource, KeyAction, KeyActions, MouseActions, NullActions, PointerAction, MOUSE_BUTTON_LEFT, MOUSE_BUTTON_RIGHT};
use fantoccini::key::Key;

use url::{Host, ParseError, Url};

use lazy_static::lazy_static;
use regex::{Regex, Captures};

use web_scraper3::*;

use std::time::Duration;
use reqwest::Client;
use visdom::types::BoxDynError;
use visdom::Vis;

use sanitize_filename;
use tokio::time::sleep;
use webdriver::capabilities::Capabilities;

// use std::collections::HashSet;
use std::collections::BTreeSet;

use sled::{Db,Config};

lazy_static! {
    // url中 当前页码 正则匹配
    static ref PAGE_REGEX: Regex = Regex::new(r"page=(?P<n>\d+)").unwrap();
}


const DL_DIR: &'static str = "__downloads";
pub(crate) const PROGRAM_NAME: &str = "xlysauc";
pub(crate) const PROGRAM_PREFIX: &str = "program_xlysauc_";
pub(crate) const PROGRAM_DB: &str = "__data/xlysauc";

lazy_static! {
    pub static ref ROOT_DB: sled::Db = Config::default()
        .path(PROGRAM_DB)
        .open()
        .expect("Unable to open root database");
}

// ⚠️ 这个网站用了 shadowDom ：[Web Components详解-Shadow DOM基础](https://developer.aliyun.com/article/1557247)

// let's set up the sequence of steps we want the browser to take
#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {

    let settings = config::Conf::new();

    // Print out our settings
    println!("{settings:?}");

    println!("😄：just for learning ；scrap images from 西冷印社");

    // println!("{:#}",RO);
    // let tree = ROOT_DB.open_tree("my_tree");

    // 无头配置
    let cap: Capabilities = serde_json::from_str(
        r#"{"browserName":"chrome","goog:chromeOptions":{"args":["--headless"]}}"#,
     )
        .unwrap();

    let c = ClientBuilder::native()
        // .capabilities(cap) // ⚠️ 无头配置如果看不到情况 可以注释掉此行变为有头 看效果
        .connect("http://localhost:4444")
        .await
        .expect("failed to connect to WebDriver");




    let search_user = "溥儒";
    // let search_user = "溥心畬";
    let search_user = "黄君璧";
    // let search_user = "陈少梅";
    // let search_user = "赵国经";
    // // let search_user = "王美芳";
    // // let search_user = "齐白石";
    // let search_user = "丰子恺";
    // let search_user = "徐燕孙";
    // let search_user = "黄均";
    // let search_user = "王时敏";
    // let search_user = "陆俨少"; // 没有
    // let search_user = "陈云彰"; // 陈少梅
    // let search_user = "沈周";
    // let search_user = "八大山人";
    // let search_user = "林风眠";
    // build|make target_url
    // let page = 1;
    let target_url = format!("http://www.xlysauc.com/index/search/types/lots/keywords/{}.html", search_user) ;
    println!("👨👩: search items for user: {}",search_user);
    // 陈少梅



    c.goto(&target_url).await?;


    loop {
        println!("enter_loop");

        let url = c.current_url().await?;
        let page_num = get_page_num(url.to_string().as_str()) ;
        // println!("{}", url);
        println!("page: {}", page_num);
        // let tmp_path = "/somePath/hi";
        // println!("{}", url.join(tmp_path)?);

        #[allow(deprecated)]
        c.wait_for(move |_| {
            std::thread::sleep(Duration::from_secs(7));
            async move { Ok(true) }
        })
            .await?;


        // -----------------------------执行异步js--------------------------------------------------

        const JS_PAGINATION: &'static str = r#"
            // 😄回调函数就叫 callback 可以给你用来返回任何东东哦！

            // const [date, callback] = arguments;
            const [ callback] = arguments;

            console.log("😄 开始执行js啦！");

            function addSuffix4Duplicates(arr) {
                let newArr = [];
                let countMap = {}; // 用于记录每个元素出现的次数

                for (let i = 0; i < arr.length; i++) {
                    let element = arr[i];
                    let suffix = '';
                    if (countMap[element]) {
                        suffix = `_${countMap[element]}`; // 添加后缀
                        countMap[element]++; // 更新计数
                    } else {
                        countMap[element] = 1; // 初始化计数
                    }
                    newArr.push(element + suffix); // 将带有后缀的元素添加到新数组中
                }

                return newArr;
            }

            const zip = (arr1, arr2) => arr1.map((item1, index) => {
               return  item1+'|'+  arr2[index] ; //[num, arr2[index]]
             }
            );


            // 😄TODO：这里因页面不同逻辑不通
            // var my_f = function(){}
            function getCurrentPageItems()
            {

                // ⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️ 这里是获取当前页面的分页链接的css选择逻辑 为主要逻辑
                var itemCssSelector = "ul.search-auction>li>a" ;
                var elements = document.querySelectorAll( itemCssSelector );


                var js_ruslts = [] ;
                for (let element of elements) {
                  // console.log(element);

                  let s_item_href = element.href;

                  js_ruslts.push(s_item_href);
                  // 🤔 其实可以在这里一并处理 获取相关的信息
                }

                // 🐯这里获取链接的名字 可以作为文件存储的名称 但也可以在单个页面中获取 哪里信息更全面
                 var ele_infos = document.querySelectorAll(itemCssSelector+' div.title');;
                var js_ruslts2 = [] ;
                for (let element of ele_infos) {
                  // console.log(element);
                  //
                  var title = element.textContent ; // element.innerText ;
                  console.log('目标item名称--- >>>'+title);
                  js_ruslts2.push(title);
                }

                // js_ruslts2 =  addSuffix4Duplicates(js_ruslts2);
                // js_ruslts = zip(js_ruslts,js_ruslts2);


                return js_ruslts;
            }

             // 😄TODO：这里因页面不同逻辑不通
            function getNextPageLink(){
                result = null ;

                var elements = document.querySelectorAll("div.page a.next");
                var text = "下一页";
                var text2 = "下一個";
                var found = false;

                elements.forEach(function(element) {
                    if (element.textContent.includes(text) || element.textContent.includes(text2)) { // 或者使用 element.innerText.includes(text)
                        found = true;

                        console.log("找到了元素", element);
                         result = element

                        return; // 如果只需要找到第一个匹配的元素，可以使用 return 终止循环
                    }
                });

                return result ;
            };

            function getIsNeedPage(){

                var nextPageLink = getNextPageLink();
                if(nextPageLink == null){
                     return false ;
                }else{
                 return true;
                }
            }

            // 滚动到底部
            window.scrollTo(0, document.body.scrollHeight);

            console.log("is_need_page:"+ getIsNeedPage());

            // ⚠️ 异步执行
            //  setTimeout(function(){
             setTimeout(()=>{
                    console.log("is_need_page2:"+ getNextPageLink());
                    console.log("is_need_page3:"+ getIsNeedPage());
                     console.log("---->>>----");
                     console.log("---->>>----");

                     var isNeedPage = getIsNeedPage();

                     var js_ruslts =
                     {
                       items: getCurrentPageItems(),
                       need_page: isNeedPage,
                       load_more: isNeedPage
                     };

                     // 分页逻辑
                     console.log('分页逻辑开始....');

                      if(isNeedPage) {

                        // 🐱可选逻辑 获取当前页面的页码 也可以通过分析location 浏览器url获取
                        var currentPageLink = document.querySelector("div.page a.num.on") ;
                       // var pageNumber = currentPageLink.querySelector('span').innerText;
                        var pageNumber = currentPageLink.innerText;
                        js_ruslts['page_number'] = pageNumber ;

                        const nextPage =getNextPageLink();
                        console.log(nextPage);

                        // 检查是否存在下一个兄弟元素节点
                        if (nextPage !== null) {
                             console.log('模拟下点击分页');

                             // 点击分页
                             nextPage.click();

                        }
                     }

                      // callback( "hi heree" )
                      callback( js_ruslts );

             },1000) ;
        "#;
        //
        // let mut js_result = c.execute_async(JS, vec![]).await?;
        // println!("js result here :{:?}", js_result);
        // // -------------------------------------------------------------------------------
        // 检查是否为数组
        // https://docs.rs/serde_json/latest/serde_json/enum.Value.html
        use serde_json::Value;

        // let mut need_page = true;
        // let mut js_result_items = js_result["items"].as_array().unwrap() ;
        //
        // println!("js result length :{:?}", js_result_items.len());
        // for item in js_result_items {
        //     println!("{}", item.as_str().unwrap());
        // }


        //
        const JS_SCROLL_FOR_LOAD_MORE: &'static str = r#"
         // 😄回调函数就叫 callback 可以给你用来返回任何东东哦！

            // const [date, callback] = arguments;
            const [ callback] = arguments;

            console.log("😄 开始执行js啦！");

            function getResultsTotal(){
                // 获取所有符合xpath表达式的元素
                var result = document.evaluate('//p[@data-testid="results-total"]', document, null,XPathResult.ANY_TYPE, null);
                // 获取第一个元素
                var node = result.iterateNext();
                // console.log(node.innerText);
                var resultText = node.innerText;
                return extractAndConvertToNumber(resultText);

            }

            // 😄TODO：这里因页面不同逻辑不通
            // var my_f = function(){}
            function getCurrentPageItems()
            {

               // 获取所有符合xpath表达式的元素
               //     var result = document.evaluate('//p[@data-testid="results-search-item"]', document, null,XPathResult.ANY_TYPE, null);
               // 获取所有div元素
                var xpathResult = document.evaluate('//div[starts-with(@class, "SearchResultsGrid_lotCardWrapper__")]', document, null, XPathResult.ORDERED_NODE_SNAPSHOT_TYPE, null);

                var allElements = [];
                for (var i = 0, length = xpathResult.snapshotLength; i < length; ++i) {
                    allElements.push(xpathResult.snapshotItem(i));
                }
                //console.log(allElements);
                return allElements;
                // 🤔其实可以返回所有待处理项的url的


                // 🐶👇是大致逻辑
                // var elements = document.querySelectorAll("ul.artworks>li>a");
                // var js_ruslts = [] ;
                // for (let element of elements) {
                //   // console.log(element);
                //
                //   let s_item_href = element.href;
                //
                //   js_ruslts.push(s_item_href);
                //   // 🤔 其实可以在这里一并处理 获取相关的信息
                // }
                //
                // return js_ruslts;
            }

            // 将字符串转换为数字
            function extractAndConvertToNumber(str) {
                // 使用正则表达式匹配字符串中的所有数字
                const numbers = str.match(/\d+/g);
                // 如果匹配到数字，则将它们连接起来并转换为数字
                if (numbers) {
                    return parseInt(numbers.join(''), 10);
                }
                // 如果没有匹配到数字，返回0或其他你希望返回的值
                return 0;
            }

            // 滚动到底部
            function smoothScroll(targetY) {
                  const startY = window.scrollY;
                  const distance = targetY - startY;
                  const duration = 1000; // 动画持续时间(ms)
                  let startTime = null;

                  function scrollStep(timestamp) {
                    if (!startTime) startTime = timestamp;
                    const elapsed = timestamp - startTime;
                    const progress = Math.min(elapsed / duration, 1);
                    const easeInOut = progress < 0.5 ?
                      2 * progress * progress :
                      1 - Math.pow(-2 * progress + 2, 2) / 2;

                    window.scrollTo(0, startY + distance * easeInOut);

                    if (progress < 1) {
                      window.requestAnimationFrame(scrollStep);
                    }
                  }
                  window.requestAnimationFrame(scrollStep);
            }
            function getElementTop(element) {
                const rect = element.getBoundingClientRect();
                return rect.top + window.scrollY;
            }

            function scrollToBottom(){
                // 示例用法
                const element = document.getElementById('BrightSpotFooter');
                const topPosition = getElementTop(element);
                console.log(topPosition);
                // 使用示例：滚动到页面底部
                smoothScroll(topPosition - 150);

            }

            console.log("共有："+getResultsTotal());

            function recursiveTimeout(count) {

                console.log(count);
                console.log(count < getResultsTotal());
                if (count < getResultsTotal()) {
                    setTimeout(function() {

                        scrollToBottom();
                        document.documentElement.scrollIntoView({ behavior: 'smooth' });

                        recursiveTimeout(0); // 😺这里条件不满足会一直滚动的
                    }, 10000); // 延迟10000毫秒后再次调用
                }
            }

            // recursiveTimeout(0);

            // 滚动到底部去
            scrollToBottom();
            // ⚠️ 异步执行
             setTimeout(function(){


                     console.log("---->>>----");
                     console.log("---->>>----");

                    var items = getCurrentPageItems();

                     var js_ruslts =
                     {
                       items: []  , // getCurrentPageItems(),
                       need_page: false ,
                       // load_more:  items.length < getResultsTotal(),
                       load_more:  document.querySelector('#loadMore') != null ,
                       loaded_progress:""+items.length+ "/" +getResultsTotal()
                     };

                     // callback( "hi heree" )
                     callback( js_ruslts );

             },7000) ;

        "#;
        // let mut items_from_js: Vec<&str> = Vec::new();
        let mut items_from_js  = Vec::new();
        loop {
            // 循环体代码
            // let mut js_result = c.execute_async(JS_SCROLL_FOR_LOAD_MORE, vec![]).await?;
            let mut js_result = c.execute_async(JS_PAGINATION, vec![]).await?;
            println!(" 🈲️[debug:] js result here :{:?}", js_result);

            // TODO 在这里收集当前页面的链接
            let mut js_result_items = js_result["items"].as_array().unwrap() ;

            println!("js result length :{:?}", js_result_items.len());
            // for item in js_result_items {
            //     println!("{}", item.as_str().unwrap());
            // }

            let my_items: Vec<_> = js_result_items.into_iter().map(|item| item.as_str().unwrap().to_string()).collect();
            items_from_js.extend(my_items);

            let mut js_result_load_more = js_result["load_more"].as_bool().unwrap() ;

            if js_result_load_more == false {
                break;
            }

            // sleep_random(&c).await ;
            // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            tokio::time::sleep(Duration::from_secs(10)).await; // 模拟耗时任务
        }
        if items_from_js.len() > 0 {
            // println!("🈲️[debug:] all items from js here :{:?}", items_from_js);
        }
        // ---------------------------------👆是js获取当前页的待处理元素的逻辑 跟👇下面逻辑类似 可以任选一个修改逻辑         -------------------------------------------

        let mut handeled_items = BTreeSet::new();

        if items_from_js.len() > 0 {
            // ⚠️注意into_iter 跟 iter 是不同的 🤔
            handeled_items = items_from_js.into_iter().collect();
        }else{
            // 🐯用rust来收集当前页面元素
            // 页码
            let mut page_number =0;
            // # 使用XPath表达式选择具有特定类的元素
            // elements = root.xpath('//*[@class="my-class"]')
            // 获取本页需要处理的图片
            // let selector = r#"//div[@class="swiper-slide"]/a"#;
            let selector = r#"//div[starts-with(@class, "SearchResultsGrid_lotCardWrapper__")]//a"#;
            let eles = c.find_all(Locator::XPath(selector)).await?;
            // ⚠️ 注意这里面有重复的a链接 想办法去重
            println!("🤔==== 当前页面有元素 > {:?} ", eles.len());
            // let mut handeled_items = HashSet::new();
            let window_1 = c.window().await?;
            // 遍历元素
            for elem in eles {


                let item_path = elem.attr("href").await?.unwrap();
                let item_url = url.join(&item_path);

                let item_url2 = item_url.clone().unwrap();
                let path_segments =item_url2.path_segments().map(|c| c.collect::<Vec<_>>());
                let mut path_segments = path_segments.unwrap();
                // println!("🐶：{:?}",path_segments);

                let target_item_url = item_url.unwrap().to_string() ;

                if handeled_items.contains(&target_item_url) {
                    // println!("🐼： 已经处理过啦！");
                    continue
                }else{
                    let tmp_item_url = target_item_url.clone() ;
                    handeled_items.insert(tmp_item_url);
                }

            }
        }



        let item_count = handeled_items.len() ;
        println!("🐯元素个数:{}",handeled_items.len());

        // 😄🐲: 存到有序集合或者db中 然后就可以安全跳过某个区域 避免每次都重新运行 根据上次异常断点位置继续开始
        let mut skip_from_number = 67;

        // 使用 for 循环遍历集合
        // for item_url in handeled_items {
        for (idx, item_url) in handeled_items.into_iter().enumerate() {
            println!("🐢🐰🏃‍♀️: {}/{} => {}",idx,item_count , item_url);

             if idx < skip_from_number {
                continue ;
             }

            c.new_window(true).await?;
            // let window_2 = c.window().await?;

            let mut all_windows = c.windows().await?;
            // let new_window = all_windows
            //     .into_iter()
            //     .find(|handle| handle != &window_1)
            //     .expect("Should find a differing window handle");
            // let last_window = all_windows.last().unwrap();
            // let window_size = all_windows.len();
            // let last_window = all_windows.get(window_size - 1).unwrap();
            let last_window = all_windows.pop().unwrap();
            c.switch_to_window(last_window).await?;
            // c.goto(String::from(item_url.unwrap()).as_str()).await?;
            c.goto(item_url.as_str()).await?;


            // 点击图片
            // let selector = r#"a.pointer.image-holder"#;
            // 🤔：可以使用js 端技术 https://developer.mozilla.org/zh-CN/docs/Web/API/Document/querySelector
            // TODO: 这里有多余逻辑 懒得改了 用的其他网站逻辑
            // let selector = r#"li.slide.previous"#;
            let selector = r#"body"#;

            let ele_result = c.wait().for_element(Locator::Css(selector)).await;

            if ele_result.is_ok(){
                // 点击一下
                // ele_result.unwrap().click().await?;
                // println!("[debug:]<{:?}", ele_result);
                sleep_random(&c).await ;

                let img_urls = get_image_urls(&c).await;

                let img_info = get_image_info(&c).await;

                let img_url_count = img_urls.len();

                for (idx ,img_url) in img_urls.iter().enumerate() {
                    // println!("{:?}",img_info);
                    println!("img_url {:?}",img_url);
                    // let parts = img_url.rsplit('=');
                    let splits: Vec<&str> = img_url.rsplitn(2, '=').collect();
                    let target_url = splits.first().unwrap() ;
                    // https://www.iamwawa.cn/urldecode.html 在线解码工具
                    let decoded_url = urlencoding::decode( target_url).unwrap() ;
                    println!("splits 👍 {:?}",decoded_url);
                    println!("😷: {:?}",sanitize_filename::sanitize(img_info.clone()));

                    let folder_name = format!("{}_{}",search_user,PROGRAM_NAME);
                    let mut img_save_to_name =sanitize_filename::sanitize(img_info.clone());
                    if img_url_count > 1{
                         img_save_to_name = format!("{}_{}",img_save_to_name,idx);
                    }else{
                         // img_save_to_name = format!("{}_{}",img_save_to_name,idx);
                    }


                    download_image(&folder_name,&img_save_to_name,&decoded_url).await ;

                }



                // let selector = r#"//ul[@class="slider"]/li[@class="slide"]//img"#;
                // // class="slider-wrapper axis-horizontal"
                // let selector = r#"//div[@class="slider-wrapper"]/ul/li[@class="slide"]/img"#;
                // // let selector = r#"//li[@class="slide"]/img"#;
                // // let selector = r#"//div[@class="swiper-slide"]//img"#;
                // let eles = c.find_all(Locator::XPath(selector)).await?;
                // println!("🤔==== 相册元素 > {:?} ", eles.len());
                // for ele in eles {
                //     println!("💪 图片路径: {}", ele.attr("src").await?.unwrap());
                // }

                // 注意可能出现gallery的情况 TODO ：
                // ele.click().await?;
                //

                sleep_random(&c).await ;
                println!(" 😄 a single item is done 🤔") ;
            }else{
                println!("no image! 🤔") ;
            }


            // 关闭当前窗口
            c.close_window().await?;


            let mut  all_windows = c.windows().await?;
            all_windows.split_off(1) ; // 这步代码如果不人为干扰也可以不要 😄
            c.switch_to_window(all_windows.pop().unwrap()).await?
        }

        println!("🤔 接下来寻找分页链接");
        // xpath 选择表达式 ;
        let selector = r#"//li[contains(@class, 'pagination-next') and contains(@class, 'disabled')]"#; //  双斜杠，表示不管层级关系
        let selector = r#"//li[contains(@class, 'pagination-next') ]/a"#; //  双斜杠，表示不管层级关系
        // xpath 有大量内置函数 可以协助提取信息如 text()
        // let next_page_button = c.find(Locator::XPath(selector)).await?;

        let button_result = c.wait().for_element(Locator::XPath(selector)).await;

        if button_result.is_ok(){
            let button = button_result.unwrap();

            // // 查看是否含有 disable属性
            // let ele_attr =  button.attr("class").await?.unwrap();
            // println!("[ele_attr:] {}", ele_attr);
            println!("[next:]");
            // je.executeScript("arguments[0].scrollIntoView(true);", element);
            c
                .execute_async(
                    " setTimeout(() =>{arguments[0].scrollIntoView(true);arguments[0].click();arguments[1]( 1);},Math.floor(Math.random() * 100))",
                    // " setTimeout(() => alert('😄'),Math.floor(Math.random() * 3000))",
                    vec![serde_json::to_value(button.clone())?],
                )
                .await;
        // button.click().await?;
        }else{
            println!("[ 🐮 all  done! ]") ;
            break ;
        }



        //  // let source_html = c.source().await? ;
        //  // println!("{}", source_html);
        //
        // let button = c.wait().for_element(Locator::XPath("//center/a[last()]")).await?;
        //  // button.click().await?;
        //  let next_page_href = button.attr("href").await?.unwrap() ;
        //  println!("next href: {}", next_page_href.as_str());
        //  let next_page_num = get_page_num(next_page_href.as_str()) ;
        //  let next_page_url = get_page_url( url.as_str(),next_page_num);
        //  c.goto(next_page_url.as_str()).await ;
        // // c.goto(next_page_href.as_str()).await?;
    }


    c.close().await
}


fn get_page_num(item: &str) -> usize {
    if PAGE_REGEX.is_match(item) {
        println!("OK:{:#?}", item);
        let cap_names = PAGE_REGEX.captures(item).unwrap();
        println!("{:#?}", cap_names); // 这个结构有点像hash_map 呀
        // let page = cap_names.unwrap().name("n");
        // println!("{:#?}", page);

        let page_num = cap_names.name("n").unwrap();
        //println!("{:#?}", page_num.as_str());
        let page: i32 = page_num.as_str().parse::<i32>().unwrap();
        return page as usize;
    }
    0
}
fn get_page_url(item: &str , page: usize) -> String {
    if PAGE_REGEX.is_match(item) {
        println!("OK:{:#?}", item);
        let k = PAGE_REGEX.replace(item, |caps: &Captures| {
            format!("page={}", page) });
        return k.as_ref().to_string();
    }else{
        return "".to_owned() ;
    }

}

pub fn make_item_url(s: &str) -> String {
    // match s {
    //     "firefox" => "http://localhost:4444",
    //     "chrome" => "http://localhost:9515",
    //     browser => unimplemented!("unsupported browser backend {}", browser),
    // }
    String::new()
}

pub async  fn sleep_random(c:&fantoccini::Client) {
    // println!("<< enter: sleep_random");
    let rslt =  c
        .execute_async(
            " setTimeout(() => arguments[1](arguments[0] + 1),Math.floor(Math.random() * 1000))",
            // " setTimeout(() => alert('😄'),Math.floor(Math.random() * 3000))",
            vec![1_u32.into()],
        )
        .await.unwrap().as_u64(); // 返回值是2 搞测试而已

    // println!("[sleep_random->]{:#?}", rslt);
}

pub async  fn get_image_url(c:&fantoccini::Client) -> String {
    println!("<< enter: get_image_url");

    const JS: &'static str = r#"
            // 😄回调函数就叫 callback 可以给你用来返回任何东东哦！
            // ⚠️多参数时 callback回调函数总是出现在尾部

            // const [date, callback] = arguments;
            const [ callback] = arguments;

            // @see https://blog.darkthread.net/blog/js-delayexecute/
            //Parameters:
            //check      : a function to return true/false
            //proc       : a function which won't run until check() return true
            //chkInterval: optional, checking interval, default value is 500ms
            function delayExecute(check, proc, chkInterval) {
                //default interval = 500ms
                var x = chkInterval || 500;
                var hnd = window.setInterval(function () {
                    //if check() return true,
                    //stop timer and execute proc()
                    if (check()) {
                        window.clearInterval(hnd);
                        proc();
                    }
                }, x);
            }

             // var js_ruslts =
             // {
             //   items: getCurrentPageImages(),
             //   need_page: false
             // };
             //  callback( js_ruslts );

             var selectors = ".pointer.image-holder";

             delayExecute(
              function() {

                return document.querySelector(selectors) !== null ;
              },
              function() {
                // alert("Executed!");
                console.log("Executed!");

                const el = document.querySelector(selectors);
                el.addEventListener("click", function(e) {
                  // 获取大图地址 并异步返回给rust

                  setTimeout(function(){

                        const el = document.querySelector("\#zoom-img img");
                        let attribute = el.getAttribute("src");
                        callback(attribute);

                  }
                  , 6000); // 给6秒加载图片 应该差不多了😄
                  // callback(666); // 这里就是凑形式
                });

                el.click();
              }
            );

        "#;

    let rslt =  c
        .execute_async(
            JS,
            // " setTimeout(() => alert('😄'),Math.floor(Math.random() * 3000))",
            // vec![1_u32.into()], // ⚠️ 这里传参与否 或者传的多少都会改变callback的位置
            vec![],
        )
        .await.unwrap(); // 返回值是2 搞测试而已
    // rslt.as_str() ;// as_xxx 方法是做转型的

    println!("[get_image_url->]{:#?}", rslt);
    rslt.to_string()
}
pub async  fn get_image_info(c:&fantoccini::Client) -> String {
    println!("<< enter: get_image_info");

    const JS: &'static str = r#"
            // 😄回调函数就叫 callback 可以给你用来返回任何东东哦！
            // ⚠️多参数时 callback回调函数总是出现在尾部

            // const [date, callback] = arguments;
            const [ callback] = arguments;

            function getItemInfo(){
                // 获取所有符合xpath表达式的元素
                var result = document.evaluate('//div[@class="title"]', document, null,XPathResult.ANY_TYPE, null);
                // 获取第一个元素
                var node = result.iterateNext();
                // console.log(node.innerText);
                var resultText = node.innerText;
                if(resultText.includes('）')){
                  resultText = resultText.split('）').pop();
                }


                // 获取唯一id 做后缀区分不同图片的名称 防止冲突 ；
                var currentUrl = window.location.href; // 当前url获取有很多方法
                let regex = /id\/\d+/g; // 使用 'g' 标志进行全局搜索
                let matches = currentUrl.match(regex);
                console.log(matches);
                // var suffix = currentUrl.split('/').pop();
                let suffix = matches[0].replace('/','_');

                // 图录号
                var  el = document.querySelector('div.info>p');
                var imgNum = el.innerText;
                console.log(imgNum.split(':').pop());


                return resultText + '_'+ suffix;

            }

          setTimeout(function(){

                callback(getItemInfo());
          }
          , 6000); // 给6秒加载图片 应该差不多了😄

        "#;

    let rslt =  c
        .execute_async(
            JS,
            // " setTimeout(() => alert('😄'),Math.floor(Math.random() * 3000))",
            // vec![1_u32.into()], // ⚠️ 这里传参与否 或者传的多少都会改变callback的位置
            vec![],
        )
        .await.unwrap(); // 返回值是2 搞测试而已
    // rslt.as_str() ;// as_xxx 方法是做转型的

    println!("[get_image_info->]{:#?}", rslt);
    rslt.to_string()
}
// pub async  fn get_image_urls(c:&fantoccini::Client) -> String {
pub async  fn get_image_urls(c:&fantoccini::Client) -> Vec<String> {
// pub async  fn get_image_urls(c:&fantoccini::Client) -> Vec<&'static str> {
    println!("<< enter: get_image_urls");

    const JS: &'static str = r#"
            // 😄回调函数就叫 callback 可以给你用来返回任何东东哦！
            // ⚠️多参数时 callback回调函数总是出现在尾部

            // const [date, callback] = arguments;
            const [ callback] = arguments;


              // 获取大图地址 并异步返回给rust

              setTimeout(function(){

                    var eles = document.querySelectorAll("div.gallery-thumbs div.swiper-slide img") ;
                    console.log(eles) ;

                    var src_list = [] ;
                    eles.forEach(function(ele) {
                        // console.log(ele.src);
                        src_list.push(ele.src);
                    });

                    // ⚠️ 如果是空的 可能不属于相册情形 就只有一张图片啦！
                    if(eles.length == 0){
                        // alert('一张图片呀');
                        var ele = document.querySelector("div.chr-lot-header__media div.chr-img__wrapper img") ;
                         src_list.push(ele.src);

                    }

                    // 没有特殊计算就不用做这个！
                    // 所有的图片url做原地址获取
                    // src_list = src_list.map(function(src){
                    //     return src.split('?')[0];
                    // });


                    callback(src_list);

              }
              , 5000); // 给3秒加载图片 应该差不多了😄




        "#;

    let rslt =  c
        .execute_async(
            JS,
            // " setTimeout(() => alert('😄'),Math.floor(Math.random() * 3000))",
            // vec![1_u32.into()], // ⚠️ 这里传参与否 或者传的多少都会改变callback的位置
            vec![],
        )
        .await.unwrap(); // 返回值是2 搞测试而已

    println!("[get_image_urls->]{:#?}", rslt);
    let items = rslt.as_array().unwrap() ;

    // println!("[🐯图片总数->]{:#?}", items.len());
    let mut results = vec![] ;
    for item in items {
         //println!("[🐷get_image_url->]{:#?}", item) ;
        // results.push(item.to_string()) ;
        results.push(item.as_str().unwrap().to_string() ) ;
        // results.push(item.as_str().unwrap() ) ;
    }
    // println!(" 🌹 [get_image_urls->]{:#?}", results);
    // // rslt.as_str() ;// as_xxx 方法是做转型的
    // //
    // // println!("[get_image_urls->]{:#?}", rslt);
    // // rslt.to_string()
    // String::from("test")

    return results ;


}

pub async fn download_image(folder_name:&str,img_save_to_name:&str,img_url:&str,){

    // 临时测试下
    // let img_url = "\"https://www.polyauctionhk.com/lib/uploads/165598976462b4660473f07/424/424.jpg\"";
    let img_url = img_url.trim_matches('"');
    if img_url.ends_with(".svg"){
        println!("💢 没图片么！");
        return () ;
    }

    let img_save_to_name2 = img_save_to_name.clone();

    let img_save_to_name = format!("{}.jpg",img_save_to_name);
    println!("[debug 🤔]: {folder_name}_{img_save_to_name} from 【{img_url}】") ;
    // 下载
    let mut download_dir = PathBuf::from(DL_DIR);
    download_dir.push(folder_name);
    let save_to_folder = download_dir.as_path() ;

    if !std::path::Path::new(save_to_folder).is_dir(){
        std::fs::create_dir_all(save_to_folder).expect("can't create download folder!");
    }

    let mut save_to_path = std::path::Path::new(save_to_folder)
        // .join(folder_name)
        .join(img_save_to_name) ;
    println!("save to path: {}", save_to_path.display());

    let mut download_flag = false;

    if save_to_path.exists(){
        let matedata = std::fs::metadata(save_to_path.clone()).unwrap();
        if matedata.is_file() {
            if   matedata.len() == 0 {
                // 文件存在且是空文件
                download_flag = true;

            }else{
                // 已经下载过了
                println!("文件已下载! 不需要重复下载");
            }
        }
    }else{
        // 不存在
        download_flag = true;
    }

    if download_flag {

        println!("now will download it! {}",save_to_path.display()) ;

        // TODO 下载老出现超时 😢 https://docs.rs/tokio/latest/tokio/macro.select.html 可以尝试加入超时机制
        let rslt = download::fetch_url2(
            img_url.to_string(),
            save_to_path.to_str().unwrap().to_string(),
        )
            .await;
        if rslt.is_err() {
            // TODO : 出现下载失败可以保存到某个数据库 然后在重新执行尝试下载
            println!("some error happened :{:?}", rslt.err());

            let tree = db_tree();
            let key = format!("{}:{}",save_to_folder.display(),img_save_to_name2);
            tree.insert(key, img_url);

        }else{

            println!("download ok!") ;
        }
    }

    // =================================== 下载 >> ======================
}

fn db_tree()->sled::Tree{
    ROOT_DB.open_tree("my_tree_space").unwrap()
}

#[test]
fn test_get_page_url() {
    let url = "index.asp?page=10";
    let url2 = get_page_url(url,12);
    assert_eq!(url2,"index.asp?page=12")
}