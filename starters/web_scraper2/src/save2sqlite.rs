use rusqlite::{params, Result};
use tokio::task::JoinHandle;
use tokio_rusqlite::Connection;

use thirtyfour::{error::WebDriverError, prelude::*};


pub async fn run() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;


    let mut url = format!(
        "https://theme.npm.edu.tw/opendata/DigitImageSets.aspx?Key=^^11&pageNo={}",
        82
    );

    
   
    loop {
       let rslt = page_for(&driver, url).await;
       match rslt {
         Ok(next_page_url) => { url = next_page_url; },
         Err(err) => break,
       }
    }
   
    
    

    // elem_text.send_keys("selenium").await?;

    // // Click the search button.
    // let elem_button = elem_form.find(By::Css("button[type='submit']")).await?;
    // elem_button.click().await?;
 

    // Always explicitly close the browser.
    driver.quit().await?;

    Ok(())
}

async fn page_for(driver: &WebDriver , url:String) -> WebDriverResult<String>{

    driver.goto(url.as_str()).await?;

    let ele_a_links = driver.find_all(By::Css(".list>a")).await?;
   
    // 终结条件是本页下面没内容了 就是这个links数量是0
    if ele_a_links.len() != 0 {
        for el in ele_a_links {
            let href = el.attr("href").await? ;
             if let Some(h) = href{
                 println!("href: {}", h) ;
                 // 存入db 
             }
         }
         // 处理下一页
         let ele_next_page = driver.find(By::Css("a.next-page")).await?;
         // ele_next_page.click().await?;
         let href =  ele_next_page.attr("href").await? ; 
          
         if href.is_some() {
            let url =  href.unwrap() ;
            println!("next url: {}", url);
           return  Ok( url) ;
         }
    }
    

   Err(  WebDriverError::CustomError(String::from("done!")))
}

