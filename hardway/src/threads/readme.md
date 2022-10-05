
Rust #1: Epidemiology model (part 1)
https://www.youtube.com/watch?v=_1TXKRYR1nA&list=PLDL9Y2U6Tu4YeCC4kpp2FslLYqgxJNojN



## JoinHandle

The function `spawn<F, T>(f: F) -> JoinHandle<T>` returns a JoinHandle.

注意这是范型参数 什么函数都可以往里面塞的！