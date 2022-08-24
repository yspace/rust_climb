pub fn main() {
    let n = 8;
    if n % 2 == 0 {
        println!("n= {} , which is even", n);
    } else {
        println!("n = {} , which is odd", n);
    }

    {
        let n = 8;
        if n % 2 == 0 {
            println!("n= {} , which is even", n);
        } else if n == 1{
            println!("n = 1 , which is special case");
        } 
        
        else {
            println!("n = {} , which is odd and not equals to 1", n);
        }
    }
}

fn short_circuit(){
    if 1 != 1 || { println!("can't evaluated !") ; 1!=0} {

    }
}

fn assign(){
    let n = 10; 
    let number = if n == 1 {"one"} else {"not one"} ;
}

fn expression1(){
    let number = {
        let n = 10; 
        if n ==1 {"one"}else{"not one"}  

    } ;

    println!("number = {}", number);
}