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

    // Import thread-local storage for enum queues
    out << "// Thread-local storage for enum queues\n";
    out << "thread_local!(static ENUM_QUEUE: std::cell::RefCell<Vec<(u64, String)>> = std::cell::RefCell::new(Vec::new()));\n";
    out << "thread_local!(static ENUM_VALUE_QUEUE: std::cell::RefCell<Vec<(u64, u64)>> = std::cell::RefCell::new(Vec::new()));\n\n";

    // Generate FFI functions to insert enum records into PostgreSQL
    for (const auto &[enum_decl, idx] : enum_to_idx) {
      std::string enum_name = enum_decl->getQualifiedNameAsString();
      // Replace special characters in function name
      std::replace(enum_name.begin(), enum_name.end(), ' ', '_');
      std::replace(enum_name.begin(), enum_name.end(), '<', '_');
      std::replace(enum_name.begin(), enum_name.end(), '>', '_');
      std::replace(enum_name.begin(), enum_name.end(), ':', '_');
      
      out << "#[no_mangle]\n";
      out << "pub extern \"C\" fn arboretum_emit_enum_" << enum_name
          << "(id: u64, name: *const c_char) {\n";
      out << "  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };\n";
      out << "  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));\n";
      out << "}\n\n";
    }

    // Generate FFI functions to insert enum constant records into PostgreSQL
    for (const auto &[enum_decl, enum_idx] : enum_to_idx) {
      std::string enum_name = enum_decl->getQualifiedNameAsString();
      // Replace special characters in function name
      std::replace(enum_name.begin(), enum_name.end(), ' ', '_');
      std::replace(enum_name.begin(), enum_name.end(), '<', '_');
      std::replace(enum_name.begin(), enum_name.end(), '>', '_');
      std::replace(enum_name.begin(), enum_name.end(), ':', '_');
      
      for (const auto *enumerator : enum_decl->enumerators()) {
        auto enum_constant_it = enum_constant_to_idx.find(enumerator);
        if (enum_constant_it == enum_constant_to_idx.end()) {
          continue;
        }

        std::string enum_const_name = enumerator->getQualifiedNameAsString();
        // Replace special characters in function name
        std::replace(enum_const_name.begin(), enum_const_name.end(), ' ', '_');
        std::replace(enum_const_name.begin(), enum_const_name.end(), '<', '_');
        std::replace(enum_const_name.begin(), enum_const_name.end(), '>', '_');
        std::replace(enum_const_name.begin(), enum_const_name.end(), ':', '_');
        
        out << "#[no_mangle]\n";
        out << "pub extern \"C\" fn arboretum_emit_enum_value_" 
            << enum_const_name << "(id: u64, enum_id: u64) {\n";
        out << "  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));\n";
        out << "}\n\n";
      }
    }

    return true;
  }));
}

}  // namespace arboretum