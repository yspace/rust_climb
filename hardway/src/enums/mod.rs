pub mod matches;
pub mod options;
pub mod results;
pub mod behaviours;
pub mod effective_rust;

enum Payment {
    Cash(f32),
    CreditCard,
    DebitCard(DebitData),
    // Crypto(
    //     account_id: String,
    //     amount: f32
    // ),
}

struct DebitData {
    pub card_number: String,
    pub amt: f32,
}

pub fn run() {
    let p = Payment::Cash(100_f32);

    match p {
        Payment::Cash(amt) => println!("Paying with cash: {}", amt),
        Payment::CreditCard => println!("Paying with credit card"),
        Payment::DebitCard(debit_data) => println!("Paying with......debit card"),
        _ => {}
    }
}

fn process_payment(payment: Payment) {
    match payment {
        Payment::CreditCard => println!("paying with credit card"),
        _ => {
            println!("not handled! ")
        }
    }
}

#[derive(Debug)]
enum GENDER {
    MALE,
    FEMALE,
}

impl From<u32> for GENDER {
    fn from(value: u32) -> Self {
        // 使用了  match guard feature
        match value {
            value if GENDER::MALE as u32 == value => GENDER::MALE,
            value if GENDER::FEMALE as u32 == value => GENDER::FEMALE,
            _ => panic!("Unknow gender"),
        }
    }
}

#[test]
fn test_gentder() {
    println!("{:?}", GENDER::MALE);
    println!("{:?}", GENDER::FEMALE as u32);
}

enum Numbers {
    One = 1,
    Two = 2,
    Three = 3,
}
#[test]
fn test_numbers() {
    // assert_eq!(1, Numbers::One); // 这个是需要formatted 后才比较 需要实现DEBUG？
    assert_eq!(1, Numbers::One as u32);
}

// 常规 比较好的实践是不要在enum中混用命名和非命名变体 容易混爻
enum EnumTypes {
    NamedType,
    String,
    NamedString(String), // new type within enum
    StructLike { name: String },
    TupleLike(String, i32),
}


enum HttpResultCode {
    Ok = 200,
    NotFound = 404,
    Teapot = 418,
}

#[test]
fn test_http_result_code() {
    let code = HttpResultCode::NotFound;
    // assert_eq!(code as i32, 404);

    let msg = match &code {
        HttpResultCode::Ok => "Ok",
        HttpResultCode::NotFound => "Not found",
        // forgot to deal with the all-important "I'm a teapot" code
        _ => "others!"
    };
    println!("{}", msg);
    assert_eq!(code as i32, 404);
}