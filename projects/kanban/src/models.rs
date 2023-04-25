use crate::schema::*;

// #[derive(serde::Serialize)]
#[derive(serde::Serialize, diesel::Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Board {
    pub id: i64, // PostgreSQL only supports signed integer types.
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub id: i64,
    pub board_id: i64,
    pub description: String,
    pub status: Status,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// #[derive(serde::Serialize, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
#[derive(Debug,serde::Serialize, serde::Deserialize, diesel_derive_enum::DbEnum)]
#[DieselType = "Status_enum"]
pub enum Status {
    Todo,
    Doing,
    Done,
}


/*  =======
    根据查询设计所需结构
    ======= */
#[derive(serde::Serialize)]
pub struct BoardSummary {
    pub todo: i64,
    pub doing: i64,
    pub done: i64,
}

/* =======
    api 创建时需要的模型
====== */
#[derive(serde::Deserialize)]
pub struct CreateBoard {
    pub name: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCard {
    pub board_id: i64,
    pub description: String,
}

#[derive(serde::Deserialize)]
pub struct UpdateCard {
    pub description: String,
    pub status: Status,
}