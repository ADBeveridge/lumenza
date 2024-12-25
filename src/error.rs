/*	Copyright (C) 2019-2024 Angelo Frangione & Alan Beveridge

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