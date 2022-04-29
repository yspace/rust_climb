fn main() {
    struct Book {
        title: String,
        is_used: Option<bool>,
    }

    let book = Book {
        title: "Great book".to_string(),
        is_used: None, //Some(true),
    };

    match book.is_used {
        // Some(true) => println!("{} is used", book.title),
        // Some(false) => println!("{} is not used", book.title),
        Some(v) => println!("{} is used", v),
        None => println!("{} we don't know if book is used", book.title),
    }
}

#[test]
fn test_book_is_used() {
     main() ;
}