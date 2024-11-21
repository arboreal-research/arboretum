#include "emit_graphql_model.h"

#include <filesystem>
#include <fstream>
#include <map>
#include <set>
#include <string>

#include "model.h"
#include "partially_generated_file.h"
#include "util.h"

namespace arboretum {

struct EmitGraphQLModelResult {
  bool skip = false;
  std::string module_name;
  std::vector<std::tuple<std::string, std::string>> entries;
};

EmitGraphQLModelResult emit_graphql_model_rec(std::filesystem::path dir, InheritanceNode *node) {
  EmitGraphQLModelResult result;

  std::string decl_name = node->decl->getNameAsString();

  if (decl_name[0] == 'O' && decl_name[1] == 'M' && decl_name[2] == 'P') {
    result.skip = true;
    return result;
  }

  for (size_t i = decl_name.size(); i > 0; --i) {
    if (decl_name[i - 1] == '_') {
      decl_name.erase(decl_name.begin() + (i - 1));
    }
  }

  std::string decl_snake_name = to_snake_case(decl_name) + "_node";

  std::filesystem::path decl_dir_path = dir / decl_snake_name;
  std::filesystem::path decl_rs_path = dir / (decl_snake_name + ".rs");

  result.module_name = decl_snake_name;

  if (node->subs.empty()) {
    result.entries.push_back(std::make_tuple(decl_name, decl_name));
  } else {
    result.entries.push_back(std::make_tuple(decl_name, decl_snake_name + "::" + decl_name));

    std::vector<EmitGraphQLModelResult> sub_results;
    for (auto sub : node->subs) {
      auto sub_result = emit_graphql_model_rec(decl_dir_path, sub);
      for (auto &[name, path] : sub_result.entries) {
        result.entries.push_back(std::make_tuple(name, decl_snake_name + "::" + path));
      }

      sub_results.push_back(std::move(sub_result));
    }

    std::ofstream decl_rs(decl_rs_path);
    decl_rs << "use juniper::*;\n\n";

    std::set<std::string> module_names;
    for (auto &sub_result : sub_results) {
      if (sub_result.skip) continue;
      if (sub_result.entries.size() > 1) {
        module_names.insert(sub_result.module_name);
      }
    }
    if (module_names.size() > 0) {
      std::filesystem::create_directories(decl_dir_path);
    }

    std::map<std::string, std::string> enum_entries;
    for (auto sub_result : sub_results) {
      if (sub_result.skip) continue;
      for (auto &[name, path] : sub_result.entries) {
        enum_entries.emplace(name, path);
      }
    }

    for (auto module_name : module_names) {
      decl_rs << "pub mod " << module_name << ";\n";
    }
    decl_rs << "\n";

    decl_rs << "#[derive(GraphQLObject)]\n";
    decl_rs << "pub struct " << decl_name << " {\n";
    decl_rs << "  id: juniper::ID,\n";
    decl_rs << "}\n\n";

    for (auto sub_result : sub_results) {
      if (sub_result.entries.size() == 1) {
        decl_rs << "#[derive(GraphQLObject)]\n";
        decl_rs << "pub struct " << std::get<0>(sub_result.entries[0]) << " {\n";
        decl_rs << "  id: juniper::ID,\n";
        decl_rs << "}\n\n";
      }
    }

    decl_rs << "#[derive(GraphQLUnion)]\n";
    decl_rs << "pub enum " << decl_name << "Node {\n";
    decl_rs << "  " << decl_name << "(" << decl_name << "),\n";
    for (auto &[name, path] : enum_entries) {
      decl_rs << "  " << name << "(" << path << "),\n";
    }
    decl_rs << "}\n\n";
  }

  return result;
}

void EmitGraphqlModel(Model &model, std::map<std::string, bool> property_table,
                      const std::string &arboretum_graphql_dir) {
  std::filesystem::path arboretum_graphql_dir_path(arboretum_graphql_dir);
  auto graphql_src_dir = arboretum_graphql_dir_path / "src";
  auto graphql_src_model_dir = graphql_src_dir / "model";
  std::filesystem::create_directory(graphql_src_model_dir);

  auto ih_tree = model.index.inheritance.as_tree();
  ih_tree.filter([&](const clang::CXXRecordDecl *decl) -> bool {
    return model.index.clang.all_decls.contains(decl) && !model.index.clang.typeloc_decls.contains(decl);
  });

  auto graphql_model_rs = PartiallyGeneratedFile::Read(graphql_src_dir / "model.rs");
  graphql_model_rs.Write([&](std::ostream &out) -> bool {
    std::vector<EmitGraphQLModelResult> root_results;
    for (auto root : ih_tree.roots()) {
      root_results.push_back(emit_graphql_model_rec(graphql_src_model_dir, root));
    }

    std::set<std::string> module_names;
    for (auto &root_result : root_results) {
      if (root_result.skip) continue;
      module_names.insert(root_result.module_name);
    }

    std::map<std::string, std::string> enum_entries;
    for (auto root_result : root_results) {
      if (root_result.skip) continue;
      for (auto &[name, path] : root_result.entries) {
        enum_entries.emplace(name, path);
      }
    }

    for (auto &module_name : module_names) {
      out << "pub mod " << module_name << ";\n";
    }
    out << "\n";

    out << "#[derive(GraphQLUnion)]\n";
    out << "pub enum ASTNode {\n";
    for (auto &[name, path] : enum_entries) {
      out << "  " << name << "(" << path << "),\n";
    }
    out << "}\n\n";

    return true;
  });
}

}  // namespace arboretum