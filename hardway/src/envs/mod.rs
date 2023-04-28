use std::env;

pub fn run() {
    print_envs();
}

fn print_envs() {
    println!("{:#?}", env::vars());
    println!("===== env vars ===");
    for (k, v) in env::vars() {
        println!("{} = {}", k, v);
    }
    println!("===== env vars ===");
}

#[test]
fn it_works() {
    use std::env;

    println!("===== env vars ===");
    for (k, v) in env::vars() {
        println!("{} = {}", k, v);
    }
    println!("===== env vars ===");
}

#[test]
fn unwrap_or_else() {
    let db_host = std::env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());

    assert_eq!(db_host, "localhost".to_owned());
}

#[test]
fn env_map() {
    let db_host = std::env::var("POSTGRES_HOST")
    // .map(Some)
    .unwrap_or_default();

    // assert_eq!(db_host, Some("localhost".to_owned()));
    assert_eq!(db_host,  "".to_owned());
}
 
#[test]
fn env_unwrap_or_default() {
    let db_host = std::env::var("POSTGRES_HOST")
    .map(Some)
    .unwrap_or_default();

    assert_eq!(db_host, None);
    // assert_eq!(db_host,  "".to_owned());
}