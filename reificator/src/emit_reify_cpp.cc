#include "emit_reify_cpp.h"

#include <filesystem>
#include <map>
#include <optional>
#include <set>
#include <string>

#include "get_usr.h"
#include "model.h"
#include "partially_generated_file.h"
#include "util.h"

namespace arboretum {

std::string to_upper(const std::string &s) {
  std::string result;
  for (const char &c : s) {
    result.push_back(std::toupper(c));
  }
  return result;
}

bool AlphabetizeCXXRecordDecl(const clang::CXXRecordDecl *a, const clang::CXXRecordDecl *b) {
  return a->getNameAsString() < b->getNameAsString();
}

struct HandlerFragment {
  HandlerFragment(std::function<std::string(const std::string &)> simple_handler)
      : handler_([=](size_t indent, const std::string &input) -> std::pair<std::vector<std::string>, std::string> {
          std::vector<std::string> stmts;
          return std::make_pair(stmts, simple_handler(input));
        }) {}

  HandlerFragment(std::function<std::pair<std::vector<std::string>, std::string>(size_t, const std::string &)> handler)
      : handler_(handler) {}

  std::string operator()(size_t indent,
                         std::function<std::string(size_t indent, const std::string &)> final_stmt_builder,
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

  std::function<std::pair<std::vector<std::string>, std::string>(size_t, const std::string &)> handler_;
};

std::optional<std::string> class_for(Model &model, clang::QualType type,
                                     std::map<std::string, const clang::EnumDecl *> &enums_to_emit) {
  if (type->isEnumeralType()) {
    const clang::EnumDecl *decl = llvm::dyn_cast<clang::EnumDecl>(type->getAsTagDecl());
    const clang::EnumDecl *decl_def = decl->getDefinition();
    if (decl_def == nullptr) llvm::errs() << "Missing definition for " << decl->getQualifiedNameAsString() << "!\n";
    enums_to_emit.insert(std::make_pair(decl->getQualifiedNameAsString(), decl));
    return model.entity_name(decl);
  } else if (type->isPointerType()) {
    auto pointer_type = type->getAs<clang::PointerType>();
    auto pointee_type = pointer_type->getPointeeType();
    if (pointee_type->isRecordType()) {
      auto record_type = pointee_type->getAs<clang::RecordType>();
      auto record_decl = llvm::dyn_cast<clang::CXXRecordDecl>(record_type->getDecl());
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
    auto record_decl = llvm::dyn_cast<clang::CXXRecordDecl>(record_type->getDecl());
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

std::optional<HandlerFragment> handler_for(Model &model, clang::QualType return_type,
                                           std::map<std::string, const clang::EnumDecl *> &enums_to_emit);

std::optional<HandlerFragment> list_handler_for(Model &model, clang::QualType element_type,
                                                std::map<std::string, const clang::EnumDecl *> &enums_to_emit) {
  std::optional<HandlerFragment> eh = handler_for(model, element_type, enums_to_emit);
  std::optional<std::string> entity_name = class_for(model, element_type, enums_to_emit);
  if (eh.has_value() && entity_name.has_value()) {
    return HandlerFragment{
        [eh, entity_name](size_t indent,
                          const std::string &input_expr) -> std::pair<std::vector<std::string>, std::string> {
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
                out << "for (auto itr=range.begin(); itr != range.end(); ++itr) {\n";
                for (size_t i = 0; i < indent + 2; ++i) out << " ";
                out << "  element_ids.push_back(" << e << ");\n";
                for (size_t i = 0; i < indent + 2; ++i) out << " ";
                out << "}\n";
                return out.str();
              },
              "(*itr)"));

          std::stringstream out;
          out << "context_.data_model_.arboretum_node_for(context_.data_model_." << *entity_name << "_, element_ids)";
          return std::make_pair(stmts, out.str());
        }};
  }
  return std::nullopt;
}

std::optional<HandlerFragment> handler_for(Model &model, clang::QualType return_type,
                                           std::map<std::string, const clang::EnumDecl *> &enums_to_emit) {
  if (return_type->isEnumeralType()) {
    const clang::EnumDecl *decl = llvm::dyn_cast<clang::EnumDecl>(return_type->getAsTagDecl());

    const clang::EnumDecl *decl_def = decl->getDefinition();
    if (decl_def == nullptr) llvm::errs() << "Missing definition for " << decl->getQualifiedNameAsString() << "!\n";

    std::optional<std::string> enum_usr = getUSR(model.ast_ctx, decl);
    if (enum_usr.has_value()) {
      enums_to_emit.insert(std::make_pair(decl->getQualifiedNameAsString(), decl));

      return HandlerFragment{
          [](const std::string &hole) { return std::string("context_.data_model_.resolve(") + hole + ")"; }};
    } else {
      llvm::errs() << "Failed to generator USR for " << decl->getQualifiedNameAsString() << "!\n";
    }
  } else if (return_type->isPointerType()) {
    auto pointer_type = return_type->getAs<clang::PointerType>();
    auto pointee_type = pointer_type->getPointeeType();
    if (pointee_type->isRecordType()) {
      auto record_type = pointee_type->getAs<clang::RecordType>();
      auto record_decl = llvm::dyn_cast<clang::CXXRecordDecl>(record_type->getDecl());
      if (model.index.clang.all_decls.contains(record_decl)) {
        return HandlerFragment{[](const std::string &hole) { return std::string("context_.resolve(") + hole + ")"; }};
      }
    }
  } else if (return_type->isBuiltinType()) {
    auto builtin_type = return_type->getAs<clang::BuiltinType>();
    switch (builtin_type->getKind()) {
      case clang::BuiltinType::Bool: {
        return HandlerFragment{[](const std::string &hole) {
          return std::string("context_.data_model_.arboretum_node_for(") + hole + ")";
        }};
      } break;
      case clang::BuiltinType::UChar:
      case clang::BuiltinType::Char_U:
      case clang::BuiltinType::UShort:
      case clang::BuiltinType::UInt:
      case clang::BuiltinType::ULong:
      case clang::BuiltinType::ULongLong: {
        return HandlerFragment{[](const std::string &hole) {
          return std::string("context_.data_model_.arboretum_node_for(static_cast<uint64_t>(") + hole + "))";
        }};
      } break;
      case clang::BuiltinType::SChar:
      case clang::BuiltinType::Char_S:
      case clang::BuiltinType::Short:
      case clang::BuiltinType::Int:
      case clang::BuiltinType::Long:
      case clang::BuiltinType::LongLong: {
        return HandlerFragment{[](const std::string &hole) {
          return std::string("context_.data_model_.arboretum_node_for(static_cast<int64_t>(") + hole + "))";
        }};
      } break;
      case clang::BuiltinType::Double: {
        return HandlerFragment{[](const std::string &hole) {
          return std::string("context_.data_model_.arboretum_node_for(") + hole + ")";
        }};
      } break;
      default:
        break;
    }
  } else if (return_type->isRecordType()) {
    auto record_type = return_type.getCanonicalType()->getAs<clang::RecordType>();
    auto record_decl = llvm::dyn_cast<clang::CXXRecordDecl>(record_type->getDecl());
    auto fqn = record_decl->getQualifiedNameAsString();
    if (fqn == "clang::QualType") {
      return HandlerFragment{[](const std::string &hole) { return std::string("context_.resolve(") + hole + ")"; }};
    } else if (fqn == "clang::SourceLocation") {
      return HandlerFragment{
          [](const std::string &hole) { return std::string("context_.source_model_.resolve(") + hole + ")"; }};
    } else if (fqn == "clang::SourceRange") {
      return HandlerFragment{
          [](const std::string &hole) { return std::string("context_.source_model_.resolve(") + hole + ")"; }};
    } else if (fqn == "llvm::StringRef") {
      return HandlerFragment{[](const std::string &hole) {
        return std::string("context_.data_model_.arboretum_node_for(") + hole + ".str())";
      }};
    } else if (fqn == "std::basic_string") {
      return HandlerFragment{
          [](const std::string &hole) { return std::string("context_.data_model_.arboretum_node_for(") + hole + ")"; }};
    } else if (fqn == "llvm::ArrayRef") {
      auto tmpl_record_decl = llvm::dyn_cast_or_null<clang::ClassTemplateSpecializationDecl>(record_decl);
      auto element_type = tmpl_record_decl->getTemplateArgs()[0].getAsType();
      return list_handler_for(model, element_type, enums_to_emit);
    } else if (fqn == "llvm::iterator_range") {
      auto iterator_range_arg = return_type->getAs<clang::TemplateSpecializationType>()
                                    ->template_arguments()[0]
                                    .getAsType()
                                    .getCanonicalType();

      if (iterator_range_arg->isPointerType()) {
        auto element_type = iterator_range_arg->getPointeeType();
        return list_handler_for(model, element_type, enums_to_emit);
      } else if (iterator_range_arg->isRecordType()) {
        auto iterator_decl = iterator_range_arg->getAsCXXRecordDecl();
        if (iterator_decl != nullptr) {
          clang::DeclarationName name = &iterator_decl->getASTContext().Idents.get("value_type");
          auto lookup = iterator_decl->lookup(name);
          if (lookup.isSingleResult()) {
            auto typedef_name_decl = llvm::dyn_cast_or_null<clang::TypedefNameDecl>(*lookup.begin());
            if (typedef_name_decl != nullptr) {
              auto element_type = typedef_name_decl->getUnderlyingType().getCanonicalType();
              return list_handler_for(model, element_type, enums_to_emit);
            }
          }
        }
      }
    }
  }

  return std::nullopt;
}

void EmitReifyCpp(Model &model, std::map<std::string, bool> property_table, const std::string &reify_cpp_dir,
                  const std::string &reify_rs_dir) {
  std::filesystem::path reify_cpp_dir_path(reify_cpp_dir);
  std::filesystem::path reify_rs_dir_path(reify_rs_dir);

  std::map<std::string, const clang::EnumDecl *> enums_to_emit;
  assert(model.index.clang.cfg_terminator_kind != nullptr);
  enums_to_emit.insert(std::make_pair(model.index.clang.cfg_terminator_kind->getQualifiedNameAsString(),
                                      model.index.clang.cfg_terminator_kind));

  std::map<std::string, uint64_t> id_by_name;
  uint64_t next_id = 0;

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

  meta_data_model.push_back(std::pair("meta_class", "/meta/class"));
  meta_data_model.push_back(std::pair("meta_subclass", "/meta/subclass"));
  meta_data_model.push_back(std::pair("meta_method", "/meta/method"));
  meta_data_model.push_back(std::pair("meta_domain", "/meta/domain"));
  meta_data_model.push_back(std::pair("meta_range", "/meta/range"));

  meta_data_model.push_back(std::pair("meta_clang_ast_node", "/meta/clang_ast/node"));
  meta_data_model.push_back(std::pair("meta_clang_ast_method", "/meta/clang_ast/method"));
  meta_data_model.push_back(std::pair("meta_clang_ast_enum", "/meta/clang_ast/enum"));
  meta_data_model.push_back(std::pair("meta_clang_ast_enum_enumerators", "/meta/clang_ast/enum/enumerators"));
  meta_data_model.push_back(std::pair("meta_clang_ast_enum_constant", "/meta/clang_ast/enum_constant"));

  meta_data_model.push_back(std::pair("builtin_string_class", "/builtin/string"));
  meta_data_model.push_back(std::pair("builtin_u64_class", "/builtin/u64"));
  meta_data_model.push_back(std::pair("builtin_i64_class", "/builtin/i64"));
  meta_data_model.push_back(std::pair("builtin_double_class", "/builtin/dbl"));
  meta_data_model.push_back(std::pair("builtin_list_class", "/builtin/list"));
  meta_data_model.push_back(std::pair("builtin_list_size", "/builtin/list/size"));
  meta_data_model.push_back(std::pair("builtin_list_item_class", "/builtin/list/item_class"));
  meta_data_model.push_back(std::pair("builtin_set_class", "/builtin/set"));
  meta_data_model.push_back(std::pair("builtin_set_size", "/builtin/set/size"));
  meta_data_model.push_back(std::pair("builtin_set_item", "/builtin/set/item"));
  meta_data_model.push_back(std::pair("builtin_set_item_class", "/builtin/set/item_class"));

  meta_data_model.push_back(std::pair("invalid_file", "/invalid/file"));
  meta_data_model.push_back(std::pair("invalid_source_location", "/invalid/clang/SourceLocation"));

  meta_data_model.push_back(std::pair("file_class", "/file"));
  meta_data_model.push_back(std::pair("file_name", "/file/name"));
  meta_data_model.push_back(std::pair("file_content", "/file/content"));

  meta_data_model.push_back(std::pair("source_location_class", "/clang/SourceLocation"));
  meta_data_model.push_back(std::pair("source_location_is_file", "/clang/SourceLocation/is_file"));
  meta_data_model.push_back(std::pair("source_location_file", "/clang/SourceLocation/file"));
  meta_data_model.push_back(std::pair("source_location_line", "/clang/SourceLocation/line"));
  meta_data_model.push_back(std::pair("source_location_column", "/clang/SourceLocation/column"));
  meta_data_model.push_back(std::pair("source_location_expansion_loc", "/clang/SourceLocation/expansion_loc"));
  meta_data_model.push_back(std::pair("source_location_spelling_loc", "/clang/SourceLocation/spelling_loc"));

  meta_data_model.push_back(std::pair("source_range_class", "/clang/SourceRange"));
  meta_data_model.push_back(std::pair("source_range_begin", "/clang/SourceRange/begin"));
  meta_data_model.push_back(std::pair("source_range_end", "/clang/SourceLocation/end"));

  meta_data_model.push_back(std::pair("qualtype_class", "/clang/QualType"));
  meta_data_model.push_back(std::pair("qualtype_is_const", "/clang/QualType/is_const"));
  meta_data_model.push_back(std::pair("qualtype_is_volatile", "/clang/QualType/is_volatile"));
  meta_data_model.push_back(std::pair("qualtype_is_restrict", "/clang/QualType/is_restrict"));
  meta_data_model.push_back(std::pair("qualtype_type", "/clang/Qualtype/type"));

  meta_data_model.push_back(std::pair("usr", "/clang/usr"));

  meta_data_model.push_back(std::pair("class_CFG", "clang::CFG"));
  meta_data_model.push_back(std::pair("cfg", "/clang/cfg"));
  meta_data_model.push_back(std::pair("cfg_nodes", "/clang/cfg/nodes"));
  meta_data_model.push_back(std::pair("cfg_entry", "/clang/cfg/entry"));
  meta_data_model.push_back(std::pair("cfg_exit", "/clang/cfg/exit"));
  meta_data_model.push_back(std::pair("cfg_indirect_goto_block", "/clang/cfg/indirect_goto_block"));
  meta_data_model.push_back(std::pair("cfg_try_blocks", "/clang/cfg/try_blocks"));
  meta_data_model.push_back(std::pair("cfg_is_linear", "/clang/cfg/is_linear"));

  meta_data_model.push_back(std::pair("class_CFGBlock", "clang:CFGBlock"));
  meta_data_model.push_back(std::pair("cfg_block_parent", "/clang/cfg_block/parents"));
  meta_data_model.push_back(std::pair("cfg_block_succs", "/clang/cfg_block/succs"));
  meta_data_model.push_back(std::pair("cfg_block_preds", "/clang/cfg_block/preds"));
  meta_data_model.push_back(std::pair("cfg_block_label", "/clang/cfg_block/label"));
  meta_data_model.push_back(std::pair("cfg_block_terminator_stmt", "/clang/cfg_block/terminator_stmt"));
  meta_data_model.push_back(std::pair("cfg_block_terminator_kind", "/clang/cfg_block/terminator_kind"));
  meta_data_model.push_back(std::pair("cfg_block_terminator_condition", "/clang/cfg_block/terminator_condition"));
  meta_data_model.push_back(std::pair("cfg_block_loop_target", "/clang/cfg_block/loop_target"));
  meta_data_model.push_back(std::pair("cfg_block_has_no_return_element", "/clang/cfg_block/has_no_return_element"));

  meta_data_model.push_back(std::pair("class_CFGElement", "clang::CFGElement"));
  meta_data_model.push_back(std::pair("class_CFGCleanupFunction", "clang::CFGCleanupFunction"));
  meta_data_model.push_back(std::pair("class_CFGImplicitDtor", "clang::CFGImplicitDtor"));
  meta_data_model.push_back(std::pair("class_CFGInitializer", "clang::CFGInitializer"));
  meta_data_model.push_back(std::pair("class_CFGLifetimeEnds", "clang::CFGLifetimeEnds"));
  meta_data_model.push_back(std::pair("class_CFGLoopExit", "clang::CFGLoopExit"));
  meta_data_model.push_back(std::pair("class_CFGNewAllocator", "clang::CFGNewAllocator"));
  meta_data_model.push_back(std::pair("class_CFGScopeBegin", "clang::CFGScopeBegin"));
  meta_data_model.push_back(std::pair("class_CFGScopeEnd", "clang::CFGScopeEnd"));
  meta_data_model.push_back(std::pair("class_CFGStmt", "clang::CFGStmt"));
  meta_data_model.push_back(std::pair("class_CFGAutomaticObjDtor", "clang::CFGAutomaticObjDtor"));
  meta_data_model.push_back(std::pair("class_CFGBaseDtor", "clang::CFGBaseDtor"));
  meta_data_model.push_back(std::pair("class_CFGDeleteDtor", "clang::CFGDeleteDtor"));
  meta_data_model.push_back(std::pair("class_CFGMemberDtor", "clang::CFGMemberDtor"));
  meta_data_model.push_back(std::pair("class_CFGTemporaryDtor", "clang::CFGTemporaryDtor"));
  meta_data_model.push_back(std::pair("class_CFGCXXRecordTypedCall", "clang::CFGCXXRecordTypedCall"));
  meta_data_model.push_back(std::pair("class_CFGConstructor", "clang::CFGConstructor"));

  meta_data_model.push_back(std::pair("cfg_element_trigger_stmt", "/clang/cfg_element/trigger_stmt"));
  meta_data_model.push_back(std::pair("cfg_element_var_decl", "/clang/cfg_element/var_decl"));
  meta_data_model.push_back(std::pair("cfg_element_alloc_expr", "/clang/cfg_element/alloc_expr"));
  meta_data_model.push_back(std::pair("cfg_element_loop_stmt", "/clang/cfg_element/loop_stmt"));
  meta_data_model.push_back(std::pair("cfg_element_stmt", "/clang/cfg_element/stmt"));
  meta_data_model.push_back(std::pair("cfg_element_ctor_context", "/clang/cfg_element/ctor_context"));
  meta_data_model.push_back(std::pair("cfg_element_dtor_decl", "/clang/cfg_element/dtor_decl"));
  meta_data_model.push_back(std::pair("cfg_element_is_no_return", "/clang/cfg_element/is_no_return"));
  meta_data_model.push_back(std::pair("cfg_element_init", "/clang/cfg_element/init"));
  meta_data_model.push_back(std::pair("cfg_element_cxx_record_decl", "/clang/cfg_element/cxx_record_decl"));
  meta_data_model.push_back(std::pair("cfg_element_delete_expr", "/clang/cfg_element/delete_expr"));
  meta_data_model.push_back(std::pair("cfg_element_base_specifier", "/clang/cfg_element/base_specifier"));
  meta_data_model.push_back(std::pair("cfg_element_field_decl", "/clang/cfg_element/field_decl"));
  meta_data_model.push_back(std::pair("cfg_element_bind_temporary_expr", "/clang/cfg_element/bind_temporary_expr"));
  meta_data_model.push_back(std::pair("cfg_element_function_decl", "/clang/cfg_element/function_decl"));

  /////////////////////////////////////////////////////////////////////////////

  auto ast_visitor_h = PartiallyGeneratedFile::Read(reify_cpp_dir_path / "include" / "arboretum_ast_visitor.h");
  assert(ast_visitor_h.Write([&](std::ostream &out) -> bool {
    out << "  // Types\n";
    for (const auto *type_decl : Sorted(model.index.clang.type_decls, AlphabetizeCXXRecordDecl)) {
      std::string decl_name = type_decl->getNameAsString();
      out << "  bool Visit" << decl_name << "(clang::" << decl_name << "* T);\n";
    }

    out << "\n  // TypeLocs\n";
    for (const auto *typeloc_decl : Sorted(model.index.clang.typeloc_decls, AlphabetizeCXXRecordDecl)) {
      std::string decl_name = typeloc_decl->getNameAsString();
      out << "  bool Visit" << decl_name << "(clang::" << decl_name << " TL);\n ";
    }

    out << "\n  // Decls\n";
    for (const auto *decl_decl : Sorted(model.index.clang.decl_decls, AlphabetizeCXXRecordDecl)) {
      std::string decl_name = decl_decl->getNameAsString();
      out << "  bool Visit" << decl_name << "(clang::" << decl_name << "* D);\n";
    }

    out << "\n  // Stmts\n";
    for (const auto *stmt_decl : Sorted(model.index.clang.stmt_decls, AlphabetizeCXXRecordDecl)) {
      std::string decl_name = stmt_decl->getNameAsString();
      out << "  bool Visit" << decl_name << "(clang::" << decl_name << "* S);\n";
    }
    return true;
  }));

  /////////////////////////////////////////////////////////////////////////////

  auto ast_visitor_cc = PartiallyGeneratedFile::Read(reify_cpp_dir_path / "src" / "arboretum_ast_visitor.cc");
  ast_visitor_cc.Write([&](std::ostream &out) -> bool {
    auto emit_methods = [&](const clang::CXXRecordDecl *decl, const std::string &decl_name) {
      for (const auto &method_decl : decl->methods()) {
        std::string method_name = method_decl->getNameAsString();
        std::optional<std::string> method_usr = getUSR(model.ast_ctx, method_decl);
        if (!method_usr.has_value()) continue;

        {
          auto find_itr = property_table.find(*method_usr);
          if (find_itr == property_table.end() || !find_itr->second) continue;
        }

        std::string method_decl_Id = model.entity_name(method_decl);

        auto return_type = method_decl->getReturnType();
        std::optional<HandlerFragment> handler_opt = handler_for(model, return_type, enums_to_emit);
        if (handler_opt.has_value()) {
          out << (*handler_opt)(
              2,
              [&](size_t indent, const std::string &e) {
                std::stringstream out;
                for (size_t i = 0; i < indent; ++i) out << " ";
                out << "arboretum_create_edge(obj, context_.data_model_." + method_decl_Id + "_, " + e + ");\n";
                return out.str();
              },
              std::string("D->") + method_name + "()");
        } else {
          out << "  // " << method_name << " ( " << return_type.getCanonicalType().getAsString() << " )\n";
        }
      }
    };

    out << "// Types\n";
    for (const auto *type_decl : Sorted(model.index.clang.type_decls, AlphabetizeCXXRecordDecl)) {
      std::string decl_name = type_decl->getNameAsString();
      out << "bool ArboretumASTVisitor::Visit" << decl_name << "(clang::" << decl_name << "* D) {\n";

      out << "  const Id* obj = context_.resolve(D);\n";

      if (type_decl == model.index.clang.type_decl) {
        out << "  switch(D->getTypeClass()) {\n";
        for (auto [decl, enum_decl] : model.index.clang.typeclass_enum_by_decl) {
          std::string decl_Id = model.entity_name(decl);

          out << "    case " << enum_decl->getQualifiedNameAsString() << ": \n";
          out << "      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_." << decl_Id
              << "_);\n";
          out << "      break; \n";
        }
        out << "    default: break;\n";
        out << "  }\n\n";
      }

      emit_methods(type_decl, decl_name);
      out << "  return true;\n}\n\n";
    }

    out << "\n// TypeLocs\n";
    for (const auto *typeloc_decl : Sorted(model.index.clang.typeloc_decls, AlphabetizeCXXRecordDecl)) {
      std::string decl_name = typeloc_decl->getNameAsString();
      out << "bool ArboretumASTVisitor::Visit" << decl_name << "(clang::" << decl_name << " D) {\n";
      out << "  return true;\n";
      out << "}\n\n";
    }

    out << "\n// Decls\n";
    for (const auto *decl_decl : Sorted(model.index.clang.decl_decls, AlphabetizeCXXRecordDecl)) {
      std::string decl_name = decl_decl->getNameAsString();
      out << "bool ArboretumASTVisitor::Visit" << decl_name << "(clang::" << decl_name << "* D) {\n";

      if (decl_name == "FunctionDecl" || decl_name == "CXXMethodDecl" || decl_name == "CXXConstructorDecl" ||
          decl_name == "CXXConversionDecl" || decl_name == "CXXDestructorDecl" || decl_name == "TagDecl" ||
          decl_name == "EnumDecl" || decl_name == "RecordDecl" || decl_name == "CXXRecordDecl" ||
          decl_name == "ClassTemplateSpecializationDecl" || decl_name == "ClassTemplatePartialSpecializationDecl") {
        out << "  if (!D->isThisDeclarationADefinition()) return true;\n\n";
      }

      out << "  const Id* obj = context_.resolve(D);\n";

      if (decl_decl == model.index.clang.decl_decl) {
        out << "  switch(D->getKind()) {\n";
        for (auto [decl, enum_decl] : model.index.clang.declkind_enum_by_decl) {
          std::string decl_Id = model.entity_name(decl);

          out << "    case " << enum_decl->getQualifiedNameAsString() << ": \n";
          out << "      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_." << decl_Id
              << "_);\n";
          out << "      break; \n";
        }
        out << "    default: break;\n";
        out << "  }\n\n";
      }

      emit_methods(decl_decl, decl_name);
      out << "  return true;\n}\n\n";
    }

    out << "\n// Stmts\n";
    for (const auto *stmt_decl : Sorted(model.index.clang.stmt_decls, AlphabetizeCXXRecordDecl)) {
      std::string decl_name = stmt_decl->getNameAsString();
      out << "bool ArboretumASTVisitor::Visit" << decl_name << "(clang::" << decl_name << "* D) {\n";

      out << "  const Id* obj = context_.resolve(D);\n";

      if (stmt_decl == model.index.clang.stmt_decl) {
        out << "  switch(D->getStmtClass()) {\n";
        for (auto [decl, enum_decl] : model.index.clang.stmtclass_enum_by_decl) {
          std::string decl_Id = model.entity_name(decl);

          out << "    case " << enum_decl->getQualifiedNameAsString() << ": \n";
          out << "      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_." << decl_Id
              << "_);\n";
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

  auto data_model_h = PartiallyGeneratedFile::Read(reify_cpp_dir_path / "include" / "arboretum_data_model.h");
  assert(data_model_h.Write([&](std::ostream &out) -> bool {
    out << "  DataModel();\n";

    for (const auto &[var, name] : meta_data_model) {
      out << "  Id* " << var << "_ = nullptr;\n";
    }

    for (auto &decl_decl : model.index.clang.all_decls) {
      out << "  Id* " << model.entity_name(decl_decl) << "_ = nullptr;\n";
      for (auto method_decl : decl_decl->methods()) {
        std::optional<std::string> method_decl_usr = getUSR(model.ast_ctx, method_decl);
        if (!method_decl_usr.has_value() || !property_table.contains(*method_decl_usr)) {
          continue;
        }

        out << "  Id* " << model.entity_name(method_decl) << "_ = nullptr;\n";
      }
    }

    for (auto &[enum_name, enum_decl] : enums_to_emit) {
      out << "  Id* resolve(" << enum_name << " e);\n";
      out << "  Id* " << model.entity_name(enum_decl) << "_ = nullptr;\n";
      for (const auto &enum_value_decl : enum_decl->enumerators()) {
        out << "  Id* " << model.entity_name(enum_value_decl) << "_ = nullptr;\n";
      }
    }

    return true;
  }));

  /////////////////////////////////////////////////////////////////////////////

  auto data_model_cc = PartiallyGeneratedFile::Read(reify_cpp_dir_path / "src" / "arboretum_data_model.cc");
  assert(data_model_cc.Write([&](std::ostream &out) -> bool {
    out << "DataModel::DataModel() {\n";
    for (const auto &[var, name] : meta_data_model) {
      out << "  " << var << "_ = arboretum_create_nameless_node_with_id(" << fqn_to_id(name) << ");\n";
    }
    out << "}\n\n";

    out << "DataModel EmitDataModel() {\n";
    out << "  DataModel data_model;\n\n";

    // Assign a named Id to the appropriate data model field for that decl.
    for (auto &decl_decl : model.index.clang.all_decls) {
      std::string decl_decl_Id = model.entity_name(decl_decl);

      out << "  data_model." << decl_decl_Id << "_ = arboretum_create_nameless_node_with_id("
          << fqn_to_id(decl_decl->getQualifiedNameAsString()) << ");\n";

      // Emit the methods
      for (auto method_decl : decl_decl->methods()) {
        std::optional<std::string> method_decl_usr = getUSR(model.ast_ctx, method_decl);
        if (!method_decl_usr.has_value() || !property_table.contains(*method_decl_usr)) {
          continue;
        }

        std::string method_decl_fqn = method_decl->getQualifiedNameAsString();
        std::string method_decl_Id = model.entity_name(method_decl);

        out << "  data_model." << method_decl_Id << "_ = arboretum_create_nameless_node_with_id("
            << fqn_to_id(method_decl_fqn) << ");\n";
      }
    }

    for (auto &[enum_name, enum_decl] : enums_to_emit) {
      std::string enum_decl_fqn = enum_decl->getQualifiedNameAsString();
      std::string enum_decl_Id = model.entity_name(enum_decl);

      out << "  data_model." << enum_decl_Id << "_ = arboretum_create_nameless_node_with_id("
          << fqn_to_id(enum_decl_fqn) << ");\n";

      for (const auto &enum_value_decl : enum_decl->enumerators()) {
        std::string enum_value_decl_fqn = enum_value_decl->getQualifiedNameAsString();
        std::string enum_value_decl_Id = model.entity_name(enum_value_decl);

        out << "  data_model." << enum_value_decl_Id << "_ = arboretum_create_nameless_node_with_id("
            << fqn_to_id(enum_value_decl_fqn) << ");\n";
      }
    }
    out << "  return data_model;\n";
    out << "}\n\n";

    for (auto &[enum_name, enum_decl] : enums_to_emit) {
      out << "Id* DataModel::resolve(" << enum_name << " e) {\n";
      out << "  switch(e) {\n";

      std::set<llvm::APSInt> seen_values;
      for (const auto &enum_value_decl : enum_decl->enumerators()) {
        std::string enum_value_name = enum_value_decl->getQualifiedNameAsString();

        llvm::APSInt enum_value = enum_value_decl->getInitVal();

        auto find_itr = seen_values.find(enum_value);
        if (find_itr != seen_values.end()) continue;
        seen_values.insert(enum_value);

        out << "    case " << enum_value_name << ": return " << model.entity_name(enum_value_decl) << "_;\n";
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

  /////////////////////////////////////////////////////////////////////////////

  auto reify_rs = PartiallyGeneratedFile::Read(reify_rs_dir_path / "src" / "lib.rs");
  assert(reify_rs.Write([&](std::ostream &out) -> bool {
    std::vector<std::pair<std::string, size_t>> constants;

    out << "pub fn build_data_model() -> GraphBuffer {\n";

    out << "  let mut g = GraphBuffer::new();\n\n";

    out << "  let mut next_id_value = " << next_id << ";\n";
    out << "  let mut next_id = || {\n";
    out << "    let result = next_id_value;\n";
    out << "    next_id_value += 1;\n";
    out << "    result\n";
    out << "  };\n\n";

    for (auto &[var_lower, name] : meta_data_model) {
      std::string var = to_upper(var_lower);
      constants.push_back(std::make_pair(var, fqn_to_id(name)));
      out << "  g.add_named_node(" << var << ", \"" << name << "\");\n";
    }
    out << "\n";

    out << "  fn add_named_node(g: &mut GraphBuffer, id: u64, name: &str) -> u64 {\n";
    out << "    g.add_named_node(id, name);\n";
    out << "    id\n";
    out << "  }\n\n";

    out << "  fn add_node_with_props(g: &mut GraphBuffer, id: u64, props: Value) -> u64 {\n";
    out << "    g.add_node_with_props(id, props);\n";
    out << "    id\n";
    out << "  }\n\n";

    out << "  fn set_node(g: &mut GraphBuffer, item_class: u64, set_id: u64, set_size_id: u64, set: Vec<u64>) -> u64 "
           "{\n";
    out << "    g.add_edge((set_id, META_CLASS, BUILTIN_SET_CLASS));\n";
    out << "    g.add_edge((set_id, BUILTIN_SET_ITEM_CLASS, item_class));\n";
    out << "    let set_size = add_node_with_props(g, set_size_id, Value::Unsigned(set.len() as u64));\n";
    out << "    g.add_edge((set_id, BUILTIN_SET_SIZE, set_size));\n";
    out << "    for i in set {\n";
    out << "      g.add_edge((set_id, BUILTIN_SET_ITEM, i));\n";
    out << "    }\n";
    out << "    set_id\n";
    out << "  }\n\n";

    out << "  add_cfg_schema(&mut g, &mut next_id, &mut set_node);\n\n";

    // Assign a named Id to the appropriate data model field for that decl.
    for (auto &decl_decl : model.index.clang.all_decls) {
      std::string decl_decl_Id = to_upper(model.entity_name(decl_decl));
      std::string decl_decl_fqn = decl_decl->getQualifiedNameAsString();

      constants.push_back(std::make_pair(decl_decl_Id, fqn_to_id(decl_decl_fqn)));

      for (auto method_decl : decl_decl->methods()) {
        std::optional<std::string> method_decl_usr = getUSR(model.ast_ctx, method_decl);
        if (!method_decl_usr.has_value() || !property_table.contains(*method_decl_usr)) {
          continue;
        }

        std::string method_decl_Id = to_upper(model.entity_name(method_decl));
        std::string method_decl_fqn = method_decl->getQualifiedNameAsString();
        constants.push_back(std::make_pair(method_decl_Id, fqn_to_id(method_decl_fqn)));
      }
    }

    for (auto &[enum_name, enum_decl] : enums_to_emit) {
      std::string enum_decl_fqn = enum_decl->getQualifiedNameAsString();
      std::string enum_decl_Id = to_upper(model.entity_name(enum_decl));

      constants.push_back(std::make_pair(enum_decl_Id, fqn_to_id(enum_decl_fqn)));
    }

    for (auto &decl_decl : model.index.clang.all_decls) {
      std::string decl_decl_Id = to_upper(model.entity_name(decl_decl));
      std::string decl_decl_fqn = decl_decl->getQualifiedNameAsString();

      out << "  g.add_named_node(" << decl_decl_Id << ", \"" << decl_decl_fqn << "\");\n";
      out << "  g.add_edge((" << decl_decl_Id << ", META_CLASS, META_CLANG_AST_NODE));\n";

      // Emit the subclass edges
      for (auto super : model.index.inheritance.supers[decl_decl]) {
        if (!model.index.clang.all_decls.contains(super)) continue;

        std::string super_Id = to_upper(model.entity_name(super));

        out << "  g.add_edge((" << decl_decl_Id << ", META_SUBCLASS, " << super_Id << "));\n";
      }

      // Emit the methods
      std::vector<const clang::CXXMethodDecl *> methods;
      for (auto method_decl : decl_decl->methods()) {
        std::optional<std::string> method_decl_usr = getUSR(model.ast_ctx, method_decl);
        if (!method_decl_usr.has_value() || !property_table.contains(*method_decl_usr)) {
          continue;
        }
        methods.push_back(method_decl);
      }

      if (methods.size() > 0) {
        out << "  {\n";
        out << "    let methods = Vec::from([\n";
        for (auto method_decl : methods) {
          std::string method_decl_Id = to_upper(model.entity_name(method_decl));
          std::string method_decl_fqn = method_decl->getQualifiedNameAsString();

          out << "      add_named_node(&mut g, " << method_decl_Id << ", \"" << method_decl_fqn << "\"),\n";
        }
        out << "    ]);\n";
        out << "    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);\n";
        out << "    g.add_edge((" << decl_decl_Id << ", META_METHOD, methods));\n";
        out << "  }\n";
      }

      out << "\n";
    }

    for (auto &[enum_name, enum_decl] : enums_to_emit) {
      std::string enum_decl_fqn = enum_decl->getQualifiedNameAsString();
      std::string enum_decl_Id = to_upper(model.entity_name(enum_decl));

      out << "  g.add_named_node(" << enum_decl_Id << ", \"" << enum_decl_fqn << "\");\n";
      out << "  g.add_edge((" << enum_decl_Id << ", META_CLASS, META_CLANG_AST_NODE));\n";

      out << "  {\n";
      out << "    let enumerators = Vec::from([\n";
      for (const auto &enum_value_decl : enum_decl->enumerators()) {
        std::string enum_value_decl_fqn = enum_value_decl->getQualifiedNameAsString();

        out << "      add_named_node(&mut g, " << fqn_to_id(enum_value_decl_fqn) << ", \"" << enum_value_decl_fqn
            << "\"),\n";
      }
      out << "    ]);\n";
      out << "    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), "
             "enumerators);\n";
      out << "    g.add_edge((" << enum_decl_Id << ", META_CLANG_AST_ENUM_ENUMERATORS, enumerators));\n";
      out << "  }\n\n";
    }
    out << "  g\n";
    out << "}\n\n";

    for (auto &[name, value] : constants) {
      out << "const " << name << ": u64 = " << value << ";\n";
    }

    return true;
  }));
}

}  // namespace arboretum