//! tests/health_check.rs

use std::net::TcpListener;
// `actix_rt::test` is the testing equivalent of `actix_web::main`.
// It also spares you from having to specify the `#[test]` attribute. //
// Use `cargo add actix-rt --dev --vers 2` to add `actix-rt`
// under `[dev-dependencies]` in Cargo.toml
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file) #[actix_rt::test]
#[actix_rt::test]
async fn health_check_works() {
    let address = spawn_app();
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application. //
    // Use `cargo add reqwest --dev --vers 0.11` to add
    // it under `[dev-dependencies]` in Cargo.toml let client = reqwest::Client::new();
    let client = reqwest::Client::new();
    // Act
    let response = client
    // Use the returned application address
     .get(&format!("{}/health_check", &address))    
    //  .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
// Launch our application in the background ~somehow~
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
    .expect("Failed to bind random port");

                                                                                 // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // We return the application address to the caller!
    format!("http://127.0.0.1:{}", port)
}
