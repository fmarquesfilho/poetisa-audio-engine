use kira::manager::{AudioManager, AudioManagerSettings};
use kira::clock::ClockSpeed;
use std::sync::{Arc, Mutex};
use crate::sequencer::Sequencer;

pub fn start_audio_playback(
    sequencer: Arc<Mutex<Sequencer>>,
    bpm: f32,
) -> Result<AudioManager, Box<dyn std::error::Error>> {
    // Initialize the audio manager
    let mut manager = AudioManager::new(AudioManagerSettings::default())?;
    let beat_duration = 60.0 / bpm / 4.0;

    // Add a clock with the specified speed
    let mut clock = manager.add_clock(ClockSpeed::SecondsPerTick(beat_duration.into()))?;
    let seq_clone = Arc::clone(&sequencer);

    // Start the clock (no need for ? here)
    clock.start();

    // Main loop to check the clock and play notes
    std::thread::spawn(move || {
        loop {
            // Get the current time of the clock
            let time = clock.time();
            let current_beat = (time.ticks as usize) % 16;

            let mut seq = seq_clone.lock().unwrap();
            for (i, instrument) in seq.instruments.iter().enumerate() {
                if instrument[current_beat] {
                    // Play the corresponding note
                    println!("Instrument {} playing at beat {}", i, current_beat);
                    // Add code to trigger sound playback here
                }
            }

            seq.beat = current_beat;

            // Sleep for a short duration to prevent busy-waiting
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    });

    Ok(manager)
}
