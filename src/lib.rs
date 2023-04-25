pub mod dat;

/// A module for playing waves on each operating system.
mod api;
/// A module for generating wave buffer from operators and music score.
mod synthesizer;

use api::*;

type WaveData = i16;
type WaveBuffer = Vec<WaveData>;

pub const SAMPLE_RATE: usize = 44100;

struct Part {
    player_idx: usize,
    #[allow(dead_code)]
    wave: WaveBuffer,
    handle: AudioHandle,
}

pub struct SbApp {
    players: Vec<AudioPlayer>,
    songs: Vec<Vec<Part>>,
}

impl SbApp {
    pub fn new(music_data: dat::MusicBlock) -> Result<Self, String> {
        // players
        let players_cnt = music_data.max_parts_count as usize;
        let mut players = Vec::with_capacity(players_cnt);
        for _ in 0..players_cnt {
            players.push(AudioPlayer::new()?);
        }
        // operators
        let operators = music_data.operators;
        // algorythms
        let mut algorythm = Vec::new();
        algorythm.push(synthesizer::AlgorythmCmd::RUN(0));
        // songs
        let mut songs = Vec::with_capacity(music_data.songs.len());
        for song in music_data.songs {
            // parts
            let mut parts = Vec::with_capacity(song.parts.len());
            for part in song.parts {
                let part_id = part.part_id as usize;
                let wave = synthesizer::synthesize(part, &operators, &algorythm);
                let handle = api::AudioHandle::new(&players[part_id], &wave)?;
                let part = Part {
                    player_idx: part_id,
                    wave,
                    handle,
                };
                parts.push(part);
            }
            songs.push(parts);
        }
        // build up
        let app = Self { players, songs };
        Ok(app)
    }

    pub fn from_dat_file(path: &str) -> Result<Self, String> {
        use std::{
            fs::File,
            io::{BufReader, Read},
        };
        let f = match File::open(path) {
            Ok(n) => n,
            Err(e) => {
                return Err(format!("{e}"));
            }
        };
        let bytes = BufReader::new(f)
            .bytes()
            .collect::<Result<Vec<u8>, _>>()
            .unwrap();
        Self::new(dat::MusicBlock::from(bytes)?)
    }

    pub fn play(&self, idx: usize) -> Result<(), String> {
        for part in self.get_parts(idx)? {
            part.handle.play()?;
        }
        Ok(())
    }

    pub fn pause(&self, idx: usize) -> Result<(), String> {
        for part in self.get_parts(idx)? {
            self.players[part.player_idx].pause()?;
        }
        Ok(())
    }

    pub fn resume(&self, idx: usize) -> Result<(), String> {
        for part in self.get_parts(idx)? {
            self.players[part.player_idx].resume()?;
        }
        Ok(())
    }

    pub fn stop(&self, idx: usize) -> Result<(), String> {
        for part in self.get_parts(idx)? {
            self.players[part.player_idx].reset()?;
        }
        Ok(())
    }

    pub fn close(self) -> Result<(), String> {
        for song in self.songs {
            for part in song {
                part.handle.close()?;
            }
        }
        for player in self.players {
            player.close()?;
        }
        Ok(())
    }

    fn get_parts(&self, idx: usize) -> Result<&Vec<Part>, String> {
        if idx >= self.songs.len() {
            return Err(format!("tried to stop a song out of index."));
        }
        Ok(&self.songs[idx])
    }
}
