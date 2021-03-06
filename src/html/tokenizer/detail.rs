/* ============================================================================
 * File:   detail.rs
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
// Implements <https://html.spec.whatwg.org/multipage/parsing.html#tokenization>
use std::char;

pub struct Attribute {
    pub name: String,
    pub value: String,
}

impl Attribute {
    pub fn new() -> Attribute {
        Attribute {
            name: "".into(),
            value: "".into(),
        }
    }
}

impl Clone for Attribute {
    fn clone(&self) -> Attribute {
        Attribute {
            name: self.name.clone(),
            value: self.value.clone(),
        }
    }
}

pub struct Comment {
    pub value: String,
}

impl Comment {
    pub fn new() -> Comment {
        Comment { value: "".into() }
    }
}

impl Clone for Comment {
    fn clone(&self) -> Comment {
        Comment {
            value: self.value.clone(),
        }
    }
}

pub struct Doctype {
    pub force_quirks: bool,
    pub name: Option<String>,
    pub public_id: Option<String>,
    pub system_id: Option<String>,
}

impl Doctype {
    pub fn new() -> Doctype {
        Doctype {
            force_quirks: false,
            name: None,
            public_id: None,
            system_id: None,
        }
    }

    pub fn append_to_name(&mut self, c: char) {
        if self.name.is_none() {
            self.name = Some("".into());
        }
        self.name.as_mut().unwrap().push(c);
    }

    pub fn append_to_public_id(&mut self, c: char) {
        if self.public_id.is_none() {
            self.public_id = Some("".into());
        }
        self.public_id.as_mut().unwrap().push(c);
    }

    pub fn append_to_system_id(&mut self, c: char) {
        if self.system_id.is_none() {
            self.system_id = Some("".into());
        }
        self.system_id.as_mut().unwrap().push(c);
    }
}

impl Clone for Doctype {
    fn clone(&self) -> Doctype {
        Doctype {
            force_quirks: self.force_quirks,
            name: self.name.clone(),
            public_id: self.public_id.clone(),
            system_id: self.system_id.clone(),
        }
    }
}

pub struct Tag {
    pub name: String,
    pub self_closing: bool,
    pub is_end_tag: bool,
    pub attributes: Option<Vec<Attribute>>,
}

impl Tag {
    pub fn new(end_tag: bool) -> Tag {
        Tag {
            name: "".into(),
            self_closing: false,
            is_end_tag: end_tag,
            attributes: None,
        }
    }

    pub fn create_attribute(&mut self) {
        // create the list if needed
        if self.attributes.is_none() {
            self.attributes = Some(vec![])
        }
        self.attributes.as_mut().unwrap().push(Attribute::new());
    }

    pub fn append_to_cur_attr_name(&mut self, c: char) {
        let last = self.attributes.as_mut().unwrap().last_mut();
        last.unwrap().name.push(c);
    }

    pub fn append_to_cur_attr_value(&mut self, c: char) {
        let last = self.attributes.as_mut().unwrap().last_mut();
        last.unwrap().value.push(c);
    }

    pub fn append_to_cur_attr_value_str(&mut self, chars: &str) {
        let last = self.attributes.as_mut().unwrap().last_mut();
        last.unwrap().value.push_str(chars);
    }

    pub fn set_self_closing_flag(&mut self) {
        self.self_closing = true;
    }
}

impl Clone for Tag {
    fn clone(&self) -> Tag {
        Tag {
            name: self.name.clone(),
            self_closing: self.self_closing.clone(),
            is_end_tag: self.is_end_tag,
            attributes: self.attributes.clone(),
        }
    }
}

#[derive(Clone)]
pub struct TokenMetadata {
    pub file_offset: usize,
    pub line: u32,
    pub line_offset: u32,
}

impl TokenMetadata {
    pub fn new(file_offset: usize, line: u32, line_offset: u32) -> TokenMetadata {
        TokenMetadata {
            file_offset,
            line,
            line_offset,
        }
    }
}

#[derive(Clone)]
pub enum Token {
    Attribute(Attribute),
    Character(char),
    Comment(Comment),
    Doctype(Doctype),
    Eof,
    Tag(Tag),
}

#[derive(Copy, Clone)]
pub enum State {
    Data,
    Rcdata,
    Rawtext,
    ScriptData,
    Plaintext,
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

    Doctype,
    BeforeDoctypeName,
    DoctypeName,
    AfterDoctypeName,
    AfterDoctypePublicKeyword,
    BeforeDoctypePublicIdentifier,
    DoctypePublicIdentifierDoubleQuoted,
    DoctypePublicIdentifierSingleQuoted,
    AfterDoctypePublicIdentifier,
    BetweenDoctypePublicAndSystemIdentifiers,
    AfterDoctypeSystemKeyword,
    BeforeDoctypeSystemIdentifier,
    DoctypeSystemIdentifierDoubleQuoted,
    DoctypeSystemIdentifierSingleQuoted,
    AfterDoctypeSystemIdentifier,
    BogusDoctype,

    CdataSection,
    CdataSectionBracket,
    CdataSectionEnd,

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
