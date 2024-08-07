#include "model.h"

#include <queue>
#include <sstream>
#include <stack>

namespace arboretum {

void assign_entity_names(Index &index,
                         std::unordered_map<const clang::CXXMethodDecl *,
                                            std::string> &method_entity_map,
                         std::unordered_map<const clang::CXXRecordDecl *,
                                            std::string> &class_entity_map) {
  std::unordered_map<std::string, size_t> name_use_count;

  auto make_name = [&](const std::string &name) {
    auto find_itr = name_use_count.find(name);
    if (find_itr != name_use_count.end()) {
      std::stringstream ss;
      ss << name << find_itr->second++;
      return ss.str();
    } else {
      name_use_count.insert(std::pair(name, 1));
      return name;
    }
  };

  for (auto decl : index.clang.all_decls) {
    class_entity_map[decl] = make_name(decl->getNameAsString());
    for (auto method : decl->methods()) {
      method_entity_map[method] = make_name(method->getNameAsString());
    }
  }
}

Model::Model(clang::ASTContext &ctx_, Index &&index_)
    : ast_ctx(ctx_), index(std::move(index_)) {
  assign_entity_names(index, method_entity_map, class_entity_map);
}

std::string Model::assign_entity_name(const clang::CXXMethodDecl *decl) {
  std::string name = decl->getNameAsString();
  auto find_itr = method_entity_map.find(decl);
  if (find_itr != method_entity_map.end()) {
    return find_itr->second;
  } else {
    std::stringstream ss;
    ss << "method_" << name << '_' << name_idx++;
    name = ss.str();
    method_entity_map.insert(std::pair(decl, name));
    return name;
  }
}

std::string Model::assign_entity_name(const clang::CXXRecordDecl *decl) {
  std::string name = decl->getNameAsString();
  auto find_itr = class_entity_map.find(decl);
  if (find_itr != class_entity_map.end()) {
    return find_itr->second;
  } else {
    std::stringstream ss;
    ss << "class_" << name << '_' << name_idx++;
    name = ss.str();
    class_entity_map.insert(std::pair(decl, name));
    return name;
  }
}

std::string Model::assign_entity_name(const clang::EnumDecl *decl) {
  std::string name = decl->getNameAsString();
  auto find_itr = enum_entity_map.find(decl);
  if (find_itr != enum_entity_map.end()) {
    return find_itr->second;
  } else {
    std::stringstream ss;
    ss << "enum_" << name << '_' << name_idx++;
    name = ss.str();
    enum_entity_map.insert(std::pair(decl, name));
    return name;
  }
}

std::string Model::assign_entity_name(const clang::EnumConstantDecl *decl) {
  std::string name = decl->getNameAsString();
  auto find_itr = enum_constant_entity_map.find(decl);
  if (find_itr != enum_constant_entity_map.end()) {
    return find_itr->second;
  } else {
    std::stringstream ss;
    ss << "enum_constant_" << name << '_' << name_idx++;
    name = ss.str();
    enum_constant_entity_map.insert(std::pair(decl, name));
    return name;
  }
}

} // namespace arboretum
