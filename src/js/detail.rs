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
use crate::js::JsValue;
use crate::string::Utf16String;
use gc::*;

pub type GcJsValue = Gc<JsValue>;

#[derive(Trace, Finalize)]
pub struct JsBigInt {}

#[derive(Trace, Finalize)]
pub enum JsNumber {
    Integer(i64),
    Float(f64),
}

pub trait JsSymbol {
    fn typename(&self) -> &str;

    // NOTE: returns `undefined` or string
    fn description(&self) -> Option<&Utf16String>;
}

pub trait JsObject {
    fn typename(&self) -> &str;

    fn get_prototype(&mut self) -> Option<GcJsValue>;
    fn set_prototype(&mut self, v: Option<GcJsValue>) -> bool;
    fn is_extensible(&mut self) -> bool;
    fn prevent_extensions(&mut self) -> bool;
    // TODO: return type is a PropertyDescriptor
    fn get_own_property(&mut self, key: GcJsValue) -> ();
    // TODO: desc is a PropertyDescriptor
    fn define_own_property(&mut self, key: GcJsValue, desc: ()) -> bool;
    fn has_property(&mut self, key: GcJsValue) -> bool;
    fn get(&mut self, key: GcJsValue, this: GcJsValue) -> GcJsValue;
    fn set(&mut self, key: GcJsValue, val: GcJsValue, this: GcJsValue) -> bool;
    fn delete(&mut self, key: GcJsValue) -> bool;
    fn own_property_keys(&mut self) -> Vec<GcJsValue>;
    //fn call(&mut self, this: GcJsValue, args: &Vec<GcJsValue>) -> GcJsValue;
    //fn construct(&mut self, args: &Vec<GcJsValue>, this: GcJsValue) -> GcJsValue;
}

#[derive(Trace, Finalize)]
pub struct JsDataProp {
    value: JsValue,
    writable: bool,
    enumerable: bool,
    configurable: bool,
}

#[derive(Trace, Finalize)]
pub struct JsAccessorProp {
    get: JsValue,
    set: JsValue,
    enumerable: bool,
    configurable: bool,
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
