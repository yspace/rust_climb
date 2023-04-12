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
