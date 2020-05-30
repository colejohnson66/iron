/* ============================================================================
 * File:   code_point.rs
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
// Implements <https://infra.spec.whatwg.org/> section 4.5
// TODO: implement "convert a string into a scalar value string"

pub fn surrogate(code_point: u32) -> bool {
    match code_point {
        0xD800..=0xDFFF => true,
        _ => false,
    }
}

pub fn scalar_value(code_point: u32) -> bool {
    !surrogate(code_point)
}

pub fn noncharacter(code_point: u32) -> bool {
    match code_point {
        0xFDD0..=0xFDEF => true,
        0xFFFE..=0xFFFF => true,
        0x1FFFE..=0x1FFFF => true,
        0x2FFFE..=0x2FFFF => true,
        0x3FFFE..=0x3FFFF => true,
        0x4FFFE..=0x4FFFF => true,
        0x5FFFE..=0x5FFFF => true,
        0x6FFFE..=0x6FFFF => true,
        0x7FFFE..=0x7FFFF => true,
        0x8FFFE..=0x8FFFF => true,
        0x9FFFE..=0x9FFFF => true,
        0xAFFFE..=0xAFFFF => true,
        0xBFFFE..=0xBFFFF => true,
        0xCFFFE..=0xCFFFF => true,
        0xDFFFE..=0xDFFFF => true,
        0xEFFFE..=0xEFFFF => true,
        0xFFFFE..=0xFFFFF => true,
        0x10FFFE..=0x10FFFF => true,
        _ => false,
    }
}

pub fn ascii_code_point(code_point: u32) -> bool {
    match code_point {
        0x0..=0x7F => true,
        _ => false,
    }
}

pub fn ascii_tab_or_newline(code_point: u32) -> bool {
    match code_point {
        0x9 | 0xA | 0xD => true,
        _ => false,
    }
}

pub fn ascii_whitespace(code_point: u32) -> bool {
    match code_point {
        0x9 | 0xA | 0xC | 0xD | 0x20 => true,
        _ => false,
    }
}

pub fn c0_control(code_point: u32) -> bool {
    match code_point {
        0x0..=0x1F => true,
        _ => false,
    }
}

pub fn c0_control_or_space(code_point: u32) -> bool {
    c0_control(code_point) || code_point == 0x20
}

pub fn control(code_point: u32) -> bool {
    if c0_control(code_point) {
        return true;
    }
    match code_point {
        0x7F..=0x9F => true,
        _ => false,
    }
}

pub fn ascii_digit(code_point: u32) -> bool {
    match code_point {
        0x30..=0x39 => true,
        _ => false,
    }
}

pub fn ascii_upper_hex_digit(code_point: u32) -> bool {
    match code_point {
        0x41..=0x46 => true,
        _ => false,
    }
}

pub fn ascii_lower_hex_digit(code_point: u32) -> bool {
    match code_point {
        0x61..=0x66 => true,
        _ => false,
    }
}

pub fn ascii_hex_digit(code_point: u32) -> bool {
    match code_point {
        0x41..=0x46 => true,
        0x61..=0x66 => true,
        _ => false,
    }
}

pub fn ascii_upper_alpha(code_point: u32) -> bool {
    match code_point {
        0x41..=0x5A => true,
        _ => false,
    }
}

pub fn ascii_lower_alpha(code_point: u32) -> bool {
    match code_point {
        0x61..=0x7A => true,
        _ => false,
    }
}

pub fn ascii_alpha(code_point: u32) -> bool {
    match code_point {
        0x41..=0x5A => true,
        0x61..=0x7A => true,
        _ => false,
    }
}

pub fn ascii_alphanumeric(code_point: u32) -> bool {
    ascii_digit(code_point) || ascii_alpha(code_point)
}

// TODO: implement "convert a string into a scalar value string"

pub fn isomorphic_encode(string: &str) -> Option<Vec<u8>> {
    let mut output = Vec::with_capacity(string.len());
    loop {
        match string.chars().next() {
            Some(c) if (c as u32) > 0xFF => return None,
            Some(c) => output.push(c as u8),
            None => break,
        }
    }

    Some(output)
}
