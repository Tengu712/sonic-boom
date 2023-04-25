pub mod decode;
pub mod encode;

#[derive(Debug)]
pub struct MusicBlock {
    pub max_parts_count: u8,
    pub operators: Vec<OperatorBlock>,
    pub songs: Vec<SongBlock>,
}

#[derive(Debug, Clone)]
pub struct OperatorBlock {
    pub attack: u8,
    pub decay: u8,
    pub sustain: u8,
    pub release: u8,
    pub total: u8,
    pub multiple: u8,
}

#[derive(Debug)]
pub struct SongBlock {
    pub parts: Vec<PartBlock>,
}

#[derive(Debug)]
pub struct PartBlock {
    pub part_id: u8,
    pub source_id: u8,
    pub notes: Vec<NoteBlock>,
}

#[derive(Debug)]
pub struct NoteBlock {
    pub time: u32,
    pub gate: u32,
    pub amplitude: f32,
    pub frequency: f32,
}
