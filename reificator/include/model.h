#pragma once

#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/AST/DeclCXX.h>

#include <memory>
#include <optional>
#include <unordered_set>

#include "decl_handler.h"
#include "index.h"
#include "table.h"

namespace clang {
class CXXRecordDecl;
}

namespace arboretum {

struct Model {
  explicit Model(clang::ASTContext &ctx_, Index &index_);

  void PopulateMetaTables();

  void PopulateClangTables(
      const std::map<std::string, bool> &allowed_properties);

  clang::ASTContext &ast_ctx;

  Index &index;

  std::vector<std::shared_ptr<Table>> tables;
  std::unordered_map<const clang::CXXRecordDecl *, DeclHandler> handler_by_decl;

  std::map<std::string, const clang::EnumDecl *> enums;
  std::map<std::string, const clang::EnumConstantDecl *> enum_constants;

  std::unordered_map<const clang::EnumDecl *, uint64_t> GetEnumToIndex();
  std::unordered_map<const clang::EnumConstantDecl *, uint64_t>
  GetEnumConstantToIndex();

 private:
  bool MarkEnumUsed(const clang::EnumDecl *decl);
  std::unique_ptr<MethodHandler> HandlerForType(
      clang::QualType return_type, const clang::CXXMethodDecl *method_decl);
  std::unique_ptr<MethodHandler> HandlerForMethod(
      const clang::CXXRecordDecl *record_decl,
      const clang::CXXMethodDecl *method_decl);
  void PopulateClangTable(const std::map<std::string, bool> &allowed_properties,
                          const clang::CXXRecordDecl *decl);
};

}  // namespace arboretum