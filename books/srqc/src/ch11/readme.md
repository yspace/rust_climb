# 

## 11.6.3 Drop VS.Copy

带有析构函数的类型都是不能满足Copy语义
的。因为我们不能保证，对于带析构函数的类型，使用memcpy复制一
个副本一定不会有内存安全问题。所以对于这种情况，编译器是直接禁
止的。
> use std::ops::Drop;
    struct T;
    impl Drop for T {

        fn drop(&mut self){}
    }
    impl Copy for T {}
    fn main() {}

编译器会报错的    