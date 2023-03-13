https://blog.rust-lang.org/2018/05/10/Rust-1.26.html#impl-trait

> Side note for you type theorists out there: this isn't an existential, still a universal. In other words, impl Trait is universal in an input position, but existential in an output position.


## Is there a way to combine multiple traits in order to define a new trait

https://stackoverflow.com/questions/26983355/is-there-a-way-to-combine-multiple-traits-in-order-to-define-a-new-trait

~~~rust


trait NewTrait: Clone + Default + OtherTraits {}
impl<T> NewTrait for T where T: Clone + Default + OtherTraits {}

~~~

https://stackoverflow.com/questions/26070559/is-there-any-way-to-create-a-type-alias-for-multiple-traits

~~~rust

use std::fmt::Display;

trait PartialDisplay: PartialOrd + Display {}

impl<T: PartialOrd + Display> PartialDisplay for T {}

fn print_min<T: PartialDisplay>(a: &T, b: &T) {
    println!("min = {}", if a < b { a } else { b });
}

fn main() {
    print_min(&45, &46);
    print_min(&"aa", &"bb");
}
~~~

Macro for defining trait aliases
https://stackoverflow.com/questions/30291584/macro-for-defining-trait-aliases

~~~rust

macro_rules! trait_alias {
    ($name:ident = $base1:ident + $($base2:ident +)+) => {
        trait $name: $base1 $(+ $base2)+ { }
        impl<T: $base1 $(+ $base2)+> $name for T { }
    };
}

trait Foo { }
trait Bar { }

trait_alias!(Alias = Foo + Bar +);
~~~

[trait-set](https://crates.io/crates/trait-set)

## blanket implementations
~~~rust

impl<T> ToString for T where
    T: Display + ?Sized,
{ ... }
~~~

## impl dyn trait

https://radicle.community/t/rust-s-impl-dyn-trait-syntax/102

~~~rust
trait AsBool {
  fn as_bool(&self) -> bool;

    // 这个方法可能被复写 导致不正确的实现 解决方法是提取出去作为独立逻辑 或者impl dyn trait
  fn is_false(&self) -> bool {
    !self.as_bool()
  }
}

impl dyn AsBool {
    // 这种方法 is_false 不可以被复写掉 。仅在类型转换为dyn AsBool 时才可以使用该方法
  fn is_false(&self) -> bool {
    !self.as_bool()
  }
}
~~~