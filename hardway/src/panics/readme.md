@see https://github.com/yujinliang/rust_learn/blob/master/panic_in_rust/README.md



## Panic in Rust

Option::None , Result::Err 做unwrap excect 操作会导致panic

panic 会unwind 栈，运行destructor 并保证内存被清理。
Abort 却不会如此，它会依赖os去妥善清理内存。

默认 当panic发生时，程序开始unwinding，意味Rust会回溯栈并清理它中途遇到的每个函数的数据。
这种回溯清理行为是个大工作量的。替换方案是立即abort，此会没有清理动作的结束程序。程序正被使用的内存将会
被操作系统所清理。如果在你自己的项目中需要使得最终的二进制文件尽量小，你可以在面对panic时从unwinding切换到aborting。在cargo.toml 的[profile]  配置段 添加`panic='abort'` 如
`[profile.release] panic = 'abort'`

> RUST_BACKTRACE=1 cargo run 
可以打印详细栈信息

调试是默认开启的 当我们使用cargo build 或者 cargo run 并不是用--release flag时。

编译器提供了一个选项，供用户指定 panic 的实现方式。如下所示：

> rustc -C panic=unwind test.rs  

> rustc -C panic=abort test.rs

如果我们尝试使用 “-C panic=abort” 选项编译代码，可以看到，这个 std::panic::catch_unwind 起不了什么作用。但是，请大家注意，这个 catch_unwind 机制绝对不是设计用于模拟 “try catch” 机制的。请大家永远不要利用这个机制，来做正常的流程控制。它的主要用处在哪里呢，比如下面的这些情况：

- 在FFI场景下的时候，如果说C语言调用了Rust的函数，在Rust内部出现了panic，如果这个panic在Rust内部没有处理好，直接扔到C代码中去，会导致C语言产生“未定义行为（undefined behavior）”。
- 某些高级抽象机制，需要阻止栈展开，比如线程池，如果一个线程中出现了panic，我们希望只把这个线程关闭，而不至于将整个线程池一起被拖下水。

异常安全存在4种层次的保证：
1. No-throw. 这种层次的安全性，保证了所有的异常都在内部正确处理完毕，外部毫无影响。
2. Strong exception safety. 强异常安全保证，可以保证异常发生的时候，所有的状态都可以“回滚”到初始状 态，不会导致状态不一致问题。
3. Basic exception safety. 基本异常安全保证，可以保证异常发生的时候，不会导致资源泄漏。
4. No exception safety. 没有任何异常安全保证。

forget 函数可以阻止一个对象的析构函数调用。FFI用得着！

在rust中，一个panic会终止`当前线程` 但不会发回给并打断主线程。如果主线程panics 它会终止所有的线程，并结束
程序 以code 101.
> 注意：以上两点， panic only terminates the current thread, then return a Result::Err(Any) to the  parent thread.

## 捕获并恢复panics
注意 panic::catch_unwind 函数可能并不会捕获rust中所有的panics。panic在rust中并不是总通过unwinding实现，也可以通过终止进程来实现。 这个函数只是捕获 unwinding panics 不是那些导致进程abort的panics。


## What is unwind safety? that is panic safe.

类比理解 类似参加过一场大战 战争中途panic失败了 这些参战的人员如果回来 是被重新信任 还是一律处斩（因为他们已经不可信 不安全了）

A data structure is in a temporarily invalid state when the thread panics.
This broken invariant is then later observed.

Types such as &mut T and &RefCell are examples which are not unwind safe. The general idea is that any mutable state which can be shared across catch_unwind is not unwind safe by default. This is because it is very easy to witness a broken invariant outside of catch_unwind as the data is simply accessed as usual.

Types like &Mutex, however, are unwind safe because they implement poisoning by default. They still allow witnessing a broken invariant, but they already provide their own "speed bumps" to do so.

> 共享不可变，可变不共享，按照这个Rust最高哲学原则之一来判定， 通常而言那些可变且共享的元素(包括内部可变性)就是不安全的， 故此不满足UnwindSafe 。

> 采用C++RAII模式， 当Panic发生时， 那么在Unwind模式下，Rust保证自动调用每一个栈对象的析构函数(但forget主动放弃析构函数被调用的对象除外) ， 从而保证内存和各种资源的有效释放清理。 但是如果是Abort模式 ， 亦或直接调用了exit()或abort()等系统接口， 则进程当即死亡， 故而Rust 没有自动调用析构函数的机会，内存和资源只能泄露了， 由操作系统打扫战场。

## 参考

https://github.com/yujinliang/rust_learn/tree/master/panic_in_rust