trait Greetable{
    fn say(&self) -> String ;
}

struct Person {
    // DIP 依赖反转的核心思路就是依赖抽象不要依赖具体 在clean-arch中 具体的术语就是细节details
    greet_service: Box<dyn Greetable>,
}

impl Person {
    fn new(greet_service: Box<dyn Greetable>)->Self{
        Self{
            // greet_service: greet_service ,
            greet_service ,
        }
    }

    fn greet(&self){
        println!("{}",self.greet_service.say());
    }
}

mod details{
    use super::*;

   pub  struct Greet;

    impl Greet {
       pub  fn new() -> Self {
            Self{}
        }
    }

    impl Greetable for Greet {
        fn say(&self) -> String  {
            // to_string 内部也是调用的to_owned!
        "Hello".to_string()
    }
    }
}

mod details_v2{
    use super::*;

   pub  struct Greet;

    impl Greet {
       pub  fn new() -> Self {
            Self{}
        }
    }

    impl Greetable for Greet {
        fn say(&self) -> String  {
            // to_string 内部也是调用的to_owned!
        "v2: Hello".to_string()
    }
    }
}

#[test]
fn test_greet(){
  let greet = details::Greet::new();
  let person = Person::new(Box::new(greet));

  person.greet() ;

  // details v2
  let greet = details_v2::Greet::new();
  let person = Person::new(Box::new(greet));

  person.greet() ;
}