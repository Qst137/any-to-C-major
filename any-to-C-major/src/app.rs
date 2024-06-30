use std::path::PathBuf;

use clap::{Parser, ValueHint};


#[derive(Debug, Parser)]
#[command(about, version, args_override_self = true, disable_help_flag = true)]
pub struct Cli {
    #[arg(value_name = "FILE", required = true,value_hint = ValueHint::AnyPath)]
    pub original_sheet: PathBuf,

    pub original_style:Option<String>,

    #[arg(long, value_name = "MAJOR KEY")]
    pub key: Option<String>,

    pub target_style:Option<String>,
}