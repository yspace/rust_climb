// 有的设计中是吧模型搞到项目根目录去了 让所有modules 共享 貌似也是可以的 只不过好像会越来越多

#[derive(Debug,Default)]
pub struct User {
    name : String,
}