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
use crate::io::LineCountingChars;

pub struct JsTokenizer {
    js: LineCountingChars,
    //tokens_to_emit: VecDeque<>
}

impl JsTokenizer {
    pub fn new(js: &str) -> JsTokenizer {
        JsTokenizer {
            js: LineCountingChars::new(js),
            //tokens_to_emit: VecDeque::new(),
        }
    }
}

// implementation of <https://tc39.es/ecma262/#sec-comments>
impl JsTokenizer {
    fn comment(&mut self) -> Option<()> {
        // Comment ::
        //     MultiLineComment
        //     SingleLineComment
        match self.multi_line_comment() {
            Some(_) => return Some(()),
            None => (),
        }
        match self.single_line_comment() {
            Some(_) => return Some(()),
            None => (),
        }
        None
    }

    fn multi_line_comment(&mut self) -> Option<()> {
        unimplemented!();
    }

    fn multi_line_comment_chars(&mut self) -> Option<()> {
        unimplemented!();
    }

    fn post_asterisk_comment_chars(&mut self) -> Option<()> {
        unimplemented!();
    }

    fn multi_line_asterisk_char(&mut self) -> Option<()> {
        unimplemented!();
    }

    fn multi_line_not_forward_slash_or_asterisk_char(&mut self) -> Option<()> {
        unimplemented!();
    }

    fn single_line_comment(&mut self) -> Option<()> {
        unimplemented!();
    }

    fn single_line_comment_chars(&mut self) -> Option<()> {
        unimplemented!();
    }

    fn single_line_comment_char(&mut self) -> Option<()> {
        unimplemented!();
    }
}
