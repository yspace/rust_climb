一些启示

应用级对象 状态如何被共享的 data方法啥原理？

```rust
let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
let manager = ConnectionManager::<MysqlConnection>::new(database_url);
let pool = r2d2::Pool::builder()
.build(manager)
.expect("Failed to create pool.");

HttpServer::new(move || {
    App::new()
    // 这个data方法内部到底咋回事？
        .data(pool.clone())
        .route("/", web::get().to(index))
        .configure(todoController::init)
})
```

@see https://github.com/Trioxidation/Triox/blob/master/src/main.rs#L56


## 应用状态共享

@see https://www.newline.co/books/fullstack-rust/adding-state-to-our-web-app
~~~rust
struct AppState {
    server_id: usize,
    request_count: Cell<usize>,
    messages: Arc<Mutex<Vec<String>>>,
}
~~~

[Actix-web updating web::Data](https://users.rust-lang.org/t/actix-web-updating-web-data/54774/3)

>> 
    To be able to mutate data stored in an Arc (or a Data, which is just a wrapper for an Arc), you need some way of making those mutations thread-safe.

    The simplest way of doing this would be to wrap AppState (or individual fields of AppState, if you want to be more fine grained) in a Mutex, which allows you to 'lock' the data so that only one thread can access it at a time. There's also RwLock, which allows multiple readers or one writer, but never both.

    However, since you're specifically using a HashMap, you might want to look into concurrent data structures like dashmap 6, which are specifically designed to be concurrently accessed.

[web::Data](https://docs.rs/actix-web/3.3.2/src/actix_web/data.rs.html#67-82)