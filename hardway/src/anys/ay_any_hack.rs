// @see https://lucumr.pocoo.org/2022/1/7/as-any-hack/

use std::any::Any;
use std::fmt::Debug ;


#[derive(Debug)]
struct AnyBox(Box<dyn DebugAny>);

trait DebugAny: Any + Debug {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

//  implement our super trait for all types implementing the individual traits
impl<T: Any + Debug + 'static> DebugAny for T {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

#[test]
fn main() {
    let any_box = AnyBox(Box::new(42i32));
    dbg!(any_box.0.as_any().downcast_ref::<i32>());
    dbg!((*any_box.0).as_any().downcast_ref::<i32>());

    dbg!(&any_box);
}
