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
// Implements <https://html.spec.whatwg.org/multipage/parsing.html#tokenization>
mod detail;

use crate::html::parser::{quirks, ParseHtmlError};
use crate::html::tokenizer::detail::*;
use crate::infra::*;
use std::char;
use std::collections::VecDeque;

pub struct HtmlTokenizer {
    html: LineCountingChars,
    state: State,

    return_state: Option<State>,
    last_emitted_tag: Option<Tag>,

    comment: Option<Comment>,
    tag: Option<Tag>,
    attr: Option<Attribute>,
    doctype: Option<Doctype>,

    temp_buf: String,
    char_ref_code: u32,

    tokens_to_emit: VecDeque<Token>,
}

impl HtmlTokenizer {
    fn new(html: &str) -> HtmlTokenizer {
        HtmlTokenizer {
            html: LineCountingChars::new(html),
            state: State::Data,
            return_state: None,
            last_emitted_tag: None,
            comment: None,
            tag: None,
            attr: None,
            doctype: None,
            temp_buf: "".into(),
            char_ref_code: 0,
            tokens_to_emit: VecDeque::new(),
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

    fn error(&mut self, _err: ParseHtmlError) {
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

    fn end_tag_appropriate(&mut self) -> bool {
        // NOTE: will intentionally panic if `tag` is `None`
        match self.last_emitted_tag.as_ref() {
            None => false,
            Some(last_tag) => last_tag.name == self.tag.as_ref().unwrap().name,
        }
    }

    fn temp_buf_to_tokens(&mut self) -> Vec<Token> {
        let mut buf: Vec<Token> = Vec::with_capacity(32);
        for c in self.temp_buf.chars() {
            buf.push(HtmlTokenizer::char_token(c))
        }
        buf
    }

    // actual tokenization takes place below
}

impl Iterator for HtmlTokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        // spit out a stored token from the vector if it exists
        match self.tokens_to_emit.pop_front() {
            Some(tok) => return Some(tok),
            None => (),
        }

        loop {
            let c = self.html.read();
            let tokens = match self.state {
                State::Data => self.data(c),
                State::Rcdata => self.rcdata(c),
                State::Rawtext => self.rawtext(c),
                State::ScriptData => self.script_data(c),
                State::Plaintext => self.plaintext(c),
                State::TagOpen => self.tag_open(c),
                State::EndTagOpen => self.end_tag_open(c),
                State::TagName => self.tag_name(c),
                State::RcdataLessThanSign => self.rcdata_less_than_sign(c),
                State::RcdataEndTagOpen => self.rcdata_end_tag_open(c),
                State::RcdataEndTagName => self.rcdata_end_tag_name(c),
                State::RawtextLessThanSign => self.rawtext_less_than_sign(c),
                State::RawtextEndTagOpen => self.rawtext_end_tag_open(c),
                State::RawtextEndTagName => self.rawtext_end_tag_name(c),
                State::ScriptDataLessThanSign => self.script_data_less_than_sign(c),
                State::ScriptDataEndTagOpen => self.script_data_end_tag_open(c),
                State::ScriptDataEndTagName => self.script_data_end_tag_name(c),
                State::ScriptDataEscapeStart => self.script_data_escape_start(c),
                State::ScriptDataEscapeStartDash => self.script_data_escape_start_dash(c),
                State::ScriptDataEscaped => self.script_data_escaped(c),
                State::ScriptDataEscapedDash => self.script_data_escaped_dash(c),
                State::ScriptDataEscapedDashDash => self.script_data_escaped_dash_dash(c),
                State::ScriptDataEscapedLessThanSign => self.script_data_escaped_less_than_sign(c),
                State::ScriptDataEscapedEndTagOpen => self.script_data_end_tag_open(c),
                State::ScriptDataEscapedEndTagName => self.script_data_end_tag_name(c),
                State::ScriptDataDoubleEscapeStart => self.script_data_double_escape_start(c),
                State::ScriptDataDoubleEscaped => self.script_data_double_escaped(c),
                State::ScriptDataDoubleEscapedDash => self.script_data_double_escaped_dash(c),
                State::ScriptDataDoubleEscapedDashDash => {
                    self.script_data_double_escaped_dash_dash(c)
                }
                State::ScriptDataDoubleEscapedLessThanSign => {
                    self.script_data_double_escaped_less_than_sign(c)
                }
                State::ScriptDataDoubleEscapeEnd => self.script_data_double_escape_end(c),
                State::BeforeAttributeName => self.before_attribute_name(c),
                State::AttributeName => self.attribute_name(c),
                State::AfterAttributeName => self.after_attribute_name(c),
                State::BeforeAttributeValue => self.before_attribute_value(c),
                State::AttributeValueDoubleQuoted => self.attribute_value_double_quoted(c),
                State::AttributeValueSingleQuoted => self.attribute_value_single_quoted(c),
                State::AttributeValueUnquoted => self.attribute_value_unquoted(c),
                State::AfterAttributeValueQuoted => self.after_attribute_value_quoted(c),
                State::SelfClosingStartTag => self.self_closing_start_tag(c),
                State::BogusComment => self.bogus_comment(c),
                State::MarkupDeclarationOpen => self.markup_declaration_open(c),
                State::CommentStart => self.comment_start(c),
                State::CommentStartDash => self.comment_start_dash(c),
                State::Comment => self.comment(c),
                State::CommentLessThanSign => self.comment_less_than_sign(c),
                State::CommentLessThanSignBang => self.comment_less_than_sign_bang(c),
                State::CommentLessThanSignBangDash => self.comment_less_than_sign_bang_dash(c),
                State::CommentLessThanSignBangDashDash => {
                    self.comment_less_than_sign_bang_dash_dash(c)
                }
                State::CommentEndDash => self.comment_end_dash(c),
                State::CommentEnd => self.comment_end(c),
                State::CommentEndBang => self.comment_end_bang(c),
                State::Doctype => self.doctype(c),
                State::BeforeDoctypeName => self.before_doctype_name(c),
                State::DoctypeName => self.doctype_name(c),
                State::AfterDoctypeName => self.after_doctype_name(c),
                State::AfterDoctypePublicKeyword => self.after_doctype_public_keyword(c),
                State::BeforeDoctypePublicIdentifier => self.before_doctype_public_identifier(c),
                State::DoctypePublicIdentifierDoubleQuoted => {
                    self.doctype_public_identifier_double_quoted(c)
                }
                State::DoctypePublicIdentifierSingleQuoted => {
                    self.doctype_public_identifier_single_quoted(c)
                }
                State::AfterDoctypePublicIdentifier => self.after_doctype_public_identifier(c),
                State::BetweenDoctypePublicAndSystemIdentifiers => {
                    self.between_doctype_public_and_system_identifiers(c)
                }
                State::AfterDoctypeSystemKeyword => self.after_doctype_system_keyword(c),
                State::BeforeDoctypeSystemIdentifier => self.before_doctype_system_identifier(c),
                State::DoctypeSystemIdentifierDoubleQuoted => {
                    self.doctype_system_identifier_double_quoted(c)
                }
                State::DoctypeSystemIdentifierSingleQuoted => {
                    self.doctype_system_identifier_single_quoted(c)
                }
                State::AfterDoctypeSystemIdentifier => self.after_doctype_system_identifier(c),
                State::BogusDoctype => self.bogus_doctype(c),
                State::CdataSection => self.cdata_section(c),
                State::CdataSectionBracket => self.cdata_section_bracket(c),
                State::CdataSectionEnd => self.cdata_section_end(c),
                State::CharacterReference => self.character_reference(c),
                State::NamedCharacterReference => self.named_character_reference(c),
                State::AmbiguousAmpersand => self.ambiguous_ampersand(c),
                State::NumericCharacterReference => self.numeric_character_reference(c),
                State::HexadecimalCharacterReferenceStart => {
                    self.hexadecimal_character_reference_start(c)
                }
                State::DecimalCharacterReferenceStart => self.decimal_character_reference_start(c),
                State::HexadecimalCharacterReference => self.hexadecimal_character_reference(c),
                State::DecimalCharacterReference => self.decimal_character_reference(c),
                State::NumericCharacterReferenceEnd => self.numeric_character_reference_end(c),
            };

            match tokens {
                Some(tokens) => {
                    if tokens.len() == 1 {
                        // if there's only one token, don't store it; just return it
                        return Some(tokens.get(0).unwrap().clone());
                    } else {
                        self.tokens_to_emit = VecDeque::from(tokens);
                        return Some(self.tokens_to_emit.pop_front().unwrap().clone());
                    }
                }
                None => {
                    // No tokens returned; consume next character and try again
                }
            }
        }
    }
}

// implementation
impl HtmlTokenizer {
    fn data(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        match c {
            Some('&') => {
                self.return_state = Some(State::Data);
                self.state = State::CharacterReference;
                None
            }
            Some('<') => {
                self.state = State::TagOpen;
                None
            }
            Some('\0') => {
                self.error(ParseHtmlError::UnexpectedNullCharacter);
                Some(vec![HtmlTokenizer::char_token('\0')])
            }
            None => Some(vec![HtmlTokenizer::eof_token()]),
            Some(c) => Some(vec![HtmlTokenizer::char_token(c)]),
        }
    }

    fn rcdata(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        match c {
            Some('&') => {
                self.return_state = Some(State::Rcdata);
                self.state = State::CharacterReference;
                None
            }
            Some('<') => {
                self.state = State::RcdataLessThanSign;
                None
            }
            Some('\0') => {
                self.error(ParseHtmlError::UnexpectedNullCharacter);
                Some(vec![HtmlTokenizer::replacement_char_token()])
            }
            None => Some(vec![HtmlTokenizer::eof_token()]),
            Some(c) => Some(vec![HtmlTokenizer::char_token(c)]),
        }
    }

    fn rawtext(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        match c {
            Some('<') => {
                self.state = State::RawtextLessThanSign;
                None
            }
            Some('\0') => {
                self.error(ParseHtmlError::UnexpectedNullCharacter);
                Some(vec![HtmlTokenizer::replacement_char_token()])
            }
            None => Some(vec![HtmlTokenizer::eof_token()]),
            Some(c) => Some(vec![HtmlTokenizer::char_token(c)]),
        }
    }

    fn script_data(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        match c {
            Some('<') => {
                self.state = State::ScriptDataLessThanSign;
                None
            }
            Some('\0') => {
                self.error(ParseHtmlError::UnexpectedNullCharacter);
                Some(vec![HtmlTokenizer::replacement_char_token()])
            }
            None => Some(vec![HtmlTokenizer::eof_token()]),
            Some(c) => Some(vec![HtmlTokenizer::char_token(c)]),
        }
    }

    fn plaintext(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        match c {
            Some('\0') => {
                self.error(ParseHtmlError::UnexpectedNullCharacter);
                Some(vec![HtmlTokenizer::replacement_char_token()])
            }
            None => Some(vec![HtmlTokenizer::eof_token()]),
            Some(c) => Some(vec![HtmlTokenizer::char_token(c)]),
        }
    }

    fn tag_open(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        match c {
            Some('!') => {
                self.state = State::MarkupDeclarationOpen;
                None
            }
            Some('/') => {
                self.state = State::EndTagOpen;
                None
            }
            Some(c) if is_ascii_alpha(c as u32) => {
                self.tag = Some(Tag::new(false));
                self.tag_name(Some(c))
            }
            Some('?') => {
                self.error(ParseHtmlError::UnexpectedQuestionMarkInsteadOfTagName);
                self.comment = Some(Comment::new());
                self.bogus_comment(c)
            }
            None => Some(vec![
                HtmlTokenizer::char_token('<'),
                HtmlTokenizer::eof_token(),
            ]),
            Some(c) => {
                self.error(ParseHtmlError::InvalidFirstCharacterOfTagName);
                let mut tok = vec![HtmlTokenizer::char_token('<')];
                let mut reconsumed = self.data(Some(c)).unwrap_or_default();
                tok.append(&mut reconsumed);
                Some(tok)
            }
        }
    }

    fn end_tag_open(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        match c {
            Some(c) if is_ascii_alpha(c as u32) => {
                self.tag = Some(Tag::new(true));
                self.tag_name(Some(c))
            }
            Some('<') => {
                self.error(ParseHtmlError::MissingEndTagName);
                self.state = State::Data;
                None
            }
            None => {
                self.error(ParseHtmlError::EofBeforeTagName);
                Some(vec![
                    HtmlTokenizer::char_token('<'),
                    HtmlTokenizer::char_token('/'),
                    HtmlTokenizer::eof_token(),
                ])
            }
            Some(c) => {
                self.error(ParseHtmlError::InvalidFirstCharacterOfTagName);
                self.comment = Some(Comment::new());
                self.bogus_comment(Some(c))
            }
        }
    }

    fn tag_name(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        match c {
            Some(c) if is_ascii_whitespace(c as u32) => {
                self.state = State::BeforeAttributeName;
                None
            }
            Some('/') => {
                self.state = State::SelfClosingStartTag;
                None
            }
            Some('>') => {
                self.state = State::Data;
                match self.tag.as_ref() {
                    Some(tag) => Some(vec![Token::Tag(tag.clone())]),
                    None => None,
                }
            }
            Some(c) if is_ascii_upper_alpha(c as u32) => {
                let c = HtmlTokenizer::lowercase_char_from_ascii_upper(c);
                self.tag.as_mut().unwrap().name.push(c);
                None
            }
            Some('\0') => {
                self.error(ParseHtmlError::UnexpectedNullCharacter);
                self.tag.as_mut().unwrap().name.push('\u{FFFD}');
                None
            }
            None => {
                self.error(ParseHtmlError::EofInTag);
                Some(vec![HtmlTokenizer::eof_token()])
            }
            Some(c) => {
                self.tag.as_mut().unwrap().name.push(c);
                None
            }
        }
    }

    fn rcdata_less_than_sign(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        match c {
            Some('/') => {
                self.temp_buf = "".into();
                self.state = State::RcdataEndTagOpen;
                None
            }
            _ => {
                let mut tok = vec![HtmlTokenizer::char_token('<')];
                let mut reconsumed = self.rcdata(c).unwrap_or_default();
                tok.append(&mut reconsumed);
                Some(tok)
            }
        }
    }

    fn rcdata_end_tag_open(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        match c {
            Some(c) if is_ascii_alpha(c as u32) => {
                self.tag = Some(Tag::new(true));
                self.rcdata_end_tag_name(Some(c))
            }
            _ => {
                let mut tok = vec![
                    HtmlTokenizer::char_token('<'),
                    HtmlTokenizer::char_token('/'),
                ];
                let mut reconsumed = self.rcdata(c).unwrap_or_default();
                tok.append(&mut reconsumed);
                Some(tok)
            }
        }
    }

    fn rcdata_end_tag_name(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        match c {
            Some(c) if is_ascii_whitespace(c as u32) => {
                if self.end_tag_appropriate() {
                    self.state = State::BeforeAttributeName;
                    return None;
                }
            }
            Some('/') => {
                if self.end_tag_appropriate() {
                    self.state = State::SelfClosingStartTag;
                    return None;
                }
            }
            Some('>') => {
                if self.end_tag_appropriate() {
                    self.state = State::Data;
                    let tag = self.tag.clone().unwrap();
                    self.tag = None;
                    return Some(vec![Token::Tag(tag)]);
                }
            }
            Some(c) if is_ascii_upper_alpha(c as u32) => {
                let lower_c = HtmlTokenizer::lowercase_char_from_ascii_upper(c);
                self.tag.as_mut().unwrap().name.push(lower_c);
                self.temp_buf.push(c);
                return None;
            }
            Some(c) if is_ascii_lower_alpha(c as u32) => {
                self.tag.as_mut().unwrap().name.push(c);
                self.temp_buf.push(c);
                return None;
            }
            _ => (),
        }
        let mut tok = vec![
            HtmlTokenizer::char_token('<'),
            HtmlTokenizer::char_token('/'),
        ];
        tok.append(&mut self.temp_buf_to_tokens());
        let mut reconsumed = self.rcdata(c).unwrap_or_default();
        tok.append(&mut reconsumed);
        Some(tok)
    }

    fn rawtext_less_than_sign(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        match c {
            Some('/') => {
                self.temp_buf = "".into();
                self.state = State::RawtextEndTagOpen;
                None
            }
            _ => {
                let mut tok = vec![HtmlTokenizer::char_token('<')];
                let mut reconsumed = self.rawtext(c).unwrap_or_default();
                tok.append(&mut reconsumed);
                Some(tok)
            }
        }
    }

    fn rawtext_end_tag_open(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        match c {
            Some(c) if is_ascii_alpha(c as u32) => {
                self.tag = Some(Tag::new(true));
                self.rawtext_end_tag_name(Some(c))
            }
            _ => {
                let mut tok = vec![
                    HtmlTokenizer::char_token('<'),
                    HtmlTokenizer::char_token('/'),
                ];
                let mut reconsumed = self.rawtext(c).unwrap_or_default();
                tok.append(&mut reconsumed);
                Some(tok)
            }
        }
    }

    fn rawtext_end_tag_name(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        match c {
            Some(c) if is_ascii_whitespace(c as u32) => {
                if self.end_tag_appropriate() {
                    self.state = State::BeforeAttributeName;
                    return None;
                }
            }
            Some('/') => {
                if self.end_tag_appropriate() {
                    self.state = State::SelfClosingStartTag;
                    return None;
                }
            }
            Some('>') => {
                if self.end_tag_appropriate() {
                    self.state = State::Data;
                    return None;
                }
            }
            Some(c) if is_ascii_upper_alpha(c as u32) => {
                let lower_c = HtmlTokenizer::lowercase_char_from_ascii_upper(c);
                self.tag.as_mut().unwrap().name.push(lower_c);
                self.temp_buf.push(c);
                return None;
            }
            Some(c) if is_ascii_lower_alpha(c as u32) => {
                self.tag.as_mut().unwrap().name.push(c);
                self.temp_buf.push(c);
                return None;
            }
            _ => (),
        }
        let mut tok = vec![
            HtmlTokenizer::char_token('<'),
            HtmlTokenizer::char_token('/'),
        ];
        tok.append(&mut self.temp_buf_to_tokens());
        let mut reconsumed = self.rawtext(c).unwrap_or_default();
        tok.append(&mut reconsumed);
        Some(tok)
    }

    fn script_data_less_than_sign(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn script_data_end_tag_open(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn script_data_end_tag_name(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn script_data_escape_start(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn script_data_escape_start_dash(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn script_data_escaped(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn script_data_escaped_dash(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn script_data_escaped_dash_dash(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn script_data_escaped_less_than_sign(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn script_data_escaped_end_tag_open(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn script_data_escaped_end_tag_name(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn script_data_double_escape_start(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn script_data_double_escaped(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn script_data_double_escaped_dash(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn script_data_double_escaped_dash_dash(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn script_data_double_escaped_less_than_sign(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn script_data_double_escape_end(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn before_attribute_name(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn attribute_name(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn after_attribute_name(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn before_attribute_value(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn attribute_value_double_quoted(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn attribute_value_single_quoted(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn attribute_value_unquoted(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn after_attribute_value_quoted(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn self_closing_start_tag(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn bogus_comment(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn markup_declaration_open(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn comment_start(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn comment_start_dash(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn comment(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn comment_less_than_sign(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn comment_less_than_sign_bang(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn comment_less_than_sign_bang_dash(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn comment_less_than_sign_bang_dash_dash(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn comment_end_dash(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn comment_end(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn comment_end_bang(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn doctype(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn before_doctype_name(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn doctype_name(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn after_doctype_name(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn after_doctype_public_keyword(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn before_doctype_public_identifier(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn doctype_public_identifier_double_quoted(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn doctype_public_identifier_single_quoted(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn after_doctype_public_identifier(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn between_doctype_public_and_system_identifiers(
        &mut self,
        c: Option<char>,
    ) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn after_doctype_system_keyword(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn before_doctype_system_identifier(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn doctype_system_identifier_double_quoted(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn doctype_system_identifier_single_quoted(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn after_doctype_system_identifier(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn bogus_doctype(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn cdata_section(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn cdata_section_bracket(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn cdata_section_end(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn character_reference(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn named_character_reference(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn ambiguous_ampersand(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn numeric_character_reference(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn hexadecimal_character_reference_start(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn decimal_character_reference_start(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn hexadecimal_character_reference(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn decimal_character_reference(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }

    fn numeric_character_reference_end(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        unreachable!()
    }
}
