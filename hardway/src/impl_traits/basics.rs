pub fn main() {
    let p = Person { 
        name: "John".to_string(),
        age: 30 ,
     };

     println!("persong: {}",p.to_string());
}

struct Person{
    name: String,
    age: u8,
}

impl ToString for Person {
    fn to_string(&self) -> String {
        format!("name: {}, age: {}", self.name, self.age)
    }
}