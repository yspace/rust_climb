use hardway ;

use some_mod::* ;

mod some_mod ;

fn main() {
    println!("hi this is some example");

    hardway::some_api() ;

    let mut ctx = AppContext::new();

    let state = MainState::new(ctx);

    // dbg!(ctx) ;
    dbg!(state) ;
}