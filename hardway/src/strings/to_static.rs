// https://stackoverflow.com/questions/23975391/how-to-convert-a-string-into-a-static-str

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let s: &'static str = string_to_static_str(s);
}

#[test]
fn test_str(){
    let mut s ;
    {
        let b = "123";
        s = b
    }
    println!("{}", s);
}