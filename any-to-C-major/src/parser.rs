use crate::{
    flag::SheetStyle,
    key::{Key, Note},
    sheet::Sheet,
};

pub struct Parser {
    style: SheetStyle,
    sheet: String,
}

impl Parser {
    pub fn new(style: SheetStyle, sheet: String) -> Self {
        Parser { style, sheet }
    }

    pub fn parse(self, key: Key) -> Sheet {
        return match self.style {
            SheetStyle::Brackets => Self::parse_brackets(self.sheet, key),
            SheetStyle::CNBrackets => Self::parse_cn_brackets(self.sheet, key),
            SheetStyle::Dots => Self::parse_dots(self.sheet, key),
        };
    }

    fn parse_brackets(sheet_str: String, key: Key) -> Sheet {
        let sheet_str = sheet_str
            .replace("【", "[")
            .replace("】", "]")
            .replace("）", ")")
            .replace("（", "(");
        let mut sharp_flag = false;
        let mut flat_flag = false;
        let mut field_flag = 0;
        let mut sheet = Sheet::new(key);
        for one_sign in sheet_str.chars() {
            match one_sign {
                '1'..='7' => {
                    sheet.add_note(Note::from_flags(
                        field_flag, one_sign, sharp_flag, flat_flag,
                    ));
                    sharp_flag = false;
                    flat_flag = false;
                }
                '#' => sharp_flag = true,
                'b' => flat_flag = true,
                '[' => {
                    if field_flag <= 0 {
                        field_flag = field_flag - 1;
                    } else {
                        field_flag = -1;
                    }
                }
                ']' => {
                    if field_flag < 0 {
                        field_flag = field_flag + 1;
                    } else {
                        field_flag = 0;
                    }
                }
                '(' => {
                    if field_flag >= 0 {
                        field_flag = field_flag + 1;
                    } else {
                        field_flag = 1;
                    }
                }
                ')' => {
                    if field_flag > 0 {
                        field_flag = field_flag - 1;
                    } else {
                        field_flag = 0;
                    }
                }
                _ => sheet.add_other(one_sign),
            }
        }
        sheet
    }

    fn parse_cn_brackets(sheet_str: String, key: Key) -> Sheet {
        todo!()
    }

    fn parse_dots(sheet_str: String, key: Key) -> Sheet {
        todo!()
    }
}
