#![allow(dead_code)]

use rusqlite::{params, Result};
use tokio::task::JoinHandle;
use tokio_rusqlite::Connection;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct WorkPosition {
    pub page_url: String,
    pub item_index: i32,
    pub item_xpath: String,
}

pub async fn establish_connection() -> Result<Connection> {
    let conn = Connection::open("my_data.db").await?;

    Ok(conn)
}

async fn main() -> Result<()> {
    let conn = Connection::open_in_memory().await?;

    // Create table.
    conn.call(|conn| {
        conn.execute(
            "CREATE TABLE person (
                id    INTEGER PRIMARY KEY,
                name  TEXT NOT NULL,
                data  BLOB
            )",
            [],
        )
    })
    .await?;

    // // Start tasks.
    // let add_steven = add_steven_task(conn.clone());
    // let add_bob = add_bob_task(conn.clone());

    // // Wait for tasks to finish.
    // add_steven.await.unwrap();
    // add_bob.await.unwrap();

    Ok(())
}

pub async fn init_db(conn: Connection) -> Result<()> {
    let _ = conn
        .call(|conn| {
            conn.execute(
                r#"
                CREATE TABLE "work_position" (
                    "id"	INTEGER NOT NULL,
                    "page_url"	TEXT NOT NULL,
                    "item_index"	INTEGER NOT NULL,
                    "item_xpath"	TEXT,
                    PRIMARY KEY("id" AUTOINCREMENT)
                );
                "#,
                [],
            )?;

            // conn.execute(
            //     "INSERT INTO person (name, data) VALUES (?1, ?2)",
            //     params![steven.name, steven.data],
            // )?;

            // let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
            // let people = stmt
            //     .query_map([], |row| {
            //         Ok(Person {
            //             id: row.get(0)?,
            //             name: row.get(1)?,
            //             data: row.get(2)?,
            //         })
            //     })?
            //     .collect::<Result<Vec<Person>, rusqlite::Error>>()?;

            // Ok::<_, rusqlite::Error>(people)
            Ok::<_, rusqlite::Error>(())
        })
        .await?;

    Ok(())
}

pub fn add_work_position_task(conn: Connection, item: WorkPosition) -> JoinHandle<()> {
    tokio::spawn(async move {
        conn.call(move |conn| {
            conn.execute(
                "INSERT INTO work_position (page_url,item_index,item_xpath) VALUES (?1, ?2,?3)",
                params![item.page_url, item.item_index, item.item_xpath],
            )
        })
        .await
        .unwrap();
    })
}
