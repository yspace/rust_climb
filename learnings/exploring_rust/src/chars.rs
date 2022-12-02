pub fn run() {
    let v = vec!['a', 'b', 'c', 'd','e'];
    assert_eq!(20 , v.len()*std::mem::size_of::<char>());

    let str = String::from("hello") ;
    let chars = str.chars() ;
    dbg!(chars);

    dbg!(std::char::MAX);
    dbg!(std::char::MAX as u32);
    let long_char = '\u{10FFFF}';
    dbg!(long_char);
}