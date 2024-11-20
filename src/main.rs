use my_step_sequencer::{start_websocket_server, Sequencer};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let sequencer = Arc::new(Mutex::new(Sequencer::new(4, 4, 100.0)));
    start_websocket_server(sequencer).await;
}
