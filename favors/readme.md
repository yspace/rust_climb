
## 基础

crossbeam_channel

once_cell lazy_static

[libc](https://github.com/rust-lang/libc)
代码可以看看（大致浏览下）哦！


### 连接池
- [deadpool](https://crates.io/crates/deadpool)

### 调度器
-   [scheduler-clokwerk](https://docs.rs/clokwerk/latest/clokwerk/)

https://crates.io/crates/clokwerk

## 工具｜通用库
- [derive_builder ]()
- [getset]()
- [derive_getters]

## 并行-并发
- rayon 
    async 适用于io密集应用
    rayon 适用于cpu密集应用

## 错误处理 

thiserror

## 文件

- ##directories##

## 数据
comfy_table

## 集合-容器-数据结构
- [dashmap](https://github.com/xacrimon/dashmap)
Blazingly fast concurrent map in Rust.
DashMap is an implementation of a concurrent associative array/hashmap in Rust.
DashMap tries to be very simple to use and to be a direct replacement for RwLock<HashMap<K, V>>. To accomplish these goals, all methods take &self instead of modifying methods taking &mut self. This allows you to put a DashMap in an Arc<T> and share it between threads while still being able to modify it.


## 测试
- criterion
benchmark

## 其他
+ Polars 对标pandas

+ [delegate](https://crates.io/crates/delegate)

+ [strum](https://crates.io/crates/strum)
Strum is a set of macros and traits for working with enums and strings easier in Rust.

一般需要同时引入相关的宏 strum_macros