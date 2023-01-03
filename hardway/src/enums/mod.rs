
enum Payment{
    Cash(f32),
    CreditCard,
    DebitCard(DebitData),
    // Crypto(
    //     account_id: String, 
    //     amount: f32
    // ),
}

struct DebitData{
    pub card_number: String,
    pub amt: f32,
}

pub fn run(){
    let p = Payment::Cash(100_f32);

    match p {
        Payment::Cash(amt) => println!("Paying with cash: {}", amt),
        Payment::CreditCard => println!("Paying with credit card"),
        Payment::DebitCard(debit_data) => println!("Paying with......debit card"),
        _=>{},
    }
}

fn process_payment(payment: Payment){
    match payment{
        Payment::CreditCard=> println!("paying with credit card"),
        _=>{ println("not handled! ")}
    }
}