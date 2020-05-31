/* ============================================================================
 * File:   ArrayBufferView.rs
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
// typedef (Int8Array or Int16Array or Int32Array or
//    Uint8Array or Uint16Array or Uint32Array or Uint8ClampedArray or
//    Float32Array or Float64Array or DataView) ArrayBufferView;
pub enum ArrayBufferView {
    Int8Array(()),
    Int16Array(()),
    Int32Array(()),
    Uint8Array(()),
    Uint16Array(()),
    Uint32Array(()),
    Uint8ClampedArray(()),
    Float32Array(()),
    Float64Array(()),
    DataView(()),
}
