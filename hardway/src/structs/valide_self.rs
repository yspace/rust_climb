use std::{rc::Rc, sync::Arc};

struct Bar;
// @see https://stackoverflow.com/questions/25462935/what-types-are-valid-for-the-self-parameter-of-a-method
impl Bar {
    fn consuming(self)                  { println!("self") }
    fn reference(&self)                 { println!("&self") }
    fn mut_reference(&mut self)         { println!("&mut self") }
    fn boxed(self: Box<Bar>)            { println!("Box") }
    fn ref_count(self: Rc<Bar>)         { println!("Rc") }
    fn atomic_ref_count(self: Arc<Bar>) { println!("Arc") }
}

#[test]
fn main() {
    Bar.consuming();
    Bar.reference();
    Bar.mut_reference();
    Box::new(Bar).boxed();
    Rc::new(Bar).ref_count();
    Arc::new(Bar).atomic_ref_count();
}