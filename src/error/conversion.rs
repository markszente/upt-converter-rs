use thiserror::Error;
use utf16string::Utf16Error;

use super::unipol::UnipolError;

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("Could not open file")]
    OpenFileError(
        #[from]
        #[backtrace]
        std::io::Error,
    ),
    #[error("Encoding error")]
    EncodingError(
        #[from]
        #[backtrace]
        Utf16Error,
    ),
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
