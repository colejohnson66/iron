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
use crate::html::parser::quirks;

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
