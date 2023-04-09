use std::any::Any;

#[test]
fn main() {
     let mut fs : Vec<Box<dyn Fn() -> Box<dyn Any>>> = Vec::new(); // fn() -> bool fs.push(Box::new(|| true));

    // fn() -> i32
    fs.push(Box::new(|| Box::new(10_i32))); 
    
    // fn() -> String
    let s = " World".to_string();
    fs.push(Box::new(move || Box::new("Hello ".to_owned() + &s)));
    
    for closure in fs.iter() {
        let value = closure();
        if let Some(string) = value.downcast_ref::<String>() {
            println!("String: {}", string);
        }
        else if let Some(number) = value.downcast_ref::<i32>() {
            println!("i32: {}", number);
        } else {
            println!("Unknown value");
        }
    }

}