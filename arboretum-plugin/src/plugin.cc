#include <clang/AST/ASTConsumer.h>
#include <clang/Frontend/FrontendPluginRegistry.h>

#include "arboretum_ast_visitor.h"
#include "arboretum_context.h"
#include "arboretum_data_model.h"
#include "arboretum_source_model.h"

using arboretum::ArboretumASTVisitor;
using arboretum::ArboretumContext;
using arboretum::DataModel;
using arboretum::SourceModel;

class ArboretumASTConsumer : public clang::ASTConsumer {
 public:
  ArboretumASTConsumer() {}

  void HandleTranslationUnit(clang::ASTContext &ast_ctx) override {
    if (!arboretum_connect("localhost:3232")) {
      llvm::errs() << "Failed to connect to localhost:3232!\n";
    }

    DataModel data_model(arboretum_subgraph_id());
    SourceModel source_model(ast_ctx, data_model);

    ArboretumContext arboretum_ctx(ast_ctx, data_model, source_model);
    ArboretumASTVisitor visitor(arboretum_ctx);
    visitor.TraverseAST(ast_ctx);

    llvm::errs() << "Traversal complete.\n";

    arboretum_finalize();
  }
};

class Arboretum : public clang::PluginASTAction {
 public:
  virtual ~Arboretum() {}

  std::unique_ptr<clang::ASTConsumer> CreateASTConsumer(
      clang::CompilerInstance &CI, llvm::StringRef InFile) override {
    return std::make_unique<ArboretumASTConsumer>();
  }

  bool ParseArgs(const clang::CompilerInstance &CI,
                 const std::vector<std::string> &arg) override {
    return true;
  }

  ActionType getActionType() override { return AddAfterMainAction; }
};

static clang::FrontendPluginRegistry::Add<Arboretum> X(
    "Arboretum",
    "Extracts the content of a clang AST and stores it in Surreal.");
