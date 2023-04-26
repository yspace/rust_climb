
- https://users.rust-lang.org/t/function-type-registration-at-startup/23458/10




~~~rust

// OptimisticPeach

// 1

// schets
// Oct '21
// Actually, that particular crate wouldn't work all that well in this instance: the upstream crate must be made aware of the type ahead of time in its own source code.

// Now that I look back on it, I'd say that you might want to look at something like AnyMap combined with #[ctor] functions. Or, if you wish to run the initialization code only once, then try something like an AnyMap which is full of fn() -> T and then pipe the T into a local AnyMap.

// Or... on second thought it might be reasonable to use inventory here:

struct Initializer {
    f: fn(&mut AnyMap),
}

impl Initializer {
    fn for<T: Default>() -> Self {
        Self { 
            f: |map| map.insert::<T>(T::default())
        }
    }
}

inventory::collect!(Intializer);
And then in downstream crates, call

inventory::submit! {
    Initializer::for::<MyType>()
}

~~~

- https://github.com/dtolnay/linkme

这个库看起来也不错哦  跟inventory 一个作者

[Register a Rust Function for Use in Rhai Scripts](https://rhai.rs/book/rust/functions.html)

[ plugin system ](https://rhai.rs/book/plugins/index.html)