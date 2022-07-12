## 位模式

The different mapping between bit strings and numbers explains part of the distinc- tion between binary files and text files. Text files are just binary files that happen to follow a consistent mapping between bit strings and characters. This mapping is called an encoding. Arbitrary files don’t describe their meaning to the outside world, which makes these opaque.

### unsafe

Needlessly using unsafe blocks is heavily frowned upon within the Rust community. It can expose your software to critical security vulnerabilities. Its primary purpose is to allow Rust to interact with external code, such as libraries written in other languages and OS interfaces.

## 大小端


NOTE The abbreviation MSB can be deceptive. Different authors use the same abbreviation to refer to two concepts: most significant bit and most sig- nificant byte. To avoid confusion, this text uses the term bit numbering to refer to the most significant bit and endianness to refer to most significant byte.

### 浮点数
A floating-point value is a container
with three fields:
 A sign bit
 An exponent 
 A mantissa

### rust 模块系统

 Modules are combined into crates.
 Modules can be defined by a project’s directory structure. Subdirectories
under src/ become a module when that directory contains a mod.rs file.
 Modules can also be defined within a file with the mod keyword.
 Modules can be nested arbitrarily.
 All members of a module including its submodules are private by default. Pri-
vate items can be accessed within the module and any of the module’s descendants.


 Prefix things that you want to make public with the pub keyword. The pub key- word has some specialized cases:
a pub(crate) exposes an item to other modules within the crate.
b pub(super) exposes an item to the parent module.
c pub(in path) exposes an item to a module within path.
d pub(self) explicitly keeps the item private.
 Bring items from other modules into local scope with the use keyword.

### function


Within computer science, a function is just a sequence of bytes that can be executed
4
by a CPU. CPUs start at the first opcode, then make their way to the end.