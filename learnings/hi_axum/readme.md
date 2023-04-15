## 静态文件服务
参考： [An Identity Management template for microservices ](https://github.com/opeolluwa/raccoon)
```rust
//static file mounting
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("views");
    let static_files_service = get_service(
        ServeDir::new(assets_dir).append_index_html_on_directories(true),
    )
    .handle_error(|error: std::io::Error| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", error),
        )
    });

      //mount the app routes and middleware
    let app = Router::new()
        .fallback(static_files_service)
        .nest("/api/v1/", routes::root::router())
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(Extension(database));

```

https://robert.kra.hn/posts/2022-04-03_rust-web-wasm/
