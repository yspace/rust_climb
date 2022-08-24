
pub fn main() {
    let greater_than: bool = 5<10 ;
    let not_equals: bool = 1!=0 ;
    let equals: bool = "0x5ff" == "0x5ff" ;

    let combined: bool = greater_than && not_equals || equals > true ;

    println!("{}", combined);
}