mod fm;

use fm::{api::*, generator::*, operator::*, score::*, *};

fn main() {
    let player = AudioPlayer::new().unwrap();

    let op1 = Operator {
        wave_type: WaveType::SINE,
        attack: 0,
        decay: 0,
        sustain: 0,
        release: 0,
        total: 1.0,
        multiple: 8.0,
    };
    let mut notes = Vec::new();
    notes.push(Note {
        duration: 1000,
        amplitude: 0.2,
        frequency: 261.6,
    });
    notes.push(Note {
        duration: 1000,
        amplitude: 0.2,
        frequency: 293.6,
    });
    notes.push(Note {
        duration: 1000,
        amplitude: 0.2,
        frequency: 329.6,
    });
    notes.push(Note {
        duration: 1000,
        amplitude: 0.2,
        frequency: 349.2,
    });
    notes.push(Note {
        duration: 1000,
        amplitude: 0.2,
        frequency: 391.9,
    });
    notes.push(Note {
        duration: 1000,
        amplitude: 0.2,
        frequency: 440.9,
    });
    notes.push(Note {
        duration: 1000,
        amplitude: 0.2,
        frequency: 493.8,
    });
    notes.push(Note {
        duration: 1000,
        amplitude: 0.2,
        frequency: 523.2,
    });
    let score = Score {
        total_duration: 8000,
        notes,
    };
    let wave = Generator::gen(&score, &op1, WaveType::SINE);

    let mut handle = AudioHandle::new(&player, &wave).unwrap();
    handle.play().unwrap();

    let one_sec = std::time::Duration::from_secs(1);
    while handle.is_playing() {
        std::thread::sleep(one_sec);
    }

    player.reset().unwrap();
    handle.close().unwrap();
    player.close().unwrap();
}
