#include "arboretum_context.h"

#include "clang/Analysis/CFG.h"
#include "clang/Index/USRGeneration.h"

namespace arboretum {

Id *ArboretumContext::resolve(const clang::Attr *attr) {
  if (attr == nullptr) return nullptr;

  auto find_itr = attrs.find(attr);
  if (find_itr != attrs.end()) {
    return find_itr->second;
  }
  Id *result = arboretum_create_nameless_node();
  attrs.insert(std::make_pair(attr, result));
  return result;
}

Id *ArboretumContext::resolve(const clang::Decl *decl) {
  if (decl == nullptr) return nullptr;

  auto find_itr = decls.find(decl);
  if (find_itr != decls.end()) {
    return find_itr->second;
  }
  Id *result = arboretum_create_nameless_node();

  // Extract USR (if applicable)
  {
    llvm::SmallVector<char, 512> buf;
    if (!clang::index::generateUSRForDecl(decl, buf)) {
      arboretum_create_edge(result, data_model_.usr_,
                            data_model_.arboretum_node_for(std::string(buf.data(), buf.size())));
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

      arboretum_create_edge(result, data_model_.cfg_,
                            emit_cfg(clang::CFG::buildCFG(func_decl, func_decl->getBody(), &ast_ctx_, bo)));
    }
  }

  decls.insert(std::make_pair(decl, result));
  return result;
}

Id *ArboretumContext::resolve(const clang::Type *type) {
  if (type == nullptr) return nullptr;

  auto find_itr = types.find(type);
  if (find_itr != types.end()) {
    return find_itr->second;
  }
  Id *result = arboretum_create_nameless_node();
  types.insert(std::make_pair(type, result));
  return result;
}

Id *ArboretumContext::resolve(const clang::Stmt *stmt) {
  if (stmt == nullptr) return nullptr;

  auto find_itr = stmts.find(stmt);
  if (find_itr != stmts.end()) {
    return find_itr->second;
  }
  Id *result = arboretum_create_nameless_node();
  stmts.insert(std::make_pair(stmt, result));
  return result;
}

Id *ArboretumContext::resolve(clang::QualType qt) {
  const clang::Type *type_ptr = qt.getTypePtrOrNull();
  if (type_ptr == nullptr) return nullptr;

  auto key = std::make_pair(qt.getTypePtr(), qt.getLocalFastQualifiers());
  auto find_itr = qualtypes.find(key);
  if (find_itr != qualtypes.end()) {
    return find_itr->second;
  }

  Id *result = arboretum_create_nameless_node();
  arboretum_create_edge(result, data_model_.meta_class_, data_model_.qualtype_class_);
  arboretum_create_edge(result, data_model_.qualtype_is_const_, data_model_.arboretum_node_for(qt.isConstQualified()));
  arboretum_create_edge(result, data_model_.qualtype_is_restrict_,
                        data_model_.arboretum_node_for(qt.isRestrictQualified()));
  arboretum_create_edge(result, data_model_.qualtype_is_volatile_,
                        data_model_.arboretum_node_for(qt.isVolatileQualified()));

  {
    llvm::SmallVector<char, 512> buf;
    if (!clang::index::generateUSRForType(qt, ast_ctx_, buf)) {
      arboretum_create_edge(result, data_model_.usr_,
                            data_model_.arboretum_node_for(std::string(buf.data(), buf.size())));
    }
  }

  Id *ty = resolve(qt.getTypePtr());
  arboretum_create_edge(result, data_model_.qualtype_type_, ty);

  qualtypes.insert(std::make_pair(key, result));
  return result;
}

Id *ArboretumContext::emit_cfg_block(std::unordered_map<const clang::CFGBlock *, Id *> &blocks, Id *cfg_id,
                                     const clang::CFGBlock *block) {
  if (block == nullptr) return nullptr;

  // Do not emit blocks more than once.
  auto find_itr = blocks.find(block);
  if (find_itr != blocks.end()) {
    return find_itr->second;
  }

  Id *block_id = arboretum_create_nameless_node();
  blocks.insert(std::make_pair(block, block_id));
  arboretum_create_edge(block_id, data_model_.cfg_block_parent_, cfg_id);

  // getTerminator
  auto terminator = block->getTerminator();
  if (terminator.isValid()) {
    arboretum_create_edge(block_id, data_model_.cfg_block_terminator_stmt_, resolve(terminator.getStmt()));
    arboretum_create_edge(block_id, data_model_.cfg_block_terminator_kind_, data_model_.resolve(terminator.getKind()));
    arboretum_create_edge(block_id, data_model_.cfg_block_terminator_condition_,
                          resolve(block->getTerminatorCondition()));
  }

  arboretum_create_edge(block_id, data_model_.cfg_block_label_, resolve(block->getLabel()));
  arboretum_create_edge(block_id, data_model_.cfg_block_loop_target_, resolve(block->getLoopTarget()));
  arboretum_create_edge(block_id, data_model_.cfg_block_has_no_return_element_,
                        data_model_.arboretum_node_for(block->hasNoReturnElement()));

  // succs
  {
    std::vector<Id *> succs;
    for (const auto succ : block->succs()) {
      auto *succ_block_id = emit_cfg_block(blocks, cfg_id, succ);
      if (succ_block_id != nullptr) {
        succs.push_back(succ_block_id);
      }
    }
    arboretum_create_edge(block_id, data_model_.cfg_block_succs_,
                          data_model_.arboretum_node_for(data_model_.class_CFGBlock_, succs));
  }

  // preds
  {
    std::unordered_set<Id *> preds;
    for (const auto pred : block->preds()) {
      auto *pred_block_id = emit_cfg_block(blocks, cfg_id, pred);
      if (pred_block_id != nullptr) {
        preds.insert(pred_block_id);
      }
    }
    arboretum_create_edge(block_id, data_model_.cfg_block_preds_,
                          data_model_.arboretum_node_for(data_model_.class_CFGBlock_, preds));
  }

  // elements
  {
    std::vector<Id *> elements;
    for (const auto &element : block->refs()) {
      auto *element_id = emit_cfg_element(*element);
      if (element_id != nullptr) {
        elements.push_back(element_id);
      }
    }
    arboretum_create_edge(block_id, data_model_.cfg_block_preds_,
                          data_model_.arboretum_node_for(data_model_.class_CFGElement_, elements));
  }

  return block_id;
}

Id *ArboretumContext::emit_cfg(std::unique_ptr<clang::CFG> cfg) {
  if (cfg == nullptr) return nullptr;
  std::unordered_map<const clang::CFGBlock *, Id *> blocks;

  Id *cfg_id = arboretum_create_nameless_node();
  arboretum_create_edge(cfg_id, data_model_.meta_class_, data_model_.class_CFG_);

  // nodes
  {
    std::vector<Id *> nodes;
    for (const auto *block : cfg->nodes()) {
      auto *block_id = emit_cfg_block(blocks, cfg_id, block);
      if (block_id != nullptr) {
        nodes.push_back(block_id);
      }
    }
    arboretum_create_edge(cfg_id, data_model_.cfg_nodes_,
                          data_model_.arboretum_node_for(data_model_.class_CFGBlock_, nodes));
  }

  // getEntry
  arboretum_create_edge(cfg_id, data_model_.cfg_entry_, emit_cfg_block(blocks, cfg_id, &cfg->getEntry()));

  // getExit
  arboretum_create_edge(cfg_id, data_model_.cfg_exit_, emit_cfg_block(blocks, cfg_id, &cfg->getExit()));

  // getIndirectGotoBlock
  arboretum_create_edge(cfg_id, data_model_.cfg_indirect_goto_block_,
                        emit_cfg_block(blocks, cfg_id, cfg->getIndirectGotoBlock()));

  // try_blocks
  {
    std::vector<Id *> try_blocks;
    for (const auto *block : cfg->try_blocks()) {
      auto *block_id = emit_cfg_block(blocks, cfg_id, block);
      if (block_id != nullptr) {
        try_blocks.push_back(block_id);
      }
    }
    arboretum_create_edge(cfg_id, data_model_.cfg_try_blocks_,
                          data_model_.arboretum_node_for(data_model_.class_CFGBlock_, try_blocks));
  }

  // isLinear
  arboretum_create_edge(cfg_id, data_model_.cfg_is_linear_, data_model_.arboretum_node_for(cfg->isLinear()));

  return cfg_id;
}

Id *ArboretumContext::emit_cfg_element(const clang::CFGElement &element) {
  Id *element_id = arboretum_create_nameless_node();

  switch (element.getKind()) {
    case clang::CFGElement::Kind::Initializer: {
      auto e = element.castAs<clang::CFGInitializer>();
      arboretum_create_edge(element_id, data_model_.meta_class_, data_model_.class_CFGInitializer_);
      arboretum_create_edge(element_id, data_model_.cfg_element_init_, resolve(e.getInitializer()));
    } break;
    case clang::CFGElement::Kind::ScopeBegin: {
      auto e = element.castAs<clang::CFGScopeBegin>();
      arboretum_create_edge(element_id, data_model_.meta_class_, data_model_.class_CFGScopeBegin_);
      arboretum_create_edge(element_id, data_model_.cfg_element_trigger_stmt_, resolve(e.getTriggerStmt()));
      arboretum_create_edge(element_id, data_model_.cfg_element_var_decl_, resolve(e.getVarDecl()));
    } break;
    case clang::CFGElement::Kind::ScopeEnd: {
      auto e = element.castAs<clang::CFGScopeEnd>();
      arboretum_create_edge(element_id, data_model_.meta_class_, data_model_.class_CFGScopeEnd_);
      arboretum_create_edge(element_id, data_model_.cfg_element_trigger_stmt_, resolve(e.getTriggerStmt()));
      arboretum_create_edge(element_id, data_model_.cfg_element_var_decl_, resolve(e.getVarDecl()));
    } break;
    case clang::CFGElement::Kind::NewAllocator: {
      auto e = element.castAs<clang::CFGNewAllocator>();
      arboretum_create_edge(element_id, data_model_.meta_class_, data_model_.class_CFGNewAllocator_);
      arboretum_create_edge(element_id, data_model_.cfg_element_alloc_expr_, resolve(e.getAllocatorExpr()));
    } break;
    case clang::CFGElement::Kind::LifetimeEnds: {
      auto e = element.castAs<clang::CFGLifetimeEnds>();
      arboretum_create_edge(element_id, data_model_.meta_class_, data_model_.class_CFGLifetimeEnds_);
      arboretum_create_edge(element_id, data_model_.cfg_element_trigger_stmt_, resolve(e.getTriggerStmt()));
      arboretum_create_edge(element_id, data_model_.cfg_element_var_decl_, resolve(e.getVarDecl()));
    } break;
    case clang::CFGElement::Kind::LoopExit: {
      auto e = element.castAs<clang::CFGLoopExit>();
      arboretum_create_edge(element_id, data_model_.meta_class_, data_model_.class_CFGLoopExit_);
      arboretum_create_edge(element_id, data_model_.cfg_element_loop_stmt_, resolve(e.getLoopStmt()));
    } break;
    case clang::CFGElement::Kind::Statement: {
      auto e = element.castAs<clang::CFGStmt>();
      arboretum_create_edge(element_id, data_model_.meta_class_, data_model_.class_CFGStmt_);
      arboretum_create_edge(element_id, data_model_.cfg_element_stmt_, resolve(e.getStmt()));
    } break;
    case clang::CFGElement::Kind::Constructor: {
      auto e = element.castAs<clang::CFGConstructor>();
      arboretum_create_edge(element_id, data_model_.meta_class_, data_model_.class_CFGConstructor_);
      arboretum_create_edge(element_id, data_model_.cfg_element_stmt_, resolve(e.getStmt()));
      arboretum_create_edge(element_id, data_model_.cfg_element_ctor_context_, resolve(e.getConstructionContext()));
    } break;
    case clang::CFGElement::Kind::CXXRecordTypedCall: {
      auto e = element.castAs<clang::CFGCXXRecordTypedCall>();
      arboretum_create_edge(element_id, data_model_.meta_class_, data_model_.class_CFGCXXRecordTypedCall_);
      arboretum_create_edge(element_id, data_model_.cfg_element_stmt_, resolve(e.getStmt()));
      arboretum_create_edge(element_id, data_model_.cfg_element_ctor_context_, resolve(e.getConstructionContext()));
    } break;
    case clang::CFGElement::Kind::AutomaticObjectDtor: {
      auto e = element.castAs<clang::CFGAutomaticObjDtor>();
      arboretum_create_edge(element_id, data_model_.meta_class_, data_model_.class_CFGAutomaticObjDtor_);
      arboretum_create_edge(element_id, data_model_.cfg_element_dtor_decl_, resolve(e.getDestructorDecl(ast_ctx_)));
      arboretum_create_edge(element_id, data_model_.cfg_element_trigger_stmt_, resolve(e.getTriggerStmt()));
      arboretum_create_edge(element_id, data_model_.cfg_element_var_decl_, resolve(e.getVarDecl()));
    } break;
    case clang::CFGElement::Kind::DeleteDtor: {
      auto e = element.castAs<clang::CFGDeleteDtor>();
      arboretum_create_edge(element_id, data_model_.meta_class_, data_model_.class_CFGDeleteDtor_);
      arboretum_create_edge(element_id, data_model_.cfg_element_dtor_decl_, resolve(e.getDestructorDecl(ast_ctx_)));
      arboretum_create_edge(element_id, data_model_.cfg_element_cxx_record_decl_, resolve(e.getCXXRecordDecl()));
      arboretum_create_edge(element_id, data_model_.cfg_element_delete_expr_, resolve(e.getDeleteExpr()));
    } break;
    case clang::CFGElement::Kind::BaseDtor: {
      auto e = element.castAs<clang::CFGBaseDtor>();
      arboretum_create_edge(element_id, data_model_.meta_class_, data_model_.class_CFGBaseDtor_);
      arboretum_create_edge(element_id, data_model_.cfg_element_dtor_decl_, resolve(e.getDestructorDecl(ast_ctx_)));
      arboretum_create_edge(element_id, data_model_.cfg_element_base_specifier_, resolve(e.getBaseSpecifier()));
    } break;
    case clang::CFGElement::Kind::MemberDtor: {
      auto e = element.castAs<clang::CFGMemberDtor>();
      arboretum_create_edge(element_id, data_model_.meta_class_, data_model_.class_CFGMemberDtor_);
      arboretum_create_edge(element_id, data_model_.cfg_element_dtor_decl_, resolve(e.getDestructorDecl(ast_ctx_)));
      arboretum_create_edge(element_id, data_model_.cfg_element_field_decl_, resolve(e.getFieldDecl()));
    } break;
    case clang::CFGElement::Kind::TemporaryDtor: {
      auto e = element.castAs<clang::CFGTemporaryDtor>();
      arboretum_create_edge(element_id, data_model_.meta_class_, data_model_.class_CFGTemporaryDtor_);
      arboretum_create_edge(element_id, data_model_.cfg_element_dtor_decl_, resolve(e.getDestructorDecl(ast_ctx_)));
      arboretum_create_edge(element_id, data_model_.cfg_element_bind_temporary_expr_,
                            resolve(e.getBindTemporaryExpr()));
    } break;
    case clang::CFGElement::Kind::CleanupFunction: {
      auto e = element.castAs<clang::CFGCleanupFunction>();
      arboretum_create_edge(element_id, data_model_.cfg_element_var_decl_, resolve(e.getVarDecl()));
      arboretum_create_edge(element_id, data_model_.cfg_element_function_decl_, resolve(e.getFunctionDecl()));
    }
  }

  return element_id;
}

Id *ArboretumContext::resolve(const clang::CXXCtorInitializer *ctor_init) { return nullptr; }

Id *ArboretumContext::resolve(const clang::ConstructionContext *ctor_ctx) { return nullptr; }

Id *ArboretumContext::resolve(const clang::CXXBaseSpecifier *base_specifier) { return nullptr; }

}  // namespace arboretum