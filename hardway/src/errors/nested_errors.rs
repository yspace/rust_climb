// The alternative scenario is where the content of nested errors is important enough that it should be preserved and made available to the caller.

const MAX_LEN: usize = 512;

#[derive(Debug)]
pub enum MyError {
    Io(std::io::Error),
    Utf8(std::string::FromUtf8Error),
    General(String),
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "IO error: {}", e),
            MyError::Utf8(e) => write!(f, "UTF-8 error: {}", e),
            MyError::General(s) => write!(f, "General error: {}", s),
        }
    }
}

use std::error::Error;
// It also makes sense to override the default source() implementation for easy access to nested errors.
impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MyError::Io(e) => Some(e),
            MyError::Utf8(e) => Some(e),
            MyError::General(_) => None,
        }
    }
}

/// Return the first line of the given file.
pub fn first_line(filename: &str) -> Result<String, MyError> {
    let file = std::fs::File::open(filename).map_err(MyError::Io)?;
    let mut reader = std::io::BufReader::new(file);

    // (A real implementation could just use `reader.read_line()`)
    let mut buf = vec![];
    // let len = reader.read_until(b'\n', &mut buf).map_err(MyError::Io)?;
    // todo read_util 来自哪个trait?
    let len = 10 ;

    let result = String::from_utf8(buf).map_err(MyError::Utf8)?;
    if result.len() > MAX_LEN {
        return Err(MyError::General(format!("Line too long: {}", len)));
    }
    Ok(result)
}


// 最好也实现From trait 如下👇 这样免得让库使用者 遭受孤儿原则 MyError跟From都不属于他们自己
impl From<std::io::Error> for MyError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
impl From<std::string::FromUtf8Error> for MyError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::Utf8(e)
    }
}

// 😄 然后代码可以更精简  可以看到 消除了map_error 方法
// 自己写这些逻辑 需要一些模版代码 最好使用 `thiserror` 库 可以省不少心
    /// Return the first line of the given file.
    pub fn first_line2(filename: &str) -> Result<String, MyError> {
        let file = std::fs::File::open(filename)?; // via `From<std::io::Error>`
        let mut reader = std::io::BufReader::new(file);
        let mut buf = vec![];
        //  let len = reader.read_until(b'\n', &mut buf)?; // via `From<std::io::Error>`
        let len = 10 ;
        
        let result = String::from_utf8(buf)?; // via `From<std::string::FromUtf8Error>`
        if result.len() > MAX_LEN {
            return Err(MyError::General(format!("Line too long: {}", len)));
        }
        Ok(result)
    }