use std::fmt;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum QuestionError {
    #[error("Unknown question type: {0:?}")]
    UnknownQuestionType(#[from] QuestionTypeError),
    #[error("Answer error")]
    AnswerError(#[from] AnswerError),
}

#[derive(Error, Debug)]
pub enum AnswerError {
    #[error("Unknown question type: {0:?}")]
    UnknownQuestionType(#[from] QuestionTypeError),
    #[error("No correct answer")]
    NoCorrectAnswer,
    #[error("No complex answers")]
    NoComplexAnswer(#[from] ComplexAnswerError),
    #[error("No correct answers")]
    NoPredefinedAnswer(#[from] PredefinedAnswerError),
}

#[derive(Error, Debug)]
pub enum ComplexAnswerError {
    #[error("No group")]
    NoGroup,
    #[error("No answers")]
    NoAnswers,
}

#[derive(Error, Debug)]
pub enum PredefinedAnswerError {
    #[error("No value")]
    NoValue,
    #[error("Not found")]
    NotFound,
}

#[derive(Error, Debug)]
pub struct QuestionTypeError {
    pub raw_type: String,
}

impl fmt::Display for QuestionTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid question type: {}", self.raw_type)
    }
}
