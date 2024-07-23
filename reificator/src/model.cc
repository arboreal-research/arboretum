#include "model.h"

#include <queue>
#include <sstream>
#include <stack>

namespace arboretum {

Model::Model(clang::ASTContext& ctx_, Index&& index_)
    : ast_ctx(ctx_), index(std::move(index_)) {}

}  // namespace arboretum
