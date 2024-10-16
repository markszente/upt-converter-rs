use serde_xml_rs::from_str;

use crate::{
    error::{
        conversion::ConvertError,
        model::{
            AnswerError, ComplexAnswerError, PredefinedAnswerError, QuestionError,
            QuestionTypeError,
        },
        unipol::UnipolError,
    },
    model::{Answer, AnswerWrapper, Collection, Folder, Question, QuestionType},
    unipol,
};

pub fn convert_raw(content: &str) -> Result<crate::unipol::Export, ConvertError> {
    let result: crate::unipol::Export = from_str(content)?;
    Ok(result)
}

impl Collection {
    pub fn new(name: &str, export: unipol::Export) -> Result<Collection, UnipolError> {
        let folders = export.flatten_folders()?;
        let folders = folders
            .iter()
            .map(|f| (f, get_valid_questions_from_folder(f)))
            .map(|(folder, questions)| Folder::new(folder.title.as_ref(), questions))
            .collect();

        Ok(Collection {
            name: name.to_string(),
            folders,
        })
    }

    /// Creates a collection with errors included
    ///
    /// Errors is a vec of errors for each folder (corresponding to `collection.folders`)
    pub fn new_with_error_details(
        name: &str,
        export: unipol::Export,
    ) -> Result<(Collection, Vec<Vec<QuestionError>>), UnipolError> {
        let folders = export.flatten_folders()?;
        let (folders, errors) = folders
            .iter()
            .map(|f| (f, get_all_questions_from_folder(f)))
            .map(|(folder, questions_and_errors)| {
                let (questions, errors): (Vec<_>, Vec<_>) =
                    questions_and_errors.into_iter().partition(|r| r.is_ok());

                let questions = questions.into_iter().map(|q| q.unwrap()).collect();

                let errors = errors.into_iter().map(|q| q.unwrap_err()).collect();

                let folder = Folder::new(folder.title.as_ref(), questions);

                (folder, errors)
            })
            .collect();

        let collection = Collection {
            name: name.to_string(),
            folders,
        };

        Ok((collection, errors))
    }
}

/// Converts a single folder into a list of questions
fn get_all_questions_from_folder(
    folder: &crate::unipol::Folder,
) -> Vec<Result<Question, QuestionError>> {
    let questions = match &folder.questions {
        Some(question_wrapper) => {
            let questions = question_wrapper
                .question
                .iter()
                .flatten()
                .map(|q| Question::try_from(q))
                .collect();
            questions
        }
        None => vec![],
    };
    questions
}

/// Converts a single folder into a list of questions, omitting invalid questions
fn get_valid_questions_from_folder(folder: &crate::unipol::Folder) -> Vec<Question> {
    get_all_questions_from_folder(folder)
        .into_iter()
        .filter_map(|q| q.ok())
        .collect()
}

impl TryFrom<&crate::unipol::Question> for Question {
    type Error = QuestionError;

    fn try_from(value: &crate::unipol::Question) -> Result<Self, Self::Error> {
        let text = value.title.resource.text.to_string();
        let question_type = QuestionType::try_from(value.r#type.as_str())?;
        let answer = AnswerWrapper::try_from(value)?;

        let possible_answers = match value.predefined_answers.value_set.as_ref() {
            Some(set) => set
                .values
                .value
                .iter()
                .map(|v| String::from(&v.resource_text.resource.text))
                .collect(),
            None => vec![],
        };

        let option_source = match question_type {
            QuestionType::Table => value.dimension_x.value_set.as_ref(),
            QuestionType::Group => value.dimension_y.value_set.as_ref(),
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

        Ok(Question {
            text,
            question_type,
            answer,
            possible_answers,
            possible_options,
        })
    }
}

impl TryFrom<&str> for QuestionType {
    type Error = QuestionTypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let result = match value {
            "Egysoros_szoveg" => Ok(QuestionType::ExactText),
            "Lista_egy_valaszthato_ertekkel_" => Ok(QuestionType::SingleAnswer),
            "Lista_tobb_valaszthato_ertekkel_" => Ok(QuestionType::MultipleAnswers),
            "Tablazat_soronkent_egy_lehetseges_valasszal" => Ok(QuestionType::Table),
            "Csoportokba_rendezes" => Ok(QuestionType::Group),
            _ => Err(QuestionTypeError {
                raw_type: value.to_string(),
            }),
        };

        result
    }
}

impl TryFrom<&crate::unipol::Question> for AnswerWrapper {
    type Error = AnswerError;

    fn try_from(value: &crate::unipol::Question) -> Result<Self, Self::Error> {
        let question_type = QuestionType::try_from(value.r#type.as_str())?;

        let single_answer = match question_type {
            QuestionType::ExactText => {
                let single_answer = String::from(
                    &value
                        .correct_question_answer
                        .first()
                        .ok_or(AnswerError::NoCorrectAnswer)?
                        .text_answer,
                );
                Some(single_answer)
            }
            _ => None,
        };

        let text_answers = match question_type {
            QuestionType::ExactText => value
                .correct_question_answer
                .iter()
                .map(|c| String::from(&c.text_answer))
                .collect::<Vec<_>>(),
            _ => vec![], // FIXME: this is by API design, might want to change?
        };

        let answers: Result<Vec<_>, AnswerError> = if question_type == QuestionType::ExactText {
            Ok(vec![]) // FIXME: I don't like it
        } else {
            value
                .correct_question_answer
                .first()
                .ok_or(ComplexAnswerError::NoGroup)?
                .correct_question_complex_answer
                .as_ref()
                .ok_or(ComplexAnswerError::NoAnswers)?
                .iter()
                .filter(|&c| {
                    if value.is_using_partial_points {
                        c.point_value > 0
                    } else {
                        true
                    }
                })
                .map(|c| {
                    match question_type {
                        QuestionType::Table => {
                            // this is necessary because for tables,
                            // "DimensionX" becomes the answers...
                            Ok(Answer {
                                answer_index: c.dimension_2 - 1,
                                option_index: c.dimension_1 - 1,
                            })
                        }
                        QuestionType::Group => {
                            // They seem to have abandoned the "dimension" logic (which is essentially a non-zero-based index...)
                            // In favor of using AnswerId. Yeah. Consistency
                            let index = value
                                .predefined_answers
                                .value_set
                                .as_ref()
                                .ok_or(PredefinedAnswerError::NoValue)?
                                .values
                                .value
                                .iter()
                                .position(|v| v.id == c.answer_id)
                                .ok_or(PredefinedAnswerError::NotFound)?;

                            Ok(Answer {
                                answer_index: index as i32,
                                option_index: c.dimension_2 - 1,
                            })
                        }
                        _ => Ok(Answer {
                            answer_index: c.dimension_1 - 1,
                            option_index: c.dimension_2 - 1,
                        }),
                    }
                })
                .collect()
        };

        let answers = answers?;

        let result = AnswerWrapper {
            single_answer,
            text_answers,
            answers,
        };

        Ok(result)
    }
}
