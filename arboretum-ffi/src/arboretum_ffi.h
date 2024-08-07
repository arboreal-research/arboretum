#pragma once

#include <cstdint>

struct Entity;

extern "C" {

/*

  _____      _
 /  ___|    | |
 \ `--.  ___| |_ _   _ _ __
  `--. \/ _ \ __| | | | '_ \
 /\__/ /  __/ |_| |_| | |_) |
 \____/ \___|\__|\__,_| .__/
                      | |
                      |_|

*/

uint8_t arboretum_connect(const char *addr);

/*

  _   _           _
 | \ | |         | |
 |  \| | ___   __| | ___
 | . ` |/ _ \ / _` |/ _ \
 | |\  | (_) | (_| |  __/
 \_| \_/\___/ \__,_|\___|

*/

Entity *arboretum_create_named_node(const char *name);

Entity *arboretum_create_named_node_with_id(const char *name, uint64_t id_hi,
                                            uint64_t id_lo);

Entity *arboretum_create_nameless_node();

Entity *arboretum_get_named_node(const char *name);

Entity *arboretum_get_ulid_node(uint64_t id_hi, uint64_t id_lo);

/*

  _____    _
 |  ___|  | |
 | |__  __| | __ _  ___
 |  __|/ _` |/ _` |/ _ \
 | |__| (_| | (_| |  __/
 \____/\__,_|\__, |\___|
              __/ |
             |___/
*/

void arboretum_create_edge(const Entity *sub, const Entity *pred,
                           const Entity *obj);

/**
  _____      _                        _
 |_   _|    | |                      | |
   | | _ __ | |_ ___ _ __ _ __   __ _| |
   | || '_ \| __/ _ \ '__| '_ \ / _` | |
  _| || | | | ||  __/ |  | | | | (_| | |
  \___/_| |_|\__\___|_|  |_| |_|\__,_|_|
*/

void arboretum_dump();
}