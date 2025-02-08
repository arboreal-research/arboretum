#pragma once

#include <string>
#include <vector>

#include "field.h"

namespace arboretum {

struct Table {
  Table(std::string name_, std::vector<Field> fields_);

  std::string name;
  std::vector<Field> fields;
};

}  // namespace arboretum