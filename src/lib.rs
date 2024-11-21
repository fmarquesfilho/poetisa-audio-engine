mod audio;
mod sequencer;

use wasm_bindgen::prelude::*;
use sequencer::Sequencer;

#[wasm_bindgen]
pub struct WasmSequencer {
    sequencer: Sequencer,
}

#[wasm_bindgen]
impl WasmSequencer {
    /// Create a new WasmSequencer instance
    #[wasm_bindgen(constructor)]
    pub fn new(bpm: f64, num_tracks: usize, num_beats: usize) -> Self {
        let sequencer = Sequencer::new(bpm, num_tracks, num_beats);
        Self { sequencer }
    }

    /// Set the BPM
    pub fn set_bpm(&mut self, bpm: f64) {
        self.sequencer.set_bpm(bpm);
    }

    /// Toggle a step in the pattern
    pub fn toggle_step(&mut self, track: usize, beat: usize) {
        self.sequencer.toggle_step(track, beat);
    }

    /// Play the sequence
    pub fn play(&mut self) {
        self.sequencer.play();
    }

    /// Stop the sequence
    pub fn stop(&mut self) {
        self.sequencer.stop();
    }
}
