pub fn main() {
 println!("in mod ayncs");

 // 
 basic::run();
}

mod basic{
    pub fn run(){
        do_something() ;
    }

    async fn do_something(){
        println!("go go go ") ;
    }
}