在rust中，函数和闭包都是实现了Fn、FnMut或FnOnce特质（trait）的类型。任何实现了这三种特质其中一种的类型的对象，都是 可调用对象 ，都能像函数和闭包一样通过这样name()的形式调用，()在rust中是一个操作符，操作符在rust中是可以重载的。rust的操作符重载是通过实现相应的trait来实现，而()操作符的相应trait就是Fn、FnMut和FnOnce，所以，任何实现了这三个trait中的一种的类型，其实就是重载了()操作符


在很多语言中，闭包固定在堆上分配，所以总是进行动态分发。在Rust中，我们可以在栈上分配我们闭包的环境，并静态分发调用

## 题外话
关于闭包 感觉jquery作者的 ninja书籍对js闭包的阐述令人印象深刻