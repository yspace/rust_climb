// while语句是带条件判断的循环语句
// 同循环语句一样 while语句中也可以使用continue和break来控制循环流程

pub fn main(){
    let mut n = 1 ;

    while n < 101 {
        if n & 15 == 0 {
            println!("fizzbuzz" );
        }else if n & 3 == 0 {
            println!("fizz") ;
        }else if n % 5 == 0 {
            println!("buzz") ;
        } else{
            println!("{}", n) ;
        }

        n += 1 ;
    }
}

