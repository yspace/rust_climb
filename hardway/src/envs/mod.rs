use std::env; 

pub fn run(){
    print_envs() ;
}


  fn print_envs(){
    println!("{:#?}", env::vars());     println!("===== env vars ===") ;
    for (k,v ) in env::vars() {
        println!("{} = {}", k, v);
    }
    println!("===== env vars ===") ; 
}


#[test]
fn it_works() {
     use std::env;

     println!("===== env vars ===") ;
     for (k,v ) in env::vars() {
         println!("{} = {}", k, v);
     }
     println!("===== env vars ===") ;
}