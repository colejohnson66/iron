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
use std::io::BufRead;

/// A line-based char iterator.
/// EOF is stored internally as setting `line` to `buffer.len()` or `line_offsets.len()`
pub struct LineOffsetIterator {
    buffer: Vec<String>,
    line_offsets: Vec<usize>,
    eof_offset: usize,
    line: usize,
    line_pos: usize,
}

impl LineOffsetIterator {
    pub fn new<T: BufRead>(buffer: &mut T) -> LineOffsetIterator {
        let buf = LineOffsetIterator::read_all_lines(buffer);
        let offsets = LineOffsetIterator::get_line_offsets(&buf);
        LineOffsetIterator {
            buffer: buf,
            line_offsets: offsets.0,
            eof_offset: offsets.1,
            line: 0,
            line_pos: 0,
        }
    }

    fn read_all_lines<T: BufRead>(buffer: &mut T) -> Vec<String> {
        let mut vec: Vec<String> = vec![];

        loop {
            let mut line = String::new();
            match buffer.read_line(&mut line) {
                Ok(0) => break, // EOF
                Ok(_) => vec.push(line),
                Err(_) => panic!(),
            }
        }

        vec
    }

    // returns a tuple with `0` containing the line offsets and `1` containing the EOF offset
    fn get_line_offsets(lines: &Vec<String>) -> (Vec<usize>, usize) {
        let mut vec: Vec<usize> = Vec::with_capacity(lines.len());

        let mut offset = 0usize;
        for line in lines {
            vec.push(offset);
            offset += line.len();
        }

        (vec, offset)
    }

    pub fn seek(&mut self, new_pos: usize) {
        // TODO: add error handling
        if new_pos > self.eof_offset {
            panic!();
        }

        // TODO: FIXME: fix
        if new_pos == self.eof_offset {
            unimplemented!();
        }

        // find what line `new_pos` is in
        let new_line = {
            let mut line = 0usize;
            for (i, this_off) in self.line_offsets.iter().enumerate() {
                line = i;
                if *this_off >= new_pos {
                    break;
                }
            }
            line
        };

        let new_line_off = new_pos - self.line_offsets.get(new_line).unwrap();

        self.line = new_line;
        self.line_pos = new_line_off;
    }

    // same as `peek`, but increments the pointer before returning
    pub fn read(&mut self) -> Option<char> {
        // possible EOF?
        if self.line == self.buffer.len() {
            return None;
        }

        let cur_line = self.buffer.get(self.line).unwrap();
        let cur_line_len = cur_line.chars().count();
        if self.line_pos >= cur_line_len {
            panic!();
        }
        let c = cur_line.chars().nth(self.line_pos).unwrap();

        // increment pointer
        let new_line_pos = self.line_pos + 1;
        if new_line_pos == cur_line_len {
            self.line += 1;
            self.line_pos = 0;
        } else {
            self.line_pos = new_line_pos;
        }

        Some(c)
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
        // beginning of buffer?
        if self.line == 0 && self.line_pos == 0 {
            return;
        }

        // EOF?
        if self.line == self.buffer.len() {
            self.line -= 1;
            self.line_pos = self.buffer.get(self.line).unwrap().chars().count() - 1;
            return;
        }

        // beginning of line?
        if self.line_pos == 0 {
            self.line -= 1;
            self.line_pos = self.buffer.get(self.line).unwrap().chars().count() - 1;
            return;
        }

        // middle of line
        self.line_pos -= 1;
    }

    pub fn backtrack_multiple(&mut self, count: usize) {
        for _ in 0..count {
            self.backtrack();
        }
    }

    // same as `read`, but doesn't increment the pointer
    pub fn peek(&self) -> Option<char> {
        // possible EOF?
        if self.line == self.buffer.len() {
            return None;
        }

        let cur_line = self.buffer.get(self.line).unwrap();
        let cur_line_len = cur_line.chars().count();
        if self.line_pos >= cur_line_len {
            panic!();
        }
        let c = cur_line.chars().nth(self.line_pos).unwrap();

        Some(c)
    }

    pub fn peek_multiple(&mut self, buf: &mut [char]) -> usize {
        // attempts to peek `buf.len()` chars, but aborts early if EOF is reached
        let mut line_to_peek = self.line;
        let mut line_to_peek_off = self.line_pos;

        for n in 0..buf.len() {
            // EOF?
            if line_to_peek == self.buffer.len() {
                return n;
            }

            // peek a char
            let mut peeked_line = self.buffer.get(line_to_peek).unwrap();
            let peeked_line_len = peeked_line.chars().count(); // TODO: don't do this every iteration

            // at end of line?
            if line_to_peek_off == peeked_line_len {
                line_to_peek += 1;
                line_to_peek_off = 0;
                peeked_line = match self.buffer.get(line_to_peek) {
                    None => {
                        // now at EOF
                        return n;
                    }
                    Some(line) => line,
                }
            }

            buf[n] = peeked_line.chars().nth(line_to_peek_off).unwrap();
        }

        // `buf.len()` characters read successfully
        buf.len()
    }
}
