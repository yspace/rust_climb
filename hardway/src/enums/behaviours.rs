// 

//   ## Use the type system to express common behaviour
 
//   in Rust methods can be added to to enum types as well as to struct types


enum Shape {
    Rectangle { width: f64, height: f64 },
    Circle { radius: f64 },
}

impl Shape {
    /**
     * 类似其他语言中的oop风格
     *
     * A &self parameter indicates that the contents of the data structure may be read from, but will not be modified.
     * A &mut self parameter indicates that the method might modify the contents of the data structure.
     * A self parameter indicates that the method consumes the data structure.
     *
     */
    pub fn area(&self) -> f64 {
        match self {
            Shape::Rectangle { width, height } => width * height,
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
        }
    }
}


#[test]
fn test_area(){
    let radius = 2.0f64 ;
    let shap = Shape::Circle { radius };

    assert_eq!(std::f64::consts::PI * radius * radius, shap.area());
}
