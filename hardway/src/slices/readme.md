https://zhuanlan.zhihu.com/p/131015459

1、slice类型
slice是一个没有所有权的数据类型，其允许你引用集合中一段连续的元素序列，而不引用整个集合，通用语义u如下：

 SliceType :
    [ Type ]
请注意与Array types的语义区别

 ArrayType :
    [ Type ; Expression ]
slice是一种动态类型DST（Dynamically Sized Types），无法直接使用slice，都需要将其隐藏在指针后面使用，常用的方式如下：

&[T]：shared slice
&mut [T]：mutable slice
Box<T>：boxed slice


2、动态类型（Dynamically Sized Types）
大部分类型在编译时都有确定的大小，并且编译器会自动为类型实现trait Sized。如果一个类型只能在运行时才能确定大小，则该类型称为 DST（Dynamically Sized Type），也可以说unsized type 。Slices和Trait Objects是两种常见的DSTs。DSTs类型仅能用于下列场景：

指向DSTs的指针类型是Sized的，并且拥有两倍的指针大小
指向Slices的指针同时存储了Slice元素的数量
指向Trait Objects的指针同时存储了一个指向vtable的指针（Trait Objects的定义详见Rust之Trait：https://zhuanlan.zhihu.com/p/127365605）
在有?Sized限定的前提下，DSTs可以作为类型参数使用。默认情况下，任何类型参数都是Sized限定的：
<T> 等价于 <T: Sized>
可以为DSTs实现Traits，不同于类型参数，Trait定义默认为 Self: Sized：
trait Demo {} 等价于 trait Demo: ?Sized {}
Struct可以包含DST的字段（必须在最后一个字段），此时这个struct类型也变成一个DST
 struct DstStruct {
  count: i32,
  data: [u8],
 }
 struct GenericDstStruct<T: ?Sized> {
  count: i32,
  data: T,
 }
因为动态类型缺少静态大小，所以这些类型只能隐藏在指针后面，任何只想DST的指针都会变成胖指针（wide pointer）。

