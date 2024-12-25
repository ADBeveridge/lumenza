#[derive(Debug, PartialEq)]
pub enum Error {
    DatabaseError(rusqlite::Error),
    IoError(std::io::ErrorKind),
    InternalError(InternalError),
}

#[derive(Debug, PartialEq, Eq)]
pub enum InternalError {
    /// other error
    Other = -1000,
    /// The file is not supported by the library
    NotAnImage,
    /// A directory was specified when a non-directory was expected.
    IsADirectory,
    /// No name was specified
    EmptyName,
    /// Path did not exist
    PathNotExist,
}