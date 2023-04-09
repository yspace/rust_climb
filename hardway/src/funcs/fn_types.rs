// @see https://stackoverflow.com/questions/48017290/what-does-boxfn-send-static-mean-in-rust
/*
Let's decompose it one-by-one.

Box
Box<T> is a pointer to heap-allocated T. We use it here because trait objects can only exist behind pointers.

Trait objects
In Box<Fn() + Send + 'static>, Fn() + Send + 'static is a trait object type. In future, it will be written Box<dyn (Fn() + Send + 'static)> to avoid confusion.

Inside dyn are restrictions to the original type. Box<T> can be coerced into Box<Fn() + Send + 'static> only when T: Fn() + Send + 'static. Therefore, although we don't know the original type, we can assume it was Fn() and Send and had 'static lifetime.

Fn()
This is a trait, just like Clone or Default. However, it uses a special syntax sugar.

Fn(A1, ..., An) is a syntax sugar for Fn<(A1, ..., An), Output=()>.
Fn(A1, ..., An) -> R is a syntax sugar for Fn<(A1, ..., An), Output=R>.
This syntax sugar also applies to the following traits: Fn, FnMut, FnOnce, and FnBox.
So what does Fn mean? T: Fn(A1, ..., An) -> R means x: T is a callable object with arguments A1, ..., An and return type R. Examples include function pointers and closures.

 */

// pub type Action = fn(&str);
// fn 是函数指针 不捕获环境变量
pub type Action = fn();

pub type Action2 = Box<FnMut()>;

pub fn call_it(act: Action) -> () {
    act();
}

pub fn call_it2(mut act: Action2) -> () {
    println!("==== begin call_it2 ===== \n");
    act();
}

fn call(f: impl Fn() -> ()) {}
fn call0(f: &dyn Fn() -> ()) {}

// https://medium.com/swlh/understanding-closures-in-rust-21f286ed1759
// 可以做编译期类型检测
fn is_fn<A, R>(_x: fn(A) -> R) {}
fn is_Fn<A, R, F: Fn(A) -> R>(_x: &F) {}
fn is_FnMut<A, R, F: FnMut(A) -> R>(_x: &F) {}
fn is_FnOnce<A, R, F: FnOnce(A) -> R>(_x: &F) {}

pub fn is_Action(_f: Action) {}

pub mod fnmut_examples {
    fn call_FnMut<F: FnMut()>(mut f: F) {
        f();
    }
    fn call_FnMut2<F>(mut f: F)
    where
        F: FnMut(),
    {
        f();
    }
    pub fn main() {
        let mut str: String = String::from("hi ");
        // let mut f = || {
        //     str.push_str("yiqing");
        // };
        let mut f2 = || {
            let s2 = &mut str;
            s2.push_str("yiqing");
        };
        // f();
        call_FnMut2(f2);
        println!("{str}");
    }
}

#[test]
fn test_fns() {
    // @see http://www.manongjc.com/detail/31-sgiejvzpuussneu.html
    // 函数指针是指向代码而不是数据的指针。它们可以像函数一样被调用。与引用一样，函数指针被假定为不为空，因此如果您想通过 FFI 传递函数指针并能够容纳空指针，请使用所需的签名设置您的类型 Option<fn()>。
    // 普通函数指针是通过强制转换普通函数或不捕获环境的闭包来获得的
    fn add_one(x: usize) -> usize {
        x + 1
    }

    let ptr: fn(usize) -> usize = add_one;
    assert_eq!(ptr(5), 6);

    let clos: fn(usize) -> usize = |x| x + 5;
    assert_eq!(clos(5), 10);
}

#[test]
fn test_unsafe_fn() {
    fn add_one(x: usize) -> usize {
        x + 1
    }

    unsafe fn add_one_unsafely(x: usize) -> usize {
        x + 1
    }

    let safe_ptr: fn(usize) -> usize = add_one;

    //ERROR:mismatched types:expected normal fn, found unsafe fn
    //let bad_ptr:fn(usize) -> usize = add_one_unsafely;

    let unsafe_ptr: unsafe fn(usize) -> usize = add_one_unsafely;
    // unsafe 函数指针可以指向安全函数 也可以指向不安全函数
    let really_safe_ptr: unsafe fn(usize) -> usize = add_one;
}

#[test]
fn test_fn_creation() {
    /*
    当bar 是函数名时，表达式bar 不是函数指针。相反，它表示唯一标识函数 bar 的不可命名类型的值。
    该值的大小为零，因为该类型已经标识了该函数。这样做的好处是“calling” 值(它实现了Fn* 特征)不需要动态调度。

这种大小为零的类型强制转换为常规函数指针。例如：
    
     */
    use std::mem;

    fn bar(x: i32) {}

    let not_bar_ptr = bar; // `not_bar_ptr` is zero-sized, uniquely identifying `bar`
    assert_eq!(mem::size_of_val(&not_bar_ptr), 0);

    let bar_ptr: fn(i32) = not_bar_ptr; // force coercion to function pointer
    assert_eq!(mem::size_of_val(&bar_ptr), mem::size_of::<usize>());

    let footgun = &bar; // this is a shared reference to the zero-sized type identifying `bar`
}


mod fn_vec{
    #[test]
    fn test_fn_vec() {
        let mut vec: Vec<Box<dyn Fn(i32) -> i32>> = Vec::new();

        vec.push(Box::new(|i| i+1));
        vec.push(Box::new(|i| i-1));
        vec.push(Box::new(|i| i*1));
        vec.push(Box::new(|i| i/1));

        for f in vec  {
            println!("result: {}", f(2_i32));
        }

        
    }
}