#pragma once

#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/AST/Type.h>

#include <optional>
#include <string>

namespace arboretum {

std::optional<std::string> getUSR(clang::ASTContext &ctx,
                                  const clang::Decl *decl);

} // namespace arboretum