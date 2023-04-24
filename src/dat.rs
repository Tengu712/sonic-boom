pub mod decode;
pub mod encode;

#[derive(Debug)]
pub struct MusicBlock {
    pub max_parts_count: u8,
    pub songs_count: u32,
    pub songs: Vec<SongBlock>,
}

#[derive(Debug)]
pub struct SongBlock {
    pub parts_count: u8,
    pub parts: Vec<PartBlock>,
}

#[derive(Debug)]
pub struct PartBlock {
    pub part_id: u8,
    pub source_id: u8,
    pub total_duration: u32,
    pub notes_count: u32,
    pub notes: Vec<NoteBlock>,
}

#[derive(Debug)]
pub struct NoteBlock {
    pub duration: u32,
    pub amplitude: f32,
    pub frequency: f32,
}
