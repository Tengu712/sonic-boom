use std::io::Write;
use sbl::*;

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
        let inp = inp.trim();
        match inp.parse::<usize>() {
            Ok(n) => match app.play(n) {
                Ok(()) => continue,
                Err(e) => {
                    println!("sbp warning: {}", e);
                    continue;
                }
            }
            Err(_) => (),
        }
        if inp == "q" {
            break;
        }
        println!("sbp warning: invalid command '{inp}'");
    }
}
