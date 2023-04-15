use axum::{response::{Response, IntoResponse}, http::StatusCode};


pub type Result<T> = core::result::Result<T,Error> ;

#[derive(Debug)]
pub enum Error {
    LoginFail,
}

// impl Into<Response> for Error {
//     fn into(self) -> Response {
//         todo!()
//     }
// }
impl IntoResponse for  Error {
    fn into_response(self) -> Response {
        println!("-->> {:<12} - {self:?}","INTO_RES");
        (StatusCode::INTERNAL_SERVER_ERROR,"UNHANLDED_CLIENT_ERROR").into_response()
    }
}


impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{:?}", self) 
    }
}
impl std::error::Error for Error {}