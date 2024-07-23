#include "util.h"

#include <filesystem>
#include <fstream>
#include <optional>
#include <regex>
#include <sstream>
#include <string>

namespace arboretum {

std::string fqn_to_var(const std::string& fqn) {
  return std::regex_replace(fqn, std::regex("::"), "_");
}

std::optional<std::string> GetFileContents(const char* filename) {
  std::ifstream is(filename);
  if (!is.is_open()) return std::nullopt;
  std::stringstream ss;
  ss << is.rdbuf();
  return ss.str();
}

std::optional<std::string> GetFileContents(const std::string& filename) {
  return GetFileContents(filename.c_str());
}

std::optional<std::string> GetFileContents(const std::filesystem::path& path) {
  return GetFileContents(path.c_str());
}

}  // namespace arboretum