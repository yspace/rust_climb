// https://www.koderhq.com/tutorial/rust/hashmap/

use std::collections::HashMap ;

pub fn main() {

    // create | instantiate HashMap
    let mut state_codes: HashMap<String, String> = HashMap::new();

    // insert values
    state_codes.insert(String::from("NV"), "Nevada".to_string());
    state_codes.insert("NY".to_string(), "New York".to_string());

    // access HashMap values
    println!("{}", state_codes.get("NV").unwrap());
    println!("{}", state_codes.get("NY").unwrap());

    // existance check
    if state_codes.contains_key("FL") {
        println!("FL is in the state_codes HashMap");
        println!("{}", state_codes.get("FL").unwrap());

    }else {
        println!("FL is not in the state_codes HashMap");
        state_codes.insert("FL".to_string(),"Florida".to_string());
    }

    // access HashMap values in a loop 
    for (key, value) in &state_codes {
        println!("{} : {}", key, value);
    }
    // The iter() method will return an iterator that contains the key:value reference of the current iteration.
    for (k, v) in state_codes.iter() {
        println!("{} : {}", k, v);
    }

    // remove values
    state_codes.remove("NV");
    if !state_codes.contains_key("NV") {
        println!("NV is not in the state_codes HashMap");
    }
}

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);

    main();
}