
// #[derive(Debug, Copy, Clone,Default)]
#[derive(Debug, Default)]
struct Context {
    input_var: i32,
}

fn my_fn(param:&Context  ){
    println!("{:?}", param);
}

fn my_write_fn(param:&mut Context){
    param.input_var += 2 ;
}

#[test]
fn test_my_fn() {
    let mut param  = Context::default();

    let result = my_fn(&param);
    my_write_fn(&mut param);
    my_write_fn(&mut param);
    my_fn(&param);
}