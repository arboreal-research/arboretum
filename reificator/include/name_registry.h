#pragma once

#include <cstdint>
#include <map>
#include <string>

namespace arboretum {

struct NameRegistry {
  std::map<std::string, uint64_t> id_by_name;
  uint64_t next_id = 0;

  uint64_t fqn_to_id(uint32_t graph_id, const std::string &s);
};

}  // namespace arboretum
