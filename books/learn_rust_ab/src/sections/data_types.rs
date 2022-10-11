pub fn run() {
    declare_var();
    integers();
    floats();

    number_separator();
    booleans();

    characters();
}

fn declare_var() {
    let company_string = "TutorialsPoint"; // string type
    let rating_float = 4.5; // float type
    let is_growing_boolean = true; // boolean type
    let icon_char = '♥'; //unicode character type

    println!("company name is:{}", company_string);
    println!("company rating on 5 is:{}", rating_float);
    println!("company is growing :{}", is_growing_boolean);
    println!("company icon is:{}", icon_char);
}
// The size of an integer can be arch. This means the size of the data type will be derived from the architecture of the machine.
// An arch integer is primarily used when indexing some sort of collection.
fn integers() {
    let result = 10; // i32 by default
    let age: u32 = 20;
    let sum: i32 = 5 - 15;
    let mark: isize = 10;
    let count: usize = 30;
    println!("result value is {}", result);
    println!("sum is {} and age is {}", sum, age);
    println!("mark is {} and count is {}", mark, count);
    /*
            Integer Range

    Each signed variant can store numbers from -(2^(n-1) to 2^(n-1) -1, where n is the number of bits that variant uses. For example, i8 can store numbers from -(2^7) to 2^7 -1 − here we replaced n with 8.

    Each unsigned variant can store numbers from 0 to (2^n)-1. For example, u8 can store numbers from 0 to (2^8)-1, which is equal to 0 to 255.
            */

    // ## integer overflow

    let age: u8 = 255;

    // 0 to 255 only allowed for u8
    // let weight: u8 = 256; //overflow value is 0
    // let height: u8 = 257; //overflow value is 1
    // let score: u8 = 258; //overflow value is 2

    // println!("age is {} ", age);
    // println!("weight is {}", weight);
    // println!("height is {}", height);
    // println!("score is {}", score);
}

// Float data type in Rust can be classified as f32 and f64.
fn floats() {
    let result = 10.00; //f64 by default
    let interest: f32 = 8.35;
    let cost: f64 = 15000.600; //double precision

    println!("result value is {}", result);
    println!("interest is {}", interest);
    println!("cost is {}", cost);
}

// For easy readability of large numbers, we can use a visual separator _ underscore to separate digits.
fn number_separator() {
    let float_with_separator = 11_000.555_001;
    println!("float value {}", float_with_separator);

    let int_with_separator = 50_000;
    println!("int value {}", int_with_separator);
}

fn booleans() {
    let isfun: bool = true;
    println!("Is Rust Programming Fun ? {}", isfun);
}

// Rust’s char type represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.
// Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.
// @ see https://doc.rust-lang.org/std/primitive.char.html
fn characters() {
    let special_character = '@'; //default
    let alphabet: char = 'A';
    let emoji: char = '😁';

    println!("special character is {}", special_character);
    println!("alphabet is {}", alphabet);
    println!("emoji is {}", emoji);

    // 转换 std::char::from_u32 ， as操作符把字符型转换成整型，只有u8的整型可以转换成字符型
    // std::convert::TryFrom ,char::try_from(1023) ;
    // use std::convert::TryFrom;
    let c = char::try_from(100u8);
    println!("try from {}", c.unwrap_or_default());

    let c: char = 'a';
    let is_match = match c {
        '\0'..='\u{D7FF}' => false,
        '\u{E000}'..='\u{10FFFF}' => true,
    };
    println!("is match {}", is_match) ;

    println!("max: {}", std::char::MAX as usize) ;
}
