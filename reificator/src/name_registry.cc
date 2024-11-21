#include "name_registry.h"

namespace arboretum {

uint64_t NameRegistry::fqn_to_id(uint32_t graph_id, const std::string &s) {
  auto find_itr = id_by_name.find(s);
  if (find_itr == id_by_name.end()) {
    uint64_t id = next_id++;
    id_by_name[s] = id;
    return id + (static_cast<uint64_t>(graph_id) << 32);
  }
  return find_itr->second + (static_cast<uint64_t>(graph_id) << 32);
};

}