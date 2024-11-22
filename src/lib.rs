mod audio;
mod sequencer;

use wasm_bindgen::prelude::*;
use sequencer::Sequencer;
use audio::AudioEngine;

#[wasm_bindgen]
pub struct WasmSequencer {
    sequencer: Sequencer,
    audio_engine: AudioEngine, // Add the AudioEngine here to manage sound playback
}

#[wasm_bindgen]
impl WasmSequencer {
    /// Create a new WasmSequencer instance
    #[wasm_bindgen(constructor)]
    pub fn new(bpm: f64, num_tracks: usize, num_beats: usize, audio_file_paths: Vec<String>) -> Self {
        // Convert Vec<String> to Vec<&str> for AudioEngine and Sequencer
        let audio_file_paths_str: Vec<&str> = audio_file_paths.iter().map(|s| s.as_str()).collect();

        let sequencer = Sequencer::new(bpm, num_beats, num_tracks); // Create sequencer with the number of tracks
        let audio_engine = AudioEngine::new(audio_file_paths_str).expect("Failed to create audio engine");

        Self { sequencer, audio_engine }
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
        // Play the corresponding audio track for each active step
        for track_index in 0..self.sequencer.num_tracks() {
            if let Some(track) = self.sequencer.get_active_track(track_index) {
                if track[self.sequencer.beat_index] { // Check if the current beat is active
                    self.audio_engine.play_track(track_index);
                }
            }
        }
    }

    /// Stop the sequence and audio playback
    pub fn stop(&mut self) {
        self.sequencer.stop();
        self.audio_engine.stop(); // Stop all audio tracks
    }

    /// Toggle a track (enable or disable it)
    pub fn toggle_track(&mut self, track_index: usize) {
        self.sequencer.toggle_step(track_index, self.sequencer.beat_index); // Toggle the track's current step
    }
}
