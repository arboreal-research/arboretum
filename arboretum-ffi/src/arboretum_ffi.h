#pragma once

#include <cstdint>

struct ObjectBuilder;
struct Thing;

extern "C" {
uint8_t arboretum_connect_surreal(const char* addr);

Thing* arboretum_node_new(const char* table, ObjectBuilder* fields);
Thing* arboretum_node_new_with_id(const char* table, const char* id, ObjectBuilder* fields);

void arboretum_create_relation(const Thing* left, const char* relation,
                               const Thing* right);

void aboretum_merge_fields(const Thing* obj, ObjectBuilder* fields);

ObjectBuilder* arboretum_object_builder_new();

void arboretum_thing_destroy(Thing* thing);

void arboretum_object_builder_destroy(ObjectBuilder* builder);

void arboretum_object_builder_set_bool(ObjectBuilder* builder,
                                       const char* field, uint8_t value);

void arboretum_object_builder_set_i8(ObjectBuilder* builder, const char* field,
                                     int8_t value);
void arboretum_object_builder_set_i16(ObjectBuilder* builder, const char* field,
                                      int16_t value);
void arboretum_object_builder_set_i32(ObjectBuilder* builder, const char* field,
                                      int32_t value);
void arboretum_object_builder_set_i64(ObjectBuilder* builder, const char* field,
                                      int64_t value);

void arboretum_object_builder_set_u8(ObjectBuilder* builder, const char* field,
                                     uint8_t value);
void arboretum_object_builder_set_u16(ObjectBuilder* builder, const char* field,
                                      uint16_t value);
void arboretum_object_builder_set_u32(ObjectBuilder* builder, const char* field,
                                      uint32_t value);
void arboretum_object_builder_set_u64(ObjectBuilder* builder, const char* field,
                                      uint64_t value);

void arboretum_object_builder_set_f32(ObjectBuilder* builder, const char* field,
                                      float value);
void arboretum_object_builder_set_f64(ObjectBuilder* builder, const char* field,
                                      double value);

void arboretum_object_builder_set_string(ObjectBuilder* builder,
                                         const char* field, const char* value);

void arboretum_object_builder_set_record(ObjectBuilder* builder,
                                         const char* field, const Thing* thing);
}
