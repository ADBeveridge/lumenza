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

//! Lumenza is a cross-platform picture manager. 
//! It handles core tasks, such as storing data in a database,
//! tagging, and album management.

/// Lumenza specific errors
pub mod error;

/// Library level functions
pub mod library;

/// Picture level functions
pub mod picture;

pub(crate) mod systems;
