mod hashmaps ;

pub fn main() {
    // vecs::run() ;
    hashmaps::run() ;
}

mod vecs{
    // Vec的所有内容项都是生成在堆空间上的
    // Vec<T>中的泛型T必须是Sized的，也就是说必须在编译的时候就知道存一个内容项需要多少内存。对于那些在编译时候未知大小的项（函数类型等），我们可以用Box将其包裹，当成一个指针。

    pub fn run() {


        let v1 : Vec<i32> = Vec::new() ;
        println!("{:?}", v1) ;

        // new函数并没有提供一个能显式规定其泛型类型的参数
        // let mut v1 = Vec::new::<i32>();
        // 与之对比的,collect函数就能指定：
        // let mut v2 = (0i32..5).collect::<Vec<i32>>();

        macro_creating() ;
        from_iter();
        access();
        iter() ;

        push() ;
    }

    fn macro_creating() {
        let v : Vec<i32> = vec![] ;

        let v = vec![1,2,3] ;

        assert_eq!(v[0] , 1) ;

        let v = vec![0; 10] ; //注意分号，这句话声明了一个 内容为10个0的动态数组
        assert_eq!(v[9] , 0) ;
        assert_eq!(v.len(), 10) ;
    }

    fn from_iter() {
        //因为Vec实现了FromIterator这个trait，因此，借助collect，我们能将任意一个迭代器转换为Vec。
        let v: Vec<_> = (1..5).collect() ;

        println!("{:?}" , v) ;
    }

    fn access(){
        // Rust中，一旦越界的后果是极其严重的，可以导致Rust当前线程panic。
        // 因此，除非你确定自己在干什么或者在for循环中，不然我们不推荐通过下标访问
        let v = vec![1,2,3] ;
        assert_eq!(v[1usize], 2) ;

        // 安全的下标访问
        assert_eq!(v.get(1), Some(&2)) ;
        assert_eq!(v.get(4), None) ;
    }

    fn iter(){
        let mut v = vec![1,2,3] ;
        // 获取饮用
        for i in &v {
            println!("{}", i) ;
        }
        // 可变饮用
        for i in &mut v {
            //println!("mut {}", *i) ;
            *i =   (*i)+1;
        }
        // 获得所有权，注意此时Vec的属主将会被转移！！
        for i in v {
            println!("{}", i) ;
        }
    }

    fn push(){
        use std::time;

        fn push_1m(v: &mut Vec<usize>, total: usize) {
            let e = time::SystemTime::now();
            for i in 1..total {
                v.push(i);
            }
            let ed = time::SystemTime::now();
            println!("time spend: {:?}", ed.duration_since(e).unwrap());
        }

        let mut v: Vec<usize> = vec![];
        push_1m(&mut v, 5_000_000);
        let mut v: Vec<usize> = vec![];
        v.reserve(5_000_000);
        push_1m(&mut v, 5_000_000);
    }
}