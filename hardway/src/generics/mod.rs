mod impls;
pub mod closure_in_struct;

pub fn main() {
    impls::main();

    let p1 = Point { x: 1, y: 2 };
    println!("{:?}", p1);

    let p2 = Point { x: 5.3, y: 2.1 };
    println!("{:?}", p2);

    let p3 = Point2 { x: 12, y: 2.1 };
    println!("{:?}", p3);
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

fn add<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

// a generic function, whose type parameter T is constrained
fn generic_display<T: std::fmt::Display>(item: T) {
    println!("{}", item);
}

#[test]
fn test_generic_display() {
    let a: &str = "42";
    let b: i64 = 42;
    generic_display(a);
    generic_display(b);
}

mod demo1{
    use std::collections::HashMap;

    #[derive(Debug)]
    struct Person{
        mobile_number: String,
        name: String,
        email: String,
    }
    pub fn run(){
        let persons = vec![
            Person{
                mobile_number:"18713657897".to_string(),
                name:"John Smith".to_string(),
                email:"some-name@qq.com".to_string(),
            },
            Person{
                mobile_number:"18713657896".to_string(),
                name:"John Smith".to_string(),
                email:"some-name2@qq.com".to_string(),
            },
            Person{
                mobile_number:"18713657895".to_string(),
                name:"John Smith2".to_string(),
                email:"some-name2@qq.com".to_string(),
            },
        ];

        let unique_persons: HashMap<String, Person> = persons
        .into_iter()
        .map(|p|(p.mobile_number.clone(),p))
        .collect() ;
            
            println!("{:?}", unique_persons);
    }

    #[test]
    fn test_run() {
        run();
    }
}