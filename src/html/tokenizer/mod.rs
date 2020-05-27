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
use crate::html::parser::{quirks, ParseHtmlError};
use std::char;

struct LineCountingChars {
    contents: Vec<char>,
    pos: usize,
    line: u32,
    line_pos: u32,
}

impl LineCountingChars {
    fn new(string: &str) -> LineCountingChars {
        LineCountingChars {
            contents: string.chars().collect(),
            pos: 0,
            line: 0,
            line_pos: 0,
        }
    }

    fn seek(&mut self, new_pos: usize) {
        self.pos = 0;
        self.line = 0;
        self.line_pos = 0;
        // TODO: inefficient
        for _ in 0..new_pos {
            self.read();
        }
    }

    fn read(&mut self) -> Option<char> {
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

    fn read_multiple(&mut self, buf: &mut [char]) -> usize {
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

    fn backtrack(&mut self) {
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

    fn backtrack_multiple(&mut self, count: usize) {
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

pub struct Attribute {
    name: String,
    value: String,
}

pub enum Token {
    Attribute(Attribute),
    Character(char),
    Comment(String),
    Doctype {
        quirks: quirks::QuirksMode,
        name: String,
        public_id: String,
        system_id: String,
    },
    Eof,
    Tag {
        name: String,
        self_closing: bool,
        is_end_tag: bool,
        attributes: Vec<Attribute>,
    },
}

pub enum State {
    Data,
    Rcdata,
    Rawtext,
    ScriptData,
    PLAINTEXT,
    TagOpen,
    EndTagOpen,
    TagName,

    RcdataLessThanSign,
    RcdataEndTagOpen,
    RcdataEndTagName,

    RawtextLessThanSign,
    RawtextEndTagOpen,
    RawtextEndTagName,

    ScriptDataLessThanSign,
    ScriptDataEndTagOpen,
    ScriptDataEndTagName,
    ScriptDataEscapeStart,
    ScriptDataEscapeStartDash,
    ScriptDataEscaped,
    ScriptDataEscapedDash,
    ScriptDataEscapedDashDash,
    ScriptDataEscapedLessThanSign,
    ScriptDataEscapedEndTagOpen,
    ScriptDataEscapedEndTagName,

    ScriptDataDoubleEscapeStart,
    ScriptDataDoubleEscaped,
    ScriptDataDoubleEscapedDash,
    ScriptDataDoubleEscapedDashDash,
    ScriptDataDoubleEscapedLessThanSign,
    ScriptDataDoubleEscapeEnd,

    BeforeAttributeName,
    AttributeName,
    AfterAttributeName,
    BeforeAttributeValue,
    AttributeValueDoubleQuoted,
    AttributeValueSingleQuoted,
    AttributeValueUnquoted,
    AfterAttributeValueQuoted,

    SelfClosingStartTag,
    BogusComment,
    MarkupDeclarationOpen,

    CommentStart,
    CommentStartDash,
    Comment,
    CommentLessThanSign,
    CommentLessThanSignBang,
    CommentLessThanSignBangDash,
    CommentLessThanSignBangDashDash,
    CommentEndDash,
    CommentEnd,
    CommentEndBang,

    DOCTYPE,
    BeforeDOCTYPEName,
    DOCTYPEName,
    AfterDOCTYPEName,
    AfterDOCTYPEPublicKeyword,
    BeforeDOCTYPEPublicIdentifier,
    DOCTYPEPublicIdentifierDoubleQuoted,
    DOCTYPEPublicIdentifierSingleQuoted,
    AfterDOCTYPEPublicIdentifier,
    BetweenDOCTYPEPublicAndSystemIdentifiers,
    AfterDOCTYPESystemKeyword,
    BeforeDOCTYPESystemIdentifier,
    DOCTYPESystemIdentifierDoubleQuoted,
    DOCTYPESystemIdentifierSingleQuoted,
    AfterDOCTYPESystemIdentifier,
    BogusDOCTYPE,

    CDATASection,
    CDATASectionBracket,
    CDATASectionEnd,

    CharacterReference,
    NamedCharacterReference,
    AmbiguousAmpersand,
    NumericCharacterReference,
    HexadecimalCharacterReferenceStart,
    DecimalCharacterReferenceStart,
    HexadecimalCharacterReference,
    DecimalCharacterReference,
    NumericCharacterReferenceEnd,
}

pub struct HtmlTokenizer {
    html: LineCountingChars,
    return_state: Option<State>,
    last_emitted_tag: Option<Token>,

    comment: Option<Token>,
    tag: Option<Token>,
    attr: Option<Token>,
    doctype: Option<Token>,

    temp_buf: String,
    char_ref_code: u32,
}

impl HtmlTokenizer {
    fn new(html: &str) -> HtmlTokenizer {
        HtmlTokenizer {
            html: LineCountingChars::new(html),
            return_state: None,
            last_emitted_tag: None,
            comment: None,
            tag: None,
            attr: None,
            doctype: None,
            temp_buf: "".into(),
            char_ref_code: 0,
        }
    }

    fn char_token(c: char) -> Token {
        Token::Character(c)
    }
    fn null_char_token() -> Token {
        Token::Character('\0')
    }
    fn replacement_char_token() -> Token {
        Token::Character(char::REPLACEMENT_CHARACTER)
    }
    fn eof_token() -> Token {
        Token::Eof
    }

    fn lowercase_char_from_ascii_upper(c: char) -> char {
        // TODO: assert `c` is ascii upper
        char::from_u32((c as u32) + 0x20).unwrap()
    }

    fn error(_err: ParseHtmlError) {
        // TODO: call a callback
    }

    fn in_attr_state(return_state: State) -> bool {
        match return_state {
            State::AttributeValueDoubleQuoted => true,
            State::AttributeValueSingleQuoted => true,
            State::AttributeValueUnquoted => true,
            _ => false,
        }
    }
}

impl Iterator for HtmlTokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        // TODO: read a char and return Some(tokenize(c.unwrap()))
        None
    }
}
