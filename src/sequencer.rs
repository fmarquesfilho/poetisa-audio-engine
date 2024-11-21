use crate::audio::AudioEngine;

/// A simple step sequencer
pub struct Sequencer {
    pub bpm: f64,
    pub beat_index: usize,
    pub pattern: Vec<Vec<bool>>, // A 2D vector to represent the pattern for each track
    pub audio_engine: AudioEngine,
}

impl Sequencer {
    /// Creates a new sequencer with the given parameters
    pub fn new(bpm: f64, num_tracks: usize, num_beats: usize) -> Self {
        let audio_engine = AudioEngine::new(num_tracks);
        let pattern = vec![vec![false; num_beats]; num_tracks];

        Self {
            bpm,
            beat_index: 0,
            pattern,
            audio_engine,
        }
    }

    /// Set the BPM
    pub fn set_bpm(&mut self, bpm: f64) {
        self.bpm = bpm;
    }

    /// Toggle a step in the pattern
    pub fn toggle_step(&mut self, track: usize, beat: usize) {
        if let Some(track_pattern) = self.pattern.get_mut(track) {
            if let Some(step) = track_pattern.get_mut(beat) {
                *step = !*step;
            }
        }
    }

    /// Play the sequence for one cycle
    pub fn play(&mut self) {
        let interval = 60.0 / self.bpm / 4.0; // 16th notes
        let num_beats = self.pattern[0].len();

        for (track_index, track) in self.pattern.iter().enumerate() {
            if track[self.beat_index] {
                let frequency = 220.0 * (track_index + 1) as f64; // Example: Different pitches for tracks
                self.audio_engine.play_note(track_index, frequency);
            }
        }

        self.beat_index = (self.beat_index + 1) % num_beats;
    }

    /// Stop the sequence
    pub fn stop(&mut self) {
        self.audio_engine.stop();
    }
}
