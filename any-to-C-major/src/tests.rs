#[cfg(test)]
use crate::{flag::SheetStyle, key::Key, parser::Parser, transformer::Transformer};

// #[test]
// fn test_str_to_sheet() {
//     let sheet_string = "3212343,321#5673,3#23(211)776365".to_string();
//     let style = SheetStyle::Brackets;
//     let parser = Parser::new(style, sheet_string);
//     println!("{:?}", parser.parse(Key::from_letter("f")));
// }

// #[test]
// fn test_sheet_to_c() {
//     let sheet_string = "3212343,321#5673,3#23(211)776365".to_string();
//     let style = SheetStyle::Brackets;
//     let sheet = Parser::new(style, sheet_string).parse(Key::from_letter("f"));
//     let c_sheet = Transformer::new(sheet).to_c();
//     println!("{:?}", c_sheet);
// }

#[test]
fn test_sheet_to_c() {
    let sheet_string = "1312312[55] [3]1[67]1[67]55 443234321[7] 11[7]1232 [77]21"
        .to_string();
    let style = SheetStyle::Brackets;
    let sheet = Parser::new(style, sheet_string).parse(Key::from_letter("E"));
    let c_sheet = Transformer::new(sheet).to_c();
    let formatted = c_sheet.format(SheetStyle::Brackets);
    println!("{formatted}");
}
