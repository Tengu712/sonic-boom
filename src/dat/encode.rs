use super::*;

impl MusicBlock {
    pub fn to(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.max_parts_count);
        bytes.push(self.operators.len() as u8);
        for n in self.operators {
            n.to(&mut bytes);
        }
        bytes.push(self.algorythms.len() as u8);
        for n in self.algorythms {
            n.to(&mut bytes);
        }
        push_all(&mut bytes, self.songs.len() as u32);
        for n in self.songs {
            n.to(&mut bytes);
        }
        bytes
    }
}

impl OperatorBlock {
    fn to(self, bytes: &mut Vec<u8>) {
        bytes.push(self.attack);
        bytes.push(self.decay);
        bytes.push(self.sustain);
        bytes.push(self.release);
        bytes.push(self.total);
        bytes.push(self.multiple);
    }
}

impl AlgorythmBlock {
    fn to(self, bytes: &mut Vec<u8>) {
        bytes.push(self.commands.len() as u8);
        for n in self.commands {
            n.to(bytes);
        }
    }
}

impl AlgorythmCommandBlock {
    fn to(self, bytes: &mut Vec<u8>) {
        bytes.push(self.command_id);
        bytes.push(self.operator_id);
    }
}

impl SongBlock {
    fn to(self, bytes: &mut Vec<u8>) {
        bytes.push(self.parts.len() as u8);
        for part in self.parts {
            part.to(bytes);
        }
    }
}

impl PartBlock {
    fn to(self, bytes: &mut Vec<u8>) {
        bytes.push(self.part_id);
        bytes.push(self.algorythm_id);
        push_all(bytes, self.notes.len() as u32);
        for note in self.notes {
            note.to(bytes);
        }
    }
}

impl NoteBlock {
    fn to(self, bytes: &mut Vec<u8>) {
        push_all(bytes, self.time);
        push_all(bytes, self.gate);
        push_all(bytes, self.amplitude.to_bits());
        push_all(bytes, self.frequency.to_bits());
    }
}

fn push_all(trg: &mut Vec<u8>, p: u32) {
    for n in p.to_be_bytes() {
        trg.push(n);
    }
}
