
struct Greet{

}

impl Greet{
    fn say(&self)-> String{
        // "hello".to_string()
        "hello".to_owned()
    }
}

struct Person{}

impl Person {
    fn greet(&self){
        let greet = Greet{};
        println!("{}", &greet.say());
    }    
}