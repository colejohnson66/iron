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
pub trait GarbageCollected {
    fn mark(&mut self);
    fn unmark(&mut self);
    fn marked(&self) -> bool;
}

pub type GcHandle = usize;

pub struct Gc<T>
where
    T: GarbageCollected,
{
    roots: Vec<GcHandle>,
    data: Vec<Option<Box<T>>>,
}

impl<T> Gc<T>
where
    T: GarbageCollected,
{
    pub fn new() -> Gc<T> {
        Gc {
            roots: vec![],
            data: vec![],
        }
    }

    /// Adds an object to the garbage heap and returns it's handle
    pub fn add(&mut self, obj: T) -> GcHandle {
        // TODO: look for a `None` in `self.data` and put there
        self.data.push(Some(Box::new(obj)));
        self.data.len() - 1
    }

    pub fn add_root(&mut self, obj: T) -> GcHandle {
        let handle = self.add(obj);
        self.roots.push(handle);
        handle
    }
}
