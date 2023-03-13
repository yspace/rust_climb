#[derive(Debug)]
enum Error {
    ErrUserNotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ErrUserNotFound => {
                
                write!(f, "error: user not found!")
            },
        }
       
    }
}

impl std::error::Error for Error {}

pub type BoxedDynStdError = Box<dyn std::error::Error>;
