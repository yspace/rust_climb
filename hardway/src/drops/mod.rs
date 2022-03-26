
fn drop_test(){
    struct MyString(String);
    struct MyBox<T>(Box<T>);
    struct MyVec<T>(Vec<T>);

    impl Drop for MyString{
        fn drop(&mut self) {
            println!("MyString {} dropped", self.0);
        }
    }
    impl<T> Drop for MyBox<T>{
        fn drop(&mut self) {
            // println!("MyBox {} dropped", self.0);
            println!("MyBox  dropped");
        }
    }

    impl<T> Drop for MyVec<T>{
        fn drop(&mut self) {
            println!("MyVec dropped");
        }
    }

    let s1 = MyString("hello world".to_string());
    let s2 = MyString("Goodbye world".to_string());
    let arr = MyVec(vec![
        MyBox(Box::new(s1)),
        MyBox(Box::new(s2)),
        ]);
}

#[test]
fn test_drop(){
    drop_test();
}