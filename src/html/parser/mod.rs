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
pub mod quirks;

use crate::html::element::Node;
use crate::html::parser::detail::*;
use crate::html::tokenizer::detail::*;

pub struct HtmlParser {
    character_encoding: Option<EncodingCertainty>,

    insertion_mode: InsertionMode,
    orig_insertion_mode: Option<InsertionMode>,

    open_elements_stack: Vec<Node>,
    active_formatting_elements: Vec<Node>,
    head_element_pointer: Option<Node>,
    form_element_pointer: Option<Node>,

    scripting: bool,
    frameset_ok: bool,

    template_insertion_modes: Vec<InsertionMode>,

    script_nesting_level: u32,
    pub parser_pause_flag: bool,
}

impl HtmlParser {
    pub fn new() -> HtmlParser {
        HtmlParser {
            character_encoding: None,
            insertion_mode: InsertionMode::Initial,
            orig_insertion_mode: None,
            open_elements_stack: vec![],
            active_formatting_elements: vec![],
            head_element_pointer: None,
            form_element_pointer: None,
            scripting: true,
            frameset_ok: true,
            template_insertion_modes: vec![],
            script_nesting_level: 0,
            parser_pause_flag: false,
        }
    }

    fn start_tag_with_name(tag: &Tag, tag_name: &str) -> bool {
        &tag.name[..] == tag_name && !tag.is_end_tag
    }

    fn start_tag_with_names(tag: &Tag, tag_names: Vec<&str>) -> bool {
        if tag.is_end_tag {
            return false;
        }

        let tag = &tag.name[..];
        for name in tag_names {
            if tag == name {
                return true;
            }
        }
        false
    }

    fn end_tag_with_name(tag: &Tag, tag_name: &str) -> bool {
        &tag.name[..] == tag_name && tag.is_end_tag
    }

    fn end_tag_with_names(tag: &Tag, tag_names: Vec<&str>) -> bool {
        if !tag.is_end_tag {
            return false;
        }

        let tag = &tag.name[..];
        for name in tag_names {
            if tag == name {
                return true;
            }
        }
        false
    }

    // TODO: appropriate place for inserting a node

    // TODO: create element for token

    // TODO: insert a foreign element

    // TODO: adjust MathML attributes

    // TODO: adjust SVG attributes

    // TODO: adjust foreign attributes

    // TODO: insert a character

    // TODO: insert a comment

    // TODO: more
}
