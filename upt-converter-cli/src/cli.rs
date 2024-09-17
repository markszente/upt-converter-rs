use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Input file. By default, it's treated as a UTF-16LE encoded xml
    #[arg(short, long, value_name = "FILE")]
    pub file: PathBuf,

    /// Output file name, optional
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    // Name, optional
    #[arg(short, long)]
    pub name: Option<String>,
}
