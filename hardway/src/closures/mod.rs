pub fn main(){
    // 闭包是引用了自由变量的函数
    // 在rust中，函数和闭包都是实现了Fn、FnMut或FnOnce特质（trait）的类型。任何实现了这三种特质其中一种的类型的对象，都是 可调用对象

    closure_syntax::run();
}

mod closure_syntax{
    pub fn run() {
        let plus_one = |x: i32| x + 1 ;

        assert_eq!(2, plus_one(1)) ;


        let plus_two = |x| {
            let mut result: i32 = x ;

            result += 1 ;
            result += 1 ;

            result
        };
        assert_eq!(4, plus_two(2)) ;
        // 闭包 可以不用写全 参数类型 和返回值类型 因为没有文档的需求

        fn  plus_one_v1   (x: i32) -> i32 { x + 1 }
        let plus_one_v2 = |x: i32| -> i32 { x + 1 };
        let plus_one_v3 = |x: i32|          x + 1  ;
        println!(
            "{},{}, {}",
            plus_one_v1(1),
            plus_one_v2(1),
            plus_one_v3(1)
        )
    }

    fn capture_vars(){
            let mut num = 5;
    {
        let plus_num = |x: i32| x + num;

    } // plus_num goes out of scope, borrow of num ends
        let y = &mut num ;

        //

        let num = 5;

        let owns_num = move |x: i32| x + num;

        // 那么在这个例子中，我们的闭包取得了一个num的可变引用，然后接着我们调用了add_num，它改变了其中的值，正如我们期望的。
        // 我们也需要将add_num声明为mut，因为我们会改变它的环境。
        let mut num = 5;

        {
            let mut add_num = |x: i32| num += x;

            add_num(5);
        }

        assert_eq!(10, num);

        // 
        {
            let mut num = 5;

            {
                let mut add_num = move |x: i32| num += x;

                add_num(5);
            }

            assert_eq!(5, num);
        }
    }
    
    

}