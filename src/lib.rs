#![feature(error_generic_member_access)]

use error::convert::ConvertError;
use model::Collection;
use serde::Deserialize;
use serde_derive::{Deserialize, Serialize};
use serde_xml_rs::{de::Deserializer, from_str, to_string};
use std::{collections::VecDeque, error::Error, fs, path::Path};
use unipol::{Export, Folder};
use utf16string::{WStr, LE};

mod convert;
pub mod error;
mod macros;
pub mod model;
pub mod unipol;

pub fn convert_raw(content: &str) -> Result<crate::unipol::Export, ConvertError> {
    let result: crate::unipol::Export = from_str(content)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use model::QuestionType;

    use crate::model::Question;

    use super::*;

    #[test]
    fn test_group() -> Result<(), Box<dyn Error>> {
        let content = include_str!("../assets/normalized/grouping.xml");
        let result = convert_raw(&content)?;
        let folders = result.flatten_folders()?;
        let collection = Collection::new("test", folders);

        assert_eq!(collection.folders.len(), 1);

        let folder = &collection.folders[0];
        assert_eq!(folder.questions.len(), 1);

        let question = &folder.questions[0];

        assert_eq!(question.question_type, QuestionType::Group);

        Ok(())
    }

    #[test]
    fn test_other() -> Result<(), Box<dyn Error>> {
        let content = include_str!("../assets/normalized/other.xml");
        let result = convert_raw(&content)?;
        let folders = result.flatten_folders()?;
        let collection = Collection::new("test", folders);

        assert_eq!(collection.folders.len(), 1);

        let folder = collection.folders.into_iter().next().unwrap();

        assert_eq!(folder.questions.len(), 4);

        Ok(())
    }
}
