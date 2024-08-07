#pragma once

#include <filesystem>
#include <functional>
#include <optional>
#include <string>
#include <unordered_set>
#include <vector>

namespace arboretum {

std::string fqn_to_name(const std::string &fqn);

std::optional<std::string> GetFileContents(const char *filename);
std::optional<std::string> GetFileContents(const std::string &filename);
std::optional<std::string> GetFileContents(const std::filesystem::path &path);

template <typename T>
std::vector<T> Sorted(const std::unordered_set<T> &s, bool (*cmp)(T, T)) {
  std::vector<T> v(s.begin(), s.end());
  std::sort(v.begin(), v.end(), cmp);
  return v;
}

} // namespace arboretum
