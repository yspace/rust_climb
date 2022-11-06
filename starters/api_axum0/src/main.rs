use axum::{
    routing::get,
    Router,
    Extension,
    response::{Html}
};


mod db ; 

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
    
    .route("/", get(|| async { "Hello, World!" }))
    // 同时绑定了GET及POST方法的路由
    .route("/index", get(root).post(post_foo))
    .route("/db",get(db))
    .route("/create_table",get(db::create))
    .layer(db::init_db().unwrap());

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn root() -> String {
    String::from("hello axum")
}
async fn get_foo() -> String {
    String::from("get:foo")
}
async fn post_foo() -> String {
    String::from("post:foo")
}
async fn foo_bar() -> String {
    String::from("foo:bar")
}

async fn db(Extension(database): Extension<db::Database>) -> Html<&'static str> {
    let connection = database.connection().unwrap(); // Do stuff with connection
    Html("Hello, sqlite!")
}

