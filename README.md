# rust_climb
learn rust the hard way


## 运行某个项目

采用了工作空间结构 可以单独运行某个项目

~~~cmd

cargo run -p favorites 3rd-log
~~~

## 参考
- [菜鸟教程](https://www.runoob.com/rust/rust-object.html)
- [rust 环境配置](https://www.bilibili.com/video/BV1DV41167xs?p=19)
- [ Exercism practice in Rust trace ](https://github.com/Binlogo/Exercism-Rust-Track) 类似leetcode？
- [  Writing an OS in Rust series](https://github.com/phil-opp/blog_os)
    用rust 写操作系统 https://os.phil-opp.com/

- [rust-learning](https://github.com/ctjhoa/rust-learning)    

## 项目布局
- [dtool](https://github.com/guoxbin/dtool)
- [cargo]()

## 不错的文章
- [12 Killer Rust Libraries You Should Try](https://medium.com/@jondot/12-killer-rust-libraries-you-should-know-c60bab07624f)

## todos
- https://blog.ndepend.com/hexagonal-architecture/ 六边形架构试试呦！
- 

## 杂项
- ssl错误：
今天下载 cargo install starship     时报ssl错误 找到网上的解决方案 管用呀
  > 解决方案：
    在~/.cargo/config中加入
     
    [http]
    check-revoke = false
    
    或者：
    调置环境变量CARGO_HTTP_CHECK_REVOKE=false