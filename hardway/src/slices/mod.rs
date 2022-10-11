pub fn main() {
    println!("learn the slice");

    string_slice();
    slicing_int_array();
    mutable_slices();

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

fn analyzi_slice<T>(slice: &[T]) {
    println!("Slice size : {}", std::mem::size_of_val(&slice));
}
fn slice_size<T>(slice: &[T]) -> usize {
    std::mem::size_of_val(&slice)
}

fn string_slice() {
    let n1 = "Tutorials".to_string();
    println!("length of string is {}", n1.len());
    let c1 = &n1[4..9];

    // fetches characters at 4,5,6,7, and 8 indexes
    println!("{}", c1);
}
fn slicing_int_array() {
    fn use_slice(slice: &[i32]) {
        // is taking a slice or borrowing a part of an array of i32s
        println!("length of slice is {:?}", slice.len());
        println!("{:?}", slice);
    }

    let data = [10, 20, 30, 40, 50];
    use_slice(&data[1..4]);
     //this is effectively borrowing elements for a while
}

fn mutable_slices(){
    let mut data = [10,20,30,40, 50] ;
    use_slice(&mut data[1..4]);
    println!("{:?}", data);


    fn use_slice(slice: &mut [i32]) {
        println!("length of slice is {:?}",slice.len());
        println!("{:?}",slice);
        slice[0] = 1010; // replaces 20 with 1010
    }
}

#[test]
fn test_analyzi_slice() {
    let arr = ['a', '呀', 'b'];
    let slice = &arr[..];

    assert_eq!(slice_size(slice), 2 * std::mem::size_of::<usize>());
}
