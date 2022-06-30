
### 浮点数
二进制实现的浮点数

f32 and f64 types only implement the std::cmp::PartialEq trait, whereas other numeric types also implement std::cmp::Eq.

- Avoid testing floating-point numbers for equality.
- Be wary when results may be mathematically undefined.

### NAN

Operations that produce mathematically undefined results, such as taking the square root of a negative number (-42.0.sqrt()), present particular problems. Floating- point types include “not a number” values (represented in Rust syntax as NAN values) to handle these cases.
NAN values poison other numbers. Almost all operations interacting with NAN return NAN. Another thing to be mindful of is that, by definition, NAN values are never equal

### 特殊的数字类型支持
有理数 复数
特大 特小的数字
用于货币的定点的十进制数字
To access these specialized numeric types, you can use the num crate