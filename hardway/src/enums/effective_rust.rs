// Avoid matching Option and Result
fn foo(){
    struct S {
        field: Option<i32>,
    }

    let s = S { field: Some(42) };
    match &s.field {
        Some(i) => println!("field is {}", i),
        None => {}
    }

    // better 
    if let Some(i) = &s.field {
        println!("field is {}", i);
    }

}


fn explicit_match(){
    let result = std::fs::File::open("/etc/passwd");
    let f = match result {
        Ok(f) => f,
        Err(_e) => panic!("Failed to open /etc/passwd!"),
    };

    let f = std::fs::File::open("/etc/passwd").unwrap();

}

type UserId = usize ;
pub fn find_user(username: &str) -> Result<UserId, std::io::Error> {
    let f = match std::fs::File::open("/etc/passwd") {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    // ...

    // let f = std::fs::File::open("/etc/passwd")?;

     Ok(1)
}
