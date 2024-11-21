#include "emit_reify_rs.h"

#include <filesystem>
#include <fstream>
#include <map>
#include <string>

#include "get_usr.h"
#include "model.h"
#include "name_registry.h"
#include "partially_generated_file.h"
#include "util.h"

namespace arboretum {

void EmitReifyRs(Model &model, std::map<std::string, bool> property_table, const std::string &reify_rs_dir,
                 NameRegistry &name_registry, const std::map<std::string, const clang::EnumDecl *> &enums_to_emit) {
  std::filesystem::path reify_rs_dir_path(reify_rs_dir);
  auto reify_rs = PartiallyGeneratedFile::Read(reify_rs_dir_path / "src" / "lib.rs");
  assert(reify_rs.Write([&](std::ostream &out) -> bool {
    std::vector<std::pair<std::string, size_t>> constants;

    out << "pub fn build_data_model() -> GraphBuffer {\n";

    out << "  let mut g = GraphBuffer::new();\n\n";

    out << "  let mut next_id_value = " << name_registry.next_id << ";\n";
    out << "  let mut next_id = || {\n";
    out << "    let result = next_id_value;\n";
    out << "    next_id_value += 1;\n";
    out << "    result\n";
    out << "  };\n\n";

    for (auto &[var_lower, name] : model.meta_data_model) {
      std::string var = to_upper(var_lower);
      constants.push_back(std::make_pair(var, name_registry.fqn_to_id(0, name)));
      out << "  g.add_named_node(" << var << ", \"" << name << "\");\n";
    }
    out << "\n";

    out << "  fn add_named_node(g: &mut GraphBuffer, id: u64, name: &str) -> "
           "u64 {\n";
    out << "    g.add_named_node(id, name);\n";
    out << "    id\n";
    out << "  }\n\n";

    out << "  fn add_node_with_props(g: &mut GraphBuffer, id: u64, props: "
           "Value) -> u64 {\n";
    out << "    g.add_node_with_props(id, props);\n";
    out << "    id\n";
    out << "  }\n\n";

    out << "  fn set_node(g: &mut GraphBuffer, item_class: u64, set_id: u64, "
           "set_size_id: u64, set: Vec<u64>) -> u64 "
           "{\n";
    out << "    g.add_edge((set_id, META_CLASS, BUILTIN_SET_CLASS));\n";
    out << "    g.add_edge((set_id, BUILTIN_SET_ITEM_CLASS, item_class));\n";
    out << "    let set_size = add_node_with_props(g, set_size_id, "
           "Value::Unsigned(set.len() as u64));\n";
    out << "    g.add_edge((set_id, BUILTIN_SET_SIZE, set_size));\n";
    out << "    for i in set {\n";
    out << "      g.add_edge((set_id, BUILTIN_SET_ITEM, i));\n";
    out << "    }\n";
    out << "    set_id\n";
    out << "  }\n\n";

    out << "  add_cfg_schema(&mut g, &mut next_id, &mut set_node);\n\n";

    // Assign a named Id to the appropriate data model field for that decl.
    for (auto &decl_decl : model.index.clang.all_decls) {
      std::string decl_decl_Id = to_upper(model.entity_name(decl_decl));
      std::string decl_decl_fqn = decl_decl->getQualifiedNameAsString();

      constants.push_back(std::make_pair(decl_decl_Id, name_registry.fqn_to_id(0, decl_decl_fqn)));

      for (auto method_decl : decl_decl->methods()) {
        std::optional<std::string> method_decl_usr = getUSR(model.ast_ctx, method_decl);
        if (!method_decl_usr.has_value() || !property_table.contains(*method_decl_usr)) {
          continue;
        }

        std::string method_decl_Id = to_upper(model.entity_name(method_decl));
        std::string method_decl_fqn = method_decl->getQualifiedNameAsString();
        constants.push_back(std::make_pair(method_decl_Id, name_registry.fqn_to_id(0, method_decl_fqn)));
      }
    }

    for (auto &[enum_name, enum_decl] : enums_to_emit) {
      std::string enum_decl_fqn = enum_decl->getQualifiedNameAsString();
      std::string enum_decl_Id = to_upper(model.entity_name(enum_decl));

      constants.push_back(std::make_pair(enum_decl_Id, name_registry.fqn_to_id(0, enum_decl_fqn)));
    }

    for (auto &decl_decl : model.index.clang.all_decls) {
      std::string decl_decl_Id = to_upper(model.entity_name(decl_decl));
      std::string decl_decl_fqn = decl_decl->getQualifiedNameAsString();

      out << "  g.add_named_node(" << decl_decl_Id << ", \"" << decl_decl_fqn << "\");\n";
      out << "  g.add_edge((" << decl_decl_Id << ", META_CLASS, META_CLANG_AST_NODE));\n";

      // Emit the subclass edges
      for (auto super : model.index.inheritance.supers[decl_decl]) {
        if (!model.index.clang.all_decls.contains(super)) continue;

        std::string super_Id = to_upper(model.entity_name(super));

        out << "  g.add_edge((" << decl_decl_Id << ", META_SUBCLASS, " << super_Id << "));\n";
      }

      // Emit the methods
      std::vector<const clang::CXXMethodDecl *> methods;
      for (auto method_decl : decl_decl->methods()) {
        std::optional<std::string> method_decl_usr = getUSR(model.ast_ctx, method_decl);
        if (!method_decl_usr.has_value() || !property_table.contains(*method_decl_usr)) {
          continue;
        }
        methods.push_back(method_decl);
      }

      if (methods.size() > 0) {
        out << "  {\n";
        out << "    let methods = Vec::from([\n";
        for (auto method_decl : methods) {
          std::string method_decl_Id = to_upper(model.entity_name(method_decl));
          std::string method_decl_fqn = method_decl->getQualifiedNameAsString();

          out << "      add_named_node(&mut g, " << method_decl_Id << ", \"" << method_decl_fqn << "\"),\n";
        }
        out << "    ]);\n";
        out << "    let methods = set_node(&mut g, META_CLANG_AST_METHOD, "
               "next_id(), next_id(), methods);\n";
        out << "    g.add_edge((" << decl_decl_Id << ", META_METHOD, methods));\n";
        out << "  }\n";
      }

      out << "\n";
    }

    for (auto &[enum_name, enum_decl] : enums_to_emit) {
      std::string enum_decl_fqn = enum_decl->getQualifiedNameAsString();
      std::string enum_decl_Id = to_upper(model.entity_name(enum_decl));

      out << "  g.add_named_node(" << enum_decl_Id << ", \"" << enum_decl_fqn << "\");\n";
      out << "  g.add_edge((" << enum_decl_Id << ", META_CLASS, META_CLANG_AST_NODE));\n";

      out << "  {\n";
      out << "    let enumerators = Vec::from([\n";
      for (const auto &enum_value_decl : enum_decl->enumerators()) {
        std::string enum_value_decl_fqn = enum_value_decl->getQualifiedNameAsString();

        out << "      add_named_node(&mut g, " << name_registry.fqn_to_id(0, enum_value_decl_fqn) << ", \""
            << enum_value_decl_fqn << "\"),\n";
      }
      out << "    ]);\n";
      out << "    let enumerators = set_node(&mut g, "
             "META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), "
             "enumerators);\n";
      out << "    g.add_edge((" << enum_decl_Id << ", META_CLANG_AST_ENUM_ENUMERATORS, enumerators));\n";
      out << "  }\n\n";
    }
    out << "  g\n";
    out << "}\n\n";

    for (auto &[name, value] : constants) {
      out << "pub const " << name << ": u64 = " << value << ";\n";
    }

    return true;
  }));
}

}  // namespace arboretum