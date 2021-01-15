use std::{sync::Arc, thread::spawn};
use std::{thread, time};
//#[derive(Copy, Clone)]
struct ThreadInfo {
    number: i32,
}

pub fn main() {
    for n in 0..10 {
        let t_info = Arc::new(ThreadInfo { number: n });

        spawn(move || {
            // 注意线程的调度是随机的  所以打印的数字也是随机的
            print!("Ran: {}", t_info.number);
        });
    }

    thread::sleep(time::Duration::from_millis(1));

    // 实现v2
    v2::main();
}

mod v2 {
    use std::{sync::Arc, thread::spawn};
    use std::{thread, time};
    #[derive(Copy, Clone)]
    struct ThreadInfo {
        number: i32,
    }

    pub fn main() {
        for n in 0..10 {
            let t_info = ThreadInfo { number: n };

            spawn(move || {
                println!("Ran: {}", t_info.number);
            });
        }

        thread::sleep(time::Duration::from_millis(1));
    }
}

mod v3 {
    use std::{sync::Arc, thread::spawn};
    use std::{thread, time};
    struct ThreadInfo {
        number: i32,
    }

    // 手动实现Clone Copy 特征
    impl Copy for ThreadInfo {}
    impl Clone for ThreadInfo {
        fn clone(&self) -> Self {
            println!("Cloned : {}", self.number) ;
            ThreadInfo{
                number: self.number ,
            }
        }
    }

    pub fn main() {
        for n in 0..10 {
            let t_info = ThreadInfo { number: n };

            spawn(move || {
                println!("Ran: {}", t_info.number);
            });
        }

        thread::sleep(time::Duration::from_millis(1));
    }
}
