use super::*;

pub(super) fn parse(line: &str, param: &mut ParsingParameter) -> Result<()> {
    // validation
    let splitted = line.split(' ').collect::<Vec<&str>>();
    if splitted.len() < 2 {
        return Err(String::from("invalid algorythm line found"));
    }
    // construct
    let name = splitted[0].to_string();
    let mut commands = Vec::new();
    for i in 1..splitted.len() {
        commands.push(parse_command(splitted[i], param));
    }
    let block = AlgorythmBlock { commands };
    // finish
    if let Some(n) = param.alg_map.get(&name) {
        if param.algs[*n as usize].is_none() {
            param.algs[*n as usize] = Some(block);
        } else {
            return Err(format!("algorythm '{}' redefined", name));
        }
    } else {
        param.alg_map.insert(name, param.algs.len() as u8);
        param.algs.push(Some(block));
    }
    Ok(())
}

fn parse_command(s: &str, param: &mut ParsingParameter) -> AlgorythmCommandBlock {
    if s == "^" {
        AlgorythmCommandBlock {
            command_id: ALGORYTHM_COMMAND_PUSH,
            operator_id: 0,
        }
    } else if s == "+" {
        AlgorythmCommandBlock {
            command_id: ALGORYTHM_COMMAND_ADD,
            operator_id: 0,
        }
    } else if s.starts_with('*') {
        AlgorythmCommandBlock {
            command_id: ALGORYTHM_COMMAND_FM,
            operator_id: op_id(&s[1..], param),
        }
    } else if s.starts_with('%') {
        AlgorythmCommandBlock {
            command_id: ALGORYTHM_COMMAND_AM,
            operator_id: op_id(&s[1..], param),
        }
    } else {
        AlgorythmCommandBlock {
            command_id: ALGORYTHM_COMMAND_PM,
            operator_id: op_id(s, param),
        }
    }
}

fn op_id(name: &str, param: &mut ParsingParameter) -> u8 {
    if let Some(n) = param.op_map.get(name) {
        *n
    } else {
        let res = param.ops.len() as u8;
        param.op_map.insert(name.to_string(), res);
        param.ops.push(None);
        res
    }
}
