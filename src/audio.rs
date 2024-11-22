use std::error::Error;

pub struct AudioEngine {
    tracks: Vec<String>, // List of paths to audio files
}

impl AudioEngine {
    pub fn new(file_paths: Vec<&str>) -> Result<Self, Box<dyn Error>> {
        let tracks = file_paths.iter().map(|&s| s.to_string()).collect();
        Ok(AudioEngine { tracks })
    }

    pub fn play_track(&self, track_index: usize) {
        // Logic to play the audio track at the specified index
        if let Some(track) = self.tracks.get(track_index) {
            // Play the track (this is a placeholder, integrate with an audio playback library)
            println!("Playing track: {}", track);
        }
    }

    pub fn stop(&self) {
        // Logic to stop all audio tracks
        println!("Stopping all audio tracks.");
    }
}
