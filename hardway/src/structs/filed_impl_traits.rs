// @see https://stackoverflow.com/questions/26212397/how-do-i-specify-that-a-struct-field-must-implement-a-trait?rq=1

trait Foo {}

struct MyFoo;

impl Foo for MyFoo {}

struct Bar<'a> {
    foo: Box<dyn Foo + 'a>,
}

impl<'a> Bar<'a> {
    fn new(the_foo: Box<dyn Foo + 'a>) -> Bar<'a> {
        Bar { foo: the_foo }
    }

    fn get_foo(&'a self) -> &'a dyn Foo {
        &*self.foo
    }
}

fn main() {
    let mybar = Bar::new(Box::new(MyFoo));
}