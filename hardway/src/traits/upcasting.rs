// https://stackoverflow.com/questions/28632968/why-doesnt-rust-support-trait-object-upcasting
// crates.io/crates/downcast-rs
// https://crates.io/crates/mopa
trait Base: AsBase {
    // ...
}

trait AsBase {
    fn as_base(&self) -> &Base;
}

impl<T: Base> AsBase for T {
    fn as_base(&self) -> &Base {
        self
    }
}
