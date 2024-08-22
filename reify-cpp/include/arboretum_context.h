#pragma once

#include <clang/AST/Decl.h>
#include <clang/AST/Stmt.h>
#include <clang/AST/Type.h>
#include <clang/Analysis/ConstructionContext.h>

#include <map>
#include <sstream>
#include <tuple>

#include "arboretum_data_model.h"
#include "arboretum_ffi.h"
#include "arboretum_source_model.h"

namespace clang {
class CFGBlock;
class CFG;
}  // namespace clang

namespace arboretum {

struct ArboretumContext {
  ArboretumContext(clang::ASTContext &ast_ctx, DataModel &data_model, SourceModel &source_model)
      : ast_ctx_(ast_ctx), data_model_(data_model), source_model_(source_model) {}

  clang::ASTContext &ast_ctx_;
  DataModel &data_model_;
  SourceModel &source_model_;

  std::map<const clang::Attr *, Id *> attrs;
  Id *resolve(const clang::Attr *attr);

  std::map<const clang::Decl *, Id *> decls;
  Id *resolve(const clang::Decl *decl);

  std::map<const clang::Type *, Id *> types;
  Id *resolve(const clang::Type *type);

  std::map<const clang::Stmt *, Id *> stmts;
  Id *resolve(const clang::Stmt *stmt);

  std::map<std::pair<const clang::Type *, unsigned>, Id *> qualtypes;
  Id *resolve(clang::QualType qt);

  // std::map<std::tuple<const clang::Type*, clang::SourceRange>, Id*>
  // typelocs;

  Id *resolve(const clang::CXXCtorInitializer *ctor_init);

  Id *resolve(const clang::ConstructionContext *ctor_ctx);

  Id *resolve(const clang::CXXBaseSpecifier *base_specifier);

 private:
  Id *emit_cfg(std::unique_ptr<clang::CFG> cfg);
  Id *emit_cfg_block(std::unordered_map<const clang::CFGBlock *, Id *> &blocks, Id *cfg_id,
                     const clang::CFGBlock *block);
  Id *emit_cfg_element(const clang::CFGElement &element);
};

}  // namespace arboretum