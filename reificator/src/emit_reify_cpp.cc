#include "emit_reify_cpp.h"

#include <filesystem>
#include <fstream>
#include <map>
#include <optional>
#include <set>
#include <string>

#include "get_usr.h"
#include "model.h"
#include "name_registry.h"
#include "partially_generated_file.h"
#include "util.h"

namespace arboretum {

bool AlphabetizeCXXRecordDecl(const clang::CXXRecordDecl *a,
                              const clang::CXXRecordDecl *b) {
  return a->getNameAsString() < b->getNameAsString();
}

namespace {
// Helper to emit code for resolving a field value based on its handler kind
void EmitFieldResolution(std::ostream &out, const std::string &target_var,
                         const std::string &source_expr,
                         const MethodHandler &handler, int indent = 1) {
  std::string indentation(indent * 2, ' ');  // 2 spaces per indent level

  switch (handler.Kind()) {
    case MethodHandlerKind::Identity:
      out << indentation << cpp_type_for_datatype(*handler.Datatype()) << " "
          << target_var << " = D->" << handler.method_decl->getNameAsString()
          << "();\n";
      break;
    case MethodHandlerKind::Enum:
      out << indentation << "uint64_t " << target_var
          << " = context_.data_model_.resolve(" << source_expr << ");\n";
      break;
    case MethodHandlerKind::ClangPointer:
    case MethodHandlerKind::QualType:
      out << indentation << "uint64_t " << target_var << " = context_.resolve("
          << source_expr << ");\n";
      break;
    case MethodHandlerKind::SourceLocation:
    case MethodHandlerKind::SourceRange:
      out << indentation << "uint64_t " << target_var
          << " = context_.source_model_.resolve(" << source_expr << ");\n";
      break;
    case MethodHandlerKind::StringRef:
      out << indentation << "const char* " << target_var << " = " << source_expr
          << ".data();\n";
      break;
    case MethodHandlerKind::BasicString:
      out << indentation << "std::string " << target_var << "_str = D->"
          << handler.method_decl->getNameAsString() << "();\n";
      out << indentation << "const char* " << target_var << " = " << target_var
          << "_str.c_str();\n";
      break;
    default:
      break;
  }
}

// New helper function to emit common handler logic
void EmitHandlerLogic(std::ostream &out, const clang::CXXRecordDecl *decl,
                      const Model &model, const std::string &var_name = "D") {
  auto find_itr = model.handler_by_decl.find(decl);
  if (find_itr != model.handler_by_decl.end()) {
    auto &handler = find_itr->second;

    out << "  uint64_t c0 = context_.resolve(" << var_name << ");\n";

    for (size_t i = 0; i < handler.data_field_handlers.size(); ++i) {
      auto field = handler.data_field_handlers[i].get();
      std::string target = "c" + std::to_string(i + 1);
      std::string source =
          var_name + "->" + field->method_decl->getNameAsString() + "()";
      EmitFieldResolution(out, target, source, *field, 1);
    }

    for (auto &assoc_field : handler.assoc_field_handlers) {
      out << "  {\n";

      out << "    uint64_t idx = 0;\n";
      out << "    for (const auto& element : " << var_name << "->"
          << assoc_field->method_decl->getNameAsString() << "()) {\n";

      std::unique_ptr<MethodHandler> *element_adapter = nullptr;
      if (assoc_field->Kind() == MethodHandlerKind::ArrayRef) {
        element_adapter =
            &static_cast<ArrayRefMethodHandler *>(assoc_field.get())
                 ->element_adapter;
      } else if (assoc_field->Kind() == MethodHandlerKind::IteratorRange) {
        element_adapter =
            &static_cast<IteratorRangeMethodHandler *>(assoc_field.get())
                 ->element_adapter;
      } else {
        llvm::errs() << "ERROR: Unexpected associated field handler kind: "
                     << static_cast<int>(assoc_field->Kind()) << "\n";
        assert(false && "Invalid associated field handler kind");
      }

      EmitFieldResolution(out, "element_id", "element", **element_adapter, 3);

      if (assoc_field->Kind() == MethodHandlerKind::ArrayRef) {
        auto array_handler =
            static_cast<ArrayRefMethodHandler *>(assoc_field.get());
        out << "      arboretum_emit_" << array_handler->owned_table->name
            << "(c0, idx, element_id);\n";
      } else if (assoc_field->Kind() == MethodHandlerKind::IteratorRange) {
        auto iter_handler =
            static_cast<IteratorRangeMethodHandler *>(assoc_field.get());
        out << "      arboretum_emit_" << iter_handler->owned_table->name
            << "(c0, idx, element_id);\n";
        out << "      ++idx;\n";
      }

      out << "    }\n";
      out << "  }\n";
    }

    out << "  arboretum_emit_" << handler.owning_table->name << "(c0";
    for (size_t i = 0; i < handler.data_field_handlers.size(); ++i) {
      out << ", c" << (i + 1);
    }
    out << ");\n";
  }
}
}  // namespace

EmitReifyCppResult EmitReifyCpp(Model &model,
                                std::map<std::string, bool> property_table,
                                const std::string &reify_cpp_dir) {
  EmitReifyCppResult result;

  std::filesystem::path reify_cpp_dir_path(reify_cpp_dir);

  /////////////////////////////////////////////////////////////////////////////

  auto data_model_h = PartiallyGeneratedFile::Read(
      reify_cpp_dir_path / "include" / "arboretum_data_model.h");
  assert(data_model_h.Write([&](std::ostream &out) -> bool {
    for (const auto &[enum_usr, decl] : model.enums) {
      out << "  uint64_t resolve(" << decl->getQualifiedNameAsString()
          << " kind);\n";
    }
    return true;
  }));

  /////////////////////////////////////////////////////////////////////////////

  auto data_model_cc = PartiallyGeneratedFile::Read(reify_cpp_dir_path / "src" /
                                                    "arboretum_data_model.cc");
  assert(data_model_cc.Write([&](std::ostream &out) -> bool {
    auto enum_to_idx = model.GetEnumToIndex();
    auto enum_constant_to_idx = model.GetEnumConstantToIndex();

    for (const auto &[enum_usr, decl] : model.enums) {
      out << "  uint64_t resolve(" << decl->getQualifiedNameAsString()
          << " kind) {\n";
      out << "    switch (kind) {\n";
      for (const auto &enumerator : decl->enumerators()) {
        auto find_itr = enum_constant_to_idx.find(enumerator);
        if (find_itr == enum_constant_to_idx.end()) {
          continue;
        }

        out << "      case " << enumerator->getQualifiedNameAsString()
            << ": return " << find_itr->second << ";\n";
      }
      out << "      default: break;\n";
      out << "    }\n";
      out << "  assert(false && \"Invalid enum kind\");\n";
      out << "  return 0;\n";
      out << "  }\n";
    }
    return true;
  }));

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
  ast_visitor_cc.Write([&](std::ostream &out) -> bool {
    out << "// Types\n";
    for (const auto *type_decl :
         Sorted(model.index.clang.type_decls, AlphabetizeCXXRecordDecl)) {
      std::string decl_name = type_decl->getNameAsString();
      out << "bool ArboretumASTVisitor::Visit" << decl_name
          << "(clang::" << decl_name << "* D) {\n";

      EmitHandlerLogic(out, type_decl, model);

      out << "  return true;\n}\n\n";
    }

    out << "\n// TypeLocs\n";
    for (const auto *typeloc_decl :
         Sorted(model.index.clang.typeloc_decls, AlphabetizeCXXRecordDecl)) {
      std::string decl_name = typeloc_decl->getNameAsString();
      out << "bool ArboretumASTVisitor::Visit" << decl_name
          << "(clang::" << decl_name << " D) {\n";
      out << "  return true;\n";
      out << "}\n\n";
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

      EmitHandlerLogic(out, decl_decl, model);

      out << "  return true;\n}\n\n";
    }

    out << "\n// Stmts\n";
    for (const auto *stmt_decl :
         Sorted(model.index.clang.stmt_decls, AlphabetizeCXXRecordDecl)) {
      std::string decl_name = stmt_decl->getNameAsString();
      out << "bool ArboretumASTVisitor::Visit" << decl_name
          << "(clang::" << decl_name << "* D) {\n";

      EmitHandlerLogic(out, stmt_decl, model);

      out << "  return true;\n}\n\n";
    }

    return true;
  });

  return result;
}

}  // namespace arboretum