#pragma once

#include <map>
#include <string>

#include "model.h"

namespace arboretum {

void EmitReifyCpp(Model &model, std::map<std::string, bool> property_table, const std::string &reify_cpp_dir,
                  const std::string &reify_rs_dir);

}