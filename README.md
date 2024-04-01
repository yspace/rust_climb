# rust_climb
learn rust the hard way

路虽远 行则将至
事虽难 做则必成

## rust 

升级
> rustup update

## cargo 走代理

https://blog.csdn.net/bu2_int/article/details/79758847
>  在 C:\Users\用户名\.cargo 下创建 config 文件内容为

    [http]
    proxy = "127.0.0.1:1080"

    [https]
    proxy = "127.0.0.1:1080"

    注意走代理是要全局配置的，在单个项目里的 cargo.toml 文件里配代理是没用的

## cargo 相关
- cargo 书： [指定依赖的路径 或者仓库](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/cargo/reference/specifying-dependencies.html)
[官方书](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)

## rustup 走 代理

rustup 管理rust的安装 关心的是rust工具链 可以使我们在不同的编译器版本间移动
它也能简单的访问rust的文档 可以打开rust标准库的本地拷贝

> rustup doc

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

## toolchain 切换
toolchain 是平台 架构 通道的组合。

~~~sh
# Runs tests with stable channel: 
$ cargo +stable test
...
# Runs tests with nightly channel: 
$ cargo +nightly test
~~~
所有的cargo 命令都支持 +stable|+nightly 选项
发布项目前 可以在这两个通道上都测试下 有的项目可能只使用其中一个

想对某个项目临时切换通道的话可以使用override选项
~~~sh
 # 只针对当前目录及其子目录
$ rustup override set nightly
~~~
这样你全局仍默认仍可以使用stable通道 

## rust 测试
- [a snapshot testing library for Rust](https://github.com/mitsuhiko/insta)


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

## 帮助工具
- [代码生成](https://transform.tools/json-to-rust-serde)
通过json 转化为其他语言中的结构 神器！

## 参考
- [rust 标准库内幕](https://github.com/Warrenren/inside-rust-std-library/)
- [rust rfc book](https://rust-lang.github.io/rfcs/2057-refcell-replace.html)

- [菜鸟教程](https://www.runoob.com/rust/rust-object.html)
- [rust 环境配置](https://www.bilibili.com/video/BV1DV41167xs?p=19)
- [ Exercism practice in Rust trace ](https://github.com/Binlogo/Exercism-Rust-Track) 类似leetcode？
- [  Writing an OS in Rust series](https://github.com/phil-opp/blog_os)
    用rust 写操作系统 https://os.phil-opp.com/
    [async await](https://os.phil-opp.com/async-await/)

- [rust-learning](https://github.com/ctjhoa/rust-learning)    
- [The Embedded Rust Book](https://rust-embedded.github.io/book/intro/index.html)
- [Too many lists](https://rust-unofficial.github.io/too-many-lists/index.html)
- [Educational blog posts for Rust beginners ](https://github.com/pretzelhammer/rust-blog)

- [Rust Language Cheat Sheet](https://cheats.rs/)

- [rust by example](https://rustwiki.org/zh-CN/rust-by-example/mod/visibility.html)

- [Learning Rust the wrong way](https://www.youtube.com/watch?v=DL9LANLg5EA&ab_channel=NDCConferences) 这里介绍了学习新事物的几种方法对比 很有意思 

- [rust-tutorial](https://www.tutorialspoint.com/rust/index.htm)
- [Rust Lifetimes](https://www.youtube.com/watch?v=1QoT9fmPYr8&ab_channel=DougMilford) 据说是生命周期解说最好的视频
- [带生命周期的trait ｜ trait bound](https://serde.rs/lifetimes.html)

- [The "24 days of Rust" article series.](https://zsiciarz.github.io/24daysofrust/book/vol2/day6.html)

- [rust-cookbook](https://rust-lang-nursery.github.io/rust-cookbook/intro.html)
    例子不错 
- [Rust RFCs - RFC Book - Active RFC List](https://rust-lang.github.io/rfcs/introduction.html)

- [design-patterns-rust](https://github.com/fadeevab/design-patterns-rust)

- [From JavaScript to Rust ebook](https://github.com/wasmflow/node-to-rust)

- [inside-rust-std-library](https://github.com/Warrenren/inside-rust-std-library)

- [rust 标准库指南](https://rustmagazine.github.io/rust_magazine_2021/chapter_7/rusts-standard-library-traits.html)
[原文](https://github.com/pretzelhammer/rust-blog/blob/master/posts/tour-of-rusts-standard-library-traits.md)


- [rust learn](https://github.com/yujinliang/rust_learn)

来自：Rust Programmer，Rust Fans, C++老兵

- [微软rust培训教程](https://learn.microsoft.com/zh-cn/training/modules/rust-create-program/0-introduction)


- [effective-rust](https://www.lurklurk.org/effective-rust/use-types.html)
- [std标准库 stdio 阅读](https://rustwiki.org/zh-CN/src/std/io/stdio.rs.html)

- [fantoccini 源码阅读](https://github.com/jonhoo/fantoccini/)
### 插件系统
- [A Plugin System in Rust](https://nullderef.com/series/rust-plugins/)

- [Implementing plugins with dynamically loaded libraries and Rust](https://github.com/Michael-F-Bryan/plugins_in_rust)
- [This example is the simplest plugin I could think of that is both written and loaded with Rust under the C ABI.](https://github.com/marioortizmanero/pdk-experiments/tree/master/dynamic-simple)
  https://github.com/marioortizmanero/pdk-experiments
  来自[tremor](https://github.com/tremor-rs)
- https://github.com/nagisa/rust_libloading
- https://docs.rs/dlopen/0.1.8/dlopen/
- https://github.com/emoon/dynamic_reload
- https://github.com/rodrimati1992/abi_stable_crates

- [So you want to live-reload Rust](https://fasterthanli.me/articles/so-you-want-to-live-reload-rust)
- [Plugins in Rust](https://adventures.michaelfbryan.com/posts/plugins-in-rust/)

- [Dynamic Loading & Plugins](https://michael-f-bryan.github.io/rust-ffi-guide/dynamic_loading.html)
- [](https://zicklag.github.io/rust-tutorials/rust-plugins.html)
- [bevy-dynamic-plugin](https://github.com/bevyengine/bevy/blob/v0.5.0/crates/bevy_dynamic_plugin/src/loader.rs)


## 项目布局
- [dtool](https://github.com/guoxbin/dtool)
- [cargo]()

## 不错的文章
- [12 Killer Rust Libraries You Should Try](https://medium.com/@jondot/12-killer-rust-libraries-you-should-know-c60bab07624f)

- [Railway Oriented Programming](https://fsharpforfunandprofit.com/rop/)
深刻理解Result
https://swlaschin.gitbooks.io/fsharpforfunandprofit/content/posts/recipe-part2.html

- [Against Railway-Oriented Programming](https://fsharpforfunandprofit.com/posts/against-railway-oriented-programming/)
跟这个或许有关： Rust组合器 [Combinators](https://learning-rust.github.io/docs/combinators/)

## todos
- https://blog.ndepend.com/hexagonal-architecture/ 六边形架构试试呦！
- 

## IDE 
vscode 下可以安装IDEA Jetbrains快捷键

- 撤销 Ctrl+Z 取消撤销是 Ctr+shift+Z
MacOS 的 Command + Z 取消操作是 Command + Shift + Z。

## 杂项
- [rust-lang/rfcs](https://github.com/rust-lang/rfcs/tree/master/text)
学某个知识点 如果有时间可以先在这里找找最初的动机与实现想法！``这是个被忽略的学习路径!``

- rustc version 查看当前rustc版本
> rustup show 

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

- 奇怪的错误：
https://stackoverflow.com/questions/74045131/error-linking-with-cc-failed-exit-status-1-when-building-rust-program-on-ma

>   = note:   error: could not compile `syn` due to previous error error: linking with `cc` failed: signal: 11 (SIGSEGV)

macos 独有现象 
做： （`xcode-select reset` Reset your Xcode Install on macOS)[https://gist.github.com/Justintime50/2349ff5e62555aa097acbf519bbc27af] 后正常

先确保安装了 c 编译器
> xcode-select --install


### 查看库features
cargo tree --edges features

### 不错的cargo 工具
[cargo-edit](https://github.com/killercup/cargo-edit)
> $ cargo install cargo-edit

添加依赖的crate
> cargo add num


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

- 可以手动 删掉~/.cargo 下的一些源码 有的太占空间了
> rm -R ~/.cargo/registry
> open ~/.cargo/registry/src

- 使用代理