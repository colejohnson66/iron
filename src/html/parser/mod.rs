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

use crate::html::element::*;
use crate::html::parser::detail::*;
use crate::html::tokenizer::detail::*;
use crate::infra::namespace::Namespace;

pub struct HtmlParser {
    character_encoding: Option<EncodingCertainty>,

    document: RcDom,

    insertion_mode: InsertionMode,
    orig_insertion_mode: Option<InsertionMode>,

    open_elements_stack: Vec<Handle>,
    active_formatting_elements: Vec<Handle>,
    head_elem: Option<Handle>,
    form_elem: Option<Handle>,

    scripting: bool,
    frameset_ok: bool,

    template_insertion_modes: Vec<InsertionMode>,

    foster_parenting: bool,

    script_nesting_level: u32,
    parser_pause_flag: bool,

    context_elem: Option<Handle>,
}

impl HtmlParser {
    pub fn new() -> HtmlParser {
        HtmlParser {
            character_encoding: None,
            document: RcDom::new(),
            insertion_mode: InsertionMode::Initial,
            orig_insertion_mode: None,
            open_elements_stack: vec![],
            active_formatting_elements: vec![],
            head_elem: None,
            form_elem: None,
            scripting: true,
            frameset_ok: true,
            template_insertion_modes: vec![],
            foster_parenting: false,
            script_nesting_level: 0,
            parser_pause_flag: false,
            context_elem: None,
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

    fn elem_name(&self, target: &Handle) -> ExpandedName {
        match target.data {
            NodeData::Element { ref name, .. } => ExpandedName {
                ns: name.ns.clone(),
                local: name.local.clone(),
            },
            _ => panic!(),
        }
    }

    fn html_elem_named(&self, elem: &Handle, name: &str) -> bool {
        let expanded = self.elem_name(elem);
        expanded.ns == Namespace::Html && &expanded.local[..] == name
    }

    // https://html.spec.whatwg.org/multipage/parsing.html#reset-the-insertion-mode-appropriately
    fn reset_insertion_mode(&mut self) -> InsertionMode {
        for (i, mut node) in self.open_elements_stack.iter().enumerate().rev() {
            let last = i == 0usize;
            if last {
                match self.context_elem.as_ref() {
                    Some(ctx) => node = ctx,
                    _ => (),
                }
            }

            let name = match self.elem_name(node) {
                ExpandedName { ns, local } => match ns {
                    Namespace::Html => local,
                    _ => continue,
                },
            };
            match &name[..] {
                "select" => {
                    for ancestor in self.open_elements_stack[0..i].iter().rev() {
                        if self.html_elem_named(ancestor, "template") {
                            return InsertionMode::InSelect;
                        } else if self.html_elem_named(ancestor, "table") {
                            return InsertionMode::InSelectInTable;
                        }
                    }
                    return InsertionMode::InSelect;
                }
                "td" | "th" => {
                    if !last {
                        return InsertionMode::InCell;
                    }
                }
                "tr" => return InsertionMode::InRow,
                "tbody" | "thead" | "tfoot" => return InsertionMode::InTableBody,
                "caption" => return InsertionMode::InCaption,
                "colgroup" => return InsertionMode::InColumnGroup,
                "table" => return InsertionMode::InTable,
                "template" => return *self.template_insertion_modes.last().unwrap(),
                "head" => {
                    if !last {
                        return InsertionMode::InHead;
                    }
                }
                "body" => return InsertionMode::InBody,
                "frameset" => return InsertionMode::InFrameset,
                "html" => {
                    return match self.head_elem {
                        None => InsertionMode::BeforeHead,
                        Some(_) => InsertionMode::AfterHead,
                    }
                }
                _ => (),
            }
        }
        InsertionMode::InBody
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

impl HtmlParser {
    fn tree_construction_dispatcher(&mut self, _tok: Token) {
        let mut _foreign_content = true;
        if self.open_elements_stack.is_empty() {
            _foreign_content = true;
        }
    }
}
