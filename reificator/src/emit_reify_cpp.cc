#include "emit_reify_cpp.h"

#include <filesystem>
#include <map>
#include <set>
#include <string>

#include "get_usr.h"
#include "model.h"
#include "partially_generated_file.h"
#include "util.h"

namespace arboretum {

bool AlphabetizeCXXRecordDecl(const clang::CXXRecordDecl *a,
                              const clang::CXXRecordDecl *b) {
  return a->getNameAsString() < b->getNameAsString();
}

void EmitReifyCpp(Model &model, std::map<std::string, bool> property_table,
                  const std::string &reify_cpp_dir,
                  const std::string &reify_rs_dir) {
  std::filesystem::path reify_cpp_dir_path(reify_cpp_dir);
  std::filesystem::path reify_rs_dir_path(reify_rs_dir);

  std::map<std::string, const clang::EnumDecl *> enums_to_emit;

  std::unordered_map<std::string, uint64_t> id_by_name;
  uint64_t next_id = 1UL << 32;

  auto fqn_to_id = [&](const std::string &s) {
    auto find_itr = id_by_name.find(s);
    if (find_itr == id_by_name.end()) {
      uint64_t id = next_id++;
      id_by_name[s] = id;
      return id;
    }
    return find_itr->second;
  };

  std::vector<std::pair<std::string, std::string>> meta_data_model;
  meta_data_model.push_back(std::pair("true_", "true"));
  meta_data_model.push_back(std::pair("false_", "false"));
  meta_data_model.push_back(std::pair("meta_class_", "/meta/class"));
  meta_data_model.push_back(std::pair("meta_subclass_", "/meta/subclass"));
  meta_data_model.push_back(std::pair("meta_method_", "/meta/method"));
  meta_data_model.push_back(std::pair("meta_domain_", "/meta/domain"));
  meta_data_model.push_back(std::pair("meta_range_", "/meta/range"));
  meta_data_model.push_back(
      std::pair("builtin_string_class_", "/builtin/string"));
  meta_data_model.push_back(std::pair("builtin_u64_class_", "/builtin/u64"));
  meta_data_model.push_back(std::pair("builtin_i64_class_", "/builtin/i64"));
  meta_data_model.push_back(std::pair("builtin_double_class_", "/builtin/dbl"));
  meta_data_model.push_back(std::pair("builtin_list_class_", "/builtin/list"));
  meta_data_model.push_back(
      std::pair("builtin_list_size_", "/builtin/list/size"));
  meta_data_model.push_back(
      std::pair("builtin_list_item_class_", "/builtin/list/item_class"));
  meta_data_model.push_back(std::pair("builtin_set_class_", "/builtin/set"));
  meta_data_model.push_back(
      std::pair("builtin_set_size_", "/builtin/set/size"));
  meta_data_model.push_back(
      std::pair("builtin_set_item_", "/builtin/set/item"));
  meta_data_model.push_back(
      std::pair("builtin_set_item_class_", "/builtin/set/item_class"));
  meta_data_model.push_back(std::pair("invalid_file_", "/invalid/file"));
  meta_data_model.push_back(
      std::pair("invalid_source_location_", "/invalid/clang/SourceLocation"));
  meta_data_model.push_back(std::pair("file_class_", "/file"));
  meta_data_model.push_back(std::pair("file_name_", "/file/name"));
  meta_data_model.push_back(std::pair("file_content_", "/file/content"));
  meta_data_model.push_back(
      std::pair("source_location_class_", "/clang/SourceLocation"));
  meta_data_model.push_back(
      std::pair("source_location_is_file", "/clang/SourceLocation/is_file"));
  meta_data_model.push_back(
      std::pair("source_location_file_", "/clang/SourceLocation/file"));
  meta_data_model.push_back(
      std::pair("source_location_line_", "/clang/SourceLocation/line"));
  meta_data_model.push_back(
      std::pair("source_location_column_", "/clang/SourceLocation/column"));
  meta_data_model.push_back(std::pair("source_location_expansion_loc_",
                                      "/clang/SourceLocation/expansion_loc"));
  meta_data_model.push_back(std::pair("source_location_spelling_loc_",
                                      "/clang/SourceLocation/spelling_loc"));
  meta_data_model.push_back(
      std::pair("source_range_class_", "/clang/SourceRange"));
  meta_data_model.push_back(
      std::pair("source_range_begin_", "/clang/SourceRange/begin"));
  meta_data_model.push_back(
      std::pair("source_range_end_", "/clang/SourceLocation/end"));
  meta_data_model.push_back(std::pair("qualtype_class_", "/clang/QualType"));
  meta_data_model.push_back(
      std::pair("qualtype_is_const_", "/clang/QualType/is_const"));
  meta_data_model.push_back(
      std::pair("qualtype_is_volatile_", "/clang/QualType/is_volatile"));
  meta_data_model.push_back(
      std::pair("qualtype_is_restrict_", "/clang/QualType/is_restrict"));
  meta_data_model.push_back(
      std::pair("qualtype_type_", "/clang/Qualtype/type"));

  /////////////////////////////////////////////////////////////////////////////

  auto ast_visitor_h = PartiallyGeneratedFile::Read(
      reify_cpp_dir_path / "include" / "arboretum_ast_visitor.h");
  assert(ast_visitor_h.Write([&](std::ostream &out) -> bool {
    out << "  // Types\n";
    for (const auto *type_decl :
         Sorted(model.index.clang.type_decls, AlphabetizeCXXRecordDecl)) {
      std::string decl_name = type_decl->getNameAsString();
      out << "  bool Visit" << decl_name << "(clang::" << decl_name
          << "* T);\n";
    }

    out << "\n  // TypeLocs\n";
    for (const auto *typeloc_decl :
         Sorted(model.index.clang.typeloc_decls, AlphabetizeCXXRecordDecl)) {
      std::string decl_name = typeloc_decl->getNameAsString();
      out << "  bool Visit" << decl_name << "(clang::" << decl_name
          << " TL);\n ";
    }

    out << "\n  // Decls\n";
    for (const auto *decl_decl :
         Sorted(model.index.clang.decl_decls, AlphabetizeCXXRecordDecl)) {
      std::string decl_name = decl_decl->getNameAsString();
      out << "  bool Visit" << decl_name << "(clang::" << decl_name
          << "* D);\n";
    }

    out << "\n  // Stmts\n";
    for (const auto *stmt_decl :
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
  assert(ast_visitor_cc.Write([&](std::ostream &out) -> bool {
    auto emit_methods = [&](const clang::CXXRecordDecl *decl,
                            const std::string &decl_name) {
      for (const auto &method_decl : decl->methods()) {
        std::string method_name = method_decl->getNameAsString();
        std::optional<std::string> method_usr =
            getUSR(model.ast_ctx, method_decl);
        if (!method_usr.has_value())
          continue;

        {
          auto find_itr = property_table.find(*method_usr);
          if (find_itr == property_table.end() || !find_itr->second)
            continue;
        }

        std::string method_decl_Id = model.entity_name(method_decl);

        auto return_type = method_decl->getReturnType();
        out << "  //" << method_name << "\n";
        // out << "  llvm::errs() << \"  " << method_name << "\\n\";\n";
        if (return_type->isEnumeralType()) {
          const clang::EnumDecl *decl =
              llvm::dyn_cast<clang::EnumDecl>(return_type->getAsTagDecl());

          const clang::EnumDecl *decl_def = decl->getDefinition();
          if (decl_def == nullptr)
            llvm::errs() << "Missing definition for "
                         << decl->getQualifiedNameAsString() << "!\n";

          std::optional<std::string> enum_usr = getUSR(model.ast_ctx, decl);
          if (!enum_usr.has_value()) {
            llvm::errs() << "Failed to generator USR for "
                         << decl->getQualifiedNameAsString() << "!\n";
            continue;
          }

          enums_to_emit.insert(
              std::make_pair(decl->getQualifiedNameAsString(), decl));

          out << "  {\n";
          out << "    const Id* enum_value = "
                 "context_.data_model_.resolve(D->"
              << method_name << "());\n";
          out << "    if (enum_value != nullptr) {\n";
          out << "      arboretum_create_edge(obj, "
                 "context_.data_model_."
              << method_decl_Id << ", enum_value);\n";
          out << "    }\n";
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
              out << "    const Id* other = context_.resolve(D->" << method_name
                  << "());\n";
              out << "    arboretum_create_edge(obj, "
                     "context_.data_model_."
                  << method_decl_Id << ", other);\n";
              out << "  }\n";
            }
          }
        } else if (return_type->isBuiltinType()) {
          auto builtin_type = return_type->getAs<clang::BuiltinType>();
          switch (builtin_type->getKind()) {
          case clang::BuiltinType::Bool: {
            out << "  arboretum_create_edge(obj, "
                   "context_.data_model_."
                << method_decl_Id
                << ", context_.data_model_.arboretum_node_for(D->"
                << method_name << "()));\n";
          } break;
          default:
            out << "  // " << return_type.getAsString() << "\n";
            break;
          }
        } else if (return_type->isRecordType()) {
          auto record_type = return_type->getAs<clang::RecordType>();
          auto record_decl =
              llvm::dyn_cast<clang::CXXRecordDecl>(record_type->getDecl());
          auto fqn = record_decl->getQualifiedNameAsString();
          if (fqn == "clang::QualType") {
            out << "  {\n";
            out << "    const Id* other = context_.resolve(D->" << method_name
                << "());\n";
            out << "    arboretum_create_edge(obj, "
                   "context_.data_model_."
                << method_decl_Id << ", other);\n";
            out << "  }\n";
          } else if (fqn == "clang::SourceLocation") {
            out << "  {\n";
            out << "    const Id* other = "
                   "context_.source_model_.resolve(D->"
                << method_name << "());\n";
            out << "    arboretum_create_edge(obj, "
                   "context_.data_model_."
                << method_decl_Id << ", other);\n";
            out << "  }\n";
          } else if (fqn == "clang::SourceRange") {
            out << "  {\n";
            out << "    const Id* other = "
                   "context_.source_model_.resolve(D->"
                << method_name << "());\n";
            out << "    arboretum_create_edge(obj, "
                   "context_.data_model_."
                << method_decl_Id << ", other);\n";
            out << "  }\n";
          } else {
            out << "  // " << return_type.getAsString() << "\n";
          }
        } else {
          out << "  // " << return_type.getAsString() << "\n";
        }
      }
    };

    out << "// Types\n";
    for (const auto *type_decl :
         Sorted(model.index.clang.type_decls, AlphabetizeCXXRecordDecl)) {
      std::string decl_name = type_decl->getNameAsString();
      out << "bool ArboretumASTVisitor::Visit" << decl_name
          << "(clang::" << decl_name << "* D) {\n";

      out << "  const Id* obj = context_.resolve(D);\n";

      if (type_decl == model.index.clang.stmt_decl) {
        out << "  switch(D->getTypeClass()) {\n";
        for (auto [decl, enum_decl] :
             model.index.clang.typeclass_enum_by_decl) {
          std::string decl_Id = model.entity_name(decl);

          out << "    case " << enum_decl->getQualifiedNameAsString()
              << ": {\n";
          out << "assert(context_.data_model_." << decl_Id << " != nullptr);\n";
          out << "     arboretum_create_edge(obj, "
                 "context_.data_model_.meta_class_, context_.data_model_."
              << decl_Id << "); \n";
          out << "    } break;\n";
        }
        out << "    default: break;\n";
        out << "  }\n\n";
      }

      emit_methods(type_decl, decl_name);
      out << "  return true;\n}\n\n";
    }

    out << "\n// TypeLocs\n";
    for (const auto *typeloc_decl :
         Sorted(model.index.clang.typeloc_decls, AlphabetizeCXXRecordDecl)) {
      std::string decl_name = typeloc_decl->getNameAsString();
      out << "bool ArboretumASTVisitor::Visit" << decl_name
          << "(clang::" << decl_name << " D) {\n";

      out << "  return true;\n}\n\n";
    }

    out << "\n// Decls\n";
    for (const auto *decl_decl :
         Sorted(model.index.clang.decl_decls, AlphabetizeCXXRecordDecl)) {
      std::string decl_name = decl_decl->getNameAsString();
      out << "bool ArboretumASTVisitor::Visit" << decl_name
          << "(clang::" << decl_name << "* D) {\n";

      if (decl_name == "FunctionDecl" || decl_name == "CXXMethodDecl" ||
          decl_name == "CXXConstructorDecl" ||
          decl_name == "CXXConversionDecl" ||
          decl_name == "CXXDestructorDecl" || decl_name == "TagDecl" ||
          decl_name == "EnumDecl" || decl_name == "RecordDecl" ||
          decl_name == "CXXRecordDecl" ||
          decl_name == "ClassTemplateSpecializationDecl" ||
          decl_name == "ClassTemplatePartialSpecializationDecl") {
        out << "  if (!D->isThisDeclarationADefinition()) return true;\n\n";
      }

      out << "  const Id* obj = context_.resolve(D);\n";

      if (decl_decl == model.index.clang.decl_decl) {
        out << "  switch(D->getKind()) {\n";
        for (auto [decl, enum_decl] : model.index.clang.declkind_enum_by_decl) {
          std::string decl_Id = model.entity_name(decl);

          out << "    case " << enum_decl->getQualifiedNameAsString()
              << ": {\n";
          out << "assert(context_.data_model_." << decl_Id << " != nullptr);\n";
          out << "     arboretum_create_edge(obj, "
                 "context_.data_model_.meta_class_, context_.data_model_."
              << decl_Id << "); \n";
          out << "    } break;\n";
        }
        out << "    default: break;\n";
        out << "  }\n\n";
      }

      emit_methods(decl_decl, decl_name);
      out << "  return true;\n}\n\n";
    }

    out << "\n// Stmts\n";
    for (const auto *stmt_decl :
         Sorted(model.index.clang.stmt_decls, AlphabetizeCXXRecordDecl)) {
      std::string decl_name = stmt_decl->getNameAsString();
      out << "bool ArboretumASTVisitor::Visit" << decl_name
          << "(clang::" << decl_name << "* D) {\n";

      out << "  const Id* obj = context_.resolve(D);\n";

      if (stmt_decl == model.index.clang.stmt_decl) {
        out << "  switch(D->getStmtClass()) {\n";
        for (auto [decl, enum_decl] :
             model.index.clang.stmtclass_enum_by_decl) {
          std::string decl_Id = model.entity_name(decl);

          out << "    case " << enum_decl->getQualifiedNameAsString()
              << ": {\n";
          out << "assert(context_.data_model_." << decl_Id << " != nullptr);\n";
          out << "     arboretum_create_edge(obj, "
                 "context_.data_model_.meta_class_, context_.data_model_."
              << decl_Id << "); \n";
          out << "    } break;\n";
        }
        out << "    default: break;\n";
        out << "  }\n\n";
      }

      emit_methods(stmt_decl, decl_name);
      out << "  return true;\n}\n\n";
    }

    return true;
  }));

  /////////////////////////////////////////////////////////////////////////////

  auto data_model_h = PartiallyGeneratedFile::Read(
      reify_cpp_dir_path / "include" / "arboretum_data_model.h");
  assert(data_model_h.Write([&](std::ostream &out) -> bool {
    for (const auto &[var, name] : meta_data_model) {
      out << "  Id* " << var << " = nullptr;\n";
    }

    for (auto &decl_decl : model.index.clang.all_decls) {
      out << "  Id* " << model.entity_name(decl_decl) << " = nullptr;\n";
      for (auto method_decl : decl_decl->methods()) {
        std::optional<std::string> method_decl_usr =
            getUSR(model.ast_ctx, method_decl);
        if (!method_decl_usr.has_value() ||
            !property_table.contains(*method_decl_usr)) {
          continue;
        }

        out << "  Id* " << model.entity_name(method_decl) << " = nullptr;\n";
      }
    }

    for (auto &[enum_name, enum_decl] : enums_to_emit) {
      out << "  Id* resolve(" << enum_name << " e);\n";
      out << "  Id* " << model.entity_name(enum_decl) << " = nullptr;\n";
      for (const auto &enum_value_decl : enum_decl->enumerators()) {
        out << "  Id* " << model.entity_name(enum_value_decl)
            << " = nullptr;\n";
      }
    }

    return true;
  }));

  /////////////////////////////////////////////////////////////////////////////

  auto data_model_cc = PartiallyGeneratedFile::Read(reify_cpp_dir_path / "src" /
                                                    "arboretum_data_model.cc");
  assert(data_model_cc.Write([&](std::ostream &out) -> bool {
    out << "DataModel::DataModel() {\n";
    for (const auto &[var, name] : meta_data_model) {
      out << "  " << var << " = arboretum_create_nameless_node_with_id("
          << fqn_to_id(name) << ");\n";
    }
    out << "}\n\n";

    out << "DataModel EmitDataModel() {\n";
    out << "  DataModel data_model;\n\n";

    // Assign a named Id to the appropriate data model field for that decl.
    for (auto &decl_decl : model.index.clang.all_decls) {
      std::string decl_decl_Id = model.entity_name(decl_decl);

      out << "  data_model." << decl_decl_Id
          << " = arboretum_create_nameless_node_with_id("
          << fqn_to_id(decl_decl->getQualifiedNameAsString()) << ");\n";
      out << "  arboretum_create_edge(data_model." << decl_decl_Id
          << ", data_model.meta_class_, data_model."
          << model.entity_name(model.index.clang.cxx_record_decl) << ");\n";

      // Emit the subclass edges
      for (auto super : model.index.inheritance.supers[decl_decl]) {
        if (!model.index.clang.all_decls.contains(super))
          continue;

        std::string super_Id = model.entity_name(super);
        out << "  arboretum_create_edge(data_model." << decl_decl_Id
            << ", data_model.meta_subclass_, data_model." << super_Id << ");\n";
      }

      // Emit the methods
      out << "  {\n";
      out << "    std::vector<Id*> methods;\n";

      for (auto method_decl : decl_decl->methods()) {
        std::optional<std::string> method_decl_usr =
            getUSR(model.ast_ctx, method_decl);
        if (!method_decl_usr.has_value() ||
            !property_table.contains(*method_decl_usr)) {
          continue;
        }

        std::string method_decl_fqn = method_decl->getQualifiedNameAsString();
        std::string method_decl_Id = model.entity_name(method_decl);

        out << "      data_model." << method_decl_Id
            << " = arboretum_create_nameless_node_with_id("
            << fqn_to_id(method_decl_fqn) << ");\n";
        out << "      methods.push_back(data_model." << method_decl_Id
            << ");\n";
      }

      out << "    arboretum_create_edge(data_model." << decl_decl_Id
          << ", data_model.meta_method_, "
             "data_model.arboretum_node_for(data_model."
          << model.entity_name(model.index.clang.cxx_method_decl)
          << ", methods));\n";

      out << "  }\n";
    }

    for (auto &[enum_name, enum_decl] : enums_to_emit) {
      std::string enum_decl_fqn = enum_decl->getQualifiedNameAsString();
      std::string enum_decl_Id = model.entity_name(enum_decl);

      out << "  {\n";
      out << "    data_model." << enum_decl_Id
          << " = arboretum_create_nameless_node_with_id("
          << fqn_to_id(enum_decl_fqn) << ");\n";
      out << "    arboretum_create_edge(data_model." << enum_decl_Id
          << ", data_model.meta_class_, data_model."
          << model.entity_name(model.index.clang.enum_decl) << ");\n ";

      out << "    std::vector<Id*> enumerators;\n";
      for (const auto &enum_value_decl : enum_decl->enumerators()) {
        std::string enum_value_decl_fqn =
            enum_value_decl->getQualifiedNameAsString();
        std::string enum_value_decl_Id = model.entity_name(enum_value_decl);

        out << "    {\n";
        out << "      data_model." << enum_value_decl_Id
            << " = arboretum_create_nameless_node_with_id("
            << fqn_to_id(enum_value_decl_fqn) << ");\n";
        out << "      arboretum_create_edge(data_model." << enum_value_decl_Id
            << ", data_model.meta_class_, "
               "data_model."
            << model.entity_name(model.index.clang.enum_constant_decl)
            << ");\n";
        out << "      enumerators.push_back(data_model." << enum_value_decl_Id
            << ");\n";

        out << "    }\n";
      }
      out << "    data_model.arboretum_node_for(data_model."
          << model.entity_name(model.index.clang.enum_constant_decl)
          << ", enumerators);\n";
      out << "  }\n";
    }
    out << "  return data_model;\n";
    out << "}\n\n";

    for (auto &[enum_name, enum_decl] : enums_to_emit) {
      out << "Id* DataModel::resolve(" << enum_name << " e) {\n";
      out << "  switch(e) {\n";

      std::set<llvm::APSInt> seen_values;
      for (const auto &enum_value_decl : enum_decl->enumerators()) {
        std::string enum_value_name =
            enum_value_decl->getQualifiedNameAsString();

        llvm::APSInt enum_value = enum_value_decl->getInitVal();

        auto find_itr = seen_values.find(enum_value);
        if (find_itr != seen_values.end())
          continue;
        seen_values.insert(enum_value);

        out << "    case " << enum_value_name << ": return "
            << model.entity_name(enum_value_decl) << ";\n";
      }

      std::string enum_type = enum_decl->getIntegerType().getAsString();
      if (enum_type == "_Bool")
        enum_type = "bool";
      if (enum_type == "uint8_t")
        enum_type = "uint32_t";
      // out << "    default: llvm::errs() << \"" << enum_name
      //     << ": Unexpected enum value: \" << static_cast<" << enum_type
      //     << ">(e) << \"\\n\";\n";
      out << "    default: break;\n";
      out << "  }\n";
      out << "  return nullptr;\n";
      out << "}\n\n";
    }

    return true;
  }));

  /////////////////////////////////////////////////////////////////////////////

  auto reify_rs =
      PartiallyGeneratedFile::Read(reify_rs_dir_path / "src" / "lib.rs");
  assert(reify_rs.Write([&](std::ostream &out) -> bool {
    out << "use arboretum_graph::GraphBuffer;\n\n";

    out << "pub fn build_data_model() -> GraphBuffer {\n";
    out << "  let mut g = GraphBuffer::new();\n";
    for (const auto &[s, id] : id_by_name) {
      out << "  g.add_named_node(" << id << ", \"" << s << "\");\n";
    }
    out << "  g\n";
    out << "}\n\n";

    return true;
  }));
}

} // namespace arboretum