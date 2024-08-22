#include "partially_generated_file.h"

#include <cassert>
#include <fstream>
#include <optional>

#include "util.h"

namespace arboretum {
namespace {

const char kBEGIN_SATURN_V_GENERATED_CODE[] = "//// BEGIN ARBORETUM GENERATED CODE ////";
const char kEND_SATURN_V_GENERATED_CODE[] = "////   END ARBORETUM GENERATED CODE ////";

const char kDO_NOT_EDIT[] = "//////////// DO NOT EDIT //////////////";

template <size_t N>
std::optional<TextLocation> GetTextLocation(const std::string& text, const char query[N], size_t start_offset) {
  // Attempt to find the query in the text
  auto find_pos = text.find(query, start_offset);
  if (find_pos == std::string::npos) return std::nullopt;

  TextLocation loc;

  // Find the line ending before this match. If we don't find one this is the
  // first line, so use 0.
  loc.line_start_offset = text.rfind('\n', find_pos) + 1;
  if (loc.line_start_offset == std::string::npos) loc.line_start_offset = 0;

  // Record the location of the match as is.
  loc.match_start_offset = find_pos;
  loc.match_end_offset = find_pos + N - 2;

  // Find the line ending after this match. If we don't find one, this is the
  // final line, so use size().
  loc.line_end_offset = text.find('\n', loc.match_end_offset);
  if (loc.line_end_offset == std::string::npos) loc.line_end_offset = text.size();

  return loc;
}

std::vector<GeneratedBlock> find_generated_blocks(const std::string& text) {
  std::vector<GeneratedBlock> result;
  size_t offset = 0;
  while (offset < text.size()) {
    std::optional<TextLocation> start_banner =
        GetTextLocation<sizeof(kBEGIN_SATURN_V_GENERATED_CODE)>(text, kBEGIN_SATURN_V_GENERATED_CODE, offset);
    if (!start_banner.has_value()) break;

    std::optional<TextLocation> end_banner = GetTextLocation<sizeof(kEND_SATURN_V_GENERATED_CODE)>(
        text, kEND_SATURN_V_GENERATED_CODE, start_banner->line_end_offset);
    if (!end_banner.has_value()) break;

    offset = end_banner->line_end_offset;

    result.push_back(GeneratedBlock{.begin_generated_location = std::move(start_banner.value()),
                                    .end_generated_location = std::move(end_banner.value())});
  }
  return result;
}

}  // namespace

PartiallyGeneratedFile PartiallyGeneratedFile::Read(std::filesystem::path p) {
  std::optional<std::string> input_file = GetFileContents(p);
  assert(input_file.has_value());

  std::vector<GeneratedBlock> generated_blocks = find_generated_blocks(input_file.value());
  assert(generated_blocks.size() == 1);

  return {.filepath = p,
          .original_contents = std::move(input_file.value()),
          .generated_block = std::move(generated_blocks[0])};
}

bool PartiallyGeneratedFile::Write(std::function<bool(std::ostream&)> output_fn) {
  std::filesystem::path tmp = filepath;
  tmp += ".tmp";

  {
    std::ofstream os(tmp);

    os << original_contents.substr(0, generated_block.begin_generated_location.line_end_offset + 1);

    if (!output_fn(os)) return false;

    os << original_contents.substr(generated_block.end_generated_location.line_start_offset);
  }

  copy(tmp, filepath, std::filesystem::copy_options::overwrite_existing);
  remove(tmp);
  return true;
}

}  // namespace arboretum