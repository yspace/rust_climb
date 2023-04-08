[Tour of Rust's Standard Library Traits](https://github.com/pretzelhammer/rust-blog/blob/master/posts/tour-of-rusts-standard-library-traits.md)

[rust 标准库中的 traits 指南](https://rustmagazine.github.io/rust_magazine_2021/chapter_7/rusts-standard-library-traits.html)

https://stackoverflow.com/questions/33116317/when-would-you-use-a-mutex-without-an-arc

```rust
use crossbeam; // 0.7.3
use std::sync::Mutex;

fn main() {
    let data = Mutex::new(vec![0, 1]);

    crossbeam::scope(|scope| {
        // these run concurrently:
        let _guard = scope.spawn(|_| {
            data.lock().unwrap().push(2);
        });
        data.lock().unwrap().push(3);
    })
    .unwrap();

    println!("{:?}", data.lock().unwrap());
    // one of [0, 1, 2, 3] or [0, 1, 3, 2]
}
```
