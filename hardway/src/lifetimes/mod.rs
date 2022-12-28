pub fn main() {
    // 函数中的每个引用型参数都有生命周期 必要时才需要程序员明确给出

    let some_int_val = 10 ;
    let result_ref = get_int_ref(&some_int_val);
    println!("result: {}", result_ref);

    let r = get_int_ref(&SOME_INT);


    vectors::run() ;
    lifetimes_with_generics::run() ;
}

// 静态生命周期
// 也只是一种生命周期 不过会持续存活于整个程序运行期间
// 常量是天然的静态生命周期的变量 当然 static 型也是静态生命周期的
const SOME_INT :i32 = 10 ;

// 当函数返回引用时 就需要考虑生命周期问题了 
// 把函数想成`黑盒`  调用函数时 可能需要传递若干参数｜或不用传递 返回引用需要考虑函数的返回值跟入参的生命周期对应关系
fn get_int_ref<'a>(param1: &'a i32) ->&'a i32{
    param1
}
fn get_int_ref1<'a>(param1: &'a i32) ->&i32{
    param1
}

fn get_int_ref0(param1: &i32) ->&i32{
    param1
}

fn get_max0<'a,'b>(p1: &'a i32, p2: &'b i32) ->&'a i32{
    p1
}

// 此语法中 'b: 'a 表示b至少活的跟a一样长 术语：lifetime subtyping
fn get_max1<'a,'b:'a>(p1: &'a i32, p2: &'b i32) ->&'a i32{
    if p1 > p2 {
        p1
    }else{
        p2
    }
}
// 对比上面的版本 此版表示p1 和 p2 有相同生命周期 至少在使用它的那时候
// 那为啥rust 不给每个引用型入参数都一样的生命周期标识 我们就不用手动标出？
fn get_max<'a >(p1: &'a i32, p2: &'a i32) ->&'a i32{
    if p1 > p2 {
        p1
    }else{
        p2
    }
}

#[allow(dead_code)]
fn test1(p1: Vec<f64>)->Vec<f64> {
    p1
}
#[allow(dead_code)]
// 对于出参不是引用型的情况 入参给不给生命周期都无妨
fn test2(p1: &Vec<f64>)->Vec<f64> {
    p1.clone()
}
#[allow(dead_code)]
// 对于出参不是引用型的情况 入参给不给生命周期都无妨 在编译器眼里它就是下面这样的
fn test2_v1<'a>(p1: &'a Vec<f64>)->Vec<f64> {
    p1.clone()
}

// 对于出参是引用 而没有入参的情况 只能是静态生命周期了
fn get_ref_static()-> &'static i32{
    &SOME_INT
}

mod vectors{
    pub fn run(){
        // 部分的生命周期跟整体是一样的 即使只返回一个子集或者一个切片

        // NOTE: String 也是一种u8的向量哦！

        let a:Vec<i32> = vec![1,2,3,4,5,6];
        let r = get_vec_slice(&a);
        println!("{:?}",r) ;
    }

    fn get_vec_slice(p1: &[i32]) -> &[i32]{
        &p1[0..2]
    }
    fn get_vec_slice2<'a>(p1: &'a [i32],p2: &'a [i32]) -> &'a [i32]{
        if p1.len() > p2.len() {
            &p1[0..2]
        }else{

            &p2[0..2]
        }
    }
}

mod consts{

    pub fn run(){
        let a = String::from("some value");


        let greater = longger(&a, SOME_CONST_A) ;
        println!("{}", greater);
    }

    const SOME_CONST_A: &str = "hi i am a const str" ;
    const SOME_CONST_B: &str = "hi i am a const str too!" ;

    fn some_func()-> &'static str{
        SOME_CONST_A
    }

    // 这个版本比下面那个更好 更通用些 下面的限制了入参生命周期只能是静态的
    fn longger<'a>(p1: &'a str, p2: &'a str)-> &'a str{
        if p1> p2{
            p1
        }else{
            p2
        }
    }

    fn longger_v0(p1: &'static str, p2: &'static str)-> &'static str{
        if p1> p2{
            p1
        }else{
            p2
        }
    }
}

mod lifetimes_with_generics {


    pub fn run (){
        let f1 = 1.00 ;
        let f2 = 2.00 ;
        println!("smaller {}",get_smaller(&f1,&f2)) ;

        let s1 = "a";
        let s2 = "b";
        println!("smaller str: {}",get_smaller(&s1,&s2)) ;
    }
    fn get_smaller<'a ,T: std::cmp::PartialOrd>(p1: &'a T,p2: &'a T)-> &'a T{
        if p1<p2{
            p1
        }else {
            p2
        }
    }
}

mod structs{
    // 在结构体上用生命周期的情况类似于函数。
    
    struct SomeStruct<'a>{
        some_data: Vec<i32>,
        some_ref_data: &'a Vec<i32>, 

    }

    // a b 也可以用同一个 看你使用场景 到底需要他们活的一样长不
    struct SomeStruct2<'a,'b>{
        some_data: Vec<i32>,
        some_ref_data: &'a Vec<i32>, 
        some_ref_data2: &'b Vec<i32>, 

    }
    // b 至少跟a一样长
    struct SomeStruct2_<'a,'b:'a>{
        some_data: Vec<i32>,
        some_ref_data: &'a Vec<i32>, 
        some_ref_data2: &'b Vec<i32>, 

    }
}