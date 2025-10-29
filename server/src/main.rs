use std::sync::{Arc, Mutex};

use axum::{
    Extension, Router,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
};
use tokio::sync::broadcast;
use yrs::updates::decoder::Decode;
use yrs::{Doc, ReadTxn, StateVector, Transact, Update};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let doc = Arc::new(Mutex::new(Doc::new()));
    let (tx, _rx) = broadcast::channel::<Vec<u8>>(32);

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .layer(Extension(doc))
        .layer(Extension(tx));

    tracing::info!("Server running on ws://0.0.0.0:9000/ws");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(doc): Extension<Arc<Mutex<Doc>>>,
    Extension(tx_broadcast): Extension<broadcast::Sender<Vec<u8>>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, doc, tx_broadcast))
}

async fn handle_socket(
    mut socket: WebSocket,
    doc: Arc<Mutex<Doc>>,
    tx_broadcast: broadcast::Sender<Vec<u8>>,
) {
    tracing::info!("Client connected");

    // Send full document state to new client
    let full_state = {
        let doc_guard = doc.lock().unwrap();
        let txn = doc_guard.transact();
        txn.encode_state_as_update_v2(&StateVector::default())
    };
    if socket
        .send(Message::Binary(full_state.into()))
        .await
        .is_err()
    {
        return;
    }

    let mut rx = tx_broadcast.subscribe();

    loop {
        tokio::select! {
            Some(Ok(Message::Binary(update_bytes))) = socket.recv() => {
                if let Ok(update) = Update::decode_v2(&update_bytes) {
                    let _ = doc.lock().unwrap().transact_mut().apply_update(update);
                    let _ = tx_broadcast.send(update_bytes.into());
                }
            }

            Ok(broadcast_update) = rx.recv() => {
                if socket.send(Message::Binary(broadcast_update.into())).await.is_err() {
                    break;
                }
            }
        }
    }

    tracing::info!("Client disconnected");
}
