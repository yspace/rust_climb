

pub struct Flag {
    short: char,
    name: &'static str,
    /* ... */
}

impl Flag {
    pub const fn new(short: char, name: &'static str) -> Self {
        Flag { short, name }
    }
}

inventory::collect!(Flag);  

struct MyAction{
    f: fn(),
}
inventory::collect!(MyAction);  

mod user{

    use super::*;
    inventory::submit! {
        Flag::new('v', "verbose")
    }
    inventory::submit! {
       MyAction{
        f: create_user,
       }
    }

    fn create_user(){
        println!("do create user");
    }


}


#[test]
fn main(){
    for flag in inventory::iter::<Flag> {
        println!("-{}, --{}", flag.short, flag.name);
    } 

    for my_action in inventory::iter::<MyAction>{
        (my_action.f)();
    }
}