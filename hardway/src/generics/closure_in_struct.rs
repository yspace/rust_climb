mod unboxed_closure {
    struct Foo<F>
    where
        F: Fn(usize) -> usize,
    {
        pub foo: F,
    }

    impl<F> Foo<F>
    where
        F: Fn(usize) -> usize,
    {
        fn new(foo: F) -> Self {
            Self { foo }
        }
    }
    #[test]
    fn main() {
        let foo = Foo { foo: |a| a + 1 };
        (foo.foo)(42);

        (Foo::new(|a| a + 1).foo)(42);
    }
}

mod boxed_trait_object {
    struct Foo {
        pub foo: Box<dyn Fn(usize) -> usize>,
    }

    impl Foo {
        fn new(foo: impl Fn(usize) -> usize + 'static) -> Self {
            Self { foo: Box::new(foo) }
        }
    }

    #[test]
    fn main() {
        let foo = Foo {
            foo: Box::new(|a| a + 1),
        };
        (foo.foo)(42);

        (Foo::new(|a| a + 1).foo)(42);
    }
}

mod trait_object_reference {
    struct Foo<'a> {
        pub foo: &'a dyn Fn(usize) -> usize,
    }

    impl<'a> Foo<'a> {
        fn new(foo: &'a dyn Fn(usize) -> usize) -> Self {
            Self { foo }
        }
    }

    #[test]
    fn main() {
        let foo = Foo { foo: &|a| a + 1 };
        (foo.foo)(42);

        (Foo::new(&|a| a + 1).foo)(42);
    }
}

mod function_pointer {
    struct Foo {
        pub foo: fn(usize) -> usize,
    }

    impl Foo {
        fn new(foo: fn(usize) -> usize) -> Self {
            Self { foo }
        }
    }

    fn main() {
        let foo = Foo { foo: |a| a + 1 };
        (foo.foo)(42);

        (Foo::new(|a| a + 1).foo)(42);
    }
}
