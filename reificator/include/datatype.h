#pragma once

namespace arboretum {

enum class DataType {
  BOOL,
  I8,
  I16,
  I32,
  I64,
  U8,
  U16,
  U32,
  U64,
  F32,
  F64,
  STRING,
};

/// Return the type for the arrow builder for the given datatype.
const char *cpp_type_for_datatype(DataType dt);
const char *rust_type_for_datatype(DataType dt);
const char *rust_arrow_type_for_datatype(DataType dt);
const char *rust_ffi_type_for_datatype(DataType dt);
const char *rust_builder_for_datatype(DataType dt);

}