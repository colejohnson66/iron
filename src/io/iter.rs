/* ============================================================================
 * File:   iter.rs
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
pub struct CharWithOffsetIterator {
    contents: Vec<char>,
    pub pos: usize,
    pub line: u32,
    pub line_pos: u32,
}

impl CharWithOffsetIterator {
    pub fn new(string: &str) -> CharWithOffsetIterator {
        CharWithOffsetIterator {
            contents: string.chars().collect(),
            pos: 0,
            line: 0,
            line_pos: 0,
        }
    }

    pub fn seek(&mut self, new_pos: usize) {
        self.pos = 0;
        self.line = 0;
        self.line_pos = 0;
        // TODO: inefficient
        for _ in 0..new_pos {
            self.read();
        }
    }

    pub fn read(&mut self) -> Option<char> {
        if self.pos >= self.contents.len() {
            return None;
        }
        // Bounds checked already performed, so just `unwrap()`
        let c = self.contents.get(self.pos).unwrap();
        if *c == '\n' {
            self.line += 1;
            self.line_pos = 0;
        } else {
            self.line_pos += 1;
        }

        Some(*c)
    }

    pub fn read_multiple(&mut self, buf: &mut [char]) -> usize {
        // attempts to read `buf.len()` chars, but if `self.read()` returns `None`, aborts
        for n in 0..buf.len() {
            let c = self.read();
            match c {
                Some(c) => buf[n] = c,
                None => return n,
            }
        }
        // `buf.len()` characters read successfully
        buf.len()
    }

    pub fn backtrack(&mut self) {
        if self.pos >= self.contents.len() {
            unreachable!();
        }
        // Bounds checked already performed, so just `unwrap()`
        let c = self.contents.get(self.pos).unwrap();
        if *c == '\n' {
            self.line -= 1;
            // TODO: calculate this by counting number of characters until last '\n'
            self.line_pos = 5000;
        }

        self.pos -= 1;
    }

    pub fn backtrack_multiple(&mut self, count: usize) {
        if self.pos >= self.contents.len() {
            unreachable!();
        }

        // if backtracking more than possible, just reset to 0
        if count >= self.pos {
            self.pos = 0;
            self.line = 0;
            self.line_pos = 0;
            return;
        }

        // TODO: inefficient
        for _ in 0..count {
            self.backtrack();
        }
    }
}
