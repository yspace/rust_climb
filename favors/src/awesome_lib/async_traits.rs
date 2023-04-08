// 干倒一群初学者： https://hirrolot.github.io/posts/rust-is-hard-or-the-misery-of-mainstream-programming.html

use async_trait::async_trait;

#[derive(Debug)]
struct Update;

#[async_trait]
trait Handler {
    async fn update(&self, update: &Update) -> ();
}

#[derive(Default)]
struct Dispatcher {
    handlers: Vec<Box<dyn Handler>>,
}

impl Dispatcher {
    pub fn push_handler(&mut self, handler: Box<dyn Handler>) {
        self.handlers.push(handler);
    }
}

// example handler
struct Foo {}
#[async_trait]
impl Handler for Foo {
    async fn update(&self, update: &Update) -> () {
        println!("Update: {:?}", update);
    }
}

// #[tokio::main]
async fn main() {
    let mut dispatcher = Dispatcher::default();
    let handler = Box::new(Foo {});

    dispatcher.push_handler(handler);
}
#[test]
fn test_it() {}
/*
mod original {
    // from :
    use futures::future::BoxFuture;
    use std::future::Future;

    #[derive(Debug)]
    struct Update;

    type Handler = Box<dyn for<'a> Fn(&'a Update) -> BoxFuture<'a, ()> + Send + Sync>;

    struct Dispatcher(Vec<Handler>);

    impl Dispatcher {
        fn push_handler<'a, H, Fut>(&mut self, handler: H)
        where
            H: Fn(&'a Update) -> Fut + Send + Sync + 'a,
            Fut: Future<Output = ()> + Send + 'a,
        {
            self.0.push(Box::new(move |upd| Box::pin(handler(upd))));
        }
    }

    fn main() {
        let mut dp = Dispatcher(vec![]);

        dp.push_handler(|upd| async move {
            println!("{:?}", upd);
        });
    }
}

*/

mod original {
    // @see https://hirrolot.github.io/posts/rust-is-hard-or-the-misery-of-mainstream-programming.html
    use futures::future::BoxFuture;
    use std::future::Future;
    use std::sync::Arc;

    #[derive(Debug)]
    struct Update;

    type Handler = Box<dyn Fn(Arc<Update>) -> BoxFuture<'static, ()> + Send + Sync>;

    struct Dispatcher(Vec<Handler>);

    impl Dispatcher {
        fn push_handler<H, Fut>(&mut self, handler: H)
        where
            H: Fn(Arc<Update>) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = ()> + Send + 'static,
        {
            self.0.push(Box::new(move |upd| Box::pin(handler(upd))));
        }
    }

    fn main() {
        let mut dp = Dispatcher(vec![]);

        dp.push_handler(|upd| async move {
            println!("{:?}", upd);
        });
    }
}
