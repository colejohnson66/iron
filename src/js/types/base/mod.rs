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
mod boolean;
mod error;
mod function;
mod object;

pub use self::boolean::Boolean;
pub use self::error::Error;
pub use self::function::Function;
pub use self::object::Object;

// TODO: Symbol type
// TODO: Error types: EvalError, RangeError, ReferenceError, SyntaxError, TypeError, URIError
// TODO: NativeError type
// TODO: Number type
// TODO: BigInt type
// TODO: Math object
// TODO: Date object
// TODO: String object
// TODO: Regex objects
// TODO: Array objects
// TODO: TypedArray types + objects
// TODO: Map objects
// TODO: Set objects
// TODO: WeakMap objects
// TODO: WeakSet objects
// TODO: ArrayBuffer objects
// TODO: SharedArrayBuffer objects
// TODO: DataView objects
// TODO: Atomics object
// TODO: JSON object
// TODO: section 25
// TODO: Reflect object
// TODO: Proxy objects
