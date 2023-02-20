
https://ryhl.io/blog/async-what-is-blocking/

To give a sense of scale of how much time is too much, a good rule of thumb is no more than 10 to 100 microseconds between each .await. That said, this depends on the kind of application you are writing

### Read LIST

[Async Rust: What is a runtime? Here is how tokio works under the hood](https://kerkour.com/rust-async-await-what-is-a-runtime)
[Using Rustlang’s Async Tokio Runtime for CPU-Bound Tasks](https://thenewstack.io/using-rustlangs-async-tokio-runtime-for-cpu-bound-tasks/)

### 揭秘

~~~rust
#[async_std::main]
async fn main() {
}
~~~
https://github.com/async-rs/async-attributes/blob/master/src/lib.rs#L75

等价
~~~rust
let result = quote! {
        #vis fn main() #ret {
            #(#attrs)*
            async fn main(#inputs) #ret {
                #body
            }

            async_std::task::block_on(async {
                main().await
            })
        }

    };

    result.into()
~~~

书籍： https://book.async.rs/concepts/tasks.html

## 深入理解async
https://github.com/nyxtom/async-in-depth-rust-series

### Futures
Futures 是在计算上的抽象
> Futures abstract over computation. They describe the "what", independent of the "where" and the "when". For that, they aim to break code into small, composable actions that can then be executed by a part of our system. 

`Send` abstracts over passing data in a computation to another concurrent computation (let's call it the receiver), losing access to it on the sender side.

`Sync` is about sharing data between two concurrent parts of a program.

### An easy view of `computation`

While computation is a subject to write a whole book about, a very simplified view suffices for us: A sequence of composable operations which can branch based on a decision, run to succession and yield a result or yield an error

**Deferring computation**

As mentioned above, Send and Sync are about data. But programs are not only about data, they also talk about computing the data. And that's what Futures do.

> 线程用于并行，异步用于等待并行！

### 一些启发
https://github.com/stevepryde/thirtyfour/blob/main/thirtyfour/examples/tokio_basic.rs