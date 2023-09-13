
函数调用 类似打仗
初始时 一个main方法 里面有些开国元勋 调用函数就相当于一场战役 有的战役搞不好就把参数（开国将领）带走了 有的是只读的(&param)
有的会对将领产生伤害(&mut param )

有的编程课程中 把函数调用比喻为黑盒 不需要了解内部细节 只需要知道函数签名 入参出参的类型
按照这种思想 
一个函数内部的实现 可能会调用其他函数 那么就是盒子套多个盒子

这样感觉就像在实验室的工作台上 地上一堆原料盒子装着我们需要的东西
我们把需要的盒子找出来 然后倒出需要的原料 按照某种步骤顺序 完成实验


## 函数相关的类型：

### trait
三种trait的关系 Fn:FnMut:FnOnce
- FnOnce
- FnMut
- Fn

### fn 
此种类型不捕获外部变量 感觉是最纯的函数了
> When a closure does not depend on context at all, the type of our closure is fn


## 如何选择
>    Rust 中，最简单高阶函数一般这样书写：

        fn higher_order_fn<F>(f: F)
        where
            F: Fn() -> i32,
        不理会对 F 的更多约束，考虑在编写高阶函数时，应该选择 FnOnce，FnMut 还是 Fn？
Rust 中的函数也是“unboxed“实现，同样也实现了 Fn 系列 traits

选择依据使用语义：

Fn，函数不保有自己的状态
FnMut，函数可以改变自己的状态
FnOnce，函数消费自己的状态
也就是说：

需要纯函数的时候，书写 Fn

需要函数保存内部状态的时候，如伪随机数生成函数，书写 FnMut

类似于创建线程这样的调用，选择 FnOnce



## 闭包

~~~rust
    let amount_to_add = 3;
    let add_n = |y| {
        // a closure capturing `amount_to_add`
        y + amount_to_add
    };
    let z = add_n(5);
    assert_eq!(z, 8);

~~~

To (roughly) understand how the capture works, imagine that the compiler creates a one-off, internal type that holds all of the parts of the environment that get mentioned in the lambda expression. When the closure is created, an instance of this ephemeral type is created to hold the relevant values, and when the closure is invoked that instance is used as additional context.

~~~rust
    let amount_to_add = 3;
    // *Rough* equivalent to a capturing closure.
    struct InternalContext<'a> {
        // references to captured variables
        amount_to_add: &'a u32,
    }
    impl<'a> InternalContext<'a> {
        fn internal_op(&self, y: u32) -> u32 {
            // body of the lambda expression
            y + *self.amount_to_add
        }
    }
    let add_n = InternalContext {
        amount_to_add: &amount_to_add,
    };
    let z = add_n.internal_op(5);
    assert_eq!(z, 8);

~~~


## 内联函数

impl Struct {
    #[inline]
    pub fn new() -> Self {
        // initialization code
    }

    // const 声明允许编译期评估
     pub const fn new() -> Self {
        // initialization code
     }
} 

- 更长的编译时间
- 少了函数调用的开销

#[inline]并且const是完全不同的东西，一个不是另一个的超集：

    #[inline] 指导编译器不做出是否将函数体内联到调用者的启发式决定。编译器仍然可以自由地完全或在特定的调用站点忽略此属性，如果它出于任何原因选择这样做，因为内联不会以任何可观察的方式改变程序的行为。
    const是函数的签名和部分保证该功能可以（但不一定是在编译时计算）。如果可以，编译器可以选择在编译时评估函数体，但它可以自由地将其移动到运行时。创建函数的原因const是在 const 上下文中评估的可能性是函数签名的一部分，以 semver-stable 方式保证此属性。也就是说，一个可以在 const 上下文中调用的函数不应该在没有 semver-bump 的情况下突然失去这个属性。

以上是const公开记录的函数签名的一部分但#[inline]不是的原因。

所以，不，不要盲目更换#[inline]了const。

FROM： https://qa.1r1g.com/sf/ask/4255907351/

## 参考
- https://zhuanlan.zhihu.com/p/341815515
- https://blog.linyinfeng.com/posts/how-do-rust-closures-work/ 很优秀
- [rust 如何调用动态fn](https://stackoverflow.com/questions/73563278/how-does-rust-call-a-dyn-fn)
