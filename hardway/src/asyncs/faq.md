- The Rust Programming Language Forum
How to wait an async in non-async function?

[原问链接](https://users.rust-lang.org/t/how-to-wait-an-async-in-non-async-function/28388)

>>> 
    Rust's async functions do not block by themselves. What they do is to build a little state machine describing the various stages of an asynchronous task (what Rust calls a Future), that is eventually destined to be sent to an event loop for processing. It is this event loop that will then handle the blocking.

    The classic and most mature event loop is tokio , but it was designed for an earlier iteration of Rust's asynchronous story before async/await, and using the new futures with it may not be 100% seamless yet.

    For a more async/await-friendly but less mature experience, you may want to look into recent projects of the (Rust Async WG)[https://github.com/rustasync]  such as tide  and runtime 174, or perhaps the lower-level [romio](https://github.com/withoutboats/romio) and juliex 61 building blocks.

The Future trait is staying with us as the standard interface to the opaque state machine types returned by async fns. Much like the Fn/FnMut/FnOnce traits allow us to manipulate opaque closure types.

However, this trait is migrating into the standard library as std::future::Future 200, because with async fn, the language now needs to have built-in support for futures. futures::future::Future will likely remain around for backwards compatibility, but become a mere reexport of std::future::Future.

I think the Future trait has received slight API changes during the standardization process, but the core ideas remain the same.

- [Designing futures for Rust](https://aturon.github.io/blog/2016/09/07/futures-design/)


- [The Rust Programming Language Forum
How to make a manual future implementation into a async fn
](https://users.rust-lang.org/t/how-to-make-a-manual-future-implementation-into-a-async-fn/36042)


- [The Rust Programming Language Forum
How to wait an async in non-async function?](https://users.rust-lang.org/t/how-to-wait-an-async-in-non-async-function/28388/9)

- [Call ".await" on a non-async function](https://www.reddit.com/r/rust/comments/g48gy2/call_await_on_a_nonasync_function/)

    You can probably use futures::block_on if you want to wait, or tokio::spawn if not.