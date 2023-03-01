
[Rust Lifetimes](https://www.youtube.com/watch?v=1QoT9fmPYr8&ab_channel=DougMilford)


## from black hat rust
生命周期标注被用来告诉编译器 我们正在使用一些长生命周期的引用

生命周期消除 有三个规则
- 
- 
- 

~~~rust
fn do_something(x: &u64)-> &u64 { println!("{}", x);
x
}
// is equivalent to
fn do_something_else<'a>(x: &'a u64)-> &'a u64 {
    println!("{}", x);
x
}
~~~