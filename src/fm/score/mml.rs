mod command;
mod parser;
mod state;

use super::*;

type MMLNumType = u8;

impl Score {
    pub fn from_mml(mml: Vec<&str>) -> Result<Vec<Self>, String> {
        parser::parse(mml)
    }
}
