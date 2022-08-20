Closures and functions have different internal representations. Closures are anon- ymous structs that implement the std::ops::FnOnce trait and potentially std::ops::Fn
and std::ops::FnMut. Those structs are invisible in source code but contain any vari- ables from the closure’s environment that are used inside it.

Functions are implemented as function pointers. A function pointer is a pointer that points to code, not data. Code, when used in this sense, is computer memory that has been marked as executable. To complicate matters, closures that do not enclose any variables from their environment are also function pointers.

~~~rs
    let add_to_a =|b|{a+b}; 
    add_to_a == (); // 故意报错 让编译器告诉我们类型信息
~~~


### 

Mutex and Arc are not unified into a single type to provide programmers with added flexibility. Consider a struct with several fields. You may only need a Mutex on a single field, but you could put the Arc around the whole struct. This approach pro- vides faster read access to the fields that are not protected by the Mutex. A single Mutex retains maximum protection for the field that has read-write access. The lock approach, while workable, is cumbersome. Channels offer a simpler alternative.
Channels have two ends: sending and receiving. Programmers don’t get access to what is happening inside the channel. But placing data at the sending end means it’ll appear at the receiving end at some future stage. Channels can be used as a task queue because multiple items can be sent, even if a receiver is not ready to receive any messages

### channels

WHAT CAN BE SENT THROUGH A CHANNEL?
Mentally, you might be thinking of a channel like you would envision a network proto- col. Over the wire, however, you only have the type [u8] available to you. That byte stream needs to be parsed and validated before its contents can be interpreted.
Channels are richer than simply streaming bytes ([u8]). A byte stream is opaque and requires parsing to have structure extracted out of it. Channels offer you the full power of Rust’s type system. I recommend using an enum for messages as it offers exhaustiveness testing for robustness and has a compact internal representation.