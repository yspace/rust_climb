pub fn main(){

    dbg!(1 == true.into());

    let x = true ;
    let y = x.then(|| 3) ;

    let b = 5>3 ;
    match b {
        true => println!(">"),
        false => println!("<="),
    }
}