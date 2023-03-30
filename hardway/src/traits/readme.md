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

## trait 问题

- 污染：太多的trait
比如一个组织中 太多的部门 ； 导致垃圾了
- 重复：重叠了
比如一个部门 几个经理都想串权 各自搞了一套班子 功能几乎一样；如明朝 东厂 西厂 锦衣卫 

如果可能尽量建立在已有的trait之上
如果使用了第三方库 他们中的trait也可能重复 需要我们花大量时间写胶水代码做bridge到我们自己的代码。 对于公有的编程模式 发明trait前 尽量使用已有的（往往可能存在了 比如std库里找一找）对于公共的trait往往可以使用派生宏搞定