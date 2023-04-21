use super::*;

pub struct Operator {
    pub wave_type: WaveType,
    pub attack: u32,
    pub decay: u32,
    pub sustain: u32,
    pub release: u32,
    pub total: f32,
    pub multiple: f32,
}

impl Operator {
    pub(super) fn eval(&self, f: f32, t: f32) -> f32 {
        self.wave_type.eval(self.total, f * self.multiple, t, 0.0)
    }
}
