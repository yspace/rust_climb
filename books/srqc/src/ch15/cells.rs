use std::{rc::Rc, vec} ;
pub fn main(){


    let r1 = Rc::new(1) ;
    println!("reference count {}", Rc::strong_count(&r1)) ;

    let r2 = r1.clone() ;
    println!("ref count {}", Rc::strong_count(&r2)) ;

    //
    basic_usage_cell() ;
    using_refcell() ;
}
// 如果你需要把一个类型T封装到内部可变性类型中去，要怎样选择
// Cell和RefCell呢？原则就是，如果你只需要整体性地存入、取出T，那
// 么就选Cell。如果你需要有个可读写指针指向这个T修改它，那么就选
// RefCell。
fn basic_usage_cell(){
    use std::cell::Cell ;

    // Cell类型最有用的地方就
// 在于，它可以通过不可变引用改变内部的值

    let data : Cell<i32> = Cell::new(100) ;
    let p = &data ;
    data.set(10) ;
    println!("{}", p.get()) ;

    p.set(20) ;
    println!("{:?}", data) ;
}


fn using_refcell(){
//     // 在函数的签名中，borrow
// 方法和borrow_mut方法返回的并不是&T和&mut T，而是Ref<T>和
// RefMut<T>。它们实际上是一种“智能指针”，完全可以当作&T和&mut
// T的等价物来使用。标准库之所以返回这样的类型，而不是原生指针类
// 型，是因为它需要这个指针生命周期结束的时候做点事情，需要自定义
// 类型包装一下，加上自定义析构函数。
    use std::cell::RefCell ;

    let shared_vec: RefCell<Vec<isize>> = RefCell::new(vec![1,2,3]) ;
    let shared1 = &shared_vec ;
    let shared2 = &shared_vec ;

    shared1.borrow_mut().push(4) ;
    println!("{:?}", shared_vec.borrow()) ;

    shared2.borrow_mut().push(5) ;
    println!("{:?}", shared_vec.borrow()) ;
}

mod _my{
    struct CellV2<T> {
        value: T
    }

    impl<T> CellV2<T> {
        fn new(v: T) -> Self where T: Copy {
            CellV2 { value: v}
        }
        fn set(&self, v: T) where T: Copy {
            unsafe {
                let p = &(self.value) as *const T as *mut T;//此处实际上引入了未定义行为
                *p = v;
            }
        }
        fn get(&self) -> T where T: Copy {
            self.value
        }
    }
}