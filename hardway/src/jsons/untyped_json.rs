use serde_json::{Number, json};


#[test]
pub fn run(){
    let json_data = r#"
    {"test": 200}
    "#;

    let mut p: serde_json::Value = serde_json::from_str(json_data).unwrap();
    println!("serde data before: {:?}", p);

// 用类型来构造显然很麻烦 不及json!宏的方式 这些类都是在rust原生类型上包了一层 所以构造起来比较麻烦！
    p["test"] = serde_json::Value::Number(Number::from(300)) ;
    p["test"] = json!{500} ;
    println!("serde data after: {:?}", p);

    // 直接添加新key
    p["hi"] = json!({"yes":"nonono","rust":"wonderful!"}) ;
    println!("serde data after: {:?}", p);

}

use serde_json::{Result, Value};

#[test]
pub fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}

use std::fs;

#[test]
fn from_file(){
    println!("from file!");

    // Grab JSON file
    let file_path = "data/test.json".to_owned();
    let contents = fs::read_to_string(file_path.clone());
    //.expect("Couldn't find or load that file.");
    if contents.is_err() {
        println!("file not found: {}", file_path);
    }else{
        let v: Value = serde_json::from_str(contents.unwrap().as_str()).unwrap();
    }

    // untyped_example(&contents);
}