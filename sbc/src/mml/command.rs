use super::*;

// ================================================================================================================= //
//     External                                                                                                      //
// ================================================================================================================= //

pub(super) enum NoteName {
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

pub(super) enum NoteModulation {
    None,
    Sharp,
    Flat,
    Natural,
}

pub(super) struct NoteCommandDetail {
    pub(super) name: NoteName,
    pub(super) modulation: NoteModulation,
    pub(super) long: MMLNumType,
    pub(super) is_dotted: bool,
}

pub(super) enum Command {
    Note(NoteCommandDetail),
    Octave(MMLNumType),
    OctaveUp,
    OctaveDown,
    Long(MMLNumType),
    Volume(MMLNumType),
    Algorythm(String),
}

impl Command {
    /// A constructor.
    pub(super) fn from(chars: &Vec<char>, cc: &mut usize) -> Result<Self> {
        while *cc < chars.len() && chars[*cc] == ' ' {
            *cc += 1;
        }
        if *cc >= chars.len() {
            return Err(String::from("empty line found"));
        }
        match chars[*cc] {
            'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'r' | 'x' => Self::eat_note(chars, cc),
            'o' => Self::eat_octave(chars, cc),
            '>' => {
                *cc += 1;
                Ok(Self::OctaveUp)
            }
            '<' => {
                *cc += 1;
                Ok(Self::OctaveDown)
            }
            'l' => Self::eat_long(chars, cc),
            'v' => Self::eat_volume(chars, cc),
            '@' => Self::eat_algorythm(chars, cc),
            c => Err(format!("invalid character '{c}' found")),
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

impl Command {
    fn eat_note(chars: &Vec<char>, cc: &mut usize) -> Result<Self> {
        let mut detail = NoteCommandDetail {
            name: NoteName::from(chars[*cc]).unwrap(),
            modulation: NoteModulation::None,
            long: 0,
            is_dotted: false,
        };
        *cc += 1;
        // modulation
        if *cc < chars.len() {
            if let Some(n) = NoteModulation::from(chars[*cc]) {
                detail.modulation = n;
                *cc += 1;
            }
        }
        // long
        if let Some(n) = eat_number(chars, cc)? {
            detail.long = n;
        }
        // dot
        if *cc < chars.len() && chars[*cc] == '.' {
            detail.is_dotted = true;
            *cc += 1;
        }
        // finish
        Ok(Self::Note(detail))
    }

    fn eat_octave(chars: &Vec<char>, cc: &mut usize) -> Result<Self> {
        *cc += 1;
        if let Some(n) = eat_number(chars, cc)? {
            if n >= 1 && n <= 8 {
                Ok(Self::Octave(n))
            } else {
                Err(format!("octave parameter must be 1-8 but found '{n}'"))
            }
        } else {
            Err(String::from("octave parameter not found"))
        }
    }

    fn eat_long(chars: &Vec<char>, cc: &mut usize) -> Result<Self> {
        *cc += 1;
        if let Some(n) = eat_number(chars, cc)? {
            if n > 0 {
                Ok(Self::Long(n))
            } else {
                Err(format!("long parameter must not be 0 but found '{n}'"))
            }
        } else {
            Err(String::from("long parameter not found"))
        }
    }

    fn eat_volume(chars: &Vec<char>, cc: &mut usize) -> Result<Self> {
        *cc += 1;
        if let Some(n) = eat_number(chars, cc)? {
            Ok(Self::Volume(n))
        } else {
            Err(String::from("volume parameter not found"))
        }
    }

    fn eat_algorythm(chars: &Vec<char>, cc: &mut usize) -> Result<Self> {
        let mut buf = String::new();
        while *cc < chars.len() {
            match chars[*cc] {
                ' ' | '\t' => break,
                c => buf.push(c),
            }
            *cc += 1;
        }
        if buf.len() > 1 {
            Ok(Self::Algorythm(buf))
        } else {
            Err(String::from("algorythm name not found"))
        }
    }
}

fn eat_number(chars: &Vec<char>, cc: &mut usize) -> Result<Option<MMLNumType>> {
    let mut buf = String::new();
    while *cc < chars.len() {
        match chars[*cc] {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => buf.push(chars[*cc]),
            _ => break,
        }
        *cc += 1;
    }
    if buf.len() == 0 {
        Ok(None)
    } else {
        match buf.parse::<MMLNumType>() {
            Ok(n) => Ok(Some(n)),
            Err(_) => Err(format!("invalid number '{buf}' found")),
        }
    }
}
