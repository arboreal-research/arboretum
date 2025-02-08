#include "datatype.h"

namespace arboretum {

const char *cpp_type_for_datatype(DataType dt) {
  switch (dt) {
    using enum DataType;
    case BOOL:
      return "bool";
    case I8:
      return "int8_t";
    case I16:
      return "int16_t";
    case I32:
      return "int32_t";
    case I64:
      return "int64_t";
    case U8:
      return "uint8_t";
    case U16:
      return "uint16_t";
    case U32:
      return "uint32_t";
    case U64:
      return "uint64_t";
    case F32:
      return "float";
    case F64:
      return "double";
    case STRING:
      return "const char*";
    default:
      return "void";
  }
}

const char *rust_arrow_type_for_datatype(DataType dt) {
  switch (dt) {
    using enum DataType;
    case BOOL:
      return "DataType::Boolean";
    case I8:
      return "DataType::Int8";
    case I16:
      return "DataType::Int16";
    case I32:
      return "DataType::Int32";
    case I64:
      return "DataType::Int64";
    case U8:
      return "DataType::UInt8";
    case U16:
      return "DataType::UInt16";
    case U32:
      return "DataType::UInt32";
    case U64:
      return "DataType::UInt64";
    case F32:
      return "DataType::Float32";
    case F64:
      return "DataType::Float64";
    case STRING:
      return "DataType::Utf8";
    default:
      return "!";
  }
}

const char *rust_type_for_datatype(DataType dt) {
  switch (dt) {
    using enum DataType;
    case BOOL:
      return "bool";
    case I8:
      return "i8";
    case I16:
      return "i16";
    case I32:
      return "i32";
    case I64:
      return "i64";
    case U8:
      return "u8";
    case U16:
      return "u16";
    case U32:
      return "u32";
    case U64:
      return "u64";
    case F32:
      return "f32";
    case F64:
      return "f64";
    case STRING:
      return "String";
    default:
      return "!";
  }
}

const char *rust_ffi_type_for_datatype(DataType dt) {
  switch (dt) {
    using enum DataType;
    case BOOL:
      return "bool";
    case I8:
      return "i8";
    case I16:
      return "i16";
    case I32:
      return "i32";
    case I64:
      return "i64";
    case U8:
      return "u8";
    case U16:
      return "u16";
    case U32:
      return "u32";
    case U64:
      return "u64";
    case F32:
      return "f32";
    case F64:
      return "f64";
    case STRING:
      return "*const c_char";
    default:
      return "!";
  }
}

const char *rust_builder_for_datatype(DataType dt) {
  switch (dt) {
    using enum DataType;
    case BOOL:
      return "arrow::array::builder::BooleanBuilder";
    case I8:
      return "arrow::array::builder::Int8Builder";
    case I16:
      return "arrow::array::builder::Int16Builder";
    case I32:
      return "arrow::array::builder::Int32Builder";
    case I64:
      return "arrow::array::builder::Int64Builder";
    case U8:
      return "arrow::array::builder::UInt8Builder";
    case U16:
      return "arrow::array::builder::UInt16Builder";
    case U32:
      return "arrow::array::builder::UInt32Builder";
    case U64:
      return "arrow::array::builder::UInt64Builder";
    case F32:
      return "arrow::array::builder::Float32Builder";
    case F64:
      return "arrow::array::builder::Float64Builder";
    case STRING:
      return "arrow::array::builder::StringBuilder";
    default:
      return "!";
  }
}

}