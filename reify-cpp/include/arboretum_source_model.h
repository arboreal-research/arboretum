#pragma once

#include <clang/AST/ASTContext.h>
#include <clang/Basic/SourceManager.h>

#include <map>
#include <unordered_map>

#include "arboretum_data_model.h"

namespace arboretum {

struct SourceModel {
  explicit SourceModel(clang::ASTContext &ctx, DataModel &data_model) : ctx_(ctx), data_model_(data_model) {}

  clang::ASTContext &ctx_;
  DataModel &data_model_;

  // FileId.getHashValue() -> Id
  std::unordered_map<unsigned int, Id *> file_lookup_;
  Id *resolve(clang::FileID fid);

  std::unordered_map<void *, Id *> source_location_lookup_;
  Id *resolve(clang::SourceLocation source_location);

  std::map<std::pair<Id *, Id *>, Id *> source_range_lookup_;
  Id *resolve(clang::SourceRange source_range);
};

SourceModel EmitSourceModel(clang::ASTContext &ctx, DataModel &data_model);

}  // namespace arboretum