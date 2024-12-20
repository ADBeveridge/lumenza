/*	libospl - Open Source Photo Library
    an opensource and multiplateform photo library management that can be used
    to store and sort all your photos.
    Copyright (C) 2019-2023 Angelo Frangione & Alan Beveridge

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

pub fn sql_schema() -> String {
    let string = "
        -- Table where each row represents a photo.
        CREATE TABLE IF NOT EXISTS photos (
            id                      INTEGER NOT NULL UNIQUE,
            filename                TEXT NOT NULL,
            hash                    BLOB NOT NULL,
            thumbnail_name          TEXT,
            width                   INTEGER NOT NULL,
            height                  INTEGER NOT NULL,
            missing                 INTEGER NOT NULL,
            PRIMARY KEY(id AUTOINCREMENT)
        );
        
        -- Table where each row represents a tag.
        CREATE TABLE IF NOT EXISTS tags (
            id                      INTEGER NOT NULL UNIQUE,
            name                    TEXT NOT NULL UNIQUE,
            PRIMARY KEY(id AUTOINCREMENT)
        );
        
        -- Link table between photos and tags.
        CREATE TABLE IF NOT EXISTS photos_tags_map (
            containing_tag          INTEGER NOT NULL,
            contained_photo         INTEGER NOT NULL,
            FOREIGN KEY(contained_photo) REFERENCES photos(id),
            FOREIGN KEY(containing_tag) REFERENCES tags(id)
        );
    ";

    string.to_string()
}