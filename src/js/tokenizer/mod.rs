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
pub mod detail;

use crate::io::iter::LineOffsetIterator;
use crate::js::tokenizer::detail::*;
use crate::js::*;
use std::io::Cursor;

const ZWNJ: u32 = 0x200C;
const ZWJ: u32 = 0x200D;
const ZWNBSP: u32 = 0xFEFF;

pub struct JsTokenizer {
    js: LineOffsetIterator,
    //tokens_to_emit: VecDeque<>
}

impl JsTokenizer {
    pub fn new(js: &str) -> JsTokenizer {
        let mut cursor = Cursor::new(js);
        JsTokenizer {
            js: LineOffsetIterator::new(&mut cursor),
            //tokens_to_emit: VecDeque::new(),
        }
    }

    // In the impl blocks below, each fn MUST reset the state
    //   if nothing matches.
    // The return value is an Option<T> struct with two states:
    //  - None => no match
    //  - Some(T) => a match; T may or may not contain data
}

// implementation of <https://tc39.es/ecma262/#sec-comments>
impl JsTokenizer {
    fn comment(&mut self) -> Option<()> {
        // Comment ::
        //     MultiLineComment
        //     SingleLineComment

        let state = self.js.state();

        // match MultiLineComment
        match self.multi_line_comment() {
            Some(_) => return Some(()),
            None => (),
        }

        // match SingleLineComment
        match self.single_line_comment() {
            Some(_) => return Some(()),
            None => (),
        }

        self.js.set_state(state);
        None
    }

    fn multi_line_comment(&mut self) -> Option<()> {
        // MultiLineComment ::
        //     "/*" opt[MultiLineCommentChars] "*/"

        let state = self.js.state();

        // match "/*"
        let mut peek: [char; 2] = ['\0'; 2];
        if self.js.peek_multiple(&mut peek) == 2 && peek.iter().collect::<String>() == "/*" {
            self.js.consume_multiple(2);

            // match opt[MultiLineCommentChars]
            self.multi_line_comment_chars();

            // match "*/"
            if self.js.peek_multiple(&mut peek) == 2 && peek.iter().collect::<String>() == "*/" {
                self.js.consume_multiple(2);
                return Some(());
            }
        }

        self.js.set_state(state);
        None
    }

    fn multi_line_comment_chars(&mut self) -> Option<()> {
        // MultiLineCommentChars ::
        //     MultiLineNotAsteriskChar opt[MultiLineCommentChars]
        //     "*" opt[PostAsteriskCommentChars]

        let state = self.js.state();

        // match MultiLineNotAsteriskChar
        match self.multi_line_not_asterisk_char() {
            Some(_) => {
                // match opt[MultiLineCommentChars]
                self.multi_line_comment_chars();
                return Some(());
            }
            None => (),
        }

        self.js.set_state(state);

        // match "*"
        match self.js.peek() {
            Some('*') => {
                self.js.consume();
                // match opt[PostAsteriskCommentChar]
                self.post_asterisk_comment_chars();
                return Some(());
            }
            _ => (),
        }

        self.js.set_state(state);
        None
    }

    fn post_asterisk_comment_chars(&mut self) -> Option<()> {
        // PostAsteriskCommentChars ::
        //     MultiLineNotForwardSlashOrAsteriskChar opt[MultiLineCommentChars]
        //     "*" opt[PostAsteriskCommentChars]

        let state = self.js.state();

        // match MultiLineNotForwardSlashOrAsteriskChar
        match self.multi_line_not_forward_slash_or_asterisk_char() {
            Some(_) => {
                // match MultiLineCommentChars
                self.multi_line_comment_chars();
                return Some(());
            }
            _ => (),
        }

        self.js.set_state(state);

        // match "*"
        match self.js.peek() {
            Some('*') => {
                self.js.consume();
                // match opt[PostAsteriskCommentChar]
                self.post_asterisk_comment_chars();
                return Some(());
            }
            _ => (),
        }

        self.js.set_state(state);
        None
    }

    fn multi_line_not_asterisk_char(&mut self) -> Option<()> {
        // MultiLineNotAsteriskChar ::
        //     SourceChar[excl['*']]

        match self.js.peek() {
            Some('*') => None,
            Some(_) => {
                self.js.consume();
                Some(())
            }
            None => None,
        }
    }

    fn multi_line_not_forward_slash_or_asterisk_char(&mut self) -> Option<()> {
        // MultiLineNotForwardSlashOrAsteriskChar ::
        //     SourceChar[excl['/', '*']]

        match self.js.peek() {
            Some('/') | Some('*') => None,
            Some(_) => {
                self.js.consume();
                Some(())
            }
            None => None,
        }
    }

    fn single_line_comment(&mut self) -> Option<()> {
        // SingleLineComment ::
        //     "//" opt[SingleLineCommentChars]

        // match "/*"
        let mut peek: [char; 2] = ['\0'; 2];
        if self.js.peek_multiple(&mut peek) == 2 && peek.iter().collect::<String>() == "//" {
            self.js.consume_multiple(2);

            // match opt[SingleLineCommentChars]
            self.single_line_comment_chars();

            return Some(());
        };

        None
    }

    fn single_line_comment_chars(&mut self) -> Option<()> {
        // SingleLineCommentChars ::
        //     SingleLineCommentChar opt[SingleLineCommentChars]

        let state = self.js.state();

        match self.single_line_comment_char() {
            Some(_) => {
                // match opt[SingleLineCommentChars]
                self.single_line_comment_chars();
                return Some(());
            }
            None => (),
        }

        self.js.set_state(state);
        None
    }

    fn single_line_comment_char(&mut self) -> Option<()> {
        // SingleLineCommentChar ::
        //     SourceChar[excl[LineTerminator]]

        match self.js.peek() {
            Some(c) if line_terminator(c as u32) => None,
            Some(_) => {
                self.js.consume();
                Some(())
            }
            None => None,
        }
    }
}

// implementation of <https://tc39.es/ecma262/#sec-tokens>
impl JsTokenizer {
    fn common_token(&mut self) -> Option<CommonToken> {
        unimplemented!();
    }
}

// implementation of <https://tc39.es/ecma262/#sec-names-and-keywords>
impl JsTokenizer {
    fn identifier_name(&mut self) -> Option<()> {
        // IdentifierName ::
        //     IdentifierStart
        //     IdentifierName IdentifierPart

        let state = self.js.state();

        // match IdentifierStart
        match self.identifier_start() {
            Some(_) => return Some(()),
            _ => (),
        }

        // match IdentifierName
        match self.identifier_name() {
            Some(_) => {
                // match IdentifierPart
                match self.identifier_part() {
                    Some(_) => return Some(()),
                    _ => (),
                }
            }
            _ => (),
        }

        self.js.set_state(state);
        None
    }

    fn identifier_start(&mut self) -> Option<()> {
        // IdentifierStart ::
        //     UnicodeIDStart
        //     "$"
        //     "_"
        //     "\" UnicodeEscapeSequence

        let state = self.js.state();

        // match UnicodeIDStart
        match self.unicode_id_start() {
            Some(_) => return Some(()),
            None => (),
        }

        // match "$" or "_"
        match self.js.peek() {
            Some('$') | Some('_') => return Some(()),
            Some(_) | None => (),
        }

        // match "\"
        match self.js.peek() {
            Some('\\') => {
                // match UnicodeEscapeSequence
                unimplemented!();
            }
            Some(_) | None => (),
        }

        self.js.set_state(state);
        None
    }

    fn identifier_part(&mut self) -> Option<()> {
        // IdentifierPart ::
        //     UnicodeIDContinue
        //     "$"
        //     "\" UnicodeEscapeSequence
        //     <ZWNJ>
        //     <ZWJ>

        // match UnicodeIDContinue
        match self.unicode_id_continue() {
            Some(_) => return Some(()),
            _ => (),
        }

        // match "$"
        match self.js.peek() {
            Some('$') => {
                self.js.consume();
                return Some(());
            }
            _ => (),
        }

        // match "\"
        match self.js.peek() {
            Some('\\') => {
                self.js.consume();
                // match UnicodeEscapeSequence
                unimplemented!();
            }
            _ => (),
        }

        // match <ZWNJ> or <ZWJ>
        match self.js.peek() {
            Some(c) if c as u32 == ZWNJ || c as u32 == ZWJ => {
                self.js.consume();
                return Some(());
            }
            _ => (),
        }

        None
    }

    fn unicode_id_start(&mut self) -> Option<()> {
        // UnicodeIDStart ::
        //     == any Unicode code point with the Unicode property "ID_Start"
        unimplemented!();
    }

    fn unicode_id_continue(&mut self) -> Option<()> {
        // UnicodeIDContinue ::
        //     == any Unicode code point with the Unicode property "ID_Continue"
        unimplemented!();
    }
}
