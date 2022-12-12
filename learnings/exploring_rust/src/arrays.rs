// 数组实现了一些traits , 注意他们来自哪些包
// - Copy
// - Clone
// - Debug
// - IntoIterator
// - cmp::PartialEq , PartialOrd,  Eq , Ord
// - Hash
// - AsRef, AsMut
// - Borrow, BorrowMut 
// - Default
// - Index , IndexMut

// 可以协变（coerce to） 到slice

// ## 重要实现
// - map
// - zip


#[derive(Debug,Default)]
struct Size(i32) ;

pub fn run() {
    // let sizes = [Size(1); 3];

    array_maps();
}

fn array_maps(){
    let names = ["yiqing","qingyi"];
    let names2 = names.map(|x| format!("{}!",x));

    println!("{:?}", names2);
    println!("{:#?}", names);

    let nums = [1, 2, 3];
    let nums2 = nums
    .map(|n| n+1)
    .map(|n| n * 2)
    .map(|n| println!("{}",n));
    dbg!(nums2) ;
}