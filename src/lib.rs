use error::convert::ConvertError;
use model::Collection;
use serde::Deserialize;
use serde_derive::{Deserialize, Serialize};
use serde_xml_rs::{de::Deserializer, from_str, to_string};
use std::{collections::VecDeque, error::Error, fs, path::Path};
use unipol::{Export, Folder};
use utf16string::{WStr, LE};

mod convert;
mod error;
mod model;
mod unipol;

pub fn get_export_from_path<P: AsRef<Path>>(path: P) -> Result<Export, ConvertError> {
    let utf16 = open_utf16_file(path)?;
    let content = convert_to_utf8(&utf16)?;
    let result = convert_raw(&content)?;
    Ok(result)
}

fn open_utf16_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, ConvertError> {
    let result = fs::read(path)?;
    Ok(result)
}

/// Converts a UTF16LE file to String
fn convert_to_utf8(utf16: &[u8]) -> Result<String, ConvertError> {
    let s0: &WStr<LE> = WStr::from_utf16(utf16)?;

    let content = s0.to_utf8();
    Ok(content)
}

fn convert_raw(content: &str) -> Result<Export, ConvertError> {
    let result: Export = from_str(content)?;
    Ok(result)
}

pub fn convert_to_folders() -> Result<(), Box<dyn Error>> {
    let utf16 = open_utf16_file("A:/Chrome/Csoportosítás teszt/189788754.upt")?;
    let content = convert_to_utf8(&utf16)?;
    let result = convert_raw(&content)?;
    let folders = result.flatten_folders()?;
    let collection = Collection::from(folders);
    // let first = result.folders;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::model::Question;

    use super::*;

    #[test]
    fn test_group() -> Result<(), Box<dyn Error>> {
        let utf16 = open_utf16_file("A:/Chrome/Csoportosítás teszt/222600431.upt")?;
        let content = convert_to_utf8(&utf16)?;
        let result = convert_raw(&content)?;
        let folders = result.flatten_folders()?;
        // let first = result.folders;
        // let x = flatten_folders(first.expect("empty")[0]);
        // for y in x {
        //     let title = &y.title;
        //     println!("{}", title.as_ref().unwrap());
        // }
        Ok(())
    }

    #[test]
    fn test_other() -> Result<(), Box<dyn Error>> {
        let utf16 = open_utf16_file("A:/Chrome/DzsidaOrsolyaKerdestar20210331/189788754.upt")?;
        let content = convert_to_utf8(&utf16)?;
        let result = convert_raw(&content)?;
        let folders = result.flatten_folders()?;
        let collection = Collection::from(folders);

        //recursive(&first);
        Ok(())
    }
}
