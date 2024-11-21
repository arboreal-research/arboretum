#pragma once

#include <map>
#include <string>

#include "model.h"
#include "name_registry.h"

namespace arboretum {

void EmitReifyRs(Model &model, std::map<std::string, bool> property_table, const std::string &reify_rs_dir,
                 NameRegistry &name_registry, const std::map<std::string, const clang::EnumDecl *> &enums_to_emit);

}