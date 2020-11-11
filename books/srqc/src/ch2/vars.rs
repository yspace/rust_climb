
pub fn main(){
    // 声明变量
    let variable : i32 = 100 ;

//    let _ = variable ;

    // 可写变量
    let mut x = 5 ;
    x = 10 ;
    println!("x = {} ", x) ;

    // 模式
    let (mut a , mut b) = (1,2) ;
//    let Point{x: ref a, y: ref b} = p;
    {
        // 变量遮蔽 shadowing
        let x = "hello" ;
        println!("x is {}", x);

        let x = 5 ;
        println!("x is {}", x);

        let mut v = Vec::new() ;
        v.push(1);
        v.push(2);
        v.push(3);

        let v = v ; // 从此以下v变为只读了 无法再访问上面可写的v了
        for i in &v {
            println!("{}", i) ;
        }

        // ##
        let v = Vec::new() ;
        let mut v = v ;
        v.push(1);
        println!("{:?}", &v);
//        println!("{:?}", v);

    }

    {
        // ## 类型推导
        let elem = 5u8 ; // 通过后缀推导类型

        let mut vec = Vec::new() ;
        vec.push(elem) ; // 通过操作参数类型推导向量类型
        println!("{:?}", vec) ;

        // ## 部分推导
        let player_scores = [
            ("jack",20),
            ("jane",23),
            ("jill",18),
            ("john",19),
        ];

        // 动态数组  成员类型交由编译器推导
        let players : Vec<_> = player_scores
            .iter()
            .map(|&(player, _score)|{
//                player
                (player,"大坏蛋")
            })
            .collect() ;
        println!("{:?}", players);
    }

    {
        // ## 类型别名
        type Age = u32 ;

        fn grow(age : Age , year: u32) ->Age {
            age + year
        }

        let x : Age = 20 ;
        println!("20 years later: {}", grow(x, 20));

        // 泛型场景
        type Double<T> = (T, Vec<T>) ; // 可以简化代码
    }
}

fn test(condition: bool){
    let x: i32;
    if condition {
        x  = 1 ;
        println!("{}", x) ;
    }
    // 条件不满足不会初始化x
    // 但是只要不再使用x 就没事
}