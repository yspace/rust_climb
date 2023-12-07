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


## webdriver

- https://api.flutter.dev/flutter/package-webdriver_async_io/WebDriver/executeAsync.html

executeAsync method
Future executeAsync(

    String script,
    List args

)

Inject a snippet of JavaScript into the page for execution in the context of the currently selected frame. The executed script is assumed to be asynchronous and must signal that is done by invoking the provided callback, which is always provided as the final argument to the function. The value to this callback will be returned to the client.

Asynchronous script commands may not span page loads. If an unload event is fired while waiting for a script result, an error will be thrown.

The script argument defines the script to execute in the form of a function body. The function will be invoked with the provided args array and the values may be accessed via the arguments object in the order specified. The final argument will always be a callback function that must be invoked to signal that the script has finished.

Arguments may be any JSON-able object. WebElements will be converted to the corresponding DOM element. Likewise, any DOM Elements in the script result will be converted to WebElements.


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
- [xpath 语法](https://www.w3school.com.cn/xpath/xpath_syntax.asp)

- [Improving async-await's "Future is not Send" diagnostic](https://blog.rust-lang.org/inside-rust/2019/10/11/AsyncAwait-Not-Send-Error-Improvements.html)

- [ Getting in and out of trouble with Rust futures ](https://fasterthanli.me/articles/getting-in-and-out-of-trouble-with-rust-futures)