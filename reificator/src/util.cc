#include "util.h"

#include <filesystem>
#include <fstream>
#include <optional>
#include <regex>
#include <sstream>
#include <string>

namespace arboretum {

std::string fqn_to_name(const std::string &fqn) {
  return std::string("/") + std::regex_replace(fqn, std::regex("::"), "/");
}

std::optional<std::string> GetFileContents(const char *filename) {
  std::ifstream is(filename);
  if (!is.is_open()) return std::nullopt;
  std::stringstream ss;
  ss << is.rdbuf();
  return ss.str();
}

std::optional<std::string> GetFileContents(const std::string &filename) { return GetFileContents(filename.c_str()); }

std::optional<std::string> GetFileContents(const std::filesystem::path &path) { return GetFileContents(path.c_str()); }

std::string to_upper(const std::string &s) {
  std::string result;
  for (const char &c : s) {
    result.push_back(std::toupper(c));
  }
  return result;
}

std::string to_snake_case(const std::string &input) {
  std::stringstream ss;
  bool wasPrevUpper = false;

  for (size_t i = 0; i < input.size(); ++i) {
    char current = input[i];
    char next = (i + 1 < input.size()) ? input[i + 1] : '\0';

    // Check if current character is uppercase
    if (std::isupper(current)) {
      // Add underscore if needed (between words, not at the start)
      if (i > 0 && (!wasPrevUpper || (wasPrevUpper && std::islower(next)))) {
        ss << '_';
      }
      // Convert to lowercase and add to the stream
      ss << static_cast<char>(std::tolower(current));
      wasPrevUpper = true;
    } else {
      // Append lowercase characters as-is
      ss << current;
      wasPrevUpper = false;
    }
  }

  return ss.str();
}

}  // namespace arboretum