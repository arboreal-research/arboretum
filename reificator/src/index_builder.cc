#include "index_builder.h"

#include <iostream>
#include <sstream>
#include <stack>

namespace arboretum {
namespace {
std::unordered_set<const clang::CXXRecordDecl *> TransitiveClosure(
    InheritanceHierarchy &ih, const clang::CXXRecordDecl *D) {
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
    if (name == "ConcreteTypeLoc") continue;
    if (name == "InheritingConcreteTypeLoc") continue;
    if (name == "PointerLikeTypeLoc") continue;
    if (name == "TypeofLikeTypeLoc") continue;

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
    if (name == "OMPDeclarativeDirective") continue;

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
}  // namespace

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
  if (D->getDefinition() != D) return true;

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
  } else if (name == "clang::CFGTerminator::Kind") {
    data.clang.cfg_terminator_kind = D;
  } else if (name == "clang::CFGElement::Kind") {
    data.clang.cfg_element_kind = D;
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
  if (D->getDefinition() != D) return true;

  std::string name = D->getQualifiedNameAsString();

  if (name == "clang::RecursiveASTVisitor") {
    data.clang.recursive_ast_visitor = D;
    for (auto *method : D->methods()) {
      std::string name = method->getNameAsString();
      if (name.find("Visit") != 0) continue;
      if (method->getNumParams() != 1) continue;

      clang::QualType arg_type = method->getParamDecl(0)->getType();
      auto *record_decl = arg_type->getAsCXXRecordDecl();
      if (record_decl == nullptr) continue;
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

std::unordered_map<const clang::CXXRecordDecl *,
                   std::vector<const clang::CXXRecordDecl *>>
InheritanceHierarchy::supers_star() const {
  std::unordered_map<const clang::CXXRecordDecl *,
                     std::vector<const clang::CXXRecordDecl *>>
      result;

  std::stack<const clang::CXXRecordDecl *> stack;
  for (const auto &[cur, _] : this->supers) {
    stack.push(cur);
  }

  while (!stack.empty()) {
    auto top = stack.top();

    // If the result is already calculcated, move on.
    if (result.contains(top)) {
      stack.pop();
      continue;
    }

    // If there are no supers, record that and move on.
    auto top_supers_itr = this->supers.find(top);
    if (top_supers_itr == this->supers.end()) {
      result.emplace(top, std::vector<const clang::CXXRecordDecl *>{});
      stack.pop();
      continue;
    }

    // Make sure that the result for all the direct supers have been calculcated
    // already.
    bool found_missing = false;
    for (auto super : top_supers_itr->second) {
      if (!result.contains(super)) {
        stack.push(super);
        found_missing = true;
      }
    }

    // If we found at least one element which wasn't previously calculcated,
    // then we need to come back to this later.
    if (found_missing) continue;

    // Build the result for this element since all supers are calculcated.
    std::unordered_set<const clang::CXXRecordDecl *> supers;
    for (auto super : top_supers_itr->second) {
      auto super_supers = result.find(super);
      assert(super_supers != result.end());
      supers.insert(super);
      supers.insert(super_supers->second.begin(), super_supers->second.end());
    }

    // Record the results and move on.
    result.emplace(top, std::vector<const clang::CXXRecordDecl *>(
                            supers.begin(), supers.end()));
    stack.pop();
  }

  return result;
}

std::unordered_map<const clang::CXXRecordDecl *,
                   std::vector<const clang::CXXRecordDecl *>>
InheritanceHierarchy::subs_star() const {
  std::unordered_map<const clang::CXXRecordDecl *,
                     std::vector<const clang::CXXRecordDecl *>>
      result;

  std::stack<const clang::CXXRecordDecl *> stack;
  for (const auto &[cur, _] : this->subs) {
    stack.push(cur);
  }

  while (!stack.empty()) {
    auto top = stack.top();

    // If the result is already calculcated, move on.
    if (result.contains(top)) {
      stack.pop();
      continue;
    }

    // If there are no subs, record that and move on.
    auto top_subs_itr = this->subs.find(top);
    if (top_subs_itr == this->subs.end()) {
      result.emplace(top, std::vector<const clang::CXXRecordDecl *>{});
      stack.pop();
      continue;
    }

    // Make sure that the result for all the direct subs have been calculcated
    // already.
    bool found_missing = false;
    for (auto super : top_subs_itr->second) {
      if (!result.contains(super)) {
        stack.push(super);
        found_missing = true;
      }
    }

    // If we found at least one element which wasn't previously calculcated,
    // then we need to come back to this later.
    if (found_missing) continue;

    // Build the result for this element since all subs are calculcated.
    std::unordered_set<const clang::CXXRecordDecl *> subs;
    for (auto sub : top_subs_itr->second) {
      auto sub_subs = result.find(sub);
      assert(sub_subs != result.end());
      subs.insert(sub);
      subs.insert(sub_subs->second.begin(), sub_subs->second.end());
    }

    // Record the results and move on.
    result.emplace(top, std::vector<const clang::CXXRecordDecl *>(subs.begin(),
                                                                  subs.end()));
    stack.pop();
  }

  return result;
}

InheritanceTree InheritanceHierarchy::as_tree() {
  std::unordered_map<const clang::CXXRecordDecl *,
                     std::unique_ptr<InheritanceNode>>
      nodes;

  auto get_node = [&](const clang::CXXRecordDecl *decl) -> InheritanceNode * {
    auto find_itr = nodes.find(decl);
    if (find_itr != nodes.end()) {
      return find_itr->second.get();
    }

    auto node = std::make_unique<InheritanceNode>();
    auto result = node.get();
    node->decl = decl;
    nodes.emplace(decl, std::move(node));
    return result;
  };

  for (auto &[decl, decl_subs] : this->subs) {
    auto decl_node = get_node(decl);
    for (auto sub : decl_subs) {
      auto sub_node = get_node(sub);

      decl_node->subs.push_back(sub_node);
      sub_node->supers.push_back(decl_node);
    }
  }

  // Move the built nodes
  InheritanceTree result;
  for (auto &[_, node] : nodes) {
    result.nodes.push_back(std::move(node));
  }
  return result;
}

std::vector<InheritanceNode *> InheritanceTree::roots() const {
  std::vector<InheritanceNode *> result;
  for (auto &node : nodes) {
    if (node->supers.empty()) {
      result.push_back(node.get());
    }
  }
  return result;
}

std::vector<InheritanceNode *> InheritanceTree::leaves() const {
  std::vector<InheritanceNode *> result;
  for (auto &node : nodes) {
    if (node->subs.empty()) {
      result.push_back(node.get());
    }
  }
  return result;
}

void InheritanceTree::filter(
    std::function<bool(const clang::CXXRecordDecl *)> f) {
  std::unordered_set<InheritanceNode *> pruned_nodes;

  // Walk backwards through the vector pruning as we go.
  for (size_t i = nodes.size(); i > 0; --i) {
    InheritanceNode *node = nodes[i - 1].get();
    if (!f(node->decl)) {
      pruned_nodes.insert(node);
      nodes.erase(nodes.begin() + (i - 1));
    }
  }

  // Walk through the vector again removing references to pruned elements.
  for (auto &node : nodes) {
    for (size_t i = node->supers.size(); i > 0; --i) {
      if (pruned_nodes.contains(node->supers[i - 1])) {
        node->supers.erase(node->supers.begin() + (i - 1));
      }
    }

    for (size_t i = node->subs.size(); i > 0; --i) {
      if (pruned_nodes.contains(node->subs[i - 1])) {
        node->subs.erase(node->subs.begin() + (i - 1));
      }
    }
  }
}

}  // namespace arboretum
