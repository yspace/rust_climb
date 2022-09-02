#[test]
fn test_size(){
    let mut tup = (10, 'a', 1.0) ;
    assert_eq!(10, tup.0);

    tup.0 = 20 ;
    assert_eq!(20, tup.0   );

}