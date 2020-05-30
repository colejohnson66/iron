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
use crate::infra::namespace::Namespace;

pub enum ParseHtmlError {
    AbruptClosingOfEmptyComment,
    AbruptDoctypePublicIdentifier,
    AbruptDoctypeSystemIdentifier,
    AbsenseOfDigitsInNumericCharacterReference,
    CDataInHtmlContext,
    CharacterReferenceOutsideUnicodeRange,
    ControlCharacterInInputStream,
    ControlCharacterReference,
    EndTagWithAttributes,
    DuplicateAttribute,
    EndTagWithTrailingSolidus,
    EofBeforeTagName,
    EofInCData,
    EofInComment,
    EofInDoctype,
    EofInScriptHtmlCommentLikeText,
    EofInTag,
    IncorrectlyClosedComment,
    IncorrectlyOpenedComment,
    InvalidCharacterSequenceAfterDoctypeName,
    InvalidFirstCharacterOfTagName,
    MissingAttributeValue,
    MissingDoctypeName,
    MissingDoctypePublicIdentifier,
    MissingDoctypeSystemIdentifier,
    MissingEndTagName,
    MissingQuoteBeforeDoctypePublicIdentifier,
    MissingQuoteBeforeDoctypeSystemIdentifier,
    MissingSemicolonAfterCharacterReference,
    MissingWhitespaceAfterDoctypePublicKeyword,
    MissingWhitespaceAfterDoctypeSystemKeyword,
    MissingWhitespaceBeforeDoctypeName,
    MissingWhitespaceBetweenAttributes,
    MissingWhitespaceBetweenDoctypePublicAndSystemIdentifiers,
    NestedComment,
    NoncharacterCharacterReference,
    NoncharacterInInputStream,
    NonVoidHtmlElementStartTagWithTrailingSolidus,
    NullCharacterReference,
    SurrogateCharacterReference,
    SurrogateCharacterInInputStream,
    UnexpectedCharacterAfterDoctypeSystemIdentifier,
    UnexpectedCharacterInAttributeName,
    UnexpectedCharacterInUnquotedAttributeValue,
    UnexpectedEqualsSignBeforeAttributeName,
    UnexpectedNullCharacter,
    UnexpectedQuestionMarkInsteadOfTagName,
    UnexpectedSolidusInTag,
    UnknownNamedCharacterReference,
}

#[derive(Copy, Clone)]
pub enum InsertionMode {
    Initial,
    BeforeHtml,
    BeforeHead,
    InHead,
    InHeadNoscript,
    AfterHead,
    InBody,
    Text,
    InTable,
    InTableText,
    InCaption,
    InColumnGroup,
    InTableBody,
    InRow,
    InCell,
    InSelect,
    InSelectInTable,
    InTemplate,
    AfterBody,
    InFrameset,
    AfterFrameset,
    AfterAfterBody,
    AfterAfterFrameset,
}

pub enum EncodingCertainty {
    Certain(String),
    Irrelevant,
    Tentative(String),
}

pub struct ElementDetail;

impl ElementDetail {
    pub fn special_element(ns: Namespace, element_name: &str) -> bool {
        let name = &element_name.to_ascii_lowercase()[..];

        match ns {
            Namespace::Html => match name {
                "address" => true,
                "applet" => true,
                "area" => true,
                "article" => true,
                "aside" => true,
                "base" => true,
                "basefont" => true,
                "bgsound" => true,
                "blockquote" => true,
                "body" => true,
                "br" => true,
                "button" => true,
                "caption" => true,
                "center" => true,
                "col" => true,
                "colgroup" => true,
                "dd" => true,
                "details" => true,
                "dir" => true,
                "div" => true,
                "dl" => true,
                "dt" => true,
                "embed" => true,
                "fieldset" => true,
                "figcaption" => true,
                "figure" => true,
                "footer" => true,
                "form" => true,
                "frame" => true,
                "frameset" => true,
                "h1" => true,
                "h2" => true,
                "h3" => true,
                "h4" => true,
                "h5" => true,
                "h6" => true,
                "head" => true,
                "header" => true,
                "hgroup" => true,
                "hr" => true,
                "html" => true,
                "iframe" => true,
                "img" => true,
                "input" => true,
                "li" => true,
                "link" => true,
                "listing" => true,
                "main" => true,
                "marquee" => true,
                "menu" => true,
                "meta" => true,
                "nav" => true,
                "noembed" => true,
                "noframes" => true,
                "noscript" => true,
                "object" => true,
                "ol" => true,
                "p" => true,
                "param" => true,
                "plaintext" => true,
                "pre" => true,
                "script" => true,
                "section" => true,
                "select" => true,
                "source" => true,
                "style" => true,
                "summary" => true,
                "table" => true,
                "tbody" => true,
                "td" => true,
                "template" => true,
                "textarea" => true,
                "tfoot" => true,
                "th" => true,
                "thead" => true,
                "title" => true,
                "tr" => true,
                "track" => true,
                "ul" => true,
                "wbr" => true,
                "xmp" => true,
                _ => false,
            },
            Namespace::MathML => match name {
                "annotation-xml" => true,
                "mi" => true,
                "mo" => true,
                "mn" => true,
                "ms" => true,
                "mtext" => true,
                _ => false,
            },
            Namespace::Svg => match name {
                "desc" => true,
                "foreignobject" => true,
                "title" => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn formatting_element(ns: Namespace, element_name: &str) -> bool {
        let name = &element_name.to_ascii_lowercase()[..];

        match ns {
            Namespace::Html => match name {
                "a" => true,
                "b" => true,
                "big" => true,
                "code" => true,
                "em" => true,
                "font" => true,
                "i" => true,
                "nobr" => true,
                "s" => true,
                "small" => true,
                "strike" => true,
                "strong" => true,
                "tt" => true,
                "u" => true,
                _ => false,
            },
            _ => false,
        }
    }
}

//static Tags: &'static [&'static str] = &[""];
