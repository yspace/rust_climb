### ?

In Rust-like pseudocode, the try! macro could be defined as
 macro try {
  match expression {
    // Uses val when an expression matches Result::Ok(val)
    Result::Ok(val) => val,
    Result::Err(err) => {
<!-- 
Converts err to the outer function’s error type when it matches Result::Err(err) and then returns early -->
let converted = convert::From::from(err);
<!-- Returns from the calling function, not the try! macro itself -->
      return Result::Err(converted);
    }
}); }

### 错误处理

Cheating with unwrap() and expect()

The final approach for dealing with multiple error types is to use unwrap() and expect(). Now that we have the tools to handle multiple error types in a function, we can continue our journey.
NOTE This is a reasonable approach when writing a main() function, but it isn’t recommended for library authors. Your users don’t want their programs to crash because of things outside of their control.
