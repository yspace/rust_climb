// 标识符重名 
macro_rules! inc_var {
    ($i: ident) => {
        {
            let a = "any thing" ; // 宏定义中出现的标识符 跟外部传入的标识符 即便看似一个东西 但实际还是不一样的 互相独立
            // $i += 1 ;
            $i += 1 
        }
    };
}

#[test]
fn test_inc_var(){
    let (mut a , mut b) = (0,0) ;
      let c = inc_var!(a) ;
    // assert_eq!(inc_var!(a) , 1) ;
    assert_eq!(a , 1) ;
}