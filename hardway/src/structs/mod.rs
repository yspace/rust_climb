mod chat;
mod tuple_structs;
mod sizes;
mod impls;
mod no_fields;
pub mod consumes;

pub fn run() {

    // 最近学习的模块入口
    return sizes::main();
    return chat::main();

    {
        let bg = Color {
            red: 255,
            green: 70,
            blue: 15,
        };

        println!("{},{},{}", bg.red, bg.green, bg.blue);
    }
    {
        let mut bg = Color {
            red: 255,
            green: 70,
            blue: 15,
        };

        bg.blue = 45;

        println!("{},{},{}", bg.red, bg.green, bg.blue);
    }

    tuple_structs::run();
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct Door {
    width: u32,
    height: u32,
    is_open: bool,
}

impl Door {
    pub fn new(width: u32, height: u32, is_open: bool) -> Door {
        Door {
            width,
            height,
            is_open,
        }
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }
}
