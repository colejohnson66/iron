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
pub struct Utf16String {
    vec: Vec<u16>,
}

impl Utf16String {
    pub fn new() -> Utf16String {
        Utf16String { vec: vec![] }
    }

    pub fn with_capacity(capacity: usize) -> Utf16String {
        Utf16String {
            vec: Vec::with_capacity(capacity),
        }
    }

    pub fn push_str(&mut self, string: &Utf16String) {
        self.vec.extend_from_slice(&string.vec[..]);
    }

    pub fn capacity(&self) -> usize {
        self.vec.capacity()
    }

    pub fn reserve(&mut self, extra: usize) {
        self.vec.reserve(extra);
    }

    pub fn push(&mut self, cp: u16) {
        self.vec.push(cp);
    }

    // NOTE: will add two codepoints if surrogate pairs are needed
    pub fn push_char(&mut self, c: char) {
        match c as u32 {
            0..=0xD7FF => self.vec.push(c as u16),
            0xD800..=0xDFFF => panic!(), // TODO: is this correct behavior?
            0xE000..=0xFFFF => self.vec.push(c as u16),
            0x10000..=0x10FFFF => {
                let u = c as u32;
                let u_prime = u - 0x10000;
                let high = u_prime / 0x400 + 0xD800;
                let low = u_prime % 0x400 + 0xDC00;
                self.vec.push(high as u16);
                self.vec.push(low as u16);
            }
            _ => unreachable!(),
        }
    }
}
