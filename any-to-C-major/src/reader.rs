use std::{fs, path::PathBuf};

use crate::{
    flag::SheetStyle,
    key::{Key, Note},
    sheet::Sheet,
};

pub struct Reader {
    path: PathBuf,
}

impl Reader {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
        }
    }

    pub fn to_string(self) -> Result<String, std::io::Error> {
        let sheet_str = fs::read_to_string(self.path)?;
        
        Ok(sheet_str)
    }
}
