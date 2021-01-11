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