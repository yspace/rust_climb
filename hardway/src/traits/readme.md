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