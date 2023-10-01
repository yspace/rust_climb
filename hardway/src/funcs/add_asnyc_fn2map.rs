use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

type BoxedResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

// type CalcFn = Box<dyn Fn(i32, i32) -> dyn Future<Output = BoxedResult<i32>>>;

// type CalcFn = Box<dyn Fn(i32, i32) -> Pin<Box<dyn Future<Output = i32>>>>;
type CalcFn = Box<dyn Fn(i32, i32) -> Pin<Box<dyn Future<Output = BoxedResult<i32>>>>>;

#[test]
fn it_works() {
    async fn add(a: i32, b: i32) -> BoxedResult<i32> {
        Ok(a + b)
    }

    async fn sub(a: i32, b: i32) -> BoxedResult<i32> {
        Ok(a - b)
    }

    let mut map: HashMap<&str, CalcFn> = Default::default();
    map.insert("add", Box::new(|a, b| Box::pin(add(a, b))));
    map.insert("sub", Box::new(|a, b| Box::pin(sub(a, b))));

    println!("map size: {}", map.len());

    //map.get("add").unwrap()(2, 3).await
}

mod _2 {
    use futures::future::{FutureExt, LocalBoxFuture}; // 0.3.5
    use std::collections::HashMap;

    type BoxedResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
    type CalcFn = Box<dyn Fn(i32, i32) -> LocalBoxFuture<'static, BoxedResult<i32>>>;

    async fn add(a: i32, b: i32) -> BoxedResult<i32> {
        Ok(a + b)
    }

    async fn sub(a: i32, b: i32) -> BoxedResult<i32> {
        Ok(a - b)
    }

    async fn example() {
        let mut map: HashMap<&str, CalcFn> = Default::default();
        map.insert("add", Box::new(|a, b| add(a, b).boxed()));
        map.insert("sub", Box::new(|a, b| sub(a, b).boxed()));

        println!("map size: {}", map.len());

        //map.get("add").unwrap()(2, 3).await
    }
}
