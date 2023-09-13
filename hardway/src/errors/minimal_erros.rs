#[derive(Debug)]
pub struct MyError(String); // new type pattern 

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for MyError {}

type UserId = usize ;
pub fn find_user(username: &str) -> Result<UserId, MyError> {
    let f = std::fs::File::open("/etc/passwd").map_err(|e| {
        MyError(format!("Failed to open password file: {:?}", e))
    })?;
    // ...

    Ok(0)
}

//  为了方便 顺带实现From
impl std::convert::From<String> for MyError {
    fn from(msg: String) -> Self {
        Self(msg)
    }
}

/**
 * When it encounters the question mark operator (?), the compiler will automatically apply any relevant From trait implementations that are needed to reach the destination error return type. This allows further minimization:
 */
pub fn find_user2(username: &str) -> Result<UserId, MyError> {
    let f = std::fs::File::open("/etc/passwd")
        .map_err(|e| format!("Failed to open password file: {:?}", e))?;
    // ...

    Ok(0)
}
