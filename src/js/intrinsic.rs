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
// TODO: REMOVE!
#![allow(unused)]
// abstract operations as defined in section 7 of the ECMAScript specification
//use crate::gc::GcJsObject;
use crate::js::{GcJsObject, JsType, JsValue};
use gc::*;

pub fn type_(x: Gc<JsValue>) -> JsType {
    match x.as_ref() {
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

pub fn to_primitive(input: Gc<JsValue>, preferred_type: Option<&str>) -> JsValue {
    fn ordinary_to_primitive(o: GcJsObject, hint: &str) -> GcJsObject {
        unimplemented!();
    }
    unimplemented!();
}
pub fn to_boolean(o: GcJsObject, hint: &str) -> JsValue {
    unimplemented!();
}
pub fn to_numeric(value: GcJsObject) -> JsValue {
    unimplemented!();
}
pub fn to_number(argument: GcJsObject) -> JsValue {
    unimplemented!();
}
pub fn to_integer(argument: GcJsObject) -> JsValue {
    unimplemented!();
}
pub fn to_int32(argument: GcJsObject) -> i32 {
    unimplemented!();
}
pub fn to_uint32(argument: GcJsObject) -> u32 {
    unimplemented!();
}
pub fn to_int16(argument: GcJsObject) -> i16 {
    unimplemented!();
}
pub fn to_uint16(argument: GcJsObject) -> u16 {
    unimplemented!();
}
pub fn to_int8(argument: GcJsObject) -> i8 {
    unimplemented!();
}
pub fn to_uint8(argument: GcJsObject) -> u8 {
    unimplemented!();
}
pub fn to_uint8_clamp(argument: GcJsObject) -> u8 {
    unimplemented!();
}
//pub fn to_bigint(argument: GcJsObject) -> JsValue {
//    unimplemented!();
//}
//pub fn string_to_bigint(argument: &str) -> JsBigInt {
//    unimplemented!();
//}
//pub fn to_bigint64(argument: GcJsObject) -> JsBigInt {
//    unimplemented!();
//}
//pub fn to_biguint64(argument: GcJsObject) -> JsBigInt {
//    unimplemented!();
//}
pub fn to_string(argument: GcJsObject) -> String {
    unimplemented!();
}
pub fn to_object(argument: GcJsObject) -> GcJsObject {
    unimplemented!();
}
pub fn to_property_key(argument: GcJsObject) -> String {
    unimplemented!();
}
pub fn to_length(argument: GcJsObject) -> u64 {
    unimplemented!();
}
pub fn canonical_numeric_index_string(argument: GcJsObject) -> JsValue {
    unimplemented!();
}
pub fn to_index(value: GcJsObject) -> u64 {
    unimplemented!();
}

pub fn require_object_coercible(argument: GcJsObject) -> JsValue {
    unimplemented!();
}
pub fn is_array(argument: GcJsObject) -> bool {
    unimplemented!();
}
pub fn is_callable(argument: GcJsObject) -> bool {
    unimplemented!();
}
pub fn is_constructor(argument: GcJsObject) -> bool {
    unimplemented!();
}
pub fn is_extensible(o: GcJsObject) -> bool {
    unimplemented!();
}
pub fn is_integer(argument: GcJsObject) -> bool {
    unimplemented!();
}
//pub fn is_non_negative_integer(argument: GcJsObject) -> bool {
//    unimplemented!();
//}
pub fn is_property_key(argument: GcJsObject) -> bool {
    unimplemented!();
}
pub fn is_regexp(argument: GcJsObject) -> bool {
    unimplemented!();
}
pub fn is_string_prefix(p: GcJsObject, q: GcJsObject) -> bool {
    unimplemented!();
}
pub fn same_value(x: GcJsObject, y: GcJsObject) -> () {
    unimplemented!();
}
pub fn same_value_zero(x: GcJsObject, y: GcJsObject) -> () {
    unimplemented!();
}
pub fn same_value_non_numeric(x: GcJsObject, y: GcJsObject) -> () {
    unimplemented!();
}

// TODO: equality

pub fn get(o: GcJsObject, p: GcJsObject) -> JsValue {
    unimplemented!();
}
pub fn get_v(v: GcJsObject, p: GcJsObject) -> JsValue {
    unimplemented!();
}
pub fn set(o: GcJsObject, p: GcJsObject, v: GcJsObject, throw: bool) -> bool {
    unimplemented!();
}
pub fn create_data_property(o: GcJsObject, p: GcJsObject, v: GcJsObject) -> () {
    unimplemented!();
}
pub fn create_method_property(o: GcJsObject, p: GcJsObject, v: GcJsObject) -> () {
    unimplemented!();
}
pub fn create_data_property_or_throw(o: GcJsObject, p: GcJsObject, v: GcJsObject) -> () {
    unimplemented!();
}
pub fn define_property_or_throw(o: GcJsObject, p: GcJsObject, desc: GcJsObject) -> bool {
    unimplemented!();
}
pub fn delete_property_or_throw(o: GcJsObject, p: GcJsObject) -> bool {
    unimplemented!();
}
pub fn get_method(v: GcJsObject, p: GcJsObject) -> () {
    unimplemented!();
}
pub fn has_property(o: GcJsObject, p: GcJsObject) -> bool {
    unimplemented!();
}
pub fn has_own_property(o: GcJsObject, p: GcJsObject) -> bool {
    unimplemented!();
}
pub fn call(f: GcJsObject, v: GcJsObject, arguments_list: Option<Vec<GcJsObject>>) -> GcJsObject {
    unimplemented!();
}
pub fn construct(
    f: GcJsObject,
    arguments_list: Option<Vec<GcJsObject>>,
    new_target: Option<GcJsObject>,
) -> () {
    unimplemented!();
}
pub fn set_integrity_level(o: GcJsObject, level: &str) -> bool {
    unimplemented!();
}
pub fn test_integrity_level(o: GcJsObject, level: &str) -> bool {
    unimplemented!();
}
pub fn create_array_from_list(elements: Vec<GcJsObject>) -> () {
    unimplemented!();
}
pub fn create_list_from_array_like(obj: GcJsObject, element_types: JsType) -> () {
    unimplemented!();
}
pub fn invoke(v: GcJsObject, p: GcJsObject, arguments_list: Option<Vec<GcJsObject>>) -> () {
    unimplemented!();
}
pub fn ordinary_has_instance(c: GcJsObject, o: GcJsObject) -> bool {
    unimplemented!();
}
// TODO: is `default_constructor` and return type correct?
pub fn species_constructor(o: GcJsObject, default_constructor: GcJsObject) -> GcJsObject {
    unimplemented!();
}
pub fn enumerable_own_property_names(o: GcJsObject, kind: &str) -> Vec<GcJsObject> {
    unimplemented!();
}
// TODO: return type should be JsRealm
pub fn get_function_realm(obj: GcJsObject) -> () {
    unimplemented!();
}
pub fn copy_data_properties(
    target: GcJsObject,
    source: GcJsObject,
    excluded_items: &Vec<GcJsObject>,
) -> GcJsObject {
    unimplemented!();
}

// TODO: return type should be a Record
pub fn get_iterator(obj: GcJsObject, hint: Option<&str>, method: Option<GcJsObject>) -> () {
    unimplemented!();
}
pub fn iterator_next(iterator_record: (), value: Option<GcJsObject>) -> GcJsObject {
    unimplemented!();
}
pub fn iterator_complete(iter_result: GcJsObject) -> bool {
    unimplemented!();
}
pub fn iterator_value(iter_result: GcJsObject) -> GcJsObject {
    unimplemented!();
}
// TODO: iterator_record is a Record
pub fn iterator_step(iterator_record: ()) -> GcJsObject {
    unimplemented!();
}
// TODO: iterator_record is a Record
// TODO: completion is a Record
// TODO: return type is a Record
pub fn iterator_close(iterator_record: (), completion: ()) -> () {
    unimplemented!();
}
// TODO: iterator_record is a Record
// TODO: completion is a Record
// TODO: return type is a Record
pub fn async_iterator_close(iterator_record: (), completion: ()) -> () {
    unimplemented!();
}
pub fn create_iter_result_object(value: &str, done: bool) -> GcJsObject {
    unimplemented!();
}
// TODO: is list type correct?
// TODO: return type is a Record
pub fn create_list_iterator_record(list: Vec<GcJsObject>) -> () {
    unimplemented!();
}
