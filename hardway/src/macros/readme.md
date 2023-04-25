[macro by example](doc.rust-lang.org/reference/macros-by-example.html.)

> cargo install cargo-expand

[Macros in Rust: A tutorial with examples](https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/)
很不错的文章

- [How do I use a macro across module files?](https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files)

Macros within the same crate
New method (since Rust 1.32, 2019-01-17)

```rust
foo::bar!();  // works

mod foo {
    macro_rules! bar {
        () => ()
    }

    pub(crate) use bar;    // <-- the trick
}

foo::bar!();  // works
```

With the pub use, the macro can be used and imported like any other item. And unlike the older method, this does not rely on source code order, so you can use the macro before (source code order) it has been defined.

### macros across crates

```rust

#[macro_export]
macro_rules! foo {
    () => ()
}

```

Note that with this method, macros always live at the top-level of a crate! So even if foo would be inside a mod bar {}, the user crate would still have to write use util::foo; and not use util::bar::foo;. By using pub use, you can export a macro from a module of your crate (in addition to it being exported at the root).

Before Rust 2018, you had to import macro from other crates by adding the attribute #[macro_use] to the extern crate util; statement. That would import all macros from util. This syntax should not be necessary anymore.

## 宏相关知识

rust 中主要分 声明宏 和 过程宏 后者相对高级复杂一些

宏可用来做一些重复的工作 比如一些结构 或者过程只有名称不同 属性不一样 这些内部使用的宏一般不会作为公共行为暴露出去的

宏也是实现`反射`特性的一种可用手段

优秀 crate: derive_builder

### 声明宏

- 可以递归调用 宏可以调用其他宏

## 宏经典用法

声明宏很像方法 但为啥非要用它

优势：
可以做变参函数的功能 如 println!

- logging log crate
- lazy_static mini-dsl
