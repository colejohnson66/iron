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
mod detail;
pub mod tokenizer;

// <https://tc39.es/ecma262/#sec-ecmascript-language-types>
pub enum JsType {
    Undefined,
    Null,
    Boolean(bool),
    String(Vec<u16>),
    Symbol(Box<dyn JsSymbol>),
    Number(f64),
    BigInt(()),
    Object(Box<dyn JsObject>),
}

pub trait JsSymbol {}

pub trait JsObject {}

// TODO: implement section 7 <https://tc39.es/ecma262/#sec-abstract-operations>