use std::{io::Error, path::PathBuf};

use crate::{flag::Flag, parser::Parser, reader::Reader, transformer::Transformer};

pub struct Core {
    flag: Flag,
}

impl Core {
    pub fn new(flag: Flag) -> Self {
        Self { flag }
    }

    pub fn run(self, sheet_dir: PathBuf) -> Result<(), Error> {
        let sheet_string = Reader::new(sheet_dir).to_string()?;
        let sheet = Parser::new(self.flag.original_style, sheet_string).parse(self.flag.key);
        let sheet_c = Transformer::new(sheet).to_c();
        let string = sheet_c.format(self.flag.target_style);
        Ok(())
    }
}
