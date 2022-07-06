pub fn main() {
    println!("hello control_flows");
    let n = 10 ;
    fib_loop(n);
    fib_while(n) ;
    fib_for(n) ;
}

fn fib_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;

    loop {
        let c = a + b;
        a = b;
        b = c;
        i += 1;

        println!("next var is {}", b);

        if i >= n {
            break;
        }
    }
}

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);
    while i < n{
        let c = a + b ;
        a = b ; 
        b = c ;

        i += 1 ;

        println!("next var is {}", b);
    }
}


fn fib_for(n: u8) {
    let (mut a, mut b ) = (1, 1 );

    for _i in 2..n {
        let c = a + b ;
        a = b ; 
        b = c ;
        println!("next var is {}", b);

    }
}

//
struct MyStruct ;
// impl IntoIterator for MyStruct {
//     type Item = i32;
//     type IntoIter = MyIntoIter;

//     fn into_iter(self) -> Self::IntoIter {
//         MyIntoIter {
//             next_num: 1,
//         }
//     }
// }