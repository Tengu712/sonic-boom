mod command;
mod state;

use sbl::dat::*;
use std::collections::HashMap;

type MMLNumType = u8;

/// TODO: title, bpm, etc.
pub(super) struct TempSongBlock {
    pub(super) parts: HashMap<String, PartBlock>,
}

/// A function to parse a mml file into parts block.
/// It returns a hash-map whose key is a user-defined part name and whose value is PartBlock struct whose part_id is not set.
pub(super) fn parse(lines: Vec<String>) -> Result<TempSongBlock, String> {
    use command::Command;
    use state::State;

    let mut lc = 0;

    // headers
    while lc < lines.len() && lines[lc].starts_with('#') {
        if let Some((_, _)) = lines[lc].split_once(' ') {
            // TODO: title, bpm, etc.
        } else {
            return Err(format!(
                "parameter for a header '{}' not found : {} line",
                lines[lc], lc
            ));
        }
        lc += 1;
    }

    // TODO: audio sources

    // score
    let mut states = HashMap::new();
    while lc < lines.len() {
        let line = lines[lc].trim();
        if line.len() == 0 {
            lc += 1;
            continue;
        }
        if let Some((parts, command_line)) = lines[lc].split_once(':') {
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
                        return Err(format!("{} : {} line", e, lc + 1,));
                    }
                }
            }
            // apply to all of parts
            let parts = parts.split(',').collect::<Vec<&str>>();
            for part in parts {
                let part = part.to_string();
                if !states.contains_key(&part) {
                    states.insert(part.clone(), State::new());
                }
                let state = states.get_mut(&part).unwrap();
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

    // build up
    let mut parts = HashMap::new();
    for (k, v) in states {
        parts.insert(k, v.block);
    }
    Ok(TempSongBlock { parts })
}
