use tokio::sync::{mpsc, oneshot};


use super::tcp_actor::{TcpClientActor, run_tcp_client_actor, TcpActorMessage};



#[derive(Clone)]
pub struct TcpClientActorHandle {
    sender: mpsc::Sender<TcpActorMessage>,
}

impl TcpClientActorHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = TcpClientActor::new(receiver);
        tokio::spawn(run_tcp_client_actor(actor));

        Self { sender }
    }

    pub async fn connect(&self, address: String) -> Result<(), std::io::Error> {
        let (send, recv) = oneshot::channel();
        let msg = TcpActorMessage::Connect {
            address,
            respond_to: send,
        };

        let _ = self.sender.send(msg).await;
        recv.await.unwrap()
    }

    pub async fn send_data(&self, data: Vec<u8>) -> Result<(), std::io::Error> {
        let (send, recv) = oneshot::channel();
        let msg = TcpActorMessage::SendData {
            data,
            respond_to: send,
        };

        let _ = self.sender.send(msg).await;
        recv.await.unwrap()
    }

    pub async fn disconnect(&self) {
        let (send, recv) = oneshot::channel();
        let msg = TcpActorMessage::Disconnect {
            respond_to: send,
        };

        let _ = self.sender.send(msg).await;
        let _ = recv.await;
    }
}