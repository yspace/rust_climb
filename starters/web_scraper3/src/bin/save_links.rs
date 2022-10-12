 
use web_scraper3;
   
/**
 * 先运行webdriver 
  > cd starters/web_scraper2/bins 
  ❯ ./chromedriver --port=4444   
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
 // 构造config对象 里面应该有数据库位置 被抓网站的baseurl 等 然后run(config...) 

    web_scraper3::main().await? ;

 Ok(())

}