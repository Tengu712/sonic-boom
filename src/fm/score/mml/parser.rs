use super::*;

use super::{command::Command, state::State};
use std::collections::HashMap;

/// A function to parse an MML and create music scores for each part.
pub fn parse(mml: Vec<&str>) -> Result<Vec<Score>, String> {
    let mut lc = 0;

    // headers
    while lc < mml.len() && mml[lc].starts_with('#') {
        if let Some((_, _)) = mml[lc].split_once(' ') {
            // TODO: title, bpm, etc.
        } else {
            return Err(format!(
                "parameter for a header '{}' not found : {} line",
                mml[lc], lc
            ));
        }
        lc += 1;
    }

    // TODO: audio sources

    // score
    let mut states = HashMap::new();
    while lc < mml.len() {
        let line = mml[lc].trim();
        if line.len() == 0 {
            lc += 1;
            continue;
        }
        if let Some((parts, command_line)) = mml[lc].split_once(':') {
            // parse commands
            let mut cc = 0;
            let mut commands = Vec::new();
            let chars = command_line.chars().collect::<Vec<char>>();
            while cc < chars.len() {
                match Command::from(&chars, cc) {
                    Ok((command, new_cc)) => {
                        commands.push(command);
                        cc = new_cc;
                    }
                    Err(e) => {
                        return Err(format!(
                            "{} : {} line {} char",
                            e,
                            lc + 1,
                            cc + parts.len() + 1
                        ));
                    }
                }
            }
            // apply to all of parts
            let parts = parts.split(',').collect::<Vec<&str>>();
            for part_str in parts {
                if !states.contains_key(part_str) {
                    states.insert(part_str, State::new());
                }
                let state = states.get_mut(part_str).unwrap();
                for command in &commands {
                    match state.update(command) {
                        Ok(()) => (),
                        Err(e) => return Err(format!("{} : {} line", e, lc + 1)),
                    }
                }
            }
        } else {
            return Err(format!("parts not declarated : {} line", lc + 1));
        }
        lc += 1;
    }

    let scores = states
        .into_iter()
        .map(|(_, v)| v.score)
        .collect::<Vec<Score>>();
    Ok(scores)
}
