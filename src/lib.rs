#![feature(error_generic_member_access)]

pub mod conversion;
pub mod error;
pub mod model;
pub mod unipol;

#[cfg(test)]
mod tests {
    use crate::model::Collection;
    use conversion::convert_raw;
    use model::QuestionType;
    use std::error::Error;

    use super::*;

    #[test]
    fn test_group() -> Result<(), Box<dyn Error>> {
        let content = include_str!("../assets/normalized/grouping.xml");
        let result = convert_raw(&content)?;
        let collection = Collection::new("test", result)?;

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
        let collection = Collection::new("test", result)?;

        assert_eq!(collection.folders.len(), 1);

        let folder = collection.folders.into_iter().next().unwrap();

        assert_eq!(folder.questions.len(), 4);

        Ok(())
    }
}
