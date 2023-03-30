fn main() {
    struct Book {
        title: String,
        isbn: Option<String>,
    }
    let book = Book {
        title: "some book title here".to_string(),
        isbn: Some(String::from("1-123-456")),
    };
    match book.isbn {
        Some(i) => println!("The ISBN of the book: {} is: {}", book.title, i),
        None => println!(" ISBN not provided"),
    }
}

fn some_or_none<T>(option: &Option<T>)  {
    match option {
        Some(_v) => println!("is some!"),
        None => {
            println!("is none")
        },
    }

}

fn match_integer(val: i32){
    match val {
        1 => println!("one"),
        2|3 => println!("two or three"),
        4 ..=10 => println!("[4,10]"),
        _=> println!("others (-⭕️⭕️ 1) (10 , ⭕️⭕️）")
    }
}
#[test]
fn test_match_integer() {
    match_integer(1);
    match_integer(-1);
    match_integer(11);
}
