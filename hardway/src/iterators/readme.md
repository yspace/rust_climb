black hat rust 
---

An Iterator is an object that enables developers to traverse collections.

Iterators can be obtained from most of the collections of the standard library. First, into_iter which provides an owned iterator: the collection is moved.

## Cosuming iterators

iterator 是惰性的

可用 for loop 来消费掉 iterator

但 rust 更倾向函数式风格 更匹配其ownership模型

for_each is the functionnal equivalent of loops：

```rust

fn for_each() {
    let v = vec!["Hello", "World", "!"].into_iter();
    v.for_each(|word| { println!("{}", word);
}); 
}

```


collect can be used to transform an iterator into a collection
collect 方法可用来转化一个迭代器为一个集合类。

~~~rust

fn collect() {
let x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10].into_iter();
let _: Vec<u64> = x.collect(); 
}

~~~

可以用迭代器来构造集合

~~~rust 

fn from_iter() {
let x = vec![(1,2 ), (3, 4), (5, 6)].into_iter();
let _: HashMap<u64, u64> = HashMap::from_iter(x);
 }
~~~


reduce

~~~rust

fn fold() {
let values = vec![1, 2, 3, 4, 5].into_iter();
let _sum = values.reduce(|acc, x| acc + x); }

~~~

fold 类似 但可以返回一个不同于迭代器的单项类型的聚合器  
~~~rust
fn fold() {
let values = vec!["Hello", "World", "!"].into_iter();
let _sentence = values.fold(String::new(), |acc, x| acc + x); }
~~~