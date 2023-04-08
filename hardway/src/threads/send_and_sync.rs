// #![feature(negative_impls)] // 这个要放到crate的根module去

use std::thread::spawn;


#[derive(Debug)]
struct Foo {}
// Send 跟move 和channel的发送移动有关
// 非trait需要开启特征：negative_impls
// impl !Send for Foo {}

#[test]
fn main() {
    let foo = Foo {};
    spawn(move || {
       // dbg!(foo);
       println!("foo can't be sent ");
    });
}