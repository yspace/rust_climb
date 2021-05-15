#![feature(proc_macro_hygiene, decl_macro)]

use std::sync::Mutex;

#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

lazy_static!{
    // static ref VALUE:u8 = 0 ;
     static ref VALUE: Mutex<u8> = Mutex::new(0) ;
}


#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/value")]
fn get_value() -> String{
    // unsafe{
    //     format!("GET OK :{}", counter)
    // }
    format!("GET OK :{}", VALUE.lock().unwrap())
}
#[post("/value/<val>")]
fn post_value(val : u8) -> String{
    // unsafe{
    //     counter = val ;
    // }
    print!("{:?}",VALUE.lock().unwrap()) ;
    *VALUE.lock().unwrap() = val ;
    format!("POST OK: {}", val)
}

static mut counter : u8 = 0 ;

fn main() {

    unsafe{
        counter = 1 ;
    }

    rocket::ignite().mount("/", 
    routes![hello,get_value,post_value]
)
    .launch();
}