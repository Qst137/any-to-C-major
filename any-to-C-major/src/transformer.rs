use crate::sheet::Sheet;

pub struct Transformer {
    base_sheet: Sheet,
}

impl Transformer {
    fn new(base_sheet: Sheet) -> Self {
        Self { base_sheet }
    }
    
}
