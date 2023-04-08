
Rc: 

单线程引用计数智能指针，允许共享一个对象的所有权


Arc: 

多线程引用计数智能指针 允许跨线程共享对象的所有权



## 内部可变性
应对borrow-checker没有对可变引用提供足够的灵活性 ，像一个逃生舱 但并不会打破rust的安全协议，仍旧允许我们写出安全代码。

为了允许内部可变性 需要引入两个新的类型
  Cell and RefCell.
   
   

   大部分情况 我们会使用RefCell 而不是Cell
   前者允许我们借用引用，后者直接把值移出来了-这并不是我们经常期待的行为


极少情况 我们才会用到这个特性 比如容器或者数据结构其持有的数据需要被可变性的访问

对Cell和RefCell有个限制，只能用于单线程应用

穿越多线程场景时 可以使用Mutex或者RwLock 他们也提供了相同的内部可变性 但是可以跨线程 经常搭配Arc而不是Rc来使用。


## 一些资料：
["Does not live long enough” for borrow()](https://users.rust-lang.org/t/does-not-live-long-enough-for-borrow/27889/4) 

For OOP-like design, you'll usually need to use Rc<RefCell<Client>> or Arc<Mutex<Client>> pretty much everywhere.