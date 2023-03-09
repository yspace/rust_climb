
#[test]
fn test_modify_string() {
    let s = String::from("some string ");

    let s2 = modify_string(s);

    println!("s2 {}", s2);

    let mut  s3 = String::from("some string3");
    modify_string2(&mut s3);
    println!("s3 {}", s3);
}


fn modify_string(mut string: String) -> String {
    
    string.push_str(" suffix");

    string
}
fn modify_string2( string: &mut String)  {
    
    string.push_str(" suffix");

    
}