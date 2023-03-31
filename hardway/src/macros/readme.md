[macro by example](doc.rust-lang.org/reference/macros-by-example.html.)

> cargo install cargo-expand


[Macros in Rust: A tutorial with examples](https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/)
很不错的文章

## 宏相关知识
rust中主要分 声明宏 和 过程宏 后者相对高级复杂一些 

宏可用来做一些重复的工作 比如一些结构 或者过程只有名称不同 属性不一样 这些内部使用的宏一般不会作为公共行为暴露出去的

宏也是实现`反射`特性的一种可用手段

优秀crate: derive_builder

### 声明宏
* 可以递归调用 宏可以调用其他宏


## 宏经典用法
声明宏很像方法 但为啥非要用它

优势：
可以做变参函数的功能 如println!

- logging  log crate
- lazy_static mini-dsl