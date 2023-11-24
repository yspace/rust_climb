
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

- [How to Build a Custom Integration Test Harness in Rust](https://devpress.csdn.net/cicd/62ec442b19c509286f4169ff.html) 使用了inventory 构建测试系统

~~~rust

// tests/mod.rs

#[derive(Debug)]
pub struct IntegrationTest {
   pub name: &'static str,
   pub test_fn: fn(),
}

inventory::collect!(IntegrationTest);

// tests/basic.rs

use super::IntegrationTest;

fn basic_test() {
   println!("Running basic test")
}

inventory::submit!(IntegrationTest {
   name: "basic",
   test_fn: basic_test
});

// 可以只运行特定的测试
impl IntegrationTest {
   pub fn all_test_names() -> Vec<&'static str> {
       inventory::iter::<IntegrationTest>
           .into_iter()
           .map(|x| x.name)
           .collect::<Vec<&str>>()
   }

   pub fn from_name<S: AsRef<str>>(test_name: S) -> Option<&'static IntegrationTest> {
       inventory::iter::<IntegrationTest>
           .into_iter()
           .find(|t| t.name == test_name.as_ref())
   }
}


~~~

- https://github.com/dtolnay/linkme

这个库看起来也不错哦  跟inventory 一个作者

也可参考：
https://github.com/elinorbgr/dlib

[👀这里](https://github.com/neon-bindings/neon/blob/2277e943a619579c144c1da543874f4a7ec39879/src/lib.rs#L42)

[Register a Rust Function for Use in Rhai Scripts](https://rhai.rs/book/rust/functions.html)
[ plugin system ](https://rhai.rs/book/plugins/index.html)


- [https://nullderef.com/blog/plugin-tech/](https://nullderef.com/blog/plugin-tech/)
[rust插件系统](https://github.com/marioortizmanero/nullderef.com)
[开始阅读:](https://nullderef.com/series/rust-plugins/)