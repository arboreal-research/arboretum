#pragma once

#include <clang/AST/DeclCXX.h>
#include <clang/AST/RecursiveASTVisitor.h>

#include <memory>

#include "index.h"

namespace arboretum {

class IndexBuilder : public clang::RecursiveASTVisitor<IndexBuilder> {
 public:
  bool shouldVisitTemplateInstantiations() const;

  bool VisitTypedefNameDecl(clang::TypedefNameDecl *D);
  bool VisitEnumDecl(clang::EnumDecl *D);
  bool VisitClassTemplateDecl(clang::ClassTemplateDecl *TD);
  bool VisitCXXRecordDecl(clang::CXXRecordDecl *D);

  Index Build() &&;

  struct TempData {
    std::unordered_map<std::string, clang::EnumConstantDecl *> attrkind_enum_by_name;
    std::unordered_map<std::string, clang::EnumConstantDecl *> typeclass_enum_by_name;
    std::unordered_map<std::string, clang::EnumConstantDecl *> typelocclass_enum_by_name;
    std::unordered_map<std::string, clang::EnumConstantDecl *> declkind_enum_by_name;
    std::unordered_map<std::string, clang::EnumConstantDecl *> stmtclass_enum_by_name;
  };

 private:
  Index data;
  TempData tmp;
};

}  // namespace arboretum
