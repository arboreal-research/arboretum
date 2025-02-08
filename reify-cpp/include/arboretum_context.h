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
  ArboretumContext(clang::ASTContext &ast_ctx, DataModel &data_model,
                   SourceModel &source_model)
      : ast_ctx_(ast_ctx),
        data_model_(data_model),
        source_model_(source_model) {}

  clang::ASTContext &ast_ctx_;
  DataModel &data_model_;
  SourceModel &source_model_;

    std::map<const clang::Attr *, uint64_t> attrs;
  uint64_t resolve(const clang::Attr *attr);

  std::map<const clang::Decl *, uint64_t> decls;
  uint64_t resolve(const clang::Decl *decl);

  std::map<const clang::Type *, uint64_t> types;
  uint64_t resolve(const clang::Type *type);

  std::map<const clang::Stmt *, uint64_t> stmts;
  uint64_t resolve(const clang::Stmt *stmt);

  std::map<std::pair<const clang::Type *, unsigned>, uint64_t> qualtypes;
  uint64_t resolve(clang::QualType qt);

  // std::map<std::tuple<const clang::Type*, clang::SourceRange>, DataValue*>
  // typelocs;

  uint64_t resolve(const clang::CXXCtorInitializer *ctor_init);

  uint64_t resolve(const clang::ConstructionContext *ctor_ctx);

  uint64_t resolve(const clang::CXXBaseSpecifier *base_specifier);

 private:
  uint64_t emit_cfg(std::unique_ptr<clang::CFG> cfg);
  uint64_t emit_cfg_block(
      std::unordered_map<const clang::CFGBlock *, uint64_t> &blocks,
      uint64_t cfg_id, const clang::CFGBlock *block);
  uint64_t emit_cfg_element(const clang::CFGElement &element);
};

}  // namespace arboretum