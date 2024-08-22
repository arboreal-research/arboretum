#include "get_usr.h"

#include <clang/Basic/LLVM.h>

#include <string>

#include "clang/Index/USRGeneration.h"

namespace arboretum {

std::optional<std::string> getUSR(clang::ASTContext &ctx, const clang::Decl *decl) {
  llvm::SmallVector<char, 1024> buf;
  if (clang::index::generateUSRForDecl(decl, buf)) {
    return std::nullopt;
  }
  return std::string(buf.data(), buf.size());
}

std::optional<std::string> getUSR(clang::ASTContext &ctx, clang::QualType qt) {
  llvm::SmallVector<char, 1024> buf;
  if (clang::index::generateUSRForType(qt, ctx, buf)) {
    return std::nullopt;
  }
  return std::string(buf.data(), buf.size());
}

}  // namespace arboretum