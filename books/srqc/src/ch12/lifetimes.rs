struct T{
    member: i32 ,
}

fn test<'a>(arg: &'a T) ->&'a i32 {
    /**
    生命周期之间有重要的包含关系。如果生命周期'a比'b更长或相
等，则记为'a：'b，意思是'a至少不会比'b短，英语读做“lifetime a
outlives lifetime b”。对于借用指针类型来说，如果&'a是合法的，那么'b
作为'a的一部分，&'b也一定是合法的
    */
    &arg.member
}



pub fn main(){
    let t = T{member:0} ;
    let x = test(&t) ;
    println!("{}",x) ;

    // 
    let x = 1;
    let y = 2;
    // 让x y 生命周期相同了 
    let selected = select(&x, &y);
    println!("{}", selected);

}


fn test_v2<'a, 'b>(arg: &'a T) -> &'b i32
    where 'a:'b // 'a 比 'b 活得长
{
    &arg.member
}

// 协变
fn select<'a>(arg1: &'a i32, arg2: &'a i32) -> &'a i32 {
    if *arg1 > *arg2 {
        arg1
    } else {
        arg2
    }
}

// ## 类型的生命周期
struct Test<'a> {
    member: &'a str
}
impl<'t> Test<'t> {
    fn test<'a>(&self, s: &'a str) {

    }

}


fn ignore_lifetime_marker(){
    fn get_str(s: &String) -> &str {
        s.as_ref()
    }

    // 等价
    fn _get_str<'a>(s: &'a String) -> &'a str {
        s.as_ref()
    }
}