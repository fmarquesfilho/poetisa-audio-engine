
pub struct Sequencer {
    pub instruments: Vec<Vec<bool>>,
    pub bpm: f32,
    pub beat: usize,
}

impl Sequencer {
    pub fn new(beats_per_measure: usize, number_of_measures: usize, bpm: f32) -> Self {
        let instruments = vec![
            vec![false; beats_per_measure * number_of_measures],
            vec![false; beats_per_measure * number_of_measures],
            vec![false; beats_per_measure * number_of_measures],
            vec![false; beats_per_measure * number_of_measures],
        ];

        Self {
            instruments,
            bpm,
            beat: 0,
        }
    }

    pub fn toggle_note(&mut self, instrument_index: usize, note_index: usize) {
        if let Some(note) = self.instruments.get_mut(instrument_index).and_then(|row| row.get_mut(note_index)) {
            *note = !*note;
        }
    }

    pub fn set_bpm(&mut self, bpm: f32) {
        self.bpm = bpm;
    }
}
