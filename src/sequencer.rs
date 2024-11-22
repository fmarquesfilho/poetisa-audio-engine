pub struct Sequencer {
    pub bpm: f64,
    pub beat_index: usize,
    pub pattern: Vec<Vec<bool>>, // Each track has a pattern of beats (true = active, false = inactive)
}

impl Sequencer {
    pub fn new(bpm: f64, num_beats: usize, num_tracks: usize) -> Self {
        let pattern = vec![vec![false; num_beats]; num_tracks]; // Initialize a pattern for each track
        Sequencer {
            bpm,
            beat_index: 0,
            pattern,
        }
    }

    // Start or continue playing the sequence
    pub fn play(&mut self) {
        // Implement playback logic here (e.g., loop through beats)
    }

    // Stop playback
    pub fn stop(&mut self) {
        // Implement stop logic here
    }

    // Get a reference to the active track
    pub fn get_active_track(&self, track_index: usize) -> Option<&Vec<bool>> {
        self.pattern.get(track_index) // Returns the track's pattern if valid
    }

    // Set a new BPM
    pub fn set_bpm(&mut self, bpm: f64) {
        self.bpm = bpm;
    }

    // Toggle the step (on/off) in a specific track's pattern
    pub fn toggle_step(&mut self, track: usize, beat: usize) {
        if let Some(track_pattern) = self.pattern.get_mut(track) {
            if let Some(step) = track_pattern.get_mut(beat) {
                *step = !*step; // Toggle the step (true/false)
            }
        }
    }

    // Get the number of tracks in the sequencer
    pub fn num_tracks(&self) -> usize {
        self.pattern.len()
    }
}
