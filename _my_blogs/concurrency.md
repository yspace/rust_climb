Rust provides two types of concurrency 

- classic multi-threading and 

    Multi-threading: Rust’s traditional multi-threading support provides for both shared-memory and message-passing concurrency. Type-level guarantees are provided for sharing of values. Threads can borrow values, assume ownership and transition the scope of a value to a new thread. Rust also
    provides data race safety which prevents thread blocking, improving performance. In order to improve memory efficiency and avoid copying of data shared across threads, Rust provides reference counting as a mechanism to track the use of a variable by other processes/threads. The value is dropped when the count reaches zero, which provides for safe memory management. Additionally, mutexes are available in Rust for data synchronisation across threads. References to immutable data need not use mutex.

- asynchronous I/O.

    Async event-loop based non-blocking I/O concurrency primitives are built into the Rust language with zero-cost futures and async-await. Non- blocking I/O ensures that code does not hang while waiting for data to be processed.