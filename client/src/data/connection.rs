use dioxus::prelude::*;
use futures_timer::Delay;
use futures_util::{select, FutureExt, SinkExt, StreamExt};
use reqwest::Client;
use reqwest_websocket::RequestBuilderExt;
use std::time::Duration;

use crate::data::Graph;
#[derive(Clone)]
enum Message {
    SendUpdate(Vec<u8>),
}

#[derive(Clone, PartialEq)]
pub struct Connection {
    graph: Graph,
    coroutine: Coroutine<Message>,
    subscription: Signal<Option<yrs::Subscription>>,
}

impl Connection {
    pub fn new(graph: Graph) -> Self {
        let doc = graph.get_doc();
        let coroutine = use_coroutine(move |mut rx: UnboundedReceiver<Message>| {
            let mut doc = doc.clone();

            async move {
                loop {
                    // CONNECT
                    let ws = match Client::new()
                        .get("ws://localhost:9000/ws")
                        .upgrade()
                        .send()
                        .await
                    {
                        Ok(resp) => match resp.into_websocket().await {
                            Ok(ws) => ws,
                            Err(e) => {
                                dbg!("WebSocket upgrade failed:", e);
                                Delay::new(Duration::from_secs(3)).await;
                                continue;
                            }
                        },
                        Err(e) => {
                            dbg!("WebSocket connect failed:", e);
                            Delay::new(Duration::from_secs(3)).await;
                            continue;
                        }
                    };
                    let (mut sender, mut receiver) = ws.split();
                    let mut outgoing = rx.next().fuse();
                    let mut incoming = receiver.next().fuse();

                    // COMM LOOP
                    loop {
                        select! {
                            // OUTGOING updates from local doc
                            msg = outgoing => match msg {
                                Some(Message::SendUpdate(bytes)) => {
                                    sender.send(reqwest_websocket::Message::Binary(bytes.into())).await.ok();
                                    outgoing = rx.next().fuse();
                                }
                                None => break, // Coroutine closed
                            },

                            // INCOMING updates from server
                            msg = incoming => match msg {
                                Some(Ok(reqwest_websocket::Message::Binary(bytes))) => {
                                  doc.write().update(bytes.to_vec());
                                  incoming = receiver.next().fuse();
                                }
                                Some(Ok(_)) => {
                                    incoming = receiver.next().fuse(); // Ignore other ws message types
                                }
                                _ => break, // Lost connection
                            }
                        }
                    }

                    dbg!("Disconnected, retrying...");
                    Delay::new(Duration::from_secs(2)).await;
                }
            }
            //  let mut doc = doc.clone();
            //             async move {
            //                 loop {
            //                   let websocket = Client::default()
            //                   .get("ws://localhost:9000/ws")
            //                   .upgrade()
            //                   .send()
            //         .await?
            //         .into_websocket()
            //         .await?;

            //                    // connect  {
            //                         Ok(mut ws) => {

            //     // let (mut wtx, mut wrx) = websocket.split();
            // // while let Some(msg) = rx.next().await {
            // //                                match msg {
            // //                                    Message::SendUpdate(bytes) => {
            // //                                        sender.send(WsMessage::Binary(bytes))
            // //                                    }
            // //                                }
            // //                            }

            //                         }
            //                         Err(_) => {
            //                             dbg!("retry");
            //                             // Retry after delay if connection fails
            //                             Delay::new(Duration::from_secs(3)).await;
            //                         }
            //                     }
            //                 }
            //             }
        });

        let mut connection = Self {
            graph,
            coroutine,
            subscription: use_signal(|| None),
        };
        connection.subscribe();
        connection
    }

    fn subscribe(&mut self) {
        use_hook(|| {
            let coroutine = self.coroutine.clone();
            let subscription = self.graph.get_doc().read().observe_doc(move |update| {
                coroutine.send(Message::SendUpdate(update));
            });
            self.subscription.set(Some(subscription));
        });
    }
}

// Ok(Some(Message::Binary(bin)))  => {
//                                 doc.write().update(bytes);
//                             }
