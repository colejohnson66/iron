/* ============================================================================
 * File:   object.rs
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
use crate::js::{GcJsValue, JsObject};

pub struct Object {}

impl JsObject for Object {
    fn typename(&self) -> &str {
        unimplemented!();
    }

    fn get_prototype(&mut self) -> Option<GcJsValue> {
        unimplemented!();
    }
    fn set_prototype(&mut self, v: Option<GcJsValue>) -> bool {
        unimplemented!();
    }
    fn is_extensible(&mut self) -> bool {
        unimplemented!();
    }
    fn prevent_extensions(&mut self) -> bool {
        unimplemented!();
    }
    // TODO: return type is a PropertyDescriptor
    fn get_own_property(&mut self, key: GcJsValue) -> () {
        unimplemented!();
    }
    // TODO: desc is a PropertyDescriptor
    fn define_own_property(&mut self, key: GcJsValue, desc: ()) -> bool {
        unimplemented!();
    }
    fn has_property(&mut self, key: GcJsValue) -> bool {
        unimplemented!();
    }
    fn get(&mut self, key: GcJsValue, this: GcJsValue) -> GcJsValue {
        unimplemented!();
    }
    fn set(&mut self, key: GcJsValue, val: GcJsValue, this: GcJsValue) -> bool {
        unimplemented!();
    }
    fn delete(&mut self, key: GcJsValue) -> bool {
        unimplemented!();
    }
    fn own_property_keys(&mut self) -> Vec<GcJsValue> {
        unimplemented!();
    }
}
