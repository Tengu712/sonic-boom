use super::*;

impl MusicBlock {
    pub fn from(bytes: Vec<u8>) -> Result<Self, String> {
        let mut idx = 0;
        let max_parts_count = get_u8(&bytes, &mut idx)?;
        let songs_count = get_u32(&bytes, &mut idx)?;
        let mut songs = Vec::with_capacity(songs_count as usize);
        for _ in 0..songs_count {
            songs.push(SongBlock::from(&bytes, &mut idx)?);
        }
        let res = Self {
            max_parts_count,
            songs_count,
            songs,
        };
        Ok(res)
    }
}

impl SongBlock {
    fn from(bytes: &Vec<u8>, idx: &mut usize) -> Result<Self, String> {
        let parts_count = get_u8(bytes, idx)?;
        let mut parts = Vec::new();
        for _ in 0..parts_count {
            parts.push(PartBlock::from(bytes, idx)?);
        }
        let res = Self { parts_count, parts };
        Ok(res)
    }
}

impl PartBlock {
    fn from(bytes: &Vec<u8>, idx: &mut usize) -> Result<Self, String> {
        let part_id = get_u8(bytes, idx)?;
        let source_id = get_u8(bytes, idx)?;
        let total_duration = get_u32(bytes, idx)?;
        let notes_count = get_u32(bytes, idx)?;
        let mut notes = Vec::new();
        for _ in 0..notes_count {
            notes.push(NoteBlock::from(bytes, idx)?);
        }
        let res = Self {
            part_id,
            source_id,
            total_duration,
            notes_count,
            notes,
        };
        Ok(res)
    }
}

impl NoteBlock {
    fn from(bytes: &Vec<u8>, idx: &mut usize) -> Result<Self, String> {
        let duration = get_u32(bytes, idx)?;
        let amplitude = get_f32(bytes, idx)?;
        let frequency = get_f32(bytes, idx)?;
        let res = Self {
            duration,
            amplitude,
            frequency,
        };
        Ok(res)
    }
}

fn get_u8(bytes: &Vec<u8>, idx: &mut usize) -> Result<u8, String> {
    if *idx >= bytes.len() {
        return Err(String::from("invalid music data"));
    }
    let res = bytes[*idx];
    *idx += 1;
    Ok(res)
}

fn get_u32(bytes: &Vec<u8>, idx: &mut usize) -> Result<u32, String> {
    if *idx + 3 >= bytes.len() {
        return Err(String::from("invalid music data"));
    }
    let sub = [
        bytes[*idx],
        bytes[*idx + 1],
        bytes[*idx + 2],
        bytes[*idx + 3],
    ];
    let res = u32::from_be_bytes(sub);
    *idx += 4;
    Ok(res)
}

fn get_f32(bytes: &Vec<u8>, idx: &mut usize) -> Result<f32, String> {
    if *idx + 3 >= bytes.len() {
        return Err(String::from("invalid music data"));
    }
    let sub = [
        bytes[*idx],
        bytes[*idx + 1],
        bytes[*idx + 2],
        bytes[*idx + 3],
    ];
    let res = f32::from_be_bytes(sub);
    *idx += 4;
    Ok(res)
}
