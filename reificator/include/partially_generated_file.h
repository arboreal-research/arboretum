#pragma once

#include <cstddef>
#include <filesystem>
#include <functional>
#include <ostream>
#include <string>

namespace arboretum {

struct TextLocation {
  size_t line_start_offset;
  size_t match_start_offset;
  size_t match_end_offset;
  size_t line_end_offset;
};

struct GeneratedBlock {
  TextLocation begin_generated_location;
  TextLocation end_generated_location;
};

struct PartiallyGeneratedFile {
  std::filesystem::path filepath;
  std::string original_contents;
  GeneratedBlock generated_block;

  static PartiallyGeneratedFile Read(std::filesystem::path p);

  bool Write(std::function<bool(std::ostream&)> output_fn);
};

}  // namespace arboretum
