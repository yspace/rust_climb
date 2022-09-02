
[https://ryhl.io/blog/actors-with-tokio/](https://ryhl.io/blog/actors-with-tokio/)

Before we can talk about how to write an actor, we need to know what an actor is. The basic idea behind an actor is to spawn a self-contained task that performs some job independently of other parts of the program. Typically these actors communicate with the rest of the program through the use of message passing channels. Since each actor runs independently, programs designed using them are naturally parallel.

