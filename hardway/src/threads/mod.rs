use std::{
    sync::{Arc, RwLock},
    thread::spawn,
};

mod copy_types;
pub mod basics;
pub mod scoped;
pub mod arcs;
pub mod send_and_sync;
pub mod mutex;
pub mod thread_pool;

pub fn main() {
    _threads::main();
    _threads::usecase_closure();
    _threads::usecase_value_from_thread();
    _threads::builder();
    _threads::statics();
    _threads::ref_counting();
    return;
    // ==

    basic();
    learn_move();

    copy_types::main();
}

fn basic() {
    let list = vec!["hello", "world"];

    let mut handlers = vec![];
    for i in 0..10 {
        // spawn is short hand for thread::Builder::new().spawn().unwrap()
        let jh = spawn(move || {
            println!("hello from thread: #{}", i);
        });
        handlers.push(jh);
    }

    for handle in handlers {
        handle.join().expect("Could not join thread");
    }

    println!("Result: {}", list.join(","));
}

fn learn_move() {
    let list = vec!["hello".to_string(), "world".to_string()];

    // 创建共享的安全内存结构
    //    let mut rwlock = RwLock::new(&list) ;
    let mut rwlock = RwLock::new(list);
    let mut arc = Arc::new(rwlock);

    let mut handlers = vec![];
    for i in 0..10 {
        let arc_clone = arc.clone();
        let jh = spawn(move || {
            let mut list = arc_clone.write().expect("Could not get write lock!");

            list.push(format!("Thread #{}", i));
            // println!("hello from thread: #{}", i) ;
        });
        handlers.push(jh);
    }

    for handle in handlers {
        handle.join().expect("Could not join thread");
    }

    let list = arc.read().expect("Could not get read loack");

    println!("Result: {}", list.join(","));
}

mod _threads {
    use std::{iter::FromIterator, sync::Arc, thread};
    pub fn main() {
        let t1 = thread::spawn(f);
        let t2 = thread::spawn(f);
        println!("msg from the main thread.");

        // 等
        /**
         * The .join() method will wait until the thread has finished executing, and returns a thread::Result.
         * If the thread did not successfully finish its function because it panicked, this will contain the panic message.
         */
        t1.join().expect("Could not join thread");
        t2.join().unwrap();

        //
        let t_panic = thread::spawn(f_panic);
        match t_panic.join() {
            Ok(rslt) => {}
            Err(err) => {
                println!("err from panic thread: {:?}", err);
            }
        }
    }

    fn f() {
        // The println macro uses io::Stdout::lock() to make sure its output does not get interrupted.
        println!("from child thread!");
        let id = thread::current().id();
        println!("thread id is: {:?}", id);
    }

    fn f_panic() {
        println!("from child thread panic!");

        panic!("hi this thread is paniced!");
    }

    pub fn usecase_closure() {
        let numbers = vec![1, 2, 3];
        // 捕获外部作用域中的变量
        thread::spawn(move || {
            for n in numbers {
                println!("{n}");
            }
        })
        .join()
        .unwrap();
    }

    pub fn usecase_value_from_thread() {
        let numbers = Vec::from_iter(0..=100);
        let t = thread::spawn(move || {
            let len = numbers.len();
            let sum = numbers.into_iter().sum::<usize>();
            sum / len
        });
        let average = t.join().unwrap();
        println!("average: {average}");
    }

    pub fn builder() {
        // 此builder具有更多的可配置性 可防御性 thread::spawn方法是一把梭的
        let mut tb = thread::Builder::new();
        tb = tb.name("my_thread".to_string());
        // tb.stack_size(1024) ;
        //  spawning a new thread fails.
        // This might happen if the operating system runs out of memory, or if resource limits have been applied to your program.
        let rslt = tb.spawn(|| {
            println!("thread name: {}", thread::current().name().unwrap());
        });

        rslt.unwrap().join().unwrap();
    }

    pub fn statics() {
        // 静态变量属于整个应用
        // 一个静态项有一个常量初始化器 永不会drop掉 并且在程序的main方法启动之前存在 每个现场都能借用它 因其被确保永远存在
        static X: [i32; 3] = [1, 2, 3];

        thread::spawn(|| dbg!(&X));
        thread::spawn(|| dbg!(&X));
    }

    pub fn leaking() {
        // 使用leak可以释放一个Box的所有权 承诺永不drop它。从此后 Box会永生，无主 只要程序运行就允许其被任何线程借用
        let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));
        // 注意 静态生命周期不是说从程序开始后它就活着了 只意味它会活到程序结束 过去的只是简单地不相关了

        // leak的缺陷是 我们会内存泄露！我们分配了 但从没有drop或回收它 如果只是有限的几次还好 如果持续这样干
        // 程序会慢慢的耗光内存

        // 此处的move并非转移所有权 只是给了线程一个引用
        // 引用被拷贝 意味当你move它们时 原始的会继续存在 ，就如同一个整数或者boolean值那样
        thread::spawn(move || dbg!(x));
        thread::spawn(move || dbg!(x));
    }

    pub fn ref_counting() {
        // Rc 非线程安全的！没实现Send 和 Sync 因为多个线程同时更新计数器就可能导致不可预测的值
        use std::rc::Rc;
        let a = Rc::new([1, 2, 3]);
        let b = a.clone(); // rc很像box 克隆它不会分配任何新东西 换之 只是更新递增包含其值旁边的计数器
                           //  原始的和新客隆的rc会指向相同的位置 即他们共享了所有权
        assert_eq!(a.as_ptr(), b.as_ptr());
        println!("rc counter is {}", Rc::strong_count(&a));

        // 线程安全版的rc
        use std::sync::Arc;
        let a = Arc::new([1, 2, 3]);
        let b = a.clone();

        // 名称泛滥 不停地clone 会导致不停的变量名出现 如上面的b 通过shadow 可以复用变量名
        thread::spawn({
            let a = a.clone();
            move || dbg!(a)
            // 这样可在不同的线程中使用同名变量 互不干扰还不用为其名犯难
        });

        thread::spawn(move || dbg!(a));
        thread::spawn(move || dbg!(b));
        // Rc<T> and Arc<T>) have the same restrictions as shared references (&T) 没有可变访问权
    }
}
mod scoped_thread {
    #![feature(scoped_threads)]
    use std::thread;

    pub fn scoped() {
        let nums = vec![1, 2, 3];

        // thread::scope(|s|{
        //     s.spawn(||{
        //         println!("len: {}", nums.len());
        //     });
        //     s.spawn(||{
        //         for n in nums{
        //             println!("{n}") ;
        //         }
        //     });
        // });
    }
}

mod borrowing {
    fn f(a: &i32, b: &mut i32) {
        let before = *a;
        *b += 1;
        let after = *a;
        if before != after {
            // x(); // never happens 编译器优化会移除掉这个不可能发生的分支的！
        }
    }
}

mod interior_mutability {
    // 更准确的词： &T -》共享引用  ; &mut T 互斥引用（排外引用）｜独占引用
    // 注意：⚠️ 内部可变性只是针对共享借用的规则弯曲（对可变不可变规则进行了稍微通融  ）允许共享时进行更改 关于
    // 互斥｜独占借用 未做任何改变 。互斥借用仍旧确保没有其他活动借用了 ，创建多个互斥引用（通过unsafe代码）
    // 总是未定义行为 根本不用考虑内部可变性了（遑论内部可变性）

    use std::cell::{Cell, RefCell};

    // a b可能共指一个东西
    fn f(a: &Cell<i32>, b: &Cell<i32>) {
        /*
         * A Cell<T> simply wraps a T,
         * but allows mutations through a shared reference.
         * To avoid undefined behavior, it only allows you to copy the value out (if T is Copy),
         *  or replace it with another value as a whole.
         * In addition, it can only be used within a single thread.
         */
        let before = a.get();
        b.set(b.get() + 1);
        let after = a.get();
        if before != after {
            // x(); // might happen
        }
    }

    fn f2(v: &Cell<Vec<i32>>) {
        let mut v2 = v.take();
        // Replaces the contents of the Cell with an empty Vec v2.push(1);
        v.set(v2); // Put the modified Vec back
    }

    // ====
    // 不同于常规的Cell RefCell 确允许借用其内容 一点点的运行时开销。它除了持有T外还有个计数器用来跟踪外部借用 如果多次做可变借用就会Panic了
    // 同Cell一样 也只能用于单线程 多线程环境中 没啥用
    fn f3(v: &RefCell<Vec<i32>>) {
        v.borrow_mut().push(1); // We can modify the `Vec` directly.
    }
}
