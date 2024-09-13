use thiserror::Error;

#[derive(Error, Debug)]
pub enum QuestionError {
    #[error("Unknown question type: {0:?}")]
    UnknownQuestionType(QuestionTypeError),
    #[error("Answer error")]
    AnswerError(AnswerError),
}

#[derive(Error, Debug)]
pub enum AnswerError {
    #[error("Unknown question type: {0:?}")]
    UnknownQuestionType(QuestionTypeError),
    #[error("No correct answer")]
    NoCorrectAnswer,
    #[error("No complex answers")]
    NoComplexAnswer(ComplexAnswerError),
    #[error("No correct answers")]
    NoPredefinedAnswer(PredefinedAnswerError),
}

impl From<AnswerError> for QuestionError {
    fn from(error: AnswerError) -> Self {
        QuestionError::AnswerError(error)
    }
}

#[derive(Error, Debug)]
pub enum ComplexAnswerError {
    #[error("No group")]
    NoGroup,
    #[error("No answers")]
    NoAnswers,
}

impl From<ComplexAnswerError> for AnswerError {
    fn from(error: ComplexAnswerError) -> Self {
        AnswerError::NoComplexAnswer(error)
    }
}

#[derive(Error, Debug)]
pub enum PredefinedAnswerError {
    #[error("No value")]
    NoValue,
    #[error("Not found")]
    NotFound,
}

impl From<PredefinedAnswerError> for AnswerError {
    fn from(error: PredefinedAnswerError) -> Self {
        AnswerError::NoPredefinedAnswer(error)
    }
}

#[derive(Error, Debug)]
pub struct QuestionTypeError {
    pub raw_type: String,
}

impl From<QuestionTypeError> for AnswerError {
    fn from(error: QuestionTypeError) -> Self {
        AnswerError::UnknownQuestionType(error)
    }
}

impl From<QuestionTypeError> for QuestionError {
    fn from(error: QuestionTypeError) -> Self {
        QuestionError::UnknownQuestionType(error)
    }
}
