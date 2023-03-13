mod user ;

// re-export https://users.rust-lang.org/t/re-exporting-with-pub-use-documentation/39286
// go版本的示例中 user模块隶属于model的 但rust是一个文件就是一个mod 所以重导出一下
pub use  user::* ;

