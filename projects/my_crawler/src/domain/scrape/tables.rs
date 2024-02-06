// use chrono::DateTime;
use log::LevelFilter;
use rbatis::dark_std::defer;
use rbatis::intercept_log::LogInterceptor;
use rbatis::RBatis;
use crate::domain::table::LoginCheck;
use rbatis::rbdc::DateTime;
use rbatis::table_sync::{ColumMapper, MssqlTableMapper, MysqlTableMapper, PGTableMapper, SqliteTableMapper};

// use rbatis::executor::{Executor, RBatisConnExecutor, RBatisTxExecutor};
use serde::Serialize;

///Permission Resource Table
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ScrapeJob {
    // pub id: Option<String>,
    pub id: u64,
    pub page_url:  String ,
    //permission
    pub ext_data: Option<String>,
    pub content_hash: Option<String>,
    pub create_date: Option<DateTime>,
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ProcessItem {
    pub id: u64,
    pub job_id: u64,
    //father id(can empty)
    // pub parent_id: Option<String>,
    pub title: String,
    pub ext_data: Option<String>,
    pub download_url: String,
    pub status: u8,
    pub create_date: Option<DateTime>,
    pub update_date: Option<DateTime>,
}

pub async fn sync(
    rb: &RBatis,
    column_mapper: &dyn ColumMapper,

)  {
    let conn = rb.acquire().await.expect("connection database fail");

    let table = ScrapeJob {
        // id: Some("".to_string()),
        // parent_id: Some("".to_string()),
        // name: Some("".to_string()),
        // permission: Some("".to_string()),
        // path: Some("".to_string()),
        // create_date: Some(DateTime::now()),
        id:0u64,
        page_url:"".to_string(),
        ext_data: Some("".to_string()),
        content_hash: Some("".to_string()),
        create_date: Some(DateTime::now()),
    };
    let _ = RBatis::sync(&conn, column_mapper, &table, "scrape_job").await;
let table = ProcessItem {

        id:0u64,
        job_id: 0u64,
        title:  "".to_string(),
        ext_data: Some("".to_string()),
        status: 0u8,
        download_url: "".to_string(),
        create_date: Some(DateTime::now()),
        update_date: Some(DateTime::now()),
    };
    let _ = RBatis::sync(&conn, column_mapper, &table, "process_item").await;

}
