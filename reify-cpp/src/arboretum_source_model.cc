#include "arboretum_source_model.h"

#include <clang/Basic/SourceManager.h>
#include <filesystem>

namespace arboretum {

const Id *SourceModel::resolve(clang::FileID fid) {
  clang::SourceManager &sm = ctx_.getSourceManager();
  auto file_entry = sm.getFileEntryRefForID(fid);
  if (!file_entry.has_value())
    return data_model_.invalid_file_;

  unsigned int key = fid.getHashValue();
  auto find_itr = file_lookup_.find(key);
  if (find_itr != file_lookup_.end())
    return find_itr->second;

  auto file = arboretum_create_nameless_node();
  file_lookup_.insert(std::make_pair(key, file));

  std::filesystem::path name(file_entry->getName().data());

  // Class
  arboretum_create_edge(file, data_model_.meta_class_, data_model_.file_class_);

  // Filename
  arboretum_create_edge(
      file, data_model_.file_name_,
      data_model_.arboretum_node_for(std::filesystem::canonical(name).c_str()));

  // Content (if available)
  auto content = sm.getBufferDataOrNone(fid);
  if (content.has_value()) {
    arboretum_create_edge(file, data_model_.file_content_,
                          data_model_.arboretum_node_for(content->data()));
  }

  return file;
}

const Id *SourceModel::resolve(clang::SourceLocation source_location) {
  clang::SourceManager &sm = ctx_.getSourceManager();
  void *key = source_location.getPtrEncoding();

  if (!source_location.isValid())
    return data_model_.invalid_source_location_;

  auto find_itr = source_location_lookup_.find(key);
  if (find_itr != source_location_lookup_.end()) {
    return find_itr->second;
  }

  Id *source_loc = arboretum_create_nameless_node();
  source_location_lookup_.insert(std::make_pair(key, source_loc));
  arboretum_create_edge(source_loc, data_model_.meta_class_,
                        data_model_.source_location_class_);
  arboretum_create_edge(
      source_loc, data_model_.source_location_is_file_,
      data_model_.arboretum_node_for(source_location.isFileID()));

  if (source_location.isFileID()) {
    clang::FileID fid = sm.getFileID(source_location);
    arboretum_create_edge(source_loc, data_model_.source_location_file_,
                          resolve(fid));

    clang::PresumedLoc PLoc = sm.getPresumedLoc(source_location);
    if (!PLoc.isInvalid()) {
      arboretum_create_edge(source_loc, data_model_.source_location_line_,
                            data_model_.arboretum_node_for(
                                static_cast<uint64_t>(PLoc.getLine())));
      arboretum_create_edge(source_loc, data_model_.source_location_column_,
                            data_model_.arboretum_node_for(
                                static_cast<uint64_t>(PLoc.getColumn())));
    }
  }

  const Id *expansion_loc = resolve(sm.getExpansionLoc(source_location));
  if (expansion_loc != source_loc) {
    arboretum_create_edge(
        source_loc, data_model_.source_location_expansion_loc_, expansion_loc);
  }

  const Id *spelling_loc = resolve(sm.getSpellingLoc(source_location));
  if (spelling_loc != source_loc) {
    arboretum_create_edge(source_loc, data_model_.source_location_spelling_loc_,
                          spelling_loc);
  }

  return source_loc;
}

const Id *SourceModel::resolve(clang::SourceRange source_range) {

  auto begin_loc = resolve(source_range.getBegin());
  auto end_loc = resolve(source_range.getEnd());
  auto key = std::make_pair(begin_loc, end_loc);

  auto find_itr = source_range_lookup_.find(key);
  if (find_itr != source_range_lookup_.end()) {
    return find_itr->second;
  }

  Id *result = arboretum_create_nameless_node();
  source_range_lookup_.insert(std::make_pair(key, result));

  arboretum_create_edge(result, data_model_.meta_class_,
                        data_model_.source_range_class_);
  arboretum_create_edge(result, data_model_.source_range_begin_, begin_loc);
  arboretum_create_edge(result, data_model_.source_range_end_, end_loc);

  return result;
}

SourceModel EmitSourceModel(clang::ASTContext &ctx, DataModel &data_model) {
  SourceModel model(ctx, data_model);

  return model;
}

} // namespace arboretum
