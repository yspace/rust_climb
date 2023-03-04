you must first implement the Serialize and Deserialize traits on your types. Thanks to derive macros, this is really trivial for most types. To use derive macros, make sure you enable the “derive” feature flag for serde in your dependencies.

cargo.toml
~~~toml

[dependencies]
serde = { version = "1", features = ["derive"] }
# 跟serde相关的库是按需引入的 除了json 还有yaml 之类
serde_json = "1"

~~~

## todos
- 读完 https://serde.rs/