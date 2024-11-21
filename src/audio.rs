use fundsp::hacker::*;
use kira::{
    manager::{AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};

/// The Audio Engine responsible for playing and managing sound
pub struct AudioEngine {
    pub manager: AudioManager,
    pub tracks: Vec<Box<dyn AudioNode<Sample = f64>>>, // Each track is a synthesizer
}

impl AudioEngine {
    /// Creates a new audio engine
    pub fn new(num_tracks: usize) -> Self {
        let manager = AudioManager::new(AudioManagerSettings::default()).unwrap();
        let mut tracks = Vec::new();

        // Initialize each track with a simple sine wave synthesizer
        for _ in 0..num_tracks {
            let synth = Box::new(sine_hz(440.0) * envelope(|t| if t < 0.1 { 1.0 - t * 10.0 } else { 0.0 }));
            tracks.push(synth);
        }

        Self { manager, tracks }
    }

    /// Play a note on a specific track
    pub fn play_note(&self, track_index: usize, frequency: f64) {
        if let Some(track) = self.tracks.get(track_index) {
            let mut instance = track.clone();
            instance.set_frequency(frequency);
            self.manager
                .play(instance, StaticSoundSettings::default())
                .unwrap();
        }
    }

    /// Stop all sounds
    pub fn stop(&self) {
        self.manager.stop_all();
    }
}
