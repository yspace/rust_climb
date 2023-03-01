# The event loop(s)

At the core of all async runtimes (whether it be in Rust, Node.js or other languages) are the event loops, also called processors.
In reality, for better performance, there are often multiple processors per program, one per CPU core.

[different kinds of event loops](https://tokio.rs/blog/2019-10-scheduler)
