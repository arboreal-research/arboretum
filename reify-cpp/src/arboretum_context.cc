#include "arboretum_context.h"

namespace arboretum {

Entity *ArboretumContext::resolve(const clang::Attr *attr) {
  auto find_itr = attrs.find(attr);
  if (find_itr != attrs.end()) {
    return find_itr->second;
  }
  Entity *result = arboretum_create_nameless_node();
  attrs.insert(std::make_pair(attr, result));
  return result;
}

Entity *ArboretumContext::resolve(const clang::Decl *decl) {
  auto find_itr = decls.find(decl);
  if (find_itr != decls.end()) {
    return find_itr->second;
  }
  Entity *result = arboretum_create_nameless_node();
  decls.insert(std::make_pair(decl, result));
  return result;
}

Entity *ArboretumContext::resolve(const clang::Type *type) {
  auto find_itr = types.find(type);
  if (find_itr != types.end()) {
    return find_itr->second;
  }
  Entity *result = arboretum_create_nameless_node();
  types.insert(std::make_pair(type, result));
  return result;
}

Entity *ArboretumContext::resolve(const clang::Stmt *stmt) {
  auto find_itr = stmts.find(stmt);
  if (find_itr != stmts.end()) {
    return find_itr->second;
  }
  Entity *result = arboretum_create_nameless_node();
  stmts.insert(std::make_pair(stmt, result));
  return result;
}

Entity *ArboretumContext::resolve(clang::QualType qt) {
  const clang::Type *type_ptr = qt.getTypePtrOrNull();
  if (type_ptr == nullptr)
    return nullptr;

  auto key = std::make_pair(qt.getTypePtr(), qt.getLocalFastQualifiers());
  auto find_itr = qualtypes.find(key);
  if (find_itr != qualtypes.end()) {
    return find_itr->second;
  }

  Entity *result = arboretum_create_nameless_node();
  arboretum_create_edge(result, data_model_.meta_class_,
                        data_model_.qualtype_class_);
  arboretum_create_edge(result, data_model_.qualtype_is_const_,
                        data_model_.arboretum_node_for(qt.isConstQualified()));
  arboretum_create_edge(
      result, data_model_.qualtype_is_restrict_,
      data_model_.arboretum_node_for(qt.isRestrictQualified()));
  arboretum_create_edge(
      result, data_model_.qualtype_is_volatile_,
      data_model_.arboretum_node_for(qt.isVolatileQualified()));

  Entity *ty = resolve(qt.getTypePtr());
  arboretum_create_edge(result, data_model_.qualtype_type_, ty);

  qualtypes.insert(std::make_pair(key, result));
  return result;
}

} // namespace arboretum