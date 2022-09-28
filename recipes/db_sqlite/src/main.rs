use rusqlite::{params,Connection, Result} ;


mod from_cookbook;
mod orms ;
mod queries ;
mod datetimes ;

#[derive(Debug)]
struct Person {
    id: i32 , 
    name: String,
    data: Option<Vec<u8>>,
}


fn create_db() -> Result<Connection> {
    let db_file = "data.db" ;
    let conn = Connection::open(db_file)?;
    let _ = conn.execute("DROP TABLE person" , []);
    
    conn.execute("CREATE TABLE person(
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
    )
        ", [])?;

        Ok(conn)
}

fn insert_data(conn: &Connection) -> Result<()>{
    let p1 = Person{
        id: 1, 
        name: "John".to_string(),
        data: None,
    };
    let p2 = Person{
        id: 2, 
        name: "Nick".to_string(),
        data: None,
    };

    conn.execute(
        "INSERT INTO person(id, name, data) 
        VALUES(?1,?2,?3),(?4,?5,?6);",
        params![p1.id, p1.name, p1.data,p2.id, p2.name, p2.data]
    )?;

    Ok(())
}

fn get_data(conn: &Connection) -> Result<Vec<Person>> {
    let mut stmt
     = conn.prepare("SELECT id, name, data from person")?;
    let person_iter = stmt.query_map([],|row|{
        Ok(Person{
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    let mut persons = Vec::new();
    for p in person_iter{
        persons.push(p?) ;
    }

    Ok(persons)
}

fn main() -> Result<()>{
    println!("datetimes");
    datetimes::run() ; return Ok(()) ;
    // ===

    println!("Hello, world!");
    queries::run(); return Ok(()) ;

    //=== 
    
    async_examples::run() ; return Ok(()) ;
    // ==================
    from_cookbook::run();
    return Ok(()) ; 
    // =======================

    let conn = create_db()? ;
    insert_data(&conn)?;
    let persons = get_data(&conn)?;

    for p in persons{
        println!("hi: {:?}", p) ;
    }

      Ok(())
}

mod async_examples {

    pub fn run() {
        // 下面这段代码就是async_std::main 的内容
        async_std::task::block_on(async {
            main1().await.unwrap() ;
        })
    }

    async fn main1( ) -> Result<(), Box<dyn std::error::Error>>{

        println!("hi this is a async function") ;
       Ok(() )
    }

   
}