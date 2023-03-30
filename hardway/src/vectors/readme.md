// from code like a pro in rust

向量

变长值的序列 作为一种通用目的的容器 可以是任何类型的序列（包括其自身类型 即 序列的序列）

是一种可以分配内存在堆上的手段

有一些内部优化来限制过高的内存分配 比如以块分配内存 此外在 nightly rust 中 可以自己指定分配器来实现我们自己的内存分配行为

## 进一步了解

方法继承自 slice 因为我们可以从向量获取一个 slice 引用 虽然 rust 没有 oop 意义的继承，
但向量是一种特殊的类型 既是 vec 也是 slice！
标准库中有个方法是 as_slice

```rust
 pub fn as_slice(&self) -> &[T] {
    self
}
```

我们自己编译是通不过的！ 里面涉及解引用 将一种类型隐式的协变为另一种类型

通过 Deref （和其可变伴侣 DerefMut）编译器隐式的用它们来协变一个类型到另一个类型
如果类型 A 实现了此 trait 可以解引用到类型 B 那么 A 将自动实现所有 B 类型的方法！

对 Vec 的情况

```rust
impl<T, A: Allocator> ops::Deref for Vec<T, A> {
    type Target = [T];
fn deref(&self) -> &[T] {
unsafe { slice::from_raw_parts(self.as_ptr(), self.len) }
} }
impl<T, A: Allocator> ops::DerefMut for Vec<T, A> { fn deref_mut(&mut self) -> &mut [T] {
unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.len) } }
}
```
就是Vec 拥有切片的所有方法！    
值得注意的是 这种操作是暂时的 slice不能是可变size的 提供的长度len也只是在解引用的那个时刻。
