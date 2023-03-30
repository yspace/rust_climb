use std::ops::Deref;


// 惯用法 因为Vec是首选用来实现任意类型的可变序列的方式
pub struct MyString {
    vec: Vec<u8>,
}

impl Deref for MyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        // 实现细节看看 String类
        // unsafe { str::from_utf8_unchecked(&self.vec) }
        todo!()
    }
}