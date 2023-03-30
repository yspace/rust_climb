#[derive(Debug, Clone, Default)]
struct DebuggableStruct {
    string: String,
    number: i32,
}

impl DebuggableStruct {
    // 消化掉参数
    fn incremented_number(mut self) -> Self {
        self.number += 1;
        self
    }
    // 功能上等价上面的 首选这种方式
    fn increment_number0(&mut self) {
        self.number += 1; 
    }
}
