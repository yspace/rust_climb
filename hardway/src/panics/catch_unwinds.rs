use std::panic;
// Triggers a panic without invoking the panic hook. 注意:不会调用panic hook

#[test]
fn main() {
    let result = panic::catch_unwind(|| {
        println!("no panics , all is ok!");
    });

    debug_assert!(result.is_ok());

    let result = panic::catch_unwind(|| {
        panic!("oh panic occured !");
    });
    debug_assert!(result.is_err());

    println!("main thread is ok");
}

mod _1 {
    fn divide(a: i64, b: i64) -> i64 {
        if b == 0 {
            panic!("Cowardly refusing to divide by zero!");
        }
        a / b
    }

    fn divide_recover(a: i64, b: i64, default: i64) -> i64 {
        let result = std::panic::catch_unwind(|| divide(a, b));
        match result {
            Ok(x) => x,
            Err(_) => default,
        }
    }

    #[test]
    fn test_divide_recover(){
        let result = divide_recover(0, 0, 42);
        println!("result = {}", result);
        assert_eq!(result, 42);
    }
}
