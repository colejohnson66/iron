/* ============================================================================
 * File:   percent_encoding.rs
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
// Implements <https://url.spec.whatwg.org/#percent-encoded-bytes>
// TODO: Implement "UTF-8 percent-encode a code point codePoint using a percentEncodeSet"
// TODO: Implement "UTF-8 percent-encode a string input using a percentEncodeSet"

pub fn encode(byte: u8) -> String {
    static CHARS: &'static [u8] = b"0123456789abcdef";

    let mut buf = String::with_capacity(3);
    buf.push('%');
    buf.push(CHARS[(byte >> 4) as usize].into());
    buf.push(CHARS[(byte & 0xF) as usize].into());
    buf
}

pub fn decode(bytes: &[u8]) -> Vec<u8> {
    let mut output = vec![];

    // set to 2 to skip the `xx` in `%xx` after decoding
    let mut skips: u8 = 0;
    for n in 0..bytes.len() {
        if skips != 0 {
            skips -= 1;
            continue;
        }

        if bytes[n] != 0x25 {
            output.push(bytes[n]);
            continue;
        }

        // if next byte doesn't exist, return now
        if n + 1 >= bytes.len() {
            return output;
        }
        match bytes[n + 1] {
            0x30..=0x39 | 0x41..=0x46 | 0x61..=0x66 => {
                // if next next byte doesn't exist, return now
                if n + 2 >= bytes.len() {
                    // but first, add the last byte to the output
                    output.push(bytes[n + 1]);
                    return output;
                }
                match bytes[n + 2] {
                    0x30..=0x39 | 0x41..=0x46 | 0x61..=0x66 => {
                        skips = 2;
                        // decode bytes[n+1] and bytes[n+2]
                        let high_nibble = match bytes[n + 1] {
                            0x30..=0x39 => bytes[n + 1] - 0x30,
                            0x41..=0x46 => bytes[n + 1] - 0x41 + 10,
                            0x61..=0x66 => bytes[n + 1] - 0x61 + 10,
                            _ => unreachable!(),
                        };
                        let low_nibble = match bytes[n + 2] {
                            0x30..=0x39 => bytes[n + 2] - 0x30,
                            0x41..=0x46 => bytes[n + 2] - 0x41 + 10,
                            0x61..=0x66 => bytes[n + 2] - 0x61 + 10,
                            _ => unreachable!(),
                        };
                        output.push((high_nibble << 4) | low_nibble);
                        continue;
                    }
                    _ => {
                        // add the next byte because the next next one isn't valid
                        output.push(bytes[n + 1]);
                        continue;
                    }
                }
            }
            _ => continue,
        }
    }

    output
}

pub fn decode_str(string: &str) -> Vec<u8> {
    let bytes = string.as_bytes();
    decode(bytes)
}
