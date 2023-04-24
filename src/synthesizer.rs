use super::{dat::*, *};

const MAX_AMPLITUDE: f32 = WaveData::MAX as f32;

pub(super) enum AlgorythmCmd {
    RUN(usize),
}

// TODO: implement envelope
pub(super) fn synthesize(
    part: PartBlock,
    operators: &Vec<OperatorBlock>,
    algorythm: &Vec<AlgorythmCmd>,
) -> WaveBuffer {
    // constants
    let k_duration = SAMPLE_RATE / 1000;
    let k_frequency = 1.0 / SAMPLE_RATE as f32;
    let buffer_size = part.total_duration as usize * k_duration;
    // for all notes
    let mut t = 0.0;
    let mut buffer = Vec::with_capacity(buffer_size);
    for note in part.notes {
        let duration = note.duration as usize * k_duration;
        let frequency = note.frequency * k_frequency;
        for i in 0..duration {
            // run algorythm
            let mut v = 0.0;
            for cmd in algorythm {
                match cmd {
                    AlgorythmCmd::RUN(n) => {
                        let op = &operators[*n];
                        let total = op.total as f32 / 255.0;
                        let multiple = op.multiple as f32 / 10.0;
                        v = sine_wave(total, multiple * frequency, t, v);
                    }
                }
            }
            // remove click noise
            if i < 300 {
                let i = i as f32;
                v = v * i / 300.0;
            } else if i > duration - 300 {
                let i = (duration - i) as f32;
                v = v * i / 300.0;
            }
            // finish
            let v = v * note.amplitude;
            let v = (MAX_AMPLITUDE as f32 * v) as WaveData;
            buffer.push(v);
            t += 1.0;
        }
    }
    buffer
}

fn sine_wave(a: f32, f: f32, t: f32, p: f32) -> f32 {
    a * f32::sin(2.0 * std::f32::consts::PI * f * t + p)
}
