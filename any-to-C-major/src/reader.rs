use std::{fs, path::PathBuf};

use crate::{flag::SheetStyle, sheet::Sheet};

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

    pub fn to_sheet(self, style: SheetStyle) -> Result<Sheet, std::io::Error> {
        let sheet_str = fs::read_to_string(self.path)?;
        if (!self.protect_lines) {
            let sheet_str = sheet_str.replace("\n", "");
        }
        return match style {
            SheetStyle::Brackets => Ok(parse_brackets(sheet_str)?),
            SheetStyle::CNBrackets=> Ok(parse_cn_brackets(sheet_str)?),
            SheetStyle::Dots=> Ok(parse_dots(sheet_str)?),
        };
        // let mut sharp_flag = false;
        // let mut flat_flag = false;
        // let mut field_flag = 0;
        // for each_char in sheet_str.chars(){
        //     match each_char{
        //         '#'=>sharp_flag=true,
        //         'b'=>flat_flag=true,
        //         ''
        //     }
        // }
        fn parse_brackets(sheet_str: String) -> Result<Sheet, std::io::Error> {
            unimplemented!()
        }
        fn parse_cn_brackets(sheet_str: String) -> Result<Sheet, std::io::Error> {
            unimplemented!()
        }
        fn parse_dots(sheet_str: String) -> Result<Sheet, std::io::Error> {
            unimplemented!()
        }
    }
}
