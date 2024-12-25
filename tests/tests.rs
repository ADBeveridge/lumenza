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
    use lumenza::library;
    use tempdir::TempDir;

    #[test]
    fn create_library() {
        let dir = TempDir::new("lumenza").unwrap();

        let config = dir.path().join("default.conf");
        let thumbnails = dir.path().join("thumbnails/");
        let pictures = dir.path().join("pictures/");
        let database = dir.path().join("database.sqlite3");

        library::Library::create(config.clone(), thumbnails.clone(), pictures.clone(), database.clone()).unwrap();

        assert!(std::fs::exists(config).unwrap());
        assert!(std::fs::exists(thumbnails).unwrap());
        assert!(std::fs::exists(pictures).unwrap());
        assert!(std::fs::exists(database).unwrap());
    }
}
