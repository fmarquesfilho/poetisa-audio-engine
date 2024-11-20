pub mod sequencer;
pub mod audio;
pub mod web_socket;

pub use sequencer::Sequencer;
pub use web_socket::start_websocket_server;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
