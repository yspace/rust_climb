// The default visibility (which is equivalent to pub(self)) allows any code within the same module to access and modify the elements within a struct.

pub struct MixedVisibilityStruct {
    pub name: String,
    pub(crate) value: String, // 只针对当前crate内可见
    pub(super) number: i32, // 只在父级域内可见
}
