
RUST 的日志系统 使用了Facade 设计模式

除了导入log 门面crate 还需要导入一个实现库

想看更全面的日志输出 需要用环境变量控制

cargo run 前面 冠以环境变量设置
> RUST_LOG=debug cargo run -p hardway -- logs

## 重定向
每个程序默认会开三个流 stdout  stderr stdin 对应数字编号 2 3 1
RUST_LOG=info cargo run 2>logs.txt