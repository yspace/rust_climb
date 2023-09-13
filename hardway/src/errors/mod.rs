mod status;
pub mod classics;
pub mod trait_bounds;
pub mod minimal_erros;
pub mod nested_errors;

pub fn main() {
    println!("hi this is demo for handling error in rust");
    // https://www.youtube.com/watch?v=UgIQo__luHw&ab_channel=BoyMaas

    // let mut byte : u8 = 0b000000001 ;
    let ret = status::ErrorStatus::Success ;

    // if  0 == (ret | status::ErrorStatus::Success) {
    //     println!("no errors");
    // }else{
    //     println!("some errors happened!"); 
    // }
    println!("{:?}", ret);
}

mod anyhow_demo {

    #[derive(Debug)]
    pub enum MyError {
        IoError(String),
        Inexist(String),
    }

    pub type Result<T> = std::result::Result<T, MyError>;

    // pub fn fetch_id() -> Result<u64> {
    //     let content = std::fs::read_to_string("/tmp/tmp_id")?;
    //     let id = content.parse::<u64>()?;
    //     Ok(id)
    // }
}

mod thiserror_demo {
    // from : https://dev.to/joaoh82/what-would-sqlite-look-like-if-written-in-rust-part-2-4g66
    use thiserror::Error;

    use std::result;

    // use sqlparser::parser::ParserError;

    pub type Result<T> = result::Result<T, MyError>;

    #[derive(Error, Debug, PartialEq)]
    pub enum MyError {
        #[error("Not Implemented error: {0}")]
        NotImplemented(String),
        #[error("General error: {0}")]
        General(String),
        #[error("Internal error: {0}")]
        Internal(String),
        #[error("Unknown command error: {0}")]
        UnknownCommand(String),
        // #[error("SQL error: {0:?}")]
        // SqlError(#[from] ParserError),
    }

    /// Return SQLRite errors from String
    pub fn sqlrite_error(message: &str) -> MyError {
        MyError::General(message.to_owned())
    }
}

mod error_chain_demo {}

mod color_eyre_demo {
    // @see https://github.com/stevepryde/thirtyfour/blob/main/thirtyfour/examples/tokio_basic.rs
    fn run() -> color_eyre::Result<()> {
        // The use of color_eyre gives much nicer error reports, including making
        // it much easier to locate where the error occurred.
        color_eyre::install()?;

        Ok(())
    }
}
