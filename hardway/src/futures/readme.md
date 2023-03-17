
rust 中的异步块 实际在编译期被转化为了实现Future特征的结构体 有个pool方法 这是他们的共同抽象
又说 async块会返回一个Future ；比如 
~~~rust

    fn give_me_a_future()->Future<Xxx>{
        async {

        }
    }

    asnyc some_foo(){
        ...
    } 等价于返回一个Futrue<()>

~~~

调用者通过await方法 指示runtime去执行它 还有其他方法开始一个future 如tokio的join! 宏

rust 只提供了语法和类型 至于实现留给了社区

异步实现有两类：
- kernel 层目前是mio
- runtime 层   两种最常用的是tokio 和 async-std