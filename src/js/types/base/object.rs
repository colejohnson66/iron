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
use crate::js::{JsKey, JsObject, JsType};

pub struct Object;

impl JsObject for Object {
    fn typename(&self) -> &str {
        unimplemented!();
    }

    fn get_prototype(&mut self) -> Option<&Box<dyn JsObject>> {
        unimplemented!();
    }
    fn set_prototype(&mut self, v: Option<&Box<dyn JsObject>>) -> bool {
        unimplemented!();
    }
    fn is_extensible(&mut self) -> bool {
        unimplemented!();
    }
    fn prevent_extensions(&mut self) -> bool {
        unimplemented!();
    }
    fn get_own_property(&mut self, key: &JsKey) -> &JsType {
        unimplemented!();
    }
    fn define_own_property(&mut self, key: &JsKey, desc: &JsType) -> bool {
        unimplemented!();
    }
    fn has_property(&mut self, key: &JsKey) -> bool {
        unimplemented!();
    }
    fn get(&mut self, key: &JsKey, this: ()) -> &JsType {
        unimplemented!();
    }
    fn set(&mut self, key: &JsKey, val: &JsType, this: ()) -> bool {
        unimplemented!();
    }
    fn delete(&mut self, key: &JsKey) -> bool {
        unimplemented!();
    }
    fn own_property_keys(&mut self) -> Vec<&JsKey> {
        unimplemented!();
    }
    fn call(&mut self, this: &JsType, args: Vec<&JsType>) -> &JsType {
        unimplemented!();
    }
    fn construct(&mut self, args: Vec<&JsType>, this: &Box<dyn JsObject>) -> &Box<dyn JsObject> {
        unimplemented!();
    }
}
