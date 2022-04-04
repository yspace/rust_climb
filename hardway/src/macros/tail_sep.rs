// tail separator

macro_rules! my_tuple{
    // 尾部的那个, 可有可无都可以被匹配到
    ($e: expr , $e2: expr $(,)? ) => {
        ( $e , $e2 )
    };
}



#[test]
fn test_my_tuple(){
    assert_eq!(my_tuple!(1,2),(1,2));
}