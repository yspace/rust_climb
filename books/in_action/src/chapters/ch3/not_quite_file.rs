#![allow(unused_variables)]

type File = String ;

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool
{
    true
}

#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>)-> !{

    unimplemented!()
}

pub fn main() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    // read(f1, vec![])
    close(&mut f1);
}

mod special_return_types{
    use std::fmt::Debug;

    fn report<T: Debug>(item: T){
        println!("{:?}", item);
    }

    fn clear(text: &mut String) ->() {
        *text = String::from("");
    }

    fn dead_end() -> ! {
        panic!("you have reached a dead end");
    }

    fn forever() -> ! {
        loop {
            // 两千年以后 break
        }
    }
}