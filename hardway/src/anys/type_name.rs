// 这玩意不稳定 最好使用TypeId
fn tname<T: ?Sized>(_v: &T) -> &'static str {
    std::any::type_name::<T>()
}

use std::any::TypeId;

/*
Any already comes with a blanket implementation for every type T:
impl<T: 'static + ?Sized> Any for T {
    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
}
*/
fn type_id<T: 'static + ?Sized>(_v: &T) -> TypeId {
    TypeId::of::<T>()
}

#[test]
fn test_tname() {
    let x = 42u32;
    let tn = tname(&x);
    assert_eq!(tn, "u32");
}
