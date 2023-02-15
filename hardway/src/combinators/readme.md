[【译文】Rust组合器](https://zhuanlan.zhihu.com/p/342525435c)

[Learning Rust Error Handling Combinators](https://learning-rust.github.io/docs/e6.combinators.html)

[combinators](https://kerkour.com/rust-combinators)
`black hat rust` 作者


- [神奇的代码提示生成](https://jethrogb.github.io/rust-combinators/)
有点省市区 联动的意思

- [有youtube 配套视频哦！](https://github.com/letsgetrusty/combinators)

- [Rust By Example](http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/rust-by-example/error/option_unwrap/map.html)
这个RBE 是mit官网的？
中文翻译地址：https://rustwiki.org/zh-CN/rust-by-example/ 

- [Functional Programming in Rust - Part 2 : Functional Combinators](https://blog.madhukaraphatak.com/functional-programming-in-rust-part-2)
组合子 看来跟函数式编程关系极大 有本rust的书就是 [[Hands-On Functional Programming in Rust]] 有时间可以一观

### 函数式组合子
>  As rust supports functions as first class objects, we can manipulate collections like arrays,     vectors using functional combinator rather than using typical loops structure.
可以认为是loop结构的另一种手法么

所有的函数式组合子在rust中被定义在Iterator trait中 。
所有的内置的集合 如数组 vectors 都实现了迭代器trait 。

Laziness
rust迭代器天然有惰性。所以无论何时我们使用任何组合子 ，它不会立即执行 除非我们调用了特定动作 。

~~~rust

// defining a vector
let v = vec!(1, 2, 3, 4);
// map 组合子
let mapped_v = v.iter().map(|&x| x + 1).collect::<Vec<i32>>();
// filter
let filtered_values = vs.iter().filter(|&x| x%2 ==0).collect::<Vec<&i32>>();
// Count
let vec_count = vector.iter().count();
// zip with index
let index_vec = 0..vec_count;
let index_zipped_vector = vector.iter().zip(index_vec).collect::<Vec<(&i32,usize)>>(); 

// fold
let sum = vector.iter().fold(0,(|sum,value| sum+value));
// max 
let max = vector.iter().max().unwrap();
// for all
let greater_than_zero = vector.iter().all(|&x| x > 0 ) ;

// flatten
let lines_vec = vec!("hello,how","are,you");
let words_vec = lines_vec.iter().flat_map(|&x| x.split(",")).collect::<Vec<&str>>();
~~~
