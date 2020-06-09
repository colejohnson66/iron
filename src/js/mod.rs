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
//pub mod intrinsic;
pub mod tokenizer;
pub mod vm;

//use crate::string::Utf16String;
use gc::*;
use std::collections::HashMap;

pub type GcVtable = Gc<&'static dyn JsVtable>;
pub type GcVtableFn = fn(Vec<GcVtable>) -> GcVtable;

pub trait JsVtable {
    fn vtable(&self) -> &HashMap<String, GcVtableFn>;
    fn add_fn(&mut self, name: &str, func: GcVtableFn);
    fn delete_fn(&mut self, name: &str);
    fn call(&mut self, name: &str, args: Vec<GcVtable>) -> GcVtable;
}

pub struct DataProp {
    value: GcVtable,
    writable: bool,
    enumerable: bool,
    configurable: bool,
}
pub struct AccessorProp {
    get: GcVtable,
    set: GcVtable,
    enumerable: bool,
    configurable: bool,
}

#[macro_export]
macro_rules! vtable_impl {
    ($type:ty) => {
        impl crate::js::JsVtable for $type {
            fn vtable(&self) -> &std::collections::HashMap<String, crate::js::GcVtableFn> {
                &self.vtable
            }
            fn add_fn(&mut self, name: &str, func: crate::js::GcVtableFn) {
                self.vtable.insert(name.into(), func);
            }
            fn delete_fn(&mut self, name: &str) {
                self.vtable.remove(name);
            }
            fn call(&mut self, name: &str, args: Vec<crate::js::GcVtable>) -> crate::js::GcVtable {
                self.vtable.get(name).unwrap()(args)
            }
        }
    };
}

pub fn control(c: u32) -> bool {
    match c {
        0x200C | 0x200D | 0xFEFF => true,
        _ => false,
    }
}

pub fn white_space(c: u32) -> bool {
    match c {
        0x9 | 0xB | 0xC | 0x20 | 0xA0 | 0xFEFF => true,
        // TODO: any other Unicode "Space_Separator" code point
        _ => false,
    }
}

pub fn line_terminator(c: u32) -> bool {
    match c {
        0xA | 0xD | 0x2028 | 0x2029 => true,
        _ => false,
    }
}
