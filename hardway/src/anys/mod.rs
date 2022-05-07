pub fn print_type_name<T>(_val: &T) {
    println!("{}", std::any::type_name::<T>());
}

#[test]
fn test_type_id() {
    use std::any::{Any, TypeId};

    fn is_string(s: &dyn Any) -> bool {
        TypeId::of::<String>() == s.type_id()
    }

    assert_eq!(is_string(&0), false);
    assert_eq!(is_string(&"cookie monster".to_string()), true);
}

#[test]
fn test_print_type_name() {
    let s = "cookie monster";   // &str

    print_type_name(&s);

}