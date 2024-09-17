use super::unipol::UnipolError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("Serde error")]
    SerdeError(
        #[from]
        #[backtrace]
        serde_xml_rs::Error,
    ),
    #[error("Unipol error")]
    UnipolError(
        #[from]
        #[backtrace]
        UnipolError,
    ),
}
