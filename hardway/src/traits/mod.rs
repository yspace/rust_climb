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