use crate::js::GcObjectFn;
use crate::vtable_impl;
use std::collections::HashMap;

pub struct Object {
    vtable: HashMap<String, GcObjectFn>,
}

vtable_impl!(Object);
