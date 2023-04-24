use super::*;

use super::command::*;

// ================================================================================================================= //
//     External                                                                                                      //
// ================================================================================================================= //

pub(super) struct State {
    /// The result.
    /// block.part_id is set when resolving part name.
    pub(super) block: PartBlock,
    /// Current octave.
    /// It must be between -3 and 4.
    octave: i32,
    /// Current mili seconds per beats.
    mspb: f32,
    /// Current default long of note.
    long: f32,
    /// Current volume of note.
    volume: f32,
}

impl State {
    pub(super) fn new() -> Self {
        Self {
            block: PartBlock {
                part_id: 0,
                source_id: 0,
                total_duration: 0,
                notes_count: 0,
                notes: Vec::new(),
            },
            octave: 0,
            mspb: 500.0,
            long: 4.0,
            volume: 0.6,
        }
    }
    pub(super) fn update(&mut self, command: &Command) -> Result<(), String> {
        match command {
            Command::Note(detail) => self.for_note(detail),
            Command::Octave(v) => {
                self.octave = *v as i32 - 4;
                Ok(())
            }
            Command::OctaveUp => {
                if self.octave < 4 {
                    self.octave += 1;
                    Ok(())
                } else {
                    return Err(String::from("octave must be between 1 and 8"));
                }
            }
            Command::OctaveDown => {
                if self.octave > -3 {
                    self.octave -= 1;
                    Ok(())
                } else {
                    return Err(String::from("octave must be between 1 and 8"));
                }
            }
            Command::Long(v) => {
                self.long = *v as f32;
                Ok(())
            }
            Command::Volume(v) => {
                self.volume = *v as f32 / MMLNumType::MAX as f32;
                Ok(())
            }
        }
    }
}

// ================================================================================================================= //
//     Internal                                                                                                      //
// ================================================================================================================= //

const INTERVAL: f32 = 1.0594630943592952645618252949463417007792043174941856285592084314;
const INTERVAL_REV: f32 = 0.9438743126816934966419131566675343760075683033387428137421251423;

impl State {
    fn for_note(&mut self, detail: &NoteCommandDetail) -> Result<(), String> {
        let (f, a) = match detail.name {
            NoteName::R => (440.000, 0.0),
            NoteName::C => (261.626, 1.0),
            NoteName::D => (293.665, 1.0),
            NoteName::E => (329.628, 1.0),
            NoteName::F => (349.228, 1.0),
            NoteName::G => (391.995, 1.0),
            NoteName::A => (440.000, 1.0),
            NoteName::B => (493.883, 1.0),
            NoteName::X => {
                if let Some(last) = self.block.notes.last() {
                    (last.frequency, 1.0)
                } else {
                    return Err(String::from("The note `x` must not be the first note"));
                }
            }
        };
        let f = match detail.modulation {
            NoteModulation::Sharp => f * INTERVAL,
            NoteModulation::Flat => f * INTERVAL_REV,
            _ => f, // TODO: natural
        };
        let f = f * 2.0_f32.powi(self.octave);
        let long = if detail.long > 0 {
            detail.long as f32
        } else {
            self.long
        };
        let duration = (self.mspb * 4.0 / long) as u32;
        let note = NoteBlock {
            duration: duration,
            frequency: f,
            amplitude: a * self.volume,
        };
        self.block.total_duration += duration;
        self.block.notes_count += 1;
        self.block.notes.push(note);
        Ok(())
    }
}
