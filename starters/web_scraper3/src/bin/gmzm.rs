use std::{path::PathBuf, borrow::Borrow};

use fantoccini::{ClientBuilder, Locator};
use url::{Host, ParseError, Url};

use lazy_static::lazy_static;
use regex::{Regex, Captures};

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
    let target_url = "http://gmzm.org/gudaizihua/yuanmingyuan/index.asp?page=39";

    let target_url = "http://gmzm.org/gudaizihua/CHANGJUAN/%E4%B9%9D%E9%BE%99%E5%9B%BE/index.asp?page=19";
    let target_url = "http://gmzm.org/gudaizihua/changjuan/%E6%99%8B%E6%96%87%E5%85%AC%E5%A4%8D%E5%9B%BD%E5%9B%BE/index.asp?page=16";
    let target_url = "http://gmzm.org/gudaizihua/%E5%8D%97%E4%BA%AC%E5%8D%9A%E7%89%A9%E9%99%A2%E8%97%8F/index.asp?page=126";
    let target_url = "http://gmzm.org/gudaizihua/%E8%B5%A4%E5%A3%81%E6%80%80%E5%8F%A4/index.asp?page=1";
    let target_url = "http://gmzm.org/gudaizihua/changjuan/index.asp?page=1";
    let target_url = "http://gmzm.org/gudaizihua/%E5%AE%8B%E5%90%8D%E6%B5%81%E9%9B%86%E8%97%BB%E5%86%8C/index.asp?page=2";
    let target_url = "http://gmzm.org/gudaizihua/erwang/index.asp?page=1";
    let target_url = "http://gmzm.org/gudaizihua/%E9%81%93%E5%BE%B3%E7%B5%8C%E6%99%8B%E5%8F%B3%E8%BB%8D%E7%8E%8B%E7%BE%B2%E4%B9%8B%E6%9B%B8/index.asp?page=1";
    let target_url = "http://gmzm.org/gudaizihua/qianchashan/index.asp?page=1";
    let target_url = "http://gmzm.org/gudaizihua/ziyan/index.asp?page=1";
    let target_url = "http://gmzm.org/gudaizihua/jieziyuan/03/index.asp?page=24";
    let target_url = "http://gmzm.org/gudaizihua/CHANGJUAN/%E4%B8%AD%E7%A7%8B%E5%B8%96/index.asp?page=2";
    let target_url = "http://gmzm.org/gudaizihua/%E9%81%93%E5%BE%B3%E7%B5%8C%E6%99%8B%E5%8F%B3%E8%BB%8D%E7%8E%8B%E7%BE%B2%E4%B9%8B%E6%9B%B8/index.asp?page=1";
    let target_url = "http://gmzm.org/gudaizihua/shuihurenwu/index.asp?page=1";
    let target_url = "http://gmzm.org/gudaizihua/yuanqutu/index.asp?page=1";

    c.goto(target_url).await?;

    let last_page_link = c.wait().for_element(Locator::XPath("//center/a[last()-1]")).await?;
    let last_page_link = c.wait().for_element(Locator::XPath("//center/a[last()-1]")).await?;
    let last_page_href = last_page_link.attr("href").await ? ;
    // println!("last_page_href:{}", last_page_href.unwrap()) ;
    let last_page_num = get_page_num(last_page_href.unwrap().as_str()) ;

    let mut pre_url:Option<Url> = None;

    loop {
        let url = c.current_url().await?;

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