
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}


fn print_color(color: &Color) {
    println!("{},{},{}", color.red, color.green, color.blue);
}

#[test]
fn it_works() {
    let bg = Color {
        red: 255,
        green: 70,
        blue: 15,
    };

    print_color(&bg);
}