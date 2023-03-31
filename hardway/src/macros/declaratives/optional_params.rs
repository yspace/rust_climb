// 可选参数和函数重载都是语法糖的某种形式

trait First {
    fn name(&self) {}
}
trait Second {
    fn name(&self, _: bool) {}
}

struct Container {
    name: String,
}

impl First for Container {
    fn name(&self) {}
}
impl Second for Container {
    fn name(&self, _: bool) {}
}

// design our software to expect functionality delivered via traits
fn get_name_from_first<T: First>(t: &T) {
    t.name()
}
fn get_name_from_second<T: Second>(t: &T) {
    t.name(true)
}

#[test]
fn it_works() {
    let container = Container {
        name: "Henry".into(),
    };
    get_name_from_first(&container);
    get_name_from_second(&container);
}
