#pragma once

#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/AST/DeclCXX.h>

#include <memory>
#include <unordered_map>
#include <unordered_set>

#include "index.h"

namespace arboretum {

template <typename T>
struct EntityNameMap {
  std::unordered_map<std::string, size_t> name_count;
  std::unordered_map<T, std::string> entity_map;
};

struct Model {
  explicit Model(clang::ASTContext &ctx_, Index &&index_);

  clang::ASTContext &ast_ctx;

  Index index;

  size_t name_idx = 0;

  EntityNameMap<const clang::CXXMethodDecl *> method_entity_map;
  std::string entity_name(const clang::CXXMethodDecl *method);

  EntityNameMap<const clang::CXXRecordDecl *> class_entity_map;
  std::string entity_name(const clang::CXXRecordDecl *cls);

  EntityNameMap<const clang::EnumDecl *> enum_entity_map;
  std::string entity_name(const clang::EnumDecl *cls);

  EntityNameMap<const clang::EnumConstantDecl *> enum_constant_entity_map;
  std::string entity_name(const clang::EnumConstantDecl *cls);
};

}  // namespace arboretum