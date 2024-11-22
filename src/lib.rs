use kira::{
    manager::{AudioManager, AudioManagerSettings},
    clock::{ClockHandle, ClockSpeed},
    sound::static_sound::{StaticSoundData, StaticSoundHandle},
    tween::Tween,
};
use wasm_bindgen::prelude::*;
use js_sys::Function;
use std::sync::{Arc, Mutex};

#[wasm_bindgen]
pub struct Sequencer {
    manager: AudioManager,
    clock: ClockHandle,
    sounds: Vec<StaticSoundHandle>,
    step_callback: Option<Function>,
    current_step: Arc<Mutex<usize>>,
}

#[wasm_bindgen]
impl Sequencer {
    #[wasm_bindgen(constructor)]
    pub fn new(bpm: f32) -> Result<Sequencer, JsValue> {
        let mut manager = AudioManager::new(AudioManagerSettings::default())
            .map_err(|e| JsValue::from_str(&format!("Failed to create AudioManager: {:?}", e)))?;
        
        let clock = manager
            .add_clock(ClockSpeed::TicksPerMinute(bpm.into()))
            .map_err(|e| JsValue::from_str(&format!("Failed to create clock: {:?}", e)))?;
        
        Ok(Sequencer {
            manager,
            clock,
            sounds: Vec::new(),
            step_callback: None,
            current_step: Arc::new(Mutex::new(0)),
        })
    }

    pub fn load_sound(&mut self, sound_data: Vec<u8>) -> Result<usize, JsValue> {
        let cursor = std::io::Cursor::new(sound_data);
        let sound = StaticSoundData::from_cursor(cursor)
            .map_err(|e| JsValue::from_str(&format!("Failed to load sound: {:?}", e)))?;
        
        let sound_handle = self
            .manager
            .play(sound)
            .map_err(|e| JsValue::from_str(&format!("Failed to play sound: {:?}", e)))?;
        
        self.sounds.push(sound_handle);
        Ok(self.sounds.len() - 1)
    }

    pub fn set_step_callback(&mut self, callback: Option<Function>) {
        self.step_callback = callback;
    }

    pub fn start(&mut self) -> Result<(), JsValue> {
        self.clock
            .set_speed(ClockSpeed::TicksPerMinute(120.0), Tween::default());
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), JsValue> {
        self.clock
            .set_speed(ClockSpeed::TicksPerMinute(0.0), Tween::default());
        Ok(())
    }

    pub fn play_step(&mut self, sound_index: usize) -> Result<(), JsValue> {
        self.sounds[sound_index].resume(Tween::default());
        Ok(())
    }

    pub fn get_current_step(&self) -> usize {
        *self.current_step.lock().unwrap()
    }
}
