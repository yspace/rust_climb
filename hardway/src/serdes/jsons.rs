use serde::{Deserialize, Serialize};

// @see https://blog.logrocket.com/json-and-rust-why-serde_json-is-the-top-choice/

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "shape")]
enum Shape {
    Circle {
        radius: f64,
    },
    Rectangle {
        length: f64,
        width: f64,
    },
}

#[derive(Debug, Deserialize, Serialize)]
struct Request {
    // calculation: Calculation,
    #[serde(flatten)] 
    shape: Shape,
}