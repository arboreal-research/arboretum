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

struct HandlerFragment {
  HandlerFragment(
      std::function<std::string(const std::string &)> simple_handler)
      : handler_([=](size_t indent, const std::string &input)
                     -> std::pair<std::vector<std::string>, std::string> {
          std::vector<std::string> stmts;
          return std::make_pair(stmts, simple_handler(input));
        }) {}

  HandlerFragment(
      std::function<std::pair<std::vector<std::string>, std::string>(
          size_t, const std::string &)>
          handler)
      : handler_(handler) {}

  std::string operator()(
      size_t indent,
      std::function<std::string(size_t indent, const std::string &)>
          final_stmt_builder,
      const std::string &input_expr) {
    auto [stmts, expr] = handler_(indent, input_expr);
    std::stringstream out;
    if (stmts.size() == 0) {
      out << final_stmt_builder(indent, expr);
    } else {
      for (size_t i = 0; i < indent; ++i) out << " ";
      out << "{\n";
      for (const auto &stmt : stmts) {
        out << stmt;
      }
      out << final_stmt_builder(indent + 2, expr);
      for (size_t i = 0; i < indent; ++i) out << " ";
      out << "}\n";
    }
    return out.str();
  }

  std::function<std::pair<std::vector<std::string>, std::string>(
      size_t, const std::string &)>
      handler_;
};

std::optional<std::string> class_for(
    Model &model, clang::QualType type,
    std::map<std::string, const clang::EnumDecl *> &enums_to_emit) {
  if (type->isEnumeralType()) {
    const clang::EnumDecl *decl =
        llvm::dyn_cast<clang::EnumDecl>(type->getAsTagDecl());
    const clang::EnumDecl *decl_def = decl->getDefinition();
    if (decl_def == nullptr)
      llvm::errs() << "Missing definition for "
                   << decl->getQualifiedNameAsString() << "!\n";
    enums_to_emit.insert(
        std::make_pair(decl->getQualifiedNameAsString(), decl));
    return model.entity_name(decl);
  } else if (type->isPointerType()) {
    auto pointer_type = type->getAs<clang::PointerType>();
    auto pointee_type = pointer_type->getPointeeType();
    if (pointee_type->isRecordType()) {
      auto record_type = pointee_type->getAs<clang::RecordType>();
      auto record_decl =
          llvm::dyn_cast<clang::CXXRecordDecl>(record_type->getDecl());
      if (model.index.clang.all_decls.contains(record_decl)) {
        return model.entity_name(record_decl);
      }
    }
  } else if (type->isBuiltinType()) {
    auto builtin_type = type->getAs<clang::BuiltinType>();
    switch (builtin_type->getKind()) {
      case clang::BuiltinType::Bool: {
        return std::string("builtin_bool_class");
      } break;
      case clang::BuiltinType::UChar:
      case clang::BuiltinType::Char_U:
      case clang::BuiltinType::UShort:
      case clang::BuiltinType::UInt:
      case clang::BuiltinType::ULong:
      case clang::BuiltinType::ULongLong: {
        return std::string("builtin_u64_class");
      } break;
      case clang::BuiltinType::SChar:
      case clang::BuiltinType::Char_S:
      case clang::BuiltinType::Short:
      case clang::BuiltinType::Int:
      case clang::BuiltinType::Long:
      case clang::BuiltinType::LongLong: {
        return std::string("builtin_i64_class");
      } break;
      case clang::BuiltinType::Double: {
        return std::string("builtin_double_class");
      } break;
      default:
        break;
    }
  } else if (type->isRecordType()) {
    auto record_type = type->getAs<clang::RecordType>();
    auto record_decl =
        llvm::dyn_cast<clang::CXXRecordDecl>(record_type->getDecl());
    auto fqn = record_decl->getQualifiedNameAsString();
    if (fqn == "clang::QualType") {
      return std::string("qualtype_class");
    } else if (fqn == "clang::SourceLocation") {
      return std::string("source_location_class");
    } else if (fqn == "clang::SourceRange") {
      return std::string("source_range_class");
    } else if (fqn == "llvm::StringRef") {
      return std::string("builtin_string_class");
    } else if (fqn == "std::basic_string") {
      return std::string("builtin_string_class");
    } else if (fqn == "llvm::ArrayRef") {
      return std::string("builtin_list_class");
    } else if (fqn == "llvm::iterator_range") {
      return std::string("builtin_list_class");
    }
  }

  return std::nullopt;
}

std::optional<HandlerFragment> handler_for(
    Model &model, clang::QualType return_type,
    std::map<std::string, const clang::EnumDecl *> &enums_to_emit);

std::optional<HandlerFragment> list_handler_for(
    Model &model, clang::QualType element_type,
    std::map<std::string, const clang::EnumDecl *> &enums_to_emit) {
  std::optional<HandlerFragment> eh =
      handler_for(model, element_type, enums_to_emit);
  std::optional<std::string> entity_name =
      class_for(model, element_type, enums_to_emit);
  if (eh.has_value() && entity_name.has_value()) {
    return HandlerFragment{
        [eh, entity_name](size_t indent, const std::string &input_expr)
            -> std::pair<std::vector<std::string>, std::string> {
          HandlerFragment element_handler = *eh;

          std::vector<std::string> stmts;
          {
            std::stringstream out;
            for (size_t i = 0; i < indent + 2; ++i) out << " ";
            out << "std::vector<Id*> element_ids;\n";
            stmts.push_back(out.str());
          }
          stmts.push_back(element_handler(
              indent,
              [=](size_t indent, const std::string &e) -> std::string {
                std::stringstream out;
                for (size_t i = 0; i < indent + 2; ++i) out << " ";
                out << "auto range = " << input_expr << ";\n";
                for (size_t i = 0; i < indent + 2; ++i) out << " ";
                out << "for (auto itr=range.begin(); itr != range.end(); "
                       "++itr) {\n";
                for (size_t i = 0; i < indent + 2; ++i) out << " ";
                out << "  element_ids.push_back(" << e << ");\n";
                for (size_t i = 0; i < indent + 2; ++i) out << " ";
                out << "}\n";
                return out.str();
              },
              "(*itr)"));

          std::stringstream out;
          out << "context_.data_model_.arboretum_node_for(context_.data_model_."
              << *entity_name << "_, element_ids)";
          return std::make_pair(stmts, out.str());
        }};
  }
  return std::nullopt;
}

std::optional<HandlerFragment> handler_for(
    Model &model, clang::QualType return_type,
    std::map<std::string, const clang::EnumDecl *> &enums_to_emit) {
  if (return_type->isEnumeralType()) {
    const clang::EnumDecl *decl =
        llvm::dyn_cast<clang::EnumDecl>(return_type->getAsTagDecl());

    const clang::EnumDecl *decl_def = decl->getDefinition();
    if (decl_def == nullptr)
      llvm::errs() << "Missing definition for "
                   << decl->getQualifiedNameAsString() << "!\n";

    std::optional<std::string> enum_usr = getUSR(model.ast_ctx, decl);
    if (enum_usr.has_value()) {
      enums_to_emit.insert(
          std::make_pair(decl->getQualifiedNameAsString(), decl));

      return HandlerFragment{[](const std::string &hole) {
        return std::string("context_.data_model_.resolve(") + hole + ")";
      }};
    } else {
      llvm::errs() << "Failed to generator USR for "
                   << decl->getQualifiedNameAsString() << "!\n";
    }
  } else if (return_type->isPointerType()) {
    auto pointer_type = return_type->getAs<clang::PointerType>();
    auto pointee_type = pointer_type->getPointeeType();
    if (pointee_type->isRecordType()) {
      auto record_type = pointee_type->getAs<clang::RecordType>();
      auto record_decl =
          llvm::dyn_cast<clang::CXXRecordDecl>(record_type->getDecl());
      if (model.index.clang.all_decls.contains(record_decl)) {
        return HandlerFragment{[](const std::string &hole) {
          return std::string("context_.resolve(") + hole + ")";
        }};
      }
    }
  } else if (return_type->isBuiltinType()) {
    auto builtin_type = return_type->getAs<clang::BuiltinType>();
    switch (builtin_type->getKind()) {
      case clang::BuiltinType::Bool: {
        return HandlerFragment{[](const std::string &hole) {
          return std::string("context_.data_model_.arboretum_node_for(") +
                 hole + ")";
        }};
      } break;
      case clang::BuiltinType::UChar:
      case clang::BuiltinType::Char_U:
      case clang::BuiltinType::UShort:
      case clang::BuiltinType::UInt:
      case clang::BuiltinType::ULong:
      case clang::BuiltinType::ULongLong: {
        return HandlerFragment{[](const std::string &hole) {
          return std::string(
                     "context_.data_model_.arboretum_node_for(static_cast<"
                     "uint64_t>(") +
                 hole + "))";
        }};
      } break;
      case clang::BuiltinType::SChar:
      case clang::BuiltinType::Char_S:
      case clang::BuiltinType::Short:
      case clang::BuiltinType::Int:
      case clang::BuiltinType::Long:
      case clang::BuiltinType::LongLong: {
        return HandlerFragment{[](const std::string &hole) {
          return std::string(
                     "context_.data_model_.arboretum_node_for(static_cast<"
                     "int64_t>(") +
                 hole + "))";
        }};
      } break;
      case clang::BuiltinType::Double: {
        return HandlerFragment{[](const std::string &hole) {
          return std::string("context_.data_model_.arboretum_node_for(") +
                 hole + ")";
        }};
      } break;
      default:
        break;
    }
  } else if (return_type->isRecordType()) {
    auto record_type =
        return_type.getCanonicalType()->getAs<clang::RecordType>();
    auto record_decl =
        llvm::dyn_cast<clang::CXXRecordDecl>(record_type->getDecl());
    auto fqn = record_decl->getQualifiedNameAsString();
    if (fqn == "clang::QualType") {
      return HandlerFragment{[](const std::string &hole) {
        return std::string("context_.resolve(") + hole + ")";
      }};
    } else if (fqn == "clang::SourceLocation") {
      return HandlerFragment{[](const std::string &hole) {
        return std::string("context_.source_model_.resolve(") + hole + ")";
      }};
    } else if (fqn == "clang::SourceRange") {
      return HandlerFragment{[](const std::string &hole) {
        return std::string("context_.source_model_.resolve(") + hole + ")";
      }};
    } else if (fqn == "llvm::StringRef") {
      return HandlerFragment{[](const std::string &hole) {
        return std::string("context_.data_model_.arboretum_node_for(") + hole +
               ".str())";
      }};
    } else if (fqn == "std::basic_string") {
      return HandlerFragment{[](const std::string &hole) {
        return std::string("context_.data_model_.arboretum_node_for(") + hole +
               ")";
      }};
    } else if (fqn == "llvm::ArrayRef") {
      auto tmpl_record_decl =
          llvm::dyn_cast_or_null<clang::ClassTemplateSpecializationDecl>(
              record_decl);
      auto element_type = tmpl_record_decl->getTemplateArgs()[0].getAsType();
      return list_handler_for(model, element_type, enums_to_emit);
    } else if (fqn == "llvm::iterator_range") {
      auto iterator_range_arg =
          return_type->getAs<clang::TemplateSpecializationType>()
              ->template_arguments()[0]
              .getAsType()
              .getCanonicalType();

      if (iterator_range_arg->isPointerType()) {
        auto element_type = iterator_range_arg->getPointeeType();
        return list_handler_for(model, element_type, enums_to_emit);
      } else if (iterator_range_arg->isRecordType()) {
        auto iterator_decl = iterator_range_arg->getAsCXXRecordDecl();
        if (iterator_decl != nullptr) {
          clang::DeclarationName name =
              &iterator_decl->getASTContext().Idents.get("value_type");
          auto lookup = iterator_decl->lookup(name);
          if (lookup.isSingleResult()) {
            auto typedef_name_decl =
                llvm::dyn_cast_or_null<clang::TypedefNameDecl>(*lookup.begin());
            if (typedef_name_decl != nullptr) {
              auto element_type =
                  typedef_name_decl->getUnderlyingType().getCanonicalType();
              return list_handler_for(model, element_type, enums_to_emit);
            }
          }
        }
      }
    }
  }

  return std::nullopt;
}

EmitReifyCppResult EmitReifyCpp(Model &model,
                                std::map<std::string, bool> property_table,
                                const std::string &reify_cpp_dir) {
  EmitReifyCppResult result;

  std::filesystem::path reify_cpp_dir_path(reify_cpp_dir);

  assert(model.index.clang.cfg_terminator_kind != nullptr);
  result.enums_to_emit.insert(std::make_pair(
      model.index.clang.cfg_terminator_kind->getQualifiedNameAsString(),
      model.index.clang.cfg_terminator_kind));

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
    auto emit_methods = [&](const clang::CXXRecordDecl *decl,
                            const std::string &decl_name) {
      for (const auto &method_decl : decl->methods()) {
        std::string method_name = method_decl->getNameAsString();
        std::optional<std::string> method_usr =
            getUSR(model.ast_ctx, method_decl);
        if (!method_usr.has_value()) continue;

        {
          auto find_itr = property_table.find(*method_usr);
          if (find_itr == property_table.end() || !find_itr->second) continue;
        }

        std::string method_decl_Id = model.entity_name(method_decl);

        auto return_type = method_decl->getReturnType();
        std::optional<HandlerFragment> handler_opt =
            handler_for(model, return_type, result.enums_to_emit);
        if (handler_opt.has_value()) {
          out << (*handler_opt)(
              2,
              [&](size_t indent, const std::string &e) {
                std::stringstream out;
                for (size_t i = 0; i < indent; ++i) out << " ";
                out << "arboretum_create_edge(obj, context_.data_model_." +
                           method_decl_Id + "_, " + e + ");\n";
                return out.str();
              },
              std::string("D->") + method_name + "()");
        } else {
          out << "  // " << method_name << " ( "
              << return_type.getCanonicalType().getAsString() << " )\n";
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

      if (type_decl == model.index.clang.type_decl) {
        out << "  switch(D->getTypeClass()) {\n";
        for (auto [decl, enum_decl] :
             model.index.clang.typeclass_enum_by_decl) {
          std::string decl_Id = model.entity_name(decl);

          out << "    case " << enum_decl->getQualifiedNameAsString() << ": \n";
          out << "      arboretum_create_edge(obj, "
                 "context_.data_model_.meta_class_, context_.data_model_."
              << decl_Id << "_);\n";
          out << "      break; \n";
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

      out << "  const Id* obj = context_.resolve(D);\n";

      if (decl_decl == model.index.clang.decl_decl) {
        out << "  switch(D->getKind()) {\n";
        for (auto [decl, enum_decl] : model.index.clang.declkind_enum_by_decl) {
          std::string decl_Id = model.entity_name(decl);

          out << "    case " << enum_decl->getQualifiedNameAsString() << ": \n";
          out << "      arboretum_create_edge(obj, "
                 "context_.data_model_.meta_class_, context_.data_model_."
              << decl_Id << "_);\n";
          out << "      break; \n";
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

          out << "    case " << enum_decl->getQualifiedNameAsString() << ": \n";
          out << "      arboretum_create_edge(obj, "
                 "context_.data_model_.meta_class_, context_.data_model_."
              << decl_Id << "_);\n";
          out << "      break; \n";
        }
        out << "    default: break;\n";
        out << "  }\n\n";
      }

      emit_methods(stmt_decl, decl_name);
      out << "  return true;\n}\n\n";
    }

    return true;
  });

  /////////////////////////////////////////////////////////////////////////////

  auto data_model_h = PartiallyGeneratedFile::Read(
      reify_cpp_dir_path / "include" / "arboretum_data_model.h");
  assert(data_model_h.Write([&](std::ostream &out) -> bool {
    out << "  DataModel();\n";

    for (const auto &[var, name] : model.meta_data_model) {
      out << "  Id* " << var << "_ = nullptr;\n";
    }

    for (auto &decl_decl : model.index.clang.all_decls) {
      out << "  Id* " << model.entity_name(decl_decl) << "_ = nullptr;\n";
      for (auto method_decl : decl_decl->methods()) {
        std::optional<std::string> method_decl_usr =
            getUSR(model.ast_ctx, method_decl);
        if (!method_decl_usr.has_value() ||
            !property_table.contains(*method_decl_usr)) {
          continue;
        }

        out << "  Id* " << model.entity_name(method_decl) << "_ = nullptr;\n";
      }
    }

    for (auto &[enum_name, enum_decl] : result.enums_to_emit) {
      out << "  Id* resolve(" << enum_name << " e);\n";
      out << "  Id* " << model.entity_name(enum_decl) << "_ = nullptr;\n";
      for (const auto &enum_value_decl : enum_decl->enumerators()) {
        out << "  Id* " << model.entity_name(enum_value_decl)
            << "_ = nullptr;\n";
      }
    }

    return true;
  }));

  /////////////////////////////////////////////////////////////////////////////

  auto data_model_cc = PartiallyGeneratedFile::Read(reify_cpp_dir_path / "src" /
                                                    "arboretum_data_model.cc");
  assert(data_model_cc.Write([&](std::ostream &out) -> bool {
    out << "DataModel::DataModel() {\n";
    for (const auto &[var, name] : model.meta_data_model) {
      out << "  " << var << "_ = arboretum_create_nameless_node_with_id("
          << result.name_registry.fqn_to_id(1, name) << ");\n";
    }
    out << "}\n\n";

    out << "DataModel EmitDataModel() {\n";
    out << "  DataModel data_model;\n\n";

    // Assign a named Id to the appropriate data model field for that decl.
    for (auto &decl_decl : model.index.clang.all_decls) {
      std::string decl_decl_Id = model.entity_name(decl_decl);

      out << "  data_model." << decl_decl_Id
          << "_ = arboretum_create_nameless_node_with_id("
          << result.name_registry.fqn_to_id(
                 1, decl_decl->getQualifiedNameAsString())
          << ");\n";

      // Emit the methods
      for (auto method_decl : decl_decl->methods()) {
        std::optional<std::string> method_decl_usr =
            getUSR(model.ast_ctx, method_decl);
        if (!method_decl_usr.has_value() ||
            !property_table.contains(*method_decl_usr)) {
          continue;
        }

        std::string method_decl_fqn = method_decl->getQualifiedNameAsString();
        std::string method_decl_Id = model.entity_name(method_decl);

        out << "  data_model." << method_decl_Id
            << "_ = arboretum_create_nameless_node_with_id("
            << result.name_registry.fqn_to_id(1, method_decl_fqn) << ");\n";
      }
    }

    for (auto &[enum_name, enum_decl] : result.enums_to_emit) {
      std::string enum_decl_fqn = enum_decl->getQualifiedNameAsString();
      std::string enum_decl_Id = model.entity_name(enum_decl);

      out << "  data_model." << enum_decl_Id
          << "_ = arboretum_create_nameless_node_with_id("
          << result.name_registry.fqn_to_id(1, enum_decl_fqn) << ");\n";

      for (const auto &enum_value_decl : enum_decl->enumerators()) {
        std::string enum_value_decl_fqn =
            enum_value_decl->getQualifiedNameAsString();
        std::string enum_value_decl_Id = model.entity_name(enum_value_decl);

        out << "  data_model." << enum_value_decl_Id
            << "_ = arboretum_create_nameless_node_with_id("
            << result.name_registry.fqn_to_id(1, enum_value_decl_fqn) << ");\n";
      }
    }
    out << "  return data_model;\n";
    out << "}\n\n";

    for (auto &[enum_name, enum_decl] : result.enums_to_emit) {
      out << "Id* DataModel::resolve(" << enum_name << " e) {\n";
      out << "  switch(e) {\n";

      std::set<llvm::APSInt> seen_values;
      for (const auto &enum_value_decl : enum_decl->enumerators()) {
        std::string enum_value_name =
            enum_value_decl->getQualifiedNameAsString();

        llvm::APSInt enum_value = enum_value_decl->getInitVal();

        auto find_itr = seen_values.find(enum_value);
        if (find_itr != seen_values.end()) continue;
        seen_values.insert(enum_value);

        out << "    case " << enum_value_name << ": return "
            << model.entity_name(enum_value_decl) << "_;\n";
      }

      std::string enum_type = enum_decl->getIntegerType().getAsString();
      if (enum_type == "_Bool") enum_type = "bool";
      if (enum_type == "uint8_t") enum_type = "uint32_t";
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

  return result;
}

}  // namespace arboretum