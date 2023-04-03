
## 监控
tokio-console

> cargo install tokio-console

需要编译指定Flag
> RUSTFLAGS="--cfg tokio_unstable" cargo run. 

~~~toml

[dependencies]
tokio = { version = "1", features = ["full", "tracing"] } tracing = "0.1"
console-subscriber = "0.1"
~~~

## 测试
tokio::test

需要单独添加 `test-util` features 并没有包含在tokio的full特性列表里