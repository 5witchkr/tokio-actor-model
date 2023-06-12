use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt};
use tokio::sync::{oneshot, mpsc};

pub struct TcpClientActor {
    receiver: mpsc::Receiver<TcpActorMessage>,
    stream: Option<TcpStream>,
}

pub enum TcpActorMessage {
    Connect {
        address: String,
        respond_to: oneshot::Sender<Result<(), std::io::Error>>,
    },
    SendData {
        data: Vec<u8>,
        respond_to: oneshot::Sender<Result<(), std::io::Error>>,
    },
    Disconnect {
        respond_to: oneshot::Sender<()>,
    },
}

impl TcpClientActor {
    pub fn new(receiver: mpsc::Receiver<TcpActorMessage>) -> Self {
        TcpClientActor {
            receiver,
            stream: None,
        }
    }

    async fn connect(&mut self, address: String) -> Result<(), std::io::Error> {
        let stream = TcpStream::connect(address).await?;
        self.stream = Some(stream);
        Ok(())
    }

    async fn send_data(&mut self, data: Vec<u8>) -> Result<(), std::io::Error> {
        if let Some(ref mut stream) = self.stream {
            stream.write_all(&data).await?;
            stream.flush().await?;
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "Not connected to a server",
            ))
        }
    }

    async fn disconnect(&mut self) {
        self.stream = None;
    }

    async fn handle_message(&mut self, msg: TcpActorMessage) {
        match msg {
            TcpActorMessage::Connect { address, respond_to } => {
                let result = self.connect(address).await;
                let _ = respond_to.send(result);
            }
            TcpActorMessage::SendData { data, respond_to } => {
                let result = self.send_data(data).await;
                let _ = respond_to.send(result);
            }
            TcpActorMessage::Disconnect { respond_to } => {
                self.disconnect().await;
                let _ = respond_to.send(());
            }
        }
    }
}

pub async fn run_tcp_client_actor(mut actor: TcpClientActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg).await;
    }
}
