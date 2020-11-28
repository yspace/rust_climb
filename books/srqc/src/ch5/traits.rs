pub fn main(){
    member_methods() ;
    self_is_box_type() ;

    static_method() ;
}

fn member_methods(){
    trait Shape{
        // 。self参数同样也可以指定
        // 类型，当然这个类型是有限制的，必须是包装在Self类型之上的类型。
        // 对于第一个self参数，常见的类型有self：Self、self：&Self、self：&mut
        // Self等类型
        fn area(&self) -> f64 ; // 等同 fn area(self: &Self)->f64 ;
    }

    trait T{
        fn method1(self: Self) ;
        fn method2(self: &Self) ;
        fn method3(self: &mut Self) ;
    }
    // 同下面
    trait T2{
        fn method1(self);
        fn method2(&self);
        fn method3(&mut self);
    }

    // 实现trait
    struct Circle{
        radius: f64 ,
    }

    impl Shape for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }
    // 直接增加成员函数
    // 看作是为Circle类型impl了一个匿名的trait
    impl Circle{
        // “内在方法”（inherent methods）

        /// get_radius return the circle 's radius
        fn get_radius(&self) -> f64{
            self.radius
        }
    }

    // 调用下
    let c = Circle{
        radius: 2f64,
    };
    println!("The area is {}", c.area()) ;


}

fn self_is_box_type(){
    trait Shape{
        fn area(self: Box<Self>) -> f64 ;
    }

    struct Circle{
        radius: f64 ,
    }

    impl Shape for Circle{
        // Self 类型就是 Circle
        // self 的类型是 Box<Self>,即 Box<Circle>
        fn area(self: Box<Self>) -> f64 {
            use std::f64::consts::PI;
            PI* self.radius * self.radius
        }
    }

    //
    let c = Circle{
        radius: 2f64,
    };
    // 编译错误
//    c.area() ;

    let b = Box::new(Circle{
        radius: 4f64 ,
    });

    b.area() ;
}

fn impl_trait(){
    trait Shape{
        fn area(&self) -> f64 ;
    }

    trait  Round{
        fn get_radius(&self) -> f64 ;
    }

    struct Circle{
        radius: f64,
    }
    impl Round for Circle{
        fn get_radius(&self) -> f64 {
           self.radius
        }
    }

    // NOTE 此处是为trait 实现trait
    // impl Shape for Round和impl<T：Round>Shape for
    // T是不一样的。在前一种写法中，self是&Round类型，它是一个trait
    // object，是胖指针。而在后一种写法中，self是&T类型，是具体类型。
    // 前一种写法是为trait object增加一个成员方法，而后一种写法是为所有
    // 的满足T：Round的具体类型增加一个成员方法。
    impl Shape for Round {
        // ，self是&Round类型，它是一个trait object，是胖指针
        fn area(&self) -> f64 {
           std::f64::consts::PI * self.get_radius()* self.get_radius()
        }
    }

    //
    let c = Circle{
        radius: 2f64 ,
    } ;
    // 编译错误
    //c.area() ;

    let b = Box::new(
        Circle{
            radius: 4f64
        }
    ) as Box<Round> ;

    b.area() ;
}

fn static_method(){
    struct T(i32) ;
    impl T {
        //  这是一个静态方法 ,  因为其名称不是self
        fn func(this: &Self){
            println!("value: {}", this.0) ;
        }
    }

    let x = T(42);
    T::func(&x) ;
//    x.func() ;
}

fn static_func(){
    pub trait Default {
        fn default() -> Self;
    }
}