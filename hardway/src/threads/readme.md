
Rust #1: Epidemiology model (part 1)
https://www.youtube.com/watch?v=_1TXKRYR1nA&list=PLDL9Y2U6Tu4YeCC4kpp2FslLYqgxJNojN


[guards pattern](https://willcrichton.net/rust-api-type-patterns/guards.html)

## JoinHandle

The function `spawn<F, T>(f: F) -> JoinHandle<T>` returns a JoinHandle.

注意这是范型参数 什么函数都可以往里面塞的！

一般而言spawn一个线程 就像树干上发散的树枝一样 树干就是main函数 树干完了 整个树就完了
这是有散无收 树干需要等树枝（不然树枝来不及生长就死了）    

join 有收拢之意  类似树叶的筋脉 有发散 有收拢 🍂 这样就圆满了 

## Mutex and RwLock

RwLock 或言 读-写锁是并发版本的一个RefCell RwLock<T>持有一个T，并追踪记录任何对外的借用。然而不似RefCell，
它不在冲突的借用上panic。换之，他会阻塞当前线程-让他休眠-一直等到冲突的借用消失。我们只是不得不耐心的等待轮到我们的数据，在其他线程对它做完之后。
借用一个RwLock的内容被称为`locking` 通过锁定它我们临时地阻塞并发冲突借用，允许我们借用它而不导致数据竞争。

Mutex（mutual exclusion 的简称）是非常像的，但概念上略微简单些。它只是允许互斥借用 而不似RwLock那样跟踪共享和互斥借用的数量。

## Atomics
atomic类型代表并发版本的Cell 同Cell一样，它们通过让我们拷贝值以整体的进出 来避免未定义行为，而不是让我们直接借用内容。

也不像Cell那样可以是任何类型，因此也就没有针对任何T的范型Atomic<T>类型 ，但只有特定的原子类型 如AtomicU32 AtomicPtr 那个可用取决于依赖的平台 因其需要来自处理器的支持来避免数据竞争。

### UnsafeCell
An UnsafeCell is the primitive building block for interior mutability.

封装T 无任何条件限制来避免未定义行为，换之 其get方法只是给你一个指向其封装的值的原始指针 此指针只在unsafe块中有意义。

通常 不会直接使用它 而是被其他封装类用来通过有限接口提供安全性。比如Cell或者Mutex 所有有内部可访问性的类型 基本都构建在UnsafeCell之上。

可以配合这个：https://willcrichton.net/rust-api-type-patterns/guards.html

## 线程安全性： Send 和 Sync

有些类只能用在单线程环境 如Rc,Cell 等，因为这些限制被用来避免未定义错误，编译器需要理解某些东西并为你检查把关，这样你可以使用这些类而不需要使用unsafe块。

rust语言使用两类特殊的traits来追踪哪些类可以被安全的用于跨线程环境

`Send`
可发给其他线程。即 如果该类型的值的所有权可被传给其他线程 则该类型是Send的。

`Sync`

可被其他线程共享的类型是Sync的
即 一个类型T是Sync的 当且仅当指向那个类型的共享引用 &T 是Send的

所有的原生类型 如i32，bool str 都是Send和Sync的 这些traits都是auto traits 意味着它们会根据其字段自动为你的类型实现的。 所有字段如果都是Send和Sync的 那么其就是Send和Sync！

from 《Rust Atomics and Locks》
～～～

The way to opt-out of either of these is to add a field to your type that does not implement the trait. For that purpose, the special PhantomData<T> type often comes in handy. That type is treated by the compiler as a T, except it doesn’t actually exist at runtime. It’s a zero sized type, taking no space.

～～～

~~~rust
struct X {
      handle: i32,
_not_sync: PhantomData<Cell<()>>,
  }

~~~
zero-sized PhantomData<Cell<()>> field, which is treated as if it were a Cell<()>. Since a Cell<()> is not Sync, neither is X. It is still Send, however, since all its fields implement Send.

Raw pointers (*const T and *mut T) are neither Send nor Sync, since the compiler doesn’t know much about what they represent.

struct X {
      p: *mut i32,
}
  unsafe impl Send for X {}
  unsafe impl Sync for X {}

Note how implementing these traits requires the unsafe keyword, since the compiler cannot check for you if it’s correct. It’s a promise you make to the compiler, which it will just have to trust.

如果你试图把没有实现Send 的东西move进另一个线程 编译器会礼貌的阻止你的。

~~~rust

fn main() {
let a = Rc::new(123); thread::spawn(move || {
dbg!(a); });
}

~~~

The thread::spawn function requires its argument to be Send, and a closure is only Send if all of its captures are. If we try to capture something that’s not Send our mistake is caught, protecting us from undefined behavior.


## 其他观点：

from [](https://itsallaboutthebit.com/async-simple/)

Arc should be the first thing you try when you need to share stuff between threads, not the last. 
