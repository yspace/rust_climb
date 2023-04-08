mod _stdlib {
    trait Default {
        fn default() -> Self;
    }
}

#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Default for Color {
    // default color is black
    fn default() -> Self {
        Color { r: 0, g: 0, b: 0 }
    }
}

#[test]
fn test_default() {
    let color = Color::default();
    assert_eq!(color.r, 0);
}

struct Canvas;
enum Shape {
    Circle,
    Rectangle,
}

impl Canvas {
    // let user optionally pass a color
    fn paint(&mut self, shape: Shape, color: Option<Color>) {
        // if no color is passed use the default color
        let color = color.unwrap_or_default();
        // etc

        println!("Painting with color: {:?}", color);
    }
}

fn guarantee_length<T: Default>(mut vec: Vec<T>, min_len: usize) -> Vec<T> {
    for _ in 0..min_len.saturating_sub(vec.len()) {
        vec.push(T::default());
    }
    vec
}

impl Color {
    fn red(r: u8) -> Self {
        Color {
            r,
            ..Color::default()
        }
    }
    fn green(g: u8) -> Self {
        Color {
            g,
            ..Color::default()
        }
    }
    fn blue(b: u8) -> Self {
        Color {
            b,
            ..Color::default()
        }
    }
}

mod _derive {
    // default color is still black
    // because u8::default() == 0
    #[derive(Default)]
    struct Color {
        r: u8,
        g: u8,
        b: u8,
    }
}
