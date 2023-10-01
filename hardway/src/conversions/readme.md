@see https://www.lurklurk.org/effective-rust/casts.html

Rust type conversions fall into three categories:


    manual: user-defined type conversions provided by implementing the From and Into traits
    手动 通过实现From Into traits来提供类型转换

    semi-automatic: explicit casts between values using the as keyword
    半自动化 值间显式使用`as`

    automatic: implicit coercion into a new type.
    自动化 隐式协变到新类型

The four relevant traits that express the ability to convert values of a type are:

    From<T>: Items of this type can be built from items of type T.
    TryFrom<T>: Items of this type can sometimes be built from items of type T.
    Into<T>: Items of this type can converted into items of type T.
    TryInto<T>: Items of this type can sometimes be converted into items of type T.

TryXxx 系列方法可能会转型失败 所以返回值都是Result<>

### 对称性

T ==> U      T类型能into转换到U 那么U类型就能From T转换而来 
我们一般只需要实现From trait Into可以自动被实现

`implement the From trait for conversions. `
`use the Into trait for trait bounds.`

blanket trait implementation:
```rust
impl<T, U> Into<U> for T
where
    U: From<T>,
{
    fn into(self) -> U {
        U::from(self)
    }
}
```
