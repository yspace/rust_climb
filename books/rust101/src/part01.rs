
fn sqr(i: i32) -> i32 { i * i }
fn abs(i: i32) -> i32 { if i>=0 {i} else { -i }}
fn compute_stuff(x: i32) -> i32 {
    let y = { let z = x*x; z + 14 };
    y*y
}


enum NumberOrNothing{
    Number(i32),
    Nothing,
}

fn nuber_or_default(n: NumberOrNothing, default: i32) -> i32 { 
    match n {
        Nothing => default,
        Number(n) => n ,
    }
}

// 此是重构版函数 更简洁一些 可以对比00中的实现
fn vec_min(vec: Vec<i32>)-> NumberOrNothing{
    // 此处是一个helper函数 ，也是一种名空间机制 在一个函数中可以定义其他函数
    fn min_i32(a: i32, b: i32) -> i32 {
    if a < b {
        a
    }else{
        b
    }
}
let mut min = Nothing;
     
    for el in vec {

        min = Number(
            match min {
                Nothing  => el ,
                Number(n) => min_i32(n, el) 
            }
        );
    }

    return min ;
}


use self::NumberOrNothing::{
    Number,
    Nothing,
};

fn read_vec() -> Vec<i32> {
    vec![
        18,5,7,1,9,27
    ]
}



impl NumberOrNothing {
    fn print(self)   {
        match self {
            Nothing => println!("the number is: <nothing>") ,
            Number(n) => println!("the number is: {}", n ) ,
        }
    }
    fn number_or_default(self, default: i32) -> i32 {
        match self {
            Nothing => default,
            Number(n) => n,
        }
    }
}

// Exercise 01.1: Write a function vec_sum that computes the sum of all values of a Vec<i32>.
fn vec_sum(vec: Vec<i32>) -> i32{
    let mut result =  0 ;

    for el in vec {
        result += el ;
    }

    return result ;
}
// Exercise 01.2: Write a function vec_print that takes a vector and prints all its elements.
fn vec_print(vec: Vec<i32>) -> (){
    for el in vec{
        println!("{}", el) ;
    }
}

pub fn main() {
    let vec = read_vec(); 
    let min = vec_min(vec);
    // print_number_or_nothing(min) ;
    min.print();
    // simple test the sum of a Vec<i32>
    let vec = vec![1,2,3,4,5];
    println!("sum of Vec<i32> is {}", 
      vec_sum(vec)
    ) ;

    let vec = vec![1,2,3,4,5] ;
    vec_print(vec) ;
}