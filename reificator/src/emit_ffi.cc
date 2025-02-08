#include <filesystem>
#include <string>
#include <vector>

#include "model.h"
#include "partially_generated_file.h"

namespace arboretum {

namespace {

void emit_ffi_table_cpp(Table& table, std::ostream& os) {
  os << "void arboretum_emit_" << table.name << "(";

  for (size_t col_idx = 0; col_idx < table.fields.size(); ++col_idx) {
    if (col_idx != 0) {
      os << ", ";
    }

    os << cpp_type_for_datatype(table.fields[col_idx].datatype) << " "
       << table.fields[col_idx].name;
  }

  os << ");\n";
}

void emit_io_tablebuilder_rust(Table& table, std::ostream& os, size_t idx) {
  os << "pub struct TableBuilder" << idx << " {\n";
  for (size_t col_idx = 0; col_idx < table.fields.size(); ++col_idx) {
    os << "  c" << col_idx << ": "
       << rust_builder_for_datatype(table.fields[col_idx].datatype) << ", // "
       << table.fields[col_idx].name << "\n";
  }
  os << "  writer: ParquetWriter,\n";
  os << "}\n\n";

  os << "impl TableBuilder" << idx << " {\n";
  os << "  fn schema() -> Arc<Schema> {\n";
  os << "    Arc::new(Schema::new(vec![\n";
  for (size_t col_idx = 0; col_idx < table.fields.size(); ++col_idx) {
    os << "      Field::new(\"" << table.fields[col_idx].name << "\", "
       << rust_arrow_type_for_datatype(table.fields[col_idx].datatype)
       << ", false),\n";
  }
  os << "    ]))\n";
  os << "  }\n\n";

  os << "  pub fn new(output_path: impl AsRef<Path>, partition_size: usize) -> "
        "Self {\n";
  os << "    Self {\n";
  for (size_t col_idx = 0; col_idx < table.fields.size(); ++col_idx) {
    os << "      c" << col_idx << ": "
       << rust_builder_for_datatype(table.fields[col_idx].datatype)
       << "::new(),\n";
  }
  os << "        writer: ParquetWriter::new(output_path, partition_size, "
        "Self::schema()),\n";
  os << "    }\n";
  os << "  }\n\n";

  os << "  pub fn reset(&mut self) {\n";
  for (size_t col_idx = 0; col_idx < table.fields.size(); ++col_idx) {
    os << "    self.c" << col_idx << " = "
       << rust_builder_for_datatype(table.fields[col_idx].datatype)
       << "::new();\n";
  }
  os << "  }\n\n";

  os << "  pub async fn push(&mut self, record: Record" << idx
     << ") -> Result<()> {\n";
  for (size_t col_idx = 0; col_idx < table.fields.size(); ++col_idx) {
    os << "    self.c" << col_idx << ".append_value(record.c" << col_idx
       << ");\n";
  }
  os << "    if self.writer.add_row() {\n";
  os << "      self.flush().await?;\n";
  os << "    }\n";
  os << "    Ok(())\n";
  os << "  }\n\n";

  os << "  pub async fn flush(&mut self) -> Result<()> {\n";
  os << "    let batch = RecordBatch::try_new("
     << "self.writer.schema.clone(), vec![\n";
  for (size_t col_idx = 0; col_idx < table.fields.size(); ++col_idx) {
    os << "        Arc::new(self.c" << col_idx << ".finish()),\n";
  }
  os << "    ], )?;\n";
  os << "    self.writer.flush(batch).await?;\n";
  os << "    self.reset();\n";
  os << "    Ok(())\n";
  os << "  }\n\n";

  os << "  pub async fn cancel(&mut self) -> Result<()> {\n";
  os << "    self.writer.cancel().await?;\n";
  os << "    Ok(())\n";
  os << "  }\n\n";

  os << "}\n\n";
}

void emit_ffi_table_rust(Table& table, std::ostream& os, size_t idx) {
  os << "// " << table.name << "\n";
  os << "#[derive(Debug, Serialize, Deserialize)]\n";
  os << "pub struct Record" << idx << " {\n";
  for (size_t col_idx = 0; col_idx < table.fields.size(); ++col_idx) {
    os << "  pub c" << col_idx << ": "
       << rust_type_for_datatype(table.fields[col_idx].datatype) << ", // "
       << table.fields[col_idx].name << "\n";
  }
  os << "}\n\n";

  os << "#[no_mangle]\n";
  os << "pub extern \"C\" fn " << "arboretum_emit_" << table.name << "(";
  for (size_t col_idx = 0; col_idx < table.fields.size(); ++col_idx) {
    if (col_idx != 0) {
      os << ", ";
    }

    os << "c" << col_idx << ": "
       << rust_ffi_type_for_datatype(table.fields[col_idx].datatype);
  }
  os << ") {\n";
  os << "  let sink = unsafe { RECORD_SINK.as_ref() }.unwrap();\n";
  for (size_t col_idx = 0; col_idx < table.fields.size(); ++col_idx) {
    switch (table.fields[col_idx].datatype) {
      case DataType::STRING: {
        os << "  let c" << col_idx << " = unsafe { CStr::from_ptr(c" << col_idx
           << ") }.to_string_lossy().to_string();\n";
      } break;
      default:
        break;
    }
  }
  os << "  sink(FfiMessage::Record(Record::Record" << idx << "(Record" << idx
     << "{";
  for (size_t col_idx = 0; col_idx < table.fields.size(); ++col_idx) {
    os << "c" << col_idx << ", ";
  }
  os << "})));\n";

  os << "}\n\n";
}

void emit_ffi_rust(Model& model, const std::string& reify_rs_dir) {
  std::filesystem::path reify_rs_dir_path(reify_rs_dir);
  auto reify_rs_ffi =
      PartiallyGeneratedFile::Read(reify_rs_dir_path / "src" / "ffi.rs");
  assert(reify_rs_ffi.Write([&](std::ostream& os) -> bool {
    os << "#[derive(Debug, Serialize, Deserialize)]\n";
    os << "pub enum Record {\n";
    for (size_t i = 0; i < model.tables.size(); ++i) {
      os << "  Record" << i << "(Record" << i << "),\n";
    }
    os << "}\n\n";

    os << "pub static mut RECORD_SINK: Option<Box<dyn Fn(FfiMessage)>> = "
          "None;\n\n";

    for (size_t i = 0; i < model.tables.size(); ++i) {
      emit_ffi_table_rust(*model.tables[i], os, i);
    }
    return true;
  }));
}

void emit_ffi_cpp(Model& model, const std::string& arboretum_ffi_dir) {
  std::filesystem::path arboretum_ffi_dir_path(arboretum_ffi_dir);
  auto arboretum_ffi = PartiallyGeneratedFile::Read(arboretum_ffi_dir_path /
                                                    "src" / "arboretum_ffi.h");
  assert(arboretum_ffi.Write([&](std::ostream& os) -> bool {
    std::map<std::string, std::shared_ptr<Table>> ordered_tables;
    for (auto& table : model.tables) {
      ordered_tables.insert(std::make_pair(table->name, table));
    }
    for (auto& [_, table] : ordered_tables) {
      emit_ffi_table_cpp(*table, os);
    }
    return true;
  }));
}

}  // namespace

void emit_io(Model& model, const std::string& reify_rs_dir) {
  std::filesystem::path reify_rs_dir_path(reify_rs_dir);
  auto reify_rs_ffi =
      PartiallyGeneratedFile::Read(reify_rs_dir_path / "src" / "io.rs");
  assert(reify_rs_ffi.Write([&](std::ostream& os) -> bool {
    os << "pub struct TableBuilders {\n";
    for (size_t i = 0; i < model.tables.size(); ++i) {
      os << "  t" << i << ": TableBuilder" << i << ",\n";
    }
    os << "}\n\n";

    os << "impl TableBuilders {\n";
    os << "  pub fn new(output_path: impl AsRef<Path>, partition_size: usize) "
          "-> "
          "Self {\n";
    os << "    Self {\n";
    for (size_t i = 0; i < model.tables.size(); ++i) {
      os << "      t" << i << ": TableBuilder" << i
         << "::new(output_path.as_ref().join(\"" << model.tables[i]->name
         << "\"), "
         << "partition_size),\n";
    }
    os << "    }\n";
    os << "  }\n";

    os << "  pub async fn push(&mut self, record: Record) -> Result<()> {\n";

    os << "    match record {\n";
    for (size_t i = 0; i < model.tables.size(); ++i) {
      os << "      Record::Record" << i << "(record) => self.t" << i
         << ".push(record).await,\n";
    }
    os << "    }\n";
    os << "  }\n\n";

    os << "  pub async fn flush(&mut self) -> Result<()> {\n";
    for (size_t i = 0; i < model.tables.size(); ++i) {
      os << "    self.t" << i << ".flush().await?;\n";
    }
    os << "    Ok(())\n";
    os << "  }\n";

    os << "  pub async fn cancel(&mut self) -> Result<()> {\n";
    for (size_t i = 0; i < model.tables.size(); ++i) {
      os << "    self.t" << i << ".cancel().await?;\n";
    }
    os << "    Ok(())\n";
    os << "  }\n";
    os << "}\n\n";

    for (size_t i = 0; i < model.tables.size(); ++i) {
      emit_io_tablebuilder_rust(*model.tables[i], os, i);
    }

    return true;
  }));
}

void emit_ffi(Model& model, const std::string& arboretum_ffi_dir,
              const std::string& reify_rs_dir) {
  emit_ffi_cpp(model, arboretum_ffi_dir);
  emit_ffi_rust(model, reify_rs_dir);
}

}  // namespace arboretum