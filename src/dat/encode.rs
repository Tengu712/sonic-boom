use super::*;

impl MusicBlock {
    pub fn to(self) -> Result<Vec<u8>, String> {
        let mut bytes = Vec::new();
        bytes.push(self.max_parts_count);
        push_all(&mut bytes, self.songs.len() as u32);
        for song in self.songs {
            song.to(&mut bytes)?;
        }
        Ok(bytes)
    }
}

impl SongBlock {
    fn to(self, bytes: &mut Vec<u8>) -> Result<(), String> {
        bytes.push(self.parts.len() as u8);
        for part in self.parts {
            part.to(bytes)?;
        }
        Ok(())
    }
}

impl PartBlock {
    fn to(self, bytes: &mut Vec<u8>) -> Result<(), String> {
        bytes.push(self.part_id);
        bytes.push(self.source_id);
        push_all(bytes, self.total_duration);
        push_all(bytes, self.notes.len() as u32);
        for note in self.notes {
            note.to(bytes)?;
        }
        Ok(())
    }
}

impl NoteBlock {
    fn to(self, bytes: &mut Vec<u8>) -> Result<(), String> {
        push_all(bytes, self.duration);
        push_all(bytes, self.amplitude.to_bits());
        push_all(bytes, self.frequency.to_bits());
        Ok(())
    }
}

fn push_all(trg: &mut Vec<u8>, p: u32) {
    for d in p.to_be_bytes() {
        trg.push(d);
    }
}
