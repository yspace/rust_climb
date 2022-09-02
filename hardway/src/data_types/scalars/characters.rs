
#[test]
fn test_size(){
    let mut c = '\u{263A}';
    //c = c + 1 ;

    // assert_eq!(
    //    c,
    //     ()

    // );
        
    assert_eq!(
        std::mem::size_of::<char>(),
        4

    );
}