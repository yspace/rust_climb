
pub fn run(){
    // 条件表达式
    let a = 7 ;
    let is_even = if a % 2 == 0 { true } else { false };
    if is_even {
        println!("{} is even", a);

    }else {
        println!("{} is odd", a);
    }

    // 块表达式： In an expression based language any block of code can return a value.
    let result_stmt = {
        1+2 ;
    };
    println!("block stmt result is {:?}", result_stmt);

    let result_expr = {
        // In Rust, a semicolon is used to turn expression into a statement.
        1+2 
    };
    println!("expression  result is {:?}", result_expr);
}