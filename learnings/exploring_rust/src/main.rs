mod arrays ;
mod bools ;
mod chars ;
mod copy ;
mod clone ;
mod debug ;
mod fmt ;
fn main() {
    println!("Hello, world!");
    fmt::run();

    return ();

    debug::run();

    clone::run();
    bools::main();
    chars::run();
}
