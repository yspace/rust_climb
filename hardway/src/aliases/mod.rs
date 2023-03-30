// 两个常用方式:
// - 常用于库 对公共类型提供别名
// - 为复杂的类型的组合提供短类型
pub struct MyStruct;

pub(crate) type MyMap = std::collections::HashMap<String, MyStruct>;