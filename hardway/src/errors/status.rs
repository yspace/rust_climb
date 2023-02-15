///
/// 使用enumflags2::BitFlags 完成自动转化 拥有位操作能力！
/// ~~~rust
///  #derive(BitFlags,Copy , Clone, Debug, PartialEq)
/// #[repr[u8]]
/// enum MyStatus{
///     A = 0b0001,
///     B = 0b0010,
///     C = 0b0100,
/// }
/// ~~~
#[derive(Debug)]
pub enum ErrorStatus{
    Success = 0b00000000,
    /// 通用错误类型
    Error = 0b000000001,
    /// 用户取消
    Cancel = 0b000000010,
    None = 0b000000100,
}
// @todo plant_uml
// > [preprocessor.plantuml]
// > plantuml-cmd="plantuml.exe"
// 如果同时有Error 和 Cancel , 就按位求或， 组合成  0011
// 然后你判断时 再用     Ret & ErrorStatus::Error  按位求与， 判定结果是0 还是非0 就知道Ret里边有没有 你要想的 比如 ErrroStatus：：Error这个值了