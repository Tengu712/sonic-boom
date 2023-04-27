use super::*;

use super::command::*;

// ================================================================================================================= //
//     External                                                                                                      //
// ================================================================================================================= //

pub(super) struct State {
    pub(super) notes: Vec<NoteBlock>,
    pub(super) algorythm_name: String,
    /// Current timestamp that represents the current time of the song being played.
    time: u32,
    /// Current octave.
    /// It must be between -3 and 4.
    octave: i32,
    /// Current seconds per beats.
    spb: f32,
    /// Current default long of note.
    long: f32,
    /// Current volume of note.
    volume: f32,
}

impl State {
    pub(super) fn new() -> Self {
        Self {
            notes: Vec::new(),
            algorythm_name: String::from("@sine"),
            time: 0,
            octave: 0,
            spb: 0.5,
            long: 4.0,
            volume: 0.6,
        }
    }
    pub(super) fn update(&mut self, command: &Command) -> Result<()> {
        match command {
            Command::Note(detail) => self.for_note(detail)?,
            Command::Octave(v) => self.octave = *v as i32 - 4,
            Command::OctaveUp => {
                if self.octave < 4 {
                    self.octave += 1;
                } else {
                    return Err(String::from("octave must be between 1 and 8"));
                }
            }
            Command::OctaveDown => {
                if self.octave > -3 {
                    self.octave -= 1;
                } else {
                    return Err(String::from("octave must be between 1 and 8"));
                }
            }
            Command::Long(v) => self.long = *v as f32,
            Command::Volume(v) => self.volume = *v as f32 / MMLNumType::MAX as f32,
            Command::Algorythm(s) => self.algorythm_name = s.clone(),
        }
        Ok(())
    }
}

// ================================================================================================================= //
//     Internal                                                                                                      //
// ================================================================================================================= //

const INTERVAL: f32 = 1.0594630943592952645618252949463417007792043174941856285592084314;
const INTERVAL_REV: f32 = 0.9438743126816934966419131566675343760075683033387428137421251423;

impl State {
    fn for_note(&mut self, detail: &NoteCommandDetail) -> Result<()> {
        let long = if detail.long > 0 {
            detail.long as f32
        } else {
            self.long
        };
        let gate = (self.spb * 4.0 / long * sbl::SAMPLE_RATE as f32) as u32;
        let (f, a) = match detail.name {
            NoteName::C => (261.626, 1.0),
            NoteName::D => (293.665, 1.0),
            NoteName::E => (329.628, 1.0),
            NoteName::F => (349.228, 1.0),
            NoteName::G => (391.995, 1.0),
            NoteName::A => (440.000, 1.0),
            NoteName::B => (493.883, 1.0),
            NoteName::X => {
                if let Some(last) = self.notes.last() {
                    (last.frequency, 1.0)
                } else {
                    return Err(String::from("The note `x` must not be the first note"));
                }
            }
            NoteName::R => {
                self.time += gate;
                return Ok(());
            }
        };
        let f = match detail.modulation {
            NoteModulation::Sharp => f * INTERVAL,
            NoteModulation::Flat => f * INTERVAL_REV,
            _ => f, // TODO: natural
        };
        let f = f * 2.0_f32.powi(self.octave);
        let note = NoteBlock {
            time: self.time,
            gate,
            frequency: f,
            amplitude: a * self.volume,
        };
        self.time += gate;
        self.notes.push(note);
        Ok(())
    }
}
