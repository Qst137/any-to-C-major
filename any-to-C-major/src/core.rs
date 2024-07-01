use std::{io::Error, path::PathBuf};

use crate::{flag::Flag, parser::Parser, reader::Reader};

pub struct Core {
    flag: Flag,
}

impl Core {
    pub fn new(flag: Flag) -> Self {
        Self { flag }
    }

    pub fn run(self, sheet_dir: PathBuf) ->Result<(),Error>{
        let reader = Reader::new(sheet_dir);
        let sheet_string = reader.to_string()?;
        let parser=Parser::new(self.flag.original_style, sheet_string);
        let sheet = parser.parse();
        Ok(())
    }
}