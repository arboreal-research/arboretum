#include <clang/AST/ASTConsumer.h>
#include <clang/Frontend/FrontendPluginRegistry.h>

#include "arboretum_ast_visitor.h"
#include "arboretum_context.h"
#include "arboretum_data_model.h"

using arboretum::ArboretumASTVisitor;
using arboretum::ArboretumContext;
using arboretum::DataModel;
using arboretum::EmitDataModel;

class ArboretumASTConsumer : public clang::ASTConsumer {
 public:
  ArboretumASTConsumer() {}

  void HandleTranslationUnit(clang::ASTContext& ctx) override {
    DataModel data_model = EmitDataModel();

    ArboretumContext arboretum_ctx(data_model);
    ArboretumASTVisitor visitor(arboretum_ctx);
    visitor.TraverseAST(ctx);
  }
};

class Arboretum : public clang::PluginASTAction {
 public:
  virtual ~Arboretum() {}

  std::unique_ptr<clang::ASTConsumer> CreateASTConsumer(
      clang::CompilerInstance& CI, llvm::StringRef InFile) override {
    if (!arboretum_connect_surreal("127.0.0.1:8000")) {
      llvm::errs() << "Failed to connect to database. Is it started?\n";
      std::exit(-1);
    }

    return std::make_unique<ArboretumASTConsumer>();
  }

  bool ParseArgs(const clang::CompilerInstance& CI,
                 const std::vector<std::string>& arg) override {
    return true;
  }

  ActionType getActionType() override { return AddAfterMainAction; }
};

static clang::FrontendPluginRegistry::Add<Arboretum> X(
    "Arboretum",
    "Extracts the content of a clang AST and stores it in Surreal.");
