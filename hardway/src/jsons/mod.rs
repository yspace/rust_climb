// @see https://stackoverflow.com/questions/30292752/how-do-i-parse-a-json-file
// @see https://whoisryosuke.com/blog/2022/parsing-json-with-rust/

use serde::{Deserialize, Serialize};
use serde_json::Result;

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Theme {
    colors: HashMap<String, String>,
    space: Vec<i32>,
    font_sizes: Vec<i32>,
    fonts: HashMap<String, String>,
    font_weights: HashMap<String, i32>,
    line_heights: HashMap<String, f32>,
    breakpoints: HashMap<String, String>,
    animation: HashMap<String, String>,
    gradients: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

pub fn run() {
    let the_file = r#"{
        "FirstName": "John",
        "LastName": "Doe",
        "Age": 43,
        "Address": {
            "Street": "Downing Street 10",
            "City": "London",
            "Country": "Great Britain"
        },
        "PhoneNumbers": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#;

    let mut json: serde_json::Value =
        serde_json::from_str(the_file).expect("JSON was not well-formatted");
    if let Some(age) = json.get_mut("Age") {
        *age = 40.into();
    }

    println!("{}", json);

    json_macro();
}

fn json_macro() {
    use serde_json::json;

    let age = 25 ;
    let john = json!({
        "name": "John Doe",
        "age": age,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    println!("first phone number: {}", john["phones"][0]);

    // Convert to a string of JSON and print it out
    println!("{}", john.to_string());
}
#[test]
fn test_json_macro() {
    json_macro();
}

fn pretty_json() {}

mod utils {
    use std::{error::Error, fs::File, io::BufReader, path::Path};

    use serde_json::Value;

    fn read_payload_from_file<P: AsRef<Path>>(path: P) -> Result<Value, Box<dyn Error>> {
        // Open file in RO mode with buffer
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        // Read the JSON contents of the file
        let u = serde_json::from_reader(reader)?;

        Ok(u)
    }

    fn main() {
        let payload: Value = read_payload_from_file("./config/payload.json").unwrap();
    }
}

fn optional_types() {
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    enum Colors {
        Name(String),
        Number(i32),
    }

    #[derive(Serialize, Deserialize)]
    struct Theme {
        test: Colors,
    }

    let json_data = r#"
    {"test": 200}
    "#;

    let p: Theme = serde_json::from_str(json_data).unwrap();
    // println!("{:#?}", p.test);
    match p.test {
        Colors::Name(_) => {()},
        Colors::Number(num) => println!("Theme Color is number: {}", num),
        // We don't want the name so we do nothing by passing empty tuple
        // Name(val)=> {()},
        // Number(num) => println!("Theme Color is number: {}", num),
    }
}
#[test]
fn test_optional_types(){
    optional_types();
}

#[test]
fn test_json() {
    run();
}
