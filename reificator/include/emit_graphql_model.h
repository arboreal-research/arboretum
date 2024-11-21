#pragma once

#include <map>
#include <string>

#include "model.h"

namespace arboretum {

void EmitGraphqlModel(Model &model, std::map<std::string, bool> property_table,
                      const std::string &arboretum_graphql_path);

}