#pragma once

#include <clang/AST/DeclCXX.h>
#include <clang/AST/DeclTemplate.h>

#include <unordered_map>
#include <unordered_set>

namespace arboretum {

struct InheritanceHierarchy {
  std::unordered_map<const clang::CXXRecordDecl *, std::vector<const clang::CXXRecordDecl *>> supers;
  std::unordered_map<const clang::CXXRecordDecl *, std::vector<const clang::CXXRecordDecl *>> subs;
};

struct StdLibrary {
  const clang::TypedefNameDecl *uint64_decl = nullptr;
  const clang::TypedefNameDecl *uint32_decl = nullptr;
  const clang::TypedefNameDecl *uint16_decl = nullptr;
  const clang::TypedefNameDecl *uint8_decl = nullptr;

  const clang::TypedefNameDecl *string_decl = nullptr;
  std::unordered_set<const clang::ClassTemplateSpecializationDecl *> vector_decls;
  std::unordered_set<const clang::ClassTemplateSpecializationDecl *> pair_decls;
};

struct LLVM {
  std::unordered_set<const clang::ClassTemplateSpecializationDecl *> arrayref_decls;
  std::unordered_set<const clang::ClassTemplateSpecializationDecl *> pointer_union_decls;
  std::unordered_set<const clang::ClassTemplateSpecializationDecl *> iterator_range_decls;
  std::unordered_set<const clang::ClassTemplateSpecializationDecl *> small_vector_decls;

  const clang::CXXRecordDecl *string_ref_decl = nullptr;
  const clang::CXXRecordDecl *ap_fixedpoint_decl = nullptr;
  const clang::CXXRecordDecl *ap_float_decl = nullptr;
  const clang::CXXRecordDecl *ap_int_decl = nullptr;
  const clang::CXXRecordDecl *aps_int_decl = nullptr;
};

struct Clang {
  const clang::CXXRecordDecl *ast_context_decl = nullptr;
  const clang::CXXRecordDecl *recursive_ast_visitor = nullptr;

  const clang::CXXRecordDecl *cxx_record_decl;
  const clang::CXXRecordDecl *cxx_method_decl;
  const clang::CXXRecordDecl *enum_decl;
  const clang::CXXMethodDecl *enum_decl_enumerators;
  const clang::CXXRecordDecl *enum_constant_decl;

  std::unordered_set<const clang::CXXRecordDecl *> all_decls;
  std::unordered_set<const clang::CXXRecordDecl *> visitable_decls;

  const clang::CXXRecordDecl *attr_decl = nullptr;
  std::unordered_set<const clang::CXXRecordDecl *> attr_decls;
  std::unordered_map<const clang::CXXRecordDecl *, clang::EnumConstantDecl *> attrkind_enum_by_decl;

  const clang::CXXRecordDecl *type_decl = nullptr;
  std::unordered_set<const clang::CXXRecordDecl *> type_decls;
  std::unordered_map<const clang::CXXRecordDecl *, clang::EnumConstantDecl *> typeclass_enum_by_decl;

  const clang::CXXRecordDecl *typeloc_decl = nullptr;
  std::unordered_set<const clang::CXXRecordDecl *> typeloc_decls;
  std::unordered_map<const clang::CXXRecordDecl *, clang::EnumConstantDecl *> typelocclass_enum_by_decl;

  const clang::CXXRecordDecl *decl_decl = nullptr;
  std::unordered_set<const clang::CXXRecordDecl *> decl_decls;

  std::unordered_map<const clang::CXXRecordDecl *, clang::EnumConstantDecl *> declkind_enum_by_decl;

  const clang::CXXRecordDecl *stmt_decl = nullptr;
  std::unordered_set<const clang::CXXRecordDecl *> stmt_decls;
  std::unordered_map<const clang::CXXRecordDecl *, clang::EnumConstantDecl *> stmtclass_enum_by_decl;

  const clang::EnumDecl *cfg_terminator_kind = nullptr;
};

struct RawIndex {};

struct Index {
  InheritanceHierarchy inheritance;
  StdLibrary std;
  LLVM llvm;
  Clang clang;
};

}  // namespace arboretum
