use thirtyfour::{error::WebDriverError, prelude::*};
use tokio;

#[tokio::main]
async fn main() -> Result<(), WebDriverError> {
    // return examples::foo1().await;
    // return examples::scrape_wikipedia().await ;
    return examples::scrape2().await ;
}

mod examples {
    use super::*;

    pub async fn foo1() -> Result<(), WebDriverError> {
        let url = "https://spa1.scrape.center/";
        let caps = DesiredCapabilities::chrome();

        let driver = WebDriver::new("http://localhost:9515", caps).await?;
        driver.goto(url).await?;
        // 等待我们要的元素
        let check = driver.query(By::Css(".m-b-sm")).first().await?;
        check.wait_until().displayed().await?;
        let els = driver.find_elements(By::Css(".m-b-sm")).await?;
        for el in els {
            println!("el:{}", el.inner_html().await?.as_str());
        }
        driver.quit().await?;
        Ok(())
    }

    pub async fn scrape_wikipedia() -> WebDriverResult<()> {
        let caps = DesiredCapabilities::chrome();
        let driver = WebDriver::new("http://localhost:9515", caps).await?;

        // Navigate to https://wikipedia.org.
        driver.goto("https://wikipedia.org").await?;
        let elem_form = driver.find(By::Id("search-form")).await?;

        // Find element from element.
        let elem_text = elem_form.find(By::Id("searchInput")).await?;

        // Type in the search terms.
        elem_text.send_keys("selenium").await?;

        // Click the search button.
        let elem_button = elem_form.find(By::Css("button[type='submit']")).await?;
        elem_button.click().await?;

        // Look for header to implicitly wait for the page to load.
        driver.find(By::ClassName("firstHeading")).await?;
        assert_eq!(driver.title().await?, "Selenium - Wikipedia");

        // Always explicitly close the browser.
        driver.quit().await?;

        Ok(())
    }
    pub async fn scrape2() -> WebDriverResult<()> {
        let caps = DesiredCapabilities::chrome();
        let driver = WebDriver::new("http://localhost:9515", caps).await?;


        let url = format!(
            "https://theme.npm.edu.tw/opendata/DigitImageSets.aspx?Key=^^11&pageNo={}",
            82
        );
    
        // let url = "https://wikipedia.org" ;

        // Navigate to https://wikipedia.org.
        driver.goto(url.as_str()).await?;

        //  
        // driver.execute_async(r#"
        //     alert('from rust');
        // "#, Vec::new()).await?;

        let ele_a_links = driver.find_all(By::Css(".list>a")).await?;
       
        // 终结条件是本页下面没内容了 就是这个links数量是0
        if ele_a_links.len() != 0 {
            for el in ele_a_links {
                let href = el.attr("href").await? ;
                 if let Some(h) = href{
                     println!("href: {}", h) ;
                 }
             }
        }else {
            println!("done!") ;
        }
        

        // let elem_form = driver.find(By::Id("search-form")).await?;

        // // Find element from element.
        // let elem_text = elem_form.find(By::Id("searchInput")).await?;

        // // Type in the search terms.
        // elem_text.send_keys("selenium").await?;

        // // Click the search button.
        // let elem_button = elem_form.find(By::Css("button[type='submit']")).await?;
        // elem_button.click().await?;

        // // Look for header to implicitly wait for the page to load.
        // driver.find(By::ClassName("firstHeading")).await?;
        // assert_eq!(driver.title().await?, "Selenium - Wikipedia");

        // Always explicitly close the browser.
        driver.quit().await?;

        Ok(())
    }
}
