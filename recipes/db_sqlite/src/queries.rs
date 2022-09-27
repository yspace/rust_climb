use sea_query::{*, tests_cfg::*};

use chrono::{NaiveDate, NaiveDateTime};
use rusqlite::{Connection, Result, Row};
use sea_query::{ColumnDef, Expr, Func, Iden, Order, Query, SqliteQueryBuilder, Table};

// use sea_query_rusqlite::RusqliteBinder;
use serde_json::{json, Value as Json};
use time::{
    format_description,
    macros::{date, time},
    PrimitiveDateTime,
};
use uuid::Uuid;
  

pub fn run() {
    let sql = [
        Table::drop()
            .table(Character::Table)
            .if_exists()
            .build(SqliteQueryBuilder),
        Table::create()
            .table(Character::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Character::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(Character::Uuid).uuid())
            .col(ColumnDef::new(Character::FontSize).integer())
            .col(ColumnDef::new(Character::Character).string())
            .col(ColumnDef::new(Character::Meta).json())
            .col(ColumnDef::new(Character::Created).date_time())
            .build(SqliteQueryBuilder),
    ]
    .join("; ");

    println!("{:?}", sql) ;
}


// ................
#[derive(Iden)]
enum Character {
    Table,
    Id,
    Uuid,
    Character,
    FontSize,
    Meta,
    Created,
}

#[derive(Debug)]
#[allow(dead_code)]
struct CharacterStructChrono {
    id: i32,
    uuid: Uuid,
    character: String,
    font_size: i32,
    meta: Json,
    created: Option<NaiveDateTime>,
}

#[derive(Debug)]
#[allow(dead_code)]
struct CharacterStructTime {
    id: i32,
    uuid: Uuid,
    character: String,
    font_size: i32,
    meta: Json,
    created: Option<PrimitiveDateTime>,
}
 