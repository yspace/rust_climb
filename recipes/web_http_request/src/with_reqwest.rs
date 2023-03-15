
use super::* ;
async fn main() -> Result<()> {
    let client = reqwest::Client::new();
    let res = client.post("http:/ /httpbin.org/post")
    .body("the exact body is sent") .send()
    .await?
    .text()
    .await?;
    println!("{:?}", res);
    Ok(())
 }