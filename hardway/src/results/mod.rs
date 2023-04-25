


fn to_option(){
    let _port = std::env::var("PORT").ok() ;
}

fn use_default_if_err(){
    let _port:Result<String, std::env::VarError> = std::env::var("PORT").or(
        Ok(String::from("8080"))
    ) ;
}


// 
pub type MyResult<T> = core::result::Result<T, MyError>;
pub enum MyError {
   ValidateError,
   IoError,
   DbError,
   ApiError, 
}
fn f()-> MyResult<()> /*std::io::Result<()> */{
    Ok(())
}
