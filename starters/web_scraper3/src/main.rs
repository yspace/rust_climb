
use fantoccini::{ClientBuilder, Locator};
use std::time::Duration;
use tokio::time::sleep;

use visdom::Vis;
use visdom::types::BoxDynError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to webdriver instance that is listening on port 4444
    let client = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await?;


        let url = format!(
            "https://theme.npm.edu.tw/opendata/DigitImageSets.aspx?Key=^^11&pageNo={}",
            1
        );
    

    // Go to the website.
    client.goto(&url).await?;

    // 等候某元素被加载.
    let ele_pager_block = client
        .wait()
        .for_element(Locator::Css(
            r#"div.page"#,
        ))
        .await?;
     
     let el_items = client
     .wait()
     .for_element(Locator::Css("ul.painting-list"))
     .await?;
     println!("分页加载了！");

     let el_items_html = el_items.html(false).await? ;

    //  println!("html: {}", el_items_html);

     // 在rust端处理元素 析取数据
     // 文档： https://github.com/fefit/visdom/wiki/%E4%B8%AD%E6%96%87API%E6%96%87%E6%A1%A3
     // load html
    let root = Vis::load(el_items_html).unwrap();
    let lis = root.find("a");

    for li in lis{
        /*
        let lis_text = li.html();
        println!("{}", lis_text); 
        */
        let href = li.get_attribute("href").unwrap() ;
        let href_url = href.to_string() ;
        println!("{}", href_url);
    }
   
 
    let el_next_page  = client
    .wait()
    .for_element(Locator::Css(
        r#"a.next-page"#,
    ))
    .await?;
    println!("next page url{}", el_next_page.attr("href").await?.unwrap());


    println!("要睡觉了！");
    // Let the user marvel at what we achieved.
    // sleep(Duration::from_millis(6000)).await;
    // Then close the browser window.
    client.close().await?;

    Ok(())
}