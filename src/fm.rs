/// A module for playing waves on each operating system.
pub mod api;
/// A module for generating wave buffer from operators and music score.
pub mod generator;
/// A module for an operator that determines tone color.
/// We can create a complex tone color by combining one or more operators.
pub mod operator;
/// A module for music score;
pub mod score;

use std::f32::consts::PI;

pub type WaveData = i16;
pub type WaveBuffer = Vec<WaveData>;

const SAMPLE_RATE: usize = 44100;
const MAX_AMPLITUDE: f32 = WaveData::MAX as f32;

pub enum WaveType {
    SQUARE,
    SINE,
}

impl WaveType {
    fn eval(&self, a: f32, f: f32, t: f32, p: f32) -> f32 {
        match self {
            Self::SQUARE => 0.0, // TODO:
            Self::SINE => Self::sine_wave(a, f, t, p),
        }
    }
    fn sine_wave(a: f32, f: f32, t: f32, p: f32) -> f32 {
        a * f32::sin(2.0 * PI * f * t + p)
    }
}
