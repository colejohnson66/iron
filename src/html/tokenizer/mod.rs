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
pub mod detail;

use crate::html::parser::detail::ParseHtmlError;
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
    doctype: Option<Doctype>,

    temp_buf: String,
    char_ref_code: u32,

    tokens_to_emit: VecDeque<Token>,
}

impl HtmlTokenizer {
    pub fn new(html: &str) -> HtmlTokenizer {
        HtmlTokenizer {
            html: LineCountingChars::new(html),
            state: State::Data,
            return_state: None,
            last_emitted_tag: None,
            comment: None,
            tag: None,
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
        // section 12.2.5.1
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
        // section 12.2.5.2
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
        // section 12.2.5.3
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
        // section 12.2.5.4
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
        // section 12.2.5.5
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
        // section 12.2.5.6
        match c {
            Some('!') => {
                self.state = State::MarkupDeclarationOpen;
                None
            }
            Some('/') => {
                self.state = State::EndTagOpen;
                None
            }
            Some(c) if ascii_alpha(c as u32) => {
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
        // section 12.2.5.7
        match c {
            Some(c) if ascii_alpha(c as u32) => {
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
        // section 12.2.5.8
        match c {
            Some(c) if ascii_whitespace(c as u32) => {
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
                    None => panic!(),
                }
            }
            Some(c) if ascii_upper_alpha(c as u32) => {
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
        // section 12.2.5.9
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
        // section 12.2.5.10
        match c {
            Some(c) if ascii_alpha(c as u32) => {
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
        // section 12.2.5.11
        match c {
            Some(c) if ascii_whitespace(c as u32) => {
                if self.end_tag_appropriate() {
                    self.state = State::BeforeAttributeName;
                    return None;
                }
                // otherwise treat as "anything else"
            }
            Some('/') => {
                if self.end_tag_appropriate() {
                    self.state = State::SelfClosingStartTag;
                    return None;
                }
                // otherwise treat as "anything else"
            }
            Some('>') => {
                if self.end_tag_appropriate() {
                    self.state = State::Data;
                    let tag = self.tag.clone().unwrap();
                    self.tag = None;
                    return Some(vec![Token::Tag(tag)]);
                }
                // otherwise treat as "anything else"
            }
            Some(c) if ascii_upper_alpha(c as u32) => {
                let lower_c = HtmlTokenizer::lowercase_char_from_ascii_upper(c);
                self.tag.as_mut().unwrap().name.push(lower_c);
                self.temp_buf.push(c);
                return None;
            }
            Some(c) if ascii_lower_alpha(c as u32) => {
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
        // section 12.2.5.12
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
        // section 12.2.5.13
        match c {
            Some(c) if ascii_alpha(c as u32) => {
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
        // section 12.2.5.14
        match c {
            Some(c) if ascii_whitespace(c as u32) => {
                if self.end_tag_appropriate() {
                    self.state = State::BeforeAttributeName;
                    return None;
                }
                // otherwise treat as "anything else"
            }
            Some('/') => {
                if self.end_tag_appropriate() {
                    self.state = State::SelfClosingStartTag;
                    return None;
                }
                // otherwise treat as "anything else"
            }
            Some('>') => {
                if self.end_tag_appropriate() {
                    self.state = State::Data;
                    return None;
                }
                // otherwise treat as "anything else"
            }
            Some(c) if ascii_upper_alpha(c as u32) => {
                let lower_c = HtmlTokenizer::lowercase_char_from_ascii_upper(c);
                self.tag.as_mut().unwrap().name.push(lower_c);
                self.temp_buf.push(c);
                return None;
            }
            Some(c) if ascii_lower_alpha(c as u32) => {
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
        // section 12.2.5.15
        match c {
            Some('/') => {
                self.temp_buf = "".into();
                self.state = State::ScriptDataEndTagOpen;
                None
            }
            Some('!') => {
                self.state = State::ScriptDataEscapeStart;
                Some(vec![
                    HtmlTokenizer::char_token('<'),
                    HtmlTokenizer::char_token('!'),
                ])
            }
            _ => {
                let mut tok = vec![HtmlTokenizer::char_token('<')];
                let mut reconsumed = self.script_data(c).unwrap_or_default();
                tok.append(&mut reconsumed);
                Some(tok)
            }
        }
    }

    fn script_data_end_tag_open(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.16
        match c {
            Some(c) if ascii_alpha(c as u32) => {
                self.tag = Some(Tag::new(true));
                self.script_data_end_tag_name(Some(c))
            }
            _ => {
                let mut tok = vec![
                    HtmlTokenizer::char_token('<'),
                    HtmlTokenizer::char_token('/'),
                ];
                let mut reconsumed = self.script_data(c).unwrap_or_default();
                tok.append(&mut reconsumed);
                Some(tok)
            }
        }
    }

    fn script_data_end_tag_name(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.17
        match c {
            Some(c) if ascii_whitespace(c as u32) => {
                if self.end_tag_appropriate() {
                    self.state = State::BeforeAttributeName;
                    return None;
                }
                // otherwise treat as "anything else"
            }
            Some('/') => {
                if self.end_tag_appropriate() {
                    self.state = State::SelfClosingStartTag;
                    return None;
                }
                // otherwise treat as "anything else"
            }
            Some('>') => {
                if self.end_tag_appropriate() {
                    self.state = State::Data;
                    match self.tag.as_ref() {
                        Some(tag) => return Some(vec![Token::Tag(tag.clone())]),
                        None => panic!(),
                    }
                }
                // otherwise treat as "anything else"
            }
            Some(c) if ascii_upper_alpha(c as u32) => {
                let lower_c = HtmlTokenizer::lowercase_char_from_ascii_upper(c);
                self.tag.as_mut().unwrap().name.push(lower_c);
                self.temp_buf.push(c);
                return None;
            }
            Some(c) if ascii_lower_alpha(c as u32) => {
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
        let mut reconsumed = self.script_data(c).unwrap_or_default();
        tok.append(&mut reconsumed);
        Some(tok)
    }

    fn script_data_escape_start(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.18
        match c {
            Some('-') => {
                self.state = State::ScriptDataEscapeStartDash;
                Some(vec![HtmlTokenizer::char_token('-')])
            }
            _ => self.script_data(c),
        }
    }

    fn script_data_escape_start_dash(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.19
        match c {
            Some('-') => {
                self.state = State::ScriptDataEscapedDashDash;
                Some(vec![HtmlTokenizer::char_token('-')])
            }
            _ => self.script_data(c),
        }
    }

    fn script_data_escaped(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.20
        match c {
            Some('-') => {
                self.state = State::ScriptDataEscapedDash;
                Some(vec![HtmlTokenizer::char_token('-')])
            }
            Some('<') => {
                self.state = State::ScriptDataEscapedLessThanSign;
                None
            }
            Some('\0') => {
                self.error(ParseHtmlError::UnexpectedNullCharacter);
                Some(vec![HtmlTokenizer::replacement_char_token()])
            }
            None => {
                self.error(ParseHtmlError::EofInScriptHtmlCommentLikeText);
                Some(vec![HtmlTokenizer::eof_token()])
            }
            Some(c) => Some(vec![HtmlTokenizer::char_token(c)]),
        }
    }

    fn script_data_escaped_dash(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.21
        match c {
            Some('-') => {
                self.state = State::ScriptDataEscapedDashDash;
                Some(vec![HtmlTokenizer::char_token('-')])
            }
            Some('<') => {
                self.state = State::ScriptDataEscapedLessThanSign;
                None
            }
            Some('\0') => {
                self.error(ParseHtmlError::UnexpectedNullCharacter);
                self.state = State::ScriptDataEscaped;
                Some(vec![HtmlTokenizer::replacement_char_token()])
            }
            None => {
                self.error(ParseHtmlError::EofInScriptHtmlCommentLikeText);
                Some(vec![HtmlTokenizer::eof_token()])
            }
            Some(c) => {
                self.state = State::ScriptDataEscaped;
                Some(vec![HtmlTokenizer::char_token(c)])
            }
        }
    }

    fn script_data_escaped_dash_dash(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.22
        match c {
            Some('-') => Some(vec![HtmlTokenizer::char_token('-')]),
            Some('<') => {
                self.state = State::ScriptDataEscapedLessThanSign;
                None
            }
            Some('>') => {
                self.state = State::ScriptData;
                Some(vec![HtmlTokenizer::char_token('>')])
            }
            Some('\0') => {
                self.error(ParseHtmlError::UnexpectedNullCharacter);
                self.state = State::ScriptDataEscaped;
                Some(vec![HtmlTokenizer::replacement_char_token()])
            }
            None => {
                self.error(ParseHtmlError::EofInScriptHtmlCommentLikeText);
                Some(vec![HtmlTokenizer::eof_token()])
            }
            Some(c) => {
                self.state = State::ScriptDataEscaped;
                Some(vec![HtmlTokenizer::char_token(c)])
            }
        }
    }

    fn script_data_escaped_less_than_sign(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.23
        match c {
            Some('/') => {
                self.temp_buf = "".into();
                self.state = State::ScriptDataEscapedEndTagOpen;
                None
            }
            Some(c) if ascii_alpha(c as u32) => {
                self.temp_buf = "".into();
                let mut tok = vec![HtmlTokenizer::char_token('<')];
                let mut reconsumed = self
                    .script_data_double_escape_start(Some(c))
                    .unwrap_or_default();
                tok.append(&mut reconsumed);
                Some(tok)
            }
            _ => {
                let mut tok = vec![HtmlTokenizer::char_token('<')];
                let mut reconsumed = self.script_data_escaped(c).unwrap_or_default();
                tok.append(&mut reconsumed);
                Some(tok)
            }
        }
    }

    fn script_data_escaped_end_tag_open(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.24
        match c {
            Some(c) if ascii_alpha(c as u32) => {
                self.tag = Some(Tag::new(true));
                self.script_data_escaped_end_tag_name(Some(c))
            }
            _ => {
                let mut tok = vec![
                    HtmlTokenizer::char_token('<'),
                    HtmlTokenizer::char_token('/'),
                ];
                let mut reconsumed = self.script_data_escaped(c).unwrap_or_default();
                tok.append(&mut reconsumed);
                Some(tok)
            }
        }
    }

    fn script_data_escaped_end_tag_name(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.25
        match c {
            Some(c) if ascii_whitespace(c as u32) => {
                if self.end_tag_appropriate() {
                    self.state = State::BeforeAttributeName;
                    return None;
                }
                // otherwise treat as "anything else"
            }
            Some('/') => {
                if self.end_tag_appropriate() {
                    self.state = State::SelfClosingStartTag;
                    return None;
                }
                // otherwise treat as "anything else"
            }
            Some('>') => {
                if self.end_tag_appropriate() {
                    self.state = State::Data;
                    match self.tag.as_ref() {
                        Some(tag) => return Some(vec![Token::Tag(tag.clone())]),
                        None => panic!(),
                    }
                }
                // otherwise treat as "anything else"
            }
            Some(c) if ascii_upper_alpha(c as u32) => {
                let lower_c = HtmlTokenizer::lowercase_char_from_ascii_upper(c);
                self.tag.as_mut().unwrap().name.push(lower_c);
                self.temp_buf.push(c);
                return None;
            }
            Some(c) if ascii_lower_alpha(c as u32) => {
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
        let mut reconsumed = self.script_data_escaped(c).unwrap_or_default();
        tok.append(&mut reconsumed);
        Some(tok)
    }

    fn script_data_double_escape_start(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.26
        match c {
            Some(c) if ascii_whitespace(c as u32) || c == '/' || c == '>' => {
                if self.temp_buf == "script" {
                    self.state = State::ScriptDataDoubleEscaped;
                } else {
                    self.state = State::ScriptDataEscaped;
                }
                Some(vec![HtmlTokenizer::char_token(c)])
            }
            Some(c) if ascii_upper_alpha(c as u32) => {
                let lower_c = HtmlTokenizer::lowercase_char_from_ascii_upper(c);
                self.temp_buf.push(lower_c);
                Some(vec![HtmlTokenizer::char_token(c)])
            }
            Some(c) if ascii_lower_alpha(c as u32) => {
                self.temp_buf.push(c);
                Some(vec![HtmlTokenizer::char_token(c)])
            }
            _ => self.script_data_escaped(c),
        }
    }

    fn script_data_double_escaped(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.27
        match c {
            Some('-') => {
                self.state = State::ScriptDataDoubleEscapedDash;
                Some(vec![HtmlTokenizer::char_token('-')])
            }
            Some('<') => {
                self.state = State::ScriptDataDoubleEscapedLessThanSign;
                Some(vec![HtmlTokenizer::char_token('<')])
            }
            Some('\0') => {
                self.error(ParseHtmlError::UnexpectedNullCharacter);
                Some(vec![HtmlTokenizer::replacement_char_token()])
            }
            None => {
                self.error(ParseHtmlError::EofInScriptHtmlCommentLikeText);
                Some(vec![HtmlTokenizer::eof_token()])
            }
            Some(c) => Some(vec![HtmlTokenizer::char_token(c)]),
        }
    }

    fn script_data_double_escaped_dash(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.28
        match c {
            Some('-') => {
                self.state = State::ScriptDataDoubleEscapedDashDash;
                Some(vec![HtmlTokenizer::char_token('-')])
            }
            Some('<') => {
                self.state = State::ScriptDataDoubleEscapedLessThanSign;
                Some(vec![HtmlTokenizer::char_token('<')])
            }
            Some('\0') => {
                self.error(ParseHtmlError::UnexpectedNullCharacter);
                self.state = State::ScriptDataDoubleEscaped;
                Some(vec![HtmlTokenizer::replacement_char_token()])
            }
            None => {
                self.error(ParseHtmlError::EofInScriptHtmlCommentLikeText);
                Some(vec![HtmlTokenizer::eof_token()])
            }
            Some(c) => {
                self.state = State::ScriptDataDoubleEscaped;
                Some(vec![HtmlTokenizer::char_token(c)])
            }
        }
    }

    fn script_data_double_escaped_dash_dash(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.29
        match c {
            Some('-') => Some(vec![HtmlTokenizer::char_token('-')]),
            Some('<') => {
                self.state = State::ScriptDataDoubleEscapedLessThanSign;
                Some(vec![HtmlTokenizer::char_token('<')])
            }
            Some('>') => {
                self.state = State::ScriptData;
                Some(vec![HtmlTokenizer::char_token('>')])
            }
            Some('\0') => {
                self.error(ParseHtmlError::UnexpectedNullCharacter);
                self.state = State::ScriptDataDoubleEscaped;
                Some(vec![HtmlTokenizer::replacement_char_token()])
            }
            None => {
                self.error(ParseHtmlError::EofInScriptHtmlCommentLikeText);
                Some(vec![HtmlTokenizer::eof_token()])
            }
            Some(c) => {
                self.state = State::ScriptDataDoubleEscaped;
                Some(vec![HtmlTokenizer::char_token(c)])
            }
        }
    }

    fn script_data_double_escaped_less_than_sign(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.30
        match c {
            Some('/') => {
                self.temp_buf = "".into();
                self.state = State::ScriptDataDoubleEscapeEnd;
                Some(vec![HtmlTokenizer::char_token('/')])
            }
            _ => self.script_data_double_escaped(c),
        }
    }

    fn script_data_double_escape_end(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.31
        match c {
            Some(c) if ascii_whitespace(c as u32) || c == '/' || c == '>' => {
                if self.temp_buf == "script" {
                    self.state = State::ScriptDataEscaped;
                } else {
                    self.state = State::ScriptDataDoubleEscaped;
                }
                Some(vec![HtmlTokenizer::char_token(c)])
            }
            Some(c) if ascii_upper_alpha(c as u32) => {
                let lower_c = HtmlTokenizer::lowercase_char_from_ascii_upper(c);
                self.temp_buf.push(lower_c);
                Some(vec![HtmlTokenizer::char_token(c)])
            }
            Some(c) if ascii_lower_alpha(c as u32) => {
                self.temp_buf.push(c);
                Some(vec![HtmlTokenizer::char_token(c)])
            }
            _ => self.script_data_double_escaped(c),
        }
    }

    fn before_attribute_name(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.32
        match c {
            Some(c) if ascii_whitespace(c as u32) => None,
            Some('/') | Some('>') | None => self.after_attribute_name(c),
            Some('=') => {
                self.error(ParseHtmlError::UnexpectedEqualsSignBeforeAttributeName);
                self.tag.as_mut().unwrap().create_attribute();
                self.tag.as_mut().unwrap().append_to_cur_attr_name('=');
                self.state = State::AttributeName;
                None
            }
            _ => {
                self.tag.as_mut().unwrap().create_attribute();
                self.attribute_name(c)
            }
        }
    }

    fn attribute_name(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.33
        // TODO: When leaving this state, and before emitting the tag token, check for duplicate attributes
        match c {
            Some(c) if ascii_whitespace(c as u32) || c == '/' || c == '>' => {
                return self.after_attribute_name(Some(c))
            }
            None => return self.after_attribute_name(None),
            Some('=') => {
                self.state = State::BeforeAttributeValue;
                return None;
            }
            Some(c) if ascii_upper_alpha(c as u32) => {
                let lower_c = HtmlTokenizer::lowercase_char_from_ascii_upper(c);
                self.tag.as_mut().unwrap().append_to_cur_attr_name(lower_c);
                return None;
            }
            Some('\0') => {
                self.error(ParseHtmlError::UnexpectedNullCharacter);
                self.tag
                    .as_mut()
                    .unwrap()
                    .append_to_cur_attr_name('\u{FFFD}');
                return None;
            }
            Some(c) if c == '"' || c == '\'' || c == '<' => {
                self.error(ParseHtmlError::UnexpectedCharacterInAttributeName);
                // treat as "anything else"
            }
            _ => (),
        }
        self.tag
            .as_mut()
            .unwrap()
            .append_to_cur_attr_name(c.unwrap());
        None
    }

    fn after_attribute_name(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.34
        match c {
            Some(c) if ascii_whitespace(c as u32) => None,
            Some('/') => {
                self.state = State::SelfClosingStartTag;
                None
            }
            Some('=') => {
                self.state = State::BeforeAttributeValue;
                None
            }
            Some('>') => {
                self.state = State::Data;
                let tag = self.tag.clone().unwrap();
                self.tag = None;
                Some(vec![Token::Tag(tag)])
            }
            None => {
                self.error(ParseHtmlError::EofInTag);
                Some(vec![HtmlTokenizer::eof_token()])
            }
            Some(c) => {
                self.tag.as_mut().unwrap().create_attribute();
                self.attribute_name(Some(c))
            }
        }
    }

    fn before_attribute_value(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.35
        match c {
            Some(c) if ascii_whitespace(c as u32) => None,
            Some('"') => {
                self.state = State::AttributeValueDoubleQuoted;
                None
            }
            Some('\'') => {
                self.state = State::AttributeValueSingleQuoted;
                None
            }
            Some('>') => {
                self.error(ParseHtmlError::MissingAttributeValue);
                self.state = State::Data;
                let tag = self.tag.clone().unwrap();
                self.tag = None;
                Some(vec![Token::Tag(tag)])
            }
            _ => self.attribute_value_unquoted(c),
        }
    }

    fn attribute_value_double_quoted(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.36
        match c {
            Some('"') => {
                self.state = State::AfterAttributeValueQuoted;
                None
            }
            Some('&') => {
                self.return_state = Some(State::AttributeValueDoubleQuoted);
                self.state = State::CharacterReference;
                None
            }
            Some('\0') => {
                self.error(ParseHtmlError::UnexpectedNullCharacter);
                self.tag
                    .as_mut()
                    .unwrap()
                    .append_to_cur_attr_value('\u{FFFD}');
                None
            }
            None => {
                self.error(ParseHtmlError::EofInTag);
                Some(vec![HtmlTokenizer::eof_token()])
            }
            Some(c) => {
                self.tag.as_mut().unwrap().append_to_cur_attr_value(c);
                None
            }
        }
    }

    fn attribute_value_single_quoted(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.37
        match c {
            Some('\'') => {
                self.state = State::AfterAttributeValueQuoted;
                None
            }
            Some('&') => {
                self.return_state = Some(State::AttributeValueSingleQuoted);
                self.state = State::CharacterReference;
                None
            }
            Some('\0') => {
                self.error(ParseHtmlError::UnexpectedNullCharacter);
                self.tag
                    .as_mut()
                    .unwrap()
                    .append_to_cur_attr_value('\u{FFFD}');
                None
            }
            None => {
                self.error(ParseHtmlError::EofInTag);
                Some(vec![HtmlTokenizer::eof_token()])
            }
            Some(c) => {
                self.tag.as_mut().unwrap().append_to_cur_attr_value(c);
                None
            }
        }
    }

    fn attribute_value_unquoted(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.38
        match c {
            Some(c) if ascii_whitespace(c as u32) => {
                self.state = State::BeforeAttributeName;
                return None;
            }
            Some('&') => {
                self.return_state = Some(State::AttributeValueUnquoted);
                self.state = State::CharacterReference;
                return None;
            }
            Some('>') => {
                self.state = State::Data;
                let tag = self.tag.clone().unwrap();
                self.tag = None;
                return Some(vec![Token::Tag(tag)]);
            }
            Some('\0') => {
                self.error(ParseHtmlError::UnexpectedNullCharacter);
                self.tag
                    .as_mut()
                    .unwrap()
                    .append_to_cur_attr_value('\u{FFFD}');
                return None;
            }
            Some('"') | Some('\'') | Some('<') | Some('=') | Some('`') => {
                self.error(ParseHtmlError::UnexpectedCharacterInUnquotedAttributeValue);
                // treat as "anything else"
            }
            None => {
                self.error(ParseHtmlError::EofInTag);
                return Some(vec![HtmlTokenizer::eof_token()]);
            }
            _ => (),
        }
        self.tag
            .as_mut()
            .unwrap()
            .append_to_cur_attr_value(c.unwrap());
        None
    }

    fn after_attribute_value_quoted(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.39
        match c {
            Some(c) if ascii_whitespace(c as u32) => {
                self.state = State::BeforeAttributeName;
                None
            }
            Some('/') => {
                self.state = State::SelfClosingStartTag;
                None
            }
            Some('>') => {
                self.state = State::Data;
                let tag = self.tag.clone().unwrap();
                self.tag = None;
                Some(vec![Token::Tag(tag)])
            }
            None => {
                self.error(ParseHtmlError::EofInTag);
                Some(vec![HtmlTokenizer::eof_token()])
            }
            _ => {
                self.error(ParseHtmlError::MissingWhitespaceBetweenAttributes);
                self.before_attribute_name(c)
            }
        }
    }

    fn self_closing_start_tag(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.40
        match c {
            Some('>') => {
                self.tag.as_mut().unwrap().set_self_closing_flag();
                self.state = State::Data;
                let tag = self.tag.clone().unwrap();
                self.tag = None;
                Some(vec![Token::Tag(tag)])
            }
            None => {
                self.error(ParseHtmlError::EofInTag);
                Some(vec![HtmlTokenizer::eof_token()])
            }
            _ => {
                self.error(ParseHtmlError::UnexpectedSolidusInTag);
                self.before_attribute_name(c)
            }
        }
    }

    fn bogus_comment(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.41
        match c {
            Some('>') => {
                self.state = State::Data;
                let comment = self.comment.clone().unwrap();
                self.comment = None;
                Some(vec![Token::Comment(comment)])
            }
            None => {
                let comment = self.comment.clone().unwrap();
                self.comment = None;
                Some(vec![Token::Comment(comment), HtmlTokenizer::eof_token()])
            }
            Some('\0') => {
                self.error(ParseHtmlError::UnexpectedNullCharacter);
                self.comment.as_mut().unwrap().value.push('\u{FFFD}');
                None
            }
            Some(c) => {
                self.comment.as_mut().unwrap().value.push(c);
                None
            }
        }
    }

    fn markup_declaration_open(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.42
        // NOTE: this fn only returns `None`
        match c {
            Some(c) => {
                let mut peek: [char; 6] = ['\0'; 6];
                let read = self.html.read_multiple(&mut peek);

                // Two U+002D HYPHEN-MINUS characters (-)
                if read >= 1 && c == '-' && peek[0] == '-' {
                    // consume only the current and the peeked char
                    // do so by backtracking until right after the second '-'
                    self.html.backtrack_multiple(read - 1);
                    self.comment = Some(Comment::new());
                    self.state = State::CommentStart;
                    return None;
                }

                let peeked: String = peek.iter().collect();

                // ASCII case-insensitive match for the word "DOCTYPE"
                if c.to_ascii_lowercase() == 'd' && peeked.to_ascii_lowercase() == "octype" {
                    // consume and switch state
                    self.state = State::Doctype;
                    return None;
                }

                // The string "[CDATA[" (the five uppercase letters "CDATA"  with
                //   a U+005B LEFT SQUARE BRACKET character before and after)
                if c == '[' && peeked == "CDATA[" {
                    // consume
                    // TODO: Consume those characters. If there is an adjusted
                    //   current node and it is not an element in the HTML
                    //   namespace, then switch to the CDATA section state.
                    //   Otherwise, this is a cdata-in-html-content parse error.
                    //   Create a comment token whose data is the "[CDATA[" string.
                    //   Switch to the bogus comment state.
                    panic!();
                }

                // anything else
                self.error(ParseHtmlError::IncorrectlyOpenedComment);
                self.comment = Some(Comment::new());
                self.state = State::BogusComment;
                // consume nothing here, so backtrack 7 (because `next()` read 1
                //   and this fn read 6)
                self.html.backtrack_multiple(7);
            }
            _ => {
                // anything else, but this is an EOF, so don't backtrack
            }
        }
        None
    }

    fn comment_start(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.43
        match c {
            Some('-') => {
                self.state = State::CommentStartDash;
                None
            }
            Some('>') => {
                self.error(ParseHtmlError::AbruptClosingOfEmptyComment);
                self.state = State::Data;
                let comment = self.comment.clone().unwrap();
                self.comment = None;
                Some(vec![Token::Comment(comment)])
            }
            _ => self.comment(c),
        }
    }

    fn comment_start_dash(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.44
        match c {
            Some('-') => {
                self.state = State::CommentEnd;
                None
            }
            Some('>') => {
                self.error(ParseHtmlError::AbruptClosingOfEmptyComment);
                self.state = State::Data;
                let comment = self.comment.clone().unwrap();
                self.comment = None;
                Some(vec![Token::Comment(comment)])
            }
            None => {
                self.error(ParseHtmlError::EofInComment);
                let comment = self.comment.clone().unwrap();
                self.comment = None;
                Some(vec![Token::Comment(comment), HtmlTokenizer::eof_token()])
            }
            _ => {
                self.comment.as_mut().unwrap().value.push('-');
                self.comment(c)
            }
        }
    }

    fn comment(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.45
        match c {
            Some('<') => {
                self.comment.as_mut().unwrap().value.push('<');
                self.state = State::CommentLessThanSign;
                None
            }
            Some('-') => {
                self.state = State::CommentEndDash;
                None
            }
            Some('\0') => {
                self.error(ParseHtmlError::UnexpectedNullCharacter);
                self.comment.as_mut().unwrap().value.push('\u{FFFD}');
                None
            }
            None => {
                self.error(ParseHtmlError::EofInComment);
                let comment = self.comment.clone().unwrap();
                self.comment = None;
                Some(vec![Token::Comment(comment), HtmlTokenizer::eof_token()])
            }
            Some(c) => {
                self.comment.as_mut().unwrap().value.push(c);
                None
            }
        }
    }

    fn comment_less_than_sign(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.46
        match c {
            Some('!') => {
                self.comment.as_mut().unwrap().value.push('!');
                self.state = State::CommentLessThanSignBang;
                None
            }
            Some('<') => {
                self.comment.as_mut().unwrap().value.push('<');
                None
            }
            _ => self.comment(c),
        }
    }

    fn comment_less_than_sign_bang(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.47
        match c {
            Some('-') => {
                self.state = State::CommentLessThanSignBangDash;
                None
            }
            _ => self.comment(c),
        }
    }

    fn comment_less_than_sign_bang_dash(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.48
        match c {
            Some('-') => {
                self.state = State::CommentLessThanSignBangDashDash;
                None
            }
            _ => self.comment_end_dash(c),
        }
    }

    fn comment_less_than_sign_bang_dash_dash(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.49
        match c {
            Some('<') => self.comment_end(Some('<')),
            None => self.comment_end(None),
            _ => {
                self.error(ParseHtmlError::NestedComment);
                self.comment_end(c)
            }
        }
    }

    fn comment_end_dash(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.50
        match c {
            Some('-') => {
                self.state = State::CommentEnd;
                None
            }
            None => {
                self.error(ParseHtmlError::EofInComment);
                let comment = self.comment.clone().unwrap();
                self.comment = None;
                Some(vec![Token::Comment(comment), HtmlTokenizer::eof_token()])
            }
            _ => {
                self.comment.as_mut().unwrap().value.push('-');
                self.comment(c)
            }
        }
    }

    fn comment_end(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.51
        match c {
            Some('>') => {
                self.state = State::Data;
                let comment = self.comment.clone().unwrap();
                self.comment = None;
                Some(vec![Token::Comment(comment)])
            }
            Some('!') => {
                self.state = State::CommentEndBang;
                None
            }
            Some('-') => {
                self.comment.as_mut().unwrap().value.push('-');
                None
            }
            None => {
                self.error(ParseHtmlError::EofInComment);
                let comment = self.comment.clone().unwrap();
                self.comment = None;
                Some(vec![Token::Comment(comment), HtmlTokenizer::eof_token()])
            }
            _ => {
                self.comment.as_mut().unwrap().value.push_str("--");
                self.comment(c)
            }
        }
    }

    fn comment_end_bang(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.52
        match c {
            Some('-') => {
                self.comment.as_mut().unwrap().value.push_str("--!");
                self.state = State::CommentEndDash;
                None
            }
            Some('>') => {
                self.error(ParseHtmlError::IncorrectlyClosedComment);
                self.state = State::Data;
                let comment = self.comment.clone().unwrap();
                self.comment = None;
                Some(vec![Token::Comment(comment)])
            }
            None => {
                self.error(ParseHtmlError::EofInComment);
                let comment = self.comment.clone().unwrap();
                self.comment = None;
                Some(vec![Token::Comment(comment), HtmlTokenizer::eof_token()])
            }
            _ => {
                self.comment.as_mut().unwrap().value.push_str("--!");
                self.comment(c)
            }
        }
    }

    fn doctype(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.53
        match c {
            Some(c) if ascii_whitespace(c as u32) => {
                self.state = State::BeforeDoctypeName;
                None
            }
            Some('>') => self.before_doctype_name(Some('>')),
            None => {
                self.error(ParseHtmlError::EofInDoctype);
                let mut doctype = Doctype::new();
                doctype.force_quirks = true;
                Some(vec![Token::Doctype(doctype), HtmlTokenizer::eof_token()])
            }
            _ => {
                self.error(ParseHtmlError::MissingWhitespaceBeforeDoctypeName);
                self.before_doctype_name(c)
            }
        }
    }

    fn before_doctype_name(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.54
        match c {
            Some(c) if ascii_whitespace(c as u32) => None,
            Some(c) if ascii_upper_alpha(c as u32) => {
                let mut doctype = Doctype::new();
                doctype.append_to_name(c.to_ascii_lowercase());
                self.doctype = Some(doctype);
                self.state = State::DoctypeName;
                None
            }
            Some('\0') => {
                self.error(ParseHtmlError::UnexpectedNullCharacter);
                let mut doctype = Doctype::new();
                doctype.append_to_name('\u{FFFD}');
                self.doctype = Some(doctype);
                self.state = State::DoctypeName;
                None
            }
            Some('>') => {
                self.error(ParseHtmlError::MissingDoctypeName);
                let mut doctype = Doctype::new();
                doctype.force_quirks = true;
                Some(vec![Token::Doctype(doctype)])
            }
            None => {
                self.error(ParseHtmlError::EofInDoctype);
                let mut doctype = Doctype::new();
                doctype.force_quirks = true;
                Some(vec![Token::Doctype(doctype), HtmlTokenizer::eof_token()])
            }
            Some(c) => {
                let mut doctype = Doctype::new();
                doctype.append_to_name(c);
                self.doctype = Some(doctype);
                self.state = State::DoctypeName;
                None
            }
        }
    }

    fn doctype_name(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.55
        match c {
            Some(c) if ascii_whitespace(c as u32) => {
                self.state = State::AfterAttributeName;
                None
            }
            Some('>') => {
                self.state = State::Data;
                let doctype = self.doctype.clone().unwrap();
                self.doctype = None;
                Some(vec![Token::Doctype(doctype)])
            }
            Some(c) if ascii_upper_alpha(c as u32) => {
                self.doctype
                    .as_mut()
                    .unwrap()
                    .append_to_name(c.to_ascii_lowercase());
                None
            }
            Some('\0') => {
                self.error(ParseHtmlError::UnexpectedNullCharacter);
                self.doctype.as_mut().unwrap().append_to_name('\u{FFFD}');
                None
            }
            None => {
                self.error(ParseHtmlError::EofInDoctype);
                let mut doctype = Doctype::new();
                doctype.force_quirks = true;
                Some(vec![Token::Doctype(doctype), HtmlTokenizer::eof_token()])
            }
            Some(c) => {
                self.doctype.as_mut().unwrap().append_to_name(c);
                None
            }
        }
    }

    fn after_doctype_name(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.56
        match c {
            Some(c) if ascii_whitespace(c as u32) => return None,
            Some('>') => {
                self.state = State::Data;
                let doctype = self.doctype.clone().unwrap();
                self.doctype = None;
                return Some(vec![Token::Doctype(doctype)]);
            }
            None => {
                self.error(ParseHtmlError::EofInDoctype);
                let mut doctype = Doctype::new();
                doctype.force_quirks = true;
                return Some(vec![Token::Doctype(doctype), HtmlTokenizer::eof_token()]);
            }
            Some(c) => {
                let mut peek: [char; 5] = ['\0'; 5];
                let read = self.html.read_multiple(&mut peek);
                if read != 5 {
                    // backtrack and reconsume `c`
                    self.html.backtrack_multiple(read);
                    self.error(ParseHtmlError::InvalidCharacterSequenceAfterDoctypeName);
                    self.doctype.as_mut().unwrap().force_quirks = true;
                    return self.bogus_doctype(Some(c));
                }

                let peeked = peek.iter().collect::<String>().to_ascii_lowercase();
                if c.to_ascii_lowercase() == 'p' && peeked == "ublic" {
                    // consume and switch state
                    self.state = State::AfterDoctypePublicKeyword;
                    return None;
                }
                if c.to_ascii_lowercase() == 's' && peeked == "ystem" {
                    // consume and switch state
                    self.state = State::AfterDoctypeSystemKeyword;
                    return None;
                }

                self.error(ParseHtmlError::InvalidCharacterSequenceAfterDoctypeName);
                self.doctype.as_mut().unwrap().force_quirks = true;
                return self.bogus_doctype(Some(c));
            }
        }
    }

    fn after_doctype_public_keyword(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.57
        match c {
            Some(c) if ascii_whitespace(c as u32) => {
                self.state = State::BeforeDoctypePublicIdentifier;
                None
            }
            Some('"') => {
                self.error(ParseHtmlError::MissingWhitespaceAfterDoctypePublicKeyword);
                self.doctype.as_mut().unwrap().public_id = Some("".into());
                self.state = State::DoctypePublicIdentifierDoubleQuoted;
                None
            }
            Some('\'') => {
                self.error(ParseHtmlError::MissingWhitespaceAfterDoctypePublicKeyword);
                self.doctype.as_mut().unwrap().public_id = Some("".into());
                self.state = State::DoctypePublicIdentifierSingleQuoted;
                None
            }
            Some('>') => {
                self.error(ParseHtmlError::MissingDoctypePublicIdentifier);
                let mut doctype = self.doctype.clone().unwrap();
                self.doctype = None;
                doctype.force_quirks = true;
                self.state = State::Data;
                Some(vec![Token::Doctype(doctype)])
            }
            None => {
                self.error(ParseHtmlError::EofInDoctype);
                let mut doctype = self.doctype.clone().unwrap();
                self.doctype = None;
                doctype.force_quirks = true;
                Some(vec![Token::Doctype(doctype), HtmlTokenizer::eof_token()])
            }
            _ => {
                self.error(ParseHtmlError::MissingQuoteBeforeDoctypePublicIdentifier);
                self.doctype.as_mut().unwrap().force_quirks = true;
                self.bogus_doctype(c)
            }
        }
    }

    fn before_doctype_public_identifier(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.58
        match c {
            Some(c) if ascii_whitespace(c as u32) => None,
            Some('"') => {
                self.doctype.as_mut().unwrap().public_id = Some("".into());
                self.state = State::DoctypePublicIdentifierDoubleQuoted;
                None
            }
            Some('\'') => {
                self.doctype.as_mut().unwrap().public_id = Some("".into());
                self.state = State::DoctypePublicIdentifierSingleQuoted;
                None
            }
            Some('>') => {
                self.error(ParseHtmlError::MissingDoctypePublicIdentifier);
                let mut doctype = self.doctype.clone().unwrap();
                self.doctype = None;
                doctype.force_quirks = true;
                self.state = State::Data;
                Some(vec![Token::Doctype(doctype)])
            }
            None => {
                self.error(ParseHtmlError::EofInDoctype);
                let mut doctype = self.doctype.clone().unwrap();
                self.doctype = None;
                doctype.force_quirks = true;
                Some(vec![Token::Doctype(doctype), HtmlTokenizer::eof_token()])
            }
            _ => {
                self.error(ParseHtmlError::MissingQuoteBeforeDoctypePublicIdentifier);
                self.doctype.as_mut().unwrap().force_quirks = true;
                self.bogus_doctype(c)
            }
        }
    }

    fn doctype_public_identifier_double_quoted(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.59
        match c {
            Some('"') => {
                self.state = State::AfterDoctypePublicIdentifier;
                None
            }
            Some('\0') => {
                self.error(ParseHtmlError::UnexpectedNullCharacter);
                self.doctype
                    .as_mut()
                    .unwrap()
                    .append_to_public_id('\u{FFFD}');
                None
            }
            Some('>') => {
                self.error(ParseHtmlError::AbruptDoctypePublicIdentifier);
                let mut doctype = self.doctype.clone().unwrap();
                self.doctype = None;
                doctype.force_quirks = true;
                self.state = State::Data;
                Some(vec![Token::Doctype(doctype)])
            }
            None => {
                self.error(ParseHtmlError::EofInDoctype);
                let mut doctype = self.doctype.clone().unwrap();
                self.doctype = None;
                doctype.force_quirks = true;
                Some(vec![Token::Doctype(doctype), HtmlTokenizer::eof_token()])
            }
            Some(c) => {
                self.doctype.as_mut().unwrap().append_to_public_id(c);
                None
            }
        }
    }

    fn doctype_public_identifier_single_quoted(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.60
        match c {
            Some('\'') => {
                self.state = State::AfterDoctypePublicIdentifier;
                None
            }
            Some('\0') => {
                self.error(ParseHtmlError::UnexpectedNullCharacter);
                self.doctype
                    .as_mut()
                    .unwrap()
                    .append_to_public_id('\u{FFFD}');
                None
            }
            Some('>') => {
                self.error(ParseHtmlError::AbruptDoctypePublicIdentifier);
                let mut doctype = self.doctype.clone().unwrap();
                self.doctype = None;
                doctype.force_quirks = true;
                self.state = State::Data;
                Some(vec![Token::Doctype(doctype)])
            }
            None => {
                self.error(ParseHtmlError::EofInDoctype);
                let mut doctype = self.doctype.clone().unwrap();
                self.doctype = None;
                doctype.force_quirks = true;
                Some(vec![Token::Doctype(doctype), HtmlTokenizer::eof_token()])
            }
            Some(c) => {
                self.doctype.as_mut().unwrap().append_to_public_id(c);
                None
            }
        }
    }

    fn after_doctype_public_identifier(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.61
        match c {
            Some(c) if ascii_whitespace(c as u32) => {
                self.state = State::BetweenDoctypePublicAndSystemIdentifiers;
                None
            }
            Some('>') => {
                self.state = State::Data;
                let doctype = self.doctype.clone().unwrap();
                self.doctype = None;
                Some(vec![Token::Doctype(doctype)])
            }
            Some('"') => {
                self.error(
                    ParseHtmlError::MissingWhitespaceBetweenDoctypePublicAndSystemIdentifiers,
                );
                self.doctype.as_mut().unwrap().system_id = Some("".into());
                self.state = State::DoctypeSystemIdentifierDoubleQuoted;
                None
            }
            Some('\'') => {
                self.error(
                    ParseHtmlError::MissingWhitespaceBetweenDoctypePublicAndSystemIdentifiers,
                );
                self.doctype.as_mut().unwrap().system_id = Some("".into());
                self.state = State::DoctypeSystemIdentifierSingleQuoted;
                None
            }
            None => {
                self.error(ParseHtmlError::EofInDoctype);
                let mut doctype = self.doctype.clone().unwrap();
                self.doctype = None;
                doctype.force_quirks = true;
                Some(vec![Token::Doctype(doctype), HtmlTokenizer::eof_token()])
            }
            _ => {
                self.error(ParseHtmlError::MissingQuoteBeforeDoctypeSystemIdentifier);
                self.doctype.as_mut().unwrap().force_quirks = true;
                self.bogus_doctype(c)
            }
        }
    }

    fn between_doctype_public_and_system_identifiers(
        &mut self,
        c: Option<char>,
    ) -> Option<Vec<Token>> {
        // section 12.2.5.62
        match c {
            Some(c) if ascii_whitespace(c as u32) => None,
            Some('>') => {
                self.state = State::Data;
                let doctype = self.doctype.clone().unwrap();
                self.doctype = None;
                Some(vec![Token::Doctype(doctype)])
            }
            Some('"') => {
                self.doctype.as_mut().unwrap().system_id = Some("".into());
                self.state = State::DoctypeSystemIdentifierDoubleQuoted;
                None
            }
            Some('\'') => {
                self.doctype.as_mut().unwrap().system_id = Some("".into());
                self.state = State::DoctypeSystemIdentifierSingleQuoted;
                None
            }
            None => {
                self.error(ParseHtmlError::EofInDoctype);
                let mut doctype = self.doctype.clone().unwrap();
                self.doctype = None;
                doctype.force_quirks = true;
                Some(vec![Token::Doctype(doctype), HtmlTokenizer::eof_token()])
            }
            _ => {
                self.error(ParseHtmlError::MissingQuoteBeforeDoctypeSystemIdentifier);
                self.doctype.as_mut().unwrap().force_quirks = true;
                self.bogus_doctype(c)
            }
        }
    }

    fn after_doctype_system_keyword(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.63
        match c {
            Some(c) if ascii_whitespace(c as u32) => {
                self.state = State::BeforeDoctypeSystemIdentifier;
                None
            }
            Some('"') => {
                self.error(ParseHtmlError::MissingWhitespaceAfterDoctypeSystemKeyword);
                self.doctype.as_mut().unwrap().system_id = Some("".into());
                self.state = State::DoctypeSystemIdentifierDoubleQuoted;
                None
            }
            Some('\'') => {
                self.error(ParseHtmlError::MissingWhitespaceAfterDoctypeSystemKeyword);
                self.doctype.as_mut().unwrap().system_id = Some("".into());
                self.state = State::DoctypeSystemIdentifierSingleQuoted;
                None
            }
            Some('>') => {
                self.error(ParseHtmlError::MissingDoctypeSystemIdentifier);
                let mut doctype = self.doctype.clone().unwrap();
                self.doctype = None;
                doctype.force_quirks = true;
                self.state = State::Data;
                Some(vec![Token::Doctype(doctype)])
            }
            None => {
                self.error(ParseHtmlError::EofInDoctype);
                let mut doctype = self.doctype.clone().unwrap();
                self.doctype = None;
                doctype.force_quirks = true;
                Some(vec![Token::Doctype(doctype), HtmlTokenizer::eof_token()])
            }
            _ => {
                self.error(ParseHtmlError::MissingQuoteBeforeDoctypeSystemIdentifier);
                self.doctype.as_mut().unwrap().force_quirks = true;
                self.bogus_doctype(c)
            }
        }
    }

    fn before_doctype_system_identifier(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.64
        match c {
            Some(c) if ascii_whitespace(c as u32) => None,
            Some('"') => {
                self.doctype.as_mut().unwrap().system_id = Some("".into());
                self.state = State::DoctypeSystemIdentifierDoubleQuoted;
                None
            }
            Some('\'') => {
                self.doctype.as_mut().unwrap().system_id = Some("".into());
                self.state = State::DoctypeSystemIdentifierSingleQuoted;
                None
            }
            Some('>') => {
                self.error(ParseHtmlError::MissingDoctypeSystemIdentifier);
                let mut doctype = self.doctype.clone().unwrap();
                self.doctype = None;
                doctype.force_quirks = true;
                self.state = State::Data;
                Some(vec![Token::Doctype(doctype)])
            }
            None => {
                self.error(ParseHtmlError::EofInDoctype);
                let mut doctype = self.doctype.clone().unwrap();
                self.doctype = None;
                doctype.force_quirks = true;
                Some(vec![Token::Doctype(doctype), HtmlTokenizer::eof_token()])
            }
            _ => {
                self.error(ParseHtmlError::MissingQuoteBeforeDoctypeSystemIdentifier);
                self.doctype.as_mut().unwrap().force_quirks = true;
                self.bogus_doctype(c)
            }
        }
    }

    fn doctype_system_identifier_double_quoted(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.65
        match c {
            Some('"') => {
                self.state = State::AfterDoctypeSystemIdentifier;
                None
            }
            Some('\0') => {
                self.error(ParseHtmlError::UnexpectedNullCharacter);
                self.doctype
                    .as_mut()
                    .unwrap()
                    .append_to_system_id('\u{FFFD}');
                None
            }
            Some('>') => {
                self.error(ParseHtmlError::AbruptDoctypeSystemIdentifier);
                let mut doctype = self.doctype.clone().unwrap();
                self.doctype = None;
                doctype.force_quirks = true;
                self.state = State::Data;
                Some(vec![Token::Doctype(doctype)])
            }
            None => {
                self.error(ParseHtmlError::EofInDoctype);
                let mut doctype = self.doctype.clone().unwrap();
                self.doctype = None;
                doctype.force_quirks = true;
                Some(vec![Token::Doctype(doctype), HtmlTokenizer::eof_token()])
            }
            Some(c) => {
                self.doctype.as_mut().unwrap().append_to_system_id(c);
                None
            }
        }
    }

    fn doctype_system_identifier_single_quoted(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.66
        match c {
            Some('\'') => {
                self.state = State::AfterDoctypeSystemIdentifier;
                None
            }
            Some('\0') => {
                self.error(ParseHtmlError::UnexpectedNullCharacter);
                self.doctype
                    .as_mut()
                    .unwrap()
                    .append_to_system_id('\u{FFFD}');
                None
            }
            Some('>') => {
                self.error(ParseHtmlError::AbruptDoctypeSystemIdentifier);
                let mut doctype = self.doctype.clone().unwrap();
                self.doctype = None;
                doctype.force_quirks = true;
                self.state = State::Data;
                Some(vec![Token::Doctype(doctype)])
            }
            None => {
                self.error(ParseHtmlError::EofInDoctype);
                let mut doctype = self.doctype.clone().unwrap();
                self.doctype = None;
                doctype.force_quirks = true;
                Some(vec![Token::Doctype(doctype), HtmlTokenizer::eof_token()])
            }
            Some(c) => {
                self.doctype.as_mut().unwrap().append_to_system_id(c);
                None
            }
        }
    }

    fn after_doctype_system_identifier(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.67
        match c {
            Some(c) if ascii_whitespace(c as u32) => None,
            Some('>') => {
                self.state = State::Data;
                let doctype = self.doctype.clone().unwrap();
                self.doctype = None;
                Some(vec![Token::Doctype(doctype)])
            }
            None => {
                self.error(ParseHtmlError::EofInDoctype);
                let mut doctype = self.doctype.clone().unwrap();
                self.doctype = None;
                doctype.force_quirks = true;
                Some(vec![Token::Doctype(doctype), HtmlTokenizer::eof_token()])
            }
            _ => {
                self.error(ParseHtmlError::UnexpectedCharacterAfterDoctypeSystemIdentifier);
                // do NOT set the force-quirks flag
                self.bogus_doctype(c)
            }
        }
    }

    fn bogus_doctype(&mut self, c: Option<char>) -> Option<Vec<Token>> {
        // section 12.2.5.68
        match c {
            Some('>') => {
                self.state = State::Data;
                let doctype = self.doctype.clone().unwrap();
                self.doctype = None;
                Some(vec![Token::Doctype(doctype)])
            }
            Some('\0') => {
                self.error(ParseHtmlError::UnexpectedNullCharacter);
                None
            }
            None => {
                let doctype = self.doctype.clone().unwrap();
                self.doctype = None;
                Some(vec![Token::Doctype(doctype), HtmlTokenizer::eof_token()])
            }
            _ => None,
        }
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
