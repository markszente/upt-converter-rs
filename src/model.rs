use serde_derive::Deserialize;
use serde_derive::{self, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub name: String,
    pub folders: Vec<Folder>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    pub name: String,
    pub questions: Vec<Question>,
}

impl Folder {
    pub(crate) fn new<T: AsRef<str>>(name: Option<T>, questions: Vec<Question>) -> Folder {
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
