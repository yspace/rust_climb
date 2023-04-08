
/*
trait Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}

Debug has an identical signature to Display. The only difference is that the Debug impl is called when we use the {:?} formatting specifier.
Debug can be derived

Impling Debug for a type also allows it to be used within the dbg! macro which is superior to println!


    dbg! prints to stderr instead of stdout so the debug logs are easy to separate from the actual stdout output of our program.
    dbg! prints the expression passed to it as well as the value the expression evaluated to.
    dbg! takes ownership of its arguments and returns them so you can use it within expressions:

 */

use std::fmt;

// #[derive(Debug)]
struct Point{
    x: i32,
    y: i32,
}

// derive macro generates impl below
impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}


fn some_condition() -> bool {
    true
}

// no logging
fn example() {
    if some_condition() {
        // some code
    }
}

// println! logging
fn example_println() {
    // 🤦
    let result = some_condition();
    println!("{}", result); // just prints "true"
    if result {
        // some code
    }
}

#[test]
// dbg! logging
fn example_dbg() {
    // 😍
    if dbg!(some_condition()) { // prints "[src/main.rs:22] some_condition() = true"
        // some code
    }
}

/*
The only downside is that dbg! isn't automatically stripped in release builds
so we have to manually remove it from our code if we don't want to ship it in the final executable.
 */