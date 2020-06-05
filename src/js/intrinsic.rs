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
use crate::gc::GcHandle;
use crate::js::{JsBigInt, JsKey, JsObject, JsSymbol, JsType, JsValue};
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
    unimplemented!();
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
pub fn construct(
    f: &JsValue,
    arguments_list: Option<Vec<&JsValue>>,
    new_target: Option<&JsValue>,
) -> () {
    unimplemented!();
}
pub fn set_integrity_level(o: &Box<dyn JsObject>, level: &str) -> bool {
    unimplemented!();
}
pub fn test_integrity_level(o: &Box<dyn JsObject>, level: &str) -> bool {
    unimplemented!();
}
pub fn create_array_from_list(elements: Vec<&JsValue>) -> () {
    unimplemented!();
}
pub fn create_list_from_array_like(obj: JsHandle, element_types: JsType) -> () {
    unimplemented!();
}
pub fn invoke(v: JsHandle, p: JsKey, arguments_list: Option<Vec<&JsValue>>) -> () {
    unimplemented!();
}
pub fn ordinary_has_instance(c: GcHandle, o: GcHandle) -> bool {
    unimplemented!();
}
// TODO: is `default_constructor` and return type correct?
pub fn species_constructor(o: GcHandle, default_constructor: GcHandle) -> &Box<dyn JsObject> {
    unimplemented!();
}
pub fn enumerable_own_property_names(o: GcHandle, kind: &str) -> Vec<GcHandle> {
    unimplemented!();
}
// TODO: return type should be JsRealm
pub fn get_function_realm(obj: GcHandle) -> () {
    unimplemented!();
}
pub fn copy_data_properties(
    target: GcHandle,
    source: GcHandle,
    excluded_items: &Vec<GcHandle>,
) -> GcHandle {
    unimplemented!();
}

// TODO: return type should be a Record
pub fn get_iterator(obj: GcHandle, hint: Option<&str>, method: Option<GcHandle>) -> () {
    unimplemented!();
}
pub fn iterator_next(iterator_record: (), value: Option<GcHandle>) -> GcHandle {
    unimplemented!();
}
pub fn iterator_complete(iter_result: GcHandle) -> bool {
    unimplemented!();
}
pub fn iterator_value(iter_result: GcHandle) -> GcHandle {
    unimplemented!();
}
// TODO: iterator_record is a Record
pub fn iterator_step(iterator_record: ()) -> GcHandle {
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
pub fn create_iter_result_object(value: &str, done: bool) -> GcHandle {
    unimplemented!();
}
// TODO: is list type correct?
// TODO: return type is a Record
pub fn create_list_iterator_record(list: Vec<GcHandle>) -> () {
    unimplemented!();
}
