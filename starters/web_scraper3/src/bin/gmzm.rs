use std::{path::PathBuf, borrow::Borrow};

use fantoccini::{ClientBuilder, Locator};
use url::{Host, ParseError, Url};

use lazy_static::lazy_static;
use regex::Regex;

use web_scraper3::*;

lazy_static! {
    static ref PAGE_REGEX: Regex = Regex::new(r"page=(?P<n>\d+)").unwrap();
}
const DL_DIR: &'static str = "__downloads";

// let's set up the sequence of steps we want the browser to take
#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let c = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await
        .expect("failed to connect to WebDriver");

    let target_url = "http://gmzm.org/SUCAI/%E5%9B%BD%E7%94%BB%E7%B4%A0%E6%9D%90-%E5%B1%B1%E6%B0%B4%E7%94%BB/index.asp?page=2";
    let target_url = "http://gmzm.org/gudaizihua/jieziyuan/04/index.asp?page=544";
    let target_url = "http://b.gmzm.org/2018/%E5%8F%A4%E4%BB%A3%E5%AD%97%E7%94%BB/%E6%9D%8E%E5%94%90%E9%A3%8E%E6%A0%BC%E7%9A%84%E5%B1%B1%E6%B0%B4%E7%94%BB/index.asp?page=1";
    let target_url = "http://gmzm.org/gudaizihua/mingren/index.asp?page=18";
    let target_url = "http://gmzm.org/gudaizihua/CHANGJUAN/%E4%B8%AD%E7%A7%8B%E5%B8%96/";
    let target_url = "http://gmzm.org/sucai/%E4%BC%A0%E4%B8%96%E5%90%8D%E7%94%BB%E8%8A%B1%E9%B8%9F/index.asp?page=1";
    let target_url = "http://gmzm.org/gudaizihua/CHANGJUAN/%E4%BA%94%E8%89%B2%E9%B9%A6%E9%B9%89%E5%9B%BE/index.asp?page=1";
    let target_url = "http://gmzm.org/GUDAIZIHUA/changjuan/%E8%99%8E%E4%B8%98%E5%9B%BE%E5%8D%B7/index.asp?page=1";
    let target_url = "http://gmzm.org/gudaizihua/CHANGJUAN/index.asp?page=3"; // 这个老超时
    let target_url = "http://b.gmzm.org/2018/%E5%8F%A4%E4%BB%A3%E5%AD%97%E7%94%BB/%E7%A7%8B%E5%A4%A9%E6%B2%B3%E7%9A%84%E6%B8%94%E5%A4%AB/";

    c.goto(target_url).await?;

    let mut pre_url = None;

    loop {
        let url = c.current_url().await?;

        if pre_url.is_some() {
            if url == pre_url.unwrap() {
                println!("last page reached");
                break;
            }
        }

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

        button.click().await?;
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
