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
    // TODO: implement
    pub fn from_mml(mml: &str) -> Self {
        Self {
            total_duration: 0,
            notes: Vec::new(),
        }
    }
}
