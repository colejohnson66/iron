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
use std::collections::HashMap;

pub struct Object {
    proto: Option<GcJsValue>,
    realm: (),
    props: HashMap<GcJsValue, GcJsValue>,
}

impl JsObject for Object {
    fn typename(&self) -> &str {
        "Object"
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

impl Object {
    fn new(value: Option<GcJsValue>) -> Object {
        unimplemented!();
    }
    fn assign(target: GcJsValue, sources: Vec<()>) -> GcJsValue {
        unimplemented!();
    }
    fn create(o: GcJsValue, properties: GcJsValue) -> GcJsValue {
        unimplemented!();
    }
    fn define_properties(o: GcJsValue, properties: GcJsValue) -> GcJsValue {
        unimplemented!();
    }
    fn entries(o: GcJsValue) -> GcJsValue {
        unimplemented!();
    }
    fn freeze(o: GcJsValue) -> GcJsValue {
        unimplemented!();
    }
    fn from_entries(iterable: GcJsValue) -> GcJsValue {
        unimplemented!();
    }
    fn get_own_property_descriptor(o: GcJsValue, p: GcJsValue) -> GcJsValue {
        unimplemented!();
    }
    fn get_own_property_descriptors(o: GcJsValue) -> GcJsValue {
        unimplemented!();
    }
    fn get_own_property_names(o: GcJsValue) -> GcJsValue {
        unimplemented!();
    }
    fn get_own_property_symbols(o: GcJsValue) -> GcJsValue {
        unimplemented!();
    }
    fn get_prototype_of(o: GcJsValue) -> GcJsValue {
        unimplemented!();
    }
    fn is(value1: GcJsValue, value2: GcJsValue) -> GcJsValue {
        unimplemented!();
    }
    fn is_extensible(o: GcJsValue) -> GcJsValue {
        unimplemented!();
    }
    fn is_frozen(o: GcJsValue) -> GcJsValue {
        unimplemented!();
    }
    fn is_sealed(o: GcJsValue) -> GcJsValue {
        unimplemented!();
    }
    fn keys(o: GcJsValue) -> GcJsValue {
        unimplemented!();
    }
    fn prevent_extensions(o: GcJsValue) -> GcJsValue {
        unimplemented!();
    }
    // Object.prototype
    fn seal(o: GcJsValue) -> GcJsValue {
        unimplemented!();
    }
    fn set_prototype_of(o: GcJsValue, proto: GcJsValue) -> GcJsValue {
        unimplemented!();
    }
    fn values(o: GcJsValue) -> GcJsValue {
        unimplemented!();
    }
    // TODO: properties (section 19.1.3)
}
