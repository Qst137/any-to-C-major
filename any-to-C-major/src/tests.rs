use crate::{flag::SheetStyle, key::Key, parser::Parser};

#[test]
fn test_str_to_sheet() {
    let sheet_string = "3212343,321#5673,3#23(211)776365".to_string();
    let style = SheetStyle::Brackets;
    let parser = Parser::new(style, sheet_string);
    println!("{:?}", parser.parse(Key::from_letter("f")));
}