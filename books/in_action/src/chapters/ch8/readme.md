### http

HTML includes a mechanism to supplement or overwrite directives omitted or specified within HTTP: the <meta> tag’s http-equiv attribute. HTTP can make adjust- ments downwards to TCP. The “Connection: keep-alive” HTTP header instructs TCP to maintain its connection after this HTTP message has been received. These sorts of interactions occur all through the stack.

### trait object

Box<dynstd::error::Error>? This is an example of a trait object that enables Rust to support polymorphism at runtime. Trait objects are proxies for concrete types. The syn- tax Box<dyn std::error::Error> means a Box (a pointer) to any type that imple- ments std::error:Error’s

What is a trait object?
Trait objects add a form of polymorphism—the ability to share an interface between types—to Rust via dynamic dispatch. Trait objects are similar to generic objects. Gener- ics offer polymorphism via static dispatch. Choosing between generics and type objects typically involves a trade off between disk space and time:
 Generics use more disk space with faster runtimes.
 Trait objects use less disk space but incur a small runtime overhead caused by
pointer indirection.
Trait objects are dynamically-sized types, which means that these are always seen in the
wild behind a pointer. Trait objects appear in three forms: &dyn Trait, &mut dyn Trait, and Box<dyn Trait>.
The primary difference between the three forms is that Box<dyn Trait> is an owned trait object, whereas the other two are borrowed.

1 In old Rust code, you may see &Trait, and Box<Trait>. While legal syntax, these are officially deprecated. Adding dyn keyword is strongly encouraged.

### DNS

Public DNS providers
At the time of writing, several companies provide DNS servers for public use. Any of the IP addresses listed here should offer roughly equivalent service:
 1.1.1.1 and 1.0.0.1 by Cloudflare
 8.8.8.8 and 8.8.4.4. by Google
 9.9.9.9 by Quad9 (founded by IBM)
 64.6.64.6 and 64.6.65.6 by VeriSign

## Enums

Implementing state machines with Rust’s enums

```rust
enum HttpState {
            Connect,
            Request,
Response, }
        loop {
            state = match state {
HttpState::Connect if !socket.is_active() => { socket.connect();
HttpState::Request
}
HttpState::Request if socket.may_send() => { socket.send(data);
HttpState::Response
}
HttpState::Response if socket.can_recv() => {
received = socket.recv();
                   HttpState::Response
               }

               HttpState::Response if !socket.may_recv() => { break;
}
_ => state, }
}

```
