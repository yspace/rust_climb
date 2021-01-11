// for循环的主要用处是利用迭代器对包含同样类型的多个元素的容器
// 执行遍历，如数组、链表、HashMap、HashSet等。在Rust中，我们可以
// 轻松地定制自己的容器和迭代器，因此也很容易使for循环也支持自定义
// 类型。
pub fn main(){
    let array = &[1,2,3,4,5] ;
    for i in array{
        // for循环内部也可以使用continue和break控制执行流程。
        println!("The number is {}", i) ;
    }
//    for i in array{
//        println!("The number is {}", i) ;
//    }
}