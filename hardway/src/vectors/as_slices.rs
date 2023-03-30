
#[test]
pub fn run(){

    let mut vec = vec![1, 2, 3];
     let slice = vec.as_slice(); 
    //  vec.resize(10, 0); // 这是不被允许的 👆上面已经进行了不可变借用了
      println!("{}", slice[0]);
}