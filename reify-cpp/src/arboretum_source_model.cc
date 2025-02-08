#include "arboretum_source_model.h"

#include <clang/Basic/SourceManager.h>

#include <filesystem>

namespace arboretum {

uint64_t SourceModel::resolve(clang::FileID fid) {
  clang::SourceManager &sm = ctx_.getSourceManager();
  auto file_entry = sm.getFileEntryRefForID(fid);
  if (!file_entry.has_value()) return 0;

  unsigned int key = fid.getHashValue();
  auto find_itr = file_lookup_.find(key);
  if (find_itr != file_lookup_.end()) return find_itr->second;

  auto file = data_model_.next_id();
  file_lookup_.insert(std::make_pair(key, file));

  // Content (if available)
  auto content_str = sm.getBufferDataOrNone(fid);
  const char *content = nullptr;
  if (content_str.has_value()) {
    content = content_str->data();
  }

  arboretum_emit_file(file,
                      std::filesystem::canonical(
                          std::filesystem::path(file_entry->getName().data()))
                          .c_str(),
                      content);

  return file;
}

uint64_t SourceModel::resolve(clang::SourceLocation source_location) {
  clang::SourceManager &sm = ctx_.getSourceManager();
  void *key = source_location.getPtrEncoding();

  if (!source_location.isValid()) return 0;

  auto find_itr = source_location_lookup_.find(key);
  if (find_itr != source_location_lookup_.end()) {
    return find_itr->second;
  }

  uint64_t source_loc = data_model_.next_id();
  source_location_lookup_.insert(std::make_pair(key, source_loc));

  uint64_t file = 0;
  uint64_t line = 0;
  uint64_t col = 0;
  if (source_location.isFileID()) {
    clang::FileID fid = sm.getFileID(source_location);

    file = resolve(fid);

    clang::PresumedLoc PLoc = sm.getPresumedLoc(source_location);
    if (!PLoc.isInvalid()) {
      line = static_cast<uint64_t>(PLoc.getLine());
      col = static_cast<uint64_t>(PLoc.getColumn());
    }
  }

  uint64_t expansion_loc = resolve(sm.getExpansionLoc(source_location));
  if (expansion_loc == source_loc) {
    expansion_loc = 0;
  }

  uint64_t spelling_loc = resolve(sm.getSpellingLoc(source_location));
  if (spelling_loc == source_loc) {
    spelling_loc = 0;
  }

  arboretum_emit_source_loc(source_loc, file, line, col, expansion_loc,
                            spelling_loc);

  return source_loc;
}

uint64_t SourceModel::resolve(clang::SourceRange source_range) {
  auto begin_loc = resolve(source_range.getBegin());
  auto end_loc = resolve(source_range.getEnd());
  auto key = std::make_pair(begin_loc, end_loc);

  auto find_itr = source_range_lookup_.find(key);
  if (find_itr != source_range_lookup_.end()) {
    return find_itr->second;
  }

  uint64_t result = data_model_.next_id();
  source_range_lookup_.insert(std::make_pair(key, result));

  arboretum_emit_source_range(result, begin_loc, end_loc);

  return result;
}

}  // namespace arboretum
