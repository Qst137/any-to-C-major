use crate::sheet::Sheet;

pub struct Transformer {
    base_sheet: Sheet,
}

impl Transformer {
    pub fn new(base_sheet: Sheet) -> Self {
        Self { base_sheet }
    }

    pub fn to_c(self) -> Sheet {
        let mut sheet_inside = self.base_sheet.to_sheet_inside();
        sheet_inside.to_c();
        println!("{:?}",sheet_inside);
        sheet_inside.to_sheet()
    }
}
