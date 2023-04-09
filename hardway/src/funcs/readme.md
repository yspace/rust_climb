
函数调用 类似打仗
初始时 一个main方法 里面有些开国元勋 调用函数就相当于一场战役 有的战役搞不好就把参数（开国将领）带走了 有的是只读的(&param)
有的会对将领产生伤害(&mut param )

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


## 参考
- https://zhuanlan.zhihu.com/p/341815515
- https://blog.linyinfeng.com/posts/how-do-rust-closures-work/ 很优秀
- [rust 如何调用动态fn](https://stackoverflow.com/questions/73563278/how-does-rust-call-a-dyn-fn)
