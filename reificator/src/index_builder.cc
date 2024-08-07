#include "index_builder.h"

#include <iostream>
#include <sstream>
#include <stack>

namespace arboretum {
namespace {
std::unordered_set<const clang::CXXRecordDecl *>
TransitiveClosure(InheritanceHierarchy &ih, const clang::CXXRecordDecl *D) {
  std::unordered_set<const clang::CXXRecordDecl *> result;

  std::stack<const clang::CXXRecordDecl *> decls;
  decls.push(D);
  while (!decls.empty()) {
    // Get the current top of the stack for processing.
    const clang::CXXRecordDecl *cur = decls.top();
    decls.pop();

    // Save it in the output since it was discovered during traversal.
    result.insert(cur);

    // Push any subclasses of this into the stack for processing.
    auto find_itr = ih.subs.find(cur);
    if (find_itr != ih.subs.end()) {
      for (const clang::CXXRecordDecl *child : find_itr->second) {
        decls.push(child);
      }
    }
  }

  return result;
}

void PopulateDerivedIndexFields(IndexBuilder::TempData &tmp, Index &data) {
  for (auto *A : TransitiveClosure(data.inheritance, data.clang.attr_decl)) {
    data.clang.attr_decls.insert(A);
    data.clang.all_decls.insert(A);
  }

  for (auto *T : TransitiveClosure(data.inheritance, data.clang.type_decl)) {
    {
      llvm::StringRef name = T->getName();
      llvm::StringRef base_name = name;
      if (name.size() != 4 && name.substr(name.size() - 4) == "Type") {
        base_name = base_name.substr(0, base_name.size() - 4);
      }

      auto find_itr = tmp.typeclass_enum_by_name.find(
          std::string(base_name.data(), base_name.size()));
      if (find_itr != tmp.typeclass_enum_by_name.end()) {
        data.clang.typeclass_enum_by_decl[T] = find_itr->second;
      }
    }

    data.clang.type_decls.insert(T);
    data.clang.all_decls.insert(T);
  }

  for (auto *TL :
       TransitiveClosure(data.inheritance, data.clang.typeloc_decl)) {
    llvm::StringRef name = TL->getName();
    if (name == "ConcreteTypeLoc")
      continue;
    if (name == "InheritingConcreteTypeLoc")
      continue;
    if (name == "PointerLikeTypeLoc")
      continue;
    if (name == "TypeofLikeTypeLoc")
      continue;

    {
      llvm::StringRef name = TL->getName();
      llvm::StringRef base_name = name;
      if (name.size() != 7 && name.substr(name.size() - 7) == "TypeLoc") {
        base_name = base_name.substr(0, base_name.size() - 7);
      }

      auto find_itr = tmp.typelocclass_enum_by_name.find(
          std::string(base_name.data(), base_name.size()));
      if (find_itr != tmp.typelocclass_enum_by_name.end()) {
        data.clang.typelocclass_enum_by_decl[TL] = find_itr->second;
      }
    }

    data.clang.typeloc_decls.insert(TL);
    data.clang.all_decls.insert(TL);
  }

  for (auto *D : TransitiveClosure(data.inheritance, data.clang.decl_decl)) {
    llvm::StringRef name = D->getName();
    if (name == "OMPDeclarativeDirective")
      continue;

    {
      llvm::StringRef name = D->getName();
      llvm::StringRef base_name = name;
      if (name.size() != 4 && name.substr(name.size() - 4) == "Decl") {
        base_name = base_name.substr(0, base_name.size() - 4);
      }

      auto find_itr = tmp.declkind_enum_by_name.find(
          std::string(base_name.data(), base_name.size()));
      if (find_itr != tmp.declkind_enum_by_name.end()) {
        data.clang.declkind_enum_by_decl[D] = find_itr->second;
      }
    }

    data.clang.decl_decls.insert(D);
    data.clang.all_decls.insert(D);
  }

  for (auto *S : TransitiveClosure(data.inheritance, data.clang.stmt_decl)) {
    {
      llvm::StringRef name = S->getName();

      auto find_itr = tmp.stmtclass_enum_by_name.find(
          std::string(name.data(), name.size()) + "Class");
      if (find_itr != tmp.stmtclass_enum_by_name.end()) {
        data.clang.stmtclass_enum_by_decl[S] = find_itr->second;
      }
    }

    data.clang.stmt_decls.insert(S);
    data.clang.all_decls.insert(S);
  }
}
} // namespace

bool IndexBuilder::shouldVisitTemplateInstantiations() const { return true; }

bool IndexBuilder::VisitTypedefNameDecl(clang::TypedefNameDecl *D) {
  std::string name = D->getQualifiedNameAsString();

  if (name == "uint8_t") {
    data.std.uint8_decl = D;
  } else if (name == "uint16_t") {
    data.std.uint16_decl = D;
  } else if (name == "uint32_t") {
    data.std.uint32_decl = D;
  } else if (name == "uint64_t") {
    data.std.uint64_decl = D;
  } else if (name == "std::string") {
    data.std.string_decl = D;
  }
  return true;
}

bool IndexBuilder::VisitEnumDecl(clang::EnumDecl *D) {
  if (D->getDefinition() != D)
    return true;

  std::string name = D->getQualifiedNameAsString();
  if (name == "clang::attr::Kind") {
    for (auto *tag : D->enumerators()) {
      tmp.attrkind_enum_by_name[tag->getNameAsString()] = tag;
    }
  } else if (name == "clang::Type::TypeClass") {
    for (auto *tag : D->enumerators()) {
      tmp.typeclass_enum_by_name[tag->getNameAsString()] = tag;
    }
  } else if (name == "clang::TypeLoc::TypeLocClass") {
    for (auto *tag : D->enumerators()) {
      tmp.typelocclass_enum_by_name[tag->getNameAsString()] = tag;
    }
  } else if (name == "clang::Decl::Kind") {
    for (auto *tag : D->enumerators()) {
      tmp.declkind_enum_by_name[tag->getNameAsString()] = tag;
    }
  } else if (name == "clang::Stmt::StmtClass") {
    for (auto *tag : D->enumerators()) {
      tmp.stmtclass_enum_by_name[tag->getNameAsString()] = tag;
    }
  }
  return true;
}

bool IndexBuilder::VisitClassTemplateDecl(clang::ClassTemplateDecl *TD) {
  std::string name = TD->getQualifiedNameAsString();
  if (name == "std::vector") {
    for (const auto *CTS : TD->specializations()) {
      data.std.vector_decls.insert(CTS);
    }
  } else if (name == "std::pair") {
    for (const auto *CTS : TD->specializations()) {
      data.std.pair_decls.insert(CTS);
    }
  } else if (name == "llvm::ArrayRef") {
    for (const auto *CTS : TD->specializations()) {
      data.llvm.arrayref_decls.insert(CTS);
    }
  } else if (name == "llvm::PointerUnion") {
    for (const auto *CTS : TD->specializations()) {
      data.llvm.pointer_union_decls.insert(CTS);
    }
  } else if (name == "llvm::iterator_range") {
    for (const auto *CTS : TD->specializations()) {
      data.llvm.iterator_range_decls.insert(CTS);
    }
  } else if (name == "llvm::SmallVector") {
    for (const auto *CTS : TD->specializations()) {
      data.llvm.small_vector_decls.insert(CTS);
    }
  }

  return true;
}

bool IndexBuilder::VisitCXXRecordDecl(clang::CXXRecordDecl *D) {
  if (D->getDefinition() != D)
    return true;

  std::string name = D->getQualifiedNameAsString();

  if (name == "clang::RecursiveASTVisitor") {
    data.clang.recursive_ast_visitor = D;
    for (auto *method : D->methods()) {
      std::string name = method->getNameAsString();
      if (name.find("Visit") != 0)
        continue;
      if (method->getNumParams() != 1)
        continue;

      clang::QualType arg_type = method->getParamDecl(0)->getType();
      auto *record_decl = arg_type->getAsCXXRecordDecl();
      if (record_decl == nullptr)
        continue;
      data.clang.visitable_decls.insert(record_decl);
    }
  } else if (name == "clang::ASTContext") {
    data.clang.ast_context_decl = D;
  } else if (name == "llvm::APFixedPoint") {
    data.llvm.ap_fixedpoint_decl = D;
  } else if (name == "llvm::APFloat") {
    data.llvm.ap_float_decl = D;
  } else if (name == "llvm::APInt") {
    data.llvm.ap_int_decl = D;
  } else if (name == "llvm::APSInt") {
    data.llvm.aps_int_decl = D;
  } else if (name == "llvm::StringRef") {
    data.llvm.string_ref_decl = D;
  } else if (name == "clang::Type") {
    data.clang.type_decl = D;
  } else if (name == "clang::TypeLoc") {
    data.clang.typeloc_decl = D;
  } else if (name == "clang::Decl") {
    data.clang.decl_decl = D;
  } else if (name == "clang::Stmt") {
    data.clang.stmt_decl = D;
  } else if (name == "clang::Attr") {
    data.clang.attr_decl = D;
  } else if (name == "clang::CXXRecordDecl") {
    data.clang.cxx_record_decl = D;
  } else if (name == "clang::CXXMethodDecl") {
    data.clang.cxx_method_decl = D;
  } else if (name == "clang::EnumDecl") {
    data.clang.enum_decl = D;
    for (auto method : D->methods()) {
      if (method->getNameAsString() == "enumerators") {
        data.clang.enum_decl_enumerators = method;
        break;
      }
    }
  } else if (name == "clang::EnumConstantDecl") {
    data.clang.enum_constant_decl = D;
  }

  for (auto &base_specifier : D->bases()) {
    if (clang::CXXRecordDecl *base = base_specifier.getType()
                                         ->getCanonicalTypeInternal()
                                         ->getAsCXXRecordDecl()) {
      data.inheritance.subs[base].push_back(D);
      data.inheritance.supers[D].push_back(base);
    }
  }
  return true;
}

Index IndexBuilder::Build() && {
  assert(data.std.uint64_decl != nullptr);
  assert(data.std.uint32_decl != nullptr);
  assert(data.std.uint16_decl != nullptr);
  assert(data.std.uint8_decl != nullptr);
  assert(data.std.string_decl != nullptr);

  assert(data.llvm.string_ref_decl != nullptr);
  assert(data.llvm.ap_fixedpoint_decl != nullptr);
  assert(data.llvm.ap_float_decl != nullptr);
  assert(data.llvm.ap_int_decl != nullptr);
  assert(data.llvm.aps_int_decl != nullptr);

  assert(data.clang.ast_context_decl != nullptr);
  assert(data.clang.recursive_ast_visitor != nullptr);
  assert(data.clang.attr_decl != nullptr);
  assert(data.clang.type_decl != nullptr);
  assert(data.clang.typeloc_decl != nullptr);
  assert(data.clang.decl_decl != nullptr);
  assert(data.clang.stmt_decl != nullptr);

  PopulateDerivedIndexFields(tmp, data);
  return std::move(data);
}

} // namespace arboretum
