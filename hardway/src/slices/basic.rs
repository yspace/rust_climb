
#[test]
pub fn run(){
    let arr = [0u8; 64]; // 初始化数组 元素为u8类型的0
    let slice: &[u8] = &arr ; // 借用数组作为切片 ；切片可以源自数组！或者其他切片

    // split at 方法很多类都有实现：slices arrays vectors
    let (first_half, second_half) = slice.split_at(arr.len()/2);

    println!(
        "left_half.len = {} right_half.len = {}",
        first_half.len(),
        second_half.len(),
    );
}

fn destructuring(){
    // 解构 借用数组或切片的一部分 常见的用例是用来解析或解码文本或者二进制数据

    let wordlist = "one,two,three,four";
     for word in wordlist.split(',') {
        println!("word={}", word);
    }
    // 上面的代码 👆 不会发生堆分配 所有的内存分配在栈上
}