#pragma once

#include <string>

#include "datatype.h"

namespace arboretum {

struct Field {
  explicit Field(std::string name_, DataType datatype_);

  std::string name;
  DataType datatype;
};

}