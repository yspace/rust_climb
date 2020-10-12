mod _1 {
    pub fn say_hello() {
        println!("hello, world!");
    }

    pub fn print_0() {
        let numbers = [1, 2, 3, 4, 5];
// Rust abstracts the idea of
// iteration into yet another trait, this one called Iterator. We have to call iter here
// to turn an array into an Iterator because arrays do not automatically coerce into
// into an Iterator.
        for n in numbers.iter() {
            println!("{}", n);
        }

// trick: 通过模式匹配 故意制造错误 让编译器告诉我们类型
//    let () = numbers ;

        let numbers = [1u8, 2, 3, 4, 5];
        let numbers: [u8; 5] = [1, 2, 3, 4, 5];
    }

    pub fn print() {
        let numbers = vec![
            1, 2, 3, 4, 5];

        // 向量类型也实现了迭代器trait 但这里是编译器帮我们做了隐式调用
//        for n in numbers {
//            println!("{}", n);
//        }

        for n in numbers.iter(){
            println!("{}", n) ;
        }
    }
}
mod _2{

    pub fn print(){
        let numbers = [1,2,3,4,5] ;
        output_sequence(numbers) ;
    }
    fn output_sequence(numbers: [u8; 5]) {
        for n in numbers.iter(){
            println!("{}",n) ;
        }
    }
}

pub fn print(){
    let numbers = vec![1,2,3,4,5] ;

    output_sequence(numbers) ;
}

fn output_sequence(numbers: Vec<u8>){
    for n in numbers{
        println!("{}", n) ;
    }
}