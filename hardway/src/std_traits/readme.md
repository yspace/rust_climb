我们自己的类型 往往要实现一些标准traits 才能跟其他类型进行交互

经常会借助派生宏

~~~rust

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum MyBooleanOption {
        Off,
        On,
    }

~~~

一句话概括标准trait的作用

A rough one-sentence summary of each of the standard traits that this Item covers is:

    Clone: Items of this type can make a copy of themselves when asked.

    Copy: If the compiler makes a bit-for-bit copy of this item's memory representation, the result is a valid new item.

    Default: It's possible to make new instance of this type with sensible default values.

    PartialEq: There's a partial equivalence relation for items of this type – any two items can be definitively compared, but it may not always be true that x==x.

    Eq: There's an equivalence relation for items of this type: any two items can be definitively compared, and it is always true that x==x.

    PartialOrd: Some items of this type can be compared and ordered.

    Ord: All items of this type can be compared and ordered.

    Hash: Items of this type can produce a stable hash of their contents when asked.

    Debug: Items of this type can be displayed to programmers.

    Display: Items of this type can be displayed to users.

👆上面这些trait 除了Display 都可以通过derive注解自动实现 特定情况下 可能需要手动实现或者不实现会是更好的选择


在std::ops 模块 也有一些一元 二元操作符 可以作为trait来复写 这些trait基本都不可以derive 经常用于表示`algebraic`对象的类型上

其他不可以derived的标准traits 如：

Fn, FnOnce and FnMut: Items implementing this trait represent closures that can be invoked. 

Error: Items implementing this trait represent error information that can be displayed to users or programmers, and which may hold nested sub-error information. 

Drop: Items implementing this trait perform processing when they are destroyed, which is essential for RAII patterns. 

From and TryFrom: Items implementing this trait can be automatically created from items of some other type, but with a possibility of failure in the latter case.

Deref and DerefMut: Items implementing this trait are pointer-like objects that can be dereferenced to get access to an inner item. 

Iterator and friends: Items implementing this trait represent collections that can be iterated over.

Send: Items implementing this trait are safe to transfer between multiple threads. 

Sync: Items implementing this trait are safe to be referenced by multiple threads. 




## 参考
- [标准traits](https://www.lurklurk.org/effective-rust/std-traits.html)