// https://rust-lang-nursery.github.io/rust-cookbook/database/sqlite.html

pub fn run() {
    open_or_create().unwrap();
    insert_select().unwrap();

    using_tx::run().unwrap() ;
}

fn open_or_create() -> Result<(), Box<dyn std::error::Error>> {
    use rusqlite::NO_PARAMS;
    use rusqlite::{Connection, Result};

    let db_file = "cats.db";
    let conn = Connection::open(db_file)?;

    conn.execute(
        "CREATE TABLE if not exists cat_colors (
        id INTEGER PRIMARY KEY,
        name text NOT NULL unique 
    )",
        NO_PARAMS,
    )?;

    conn.execute(
        "CREATE TABLE if not exists cats (
        id INTEGER PRIMARY KEY,
        name text NOT NULL,
        color_id integer NOT NULL references cat_colors(id)
    )",
        NO_PARAMS,
    )?;

    Ok(())
}

fn insert_select() -> Result<(), Box<dyn std::error::Error>> {
    use rusqlite::NO_PARAMS;
    use rusqlite::{Connection, Result};
    use std::collections::HashMap;

    #[derive(Debug)]
    struct Cat {
        name: String,
        color: String,
    }

    // ====
    let conn = Connection::open("cats.db")?;

    let mut cat_colors = HashMap::new();
    cat_colors.insert(String::from("Blue"), vec!["Tigger", "Sammy"]);
    cat_colors.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);

    for (color, catnames) in &cat_colors {
        conn.execute(
            "INSERT INTO cat_colors (name) values (?1)",
            &[&color.to_string()],
        )?;
        let last_id: String = conn.last_insert_rowid().to_string();

        for cat in catnames {
            conn.execute(
                "INSERT INTO cats (name, color_id) values (?1, ?2)",
                &[&cat.to_string(), &last_id],
            )?;
        }
    }
    let mut stmt = conn.prepare(
        "SELECT c.name, cc.name from cats c
         INNER JOIN cat_colors cc
         ON cc.id = c.color_id;",
    )?;

    let cats = stmt.query_map(NO_PARAMS, |row| {
        Ok(Cat {
            name: row.get(0)?,
            color: row.get(1)?,
        })
    })?;

    for cat in cats {
        println!("Found cat {:?}", cat);
    }

    Ok(())
}

mod using_tx {
    use rusqlite::{Connection, Result, NO_PARAMS};

   pub  fn run() -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = Connection::open("cats.db")?;

        successful_tx(&mut conn)?;

        let res = rolled_back_tx(&mut conn);
        assert!(res.is_err());

        Ok(())
    }

    fn successful_tx(conn: &mut Connection) -> Result<()> {
        let tx = conn.transaction()?;

        tx.execute("delete from cat_colors", NO_PARAMS)?;
        tx.execute("insert into cat_colors (name) values (?1)", &[&"lavender"])?;
        tx.execute("insert into cat_colors (name) values (?1)", &[&"blue"])?;

        tx.commit()
    }

    fn rolled_back_tx(conn: &mut Connection) -> Result<()> {
        let tx = conn.transaction()?;

        tx.execute("delete from cat_colors", NO_PARAMS)?;
        tx.execute("insert into cat_colors (name) values (?1)", &[&"lavender"])?;
        tx.execute("insert into cat_colors (name) values (?1)", &[&"blue"])?;
        tx.execute("insert into cat_colors (name) values (?1)", &[&"lavender"])?;

        tx.commit()
    }
}
