
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