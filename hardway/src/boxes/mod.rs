use std::fmt::Display;


pub fn main() {
    let c = math();
    println!("{c}");

    let mut l = list_Xxx(); 

    l.push(Box::new('y'));
    l.push(Box::new(true));

    for item in l.iter() {
        println!("{}", item);
    }

    // 
    save_space_with_box();
}

fn math() -> i32 {
    let a = 123_i32 ;
    let b = Box::new(123_i32);

    a+ (*b)
}

struct Large ;

fn build() -> Box<Large> {
    // local variable
    let a = Large ;
    Box::new(a) 
    // after call Box::new it will be moved|sent to the heap ;

   
    
}

// github.com/Idekubaguscom/aptos-core

// Creating a list of trait objects(or any ?Sized object)

// 在github上用关键字：`FuzzTargetImpl` 搜索 会看到一些好的代码片段

fn list_Xxx() -> Vec<Box<dyn Display>> {
    vec![
       Box::new( 1),
        Box::new("hi"),
    ]
}

enum Selection{
    Small,
    Med ,
    Large,
    // Other([u8; 1024]) ,
    Other(Box<[u8; 1024]>) ,
}
enum Selection0{
    Small,
    Med ,
    Large,
    Other([u8; 1024]) ,
}

fn save_space_with_box(){
    println!("orig: {}",std::mem::size_of::<Selection0>());
    println!("with box: {}",std::mem::size_of::<Selection>());
}

// Box<T> 是一些集合类型的基础 比如Vet<T> 和 String
// Box<T> 可用来创建递归结构