
pub fn run(){
    greet_world() ;
}

fn greet_world(){
    println!("hello, world!") ;
    let southen_germany = "xxxx";
    let japan = "小 日 本" ;
    let regions = [
      southen_germany ,
      japan
    ] ;
    for region in regions.iter() {
        println!("{}",region) ;
    }
}