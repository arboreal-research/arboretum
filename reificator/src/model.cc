#include "model.h"

#include <queue>
#include <sstream>
#include <stack>

namespace arboretum {

namespace {

template <typename T>
std::string build_fresh_name(EntityNameMap<T> &entity_map, const char *prefix,
                             std::string &&root) {
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
std::string entity_name_impl(const char *prefix, EntityNameMap<T> &entity_map,
                             T t) {
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

} // namespace

Model::Model(clang::ASTContext &ctx_, Index &&index_)
    : ast_ctx(ctx_), index(std::move(index_)) {}

std::string Model::entity_name(const clang::CXXMethodDecl *decl) {
  return entity_name_impl("method", method_entity_map, decl);
}

std::string Model::entity_name(const clang::CXXRecordDecl *decl) {
  return entity_name_impl("class", class_entity_map, decl);
}

std::string Model::entity_name(const clang::EnumDecl *decl) {
  return entity_name_impl("enum", enum_entity_map, decl);
}

std::string Model::entity_name(const clang::EnumConstantDecl *decl) {
  return entity_name_impl("enum_constant", enum_constant_entity_map, decl);
}

} // namespace arboretum
