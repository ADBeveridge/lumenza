/*	Copyright (C) 2024 Alan Beveridge

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License along
    with this program; if not, write to the Free Software Foundation, Inc.,
    51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
*/

#[cfg(test)]
mod tests {
    use lumenza::library::Library;
    use lumenza::picture::Picture;
    use std::path::{self, PathBuf};
    use tempdir::TempDir;

    #[test]
    fn create_library() {
        let dir = TempDir::new("lumenza").unwrap();

        let config = dir.path().join("default.conf");
        let thumbnails = dir.path().join("thumbnails/");
        let pictures = dir.path().join("pictures/");
        let database = dir.path().join("database.sqlite3");

        Library::create(
            &config,
            &thumbnails,
            &vec![pictures.clone()],
            &database
        )
        .unwrap();

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

        let library = Library::create(
            &config,
            &thumbnails,
            &vec![pictures],
            &database
        )
        .unwrap();

        let file = PathBuf::from("tests/images/lake.jpg");
        Picture::new(&library, &file).unwrap();
    }
    #[test]
    fn scan_folder() {
        let dir = TempDir::new("lumenza").unwrap();

        let config = dir.path().join("default.conf");
        let thumbnails = dir.path().join("thumbnails/");
        let pictures = dir.path().join("pictures/");
        let database = dir.path().join("database.sqlite3");

        let library_new = Library::create(
            &config,
            &thumbnails,
            &vec![pictures],
            &database
        )
        .unwrap();
        let folder_path = path::Path::new("tests/images/").to_path_buf();
        let res = library_new.scan_folder(&folder_path).unwrap();

        assert_eq!(res, ());
    }

    #[test]
    fn open_library() {
        let dir = TempDir::new("lumenza").unwrap();

        let config = dir.path().join("default.conf");
        let thumbnails = dir.path().join("thumbnails/");
        let pictures = dir.path().join("pictures/");
        let database = dir.path().join("database.sqlite3");

        let library_new = Library::create(
            &config,
            &thumbnails,
            &vec![pictures.clone()],
            &database
        )
        .unwrap();
        let folder_path = PathBuf::from("tests/images/");
        library_new.scan_folder(&folder_path).unwrap();

        let library = Library::open(&config).unwrap();
        
        assert_eq!(library.fs.get_config_path(), config);
        assert_eq!(library.fs.get_pictures_path(), vec![pictures]);
        assert_eq!(library.fs.get_thumbnails_path(), thumbnails);
    }

    #[test]
    fn list_all_pictures() {
        let dir = TempDir::new("lumenza").unwrap();

        let config = dir.path().join("default.conf");
        let thumbnails = dir.path().join("thumbnails/");
        let pictures = dir.path().join("pictures/");
        let database = dir.path().join("database.sqlite3");

        let library = Library::create(
            &config,
            &thumbnails,
            &vec![pictures.clone()],
            &database
        )
        .unwrap();
        
        let folder_path = PathBuf::from("tests/images/");
        library.scan_folder(&folder_path).unwrap();

        let pictures = library.list_all_pictures().unwrap();

        // There are two pictures bundled in the tests folder, hence the 2.
        assert_eq!(2, pictures.len());
    }
}
