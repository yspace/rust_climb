在rust中，函数和闭包都是实现了Fn、FnMut或FnOnce特质（trait）的类型。任何实现了这三种特质其中一种的类型的对象，都是 可调用对象 ，都能像函数和闭包一样通过这样name()的形式调用，()在rust中是一个操作符，操作符在rust中是可以重载的。rust的操作符重载是通过实现相应的trait来实现，而()操作符的相应trait就是Fn、FnMut和FnOnce，所以，任何实现了这三个trait中的一种的类型，其实就是重载了()操作符


在很多语言中，闭包固定在堆上分配，所以总是进行动态分发。在Rust中，我们可以在栈上分配我们闭包的环境，并静态分发调用

## 题外话
关于闭包 感觉jquery作者的 ninja书籍对js闭包的阐述令人印象深刻


## from <<effictive-rust>>

Rust has three different Fn* traits, which between them express some distinctions around this environment capturing behaviour.

    FnOnce describes a closure that can only be called once. If some part of its environment is moved into the closure, then that move can only happen once – there's no other copy of the source item to move from – and so the closure can only be invoked once.
    FnMut describes a closure that can be called repeatedly, and which can make changes to its environment because it mutably borrows from the environment.
    Fn describes a closure that can be called repeatedly, and which only borrows values from the environment immutably.

The compiler automatically implements the appropriate subset of these Fn* traits for any lambda expression in the code; it's not possible to manually implement any of these traits1 (unlike C++'s operator() overload).

Returning to the rough mental model of closures above, which of the traits the compiler auto-implements roughly corresponds to whether the captured environmental context has:

    FnOnce: any moved values
    FnMut: any mutable references to values (&mut T)
    Fn: only normal references to values (&T).

The latter two traits in the list above each has a trait bound of the preceding trait, which makes sense when you consider the things that use the closures.

    If something only expects to call a closure once (indicated by receiving a FnOnce), it's OK to pass it a closure that's capable of being repeatedly called (FnMut).
    If something expects to repeatedly call a closure that might mutate its environment (indicated by receiving a FnMut), it's OK to pass it a closure that doesn't need to mutate its environment (Fn).

The bare function pointer type fn also notionally belongs at the end of this list; any (not-unsafe) fn type automatically implements all of the Fn* traits, because it borrows nothing from the environment.