use crate::key::Key;

pub enum SheetStyle {
    Brackets,
    CNBrackets,
    Dots,
}

pub struct Flag {
    original_style: SheetStyle,
    target_style: SheetStyle,
    key: Key,
}

impl Flag {
    pub fn new(original_style: &str, target_style: &str, key: &str) -> Self {
        let to_sheet_style = |style: &str| match style.to_lowercase().as_str() {
            "brackets" => SheetStyle::Brackets,
            "cnbrackets" => SheetStyle::CNBrackets,
            "dots" => SheetStyle::Dots,
            _ => unimplemented!(),
        };

        let original_style = to_sheet_style(&original_style);
        let target_style = to_sheet_style(&target_style);

        let key = Key::from_letter(key);

        Self {
            original_style,
            target_style,
            key,
        }
    }
}
