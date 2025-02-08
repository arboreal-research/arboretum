#include "table.h"

namespace arboretum {

Table::Table(std::string name_, std::vector<Field> fields_)
    : name(name_), fields(std::move(fields_)) {}

}