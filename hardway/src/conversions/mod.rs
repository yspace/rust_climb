fn conversion() {
    let x = 42i32; // Integer literal with type suffix
    let y: i64 = x.into();
}


#[test]
fn it_works(){
    conversion()
}