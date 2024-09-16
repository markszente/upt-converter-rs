use std::collections::HashMap;

use serde_derive::Deserialize;
use serde_derive::{self, Serialize};

use crate::convert::{get_all_questions_from_folder, get_valid_questions_from_folder};
use crate::error::model::QuestionError;
use crate::unipol;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub name: String,
    pub folders: Vec<Folder>,
}

impl Collection {
    pub fn new(name: &str, folders: Vec<unipol::Folder>) -> Collection {
        let folders = folders
            .iter()
            .map(|f| (f, get_valid_questions_from_folder(f)))
            .map(|(folder, questions)| Folder::new(folder.title.as_ref(), questions))
            .collect();

        Collection {
            name: name.to_string(),
            folders,
        }
    }

    /// Creates a collection with errors included
    ///
    /// Errors is hash map of indices of folder (as returned in `collection.folders`) and errors
    pub fn new_with_error_details(
        name: &str,
        folders: Vec<unipol::Folder>,
    ) -> (Collection, HashMap<usize, Vec<QuestionError>>) {
        let (folders, errors) = folders
            .iter()
            .map(|f| (f, get_all_questions_from_folder(f)))
            .enumerate()
            .map(|(i, (folder, questions_and_errors))| {
                let (questions, errors): (Vec<_>, Vec<_>) =
                    questions_and_errors.into_iter().partition(|r| r.is_ok());

                let questions = questions.into_iter().map(|q| q.unwrap()).collect();

                let errors = errors.into_iter().map(|q| q.unwrap_err()).collect();

                let folder = Folder::new(folder.title.as_ref(), questions);

                (folder, (i, errors))
            })
            .collect();

        let collection = Collection {
            name: name.to_string(),
            folders,
        };

        (collection, errors)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    pub name: String,
    pub questions: Vec<Question>,
}

impl Folder {
    fn new<T: AsRef<str>>(name: Option<T>, questions: Vec<Question>) -> Folder {
        let name = name
            .map(|n| n.as_ref().to_string())
            .unwrap_or_else(|| "Unnamed".to_string());

        Folder { name, questions }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum QuestionType {
    ExactText = 0,
    SingleAnswer = 1,
    MultipleAnswers = 2,
    Table = 3, // not sure about the naming here
    Group = 4,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    pub text: String,
    #[serde(rename = "type")]
    pub question_type: QuestionType,
    pub possible_answers: Vec<String>,
    pub possible_options: Vec<String>,
    #[serde(rename = "answers")]
    pub answer: AnswerWrapper,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AnswerWrapper {
    pub single_answer: Option<String>,
    pub text_answers: Vec<String>,
    pub answers: Vec<Answer>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Answer {
    pub answer_index: i32,
    pub option_index: i32,
}
