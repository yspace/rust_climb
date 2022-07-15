
### 地址空间
Why are memory addresses encoded as usize? Surely there’s no 64-bit computer with 264 bytes of RAM. The range of the address space is a façade provided by the OS and the CPU. Programs only know an orderly series of bytes, irrespective of the amount of RAM that is actually available in the system.

### 引用 指针 内存地址

References, pointers, and memory addresses are confusingly similar:
 A memory address, often shortened to address, is a number that happens to refer to a single byte in memory. Memory addresses are abstractions provided by assembly languages.
 A pointer, sometimes expanded to raw pointer, is a memory address that points to a value of some type. Pointers are abstractions provided by higher-level lan- guages.
 A reference is a pointer, or in the case of dynamically sized types, a pointer and an integer with extra guarantees. References are abstractions provided by Rust.

### pointer and memory address

The terms pointer and memory address are sometimes used interchangeably. These are integers that represent a location in virtual memory. From the compiler’s point of view, though, there is one important difference. Rust’s pointer types *const T and *mut T always point to the starting byte of T, and these also know the width of type T in bytes. A memory address might refer to anywhere in memory.

### stack and heal
The critical difference from a usage point of view is that variables on the heap must be accessed via a pointer, whereas this is not required with variables accessed on the stack.