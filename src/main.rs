use crate::tcp_server::TcpClientActorHandle;



mod actor_ex;
mod tcp_server;

#[tokio::main]
async fn main() {
    let actor = TcpClientActorHandle::new();
    let address = "127.0.0.1:6666".to_string();

    match actor.connect(address.clone()).await {
        Ok(()) => println!("Connected to the server"),
        Err(err) => eprintln!("Failed to connect: {:?}", err),
    }

    let data:Vec<u8> = b"Hello, server!".to_vec();
    match actor.send_data(data).await {
        Ok(()) => println!("Data sent successfully"),
        Err(err) => eprintln!("Failed to send data: {:?}", err),
    }

    let data2:Vec<u8> = b"Hello, world!".to_vec();
    match actor.send_data(data2).await {
        Ok(()) => println!("Data sent successfully"),
        Err(err) => eprintln!("Failed to send data: {:?}", err),
    }

    actor.disconnect().await;
    println!("Disconnected from the server");
}