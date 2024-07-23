use std::{
    collections::BTreeMap,
    ffi::{c_char, CStr},
};
use surrealdb::sql::{Thing, Value};

pub struct ObjectBuilder {
    pub fields: BTreeMap<String, Value>,
}

impl ObjectBuilder {
    pub fn new() -> Self {
        Self {
            fields: BTreeMap::new(),
        }
    }
}

impl From<ObjectBuilder> for BTreeMap<String, Value> {
    fn from(value: ObjectBuilder) -> Self {
        value.fields
    }
}

#[no_mangle]
pub extern "C" fn arboretum_object_builder_new() -> *mut ObjectBuilder {
    Box::into_raw(ObjectBuilder::new().into())
}

#[no_mangle]
pub extern "C" fn arboretum_object_builder_destroy(builder: *mut ObjectBuilder) {
    if !builder.is_null() {
        let _ = unsafe { Box::from_raw(builder) };
    }
}

#[no_mangle]
pub extern "C" fn arboretum_object_builder_set_bool(
    builder: *mut ObjectBuilder,
    field: *const c_char,
    value: u8,
) {
    let builder = unsafe { &mut *builder };
    let field = unsafe { CStr::from_ptr(field) }
        .to_string_lossy()
        .to_string();
    let value = value != 0;
    builder.fields.insert(field, value.into());
}

#[no_mangle]
pub extern "C" fn arboretum_object_builder_set_i8(
    builder: *mut ObjectBuilder,
    field: *const c_char,
    value: i8,
) {
    let builder = unsafe { &mut *builder };
    let field = unsafe { CStr::from_ptr(field) }
        .to_string_lossy()
        .to_string();
    builder.fields.insert(field, value.into());
}

#[no_mangle]
pub extern "C" fn arboretum_object_builder_set_i16(
    builder: *mut ObjectBuilder,
    field: *const c_char,
    value: i16,
) {
    let builder = unsafe { &mut *builder };
    let field = unsafe { CStr::from_ptr(field) }
        .to_string_lossy()
        .to_string();
    builder.fields.insert(field, value.into());
}

#[no_mangle]
pub extern "C" fn arboretum_object_builder_set_i32(
    builder: *mut ObjectBuilder,
    field: *const c_char,
    value: i32,
) {
    let builder = unsafe { &mut *builder };
    let field = unsafe { CStr::from_ptr(field) }
        .to_string_lossy()
        .to_string();
    builder.fields.insert(field, value.into());
}

#[no_mangle]
pub extern "C" fn arboretum_object_builder_set_i64(
    builder: *mut ObjectBuilder,
    field: *const c_char,
    value: i64,
) {
    let builder = unsafe { &mut *builder };
    let field = unsafe { CStr::from_ptr(field) }
        .to_string_lossy()
        .to_string();
    builder.fields.insert(field, value.into());
}

#[no_mangle]
pub extern "C" fn arboretum_object_builder_set_u8(
    builder: *mut ObjectBuilder,
    field: *const c_char,
    value: u8,
) {
    let builder = unsafe { &mut *builder };
    let field = unsafe { CStr::from_ptr(field) }
        .to_string_lossy()
        .to_string();
    builder.fields.insert(field, value.into());
}

#[no_mangle]
pub extern "C" fn arboretum_object_builder_set_u16(
    builder: *mut ObjectBuilder,
    field: *const c_char,
    value: u16,
) {
    let builder = unsafe { &mut *builder };
    let field = unsafe { CStr::from_ptr(field) }
        .to_string_lossy()
        .to_string();
    builder.fields.insert(field, value.into());
}

#[no_mangle]
pub extern "C" fn arboretum_object_builder_set_u32(
    builder: *mut ObjectBuilder,
    field: *const c_char,
    value: u32,
) {
    let builder = unsafe { &mut *builder };
    let field = unsafe { CStr::from_ptr(field) }
        .to_string_lossy()
        .to_string();
    builder.fields.insert(field, value.into());
}

#[no_mangle]
pub extern "C" fn arboretum_object_builder_set_u64(
    builder: *mut ObjectBuilder,
    field: *const c_char,
    value: u64,
) {
    let builder = unsafe { &mut *builder };
    let field = unsafe { CStr::from_ptr(field) }
        .to_string_lossy()
        .to_string();
    builder.fields.insert(field, value.into());
}

#[no_mangle]
pub extern "C" fn arboretum_object_builder_set_f32(
    builder: *mut ObjectBuilder,
    field: *const c_char,
    value: f32,
) {
    let builder = unsafe { &mut *builder };
    let field = unsafe { CStr::from_ptr(field) }
        .to_string_lossy()
        .to_string();
    builder.fields.insert(field, value.into());
}

#[no_mangle]
pub extern "C" fn arboretum_object_builder_set_f64(
    builder: *mut ObjectBuilder,
    field: *const c_char,
    value: f64,
) {
    let builder = unsafe { &mut *builder };
    let field = unsafe { CStr::from_ptr(field) }
        .to_string_lossy()
        .to_string();
    builder.fields.insert(field, value.into());
}

#[no_mangle]
pub extern "C" fn arboretum_object_builder_set_string(
    builder: *mut ObjectBuilder,
    field: *const c_char,
    value: *const c_char,
) {
    let builder = unsafe { &mut *builder };
    let field = unsafe { CStr::from_ptr(field) }
        .to_string_lossy()
        .to_string();
    let value = unsafe { CStr::from_ptr(value) }
        .to_string_lossy()
        .to_string();
    builder.fields.insert(field, value.into());
}

#[no_mangle]
pub extern "C" fn arboretum_object_builder_set_record(
    builder: *mut ObjectBuilder,
    field: *const c_char,
    thing: *const Thing,
) {
    let builder = unsafe { &mut *builder };
    let field = unsafe { CStr::from_ptr(field) }
        .to_string_lossy()
        .to_string();
    let thing = unsafe { &*thing }.clone();
    builder.fields.insert(field, thing.into());
}
