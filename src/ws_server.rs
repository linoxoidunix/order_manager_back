use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio_tungstenite::tungstenite::protocol::Message as WsMessage;
use tokio_tungstenite::accept_async;
use futures::{SinkExt, StreamExt};

pub async fn run_ws_server(addr: &str, tx: broadcast::Sender<String>) {
    let listener = TcpListener::bind(addr).await.expect("Can't bind WS server");
    println!("WebSocket server listening on ws://{}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.expect("WS accept failed");
            println!("New WS client connected");
            let (mut ws_sender, mut ws_receiver) = ws_stream.split();

            let client_read = async move {
                while let Some(msg) = ws_receiver.next().await {
                    if let Ok(msg) = msg {
                        if msg.is_text() {
                            println!("Received from client: {}", msg.to_text().unwrap());
                        }
                    } else {
                        break;
                    }
                }
            };

            let client_write = async move {
                while let Ok(msg) = rx.recv().await {
                    println!("📤 Sending to WS client: {}", msg); // Логируем каждое сообщение, которое отправляем клиенту
                    if ws_sender.send(WsMessage::Text(msg)).await.is_err() {
                        break; // клиент отключился
                    }
                }
            };

            tokio::select! {
                _ = client_read => {},
                _ = client_write => {},
            }

            println!("WS client disconnected");
        });
    }
}
