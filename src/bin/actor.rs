//! Implement a Actor model in Rust.
// Before starting, run following command in command line to add `tokio`
// dependency to your cargo crate with features `full`.
// `$ cargo add tokio --features full`

use tokio::sync::{mpsc, oneshot};

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

struct Actor {
    receiver: mpsc::Receiver<ActorMessage>,
}

enum ActorMessage {
    ShutDown { respond_to: oneshot::Sender<&'static str> },
    Echo { msg: String, respond_to: oneshot::Sender<String> },
}

impl Actor {
    fn new(receiver: mpsc::Receiver<ActorMessage>) -> Self {
        Self {receiver}
    }

    async fn clear_up(&mut self) {
        self.receiver.close();
    }

    async fn handle_message(&mut self, msg: ActorMessage) -> Result<()> {
        match msg {
            ActorMessage::ShutDown { respond_to: _ } => {
                panic!("Control flow should never reach here");
            }
            ActorMessage::Echo { msg, respond_to } => {
                let _ = respond_to.send(msg);
                Ok(())
            }
        }
    }
}

#[derive(Clone)]
struct ActorHandle {
    sender: mpsc::Sender<ActorMessage>,
    // sender: async_channel::Sender<ActorMessage>,
}

impl ActorHandle {
    fn new(sender: mpsc::Sender<ActorMessage>) -> Self {
        Self { sender }
    }

    async fn echo(&self, msg: String) -> String {
        let (send, recv) = oneshot::channel();
        let msg = ActorMessage::Echo { msg, respond_to: send };

        let _ = self.sender.send(msg).await;
        recv.await.expect("Actor task has been killed")
    }

    async fn shutdown(&self) {
        let (send, recv) = oneshot::channel();
        let msg = ActorMessage::ShutDown { respond_to: send };

        let _ = self.sender.send(msg).await;
        recv.await.expect("Fail to kill actor");
    }
}

async fn run_my_actor(mut actor: Actor) {
    while let Some(msg) = actor.receiver.recv().await {
        match msg {
            ActorMessage::ShutDown { respond_to } => {
                let _ = respond_to.send("Actor shutdown");
                actor.clear_up().await;
                break;
            }
            _ => actor.handle_message(msg).await.unwrap(),
        }
    }
}

fn build() -> (Actor, ActorHandle) {
    let (sender, receiver) = mpsc::channel(64);
    (Actor::new(receiver), ActorHandle::new(sender))
}

#[tokio::main]
async fn main() {
    let (actor, handle) = build();
    tokio::spawn(run_my_actor(actor));
    for i in 0..10 {
        let s = handle.echo(i.to_string()).await;
        println!("{}", s);
    }

    handle.shutdown().await;
}