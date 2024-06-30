use std::{io::Error, path::PathBuf};

use crate::flag::Flag;

pub struct Core {
    flag: Flag,
}

impl Core {
    pub fn new(flag: Flag) -> Self {
        Self { flag }
    }

    pub fn run(self, sheet_dir: PathBuf) ->Result<(),Error>{
        Ok(())
    }
}