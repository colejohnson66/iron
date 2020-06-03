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
// abstract operations as defined in section 7 of the ECMAScript specification
use crate::js::{JsBigInt, JsHandle, JsKey, JsObject, JsSymbol, JsType, JsValue};
use crate::string::Utf16String;

pub fn type_(x: &JsValue) -> JsType {
    match x {
        JsValue::Undefined => JsType::Undefined,
        JsValue::Null => JsType::Null,
        JsValue::Boolean(_) => JsType::Boolean,
        JsValue::String(_) => JsType::String,
        JsValue::Symbol(_) => JsType::Symbol,
        JsValue::Number(_) => JsType::Number,
        JsValue::BigInt(_) => JsType::BigInt,
        JsValue::Object(_) => JsType::Object,
    }
}

pub fn to_primitive(input: &JsValue, preferred_type: Option<&str>) -> JsValue {
    if type_(&input) == JsType::Object {
        let hint = match preferred_type {
            None => "default",
            Some("String") => "string",
            Some("Number") => "number",
            _ => panic!(),
        };

        let exotic_to_prim = get_method(&input, "@@toPrimitive");
    }

    JsValue::Undefined
}
pub fn to_boolean(o: &JsValue, hint: &str) -> JsValue {
    unimplemented!();
}
pub fn to_numeric(value: &JsValue) -> JsValue {
    unimplemented!();
}
pub fn to_number(argument: &JsValue) -> JsValue {
    unimplemented!();
}
pub fn to_integer(argument: &JsValue) -> JsValue {
    unimplemented!();
}
pub fn to_int32(argument: &JsValue) -> i32 {
    unimplemented!();
}
pub fn to_uint32(argument: &JsValue) -> u32 {
    unimplemented!();
}
pub fn to_int16(argument: &JsValue) -> i16 {
    unimplemented!();
}
pub fn to_uint16(argument: &JsValue) -> u16 {
    unimplemented!();
}
pub fn to_int8(argument: &JsValue) -> i8 {
    unimplemented!();
}
pub fn to_uint8(argument: &JsValue) -> u8 {
    unimplemented!();
}
pub fn to_uint8_clamp(argument: &JsValue) -> u8 {
    unimplemented!();
}
pub fn to_bigint(argument: &JsValue) -> JsValue {
    unimplemented!();
}
pub fn string_to_bigint(argument: &str) -> JsBigInt {
    unimplemented!();
}
pub fn to_bigint64(argument: &JsValue) -> JsBigInt {
    unimplemented!();
}
pub fn to_biguint64(argument: &JsValue) -> JsBigInt {
    unimplemented!();
}
pub fn to_string(argument: &JsValue) -> String {
    unimplemented!();
}
pub fn to_object(argument: &JsValue) -> Box<dyn JsObject> {
    unimplemented!();
}
pub fn to_property_key(argument: &JsValue) -> JsKey {
    unimplemented!();
}
pub fn to_length(argument: &JsValue) -> u64 {
    unimplemented!();
}
pub fn canonical_numeric_index_string(argument: &JsValue) -> JsValue {
    unimplemented!();
}
pub fn to_index(value: &JsValue) -> u64 {
    unimplemented!();
}

pub fn require_object_coercible(argument: &JsValue) -> JsValue {
    unimplemented!();
}
pub fn is_array(argument: &JsValue) -> bool {
    unimplemented!();
}
pub fn is_callable(argument: &JsValue) -> bool {
    unimplemented!();
}
pub fn is_constructor(argument: &JsValue) -> bool {
    unimplemented!();
}
pub fn is_extensible(o: &Box<dyn JsObject>) -> bool {
    unimplemented!();
}
pub fn is_integer(argument: &JsValue) -> bool {
    unimplemented!();
}
pub fn is_non_negative_integer(argument: &JsValue) -> bool {
    unimplemented!();
}
pub fn is_property_key(argument: &JsValue) -> bool {
    unimplemented!();
}
pub fn is_regexp(argument: &JsValue) -> bool {
    unimplemented!();
}
pub fn is_string_prefix(p: &Utf16String, q: &Utf16String) -> bool {
    unimplemented!();
}
pub fn same_value(x: &JsValue, y: &JsValue) -> () {
    unimplemented!();
}
pub fn same_value_zero(x: &JsValue, y: &JsValue) -> () {
    unimplemented!();
}
pub fn same_value_non_numeric(x: &JsValue, y: &JsValue) -> () {
    unimplemented!();
}

// TODO: equality

pub fn make_basic_object(internal_slots_list: ()) -> JsHandle {
    unimplemented!();
}
pub fn get(o: &JsValue, p: &JsKey) -> JsValue {
    unimplemented!();
}
pub fn get_v(v: &JsValue, p: &JsKey) -> JsValue {
    unimplemented!();
}
pub fn set(o: &JsValue, p: &JsKey, v: &JsValue, throw: bool) -> bool {
    unimplemented!();
}
pub fn create_data_property(o: &JsValue, p: &JsKey, v: &JsValue) -> () {
    unimplemented!();
}
pub fn create_method_property(o: &JsValue, p: &JsKey, v: &JsValue) -> () {
    unimplemented!();
}
pub fn create_data_property_or_throw(o: &JsValue, p: &JsKey, v: &JsValue) -> () {
    unimplemented!();
}
pub fn define_property_or_throw(o: &JsValue, p: &JsKey, desc: &JsValue) -> bool {
    unimplemented!();
}
pub fn delete_property_or_throw(o: &JsValue, p: &JsKey) -> bool {
    unimplemented!();
}
pub fn get_method(v: &JsValue, p: &JsKey) -> () {
    unimplemented!();
}
pub fn has_property(o: &JsValue, p: &JsKey) -> bool {
    unimplemented!();
}
pub fn has_own_property(o: &JsValue, p: &JsKey) -> bool {
    unimplemented!();
}
pub fn call(f: &JsValue, v: &JsValue, arguments_list: Option<Vec<&JsValue>>) -> JsValue {
    unimplemented!();
}
pub fn construct(x: &JsValue, y: &JsValue) -> () {
    unimplemented!();
}
pub fn set_integrity_level(x: &JsValue, y: &JsValue) -> () {
    unimplemented!();
}
pub fn test_integrity_level(x: &JsValue, y: &JsValue) -> () {
    unimplemented!();
}
pub fn create_array_from_list(x: &JsValue, y: &JsValue) -> () {
    unimplemented!();
}
pub fn length_of_array_like(x: &JsValue, y: &JsValue) -> () {
    unimplemented!();
}
pub fn create_list_from_array_like(x: &JsValue, y: &JsValue) -> () {
    unimplemented!();
}
pub fn invoke(x: &JsValue, y: &JsValue) -> () {
    unimplemented!();
}
pub fn ordinary_has_instance(x: &JsValue, y: &JsValue) -> () {
    unimplemented!();
}
pub fn species_constructor(x: &JsValue, y: &JsValue) -> () {
    unimplemented!();
}
pub fn enumerable_own_property_names(x: &JsValue, y: &JsValue) -> () {
    unimplemented!();
}
pub fn get_function_realm(x: &JsValue, y: &JsValue) -> () {
    unimplemented!();
}
pub fn copy_data_properties(x: &JsValue, y: &JsValue) -> () {
    unimplemented!();
}

pub fn get_iterator(x: &JsValue, y: &JsValue) -> () {
    unimplemented!();
}
pub fn iterator_next(x: &JsValue, y: &JsValue) -> () {
    unimplemented!();
}
pub fn iterator_complete(x: &JsValue, y: &JsValue) -> () {
    unimplemented!();
}
pub fn iterator_value(x: &JsValue, y: &JsValue) -> () {
    unimplemented!();
}
pub fn iterator_step(x: &JsValue, y: &JsValue) -> () {
    unimplemented!();
}
pub fn iterator_close(x: &JsValue, y: &JsValue) -> () {
    unimplemented!();
}
pub fn async_iterator_close(x: &JsValue, y: &JsValue) -> () {
    unimplemented!();
}
pub fn create_iter_result_object(x: &JsValue, y: &JsValue) -> () {
    unimplemented!();
}
pub fn create_list_iterator_record(x: &JsValue, y: &JsValue) -> () {
    unimplemented!();
}
