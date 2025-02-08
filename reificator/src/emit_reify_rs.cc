#include "emit_reify_rs.h"

#include <filesystem>
#include <fstream>
#include <map>
#include <string>

#include "get_usr.h"
#include "model.h"
#include "name_registry.h"
#include "partially_generated_file.h"
#include "util.h"

namespace arboretum {

void EmitReifyRs(
    Model &model, std::map<std::string, bool> property_table,
    const std::string &reify_rs_dir, NameRegistry &name_registry,
    const std::map<std::string, const clang::EnumDecl *> &enums_to_emit) {
  std::filesystem::path reify_rs_dir_path(reify_rs_dir);
  auto reify_rs =
      PartiallyGeneratedFile::Read(reify_rs_dir_path / "src" / "lib.rs");
  assert(reify_rs.Write([&](std::ostream &out) -> bool {
    auto enum_to_idx = model.GetEnumToIndex();
    auto enum_constant_to_idx = model.GetEnumConstantToIndex();

    // Output function to return enum table
    out << "pub fn get_enum_table() -> arrow::record_batch::RecordBatch {\n";
    out << "    RecordBatch::try_new(\n";
    out << "        Arc::new(Schema::new(vec![\n";
    out << "        Field::new(\"id\", DataType::UInt64, false),\n";
    out << "        Field::new(\"name\", DataType::Utf8, false),\n";
    out << "    ])),\n";
    out << "        vec![\n";
    out << "            Arc::new(UInt64Array::from_iter([";

    for (const auto &[enum_decl, idx] : enum_to_idx) {
      out << idx << ", ";
    }
    out << "])),\n";
    out << "            Arc::new(StringArray::from_iter_values([";
    for (const auto &[enum_decl, idx] : enum_to_idx) {
      out << "\"" << enum_decl->getNameAsString() << "\", ";
    }
    out << "])),\n";
    out << "        ],\n";
    out << "    ).unwrap()\n";
    out << "}\n\n";

    // Output function to return enum constant table
    out << "pub fn get_enum_constant_table() -> "
           "arrow::record_batch::RecordBatch {\n";

    std::vector<std::string> id_values;
    std::vector<std::string> enum_id_values;
    std::vector<std::string> name_values;

    for (const auto &[enum_decl, enum_idx] : enum_to_idx) {
      for (const auto *enumerator : enum_decl->enumerators()) {
        auto enum_constant_it = enum_constant_to_idx.find(enumerator);
        if (enum_constant_it == enum_constant_to_idx.end()) {
          continue;
        }
        id_values.push_back(std::to_string(enum_constant_it->second));
        enum_id_values.push_back(std::to_string(enum_idx));
        name_values.push_back(enumerator->getNameAsString());
      }
    }

    out << "    RecordBatch::try_new(\n";
    out << "        Arc::new(\n";
    out << "          Schema::new(vec![\n";
    out << "            Field::new(\"id\", DataType::UInt64, false),\n";
    out << "            Field::new(\"enum_id\", DataType::UInt64, false),\n";
    out << "            Field::new(\"name\", DataType::Utf8, false),\n";
    out << "          ])),\n";
    out << "          vec![\n";
    out << "            Arc::new(UInt64Array::from_iter([";
    for (const auto &id : id_values) {
      out << id << ", ";
    }
    out << "])),\n";
    out << "            Arc::new(UInt64Array::from_iter([";
    for (const auto &enum_id : enum_id_values) {
      out << enum_id << ", ";
    }
    out << "])),\n";
    out << "            Arc::new(StringArray::from_iter_values([";
    for (const auto &name : name_values) {
      out << "\"" << name << "\", ";
    }
    out << "])),\n";
    out << "        ],\n";
    out << "    ).unwrap()\n";
    out << "}\n";

    return true;
  }));
}

}  // namespace arboretum