#pragma once

#include <map>
#include <string>

#include "model.h"
#include "name_registry.h"

namespace arboretum {

struct EmitReifyCppResult {
  NameRegistry name_registry;
  std::map<std::string, const clang::EnumDecl *> enums_to_emit;
};

EmitReifyCppResult
EmitReifyCpp(Model &model, std::map<std::string, bool> property_table, const std::string &reify_cpp_dir);

}  // namespace arboretum