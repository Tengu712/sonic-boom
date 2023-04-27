use super::*;

pub(super) fn parse(line: &str, param: &mut ParsingParameter) -> Result<()> {
    // validation
    let splitted = line.split(' ').collect::<Vec<&str>>();
    if splitted.len() != 7 || splitted[0].len() < 2 {
        return Err(String::from("invalid operator line format found"));
    }
    // construct
    let name = splitted[0][1..].to_string();
    let block = OperatorBlock {
        attack: parse_parameter(splitted[1])?,
        decay: parse_parameter(splitted[2])?,
        sustain: parse_parameter(splitted[3])?,
        release: parse_parameter(splitted[4])?,
        total: parse_float(splitted[5])?,
        multiple: parse_float(splitted[6])?,
    };
    // finish
    if let Some(n) = param.op_map.get(&name) {
        if param.ops[*n as usize].is_none() {
            param.ops[*n as usize] = Some(block);
        } else {
            return Err(format!("operator '{}' redefined", name));
        }
    } else {
        param.op_map.insert(name, param.ops.len() as u8);
        param.ops.push(Some(block));
    }
    Ok(())
}

fn parse_parameter(p: &str) -> Result<MMLNumType> {
    p.parse::<MMLNumType>().map_err(|_| {
        format!(
            "operator parameter must be {} bytes number",
            std::mem::size_of::<MMLNumType>()
        )
    })
}

fn parse_float(p: &str) -> Result<f32> {
    p.parse::<f32>().map_err(|_| {
        format!(
            "operator parameter must be {} bytes floating number",
            std::mem::size_of::<f32>()
        )
    })
}
