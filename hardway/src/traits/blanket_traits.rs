// 用途 
// 可以为特定的类型作用一个blanket-trait
// 通过trait-bounds为其他trait绑定一个blanket-trait
// 组合trait go语言中的接口实现是值得重视的 接口不应该太臃肿 尽量独立 但是可以组合 比如读｜写是分别的 可以组合为可读可写
// 一个trait实现可能会依赖其他trait trait本身就是抽象手段 可以不妨再更抽象一些 impl<T: Display> ToString for T { // ...}

// 作为marker

trait Blanket {}
impl<T> Blanket for T {}