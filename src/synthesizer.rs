use super::{dat::*, *};

const MAX_AMPLITUDE: f32 = WaveData::MAX as f32;

pub(super) struct Operator {
    pub(super) attack: f32,
    pub(super) decay: f32,
    pub(super) sustain: f32,
    pub(super) release: f32,
    pub(super) total: f32,
    pub(super) multiple: f32,
}

// TODO: implement algorythm
// TODO: implement envelope
pub(super) fn synthesize(part: PartBlock) -> WaveBuffer {
    let k = 1.0 / SAMPLE_RATE as f32;
    let k_duration = SAMPLE_RATE / 1000;
    let buffer_size = part.total_duration as usize * k_duration;
    let mut t = 0;
    let mut buffer = Vec::with_capacity(buffer_size);
    for note in part.notes {
        let note_duration = note.duration as usize * k_duration;
        for _ in 0..note_duration {
            let t_f32 = t as f32;
            let f = note.frequency * k;
            let c = sine_wave(note.amplitude, f, t_f32, 0.0);
            let c = (MAX_AMPLITUDE as f32 * c) as WaveData;
            buffer.push(c);
            t += 1;
        }
    }
    buffer
}

fn sine_wave(a: f32, f: f32, t: f32, p: f32) -> f32 {
    a * f32::sin(2.0 * std::f32::consts::PI * f * t + p)
}
