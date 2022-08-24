
// 不可变 借用
fn f(a: &str) {}

// 参数可变
fn f2(mut a: String) {}

// 传递所用权到函数了
fn f3(a: i32){}

// 结构体｜枚举 实现trait
fn f4(a: impl std::io::Write){}

fn f5<T>(a: T){}

