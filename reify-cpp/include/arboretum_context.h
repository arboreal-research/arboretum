#pragma once

#include <clang/AST/Decl.h>
#include <clang/AST/Stmt.h>
#include <clang/AST/Type.h>

#include <map>
#include <sstream>
#include <tuple>

#include "arboretum_data_model.h"
#include "arboretum_ffi.h"
#include "arboretum_source_model.h"

namespace arboretum {

struct ArboretumContext {
  ArboretumContext(DataModel &data_model, SourceModel &source_model)
      : data_model_(data_model), source_model_(source_model) {}

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
};

} // namespace arboretum