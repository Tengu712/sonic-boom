mod fm;

use fm::{api::*, generator::*, operator::*, score::*, *};

const MML: &'static str = "A: v32cdee+gao5c-c";

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

    let scores = Score::from_mml(MML.lines().collect::<Vec<&str>>()).unwrap();
    let wave = Generator::gen(&scores[0], &op1, WaveType::SINE);

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
