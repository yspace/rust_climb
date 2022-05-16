// https://ryhl.io/blog/actors-with-tokio/

mod basic_actor {
    // simple actor

    use tokio::sync::{mpsc, oneshot};

    struct MyActor {
        receiver: mpsc::Receiver<ActorMessage>,
        next_id: u32,
    }

    enum ActorMessage {
        GetUniqueId { respond_to: oneshot::Sender<u32> },
    }

    impl MyActor {
        fn new(receiver: mpsc::Receiver<ActorMessage>) -> Self {
            MyActor {
                receiver,
                next_id: 0,
            }
        }

        fn handle_message(&mut self, message: ActorMessage) {
            match message {
                ActorMessage::GetUniqueId { respond_to } => {
                    self.next_id += 1;
                    let _ = respond_to.send(self.next_id);
                }
            }
        }
    }

    async fn run_my_actor(mut actor: MyActor) {
        while let Some(msg) = actor.receiver.recv().await {
            // we could also match on the enum `ActorMessage` in the run_my_actor function.
            // Each branch in this match could then call various methods such as get_unique_id on the actor object.
            actor.handle_message(msg);
        }
    }

    // ======= ================================
    // A handle is an object that other pieces of code can use to talk to the actor, and is also what keeps the actor alive

    #[derive(Clone)]
    struct MyActorHandle {
        sender: mpsc::Sender<ActorMessage>,
    }

    impl MyActorHandle {
        pub fn new() -> Self {
            let (sender, receiver) = mpsc::channel(8);

            let actor = MyActor::new(receiver);

            tokio::spawn(run_my_actor(actor));
            Self { sender }
        }

        pub async fn get_unique_id(&self) -> u32 {
            let (send, recv) = oneshot::channel();
            let msg = ActorMessage::GetUniqueId { respond_to: send };

            // Ignore send errors. If this send fails, so does the
            // recv.await below. There's no reason to check for the
            // same failure twice.
            let _ = self.sender.send(msg).await;
            recv.await.expect("Actor task has been killed")
        }
    }

    // ======= ================================
}
