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
// Heavily inspired by <https://github.com/servo/html5ever/blob/master/rcdom/lib.rs>
use crate::html::parser::quirks::QuirksMode;
use crate::infra::namespace::Namespace;
use std::cell::{Cell, RefCell};
use std::mem;
use std::rc::{Rc, Weak};

pub type Handle = Rc<Node>;
pub type WeakHandle = Weak<Node>;

pub struct QualName {
    pub prefix: Option<String>,
    pub ns: Namespace,
    pub local: String,
}

pub struct ExpandedName {
    pub ns: Namespace,
    pub local: String,
}

pub struct Attribute {
    pub name: QualName,
    pub value: String,
}

pub enum NodeData {
    Comment {
        content: String,
    },
    Doctype {
        name: String,
        public_id: String,
        system_id: String,
    },
    Document,
    Element {
        name: QualName,
        attrs: RefCell<Vec<Attribute>>,
        template_contents: Option<Handle>,
        mathml_annotation_xml_integration_point: bool,
    },
    ProcessingInstruction {
        target: String,
        content: String,
    },
    Text {
        content: RefCell<String>,
    },
}

pub struct Node {
    pub parent: Cell<Option<WeakHandle>>,
    pub children: RefCell<Vec<Handle>>,
    pub data: NodeData,
}

impl Node {
    pub fn new(data: NodeData) -> Rc<Node> {
        Rc::new(Node {
            data: data,
            parent: Cell::new(None),
            children: RefCell::new(vec![]),
        })
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        let mut nodes = mem::replace(&mut *self.children.borrow_mut(), vec![]);
        while let Some(node) = nodes.pop() {
            let children = mem::replace(&mut *node.children.borrow_mut(), vec![]);
            nodes.extend(children.into_iter());
        }
    }
}

// fn append(new_parent: &Handle, child: Handle) {
//     let prev_parent = child.parent.replace(Some(Rc::downgrade(new_parent)));
//     assert_eq!(prev_parent.is_none(), true);
//     new_parent.children.borrow_mut().push(child);
// }

pub struct RcDom {
    pub document: Handle,
    pub quirks: QuirksMode,
}

impl RcDom {
    pub fn new() -> RcDom {
        RcDom {
            document: Node::new(NodeData::Document),
            quirks: QuirksMode::None,
        }
    }
}
