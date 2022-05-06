mod listings ;
mod models ;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("from web_dev::main()");
   // basic_async_request().await?;
   // minimal_warp_server().await?;
   // listings::listing_3_1::run().await;
   // listings::listing_3_4::main().await;
   // listings::listing_3_8::main().await;
   // listings::listing_3_13::main().await;
   listings::listing_3_19::main().await;
   Ok(())
}

async fn basic_async_request()-> Result<(), Box<dyn std::error::Error>> {
   let resp = reqwest::get("https://httpbin.org/ip")
   .await?;

   println!("{:#?}", resp) ;

   Ok(())
}

use warp::Filter ;
async fn minimal_warp_server()-> Result<(), Box<dyn std::error::Error>> {

   let hello = warp::get()
   .map(|| format!("Hello, World!"));
   warp::serve(hello)
   .run(([127, 0, 0, 1], 1337)) .await;

   Ok(())
}