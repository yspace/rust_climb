/*

trait Default{
    fn default() -> Self ;
}

 */

#[test]
fn test_default() {
    let zero: i32 = Default::default();
    let zero2  = i32::default();

    assert_eq!(zero, zero2);
}

