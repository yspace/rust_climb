use whale::util::runner::Runner;
use crate::cli::CliRunner;

pub fn main(){
    member_methods() ;
    self_is_box_type() ;

    static_method() ;

    add_methods_for_others() ;

    fully_qualified_syntax();

    we_are_same() ;

    generic_constraint();

    derive_attribute() ;
    some_common_traits();

    trait_for_order();
//    let  mut r = &mut Runner::new()  ;
//
////    crate::cli::get_route(String::from("_r")) ;
//    let route: Option<String>   = crate::cli::get_route(String::from("_r")) ;
//    if let Some(route_string) = route {
//        println!("found it !") ;
//    }else{
//        println!("not found the -r arg")
//    }
//
//    r.run_route("hi".into()) ;
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

fn add_methods_for_others(){
    trait Double{
        fn double(&self) -> Self ;
    }

    impl Double for i32{
        fn double(&self) -> Self {
           *self * 2
        }
    }

    let x : i32 = 10.double() ;
    println!("double result：{}", x) ;
}

fn fully_qualified_syntax(){
  // UFCS（universal function
    //call syntax），也就是所谓的“通用函数调用语法”

    // 这个语法可以允许使
    // 用类似的写法精确调用任何方法，包括成员方法和静态方法。

    // ，通过小数点语法调用方法调用，有一个“隐藏着”的“取引用”步骤。

    // <T as TraitName>：：item

    trait Cook{
        fn start(&self) ;
    }

    trait Wash{
        fn start(&self) ;
    }

    struct Chef ;
    impl Cook for Chef{
        fn start(&self) {
           println!("Cook::start()");
        }
    }

    impl Wash for Chef {
        fn start(&self) {
           println!("Wash::start") ;
        }
    }

    let me = Chef ;
//    me.start() ; // 歧义

    <Cook>::start(&me) ;
    <Chef as Wash>::start(&me) ;
}

fn we_are_same(){
    struct T(usize) ;

    impl T {
        fn get1(&self) -> usize {
            self.0
        }

        fn get2(&self)->usize {
            self.0
        }
    }

    fn get3(t: &T) -> usize {
        t.0
    }

    fn check_type(_: fn(&T)-> usize) {

    }

    // ，get1、get2和get3都可以自动转成fn（&T）→usize类型
    check_type(T::get1);
    check_type(T::get2);
    check_type(get3);

}

fn generic_constraint(){
    use std::fmt::Debug;

    fn my_print<T: Debug>(x: T) {
        println!("The value is {:?}", x) ;
    }

    //
    my_print("China");
    my_print(41_i32) ;
    my_print(true) ;
    my_print(['a','b','c']) ;

    // 泛型约束的另一种写法
    mod v2{
        use std::fmt::Debug;

        pub fn my_print<T>(x: T)
        where T: Debug {
            println!("[v2] The value is {:?}.", x) ;
        }
    }
    v2::my_print(['a','b','c']);
}

// trait 继承
fn trait_extends(){
    trait Base {

    }
    trait Derived: Base{

    }

    struct T ;
    impl Derived for T {}
    impl Base for T {}
    // 实际上，在编译器的眼中，trait Derived：Base{}等同于trait Derived
    //where Self：Base{}。这两种写法没有本质上的区别，都是给Derived这
    //个trait加了一个约束条件，即实现Derived trait的具体类型 也必须满足Base trait的约束。

}

// 自动impl某些trait
fn derive_attribute(){
    // RUST 当前支持的可自动derive的trait：
    // Debug Clone Copy Hash RustcEncodable RustcDecodable PartialEq Eq
    // ParialOrd Ord Default FromPrimitive Send Sync


    #[derive(Copy, Clone, Default, Debug , Hash,
    PartialEq, Eq , PartialOrd, Ord)]
    struct Foo {
        data: i32 ,
    }


    //
    let v1 = Foo {data:  0} ;
    let v2 = v1 ;
    println!("{:?}", v2) ;
}

fn trait_alias(){
//    pub trait Service {
//        type Request;
//        type Response;
//        type Error;
//        type Future: Future<Item=Self::Response, Error=Self::Error>;
//        fn call(&self, req: Self::Request) -> Self::Future;
//    }

//    trait HttpService = Service<Request = http::Request,
//    Response = http::Response,
//    Error = http::Error>;
}

fn some_common_traits(){
    use std::fmt::{Display, Formatter, Error};

    #[derive(Debug)]
    struct T{
        field1: i32 ,
        field2: i32 ,
    }

    impl Display for T{
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            write!(f,
            "{{ fields1:{},  field2:{} }}", self.field1, self.field2
            )
        }
    }

    //
    let var  = T{
        field1: 1,
        field2: 2,
    };
    println!("{}", var) ;
    println!("{:?}", var) ;
    println!("{:#?}", var) ;
}

fn trait_for_order(){
    /*
    对于集合X中的元素a，b，c，
    ·如果a<b则一定有！（a>b）；反之，若a>b，则一定有！
    （a<b），称为反对称性。
    ·如果a<b且b<c则a<c，称为传递性。
    ·对于X中的所有元素，都存在a<b或a>b或者a==b，三者必居其一，
    称为完全性。
    如果集合X中的元素只具备上述前两条特征，则称X是“偏序”。同
    时具备以上所有特征，则称X是“全序”。

    */
    // 浮点数第三条不具备 是偏序
    let nan = std::f32::NAN;
    let x = 1.0f32;
    println!("{}", nan < x);
    println!("{}", nan > x);
    println!("{}", nan == x);

    mod _std{
//        pub trait PartialOrd<Rhs: ?Sized = Self>: PartialEq<Rhs> {
//            fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;
//            fn lt(&self, other: &Rhs) -> bool { //... }
//            fn le(&self, other: &Rhs) -> bool { //... }
//            fn gt(&self, other: &Rhs) -> bool { //... }
//            fn ge(&self, other: &Rhs) -> bool { //... }
//            }
//                pub trait Ord: Eq + PartialOrd<Self> {
//                    fn cmp(&self, other: &Self) -> Ordering;
//                }
    }

    let int_vec = [1_i32, 2, 3];
    let biggest_int = int_vec.iter().max();
//    let float_vec = [1.0_f32, 2.0, 3.0];
//    let biggest_float = float_vec.iter().max(); // NOTE: 不可求最大值
}