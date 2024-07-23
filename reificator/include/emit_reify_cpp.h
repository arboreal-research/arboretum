#pragma once

#include <string>

#include "model.h"

namespace arboretum {

void EmitReifyCpp(Model& model,
                  std::unordered_map<std::string, bool> property_table,
                  const std::string& reify_cpp_dir);

}