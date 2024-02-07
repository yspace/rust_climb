use std::fs;
use std::fs::{DirEntry, File};
use std::io::{BufRead, BufReader, Error, Write};

fn main1() -> Result<(), Error> {
    let path = "lines.txt";

    let mut output = File::create(path)?;
    write!(output, "Rust\n💖\nFun")?;

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

use serde::de::DeserializeOwned;

fn main2() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("file.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}

// use std::fs::OpenOptions;
// use std::io::prelude::*;

fn main3() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("file.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}

// use std::fs::OpenOptions;
// use std::io::prelude::*;

fn main4() -> std::io::Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open("file.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents = contents.replace("Hello", "World");
    file.set_len(0)?; // 清空文件
    file.write_all(contents.as_bytes())?;
    Ok(())
}

#[test]
fn test_file_ops() {
    use std::fs::{File, Metadata};

    fn create() -> std::io::Result<()> {
        let file: File = File::create("foo.txt")?;
        let metadata: Metadata = file.metadata()?;
        println!("{:?}", metadata);

        Ok(())
    }

    fn read_dir() -> std::io::Result<()> {
        let dir: fs::ReadDir = fs::read_dir("/")?;
        for x in dir {
            // Result<DirEntry>
            let entry: DirEntry = x?;
            println!("{:?}, {:?}", entry.file_name(), entry.path());
        }

        Ok(())
    }

    fn write() -> std::io::Result<()> {
        fs::write("foo.txt", "abc".as_bytes())?;

        Ok(())
    }

    fn options() -> std::io::Result<()> {
        let mut file = OpenOptions::new().append(true).open("foo.txt")?;

        file.write("xyz".as_bytes())?;

        Ok(())
    }

    fn read_to_string() -> std::io::Result<()> {
        let string: String = fs::read_to_string("foo.txt")?;
        println!("{}", string);

        Ok(())
    }

    fn copy() -> std::io::Result<()> {
        fs::copy("foo.txt", "foo2.txt")?;

        Ok(())
    }

    fn rename() -> std::io::Result<()> {
        fs::rename("foo2.txt", "foo3.txt")?;

        Ok(())
    }

    fn remove_file() -> std::io::Result<()> {
        fs::remove_file("foo.txt")?;

        Ok(())
    }

    fn create_dir() -> std::io::Result<()> {
        fs::create_dir("test_dir")?;

        Ok(())
    }

    fn create_dir_all() -> std::io::Result<()> {
        fs::create_dir_all("test_dir2/sub_dir")?;
        fs::create_dir_all("test_dir2/sub_dir2")?;

        // remove| remove_all
        fs::remove_dir("test_dir")?;
        fs::remove_dir_all("test_dir2")?;

        Ok(())
    }

    fn main() -> std::io::Result<()> {
        //  不存在则创建
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open("foo.txt")?;
        file.write_all("1234".as_bytes())?;

        Ok(())
    }
}

static MY_STATE_FILE: &'static str = "_runtime/state.json";

pub fn write_struct_to_file<T>(t: T) -> std::io::Result<() /* , std::io::Error */>
where T:Sized + Serialize, {
    
    fs::write( MY_STATE_FILE, serde_json::to_string_pretty(&t).unwrap())?;

    Ok(())
}

#[test]
fn test_write_struct_to_file(){
    let p = Person{
        name: "John".to_string(),
        age: 28,
        phones: vec!["John".to_string()],
    };
   let _rslt = write_struct_to_file(p) ;
}

pub fn read_struc0() {
    let file = File::open(MY_STATE_FILE);

    if file.is_err() {
        println!("open error: {:?}", file);
    }
}

// use serde::de::{self /* , Expected, Unexpected*/};
// pub fn read_struct<'_, T>() -> serde_json::Result<T>
// where
//     T: de::Deserialize<'_>,
// {
//     let data = fs::read_to_string(MY_STATE_FILE).expect("LogRocket: error reading file");
//             serde_json::from_str(&data) 
// }

// @see https://stackoverflow.com/questions/73889074/how-to-implement-a-generic-serde-jsonfrom-str
// @see reqwest::blocking::get  .json::<XXX>();
// @see https://serde.rs/lifetimes.html trait 带生命周期 太复杂 有空再说！
pub fn read_struct_from_file <T> ( ) -> Result<T, Box<dyn std::error::Error>>
where T: DeserializeOwned  /* + Default */
{
    // Open the file in read-only mode with buffer.
    let file = File::open(MY_STATE_FILE)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}

 // ## typed struct data
 use serde::{Deserialize, Serialize};
 // use serde_json::Result;

 #[derive(Serialize, Deserialize,Debug,Default)]
 struct Person {
     name: String,
     age: u8,
     phones: Vec<String>,
 }

#[test]
fn test_read_struct_from_json() {
   

    let p : Result<Person,Box<dyn std::error::Error>> = read_struct_from_file();
    println!("{:?}", p);
}

#[test]
fn test_metadata() {
    use std::fs::{File, Metadata};

    fn run() -> std::io::Result<()> {
        let file: File = File::create(MY_STATE_FILE)?; // open?
        let metadata: Metadata = file.metadata()?;
        println!("metadata {:?}", metadata);

        Ok(())
    }

    let result = run();
}

#[test]
fn test_json() {
    use serde_json::{Result, Value};

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(MY_STATE_FILE);

    if file.is_err() {
        let p = Path::new(MY_STATE_FILE);
        let dir = p.parent().unwrap();
        let result = fs::create_dir_all(dir);

        if result.is_err() {
            println!("create dir failed: {:?}", result);
        } else {
            println!("create dir ok: {:?}", dir);
        }

        return;
    }

    let mut file = file.unwrap();
    let meta = file.metadata();
    let len = meta.unwrap().len();

    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let mut v: Value;
    if len == 0 {
        // 初创建文件 那么写入一个默认对象
        file.write_all(&data.as_bytes());

        v = serde_json::from_str(data).unwrap();
    } else {
        let d = fs::read_to_string(MY_STATE_FILE).unwrap();

        v = serde_json::from_str(d.as_str()).unwrap();
    }

    // Parse the string of data into serde_json::Value.
    // let v: Value = serde_json::from_str(data).unwrap();

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    // ## typed struct data
    use serde::{Deserialize, Serialize};
    // use serde_json::Result;

    #[derive(Serialize, Deserialize,Debug,Default)]
    struct Person {
        name: String,
        age: u8,
        phones: Vec<String>,
    }

    let p: Person = serde_json::from_str(data).unwrap();
    println!("person: {:?}", p);
    println!("{:?}", serde_json::to_string_pretty(&p).expect("my-error-category: error parsing to JSON"));


    // ## 3 
    fn _run() -> std::io::Result<() /* , std::io::Error */> {
        let mut p: Person = {
            let data = fs::read_to_string(MY_STATE_FILE).expect("LogRocket: error reading file");
            serde_json::from_str(&data).unwrap()
        };
        p.age  += 10;
        fs::write( MY_STATE_FILE, serde_json::to_string_pretty(&p).unwrap())?;
    
        Ok(())
    }
    let _rslt = _run();
}



fn file_error() -> std::io::Result<()> {
    let mut file = match File::open("info.txt") {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    println!("File not found");
                    return Ok(());
                }
                _ => return Err(error),
            }
        }
    };
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    println!("File contents: {:?}", contents);

    Ok(())
}
#[test]
fn test_file_error(){
    let _rslt = file_error() ;

}