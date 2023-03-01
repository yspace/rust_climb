fn result_ok(){
    // convert result to option
    let _port: Option<String> = std::env::var("PORT").ok() ;
}

fn result_or() {
    // use default Result if err
    let _port: Result<String , std::env::VarError> =
    std::env::var("PORT").or(Ok(String::from("8080")));
}