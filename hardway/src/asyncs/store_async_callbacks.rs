use std::boxed::Box;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
// @see https://stackoverflow.com/questions/67379054/store-async-callback-in-map?rq=3

type AsyncCallback = Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>>>;

#[derive(Default)]
pub struct CallbackMap {
    callbacks: HashMap<&'static str, AsyncCallback>,
}

impl CallbackMap {
    pub fn add<C, F>(&mut self, name: &'static str, callback: C)
    where
        C: Fn() -> F,
        C: 'static,
        F: Future<Output = ()> + 'static,
    {
        self.callbacks
            // .insert(name, Box::new(|| Box::pin(callback())));
        .insert(name, Box::new(move || Box::pin(callback())));
    }

    pub async fn execute(&self) {
        for (_key, value) in &self.callbacks {
            value().await;
        }
    }
}

async fn async_callback() {
    println!("Callback 2");
}

#[test]
fn main() {
    let mut callbacks = CallbackMap::default();

    callbacks.add("test1", || async {
        println!("Callback 1");
    });

    callbacks.add("test2", async_callback);

    callbacks.execute();
}

