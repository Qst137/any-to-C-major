use std::{fs, path::PathBuf};

use crate::{
    flag::SheetStyle,
    key::{Key, Note},
    sheet::Sheet,
};

pub struct Reader {
    path: PathBuf,
    protect_lines: bool,
}

impl Reader {
    pub fn new(path: PathBuf, protect_lines: bool) -> Self {
        Self {
            path,
            protect_lines,
        }
    }

    pub fn to_sheet(self) -> Result<String, std::io::Error> {
        let mut sheet_str = fs::read_to_string(self.path)?;
        if !self.protect_lines {
            sheet_str = sheet_str.replace("\n", "");
        }
        Ok(sheet_str)
    }
}
