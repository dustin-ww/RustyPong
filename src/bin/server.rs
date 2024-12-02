use std::error::Error;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{ServerBuilder, WebSocketStream, Message};


async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let ws_stream = Arc::new(Mutex::new(ws_stream));

    let mut bcast_subscriber = bcast_tx.subscribe();
    // TODO: For a hint, see the description of the task below.
    tokio::select! {
        message = ws_stream.next() {
            match message {
                Some(Ok(msg_ok)) => {
                    if let Some(text) = msg_ok.as_text() {
                        println!("From client {}, text : {}", addr, text);
                        bcast_tx.send(text.into())?;
                    }
                }
            }
        }
        msg = bcast_subscriber.recv() => {
            ws_stream.send(Message::text(msg))
        }
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind("127.0.0.1:2000").await?;
    println!("listening on port 2000");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            // Wrap the raw TCP stream into a websocket.
            let ws_stream = ServerBuilder::new().accept(socket).await?;

            handle_connection(addr, ws_stream, bcast_tx).await
        });
    }
}