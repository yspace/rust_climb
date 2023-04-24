// src/main.rs

mod logger;

type StdErr = Box<dyn std::error::Error>;

fn main() -> Result<(), StdErr> {
    // loads env variables from .env
    dotenv::dotenv()?;
    // 也有这样用的 dotenv().ok(); 调用 ok() 之后，会把 Result 转化为 Option，而 Option 就不会产生未使用 Result 的警告了。使用 ok() 的目的就是当加载 dotenv 环境文件失败的时候可以忽略错误。    

    // example
    // assert_eq!("INFO", std::env::var("LOG_LEVEL").unwrap());

    logger::init()?;

    Ok(())
}