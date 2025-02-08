#pragma once

#include <ostream>

#include "model.h"

namespace arboretum {

void emit_io(Model& model, const std::string& reify_rs_dir);

void emit_ffi(Model& model, const std::string& arboretum_ffi_dir,
              const std::string& reify_rs_dir);

}  // namespace arboretum