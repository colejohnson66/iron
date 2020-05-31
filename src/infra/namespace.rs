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
use std::cmp::PartialEq;
// Implements <https://infra.spec.whatwg.org/> section 8

pub enum Namespace {
    Html,
    MathML,
    Svg,
    XLink,
    Xml,
    Xmlns,
    Unknown(String),
}

impl Namespace {
    pub fn from_str(string: &str) -> Namespace {
        match string {
            "http://www.w3.org/1999/xhtml" => Namespace::Html,
            "http://www.w3.org/1998/Math/MathML" => Namespace::MathML,
            "http://www.w3.org/2000/svg" => Namespace::Svg,
            "http://www.w3.org/1999/xlink" => Namespace::XLink,
            "http://www.w3.org/XML/1998/namespace" => Namespace::Xml,
            "http://www.w3.org/2000/xmlns/" => Namespace::Xmlns,
            _ => Namespace::Unknown(string.into()),
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            Namespace::Html => "http://www.w3.org/1999/xhtml",
            Namespace::MathML => "http://www.w3.org/1998/Math/MathML",
            Namespace::Svg => "http://www.w3.org/2000/svg",
            Namespace::XLink => "http://www.w3.org/1999/xlink",
            Namespace::Xml => "http://www.w3.org/XML/1998/namespace",
            Namespace::Xmlns => "http://www.w3.org/2000/xmlns/",
            Namespace::Unknown(url) => url,
        }
    }
}

impl Clone for Namespace {
    fn clone(&self) -> Namespace {
        match self {
            Namespace::Html => Namespace::Html,
            Namespace::MathML => Namespace::MathML,
            Namespace::Svg => Namespace::Svg,
            Namespace::XLink => Namespace::XLink,
            Namespace::Xml => Namespace::Xml,
            Namespace::Xmlns => Namespace::Xmlns,
            Namespace::Unknown(url) => Namespace::Unknown(url.clone()),
        }
    }
}

impl PartialEq for Namespace {
    fn eq(&self, other: &Namespace) -> bool {
        match self {
            Namespace::Html => match other {
                Namespace::Html => true,
                _ => false,
            },
            Namespace::MathML => match other {
                Namespace::MathML => true,
                _ => false,
            },
            Namespace::Svg => match other {
                Namespace::Svg => true,
                _ => false,
            },
            Namespace::XLink => match other {
                Namespace::XLink => true,
                _ => false,
            },
            Namespace::Xml => match other {
                Namespace::Xml => true,
                _ => false,
            },
            Namespace::Xmlns => match other {
                Namespace::Xmlns => true,
                _ => false,
            },
            Namespace::Unknown(url) => match other {
                Namespace::Unknown(url2) => url == url2,
                _ => false,
            },
        }
    }
}
