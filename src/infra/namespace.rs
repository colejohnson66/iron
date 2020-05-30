/* ============================================================================
 * File:   namespace.rs
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
// Implements <https://infra.spec.whatwg.org/> section 8

#[derive(Copy, Clone)]
pub enum Namespace {
    Html,
    MathML,
    Svg,
    XLink,
    Xml,
    Xmlns,
}

impl Namespace {
    pub fn from_str(string: &str) -> Option<Namespace> {
        match string {
            "http://www.w3.org/1999/xhtml" => Some(Namespace::Html),
            "http://www.w3.org/1998/Math/MathML" => Some(Namespace::MathML),
            "http://www.w3.org/2000/svg" => Some(Namespace::Svg),
            "http://www.w3.org/1999/xlink" => Some(Namespace::XLink),
            "http://www.w3.org/XML/1998/namespace" => Some(Namespace::Xml),
            "http://www.w3.org/2000/xmlns/" => Some(Namespace::Xmlns),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Namespace::Html => "http://www.w3.org/1999/xhtml",
            Namespace::MathML => "http://www.w3.org/1998/Math/MathML",
            Namespace::Svg => "http://www.w3.org/2000/svg",
            Namespace::XLink => "http://www.w3.org/1999/xlink",
            Namespace::Xml => "http://www.w3.org/XML/1998/namespace",
            Namespace::Xmlns => "http://www.w3.org/2000/xmlns/",
        }
    }
}
