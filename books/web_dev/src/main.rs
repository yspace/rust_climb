mod listings ;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  
   basic_async_request().await?;
   Ok(())
}

async fn basic_async_request()-> Result<(), Box<dyn std::error::Error>> {
   let resp = reqwest::get("https://httpbin.org/ip")
   .await?;

   println!("{:#?}", resp) ;

   Ok(())
}