mod repetitions ;
mod hygienes ;

mod pub_macros ;

mod ttm ; // token tree muncher 标记树吞噬器
mod internal_rules ;
mod tail_sep ;
pub mod declaratives;

 


use repetitions::*;

pub fn main() {
    println!("rust macros ") ;
    // expr!(println!("hello i am a expression")) ; // NOTE: 宏声明 需先于宏使用 
    run() ;

    //
    // sum!(1,2) ;

    // pub_macros::my_macros2!() ;
    // path_macros::my_macros!() ;
    //  my_macros2!() ;
}

macro_rules! my_expr{
    ($ex: expr) =>{
        $ex
    };

    ($ex: expr,$ex2: expr) =>{
        $ex
        $ex2
    };
    
}

macro_rules! m_struct{
    ($s : ident) => {
        pub struct $s{

        };
    };
}

macro_rules! m_types{
   // 这几个类型不是互斥的 前后顺序会影响结果的
    ($a: block) => {
        println!("block") ;
    };
    ($a: pat) => {
        println!("pat") ;
    };
    ($a: path) => {
        println!("path") ;
    };
    ($a: ty) => {
        println!("ty") ;
    };
    ($a: item) => {
        println!("item") ;
    };
    ($a: meta) => {
        println!("meta") ;
    };
    ($a: lifetime) => {
        println!("lifetime") ;
    };
    ($a: vis) => {
        println!("visability") ;
    };
    ($a: ident) =>{
        println!("stmt") ;
    };
    ($a: expr) => {
        println!("expr") ;
    };
        ($a: tt) => {
            println!("tt") ;
        };
}

macro_rules! literal_tokens{
    // 全匹配
 (some_str) => {
     println!("some literal string") ;
 }
}

macro_rules! space_ignore {
    ( Y E S) => {
        println!("yes !") ;
    }
}

fn run(){
    // 这里使用宏就不会报错了
        my_expr!(println!("hello i am a expression")) ;

        println!("{}", my_expr!(1+2));

        m_types!(yes) ;
        m_types!(2) ;
        m_types!( std::ptr::null_mut);
        m_types!(i64) ;
        m_types!(Some(1)) ;
        // m_types!( ‘static) ;

        literal_tokens!(some_str) ;

        space_ignore!(Y E    S) ;

}