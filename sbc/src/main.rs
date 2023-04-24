mod mml;

use sbl::dat::*;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Write},
    process::exit,
};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let args = &args[1..];
    if args.len() == 0 {
        println!("sbc <file1> [<file2> [<file3> [...]]]");
        return;
    }

    // prepare
    let mut operators = Vec::new();
    operators.push(OperatorBlock {
        attack: 0,
        decay: 0,
        sustain: 0,
        release: 0,
        total: 255,
        multiple: 10,
    });
    let mut operator_names = HashMap::new();
    operator_names.insert(String::from("sine"), 0);
    let mut part_names = HashMap::new();

    // parse MML files
    let mut songs = Vec::with_capacity(args.len());
    for arg in args {
        // open and read file
        let file = match File::open(arg) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("sbc error: {arg} : {e}");
                exit(1);
            }
        };
        let lines = BufReader::new(file)
            .lines()
            .collect::<Result<Vec<String>, _>>()
            .unwrap();
        // parse
        let result = match mml::parse(lines) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("sbc error: {arg} : {e}");
                exit(1);
            }
        };
        // map operators
        for (k, v) in result.operators {
            if operator_names.contains_key(&k) {
                eprintln!("sbc error: operator '{k}' is redefined");
                exit(1);
            }
            operator_names.insert(k, operators.len());
            operators.push(v);
        }
        // map parts
        let mut parts = Vec::new();
        for (k, mut v) in result.parts {
            if let Some(n) = part_names.get(&k) {
                v.part_id = *n as u8;
            } else {
                v.part_id = part_names.len() as u8;
                part_names.insert(k, part_names.len());
            }
            parts.push(v);
        }
        parts.sort_by(|a, b| a.part_id.cmp(&b.part_id));
        // finish
        songs.push(SongBlock { parts });
    }

    // output
    let music = MusicBlock {
        max_parts_count: part_names.len() as u8,
        operators,
        songs,
    };
    let mut out = match File::create("./music.dat") {
        Ok(n) => n,
        Err(e) => {
            eprintln!("sbc error: {e}");
            exit(1);
        }
    };
    let bytes = match music.to() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("sbc error: {e}");
            exit(1);
        }
    };
    match out.write_all(bytes.as_slice()) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("sbc error: {e}");
            exit(1);
        }
    }
    out.flush().unwrap();
}
