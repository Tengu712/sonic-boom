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

    let mut temp_songs = Vec::with_capacity(args.len());
    for arg in args {
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
        let temp_song = match mml::parse(lines) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("sbc error: {arg} : {e}");
                exit(1);
            }
        };
        temp_songs.push(temp_song);
    }

    // part name mapping
    let mut max_parts_count: u8 = 0;
    let mut part_name_map = HashMap::new();
    for temp_song in temp_songs.iter() {
        for k in temp_song.parts.keys() {
            if part_name_map.contains_key(k) {
                continue;
            }
            part_name_map.insert(k.clone(), max_parts_count);
            max_parts_count += 1;
        }
    }
    // convert part name to id
    let mut songs = Vec::with_capacity(temp_songs.len());
    for temp_song in temp_songs {
        let mut parts = Vec::with_capacity(temp_song.parts.len());
        for (k, mut v) in temp_song.parts {
            v.part_id = *part_name_map.get(&k).unwrap();
            parts.push(v);
        }
        parts.sort_by(|a, b| a.part_id.partial_cmp(&b.part_id).unwrap());
        songs.push(SongBlock { parts });
    }

    let music = MusicBlock {
        max_parts_count,
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
