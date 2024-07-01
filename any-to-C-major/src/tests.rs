use flag::SheetStyle;
use parser::Parser;

use super::*;

#[test]
fn test_str_to_sheet() {
    let sheet_string = "44(11221)b7b766554".to_string();
    let style = SheetStyle::Brackets;
    let parser = Parser::new(style, sheet_string);
    println!("{:?}", parser.parse());
}
