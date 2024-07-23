#include "emit_reify_cpp.h"

#include <filesystem>
#include <set>
#include <string>

#include "model.h"
#include "partially_generated_file.h"
#include "util.h"

namespace arboretum {

bool AlphabetizeCXXRecordDecl(const clang::CXXRecordDecl* a,
                              const clang::CXXRecordDecl* b) {
  return a->getNameAsString() < b->getNameAsString();
}

void EmitReifyCpp(Model& model,
                  std::unordered_map<std::string, bool> property_table,
                  const std::string& reify_cpp_dir) {
  std::filesystem::path reify_cpp_dir_path(reify_cpp_dir);

  std::map<std::string, const clang::EnumDecl*> enums_to_emit;

  /////////////////////////////////////////////////////////////////////////////

  auto ast_visitor_h = PartiallyGeneratedFile::Read(
      reify_cpp_dir_path / "include" / "arboretum_ast_visitor.h");
  assert(ast_visitor_h.Write([&](std::ostream& out) -> bool {
    out << "  // Types\n";
    for (const auto* type_decl :
         Sorted(model.index.clang.type_decls, AlphabetizeCXXRecordDecl)) {
      std::string decl_name = type_decl->getNameAsString();
      out << "  bool Visit" << decl_name << "(clang::" << decl_name
          << "* T);\n";
    }

    out << "\n  // TypeLocs\n";
    for (const auto* typeloc_decl :
         Sorted(model.index.clang.typeloc_decls, AlphabetizeCXXRecordDecl)) {
      std::string decl_name = typeloc_decl->getNameAsString();
      out << "  bool Visit" << decl_name << "(clang::" << decl_name
          << " TL);\n ";
    }

    out << "\n  // Decls\n";
    for (const auto* decl_decl :
         Sorted(model.index.clang.decl_decls, AlphabetizeCXXRecordDecl)) {
      std::string decl_name = decl_decl->getNameAsString();
      out << "  bool Visit" << decl_name << "(clang::" << decl_name
          << "* D);\n";
    }

    out << "\n  // Stmts\n";
    for (const auto* stmt_decl :
         Sorted(model.index.clang.stmt_decls, AlphabetizeCXXRecordDecl)) {
      std::string decl_name = stmt_decl->getNameAsString();
      out << "  bool Visit" << decl_name << "(clang::" << decl_name
          << "* S);\n";
    }
    return true;
  }));

  /////////////////////////////////////////////////////////////////////////////

  auto ast_visitor_cc = PartiallyGeneratedFile::Read(
      reify_cpp_dir_path / "src" / "arboretum_ast_visitor.cc");
  assert(ast_visitor_cc.Write([&](std::ostream& out) -> bool {
    auto emit_methods = [&](const clang::CXXRecordDecl* decl,
                            const std::string& decl_name) {
      for (const auto& method_decl : decl->methods()) {
        std::string method_name = method_decl->getNameAsString();
        std::string method_fqn = method_decl->getQualifiedNameAsString();
        {
          auto find_itr = property_table.find(method_fqn);
          if (find_itr == property_table.end() || !find_itr->second) continue;
        }

        auto return_type = method_decl->getReturnType();
        if (return_type->isEnumeralType()) {
          const clang::EnumDecl* decl =
              llvm::dyn_cast<clang::EnumDecl>(return_type->getAsTagDecl());

          const clang::EnumDecl* decl_def = decl->getDefinition();
          if (decl_def == nullptr)
            llvm::errs() << "Missing definition for "
                         << decl->getQualifiedNameAsString() << "!\n";

          enums_to_emit.insert(
              std::make_pair(decl->getQualifiedNameAsString(), decl));

          out << "  {\n";
          out << "    const Thing* enum_value = "
                 "context_.data_model_.resolve(D->"
              << method_name << "());\n";
          out << "    assert(enum_value != nullptr);\n";
          out << "    arboretum_create_relation(obj, \"" << method_name
              << "\", enum_value)\n";
          out << "  }\n";
        } else if (return_type->isPointerType()) {
          auto pointer_type = return_type->getAs<clang::PointerType>();
          auto pointee_type = pointer_type->getPointeeType();
          if (pointee_type->isRecordType()) {
            auto record_type = pointee_type->getAs<clang::RecordType>();
            auto record_decl =
                llvm::dyn_cast<clang::CXXRecordDecl>(record_type->getDecl());
            if (model.index.clang.all_decls.contains(record_decl)) {
              out << "  {\n";
              out << "    const Thing* other = context_.resolve(D->"
                  << method_name << "());\n";
              out << "    arboretum_create_relation(obj, \"" << method_name
                  << "\", other)\n";
              out << "  }\n";
            }
          }
        }
      }
    };

    out << "// Types\n";
    for (const auto* type_decl :
         Sorted(model.index.clang.type_decls, AlphabetizeCXXRecordDecl)) {
      std::string decl_name = type_decl->getNameAsString();
      out << "bool ArboretumASTVisitor::Visit" << decl_name
          << "(clang::" << decl_name << "* D) {\n";

      out << "  ObjectBuilder* builder = arboretum_object_builder_new();\n";
      out << "  const Thing* obj = context_.resolve(D);\n";
      emit_methods(type_decl, decl_name);
      out << "  return true;\n}\n\n";
    }

    out << "\n// TypeLocs\n";
    for (const auto* typeloc_decl :
         Sorted(model.index.clang.typeloc_decls, AlphabetizeCXXRecordDecl)) {
      std::string decl_name = typeloc_decl->getNameAsString();
      out << "bool ArboretumASTVisitor::Visit" << decl_name
          << "(clang::" << decl_name << " D) {\n";

      out << "  return true;\n}\n\n";
    }

    out << "\n// Decls\n";
    for (const auto* decl_decl :
         Sorted(model.index.clang.decl_decls, AlphabetizeCXXRecordDecl)) {
      std::string decl_name = decl_decl->getNameAsString();
      out << "bool ArboretumASTVisitor::Visit" << decl_name
          << "(clang::" << decl_name << "* D) {\n";

      out << "  ObjectBuilder* builder = arboretum_object_builder_new();\n";
      out << "  const Thing* obj = context_.resolve(D);\n";
      emit_methods(decl_decl, decl_name);
      out << "  aboretum_merge_fields(obj, builder);\n";
      out << "  return true;\n}\n\n";
    }

    out << "\n// Stmts\n";
    for (const auto* stmt_decl :
         Sorted(model.index.clang.stmt_decls, AlphabetizeCXXRecordDecl)) {
      std::string decl_name = stmt_decl->getNameAsString();
      out << "bool ArboretumASTVisitor::Visit" << decl_name
          << "(clang::" << decl_name << "* D) {\n";

      out << "  ObjectBuilder* builder = arboretum_object_builder_new();\n";
      out << "  const Thing* obj = context_.resolve(D);\n";
      emit_methods(stmt_decl, decl_name);
      out << "  return true;\n}\n\n";
    }

    return true;
  }));

  /////////////////////////////////////////////////////////////////////////////

  auto data_model_h = PartiallyGeneratedFile::Read(
      reify_cpp_dir_path / "include" / "arboretum_data_model.h");
  assert(data_model_h.Write([&](std::ostream& out) -> bool {
    for (auto& [enum_name, enum_decl] : enums_to_emit) {
      std::string enum_var_name = fqn_to_var(enum_name);
      out << "  Thing* " << enum_var_name << "_;\n";
      for (const auto& enum_value_decl : enum_decl->enumerators()) {
        std::string enum_value_var_name =
            fqn_to_var(enum_value_decl->getQualifiedNameAsString());
        out << "  Thing* " << enum_value_var_name << "_;\n";
      }
      out << "  Thing* resolve(" << enum_name << " e);\n";
    }
    return true;
  }));

  /////////////////////////////////////////////////////////////////////////////

  auto data_model_cc = PartiallyGeneratedFile::Read(reify_cpp_dir_path / "src" /
                                                    "arboretum_data_model.cc");
  assert(data_model_cc.Write([&](std::ostream& out) -> bool {
    out << "DataModel EmitDataModel() {\n";
    out << "  DataModel data_model;\n\n";

    for (auto& [enum_name, enum_decl] : enums_to_emit) {
      std::string enum_var_name = fqn_to_var(enum_name);
      out << "  // " << enum_name << "\n";
      out << "  data_model." << enum_var_name
          << "_ = arboretum_node_new_with_id(\"enum\", \"" << enum_name
          << "\", nullptr);\n";
      for (const auto& enum_value_decl : enum_decl->enumerators()) {
        std::string enum_value_name =
            enum_value_decl->getQualifiedNameAsString();
        std::string enum_value_var_name = fqn_to_var(enum_value_name);
        out << "  data_model." << enum_value_var_name
            << "_ = arboretum_node_new_with_id(\"enum_value\", \""
            << enum_value_name << "\", nullptr);\n";
      }
      out << "\n";
    }

    out << "  return data_model;\n";
    out << "}\n\n";

    for (auto& [enum_name, enum_decl] : enums_to_emit) {
      out << "Thing* DataModel::resolve(" << enum_name << " e) {\n";
      out << "  switch(e) {\n";

      std::set<llvm::APSInt> seen_values;
      for (const auto& enum_value_decl : enum_decl->enumerators()) {
        std::string enum_value_name =
            enum_value_decl->getQualifiedNameAsString();

        llvm::APSInt enum_value = enum_value_decl->getInitVal();

        auto find_itr = seen_values.find(enum_value);
        if (find_itr != seen_values.end()) continue;
        seen_values.insert(enum_value);

        std::string enum_value_var_name = fqn_to_var(enum_value_name);
        out << "    case " << enum_value_name << ": return "
            << enum_value_var_name << "_;\n";
      }

      std::string enum_type = enum_decl->getIntegerType().getAsString();
      if (enum_type == "_Bool") enum_type = "bool";
      out << "    default: llvm::errs() << \"Unexpected enum value: \" << "
             "static_cast<"
          << enum_type << ">(e) << \"\\n\";\n";
      out << "  }\n";
      out << "  return nullptr;\n";
      out << "}\n\n";
    }

    return true;
  }));
}

}  // namespace arboretum