https://github.com/paulkernfeld/global-data-in-rust

堆上的数据 必须要配备 allocator

> Heap allocation is convenient because you don't need to know the size of your data at compile time. However, it means that you can't use this method without an allocator. Avoiding heap allocations is most important in embedded programming, real-time systems, and really high-performance applications.

[How to Idiomatically Use Global Variables in Rust](https://www.sitepoint.com/rust-global-variables/)

https://github.com/yujinliang/rust_learn/blob/master/global_data_usage/Readme.md

[rust 全局变量揭秘](https://morestina.net/blog/1774/rust-global-variables-demystified)
理由：

Is it even ok to use global variables?
Global variables are a controversial topic in programming circles, with many educators taking the moral high ground and condemning them as code smell, a shortcut, a crutch, hallmark of throw-away code, and <insert favorite insult>. While there is good reason for the hostility, there is also an abundance of situations where global variables are either appropriate or actually the only way to proceed. For example, pre-compiled regular expressions or state of a logger are examples where you’d probably want to use globals in preference to sending the state all the way from top-level to the bottom-most part of your program. Many low-level system APIs, such as those of signal handlers or hardware-triggered interrupts, have callbacks that don’t receive a “state” argument, so communication between the callback and the rest of the system must go through globals. There are other examples, and in this article we’ll assume that you have good reasons for using globals that your CS professor would approve of. If that’s not the case, don’t worry, we won’t tell.

可变全局变量需要用 unsafe 块来提醒调用者 把数据竞争的负担转给调用者了

### const fn

常量函数 的概念

大意好像是 可以在编译器推断出占用空间的函数

function specifically marked as runnable at compile time

String::from() requires a run-time allocation and is therefore not a const fn.

std::sync::Mutex::new() is not const fn because it requires an allocation in order to keep the system mutex at a fixed address.

- [what_is_const_fn](https://www.reddit.com/r/rust/comments/iksmgk/psa_what_is_const_fn/)

````rust
So why is const fn useful? One thing you cannot ordinarily do with a function is call it in a constant position. Let's see what I mean:


    // a normal function
    fn foo(n: usize) -> usize {
        n + 1
    }

    fn main() {
        const BAR: usize = foo(5);
        let array = [0_u8; foo(7)];
    }

     // a constant function
    const fn foo(n: usize) -> usize {
        n + 1
    }
```

In summary, that's all const fn means. You can use it in more places than an ordinary function. It can be used as a static variable, a const variable, as a length for an array, etc.

- [关于此的视频](https://www.youtube.com/watch?v=Vw8BFScm0K0)


https://practice.rs/generics-traits/const-generics.html