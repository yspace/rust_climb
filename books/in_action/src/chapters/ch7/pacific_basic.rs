use std::collections::HashMap;

pub fn main() {
    let mut capitals = HashMap::new();

    capitals.insert("Cook Islands", "Avarua");
    capitals.insert("Fiji", "Suva");
    capitals.insert("Kiribati", "South Tarawa");
    capitals.insert("Niue", "Alofi");
    capitals.insert("Tonga", "Nuku'alofa");
    capitals.insert("Tuvalu", "Funafuti");

    let tongan_capital = capitals["Tonga"];

    println!("Capital of Tonga: {}", tongan_capital);

    println!("{:?}", capitals.get("kiribati"));
    println!("{:?}", capitals.get("Kiribati"));
    // Deleting key-value pairs with the .remove() method
    capitals.remove("Tonga");
    println!("{:?}", capitals.get("Tonga"));

    // Iterating over keys, values, and key-value pairs with the .keys(), .values(), and
    // .iter() methods, respectively, as well as their read-write variants, .keys_mut(), .values_mut(), and .iter_mut()
    println!("keys:");
    for k in capitals.keys() {
        println!("{}", k);
    }
    println!("\n<<-- values :");
    for v in capitals.values() {
        println!("{}", v);
    }
    println!(">>--values \n");
    
    for (k, v) in capitals.iter() {
        println!("{} : {}", k, v);
    }
}
