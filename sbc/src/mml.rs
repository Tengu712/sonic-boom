mod algorythm;
mod command;
mod operator;
mod state;

use sbl::dat::*;
use std::collections::HashMap;

type MMLNumType = u8;
type Result<T> = std::result::Result<T, String>;

pub(super) struct ParsingParameter {
    pub(super) ops: Vec<Option<OperatorBlock>>,
    pub(super) op_map: HashMap<String, u8>,
    pub(super) algs: Vec<Option<AlgorythmBlock>>,
    pub(super) alg_map: HashMap<String, u8>,
    pub(super) part_map: HashMap<String, u8>,
}
impl ParsingParameter {
    pub(super) fn new() -> Self {
        // operator
        let ops = Vec::from([Some(OperatorBlock {
            attack: 0,
            decay: 0,
            sustain: 0,
            release: 0,
            total: 1.0,
            multiple: 1.0,
        })]);
        let op_map = HashMap::from([(String::from("sine"), 0)]);
        // algorythm
        let commands = Vec::from([AlgorythmCommandBlock {
            command_id: ALGORYTHM_COMMAND_PM,
            operator_id: 0,
        }]);
        let algs = Vec::from([Some(AlgorythmBlock { commands })]);
        let alg_map = HashMap::from([(String::from("@sine"), 0)]);
        // finish
        Self {
            ops,
            op_map,
            algs,
            alg_map,
            part_map: HashMap::new(),
        }
    }
}

pub(super) fn parse(lines: Vec<String>, param: &mut ParsingParameter) -> Result<SongBlock> {
    let mut lc = 0;
    let mut states = HashMap::new();
    while lc < lines.len() {
        // skip white line
        let line = lines[lc].trim();
        if line.len() == 0 {
            lc += 1;
            continue;
        }
        // header
        else if line.starts_with('#') {
            if let Some((_, _)) = line.split_once(' ') {
                // TODO: title, bpm, etc.
            } else {
                return Err(format!(
                    "parameter for a header '{}' not found : {} line",
                    line, lc
                ));
            }
        }
        // operator
        else if line.starts_with('\'') {
            operator::parse(line, param).map_err(|e| format!("{} : {} line", e, lc + 1))?;
        }
        // algorythm
        else if line.starts_with('@') {
            algorythm::parse(line, param).map_err(|e| format!("{} : {} line", e, lc + 1))?;
        }
        // parts
        else if let Some((parts, cl)) = line.split_once(':') {
            let mut cc = 0;
            let mut commands = Vec::new();
            let chars = cl.trim().chars().collect::<Vec<char>>();
            while cc < chars.len() {
                // TODO:
                commands.push(
                    command::Command::from(&chars, &mut cc)
                        .map_err(|e| format!("{} : {} line", e, lc + 1))?,
                );
            }
            let parts = parts.split(',').collect::<Vec<&str>>();
            for n in parts {
                if !states.contains_key(n) {
                    states.insert(n.to_string(), state::State::new());
                }
                let state = states.get_mut(n).unwrap();
                for m in commands.iter() {
                    state.update(m)?;
                }
            }
        }
        // error
        else {
            return Err(format!("parts not declarated : {} line", lc + 1));
        }
        lc += 1;
    }
    // build up
    let mut parts = Vec::new();
    for (k, v) in states {
        let part_id = if let Some(n) = param.part_map.get(&k) {
            *n
        } else {
            let res = param.part_map.len() as u8;
            param.part_map.insert(k, res);
            res
        };
        let algorythm_id = if let Some(n) = param.alg_map.get(&v.algorythm_name) {
            *n
        } else {
            let res = param.algs.len() as u8;
            param.alg_map.insert(v.algorythm_name, res);
            param.algs.push(None);
            res
        };
        parts.push(PartBlock {
            part_id,
            algorythm_id,
            notes: v.notes,
        });
    }
    Ok(SongBlock { parts })
}
