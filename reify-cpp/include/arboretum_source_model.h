#pragma once

#include <clang/AST/ASTContext.h>
#include <clang/Basic/SourceManager.h>

#include <map>
#include <unordered_map>

#include "arboretum_data_model.h"

namespace arboretum {

struct SourceModel {
  explicit SourceModel(clang::ASTContext &ctx, DataModel &data_model)
      : ctx_(ctx), data_model_(data_model) {}

  clang::ASTContext &ctx_;
  DataModel &data_model_;

  // FileId.getHashValue() -> DataValue
  std::unordered_map<unsigned int, uint64_t> file_lookup_;
  uint64_t resolve(clang::FileID fid);

  std::unordered_map<void *, uint64_t> source_location_lookup_;
  uint64_t resolve(clang::SourceLocation source_location);

  std::map<std::pair<uint64_t, uint64_t>, uint64_t> source_range_lookup_;
  uint64_t resolve(clang::SourceRange source_range);
};

}  // namespace arboretum