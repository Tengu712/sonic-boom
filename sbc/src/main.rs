mod mml;

use sbl::dat::*;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Write},
    process::exit,
};

fn main() {
    match run() {
        Ok(()) => (),
        Err(e) => {
            eprintln!("sbc error: {}", e);
            exit(1);
        }
    }
}

fn run() -> Result<(), String> {
    let args = std::env::args().collect::<Vec<String>>();
    let args = &args[1..];
    if args.len() == 0 {
        println!("sbc <file1> [<file2> [<file3> [...]]]");
        return Ok(());
    }

    // parse MML files
    let mut param = mml::ParsingParameter::new();
    let mut songs = Vec::with_capacity(args.len());
    for arg in args {
        let file = File::open(arg).map_err(|e| format!("{arg} : {e}"))?;
        let lines = BufReader::new(file)
            .lines()
            .collect::<Result<Vec<String>, _>>()
            .unwrap();
        songs.push(mml::parse(lines, &mut param).map_err(|e| format!("{e} : {arg}"))?);
    }

    // check undefined and unwrap
    let operators = param
        .ops
        .into_iter()
        .enumerate()
        .map(|(i, n)| n.ok_or(fmt_undef("operator", &param.op_map, i)))
        .collect::<Result<Vec<OperatorBlock>, String>>()?;
    let algorythms = param
        .algs
        .into_iter()
        .enumerate()
        .map(|(i, n)| n.ok_or(fmt_undef("algorythm", &param.alg_map, i)))
        .collect::<Result<Vec<AlgorythmBlock>, String>>()?;

    // output
    let music = MusicBlock {
        max_parts_count: param.part_map.len() as u8,
        operators,
        algorythms,
        songs,
    };
    let mut out = File::create("./music.dat").map_err(|e| e.to_string())?;
    let bytes = music.to();
    out.write_all(bytes.as_slice()).map_err(|e| e.to_string())?;
    out.flush().map_err(|e| e.to_string())?;

    Ok(())
}

fn fmt_undef(s: &str, m: &HashMap<String, u8>, v: usize) -> String {
    for (k, u) in m {
        if *u == v as u8 {
            return format!("undefined {s} '{k}' found");
        }
    }
    panic!("unexpected error occured when unwrapping operators");
}
