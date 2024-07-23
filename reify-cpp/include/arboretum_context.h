#pragma once

#include <clang/AST/Decl.h>
#include <clang/AST/Stmt.h>
#include <clang/AST/Type.h>

#include <map>
#include <sstream>
#include <tuple>

#include "arboretum_data_model.h"
#include "arboretum_ffi.h"

namespace arboretum {

struct ArboretumContext {
  ArboretumContext(DataModel& data_model) : data_model_(data_model) {}

  DataModel& data_model_;

  std::map<const clang::Decl*, Thing*> decls;
  Thing* resolve(const clang::Decl* decl) {
    auto find_itr = decls.find(decl);
    if (find_itr != decls.end()) {
      return find_itr->second;
    }
    Thing* result = arboretum_node_new("decl", nullptr);
    decls.insert(std::make_pair(decl, result));
    return result;
  }

  std::map<const clang::Type*, Thing*> types;
  Thing* resolve(const clang::Type* type) {
    auto find_itr = types.find(type);
    if (find_itr != types.end()) {
      return find_itr->second;
    }
    Thing* result = arboretum_node_new("type", nullptr);
    types.insert(std::make_pair(type, result));
    return result;
  }

  std::map<const clang::Stmt*, Thing*> stmts;
  Thing* resolve(const clang::Stmt* stmt) {
    auto find_itr = stmts.find(stmt);
    if (find_itr != stmts.end()) {
      return find_itr->second;
    }
    Thing* result = arboretum_node_new("stmt", nullptr);
    stmts.insert(std::make_pair(stmt, result));
    return result;
  }

  // std::map<std::tuple<const clang::Type*, unsigned>, Thing*> qualtypes;
  // std::map<std::tuple<const clang::Type*, clang::SourceRange>, Thing*>
  // typelocs;
};

}  // namespace arboretum