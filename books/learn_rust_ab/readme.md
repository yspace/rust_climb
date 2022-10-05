https://www.tutorialspoint.com/rust/rust_loop.htm

注意本项目中 
routes 跟 commands 模块职责有点混乱

分层模型中 ui层离用户最近？所以routes 应该躲在commands后面 配置或者应用组件也应该是
由command流向routes