
The ownership of a value of a type implementing the Send trait can be transferred safely between threads. Almost every Rust type is Send, with some exceptions, like Rc<T>. If an Rc<T> value is cloned and transferred to another thread, both the threads might update the reference count simultaneously. Therefore, Rc<T> is not Send and can be used in single- threaded cases.

In short, Send means it is safe to transfer ownership of a type from one thread to another, while Sync means the type can be shared safely by multiple threads simultaneously.