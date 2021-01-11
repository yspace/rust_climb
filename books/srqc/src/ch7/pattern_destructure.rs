pub fn main() {
    let tuple = (1_i32, false, 3f32);
    let (head, center, tail) = tuple;
    println!("{} {} {} ", head, center, tail);

    // 解构结构
    destructure_struct();
    learn_match();
    under_score();

    // match表达式 作为表达式 从各个分支中返回一个值
    match_expr();
    value_match() ;
    range_match() ;

    match_guards() ;

    var_binding() ;
    ref_mut_match() ;

    fn_params() ;
}

fn destructure_struct() {
    struct T1(i32, char);
    struct T2 {
        item1: T1,
        item2: bool,
    }

    //
    let x = T2 {
        item1: T1(0, 'A'),
        item2: false,
    };

    let T2 {
        item1: T1(value1, value2),
        item2: value3,
    } = x;
    println!("{} {} {}", value1, value2, value3);
}

fn learn_match() {
    enum Direction {
        East,
        West,
        South,
        North,
    }

    fn print(x: Direction) {
        match x {
            Direction::East => {
                println!("east");
            }
            Direction::North => {
                println!("north");
            }
            Direction::South => {
                println!("south");
            }
            Direction::West => {
                println!("west");
            }
        }
    }

    //
    let x = Direction::East;
    print(x);

    let x = Direction::East;
    // 部分匹配 其余忽略
    match x {
        Direction::East => {
            println!("Ease");
        }
        Direction::West => {
            println!("West");
        }
        Direction::South => {
            println!("South");
        }
        _ => {
            println!("Others");
        }
    }

    // 在上游的一个库中对enum增加成员，是一个破坏兼容性的改动。
    // 非穷尽 属性 ; 好像不管用呀 要求match中必须出现下划线 _ => {}
    #[non_exhaustive]
    pub enum Error {
        NotFound,
        PermissionDenied,
        ConnectionRefused,
    }
    let e = Error::ConnectionRefused;
    match e {
        Error::ConnectionRefused => {}
        Error::NotFound => {}
        Error::PermissionDenied => {}
    }
}

struct P(f32, f32, f32);
fn under_score() {
    // let_=x；和let_y=x；具有不一样的意义。
    /*
        let_=x；和let_y=x；具有不一样的意义。这一点在后面的“析构函
    数”部分还会继续强调。如果变量x是非Copy类型，let_=x；的意思是“忽
    略绑定”，此时会直接调用x的析构函数，我们不能在后面使用下划线_
    读取这个变量的内容；而let_y=x；的意思是“所有权转移”，_y是一个正
    常的变量名，x的所有权转移到了_y上，_y在后面可以继续使用。

    下划线在Rust里面用处很多，比如：在match表达式中表示“其他分
    支”，在模式中作为占位符，还可以在类型中做占位符，在整数和小数
    字面量中做连接符，等等
    */
    //    struct P(f32, f32 , f32) ;
    fn calc(arg: P) -> f32 {
        let P(x, _, y) = arg;
        x * x + y * y
    }

    mod v2 {
        // 因为函数参数本身就
        //具备“模式解构”功能，我们可以直接在参数中完成解构：这点跟js有点类似

        use super::*;

        // 参数类型是 P,参数本身是一个模式,解构之后,变量x、y分别绑定了第一个和第三个成员
        pub fn calc(P(x, _, y): P) -> f32 {
            x * x + y * y
        }
    }
    //
    let t = P(1.0, 2.0, 3.0);
    println!("result: {}", calc(t));
    let t2 = P(1.0, 2.0, 3.0);
    println!("result: {}", v2::calc(t2));

    // 模式占位
    let x = (1, 2, 3);
    let (a, _, _) = x;
    println!("a: {}", a);
    let x = (1, 2, 3);
    let (a, ..) = x;
    println!("a: {}", a);
    let x = (1, 2, 3);
    let (a, .., b) = x;
    println!("a: {}, b: {}", a, b);

    // 调用析构函数
    struct Foo;
    impl Drop for Foo {
        fn drop(&mut self) {
            println!("Dropping the instance of Foo ");
        }
    }

    let _ = Foo ; // “忽略绑定”，此时会直接调用x的析构函数，我们不能在后面使用下划线_
    // 读取这个变量的内容
    println!("看上面的drop是不是 即刻生效了！") ;
}

fn match_expr() {
    enum Direction {
        East,
        West,
        South,
        North,
    }
    fn direction_to_int(x: Direction) -> i32 {
        match x {
            Direction::West => 20,
            Direction::East => 10,
            Direction::South => 30,
            Direction::North => 40,
        }
    }
    // run
    let x = Direction::East;
    let s = direction_to_int(x);
    println!("{}", s);
}


fn value_match(){
    fn category(x: i32) {
        match x {
            -1 => println!("negative") ,
            0 => println!("zero") ,
            1 => println!("positive") ,
            _ => println!("error") ,
        }
    }

    // 可以用竖线来匹配多个条件
    fn category2(x: i32) {
        match x {
            -1 | 1 => println!("true"),
            0  => println!("false"),
            _  => println!("error"),
        }
    }

    // 
    let x = 1 ;
    category(x) ;
    category2(x) ;
}

fn range_match(){
    let x = 'x' ;
    match x {
        'a' ..= 'z' => println!("lowercase") ,
        'A' ..= 'Z' => println!("lowercase") ,
        _ => println!("Something else ") ,
    }
}

fn  match_guards() {
    enum OptionalInt {
        Value(i32) ,
        Missing ,
    }

    let x = OptionalInt::Value(5) ;
    match x{
        OptionalInt::Value(i) if i > 5 => println!("got an integer bigger than five") ,
        OptionalInt::Value(..) => println!("Got an int") ,
        OptionalInt::Missing => println!("No such luck! ") ,
    }

    // 对数学支持并非完美的
    let x = 10;
    match x {
        i if i > 5 => println!("bigger than five"),
        i if i <= 5 => println!("small or equal to five"),
        _ =>unreachable!(),
    }

    // 重叠
    fn intersect(arg: i32) {
        // 如果我们进行匹配的值同时符合好几条分支，那么总会执行第一条
// 匹配成功的分支，忽略其他分支
        match arg {
            i if i< 0 => println!("case 1") ,
            i if i < 10 => println!("case 2") ,
            i if i * i < 1000 => println!("case 3") ,
            _ => println!("default case") ,
        }
    }
    //
    let x = 1 ;
    intersect(x) ;
}

fn var_binding() {
    let x = 1 ;
    // @符号绑定变量。@符号前面是新声明的变量，后面
// 是需要匹配的模式
    match x {
        e @ 1 ..= 5 => println!("got a range element {}" , e ) ,
        _ => println!("anything") ,
    }

    mod deep_match{
        #![feature(exclusive_range_pattern)]

        fn deep_match(v: Option<Option<i32>>) -> Option<i32> {
            match v {
                // r 绑定到的是第一层 Option 内部,r 的类型是 Option<i32>
                // 与这种写法含义不一样：Some(Some(r)) if (1..10).contains(r)
                // Some(r @ Some(1..10)) => r,  // 这是实验性特性 
                _ => None,
            }
        }
        pub fn main() {
            let x = Some(Some(5));
            println!("{:?}", deep_match(x));
            let y = Some(Some(100));
            println!("{:?}", deep_match(y));
        }
    }
    // 
    deep_match::main() ;

    // 如果在使用@的同时使用|，需要保证在每个条件上都绑定这个名字
    let x = 5 ;
    match x {
       // e @ 1.. 5 | e @ 8..10 => println!("got a range element: {}" , 3) ,
        _ => println!("anything") ,
    }
}

fn ref_mut_match(){
    /**
    注意：ref是“模式”的一部分，它只能出现在赋值号左边，而&符号
是借用运算符，是表达式的一部分，它只能出现在赋值号右边

mut关键字也可以用于模式绑定中。mut关键字和ref关键字一样，
是“模式”的一部分。Rust中，所有的变量绑定默认都是“不可更改”的。
只有使用了mut修饰的变量绑定才有修改数据的能力
    */
    
    /**
    let mut x: &mut i32;
    以上两处的mut含义是不同的。第1处mut，代表这个变量x本身可
变，因此它能够重新绑定到另外一个变量上去，具体到这个示例来说，
就是指针的指向可以变化。第2处mut，修饰的是指针，代表这个指针对
于所指向的内存具有修改能力，因此我们可以用*x=1；这样的语句，改
变它所指向的内存的值。
    
    */
    
    let x = 5_i32 ;
    match x {
        // 此时 r 的类型是 `&i32`
        // 之所以在某些时候需要使用ref，是因为模式匹配的时候有可能发生
// 变量的所有权转移，使用ref就是为了避免出现所有权转移。
        ref r => println!("Got a reference to {}",r) ,
    }

}

fn type_info(){
    fn type_id(_:()) {}

    let ref _x = 5_i32 ;
    // type_id(x) ; // 编译器会告诉我们x的信息的 故意传错类型

    /*
    
    #![feature(core_intrinsics)]
    fn print_type_name<T>(_arg: &T) {
        unsafe {
            println!("{}", std::intrinsics::type_name::<T>());
        }
    }
    fn main() {
        let ref x = 5_i32;
        print_type_name(&x);
    }

    */
}

fn fn_params(){
    struct T{
        item1: char ,
        item2: bool ,
    }

    fn test(T{item1: arg1, item2: arg2}: T) {
        println!("{}, {}", arg1, arg2) ;
    }

    //
    let x = T{
        item1: 'A' ,
        item2: false ,
    } ;
    test(x) ;
}