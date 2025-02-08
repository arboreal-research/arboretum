#include "arboretum_context.h"

#include "clang/Analysis/CFG.h"
#include "clang/Index/USRGeneration.h"

namespace arboretum {

uint64_t ArboretumContext::resolve(const clang::Attr *attr) {
  if (attr == nullptr) return 0;

  auto find_itr = attrs.find(attr);
  if (find_itr != attrs.end()) {
    return find_itr->second;
  }
  uint64_t result = data_model_.next_id();
  attrs.insert(std::make_pair(attr, result));
  return result;
}

uint64_t ArboretumContext::resolve(const clang::Decl *decl) {
  if (decl == nullptr) return 0;

  auto find_itr = decls.find(decl);
  if (find_itr != decls.end()) {
    return find_itr->second;
  }
  uint64_t result = data_model_.next_id();

  // Extract USR (if applicable)
  {
    llvm::SmallVector<char, 512> buf;
    if (!clang::index::generateUSRForDecl(decl, buf)) {
      arboretum_emit_Decl_usr(result,
                              std::string(buf.data(), buf.size()).c_str());
    }
  }

  // Extract Control Flow Graph (if applicable)
  {
    const auto *func_decl = clang::dyn_cast_or_null<clang::FunctionDecl>(decl);
    if (func_decl != nullptr && func_decl->isThisDeclarationADefinition()) {
      auto bo = clang::CFG::BuildOptions();
      bo.AddEHEdges = true;
      bo.AddInitializers = true;
      bo.AddImplicitDtors = true;
      bo.AddLifetime = true;
      bo.AddLoopExit = true;
      bo.AddTemporaryDtors = true;
      bo.AddScopes = true;
      bo.AddStaticInitBranches = true;
      bo.AddCXXNewAllocator = true;
      bo.AddCXXDefaultInitExprInCtors = true;
      bo.AddCXXDefaultInitExprInAggregates = true;
      bo.AddRichCXXConstructors = true;
      bo.MarkElidedCXXConstructors = true;
      bo.AddVirtualBaseBranches = true;
      bo.OmitImplicitValueInitializers = false;

      auto cfg = emit_cfg(
          clang::CFG::buildCFG(func_decl, func_decl->getBody(), &ast_ctx_, bo));
      arboretum_emit_FunctionDecl_cfg(result, cfg);
    }
  }

  decls.insert(std::make_pair(decl, result));
  return result;
}

uint64_t ArboretumContext::resolve(const clang::Type *type) {
  if (type == nullptr) return 0;

  auto find_itr = types.find(type);
  if (find_itr != types.end()) {
    return find_itr->second;
  }
  uint64_t result = data_model_.next_id();
  types.insert(std::make_pair(type, result));
  return result;
}

uint64_t ArboretumContext::resolve(const clang::Stmt *stmt) {
  if (stmt == nullptr) return 0;

  auto find_itr = stmts.find(stmt);
  if (find_itr != stmts.end()) {
    return find_itr->second;
  }
  uint64_t result = data_model_.next_id();
  stmts.insert(std::make_pair(stmt, result));
  return result;
}

uint64_t ArboretumContext::resolve(clang::QualType qt) {
  const clang::Type *type_ptr = qt.getTypePtrOrNull();
  if (type_ptr == nullptr) return 0;

  auto key = std::make_pair(qt.getTypePtr(), qt.getLocalFastQualifiers());
  auto find_itr = qualtypes.find(key);
  if (find_itr != qualtypes.end()) {
    return find_itr->second;
  }

  uint64_t result = data_model_.next_id();

  {
    llvm::SmallVector<char, 512> buf;
    if (!clang::index::generateUSRForType(qt, ast_ctx_, buf)) {
      arboretum_emit_QualType_usr(result,
                                  std::string(buf.data(), buf.size()).c_str());
    }
  }

  uint64_t ty = resolve(qt.getTypePtr());

  arboretum_emit_QualType(result, ty, qt.isConstQualified(),
                          qt.isVolatileQualified(), qt.isRestrictQualified());

  qualtypes.insert(std::make_pair(key, result));
  return result;
}

uint64_t ArboretumContext::emit_cfg_block(
    std::unordered_map<const clang::CFGBlock *, uint64_t> &blocks,
    uint64_t cfg_id, const clang::CFGBlock *block) {
  if (block == nullptr) return 0;

  // Do not emit blocks more than once.
  auto find_itr = blocks.find(block);
  if (find_itr != blocks.end()) {
    return find_itr->second;
  }

  uint64_t block_id = data_model_.next_id();
  blocks.insert(std::make_pair(block, block_id));

  uint64_t terminator_stmt = 0;
  uint64_t terminator_kind = 0;
  uint64_t terminator_cond = 0;

  auto terminator = block->getTerminator();
  if (terminator.isValid()) {
    terminator_stmt = resolve(terminator.getStmt());
    terminator_kind = data_model_.resolve(terminator.getKind());
    terminator_cond = resolve(block->getTerminatorCondition());
  }

  uint64_t label_stmt = resolve(block->getLabel());
  uint64_t loop_target = resolve(block->getLoopTarget());
  bool has_no_return_element = block->hasNoReturnElement();

  arboretum_emit_CFGBlock(block_id, terminator_stmt, terminator_kind,
                          terminator_cond, label_stmt, loop_target,
                          has_no_return_element);
  arboretum_emit_CFG_blocks(cfg_id, block_id);

  for (const auto succ : block->succs()) {
    auto succ_block_id = emit_cfg_block(blocks, cfg_id, succ);
    if (succ_block_id != 0) {
      arboretum_emit_CFG_edges(block_id, succ_block_id);
    }
  }

  for (const auto pred : block->preds()) {
    auto pred_block_id = emit_cfg_block(blocks, cfg_id, pred);
    if (pred_block_id != 0) {
      arboretum_emit_CFG_edges(pred_block_id, block_id);
    }
  }

  for (const auto &element : block->refs()) {
    auto element_id = emit_cfg_element(*element);
    if (element_id != 0) {
      arboretum_emit_CFGBlock_elements(block_id, element_id);
    }
  }

  return block_id;
}

uint64_t ArboretumContext::emit_cfg(std::unique_ptr<clang::CFG> cfg) {
  if (cfg == nullptr) return 0;
  std::unordered_map<const clang::CFGBlock *, uint64_t> blocks;

  uint64_t cfg_id = data_model_.next_id();

  uint64_t entry_block_id = emit_cfg_block(blocks, cfg_id, &cfg->getEntry());
  uint64_t exit_block_id = emit_cfg_block(blocks, cfg_id, &cfg->getExit());

  uint64_t indirect_goto =
      emit_cfg_block(blocks, cfg_id, cfg->getIndirectGotoBlock());

  bool is_linear = cfg->isLinear();

  arboretum_emit_CFG(cfg_id, entry_block_id, exit_block_id, is_linear,
                     indirect_goto);

  for (const auto *block : cfg->nodes()) {
    auto block_id = emit_cfg_block(blocks, cfg_id, block);
    if (block_id != 0) {
      arboretum_emit_CFG_blocks(cfg_id, block_id);
    }
  }

  for (const auto *block : cfg->try_blocks()) {
    auto block_id = emit_cfg_block(blocks, cfg_id, block);
    if (block_id != 0) {
      arboretum_emit_CFG_try_blocks(cfg_id, block_id);
    }
  }

  return cfg_id;
}

uint64_t ArboretumContext::emit_cfg_element(const clang::CFGElement &element) {
  auto element_id = data_model_.next_id();
  arboretum_emit_CFGElement(element_id, data_model_.resolve(element.getKind()));

  switch (element.getKind()) {
    case clang::CFGElement::Kind::Initializer: {
      auto e = element.castAs<clang::CFGInitializer>();
      arboretum_emit_CFGInitializer(element_id, resolve(e.getInitializer()));
    } break;
    case clang::CFGElement::Kind::ScopeBegin: {
      auto e = element.castAs<clang::CFGScopeBegin>();
      arboretum_emit_CFGScopeBegin(element_id, resolve(e.getTriggerStmt()),
                                   resolve(e.getVarDecl()));
    } break;
    case clang::CFGElement::Kind::ScopeEnd: {
      auto e = element.castAs<clang::CFGScopeEnd>();
      arboretum_emit_CFGScopeEnd(element_id, resolve(e.getTriggerStmt()),
                                 resolve(e.getVarDecl()));
    } break;
    case clang::CFGElement::Kind::NewAllocator: {
      auto e = element.castAs<clang::CFGNewAllocator>();
      arboretum_emit_CFGNewAllocator(element_id, resolve(e.getAllocatorExpr()));
    } break;
    case clang::CFGElement::Kind::LifetimeEnds: {
      auto e = element.castAs<clang::CFGLifetimeEnds>();
      arboretum_emit_CFGLifetimeEnds(element_id, resolve(e.getTriggerStmt()),
                                     resolve(e.getVarDecl()));
    } break;
    case clang::CFGElement::Kind::LoopExit: {
      auto e = element.castAs<clang::CFGLoopExit>();
      arboretum_emit_CFGLoopExit(element_id, resolve(e.getLoopStmt()));
    } break;
    case clang::CFGElement::Kind::Statement: {
      auto e = element.castAs<clang::CFGStmt>();
      arboretum_emit_CFGLoopExit(element_id, resolve(e.getStmt()));
    } break;
    case clang::CFGElement::Kind::Constructor: {
      auto e = element.castAs<clang::CFGConstructor>();
      arboretum_emit_CFGConstructor(element_id, resolve(e.getStmt()),
                                    resolve(e.getConstructionContext()));
    } break;
    case clang::CFGElement::Kind::CXXRecordTypedCall: {
      auto e = element.castAs<clang::CFGCXXRecordTypedCall>();
      arboretum_emit_CFGCXXRecordTypedCall(element_id, resolve(e.getStmt()),
                                           resolve(e.getConstructionContext()));
    } break;
    case clang::CFGElement::Kind::AutomaticObjectDtor: {
      auto e = element.castAs<clang::CFGAutomaticObjDtor>();
      arboretum_emit_CFGAutomaticObjDtor(
          element_id, resolve(e.getDestructorDecl(ast_ctx_)),
          resolve(e.getTriggerStmt()), resolve(e.getVarDecl()));
    } break;
    case clang::CFGElement::Kind::DeleteDtor: {
      auto e = element.castAs<clang::CFGDeleteDtor>();
      arboretum_emit_CFGDeleteDtor(
          element_id, resolve(e.getDestructorDecl(ast_ctx_)),
          resolve(e.getCXXRecordDecl()), resolve(e.getDeleteExpr()));
    } break;
    case clang::CFGElement::Kind::BaseDtor: {
      auto e = element.castAs<clang::CFGBaseDtor>();
      arboretum_emit_CFGBaseDtor(element_id,
                                 resolve(e.getDestructorDecl(ast_ctx_)),
                                 resolve(e.getBaseSpecifier()));
    } break;
    case clang::CFGElement::Kind::MemberDtor: {
      auto e = element.castAs<clang::CFGMemberDtor>();
      arboretum_emit_CFGMemberDtor(element_id,
                                   resolve(e.getDestructorDecl(ast_ctx_)),
                                   resolve(e.getFieldDecl()));
    } break;
    case clang::CFGElement::Kind::TemporaryDtor: {
      auto e = element.castAs<clang::CFGTemporaryDtor>();
      arboretum_emit_CFGTemporaryDtor(element_id,
                                      resolve(e.getDestructorDecl(ast_ctx_)),
                                      resolve(e.getBindTemporaryExpr()));
    } break;
    case clang::CFGElement::Kind::CleanupFunction: {
      auto e = element.castAs<clang::CFGCleanupFunction>();
      arboretum_emit_CFGCleanupFunction(element_id, resolve(e.getVarDecl()),
                                        resolve(e.getFunctionDecl()));
    }
  }

  return element_id;
}

uint64_t ArboretumContext::resolve(const clang::CXXCtorInitializer *ctor_init) {
  return 0;
}

uint64_t ArboretumContext::resolve(const clang::ConstructionContext *ctor_ctx) {
  return 0;
}

uint64_t ArboretumContext::resolve(
    const clang::CXXBaseSpecifier *base_specifier) {
  return 0;
}

}  // namespace arboretum