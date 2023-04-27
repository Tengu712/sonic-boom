pub mod decode;
pub mod encode;

use super::*;

pub const ALGORYTHM_COMMAND_FM: u8 = 1;
pub const ALGORYTHM_COMMAND_AM: u8 = 2;
pub const ALGORYTHM_COMMAND_PM: u8 = 0;
pub const ALGORYTHM_COMMAND_PUSH: u8 = 3;
pub const ALGORYTHM_COMMAND_ADD: u8 = 4;

#[derive(Debug)]
pub struct MusicBlock {
    pub max_parts_count: u8,
    pub operators: Vec<OperatorBlock>,
    pub algorythms: Vec<AlgorythmBlock>,
    pub songs: Vec<SongBlock>,
}

#[derive(Debug)]
pub struct OperatorBlock {
    pub attack: u8,
    pub decay: u8,
    pub sustain: u8,
    pub release: u8,
    pub total: u8,
    pub multiple: u8,
}

#[derive(Debug)]
pub struct AlgorythmBlock {
    pub commands: Vec<AlgorythmCommandBlock>,
}

#[derive(Debug)]
pub struct AlgorythmCommandBlock {
    pub command_id: u8,
    pub operator_id: u8,
}

#[derive(Debug)]
pub struct SongBlock {
    pub parts: Vec<PartBlock>,
}

#[derive(Debug)]
pub struct PartBlock {
    pub part_id: u8,
    pub algorythm_id: u8,
    pub notes: Vec<NoteBlock>,
}

#[derive(Debug)]
pub struct NoteBlock {
    pub time: u32,
    pub gate: u32,
    pub amplitude: f32,
    pub frequency: f32,
}
