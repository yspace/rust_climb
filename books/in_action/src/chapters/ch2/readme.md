
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

## 流程控制

| Shorthand|Equivalent to|Access|
|  ----  | ----:  | :----: |
|for item in collection|IntoIterator::into_iter(collection)| Ownership|
| for item in &collection |for item in collection.iter()|Read-only|
|for item in &mut collection|for item in collection.iter_mut()| Read-write|



### 会遇到的特殊类型
 char—A single character encoded as 4 bytes. The internal representation of char is equivalent to UCS-4/UTF-32. This differs from &str and String, which encodes single characters as UTF-8. Conversion does impose a pen- alty, but it means that char values are of fixed-width and are, therefore, eas- ier for the compiler to reason about. Characters encoded as UTF-8 can span 1 to 4 bytes.
 [u8]—A slice of raw bytes, usually found when dealing with streams of binary data.
 Vec<u8>—A vector of raw bytes, usually created when consuming [u8] data. String is to Vec<u8> as str is to [u8].
 std::ffi::OSString—A platform-native string. It’s behavior is close to String but without a guarantee that it’s encoded as UTF-8 and that it won’t contain the zero byte (0x00).
 std::path::Path—A string-like type that is dedicated to handling filesys- tem paths.

 

### Slices
Slices are important because it’s easier to implement traits for slices than arrays. Traits are how Rust programmers add methods to objects. As [T; 1], [T; 2], ..., [T; n] are all different types, implementing traits for arrays can become unwieldy. Creating a slice from an array is easy and cheap because it doesn’t need to be tied to any spe- cific size