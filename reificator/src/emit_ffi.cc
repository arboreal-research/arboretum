#include <filesystem>
#include <string>
#include <vector>

#include "datatype.h"
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
  // Generate RecordN struct
  os << "// " << table.name << "\n";
  os << "#[derive(Debug, Serialize, Deserialize)]\n";
  os << "pub struct Record" << idx << " {\n";
  for (size_t col_idx = 0; col_idx < table.fields.size(); ++col_idx) {
    os << "  pub c" << col_idx << ": "
       << rust_type_for_datatype(table.fields[col_idx].datatype) << ", // "
       << table.fields[col_idx].name << "\n";
  }
  os << "}\n\n";

  os << "pub struct TableBuilder" << idx << " {\n";
  os << "  db_url: String,\n";
  os << "  records: Vec<Record" << idx << ">,\n";
  os << "}\n\n";

  os << "impl TableBuilder" << idx << " {\n";
  os << "  pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {\n";
  os << "    Self {\n";
  os << "      db_url: db_url.as_ref().to_owned(),\n";
  os << "      records: Vec::new(),\n";
  os << "    }\n";
  os << "  }\n\n";

  // Build column list for INSERT
  std::string cols;
  std::string placeholders;
  for (size_t col_idx = 0; col_idx < table.fields.size(); ++col_idx) {
    if (col_idx != 0) {
      cols += ", ";
      placeholders += ", ";
    }
    cols += table.fields[col_idx].name;
    placeholders += "$" + std::to_string(col_idx + 1);
  }

  os << "  pub fn push(&mut self, record: Record" << idx << ") {\n";
  os << "    self.records.push(record);\n";
  os << "  }\n\n";

  // Build SQL query string
  std::string sql_query = "INSERT INTO " + table.name + " (" + cols + ") VALUES (";
  for (size_t i = 0; i < table.fields.size(); ++i) {
    if (i > 0) sql_query += ", ";
    sql_query += "$" + std::to_string(i + 1);
  }
  sql_query += ")";

  os << "  pub async fn flush(&self) -> Result<()> {\n";
  os << "    if self.records.is_empty() {\n";
  os << "      return Ok(());\n";
  os << "    }\n";
  
  // Generate parameter extraction for each record
  std::vector<std::string> param_extractions;
  for (size_t col_idx = 0; col_idx < table.fields.size(); ++col_idx) {
    switch (table.fields[col_idx].datatype) {
      case DataType::STRING:
        param_extractions.push_back("&record.c" + std::to_string(col_idx));
        break;
      case DataType::U64:
        param_extractions.push_back("&(record.c" + std::to_string(col_idx) + " as i64)");
        break;
      case DataType::BOOL:
        param_extractions.push_back("&record.c" + std::to_string(col_idx));
        break;
      default:
        param_extractions.push_back("&record.c" + std::to_string(col_idx));
        break;
    }
  }

  os << "    let pool = self.get_pool()?;\n";
  os << "    let mut conn = pool.get().await?;\n";
  os << "    for record in &self.records {\n";
  os << "      let params: [&(dyn types::ToSql + Sync); " << table.fields.size() << "] = [\n";
  for (size_t i = 0; i < table.fields.size(); ++i) {
    os << "        " << param_extractions[i] << ",\n";
  }
  os << "      ];\n";
  os << "      conn.execute(\n";
  os << "        \"" << sql_query << "\", \n";
  os << "        &params\n";
  os << "      ).await?;\n";
  os << "    }\n";
  os << "    Ok(())\n";
  os << "  }\n\n";

  os << "  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {\n";
  os << "    let mut config = deadpool_postgres::Config::new();\n";
  os << "    config.url = Some(self.db_url.clone());\n";
  os << "    use deadpool_postgres::tokio_postgres::NoTls;\n";
  os << "    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;\n";
  os << "    Ok(pool)\n";
  os << "  }\n\n";

  os << "  pub async fn cancel(&self) -> Result<()> {\n";
  os << "    // PostgreSQL transactions are atomic - if connection closes, changes are rolled back\n";
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
  
  // Store record in thread-local queue instead of inserting directly
  // We queue records in memory and flush them from arboretum_finalize
  os << "  let record = Record" << idx << " { \n";
  for (size_t col_idx = 0; col_idx < table.fields.size(); ++col_idx) {
    os << "    c" << col_idx << ": ";
    switch (table.fields[col_idx].datatype) {
      case DataType::STRING: {
        os << "unsafe { std::ffi::CStr::from_ptr(c" << col_idx
           << ") }.to_string_lossy().to_string(), \n";
      } break;
      default:
        os << "c" << col_idx << ",\n";
        break;
    }
  }
  os << "  };\n";
  
  // Push directly to thread-local queue
  os << "  RECORD_QUEUE_" << idx << ".with(|q| q.borrow_mut().push(record));\n";

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

    // Queue storage - use thread-local storage for FFI compatibility
    os << "// Using thread-local storage for FFI compatibility\n";
    os << "use std::cell::RefCell;\n";
    os << "\n";
    
    // Each table has its own queue
    for (size_t i = 0; i < model.tables.size(); ++i) {
      os << "thread_local!(static RECORD_QUEUE_" << i << ": RefCell<Vec<Record" << i << ">> = RefCell::new(Vec::new()));\n";
    }
    os << "\n";

    // Exported function to queue a record
    os << "#[no_mangle]\n";
    os << "pub extern \"C\" fn queue_record(record: Record) {\n";
    os << "  match record {\n";
    for (size_t i = 0; i < model.tables.size(); ++i) {
      os << "    Record::Record" << i << "(r) => {\n";
      os << "      RECORD_QUEUE_" << i << ".with(|tb: &RefCell<Vec<Record" << i << ">>| {\n";
      os << "        tb.borrow_mut().push(r);\n";
      os << "      });\n";
      os << "    }\n";
    }
    os << "  }\n";
    os << "}\n\n";

    // Exported function to flush all queued records
    os << "#[no_mangle]\n";
    os << "pub extern \"C\" fn flush_records(db_url: *const c_char) -> bool {\n";
    os << "  let db_url = match unsafe { std::ffi::CStr::from_ptr(db_url).to_str() } {\n";
    os << "    Ok(s) => s.to_owned(),\n";
    os << "    Err(_) => return false,\n";
    os << "  };\n\n";
    
    // Create runtime and flush each queue
    os << "  let runtime = tokio::runtime::Runtime::new().expect(\"Failed to create Tokio runtime\");\n";
    os << "\n";
    
    for (size_t i = 0; i < model.tables.size(); ++i) {
      if (i > 0) os << "\n";
      os << "  {\n";
      os << "    let records_" << i << " = RECORD_QUEUE_" << i << ".take();\n";
      os << "    if !records_" << i << ".is_empty() {\n";
      os << "      let mut tb_" << i << " = TableBuilder" << i << "::new(&db_url, 1);\n";
      os << "      for record in records_" << i << ".into_iter() {\n";
      os << "        tb_" << i << ".push(record);\n";
      os << "      }\n";
      os << "      let _ = runtime.block_on(tb_" << i << ".flush());\n";
      os << "    }\n";
      os << "  }\n";
    }
    os << "\n";
    os << "  true\n";
    os << "}\n\n";

    // Import Record and TableBuilder types from io.rs so they can be used in ffi.rs
    for (size_t i = 0; i < model.tables.size(); ++i) {
      os << "use crate::io::Record" << i << ";\n";
      os << "use crate::io::TableBuilder" << i << ";\n";
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
    // Import tokio_postgres types module for ToSql trait
    // This needs to be written at the beginning of generated content so it's
    // between BEGIN and END markers where TableBuilderN structs will be
    os << "use deadpool_postgres::tokio_postgres::types;\n\n";
    os << "pub struct TableBuilders {\n";
    os << "  db_url: String,\n";
    for (size_t i = 0; i < model.tables.size(); ++i) {
      os << "  t" << i << ": TableBuilder" << i << ",\n";
    }
    os << "}\n\n";

    os << "impl TableBuilders {\n";
    os << "  pub fn new(db_url: impl AsRef<str>) -> Self {\n";
    os << "    Self {\n";
    os << "      db_url: db_url.as_ref().to_owned(),\n";
    for (size_t i = 0; i < model.tables.size(); ++i) {
      os << "      t" << i << ": TableBuilder" << i
         << "::new(db_url.as_ref(), 1),\n";
    }
    os << "    }\n";
    os << "  }\n";

    // Simplified push - just delegate to table builder
    os << "  pub fn push(&mut self, record: Record) {\n";
    os << "    match record {\n";
    for (size_t i = 0; i < model.tables.size(); ++i) {
      os << "      Record::Record" << i << "(record) => self.t" << i
         << ".push(record),\n";
    }
    os << "    }\n";
    os << "  }\n\n";

    os << "  pub async fn flush(&self) -> Result<()> {\n";
    for (size_t i = 0; i < model.tables.size(); ++i) {
      os << "    self.t" << i << ".flush().await?;\n";
    }
    os << "    Ok(())\n";
    os << "  }\n";

    os << "  pub async fn cancel(&self) -> Result<()> {\n";
    for (size_t i = 0; i < model.tables.size(); ++i) {
      os << "    self.t" << i << ".cancel().await?;\n";
    }
    os << "    Ok(())\n";
    os << "  }\n\n";

    os << "  fn get_pool(&self) -> Result<deadpool_postgres::Pool> {\n";
    os << "    let mut config = deadpool_postgres::Config::new();\n";
    os << "    config.url = Some(self.db_url.clone());\n";
    os << "    use deadpool_postgres::tokio_postgres::NoTls;\n";
    os << "    let pool = config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;\n";
    os << "    Ok(pool)\n";
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
