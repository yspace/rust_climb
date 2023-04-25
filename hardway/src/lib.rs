pub mod copies;
pub mod polymorphisms;
pub mod types;
pub mod visibilities;
pub mod aliases;
pub mod from_intos;
pub mod rcs;
pub mod functionals;
pub mod design_patterns;
mod plugins;

use futures::Future;

pub fn some_api() {
    println!("some_api");
}

/// 此函数充当tokio::main角色
pub fn tokio_block_on<F>(future: F)
where
    F: Future + Send + 'static
    // F::Output: Send + 'static,
    //F: impl Future<Output = ())> // TODO 如何描述一个async块 或者异步函数的类型
{
    tokio::runtime::Builder::new_multi_thread()
        // .max_blocking_threads(1)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            println!("Hello world");
            future.await ;
        })
}
