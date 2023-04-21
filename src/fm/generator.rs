use super::{operator::Operator, score::Score, *};

pub struct Generator;

impl Generator {
    pub fn gen(score: &Score, op: &Operator, wave_type: WaveType) -> WaveBuffer {
        let k = 1.0 / SAMPLE_RATE as f32;
        let k_duration = SAMPLE_RATE / 1000;
        let buffer_size = score.total_duration * k_duration;
        let mut buffer = Vec::with_capacity(buffer_size);
        let mut t = 0;
        for note in score.notes.iter() {
            let note_duration = note.duration * k_duration;
            for _ in 0..note_duration {
                let t_f32 = t as f32;
                let f = note.frequency * k;
                let p = op.eval(f, t_f32);
                let c = wave_type.eval(note.amplitude, f, t_f32, p);
                let c = (MAX_AMPLITUDE as f32 * c) as WaveData;
                buffer.push(c);
                t += 1;
            }
        }
        buffer
    }
}
