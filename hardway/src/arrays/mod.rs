pub fn run() {
    let numbers = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    println!("{}", numbers[0]);

    for i in 0..numbers.len() {
        println!("{}", numbers[i]);
    }   

    for n in numbers.iter() {
        println!("{}", n);
    }

    // default values
    let numbers = [2; 10] ;
    print_numbers(&numbers);
}

fn print_numbers(numbers: &[i32]) {
    for n in numbers.iter() {
        println!("{}", n);
    }
}