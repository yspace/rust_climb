常用的两种手法

1. 大项目打碎成若干crates 以便可受益于rust的增量编译
2. 多使用`cargo check` 而不是 每次都`cargo build` 比如在只改动一个字符后 前者比后者要快很多。

最后尽量减少使用generic 它经常会增加编译时间的