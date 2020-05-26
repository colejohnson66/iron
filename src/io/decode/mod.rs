/* ============================================================================
 * File:   mod.rs
 * Author: Cole Johnson
 * ============================================================================
 * Copyright (c) 2020 Cole Johnson
 *
 * This file is part of Iron.
 *
 * Iron is free software: you can redistribute it and/or modify it under the
 *   terms of the GNU General Public License as published by the Free Software
 *   Foundation, either version 3 of the License, or (at your option) any later
 *   version.
 *
 * Iron is distributed in the hope that it will be useful, but WITHOUT ANY
 *   WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
 *   FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
 *   details.
 *
 * You should have received a copy of the GNU General Public License along with
 *   Iron. If not, see <http://www.gnu.org/licenses/>.
 * ============================================================================
 */
pub mod utf8;

use std::io;

pub enum ConvertByteResult {
    Char(char),
    Error,
    ErrorWithPrepend(u8),
    Finished,
}

/// Implemented by types that decode byte readers into characters
pub trait Decoder {
    /// Constructs a new `Decoder` from a type implementing `Read`
    fn new(reader: Box<impl io::Read + 'static>) -> Self;
    /// Reads a character from `reader` and returns it
    ///
    /// On success, Ok(...) will have one of the following values:
    /// - None: EOF reached
    /// - Some(char): The decoded character
    ///
    /// On error, Err(...) will have one of the following values:
    /// - -1: EOF reached in the middle of a codepoint
    /// - 0: Invalid byte read
    fn read(&mut self) -> ConvertByteResult;
}
