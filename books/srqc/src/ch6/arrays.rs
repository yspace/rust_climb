
pub fn main(){

    basic();
    same_type() ;
    access() ;
    array_methods() ;
    multiple_dimension() ;
    array_slice() ;
    dst_fat_ptr() ;
    learn_range() ;
    range_array() ;

    check_bounds() ;
}

fn basic(){
    // 定长数组
    let xs: [i32; 5] = [1,2,3,4,5] ;

    // 所有元素初始化为相同元素
    let ys: [i32; 500]  = [0 ; 500] ;

}

fn same_type(){
    // 对于两个数组类型，只有元素类型和元素个数都完全相
    // 同，这两个数组才是同类型的。数组与指针之间不能隐式转换。同类型
    // 的数组之间可以互相赋值。
    let mut xs:[i32; 5] = [1,2,3,4,5] ;
    let ys :[i32; 5] = [6,7,8,9,10] ;
    xs = ys ;
    println!("new array {:?}", xs) ;
}

fn access(){
    let v: [i32; 5] = [1,2,3,4,5] ;
    let x = v[0] + v[1];
    println!("sum is {}", x) ;
}

fn array_methods(){
    let v1 = [1,2,3];
    let v2 = [1,2,3];
    println!("{:?}",v1 < v2) ;
    println!("{:?}",v1 == v2) ;
    // 遍历

    let v = [0_i32; 10] ;
    for i in &v{
        println!("{:?}", i ) ;
    }
}

fn multiple_dimension(){
    let v: [
     [i32; 2];
        3
    ] = [
      [0,0],
      [0,0],
      [0,0]
    ] ;

    for i in &v{
        println!("{:?}", i);
    }
}

// 对数组取借用borrow操作，可以生成一个“数组切片”（Slice）。
fn array_slice(){
    // 个数组[T;n]，它的借用指针的类型就是&[T;n]。

    fn mut_array(a: &mut [i32]){
        a[2] = 5 ;
    }

    println!("size of &[i32, 3] : {:?}", std::mem::size_of::<&[i32; 3]>());
    println!("size of &[i32] : {:?}", std::mem::size_of::<&[i32]>());

    let mut v: [i32; 3] = [1,2,3] ;
    {
        let s: &mut [i32; 3] = &mut v ;
        mut_array(s) ;
    }
    println!("{:?}", v) ;
}

fn dst_fat_ptr(){
    // ：对于不定长数组类型[T]，有对应的胖指针&[T]类型；对于
    //不定长字符串str类型，有对应的胖指针&str类型；以及在后文中会出现
    //的Trait Object；

    // NOTE 由于不定长数组类型[T]在编译阶段是无法判断该类型占用空间的
    //大小的，目前我们不能在栈上声明一个不定长大小数组的变量实例，也
    //不能用它作为函数的参数、返回值。但是，指向不定长数组的胖指针的
    //大小是确定的，&[T]类型可以用做变量实例、函数参数、返回值。
  use std::mem ;

    fn raw_slice(arr: &[i32]){
        unsafe {
            // ，我们利用
            //了unsafe的transmute函数。我们可以把它看作一个强制类型转换，类似
            //reinterpret_cast，通过这个函数，我们把胖指针的内部数据转换成了两
            //个usize大小的整数来看待。
            let (val1, val2): (usize, usize) =
                mem::transmute(arr) ;

            println!("value in raw pointer") ;
            println!("value1: {:x}", val1);
            println!("value1: {:x}", val2);
        }
    }

    //
    let arr: [i32; 5] = [1,2,3,4,5] ;
    let address : &[i32; 5] = &arr ;

    println!("Address of arr: {:p}", address) ;

    raw_slice(address as &[i32]) ;
    // 在这个示例中，我们arr是长度为5的i32类型的数组。address是一个
    //普通的指向arr的借用指针。我们可以用as关键字把address转换为一个胖
    //指针&[i32]，并传递给raw_slice函数。

}

fn learn_range(){
    let r = 1..10 ;
    for i in r {
        print!("{:?}\t", i) ;
    }

    println!();
    // 等价：
    {
        use std::ops::Range;
            let r = Range {start: 1, end: 10}; // r是一个Range<i32>
            for i in r {
                print!("{:?}\t", i);
            }
    }

    // 实现了迭代器
    {
        println!("\n 实现了Iterator trait ");
        use std::iter::Iterator;
// 先用rev方法把这个区间反过来,然后用map方法把每个元素乘以10
        let r = (1i32..11).rev().map(|i| i * 10);
        for i in r {
            print!("{:?}\t", i);
        }
    }

    /*
    在Rust中，还有其他的几种Range，包括
    ·std：：ops：：RangeFrom代表只有起始没有结束的范围，语法为
    start..，含义是[start，+∞）；
    ·std：：ops：：RangeTo代表没有起始只有结束的范围，语法
    为..end，对有符号数的含义是（-∞，end），对无符号数的含义是[0，
    end）；
    ·std：：ops：：RangeFull代表没有上下限制的范围，语法为..，对
    有符号数的含义是（-∞，+∞），对无符号数的含义是[0，+∞）。
    */
}

fn range_array(){
    // 数组和Range之间最常用的配合就是使用Range进行索引操作。
    fn print_slice(arr: &[i32]){
        println!("Length: {}", arr.len()) ;

        for item in arr {
            print!("{}\t", item) ;
        }
        println!("") ;
    }

    //
    let arr:[i32; 5] = [1,2,3,4,5] ;
    print_slice(&arr[..]) ; // 全range;

    let slice = &arr[2..] ; // RangeFrom
    print_slice(slice) ;

    let slice2 = &slice[..2] ; // RangeTo
    print_slice(slice2) ;

    // 闭区间对应的标准库中的类型是：
    //·std：：ops：：RangeInclusive，语法为start..=end，含义是[start，end]。
    //·std：：ops：：RangeToInclusive，语法为..=end，对有符号数的含
    //义是（-∞，end]，对无符号数的含义是[0，end]
}

fn check_bounds(){
    use std::env ;
    let v = [10i32, 20,30,40,50] ;
    let index : usize = env::args().nth(1)
        .map(|x|x.parse().unwrap_or(0)).unwrap_or(0) ;

    println!("{:?}", v[index]) ;


    // 为了防止索引操作导致程序崩溃，如果我们不确定使用的“索引”是
    //否合法，应该使用get（）方法调用来获取数组中的元素，这个方法不
    //会引起panic！，它的返回类型是Option<T>，

    let v = [10i32 , 20,30,40,50 ] ;
    let first = v.get(0) ;
    let tenth = v.get(10) ;
    println!("{:?} {:?}", first, tenth) ;

    // 鼓励使用迭代器 而不是索引（索引有越界检查)
    {
        use std::iter::Iterator;

        let v = &[10i32 , 20,30,40, 50] ;

        // 如果我们同时需要index和内部元素的值,调用enumerate()方法
        for (index, value) in v.iter().enumerate() {
            println!("{} => {}",  index, value) ;
        }

        // filter方法可以执行过滤,nth函数可以获取第n个元素
        let item = v.iter().filter(
            |&x| *x % 2 == 0
        ).nth(2) ;
        println!("{:?}", item) ;
    }
}