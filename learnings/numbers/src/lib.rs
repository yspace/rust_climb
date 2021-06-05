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


mod _3{

    pub fn print(){
        let numbers = vec![1,2,3,4,5] ;

        output_sequence(numbers) ;
//    output_sequence(numbers) ; // 非拷贝类型在函数调用时 所有权已经转移到函数去了 后续再次使用就不行了
    }

    fn output_sequence(numbers: Vec<u8>){
        // Rust has a few different modes of passing arguments to functions.
        // • a function temporarily having access to a variable (borrowing) and
        // • having ownership of a variable.
        for n in numbers{
            println!("{}", n) ;
        }
    }
}

mod _4{

    pub fn print(){
        let vetor_numbers = vec![1,2,3,4,5] ;
        output_sequence(&vetor_numbers) ;
        let array_numbers = [1,2,3,4,5] ;
        output_sequence(&array_numbers) ;

    }

    fn output_sequence(numbers: &[u8]){
        // Idiomatic Rust takes slices as arguments in most cases where one needs only to read
        // the collection.

        //  The Rust compilation model does not allow functions to directly
        // take arguments of an unknown size.

        // 标准库中的切片类型 是用来克服数组的一些限制
        // you can have a slice which references an array or a vector and treat them the same.

        //  Note that slices convert automatically into iterators
        // just like vectors so we again do not call iter explicitly in the body of our
        // function.
        for n in numbers{
            println!("{}",n) ;
        }
    }
}

pub fn print(limit: u8){
    let numbers = generate_sequence(limit) ;
    output_sequence(&numbers) ;
}

fn generate_sequence(limit: u8) -> Vec<u8>{
    //  Mutability is a property
    //  of the variable or reference not of the object itself.
    let mut numbers = Vec::new() ;

    //  Ranges
    //  can be constructed with using the syntax start..end or start..=end.
    //  start 和 end 是可选的！
    for n in 1 ..= limit {
        numbers.push(n) ;
    }

    numbers
}

fn output_sequence(numbers: &[u8]) {
    for n in numbers{
        println!("{}",n) ;
    }
}

mod example{
    fn using_ranges(){
        let numbers = [1,2,3,4,5] ;
        let subset = &numbers[1..3] ;
    }

    fn generate_sequence(limit: u8) ->Vec<u8> {
        (1..=limit).collect()
    }
}

#[test]
fn generate_sequence_should_work(){
    let result = generate_sequence(3) ;
    assert_eq!(result, &[1,2,3]) ;
}