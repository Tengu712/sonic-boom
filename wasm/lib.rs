const SAMPLE_RATE: usize = 44100;
const SAMPLE_CYCLE: f32 = 1.0 / SAMPLE_RATE as f32;

#[repr(C)]
pub struct Wave {
    pub buffer: *const f32,
    pub buffer_length: u32,
}

#[no_mangle]
pub extern "C" fn create_wave() -> *const Wave {
    let buffer_length = SAMPLE_RATE * 2;
    let mut buffer = Vec::with_capacity(buffer_length);
    for i in 0..buffer_length {
        buffer.push(f32::sin(2.0 * std::f32::consts::PI * 440.0 * SAMPLE_CYCLE * i as f32));
    }

    let buffer = buffer.leak();
    let wave = Box::new(Wave {
        buffer: buffer.as_ptr(),
        buffer_length: buffer.len() as u32,
    });
    Box::leak(wave)
}
