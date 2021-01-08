use std::{
    collections::HashMap, 
    fmt::format
};


fn main() {
   
    let mut arguments = std::env::args().skip(1) ;

    // let key = arguments.next().unwrap() ;
    // let value = arguments.next().unwrap() ;
    let key = arguments.next().expect("key was not there") ;
    let value = arguments.next().expect("value was not there") ;
    println!("The key is {} and the value is {}", key, value) ;

    // let contents = format!("{}\t{}\n",key, value ) ;
    // std::fs::write("kv.db", contents).unwrap() ;

    let mut database = Database::new()
    .expect("Database::new crashed ") ;
    database.insert(key.to_uppercase(), value.clone()) ;
    database.insert(key, value) ;
    match database.flush() {
        Ok(_) =>{
            println!("flush ok!") ;
        },
        Err(err) => {
            println!("Oh no!") ;
        },
    }
}

struct Database{
    map: std::collections::HashMap<String, String>,
    flush: bool ,
}

impl Database {

    fn new() ->Result<Self, std::io::Error >{
        // 读文件
    //    let contents = match  std::fs::read_to_string("kv.db")  {
    //         Ok(c) => {
    //             c
    //         }
    //         Err(error) => {
    //             // return Result::Err(error) ;
    //             return Err(error) ;
    //         }
    //     }

        let mut  map = HashMap::new() ;

        let contents = std::fs::read_to_string("kv.db")? ;

        for line in contents.lines() {
            // let pair = line.split_once('\t').expect("Corrupt database ")
            // let (key, value) = line.split_once('\t').expect("Corrupt database ")

            let mut chunks = line.splitn(2,'\t') ;
            let key = chunks.next().expect("No key") ;
            let value = chunks.next().expect("No value") ;
            map.insert(key.to_owned(), value.to_owned()) ;
            // map.insert(key.to_string(), value.to_owned()) ;
        }
        // 解析字符串
        // 填充map
        Ok(Self{
            map: map,
            flush: false ,
        })
    }

    fn insert(&mut self, key :String , value: String) {
        self.map.insert(key, value) ;
    }
     
    // fn flush(self)-> Result<(), std::io::Error> {
    fn flush_v0(self)-> std::io::Result<()> {
        let mut contents = String::new() ;
        // for pairs in self.map{
        //     let kvpair = format!("{}\t{}\n",pairs.0 , pairs.1) ;
        //     // contents = contents + &kvpair ;
        //     contents.push_str(&kvpair);
        // }
        // for (key, value) in self.map{
        //     let kvpair = format!("{}\t{}\n",key, value) ;
        //     // contents = contents + &kvpair ;
        //     contents.push_str(&kvpair);
        // }
        for (key, value) in &self.map{
            let kvpair = format!("{}\t{}\n",key, value) ;
            // contents = contents + &kvpair ;
            // contents.push_str(&kvpair);
            contents.push_str(&key) ;
            contents.push('\t') ;
            contents.push_str(&value) ;
            contents.push('\n') ;

        }

        std::fs::write("kv.db", contents)
        // todo!("Finish this method")
    }

    fn flush(mut self) -> std::io::Result<()>{
        self.flush = true ;
        do_flush(&self) 
    }
}

impl Drop for Database{
    fn drop(&mut self) {
        if !self.flush {
            let _ = do_flush(self) ;
        }
      
    }
}

fn do_flush(database: &Database) -> std::io::Result<()>{
    println!("do flush called!") ;

    let mut contents = String::new() ;
    for (key, value) in &database.map{
        contents.push_str(&key) ;
        contents.push('\t') ;
        contents.push_str(&value) ;
        contents.push('\n') ;
    }

    std::fs::write("kv.db", contents)
}