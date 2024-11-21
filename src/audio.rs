use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use kira::manager::{AudioManager, AudioManagerSettings};

/// The Audio Engine responsible for playing and managing sound
pub struct AudioEngine {
    pub manager: AudioManager,
    pub tracks: Vec<Box<dyn Fn(f64) -> f64>>, // Each track is a synthesizer
}

impl AudioEngine {
    /// Creates a new audio engine
    pub fn new(num_tracks: usize) -> Self {
        let manager = AudioManager::new(AudioManagerSettings::default()).unwrap();
        let mut tracks: Vec<Box<dyn Fn(f64) -> f64>> = Vec::new();

        // Initialize each track with a simple sine wave synthesizer
        for _ in 0..num_tracks {
            let synth: Box<dyn Fn(f64) -> f64> =
                Box::new(move |t: f64| (2.0 * std::f64::consts::PI * 440.0 * t).sin());
            tracks.push(synth);
        }

        Self { manager, tracks }
    }

    /// Play a note on a specific track
    pub fn play_note(&self, track_index: usize, frequency: f64) {
        if let Some(track) = self.tracks.get(track_index) {
            // Generate audio samples for 1 second
            let sample_rate = 44100;
            let duration_seconds = 1.0;
            let total_samples = (sample_rate as f64 * duration_seconds) as usize;

            let audio_samples: Vec<f32> = (0..total_samples)
                .map(|i| {
                    let t = i as f64 / sample_rate as f64;
                    (track)(t) as f32
                })
                .collect();

            // Create StaticSoundData from raw samples
            let sound_data = StaticSoundData::from_samples(
                sample_rate as u32,
                audio_samples,
                StaticSoundSettings::default(),
            )
            .unwrap();

            // Play sound data
            self.manager.play(sound_data).unwrap();
        }
    }

    /// Stop all sounds
    pub fn stop(&self) {
        self.manager.stop_all();
    }
}
