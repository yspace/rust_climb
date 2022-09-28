use chrono::naive::NaiveDate; 
use rusqlite::{params, Connection, Result};


pub fn run(){
    main().unwrap();
}

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    date: NaiveDate,
}

fn main()->Result<()>{
    let date_str = "2020-04-12";
    let naive_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();

    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        date: naive_date,
    };
    println!("{:?}",me);
    
    let conn = Connection::open_in_memory()?;
    
        
    conn.execute(
        "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  date            TEXT
                  )",
        [],
    )?;
    
    conn.execute(
    "INSERT INTO person (name, date) VALUES (?1, ?2)",
        params![me.name, me.date],
    )?;
    
    let mut stmt = conn.prepare("SELECT id, name, date FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            date: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    
    Ok(())
}