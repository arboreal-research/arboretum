#pragma once

#include "method_handler.h"
#include "table.h"

namespace arboretum {

struct DeclHandler {
  DeclHandler(
      std::shared_ptr<Table> owning_table_,
      std::vector<std::unique_ptr<MethodHandler>>&& data_field_handlers_,
      std::vector<std::unique_ptr<MethodHandler>>&& assoc_field_handlers_)
      : owning_table(owning_table_),
        data_field_handlers(std::move(data_field_handlers_)),
        assoc_field_handlers(std::move(assoc_field_handlers_)) {}

  std::shared_ptr<Table> owning_table;
  std::vector<std::unique_ptr<MethodHandler>> data_field_handlers;
  std::vector<std::unique_ptr<MethodHandler>> assoc_field_handlers;
};

}  // namespace arboretum