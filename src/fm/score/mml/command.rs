use super::MMLNumType;

// ================================================================================================================= //
//     External                                                                                                      //
// ================================================================================================================= //

pub enum NoteName {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    R,
    X,
}

pub enum NoteModulation {
    None,
    Sharp,
    Flat,
    Natural,
}

pub struct NoteCommandDetail {
    pub name: NoteName,
    pub modulation: NoteModulation,
    pub long: MMLNumType,
    pub is_dotted: bool,
}

pub enum Command {
    Note(NoteCommandDetail),
    Octave(MMLNumType),
    OctaveUp,
    OctaveDown,
    Volume(MMLNumType),
}

impl Command {
    /// A constructor.
    /// It returns (command, new char index) when it succeeded.
    pub fn from(chars: &Vec<char>, cc: usize) -> Result<(Command, usize), String> {
        let mut cc = cc;
        while cc < chars.len() && chars[cc] == ' ' {
            cc += 1;
        }
        if cc >= chars.len() {
            return Err(String::from("empty line found"));
        }
        match chars[cc] {
            'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'r' | 'x' => eat_note(chars, cc),
            'o' => eat_octave(chars, cc),
            '>' => Ok((Command::OctaveUp, cc + 1)),
            '<' => Ok((Command::OctaveDown, cc + 1)),
            'v' => eat_volume(chars, cc),
            _ => Err(format!("invalid character '{}' found", chars[cc],)),
        }
    }
}

// ================================================================================================================= //
//     Internal                                                                                                      //
// ================================================================================================================= //

impl NoteName {
    fn from(c: char) -> Option<Self> {
        match c {
            'a' => Some(Self::A),
            'b' => Some(Self::B),
            'c' => Some(Self::C),
            'd' => Some(Self::D),
            'e' => Some(Self::E),
            'f' => Some(Self::F),
            'g' => Some(Self::G),
            'r' => Some(Self::R),
            'x' => Some(Self::X),
            _ => None,
        }
    }
}

impl NoteModulation {
    fn from(c: char) -> Option<Self> {
        match c {
            '+' => Some(Self::Sharp),
            '-' => Some(Self::Flat),
            '=' => Some(Self::Natural),
            _ => None,
        }
    }
}

fn eat_note(chars: &Vec<char>, cc: usize) -> Result<(Command, usize), String> {
    let mut detail = NoteCommandDetail {
        name: NoteName::from(chars[cc]).unwrap(),
        modulation: NoteModulation::None,
        long: 0,
        is_dotted: false,
    };
    let mut cc = cc + 1;
    // modulation
    if cc < chars.len() {
        if let Some(n) = NoteModulation::from(chars[cc]) {
            detail.modulation = n;
            cc += 1;
        }
    }
    // long
    if let Some((long, new_cc)) = eat_numeric(chars, cc)? {
        detail.long = long;
        cc = new_cc;
    }
    // dot
    if cc < chars.len() && chars[cc] == '.' {
        detail.is_dotted = true;
        cc += 1;
    }
    // finish
    Ok((Command::Note(detail), cc))
}

fn eat_octave(chars: &Vec<char>, cc: usize) -> Result<(Command, usize), String> {
    if let Some((v, new_cc)) = eat_numeric(chars, cc + 1)? {
        if v >= 1 && v <= 8 {
            Ok((Command::Octave(v), new_cc))
        } else {
            Err(format!(
                "octave parameter must be between 1 to 8 but found '{}'",
                v
            ))
        }
    } else {
        Err(String::from("octave parameter not found"))
    }
}

fn eat_volume(chars: &Vec<char>, cc: usize) -> Result<(Command, usize), String> {
    if let Some((v, new_cc)) = eat_numeric(chars, cc + 1)? {
        Ok((Command::Volume(v), new_cc))
    } else {
        Err(String::from("volume parameter not found"))
    }
}

fn eat_numeric(chars: &Vec<char>, cc: usize) -> Result<Option<(MMLNumType, usize)>, String> {
    let mut cur = cc;
    let mut buf = String::new();
    while cur < chars.len() {
        match chars[cur] {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => buf.push(chars[cc]),
            _ => break,
        }
        cur += 1;
    }
    if buf.len() == 0 {
        Ok(None)
    } else {
        match buf.parse::<MMLNumType>() {
            Ok(n) => return Ok(Some((n, cur))),
            Err(_) => return Err(format!("invalid number '{}' found", buf,)),
        }
    }
}
