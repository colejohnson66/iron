use crate::js::GcVtableFn;
use crate::vtable_impl;
use std::collections::HashMap;

pub struct Object {
    vtable: HashMap<String, GcVtableFn>,
}

vtable_impl!(Object);
