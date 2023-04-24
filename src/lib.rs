use crate::api::AudioHandleImpl;

pub mod dat;

/// A module for playing waves on each operating system.
mod api;
/// A module for generating wave buffer from operators and music score.
mod synthesizer;

type WaveData = i16;
type WaveBuffer = Vec<WaveData>;

const SAMPLE_RATE: usize = 44100;

pub struct SbApp {
    err_msg: String,
    players: Vec<api::AudioPlayer>,
    #[allow(dead_code)]
    waves: Vec<Vec<WaveBuffer>>,
    handles: Vec<Vec<api::AudioHandle>>,
}

impl SbApp {
    pub fn new(music_data: dat::MusicBlock) -> Result<Self, String> {
        use api::AudioPlayerImpl;
        let players_cnt = music_data.max_parts_count as usize;
        let mut players = Vec::with_capacity(players_cnt);
        for _ in 0..players_cnt {
            players.push(api::AudioPlayer::new()?);
        }
        let mut waves = Vec::with_capacity(music_data.songs_count as usize);
        let mut handles = Vec::with_capacity(music_data.songs_count as usize);
        for song in music_data.songs {
            let mut t_waves = Vec::with_capacity(song.parts_count as usize);
            let mut t_handles = Vec::with_capacity(song.parts_count as usize);
            for _ in 0..song.parts_count {
                t_waves.push(Vec::new());
            }
            for part in song.parts {
                let part_id = part.part_id as usize;
                let wave = synthesizer::synthesize(part);
                let handle = api::AudioHandle::new(&players[part_id], &wave)?;
                t_waves.push(wave);
                t_handles.push(handle);
            }
            waves.push(t_waves);
            handles.push(t_handles);
        }
        let app = Self {
            err_msg: String::new(),
            players,
            waves,
            handles,
        };
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
        if idx >= self.handles.len() {
            return Err(format!("tried to play a song out of index."));
        }
        let song = &self.handles[idx];
        for h in song {
            h.play()?;
        }
        Ok(())
    }
}
