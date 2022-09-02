
#[test]
fn test_basic(){
    let arr = [1,2,3,4,5];

    let num1 = arr[0] ;
    assert_eq!(num1,1) ;

    let mut arr = [1,2,3,4,5];
    arr[1] = 10 ;
    assert_eq!(arr[1] , 10);

    // 重复表达式
    let arr2 = [1; 10] ;
    assert_eq!(arr2[2], 1) ;
}