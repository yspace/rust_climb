/*

trait Error: Debug + Display {
    // provided default impls
    fn source(&self) -> Option<&(dyn Error + 'static)>;
    fn backtrace(&self) -> Option<&Backtrace>;
    fn description(&self) -> &str;
    fn cause(&self) -> Option<&dyn Error>;
}

In Rust errors are returned, not thrown.
 */

mod divide_by_zero {
    use std::error;
    use std::fmt;

    #[derive(Debug, PartialEq)]
    struct DivByZero;

    impl fmt::Display for DivByZero {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "division by zero error")
        }
    }

    impl error::Error for DivByZero {}

    fn safe_div(numerator: i32, denominator: i32) -> Result<i32, DivByZero> {
        if denominator == 0 {
            return Err(DivByZero);
        }
        Ok(numerator / denominator)
    }

    #[test] // ✅
    fn test_safe_div() {
        assert_eq!(safe_div(8, 2), Ok(4));
        assert_eq!(safe_div(5, 0), Err(DivByZero));
    }

    /*

    Since errors are returned and not thrown they must be explicitly handled, and if the current function cannot handle an error it should propagate it up to the caller.
    The most idiomatic way to propagate errors is to use the ? operator,
    which is just syntax sugar for the now deprecated try! macro which simply does this:

     */
    // try 现在是rust的保留字 所以先用__try 表示下意思就行 知道问好 `?`咋回事就好
    macro_rules! __try {
        ($expr:expr) => {
            match $expr {
                // if Ok just unwrap the value
                Ok(val) => val,
                // if Err map the err value using From and return
                Err(err) => {
                    return Err(From::from(err));
                }
            }
        };
    }
}

mod question_mark {
    use std::fs::File;
    use std::io;
    use std::io::Read;
    use std::path::Path;

    fn read_file_to_string(path: &Path) -> Result<String, io::Error> {
        let mut file = File::open(path)?; // ⬆️ io::Error
        let mut contents = String::new();
        file.read_to_string(&mut contents)?; // ⬆️ io::Error
        Ok(contents)
    }
}

mod quick_and_dirty_way {
    // Result Error 位置应该放什么类型？有多种方法 最简单粗暴的就是都转成String 便于调用者显示打印出来
    /*
    The first approach is recognizing that all types which impl Error also impl Display
    so we can map all the errors to Strings and use String as our error type:

    这种方法的缺点是 错误信息的类型丢失了 无法轻松的用程序式方法判断是什么类型的错误

     */
    use std::fs::File;
    use std::io;
    use std::io::Read;
    use std::path::Path;

    fn sum_file(path: &Path) -> Result<i32, String> {
        let mut file = File::open(path).map_err(|e| e.to_string())?; // ⬆️ io::Error -> String
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| e.to_string())?; // ⬆️ io::Error -> String
        let mut sum = 0;
        for line in contents.lines() {
            sum += line.parse::<i32>().map_err(|e| e.to_string())?; // ⬆️ ParseIntError -> String
        }
        Ok(sum)
    }
}

mod generic_blanket_impl {
    /*
    来自标准库
     impl<E: error::Error> From<E> for Box<dyn error::Error>;

     any Error type can be implicitly converted into a Box<dyn error::Error> by the ? operator
     */

    use std::{error, io};
    use std::fs::File;
    use std::io::Read;
    use std::num::ParseIntError;
    use std::path::Path;

    fn sum_file(path: &Path) -> Result<i32, Box<dyn error::Error>> {
        let mut file = File::open(path)?; // ⬆️ io::Error -> Box<dyn error::Error>
        let mut contents = String::new();
        file.read_to_string(&mut contents)?; // ⬆️ io::Error -> Box<dyn error::Error>
        let mut sum = 0;
        for line in contents.lines() {
            sum += line.parse::<i32>()?; // ⬆️ ParseIntError -> Box<dyn error::Error>
        }
        Ok(sum)
    }

    fn handle_sum_file_errors(path: &Path) {
        match sum_file(path) {
            Ok(sum) => println!("the sum is {}", sum),
            Err(err) => {
                if let Some(e) = err.downcast_ref::<io::Error>() {
                    // handle io::Error
                } else if let Some(e) = err.downcast_ref::<ParseIntError>() {
                    // handle ParseIntError
                } else {
                    // we know sum_file can only return one of the
                    // above errors so this branch is unreachable
                    unreachable!();
                }
            }
        }
    }
}

mod robust_type_safe_way {
    use std::error;
    use std::fmt;
    use std::fs::File;
    use std::io;
    use std::io::Read;
    use std::num::ParseIntError;
    use std::path::Path;

    #[derive(Debug)]
    enum SumFileError {
        Io(io::Error),
        Parse(ParseIntError),
    }

    impl From<io::Error> for SumFileError {
        fn from(err: io::Error) -> Self {
            SumFileError::Io(err)
        }
    }

    impl From<ParseIntError> for SumFileError {
        fn from(err: ParseIntError) -> Self {
            SumFileError::Parse(err)
        }
    }

    impl fmt::Display for SumFileError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                SumFileError::Io(err) => write!(f, "sum file error: {}", err),
                SumFileError::Parse(err) => write!(f, "sum file error: {}", err),
            }
        }
    }

    impl error::Error for SumFileError {
        // the default impl for this method always returns None
        // but we can now override it to make it way more useful!
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            Some(match self {
                SumFileError::Io(err) => err,
                SumFileError::Parse(err) => err,
            })
        }
    }

    fn sum_file(path: &Path) -> Result<i32, SumFileError> {
        let mut file = File::open(path)?; // ⬆️ io::Error -> SumFileError
        let mut contents = String::new();
        file.read_to_string(&mut contents)?; // ⬆️ io::Error -> SumFileError
        let mut sum = 0;
        for line in contents.lines() {
            sum += line.parse::<i32>()?; // ⬆️ ParseIntError -> SumFileError
        }
        Ok(sum)
    }

    fn handle_sum_file_errors(path: &Path) {
        match sum_file(path) {
            Ok(sum) => println!("the sum is {}", sum),
            Err(SumFileError::Io(err)) => {
                // handle io::Error
            }
            Err(SumFileError::Parse(err)) => {
                // handle ParseIntError
            }
        }
    }
}
