use tokio::sync::{oneshot, mpsc};


pub struct MyActor {
    receiver: mpsc::Receiver<ActorMessage>,
    next_id: u32,
}
pub enum ActorMessage {
    GetUniqueId {
        respond_to: oneshot::Sender<u32>,
    },
    UPUniqueId {
        respond_to: oneshot::Sender<u32>,
    }
}

impl MyActor {
    pub fn new(receiver: mpsc::Receiver<ActorMessage>) -> Self {
        MyActor {
            receiver,
            next_id: 0,
        }
    }
    fn handle_message(&mut self, msg: ActorMessage) {
        match msg {
            ActorMessage::UPUniqueId { respond_to } => {
                self.next_id += 1;
                let _ = respond_to.send(self.next_id);
            },
            ActorMessage::GetUniqueId { respond_to } => {
                self.next_id;
                let _ = respond_to.send(self.next_id);
            }
        }
    }
}

pub async fn run_my_actor(mut actor: MyActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg);
    }
}