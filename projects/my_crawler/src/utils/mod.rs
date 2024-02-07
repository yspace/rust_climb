use crypto::digest::Digest;
use crypto::md5::Md5;

mod file_ops;
 
pub fn md5<S:Into<String>>(input: S) -> String {
    let mut md5 = Md5::new();
    md5.input_str(&input.into());
    md5.result_str()
}
 
#[test]
fn test(){
    println!("md5: {}",md5("hello md5"))
}

use base64::{encode, decode};
 
#[test]
fn test_base64() {
    let a = b"hello world";
    let b = "aGVsbG8gd29ybGQ=";
 
    assert_eq!(encode(a), b);
    assert_eq!(a, &decode(b).unwrap()[..]);
}
 