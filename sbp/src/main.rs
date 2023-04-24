use sbl::*;
use std::io::Write;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!("sbp <file>");
        return;
    }

    let app = SbApp::from_dat_file(&args[1]).unwrap();

    loop {
        print!("sbp> ");
        std::io::stdout().flush().unwrap();
        let mut inp = String::new();
        std::io::stdin().read_line(&mut inp).unwrap();
        let inp = inp.trim().split(' ').collect::<Vec<&str>>();
        if inp[0] == "quit" {
            break;
        } else if inp[0] == "play" {
            cmd_song_control(&app, inp, |app, idx| app.play(idx));
        } else if inp[0] == "pause" {
            cmd_song_control(&app, inp, |app, idx| app.pause(idx));
        } else if inp[0] == "resume" {
            cmd_song_control(&app, inp, |app, idx| app.resume(idx));
        } else if inp[0] == "stop" {
            cmd_song_control(&app, inp, |app, idx| app.stop(idx));
        } else {
            println!("sbp warning: invalid command '{}'", inp[0]);
        }
    }

    match app.close() {
        Ok(()) => (),
        Err(e) => println!("sbp error: {e}"),
    }
}

fn cmd_song_control(app: &SbApp, inp: Vec<&str>, f: fn(&SbApp, usize) -> Result<(), String>) {
    if inp.len() != 2 {
        println!("sbp warning: parameter not found");
    }
    if let Ok(n) = inp[1].parse::<usize>() {
        match f(app, n) {
            Ok(()) => (),
            Err(e) => println!("sbp warning: {e}"),
        }
    } else {
        println!("sbp warning: invalid parameter '{}' found", inp[1]);
    }
}
