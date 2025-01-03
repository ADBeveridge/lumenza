use thiserror::Error;
use rusqlite;
use toml;

#[derive(Error, Debug, PartialEq)]
pub enum LumenzaError {
    #[error("Picture already in library")]
    PictureAlreadyInLibrary(),

    #[error("File not found")]
    FileNotFound(),

    #[error("std::io::error occurred")] // std::io::error doesn't implement PartialEq -_-
    IoError(),

    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] toml::ser::Error),

    #[error("Deserialization error: {0}")]
    DeserializationError(#[from] toml::de::Error),

    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("Image error")]
    ImageError(),
}
