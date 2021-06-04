use mini_redis::{client, Result} ;

#[tokio::main]
pub async fn main() -> Result<()> {
    // let mut client = client::connect("127.0.0.1:6379").await? ;

    // client.set("hello", "world".into()).await? ;

    // let result = client.get("hello").await? ;

    // println!("got value from the server; result={:?}",result) ;


    // ## Using async/await

    using_async_await::main().await ;

    Ok(())
    
}

mod using_async_await{
    /**
     * Async functions are called like any other Rust function. 
     * However, calling these functions does not result in the function body executing. 
     * Instead, calling an async fn returns a value representing the operation. 
     * This is conceptually analogous to a zero-argument closure. 
     * To actually run the operation, you should use the .await operator on the return value.


     * 
     */
    async fn say_world() {
        println!("world") ;
    }

    pub async fn main() {
        //  // Calling `say_world()` does not execute the body of `say_world()`.
        let op = say_world() ;

        // This println! comes first
        println!("hello") ;

        // Calling `.await` on `op` starts executing `say_world`.

        op.await ;
    }
}