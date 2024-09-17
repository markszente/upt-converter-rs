use std::{fs, path::Path};

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use upt_converter::{conversion::convert_raw, model::Collection, unipol::Export};
use utf16string::{WStr, LE};

mod cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let file = &cli.file;

    let file_name = file
        .file_stem()
        .and_then(|file_name| file_name.to_str())
        .unwrap_or("unnamed collection")
        .to_string();

    let export = get_export_from_path(file)?;

    let name = match cli.name {
        Some(name) => name,
        None => file_name,
    };

    let collection = Collection::new(name.as_str(), export)?;

    let json = serde_json::to_string_pretty(&collection)?;

    if let Some(output) = cli.output {
        fs::write(output, json)?;
    } else {
        println!("{}", json);
    }

    Ok(())
}

fn get_export_from_path<P: AsRef<Path>>(path: P) -> Result<Export> {
    let utf16 = open_utf16_file(path)?;
    let content = convert_to_utf8(&utf16)?;
    let result = convert_raw(&content)?;
    Ok(result)
}

fn open_utf16_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let result = fs::read(path)?;
    Ok(result)
}

/// Converts a UTF16LE file to String
fn convert_to_utf8(utf16: &[u8]) -> Result<String> {
    let s0: &WStr<LE> = WStr::from_utf16(utf16)?;

    let content = s0.to_utf8();
    Ok(content)
}
