use std::mem;

#[derive(Default)]
struct SomeType;
impl SomeType{
    fn new()->Self{
        Self{}
    }
}

struct A {
    field: SomeType,
}

fn foo(a: &mut A) {
    let mut my_local_var = SomeType::new();
    mem::swap(&mut a.field, &mut my_local_var);
}

fn foo2(a: &mut A) {
    let mut my_local_var = mem::replace(&mut a.field, SomeType::new());
}

/**
 * 要求实现 Default
 */
fn foo3(a: &mut A) {
    let mut my_local_var = std::mem::take(&mut a.field);
}


mod option_fileds{
    struct SomeType ;

    struct A {
        field: Option<SomeType>,
    }
    
    fn foo(a: &mut A) {
        let old = a.field.take();
        // a.field is now None, old is whatever a.field used to be
    }
}