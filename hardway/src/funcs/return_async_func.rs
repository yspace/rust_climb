//@see https://stackoverflow.com/questions/61167939/return-an-async-function-from-a-function-in-rust?rq=3

mod return_fn {
    use std::future::Future;
    use std::pin::Pin;

    pub async fn some_async_func(arg: &str) {}

    pub fn some_async_func_wrapper<'a>(arg: &'a str) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
        Box::pin(some_async_func(arg))
    }

    pub fn higher_order_func<'a>(
        action: &str,
    ) -> fn(&'a str) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
        some_async_func_wrapper
    }

    // With higher order function returning function pointer
    async fn my_function() {
        let action = "one";
        let arg = "hello";
        higher_order_func(action)(arg).await;
    }

    #[test]
    fn test_me() {}
}

mod return_Fn {
    use std::pin::Pin;

    use futures::Future;

    pub async fn some_async_func(arg: &str) {}

    pub fn higher_order_func<'a>(
        action: &str,
    ) -> impl Fn(&'a str) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
        |arg: &'a str| Box::pin(some_async_func(arg))
    }
}

mod return_Fn_on_cond {
    use std::pin::Pin;

    use futures::Future;

    pub async fn some_async_func_one(arg: &str) {}
    pub async fn some_async_func_two(arg: &str) {}

    pub fn higher_order_func<'a>(
        action: &str,
    ) -> Box<dyn Fn(&'a str) -> Pin<Box<dyn Future<Output = ()> + 'a>>> {
        if action.starts_with("one") {
            Box::new(|arg: &'a str| Box::pin(some_async_func_one(arg)))
        } else {
            Box::new(|arg: &'a str| Box::pin(some_async_func_two(arg)))
        }
    }
}

mod return_future {
    use futures::Future;

    pub async fn some_async_func(arg: &str) {}
    // Alternative: returning a future
    // To simplify things, consider returning a future itself instead of a function pointer.
    // This is virtually the same, but much nicer and does not require heap allocation:
    pub fn higher_order_func_future<'a>(
        action: &str,
        arg: &'a str,
    ) -> impl Future<Output = ()> + 'a {
        some_async_func(arg)
    }

    // With higher order function returning future
    async fn my_function() {
        let action = "one";
        let arg = "hello";
        higher_order_func_future(action, arg).await;
    }
}

mod based_on_action {
    use std::pin::Pin;

    use futures::Future;

    pub async fn some_async_func_one(arg: &str) {}
    pub async fn some_async_func_two(arg: &str) {}

    pub fn higher_order_func_future<'a>(
        action: &str,
        arg: &'a str,
    ) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
        if action.starts_with("one") {
            Box::pin(some_async_func_one(arg))
        } else {
            Box::pin(some_async_func_two(arg))
        }
    }
}

#[test]
fn test_me() {}
