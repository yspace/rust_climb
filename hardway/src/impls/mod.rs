pub fn run() {
    println!("impl keyword in Rust");

    let my_rect = Rectangle{
        width: 10,
        height: 50
    };

    my_rect.print_description();
    println!("Rectangle is a square {}", my_rect.is_square());
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn print_description(&self) {
        println!("Rectange {} x {}", self.width, self.height);
    }

    // This is the method implementation.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}