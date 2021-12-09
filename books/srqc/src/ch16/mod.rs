use std::ops::DerefMut;



pub fn main(){
    let v1 = 1 ;
    let p = &v1; // 取引用
    let v2 = *p ; // 解引用
    println!("{}, {}", v1, v2) ;

    //
    auto_deref();
    foo1();
}

fn auto_deref(){
    let s = "hello" ;
    println!("len: {}", s.len());
    println!("len: {}", (&s).len());
    println!("len: {}", (&&&&&&&&&&s).len());
}

fn foo1(){
    use std::ops::Deref;
    // use std::ops::DerefMut;
        
    struct A{}
    impl A{
        fn foo(self: & A) {
            println!("hi A ") ;
        }
    }

    struct B{
        a: A ,
    }

    impl Deref for B {
        type Target = A;
        #[inline(always)]
        fn deref(&self) -> &A {
            &self.a 
        } 
    }

    // 间接调用内部a的方法
    let mut b = B{
        a: A{}
    };
    &b.foo() ;
}