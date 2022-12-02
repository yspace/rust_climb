pub fn run() {

    std_array::main();

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

mod std_array{
    
    #[derive(Copy, Clone, Debug)]
    struct Size(   u32) ;

    #[derive(Debug, Default)]
struct Location{ 
    x: f32, y: f32 , z: f32,
}

    pub fn main() {
        let names = ["qing","lilei"];
        let zeros = [0,5];

        let sizes = [Size(0); 5] ;

        dbg!(sizes) ;

        let [n1, n2] = names ;


        let locations = [Location::default(), Location::default()];
        let locations2 = [Location::default(), Location::default()];
        let locations3 = [Location::default(), Location::default()];
        let [loc1, loc2] = locations ;
        dbg!(loc1);
        // dbg!(locations);

        let [l1,..] = locations2 ;
        let [ .. , last_loc] = locations2 ;


        let more_locations = <[Location;3]>::default();
    }


}