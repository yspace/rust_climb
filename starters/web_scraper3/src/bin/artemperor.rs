use std::{path::PathBuf, borrow::Borrow};
use std::ops::Deref;
use std::str::FromStr;
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

lazy_static! {
    static ref PAGE_REGEX: Regex = Regex::new(r"p=(?P<n>\d+)").unwrap();
}
const DL_DIR: &'static str = "__downloads";

// ⚠️ 这个网站用了 shadowDom ：[Web Components详解-Shadow DOM基础](https://developer.aliyun.com/article/1557247)

// let's set up the sequence of steps we want the browser to take
#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {

    println!("😄：just for learning ；scrap images from 帝国");

    // 无头配置
    let cap: Capabilities = serde_json::from_str(
        r#"{"browserName":"chrome","goog:chromeOptions":{"args":["--headless"]}}"#,
     )
        .unwrap();

    let c = ClientBuilder::native()
        .capabilities(cap) // ⚠️ 无头配置如果看不到情况 可以注释掉此行变为有头 看效果
        .connect("http://localhost:4444")
        .await
        .expect("failed to connect to WebDriver");



    // let target_url = "https://auctions.artemperor.tw/search/%E6%BA%A5%E5%BF%83%E7%95%AC";

    let search_user = "溥儒";
    // let search_user = "黄君璧";
    let search_user = "溥心畬";
    // let search_user = "陈少梅";
    // let search_user = "赵国经";
    // // let search_user = "王美芳";
    // // let search_user = "齐白石";
    // let search_user = "丰子恺";
    // let search_user = "徐燕孙";
    // let search_user = "黄均";
    let search_user = "石涛";
    let search_user = "黃君璧";
    // build|make target_url
    // let target_url = format!("https://www.polyauction.com.hk/en/search?q={}&type=lot&p=1", search_user) ;
    // let page = 1;
    let target_url = format!("https://auctions.artemperor.tw/search/{}", search_user) ;
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
        // c.wait_for(move |_| {
        //     std::thread::sleep(Duration::from_secs(7));
        //     async move { Ok(true) }
        // })
        //     .await?;


        // -----------------------------执行异步js--------------------------------------------------

        const JS: &'static str = r#"
            // 😄回调函数就叫 callback 可以给你用来返回任何东东哦！

            // const [date, callback] = arguments;
            const [ callback] = arguments;


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
                // var ele_images = document.querySelector("ul.artworks>li>a").shadowRoot.querySelectorAll("img.cgImage");
                // var ele_images = document.querySelector("wujie-app").shadowRoot.querySelectorAll("div.sudokuItem");


                var elements = document.querySelectorAll("ul.artworks>li>a");
                var js_ruslts = [] ;
                for (let element of elements) {
                  // console.log(element);

                  let s_item_href = element.href;

                  js_ruslts.push(s_item_href);
                  // 🤔 其实可以在这里一并处理 获取相关的信息
                }

                  var ele_infos = document.querySelectorAll("ul.artworks>li>a>div.info");;
                var js_ruslts2 = [] ;
                for (let element of ele_infos) {
                  // console.log(element);

                  //
                  var ele_h4 = element.querySelector('h4');

                  var title = element.textContent ;
                  console.log(title);

                  js_ruslts2.push(title);
                }

                // js_ruslts2 =  addSuffix4Duplicates(js_ruslts2);
                // js_ruslts = zip(js_ruslts,js_ruslts2);


                return js_ruslts;
            }

             setTimeout(function(){

                     console.log("---->>>----");
                     console.log("---->>>----");

                     var js_ruslts =
                     {
                       items: getCurrentPageItems(),
                       need_page: false
                     };

                     // 分页逻辑
                     console.log('分页逻辑开始....');

                     // var cond_need_page = true ; // 😄这里可以手动设值
                     var cond_need_page = false;
                     // TODO 这里逻辑好像有问题 😄
                     while(cond_need_page == true) {
                       var root_ele = document.querySelector("wujie-app").shadowRoot;

                        const currentPage = root_ele.querySelector('li.is-active.number');
                        const nextPage = currentPage.nextElementSibling;
                        console.log(nextPage);

                        var pageNumber = currentPage.innerText;
                        js_ruslts['page_number'] = pageNumber ;

                        // 检查是否存在下一个兄弟元素节点
                        if (nextPage !== null) {
                             //var attributeValue = nextSibling.getAttribute('属性名');
                             // console.log(attributeValue);
                             console.log('模拟下点击分页');

                              cond_need_page = false ;
                             js_ruslts['need_page'] = true ;
                             // 点击分页
                             nextPage.click();

                        } else {
                            // 分页结束
                            console.log('不存在下一个兄弟元素节点');
                            cond_need_page = false ;

                            // 通知rust

                            // callback('END');
                            js_ruslts['need_page'] = false ;
                        }
                     }

                         // callback( "hi heree" )
                      callback( js_ruslts );

             },1000) ;
        "#;

        // let mut js_result = c.execute_async(JS, vec![]).await?;
         // println!("js result here :{:?}", js_result);
        // -------------------------------------------------------------------------------
        // 检查是否为数组
        // https://docs.rs/serde_json/latest/serde_json/enum.Value.html
        use serde_json::Value;

        let mut need_page = true;
        // let mut js_result_items = js_result["items"].as_array().unwrap() ;
        //
        // println!("js result length :{:?}", js_result_items.len());
        // for item in js_result_items {
        //     println!("{}", item.as_str().unwrap());
        // }
        // ---------------------------------👆是js获取当前页的待处理元素的逻辑 跟👇下面逻辑类似 可以任选一个修改逻辑         -------------------------------------------

        println!("处理当前页面 😄");
        // 页码
        let mut page_number =0;

        // 获取本页需要处理的图片
        let selector = r#"//ul[@class="artworks"]/li/a"#;
        let selector = r#"//ul[@class="artworks"]//li/a"#;
        let eles = c.find_all(Locator::XPath(selector)).await?;

        let selector = r#"ul.artworks>li>a"#;
        let eles = c.find_all(Locator::Css(selector)).await?;
        let total_items = eles.len();
        println!("找到 {:?} 个目标元素",total_items);
        let window_1 = c.window().await?;
        // 遍历元素
        let mut count = 0;
        let mut skip_from_number = -1;
        for elem in eles {

            count += 1;
            if count < skip_from_number {
                continue ;
            }
            println!(">>>>>>>>>>>>>>>==================          🐶🐶🐶{}/{}", count, total_items );

            // //ele.
            //  // Test mouse down/up.
            //  let mouse_actions = MouseActions::new("mouse".to_string())
            //      .then(PointerAction::MoveToElement {
            //          element: elem,
            //          duration: None,
            //          x: 0,
            //          y: 0,
            //      })
            //      .then(PointerAction::Down {
            //          button: MOUSE_BUTTON_LEFT,
            //      })
            //      // .then(KeyAction::Down {
            //      //     value: Key::Down.into(),
            //      // }.into())
            //      .then(PointerAction::Up {
            //          button: MOUSE_BUTTON_LEFT,
            //      });
            //
            //  c.perform_actions(mouse_actions).await?;

            // Press CONTROL key down and hold it.
            // let key_actions = KeyActions::new("key".to_string()).then(KeyAction::Down {
            //     value:Key::Down.into(),
            // });
            // c.perform_actions(key_actions).await?;
            //
            // // Now release all actions. This should release the control key.
            // c.release_actions().await?;
            // println!("{}",elem.attr("href").await?.unwrap());
            let item_path = elem.attr("href").await?.unwrap();
            // let item_url = url.join(&item_path);
            let item_url = Url::from_str(item_path.as_str()) ;// url.join(&item_path);

            let item_url2 = item_url.clone().unwrap();
            let path_segments =item_url2.path_segments().map(|c| c.collect::<Vec<_>>());
            let mut path_segments = path_segments.unwrap();
            println!("🐶：{:?}",path_segments);


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
            c.goto(String::from(item_url.unwrap()).as_str()).await?;

            // 点击图片
            // let selector = r#"a.pointer.image-holder"#;
            // 🤔：可以使用js 端技术 https://developer.mozilla.org/zh-CN/docs/Web/API/Document/querySelector
            let selector = r#"img#WORK"#;
            println!("[debug:] for pointer");
            let ele_result = c.wait().for_element(Locator::Css(selector)).await;
            // let ele = ele_result.unwrap();
            // match ele_result {
            //     Ok(el) => el,
            //     Err(error) => panic!("Problem opening the file: {error:?}"),
            //     // Err(error) => continue,
            // };
            if ele_result.is_ok(){
                sleep_random(&c).await ;
                // 注意可能出现gallery的情况 TODO ：
                // ele.click().await?;

                // 获取图片信息 主要包括url 名称 防止冲突可以带上某种唯一标识

                let selector = r#"#WORK"#;
                // let selector = r#"img#WORK"#;
                let ele = c.find(Locator::Css(selector)).await?;
                // let img_url = ele.attr("src").await?;
                // ⚠️ 这里可能出现相册多张图片的问题 TODO 暂时没有处理！！！
                let img_url = ele.attr("data-zoom-image").await?;
                // println!("[debug:] for image {}",img_url.unwrap());

                // 获取图片其他信息
                let selector = r#"nav.nav_lot h3"#;
                let ele = c.find(Locator::Css(selector)).await?;
                // println!("[debug:]  {}",ele.text().await?);
                let img_lot = ele.text().await? ;
                let img_lot = img_lot.replace(".","");
                let img_lot = img_lot.replace(" ","");

                let selector = r#"div.info_box>h3"#;
                let ele = c.find(Locator::Css(selector)).await?;
                println!("[debug:]  {}",ele.text().await?);
                let img_name = ele.text().await?;



                // let img_url =  get_image_url(&c).await ; // ⚠️这里有坑 返回的字符串带双引号 没做json反序列化处理

                let folder_name = format!("{}_artemperor",search_user);
                // let img_save_to_name = path_segments.pop().unwrap();
                // let img_save_to_name = format!("{}_p{}",img_save_to_name,page_num);
                let mut img_save_to_name = format!("{}_{}",img_name,img_lot);
                println!("😊未净化的文件名: {}",img_save_to_name);
                img_save_to_name = sanitize_filename::sanitize(img_save_to_name);

                download_image(&folder_name,&img_save_to_name,&img_url.unwrap()).await ;

                sleep_random(&c).await ;
                println!("sleep_random 🤔") ;
            }else{
                println!("no image! 🤔") ;
            }




            // 关闭当前窗口
            c.close_window().await?;


            let mut  all_windows = c.windows().await?;
            all_windows.split_off(1) ; // 这步代码如果不人为干扰也可以不要 😄
            c.switch_to_window(all_windows.pop().unwrap()).await?
        }



        // let download_btn = client
        //     .wait()
        //     //r#"a[href="/learn/get-started"]"#,
        //     .for_element(Locator::XPath(r#"//a[@class="download-btn"]"#))
        //     .await?;
        // download_btn.click().await?;
        //
        // sleep(Duration::from_millis(30000)).await;

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
            let  _ = c
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




    }
    // 按照分页逻辑处理？


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
    // 暂停随机时间：js端或者 rust端都可以做到
    // c
    //     .execute_async(
    //         "setTimeout(() => arguments[1](arguments[0] + 1),Math.floor(Math.random() * 1000))",
    //         vec![1_u32.into()],
    //     )
    //     .await?; // 返回值是2 搞测试而已
    //
    // c.wait_for(move |_| {
    //     std::thread::sleep(Duration::from_secs(7));
    //     async move { Ok(true) }
    // })
    //     .await?;
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

pub async fn download_image(folder_name:&str,img_save_to_name:&str,img_url:&str,){

    // 临时测试下
    // let img_url = "\"https://www.polyauctionhk.com/lib/uploads/165598976462b4660473f07/424/424.jpg\"";
    let img_url = img_url.trim_matches('"');
    if img_url.ends_with(".svg"){
        println!("💢 没图片么！");
        return () ;
    }

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
            println!("some error happened :{:?}", rslt.err());
        }else{

            println!("download ok!") ;
        }
    }

    // =================================== 下载 >> ======================
}

#[test]
fn test_get_page_url() {
    let url = "index.asp?page=10";
    let url2 = get_page_url(url,12);
    assert_eq!(url2,"index.asp?page=12")
}