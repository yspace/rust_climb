

## 压测

> ab -c 20 -n 1000 -k http://127.0.0.1:3000/hello

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


https://bloerg.net/posts/serve-static-content-with-axum/

~~~rust
use include_dir::{include_dir, Dir};

static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");

async fn static_path(Path(path): Path<String>) -> impl IntoResponse {
    let path = path.trim_start_matches('/');
    let mime_type = mime_guess::from_path(path).first_or_text_plain();

    match STATIC_DIR.get_file(path) {
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new()))
            .unwrap(),
        Some(file) => Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(mime_type.as_ref()).unwrap(),
            )
            .body(body::boxed(Full::from(file.contents())))
            .unwrap(),
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = axum::Router::new()
        .route("/static/*path", get(static_path));

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
~~~

- https://benw.is/posts/serving-static-files-with-axum

~~~rust 
use axum::{
    body::{boxed, Body, BoxBody},
    http::{Request, Response, StatusCode, Uri},
};
use tower::ServiceExt;
use tower_http::services::ServeDir;

pub async fn file_handler(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let res = get_static_file(uri.clone()).await?;
    println!("{:?}", res);

    if res.status() == StatusCode::NOT_FOUND {
        // try with `.html`
        // TODO: handle if the Uri has query parameters
        match format!("{}.html", uri).parse() {
            Ok(uri_html) => get_static_file(uri_html).await,
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Invalid URI".to_string())),
        }
    } else {
        Ok(res)
    }
}

async fn get_static_file(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();

    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // When run normally, the root is the workspace root
    match ServeDir::new("./webauthn_client/pkg").oneshot(req).await {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", err),
        )),
    }
}
~~~
