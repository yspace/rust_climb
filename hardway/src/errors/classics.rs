pub enum AppError {
    FileError,
    DBError,
    ApiError,
    //
    Custom(String),
}

// 错误转换在rust中经常用到的
impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        // todo!()
        AppError::FileError
    }
}

pub fn some_use_case() -> Result<(), AppError> {
    // 一些列操作
    write_to_file()?;

    Ok(())
}

fn write_to_file() -> std::io::Result<()> {
    use std::fs::File;
    use std::io::prelude::*;
    let mut file = File::create("filename")?;
    file.write_all(b"File contents")?;
    Ok(())
}

#[test]
fn it_works() {}

mod _2 {
    enum ErrorTypes {
        IoError(std::io::Error),
        FormatError(std::fmt::Error),
    }
    struct ErrorWrapper {
        source: ErrorTypes,
        message: String,
    }

    impl From<std::io::Error> for ErrorWrapper {
        fn from(source: std::io::Error) -> Self {
            Self {
                source: ErrorTypes::IoError(source),
                message: "there was an IO error!".into(),
            }
        }
    }

    fn write_to_file() -> Result<(), ErrorWrapper> {
        use std::fs::File;
        use std::io::prelude::*;
        let mut file = File::create("filename")?;
        file.write_all(b"File contents")?;
        Ok(())
    }
    fn try_to_write_to_file() {
        match write_to_file() {
            Ok(()) => println!("Write suceeded"),
            Err(err) => println!("Write failed: {}", err.message),
        }
    }
}
