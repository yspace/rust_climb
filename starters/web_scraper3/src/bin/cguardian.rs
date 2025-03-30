use std::{path::PathBuf, borrow::Borrow};

use fantoccini::{ClientBuilder, Locator};
use url::{Host, ParseError, Url};

use lazy_static::lazy_static;
use regex::{Regex, Captures};

use web_scraper3::*;

use std::time::Duration;

use visdom::types::BoxDynError;
use visdom::Vis;


lazy_static! {
    static ref PAGE_REGEX: Regex = Regex::new(r"page=(?P<n>\d+)").unwrap();
}
const DL_DIR: &'static str = "__downloads";

// ⚠️ 这个网站用了 shadowDom ：[Web Components详解-Shadow DOM基础](https://developer.aliyun.com/article/1557247)

// let's set up the sequence of steps we want the browser to take
#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let c = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await
        .expect("failed to connect to WebDriver");



    // https://www.chanliu.com/product-tag/%e6%ba%a5%e5%84%92%ef%bc%88%e6%ba%a5%e5%bf%83%e7%95%ac%ef%bc%89/
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E6%BA%A5%E5%84%92";
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E6%BA%A5%E5%84%92";



    c.goto(target_url).await?;

    // let last_page_link = c.wait().for_element(Locator::XPath("//center/a[last()-1]")).await?;
    let last_page_link = c.wait().for_element(Locator::XPath("//a[last()-1]")).await?;
    let last_page_href = last_page_link.attr("href").await ? ;
    // println!("last_page_href:{}", last_page_href.unwrap()) ;
    let last_page_num = get_page_num(last_page_href.unwrap().as_str()) ;

    let mut pre_url:Option<Url> = None;

    loop {
        println!("enter_loop");

        let url = c.current_url().await?;


        #[allow(deprecated)]
        c.wait_for(move |_| {
            std::thread::sleep(Duration::from_secs(7));
            async move { Ok(true) }
        })
            .await?;


        println!("获取html...！>>>");
        let el_items = c
            .wait()
            .for_element(Locator::Css(".wujie_iframe"))
            .await?;
        println!("获取html...！");

        // -----------------------------执行异步js--------------------------------------------------
        const JS: &'static str = r#"
            // 😄回调函数就叫 callback 可以给你用来返回任何东东哦！

            // const [date, callback] = arguments;
            const [ callback] = arguments;

             setTimeout(function(){
             // alert("哈哈哈😄");
             console.log("---->>>----");
            //  document.addEventListener("DOMContentLoaded", function() {
            //     // DOM树加载完毕后执行这段代码
            //     // var element = document.getElementById("yourElementId");
            //     // console.log(element);
            //
            //      console.log("----dom 🌲----");
            //     console.log(document.getElementById("child-app-container").innerHtml);
            // });

            // var text = document.querySelector("wujie-app").shadowRoot.querySelector("html").outerHtml;
            // var text = document.querySelector("wujie-app").shadowRoot.querySelectorAll("img.cgImage").outerHTML;
            // var ele_images = document.querySelector("wujie-app").shadowRoot.querySelectorAll("img.cgImage");
            // var text = document.querySelector("wujie-app").shadowRoot.querySelector(".itemList").innerHTML;
            // document.querySelector("wujie-app").shadowRoot.querySelectorAll("img.cgImage");
            // console.log(text);
            console.log("---->>>----");

            // var my_f = function(){}
            function getCurrentPageImages()
            {
                var ele_images = document.querySelector("wujie-app").shadowRoot.querySelectorAll("img.cgImage");
                var js_ruslts = [] ;
                for (let element of ele_images) {
                  console.log(element);

                  let s_img_src = element.src;
                  let l_img_src = s_img_src.replace("/s.jpg", "/l.jpg");

                  js_ruslts.push(l_img_src);
                }
                return js_ruslts;
            }

            var js_ruslts = getCurrentPageImages() ;
            // for (let element of ele_images) {
            //   console.log(element);
            //
            //   let s_img_src = element.src;
            //   let l_img_src = s_img_src.replace("/s.jpg", "/l.jpg");
            //
            //   js_ruslts.push(l_img_src);
            // }

                 // callback( "hi heree" )
                // callback( text );
                callback( js_ruslts );

             },1000) ;
        "#;

        let js_result = c.execute_async(JS, vec![]).await?;
         println!("js result here :{:?}", js_result);
        // -------------------------------------------------------------------------------
        // 检查是否为数组
        use serde_json::Value;
        if js_result.is_array() {
            for value in js_result.as_array().unwrap() {
                match value {
                    Value::String(s) => println!("the src is :{}", s),
                    _ => println!("Not a string"),
                }
            }
        } else {
            println!("Not an array");
        }

        // let source = c.source().await?;
        // let source = js_result.as_str().unwrap();
        // println!("js result here :{:?}", source);

        // 在rust端处理元素 析取数据
        // 文档： https://github.com/fefit/visdom/wiki/%E4%B8%AD%E6%96%87API%E6%96%87%E6%A1%A3
        // load html
        // let root = Vis::load(source).unwrap();
        // let my_img = root.find("img");
        // // let lis = root.find("#header li");
        // // let my_text = my_img.text();
        // println!("how many image 😄 :{}", my_img.length());
        // let href = links.attr("href").unwrap().to_string();
        //
        // let dom_detail = root.find("div.project-detail");
        // let ele_title = dom_detail.find("h3");
        // let title = ele_title.text();
        // let detail = dom_detail.html();
        // println!("获取source...！{}", source);

        // let el_items_html = el_items.html(false).await?;
        // let el_items_html = el_items.text().await?;
        // let el_items_html = el_items.html(false).await?;
        //
        //  println!("html: {}", el_items_html);

        println!("ele_imgs >>");
        // 找到目标img
        // let ele_imgs = c.find_all(Locator::XPath("//div[@id=\"app\"]//img[@class=\"cgImage\"]")).await?;
        // let ele_imgs = c.find_all(Locator::XPath("//img")).await?;
        // let mut ele_imgs = c.find_all(Locator::XPath("//div[@class=\"itemList\"]")).await?;
        // let mut ele_imgs = c.find_all(Locator::XPath(r#"//div[@class="itemList"]//img"#)).await?;
        let mut ele_imgs = c.find_all(Locator::XPath(r#"//img[contains(@class, 'cgImage')]"#)).await?;
        // let mut ele_imgs = c.find_all(Locator::Css(".cgImage")).await?;
        // println!("ele_imgs: {:?}", ele_imgs);
        for (i, img) in ele_imgs.iter_mut().enumerate() {
            println!("ele_img i: {}", i );
            // Each "place" has a link element.
            // let place_title = place.find(Locator::Css("a")).await?;
            // let place_title = place_title.text().await?;
            // if place_title.is_empty() {
            //     assert!(i >= place_titles.len(), "{} >= {}", i, place_titles.len());
            // } else {
            //     assert_eq!(place_title, place_titles[i]);
            //}
            // println!("ele_img src: {}", img.attr("src").await );
            let src = img.attr("src").await?.expect("image should have a src");
            println!("ele_img src: {}", src );
        }


        let current_page_num = get_page_num(url.as_str());
        
        if current_page_num > last_page_num {
            println!("last page reached");
                break;
        }
        /* 
        if pre_url.is_some() {
            //  let pre_url2 = pre_url.unwrap() ;
           
             if url == pre_url.unwrap() {
                println!("last page reached");
                break;
            }
           
            // println!("{}: {}", pre_url2.as_str(), url.as_str());
        }*/



        // xpath 选择表达式 ;
        let selector = "//center//a[last()]"; //  双斜杠，表示不管层级关系
                                              // xpath 有大量内置函数 可以协助提取信息如 text()
        let button = c.wait().for_element(Locator::XPath(selector)).await?;

        // let source_html = c.source().await? ;
        // println!("{}", source_html);

        // TODO 是不是直接用source 获取html文件后解析比较快 不需要等待元素加载？
        let el_img = c
            .wait()
            .for_element(Locator::XPath("//center//td//span/img"))
            .await?;

        let img_src = el_img.attr("src").await?;
        let img_save_to_name = img_src.unwrap().clone();

        let url2 = url.clone(); 
        let url_segments = url2.path_segments().map(|c| c.collect::<Vec<_>>()).unwrap();
        let folder_name = url_segments.get(url_segments.len()-2).unwrap();
        let folder_name = urlencoding::decode(folder_name).unwrap();
        let folder_name:&str = folder_name.borrow();

        let img_url = url.join(img_save_to_name.as_str())?;


        // 下载
        let mut download_dir = PathBuf::from(DL_DIR);
        download_dir.push(folder_name);
        let save_to_folder = download_dir.as_path() ;
        
        if !std::path::Path::new(save_to_folder).is_dir(){
            std::fs::create_dir_all(save_to_folder).expect("can't create download folder!");
        }
      
        let mut save_to_path = std::path::Path::new(save_to_folder)
        // .join(folder_name)
        .join(img_save_to_name);
        println!("save to path: {}", save_to_path.display());
        
        let mut download_flag = false;

        if save_to_path.exists(){
            let matedata = std::fs::metadata(save_to_path.clone())?;
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

           let rslt = download::fetch_url(
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
       // NOTE: 双斜杠 取到img哪里去了 所以一直有问题！

       let button = c.wait().for_element(Locator::XPath("//center/a[last()]")).await?;
        // button.click().await?;
        let next_page_href = button.attr("href").await?.unwrap() ;
        println!("next href: {}", next_page_href.as_str());
        let next_page_num = get_page_num(next_page_href.as_str()) ;
        let next_page_url = get_page_url( url.as_str(),next_page_num); 
        c.goto(next_page_url.as_str()).await ;
       // c.goto(next_page_href.as_str()).await?;
        pre_url = Some(url);
    }

    // // click "Foo (disambiguation)"
    // c.find(Locator::Css(".mw-disambig")).await?.click().await?;

    // // click "Foo Lake"
    // c.find(Locator::LinkText("Foo Lake")).await?.click().await?;

    // let url = c.current_url().await?;
    // assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foo_Lake");

    c.close().await
}

async fn _main() -> Result<(), fantoccini::error::CmdError> {
    let c = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await
        .expect("failed to connect to WebDriver");

    // first, go to the Wikipedia page for Foobar
    c.goto("https://en.wikipedia.org/wiki/Foobar").await?;
    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foobar");

    // click "Foo (disambiguation)"
    c.find(Locator::Css(".mw-disambig")).await?.click().await?;

    // click "Foo Lake"
    c.find(Locator::LinkText("Foo Lake")).await?.click().await?;

    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foo_Lake");

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

#[test]
fn test_get_page_url() {
    let url = "index.asp?page=10";
    let url2 = get_page_url(url,12);
    assert_eq!(url2,"index.asp?page=12")
}