use serde_json::json ;

pub fn main() {
    // 
// json! takes a JSON literal and some Rust expressions to implement String values. 
// It converts these into a Rust value of type serde_json::Value, an enum that can represent every type within the JSON specification.
    let capitals = json!({
        "Cook Islands": "Avarua",
              "Fiji": "Suva",
              "Kiribati": "South Tarawa",
              "Niue": "Alofi",
     "Tonga": "Nuku'alofa",
      "Tuvalu": "Funafuti"
    });

    // Index notation is supported by all types that implement the Index trait. Accessing capitals["Tonga"] is syntactic sugar for capitals.index("Tonga").
    println!("Capital of Tonga is: {}", capitals["Tonga"]);
    
}