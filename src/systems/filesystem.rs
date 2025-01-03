use std::path::PathBuf;

use crate::error::LumenzaError;

pub struct Filesystem {
    _config_path: PathBuf,
    _pictures_paths: Vec<PathBuf>,
    _thumbnails_path: PathBuf,
}

// Static Methods
impl Filesystem {
    // Initialize filsystem structure for library.
    pub(crate) fn new(
        config: &PathBuf,
        thumbnails: &PathBuf,
        pictures: &Vec<PathBuf>,
    ) -> Result<Self, LumenzaError> {
        // Create empty folders for a new library. Config file will be created by config system, same with database.
        std::fs::create_dir_all(PathBuf::from(thumbnails)).map_err(|_| LumenzaError::IoError())?;
        for path in pictures {
            std::fs::create_dir_all(path).map_err(|_| LumenzaError::IoError())?;
        }

        // Now that the files are there, we can create the whole thing.
        let fs = Self::open(config, thumbnails, pictures)?;

        Ok(fs)
    }

    /// Loads the file paths into the Filesystem object and returns it.
    pub(crate) fn open(
        config_path: &PathBuf,
        thumbnails_path: &PathBuf,
        pictures_paths: &Vec<PathBuf>,
    ) -> Result<Self, LumenzaError> {
        return Ok(Filesystem {
            _config_path: config_path.clone(),
            _thumbnails_path: thumbnails_path.clone(),
            _pictures_paths: pictures_paths.clone(),
        });
    }
}

