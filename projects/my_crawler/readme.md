from: https://github.com/skerkour/black-hat-rust/tree/main/ch_05/crawler/src/spiders

## run
>  cargo run -p my_crawler -- run --spider=shanghai
## generic 

generic 可以这样用！

~~~rust
// let f = |a,b,c|{
    //     println!("hi");
    //     println!("{} , {} , {}", a, b, c);
    // };

    // f.call((1,2,3)) ;
    // f.call(().combine((1,2,3))) ;
    // f.call(().combine((1,)).combine((2,)).combine((3,))) ;
    // f.call(().combine((2,)).combine(one(3)).combine(one(5))) ;
    // f.call(Product{1,Product{2,3}})
~~~


## 有趣的库：

- https://github.com/plausiblelabs/hlist-rs
- https://github.com/tuguzT/hlist2
- https://github.com/lloydmeta/frunk
  Funktional generic type-level programming in Rust: HList, Coproduct, Generic, LabelledGeneric, Validated, Monoid and friends.


## 黑帽子
程序来自黑帽子 

程序段切割 跟数学题树和树空类似   两个处理节点 需要三个管道连接 类似两颗树 中间跟两边都会形成插空 

## 运行
- 先要运行webdriver
  》 geckodriver --port 4444
  或者
  》  chromedriver --port=4444

  chromedriver 需要跟chrome浏览器版本配套 不然没效果！

- 运行程序
>  cargo run -p my_crawler -- run --spider=quotes


## 参考：
- [jquery中文手册](https://www.w3school.com.cn/jquery/prop_length.asp)
- [jquery 手册](https://www.runoob.com/manual/jquery/)
- [Xpath 详解](https://www.jianshu.com/p/6a0dbb4e246a)

- [Improving async-await's "Future is not Send" diagnostic](https://blog.rust-lang.org/inside-rust/2019/10/11/AsyncAwait-Not-Send-Error-Improvements.html)

- [ Getting in and out of trouble with Rust futures ](https://fasterthanli.me/articles/getting-in-and-out-of-trouble-with-rust-futures)