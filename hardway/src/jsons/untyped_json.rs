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