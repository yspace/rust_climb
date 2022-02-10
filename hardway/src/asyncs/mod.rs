pub fn main() {
 println!("in mod ayncs");

 // 
 basic::run();
}

mod basic{
    pub fn run(){
        use futures::executor::block_on;

       let f = do_something() ;
       block_on(f) ;
    }

    async fn do_something(){
        println!("go go go ") ;
    }
}