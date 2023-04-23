use super::*;

use super::command::*;

// ================================================================================================================= //
//     External                                                                                                      //
// ================================================================================================================= //

pub struct State {
    pub score: Score,
    octave: i32, // it must be between -3 and 4.
    mspb: f32,   // mili seconds per beats.
    long: f32,   // default long of note.
    amplitude: f32,
}

impl State {
    pub fn new() -> Self {
        Self {
            score: Score::new(),
            octave: 0,
            mspb: 500.0,
            long: 4.0,
            amplitude: 0.6,
        }
    }
    pub fn update(&mut self, command: &Command) -> Result<(), String> {
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
                self.amplitude = *v as f32 / MMLNumType::MAX as f32;
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
                if let Some(last) = self.score.notes.last() {
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
        let duration = (self.mspb * 4.0 / long) as usize;
        let note = Note {
            duration: duration,
            frequency: f,
            amplitude: a * self.amplitude,
        };
        self.score.total_duration += duration;
        self.score.notes.push(note);
        Ok(())
    }
}
