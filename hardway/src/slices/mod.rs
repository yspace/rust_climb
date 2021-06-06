pub fn main() {
    println!("learn the slice");

    // https://zhuanlan.zhihu.com/p/131015459
    //    如下都是合法的slice
    //  let a = [1,2,3];
    //  let shared_slice = &a[..];
    //  ​
    //  let mut b = [1,2,3];
    //  let mutable_slice = &mut b[..];
    //  ​
    //  let boxed_array = Box::new([1,2,3]);
    //  let boxed_slice = &boxed_array[..];

    // slice patterns即可以匹配固定大小的数组，也可以匹配动态大小（Vec）
    let arr = [1, 2, 3];
    match arr {
        [1, _, _] => "starts with one",
        [a, b, c] => "starts with something else",
    };
    // ​
    // // Dynamic size
    // let v = vec![1, 2, 3];
    // match v[..] {
    //     [a, b] => { /* this arm will not apply because the length doesn't match */ }
    //     [a, b, c] => { /* this arm will apply */ }
    //     _ => { /* this wildcard is required, since the length is not known statically */ }
    // };

    println!(
        "first world of 'hello world' is {}",
        first_word(&"hello wrold".to_string())
    );
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
