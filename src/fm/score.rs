/// A auxiliary module for using MML.
pub mod mml;

pub struct Note {
    pub duration: usize,
    pub amplitude: f32,
    pub frequency: f32,
}

pub struct Score {
    pub total_duration: usize,
    pub notes: Vec<Note>,
}

impl Score {
    pub fn new() -> Self {
        Self {
            total_duration: 0,
            notes: Vec::new(),
        }
    }
}
