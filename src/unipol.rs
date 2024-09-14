use std::collections::VecDeque;
use std::error::Error;

use serde;
use serde_derive;
use serde_derive::Deserialize;

use crate::error::unipol::{UnipolError, UnipolFolderError};

#[derive(Deserialize, Debug, Clone)]
pub struct Export {
    #[serde(rename = "Folder")]
    pub folders: Option<Vec<Folder>>,
}

impl Export {
    pub fn flatten_folders(self) -> Result<Vec<Folder>, UnipolError> {
        let mut result = vec![];
        let mut q = VecDeque::new();

        let mut top_folders = match self.folders {
            None => return Err(UnipolError::FlattenError(UnipolFolderError::NoTopFolders)),
            Some(v) => v,
        };

        let top_first_folder = match top_folders.pop() {
            Some(v) => v,
            None => {
                return Err(UnipolError::FlattenError(
                    UnipolFolderError::NoFirstTopFolder,
                ))
            }
        };

        q.push_back(top_first_folder);

        while let Some(t) = q.pop_front() {
            let (without_subfolders, sub_folders) = t.remove_subfolders();
            //t.folders.folder = None;
            result.push(without_subfolders);
            if let Some(sub_folders) = sub_folders {
                for child in sub_folders {
                    q.push_back(child);
                }
            }
        }

        Ok(result)
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Folder {
    pub title: Option<String>,
    pub questions: Option<QuestionWrapper>,
    pub folders: FolderWrapper,
}

impl Folder {
    fn remove_subfolders(mut self) -> (Folder, Option<Vec<Folder>>) {
        let folders = self.folders.folder;
        self.folders.folder = None;
        (self, folders)
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct FolderWrapper {
    pub folder: Option<Vec<Folder>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct QuestionWrapper {
    pub question: Option<Vec<Question>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Question {
    pub title: ResourceWrapper,
    pub r#type: String,
    pub dimension_x: Dimension,
    pub dimension_y: Dimension,
    pub predefined_answers: Dimension,
    pub correct_question_answer: Vec<CorrectQuestionAnswer>,
    pub is_using_partial_points: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ResourceWrapper {
    pub resource: Resource,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Resource {
    pub text: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Dimension {
    pub value_set: Option<ValueSet>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ValueSet {
    pub values: ValueWrapper,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ValueWrapper {
    pub value: Vec<Value>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Value {
    pub resource_text: ResourceWrapper,
    pub id: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CorrectQuestionAnswer {
    pub text_answer: String,
    pub correct_question_complex_answer: Option<Vec<CorrectQuestionComplexAnswer>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CorrectQuestionComplexAnswer {
    pub point_value: u32,
    pub dimension_1: u32,
    pub dimension_2: u32,
    pub text_answer: String,
    pub answer_id: String,
}
