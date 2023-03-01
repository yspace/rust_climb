// 通过使用derive属性 可以简化我们的trait实现。它本质是一种过程宏（更细分就是派生宏）
// 宏是在编译期 把一些代码作为输入 经过处理后 生成代码

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Point {
    x: u64,
    y: u64,
}

mod v2 {
    use core::fmt::Debug;
    use serde::{Deserialize, Serialize}; // Import the Debug trait
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct Point<T: Debug + Clone + Serialize /*+  Deserialize*/> {
        x: T,
        y: T,
    }
}
