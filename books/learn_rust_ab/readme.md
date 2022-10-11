https://www.tutorialspoint.com/rust/rust_loop.htm

注意本项目中 
routes 跟 commands 模块职责有点混乱

分层模型中 ui层离用户最近？所以routes 应该躲在commands后面 配置或者应用组件也应该是
由command流向routes

## 读书笔记

- 应用 VS 系统编程语言
应用主要是跟user用户交互 而系统编程语言主要跟硬件级别的交互。
- 手动添加rust 进入path
add Rust to your system PATH manually −

> $ source $HOME/.cargo/env

Alternatively, you can add the following line to your ~/.bash_profile −

> $ export PATH="$HOME/.cargo/bin:$PATH"
