use bincode::serialize as to_bincode; // <1>
use serde_cbor::to_vec as to_cbor; // <1>
use serde_derive::Serialize;
use serde_json::to_string as to_json; // <1>

#[derive(Serialize)]
struct City {
    name: String,
    population: usize,
    latitude: f64,
    longitude: f64,
}
pub fn main() {
    println!("in module {}", module_path!());

    let calabar = City {
        name: String::from("Calabar"),
        population: 470_000,
        latitude: 4.95,
        longitude: 8.33,
    };

    let as_json = to_json(&calabar).unwrap();
    let as_cbor = to_cbor(&calabar).unwrap();
    let as_bincode = to_bincode(&calabar).unwrap();

    println!("json: \n{}\n", &as_json);
    println!("cbor:\n{:?}\n", &as_cbor);
    println!("bincode:\n{:?}\n", &as_bincode);
    println!(
        "json (as UTF-8):\n{}\n",
        String::from_utf8_lossy(as_json.as_bytes())
    );
    println!(
        "cbor (as UTF-8):\n{:?}\n",
        String::from_utf8_lossy(&as_cbor)
    );
    println!(
        "bincode (as UTF-8):\n{:?}\n",
        String::from_utf8_lossy(&as_bincode)
    );
}
