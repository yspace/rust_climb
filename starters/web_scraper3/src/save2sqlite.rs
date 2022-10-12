#![allow(dead_code)]

use rusqlite::{params, Result , NO_PARAMS};
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
    // let conn = Connection::open("my_data.db").await?;
    let conn = Connection::open("my_data2.db").await?;

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
                CREATE TABLE if not exists "work_position" (
                    "id"	INTEGER NOT NULL,
                    "page_url"	TEXT NOT NULL,
                    "item_index"	INTEGER NOT NULL,
                    "item_xpath"	TEXT,
                    PRIMARY KEY("id" AUTOINCREMENT)
                );
                "#,
                [],
            )?;
            conn.execute(
                r#"
                
                    CREATE TABLE if not exists "projects" (
                        "id"	INTEGER NOT NULL,
                        "title"	TEXT NOT NULL,
                        "detail"	TEXT NOT NULL,
                        "download_url" TEXT NOT NULL,
                        "status"	INTEGER NOT NULL DEFAULT 0,
                        PRIMARY KEY("id")
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

pub async  fn get_latest_work_position(conn: Connection)-> Result<WorkPosition>{
    let rslt = conn
    .call(|conn| {
         
 
        // sql: select  * from work_position order by id DESC LIMIT 1
        let mut stmt = conn.prepare("SELECT page_url, item_index,item_xpath  from work_position order by id DESC LIMIT 1")?;
        let item = stmt
            .query_row(params![], |row| {
                Ok(WorkPosition {
                    page_url: row.get(0)?,
                    item_index: row.get(1)?,
                    item_xpath: row.get(2)?,
                })
            })?;
            

        // Ok::<_, rusqlite::Error>(people)
        Ok::<_, rusqlite::Error>(item)

    })
    .await?;

     Ok(rslt)
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


#[derive(Debug)]
pub struct Project {
    pub title: String,
    pub detail: String,
    pub download_url: String,
    pub status: i32,
}
pub fn add_project_task(conn: Connection, item: Project) -> JoinHandle<()> {
    tokio::spawn(async move {
        conn.call(move |conn| {
            conn.execute(
                "INSERT INTO projects (title,detail,download_url) VALUES (?1, ?2,?3)",
                params![item.title, item.detail, item.download_url],
            )
        })
        .await
        .unwrap();
    })
}


pub async  fn get_latest_project(conn: Connection)-> Result<(i32,Project)>{
    let rslt = conn
    .call(|conn| {
         
 
        // sql: select  * from work_position order by id DESC LIMIT 1
        let mut stmt = conn.prepare("SELECT title,detail,download_url, status ,id from projects 
        where status = 0
        order by id ASC LIMIT 1")?;
        let item:(Project, i32) = stmt
            .query_row(params![], |row| {
                Ok((Project {
                    title: row.get(0)?,
                    detail: row.get(1)?,
                    download_url: row.get(2)? ,
                    status: row.get(3)?,
                },row.get(4)?))
            })?;
            

        // Ok::<_, rusqlite::Error>(people)
        // Err(QueryReturnedNoRows)
        Ok::<_, rusqlite::Error>(item)

    })
    .await?;

    // ^-^ 颠倒下 比较懒 倒来倒入会晕的！
     Ok((rslt.1, rslt.0))
}

pub async  fn get_project_by_id(conn: Connection,id: i32)-> Result<(i32,Project)>{
    let rslt = conn
    .call(move|conn| {
         
 
        // sql: select  * from work_position order by id DESC LIMIT 1
        let mut stmt = conn.prepare("SELECT title,detail,download_url, status ,id from projects 
        where id = ?
        LIMIT 1")?;
        let item:(Project, i32) = stmt
            .query_row(params![id], |row| {
                Ok((Project {
                    title: row.get(0)?,
                    detail: row.get(1)?,
                    download_url: row.get(2)? ,
                    status: row.get(3)?,
                },row.get(4)?))
            })?;
            

        // Ok::<_, rusqlite::Error>(people)
        // Err(QueryReturnedNoRows)
        Ok::<_, rusqlite::Error>(item)

    })
    .await?;

    // ^-^ 颠倒下 比较懒 倒来倒入会晕的！
     Ok((rslt.1, rslt.0))
}

// reverse order
pub async  fn get_latest_project_rev(conn: Connection)-> Result<(i32,Project)>{
    let rslt = conn
    .call(|conn| {
         
 
        // sql: select  * from work_position order by id DESC LIMIT 1
        let mut stmt = conn.prepare("SELECT title,detail,download_url, status ,id from projects 
        where status = 0
        order by id DESC LIMIT 1")?;
        let item:(Project, i32) = stmt
            .query_row(params![], |row| {
                Ok((Project {
                    title: row.get(0)?,
                    detail: row.get(1)?,
                    download_url: row.get(2)? ,
                    status: row.get(3)?,
                },row.get(4)?))
            })?;
            

        // Ok::<_, rusqlite::Error>(people)
        // Err(QueryReturnedNoRows)
        Ok::<_, rusqlite::Error>(item)

    })
    .await?;

    // ^-^ 颠倒下 比较懒 倒来倒入会晕的！
     Ok((rslt.1, rslt.0))
}

pub async fn  project_mark_as_downloaded(conn: Connection, id : i32, project: Project)
-> Result<()>{
    // 当一个下载任务完成后 在任务表里需要更新这个刚完成的项目的状态
    // 此外针对重复链接 也需要一并标记了
    // 更新条件： id=self.id OR id>self.id and download_url = self.download_url
    // 就是更新 id和传参对象相同的 或者那些id大于参数 并且下载链接相同的记录

    let rslt = conn
    .call(move |conn| {
  
        
        let mut stmt = conn.prepare("UPDATE projects 
        SET status = ? 
        WHERE 
        id = ?
        OR download_url = ?
        ")?;
        stmt.execute(params![1,id, project.download_url])?;

        
        Ok::<_, rusqlite::Error>(())

    })
    .await?;


    Ok(())

}

 pub async fn  project_update_status(conn: Connection, id : i32,  status: i32)
-> Result<()>{
   
    
    let rslt = conn
    .call(move |conn| {
  
        
        let mut stmt = conn.prepare("UPDATE projects 
        SET status = ? 
        WHERE 
        id = ?
      
        ")?;
        stmt.execute(params![status,id])?;

        
        Ok::<_, rusqlite::Error>(())

    })
    .await?;


    Ok(())

}