// pub type Action = fn(&str);
pub type Action = fn();

pub type Action2 = Box< FnMut()> ;

pub fn call_it(act: Action) ->() {
    act();
}

pub fn call_it2(mut act: Action2) ->() {
    
    println!("==== begin call_it2 ===== \n");
    act();
}

// https://medium.com/swlh/understanding-closures-in-rust-21f286ed1759
// 可以做编译期类型检测
fn is_fn <A, R>(_x: fn(A) -> R) {}
fn is_Fn <A, R, F: Fn(A) -> R> (_x: &F) {}
fn is_FnMut <A, R, F: FnMut(A) -> R> (_x: &F) {}
fn is_FnOnce <A, R, F: FnOnce(A) -> R> (_x: &F) {}


pub fn is_Action(_f: Action) {}