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
  explicit Model(clang::ASTContext& ctx_, Index&& index_);

  clang::ASTContext& ast_ctx;
  Index index;
};

}  // namespace arboretum