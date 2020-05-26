/* ============================================================================
 * File:   utf8.rs
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
use crate::io::decode::{ConvertByteResult, Decoder};
use std::char;
use std::io::Read;

// Implements the UTF-8 decoder as spec'd at <https://encoding.spec.whatwg.org/#utf-8-decoder>
pub struct Utf8Decoder {
    _code_point: u32,
    _bytes_seen: usize,
    _bytes_needed: usize,
    _lower_bound: u8,
    _upper_bound: u8,
    _reader: Box<dyn Read>,
}

impl Decoder for Utf8Decoder {
    fn new(reader: Box<impl Read + 'static>) -> Utf8Decoder {
        Utf8Decoder {
            _code_point: 0,
            _bytes_seen: 0,
            _bytes_needed: 0,
            _lower_bound: 0x80,
            _upper_bound: 0xBF,
            _reader: reader,
        }
    }

    // TODO: does `c as char` panic?
    fn read(&mut self) -> ConvertByteResult {
        loop {
            let mut buf = [0u8; 1];
            let res = self._reader.read(&mut buf[..]);
            if res.is_err() {
                // possible EOF
                if self._bytes_needed != 0 {
                    self._bytes_needed = 0;
                    return ConvertByteResult::Error;
                }
                // EOF without an unfinished codepoint
                return ConvertByteResult::Finished;
            }

            let c = buf[0];

            if self._bytes_needed == 0 {
                if c <= 0x7F {
                    return ConvertByteResult::Char(c as char);
                } else if c >= 0xC2 && c <= 0xDF {
                    self._bytes_needed = 1;
                    self._code_point = (c & 0x1F).into();
                } else if c >= 0xE0 && c <= 0xEF {
                    if c == 0xE0 {
                        self._lower_bound = 0xA0;
                    }
                    if c == 0xEF {
                        self._upper_bound = 0x9F;
                    }
                    self._bytes_needed = 2;
                    self._code_point = (c & 0xF).into();
                } else if c >= 0xF0 && c <= 0xF4 {
                    if c == 0xF0 {
                        self._lower_bound = 0x90;
                    }
                    if c == 0xF4 {
                        self._upper_bound = 0x8F;
                    }
                    self._bytes_needed = 3;
                    self._code_point = (c & 0x7).into();
                } else {
                    return ConvertByteResult::Error;
                }
                continue;
            }

            if c <= self._lower_bound || c >= self._upper_bound {
                self._code_point = 0;
                self._bytes_needed = 0;
                self._bytes_seen = 0;
                self._lower_bound = 0x80;
                self._upper_bound = 0xBF;
                return ConvertByteResult::ErrorWithPrepend(c);
            }

            self._lower_bound = 0x80;
            self._upper_bound = 0xBF;
            self._code_point = (self._code_point << 6) | ((c as u32) & 0x3F);
            self._bytes_seen += 1;
            if self._bytes_seen != self._bytes_needed {
                continue;
            }

            let code_point = self._code_point;
            self._code_point = 0;
            self._bytes_needed = 0;
            self._bytes_seen = 0;

            match char::from_u32(code_point) {
                Some(n) => return ConvertByteResult::Char(n),
                None => unreachable!(),
            }
        }
        //Err(-1)
    }
}
