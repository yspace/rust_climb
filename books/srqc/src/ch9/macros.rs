
pub fn main(){

    runtime_calc() ;
    use_my_macro();
}

fn runtime_calc(){
    println!("file {} line {}", file!(), line!()) ;
}






fn use_my_macro(){
    // 查看宏展开
    // rustc -Z unstable-options --pretty=expanded temp.rs
    macro_rules! hashmap {
        ($( $key: expr => $val: expr ),*) => {
            {
                let mut map = ::std::collections::HashMap::new();
                // map.insert($key, $val);
                $( map.insert($key, $val); )*
                map
           }
        }
    }
    let counts = hashmap!['A' => 0, 'C' => 0, 'G' => 0, 'T' => 0];

    println!("{:?}", counts);
}