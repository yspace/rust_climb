use std::fmt::Display;

#[derive(Debug)]
struct AppError {}

impl Display for AppError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AppError")
    }
}

/**
 * 为我们的应用错误实现标准错误trait
 * 有两个硬性约束  
 *
 * -  the Display trait, meaning that it can be format!ed with {}, and
 * -  the Debug trait, meaning that it can be format!ed with {:?}.
 *
 * 一个是给用户看的 一个是给程序员看的👀。
 * 实现上面👆两个trait 也可以方便的使用format宏 将之转变为字符串啦😄
 * 
 */
impl std::error::Error for AppError {
    // a default implementation (Item 13) returning None, indicating that inner error information isn't available.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
    
    //  fn description(&self) -> &str {
    //     "description() is deprecated; use Display"
    // }

    // fn cause(&self) -> Option<&dyn std::error::Error> {
    //     self.source()
    // }

    // fn provide<'a>(&'a self, demand: &mut std::any::Demand<'a>) {}
     
    
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    return Err(Box::new(AppError {}));
}

type UserId = usize;
pub fn find_user(username: &str) -> Result<UserId, String> {
    // 错误转字符串类型
    let f = std::fs::File::open("/etc/passwd")
        .map_err(|e| format!("Failed to open password file: {:?}", e))?;
    // ...


    Ok(1)
}
