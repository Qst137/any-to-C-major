use core::Core;
use std::{io::Error, path::PathBuf, str::FromStr};

mod app;
mod core;
mod flag;
mod key;
mod parser;
mod reader;
mod sheet;
mod tests;
mod transformer;

use app::Cli;
use flag::Flag;

fn main() -> Result<(), Error> {
    // let cli = Cli::parse_from(wild::args_os());

    let cli = Cli {
        original_sheet: PathBuf::from_str("test.txt").unwrap(),
        original_style: Some(String::from_str("brackets").unwrap()),
        target_style: Some(String::from_str("brackets").unwrap()),
        key: Some(String::from_str("F").unwrap()),
        protect_lines: true,
    };
    let flag = Flag::new(
        cli.original_style.unwrap().as_str(),
        cli.target_style.unwrap().as_str(),
        cli.key.unwrap().as_str(),
        cli.protect_lines,
    );
    let sheet_dir = cli.original_sheet;
    let core = Core::new(flag);
    core.run(sheet_dir)?;
    Ok(())
}
