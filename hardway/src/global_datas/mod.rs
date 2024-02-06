
mod mutable_globals;
mod once_cells;
mod lazy_statics;
mod rbatis;
mod once_lock;

#[test]
fn it_works(){

    println!("log level: {}",LOG_LEVEL);

}

// 必须明确给出类型 不能指望类型推断 
// 静态变量不一定是全局可见的   
static LOG_LEVEL: u8 = 0;

