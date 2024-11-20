use warp::Filter;
use tokio::sync::mpsc;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use futures_util::stream::StreamExt; // <-- Import StreamExt

use crate::sequencer::Sequencer;

#[derive(Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub r#type: String,
    pub instrument_index: Option<usize>,
    pub note_index: Option<usize>,
    pub bpm: Option<f32>,
}

pub async fn start_websocket_server(sequencer: Arc<Mutex<Sequencer>>) {
    let websocket = warp::path("ws")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let sequencer_clone = Arc::clone(&sequencer);
            ws.on_upgrade(move |socket| handle_websocket(socket, sequencer_clone))
        });

    warp::serve(websocket).run(([127, 0, 0, 1], 8080)).await;
}

async fn handle_websocket(ws: warp::ws::WebSocket, sequencer: Arc<Mutex<Sequencer>>) {
    let (mut tx, mut rx) = ws.split();  // Now works since StreamExt is in scope

    while let Some(Ok(msg)) = rx.next().await {
        if let Ok(text) = msg.to_str() {
            if let Ok(parsed) = serde_json::from_str::<WebSocketMessage>(text) {
                // Now `parsed` is of type WebSocketMessage
                let mut seq = sequencer.lock().unwrap();
            
                match parsed.r#type.as_str() {
                    "toggle_note" => {
                        if let (Some(inst), Some(note)) = (parsed.instrument_index, parsed.note_index) {
                            seq.toggle_note(inst, note);
                        }
                    }
                    "set_bpm" => {
                        if let Some(bpm) = parsed.bpm {
                            seq.set_bpm(bpm);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
