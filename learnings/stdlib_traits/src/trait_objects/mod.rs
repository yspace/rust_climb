/*

Generics give us compile-time polymorphism where trait objects give us run-time polymorphism. 
We can use trait objects to allow functions to dynamically return different types at run-time

 */
/*
Trait objects are unsized so they must always be behind a pointer. 

struct Struct;
trait Trait {}

// regular struct
&Struct
Box<Struct>
Rc<Struct>
Arc<Struct>

// trait objects
&dyn Trait
Box<dyn Trait>
Rc<dyn Trait>
Arc<dyn Trait>

 */

fn example(condition: bool, vec: Vec<i32>) -> Box<dyn Iterator<Item = i32>> {
    let iter = vec.into_iter();
    if condition {
        // Has type:
        // Box<Map<IntoIter<i32>, Fn(i32) -> i32>>
        // But is cast to:
        // Box<dyn Iterator<Item = i32>>
        Box::new(iter.map(|n| n * 2))
    } else {
        // Has type:
        // Box<Filter<IntoIter<i32>, Fn(&i32) -> bool>>
        // But is cast to:
        // Box<dyn Iterator<Item = i32>>
        Box::new(iter.filter(|&n| n >= 2))
    }
}

use std::f64::consts::PI;

struct Circle {
    radius: f64,
}

struct Square {
    side: f64
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn get_total_area(shapes: Vec<Box<dyn Shape>>) -> f64 {
    shapes.into_iter().map(|s| s.area()).sum()
}

fn example2() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 1.0 }), // Box<Circle> cast to Box<dyn Shape>
        Box::new(Square { side: 1.0 }), // Box<Square> cast to Box<dyn Shape>
    ];
    assert_eq!(PI + 1.0, get_total_area(shapes)); // ✅
}