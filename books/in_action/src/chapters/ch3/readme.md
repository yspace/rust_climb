There are a number of theoretical differences between methods and functions, but a detailed discussion of those computer science topics is available in other books. Briefly, functions are regarded as pure, meaning their behavior is determined solely by their arguments. Methods are inherently impure, given that one of their argu- ments is effectively a side effect. These are muddy waters, though. Functions are perfectly capable of acting on side effects themselves. Moreover, methods are implemented with functions. And, to add an exception to an exception, objects sometimes implement static methods, which do not include implicit arguments.

### enums
Enums can be a powerful aide in your quest to produce reliable, robust software. Con- sider them for your code when you discover yourself introducing “stringly-typed” data, such as message codes

### traits

What does PartialEq do for types? It enables comparisons with the == operator. “Partial” allows for cases where two values that match exactly should not be treated as equal, such as the floating point’s NAN value or SQL’s NULL.

Note:
When you see a sentence with the following structure, “...T is Debug...”, what they’re saying is that T implements the Debug trait.

### rustdoc

单独用
> rustdoc somefile.rs

为某个crate生成文档
To build an HTML version of the crate’s documentation, follow these steps:
1 Move to the project’s root directory (filebasics/), which includes the Cargo .toml file.
2 Run cargo doc --open.

注意 有很多选项: cargo doc --help 

单独某个包： -p some_package_name
--all
 --no-deps  不需要为依赖创建文档