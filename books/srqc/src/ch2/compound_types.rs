
pub fn main(){
    learn_tuple() ;

    learn_struct() ;
}

fn learn_tuple(){
let a = (1i32 , false) ;
    let b = ("a", (1i32), 2i32) ;
    // 元组类型无名称（? 可以起别名)  成员变量通过索引访问

    println!("{:?},{:?}", a, b) ;

    // NOTE 一个元素的元组 跟括号表达式
    let a = (0, ) ; //  这是元组
    let b = (0) ; // 这是括号表达式

    // 元组的访问
    // a: 模式匹配
    let p = (1i32, 2i32) ;
    let (a, b) = p ;
    println!("a=> {}, b=> {}", a, b) ;

    let x = p.0 ;
    let y = p.1 ;
    println!("x:{}, y:{}", x, y) ;

    // 单元类型 unit
    let empty: () = () ;
    // 空元组 跟空结构一样 都是占用0内存空间
    println!("size of a: {}", core::mem::size_of_val(&a)) ;
    println!("size of empty: {}", core::mem::size_of_val(&empty)) ;

    println!("size of i8 {}" , std::mem::size_of::<i8>());
    println!("size of char {}" , std::mem::size_of::<char>());
    println!("size of '()' {}" , std::mem::size_of::<()>());
}

fn learn_struct(){
    // 不同元组的是 结构体有名字 成员也都有自己的名字  类json语法

    struct Point{
        x: i32 ,
        y: i32,
    }

    let p = Point{
        x: 0 ,
        y: 0,
    } ;
    println!("Point is at {} {} ", p.x, p.y);

    // 成员名同名时的简化
    let x = 10 ;
    let y = 20 ;
    let p = Point{x,y} ;
    println!("Point is at {}, {}", p.x, p.y) ;

    // 语法糖：赋值时使用其他结构的部分成员作为“基”础
    #[derive(Debug)]
    struct Point3d{
        x: i32,
        y: i32,
        z: i32,
    }
    fn default()-> Point3d{
        Point3d{
            x:0,
            y:0,
            z:0,
        }
    }
    // ..expr 这样的语法 只能放在初始化表达式中
    let origin = Point3d{
        x: 5 ,
        ..default()
    };
let point = Point3d{z: 1, x: 2, ..default()};
    println!("origin: {:?} \n point: {:?}", origin , point) ;

    // 类似tuple struct成员内部也可以是空的
    struct Foo1 ;
    struct Foo2() ;
    struct Foo3() ;

    {
    // ### tuple struct 不需要关心成员的名称
        struct Color(i32, i32, i32) ;
        struct Point(i32, i32, i32) ;
    }

    {
        struct T1{
            v: i32
        }
        struct T2(i32);

        let v1 = T1{
            v:1
        };
        let v2 = T2(1);
        let v3 = T2{0:1} ;

        println!("{} {} {}", v1.v , v2.0, v3.0) ;
    }
    {
//        惯例： newtype idiom
        struct Inches(i32) ;
        fn f1(value: Inches){

        }
        fn f2(v: i32){

        }
        let v : i32 = 0;
//        f1(v) ; // 类型不匹配
       // 改动
        fn type_alias(){
            type I = i32 ;
            fn f1(v:I){}
            fn f2(v:i32){}

    let v: i32 = 0 ;
    f1(v) ;
    f2(v) ;
        }
    }

}

fn learn_type_recursive(){
    struct Recursive{
        data: i32 ,
        rec: Box<Recursive>,
    }
}