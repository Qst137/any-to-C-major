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

    pub fn parse(&self) -> Sheet {
        return match self.style {
            SheetStyle::Brackets => {
                let sheet = self
                    .sheet
                    .replace("【", "[")
                    .replace("】", "]")
                    .replace("）", ")")
                    .replace("（", "(");
                Self::parse_brackets(sheet)
            },
            SheetStyle::CNBrackets => unimplemented!(),
            SheetStyle::Dots => unimplemented!(),
        };
    }

    fn parse_brackets(sheet_str: String) -> Sheet {
        let mut sharp_flag = false;
        let mut flat_flag = false;
        let mut field_flag = 0;
        let mut sheet = Sheet::new(Key::from_letter("c"));
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

    fn parse_cn_brackets(sheet_str: String) -> Sheet {
        unimplemented!()
    }

    fn parse_dots(sheet_str: String) -> Sheet {
        unimplemented!()
    }

}
