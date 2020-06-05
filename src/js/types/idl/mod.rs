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
mod array_buffer_view;

pub use array_buffer_view::ArrayBufferView;

pub type StringUtf16 = Vec<u16>;

// NOTE: Nullable types are not defined here <https://heycam.github.io/webidl/#idl-nullable-type>
pub enum IdlType {
    // `any` <https://heycam.github.io/webidl/#idl-any>
    Any(Box<IdlType>),
    // `void` <https://heycam.github.io/webidl/#idl-void>
    Void,
    // `boolean` <https://heycam.github.io/webidl/#idl-boolean>
    Boolean(bool),
    // `byte` <https://heycam.github.io/webidl/#idl-byte>
    Byte(i8),
    // `octet` <https://heycam.github.io/webidl/#idl-octet>
    Octet(u8),
    // `short` <https://heycam.github.io/webidl/#idl-short>
    Short(i16),
    // `unsigned short` <https://heycam.github.io/webidl/#idl-unsigned-short>
    UnsignedShort(u16),
    // `long` <https://heycam.github.io/webidl/#idl-long>
    Long(i32),
    // `unsigned long` <https://heycam.github.io/webidl/#idl-unsigned-long>
    UnsignedLong(u32),
    // `long long` <https://heycam.github.io/webidl/#idl-long-long>
    LongLong(i64),
    // `unsigned long long` <https://heycam.github.io/webidl/#idl-unsigned-long-long>
    UnsignedLongLong(u64),
    // `float` <https://heycam.github.io/webidl/#idl-float>
    Float(f32),
    // `unrestricted float` <https://heycam.github.io/webidl/#idl-unrestricted-float>
    UnrestrictedFloat(f32),
    // `double` <https://heycam.github.io/webidl/#idl-double>
    Double(f64),
    // `unrestricted double` <https://heycam.github.io/webidl/#idl-unrestricted-double>
    UnrestrictedDouble(f64),
    // `DOMString` <https://heycam.github.io/webidl/#idl-DOMString>
    DomString(StringUtf16),
    // `ByteString` <https://heycam.github.io/webidl/#idl-ByteString>
    ByteString(String),
    // `USVString` <https://heycam.github.io/webidl/#idl-USVString>
    UsvString(StringUtf16),
    // `object` <https://heycam.github.io/webidl/#idl-object>
    Object(()),
    // `symbol` <https://heycam.github.io/webidl/#idl-symbol>
    Symbol(()),
    // interface types <https://heycam.github.io/webidl/#idl-interface>
    Interface(()),
    // callback interface types <https://heycam.github.io/webidl/#idl-callback-interface>
    Callback(()),
    // dictionary types <https://heycam.github.io/webidl/#idl-dictionary>
    Dictionary(()),
    // enumeration types <https://heycam.github.io/webidl/#idl-enumeration>
    Enum(()),
    // callback function types <https://heycam.github.io/webidl/#idl-callback-function>
    CallbackFn(()),
    // sequence types `sequence<T>` <https://heycam.github.io/webidl/#idl-sequence>
    Sequence(()),
    // record types `record<K,V>` <https://heycam.github.io/webidl/#idl-record>
    Record(()),
    // promise types `Promise<T>` <https://heycam.github.io/webidl/#idl-promise>
    Promise(()),
    // union types <https://heycam.github.io/webidl/#idl-union>
    Union(()),
    // buffer source types <https://heycam.github.io/webidl/#idl-buffer-source-types>
    // views are not defined here
    ArrayBuffer(()),
    // frozen array types `FrozenArray<T>` <https://heycam.github.io/webidl/#idl-frozen-array>
    FrozenArray(()),
    // observable array types `ObservableArray<T>` <https://heycam.github.io/webidl/#idl-observable-array>
    ObservableArray(()),
}

// TODO: is this needed?
// <https://heycam.github.io/webidl/#es-extended-attributes>
pub enum IdlAttr {
    AllowShared,
    Clamp,
    Default,
    EnforceRange,
    Exposed(()), // TODO:
    Global(()),  // TODO:
    NewObject,
    PutsForward(String),
    Replaceable,
    SameObject,
    SecureContext,
    Unscopable,

    // legacy attributes
    LegacyFactoryFunction(()),
    LegacyLenientSetter,
    LegacyLenientThis,
    LegacyNamespace,
    LegacyNoInterfaceObject,
    LegacyNullToEmptyString,
    LegacyOverrideBuiltIns,
    LegacyTreatNonObjectAsNull,
    LegacyUnenumerableNamedProperties,
    LegacyUnforgeable,
    LegacyWindowAlias,
}

pub trait IdlObject {
    fn type_name(&self) -> &str;

    fn constant(&mut self, name: &str) -> IdlType;

    fn attr(&mut self, name: &str) -> IdlType;
    fn set_attr(&mut self, name: &str, value: IdlType);

    fn static_attr(name: &str) -> IdlType;
    fn set_static_attr(name: &str, value: IdlType);

    fn op(&mut self, name: &str, params: Vec<IdlType>) -> IdlType;
    fn static_op(name: &str, params: Vec<IdlType>) -> IdlType;

    // TODO: toJSON?

    fn get(&mut self, name: &str) -> IdlType;
    fn set(&mut self, name: &str, value: IdlType);
    fn delete(&mut self, name: &str);
    fn stringify(&mut self) -> StringUtf16;

    // TODO: iterable<V> and iterable<K, V>

    // TODO: async

    // TODO: maplike<K, V>

    // TODO: setlike<T>
}
