#pragma once

#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/AST/DeclCXX.h>

#include <memory>
#include <unordered_map>
#include <unordered_set>

#include "index.h"

namespace arboretum {

struct Model {
  explicit Model(clang::ASTContext &ctx_, Index &&index_);

  clang::ASTContext &ast_ctx;

  Index index;

  size_t name_idx = 0;

  std::unordered_map<const clang::CXXMethodDecl *, std::string>
      method_entity_map;
  std::string assign_entity_name(const clang::CXXMethodDecl *method);

  std::unordered_map<const clang::CXXRecordDecl *, std::string>
      class_entity_map;
  std::string assign_entity_name(const clang::CXXRecordDecl *cls);

  std::unordered_map<const clang::EnumDecl *, std::string> enum_entity_map;
  std::string assign_entity_name(const clang::EnumDecl *cls);

  std::unordered_map<const clang::EnumConstantDecl *, std::string>
      enum_constant_entity_map;
  std::string assign_entity_name(const clang::EnumConstantDecl *cls);
};

} // namespace arboretum