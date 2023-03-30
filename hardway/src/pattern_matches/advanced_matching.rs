fn print_type_of<T>(msg: &str, _: &T) {
    println!("type of {}: {}", msg, std::any::type_name::<T>())
}

macro_rules! print_type {
    ( $x:expr ) => {
        print_type_of(stringify!($x), &$x)
    };
}
fn test() {
    let x = 1;
    print_type!(x);
    println!("x: {}", x);

    let &y = &1;
    print_type!(y);
    println!("y: {}", y);
}

pub fn run() {
    test();
}

fn with_guard(val: i32) {
    // 同一个match块里只能作用在同类型 不能做generically match
    match val {
        v if v == 1 => println!("equalt to one"),
        v if v < 10 => println!("less than 10"),
        _ => println!("others"),
    }
}

enum DistinctTypes {
    Name(String),
    Count(i32),
}
fn match_enum_types(enum_types: &DistinctTypes) {
    match enum_types {
        DistinctTypes::Name(name) => println!("name={name}"),
        DistinctTypes::Count(count) => println!("count={count}"),
    }
}

enum Colour {
    Black,
    Red,

    Blue,
}

struct Product {
    name: String,
    colour: Colour,
}

fn match_on_product(product: &Product) {
    match product {
        Product { .. } => println!("this is product"),
    }
    match product {
        Product {
            name,
            colour: Colour::Black,
        } => println!("this is black product"),
        Product { name, colour: _ } => println!("this is not black product"),
    }
}
#[test]
fn test_match_on_product() {
    let product = Product {
        name: "t-shirt".to_owned(),
        colour: Colour::Black,
    };

    match_on_product(&product);
}

mod match_on_result {
    // from clap<code like a pro in rust>
    fn write_to_file() -> std::io::Result<()> {
        use std::fs::File;
        use std::io::prelude::*;
        // FIXME ？可以用于Option 么
        let mut file = File::create("some_file_name")?;
        file.write_all(b" contents")?;
        Ok(())
    }

    fn try_to_write_to_file() {
        match write_to_file() {
            Ok(()) => println!("Write ok"),
            Err(err) => println!("Write error: {}", err.to_string()),
        }
    }

    fn write_to_file_without_result() {
        use std::fs::File;
        use std::io::prelude::*;
        let create_result = File::create("filename");
        match create_result {
            Ok(mut file) => match file.write_all(b"File contents") {
                Err(err) => {
                    println!("There was an error writing: {}", err.to_string())
                }
                _ => (),
            },
            Err(err) => println!("There was an error opening the file: {}", err.to_string()),
        }
    }
}
