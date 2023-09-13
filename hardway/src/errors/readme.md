针对Result<T,E> 中的E
---

在Result枚举中 E枝叉经常是std::error::Error trait 
虽然没有强制 但使用此类型跟其他库的互操作性会强一些 也可以使用`?`问号语法糖。

在非std环境 嵌入式等环境 错误可能会是其他类型 比如整数之类

## 错误类型设计总原则
- 库的错误 越详尽越好 （thiserror？｜color_eyre 《-后面这个没仔细考察）
- 应用级的 可以适当口大些 可以吞并不可预知的内部错误 并对用户适当进行屏蔽转化（anyhow）

## 错误类型

错误类型经常会很多 

从纵横两个维度看 错误之间存在并列关系 或者嵌套关系 
比如顶层调用底层依赖库的时候 不同的层可能有不同的错误类型 此时顶层在遇到底层返回的错误时 可以选择隐藏底层错误 完全转化为顶层语义的错误描述 或者把底层错误内嵌到自己的成员中（这对排错很有帮助）

在表单验证场景 经常出现多个表单项需要都验证一起返回一个结果 如果有错误 可以设计成聚合错误 包括多个验证错误类型（如 name字段不能为空，email不合法...）

## 嵌套错误

更底层的错误应该保留并对调用者可用 

这有点类似网络协议一样 只在外面不停加对本层有用的信息 内部原封不动的传递下去

错误传递类似冒泡 不停往更上层浮去 如果都不解决 最终就跑到main啦 

保留内部错误有两种做法
- 使用format! 将内部错误统一转化为字符串
- 使用枚举 需要覆盖不同的子错误类型 
后一种比较烦 需要考虑各种错误情况 

存在第三种选择！

Encoding the sub-error information as a trait object avoids the need for an enum variant for every possibility, but erases the details of the specific underlying error types.
~~~rust

#[derive(Debug)]
pub enum WrappedError {
    Wrapped(Box<dyn Error>),
    General(String),
}

impl std::fmt::Display for WrappedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wrapped(e) => write!(f, "Inner error: {}", e),
            Self::General(s) => write!(f, "{}", s),
        }
    }
}
~~~

A putative WrappedError would naively be expected to both implement the Error trait, and also to implement the From<Error> trait to allow sub-errors to be easily wrapped. That means that a WrappedError can be created from an inner WrappedError, as WrapperError implements Error, and that clashes with the blanket reflexive implementation of From


`anyhow` 解决了👆这个问题


## 应用级错误 vs 库级错误

Code that's written for a library can't predict the environment in which the code is used, so it's preferable to emit concrete, detailed error information, and leave the caller to figure out how to use that information. This leans towards the enum-style nested errors 

However, application code typically needs to concentrate more on how to present errors to the user. It also potentially has to cope with all of the different error types emitted by all of the libraries that are present in its dependency graph . As such, a more dynamic error type (such as anyhow::Error) makes error handling simpler and more consistent across the application.

## 总结
Summary

This item has covered a lot of ground, so a summary is in order:

    The standard Error trait requires little of you, so prefer to implement it for your error types.
    When dealing with heterogeneous underlying error types, decide whether preserving those types is needed.
        If not, consider using anyhow to wrap sub-errors in application code.
        If they are needed, encode them in an enum and provide conversions. Consider using thiserror to help with this.
    Consider using the anyhow crate for convenient, idiomatic error handling.

It's your decision, but whatever you decide, encode it in the type system 

## 参考
-  [effective-rus](https://www.lurklurk.org/effective-rust/errors.html)
- [Rust: Structuring and handling errors in 2020](https://nick.groenen.me/posts/rust-error-handling/)