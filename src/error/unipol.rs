
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UnipolError {
    #[error("Flattening failed")]
    FlattenError(UnipolFolderError),
}

#[derive(Error, Debug)]
pub enum UnipolFolderError {
    #[error("There are no top level folders")]
    NoTopFolders,
    #[error("Top level folder array is empty")]
    NoFirstTopFolder,
}
