#![allow(unused)]

use anyhow::Result;
use serde_json::json;

// >cargo test -p hi_axum -- --nocapture
#[tokio::test]
async fn quick_dev()-> Result<()>{
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/hello?name=qing").await?.print().await?;
    hc.do_get("/hello2/qing").await?.print().await?;

    // hc.do_get("/src/main.rs").await?.print().await?;
    hc.do_get("/assets/index.html").await?.print().await?;

    hc.do_post("/api/login", json!({
        "username": "demo1",
        "pwd": "welcome"
    })).await?.print().await?;

    Ok(())
}