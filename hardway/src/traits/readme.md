Rust’s traits are inspired by Haskell’s type classes.

Trait is a basic language construct which helps to define the interfaces in the program.

## 两种主要用途

- 充当其他语言中的接口概念 。
可以为自己的struct 实现自定义的trait 或者标准库或第三方库中的trait
但不可以为第三方库实现另一方的trait
可为第三方struct实现我们自己的trait

有个孤儿原则 总之至少有一个是来自我们自己的 不能两个都是别人的


- 泛型编程中 trait作为约束出现 。要求泛型类型必须实现某种trait才可以作为参数
这种用法 就是面向抽象编程 可以面向未来才会出现的类型。只要你的类型具备某种特性就可以作为参数
所谓的trait bound 翻译过来：特征约束｜限定？

比如generic 函数中 需要XxxLike|XxxAble|Xxxer|Xxxor 才可以 .
~~~rust

fn some_action<T: SomeTrait>(a: T){

}

~~~
trait 命名可以参考标准库或者知名crate做法
有用动词的 有用ext结尾的 有用like结尾的 有用er｜or able等 


## 两种用法

Once behaviour has been encapsulated into Rust's type system as a trait, there are two ways it can be used:

    as a trait bound, which constrains what types are acceptable for a generic data type or method at compile-time, or

    as a trait object. which constrains what types can be stored or passed to a method at run-time.

A trait bound indicates that generic code which is parameterized by some type T can only be used when that type T implements some specific trait. The presence of the trait bound means that the implementation of the generic can use the methods from that trait, secure in the knowledge that the compiler will ensure that any T that compiles does indeed have those methods. This check happens at compile-time, when the generic is monomorphized (Rust's term for what C++ would call "template instantiation").

This restriction on the target type T is explicit, encoded in the trait bounds: the trait can only be implemented by types that satisfy the trait bounds. This is in contrast to the equivalent situation in C++, where the constraints on the type T used in a template<typename T> are implicit 3: C++ template code still only compiles if all of the referenced methods are available at compile-time, but the checks are purely based on method and signature. (This "duck typing" leads to the chance of confusion; a C++ template that uses t.pop() might compile for a T type parameter of either Stack or Balloon – which is unlikely to be desired behaviour.)

The need for explicit trait bounds also means that a large fraction of generics use trait bounds. To see why this is, turn the observation around and consider what can be done with a struct Thing<T> where there no trait bounds on T. Without a trait bound, the Thing can only perform operations that apply to any type T; this allows for containers, collections and smart pointers, but not much else. Anything that uses the type T is going to need a trait bound.

~~~rust
pub fn dump_sorted<T>(mut collection: T)
where
    T: Sort + IntoIterator,
    T::Item: Debug,
{
    // Next line requires `T: Sort` trait bound.
    collection.sort();
    // Next line requires `T: IntoIterator` trait bound.
    for item in collection {
        // Next line requires `T::Item : Debug` trait bound
        println!("{:?}", item);
    }
}
~~~

So the advice here is to use trait bounds to express requirements on the types used in generics, but it's easy advice to follow – the compiler will force you to comply with it regardless.

A trait object is the other way of making use of the encapsulation defined by a trait, but here different possible implementations of the trait are chosen at run-time rather than compile-time. This dynamic dispatch is analogous to the use of virtual functions in C++, and under the covers Rust has 'vtable' objects that are roughly analogous to those in C++.

This dynamic aspect of trait objects also means that they always have to be handled indirectly, via a reference (&dyn Trait) or a pointer (Box<dyn Trait>). This is because the size of the object implementing the trait isn't known at compile time – it could be a giant struct or a tiny enum – so there's no way to allocate the right amount of space for a bare trait object.

A similar concern means that traits used as trait objects cannot have methods that return the Self type, because the compiled-in-advance code that uses the trait object would have no idea how big that Self might be.

A trait that has a generic method fn method<T>(t:T) allows for the possibility of an infinite number of implemented methods, for all the different types T that might exist. This is fine for a trait used as a trait bound, because the infinite set of possibly invoked generic methods becomes a finite set of actually invoked generic methods at compile time. The same is not true for a trait object: the code available at compile time has to cope with all possible Ts that might arrive at run-time.

These two restrictions – no returning Self and no generic methods – are combined into the concept of object safety. Only object safe traits can be used as trait objects.


## trait 问题

- 污染：太多的trait
比如一个组织中 太多的部门 ； 导致垃圾了
- 重复：重叠了
比如一个部门 几个经理都想串权 各自搞了一套班子 功能几乎一样；如明朝 东厂 西厂 锦衣卫 

如果可能尽量建立在已有的trait之上
如果使用了第三方库 他们中的trait也可能重复 需要我们花大量时间写胶水代码做bridge到我们自己的代码。 对于公有的编程模式 发明trait前 尽量使用已有的（往往可能存在了 比如std库里找一找）对于公共的trait往往可以使用派生宏搞定


## 又一种观点：
[不同形式](https://www.ncameron.org/blog/dyn-trait-and-impl-trait-in-rust/)

trait 不是类型 ，可以绑定到类型上 `T: SomeTrait`
有点像衣服 穿上什么衣服就有什么使能 官服｜环卫工｜护士...   

fn f(b1: impl Bar, b2: impl Bar) -> usize
is equivalent to

fn f<B1: Bar, B2: Bar>(b1: B1, b2: B2) -> usize
not

fn f<B: Bar>(b1: B, b2: B) -> usize

The impl Trait shorthand can only be used for function arguments, it cannot be used for the types of fields or local variables, etc.

imple Trati 简写形式只能用在函数参数上 不能用在字段类型或者本地变量类型等地方。



Generic functions and impl Trait in argument position are implemented using monomorphisation. This means that the compiler makes a copy of the function for each concrete type (or combination of types) that are used to call the function. For example, if our function fn f(b: impl Bar) is called with both Foo and Baz values, then the compiler will make two copies of the function, one which takes b: Foo and one which takes b: Baz.

Consequently, a call to a generic function does not require any indirection, it is a simple function call. However, the function code is duplicated, potentially many times.

impl Trait in return position does not need monomorphisation, the abstract type can simply be replaced with the concrete type in the calling code.

Using trait objects does not require monomorphisation because a function taking a trait object is not a generic function, it only takes a single type. Trait objects themselves are implemented as fat pointers. That means that a type like &dyn Bar is not just a pointer to a value, but is two pointers passed around together (or in a trench coat, if you like): one pointer to the value and one pointer to a vtable which is used to map the methods declared in the trait into methods on the concrete type.

This means that calling a function on a trait object involves an indirection via the vtable, i.e., a dynamic dispatch rather than a simple function call.

Choosing impl Trait or dyn Trait
We have two different types with some similar properties, so how do you choose which to use?

Like most things in software engineering, there is a trade-off:

Advantages of impl Trait or generics:

fine-grained control of properties of types using where clauses,
can have multiple trait bounds (e.g., impl (Foo + Qux) is allowed, but dyn (Foo + Qux) is not),
Disadvantages of impl Trait or generics:

monomorphisation causes increased code size.
Advantages of dyn Trait:

a single variable, argument, or return value can take values of multiple different types.
Disadvantages of dyn Trait:

virtual dispatch means slower method calls,
objects must always be passed by pointer,
requires object safety.
Some more details
Object safety
Not all traits can be made into trait objects, only those which are object safe. Object safety exists so that trait objects can satisfy trait bounds, in other words so that you can pass an object of type &dyn Foo to a function expecting &impl Foo. This might seem trivial, but it isn't. Effectively, there is an implicit impl impl<T: Trait> T for dyn T {...} for all traits; note that the ellipsis here is doing a lot of work, every method must be implemented for every type to delegate to the trait object.

If you were to write out this impl you'd find that it could not be written without errors for some traits. Roughly, object safety is a conservative measure of the traits for which the impl could be written without errors.

A trait is object safe if it is not bound by Sized (e.g, trait Foo: Sized) and for all methods in the trait which do not have Self: Sized in their where clause:

the method is not static (i.e., it has a self argument of some kind),
the method does not use Self in an argument or return type,
the method has no type parameters.


### dyn Trait


类似java的object 或者c++中的 virtrul object 
总是传指针形式（& , Box, ... 其他智能指针） 并随有一个vtable以便于做动态分发

The type of trait objects uses dyn Trait, e.g., &dyn Bar or Box<dyn Bar>. Unlike impl Trait, you cannot use dyn Trait as a type without a wrapping pointer.

At compile time, only the trait bound is known; at runtime any concrete type which implements the trait can be used as a trait object via an implicit coercion, e.g., let obj: &dyn Bar = &Foo { ... };.








## 参考
- [Don't use boxed trait objects](https://bennett.dev/dont-use-boxed-trait-objects-for-struct-internals/)