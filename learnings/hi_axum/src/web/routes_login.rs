
// use anyhow::Ok;
use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::{Error,Result,};

pub fn routes() -> Router{
     Router::new()
     .route("/api/login", post(api_login))
}

#[derive(Debug,Deserialize)]
struct LoginPayload{
   username: String,
   pwd: String,
}

async fn api_login(payload: Json<LoginPayload>)->Result<Json<Value>> {
    println!("-->> {:<12} - API_LOGIN","HANDLER");

    // TODO: 真正的登陆逻辑
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail)
    }

    // TODO: 创建cookie

    // 返回成功
    let body = Json(json!({
        "result":{
            "success": true
        }
    }));

    Ok(body)

       
       
}