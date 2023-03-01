两种不同库 一个用于库 一个用于应用

For libraries, the current good practice is to use the thiserror crate.

For programs, the anyhow crate is the recommended one, it will prettify
errors returned by the main function.

## --
Whether it be for libraries or for applications, errors in Rust are strongly- typed and most of the time represented as enums with one variant for each kind of error our library or program might encounter.