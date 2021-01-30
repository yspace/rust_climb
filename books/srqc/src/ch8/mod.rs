
pub fn main(){
    option_map() ;

    size_of_options();
}

fn option_map(){
    let maybe_some_string = Some(String::from("hello, World!")) ;
    let maybe_some_len = maybe_some_string.map(|s|{
        s.len()
    });

    println!("str len is :{} ", maybe_some_len.unwrap()) ;
}

fn and_then(){
    fn sq(x: u32)-> Option<u32> {
        Some(x * x)
    }

    fn nope(_: u32) -> Option<u32> {
        None
    }

    assert_eq!(
Some(2).and_then(sq).and_then(sq),
Some(16)
    );
}

fn size_of_options(){
    use std::mem::size_of;

    println!("size of isize            : {}",
        size_of::<isize>() );
    println!("size of Option<isize>    : {}",
        size_of::<Option<isize>>() );
    println!("size of &isize           : {}",
        size_of::<&isize>() );
    println!("size of Box<isize>       : {}",
        size_of::<Box<isize>>() );
    println!("size of Option<&isize>     : {}",
        size_of::<Option<&isize>>() );
    println!("size of Option<Box<isize>> : {}",
        size_of::<Option<Box<isize>>>() );
    println!("size of *const isize     : {}",
        size_of::<* const isize>() );
    println!("size of Option<*const isize> : {}",
        size_of::<Option<*const isize>>() );
}