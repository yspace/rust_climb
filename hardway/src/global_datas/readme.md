
https://github.com/paulkernfeld/global-data-in-rust

堆上的数据 必须要配备allocator
> Heap allocation is convenient because you don't need to know the size of your data at compile time. However, it means that you can't use this method without an allocator. Avoiding heap allocations is most important in embedded programming, real-time systems, and really high-performance applications.


[How to Idiomatically Use Global Variables in Rust](https://www.sitepoint.com/rust-global-variables/)