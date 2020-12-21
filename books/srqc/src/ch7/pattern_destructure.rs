
pub fn main(){
    let tuple = (1_i32, false, 3f32) ;
    let (head, center, tail) = tuple ;
    println!("{} {} {} ",head, center, tail );

    // 解构结构
    destructure_struct();
    learn_match() ;
    under_score() ;

    // match表达式 作为表达式 从各个分支中返回一个值
    match_expr() ;
}

fn destructure_struct(){
    struct T1(i32, char);
    struct T2{
        item1: T1,
        item2: bool,
    }

    //
    let x = T2{
        item1: T1(0, 'A'),
        item2: false,
    };

    let T2{
        item1: T1(value1, value2),
        item2: value3,
    } = x ;
    println!("{} {} {}", value1, value2, value3) ;
}

fn learn_match(){
    enum Direction{
        East,
        West,
        South,
        North,
    }

    fn print(x: Direction){
        match x {
            Direction::East => {
                println!("east");
            },
            Direction::North=> {
                println!("north");
            },
            Direction::South => {
                println!("south");
            },
            Direction::West => {
                println!("west") ;
            }
        }


    }

    //
    let x = Direction::East ;
    print(x) ;

    let x = Direction::East ;
    // 部分匹配 其余忽略
    match x {
        Direction::East => {
            println!("Ease") ;
        }
        Direction::West => {
            println!("West");
        }
        Direction::South => {
            println!("South") ;
        }
        _ => {
            println!("Others") ;
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
    let e = Error::ConnectionRefused ;
    match e {
        Error::ConnectionRefused => {

        },
        Error::NotFound => {

        },
        Error::PermissionDenied => {

        }
    }
}

struct P(f32, f32 , f32) ;
fn under_score(){
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
    fn calc(arg: P) -> f32{
        let P(x, _, y) = arg ;
        x*x + y*y
    }

    mod v2 {
        // 因为函数参数本身就
        //具备“模式解构”功能，我们可以直接在参数中完成解构：这点跟js有点类似

         use super::* ;

        // 参数类型是 P,参数本身是一个模式,解构之后,变量x、y分别绑定了第一个和第三个成员
        pub fn calc(P(x, _, y): P) -> f32 {
            x*x + y*y
        }
    }
    //
    let t = P(1.0, 2.0, 3.0) ;
    println!("result: {}", calc(t)) ;
    let t2 = P(1.0, 2.0, 3.0) ;
    println!("result: {}", v2::calc(t2)) ;

    // 模式占位
    let x = (1,2,3) ;
    let (a, _, _) = x ;
    println!("a: {}", a) ;
    let x = (1,2,3) ;
    let (a , ..) = x ;
    println!("a: {}", a) ;
    let x = (1,2,3) ;
    let (a, .., b) = x ;
    println!("a: {}, b: {}", a, b ) ;
}

fn match_expr(){
    enum Direction {
        East, West, South, North
    }
    fn direction_to_int(x: Direction) -> i32
    {
        match x {
            Direction::West  => 20,
            Direction::East  => 10,
            Direction::South => 30,
            Direction::North => 40,
        }
    }
    // run
        let x = Direction::East;
        let s = direction_to_int(x);
        println!("{}", s);
}