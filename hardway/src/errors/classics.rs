pub enum AppError {
    FileError,
    DBError,
    ApiError,
    //
    Custom(String),
}

impl  From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
       // todo!()
       AppError::FileError
    }
}

pub fn some_use_case() -> Result<(),AppError>{
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
fn it_works(){

}