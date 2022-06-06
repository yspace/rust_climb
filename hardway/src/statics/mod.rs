//  静态变量经常配合laze_static使用

static V: Vec<u8> = Vec::new();
pub fn main() {
    println!("hello static");
}