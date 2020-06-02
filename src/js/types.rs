/* ============================================================================
 * File:   types.rs
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
use crate::js::JsType;
use crate::string::Utf16String;

pub enum JsKey {
    String(Utf16String),
    Symbol(Box<dyn JsSymbol>),
}

pub struct JsBigInt {}

pub enum JsNumber {
    Integer(i64),
    Float(f64),
}

// TODO: same impl definition for BigInt
impl JsNumber {
    fn unary_minus(x: &JsNumber) -> JsNumber {
        unimplemented!();
    }
    fn bitwise_not(x: &JsNumber) -> JsNumber {
        unimplemented!();
    }
    fn exponentiate(x: &JsNumber, y: &JsNumber) -> Result<JsNumber, ()> {
        unimplemented!();
    }
    fn multiply(x: &JsNumber, y: &JsNumber) -> JsNumber {
        unimplemented!();
    }
    fn divide(x: &JsNumber, y: &JsNumber) -> Result<JsNumber, ()> {
        unimplemented!();
    }
    fn remainder(x: &JsNumber, y: &JsNumber) -> Result<JsNumber, ()> {
        unimplemented!();
    }
    fn add(x: &JsNumber, y: &JsNumber) -> JsNumber {
        unimplemented!();
    }
    fn subtract(x: &JsNumber, y: &JsNumber) -> JsNumber {
        unimplemented!();
    }
    fn left_shift(x: &JsNumber, y: &JsNumber) -> JsNumber {
        unimplemented!();
    }
    fn signed_right_shift(x: &JsNumber, y: &JsNumber) -> JsNumber {
        unimplemented!();
    }
    fn unsigned_right_shift(x: &JsNumber, y: &JsNumber) -> Result<JsNumber, ()> {
        unimplemented!();
    }
    fn less_than(x: &JsNumber, y: &JsNumber) -> Option<bool> {
        unimplemented!();
    }
    fn equal(x: &JsNumber, y: &JsNumber) -> bool {
        unimplemented!();
    }
    fn same_value(x: &JsNumber, y: &JsNumber) -> bool {
        unimplemented!();
    }
    fn same_value_zero(x: &JsNumber, y: &JsNumber) -> bool {
        unimplemented!();
    }
    fn bitwise_and(x: &JsNumber, y: &JsNumber) -> JsNumber {
        unimplemented!();
    }
    fn bitwise_xor(x: &JsNumber, y: &JsNumber) -> JsNumber {
        unimplemented!();
    }
    fn bitwise_or(x: &JsNumber, y: &JsNumber) -> JsNumber {
        unimplemented!();
    }
    fn to_string(x: &JsNumber, y: &JsNumber) -> Utf16String {
        unimplemented!();
    }
}

pub trait JsSymbol {
    fn typename(&self) -> &str;

    // NOTE: returns `JsType::Undefined` or `JsType::String`
    fn description(&self) -> &JsType;
}

pub trait JsObject {
    fn typename(&self) -> &str;
    fn get_prototype_of(&mut self) -> Option<&Box<dyn JsObject>>;
    fn set_prototype_of(&mut self, v: Option<&Box<dyn JsObject>>) -> bool;
    fn is_extensible(&mut self) -> bool;
    fn prevent_extensions(&mut self) -> bool;
    fn get_own_property(&mut self, p: &JsKey) -> &JsType;
    fn define_own_property(&mut self, p: &JsKey, desc: &JsType) -> bool;
    fn has_property(&self, p: &JsKey) -> bool;
    fn get(&mut self, p: &JsKey, receiver: ()) -> &JsType;
    fn set(&mut self, p: &JsKey, v: &JsType, receiver: ()) -> bool;
    fn delete(&mut self, p: &JsKey) -> bool;
    fn own_property_keys(&mut self) -> Vec<&JsKey>;
    fn call(&mut self, this: &JsType, args: Vec<&JsType>) -> &JsType;
    fn construct(&mut self, args: Vec<&JsType>, obj: &Box<dyn JsObject>) -> &Box<dyn JsObject>;
}

// TODO: implement section 7 <https://tc39.es/ecma262/#sec-abstract-operations>
