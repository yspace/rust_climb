use fantoccini::{ClientBuilder, Locator};
use url::{Host, ParseError, Url};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PAGE_REGEX: Regex = Regex::new(r"page=(?P<n>\d+)").unwrap();
}

// let's set up the sequence of steps we want the browser to take
#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let c = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await
        .expect("failed to connect to WebDriver");

    let target_url = "http://gmzm.org/SUCAI/%E5%9B%BD%E7%94%BB%E7%B4%A0%E6%9D%90-%E5%B1%B1%E6%B0%B4%E7%94%BB/index.asp?page=2";

    c.goto(target_url).await?;

    let mut pre_url = None ;

    loop {

        let url = c.current_url().await?;

        if pre_url.is_some() {
            if url == pre_url.unwrap(){
                println!("last page reached");
                break; 
            }
        }
    
        // 最后一页 可以有多种途径查到这个元素不过这里主要是想试试xpath表达式
        let last_page_ele = c
            .wait()
            .for_element(Locator::XPath("//center//a[string()=\">>\"]"))
            .await?;
        let last_page_ele_href = last_page_ele.attr("href").await;
    
        println!("last page url: {:?}", last_page_ele_href);
        println!(
            "last page number: {:?}",
            get_page_num(last_page_ele_href.unwrap().unwrap().as_str())
        );
        println!(
            " current page number: {:?}",
            get_page_num(url.as_str())
        );
    
    
        // TODO 是不是直接用source 获取html文件后解析比较快 不需要等待元素加载？
    
        let el_img = c
            .wait()
            .for_element(Locator::XPath("//center//td//span/img"))
            .await?;
    
        // let this_document = Url::parse()?;
        // let css_url = this_document.join("../main.css")?;
    
        let scheme = url.scheme();
        let host = url.host();
    
        println!("Scheme: {}", scheme);
        println!("Host: {:?}", host);
    
        let img_src = el_img.attr("src").await?;
        println!("{:#?}", img_src);
        println!(
            "full img url: {:?}",
            url.join(img_src.unwrap().as_str())?.as_str()
        );
    
        // xpath 选择表达式 ;
        let selector = "//center//a[last()]"; //  双斜杠，表示不管层级关系
                                              // xpath 有大量内置函数 可以协助提取信息如 text()
        let button = c.wait().for_element(Locator::XPath(selector)).await?;
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
