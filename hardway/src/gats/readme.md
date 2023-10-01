GAT
---
https://www.sobyte.net/post/2022-04/rust-gat-async-trait/


~~~rust
pub trait KvIterator {
    /// Get the next item from the iterator.
    async fn next(&mut self) -> Option<(&[u8], &[u8])>;
}
~~~

异步trait 不被支持
需要使用async_trait 库进行重写

~~~rust
#[async_trait]
pub trait KvIterator {
    /// Get the next item from the iterator.
    async fn next(&mut self) -> Option<(&[u8], &[u8])>;
}

/// ... will be rewritten to

pub trait KvIterator {
    /// Get the next item from the iterator.
    fn next(&mut self) -> Box<dyn Future<Output = Option<(&[u8], &[u8])>>>;
}
~~~

但这会带来额外的代价
- dyn trait  有一层间接引用 dynamic scheduling 不可以做编译器优化
- Box    数据存在堆上 next方法需要经常被调用 有些性能耗损

How can we implement async trait with zero overhead? That’s where GAT comes in.

因而引入了GAT！

```rust
pub trait KvIterator {
    type NextFuture<'a>: Future<Output = Option<(&'a [u8], &'a [u8])>>;

    /// Get the next item from the iterator.
    fn next(&mut self) -> Self::NextFuture<'_>;
}
// 因为涉及引用 需要用到生命周期标注 👆代码无法编译 修改如下

pub trait KvIterator {
    type NextFuture<'a>: Future<Output = Option<(&'a [u8], &'a [u8])>>
    where
        Self: 'a;

    /// Get the next item from the iterator.
    fn next(&mut self) -> Self::NextFuture<'_>;
}

```