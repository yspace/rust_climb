// 这两个trait 提供了类型间转换的通用接口
// 通常情况只需要实现From into是自动实现的

// 不要自己写类型转换routines 如果可能就尽量依赖well-know traits

// From 最常用的情形就是错误转换 内部被调用的函数可能返回各种不同的错误 他们需要被转换到统一的外部错误类型
// 每个crate 一般都有其自己的错误类型 当使用它们时常需要转换为我们自己的错误类型 特别是在使用`?`语法糖时

struct MyString(String);
impl From<&str> for MyString {
    fn from(other: &str) -> Self {
        Self(other.into())
    }
}
fn main() {
    println!("{}", MyString::from("Hello, world!").0);
}

mod for_result {
    use std::{fs::File, io::Read};
    struct Error(String);
    
    // 当碰到`?`时 编译器知道怎么把内部错误转换为外部方法所使用的错误类型
    fn read_file(name: &str) -> Result<String, Error> {
        let mut f = File::open(name)?;
        let mut output = String::new();
        f.read_to_string(&mut output)?;
        Ok(output)
    }
    impl From<std::io::Error> for Error {
        fn from(other: std::io::Error) -> Self {
            Self(other.to_string())
        }
    }
}


// TryFrom 和 TryInto 用法基本一样 只不过通过返回Result类型多了容错能力  不带Try的可能导致程序crash