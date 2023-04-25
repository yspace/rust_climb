use std::panic;
// Triggers a panic without invoking the panic hook. 注意:不会调用panic hook


#[test]
fn main(){
    let result = panic::catch_unwind(||{
        println!("no panics , all is ok!");
    });

    debug_assert!(result.is_ok());

    let result = panic::catch_unwind(|| {
            panic!("oh panic occured !");
         });
    debug_assert!(result.is_err());

    println!("main thread is ok" );
}