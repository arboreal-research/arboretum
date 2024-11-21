#include "model.h"

#include <queue>
#include <sstream>
#include <stack>

namespace arboretum {

namespace {

template <typename T>
std::string build_fresh_name(EntityNameMap<T> &entity_map, const char *prefix, std::string &&root) {
  auto find_itr = entity_map.name_count.find(root);
  size_t name_idx;
  if (find_itr != entity_map.name_count.end()) {
    name_idx = find_itr->second++;
  } else {
    name_idx = 0;
    entity_map.name_count.insert(std::pair(root, name_idx + 1));
  }

  std::stringstream ss;
  ss << prefix << '_' << root;
  if (name_idx != 0) {
    ss << '_' << name_idx;
  }
  return ss.str();
}

template <typename T>
std::string entity_name_impl(const char *prefix, EntityNameMap<T> &entity_map, T t) {
  std::string name = t->getNameAsString();
  auto find_itr = entity_map.entity_map.find(t);
  if (find_itr != entity_map.entity_map.end()) {
    return find_itr->second;
  } else {
    name = build_fresh_name(entity_map, prefix, std::move(name));
    entity_map.entity_map.insert(std::pair(t, name));
    return name;
  }
}

std::vector<std::pair<std::string, std::string>> build_meta_data_model() {
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

  return meta_data_model;
}

}  // namespace

Model::Model(clang::ASTContext &ctx_, Index &&index_)
    : ast_ctx(ctx_), index(std::move(index_)), meta_data_model(build_meta_data_model()) {}

std::string Model::entity_name(const clang::CXXMethodDecl *decl) {
  return entity_name_impl("method", method_entity_map, decl);
}

std::string Model::entity_name(const clang::CXXRecordDecl *decl) {
  return entity_name_impl("class", class_entity_map, decl);
}

std::string Model::entity_name(const clang::EnumDecl *decl) { return entity_name_impl("enum", enum_entity_map, decl); }

std::string Model::entity_name(const clang::EnumConstantDecl *decl) {
  return entity_name_impl("enum_constant", enum_constant_entity_map, decl);
}

}  // namespace arboretum
