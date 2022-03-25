
// 有三种 重复操作符号（uml中 也是这样 称为重复性）： * + ?      跟正则含义一致 分别表示0—n 1-n 0｜1 次
macro_rules! sum {
    ( $($a: expr), * )=>{
        0 $(+ $a)* 
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works(){
        assert_eq!(4 , sum!(0,1, 3)) ;
        assert_eq!(0, sum!(0 )) ;
        assert_eq!(5
            , sum!(1,4)) ;
    }
}