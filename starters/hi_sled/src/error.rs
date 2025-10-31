use thiserror::Error;

// @see https://google.github.io/comprehensive-rust/error-handling/thiserror.html
#[derive(Error, Debug)]
pub enum MyError {
    // DataNotFound 错误的描述
    #[error("data not found")]
    DataNotFound,
    // InvalidInput 错误的描述
    // #[error("invalid input")]
    // InvalidInput,



    #[error("Failed to read file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse data: {0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

// 示例函数，展示如何使用自定义错误
// fn search_data(query: &str) -> Result<(), MyError> {
//     if query.is_empty() {
//         // 当查询为空时，返回 InvalidInput 错误
//         return Err(MyError::InvalidInput);
//     }
//     // 这里省略了实际的数据查询逻辑
//     // ...
//     // 数据未找到时返回 DataNotFound 错误
//     Err(MyError::DataNotFound)
// }



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_search_data( ) {

    }
}
