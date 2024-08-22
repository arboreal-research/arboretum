#include <clang/AST/ASTConsumer.h>
#include <clang/Frontend/FrontendPluginRegistry.h>

#include <fstream>
#include <regex>

#include "emit_reify_cpp.h"
#include "get_usr.h"
#include "index_builder.h"
#include "model.h"

namespace arboretum {

std::vector<std::string> split_tab(const std::string &s) {
  std::regex r = std::regex{"\t"};
  std::sregex_token_iterator iter(s.begin(), s.end(), r, -1);
  std::sregex_token_iterator end;
  return {iter, end};
}

struct CommandArgs {
  std::string reify_cpp_dir = "./reify-cpp/";
  std::string reify_rs_dir = "./reify-rs/";

  std::string property_table = "./reificator/properties.csv";
};

class ReificatorASTConsumer : public clang::ASTConsumer {
  CommandArgs args_;

 public:
  ReificatorASTConsumer(CommandArgs args) : args_(std::move(args)) {}

  void HandleTranslationUnit(clang::ASTContext &ctx) override {
    Model model(ctx, LocateClangClasses(ctx));

    auto property_table = ReadPropertyTable();
    UpdatePropertyTable(property_table, model);

    EmitReifyCpp(model, property_table, args_.reify_cpp_dir, args_.reify_rs_dir);
  }

  std::map<std::string, bool> ReadPropertyTable() {
    std::map<std::string, bool> property_table;
    std::ifstream in(args_.property_table);
    std::string line;
    std::getline(in, line);
    while (std::getline(in, line)) {
      std::vector<std::string> parts = split_tab(line);
      if (parts.size() == 3) {
        property_table[parts[1]] = parts[2] == "1" || parts[2] == "true";
      }
    }
    return property_table;
  }

  void UpdatePropertyTable(std::map<std::string, bool> &original_property_table, Model &model) {
    std::ofstream out(args_.property_table);

    out << "Type\tPredicate\tEnabled\n";

    for (auto &decl : model.index.clang.all_decls) {
      std::string decl_name = decl->getNameAsString();

      if (decl_name.find("OMP") == 0 || decl_name.find("ObjC") == 0) continue;
      if (decl_name.rfind("Attr") == decl_name.size() - 4) continue;

      for (const auto *method_decl : decl->methods()) {
        if (!method_decl->isConst()) continue;
        if (method_decl->getAccess() != clang::AS_public) continue;
        if (method_decl->param_size() > 0) continue;
        if (method_decl->isOverloadedOperator()) continue;
        if (method_decl->getReturnType()->isVoidType()) continue;

        std::string method_name = method_decl->getNameAsString();
        std::optional<std::string> method_usr = getUSR(model.ast_ctx, method_decl);
        if (!method_usr.has_value()) continue;
        if (method_name == "operator bool") continue;

        bool enabled = false;
        auto find_itr = original_property_table.find(*method_usr);
        if (find_itr != original_property_table.end()) enabled = find_itr->second;

        out << method_decl->getReturnType().getCanonicalType().getAsString() << "\t" << *method_usr << "\t"
            << (enabled ? "1" : "0") << "\n";
      }
    }
  }

  Index LocateClangClasses(clang::ASTContext &ctx) {
    IndexBuilder builder;
    builder.TraverseAST(ctx);
    return std::move(builder).Build();
  }
};

class Reificator : public clang::PluginASTAction {
  CommandArgs args_;

 public:
  virtual ~Reificator() {}

  std::unique_ptr<clang::ASTConsumer> CreateASTConsumer(clang::CompilerInstance &CI, llvm::StringRef InFile) override {
    return std::make_unique<ReificatorASTConsumer>(args_);
  }

  bool ParseArgs(const clang::CompilerInstance &CI, const std::vector<std::string> &arg) override {
    for (size_t i = 0, e = arg.size(); i < e; ++i) {
      // TODO add args
    }
    return true;
  }

  ActionType getActionType() override { return ReplaceAction; }
};

}  // namespace arboretum

static clang::FrontendPluginRegistry::Add<arboretum::Reificator> X(
    "reificator",
    "Constructs a clang plugin which can serialize an AST to a collector "
    "process for analysis.");
