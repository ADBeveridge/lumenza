#[cfg(test)]
mod tests {
    use lumenza::library::Library;
    use std::path::{self, PathBuf};
    use tempdir::TempDir;

    #[test]
    fn create_library() {
        let dir = TempDir::new("lumenza").unwrap();

        let config = dir.path().join("default.conf");
        let thumbnails = dir.path().join("thumbnails/");
        let pictures = dir.path().join("pictures/");
        let database = dir.path().join("database.sqlite3");

        Library::create(&config, &thumbnails, &vec![pictures.clone()], &database).unwrap();

        assert!(std::fs::exists(config).unwrap());
        assert!(std::fs::exists(thumbnails).unwrap());
        assert!(std::fs::exists(pictures).unwrap());
        assert!(std::fs::exists(database).unwrap());
    }

    #[test]
    fn insert_picture() {
        let dir = TempDir::new("lumenza").unwrap();

        let config = dir.path().join("default.conf");
        let thumbnails = dir.path().join("thumbnails/");
        let pictures = dir.path().join("pictures/");
        let database = dir.path().join("database.sqlite3");

        let library = Library::create(&config, &thumbnails, &vec![pictures], &database).unwrap();

        let file = PathBuf::from("tests/images/lake.jpg");
        library.add_picture(&file).unwrap();
    }
    #[test]
    fn scan_folder() {
        let dir = TempDir::new("lumenza").unwrap();

        let config = dir.path().join("default.conf");
        let thumbnails = dir.path().join("thumbnails/");
        let pictures = dir.path().join("pictures/");
        let database = dir.path().join("database.sqlite3");

        let library_new = Library::create(&config, &thumbnails, &vec![pictures], &database).unwrap();
        let folder_path = path::Path::new("tests/images/").to_path_buf();
        let res = library_new.process_folder(&folder_path).unwrap();

        assert_eq!(res, ());
    }

    #[test]
    fn open_library() {
        let dir = TempDir::new("lumenza").unwrap();

        let config = dir.path().join("default.conf");
        let thumbnails = dir.path().join("thumbnails/");
        let pictures = dir.path().join("pictures/");
        let database = dir.path().join("database.sqlite3");

        let library_new =
            Library::create(&config, &thumbnails, &vec![pictures.clone()], &database).unwrap();
        let folder_path = PathBuf::from("tests/images/");
        library_new.process_folder(&folder_path).unwrap();

        // The test here is making sure we can open the library at all.
        Library::open(&config).unwrap();
    }

    #[test]
    fn list_all_pictures() {
        let dir = TempDir::new("lumenza").unwrap();

        let config = dir.path().join("default.conf");
        let thumbnails = dir.path().join("thumbnails/");
        let pictures = dir.path().join("pictures/");
        let database = dir.path().join("database.sqlite3");

        let library = Library::create(&config, &thumbnails, &vec![pictures.clone()], &database).unwrap();

        let folder_path = PathBuf::from("tests/images/");
        library.process_folder(&folder_path).unwrap();

        let pictures = library.list_all_pictures().unwrap();

        // There are two pictures bundled in the tests folder, hence the 2.
        assert_eq!(2, pictures.len());
    }
}
