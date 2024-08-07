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
                  const std::string &reify_cpp_dir) {
  std::filesystem::path reify_cpp_dir_path(reify_cpp_dir);

  std::map<std::string, const clang::EnumDecl *> enums_to_emit;

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

        std::string method_decl_entity = model.assign_entity_name(method_decl);

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
          out << "    const Entity* enum_value = "
                 "context_.data_model_.resolve(D->"
              << method_name << "());\n";
          out << "    if (enum_value != nullptr) {\n";
          out << "      arboretum_create_edge(obj, "
                 "context_.data_model_."
              << method_decl_entity << ", enum_value);\n";
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
              out << "    const Entity* other = context_.resolve(D->"
                  << method_name << "());\n";
              out << "    arboretum_create_edge(obj, "
                     "context_.data_model_."
                  << method_decl_entity << ", other);\n";
              out << "  }\n";
            }
          }
        } else if (return_type->isBuiltinType()) {
          auto builtin_type = return_type->getAs<clang::BuiltinType>();
          switch (builtin_type->getKind()) {
          case clang::BuiltinType::Bool: {
            out << "  arboretum_create_edge(obj, "
                   "context_.data_model_."
                << method_decl_entity
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
            out << "    const Entity* other = context_.resolve(D->"
                << method_name << "());\n";
            out << "    arboretum_create_edge(obj, "
                   "context_.data_model_."
                << method_decl_entity << ", other);\n";
            out << "  }\n";
          } else if (fqn == "clang::SourceLocation") {
            out << "  {\n";
            out << "    const Entity* other = "
                   "context_.source_model_.resolve(D->"
                << method_name << "());\n";
            out << "    arboretum_create_edge(obj, "
                   "context_.data_model_."
                << method_decl_entity << ", other);\n";
            out << "  }\n";
          } else if (fqn == "clang::SourceRange") {
            out << "  {\n";
            out << "    const Entity* other = "
                   "context_.source_model_.resolve(D->"
                << method_name << "());\n";
            out << "    arboretum_create_edge(obj, "
                   "context_.data_model_."
                << method_decl_entity << ", other);\n";
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

      out << "  const Entity* obj = context_.resolve(D);\n";

      if (type_decl == model.index.clang.stmt_decl) {
        out << "  switch(D->getTypeClass()) {\n";
        for (auto [decl, enum_decl] :
             model.index.clang.typeclass_enum_by_decl) {
          std::string decl_entity = model.assign_entity_name(decl);

          out << "    case " << enum_decl->getQualifiedNameAsString()
              << ": {\n";
          out << "assert(context_.data_model_." << decl_entity
              << " != nullptr);\n";
          out << "     arboretum_create_edge(obj, "
                 "context_.data_model_.meta_class_, context_.data_model_."
              << decl_entity << "); \n";
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

      out << "  const Entity* obj = context_.resolve(D);\n";

      if (decl_decl == model.index.clang.decl_decl) {
        out << "  switch(D->getKind()) {\n";
        for (auto [decl, enum_decl] : model.index.clang.declkind_enum_by_decl) {
          std::string decl_entity = model.assign_entity_name(decl);

          out << "    case " << enum_decl->getQualifiedNameAsString()
              << ": {\n";
          out << "assert(context_.data_model_." << decl_entity
              << " != nullptr);\n";
          out << "     arboretum_create_edge(obj, "
                 "context_.data_model_.meta_class_, context_.data_model_."
              << decl_entity << "); \n";
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

      out << "  const Entity* obj = context_.resolve(D);\n";

      if (stmt_decl == model.index.clang.stmt_decl) {
        out << "  switch(D->getStmtClass()) {\n";
        for (auto [decl, enum_decl] :
             model.index.clang.stmtclass_enum_by_decl) {
          std::string decl_entity = model.assign_entity_name(decl);

          out << "    case " << enum_decl->getQualifiedNameAsString()
              << ": {\n";
          out << "assert(context_.data_model_." << decl_entity
              << " != nullptr);\n";
          out << "     arboretum_create_edge(obj, "
                 "context_.data_model_.meta_class_, context_.data_model_."
              << decl_entity << "); \n";
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
    for (auto &decl_decl : model.index.clang.all_decls) {
      out << "  Entity* " << model.assign_entity_name(decl_decl)
          << " = nullptr;\n";
      for (auto method_decl : decl_decl->methods()) {
        std::optional<std::string> method_decl_usr =
            getUSR(model.ast_ctx, method_decl);
        if (!method_decl_usr.has_value() ||
            !property_table.contains(*method_decl_usr)) {
          continue;
        }

        out << "  Entity* " << model.assign_entity_name(method_decl)
            << " = nullptr;\n";
      }
    }

    for (auto &[enum_name, enum_decl] : enums_to_emit) {
      out << "  Entity* resolve(" << enum_name << " e);\n";
      out << "  Entity* " << model.assign_entity_name(enum_decl)
          << " = nullptr;\n";
      for (const auto &enum_value_decl : enum_decl->enumerators()) {
        out << "  Entity* " << model.assign_entity_name(enum_value_decl)
            << " = nullptr;\n";
      }
    }

    return true;
  }));

  /////////////////////////////////////////////////////////////////////////////

  auto data_model_cc = PartiallyGeneratedFile::Read(reify_cpp_dir_path / "src" /
                                                    "arboretum_data_model.cc");
  assert(data_model_cc.Write([&](std::ostream &out) -> bool {
    out << "DataModel EmitDataModel() {\n";
    out << "  DataModel data_model;\n\n";

    // Assign a named entity to the appropriate data model field for that decl.
    for (auto &decl_decl : model.index.clang.all_decls) {
      std::string decl_decl_entity = model.assign_entity_name(decl_decl);

      out << "  data_model." << decl_decl_entity
          << " = arboretum_create_named_node(\""
          << fqn_to_name(decl_decl->getQualifiedNameAsString()) << "\");\n";
      out << "  arboretum_create_edge(data_model." << decl_decl_entity
          << ", data_model.meta_class_, data_model."
          << model.assign_entity_name(model.index.clang.cxx_record_decl)
          << ");\n";

      // Emit the methods
      out << "  {\n";
      out << "    std::vector<Entity*> methods;\n";

      for (auto method_decl : decl_decl->methods()) {
        std::optional<std::string> method_decl_usr =
            getUSR(model.ast_ctx, method_decl);
        if (!method_decl_usr.has_value() ||
            !property_table.contains(*method_decl_usr)) {
          continue;
        }

        std::string method_decl_fqn = method_decl->getQualifiedNameAsString();
        std::string method_decl_entity = model.assign_entity_name(method_decl);

        out << "      data_model." << method_decl_entity
            << " = arboretum_create_named_node(\""
            << fqn_to_name(method_decl_fqn) << "\");\n";
        out << "      methods.push_back(data_model." << method_decl_entity
            << ");\n";
      }

      out << "    arboretum_create_edge(data_model." << decl_decl_entity
          << ", data_model.meta_method_, "
             "data_model.arboretum_node_for(data_model."
          << model.assign_entity_name(model.index.clang.cxx_method_decl)
          << ", methods));\n";

      out << "  }\n";
    }

    for (auto &[enum_name, enum_decl] : enums_to_emit) {
      std::string enum_decl_fqn = enum_decl->getQualifiedNameAsString();
      std::string enum_decl_entity = model.assign_entity_name(enum_decl);

      out << "  {\n";
      out << "    data_model." << enum_decl_entity
          << " = arboretum_create_named_node(\"" << fqn_to_name(enum_decl_fqn)
          << "\");\n";
      out << "    arboretum_create_edge(data_model." << enum_decl_entity
          << ", data_model.meta_class_, data_model."
          << model.assign_entity_name(model.index.clang.enum_decl) << ");\n ";

      out << "    std::vector<Entity*> enumerators;\n";
      for (const auto &enum_value_decl : enum_decl->enumerators()) {
        std::string enum_value_decl_fqn =
            enum_value_decl->getQualifiedNameAsString();
        std::string enum_value_decl_entity =
            model.assign_entity_name(enum_value_decl);

        out << "    {\n";
        out << "      data_model." << enum_value_decl_entity
            << " = arboretum_create_named_node(\""
            << fqn_to_name(enum_value_decl_fqn) << "\");\n";
        out << "      arboretum_create_edge(data_model."
            << enum_value_decl_entity
            << ", data_model.meta_class_, "
               "data_model."
            << model.assign_entity_name(model.index.clang.enum_constant_decl)
            << ");\n";
        out << "      enumerators.push_back(data_model."
            << enum_value_decl_entity << ");\n";

        out << "    }\n";
      }
      out << "    data_model.arboretum_node_for(data_model."
          << model.assign_entity_name(model.index.clang.enum_constant_decl)
          << ", enumerators);\n";
      out << "  }\n";
    }
    out << "  return data_model;\n";
    out << "}\n\n";

    for (auto &[enum_name, enum_decl] : enums_to_emit) {
      out << "Entity* DataModel::resolve(" << enum_name << " e) {\n";
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
            << model.assign_entity_name(enum_value_decl) << ";\n";
      }

      std::string enum_type = enum_decl->getIntegerType().getAsString();
      if (enum_type == "_Bool")
        enum_type = "bool";
      if (enum_type == "uint8_t")
        enum_type = "uint32_t";
      out << "    default: llvm::errs() << \"" << enum_name
          << ": Unexpected enum value: \" << static_cast<" << enum_type
          << ">(e) << \"\\n\";\n";
      out << "  }\n";
      out << "  return nullptr;\n";
      out << "}\n\n";
    }

    return true;
  }));
}

} // namespace arboretum