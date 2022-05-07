# rust_climb
learn rust the hard way

## cargo 走代理

https://blog.csdn.net/bu2_int/article/details/79758847
>  在 C:\Users\用户名\.cargo 下创建 config 文件内容为

    [http]
    proxy = "127.0.0.1:1080"

    [https]
    proxy = "127.0.0.1:1080"

    注意走代理是要全局配置的，在单个项目里的 cargo.toml 文件里配代理是没用的

## rustup 走 代理

没试过 网上随便找的^-^

> 不推荐使用 步骤如下：
1. 打开Powershell（注意不是cmd）
2. 输入以下文本

$proxy='http://127.0.0.1:1080' 

$ENV:HTTP_PROXY=$proxy 
$ENV:HTTPS_PROXY=$proxy

.\rustup-init.exe
proxy为代理地址


需要注意下面这个方法不适用于PowerShell， 只适用于默认的CMD命令提示符。
>
    Windows 终端使用代理
    # 使用 http 类型代理
    set http_proxy=http://127.0.0.1:8484
    set https_proxy=http://127.0.0.1:8484

    # 使用 socks 类型代理
    netsh winhttp set proxy proxy-server="socks=127.0.0.1:8484" bypass-list="localhost"
    netsh winhttp show proxy
    netsh winhttp reset proxy

    # 使用 socks 类型代理
    set http_proxy=socks5://127.0.0.1:8484
    set https_proxy=socks5://127.0.0.1:8484

这个好用(当前会话管用) ：

>   win10 进入cmd输入：
    set ALL_PROXY=socks5://127.0.0.1:1080
    然后就可以在代理终端使用代理了。    

## 运行某个项目

采用了工作空间结构 可以单独运行某个项目

~~~cmd

cargo run -p favorites 3rd-log
~~~

## 运行某个crate下的example
例如： 运行my_macro 下的examples 目录下的例子
~~~cmd
 cargo run -p my_macro  --example sql
~~~

## rust 测试
- 测试过程中输出代码中的printXxx 
> cargo test  -- --nocapture
或者
> cargo test -- --show-output

跑某个包下的测试
❯ cargo test -p hardway 
带标准输入捕获
❯ cargo test -p web_dev  --  --nocapture  

测试某个包下某个模块下的测试:
比如测试 `hardway` crate下的 模块名路径含有`anys`的测试 并打印标准输出的内容
> cargo test -p hardway  anys  -- --nocapture


## docmentation
生成文档：
>  cargo doc -p mylib  --no-deps --open
为package mylib 生成文档 且不需要为其所依赖的crates生doc 然后在默认浏览器中打开所生文档


## 参考
- [菜鸟教程](https://www.runoob.com/rust/rust-object.html)
- [rust 环境配置](https://www.bilibili.com/video/BV1DV41167xs?p=19)
- [ Exercism practice in Rust trace ](https://github.com/Binlogo/Exercism-Rust-Track) 类似leetcode？
- [  Writing an OS in Rust series](https://github.com/phil-opp/blog_os)
    用rust 写操作系统 https://os.phil-opp.com/

- [rust-learning](https://github.com/ctjhoa/rust-learning)    
- [The Embedded Rust Book](https://rust-embedded.github.io/book/intro/index.html)

- [Rust Language Cheat Sheet](https://cheats.rs/)

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

- 查汇编码
https://rust.godbolt.org/z/8dGbY8Pe1

### 不错的cargo 工具
[cargo-edit](https://github.com/killercup/cargo-edit)
> $ cargo install cargo-edit

### git 冲突
`Your local changes to the following files would be overwritten by merge` 出现这个错误
>
    git pull 代码冲突解决方法：

    1.  git stash ：保存本地代码版本

    2.  git pull ： 更新代码

    3.  git stash pop：合并冲突代码。

    4 . 修改冲突的代码。

方法二、放弃本地修改，直接覆盖

1 git reset --hard
2 git pull

### git 使用代理 
>   git config --global http.proxy socks5://127.0.0.1:1080
    git config --global --get http.proxy  
    git config --global --unset http.proxy

最后一行是重置代理的 第二行是查看是否设置正确了    

### ssh 协议
$ vim ~/.ssh/config

添加以下内容：
Host <host>

# 走 ss 代理
ProxyCommand nc -X 5 -x 127.0.0.1:1080 %h %p

# 走 http 代理
ProxyCommand nc -X connect -x <ip>:<port> %h %p

这里使用了 nc (netcat) 命令，具体的参数解析可以通过 nc -h 查阅。



### blocking waiting for file lock on package cache lock

- clean the cache dir:
> rm ~/.cargo/.package-cache

- 使用代理