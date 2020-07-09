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
pub mod gcobject;

use std::collections::HashMap;

pub struct Object {
    pub data: ObjectData,
    internal_slots: HashMap<String, ()>,
    properties: HashMap<(), ()>,
    symbol_properties: HashMap<u32, ()>,
    state: Option<()>,
    extensible: bool,
}

pub enum ObjectData {
    Array,
    BigInt(()),
    Boolean(bool),
    Function(()),
    String(()),
    Number(f64),
    Symbol(()),
    Error,
    Ordinary,
}
