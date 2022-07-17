
~~~toml

[[bin]]
name = "multi_bin"
path = "src/bin/main.rs"
[[bin]]
name = "multi_bin2"
path = "src/bin/main.rs"


~~~

多个bin 运行时需指定具体要运行哪个 或者在cargo.toml中使用default-run ：
> cargo run -p multi_bin --bin multi_bin2  

cargo 惯例：

Cargo compiles all files in src/bin into executables with the same name automatically. 

只有不遵循惯例的情况才需要指定 bin表配置段

本项目中 src同级别的是不遵循惯例的 对于遵循惯例的`src/bin/server.rs` `src/bin/client.rs` 是不需要配置的
调用：
> cargo run -p multi_bin --bin server