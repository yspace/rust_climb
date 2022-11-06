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

fn call(f: impl Fn()->()){

}
fn call0(f: &dyn Fn()->()){
    
}

// https://medium.com/swlh/understanding-closures-in-rust-21f286ed1759
// 可以做编译期类型检测
fn is_fn <A, R>(_x: fn(A) -> R) {}
fn is_Fn <A, R, F: Fn(A) -> R> (_x: &F) {}
fn is_FnMut <A, R, F: FnMut(A) -> R> (_x: &F) {}
fn is_FnOnce <A, R, F: FnOnce(A) -> R> (_x: &F) {}


pub fn is_Action(_f: Action) {}


pub mod fnmut_examples {
    fn call_FnMut<F: FnMut()>(mut f: F){
        f();
    }
    fn call_FnMut2<F>(mut f: F)
    where F: FnMut()
    {
        f();
    }
    pub fn main(){
        let mut str: String = String::from("hi ");
        // let mut f = || { 
        //     str.push_str("yiqing");
        // };
        let mut f2 = || { 
            let s2 = &mut str ;
            s2.push_str("yiqing");
        };
        // f();
        call_FnMut2(f2);
        println!("{str}");
    }
}