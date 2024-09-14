use serde_derive::Deserialize;
use serde_derive::{self, Serialize};

use crate::unipol;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub name: String,
    pub folders: Vec<Folder>,
}

impl Collection {
    pub fn from(raw: Vec<unipol::Folder>) -> Collection {
        let folders = raw
            .iter()
            .map(|f| Folder::from(f))
            .filter_map(|f| f)
            .collect();

        Collection {
            name: String::from("test"),
            folders,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    pub name: String,
    pub questions: Vec<Question>,
}

impl Folder {
    fn from(raw: &unipol::Folder) -> Option<Folder> {
        let name = match &raw.title {
            Some(t) => String::from(t),
            None => String::from("unknown"),
        };
        let questions = match &raw.questions {
            Some(question_wrapper) => {
                let questions = question_wrapper
                    .question
                    .iter()
                    .flatten()
                    .map(|q| Question::from(q))
                    .filter_map(|q| q)
                    .collect();
                questions
            }
            None => return None,
        };

        Some(Folder { name, questions })
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

impl QuestionType {
    fn from(raw: &str) -> Option<QuestionType> {
        let result = match raw {
            "Egysoros_szoveg" => Some(QuestionType::ExactText),
            "Lista_egy_valaszthato_ertekkel_" => Some(QuestionType::SingleAnswer),
            "Lista_tobb_valaszthato_ertekkel_" => Some(QuestionType::MultipleAnswers),
            "Tablazat_soronkent_egy_lehetseges_valasszal" => Some(QuestionType::Table),
            "Csoportokba_rendezes" => Some(QuestionType::Group),
            _ => None,
        };

        result
    }
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

impl Question {
    pub fn from(raw: &unipol::Question) -> Option<Question> {
        // todo: unique return type
        let text = String::from(&raw.title.resource.text);
        let question_type = match QuestionType::from(&raw.r#type) {
            Some(t) => t,
            None => return None,
        };
        let answer = AnswerWrapper::from(raw, &question_type);

        let possible_answers = match raw.predefined_answers.value_set.as_ref() {
            Some(set) => set
                .values
                .value
                .iter()
                .map(|v| String::from(&v.resource_text.resource.text))
                .collect(),
            None => vec![],
        };

        let option_source = match question_type {
            QuestionType::Table => raw.dimension_x.value_set.as_ref(),
            QuestionType::Group => raw.dimension_y.value_set.as_ref(),
            _ => None,
        };

        let possible_options = match option_source {
            Some(source) => source
                .values
                .value
                .iter()
                .map(|v| String::from(&v.resource_text.resource.text))
                .collect(),
            None => vec![],
        };

        Some(Question {
            text,
            question_type,
            answer,
            possible_answers,
            possible_options,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AnswerWrapper {
    pub single_answer: Option<String>,
    pub text_answers: Vec<String>,
    pub answers: Vec<Answer>,
}

impl AnswerWrapper {
    pub fn from(raw: &unipol::Question, r#type: &QuestionType) -> AnswerWrapper {
        let single_answer = match r#type {
            QuestionType::ExactText => {
                let single_answer = String::from(&raw.correct_question_answer[0].text_answer);
                Some(single_answer)
            }
            _ => None,
        };

        let text_answers = match r#type {
            QuestionType::ExactText => raw
                .correct_question_answer
                .iter()
                .map(|c| String::from(&c.text_answer))
                .collect::<Vec<_>>(),
            _ => vec![], // this is by API design, might want to change?
        };

        let answers = if *r#type == QuestionType::ExactText {
            vec![] // I don't like it
        } else {
            raw.correct_question_answer[0]
                .correct_question_complex_answer
                .as_ref()
                .expect("no complex answer")
                .iter()
                .filter(|&c| {
                    if raw.is_using_partial_points {
                        c.point_value > 0
                    } else {
                        true
                    }
                })
                .map(|c| {
                    match r#type {
                        QuestionType::Table => {
                            // this is necessary because for tables,
                            // "DimensionX" becomes the answers... Why...
                            Answer {
                                answer_index: c.dimension_2 - 1,
                                option_index: c.dimension_1 - 1,
                            }
                        }
                        QuestionType::Group => {
                            // Where do I get started, lmao. They've abandoned the whole """index""" logic (which doesn't start with 0, *facepalm*),
                            // In favor of using AnswerId. Yeah. Why. If you're stupid, at least be consistent about it
                            let index = raw
                                .predefined_answers
                                .value_set
                                .as_ref()
                                .expect("no value set")
                                .values
                                .value
                                .iter()
                                .position(|v| v.id == c.answer_id)
                                .expect("index not found");

                            Answer {
                                answer_index: index as u32,
                                option_index: c.dimension_2 - 1,
                            }
                        }
                        _ => Answer {
                            answer_index: c.dimension_1 - 1,
                            option_index: c.dimension_2 - 1,
                        },
                    }
                })
                .collect()
        };

        AnswerWrapper {
            single_answer,
            text_answers,
            answers,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Answer {
    pub answer_index: u32,
    pub option_index: u32,
}
