
// use anyhow::Ok;
use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookies, Cookie};

use crate::{Error,Result, web,};

pub fn routes() -> Router{
     Router::new()
     .route("/api/login", post(api_login))
}

#[derive(Debug,Deserialize)]
struct LoginPayload{
   username: String,
   pwd: String,
}

async fn api_login(cookies:Cookies, payload: Json<LoginPayload>)->Result<Json<Value>> {
    println!("-->> {:<12} - API_LOGIN","HANDLER");

    // TODO: 真正的登陆逻辑
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail)
    }

    // FIXME: 实现真正的auth-token 生成/签名
    cookies.add(Cookie::new(web::AUTH_TOKEN,"user-1.exp.sign"));

    // 返回成功
    let body = Json(json!({
        "result":{
            "success": true
        }
    }));

    Ok(body)

       
       
}