https://news.ycombinator.com/item?id=23916816

```rust

 use std::cell::UnsafeCell;
    use std::mem::MaybeUninit;
    use std::ops;

    pub struct InitializeCell<T> {
        inner: UnsafeCell<MaybeUninit<T>>,
    }

    unsafe impl<T> Sync for InitializeCell<T> {}

    impl<T> InitializeCell<T> {
        pub const unsafe fn new_uninitialized() -> InitializeCell<T> {
            InitializeCell {
                inner: UnsafeCell::new(MaybeUninit::uninit()),
            }
        }
        pub const fn new(init: T) -> InitializeCell<T> {
            InitializeCell {
                inner: UnsafeCell::new(MaybeUninit::new(init)),
            }
        }
        pub unsafe fn init(&self, init: T) {
            (*self.inner.get()) = MaybeUninit::new(init);
        }
    }

    impl<T> ops::Deref for InitializeCell<T> {
        type Target = T;
        fn deref(&self) -> &T {
            unsafe {
                &*(*self.inner.get()).as_ptr()
            }
        }
    }

    static VALUE: InitializeCell<i32> = unsafe { InitializeCell::new_uninitialized() };
    static OTHER_VALUE: InitializeCell<i32> = InitializeCell::new(6);

    fn main() {
        unsafe {
            VALUE.init(5);
        }

        println!("{}", *VALUE);
        println!("{}", *OTHER_VALUE);
    }
```
