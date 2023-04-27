use super::{dat::*, *};

const MAX_AMPLITUDE: f32 = WaveData::MAX as f32;
const PER_SAMPLE_RATE: f32 = 1.0 / SAMPLE_RATE as f32;

// TODO: implement envelope
pub(super) fn synthesize(
    part: PartBlock,
    operators: &Vec<OperatorBlock>,
    algorythms: &Vec<AlgorythmBlock>,
) -> WaveBuffer {
    let notes = part.notes;
    let commands = &algorythms[part.algorythm_id as usize].commands;
    let buffer_size = notes[notes.len() - 1].time + notes[notes.len() - 1].gate;
    let buffer_size = buffer_size as usize;
    let mut living_head = 0;
    let mut buffer = Vec::with_capacity(buffer_size);
    for t in 0..buffer_size {
        let mut v = 0.0;
        for i in living_head..notes.len() {
            let t_u32 = t as u32;
            let time = notes[i].time;
            if t_u32 < time {
                break;
            }
            if t_u32 >= time + notes[i].gate && i <= living_head {
                living_head += 1;
                continue;
            }
            v += eval(&notes[i], operators, commands, t_u32 - time);
        }
        let v = v.min(1.0);
        let v = (MAX_AMPLITUDE * v) as WaveData;
        buffer.push(v);
    }
    buffer
}

/// A private function to evaluate the amplitude of a note in just one sample.
/// The value of `t` is between 0 and note.gate.
fn eval(
    note: &NoteBlock,
    operators: &Vec<OperatorBlock>,
    commands: &Vec<AlgorythmCommandBlock>,
    t: u32,
) -> f32 {
    let t_f32 = t as f32;
    let f = note.frequency * PER_SAMPLE_RATE;
    let mut v = 0.0;
    let mut stack = Vec::new();
    // do algorythm
    for n in commands {
        match n.command_id {
            ALGORYTHM_COMMAND_PM => {
                let op = &operators[n.operator_id as usize];
                v = pm(op.total, op.multiple * f, t_f32, v);
            }
            ALGORYTHM_COMMAND_FM => {
                let op = &operators[n.operator_id as usize];
                v = fm(op.total, op.multiple * f, t_f32, v);
            }
            ALGORYTHM_COMMAND_AM => {
                let op = &operators[n.operator_id as usize];
                v = am(op.total, op.multiple * f, t_f32, v);
            }
            ALGORYTHM_COMMAND_PUSH => {
                stack.push(v);
                v = 0.0;
            }
            ALGORYTHM_COMMAND_ADD => {
                // TODO: error handle
                v += stack.pop().unwrap();
            }
            c => panic!("sonic-boom error: invalid algorythm command id '{c}' found"),
        }
    }
    // remove click noise
    if t < 300 {
        v = v * t as f32 / 300.0;
    } else if t > note.gate - 300 {
        v = v * (note.gate as f32 - t as f32) / 300.0;
    }
    // finish
    v * note.amplitude
}

fn pm(a: f32, f: f32, t: f32, p: f32) -> f32 {
    a * f32::sin(2.0 * std::f32::consts::PI * f * t + p)
}

fn fm(a: f32, f: f32, t: f32, p: f32) -> f32 {
    a * f32::sin(2.0 * std::f32::consts::PI * (f + p) * t)
}

fn am(a: f32, f: f32, t: f32, p: f32) -> f32 {
    (a + p) * f32::sin(2.0 * std::f32::consts::PI * f * t)
}
