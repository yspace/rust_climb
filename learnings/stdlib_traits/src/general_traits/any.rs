/*

// generic blanket impl:

impl<T: 'static + ?Sized> Any for T {
    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
}
 */

use std::any::Any;

mod __stdlib {
    use std::any::TypeId;

    trait Any: 'static {
        fn type_id(&self) -> TypeId;
    }
}

#[derive(Default)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn inc(&mut self) {
        self.x += 1;
        self.y += 1;
    }
}

fn map_any(mut any: Box<dyn Any>) -> Box<dyn Any> {
    if let Some(num) = any.downcast_mut::<i32>() {
        *num += 1;
    } else if let Some(string) = any.downcast_mut::<String>() {
        *string += "!";
    } else if let Some(point) = any.downcast_mut::<Point>() {
        point.inc();
    }
    any
}

#[test]
fn main() {
    let mut vec: Vec<Box<dyn Any>> = vec![
        Box::new(0),
        Box::new(String::from("a")),
        Box::new(Point::default()),
    ];
    // vec = [0, "a", Point { x: 0, y: 0 }]
    vec = vec.into_iter().map(map_any).collect();
    // vec = [1, "a!", Point { x: 1, y: 1 }]
}

mod _enum_version {

    #[derive(Default)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        fn inc(&mut self) {
            self.x += 1;
            self.y += 1;
        }
    }

    enum Stuff {
        Integer(i32),
        String(String),
        Point(Point),
    }

    fn map_stuff(mut stuff: Stuff) -> Stuff {
        match &mut stuff {
            Stuff::Integer(num) => *num += 1,
            Stuff::String(string) => *string += "!",
            Stuff::Point(point) => point.inc(),
        }
        stuff
    }

    #[test]
    fn main() {
        let mut vec = vec![
            Stuff::Integer(0),
            Stuff::String(String::from("a")),
            Stuff::Point(Point::default()),
        ];
        // vec = [0, "a", Point { x: 0, y: 0 }]
        vec = vec.into_iter().map(map_stuff).collect();
        // vec = [1, "a!", Point { x: 1, y: 1 }]
    }
}
