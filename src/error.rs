#[derive(Debug, PartialEq)]
pub enum OsplError {
    DatabaseError(rusqlite::Error),
    IoError(std::io::ErrorKind),
    InternalError(Error),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
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