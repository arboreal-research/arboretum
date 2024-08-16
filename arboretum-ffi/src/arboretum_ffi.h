#pragma once

#include <cstdint>

struct Id;
struct Value;

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

void arboretum_stop();

/*
  _   _           _
 | \ | |         | |
 |  \| | ___   __| | ___
 | . ` |/ _ \ / _` |/ _ \
 | |\  | (_) | (_| |  __/
 \_| \_/\___/ \__,_|\___|

*/

void arboretum_destroy_id(Id *ulid);

Value *arboretum_create_value_unsigned(uint64_t v);
Value *arboretum_create_value_signed(int64_t v);
Value *arboretum_create_value_double(double d);
Value *arboretum_create_value_string(const char *s);

Id *arboretum_create_named_node(const char *name);
Id *arboretum_create_named_node_with_props(const char *name, Value *props);

Id *arboretum_create_named_node_with_id(const char *name, uint64_t id);
Id *arboretum_create_named_node_with_id_props(const char *name, uint64_t id,
                                              Value *props);

Id *arboretum_create_nameless_node();
Id *arboretum_create_nameless_node_with_props(Value *props);

Id *arboretum_create_nameless_node_with_id(uint64_t id);
Id *arboretum_create_nameless_node_with_id_props(uint64_t id, Value *props);

// Ulid *arboretum_get_named_node(const char *name);

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

void arboretum_create_edge(const Id *sub, const Id *pred, const Id *obj);
void arboretum_create_edge_with_props(const Id *sub, const Id *pred,
                                      const Id *obj, Value *props);
}