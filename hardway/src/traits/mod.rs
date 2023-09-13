mod compositions;
mod derives;
mod traits_objects;
pub mod down_castings;
pub mod blanket_traits;
pub mod static_vs_dynamic_dispatch;
pub mod boxed_traits;
pub mod scenarios;
pub mod getter_setters;
pub mod impl_and_dyn;
mod upcasting;

pub fn run(){
    let str_thing = StringThing{};
    do_run(&str_thing);

    let int_thing = IntThing{};
    do_run(&int_thing);

}




// 定义trait 可以参考标准库下的io 相关的trait ；命名惯例之类 xxxer xxxor xxxable 或者直接用动词？
trait Runner{
    fn run(&self) ;
}

// 实现

struct StringThing{}
struct IntThing{}

impl Runner for StringThing{
    fn run(&self)  {
        println!("run in string-thing");
    }
}
impl Runner for IntThing{
    fn run(&self)  {
        print!("run in int-thing");
    }
}

// 定义generic方法
fn do_run(r: &Runner) {
    r.run();
}

mod default_impls{
    pub trait Hello{
        fn hello(&self) -> String{
            String::from("world")
        }
    }
    struct MyThing{}
    impl Hello for MyThing{}

    struct MyThing2{}
    impl Hello for MyThing2{
        fn hello(&self) -> String{
            String::from("MyTing2")
        }
    }

    #[test]
    fn test_hello() {
        let h1 = MyThing{};
        let h2 = MyThing2{}; 

        let mut hellos:Vec<Box<dyn Hello>> = Vec::new();
        hellos.push(Box::new(h1));
        hellos.push(Box::new(h2)) ;

        for h in hellos {
           println!("{}",  h.hello());
        }
    }
}