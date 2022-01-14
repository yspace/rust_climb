use std::collections::HashMap;

mod days ;

type ModuleMain = Box<dyn Fn()->()> ;

fn main() {
    
    println!("Hello, world!");

    let mut module_mains:HashMap<String, ModuleMain> = HashMap::new() ;

    module_mains.insert(
        "day2".to_string(),
        Box::new(days::day2::main));
    module_mains.insert(
        "day3".to_string(),
        Box::new(days::day3::main));
    // days::day2::main() ; 

    for (key, value) in &module_mains {
        println!("call:{:?} value :{:?}", key, value());
    }
}

