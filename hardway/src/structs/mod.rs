mod tuple_structs;

pub fn run() {
    {
        let bg = Color {
            red: 255,
            green: 70,
            blue: 15,
        };

        println!("{},{},{}", bg.red, bg.green, bg.blue);
    }
    {
        let mut bg = Color{
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
