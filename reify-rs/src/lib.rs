use arboretum_core::{GraphBuffer, Value};

fn add_cfg_schema<F, G>(mut g: &mut GraphBuffer, mut next_id: F, mut set_node: G) 
where 
  F: FnMut() -> u64, 
  G: FnMut(&mut GraphBuffer, u64, u64, u64, Vec<u64>) -> u64 {
    {
      let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), [
        CFG_NODES, 
        CFG_ENTRY, 
        CFG_EXIT, 
        CFG_INDIRECT_GOTO_BLOCK, 
        CFG_TRY_BLOCKS, 
        CFG_IS_LINEAR,
      ].to_vec());
      g.add_edge((CLASS_CFG, META_METHOD, methods));
    }

    {
      let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), [
        CFG_BLOCK_PARENT,
        CFG_BLOCK_SUCCS,
        CFG_BLOCK_PREDS,
        CFG_BLOCK_LABEL,
        CFG_BLOCK_TERMINATOR_STMT,
        CFG_BLOCK_TERMINATOR_KIND,
        CFG_BLOCK_TERMINATOR_CONDITION,
        CFG_BLOCK_LOOP_TARGET,
        CFG_BLOCK_HAS_NO_RETURN_ELEMENT,
      ].to_vec());
      g.add_edge((CLASS_CFGBLOCK, META_METHOD, methods));
    }

    {
      let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), [
        CFG_ELEMENT_INIT
      ].to_vec());
      g.add_edge((CLASS_CFGINITIALIZER, META_METHOD, methods));
    }

    {
      let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), [
        CFG_ELEMENT_TRIGGER_STMT, 
        CFG_ELEMENT_VAR_DECL,
      ].to_vec());
      g.add_edge((CLASS_CFGSCOPEBEGIN, META_METHOD, methods));
    }

    {
      let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), [
        CFG_ELEMENT_TRIGGER_STMT, 
        CFG_ELEMENT_VAR_DECL,
      ].to_vec());
      g.add_edge((CLASS_CFGSCOPEEND, META_METHOD, methods));
    }

    {
      let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), [
        CFG_ELEMENT_ALLOC_EXPR,
      ].to_vec());
      g.add_edge((CLASS_CFGNEWALLOCATOR, META_METHOD, methods));
    }

    {
      let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), [
        CFG_ELEMENT_TRIGGER_STMT, 
        CFG_ELEMENT_VAR_DECL,
      ].to_vec());
      g.add_edge((CLASS_CFGLIFETIMEENDS, META_METHOD, methods));
    }

    {
      let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), [
        CFG_ELEMENT_LOOP_STMT
      ].to_vec());
      g.add_edge((CLASS_CFGLOOPEXIT, META_METHOD, methods));
    }

    {
      let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), [
        CFG_ELEMENT_STMT
      ].to_vec());
      g.add_edge((CLASS_CFGSTMT, META_METHOD, methods));
    }

    {
      let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), [
        CFG_ELEMENT_STMT, 
        CFG_ELEMENT_CTOR_CONTEXT,
      ].to_vec());
      g.add_edge((CLASS_CFGCONSTRUCTOR, META_METHOD, methods));
    }

    {
      let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), [
        CFG_ELEMENT_STMT, 
        CFG_ELEMENT_CTOR_CONTEXT,
      ].to_vec());
      g.add_edge((CLASS_CFGCXXRECORDTYPEDCALL, META_METHOD, methods));
    }

    {
      let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), [
        CFG_ELEMENT_DTOR_DECL,
        CFG_ELEMENT_IS_NO_RETURN,
        CFG_ELEMENT_TRIGGER_STMT, 
        CFG_ELEMENT_VAR_DECL,
      ].to_vec());
      g.add_edge((CLASS_CFGAUTOMATICOBJDTOR, META_METHOD, methods));
    }

    {
      let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), [
        CFG_ELEMENT_DTOR_DECL,
        CFG_ELEMENT_IS_NO_RETURN,
        CFG_ELEMENT_TRIGGER_STMT, 
        CFG_ELEMENT_VAR_DECL,
      ].to_vec());
      g.add_edge((CLASS_CFGDELETEDTOR, META_METHOD, methods));
    }


    {
      let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), [
        CFG_ELEMENT_DTOR_DECL,
        CFG_ELEMENT_IS_NO_RETURN,
        CFG_ELEMENT_BASE_SPECIFIER
      ].to_vec());
      g.add_edge((CLASS_CFGBASEDTOR, META_METHOD, methods));
    }

    {
      let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), [
        CFG_ELEMENT_DTOR_DECL,
        CFG_ELEMENT_IS_NO_RETURN,
        CFG_ELEMENT_FIELD_DECL,
      ].to_vec());
      g.add_edge((CLASS_CFGMEMBERDTOR, META_METHOD, methods));
    }
    
    {
      let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), [
        CFG_ELEMENT_DTOR_DECL,
        CFG_ELEMENT_IS_NO_RETURN,
        CFG_ELEMENT_BIND_TEMPORARY_EXPR,
      ].to_vec());
      g.add_edge((CLASS_CFGTEMPORARYDTOR, META_METHOD, methods));
    }

    {
      let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), [
        CFG_ELEMENT_TRIGGER_STMT, 
        CFG_ELEMENT_VAR_DECL,
      ].to_vec());
      g.add_edge((CLASS_CFGCLEANUPFUNCTION, META_METHOD, methods));
    }
}

//// BEGIN ARBORETUM GENERATED CODE ////
pub fn build_data_model() -> GraphBuffer {
  let mut g = GraphBuffer::new();

  let mut next_id_value = 5610;
  let mut next_id = || {
    let result = next_id_value;
    next_id_value += 1;
    result
  };

  g.add_named_node(TRUE_, "true");
  g.add_named_node(FALSE_, "false");
  g.add_named_node(META_CLASS, "/meta/class");
  g.add_named_node(META_SUBCLASS, "/meta/subclass");
  g.add_named_node(META_METHOD, "/meta/method");
  g.add_named_node(META_DOMAIN, "/meta/domain");
  g.add_named_node(META_RANGE, "/meta/range");
  g.add_named_node(META_CLANG_AST_NODE, "/meta/clang_ast/node");
  g.add_named_node(META_CLANG_AST_METHOD, "/meta/clang_ast/method");
  g.add_named_node(META_CLANG_AST_ENUM, "/meta/clang_ast/enum");
  g.add_named_node(META_CLANG_AST_ENUM_ENUMERATORS, "/meta/clang_ast/enum/enumerators");
  g.add_named_node(META_CLANG_AST_ENUM_CONSTANT, "/meta/clang_ast/enum_constant");
  g.add_named_node(BUILTIN_STRING_CLASS, "/builtin/string");
  g.add_named_node(BUILTIN_U64_CLASS, "/builtin/u64");
  g.add_named_node(BUILTIN_I64_CLASS, "/builtin/i64");
  g.add_named_node(BUILTIN_DOUBLE_CLASS, "/builtin/dbl");
  g.add_named_node(BUILTIN_LIST_CLASS, "/builtin/list");
  g.add_named_node(BUILTIN_LIST_SIZE, "/builtin/list/size");
  g.add_named_node(BUILTIN_LIST_ITEM_CLASS, "/builtin/list/item_class");
  g.add_named_node(BUILTIN_SET_CLASS, "/builtin/set");
  g.add_named_node(BUILTIN_SET_SIZE, "/builtin/set/size");
  g.add_named_node(BUILTIN_SET_ITEM, "/builtin/set/item");
  g.add_named_node(BUILTIN_SET_ITEM_CLASS, "/builtin/set/item_class");
  g.add_named_node(INVALID_FILE, "/invalid/file");
  g.add_named_node(INVALID_SOURCE_LOCATION, "/invalid/clang/SourceLocation");
  g.add_named_node(FILE_CLASS, "/file");
  g.add_named_node(FILE_NAME, "/file/name");
  g.add_named_node(FILE_CONTENT, "/file/content");
  g.add_named_node(SOURCE_LOCATION_CLASS, "/clang/SourceLocation");
  g.add_named_node(SOURCE_LOCATION_IS_FILE, "/clang/SourceLocation/is_file");
  g.add_named_node(SOURCE_LOCATION_FILE, "/clang/SourceLocation/file");
  g.add_named_node(SOURCE_LOCATION_LINE, "/clang/SourceLocation/line");
  g.add_named_node(SOURCE_LOCATION_COLUMN, "/clang/SourceLocation/column");
  g.add_named_node(SOURCE_LOCATION_EXPANSION_LOC, "/clang/SourceLocation/expansion_loc");
  g.add_named_node(SOURCE_LOCATION_SPELLING_LOC, "/clang/SourceLocation/spelling_loc");
  g.add_named_node(SOURCE_RANGE_CLASS, "/clang/SourceRange");
  g.add_named_node(SOURCE_RANGE_BEGIN, "/clang/SourceRange/begin");
  g.add_named_node(SOURCE_RANGE_END, "/clang/SourceLocation/end");
  g.add_named_node(QUALTYPE_CLASS, "/clang/QualType");
  g.add_named_node(QUALTYPE_IS_CONST, "/clang/QualType/is_const");
  g.add_named_node(QUALTYPE_IS_VOLATILE, "/clang/QualType/is_volatile");
  g.add_named_node(QUALTYPE_IS_RESTRICT, "/clang/QualType/is_restrict");
  g.add_named_node(QUALTYPE_TYPE, "/clang/Qualtype/type");
  g.add_named_node(USR, "/clang/usr");
  g.add_named_node(CLASS_CFG, "clang::CFG");
  g.add_named_node(CFG, "/clang/cfg");
  g.add_named_node(CFG_NODES, "/clang/cfg/nodes");
  g.add_named_node(CFG_ENTRY, "/clang/cfg/entry");
  g.add_named_node(CFG_EXIT, "/clang/cfg/exit");
  g.add_named_node(CFG_INDIRECT_GOTO_BLOCK, "/clang/cfg/indirect_goto_block");
  g.add_named_node(CFG_TRY_BLOCKS, "/clang/cfg/try_blocks");
  g.add_named_node(CFG_IS_LINEAR, "/clang/cfg/is_linear");
  g.add_named_node(CLASS_CFGBLOCK, "clang:CFGBlock");
  g.add_named_node(CFG_BLOCK_PARENT, "/clang/cfg_block/parents");
  g.add_named_node(CFG_BLOCK_SUCCS, "/clang/cfg_block/succs");
  g.add_named_node(CFG_BLOCK_PREDS, "/clang/cfg_block/preds");
  g.add_named_node(CFG_BLOCK_LABEL, "/clang/cfg_block/label");
  g.add_named_node(CFG_BLOCK_TERMINATOR_STMT, "/clang/cfg_block/terminator_stmt");
  g.add_named_node(CFG_BLOCK_TERMINATOR_KIND, "/clang/cfg_block/terminator_kind");
  g.add_named_node(CFG_BLOCK_TERMINATOR_CONDITION, "/clang/cfg_block/terminator_condition");
  g.add_named_node(CFG_BLOCK_LOOP_TARGET, "/clang/cfg_block/loop_target");
  g.add_named_node(CFG_BLOCK_HAS_NO_RETURN_ELEMENT, "/clang/cfg_block/has_no_return_element");
  g.add_named_node(CLASS_CFGELEMENT, "clang::CFGElement");
  g.add_named_node(CLASS_CFGCLEANUPFUNCTION, "clang::CFGCleanupFunction");
  g.add_named_node(CLASS_CFGIMPLICITDTOR, "clang::CFGImplicitDtor");
  g.add_named_node(CLASS_CFGINITIALIZER, "clang::CFGInitializer");
  g.add_named_node(CLASS_CFGLIFETIMEENDS, "clang::CFGLifetimeEnds");
  g.add_named_node(CLASS_CFGLOOPEXIT, "clang::CFGLoopExit");
  g.add_named_node(CLASS_CFGNEWALLOCATOR, "clang::CFGNewAllocator");
  g.add_named_node(CLASS_CFGSCOPEBEGIN, "clang::CFGScopeBegin");
  g.add_named_node(CLASS_CFGSCOPEEND, "clang::CFGScopeEnd");
  g.add_named_node(CLASS_CFGSTMT, "clang::CFGStmt");
  g.add_named_node(CLASS_CFGAUTOMATICOBJDTOR, "clang::CFGAutomaticObjDtor");
  g.add_named_node(CLASS_CFGBASEDTOR, "clang::CFGBaseDtor");
  g.add_named_node(CLASS_CFGDELETEDTOR, "clang::CFGDeleteDtor");
  g.add_named_node(CLASS_CFGMEMBERDTOR, "clang::CFGMemberDtor");
  g.add_named_node(CLASS_CFGTEMPORARYDTOR, "clang::CFGTemporaryDtor");
  g.add_named_node(CLASS_CFGCXXRECORDTYPEDCALL, "clang::CFGCXXRecordTypedCall");
  g.add_named_node(CLASS_CFGCONSTRUCTOR, "clang::CFGConstructor");
  g.add_named_node(CFG_ELEMENT_TRIGGER_STMT, "/clang/cfg_element/trigger_stmt");
  g.add_named_node(CFG_ELEMENT_VAR_DECL, "/clang/cfg_element/var_decl");
  g.add_named_node(CFG_ELEMENT_ALLOC_EXPR, "/clang/cfg_element/alloc_expr");
  g.add_named_node(CFG_ELEMENT_LOOP_STMT, "/clang/cfg_element/loop_stmt");
  g.add_named_node(CFG_ELEMENT_STMT, "/clang/cfg_element/stmt");
  g.add_named_node(CFG_ELEMENT_CTOR_CONTEXT, "/clang/cfg_element/ctor_context");
  g.add_named_node(CFG_ELEMENT_DTOR_DECL, "/clang/cfg_element/dtor_decl");
  g.add_named_node(CFG_ELEMENT_IS_NO_RETURN, "/clang/cfg_element/is_no_return");
  g.add_named_node(CFG_ELEMENT_INIT, "/clang/cfg_element/init");
  g.add_named_node(CFG_ELEMENT_CXX_RECORD_DECL, "/clang/cfg_element/cxx_record_decl");
  g.add_named_node(CFG_ELEMENT_DELETE_EXPR, "/clang/cfg_element/delete_expr");
  g.add_named_node(CFG_ELEMENT_BASE_SPECIFIER, "/clang/cfg_element/base_specifier");
  g.add_named_node(CFG_ELEMENT_FIELD_DECL, "/clang/cfg_element/field_decl");
  g.add_named_node(CFG_ELEMENT_BIND_TEMPORARY_EXPR, "/clang/cfg_element/bind_temporary_expr");
  g.add_named_node(CFG_ELEMENT_FUNCTION_DECL, "/clang/cfg_element/function_decl");

  fn add_named_node(g: &mut GraphBuffer, id: u64, name: &str) -> u64 {
    g.add_named_node(id, name);
    id
  }

  fn add_node_with_props(g: &mut GraphBuffer, id: u64, props: Value) -> u64 {
    g.add_node_with_props(id, props);
    id
  }

  fn set_node(g: &mut GraphBuffer, item_class: u64, set_id: u64, set_size_id: u64, set: Vec<u64>) -> u64 {
    g.add_edge((set_id, META_CLASS, BUILTIN_SET_CLASS));
    g.add_edge((set_id, BUILTIN_SET_ITEM_CLASS, item_class));
    let set_size = add_node_with_props(g, set_size_id, Value::Unsigned(set.len() as u64));
    g.add_edge((set_id, BUILTIN_SET_SIZE, set_size));
    for i in set {
      g.add_edge((set_id, BUILTIN_SET_ITEM, i));
    }
    set_id
  }

  add_cfg_schema(&mut g, &mut next_id, &mut set_node);

  g.add_named_node(CLASS_CXXSTDINITIALIZERLISTEXPR, "clang::CXXStdInitializerListExpr");
  g.add_edge((CLASS_CXXSTDINITIALIZERLISTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXSTDINITIALIZERLISTEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSUBEXPR_2, "clang::CXXStdInitializerListExpr::getSubExpr"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_40, "clang::CXXStdInitializerListExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_39, "clang::CXXStdInitializerListExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_44, "clang::CXXStdInitializerListExpr::getSourceRange"),
      add_named_node(&mut g, METHOD_CHILDREN_30, "clang::CXXStdInitializerListExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXSTDINITIALIZERLISTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCPROTOCOLEXPR, "clang::ObjCProtocolExpr");
  g.add_edge((CLASS_OBJCPROTOCOLEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCPROTOCOLEXPR, META_SUBCLASS, CLASS_EXPR));

  g.add_named_node(CLASS_OMPTARGETTEAMSDISTRIBUTESIMDDIRECTIVE, "clang::OMPTargetTeamsDistributeSimdDirective");
  g.add_edge((CLASS_OMPTARGETTEAMSDISTRIBUTESIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETTEAMSDISTRIBUTESIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_EXPR, "clang::Expr");
  g.add_edge((CLASS_EXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_EXPR, META_SUBCLASS, CLASS_VALUESTMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTYPE_1, "clang::Expr::getType"),
      add_named_node(&mut g, METHOD_GETDEPENDENCE_1, "clang::Expr::getDependence"),
      add_named_node(&mut g, METHOD_ISVALUEDEPENDENT, "clang::Expr::isValueDependent"),
      add_named_node(&mut g, METHOD_ISTYPEDEPENDENT, "clang::Expr::isTypeDependent"),
      add_named_node(&mut g, METHOD_ISINSTANTIATIONDEPENDENT, "clang::Expr::isInstantiationDependent"),
      add_named_node(&mut g, METHOD_CONTAINSUNEXPANDEDPARAMETERPACK_1, "clang::Expr::containsUnexpandedParameterPack"),
      add_named_node(&mut g, METHOD_CONTAINSERRORS_1, "clang::Expr::containsErrors"),
      add_named_node(&mut g, METHOD_GETEXPRLOC_14, "clang::Expr::getExprLoc"),
      add_named_node(&mut g, METHOD_ISREADIFDISCARDEDINCPLUSPLUS11, "clang::Expr::isReadIfDiscardedInCPlusPlus11"),
      add_named_node(&mut g, METHOD_ISLVALUE, "clang::Expr::isLValue"),
      add_named_node(&mut g, METHOD_ISPRVALUE, "clang::Expr::isPRValue"),
      add_named_node(&mut g, METHOD_ISXVALUE, "clang::Expr::isXValue"),
      add_named_node(&mut g, METHOD_ISGLVALUE, "clang::Expr::isGLValue"),
      add_named_node(&mut g, METHOD_GETVALUEKIND, "clang::Expr::getValueKind"),
      add_named_node(&mut g, METHOD_GETOBJECTKIND, "clang::Expr::getObjectKind"),
      add_named_node(&mut g, METHOD_ISORDINARYORBITFIELDOBJECT, "clang::Expr::isOrdinaryOrBitFieldObject"),
      add_named_node(&mut g, METHOD_REFERSTOBITFIELD, "clang::Expr::refersToBitField"),
      add_named_node(&mut g, METHOD_GETSOURCEBITFIELD, "clang::Expr::getSourceBitField"),
      add_named_node(&mut g, METHOD_GETREFERENCEDDECLOFCALLEE, "clang::Expr::getReferencedDeclOfCallee"),
      add_named_node(&mut g, METHOD_GETOBJCPROPERTY, "clang::Expr::getObjCProperty"),
      add_named_node(&mut g, METHOD_ISOBJCSELFEXPR, "clang::Expr::isObjCSelfExpr"),
      add_named_node(&mut g, METHOD_REFERSTOVECTORELEMENT, "clang::Expr::refersToVectorElement"),
      add_named_node(&mut g, METHOD_REFERSTOMATRIXELEMENT, "clang::Expr::refersToMatrixElement"),
      add_named_node(&mut g, METHOD_REFERSTOGLOBALREGISTERVAR, "clang::Expr::refersToGlobalRegisterVar"),
      add_named_node(&mut g, METHOD_HASPLACEHOLDERTYPE, "clang::Expr::hasPlaceholderType"),
      add_named_node(&mut g, METHOD_IGNOREUNLESSSPELLEDINSOURCE, "clang::Expr::IgnoreUnlessSpelledInSource"),
      add_named_node(&mut g, METHOD_IGNOREIMPCASTS, "clang::Expr::IgnoreImpCasts"),
      add_named_node(&mut g, METHOD_IGNORECASTS, "clang::Expr::IgnoreCasts"),
      add_named_node(&mut g, METHOD_IGNOREIMPLICIT, "clang::Expr::IgnoreImplicit"),
      add_named_node(&mut g, METHOD_IGNOREIMPLICITASWRITTEN, "clang::Expr::IgnoreImplicitAsWritten"),
      add_named_node(&mut g, METHOD_IGNOREPARENS, "clang::Expr::IgnoreParens"),
      add_named_node(&mut g, METHOD_IGNOREPARENIMPCASTS, "clang::Expr::IgnoreParenImpCasts"),
      add_named_node(&mut g, METHOD_IGNOREPARENCASTS, "clang::Expr::IgnoreParenCasts"),
      add_named_node(&mut g, METHOD_IGNORECONVERSIONOPERATORSINGLESTEP, "clang::Expr::IgnoreConversionOperatorSingleStep"),
      add_named_node(&mut g, METHOD_IGNOREPARENLVALUECASTS, "clang::Expr::IgnoreParenLValueCasts"),
      add_named_node(&mut g, METHOD_IGNOREPARENBASECASTS, "clang::Expr::IgnoreParenBaseCasts"),
      add_named_node(&mut g, METHOD_ISDEFAULTARGUMENT, "clang::Expr::isDefaultArgument"),
      add_named_node(&mut g, METHOD_ISIMPLICITCXXTHIS, "clang::Expr::isImplicitCXXThis"),
      add_named_node(&mut g, METHOD_GETBESTDYNAMICCLASSTYPE, "clang::Expr::getBestDynamicClassType"),
      add_named_node(&mut g, METHOD_GETBESTDYNAMICCLASSTYPEEXPR, "clang::Expr::getBestDynamicClassTypeExpr"),
      add_named_node(&mut g, METHOD_SKIPRVALUESUBOBJECTADJUSTMENTS, "clang::Expr::skipRValueSubobjectAdjustments"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_EXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPPARALLELMASKEDTASKLOOPDIRECTIVE, "clang::OMPParallelMaskedTaskLoopDirective");
  g.add_edge((CLASS_OMPPARALLELMASKEDTASKLOOPDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPPARALLELMASKEDTASKLOOPDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_GOTOSTMT, "clang::GotoStmt");
  g.add_edge((CLASS_GOTOSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_GOTOSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETLABEL_1, "clang::GotoStmt::getLabel"),
      add_named_node(&mut g, METHOD_GETGOTOLOC, "clang::GotoStmt::getGotoLoc"),
      add_named_node(&mut g, METHOD_GETLABELLOC_1, "clang::GotoStmt::getLabelLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_81, "clang::GotoStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_80, "clang::GotoStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_70, "clang::GotoStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_GOTOSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CONTINUESTMT, "clang::ContinueStmt");
  g.add_edge((CLASS_CONTINUESTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONTINUESTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETCONTINUELOC, "clang::ContinueStmt::getContinueLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_58, "clang::ContinueStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_57, "clang::ContinueStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_48, "clang::ContinueStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CONTINUESTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCSELECTOREXPR, "clang::ObjCSelectorExpr");
  g.add_edge((CLASS_OBJCSELECTOREXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCSELECTOREXPR, META_SUBCLASS, CLASS_EXPR));

  g.add_named_node(CLASS_SWITCHSTMT, "clang::SwitchStmt");
  g.add_edge((CLASS_SWITCHSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWITCHSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_HASINITSTORAGE_1, "clang::SwitchStmt::hasInitStorage"),
      add_named_node(&mut g, METHOD_HASVARSTORAGE_1, "clang::SwitchStmt::hasVarStorage"),
      add_named_node(&mut g, METHOD_GETCOND_8, "clang::SwitchStmt::getCond"),
      add_named_node(&mut g, METHOD_GETBODY_11, "clang::SwitchStmt::getBody"),
      add_named_node(&mut g, METHOD_GETINIT_6, "clang::SwitchStmt::getInit"),
      add_named_node(&mut g, METHOD_GETCONDITIONVARIABLE_2, "clang::SwitchStmt::getConditionVariable"),
      add_named_node(&mut g, METHOD_GETCONDITIONVARIABLEDECLSTMT_2, "clang::SwitchStmt::getConditionVariableDeclStmt"),
      add_named_node(&mut g, METHOD_GETSWITCHCASELIST, "clang::SwitchStmt::getSwitchCaseList"),
      add_named_node(&mut g, METHOD_GETSWITCHLOC, "clang::SwitchStmt::getSwitchLoc"),
      add_named_node(&mut g, METHOD_GETLPARENLOC_10, "clang::SwitchStmt::getLParenLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_25, "clang::SwitchStmt::getRParenLoc"),
      add_named_node(&mut g, METHOD_ISALLENUMCASESCOVERED, "clang::SwitchStmt::isAllEnumCasesCovered"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_123, "clang::SwitchStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_122, "clang::SwitchStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_110, "clang::SwitchStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SWITCHSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PSEUDOOBJECTEXPR, "clang::PseudoObjectExpr");
  g.add_edge((CLASS_PSEUDOOBJECTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PSEUDOOBJECTEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSYNTACTICFORM_1, "clang::PseudoObjectExpr::getSyntacticForm"),
      add_named_node(&mut g, METHOD_GETRESULTEXPRINDEX, "clang::PseudoObjectExpr::getResultExprIndex"),
      add_named_node(&mut g, METHOD_GETRESULTEXPR_1, "clang::PseudoObjectExpr::getResultExpr"),
      add_named_node(&mut g, METHOD_GETNUMSEMANTICEXPRS, "clang::PseudoObjectExpr::getNumSemanticExprs"),
      add_named_node(&mut g, METHOD_SEMANTICS_BEGIN, "clang::PseudoObjectExpr::semantics_begin"),
      add_named_node(&mut g, METHOD_SEMANTICS_END, "clang::PseudoObjectExpr::semantics_end"),
      add_named_node(&mut g, METHOD_SEMANTICS, "clang::PseudoObjectExpr::semantics"),
      add_named_node(&mut g, METHOD_GETEXPRLOC_11, "clang::PseudoObjectExpr::getExprLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_106, "clang::PseudoObjectExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_105, "clang::PseudoObjectExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_94, "clang::PseudoObjectExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PSEUDOOBJECTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PARENEXPR, "clang::ParenExpr");
  g.add_edge((CLASS_PARENEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PARENEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSUBEXPR_8, "clang::ParenExpr::getSubExpr"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_103, "clang::ParenExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_102, "clang::ParenExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETLPAREN, "clang::ParenExpr::getLParen"),
      add_named_node(&mut g, METHOD_GETRPAREN, "clang::ParenExpr::getRParen"),
      add_named_node(&mut g, METHOD_CHILDREN_91, "clang::ParenExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PARENEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MSASMSTMT, "clang::MSAsmStmt");
  g.add_edge((CLASS_MSASMSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MSASMSTMT, META_SUBCLASS, CLASS_ASMSTMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETLBRACELOC_2, "clang::MSAsmStmt::getLBraceLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_90, "clang::MSAsmStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_HASBRACES_2, "clang::MSAsmStmt::hasBraces"),
      add_named_node(&mut g, METHOD_GETASMSTRING_2, "clang::MSAsmStmt::getAsmString"),
      add_named_node(&mut g, METHOD_GETALLCONSTRAINTS, "clang::MSAsmStmt::getAllConstraints"),
      add_named_node(&mut g, METHOD_GETCLOBBERS, "clang::MSAsmStmt::getClobbers"),
      add_named_node(&mut g, METHOD_GETALLEXPRS, "clang::MSAsmStmt::getAllExprs"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_91, "clang::MSAsmStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_79, "clang::MSAsmStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MSASMSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SEHEXCEPTSTMT, "clang::SEHExceptStmt");
  g.add_edge((CLASS_SEHEXCEPTSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SEHEXCEPTSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETBEGINLOC_110, "clang::SEHExceptStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETEXCEPTLOC, "clang::SEHExceptStmt::getExceptLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_109, "clang::SEHExceptStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_GETFILTEREXPR, "clang::SEHExceptStmt::getFilterExpr"),
      add_named_node(&mut g, METHOD_GETBLOCK, "clang::SEHExceptStmt::getBlock"),
      add_named_node(&mut g, METHOD_CHILDREN_97, "clang::SEHExceptStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SEHEXCEPTSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPLOOPBASEDDIRECTIVE, "clang::OMPLoopBasedDirective");
  g.add_edge((CLASS_OMPLOOPBASEDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPLOOPBASEDDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_SEHLEAVESTMT, "clang::SEHLeaveStmt");
  g.add_edge((CLASS_SEHLEAVESTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SEHLEAVESTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETLEAVELOC, "clang::SEHLeaveStmt::getLeaveLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_112, "clang::SEHLeaveStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_111, "clang::SEHLeaveStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_99, "clang::SEHLeaveStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SEHLEAVESTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_UNARYEXPRORTYPETRAITEXPR, "clang::UnaryExprOrTypeTraitExpr");
  g.add_edge((CLASS_UNARYEXPRORTYPETRAITEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNARYEXPRORTYPETRAITEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETKIND_5, "clang::UnaryExprOrTypeTraitExpr::getKind"),
      add_named_node(&mut g, METHOD_ISARGUMENTTYPE, "clang::UnaryExprOrTypeTraitExpr::isArgumentType"),
      add_named_node(&mut g, METHOD_GETARGUMENTTYPE, "clang::UnaryExprOrTypeTraitExpr::getArgumentType"),
      add_named_node(&mut g, METHOD_GETARGUMENTTYPEINFO, "clang::UnaryExprOrTypeTraitExpr::getArgumentTypeInfo"),
      add_named_node(&mut g, METHOD_GETARGUMENTEXPR, "clang::UnaryExprOrTypeTraitExpr::getArgumentExpr"),
      add_named_node(&mut g, METHOD_GETTYPEOFARGUMENT, "clang::UnaryExprOrTypeTraitExpr::getTypeOfArgument"),
      add_named_node(&mut g, METHOD_GETOPERATORLOC_9, "clang::UnaryExprOrTypeTraitExpr::getOperatorLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_26, "clang::UnaryExprOrTypeTraitExpr::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_126, "clang::UnaryExprOrTypeTraitExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_125, "clang::UnaryExprOrTypeTraitExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_113, "clang::UnaryExprOrTypeTraitExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNARYEXPRORTYPETRAITEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCENCODEEXPR, "clang::ObjCEncodeExpr");
  g.add_edge((CLASS_OBJCENCODEEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCENCODEEXPR, META_SUBCLASS, CLASS_EXPR));

  g.add_named_node(CLASS_COROUTINEBODYSTMT, "clang::CoroutineBodyStmt");
  g.add_edge((CLASS_COROUTINEBODYSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_COROUTINEBODYSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_HASDEPENDENTPROMISETYPE, "clang::CoroutineBodyStmt::hasDependentPromiseType"),
      add_named_node(&mut g, METHOD_GETBODY_6, "clang::CoroutineBodyStmt::getBody"),
      add_named_node(&mut g, METHOD_GETPROMISEDECLSTMT, "clang::CoroutineBodyStmt::getPromiseDeclStmt"),
      add_named_node(&mut g, METHOD_GETPROMISEDECL, "clang::CoroutineBodyStmt::getPromiseDecl"),
      add_named_node(&mut g, METHOD_GETINITSUSPENDSTMT, "clang::CoroutineBodyStmt::getInitSuspendStmt"),
      add_named_node(&mut g, METHOD_GETFINALSUSPENDSTMT, "clang::CoroutineBodyStmt::getFinalSuspendStmt"),
      add_named_node(&mut g, METHOD_GETEXCEPTIONHANDLER, "clang::CoroutineBodyStmt::getExceptionHandler"),
      add_named_node(&mut g, METHOD_GETFALLTHROUGHHANDLER, "clang::CoroutineBodyStmt::getFallthroughHandler"),
      add_named_node(&mut g, METHOD_GETALLOCATE, "clang::CoroutineBodyStmt::getAllocate"),
      add_named_node(&mut g, METHOD_GETDEALLOCATE, "clang::CoroutineBodyStmt::getDeallocate"),
      add_named_node(&mut g, METHOD_GETRESULTDECL, "clang::CoroutineBodyStmt::getResultDecl"),
      add_named_node(&mut g, METHOD_GETRETURNVALUEINIT, "clang::CoroutineBodyStmt::getReturnValueInit"),
      add_named_node(&mut g, METHOD_GETRETURNVALUE, "clang::CoroutineBodyStmt::getReturnValue"),
      add_named_node(&mut g, METHOD_GETRETURNSTMT, "clang::CoroutineBodyStmt::getReturnStmt"),
      add_named_node(&mut g, METHOD_GETRETURNSTMTONALLOCFAILURE, "clang::CoroutineBodyStmt::getReturnStmtOnAllocFailure"),
      add_named_node(&mut g, METHOD_GETPARAMMOVES, "clang::CoroutineBodyStmt::getParamMoves"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_61, "clang::CoroutineBodyStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_60, "clang::CoroutineBodyStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_51, "clang::CoroutineBodyStmt::children"),
      add_named_node(&mut g, METHOD_CHILDRENEXCLBODY, "clang::CoroutineBodyStmt::childrenExclBody"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_COROUTINEBODYSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ARRAYTYPETRAITEXPR, "clang::ArrayTypeTraitExpr");
  g.add_edge((CLASS_ARRAYTYPETRAITEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARRAYTYPETRAITEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETBEGINLOC_8, "clang::ArrayTypeTraitExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_7, "clang::ArrayTypeTraitExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETTRAIT, "clang::ArrayTypeTraitExpr::getTrait"),
      add_named_node(&mut g, METHOD_GETQUERIEDTYPE, "clang::ArrayTypeTraitExpr::getQueriedType"),
      add_named_node(&mut g, METHOD_GETQUERIEDTYPESOURCEINFO, "clang::ArrayTypeTraitExpr::getQueriedTypeSourceInfo"),
      add_named_node(&mut g, METHOD_GETVALUE_9, "clang::ArrayTypeTraitExpr::getValue"),
      add_named_node(&mut g, METHOD_GETDIMENSIONEXPRESSION, "clang::ArrayTypeTraitExpr::getDimensionExpression"),
      add_named_node(&mut g, METHOD_CHILDREN_4, "clang::ArrayTypeTraitExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ARRAYTYPETRAITEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCAUTORELEASEPOOLSTMT, "clang::ObjCAutoreleasePoolStmt");
  g.add_edge((CLASS_OBJCAUTORELEASEPOOLSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCAUTORELEASEPOOLSTMT, META_SUBCLASS, CLASS_STMT));

  g.add_named_node(CLASS_OMPCANONICALLOOP, "clang::OMPCanonicalLoop");
  g.add_edge((CLASS_OMPCANONICALLOOP, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPCANONICALLOOP, META_SUBCLASS, CLASS_STMT));

  g.add_named_node(CLASS_OMPTILEDIRECTIVE, "clang::OMPTileDirective");
  g.add_edge((CLASS_OMPTILEDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTILEDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPTRANSFORMATIONDIRECTIVE));

  g.add_named_node(CLASS_CASESTMT, "clang::CaseStmt");
  g.add_edge((CLASS_CASESTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CASESTMT, META_SUBCLASS, CLASS_SWITCHCASE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_CASESTMTISGNURANGE, "clang::CaseStmt::caseStmtIsGNURange"),
      add_named_node(&mut g, METHOD_GETCASELOC, "clang::CaseStmt::getCaseLoc"),
      add_named_node(&mut g, METHOD_GETELLIPSISLOC_5, "clang::CaseStmt::getEllipsisLoc"),
      add_named_node(&mut g, METHOD_GETLHS_4, "clang::CaseStmt::getLHS"),
      add_named_node(&mut g, METHOD_GETRHS_4, "clang::CaseStmt::getRHS"),
      add_named_node(&mut g, METHOD_GETSUBSTMT_1, "clang::CaseStmt::getSubStmt"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_50, "clang::CaseStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_49, "clang::CaseStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_39, "clang::CaseStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CASESTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPLOOPTRANSFORMATIONDIRECTIVE, "clang::OMPLoopTransformationDirective");
  g.add_edge((CLASS_OMPLOOPTRANSFORMATIONDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPLOOPTRANSFORMATIONDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPBASEDDIRECTIVE));

  g.add_named_node(CLASS_CXXBOOLLITERALEXPR, "clang::CXXBoolLiteralExpr");
  g.add_edge((CLASS_CXXBOOLLITERALEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXBOOLLITERALEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETVALUE_4, "clang::CXXBoolLiteralExpr::getValue"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_20, "clang::CXXBoolLiteralExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_19, "clang::CXXBoolLiteralExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETLOCATION_1, "clang::CXXBoolLiteralExpr::getLocation"),
      add_named_node(&mut g, METHOD_CHILDREN_14, "clang::CXXBoolLiteralExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXBOOLLITERALEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SYCLUNIQUESTABLENAMEEXPR, "clang::SYCLUniqueStableNameExpr");
  g.add_edge((CLASS_SYCLUNIQUESTABLENAMEEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SYCLUNIQUESTABLENAMEEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTYPESOURCEINFO_8, "clang::SYCLUniqueStableNameExpr::getTypeSourceInfo"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_114, "clang::SYCLUniqueStableNameExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_113, "clang::SYCLUniqueStableNameExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETLOCATION_14, "clang::SYCLUniqueStableNameExpr::getLocation"),
      add_named_node(&mut g, METHOD_GETLPARENLOCATION, "clang::SYCLUniqueStableNameExpr::getLParenLocation"),
      add_named_node(&mut g, METHOD_GETRPARENLOCATION, "clang::SYCLUniqueStableNameExpr::getRParenLocation"),
      add_named_node(&mut g, METHOD_CHILDREN_101, "clang::SYCLUniqueStableNameExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SYCLUNIQUESTABLENAMEEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPSIMDDIRECTIVE, "clang::OMPSimdDirective");
  g.add_edge((CLASS_OMPSIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPSIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_ASMSTMT, "clang::AsmStmt");
  g.add_edge((CLASS_ASMSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ASMSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETASMLOC_1, "clang::AsmStmt::getAsmLoc"),
      add_named_node(&mut g, METHOD_ISSIMPLE, "clang::AsmStmt::isSimple"),
      add_named_node(&mut g, METHOD_ISVOLATILE_2, "clang::AsmStmt::isVolatile"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_10, "clang::AsmStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_9, "clang::AsmStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_GETNUMOUTPUTS, "clang::AsmStmt::getNumOutputs"),
      add_named_node(&mut g, METHOD_GETNUMPLUSOPERANDS, "clang::AsmStmt::getNumPlusOperands"),
      add_named_node(&mut g, METHOD_GETNUMINPUTS, "clang::AsmStmt::getNumInputs"),
      add_named_node(&mut g, METHOD_GETNUMCLOBBERS, "clang::AsmStmt::getNumClobbers"),
      add_named_node(&mut g, METHOD_BEGIN_INPUTS, "clang::AsmStmt::begin_inputs"),
      add_named_node(&mut g, METHOD_END_INPUTS, "clang::AsmStmt::end_inputs"),
      add_named_node(&mut g, METHOD_INPUTS, "clang::AsmStmt::inputs"),
      add_named_node(&mut g, METHOD_BEGIN_OUTPUTS, "clang::AsmStmt::begin_outputs"),
      add_named_node(&mut g, METHOD_END_OUTPUTS, "clang::AsmStmt::end_outputs"),
      add_named_node(&mut g, METHOD_OUTPUTS, "clang::AsmStmt::outputs"),
      add_named_node(&mut g, METHOD_CHILDREN_6, "clang::AsmStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ASMSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPFORSIMDDIRECTIVE, "clang::OMPForSimdDirective");
  g.add_edge((CLASS_OMPFORSIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPFORSIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_OBJCBOXEDEXPR, "clang::ObjCBoxedExpr");
  g.add_edge((CLASS_OBJCBOXEDEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCBOXEDEXPR, META_SUBCLASS, CLASS_EXPR));

  g.add_named_node(CLASS_OMPPARALLELSECTIONSDIRECTIVE, "clang::OMPParallelSectionsDirective");
  g.add_edge((CLASS_OMPPARALLELSECTIONSDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPPARALLELSECTIONSDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_OBJCAVAILABILITYCHECKEXPR, "clang::ObjCAvailabilityCheckExpr");
  g.add_edge((CLASS_OBJCAVAILABILITYCHECKEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCAVAILABILITYCHECKEXPR, META_SUBCLASS, CLASS_EXPR));

  g.add_named_node(CLASS_OMPTASKLOOPSIMDDIRECTIVE, "clang::OMPTaskLoopSimdDirective");
  g.add_edge((CLASS_OMPTASKLOOPSIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTASKLOOPSIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_OMPMASTERTASKLOOPSIMDDIRECTIVE, "clang::OMPMasterTaskLoopSimdDirective");
  g.add_edge((CLASS_OMPMASTERTASKLOOPSIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPMASTERTASKLOOPSIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_CXXTRYSTMT, "clang::CXXTryStmt");
  g.add_edge((CLASS_CXXTRYSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXTRYSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETBEGINLOC_44, "clang::CXXTryStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETTRYLOC, "clang::CXXTryStmt::getTryLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_43, "clang::CXXTryStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_GETTRYBLOCK, "clang::CXXTryStmt::getTryBlock"),
      add_named_node(&mut g, METHOD_GETNUMHANDLERS, "clang::CXXTryStmt::getNumHandlers"),
      add_named_node(&mut g, METHOD_CHILDREN_33, "clang::CXXTryStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXTRYSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEFAULTSTMT, "clang::DefaultStmt");
  g.add_edge((CLASS_DEFAULTSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEFAULTSTMT, META_SUBCLASS, CLASS_SWITCHCASE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSUBSTMT_2, "clang::DefaultStmt::getSubStmt"),
      add_named_node(&mut g, METHOD_GETDEFAULTLOC_1, "clang::DefaultStmt::getDefaultLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_65, "clang::DefaultStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_64, "clang::DefaultStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_55, "clang::DefaultStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEFAULTSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPMASKEDTASKLOOPSIMDDIRECTIVE, "clang::OMPMaskedTaskLoopSimdDirective");
  g.add_edge((CLASS_OMPMASKEDTASKLOOPSIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPMASKEDTASKLOOPSIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_OBJCISAEXPR, "clang::ObjCIsaExpr");
  g.add_edge((CLASS_OBJCISAEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCISAEXPR, META_SUBCLASS, CLASS_EXPR));

  g.add_named_node(CLASS_OBJCATFINALLYSTMT, "clang::ObjCAtFinallyStmt");
  g.add_edge((CLASS_OBJCATFINALLYSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCATFINALLYSTMT, META_SUBCLASS, CLASS_STMT));

  g.add_named_node(CLASS_BLOCKEXPR, "clang::BlockExpr");
  g.add_edge((CLASS_BLOCKEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BLOCKEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETBLOCKDECL, "clang::BlockExpr::getBlockDecl"),
      add_named_node(&mut g, METHOD_GETCARETLOCATION_1, "clang::BlockExpr::getCaretLocation"),
      add_named_node(&mut g, METHOD_GETBODY_4, "clang::BlockExpr::getBody"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_15, "clang::BlockExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_14, "clang::BlockExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETFUNCTIONTYPE, "clang::BlockExpr::getFunctionType"),
      add_named_node(&mut g, METHOD_CHILDREN_11, "clang::BlockExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BLOCKEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPMASTERDIRECTIVE, "clang::OMPMasterDirective");
  g.add_edge((CLASS_OMPMASTERDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPMASTERDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_OBJCIVARREFEXPR, "clang::ObjCIvarRefExpr");
  g.add_edge((CLASS_OBJCIVARREFEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCIVARREFEXPR, META_SUBCLASS, CLASS_EXPR));

  g.add_named_node(CLASS_OMPITERATOREXPR, "clang::OMPIteratorExpr");
  g.add_edge((CLASS_OMPITERATOREXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPITERATOREXPR, META_SUBCLASS, CLASS_EXPR));

  g.add_named_node(CLASS_OMPTARGETUPDATEDIRECTIVE, "clang::OMPTargetUpdateDirective");
  g.add_edge((CLASS_OMPTARGETUPDATEDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETUPDATEDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_IFSTMT, "clang::IfStmt");
  g.add_edge((CLASS_IFSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_IFSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_HASINITSTORAGE, "clang::IfStmt::hasInitStorage"),
      add_named_node(&mut g, METHOD_HASVARSTORAGE, "clang::IfStmt::hasVarStorage"),
      add_named_node(&mut g, METHOD_HASELSESTORAGE, "clang::IfStmt::hasElseStorage"),
      add_named_node(&mut g, METHOD_GETCOND_7, "clang::IfStmt::getCond"),
      add_named_node(&mut g, METHOD_GETTHEN, "clang::IfStmt::getThen"),
      add_named_node(&mut g, METHOD_GETELSE, "clang::IfStmt::getElse"),
      add_named_node(&mut g, METHOD_GETCONDITIONVARIABLE_1, "clang::IfStmt::getConditionVariable"),
      add_named_node(&mut g, METHOD_GETCONDITIONVARIABLEDECLSTMT_1, "clang::IfStmt::getConditionVariableDeclStmt"),
      add_named_node(&mut g, METHOD_GETINIT_5, "clang::IfStmt::getInit"),
      add_named_node(&mut g, METHOD_GETIFLOC, "clang::IfStmt::getIfLoc"),
      add_named_node(&mut g, METHOD_GETELSELOC, "clang::IfStmt::getElseLoc"),
      add_named_node(&mut g, METHOD_ISCONSTEVAL_1, "clang::IfStmt::isConsteval"),
      add_named_node(&mut g, METHOD_ISNONNEGATEDCONSTEVAL, "clang::IfStmt::isNonNegatedConsteval"),
      add_named_node(&mut g, METHOD_ISNEGATEDCONSTEVAL, "clang::IfStmt::isNegatedConsteval"),
      add_named_node(&mut g, METHOD_ISCONSTEXPR_2, "clang::IfStmt::isConstexpr"),
      add_named_node(&mut g, METHOD_GETSTATEMENTKIND, "clang::IfStmt::getStatementKind"),
      add_named_node(&mut g, METHOD_ISOBJCAVAILABILITYCHECK, "clang::IfStmt::isObjCAvailabilityCheck"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_82, "clang::IfStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_81, "clang::IfStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_GETLPARENLOC_6, "clang::IfStmt::getLParenLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_18, "clang::IfStmt::getRParenLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_71, "clang::IfStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_IFSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_RECOVERYEXPR, "clang::RecoveryExpr");
  g.add_edge((CLASS_RECOVERYEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RECOVERYEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_SUBEXPRESSIONS, "clang::RecoveryExpr::subExpressions"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_107, "clang::RecoveryExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_106, "clang::RecoveryExpr::getEndLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_RECOVERYEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPDISTRIBUTEPARALLELFORDIRECTIVE, "clang::OMPDistributeParallelForDirective");
  g.add_edge((CLASS_OMPDISTRIBUTEPARALLELFORDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPDISTRIBUTEPARALLELFORDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_OMPTARGETPARALLELDIRECTIVE, "clang::OMPTargetParallelDirective");
  g.add_edge((CLASS_OMPTARGETPARALLELDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETPARALLELDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_CONSTANTEXPR, "clang::ConstantExpr");
  g.add_edge((CLASS_CONSTANTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONSTANTEXPR, META_SUBCLASS, CLASS_FULLEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETBEGINLOC_57, "clang::ConstantExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_56, "clang::ConstantExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETRESULTAPVALUEKIND, "clang::ConstantExpr::getResultAPValueKind"),
      add_named_node(&mut g, METHOD_GETRESULTSTORAGEKIND, "clang::ConstantExpr::getResultStorageKind"),
      add_named_node(&mut g, METHOD_ISIMMEDIATEINVOCATION, "clang::ConstantExpr::isImmediateInvocation"),
      add_named_node(&mut g, METHOD_HASAPVALUERESULT, "clang::ConstantExpr::hasAPValueResult"),
      add_named_node(&mut g, METHOD_GETAPVALUERESULT, "clang::ConstantExpr::getAPValueResult"),
      add_named_node(&mut g, METHOD_GETRESULTASAPSINT, "clang::ConstantExpr::getResultAsAPSInt"),
      add_named_node(&mut g, METHOD_CHILDREN_47, "clang::ConstantExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CONSTANTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_INITLISTEXPR, "clang::InitListExpr");
  g.add_edge((CLASS_INITLISTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_INITLISTEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETNUMINITS, "clang::InitListExpr::getNumInits"),
      add_named_node(&mut g, METHOD_GETINITS, "clang::InitListExpr::getInits"),
      add_named_node(&mut g, METHOD_INITS_1, "clang::InitListExpr::inits"),
      add_named_node(&mut g, METHOD_GETARRAYFILLER_1, "clang::InitListExpr::getArrayFiller"),
      add_named_node(&mut g, METHOD_HASARRAYFILLER, "clang::InitListExpr::hasArrayFiller"),
      add_named_node(&mut g, METHOD_HASDESIGNATEDINIT, "clang::InitListExpr::hasDesignatedInit"),
      add_named_node(&mut g, METHOD_GETINITIALIZEDFIELDINUNION_1, "clang::InitListExpr::getInitializedFieldInUnion"),
      add_named_node(&mut g, METHOD_ISEXPLICIT_3, "clang::InitListExpr::isExplicit"),
      add_named_node(&mut g, METHOD_ISSTRINGLITERALINIT, "clang::InitListExpr::isStringLiteralInit"),
      add_named_node(&mut g, METHOD_ISTRANSPARENT, "clang::InitListExpr::isTransparent"),
      add_named_node(&mut g, METHOD_GETLBRACELOC_1, "clang::InitListExpr::getLBraceLoc"),
      add_named_node(&mut g, METHOD_GETRBRACELOC_4, "clang::InitListExpr::getRBraceLoc"),
      add_named_node(&mut g, METHOD_ISSEMANTICFORM, "clang::InitListExpr::isSemanticForm"),
      add_named_node(&mut g, METHOD_GETSEMANTICFORM_1, "clang::InitListExpr::getSemanticForm"),
      add_named_node(&mut g, METHOD_ISSYNTACTICFORM, "clang::InitListExpr::isSyntacticForm"),
      add_named_node(&mut g, METHOD_GETSYNTACTICFORM, "clang::InitListExpr::getSyntacticForm"),
      add_named_node(&mut g, METHOD_HADARRAYRANGEDESIGNATOR, "clang::InitListExpr::hadArrayRangeDesignator"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_87, "clang::InitListExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_86, "clang::InitListExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_75, "clang::InitListExpr::children"),
      add_named_node(&mut g, METHOD_BEGIN, "clang::InitListExpr::begin"),
      add_named_node(&mut g, METHOD_END, "clang::InitListExpr::end"),
      add_named_node(&mut g, METHOD_RBEGIN, "clang::InitListExpr::rbegin"),
      add_named_node(&mut g, METHOD_REND, "clang::InitListExpr::rend"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_INITLISTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTARGETTEAMSGENERICLOOPDIRECTIVE, "clang::OMPTargetTeamsGenericLoopDirective");
  g.add_edge((CLASS_OMPTARGETTEAMSGENERICLOOPDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETTEAMSGENERICLOOPDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_SWITCHCASE, "clang::SwitchCase");
  g.add_edge((CLASS_SWITCHCASE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWITCHCASE, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETNEXTSWITCHCASE, "clang::SwitchCase::getNextSwitchCase"),
      add_named_node(&mut g, METHOD_GETKEYWORDLOC_4, "clang::SwitchCase::getKeywordLoc"),
      add_named_node(&mut g, METHOD_GETCOLONLOC_3, "clang::SwitchCase::getColonLoc"),
      add_named_node(&mut g, METHOD_GETSUBSTMT_6, "clang::SwitchCase::getSubStmt"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_122, "clang::SwitchCase::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_121, "clang::SwitchCase::getEndLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SWITCHCASE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTARGETDATADIRECTIVE, "clang::OMPTargetDataDirective");
  g.add_edge((CLASS_OMPTARGETDATADIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETDATADIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_OMPTEAMSDISTRIBUTEDIRECTIVE, "clang::OMPTeamsDistributeDirective");
  g.add_edge((CLASS_OMPTEAMSDISTRIBUTEDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTEAMSDISTRIBUTEDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_OMPTASKGROUPDIRECTIVE, "clang::OMPTaskgroupDirective");
  g.add_edge((CLASS_OMPTASKGROUPDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTASKGROUPDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_OMPUNROLLDIRECTIVE, "clang::OMPUnrollDirective");
  g.add_edge((CLASS_OMPUNROLLDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPUNROLLDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPTRANSFORMATIONDIRECTIVE));

  g.add_named_node(CLASS_FUNCTIONPARMPACKEXPR, "clang::FunctionParmPackExpr");
  g.add_edge((CLASS_FUNCTIONPARMPACKEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FUNCTIONPARMPACKEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETPARAMETERPACK, "clang::FunctionParmPackExpr::getParameterPack"),
      add_named_node(&mut g, METHOD_GETPARAMETERPACKLOCATION, "clang::FunctionParmPackExpr::getParameterPackLocation"),
      add_named_node(&mut g, METHOD_BEGIN_1, "clang::FunctionParmPackExpr::begin"),
      add_named_node(&mut g, METHOD_END_1, "clang::FunctionParmPackExpr::end"),
      add_named_node(&mut g, METHOD_GETNUMEXPANSIONS_2, "clang::FunctionParmPackExpr::getNumExpansions"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_77, "clang::FunctionParmPackExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_76, "clang::FunctionParmPackExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_67, "clang::FunctionParmPackExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FUNCTIONPARMPACKEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTARGETTEAMSDISTRIBUTEDIRECTIVE, "clang::OMPTargetTeamsDistributeDirective");
  g.add_edge((CLASS_OMPTARGETTEAMSDISTRIBUTEDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETTEAMSDISTRIBUTEDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_DECLREFEXPR, "clang::DeclRefExpr");
  g.add_edge((CLASS_DECLREFEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DECLREFEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETDECL_7, "clang::DeclRefExpr::getDecl"),
      add_named_node(&mut g, METHOD_GETNAMEINFO_4, "clang::DeclRefExpr::getNameInfo"),
      add_named_node(&mut g, METHOD_GETLOCATION_7, "clang::DeclRefExpr::getLocation"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_63, "clang::DeclRefExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_62, "clang::DeclRefExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_HASQUALIFIER_1, "clang::DeclRefExpr::hasQualifier"),
      add_named_node(&mut g, METHOD_GETQUALIFIERLOC_10, "clang::DeclRefExpr::getQualifierLoc"),
      add_named_node(&mut g, METHOD_GETQUALIFIER_13, "clang::DeclRefExpr::getQualifier"),
      add_named_node(&mut g, METHOD_GETFOUNDDECL_2, "clang::DeclRefExpr::getFoundDecl"),
      add_named_node(&mut g, METHOD_HASTEMPLATEKWANDARGSINFO, "clang::DeclRefExpr::hasTemplateKWAndArgsInfo"),
      add_named_node(&mut g, METHOD_GETTEMPLATEKEYWORDLOC_3, "clang::DeclRefExpr::getTemplateKeywordLoc"),
      add_named_node(&mut g, METHOD_GETLANGLELOC_1, "clang::DeclRefExpr::getLAngleLoc"),
      add_named_node(&mut g, METHOD_GETRANGLELOC_1, "clang::DeclRefExpr::getRAngleLoc"),
      add_named_node(&mut g, METHOD_HASTEMPLATEKEYWORD_1, "clang::DeclRefExpr::hasTemplateKeyword"),
      add_named_node(&mut g, METHOD_HASEXPLICITTEMPLATEARGS_2, "clang::DeclRefExpr::hasExplicitTemplateArgs"),
      add_named_node(&mut g, METHOD_GETTEMPLATEARGS_3, "clang::DeclRefExpr::getTemplateArgs"),
      add_named_node(&mut g, METHOD_GETNUMTEMPLATEARGS_1, "clang::DeclRefExpr::getNumTemplateArgs"),
      add_named_node(&mut g, METHOD_TEMPLATE_ARGUMENTS_3, "clang::DeclRefExpr::template_arguments"),
      add_named_node(&mut g, METHOD_HADMULTIPLECANDIDATES_1, "clang::DeclRefExpr::hadMultipleCandidates"),
      add_named_node(&mut g, METHOD_ISNONODRUSE, "clang::DeclRefExpr::isNonOdrUse"),
      add_named_node(&mut g, METHOD_REFERSTOENCLOSINGVARIABLEORCAPTURE, "clang::DeclRefExpr::refersToEnclosingVariableOrCapture"),
      add_named_node(&mut g, METHOD_ISIMMEDIATEESCALATING_2, "clang::DeclRefExpr::isImmediateEscalating"),
      add_named_node(&mut g, METHOD_ISCAPTUREDBYCOPYINLAMBDAWITHEXPLICITOBJECTPARAMETER, "clang::DeclRefExpr::isCapturedByCopyInLambdaWithExplicitObjectParameter"),
      add_named_node(&mut g, METHOD_CHILDREN_53, "clang::DeclRefExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DECLREFEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FORSTMT, "clang::ForStmt");
  g.add_edge((CLASS_FORSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FORSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETCONDITIONVARIABLE, "clang::ForStmt::getConditionVariable"),
      add_named_node(&mut g, METHOD_GETCONDITIONVARIABLEDECLSTMT, "clang::ForStmt::getConditionVariableDeclStmt"),
      add_named_node(&mut g, METHOD_GETINIT_4, "clang::ForStmt::getInit"),
      add_named_node(&mut g, METHOD_GETCOND_6, "clang::ForStmt::getCond"),
      add_named_node(&mut g, METHOD_GETINC_1, "clang::ForStmt::getInc"),
      add_named_node(&mut g, METHOD_GETBODY_8, "clang::ForStmt::getBody"),
      add_named_node(&mut g, METHOD_GETFORLOC_1, "clang::ForStmt::getForLoc"),
      add_named_node(&mut g, METHOD_GETLPARENLOC_5, "clang::ForStmt::getLParenLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_15, "clang::ForStmt::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_76, "clang::ForStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_75, "clang::ForStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_66, "clang::ForStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FORSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OVERLOADEXPR, "clang::OverloadExpr");
  g.add_edge((CLASS_OVERLOADEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OVERLOADEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETNAMINGCLASS, "clang::OverloadExpr::getNamingClass"),
      add_named_node(&mut g, METHOD_DECLS_BEGIN, "clang::OverloadExpr::decls_begin"),
      add_named_node(&mut g, METHOD_DECLS_END, "clang::OverloadExpr::decls_end"),
      add_named_node(&mut g, METHOD_DECLS_1, "clang::OverloadExpr::decls"),
      add_named_node(&mut g, METHOD_GETNUMDECLS, "clang::OverloadExpr::getNumDecls"),
      add_named_node(&mut g, METHOD_GETNAMEINFO_7, "clang::OverloadExpr::getNameInfo"),
      add_named_node(&mut g, METHOD_GETNAME_2, "clang::OverloadExpr::getName"),
      add_named_node(&mut g, METHOD_GETNAMELOC, "clang::OverloadExpr::getNameLoc"),
      add_named_node(&mut g, METHOD_GETQUALIFIER_16, "clang::OverloadExpr::getQualifier"),
      add_named_node(&mut g, METHOD_GETQUALIFIERLOC_15, "clang::OverloadExpr::getQualifierLoc"),
      add_named_node(&mut g, METHOD_GETTEMPLATEKEYWORDLOC_6, "clang::OverloadExpr::getTemplateKeywordLoc"),
      add_named_node(&mut g, METHOD_GETLANGLELOC_4, "clang::OverloadExpr::getLAngleLoc"),
      add_named_node(&mut g, METHOD_GETRANGLELOC_4, "clang::OverloadExpr::getRAngleLoc"),
      add_named_node(&mut g, METHOD_HASTEMPLATEKEYWORD_4, "clang::OverloadExpr::hasTemplateKeyword"),
      add_named_node(&mut g, METHOD_HASEXPLICITTEMPLATEARGS_5, "clang::OverloadExpr::hasExplicitTemplateArgs"),
      add_named_node(&mut g, METHOD_GETTEMPLATEARGS_6, "clang::OverloadExpr::getTemplateArgs"),
      add_named_node(&mut g, METHOD_GETNUMTEMPLATEARGS_4, "clang::OverloadExpr::getNumTemplateArgs"),
      add_named_node(&mut g, METHOD_TEMPLATE_ARGUMENTS_6, "clang::OverloadExpr::template_arguments"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OVERLOADEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPCANCELLATIONPOINTDIRECTIVE, "clang::OMPCancellationPointDirective");
  g.add_edge((CLASS_OMPCANCELLATIONPOINTDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPCANCELLATIONPOINTDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_FIXEDPOINTLITERAL, "clang::FixedPointLiteral");
  g.add_edge((CLASS_FIXEDPOINTLITERAL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FIXEDPOINTLITERAL, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETBEGINLOC_74, "clang::FixedPointLiteral::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_73, "clang::FixedPointLiteral::getEndLoc"),
      add_named_node(&mut g, METHOD_GETLOCATION_9, "clang::FixedPointLiteral::getLocation"),
      add_named_node(&mut g, METHOD_GETSCALE, "clang::FixedPointLiteral::getScale"),
      add_named_node(&mut g, METHOD_CHILDREN_64, "clang::FixedPointLiteral::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FIXEDPOINTLITERAL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCATTHROWSTMT, "clang::ObjCAtThrowStmt");
  g.add_edge((CLASS_OBJCATTHROWSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCATTHROWSTMT, META_SUBCLASS, CLASS_STMT));

  g.add_named_node(CLASS_OMPTARGETEXITDATADIRECTIVE, "clang::OMPTargetExitDataDirective");
  g.add_edge((CLASS_OMPTARGETEXITDATADIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETEXITDATADIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_OMPTARGETENTERDATADIRECTIVE, "clang::OMPTargetEnterDataDirective");
  g.add_edge((CLASS_OMPTARGETENTERDATADIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETENTERDATADIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_OMPTARGETTEAMSDISTRIBUTEPARALLELFORDIRECTIVE, "clang::OMPTargetTeamsDistributeParallelForDirective");
  g.add_edge((CLASS_OMPTARGETTEAMSDISTRIBUTEPARALLELFORDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETTEAMSDISTRIBUTEPARALLELFORDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_OMPFLUSHDIRECTIVE, "clang::OMPFlushDirective");
  g.add_edge((CLASS_OMPFLUSHDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPFLUSHDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_OMPTARGETTEAMSDIRECTIVE, "clang::OMPTargetTeamsDirective");
  g.add_edge((CLASS_OMPTARGETTEAMSDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETTEAMSDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_OMPSCANDIRECTIVE, "clang::OMPScanDirective");
  g.add_edge((CLASS_OMPSCANDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPSCANDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_OMPMETADIRECTIVE, "clang::OMPMetaDirective");
  g.add_edge((CLASS_OMPMETADIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPMETADIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_OMPATOMICDIRECTIVE, "clang::OMPAtomicDirective");
  g.add_edge((CLASS_OMPATOMICDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPATOMICDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_NULLSTMT, "clang::NullStmt");
  g.add_edge((CLASS_NULLSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NULLSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSEMILOC, "clang::NullStmt::getSemiLoc"),
      add_named_node(&mut g, METHOD_HASLEADINGEMPTYMACRO, "clang::NullStmt::hasLeadingEmptyMacro"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_99, "clang::NullStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_98, "clang::NullStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_87, "clang::NullStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NULLSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPDISTRIBUTEPARALLELFORSIMDDIRECTIVE, "clang::OMPDistributeParallelForSimdDirective");
  g.add_edge((CLASS_OMPDISTRIBUTEPARALLELFORSIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPDISTRIBUTEPARALLELFORSIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_OMPGENERICLOOPDIRECTIVE, "clang::OMPGenericLoopDirective");
  g.add_edge((CLASS_OMPGENERICLOOPDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPGENERICLOOPDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_OMPEXECUTABLEDIRECTIVE, "clang::OMPExecutableDirective");
  g.add_edge((CLASS_OMPEXECUTABLEDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPEXECUTABLEDIRECTIVE, META_SUBCLASS, CLASS_STMT));

  g.add_named_node(CLASS_CXXDEFAULTINITEXPR, "clang::CXXDefaultInitExpr");
  g.add_edge((CLASS_CXXDEFAULTINITEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXDEFAULTINITEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_HASREWRITTENINIT_1, "clang::CXXDefaultInitExpr::hasRewrittenInit"),
      add_named_node(&mut g, METHOD_GETFIELD, "clang::CXXDefaultInitExpr::getField"),
      add_named_node(&mut g, METHOD_GETEXPR_1, "clang::CXXDefaultInitExpr::getExpr"),
      add_named_node(&mut g, METHOD_GETREWRITTENEXPR_1, "clang::CXXDefaultInitExpr::getRewrittenExpr"),
      add_named_node(&mut g, METHOD_GETUSEDCONTEXT_1, "clang::CXXDefaultInitExpr::getUsedContext"),
      add_named_node(&mut g, METHOD_GETUSEDLOCATION_1, "clang::CXXDefaultInitExpr::getUsedLocation"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_24, "clang::CXXDefaultInitExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_23, "clang::CXXDefaultInitExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_18, "clang::CXXDefaultInitExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXDEFAULTINITEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPMASTERTASKLOOPDIRECTIVE, "clang::OMPMasterTaskLoopDirective");
  g.add_edge((CLASS_OMPMASTERTASKLOOPDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPMASTERTASKLOOPDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_OMPPARALLELMASTERTASKLOOPSIMDDIRECTIVE, "clang::OMPParallelMasterTaskLoopSimdDirective");
  g.add_edge((CLASS_OMPPARALLELMASTERTASKLOOPSIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPPARALLELMASTERTASKLOOPSIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_EXPLICITCASTEXPR, "clang::ExplicitCastExpr");
  g.add_edge((CLASS_EXPLICITCASTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_EXPLICITCASTEXPR, META_SUBCLASS, CLASS_CASTEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTYPEINFOASWRITTEN, "clang::ExplicitCastExpr::getTypeInfoAsWritten"),
      add_named_node(&mut g, METHOD_GETTYPEASWRITTEN_3, "clang::ExplicitCastExpr::getTypeAsWritten"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_EXPLICITCASTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPSECTIONSDIRECTIVE, "clang::OMPSectionsDirective");
  g.add_edge((CLASS_OMPSECTIONSDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPSECTIONSDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_OMPLOOPDIRECTIVE, "clang::OMPLoopDirective");
  g.add_edge((CLASS_OMPLOOPDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPLOOPDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPBASEDDIRECTIVE));

  g.add_named_node(CLASS_OMPARRAYSECTIONEXPR, "clang::OMPArraySectionExpr");
  g.add_edge((CLASS_OMPARRAYSECTIONEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPARRAYSECTIONEXPR, META_SUBCLASS, CLASS_EXPR));

  g.add_named_node(CLASS_OMPMASKEDDIRECTIVE, "clang::OMPMaskedDirective");
  g.add_edge((CLASS_OMPMASKEDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPMASKEDDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_CXXUNRESOLVEDCONSTRUCTEXPR, "clang::CXXUnresolvedConstructExpr");
  g.add_edge((CLASS_CXXUNRESOLVEDCONSTRUCTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXUNRESOLVEDCONSTRUCTEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTYPEASWRITTEN_2, "clang::CXXUnresolvedConstructExpr::getTypeAsWritten"),
      add_named_node(&mut g, METHOD_GETTYPESOURCEINFO_4, "clang::CXXUnresolvedConstructExpr::getTypeSourceInfo"),
      add_named_node(&mut g, METHOD_GETLPARENLOC_3, "clang::CXXUnresolvedConstructExpr::getLParenLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_10, "clang::CXXUnresolvedConstructExpr::getRParenLoc"),
      add_named_node(&mut g, METHOD_ISLISTINITIALIZATION_2, "clang::CXXUnresolvedConstructExpr::isListInitialization"),
      add_named_node(&mut g, METHOD_GETNUMARGS_2, "clang::CXXUnresolvedConstructExpr::getNumArgs"),
      add_named_node(&mut g, METHOD_ARG_BEGIN, "clang::CXXUnresolvedConstructExpr::arg_begin"),
      add_named_node(&mut g, METHOD_ARG_END, "clang::CXXUnresolvedConstructExpr::arg_end"),
      add_named_node(&mut g, METHOD_ARGUMENTS_1, "clang::CXXUnresolvedConstructExpr::arguments"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_46, "clang::CXXUnresolvedConstructExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_45, "clang::CXXUnresolvedConstructExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_35, "clang::CXXUnresolvedConstructExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXUNRESOLVEDCONSTRUCTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_USERDEFINEDLITERAL, "clang::UserDefinedLiteral");
  g.add_edge((CLASS_USERDEFINEDLITERAL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_USERDEFINEDLITERAL, META_SUBCLASS, CLASS_CALLEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETLITERALOPERATORKIND, "clang::UserDefinedLiteral::getLiteralOperatorKind"),
      add_named_node(&mut g, METHOD_GETCOOKEDLITERAL, "clang::UserDefinedLiteral::getCookedLiteral"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_130, "clang::UserDefinedLiteral::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_129, "clang::UserDefinedLiteral::getEndLoc"),
      add_named_node(&mut g, METHOD_GETUDSUFFIXLOC, "clang::UserDefinedLiteral::getUDSuffixLoc"),
      add_named_node(&mut g, METHOD_GETUDSUFFIX, "clang::UserDefinedLiteral::getUDSuffix"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_USERDEFINEDLITERAL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXCATCHSTMT, "clang::CXXCatchStmt");
  g.add_edge((CLASS_CXXCATCHSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXCATCHSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETBEGINLOC_21, "clang::CXXCatchStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_20, "clang::CXXCatchStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_GETCATCHLOC, "clang::CXXCatchStmt::getCatchLoc"),
      add_named_node(&mut g, METHOD_GETEXCEPTIONDECL, "clang::CXXCatchStmt::getExceptionDecl"),
      add_named_node(&mut g, METHOD_GETCAUGHTTYPE, "clang::CXXCatchStmt::getCaughtType"),
      add_named_node(&mut g, METHOD_GETHANDLERBLOCK, "clang::CXXCatchStmt::getHandlerBlock"),
      add_named_node(&mut g, METHOD_CHILDREN_15, "clang::CXXCatchStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXCATCHSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPSCOPEDIRECTIVE, "clang::OMPScopeDirective");
  g.add_edge((CLASS_OMPSCOPEDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPSCOPEDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_CXXADDRSPACECASTEXPR, "clang::CXXAddrspaceCastExpr");
  g.add_edge((CLASS_CXXADDRSPACECASTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXADDRSPACECASTEXPR, META_SUBCLASS, CLASS_CXXNAMEDCASTEXPR));

  g.add_named_node(CLASS_OMPTEAMSDISTRIBUTEPARALLELFORSIMDDIRECTIVE, "clang::OMPTeamsDistributeParallelForSimdDirective");
  g.add_edge((CLASS_OMPTEAMSDISTRIBUTEPARALLELFORSIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTEAMSDISTRIBUTEPARALLELFORSIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_OMPARRAYSHAPINGEXPR, "clang::OMPArrayShapingExpr");
  g.add_edge((CLASS_OMPARRAYSHAPINGEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPARRAYSHAPINGEXPR, META_SUBCLASS, CLASS_EXPR));

  g.add_named_node(CLASS_UNRESOLVEDLOOKUPEXPR, "clang::UnresolvedLookupExpr");
  g.add_edge((CLASS_UNRESOLVEDLOOKUPEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNRESOLVEDLOOKUPEXPR, META_SUBCLASS, CLASS_OVERLOADEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_REQUIRESADL, "clang::UnresolvedLookupExpr::requiresADL"),
      add_named_node(&mut g, METHOD_ISOVERLOADED, "clang::UnresolvedLookupExpr::isOverloaded"),
      add_named_node(&mut g, METHOD_GETNAMINGCLASS_1, "clang::UnresolvedLookupExpr::getNamingClass"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_128, "clang::UnresolvedLookupExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_127, "clang::UnresolvedLookupExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_115, "clang::UnresolvedLookupExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNRESOLVEDLOOKUPEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTEAMSDISTRIBUTESIMDDIRECTIVE, "clang::OMPTeamsDistributeSimdDirective");
  g.add_edge((CLASS_OMPTEAMSDISTRIBUTESIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTEAMSDISTRIBUTESIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_ABSTRACTCONDITIONALOPERATOR, "clang::AbstractConditionalOperator");
  g.add_edge((CLASS_ABSTRACTCONDITIONALOPERATOR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ABSTRACTCONDITIONALOPERATOR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETCOND, "clang::AbstractConditionalOperator::getCond"),
      add_named_node(&mut g, METHOD_GETTRUEEXPR, "clang::AbstractConditionalOperator::getTrueExpr"),
      add_named_node(&mut g, METHOD_GETFALSEEXPR, "clang::AbstractConditionalOperator::getFalseExpr"),
      add_named_node(&mut g, METHOD_GETQUESTIONLOC, "clang::AbstractConditionalOperator::getQuestionLoc"),
      add_named_node(&mut g, METHOD_GETCOLONLOC_1, "clang::AbstractConditionalOperator::getColonLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ABSTRACTCONDITIONALOPERATOR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BINARYCONDITIONALOPERATOR, "clang::BinaryConditionalOperator");
  g.add_edge((CLASS_BINARYCONDITIONALOPERATOR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BINARYCONDITIONALOPERATOR, META_SUBCLASS, CLASS_ABSTRACTCONDITIONALOPERATOR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETCOMMON, "clang::BinaryConditionalOperator::getCommon"),
      add_named_node(&mut g, METHOD_GETOPAQUEVALUE, "clang::BinaryConditionalOperator::getOpaqueValue"),
      add_named_node(&mut g, METHOD_GETCOND_1, "clang::BinaryConditionalOperator::getCond"),
      add_named_node(&mut g, METHOD_GETTRUEEXPR_1, "clang::BinaryConditionalOperator::getTrueExpr"),
      add_named_node(&mut g, METHOD_GETFALSEEXPR_1, "clang::BinaryConditionalOperator::getFalseExpr"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_13, "clang::BinaryConditionalOperator::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_12, "clang::BinaryConditionalOperator::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_9, "clang::BinaryConditionalOperator::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BINARYCONDITIONALOPERATOR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPBARRIERDIRECTIVE, "clang::OMPBarrierDirective");
  g.add_edge((CLASS_OMPBARRIERDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPBARRIERDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_OMPTASKDIRECTIVE, "clang::OMPTaskDirective");
  g.add_edge((CLASS_OMPTASKDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTASKDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_SHUFFLEVECTOREXPR, "clang::ShuffleVectorExpr");
  g.add_edge((CLASS_SHUFFLEVECTOREXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SHUFFLEVECTOREXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETBUILTINLOC_4, "clang::ShuffleVectorExpr::getBuiltinLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_22, "clang::ShuffleVectorExpr::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_115, "clang::ShuffleVectorExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_114, "clang::ShuffleVectorExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETNUMSUBEXPRS_2, "clang::ShuffleVectorExpr::getNumSubExprs"),
      add_named_node(&mut g, METHOD_CHILDREN_102, "clang::ShuffleVectorExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SHUFFLEVECTOREXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTARGETPARALLELFORSIMDDIRECTIVE, "clang::OMPTargetParallelForSimdDirective");
  g.add_edge((CLASS_OMPTARGETPARALLELFORSIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETPARALLELFORSIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_OMPFORDIRECTIVE, "clang::OMPForDirective");
  g.add_edge((CLASS_OMPFORDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPFORDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_CONCEPTSPECIALIZATIONEXPR, "clang::ConceptSpecializationExpr");
  g.add_edge((CLASS_CONCEPTSPECIALIZATIONEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONCEPTSPECIALIZATIONEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTEMPLATEARGUMENTS_1, "clang::ConceptSpecializationExpr::getTemplateArguments"),
      add_named_node(&mut g, METHOD_GETCONCEPTREFERENCE, "clang::ConceptSpecializationExpr::getConceptReference"),
      add_named_node(&mut g, METHOD_GETNAMEDCONCEPT, "clang::ConceptSpecializationExpr::getNamedConcept"),
      add_named_node(&mut g, METHOD_HASEXPLICITTEMPLATEARGS_1, "clang::ConceptSpecializationExpr::hasExplicitTemplateArgs"),
      add_named_node(&mut g, METHOD_GETCONCEPTNAMELOC, "clang::ConceptSpecializationExpr::getConceptNameLoc"),
      add_named_node(&mut g, METHOD_GETTEMPLATEARGSASWRITTEN_2, "clang::ConceptSpecializationExpr::getTemplateArgsAsWritten"),
      add_named_node(&mut g, METHOD_GETNESTEDNAMESPECIFIERLOC, "clang::ConceptSpecializationExpr::getNestedNameSpecifierLoc"),
      add_named_node(&mut g, METHOD_GETTEMPLATEKWLOC, "clang::ConceptSpecializationExpr::getTemplateKWLoc"),
      add_named_node(&mut g, METHOD_GETFOUNDDECL_1, "clang::ConceptSpecializationExpr::getFoundDecl"),
      add_named_node(&mut g, METHOD_GETCONCEPTNAMEINFO, "clang::ConceptSpecializationExpr::getConceptNameInfo"),
      add_named_node(&mut g, METHOD_GETSPECIALIZATIONDECL, "clang::ConceptSpecializationExpr::getSpecializationDecl"),
      add_named_node(&mut g, METHOD_ISSATISFIED, "clang::ConceptSpecializationExpr::isSatisfied"),
      add_named_node(&mut g, METHOD_GETSATISFACTION, "clang::ConceptSpecializationExpr::getSatisfaction"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_55, "clang::ConceptSpecializationExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_54, "clang::ConceptSpecializationExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETEXPRLOC_6, "clang::ConceptSpecializationExpr::getExprLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_45, "clang::ConceptSpecializationExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CONCEPTSPECIALIZATIONEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCBOOLLITERALEXPR, "clang::ObjCBoolLiteralExpr");
  g.add_edge((CLASS_OBJCBOOLLITERALEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCBOOLLITERALEXPR, META_SUBCLASS, CLASS_EXPR));

  g.add_named_node(CLASS_GNUNULLEXPR, "clang::GNUNullExpr");
  g.add_edge((CLASS_GNUNULLEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_GNUNULLEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTOKENLOCATION, "clang::GNUNullExpr::getTokenLocation"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_79, "clang::GNUNullExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_78, "clang::GNUNullExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_68, "clang::GNUNullExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_GNUNULLEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ADDRLABELEXPR, "clang::AddrLabelExpr");
  g.add_edge((CLASS_ADDRLABELEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ADDRLABELEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETAMPAMPLOC, "clang::AddrLabelExpr::getAmpAmpLoc"),
      add_named_node(&mut g, METHOD_GETLABELLOC, "clang::AddrLabelExpr::getLabelLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_4, "clang::AddrLabelExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_3, "clang::AddrLabelExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETLABEL, "clang::AddrLabelExpr::getLabel"),
      add_named_node(&mut g, METHOD_CHILDREN, "clang::AddrLabelExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ADDRLABELEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPPARALLELMASKEDTASKLOOPSIMDDIRECTIVE, "clang::OMPParallelMaskedTaskLoopSimdDirective");
  g.add_edge((CLASS_OMPPARALLELMASKEDTASKLOOPSIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPPARALLELMASKEDTASKLOOPSIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_OBJCSTRINGLITERAL, "clang::ObjCStringLiteral");
  g.add_edge((CLASS_OBJCSTRINGLITERAL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCSTRINGLITERAL, META_SUBCLASS, CLASS_EXPR));

  g.add_named_node(CLASS_OMPINTEROPDIRECTIVE, "clang::OMPInteropDirective");
  g.add_edge((CLASS_OMPINTEROPDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPINTEROPDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_REQUIRESEXPR, "clang::RequiresExpr");
  g.add_edge((CLASS_REQUIRESEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_REQUIRESEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETLOCALPARAMETERS, "clang::RequiresExpr::getLocalParameters"),
      add_named_node(&mut g, METHOD_GETBODY_10, "clang::RequiresExpr::getBody"),
      add_named_node(&mut g, METHOD_GETREQUIREMENTS, "clang::RequiresExpr::getRequirements"),
      add_named_node(&mut g, METHOD_ISSATISFIED_1, "clang::RequiresExpr::isSatisfied"),
      add_named_node(&mut g, METHOD_GETREQUIRESKWLOC, "clang::RequiresExpr::getRequiresKWLoc"),
      add_named_node(&mut g, METHOD_GETLPARENLOC_8, "clang::RequiresExpr::getLParenLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_21, "clang::RequiresExpr::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETRBRACELOC_5, "clang::RequiresExpr::getRBraceLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_108, "clang::RequiresExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_107, "clang::RequiresExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_95, "clang::RequiresExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_REQUIRESEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTCOAWAITEXPR, "clang::DependentCoawaitExpr");
  g.add_edge((CLASS_DEPENDENTCOAWAITEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEPENDENTCOAWAITEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETOPERAND_3, "clang::DependentCoawaitExpr::getOperand"),
      add_named_node(&mut g, METHOD_GETOPERATORCOAWAITLOOKUP, "clang::DependentCoawaitExpr::getOperatorCoawaitLookup"),
      add_named_node(&mut g, METHOD_GETKEYWORDLOC_2, "clang::DependentCoawaitExpr::getKeywordLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_66, "clang::DependentCoawaitExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_65, "clang::DependentCoawaitExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_56, "clang::DependentCoawaitExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTCOAWAITEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_LAMBDAEXPR, "clang::LambdaExpr");
  g.add_edge((CLASS_LAMBDAEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LAMBDAEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETCAPTUREDEFAULT, "clang::LambdaExpr::getCaptureDefault"),
      add_named_node(&mut g, METHOD_GETCAPTUREDEFAULTLOC, "clang::LambdaExpr::getCaptureDefaultLoc"),
      add_named_node(&mut g, METHOD_CAPTURES_3, "clang::LambdaExpr::captures"),
      add_named_node(&mut g, METHOD_CAPTURE_BEGIN, "clang::LambdaExpr::capture_begin"),
      add_named_node(&mut g, METHOD_CAPTURE_END, "clang::LambdaExpr::capture_end"),
      add_named_node(&mut g, METHOD_CAPTURE_SIZE_1, "clang::LambdaExpr::capture_size"),
      add_named_node(&mut g, METHOD_EXPLICIT_CAPTURES, "clang::LambdaExpr::explicit_captures"),
      add_named_node(&mut g, METHOD_EXPLICIT_CAPTURE_BEGIN, "clang::LambdaExpr::explicit_capture_begin"),
      add_named_node(&mut g, METHOD_EXPLICIT_CAPTURE_END, "clang::LambdaExpr::explicit_capture_end"),
      add_named_node(&mut g, METHOD_IMPLICIT_CAPTURES, "clang::LambdaExpr::implicit_captures"),
      add_named_node(&mut g, METHOD_IMPLICIT_CAPTURE_BEGIN, "clang::LambdaExpr::implicit_capture_begin"),
      add_named_node(&mut g, METHOD_IMPLICIT_CAPTURE_END, "clang::LambdaExpr::implicit_capture_end"),
      add_named_node(&mut g, METHOD_CAPTURE_INITS_1, "clang::LambdaExpr::capture_inits"),
      add_named_node(&mut g, METHOD_CAPTURE_INIT_BEGIN, "clang::LambdaExpr::capture_init_begin"),
      add_named_node(&mut g, METHOD_CAPTURE_INIT_END, "clang::LambdaExpr::capture_init_end"),
      add_named_node(&mut g, METHOD_GETINTRODUCERRANGE, "clang::LambdaExpr::getIntroducerRange"),
      add_named_node(&mut g, METHOD_GETLAMBDACLASS, "clang::LambdaExpr::getLambdaClass"),
      add_named_node(&mut g, METHOD_GETCALLOPERATOR, "clang::LambdaExpr::getCallOperator"),
      add_named_node(&mut g, METHOD_GETDEPENDENTCALLOPERATOR, "clang::LambdaExpr::getDependentCallOperator"),
      add_named_node(&mut g, METHOD_GETTEMPLATEPARAMETERLIST, "clang::LambdaExpr::getTemplateParameterList"),
      add_named_node(&mut g, METHOD_GETEXPLICITTEMPLATEPARAMETERS, "clang::LambdaExpr::getExplicitTemplateParameters"),
      add_named_node(&mut g, METHOD_GETTRAILINGREQUIRESCLAUSE_1, "clang::LambdaExpr::getTrailingRequiresClause"),
      add_named_node(&mut g, METHOD_ISGENERICLAMBDA_1, "clang::LambdaExpr::isGenericLambda"),
      add_named_node(&mut g, METHOD_GETBODY_9, "clang::LambdaExpr::getBody"),
      add_named_node(&mut g, METHOD_GETCOMPOUNDSTMTBODY, "clang::LambdaExpr::getCompoundStmtBody"),
      add_named_node(&mut g, METHOD_ISMUTABLE_1, "clang::LambdaExpr::isMutable"),
      add_named_node(&mut g, METHOD_HASEXPLICITPARAMETERS, "clang::LambdaExpr::hasExplicitParameters"),
      add_named_node(&mut g, METHOD_HASEXPLICITRESULTTYPE, "clang::LambdaExpr::hasExplicitResultType"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_90, "clang::LambdaExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_89, "clang::LambdaExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_78, "clang::LambdaExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_LAMBDAEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_COYIELDEXPR, "clang::CoyieldExpr");
  g.add_edge((CLASS_COYIELDEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_COYIELDEXPR, META_SUBCLASS, CLASS_COROUTINESUSPENDEXPR));

  g.add_named_node(CLASS_COAWAITEXPR, "clang::CoawaitExpr");
  g.add_edge((CLASS_COAWAITEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_COAWAITEXPR, META_SUBCLASS, CLASS_COROUTINESUSPENDEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_ISIMPLICIT_2, "clang::CoawaitExpr::isImplicit"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_COAWAITEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXFOLDEXPR, "clang::CXXFoldExpr");
  g.add_edge((CLASS_CXXFOLDEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXFOLDEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETCALLEE, "clang::CXXFoldExpr::getCallee"),
      add_named_node(&mut g, METHOD_GETLHS_2, "clang::CXXFoldExpr::getLHS"),
      add_named_node(&mut g, METHOD_GETRHS_2, "clang::CXXFoldExpr::getRHS"),
      add_named_node(&mut g, METHOD_ISRIGHTFOLD, "clang::CXXFoldExpr::isRightFold"),
      add_named_node(&mut g, METHOD_ISLEFTFOLD, "clang::CXXFoldExpr::isLeftFold"),
      add_named_node(&mut g, METHOD_GETPATTERN_1, "clang::CXXFoldExpr::getPattern"),
      add_named_node(&mut g, METHOD_GETINIT_1, "clang::CXXFoldExpr::getInit"),
      add_named_node(&mut g, METHOD_GETLPARENLOC_1, "clang::CXXFoldExpr::getLParenLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_5, "clang::CXXFoldExpr::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETELLIPSISLOC_4, "clang::CXXFoldExpr::getEllipsisLoc"),
      add_named_node(&mut g, METHOD_GETOPERATOR, "clang::CXXFoldExpr::getOperator"),
      add_named_node(&mut g, METHOD_GETNUMEXPANSIONS_1, "clang::CXXFoldExpr::getNumExpansions"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_27, "clang::CXXFoldExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_26, "clang::CXXFoldExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_21, "clang::CXXFoldExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXFOLDEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MATERIALIZETEMPORARYEXPR, "clang::MaterializeTemporaryExpr");
  g.add_edge((CLASS_MATERIALIZETEMPORARYEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MATERIALIZETEMPORARYEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSUBEXPR_7, "clang::MaterializeTemporaryExpr::getSubExpr"),
      add_named_node(&mut g, METHOD_GETSTORAGEDURATION_2, "clang::MaterializeTemporaryExpr::getStorageDuration"),
      add_named_node(&mut g, METHOD_GETLIFETIMEEXTENDEDTEMPORARYDECL, "clang::MaterializeTemporaryExpr::getLifetimeExtendedTemporaryDecl"),
      add_named_node(&mut g, METHOD_GETEXTENDINGDECL_1, "clang::MaterializeTemporaryExpr::getExtendingDecl"),
      add_named_node(&mut g, METHOD_GETMANGLINGNUMBER_1, "clang::MaterializeTemporaryExpr::getManglingNumber"),
      add_named_node(&mut g, METHOD_ISBOUNDTOLVALUEREFERENCE, "clang::MaterializeTemporaryExpr::isBoundToLvalueReference"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_95, "clang::MaterializeTemporaryExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_94, "clang::MaterializeTemporaryExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_83, "clang::MaterializeTemporaryExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MATERIALIZETEMPORARYEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXDEPENDENTSCOPEMEMBEREXPR, "clang::CXXDependentScopeMemberExpr");
  g.add_edge((CLASS_CXXDEPENDENTSCOPEMEMBEREXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXDEPENDENTSCOPEMEMBEREXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_ISIMPLICITACCESS, "clang::CXXDependentScopeMemberExpr::isImplicitAccess"),
      add_named_node(&mut g, METHOD_GETBASE_7, "clang::CXXDependentScopeMemberExpr::getBase"),
      add_named_node(&mut g, METHOD_GETBASETYPE_1, "clang::CXXDependentScopeMemberExpr::getBaseType"),
      add_named_node(&mut g, METHOD_ISARROW, "clang::CXXDependentScopeMemberExpr::isArrow"),
      add_named_node(&mut g, METHOD_GETOPERATORLOC_1, "clang::CXXDependentScopeMemberExpr::getOperatorLoc"),
      add_named_node(&mut g, METHOD_GETQUALIFIER_11, "clang::CXXDependentScopeMemberExpr::getQualifier"),
      add_named_node(&mut g, METHOD_GETQUALIFIERLOC_8, "clang::CXXDependentScopeMemberExpr::getQualifierLoc"),
      add_named_node(&mut g, METHOD_GETFIRSTQUALIFIERFOUNDINSCOPE, "clang::CXXDependentScopeMemberExpr::getFirstQualifierFoundInScope"),
      add_named_node(&mut g, METHOD_GETMEMBERNAMEINFO, "clang::CXXDependentScopeMemberExpr::getMemberNameInfo"),
      add_named_node(&mut g, METHOD_GETMEMBER, "clang::CXXDependentScopeMemberExpr::getMember"),
      add_named_node(&mut g, METHOD_GETMEMBERLOC, "clang::CXXDependentScopeMemberExpr::getMemberLoc"),
      add_named_node(&mut g, METHOD_GETTEMPLATEKEYWORDLOC_2, "clang::CXXDependentScopeMemberExpr::getTemplateKeywordLoc"),
      add_named_node(&mut g, METHOD_GETLANGLELOC, "clang::CXXDependentScopeMemberExpr::getLAngleLoc"),
      add_named_node(&mut g, METHOD_GETRANGLELOC, "clang::CXXDependentScopeMemberExpr::getRAngleLoc"),
      add_named_node(&mut g, METHOD_HASTEMPLATEKEYWORD, "clang::CXXDependentScopeMemberExpr::hasTemplateKeyword"),
      add_named_node(&mut g, METHOD_HASEXPLICITTEMPLATEARGS, "clang::CXXDependentScopeMemberExpr::hasExplicitTemplateArgs"),
      add_named_node(&mut g, METHOD_GETTEMPLATEARGS_2, "clang::CXXDependentScopeMemberExpr::getTemplateArgs"),
      add_named_node(&mut g, METHOD_GETNUMTEMPLATEARGS, "clang::CXXDependentScopeMemberExpr::getNumTemplateArgs"),
      add_named_node(&mut g, METHOD_TEMPLATE_ARGUMENTS_2, "clang::CXXDependentScopeMemberExpr::template_arguments"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_26, "clang::CXXDependentScopeMemberExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_25, "clang::CXXDependentScopeMemberExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_20, "clang::CXXDependentScopeMemberExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXDEPENDENTSCOPEMEMBEREXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPDEPOBJDIRECTIVE, "clang::OMPDepobjDirective");
  g.add_edge((CLASS_OMPDEPOBJDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPDEPOBJDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_UNRESOLVEDMEMBEREXPR, "clang::UnresolvedMemberExpr");
  g.add_edge((CLASS_UNRESOLVEDMEMBEREXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNRESOLVEDMEMBEREXPR, META_SUBCLASS, CLASS_OVERLOADEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_ISIMPLICITACCESS_3, "clang::UnresolvedMemberExpr::isImplicitAccess"),
      add_named_node(&mut g, METHOD_GETBASE_8, "clang::UnresolvedMemberExpr::getBase"),
      add_named_node(&mut g, METHOD_GETBASETYPE_2, "clang::UnresolvedMemberExpr::getBaseType"),
      add_named_node(&mut g, METHOD_HASUNRESOLVEDUSING, "clang::UnresolvedMemberExpr::hasUnresolvedUsing"),
      add_named_node(&mut g, METHOD_ISARROW_5, "clang::UnresolvedMemberExpr::isArrow"),
      add_named_node(&mut g, METHOD_GETOPERATORLOC_11, "clang::UnresolvedMemberExpr::getOperatorLoc"),
      add_named_node(&mut g, METHOD_GETNAMINGCLASS_2, "clang::UnresolvedMemberExpr::getNamingClass"),
      add_named_node(&mut g, METHOD_GETMEMBERNAMEINFO_2, "clang::UnresolvedMemberExpr::getMemberNameInfo"),
      add_named_node(&mut g, METHOD_GETMEMBERNAME, "clang::UnresolvedMemberExpr::getMemberName"),
      add_named_node(&mut g, METHOD_GETMEMBERLOC_3, "clang::UnresolvedMemberExpr::getMemberLoc"),
      add_named_node(&mut g, METHOD_GETEXPRLOC_13, "clang::UnresolvedMemberExpr::getExprLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_129, "clang::UnresolvedMemberExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_128, "clang::UnresolvedMemberExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_116, "clang::UnresolvedMemberExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNRESOLVEDMEMBEREXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXNEWEXPR, "clang::CXXNewExpr");
  g.add_edge((CLASS_CXXNEWEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXNEWEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETALLOCATEDTYPE, "clang::CXXNewExpr::getAllocatedType"),
      add_named_node(&mut g, METHOD_GETALLOCATEDTYPESOURCEINFO, "clang::CXXNewExpr::getAllocatedTypeSourceInfo"),
      add_named_node(&mut g, METHOD_SHOULDNULLCHECKALLOCATION, "clang::CXXNewExpr::shouldNullCheckAllocation"),
      add_named_node(&mut g, METHOD_GETOPERATORNEW, "clang::CXXNewExpr::getOperatorNew"),
      add_named_node(&mut g, METHOD_GETOPERATORDELETE_2, "clang::CXXNewExpr::getOperatorDelete"),
      add_named_node(&mut g, METHOD_ISARRAY, "clang::CXXNewExpr::isArray"),
      add_named_node(&mut g, METHOD_GETARRAYSIZE_1, "clang::CXXNewExpr::getArraySize"),
      add_named_node(&mut g, METHOD_GETNUMPLACEMENTARGS, "clang::CXXNewExpr::getNumPlacementArgs"),
      add_named_node(&mut g, METHOD_ISPARENTYPEID, "clang::CXXNewExpr::isParenTypeId"),
      add_named_node(&mut g, METHOD_GETTYPEIDPARENS, "clang::CXXNewExpr::getTypeIdParens"),
      add_named_node(&mut g, METHOD_ISGLOBALNEW, "clang::CXXNewExpr::isGlobalNew"),
      add_named_node(&mut g, METHOD_HASINITIALIZER, "clang::CXXNewExpr::hasInitializer"),
      add_named_node(&mut g, METHOD_GETINITIALIZATIONSTYLE, "clang::CXXNewExpr::getInitializationStyle"),
      add_named_node(&mut g, METHOD_GETINITIALIZER, "clang::CXXNewExpr::getInitializer"),
      add_named_node(&mut g, METHOD_GETCONSTRUCTEXPR, "clang::CXXNewExpr::getConstructExpr"),
      add_named_node(&mut g, METHOD_PASSALIGNMENT, "clang::CXXNewExpr::passAlignment"),
      add_named_node(&mut g, METHOD_DOESUSUALARRAYDELETEWANTSIZE_1, "clang::CXXNewExpr::doesUsualArrayDeleteWantSize"),
      add_named_node(&mut g, METHOD_PLACEMENT_ARGUMENTS, "clang::CXXNewExpr::placement_arguments"),
      add_named_node(&mut g, METHOD_PLACEMENT_ARG_BEGIN, "clang::CXXNewExpr::placement_arg_begin"),
      add_named_node(&mut g, METHOD_PLACEMENT_ARG_END, "clang::CXXNewExpr::placement_arg_end"),
      add_named_node(&mut g, METHOD_RAW_ARG_BEGIN, "clang::CXXNewExpr::raw_arg_begin"),
      add_named_node(&mut g, METHOD_RAW_ARG_END, "clang::CXXNewExpr::raw_arg_end"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_32, "clang::CXXNewExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_31, "clang::CXXNewExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETDIRECTINITRANGE, "clang::CXXNewExpr::getDirectInitRange"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_39, "clang::CXXNewExpr::getSourceRange"),
      add_named_node(&mut g, METHOD_CHILDREN_24, "clang::CXXNewExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXNEWEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_IMPLICITCASTEXPR, "clang::ImplicitCastExpr");
  g.add_edge((CLASS_IMPLICITCASTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_IMPLICITCASTEXPR, META_SUBCLASS, CLASS_CASTEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_ISPARTOFEXPLICITCAST, "clang::ImplicitCastExpr::isPartOfExplicitCast"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_84, "clang::ImplicitCastExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_83, "clang::ImplicitCastExpr::getEndLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_IMPLICITCASTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXCONSTRUCTEXPR, "clang::CXXConstructExpr");
  g.add_edge((CLASS_CXXCONSTRUCTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXCONSTRUCTEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETCONSTRUCTOR, "clang::CXXConstructExpr::getConstructor"),
      add_named_node(&mut g, METHOD_GETLOCATION_2, "clang::CXXConstructExpr::getLocation"),
      add_named_node(&mut g, METHOD_ISELIDABLE, "clang::CXXConstructExpr::isElidable"),
      add_named_node(&mut g, METHOD_HADMULTIPLECANDIDATES, "clang::CXXConstructExpr::hadMultipleCandidates"),
      add_named_node(&mut g, METHOD_ISLISTINITIALIZATION, "clang::CXXConstructExpr::isListInitialization"),
      add_named_node(&mut g, METHOD_ISSTDINITLISTINITIALIZATION, "clang::CXXConstructExpr::isStdInitListInitialization"),
      add_named_node(&mut g, METHOD_REQUIRESZEROINITIALIZATION, "clang::CXXConstructExpr::requiresZeroInitialization"),
      add_named_node(&mut g, METHOD_GETCONSTRUCTIONKIND, "clang::CXXConstructExpr::getConstructionKind"),
      add_named_node(&mut g, METHOD_ARGUMENTS, "clang::CXXConstructExpr::arguments"),
      add_named_node(&mut g, METHOD_ARG_BEGIN_1, "clang::CXXConstructExpr::arg_begin"),
      add_named_node(&mut g, METHOD_ARG_END_1, "clang::CXXConstructExpr::arg_end"),
      add_named_node(&mut g, METHOD_GETARGS, "clang::CXXConstructExpr::getArgs"),
      add_named_node(&mut g, METHOD_GETNUMARGS_1, "clang::CXXConstructExpr::getNumArgs"),
      add_named_node(&mut g, METHOD_ISIMMEDIATEESCALATING_1, "clang::CXXConstructExpr::isImmediateEscalating"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_22, "clang::CXXConstructExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_21, "clang::CXXConstructExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETPARENORBRACERANGE, "clang::CXXConstructExpr::getParenOrBraceRange"),
      add_named_node(&mut g, METHOD_CHILDREN_16, "clang::CXXConstructExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXCONSTRUCTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_COMPOUNDLITERALEXPR, "clang::CompoundLiteralExpr");
  g.add_edge((CLASS_COMPOUNDLITERALEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_COMPOUNDLITERALEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETINITIALIZER_1, "clang::CompoundLiteralExpr::getInitializer"),
      add_named_node(&mut g, METHOD_ISFILESCOPE, "clang::CompoundLiteralExpr::isFileScope"),
      add_named_node(&mut g, METHOD_GETLPARENLOC_4, "clang::CompoundLiteralExpr::getLParenLoc"),
      add_named_node(&mut g, METHOD_GETTYPESOURCEINFO_5, "clang::CompoundLiteralExpr::getTypeSourceInfo"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_53, "clang::CompoundLiteralExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_52, "clang::CompoundLiteralExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_43, "clang::CompoundLiteralExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_COMPOUNDLITERALEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXBINDTEMPORARYEXPR, "clang::CXXBindTemporaryExpr");
  g.add_edge((CLASS_CXXBINDTEMPORARYEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXBINDTEMPORARYEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTEMPORARY, "clang::CXXBindTemporaryExpr::getTemporary"),
      add_named_node(&mut g, METHOD_GETSUBEXPR_1, "clang::CXXBindTemporaryExpr::getSubExpr"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_19, "clang::CXXBindTemporaryExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_18, "clang::CXXBindTemporaryExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_13, "clang::CXXBindTemporaryExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXBINDTEMPORARYEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXDEFAULTARGEXPR, "clang::CXXDefaultArgExpr");
  g.add_edge((CLASS_CXXDEFAULTARGEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXDEFAULTARGEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETPARAM, "clang::CXXDefaultArgExpr::getParam"),
      add_named_node(&mut g, METHOD_HASREWRITTENINIT, "clang::CXXDefaultArgExpr::hasRewrittenInit"),
      add_named_node(&mut g, METHOD_GETEXPR, "clang::CXXDefaultArgExpr::getExpr"),
      add_named_node(&mut g, METHOD_GETREWRITTENEXPR, "clang::CXXDefaultArgExpr::getRewrittenExpr"),
      add_named_node(&mut g, METHOD_GETADJUSTEDREWRITTENEXPR, "clang::CXXDefaultArgExpr::getAdjustedRewrittenExpr"),
      add_named_node(&mut g, METHOD_GETUSEDCONTEXT, "clang::CXXDefaultArgExpr::getUsedContext"),
      add_named_node(&mut g, METHOD_GETUSEDLOCATION, "clang::CXXDefaultArgExpr::getUsedLocation"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_23, "clang::CXXDefaultArgExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_22, "clang::CXXDefaultArgExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETEXPRLOC_2, "clang::CXXDefaultArgExpr::getExprLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_17, "clang::CXXDefaultArgExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXDEFAULTARGEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPSECTIONDIRECTIVE, "clang::OMPSectionDirective");
  g.add_edge((CLASS_OMPSECTIONDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPSECTIONDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_CXXTHROWEXPR, "clang::CXXThrowExpr");
  g.add_edge((CLASS_CXXTHROWEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXTHROWEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSUBEXPR_3, "clang::CXXThrowExpr::getSubExpr"),
      add_named_node(&mut g, METHOD_GETTHROWLOC, "clang::CXXThrowExpr::getThrowLoc"),
      add_named_node(&mut g, METHOD_ISTHROWNVARIABLEINSCOPE, "clang::CXXThrowExpr::isThrownVariableInScope"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_43, "clang::CXXThrowExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_42, "clang::CXXThrowExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_32, "clang::CXXThrowExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXTHROWEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MSPROPERTYSUBSCRIPTEXPR, "clang::MSPropertySubscriptExpr");
  g.add_edge((CLASS_MSPROPERTYSUBSCRIPTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MSPROPERTYSUBSCRIPTEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETBASE_4, "clang::MSPropertySubscriptExpr::getBase"),
      add_named_node(&mut g, METHOD_GETIDX_1, "clang::MSPropertySubscriptExpr::getIdx"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_94, "clang::MSPropertySubscriptExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_93, "clang::MSPropertySubscriptExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETRBRACKETLOC_3, "clang::MSPropertySubscriptExpr::getRBracketLoc"),
      add_named_node(&mut g, METHOD_GETEXPRLOC_7, "clang::MSPropertySubscriptExpr::getExprLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_82, "clang::MSPropertySubscriptExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MSPROPERTYSUBSCRIPTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXREWRITTENBINARYOPERATOR, "clang::CXXRewrittenBinaryOperator");
  g.add_edge((CLASS_CXXREWRITTENBINARYOPERATOR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXREWRITTENBINARYOPERATOR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSEMANTICFORM, "clang::CXXRewrittenBinaryOperator::getSemanticForm"),
      add_named_node(&mut g, METHOD_GETDECOMPOSEDFORM, "clang::CXXRewrittenBinaryOperator::getDecomposedForm"),
      add_named_node(&mut g, METHOD_ISREVERSED, "clang::CXXRewrittenBinaryOperator::isReversed"),
      add_named_node(&mut g, METHOD_GETOPERATOR_2, "clang::CXXRewrittenBinaryOperator::getOperator"),
      add_named_node(&mut g, METHOD_GETOPCODE_1, "clang::CXXRewrittenBinaryOperator::getOpcode"),
      add_named_node(&mut g, METHOD_GETOPCODESTR_1, "clang::CXXRewrittenBinaryOperator::getOpcodeStr"),
      add_named_node(&mut g, METHOD_ISCOMPARISONOP_2, "clang::CXXRewrittenBinaryOperator::isComparisonOp"),
      add_named_node(&mut g, METHOD_ISASSIGNMENTOP_2, "clang::CXXRewrittenBinaryOperator::isAssignmentOp"),
      add_named_node(&mut g, METHOD_GETLHS_3, "clang::CXXRewrittenBinaryOperator::getLHS"),
      add_named_node(&mut g, METHOD_GETRHS_3, "clang::CXXRewrittenBinaryOperator::getRHS"),
      add_named_node(&mut g, METHOD_GETOPERATORLOC_5, "clang::CXXRewrittenBinaryOperator::getOperatorLoc"),
      add_named_node(&mut g, METHOD_GETEXPRLOC_5, "clang::CXXRewrittenBinaryOperator::getExprLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_38, "clang::CXXRewrittenBinaryOperator::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_37, "clang::CXXRewrittenBinaryOperator::getEndLoc"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_43, "clang::CXXRewrittenBinaryOperator::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXREWRITTENBINARYOPERATOR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CUDAKERNELCALLEXPR, "clang::CUDAKernelCallExpr");
  g.add_edge((CLASS_CUDAKERNELCALLEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CUDAKERNELCALLEXPR, META_SUBCLASS, CLASS_CALLEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETCONFIG, "clang::CUDAKernelCallExpr::getConfig"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CUDAKERNELCALLEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPOEXPR, "clang::TypoExpr");
  g.add_edge((CLASS_TYPOEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPOEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_CHILDREN_112, "clang::TypoExpr::children"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_125, "clang::TypoExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_124, "clang::TypoExpr::getEndLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPOEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ASTYPEEXPR, "clang::AsTypeExpr");
  g.add_edge((CLASS_ASTYPEEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ASTYPEEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSRCEXPR, "clang::AsTypeExpr::getSrcExpr"),
      add_named_node(&mut g, METHOD_GETBUILTINLOC, "clang::AsTypeExpr::getBuiltinLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_2, "clang::AsTypeExpr::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_9, "clang::AsTypeExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_8, "clang::AsTypeExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_5, "clang::AsTypeExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ASTYPEEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_EXTVECTORELEMENTEXPR, "clang::ExtVectorElementExpr");
  g.add_edge((CLASS_EXTVECTORELEMENTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_EXTVECTORELEMENTEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETBASE_3, "clang::ExtVectorElementExpr::getBase"),
      add_named_node(&mut g, METHOD_GETACCESSOR, "clang::ExtVectorElementExpr::getAccessor"),
      add_named_node(&mut g, METHOD_GETACCESSORLOC, "clang::ExtVectorElementExpr::getAccessorLoc"),
      add_named_node(&mut g, METHOD_GETNUMELEMENTS_1, "clang::ExtVectorElementExpr::getNumElements"),
      add_named_node(&mut g, METHOD_CONTAINSDUPLICATEELEMENTS, "clang::ExtVectorElementExpr::containsDuplicateElements"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_73, "clang::ExtVectorElementExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_72, "clang::ExtVectorElementExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_ISARROW_2, "clang::ExtVectorElementExpr::isArrow"),
      add_named_node(&mut g, METHOD_CHILDREN_63, "clang::ExtVectorElementExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_EXTVECTORELEMENTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PARENLISTEXPR, "clang::ParenListExpr");
  g.add_edge((CLASS_PARENLISTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PARENLISTEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETNUMEXPRS, "clang::ParenListExpr::getNumExprs"),
      add_named_node(&mut g, METHOD_GETLPARENLOC_7, "clang::ParenListExpr::getLParenLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_20, "clang::ParenListExpr::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_104, "clang::ParenListExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_103, "clang::ParenListExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_92, "clang::ParenListExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PARENLISTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NOINITEXPR, "clang::NoInitExpr");
  g.add_edge((CLASS_NOINITEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOINITEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETBEGINLOC_98, "clang::NoInitExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_97, "clang::NoInitExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_86, "clang::NoInitExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NOINITEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ARRAYINITINDEXEXPR, "clang::ArrayInitIndexExpr");
  g.add_edge((CLASS_ARRAYINITINDEXEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARRAYINITINDEXEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETBEGINLOC_5, "clang::ArrayInitIndexExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_4, "clang::ArrayInitIndexExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_1, "clang::ArrayInitIndexExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ARRAYINITINDEXEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CONVERTVECTOREXPR, "clang::ConvertVectorExpr");
  g.add_edge((CLASS_CONVERTVECTOREXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONVERTVECTOREXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSRCEXPR_1, "clang::ConvertVectorExpr::getSrcExpr"),
      add_named_node(&mut g, METHOD_GETTYPESOURCEINFO_6, "clang::ConvertVectorExpr::getTypeSourceInfo"),
      add_named_node(&mut g, METHOD_GETBUILTINLOC_3, "clang::ConvertVectorExpr::getBuiltinLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_13, "clang::ConvertVectorExpr::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_59, "clang::ConvertVectorExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_58, "clang::ConvertVectorExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_49, "clang::ConvertVectorExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CONVERTVECTOREXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CONDITIONALOPERATOR, "clang::ConditionalOperator");
  g.add_edge((CLASS_CONDITIONALOPERATOR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONDITIONALOPERATOR, META_SUBCLASS, CLASS_ABSTRACTCONDITIONALOPERATOR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETCOND_4, "clang::ConditionalOperator::getCond"),
      add_named_node(&mut g, METHOD_GETTRUEEXPR_2, "clang::ConditionalOperator::getTrueExpr"),
      add_named_node(&mut g, METHOD_GETFALSEEXPR_2, "clang::ConditionalOperator::getFalseExpr"),
      add_named_node(&mut g, METHOD_GETLHS_6, "clang::ConditionalOperator::getLHS"),
      add_named_node(&mut g, METHOD_GETRHS_6, "clang::ConditionalOperator::getRHS"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_56, "clang::ConditionalOperator::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_55, "clang::ConditionalOperator::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_46, "clang::ConditionalOperator::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CONDITIONALOPERATOR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BINARYOPERATOR, "clang::BinaryOperator");
  g.add_edge((CLASS_BINARYOPERATOR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BINARYOPERATOR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETEXPRLOC_1, "clang::BinaryOperator::getExprLoc"),
      add_named_node(&mut g, METHOD_GETOPERATORLOC, "clang::BinaryOperator::getOperatorLoc"),
      add_named_node(&mut g, METHOD_GETOPCODE, "clang::BinaryOperator::getOpcode"),
      add_named_node(&mut g, METHOD_GETLHS_1, "clang::BinaryOperator::getLHS"),
      add_named_node(&mut g, METHOD_GETRHS_1, "clang::BinaryOperator::getRHS"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_14, "clang::BinaryOperator::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_13, "clang::BinaryOperator::getEndLoc"),
      add_named_node(&mut g, METHOD_GETOPCODESTR, "clang::BinaryOperator::getOpcodeStr"),
      add_named_node(&mut g, METHOD_ISPTRMEMOP, "clang::BinaryOperator::isPtrMemOp"),
      add_named_node(&mut g, METHOD_ISMULTIPLICATIVEOP, "clang::BinaryOperator::isMultiplicativeOp"),
      add_named_node(&mut g, METHOD_ISADDITIVEOP, "clang::BinaryOperator::isAdditiveOp"),
      add_named_node(&mut g, METHOD_ISSHIFTOP, "clang::BinaryOperator::isShiftOp"),
      add_named_node(&mut g, METHOD_ISBITWISEOP, "clang::BinaryOperator::isBitwiseOp"),
      add_named_node(&mut g, METHOD_ISRELATIONALOP, "clang::BinaryOperator::isRelationalOp"),
      add_named_node(&mut g, METHOD_ISEQUALITYOP, "clang::BinaryOperator::isEqualityOp"),
      add_named_node(&mut g, METHOD_ISCOMPARISONOP, "clang::BinaryOperator::isComparisonOp"),
      add_named_node(&mut g, METHOD_ISCOMMAOP, "clang::BinaryOperator::isCommaOp"),
      add_named_node(&mut g, METHOD_ISLOGICALOP, "clang::BinaryOperator::isLogicalOp"),
      add_named_node(&mut g, METHOD_ISASSIGNMENTOP, "clang::BinaryOperator::isAssignmentOp"),
      add_named_node(&mut g, METHOD_ISCOMPOUNDASSIGNMENTOP, "clang::BinaryOperator::isCompoundAssignmentOp"),
      add_named_node(&mut g, METHOD_ISSHIFTASSIGNOP, "clang::BinaryOperator::isShiftAssignOp"),
      add_named_node(&mut g, METHOD_CHILDREN_10, "clang::BinaryOperator::children"),
      add_named_node(&mut g, METHOD_HASSTOREDFPFEATURES, "clang::BinaryOperator::hasStoredFPFeatures"),
      add_named_node(&mut g, METHOD_GETSTOREDFPFEATURES, "clang::BinaryOperator::getStoredFPFeatures"),
      add_named_node(&mut g, METHOD_GETFPFEATURES, "clang::BinaryOperator::getFPFeatures"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BINARYOPERATOR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXFUNCTIONALCASTEXPR, "clang::CXXFunctionalCastExpr");
  g.add_edge((CLASS_CXXFUNCTIONALCASTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXFUNCTIONALCASTEXPR, META_SUBCLASS, CLASS_EXPLICITCASTEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETLPARENLOC_2, "clang::CXXFunctionalCastExpr::getLParenLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_7, "clang::CXXFunctionalCastExpr::getRParenLoc"),
      add_named_node(&mut g, METHOD_ISLISTINITIALIZATION_1, "clang::CXXFunctionalCastExpr::isListInitialization"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_29, "clang::CXXFunctionalCastExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_28, "clang::CXXFunctionalCastExpr::getEndLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXFUNCTIONALCASTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_UNARYOPERATOR, "clang::UnaryOperator");
  g.add_edge((CLASS_UNARYOPERATOR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNARYOPERATOR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETOPCODE_2, "clang::UnaryOperator::getOpcode"),
      add_named_node(&mut g, METHOD_GETSUBEXPR_9, "clang::UnaryOperator::getSubExpr"),
      add_named_node(&mut g, METHOD_GETOPERATORLOC_10, "clang::UnaryOperator::getOperatorLoc"),
      add_named_node(&mut g, METHOD_CANOVERFLOW, "clang::UnaryOperator::canOverflow"),
      add_named_node(&mut g, METHOD_ISPREFIX, "clang::UnaryOperator::isPrefix"),
      add_named_node(&mut g, METHOD_ISPOSTFIX, "clang::UnaryOperator::isPostfix"),
      add_named_node(&mut g, METHOD_ISINCREMENTOP, "clang::UnaryOperator::isIncrementOp"),
      add_named_node(&mut g, METHOD_ISDECREMENTOP, "clang::UnaryOperator::isDecrementOp"),
      add_named_node(&mut g, METHOD_ISINCREMENTDECREMENTOP, "clang::UnaryOperator::isIncrementDecrementOp"),
      add_named_node(&mut g, METHOD_ISARITHMETICOP, "clang::UnaryOperator::isArithmeticOp"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_127, "clang::UnaryOperator::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_126, "clang::UnaryOperator::getEndLoc"),
      add_named_node(&mut g, METHOD_GETEXPRLOC_12, "clang::UnaryOperator::getExprLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_114, "clang::UnaryOperator::children"),
      add_named_node(&mut g, METHOD_HASSTOREDFPFEATURES_4, "clang::UnaryOperator::hasStoredFPFeatures"),
      add_named_node(&mut g, METHOD_GETSTOREDFPFEATURES_4, "clang::UnaryOperator::getStoredFPFeatures"),
      add_named_node(&mut g, METHOD_GETFPOPTIONSOVERRIDE, "clang::UnaryOperator::getFPOptionsOverride"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNARYOPERATOR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCDICTIONARYLITERAL, "clang::ObjCDictionaryLiteral");
  g.add_edge((CLASS_OBJCDICTIONARYLITERAL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCDICTIONARYLITERAL, META_SUBCLASS, CLASS_EXPR));

  g.add_named_node(CLASS_ATTRIBUTEDSTMT, "clang::AttributedStmt");
  g.add_edge((CLASS_ATTRIBUTEDSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ATTRIBUTEDSTMT, META_SUBCLASS, CLASS_VALUESTMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETATTRLOC, "clang::AttributedStmt::getAttrLoc"),
      add_named_node(&mut g, METHOD_GETATTRS_1, "clang::AttributedStmt::getAttrs"),
      add_named_node(&mut g, METHOD_GETSUBSTMT, "clang::AttributedStmt::getSubStmt"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_12, "clang::AttributedStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_11, "clang::AttributedStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_8, "clang::AttributedStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ATTRIBUTEDSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXSTATICCASTEXPR, "clang::CXXStaticCastExpr");
  g.add_edge((CLASS_CXXSTATICCASTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXSTATICCASTEXPR, META_SUBCLASS, CLASS_CXXNAMEDCASTEXPR));

  g.add_named_node(CLASS_WHILESTMT, "clang::WhileStmt");
  g.add_edge((CLASS_WHILESTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_WHILESTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_HASVARSTORAGE_2, "clang::WhileStmt::hasVarStorage"),
      add_named_node(&mut g, METHOD_GETCOND_9, "clang::WhileStmt::getCond"),
      add_named_node(&mut g, METHOD_GETBODY_12, "clang::WhileStmt::getBody"),
      add_named_node(&mut g, METHOD_GETCONDITIONVARIABLE_3, "clang::WhileStmt::getConditionVariable"),
      add_named_node(&mut g, METHOD_GETCONDITIONVARIABLEDECLSTMT_3, "clang::WhileStmt::getConditionVariableDeclStmt"),
      add_named_node(&mut g, METHOD_GETWHILELOC_1, "clang::WhileStmt::getWhileLoc"),
      add_named_node(&mut g, METHOD_GETLPARENLOC_11, "clang::WhileStmt::getLParenLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_28, "clang::WhileStmt::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_132, "clang::WhileStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_131, "clang::WhileStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_118, "clang::WhileStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_WHILESTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXSCALARVALUEINITEXPR, "clang::CXXScalarValueInitExpr");
  g.add_edge((CLASS_CXXSCALARVALUEINITEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXSCALARVALUEINITEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTYPESOURCEINFO_2, "clang::CXXScalarValueInitExpr::getTypeSourceInfo"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_9, "clang::CXXScalarValueInitExpr::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_39, "clang::CXXScalarValueInitExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_38, "clang::CXXScalarValueInitExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_29, "clang::CXXScalarValueInitExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXSCALARVALUEINITEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CSTYLECASTEXPR, "clang::CStyleCastExpr");
  g.add_edge((CLASS_CSTYLECASTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CSTYLECASTEXPR, META_SUBCLASS, CLASS_EXPLICITCASTEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETLPARENLOC, "clang::CStyleCastExpr::getLParenLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_4, "clang::CStyleCastExpr::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_18, "clang::CStyleCastExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_17, "clang::CStyleCastExpr::getEndLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CSTYLECASTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_INTEGERLITERAL, "clang::IntegerLiteral");
  g.add_edge((CLASS_INTEGERLITERAL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_INTEGERLITERAL, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETBEGINLOC_88, "clang::IntegerLiteral::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_87, "clang::IntegerLiteral::getEndLoc"),
      add_named_node(&mut g, METHOD_GETLOCATION_11, "clang::IntegerLiteral::getLocation"),
      add_named_node(&mut g, METHOD_CHILDREN_76, "clang::IntegerLiteral::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_INTEGERLITERAL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXCONSTCASTEXPR, "clang::CXXConstCastExpr");
  g.add_edge((CLASS_CXXCONSTCASTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXCONSTCASTEXPR, META_SUBCLASS, CLASS_CXXNAMEDCASTEXPR));

  g.add_named_node(CLASS_CALLEXPR, "clang::CallExpr");
  g.add_edge((CLASS_CALLEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CALLEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETCALLEE_1, "clang::CallExpr::getCallee"),
      add_named_node(&mut g, METHOD_GETADLCALLKIND, "clang::CallExpr::getADLCallKind"),
      add_named_node(&mut g, METHOD_USESADL, "clang::CallExpr::usesADL"),
      add_named_node(&mut g, METHOD_HASSTOREDFPFEATURES_1, "clang::CallExpr::hasStoredFPFeatures"),
      add_named_node(&mut g, METHOD_GETCALLEEDECL, "clang::CallExpr::getCalleeDecl"),
      add_named_node(&mut g, METHOD_GETDIRECTCALLEE, "clang::CallExpr::getDirectCallee"),
      add_named_node(&mut g, METHOD_GETNUMARGS_3, "clang::CallExpr::getNumArgs"),
      add_named_node(&mut g, METHOD_GETARGS_1, "clang::CallExpr::getArgs"),
      add_named_node(&mut g, METHOD_ARGUMENTS_2, "clang::CallExpr::arguments"),
      add_named_node(&mut g, METHOD_ARG_BEGIN_2, "clang::CallExpr::arg_begin"),
      add_named_node(&mut g, METHOD_ARG_END_2, "clang::CallExpr::arg_end"),
      add_named_node(&mut g, METHOD_GETSTOREDFPFEATURES_1, "clang::CallExpr::getStoredFPFeatures"),
      add_named_node(&mut g, METHOD_GETFPFEATURES_1, "clang::CallExpr::getFPFeatures"),
      add_named_node(&mut g, METHOD_GETBUILTINCALLEE, "clang::CallExpr::getBuiltinCallee"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_11, "clang::CallExpr::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_48, "clang::CallExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_47, "clang::CallExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_ISCALLTOSTDMOVE, "clang::CallExpr::isCallToStdMove"),
      add_named_node(&mut g, METHOD_CHILDREN_37, "clang::CallExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CALLEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PREDEFINEDEXPR, "clang::PredefinedExpr");
  g.add_edge((CLASS_PREDEFINEDEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PREDEFINEDEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETIDENTKIND, "clang::PredefinedExpr::getIdentKind"),
      add_named_node(&mut g, METHOD_ISTRANSPARENT_1, "clang::PredefinedExpr::isTransparent"),
      add_named_node(&mut g, METHOD_GETLOCATION_13, "clang::PredefinedExpr::getLocation"),
      add_named_node(&mut g, METHOD_GETFUNCTIONNAME, "clang::PredefinedExpr::getFunctionName"),
      add_named_node(&mut g, METHOD_GETIDENTKINDNAME, "clang::PredefinedExpr::getIdentKindName"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_105, "clang::PredefinedExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_104, "clang::PredefinedExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_93, "clang::PredefinedExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PREDEFINEDEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_STRINGLITERAL, "clang::StringLiteral");
  g.add_edge((CLASS_STRINGLITERAL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_STRINGLITERAL, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSTRING, "clang::StringLiteral::getString"),
      add_named_node(&mut g, METHOD_GETBYTES, "clang::StringLiteral::getBytes"),
      add_named_node(&mut g, METHOD_GETBYTELENGTH, "clang::StringLiteral::getByteLength"),
      add_named_node(&mut g, METHOD_GETLENGTH, "clang::StringLiteral::getLength"),
      add_named_node(&mut g, METHOD_GETCHARBYTEWIDTH, "clang::StringLiteral::getCharByteWidth"),
      add_named_node(&mut g, METHOD_GETKIND_4, "clang::StringLiteral::getKind"),
      add_named_node(&mut g, METHOD_ISORDINARY, "clang::StringLiteral::isOrdinary"),
      add_named_node(&mut g, METHOD_ISWIDE, "clang::StringLiteral::isWide"),
      add_named_node(&mut g, METHOD_ISUTF8, "clang::StringLiteral::isUTF8"),
      add_named_node(&mut g, METHOD_ISUTF16, "clang::StringLiteral::isUTF16"),
      add_named_node(&mut g, METHOD_ISUTF32, "clang::StringLiteral::isUTF32"),
      add_named_node(&mut g, METHOD_ISUNEVALUATED, "clang::StringLiteral::isUnevaluated"),
      add_named_node(&mut g, METHOD_ISPASCAL, "clang::StringLiteral::isPascal"),
      add_named_node(&mut g, METHOD_CONTAINSNONASCII, "clang::StringLiteral::containsNonAscii"),
      add_named_node(&mut g, METHOD_CONTAINSNONASCIIORNULL, "clang::StringLiteral::containsNonAsciiOrNull"),
      add_named_node(&mut g, METHOD_GETNUMCONCATENATED, "clang::StringLiteral::getNumConcatenated"),
      add_named_node(&mut g, METHOD_TOKLOC_BEGIN, "clang::StringLiteral::tokloc_begin"),
      add_named_node(&mut g, METHOD_TOKLOC_END, "clang::StringLiteral::tokloc_end"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_119, "clang::StringLiteral::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_118, "clang::StringLiteral::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_107, "clang::StringLiteral::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_STRINGLITERAL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FLOATINGLITERAL, "clang::FloatingLiteral");
  g.add_edge((CLASS_FLOATINGLITERAL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FLOATINGLITERAL, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETVALUE_8, "clang::FloatingLiteral::getValue"),
      add_named_node(&mut g, METHOD_GETRAWSEMANTICS, "clang::FloatingLiteral::getRawSemantics"),
      add_named_node(&mut g, METHOD_GETSEMANTICS, "clang::FloatingLiteral::getSemantics"),
      add_named_node(&mut g, METHOD_ISEXACT, "clang::FloatingLiteral::isExact"),
      add_named_node(&mut g, METHOD_GETVALUEASAPPROXIMATEDOUBLE, "clang::FloatingLiteral::getValueAsApproximateDouble"),
      add_named_node(&mut g, METHOD_GETLOCATION_10, "clang::FloatingLiteral::getLocation"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_75, "clang::FloatingLiteral::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_74, "clang::FloatingLiteral::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_65, "clang::FloatingLiteral::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FLOATINGLITERAL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OPAQUEVALUEEXPR, "clang::OpaqueValueExpr");
  g.add_edge((CLASS_OPAQUEVALUEEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OPAQUEVALUEEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETLOCATION_12, "clang::OpaqueValueExpr::getLocation"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_101, "clang::OpaqueValueExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_100, "clang::OpaqueValueExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETEXPRLOC_10, "clang::OpaqueValueExpr::getExprLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_89, "clang::OpaqueValueExpr::children"),
      add_named_node(&mut g, METHOD_GETSOURCEEXPR, "clang::OpaqueValueExpr::getSourceExpr"),
      add_named_node(&mut g, METHOD_ISUNIQUE, "clang::OpaqueValueExpr::isUnique"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OPAQUEVALUEEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DESIGNATEDINITUPDATEEXPR, "clang::DesignatedInitUpdateExpr");
  g.add_edge((CLASS_DESIGNATEDINITUPDATEEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DESIGNATEDINITUPDATEEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETBEGINLOC_69, "clang::DesignatedInitUpdateExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_68, "clang::DesignatedInitUpdateExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETBASE_2, "clang::DesignatedInitUpdateExpr::getBase"),
      add_named_node(&mut g, METHOD_GETUPDATER, "clang::DesignatedInitUpdateExpr::getUpdater"),
      add_named_node(&mut g, METHOD_CHILDREN_59, "clang::DesignatedInitUpdateExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DESIGNATEDINITUPDATEEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FULLEXPR, "clang::FullExpr");
  g.add_edge((CLASS_FULLEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FULLEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSUBEXPR_5, "clang::FullExpr::getSubExpr"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FULLEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_EXPRWITHCLEANUPS, "clang::ExprWithCleanups");
  g.add_edge((CLASS_EXPRWITHCLEANUPS, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_EXPRWITHCLEANUPS, META_SUBCLASS, CLASS_FULLEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETOBJECTS, "clang::ExprWithCleanups::getObjects"),
      add_named_node(&mut g, METHOD_GETNUMOBJECTS, "clang::ExprWithCleanups::getNumObjects"),
      add_named_node(&mut g, METHOD_CLEANUPSHAVESIDEEFFECTS, "clang::ExprWithCleanups::cleanupsHaveSideEffects"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_71, "clang::ExprWithCleanups::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_70, "clang::ExprWithCleanups::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_61, "clang::ExprWithCleanups::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_EXPRWITHCLEANUPS, META_METHOD, methods));
  }

  g.add_named_node(CLASS_LABELSTMT, "clang::LabelStmt");
  g.add_edge((CLASS_LABELSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LABELSTMT, META_SUBCLASS, CLASS_VALUESTMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETIDENTLOC, "clang::LabelStmt::getIdentLoc"),
      add_named_node(&mut g, METHOD_GETDECL_8, "clang::LabelStmt::getDecl"),
      add_named_node(&mut g, METHOD_GETNAME_1, "clang::LabelStmt::getName"),
      add_named_node(&mut g, METHOD_GETSUBSTMT_3, "clang::LabelStmt::getSubStmt"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_89, "clang::LabelStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_88, "clang::LabelStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_77, "clang::LabelStmt::children"),
      add_named_node(&mut g, METHOD_ISSIDEENTRY, "clang::LabelStmt::isSideEntry"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_LABELSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXRECORDDECL, "clang::CXXRecordDecl");
  g.add_edge((CLASS_CXXRECORDDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXRECORDDECL, META_SUBCLASS, CLASS_RECORDDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETCANONICALDECL_4, "clang::CXXRecordDecl::getCanonicalDecl"),
      add_named_node(&mut g, METHOD_GETPREVIOUSDECL, "clang::CXXRecordDecl::getPreviousDecl"),
      add_named_node(&mut g, METHOD_GETMOSTRECENTDECL_1, "clang::CXXRecordDecl::getMostRecentDecl"),
      add_named_node(&mut g, METHOD_GETMOSTRECENTNONINJECTEDDECL, "clang::CXXRecordDecl::getMostRecentNonInjectedDecl"),
      add_named_node(&mut g, METHOD_GETDEFINITION, "clang::CXXRecordDecl::getDefinition"),
      add_named_node(&mut g, METHOD_HASDEFINITION, "clang::CXXRecordDecl::hasDefinition"),
      add_named_node(&mut g, METHOD_ISDYNAMICCLASS, "clang::CXXRecordDecl::isDynamicClass"),
      add_named_node(&mut g, METHOD_MAYBEDYNAMICCLASS, "clang::CXXRecordDecl::mayBeDynamicClass"),
      add_named_node(&mut g, METHOD_MAYBENONDYNAMICCLASS, "clang::CXXRecordDecl::mayBeNonDynamicClass"),
      add_named_node(&mut g, METHOD_ISPARSINGBASESPECIFIERS, "clang::CXXRecordDecl::isParsingBaseSpecifiers"),
      add_named_node(&mut g, METHOD_GETODRHASH, "clang::CXXRecordDecl::getODRHash"),
      add_named_node(&mut g, METHOD_GETNUMBASES, "clang::CXXRecordDecl::getNumBases"),
      add_named_node(&mut g, METHOD_BASES, "clang::CXXRecordDecl::bases"),
      add_named_node(&mut g, METHOD_BASES_BEGIN, "clang::CXXRecordDecl::bases_begin"),
      add_named_node(&mut g, METHOD_BASES_END, "clang::CXXRecordDecl::bases_end"),
      add_named_node(&mut g, METHOD_GETNUMVBASES, "clang::CXXRecordDecl::getNumVBases"),
      add_named_node(&mut g, METHOD_VBASES, "clang::CXXRecordDecl::vbases"),
      add_named_node(&mut g, METHOD_VBASES_BEGIN, "clang::CXXRecordDecl::vbases_begin"),
      add_named_node(&mut g, METHOD_VBASES_END, "clang::CXXRecordDecl::vbases_end"),
      add_named_node(&mut g, METHOD_HASANYDEPENDENTBASES, "clang::CXXRecordDecl::hasAnyDependentBases"),
      add_named_node(&mut g, METHOD_METHODS, "clang::CXXRecordDecl::methods"),
      add_named_node(&mut g, METHOD_METHOD_BEGIN, "clang::CXXRecordDecl::method_begin"),
      add_named_node(&mut g, METHOD_METHOD_END, "clang::CXXRecordDecl::method_end"),
      add_named_node(&mut g, METHOD_CTORS, "clang::CXXRecordDecl::ctors"),
      add_named_node(&mut g, METHOD_CTOR_BEGIN, "clang::CXXRecordDecl::ctor_begin"),
      add_named_node(&mut g, METHOD_CTOR_END, "clang::CXXRecordDecl::ctor_end"),
      add_named_node(&mut g, METHOD_FRIENDS, "clang::CXXRecordDecl::friends"),
      add_named_node(&mut g, METHOD_FRIEND_BEGIN, "clang::CXXRecordDecl::friend_begin"),
      add_named_node(&mut g, METHOD_FRIEND_END, "clang::CXXRecordDecl::friend_end"),
      add_named_node(&mut g, METHOD_HASFRIENDS, "clang::CXXRecordDecl::hasFriends"),
      add_named_node(&mut g, METHOD_DEFAULTEDCOPYCONSTRUCTORISDELETED, "clang::CXXRecordDecl::defaultedCopyConstructorIsDeleted"),
      add_named_node(&mut g, METHOD_DEFAULTEDMOVECONSTRUCTORISDELETED, "clang::CXXRecordDecl::defaultedMoveConstructorIsDeleted"),
      add_named_node(&mut g, METHOD_DEFAULTEDDESTRUCTORISDELETED, "clang::CXXRecordDecl::defaultedDestructorIsDeleted"),
      add_named_node(&mut g, METHOD_HASSIMPLECOPYCONSTRUCTOR, "clang::CXXRecordDecl::hasSimpleCopyConstructor"),
      add_named_node(&mut g, METHOD_HASSIMPLEMOVECONSTRUCTOR, "clang::CXXRecordDecl::hasSimpleMoveConstructor"),
      add_named_node(&mut g, METHOD_HASSIMPLECOPYASSIGNMENT, "clang::CXXRecordDecl::hasSimpleCopyAssignment"),
      add_named_node(&mut g, METHOD_HASSIMPLEMOVEASSIGNMENT, "clang::CXXRecordDecl::hasSimpleMoveAssignment"),
      add_named_node(&mut g, METHOD_HASSIMPLEDESTRUCTOR, "clang::CXXRecordDecl::hasSimpleDestructor"),
      add_named_node(&mut g, METHOD_HASDEFAULTCONSTRUCTOR, "clang::CXXRecordDecl::hasDefaultConstructor"),
      add_named_node(&mut g, METHOD_NEEDSIMPLICITDEFAULTCONSTRUCTOR, "clang::CXXRecordDecl::needsImplicitDefaultConstructor"),
      add_named_node(&mut g, METHOD_HASUSERDECLAREDCONSTRUCTOR, "clang::CXXRecordDecl::hasUserDeclaredConstructor"),
      add_named_node(&mut g, METHOD_HASUSERPROVIDEDDEFAULTCONSTRUCTOR, "clang::CXXRecordDecl::hasUserProvidedDefaultConstructor"),
      add_named_node(&mut g, METHOD_HASUSERDECLAREDCOPYCONSTRUCTOR, "clang::CXXRecordDecl::hasUserDeclaredCopyConstructor"),
      add_named_node(&mut g, METHOD_NEEDSIMPLICITCOPYCONSTRUCTOR, "clang::CXXRecordDecl::needsImplicitCopyConstructor"),
      add_named_node(&mut g, METHOD_NEEDSOVERLOADRESOLUTIONFORCOPYCONSTRUCTOR, "clang::CXXRecordDecl::needsOverloadResolutionForCopyConstructor"),
      add_named_node(&mut g, METHOD_IMPLICITCOPYCONSTRUCTORHASCONSTPARAM, "clang::CXXRecordDecl::implicitCopyConstructorHasConstParam"),
      add_named_node(&mut g, METHOD_HASCOPYCONSTRUCTORWITHCONSTPARAM, "clang::CXXRecordDecl::hasCopyConstructorWithConstParam"),
      add_named_node(&mut g, METHOD_HASUSERDECLAREDMOVEOPERATION, "clang::CXXRecordDecl::hasUserDeclaredMoveOperation"),
      add_named_node(&mut g, METHOD_HASUSERDECLAREDMOVECONSTRUCTOR, "clang::CXXRecordDecl::hasUserDeclaredMoveConstructor"),
      add_named_node(&mut g, METHOD_HASMOVECONSTRUCTOR, "clang::CXXRecordDecl::hasMoveConstructor"),
      add_named_node(&mut g, METHOD_NEEDSIMPLICITMOVECONSTRUCTOR, "clang::CXXRecordDecl::needsImplicitMoveConstructor"),
      add_named_node(&mut g, METHOD_NEEDSOVERLOADRESOLUTIONFORMOVECONSTRUCTOR, "clang::CXXRecordDecl::needsOverloadResolutionForMoveConstructor"),
      add_named_node(&mut g, METHOD_HASUSERDECLAREDCOPYASSIGNMENT, "clang::CXXRecordDecl::hasUserDeclaredCopyAssignment"),
      add_named_node(&mut g, METHOD_NEEDSIMPLICITCOPYASSIGNMENT, "clang::CXXRecordDecl::needsImplicitCopyAssignment"),
      add_named_node(&mut g, METHOD_NEEDSOVERLOADRESOLUTIONFORCOPYASSIGNMENT, "clang::CXXRecordDecl::needsOverloadResolutionForCopyAssignment"),
      add_named_node(&mut g, METHOD_IMPLICITCOPYASSIGNMENTHASCONSTPARAM, "clang::CXXRecordDecl::implicitCopyAssignmentHasConstParam"),
      add_named_node(&mut g, METHOD_HASCOPYASSIGNMENTWITHCONSTPARAM, "clang::CXXRecordDecl::hasCopyAssignmentWithConstParam"),
      add_named_node(&mut g, METHOD_HASUSERDECLAREDMOVEASSIGNMENT, "clang::CXXRecordDecl::hasUserDeclaredMoveAssignment"),
      add_named_node(&mut g, METHOD_HASMOVEASSIGNMENT, "clang::CXXRecordDecl::hasMoveAssignment"),
      add_named_node(&mut g, METHOD_NEEDSIMPLICITMOVEASSIGNMENT, "clang::CXXRecordDecl::needsImplicitMoveAssignment"),
      add_named_node(&mut g, METHOD_NEEDSOVERLOADRESOLUTIONFORMOVEASSIGNMENT, "clang::CXXRecordDecl::needsOverloadResolutionForMoveAssignment"),
      add_named_node(&mut g, METHOD_HASUSERDECLAREDDESTRUCTOR, "clang::CXXRecordDecl::hasUserDeclaredDestructor"),
      add_named_node(&mut g, METHOD_NEEDSIMPLICITDESTRUCTOR, "clang::CXXRecordDecl::needsImplicitDestructor"),
      add_named_node(&mut g, METHOD_NEEDSOVERLOADRESOLUTIONFORDESTRUCTOR, "clang::CXXRecordDecl::needsOverloadResolutionForDestructor"),
      add_named_node(&mut g, METHOD_ISLAMBDA, "clang::CXXRecordDecl::isLambda"),
      add_named_node(&mut g, METHOD_ISGENERICLAMBDA, "clang::CXXRecordDecl::isGenericLambda"),
      add_named_node(&mut g, METHOD_LAMBDAISDEFAULTCONSTRUCTIBLEANDASSIGNABLE, "clang::CXXRecordDecl::lambdaIsDefaultConstructibleAndAssignable"),
      add_named_node(&mut g, METHOD_GETLAMBDACALLOPERATOR, "clang::CXXRecordDecl::getLambdaCallOperator"),
      add_named_node(&mut g, METHOD_GETDEPENDENTLAMBDACALLOPERATOR, "clang::CXXRecordDecl::getDependentLambdaCallOperator"),
      add_named_node(&mut g, METHOD_GETLAMBDASTATICINVOKER, "clang::CXXRecordDecl::getLambdaStaticInvoker"),
      add_named_node(&mut g, METHOD_GETGENERICLAMBDATEMPLATEPARAMETERLIST, "clang::CXXRecordDecl::getGenericLambdaTemplateParameterList"),
      add_named_node(&mut g, METHOD_GETLAMBDAEXPLICITTEMPLATEPARAMETERS, "clang::CXXRecordDecl::getLambdaExplicitTemplateParameters"),
      add_named_node(&mut g, METHOD_GETLAMBDACAPTUREDEFAULT, "clang::CXXRecordDecl::getLambdaCaptureDefault"),
      add_named_node(&mut g, METHOD_ISCAPTURELESSLAMBDA, "clang::CXXRecordDecl::isCapturelessLambda"),
      add_named_node(&mut g, METHOD_CAPTURES_1, "clang::CXXRecordDecl::captures"),
      add_named_node(&mut g, METHOD_CAPTURES_BEGIN, "clang::CXXRecordDecl::captures_begin"),
      add_named_node(&mut g, METHOD_CAPTURES_END, "clang::CXXRecordDecl::captures_end"),
      add_named_node(&mut g, METHOD_CAPTURE_SIZE_2, "clang::CXXRecordDecl::capture_size"),
      add_named_node(&mut g, METHOD_CONVERSION_BEGIN, "clang::CXXRecordDecl::conversion_begin"),
      add_named_node(&mut g, METHOD_CONVERSION_END, "clang::CXXRecordDecl::conversion_end"),
      add_named_node(&mut g, METHOD_GETVISIBLECONVERSIONFUNCTIONS, "clang::CXXRecordDecl::getVisibleConversionFunctions"),
      add_named_node(&mut g, METHOD_ISAGGREGATE, "clang::CXXRecordDecl::isAggregate"),
      add_named_node(&mut g, METHOD_HASINCLASSINITIALIZER, "clang::CXXRecordDecl::hasInClassInitializer"),
      add_named_node(&mut g, METHOD_HASUNINITIALIZEDREFERENCEMEMBER, "clang::CXXRecordDecl::hasUninitializedReferenceMember"),
      add_named_node(&mut g, METHOD_ISPOD, "clang::CXXRecordDecl::isPOD"),
      add_named_node(&mut g, METHOD_ISCLIKE, "clang::CXXRecordDecl::isCLike"),
      add_named_node(&mut g, METHOD_ISEMPTY, "clang::CXXRecordDecl::isEmpty"),
      add_named_node(&mut g, METHOD_HASINITMETHOD, "clang::CXXRecordDecl::hasInitMethod"),
      add_named_node(&mut g, METHOD_HASPRIVATEFIELDS, "clang::CXXRecordDecl::hasPrivateFields"),
      add_named_node(&mut g, METHOD_HASPROTECTEDFIELDS, "clang::CXXRecordDecl::hasProtectedFields"),
      add_named_node(&mut g, METHOD_HASDIRECTFIELDS, "clang::CXXRecordDecl::hasDirectFields"),
      add_named_node(&mut g, METHOD_ISPOLYMORPHIC, "clang::CXXRecordDecl::isPolymorphic"),
      add_named_node(&mut g, METHOD_ISABSTRACT, "clang::CXXRecordDecl::isAbstract"),
      add_named_node(&mut g, METHOD_ISSTANDARDLAYOUT, "clang::CXXRecordDecl::isStandardLayout"),
      add_named_node(&mut g, METHOD_ISCXX11STANDARDLAYOUT, "clang::CXXRecordDecl::isCXX11StandardLayout"),
      add_named_node(&mut g, METHOD_HASMUTABLEFIELDS, "clang::CXXRecordDecl::hasMutableFields"),
      add_named_node(&mut g, METHOD_HASVARIANTMEMBERS, "clang::CXXRecordDecl::hasVariantMembers"),
      add_named_node(&mut g, METHOD_HASTRIVIALDEFAULTCONSTRUCTOR, "clang::CXXRecordDecl::hasTrivialDefaultConstructor"),
      add_named_node(&mut g, METHOD_HASNONTRIVIALDEFAULTCONSTRUCTOR, "clang::CXXRecordDecl::hasNonTrivialDefaultConstructor"),
      add_named_node(&mut g, METHOD_HASCONSTEXPRNONCOPYMOVECONSTRUCTOR, "clang::CXXRecordDecl::hasConstexprNonCopyMoveConstructor"),
      add_named_node(&mut g, METHOD_DEFAULTEDDEFAULTCONSTRUCTORISCONSTEXPR, "clang::CXXRecordDecl::defaultedDefaultConstructorIsConstexpr"),
      add_named_node(&mut g, METHOD_HASCONSTEXPRDEFAULTCONSTRUCTOR, "clang::CXXRecordDecl::hasConstexprDefaultConstructor"),
      add_named_node(&mut g, METHOD_HASTRIVIALCOPYCONSTRUCTOR, "clang::CXXRecordDecl::hasTrivialCopyConstructor"),
      add_named_node(&mut g, METHOD_HASTRIVIALCOPYCONSTRUCTORFORCALL, "clang::CXXRecordDecl::hasTrivialCopyConstructorForCall"),
      add_named_node(&mut g, METHOD_HASNONTRIVIALCOPYCONSTRUCTOR, "clang::CXXRecordDecl::hasNonTrivialCopyConstructor"),
      add_named_node(&mut g, METHOD_HASNONTRIVIALCOPYCONSTRUCTORFORCALL, "clang::CXXRecordDecl::hasNonTrivialCopyConstructorForCall"),
      add_named_node(&mut g, METHOD_HASTRIVIALMOVECONSTRUCTOR, "clang::CXXRecordDecl::hasTrivialMoveConstructor"),
      add_named_node(&mut g, METHOD_HASTRIVIALMOVECONSTRUCTORFORCALL, "clang::CXXRecordDecl::hasTrivialMoveConstructorForCall"),
      add_named_node(&mut g, METHOD_HASNONTRIVIALMOVECONSTRUCTOR, "clang::CXXRecordDecl::hasNonTrivialMoveConstructor"),
      add_named_node(&mut g, METHOD_HASNONTRIVIALMOVECONSTRUCTORFORCALL, "clang::CXXRecordDecl::hasNonTrivialMoveConstructorForCall"),
      add_named_node(&mut g, METHOD_HASTRIVIALCOPYASSIGNMENT, "clang::CXXRecordDecl::hasTrivialCopyAssignment"),
      add_named_node(&mut g, METHOD_HASNONTRIVIALCOPYASSIGNMENT, "clang::CXXRecordDecl::hasNonTrivialCopyAssignment"),
      add_named_node(&mut g, METHOD_HASTRIVIALMOVEASSIGNMENT, "clang::CXXRecordDecl::hasTrivialMoveAssignment"),
      add_named_node(&mut g, METHOD_HASNONTRIVIALMOVEASSIGNMENT, "clang::CXXRecordDecl::hasNonTrivialMoveAssignment"),
      add_named_node(&mut g, METHOD_DEFAULTEDDESTRUCTORISCONSTEXPR, "clang::CXXRecordDecl::defaultedDestructorIsConstexpr"),
      add_named_node(&mut g, METHOD_HASCONSTEXPRDESTRUCTOR, "clang::CXXRecordDecl::hasConstexprDestructor"),
      add_named_node(&mut g, METHOD_HASTRIVIALDESTRUCTOR, "clang::CXXRecordDecl::hasTrivialDestructor"),
      add_named_node(&mut g, METHOD_HASTRIVIALDESTRUCTORFORCALL, "clang::CXXRecordDecl::hasTrivialDestructorForCall"),
      add_named_node(&mut g, METHOD_HASNONTRIVIALDESTRUCTOR, "clang::CXXRecordDecl::hasNonTrivialDestructor"),
      add_named_node(&mut g, METHOD_HASNONTRIVIALDESTRUCTORFORCALL, "clang::CXXRecordDecl::hasNonTrivialDestructorForCall"),
      add_named_node(&mut g, METHOD_ALLOWCONSTDEFAULTINIT, "clang::CXXRecordDecl::allowConstDefaultInit"),
      add_named_node(&mut g, METHOD_HASIRRELEVANTDESTRUCTOR, "clang::CXXRecordDecl::hasIrrelevantDestructor"),
      add_named_node(&mut g, METHOD_HASNONLITERALTYPEFIELDSORBASES, "clang::CXXRecordDecl::hasNonLiteralTypeFieldsOrBases"),
      add_named_node(&mut g, METHOD_HASINHERITEDCONSTRUCTOR, "clang::CXXRecordDecl::hasInheritedConstructor"),
      add_named_node(&mut g, METHOD_HASINHERITEDASSIGNMENT, "clang::CXXRecordDecl::hasInheritedAssignment"),
      add_named_node(&mut g, METHOD_ISTRIVIALLYCOPYABLE, "clang::CXXRecordDecl::isTriviallyCopyable"),
      add_named_node(&mut g, METHOD_ISTRIVIALLYCOPYCONSTRUCTIBLE, "clang::CXXRecordDecl::isTriviallyCopyConstructible"),
      add_named_node(&mut g, METHOD_ISTRIVIAL, "clang::CXXRecordDecl::isTrivial"),
      add_named_node(&mut g, METHOD_ISLITERAL, "clang::CXXRecordDecl::isLiteral"),
      add_named_node(&mut g, METHOD_ISSTRUCTURAL, "clang::CXXRecordDecl::isStructural"),
      add_named_node(&mut g, METHOD_GETINSTANTIATEDFROMMEMBERCLASS, "clang::CXXRecordDecl::getInstantiatedFromMemberClass"),
      add_named_node(&mut g, METHOD_GETMEMBERSPECIALIZATIONINFO, "clang::CXXRecordDecl::getMemberSpecializationInfo"),
      add_named_node(&mut g, METHOD_GETDESCRIBEDCLASSTEMPLATE, "clang::CXXRecordDecl::getDescribedClassTemplate"),
      add_named_node(&mut g, METHOD_GETTEMPLATESPECIALIZATIONKIND, "clang::CXXRecordDecl::getTemplateSpecializationKind"),
      add_named_node(&mut g, METHOD_GETTEMPLATEINSTANTIATIONPATTERN, "clang::CXXRecordDecl::getTemplateInstantiationPattern"),
      add_named_node(&mut g, METHOD_GETDESTRUCTOR, "clang::CXXRecordDecl::getDestructor"),
      add_named_node(&mut g, METHOD_ISANYDESTRUCTORNORETURN, "clang::CXXRecordDecl::isAnyDestructorNoReturn"),
      add_named_node(&mut g, METHOD_ISLOCALCLASS, "clang::CXXRecordDecl::isLocalClass"),
      add_named_node(&mut g, METHOD_MAYBEABSTRACT, "clang::CXXRecordDecl::mayBeAbstract"),
      add_named_node(&mut g, METHOD_ISEFFECTIVELYFINAL, "clang::CXXRecordDecl::isEffectivelyFinal"),
      add_named_node(&mut g, METHOD_GETLAMBDAMANGLINGNUMBER, "clang::CXXRecordDecl::getLambdaManglingNumber"),
      add_named_node(&mut g, METHOD_HASKNOWNLAMBDAINTERNALLINKAGE, "clang::CXXRecordDecl::hasKnownLambdaInternalLinkage"),
      add_named_node(&mut g, METHOD_GETLAMBDACONTEXTDECL, "clang::CXXRecordDecl::getLambdaContextDecl"),
      add_named_node(&mut g, METHOD_GETLAMBDAINDEXINCONTEXT, "clang::CXXRecordDecl::getLambdaIndexInContext"),
      add_named_node(&mut g, METHOD_GETLAMBDANUMBERING, "clang::CXXRecordDecl::getLambdaNumbering"),
      add_named_node(&mut g, METHOD_GETDEVICELAMBDAMANGLINGNUMBER, "clang::CXXRecordDecl::getDeviceLambdaManglingNumber"),
      add_named_node(&mut g, METHOD_GETMSINHERITANCEMODEL, "clang::CXXRecordDecl::getMSInheritanceModel"),
      add_named_node(&mut g, METHOD_CALCULATEINHERITANCEMODEL, "clang::CXXRecordDecl::calculateInheritanceModel"),
      add_named_node(&mut g, METHOD_NULLFIELDOFFSETISZERO, "clang::CXXRecordDecl::nullFieldOffsetIsZero"),
      add_named_node(&mut g, METHOD_GETMSVTORDISPMODE, "clang::CXXRecordDecl::getMSVtorDispMode"),
      add_named_node(&mut g, METHOD_ISDEPENDENTLAMBDA, "clang::CXXRecordDecl::isDependentLambda"),
      add_named_node(&mut g, METHOD_ISNEVERDEPENDENTLAMBDA, "clang::CXXRecordDecl::isNeverDependentLambda"),
      add_named_node(&mut g, METHOD_GETLAMBDADEPENDENCYKIND, "clang::CXXRecordDecl::getLambdaDependencyKind"),
      add_named_node(&mut g, METHOD_GETLAMBDATYPEINFO, "clang::CXXRecordDecl::getLambdaTypeInfo"),
      add_named_node(&mut g, METHOD_ISINTERFACELIKE, "clang::CXXRecordDecl::isInterfaceLike"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXRECORDDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_HLSLBUFFERDECL, "clang::HLSLBufferDecl");
  g.add_edge((CLASS_HLSLBUFFERDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_HLSLBUFFERDECL, META_SUBCLASS, CLASS_NAMEDDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSOURCERANGE_14, "clang::HLSLBufferDecl::getSourceRange"),
      add_named_node(&mut g, METHOD_GETLOCSTART, "clang::HLSLBufferDecl::getLocStart"),
      add_named_node(&mut g, METHOD_GETLBRACELOC, "clang::HLSLBufferDecl::getLBraceLoc"),
      add_named_node(&mut g, METHOD_GETRBRACELOC_1, "clang::HLSLBufferDecl::getRBraceLoc"),
      add_named_node(&mut g, METHOD_ISCBUFFER, "clang::HLSLBufferDecl::isCBuffer"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_HLSLBUFFERDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TEMPLATEPARAMOBJECTDECL, "clang::TemplateParamObjectDecl");
  g.add_edge((CLASS_TEMPLATEPARAMOBJECTDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TEMPLATEPARAMOBJECTDECL, META_SUBCLASS, CLASS_VALUEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETVALUE_2, "clang::TemplateParamObjectDecl::getValue"),
      add_named_node(&mut g, METHOD_GETCANONICALDECL_18, "clang::TemplateParamObjectDecl::getCanonicalDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TEMPLATEPARAMOBJECTDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_EXPRESSIONTRAITEXPR, "clang::ExpressionTraitExpr");
  g.add_edge((CLASS_EXPRESSIONTRAITEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_EXPRESSIONTRAITEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETBEGINLOC_72, "clang::ExpressionTraitExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_71, "clang::ExpressionTraitExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETTRAIT_1, "clang::ExpressionTraitExpr::getTrait"),
      add_named_node(&mut g, METHOD_GETQUERIEDEXPRESSION, "clang::ExpressionTraitExpr::getQueriedExpression"),
      add_named_node(&mut g, METHOD_GETVALUE_7, "clang::ExpressionTraitExpr::getValue"),
      add_named_node(&mut g, METHOD_CHILDREN_62, "clang::ExpressionTraitExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_EXPRESSIONTRAITEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MSPROPERTYDECL, "clang::MSPropertyDecl");
  g.add_edge((CLASS_MSPROPERTYDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MSPROPERTYDECL, META_SUBCLASS, CLASS_DECLARATORDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_HASGETTER, "clang::MSPropertyDecl::hasGetter"),
      add_named_node(&mut g, METHOD_GETGETTERID, "clang::MSPropertyDecl::getGetterId"),
      add_named_node(&mut g, METHOD_HASSETTER, "clang::MSPropertyDecl::hasSetter"),
      add_named_node(&mut g, METHOD_GETSETTERID, "clang::MSPropertyDecl::getSetterId"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MSPROPERTYDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_USINGSHADOWDECL, "clang::UsingShadowDecl");
  g.add_edge((CLASS_USINGSHADOWDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_USINGSHADOWDECL, META_SUBCLASS, CLASS_NAMEDDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETCANONICALDECL_26, "clang::UsingShadowDecl::getCanonicalDecl"),
      add_named_node(&mut g, METHOD_GETTARGETDECL, "clang::UsingShadowDecl::getTargetDecl"),
      add_named_node(&mut g, METHOD_GETINTRODUCER_1, "clang::UsingShadowDecl::getIntroducer"),
      add_named_node(&mut g, METHOD_GETNEXTUSINGSHADOWDECL, "clang::UsingShadowDecl::getNextUsingShadowDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_USINGSHADOWDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_USINGENUMDECL, "clang::UsingEnumDecl");
  g.add_edge((CLASS_USINGENUMDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_USINGENUMDECL, META_SUBCLASS, CLASS_BASEUSINGDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETUSINGLOC_4, "clang::UsingEnumDecl::getUsingLoc"),
      add_named_node(&mut g, METHOD_GETENUMLOC, "clang::UsingEnumDecl::getEnumLoc"),
      add_named_node(&mut g, METHOD_GETQUALIFIER_10, "clang::UsingEnumDecl::getQualifier"),
      add_named_node(&mut g, METHOD_GETQUALIFIERLOC_7, "clang::UsingEnumDecl::getQualifierLoc"),
      add_named_node(&mut g, METHOD_GETENUMTYPELOC, "clang::UsingEnumDecl::getEnumTypeLoc"),
      add_named_node(&mut g, METHOD_GETENUMTYPE, "clang::UsingEnumDecl::getEnumType"),
      add_named_node(&mut g, METHOD_GETENUMDECL, "clang::UsingEnumDecl::getEnumDecl"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_34, "clang::UsingEnumDecl::getSourceRange"),
      add_named_node(&mut g, METHOD_GETCANONICALDECL_24, "clang::UsingEnumDecl::getCanonicalDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_USINGENUMDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FUNCTIONDECL, "clang::FunctionDecl");
  g.add_edge((CLASS_FUNCTIONDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FUNCTIONDECL, META_SUBCLASS, CLASS_DECLARATORDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETNAMEINFO, "clang::FunctionDecl::getNameInfo"),
      add_named_node(&mut g, METHOD_GETELLIPSISLOC_1, "clang::FunctionDecl::getEllipsisLoc"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_13, "clang::FunctionDecl::getSourceRange"),
      add_named_node(&mut g, METHOD_HASBODY_1, "clang::FunctionDecl::hasBody"),
      add_named_node(&mut g, METHOD_HASTRIVIALBODY, "clang::FunctionDecl::hasTrivialBody"),
      add_named_node(&mut g, METHOD_ISDEFINED, "clang::FunctionDecl::isDefined"),
      add_named_node(&mut g, METHOD_GETDEFINITION_2, "clang::FunctionDecl::getDefinition"),
      add_named_node(&mut g, METHOD_GETBODY_3, "clang::FunctionDecl::getBody"),
      add_named_node(&mut g, METHOD_ISTHISDECLARATIONADEFINITION_1, "clang::FunctionDecl::isThisDeclarationADefinition"),
      add_named_node(&mut g, METHOD_ISTHISDECLARATIONINSTANTIATEDFROMAFRIENDDEFINITION, "clang::FunctionDecl::isThisDeclarationInstantiatedFromAFriendDefinition"),
      add_named_node(&mut g, METHOD_DOESTHISDECLARATIONHAVEABODY, "clang::FunctionDecl::doesThisDeclarationHaveABody"),
      add_named_node(&mut g, METHOD_GETDEFAULTEDFUNCTIONINFO, "clang::FunctionDecl::getDefaultedFunctionInfo"),
      add_named_node(&mut g, METHOD_ISVARIADIC_2, "clang::FunctionDecl::isVariadic"),
      add_named_node(&mut g, METHOD_ISVIRTUALASWRITTEN, "clang::FunctionDecl::isVirtualAsWritten"),
      add_named_node(&mut g, METHOD_ISPUREVIRTUAL, "clang::FunctionDecl::isPureVirtual"),
      add_named_node(&mut g, METHOD_ISLATETEMPLATEPARSED, "clang::FunctionDecl::isLateTemplateParsed"),
      add_named_node(&mut g, METHOD_ISTRIVIAL_1, "clang::FunctionDecl::isTrivial"),
      add_named_node(&mut g, METHOD_ISTRIVIALFORCALL, "clang::FunctionDecl::isTrivialForCall"),
      add_named_node(&mut g, METHOD_ISDEFAULTED, "clang::FunctionDecl::isDefaulted"),
      add_named_node(&mut g, METHOD_ISEXPLICITLYDEFAULTED, "clang::FunctionDecl::isExplicitlyDefaulted"),
      add_named_node(&mut g, METHOD_GETDEFAULTLOC, "clang::FunctionDecl::getDefaultLoc"),
      add_named_node(&mut g, METHOD_ISUSERPROVIDED, "clang::FunctionDecl::isUserProvided"),
      add_named_node(&mut g, METHOD_ISINELIGIBLEORNOTSELECTED, "clang::FunctionDecl::isIneligibleOrNotSelected"),
      add_named_node(&mut g, METHOD_HASIMPLICITRETURNZERO, "clang::FunctionDecl::hasImplicitReturnZero"),
      add_named_node(&mut g, METHOD_HASPROTOTYPE, "clang::FunctionDecl::hasPrototype"),
      add_named_node(&mut g, METHOD_HASWRITTENPROTOTYPE, "clang::FunctionDecl::hasWrittenPrototype"),
      add_named_node(&mut g, METHOD_HASINHERITEDPROTOTYPE, "clang::FunctionDecl::hasInheritedPrototype"),
      add_named_node(&mut g, METHOD_ISCONSTEXPR, "clang::FunctionDecl::isConstexpr"),
      add_named_node(&mut g, METHOD_GETCONSTEXPRKIND, "clang::FunctionDecl::getConstexprKind"),
      add_named_node(&mut g, METHOD_ISCONSTEXPRSPECIFIED, "clang::FunctionDecl::isConstexprSpecified"),
      add_named_node(&mut g, METHOD_ISCONSTEVAL, "clang::FunctionDecl::isConsteval"),
      add_named_node(&mut g, METHOD_BODYCONTAINSIMMEDIATEESCALATINGEXPRESSIONS, "clang::FunctionDecl::BodyContainsImmediateEscalatingExpressions"),
      add_named_node(&mut g, METHOD_ISIMMEDIATEESCALATING, "clang::FunctionDecl::isImmediateEscalating"),
      add_named_node(&mut g, METHOD_ISIMMEDIATEFUNCTION, "clang::FunctionDecl::isImmediateFunction"),
      add_named_node(&mut g, METHOD_INSTANTIATIONISPENDING, "clang::FunctionDecl::instantiationIsPending"),
      add_named_node(&mut g, METHOD_USESSEHTRY, "clang::FunctionDecl::usesSEHTry"),
      add_named_node(&mut g, METHOD_ISDELETED, "clang::FunctionDecl::isDeleted"),
      add_named_node(&mut g, METHOD_ISDELETEDASWRITTEN, "clang::FunctionDecl::isDeletedAsWritten"),
      add_named_node(&mut g, METHOD_ISMAIN, "clang::FunctionDecl::isMain"),
      add_named_node(&mut g, METHOD_ISMSVCRTENTRYPOINT, "clang::FunctionDecl::isMSVCRTEntryPoint"),
      add_named_node(&mut g, METHOD_ISRESERVEDGLOBALPLACEMENTOPERATOR, "clang::FunctionDecl::isReservedGlobalPlacementOperator"),
      add_named_node(&mut g, METHOD_ISINLINEBUILTINDECLARATION, "clang::FunctionDecl::isInlineBuiltinDeclaration"),
      add_named_node(&mut g, METHOD_ISDESTROYINGOPERATORDELETE, "clang::FunctionDecl::isDestroyingOperatorDelete"),
      add_named_node(&mut g, METHOD_GETLANGUAGELINKAGE, "clang::FunctionDecl::getLanguageLinkage"),
      add_named_node(&mut g, METHOD_ISEXTERNC, "clang::FunctionDecl::isExternC"),
      add_named_node(&mut g, METHOD_ISINEXTERNCCONTEXT, "clang::FunctionDecl::isInExternCContext"),
      add_named_node(&mut g, METHOD_ISINEXTERNCXXCONTEXT, "clang::FunctionDecl::isInExternCXXContext"),
      add_named_node(&mut g, METHOD_ISGLOBAL, "clang::FunctionDecl::isGlobal"),
      add_named_node(&mut g, METHOD_ISNORETURN, "clang::FunctionDecl::isNoReturn"),
      add_named_node(&mut g, METHOD_HASSKIPPEDBODY, "clang::FunctionDecl::hasSkippedBody"),
      add_named_node(&mut g, METHOD_WILLHAVEBODY, "clang::FunctionDecl::willHaveBody"),
      add_named_node(&mut g, METHOD_ISMULTIVERSION, "clang::FunctionDecl::isMultiVersion"),
      add_named_node(&mut g, METHOD_FRIENDCONSTRAINTREFERSTOENCLOSINGTEMPLATE, "clang::FunctionDecl::FriendConstraintRefersToEnclosingTemplate"),
      add_named_node(&mut g, METHOD_ISMEMBERLIKECONSTRAINEDFRIEND, "clang::FunctionDecl::isMemberLikeConstrainedFriend"),
      add_named_node(&mut g, METHOD_GETMULTIVERSIONKIND, "clang::FunctionDecl::getMultiVersionKind"),
      add_named_node(&mut g, METHOD_ISCPUDISPATCHMULTIVERSION, "clang::FunctionDecl::isCPUDispatchMultiVersion"),
      add_named_node(&mut g, METHOD_ISCPUSPECIFICMULTIVERSION, "clang::FunctionDecl::isCPUSpecificMultiVersion"),
      add_named_node(&mut g, METHOD_ISTARGETMULTIVERSION, "clang::FunctionDecl::isTargetMultiVersion"),
      add_named_node(&mut g, METHOD_ISTARGETCLONESMULTIVERSION, "clang::FunctionDecl::isTargetClonesMultiVersion"),
      add_named_node(&mut g, METHOD_GETCANONICALDECL_11, "clang::FunctionDecl::getCanonicalDecl"),
      add_named_node(&mut g, METHOD_PARAMETERS_2, "clang::FunctionDecl::parameters"),
      add_named_node(&mut g, METHOD_PARAM_EMPTY_1, "clang::FunctionDecl::param_empty"),
      add_named_node(&mut g, METHOD_PARAM_BEGIN, "clang::FunctionDecl::param_begin"),
      add_named_node(&mut g, METHOD_PARAM_END, "clang::FunctionDecl::param_end"),
      add_named_node(&mut g, METHOD_PARAM_SIZE_1, "clang::FunctionDecl::param_size"),
      add_named_node(&mut g, METHOD_GETNUMPARAMS_3, "clang::FunctionDecl::getNumParams"),
      add_named_node(&mut g, METHOD_GETMINREQUIREDARGUMENTS, "clang::FunctionDecl::getMinRequiredArguments"),
      add_named_node(&mut g, METHOD_GETMINREQUIREDEXPLICITARGUMENTS, "clang::FunctionDecl::getMinRequiredExplicitArguments"),
      add_named_node(&mut g, METHOD_HASCXXEXPLICITFUNCTIONOBJECTPARAMETER, "clang::FunctionDecl::hasCXXExplicitFunctionObjectParameter"),
      add_named_node(&mut g, METHOD_GETNUMNONOBJECTPARAMS, "clang::FunctionDecl::getNumNonObjectParams"),
      add_named_node(&mut g, METHOD_HASONEPARAMORDEFAULTARGS, "clang::FunctionDecl::hasOneParamOrDefaultArgs"),
      add_named_node(&mut g, METHOD_GETFUNCTIONTYPELOC, "clang::FunctionDecl::getFunctionTypeLoc"),
      add_named_node(&mut g, METHOD_GETRETURNTYPE_1, "clang::FunctionDecl::getReturnType"),
      add_named_node(&mut g, METHOD_GETRETURNTYPESOURCERANGE, "clang::FunctionDecl::getReturnTypeSourceRange"),
      add_named_node(&mut g, METHOD_GETPARAMETERSSOURCERANGE, "clang::FunctionDecl::getParametersSourceRange"),
      add_named_node(&mut g, METHOD_GETDECLAREDRETURNTYPE, "clang::FunctionDecl::getDeclaredReturnType"),
      add_named_node(&mut g, METHOD_GETEXCEPTIONSPECTYPE_1, "clang::FunctionDecl::getExceptionSpecType"),
      add_named_node(&mut g, METHOD_GETEXCEPTIONSPECSOURCERANGE, "clang::FunctionDecl::getExceptionSpecSourceRange"),
      add_named_node(&mut g, METHOD_GETCALLRESULTTYPE, "clang::FunctionDecl::getCallResultType"),
      add_named_node(&mut g, METHOD_GETSTORAGECLASS, "clang::FunctionDecl::getStorageClass"),
      add_named_node(&mut g, METHOD_ISINLINESPECIFIED, "clang::FunctionDecl::isInlineSpecified"),
      add_named_node(&mut g, METHOD_USESFPINTRIN, "clang::FunctionDecl::UsesFPIntrin"),
      add_named_node(&mut g, METHOD_ISINLINED, "clang::FunctionDecl::isInlined"),
      add_named_node(&mut g, METHOD_ISINLINEDEFINITIONEXTERNALLYVISIBLE, "clang::FunctionDecl::isInlineDefinitionExternallyVisible"),
      add_named_node(&mut g, METHOD_ISMSEXTERNINLINE, "clang::FunctionDecl::isMSExternInline"),
      add_named_node(&mut g, METHOD_DOESDECLARATIONFORCEEXTERNALLYVISIBLEDEFINITION, "clang::FunctionDecl::doesDeclarationForceExternallyVisibleDefinition"),
      add_named_node(&mut g, METHOD_ISSTATIC_1, "clang::FunctionDecl::isStatic"),
      add_named_node(&mut g, METHOD_ISOVERLOADEDOPERATOR, "clang::FunctionDecl::isOverloadedOperator"),
      add_named_node(&mut g, METHOD_GETOVERLOADEDOPERATOR, "clang::FunctionDecl::getOverloadedOperator"),
      add_named_node(&mut g, METHOD_GETLITERALIDENTIFIER, "clang::FunctionDecl::getLiteralIdentifier"),
      add_named_node(&mut g, METHOD_GETINSTANTIATEDFROMMEMBERFUNCTION, "clang::FunctionDecl::getInstantiatedFromMemberFunction"),
      add_named_node(&mut g, METHOD_GETTEMPLATEDKIND, "clang::FunctionDecl::getTemplatedKind"),
      add_named_node(&mut g, METHOD_GETMEMBERSPECIALIZATIONINFO_2, "clang::FunctionDecl::getMemberSpecializationInfo"),
      add_named_node(&mut g, METHOD_GETINSTANTIATEDFROMDECL, "clang::FunctionDecl::getInstantiatedFromDecl"),
      add_named_node(&mut g, METHOD_GETDESCRIBEDFUNCTIONTEMPLATE, "clang::FunctionDecl::getDescribedFunctionTemplate"),
      add_named_node(&mut g, METHOD_ISFUNCTIONTEMPLATESPECIALIZATION, "clang::FunctionDecl::isFunctionTemplateSpecialization"),
      add_named_node(&mut g, METHOD_GETTEMPLATESPECIALIZATIONINFO, "clang::FunctionDecl::getTemplateSpecializationInfo"),
      add_named_node(&mut g, METHOD_ISIMPLICITLYINSTANTIABLE, "clang::FunctionDecl::isImplicitlyInstantiable"),
      add_named_node(&mut g, METHOD_ISTEMPLATEINSTANTIATION, "clang::FunctionDecl::isTemplateInstantiation"),
      add_named_node(&mut g, METHOD_GETPRIMARYTEMPLATE, "clang::FunctionDecl::getPrimaryTemplate"),
      add_named_node(&mut g, METHOD_GETTEMPLATESPECIALIZATIONARGS, "clang::FunctionDecl::getTemplateSpecializationArgs"),
      add_named_node(&mut g, METHOD_GETTEMPLATESPECIALIZATIONARGSASWRITTEN, "clang::FunctionDecl::getTemplateSpecializationArgsAsWritten"),
      add_named_node(&mut g, METHOD_GETDEPENDENTSPECIALIZATIONINFO, "clang::FunctionDecl::getDependentSpecializationInfo"),
      add_named_node(&mut g, METHOD_GETTEMPLATESPECIALIZATIONKIND_2, "clang::FunctionDecl::getTemplateSpecializationKind"),
      add_named_node(&mut g, METHOD_GETTEMPLATESPECIALIZATIONKINDFORINSTANTIATION, "clang::FunctionDecl::getTemplateSpecializationKindForInstantiation"),
      add_named_node(&mut g, METHOD_GETPOINTOFINSTANTIATION_1, "clang::FunctionDecl::getPointOfInstantiation"),
      add_named_node(&mut g, METHOD_ISOUTOFLINE_1, "clang::FunctionDecl::isOutOfLine"),
      add_named_node(&mut g, METHOD_GETMEMORYFUNCTIONKIND, "clang::FunctionDecl::getMemoryFunctionKind"),
      add_named_node(&mut g, METHOD_GETODRHASH_1, "clang::FunctionDecl::getODRHash"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FUNCTIONDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CLASSTEMPLATEDECL, "clang::ClassTemplateDecl");
  g.add_edge((CLASS_CLASSTEMPLATEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CLASSTEMPLATEDECL, META_SUBCLASS, CLASS_REDECLARABLETEMPLATEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTEMPLATEDDECL, "clang::ClassTemplateDecl::getTemplatedDecl"),
      add_named_node(&mut g, METHOD_ISTHISDECLARATIONADEFINITION, "clang::ClassTemplateDecl::isThisDeclarationADefinition"),
      add_named_node(&mut g, METHOD_GETCANONICALDECL_5, "clang::ClassTemplateDecl::getCanonicalDecl"),
      add_named_node(&mut g, METHOD_GETPREVIOUSDECL_1, "clang::ClassTemplateDecl::getPreviousDecl"),
      add_named_node(&mut g, METHOD_GETMOSTRECENTDECL_2, "clang::ClassTemplateDecl::getMostRecentDecl"),
      add_named_node(&mut g, METHOD_GETINSTANTIATEDFROMMEMBERTEMPLATE, "clang::ClassTemplateDecl::getInstantiatedFromMemberTemplate"),
      add_named_node(&mut g, METHOD_SPECIALIZATIONS, "clang::ClassTemplateDecl::specializations"),
      add_named_node(&mut g, METHOD_SPEC_BEGIN, "clang::ClassTemplateDecl::spec_begin"),
      add_named_node(&mut g, METHOD_SPEC_END, "clang::ClassTemplateDecl::spec_end"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CLASSTEMPLATEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TEMPLATETEMPLATEPARMDECL, "clang::TemplateTemplateParmDecl");
  g.add_edge((CLASS_TEMPLATETEMPLATEPARMDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TEMPLATETEMPLATEPARMDECL, META_SUBCLASS, CLASS_TEMPLATEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_ISPARAMETERPACK_3, "clang::TemplateTemplateParmDecl::isParameterPack"),
      add_named_node(&mut g, METHOD_ISPACKEXPANSION_1, "clang::TemplateTemplateParmDecl::isPackExpansion"),
      add_named_node(&mut g, METHOD_ISEXPANDEDPARAMETERPACK_1, "clang::TemplateTemplateParmDecl::isExpandedParameterPack"),
      add_named_node(&mut g, METHOD_GETNUMEXPANSIONTEMPLATEPARAMETERS, "clang::TemplateTemplateParmDecl::getNumExpansionTemplateParameters"),
      add_named_node(&mut g, METHOD_GETDEFAULTARGSTORAGE_1, "clang::TemplateTemplateParmDecl::getDefaultArgStorage"),
      add_named_node(&mut g, METHOD_HASDEFAULTARGUMENT_1, "clang::TemplateTemplateParmDecl::hasDefaultArgument"),
      add_named_node(&mut g, METHOD_GETDEFAULTARGUMENT_1, "clang::TemplateTemplateParmDecl::getDefaultArgument"),
      add_named_node(&mut g, METHOD_GETDEFAULTARGUMENTLOC_1, "clang::TemplateTemplateParmDecl::getDefaultArgumentLoc"),
      add_named_node(&mut g, METHOD_DEFAULTARGUMENTWASINHERITED_1, "clang::TemplateTemplateParmDecl::defaultArgumentWasInherited"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_25, "clang::TemplateTemplateParmDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TEMPLATETEMPLATEPARMDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXFORRANGESTMT, "clang::CXXForRangeStmt");
  g.add_edge((CLASS_CXXFORRANGESTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXFORRANGESTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETINIT_2, "clang::CXXForRangeStmt::getInit"),
      add_named_node(&mut g, METHOD_GETLOOPVARIABLE, "clang::CXXForRangeStmt::getLoopVariable"),
      add_named_node(&mut g, METHOD_GETRANGEINIT, "clang::CXXForRangeStmt::getRangeInit"),
      add_named_node(&mut g, METHOD_GETRANGESTMT, "clang::CXXForRangeStmt::getRangeStmt"),
      add_named_node(&mut g, METHOD_GETBEGINSTMT, "clang::CXXForRangeStmt::getBeginStmt"),
      add_named_node(&mut g, METHOD_GETENDSTMT, "clang::CXXForRangeStmt::getEndStmt"),
      add_named_node(&mut g, METHOD_GETCOND_2, "clang::CXXForRangeStmt::getCond"),
      add_named_node(&mut g, METHOD_GETINC, "clang::CXXForRangeStmt::getInc"),
      add_named_node(&mut g, METHOD_GETLOOPVARSTMT, "clang::CXXForRangeStmt::getLoopVarStmt"),
      add_named_node(&mut g, METHOD_GETBODY_5, "clang::CXXForRangeStmt::getBody"),
      add_named_node(&mut g, METHOD_GETFORLOC, "clang::CXXForRangeStmt::getForLoc"),
      add_named_node(&mut g, METHOD_GETCOAWAITLOC, "clang::CXXForRangeStmt::getCoawaitLoc"),
      add_named_node(&mut g, METHOD_GETCOLONLOC_2, "clang::CXXForRangeStmt::getColonLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_6, "clang::CXXForRangeStmt::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_28, "clang::CXXForRangeStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_27, "clang::CXXForRangeStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_22, "clang::CXXForRangeStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXFORRANGESTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CONCEPTDECL, "clang::ConceptDecl");
  g.add_edge((CLASS_CONCEPTDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONCEPTDECL, META_SUBCLASS, CLASS_TEMPLATEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETCONSTRAINTEXPR, "clang::ConceptDecl::getConstraintExpr"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_4, "clang::ConceptDecl::getSourceRange"),
      add_named_node(&mut g, METHOD_ISTYPECONCEPT, "clang::ConceptDecl::isTypeConcept"),
      add_named_node(&mut g, METHOD_GETCANONICALDECL_6, "clang::ConceptDecl::getCanonicalDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CONCEPTDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCCONTAINERDECL, "clang::ObjCContainerDecl");
  g.add_edge((CLASS_OBJCCONTAINERDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCCONTAINERDECL, META_SUBCLASS, CLASS_NAMEDDECL));

  g.add_named_node(CLASS_UNRESOLVEDUSINGIFEXISTSDECL, "clang::UnresolvedUsingIfExistsDecl");
  g.add_edge((CLASS_UNRESOLVEDUSINGIFEXISTSDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNRESOLVEDUSINGIFEXISTSDECL, META_SUBCLASS, CLASS_NAMEDDECL));

  g.add_named_node(CLASS_OMPALLOCATEDECL, "clang::OMPAllocateDecl");
  g.add_edge((CLASS_OMPALLOCATEDECL, META_CLASS, META_CLANG_AST_NODE));

  g.add_named_node(CLASS_OBJCINTERFACEDECL, "clang::ObjCInterfaceDecl");
  g.add_edge((CLASS_OBJCINTERFACEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCINTERFACEDECL, META_SUBCLASS, CLASS_OBJCCONTAINERDECL));

  g.add_named_node(CLASS_BREAKSTMT, "clang::BreakStmt");
  g.add_edge((CLASS_BREAKSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BREAKSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETBREAKLOC, "clang::BreakStmt::getBreakLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_16, "clang::BreakStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_15, "clang::BreakStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_12, "clang::BreakStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BREAKSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPORDEREDDIRECTIVE, "clang::OMPOrderedDirective");
  g.add_edge((CLASS_OMPORDEREDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPORDEREDDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_CXXDELETEEXPR, "clang::CXXDeleteExpr");
  g.add_edge((CLASS_CXXDELETEEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXDELETEEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_ISGLOBALDELETE, "clang::CXXDeleteExpr::isGlobalDelete"),
      add_named_node(&mut g, METHOD_ISARRAYFORM, "clang::CXXDeleteExpr::isArrayForm"),
      add_named_node(&mut g, METHOD_ISARRAYFORMASWRITTEN, "clang::CXXDeleteExpr::isArrayFormAsWritten"),
      add_named_node(&mut g, METHOD_DOESUSUALARRAYDELETEWANTSIZE, "clang::CXXDeleteExpr::doesUsualArrayDeleteWantSize"),
      add_named_node(&mut g, METHOD_GETOPERATORDELETE_1, "clang::CXXDeleteExpr::getOperatorDelete"),
      add_named_node(&mut g, METHOD_GETARGUMENT, "clang::CXXDeleteExpr::getArgument"),
      add_named_node(&mut g, METHOD_GETDESTROYEDTYPE, "clang::CXXDeleteExpr::getDestroyedType"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_25, "clang::CXXDeleteExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_24, "clang::CXXDeleteExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_19, "clang::CXXDeleteExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXDELETEEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_STATICASSERTDECL, "clang::StaticAssertDecl");
  g.add_edge((CLASS_STATICASSERTDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_STATICASSERTDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETASSERTEXPR, "clang::StaticAssertDecl::getAssertExpr"),
      add_named_node(&mut g, METHOD_GETMESSAGE, "clang::StaticAssertDecl::getMessage"),
      add_named_node(&mut g, METHOD_ISFAILED, "clang::StaticAssertDecl::isFailed"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_1, "clang::StaticAssertDecl::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_22, "clang::StaticAssertDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_STATICASSERTDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TOPLEVELSTMTDECL, "clang::TopLevelStmtDecl");
  g.add_edge((CLASS_TOPLEVELSTMTDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TOPLEVELSTMTDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSOURCERANGE_27, "clang::TopLevelStmtDecl::getSourceRange"),
      add_named_node(&mut g, METHOD_GETSTMT_1, "clang::TopLevelStmtDecl::getStmt"),
      add_named_node(&mut g, METHOD_ISSEMIMISSING, "clang::TopLevelStmtDecl::isSemiMissing"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TOPLEVELSTMTDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXMETHODDECL, "clang::CXXMethodDecl");
  g.add_edge((CLASS_CXXMETHODDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXMETHODDECL, META_SUBCLASS, CLASS_FUNCTIONDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_ISSTATIC, "clang::CXXMethodDecl::isStatic"),
      add_named_node(&mut g, METHOD_ISINSTANCE, "clang::CXXMethodDecl::isInstance"),
      add_named_node(&mut g, METHOD_ISEXPLICITOBJECTMEMBERFUNCTION, "clang::CXXMethodDecl::isExplicitObjectMemberFunction"),
      add_named_node(&mut g, METHOD_ISIMPLICITOBJECTMEMBERFUNCTION, "clang::CXXMethodDecl::isImplicitObjectMemberFunction"),
      add_named_node(&mut g, METHOD_ISCONST_1, "clang::CXXMethodDecl::isConst"),
      add_named_node(&mut g, METHOD_ISVOLATILE_1, "clang::CXXMethodDecl::isVolatile"),
      add_named_node(&mut g, METHOD_ISVIRTUAL, "clang::CXXMethodDecl::isVirtual"),
      add_named_node(&mut g, METHOD_ISCOPYASSIGNMENTOPERATOR, "clang::CXXMethodDecl::isCopyAssignmentOperator"),
      add_named_node(&mut g, METHOD_ISMOVEASSIGNMENTOPERATOR, "clang::CXXMethodDecl::isMoveAssignmentOperator"),
      add_named_node(&mut g, METHOD_GETCANONICALDECL_3, "clang::CXXMethodDecl::getCanonicalDecl"),
      add_named_node(&mut g, METHOD_GETMOSTRECENTDECL, "clang::CXXMethodDecl::getMostRecentDecl"),
      add_named_node(&mut g, METHOD_BEGIN_OVERRIDDEN_METHODS, "clang::CXXMethodDecl::begin_overridden_methods"),
      add_named_node(&mut g, METHOD_END_OVERRIDDEN_METHODS, "clang::CXXMethodDecl::end_overridden_methods"),
      add_named_node(&mut g, METHOD_SIZE_OVERRIDDEN_METHODS, "clang::CXXMethodDecl::size_overridden_methods"),
      add_named_node(&mut g, METHOD_OVERRIDDEN_METHODS, "clang::CXXMethodDecl::overridden_methods"),
      add_named_node(&mut g, METHOD_GETPARENT, "clang::CXXMethodDecl::getParent"),
      add_named_node(&mut g, METHOD_GETTHISTYPE, "clang::CXXMethodDecl::getThisType"),
      add_named_node(&mut g, METHOD_GETFUNCTIONOBJECTPARAMETERREFERENCETYPE, "clang::CXXMethodDecl::getFunctionObjectParameterReferenceType"),
      add_named_node(&mut g, METHOD_GETFUNCTIONOBJECTPARAMETERTYPE, "clang::CXXMethodDecl::getFunctionObjectParameterType"),
      add_named_node(&mut g, METHOD_GETNUMEXPLICITPARAMS, "clang::CXXMethodDecl::getNumExplicitParams"),
      add_named_node(&mut g, METHOD_GETMETHODQUALIFIERS, "clang::CXXMethodDecl::getMethodQualifiers"),
      add_named_node(&mut g, METHOD_GETREFQUALIFIER_1, "clang::CXXMethodDecl::getRefQualifier"),
      add_named_node(&mut g, METHOD_HASINLINEBODY, "clang::CXXMethodDecl::hasInlineBody"),
      add_named_node(&mut g, METHOD_ISLAMBDASTATICINVOKER, "clang::CXXMethodDecl::isLambdaStaticInvoker"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXMETHODDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_VALUEDECL, "clang::ValueDecl");
  g.add_edge((CLASS_VALUEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_VALUEDECL, META_SUBCLASS, CLASS_NAMEDDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTYPE, "clang::ValueDecl::getType"),
      add_named_node(&mut g, METHOD_ISWEAK, "clang::ValueDecl::isWeak"),
      add_named_node(&mut g, METHOD_ISINITCAPTURE, "clang::ValueDecl::isInitCapture"),
      add_named_node(&mut g, METHOD_GETPOTENTIALLYDECOMPOSEDVARDECL, "clang::ValueDecl::getPotentiallyDecomposedVarDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_VALUEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTARGETPARALLELGENERICLOOPDIRECTIVE, "clang::OMPTargetParallelGenericLoopDirective");
  g.add_edge((CLASS_OMPTARGETPARALLELGENERICLOOPDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETPARALLELGENERICLOOPDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_OMPTHREADPRIVATEDECL, "clang::OMPThreadPrivateDecl");
  g.add_edge((CLASS_OMPTHREADPRIVATEDECL, META_CLASS, META_CLANG_AST_NODE));

  g.add_named_node(CLASS_BINDINGDECL, "clang::BindingDecl");
  g.add_edge((CLASS_BINDINGDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BINDINGDECL, META_SUBCLASS, CLASS_VALUEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETBINDING, "clang::BindingDecl::getBinding"),
      add_named_node(&mut g, METHOD_GETDECOMPOSEDDECL, "clang::BindingDecl::getDecomposedDecl"),
      add_named_node(&mut g, METHOD_GETHOLDINGVAR, "clang::BindingDecl::getHoldingVar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BINDINGDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_USINGDIRECTIVEDECL, "clang::UsingDirectiveDecl");
  g.add_edge((CLASS_USINGDIRECTIVEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_USINGDIRECTIVEDECL, META_SUBCLASS, CLASS_NAMEDDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETQUALIFIERLOC_6, "clang::UsingDirectiveDecl::getQualifierLoc"),
      add_named_node(&mut g, METHOD_GETQUALIFIER_9, "clang::UsingDirectiveDecl::getQualifier"),
      add_named_node(&mut g, METHOD_GETNOMINATEDNAMESPACEASWRITTEN, "clang::UsingDirectiveDecl::getNominatedNamespaceAsWritten"),
      add_named_node(&mut g, METHOD_GETNOMINATEDNAMESPACE, "clang::UsingDirectiveDecl::getNominatedNamespace"),
      add_named_node(&mut g, METHOD_GETCOMMONANCESTOR, "clang::UsingDirectiveDecl::getCommonAncestor"),
      add_named_node(&mut g, METHOD_GETUSINGLOC_3, "clang::UsingDirectiveDecl::getUsingLoc"),
      add_named_node(&mut g, METHOD_GETNAMESPACEKEYLOCATION, "clang::UsingDirectiveDecl::getNamespaceKeyLocation"),
      add_named_node(&mut g, METHOD_GETIDENTLOCATION, "clang::UsingDirectiveDecl::getIdentLocation"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_33, "clang::UsingDirectiveDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_USINGDIRECTIVEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCCATEGORYDECL, "clang::ObjCCategoryDecl");
  g.add_edge((CLASS_OBJCCATEGORYDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCCATEGORYDECL, META_SUBCLASS, CLASS_OBJCCONTAINERDECL));

  g.add_named_node(CLASS_IMPLICITCONCEPTSPECIALIZATIONDECL, "clang::ImplicitConceptSpecializationDecl");
  g.add_edge((CLASS_IMPLICITCONCEPTSPECIALIZATIONDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_IMPLICITCONCEPTSPECIALIZATIONDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTEMPLATEARGUMENTS, "clang::ImplicitConceptSpecializationDecl::getTemplateArguments"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_IMPLICITCONCEPTSPECIALIZATIONDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_VARTEMPLATESPECIALIZATIONDECL, "clang::VarTemplateSpecializationDecl");
  g.add_edge((CLASS_VARTEMPLATESPECIALIZATIONDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_VARTEMPLATESPECIALIZATIONDECL, META_SUBCLASS, CLASS_VARDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSPECIALIZEDTEMPLATE_1, "clang::VarTemplateSpecializationDecl::getSpecializedTemplate"),
      add_named_node(&mut g, METHOD_GETTEMPLATEARGS_1, "clang::VarTemplateSpecializationDecl::getTemplateArgs"),
      add_named_node(&mut g, METHOD_GETTEMPLATEARGSINFO, "clang::VarTemplateSpecializationDecl::getTemplateArgsInfo"),
      add_named_node(&mut g, METHOD_GETSPECIALIZATIONKIND_1, "clang::VarTemplateSpecializationDecl::getSpecializationKind"),
      add_named_node(&mut g, METHOD_ISEXPLICITSPECIALIZATION_1, "clang::VarTemplateSpecializationDecl::isExplicitSpecialization"),
      add_named_node(&mut g, METHOD_ISCLASSSCOPEEXPLICITSPECIALIZATION_1, "clang::VarTemplateSpecializationDecl::isClassScopeExplicitSpecialization"),
      add_named_node(&mut g, METHOD_ISEXPLICITINSTANTIATIONORSPECIALIZATION_1, "clang::VarTemplateSpecializationDecl::isExplicitInstantiationOrSpecialization"),
      add_named_node(&mut g, METHOD_GETPOINTOFINSTANTIATION_3, "clang::VarTemplateSpecializationDecl::getPointOfInstantiation"),
      add_named_node(&mut g, METHOD_GETINSTANTIATEDFROM_1, "clang::VarTemplateSpecializationDecl::getInstantiatedFrom"),
      add_named_node(&mut g, METHOD_GETSPECIALIZEDTEMPLATEORPARTIAL_1, "clang::VarTemplateSpecializationDecl::getSpecializedTemplateOrPartial"),
      add_named_node(&mut g, METHOD_GETTEMPLATEINSTANTIATIONARGS_1, "clang::VarTemplateSpecializationDecl::getTemplateInstantiationArgs"),
      add_named_node(&mut g, METHOD_GETTYPEASWRITTEN_1, "clang::VarTemplateSpecializationDecl::getTypeAsWritten"),
      add_named_node(&mut g, METHOD_GETEXTERNLOC_2, "clang::VarTemplateSpecializationDecl::getExternLoc"),
      add_named_node(&mut g, METHOD_GETTEMPLATEKEYWORDLOC_1, "clang::VarTemplateSpecializationDecl::getTemplateKeywordLoc"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_38, "clang::VarTemplateSpecializationDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_VARTEMPLATESPECIALIZATIONDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPERRORDIRECTIVE, "clang::OMPErrorDirective");
  g.add_edge((CLASS_OMPERRORDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPERRORDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_BASEUSINGDECL, "clang::BaseUsingDecl");
  g.add_edge((CLASS_BASEUSINGDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BASEUSINGDECL, META_SUBCLASS, CLASS_NAMEDDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_SHADOWS, "clang::BaseUsingDecl::shadows"),
      add_named_node(&mut g, METHOD_SHADOW_BEGIN, "clang::BaseUsingDecl::shadow_begin"),
      add_named_node(&mut g, METHOD_SHADOW_END, "clang::BaseUsingDecl::shadow_end"),
      add_named_node(&mut g, METHOD_SHADOW_SIZE, "clang::BaseUsingDecl::shadow_size"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BASEUSINGDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXCONSTRUCTORDECL, "clang::CXXConstructorDecl");
  g.add_edge((CLASS_CXXCONSTRUCTORDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXCONSTRUCTORDECL, META_SUBCLASS, CLASS_CXXMETHODDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETEXPLICITSPECIFIER, "clang::CXXConstructorDecl::getExplicitSpecifier"),
      add_named_node(&mut g, METHOD_ISEXPLICIT, "clang::CXXConstructorDecl::isExplicit"),
      add_named_node(&mut g, METHOD_INITS, "clang::CXXConstructorDecl::inits"),
      add_named_node(&mut g, METHOD_INIT_BEGIN, "clang::CXXConstructorDecl::init_begin"),
      add_named_node(&mut g, METHOD_INIT_END, "clang::CXXConstructorDecl::init_end"),
      add_named_node(&mut g, METHOD_INIT_RBEGIN, "clang::CXXConstructorDecl::init_rbegin"),
      add_named_node(&mut g, METHOD_INIT_REND, "clang::CXXConstructorDecl::init_rend"),
      add_named_node(&mut g, METHOD_GETNUMCTORINITIALIZERS, "clang::CXXConstructorDecl::getNumCtorInitializers"),
      add_named_node(&mut g, METHOD_ISDELEGATINGCONSTRUCTOR, "clang::CXXConstructorDecl::isDelegatingConstructor"),
      add_named_node(&mut g, METHOD_GETTARGETCONSTRUCTOR, "clang::CXXConstructorDecl::getTargetConstructor"),
      add_named_node(&mut g, METHOD_ISDEFAULTCONSTRUCTOR, "clang::CXXConstructorDecl::isDefaultConstructor"),
      add_named_node(&mut g, METHOD_ISCOPYCONSTRUCTOR, "clang::CXXConstructorDecl::isCopyConstructor"),
      add_named_node(&mut g, METHOD_ISMOVECONSTRUCTOR, "clang::CXXConstructorDecl::isMoveConstructor"),
      add_named_node(&mut g, METHOD_ISCOPYORMOVECONSTRUCTOR, "clang::CXXConstructorDecl::isCopyOrMoveConstructor"),
      add_named_node(&mut g, METHOD_ISSPECIALIZATIONCOPYINGOBJECT, "clang::CXXConstructorDecl::isSpecializationCopyingObject"),
      add_named_node(&mut g, METHOD_ISINHERITINGCONSTRUCTOR, "clang::CXXConstructorDecl::isInheritingConstructor"),
      add_named_node(&mut g, METHOD_GETINHERITEDCONSTRUCTOR, "clang::CXXConstructorDecl::getInheritedConstructor"),
      add_named_node(&mut g, METHOD_GETCANONICALDECL, "clang::CXXConstructorDecl::getCanonicalDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXCONSTRUCTORDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TEMPLATETYPEPARMDECL, "clang::TemplateTypeParmDecl");
  g.add_edge((CLASS_TEMPLATETYPEPARMDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TEMPLATETYPEPARMDECL, META_SUBCLASS, CLASS_TYPEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_WASDECLAREDWITHTYPENAME, "clang::TemplateTypeParmDecl::wasDeclaredWithTypename"),
      add_named_node(&mut g, METHOD_GETDEFAULTARGSTORAGE_2, "clang::TemplateTypeParmDecl::getDefaultArgStorage"),
      add_named_node(&mut g, METHOD_HASDEFAULTARGUMENT_2, "clang::TemplateTypeParmDecl::hasDefaultArgument"),
      add_named_node(&mut g, METHOD_GETDEFAULTARGUMENT_2, "clang::TemplateTypeParmDecl::getDefaultArgument"),
      add_named_node(&mut g, METHOD_GETDEFAULTARGUMENTINFO, "clang::TemplateTypeParmDecl::getDefaultArgumentInfo"),
      add_named_node(&mut g, METHOD_GETDEFAULTARGUMENTLOC_2, "clang::TemplateTypeParmDecl::getDefaultArgumentLoc"),
      add_named_node(&mut g, METHOD_DEFAULTARGUMENTWASINHERITED_2, "clang::TemplateTypeParmDecl::defaultArgumentWasInherited"),
      add_named_node(&mut g, METHOD_GETDEPTH_1, "clang::TemplateTypeParmDecl::getDepth"),
      add_named_node(&mut g, METHOD_GETINDEX_3, "clang::TemplateTypeParmDecl::getIndex"),
      add_named_node(&mut g, METHOD_ISPARAMETERPACK_4, "clang::TemplateTypeParmDecl::isParameterPack"),
      add_named_node(&mut g, METHOD_ISPACKEXPANSION_2, "clang::TemplateTypeParmDecl::isPackExpansion"),
      add_named_node(&mut g, METHOD_ISEXPANDEDPARAMETERPACK_2, "clang::TemplateTypeParmDecl::isExpandedParameterPack"),
      add_named_node(&mut g, METHOD_GETNUMEXPANSIONPARAMETERS, "clang::TemplateTypeParmDecl::getNumExpansionParameters"),
      add_named_node(&mut g, METHOD_GETTYPECONSTRAINT, "clang::TemplateTypeParmDecl::getTypeConstraint"),
      add_named_node(&mut g, METHOD_HASTYPECONSTRAINT, "clang::TemplateTypeParmDecl::hasTypeConstraint"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_26, "clang::TemplateTypeParmDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TEMPLATETYPEPARMDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_STMT, "clang::Stmt");
  g.add_edge((CLASS_STMT, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSTMTCLASS, "clang::Stmt::getStmtClass"),
      add_named_node(&mut g, METHOD_GETSTMTCLASSNAME, "clang::Stmt::getStmtClassName"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_49, "clang::Stmt::getSourceRange"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_133, "clang::Stmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_132, "clang::Stmt::getEndLoc"),
      add_named_node(&mut g, METHOD_STRIPLABELLIKESTATEMENTS, "clang::Stmt::stripLabelLikeStatements"),
      add_named_node(&mut g, METHOD_CHILDREN_105, "clang::Stmt::children"),
      add_named_node(&mut g, METHOD_CHILD_BEGIN, "clang::Stmt::child_begin"),
      add_named_node(&mut g, METHOD_CHILD_END, "clang::Stmt::child_end"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_STMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FUNCTIONTEMPLATEDECL, "clang::FunctionTemplateDecl");
  g.add_edge((CLASS_FUNCTIONTEMPLATEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FUNCTIONTEMPLATEDECL, META_SUBCLASS, CLASS_REDECLARABLETEMPLATEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTEMPLATEDDECL_1, "clang::FunctionTemplateDecl::getTemplatedDecl"),
      add_named_node(&mut g, METHOD_ISTHISDECLARATIONADEFINITION_2, "clang::FunctionTemplateDecl::isThisDeclarationADefinition"),
      add_named_node(&mut g, METHOD_GETCANONICALDECL_12, "clang::FunctionTemplateDecl::getCanonicalDecl"),
      add_named_node(&mut g, METHOD_GETPREVIOUSDECL_4, "clang::FunctionTemplateDecl::getPreviousDecl"),
      add_named_node(&mut g, METHOD_GETMOSTRECENTDECL_5, "clang::FunctionTemplateDecl::getMostRecentDecl"),
      add_named_node(&mut g, METHOD_GETINSTANTIATEDFROMMEMBERTEMPLATE_2, "clang::FunctionTemplateDecl::getInstantiatedFromMemberTemplate"),
      add_named_node(&mut g, METHOD_SPECIALIZATIONS_1, "clang::FunctionTemplateDecl::specializations"),
      add_named_node(&mut g, METHOD_SPEC_BEGIN_1, "clang::FunctionTemplateDecl::spec_begin"),
      add_named_node(&mut g, METHOD_SPEC_END_1, "clang::FunctionTemplateDecl::spec_end"),
      add_named_node(&mut g, METHOD_ISABBREVIATED, "clang::FunctionTemplateDecl::isAbbreviated"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FUNCTIONTEMPLATEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ACCESSSPECDECL, "clang::AccessSpecDecl");
  g.add_edge((CLASS_ACCESSSPECDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ACCESSSPECDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETACCESSSPECIFIERLOC, "clang::AccessSpecDecl::getAccessSpecifierLoc"),
      add_named_node(&mut g, METHOD_GETCOLONLOC, "clang::AccessSpecDecl::getColonLoc"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE, "clang::AccessSpecDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ACCESSSPECDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_LIFETIMEEXTENDEDTEMPORARYDECL, "clang::LifetimeExtendedTemporaryDecl");
  g.add_edge((CLASS_LIFETIMEEXTENDEDTEMPORARYDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LIFETIMEEXTENDEDTEMPORARYDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETEXTENDINGDECL, "clang::LifetimeExtendedTemporaryDecl::getExtendingDecl"),
      add_named_node(&mut g, METHOD_GETSTORAGEDURATION, "clang::LifetimeExtendedTemporaryDecl::getStorageDuration"),
      add_named_node(&mut g, METHOD_GETTEMPORARYEXPR, "clang::LifetimeExtendedTemporaryDecl::getTemporaryExpr"),
      add_named_node(&mut g, METHOD_GETMANGLINGNUMBER, "clang::LifetimeExtendedTemporaryDecl::getManglingNumber"),
      add_named_node(&mut g, METHOD_GETVALUE, "clang::LifetimeExtendedTemporaryDecl::getValue"),
      add_named_node(&mut g, METHOD_CHILDRENEXPR, "clang::LifetimeExtendedTemporaryDecl::childrenExpr"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_LIFETIMEEXTENDEDTEMPORARYDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TEMPLATEDECL, "clang::TemplateDecl");
  g.add_edge((CLASS_TEMPLATEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TEMPLATEDECL, META_SUBCLASS, CLASS_NAMEDDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTEMPLATEPARAMETERS_1, "clang::TemplateDecl::getTemplateParameters"),
      add_named_node(&mut g, METHOD_HASASSOCIATEDCONSTRAINTS_1, "clang::TemplateDecl::hasAssociatedConstraints"),
      add_named_node(&mut g, METHOD_GETTEMPLATEDDECL_2, "clang::TemplateDecl::getTemplatedDecl"),
      add_named_node(&mut g, METHOD_ISTYPEALIAS_1, "clang::TemplateDecl::isTypeAlias"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_24, "clang::TemplateDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TEMPLATEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CAPTUREDDECL, "clang::CapturedDecl");
  g.add_edge((CLASS_CAPTUREDDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CAPTUREDDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETBODY_1, "clang::CapturedDecl::getBody"),
      add_named_node(&mut g, METHOD_ISNOTHROW, "clang::CapturedDecl::isNothrow"),
      add_named_node(&mut g, METHOD_GETNUMPARAMS_2, "clang::CapturedDecl::getNumParams"),
      add_named_node(&mut g, METHOD_PARAMETERS_1, "clang::CapturedDecl::parameters"),
      add_named_node(&mut g, METHOD_GETCONTEXTPARAM, "clang::CapturedDecl::getContextParam"),
      add_named_node(&mut g, METHOD_GETCONTEXTPARAMPOSITION, "clang::CapturedDecl::getContextParamPosition"),
      add_named_node(&mut g, METHOD_PARAM_BEGIN_1, "clang::CapturedDecl::param_begin"),
      add_named_node(&mut g, METHOD_PARAM_END_1, "clang::CapturedDecl::param_end"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CAPTUREDDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPEDEFDECL, "clang::TypedefDecl");
  g.add_edge((CLASS_TYPEDEFDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPEDEFDECL, META_SUBCLASS, CLASS_TYPEDEFNAMEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSOURCERANGE_30, "clang::TypedefDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPEDEFDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPREQUIRESDECL, "clang::OMPRequiresDecl");
  g.add_edge((CLASS_OMPREQUIRESDECL, META_CLASS, META_CLANG_AST_NODE));

  g.add_named_node(CLASS_OBJCIMPLEMENTATIONDECL, "clang::ObjCImplementationDecl");
  g.add_edge((CLASS_OBJCIMPLEMENTATIONDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCIMPLEMENTATIONDECL, META_SUBCLASS, CLASS_OBJCIMPLDECL));

  g.add_named_node(CLASS_NAMESPACEALIASDECL, "clang::NamespaceAliasDecl");
  g.add_edge((CLASS_NAMESPACEALIASDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NAMESPACEALIASDECL, META_SUBCLASS, CLASS_NAMEDDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETCANONICALDECL_14, "clang::NamespaceAliasDecl::getCanonicalDecl"),
      add_named_node(&mut g, METHOD_GETQUALIFIERLOC_1, "clang::NamespaceAliasDecl::getQualifierLoc"),
      add_named_node(&mut g, METHOD_GETQUALIFIER_4, "clang::NamespaceAliasDecl::getQualifier"),
      add_named_node(&mut g, METHOD_GETNAMESPACE, "clang::NamespaceAliasDecl::getNamespace"),
      add_named_node(&mut g, METHOD_GETALIASLOC, "clang::NamespaceAliasDecl::getAliasLoc"),
      add_named_node(&mut g, METHOD_GETNAMESPACELOC, "clang::NamespaceAliasDecl::getNamespaceLoc"),
      add_named_node(&mut g, METHOD_GETTARGETNAMELOC, "clang::NamespaceAliasDecl::getTargetNameLoc"),
      add_named_node(&mut g, METHOD_GETALIASEDNAMESPACE, "clang::NamespaceAliasDecl::getAliasedNamespace"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_18, "clang::NamespaceAliasDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NAMESPACEALIASDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_LINKAGESPECDECL, "clang::LinkageSpecDecl");
  g.add_edge((CLASS_LINKAGESPECDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LINKAGESPECDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETLANGUAGE, "clang::LinkageSpecDecl::getLanguage"),
      add_named_node(&mut g, METHOD_HASBRACES_1, "clang::LinkageSpecDecl::hasBraces"),
      add_named_node(&mut g, METHOD_GETEXTERNLOC_1, "clang::LinkageSpecDecl::getExternLoc"),
      add_named_node(&mut g, METHOD_GETRBRACELOC_2, "clang::LinkageSpecDecl::getRBraceLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_2, "clang::LinkageSpecDecl::getEndLoc"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_17, "clang::LinkageSpecDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_LINKAGESPECDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_EMPTYDECL, "clang::EmptyDecl");
  g.add_edge((CLASS_EMPTYDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_EMPTYDECL, META_SUBCLASS, CLASS_DECL));

  g.add_named_node(CLASS_IMPORTDECL, "clang::ImportDecl");
  g.add_edge((CLASS_IMPORTDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_IMPORTDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETIMPORTEDMODULE, "clang::ImportDecl::getImportedModule"),
      add_named_node(&mut g, METHOD_GETIDENTIFIERLOCS, "clang::ImportDecl::getIdentifierLocs"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_15, "clang::ImportDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_IMPORTDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TRANSLATIONUNITDECL, "clang::TranslationUnitDecl");
  g.add_edge((CLASS_TRANSLATIONUNITDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TRANSLATIONUNITDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETASTCONTEXT, "clang::TranslationUnitDecl::getASTContext"),
      add_named_node(&mut g, METHOD_GETANONYMOUSNAMESPACE_1, "clang::TranslationUnitDecl::getAnonymousNamespace"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TRANSLATIONUNITDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_EXPORTDECL, "clang::ExportDecl");
  g.add_edge((CLASS_EXPORTDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_EXPORTDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETEXPORTLOC, "clang::ExportDecl::getExportLoc"),
      add_named_node(&mut g, METHOD_GETRBRACELOC, "clang::ExportDecl::getRBraceLoc"),
      add_named_node(&mut g, METHOD_HASBRACES, "clang::ExportDecl::hasBraces"),
      add_named_node(&mut g, METHOD_GETENDLOC_1, "clang::ExportDecl::getEndLoc"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_9, "clang::ExportDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_EXPORTDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCIMPLDECL, "clang::ObjCImplDecl");
  g.add_edge((CLASS_OBJCIMPLDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCIMPLDECL, META_SUBCLASS, CLASS_OBJCCONTAINERDECL));

  g.add_named_node(CLASS_VARTEMPLATEDECL, "clang::VarTemplateDecl");
  g.add_edge((CLASS_VARTEMPLATEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_VARTEMPLATEDECL, META_SUBCLASS, CLASS_REDECLARABLETEMPLATEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTEMPLATEDDECL_4, "clang::VarTemplateDecl::getTemplatedDecl"),
      add_named_node(&mut g, METHOD_ISTHISDECLARATIONADEFINITION_5, "clang::VarTemplateDecl::isThisDeclarationADefinition"),
      add_named_node(&mut g, METHOD_GETCANONICALDECL_28, "clang::VarTemplateDecl::getCanonicalDecl"),
      add_named_node(&mut g, METHOD_GETPREVIOUSDECL_7, "clang::VarTemplateDecl::getPreviousDecl"),
      add_named_node(&mut g, METHOD_GETMOSTRECENTDECL_8, "clang::VarTemplateDecl::getMostRecentDecl"),
      add_named_node(&mut g, METHOD_GETINSTANTIATEDFROMMEMBERTEMPLATE_5, "clang::VarTemplateDecl::getInstantiatedFromMemberTemplate"),
      add_named_node(&mut g, METHOD_SPECIALIZATIONS_2, "clang::VarTemplateDecl::specializations"),
      add_named_node(&mut g, METHOD_SPEC_BEGIN_2, "clang::VarTemplateDecl::spec_begin"),
      add_named_node(&mut g, METHOD_SPEC_END_2, "clang::VarTemplateDecl::spec_end"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_VARTEMPLATEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FILESCOPEASMDECL, "clang::FileScopeAsmDecl");
  g.add_edge((CLASS_FILESCOPEASMDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FILESCOPEASMDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETASMLOC, "clang::FileScopeAsmDecl::getAsmLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC, "clang::FileScopeAsmDecl::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_11, "clang::FileScopeAsmDecl::getSourceRange"),
      add_named_node(&mut g, METHOD_GETASMSTRING, "clang::FileScopeAsmDecl::getAsmString"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FILESCOPEASMDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FIELDDECL, "clang::FieldDecl");
  g.add_edge((CLASS_FIELDDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FIELDDECL, META_SUBCLASS, CLASS_DECLARATORDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETFIELDINDEX, "clang::FieldDecl::getFieldIndex"),
      add_named_node(&mut g, METHOD_ISMUTABLE, "clang::FieldDecl::isMutable"),
      add_named_node(&mut g, METHOD_ISBITFIELD, "clang::FieldDecl::isBitField"),
      add_named_node(&mut g, METHOD_ISUNNAMEDBITFIELD, "clang::FieldDecl::isUnnamedBitfield"),
      add_named_node(&mut g, METHOD_ISANONYMOUSSTRUCTORUNION, "clang::FieldDecl::isAnonymousStructOrUnion"),
      add_named_node(&mut g, METHOD_GETBITWIDTH, "clang::FieldDecl::getBitWidth"),
      add_named_node(&mut g, METHOD_ISPOTENTIALLYOVERLAPPING, "clang::FieldDecl::isPotentiallyOverlapping"),
      add_named_node(&mut g, METHOD_GETINCLASSINITSTYLE, "clang::FieldDecl::getInClassInitStyle"),
      add_named_node(&mut g, METHOD_HASINCLASSINITIALIZER_1, "clang::FieldDecl::hasInClassInitializer"),
      add_named_node(&mut g, METHOD_HASNONNULLINCLASSINITIALIZER, "clang::FieldDecl::hasNonNullInClassInitializer"),
      add_named_node(&mut g, METHOD_GETINCLASSINITIALIZER, "clang::FieldDecl::getInClassInitializer"),
      add_named_node(&mut g, METHOD_HASCAPTUREDVLATYPE, "clang::FieldDecl::hasCapturedVLAType"),
      add_named_node(&mut g, METHOD_GETCAPTUREDVLATYPE, "clang::FieldDecl::getCapturedVLAType"),
      add_named_node(&mut g, METHOD_GETPARENT_2, "clang::FieldDecl::getParent"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_10, "clang::FieldDecl::getSourceRange"),
      add_named_node(&mut g, METHOD_GETCANONICALDECL_10, "clang::FieldDecl::getCanonicalDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FIELDDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPEALIASTEMPLATEDECL, "clang::TypeAliasTemplateDecl");
  g.add_edge((CLASS_TYPEALIASTEMPLATEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPEALIASTEMPLATEDECL, META_SUBCLASS, CLASS_REDECLARABLETEMPLATEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTEMPLATEDDECL_3, "clang::TypeAliasTemplateDecl::getTemplatedDecl"),
      add_named_node(&mut g, METHOD_GETCANONICALDECL_19, "clang::TypeAliasTemplateDecl::getCanonicalDecl"),
      add_named_node(&mut g, METHOD_GETPREVIOUSDECL_6, "clang::TypeAliasTemplateDecl::getPreviousDecl"),
      add_named_node(&mut g, METHOD_GETINSTANTIATEDFROMMEMBERTEMPLATE_4, "clang::TypeAliasTemplateDecl::getInstantiatedFromMemberTemplate"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPEALIASTEMPLATEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCCATEGORYIMPLDECL, "clang::ObjCCategoryImplDecl");
  g.add_edge((CLASS_OBJCCATEGORYIMPLDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCCATEGORYIMPLDECL, META_SUBCLASS, CLASS_OBJCIMPLDECL));

  g.add_named_node(CLASS_OMPDECLAREREDUCTIONDECL, "clang::OMPDeclareReductionDecl");
  g.add_edge((CLASS_OMPDECLAREREDUCTIONDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPDECLAREREDUCTIONDECL, META_SUBCLASS, CLASS_VALUEDECL));

  g.add_named_node(CLASS_UNRESOLVEDUSINGVALUEDECL, "clang::UnresolvedUsingValueDecl");
  g.add_edge((CLASS_UNRESOLVEDUSINGVALUEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNRESOLVEDUSINGVALUEDECL, META_SUBCLASS, CLASS_VALUEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETUSINGLOC_1, "clang::UnresolvedUsingValueDecl::getUsingLoc"),
      add_named_node(&mut g, METHOD_ISACCESSDECLARATION, "clang::UnresolvedUsingValueDecl::isAccessDeclaration"),
      add_named_node(&mut g, METHOD_GETQUALIFIERLOC_4, "clang::UnresolvedUsingValueDecl::getQualifierLoc"),
      add_named_node(&mut g, METHOD_GETQUALIFIER_7, "clang::UnresolvedUsingValueDecl::getQualifier"),
      add_named_node(&mut g, METHOD_GETNAMEINFO_2, "clang::UnresolvedUsingValueDecl::getNameInfo"),
      add_named_node(&mut g, METHOD_ISPACKEXPANSION_4, "clang::UnresolvedUsingValueDecl::isPackExpansion"),
      add_named_node(&mut g, METHOD_GETELLIPSISLOC_3, "clang::UnresolvedUsingValueDecl::getEllipsisLoc"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_31, "clang::UnresolvedUsingValueDecl::getSourceRange"),
      add_named_node(&mut g, METHOD_GETCANONICALDECL_22, "clang::UnresolvedUsingValueDecl::getCanonicalDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNRESOLVEDUSINGVALUEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ENUMCONSTANTDECL, "clang::EnumConstantDecl");
  g.add_edge((CLASS_ENUMCONSTANTDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ENUMCONSTANTDECL, META_SUBCLASS, CLASS_VALUEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETINITEXPR, "clang::EnumConstantDecl::getInitExpr"),
      add_named_node(&mut g, METHOD_GETINITVAL, "clang::EnumConstantDecl::getInitVal"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_7, "clang::EnumConstantDecl::getSourceRange"),
      add_named_node(&mut g, METHOD_GETCANONICALDECL_8, "clang::EnumConstantDecl::getCanonicalDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ENUMCONSTANTDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NONTYPETEMPLATEPARMDECL, "clang::NonTypeTemplateParmDecl");
  g.add_edge((CLASS_NONTYPETEMPLATEPARMDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NONTYPETEMPLATEPARMDECL, META_SUBCLASS, CLASS_DECLARATORDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSOURCERANGE_20, "clang::NonTypeTemplateParmDecl::getSourceRange"),
      add_named_node(&mut g, METHOD_GETDEFAULTARGSTORAGE, "clang::NonTypeTemplateParmDecl::getDefaultArgStorage"),
      add_named_node(&mut g, METHOD_HASDEFAULTARGUMENT, "clang::NonTypeTemplateParmDecl::hasDefaultArgument"),
      add_named_node(&mut g, METHOD_GETDEFAULTARGUMENT, "clang::NonTypeTemplateParmDecl::getDefaultArgument"),
      add_named_node(&mut g, METHOD_GETDEFAULTARGUMENTLOC, "clang::NonTypeTemplateParmDecl::getDefaultArgumentLoc"),
      add_named_node(&mut g, METHOD_DEFAULTARGUMENTWASINHERITED, "clang::NonTypeTemplateParmDecl::defaultArgumentWasInherited"),
      add_named_node(&mut g, METHOD_ISPARAMETERPACK_2, "clang::NonTypeTemplateParmDecl::isParameterPack"),
      add_named_node(&mut g, METHOD_ISPACKEXPANSION, "clang::NonTypeTemplateParmDecl::isPackExpansion"),
      add_named_node(&mut g, METHOD_ISEXPANDEDPARAMETERPACK, "clang::NonTypeTemplateParmDecl::isExpandedParameterPack"),
      add_named_node(&mut g, METHOD_GETNUMEXPANSIONTYPES, "clang::NonTypeTemplateParmDecl::getNumExpansionTypes"),
      add_named_node(&mut g, METHOD_GETPLACEHOLDERTYPECONSTRAINT, "clang::NonTypeTemplateParmDecl::getPlaceholderTypeConstraint"),
      add_named_node(&mut g, METHOD_HASPLACEHOLDERTYPECONSTRAINT, "clang::NonTypeTemplateParmDecl::hasPlaceholderTypeConstraint"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NONTYPETEMPLATEPARMDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCATDEFSFIELDDECL, "clang::ObjCAtDefsFieldDecl");
  g.add_edge((CLASS_OBJCATDEFSFIELDDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCATDEFSFIELDDECL, META_SUBCLASS, CLASS_FIELDDECL));

  g.add_named_node(CLASS_CXXCONVERSIONDECL, "clang::CXXConversionDecl");
  g.add_edge((CLASS_CXXCONVERSIONDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXCONVERSIONDECL, META_SUBCLASS, CLASS_CXXMETHODDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETEXPLICITSPECIFIER_1, "clang::CXXConversionDecl::getExplicitSpecifier"),
      add_named_node(&mut g, METHOD_ISEXPLICIT_1, "clang::CXXConversionDecl::isExplicit"),
      add_named_node(&mut g, METHOD_GETCONVERSIONTYPE, "clang::CXXConversionDecl::getConversionType"),
      add_named_node(&mut g, METHOD_ISLAMBDATOBLOCKPOINTERCONVERSION, "clang::CXXConversionDecl::isLambdaToBlockPointerConversion"),
      add_named_node(&mut g, METHOD_GETCANONICALDECL_1, "clang::CXXConversionDecl::getCanonicalDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXCONVERSIONDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SEHFINALLYSTMT, "clang::SEHFinallyStmt");
  g.add_edge((CLASS_SEHFINALLYSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SEHFINALLYSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETBEGINLOC_111, "clang::SEHFinallyStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETFINALLYLOC, "clang::SEHFinallyStmt::getFinallyLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_110, "clang::SEHFinallyStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_GETBLOCK_1, "clang::SEHFinallyStmt::getBlock"),
      add_named_node(&mut g, METHOD_CHILDREN_98, "clang::SEHFinallyStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SEHFINALLYSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXDESTRUCTORDECL, "clang::CXXDestructorDecl");
  g.add_edge((CLASS_CXXDESTRUCTORDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXDESTRUCTORDECL, META_SUBCLASS, CLASS_CXXMETHODDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETOPERATORDELETE, "clang::CXXDestructorDecl::getOperatorDelete"),
      add_named_node(&mut g, METHOD_GETOPERATORDELETETHISARG, "clang::CXXDestructorDecl::getOperatorDeleteThisArg"),
      add_named_node(&mut g, METHOD_GETCANONICALDECL_2, "clang::CXXDestructorDecl::getCanonicalDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXDESTRUCTORDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_VARTEMPLATEPARTIALSPECIALIZATIONDECL, "clang::VarTemplatePartialSpecializationDecl");
  g.add_edge((CLASS_VARTEMPLATEPARTIALSPECIALIZATIONDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_VARTEMPLATEPARTIALSPECIALIZATIONDECL, META_SUBCLASS, CLASS_VARTEMPLATESPECIALIZATIONDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTEMPLATEPARAMETERS_2, "clang::VarTemplatePartialSpecializationDecl::getTemplateParameters"),
      add_named_node(&mut g, METHOD_GETTEMPLATEARGSASWRITTEN_1, "clang::VarTemplatePartialSpecializationDecl::getTemplateArgsAsWritten"),
      add_named_node(&mut g, METHOD_HASASSOCIATEDCONSTRAINTS_2, "clang::VarTemplatePartialSpecializationDecl::hasAssociatedConstraints"),
      add_named_node(&mut g, METHOD_GETINSTANTIATEDFROMMEMBER_1, "clang::VarTemplatePartialSpecializationDecl::getInstantiatedFromMember"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_37, "clang::VarTemplatePartialSpecializationDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_VARTEMPLATEPARTIALSPECIALIZATIONDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPEVISIBILITYATTR, "clang::TypeVisibilityAttr");
  g.add_edge((CLASS_TYPEVISIBILITYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPEVISIBILITYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_REINITIALIZESATTR, "clang::ReinitializesAttr");
  g.add_edge((CLASS_REINITIALIZESATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_REINITIALIZESATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_UNINITIALIZEDATTR, "clang::UninitializedAttr");
  g.add_edge((CLASS_UNINITIALIZEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNINITIALIZEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_VECTORCALLATTR, "clang::VectorCallAttr");
  g.add_edge((CLASS_VECTORCALLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_VECTORCALLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CXXNULLPTRLITERALEXPR, "clang::CXXNullPtrLiteralExpr");
  g.add_edge((CLASS_CXXNULLPTRLITERALEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXNULLPTRLITERALEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETBEGINLOC_34, "clang::CXXNullPtrLiteralExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_33, "clang::CXXNullPtrLiteralExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETLOCATION_4, "clang::CXXNullPtrLiteralExpr::getLocation"),
      add_named_node(&mut g, METHOD_CHILDREN_26, "clang::CXXNullPtrLiteralExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXNULLPTRLITERALEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_COMPOUNDSTMT, "clang::CompoundStmt");
  g.add_edge((CLASS_COMPOUNDSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_COMPOUNDSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_BODY_EMPTY, "clang::CompoundStmt::body_empty"),
      add_named_node(&mut g, METHOD_SIZE, "clang::CompoundStmt::size"),
      add_named_node(&mut g, METHOD_HASSTOREDFPFEATURES_3, "clang::CompoundStmt::hasStoredFPFeatures"),
      add_named_node(&mut g, METHOD_GETSTOREDFPFEATURES_3, "clang::CompoundStmt::getStoredFPFeatures"),
      add_named_node(&mut g, METHOD_BODY, "clang::CompoundStmt::body"),
      add_named_node(&mut g, METHOD_BODY_BEGIN, "clang::CompoundStmt::body_begin"),
      add_named_node(&mut g, METHOD_BODY_END, "clang::CompoundStmt::body_end"),
      add_named_node(&mut g, METHOD_BODY_FRONT, "clang::CompoundStmt::body_front"),
      add_named_node(&mut g, METHOD_BODY_BACK, "clang::CompoundStmt::body_back"),
      add_named_node(&mut g, METHOD_BODY_RBEGIN, "clang::CompoundStmt::body_rbegin"),
      add_named_node(&mut g, METHOD_BODY_REND, "clang::CompoundStmt::body_rend"),
      add_named_node(&mut g, METHOD_GETSTMTEXPRRESULT, "clang::CompoundStmt::getStmtExprResult"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_54, "clang::CompoundStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_53, "clang::CompoundStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_GETLBRACLOC, "clang::CompoundStmt::getLBracLoc"),
      add_named_node(&mut g, METHOD_GETRBRACLOC, "clang::CompoundStmt::getRBracLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_44, "clang::CompoundStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_COMPOUNDSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_WORKGROUPSIZEHINTATTR, "clang::WorkGroupSizeHintAttr");
  g.add_edge((CLASS_WORKGROUPSIZEHINTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_WORKGROUPSIZEHINTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_LOADERUNINITIALIZEDATTR, "clang::LoaderUninitializedAttr");
  g.add_edge((CLASS_LOADERUNINITIALIZEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LOADERUNINITIALIZEDATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_PREFERREDNAMEATTR, "clang::PreferredNameAttr");
  g.add_edge((CLASS_PREFERREDNAMEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PREFERREDNAMEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OSCONSUMESTHISATTR, "clang::OSConsumesThisAttr");
  g.add_edge((CLASS_OSCONSUMESTHISATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OSCONSUMESTHISATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_ALIGNNATURALATTR, "clang::AlignNaturalAttr");
  g.add_edge((CLASS_ALIGNNATURALATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ALIGNNATURALATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_UNAVAILABLEATTR, "clang::UnavailableAttr");
  g.add_edge((CLASS_UNAVAILABLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNAVAILABLEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_BUILTINALIASATTR, "clang::BuiltinAliasAttr");
  g.add_edge((CLASS_BUILTINALIASATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BUILTINALIASATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_CONSUMABLEATTR, "clang::ConsumableAttr");
  g.add_edge((CLASS_CONSUMABLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONSUMABLEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_ABITAGATTR, "clang::AbiTagAttr");
  g.add_edge((CLASS_ABITAGATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ABITAGATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_PASCALATTR, "clang::PascalAttr");
  g.add_edge((CLASS_PASCALATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PASCALATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OBJCATTRYSTMT, "clang::ObjCAtTryStmt");
  g.add_edge((CLASS_OBJCATTRYSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCATTRYSTMT, META_SUBCLASS, CLASS_STMT));

  g.add_named_node(CLASS_OMPTARGETTEAMSDISTRIBUTEPARALLELFORSIMDDIRECTIVE, "clang::OMPTargetTeamsDistributeParallelForSimdDirective");
  g.add_edge((CLASS_OMPTARGETTEAMSDISTRIBUTEPARALLELFORSIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETTEAMSDISTRIBUTEPARALLELFORSIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_ALIGNVALUEATTR, "clang::AlignValueAttr");
  g.add_edge((CLASS_ALIGNVALUEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ALIGNVALUEATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_TYPEDECL, "clang::TypeDecl");
  g.add_edge((CLASS_TYPEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPEDECL, META_SUBCLASS, CLASS_NAMEDDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTYPEFORDECL, "clang::TypeDecl::getTypeForDecl"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_3, "clang::TypeDecl::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_29, "clang::TypeDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ENFORCETCBATTR, "clang::EnforceTCBAttr");
  g.add_edge((CLASS_ENFORCETCBATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ENFORCETCBATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_SWIFTVERSIONEDREMOVALATTR, "clang::SwiftVersionedRemovalAttr");
  g.add_edge((CLASS_SWIFTVERSIONEDREMOVALATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTVERSIONEDREMOVALATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_MATRIXTYPE, "clang::MatrixType");
  g.add_edge((CLASS_MATRIXTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MATRIXTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETELEMENTTYPE_4, "clang::MatrixType::getElementType"),
      add_named_node(&mut g, METHOD_ISSUGARED_27, "clang::MatrixType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_27, "clang::MatrixType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MATRIXTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NAKEDATTR, "clang::NakedAttr");
  g.add_edge((CLASS_NAKEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NAKEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_UNSAFEBUFFERUSAGEATTR, "clang::UnsafeBufferUsageAttr");
  g.add_edge((CLASS_UNSAFEBUFFERUSAGEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNSAFEBUFFERUSAGEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_SWIFTVERSIONEDADDITIONATTR, "clang::SwiftVersionedAdditionAttr");
  g.add_edge((CLASS_SWIFTVERSIONEDADDITIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTVERSIONEDADDITIONATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_EXTVECTORTYPELOC, "clang::ExtVectorTypeLoc");
  g.add_edge((CLASS_EXTVECTORTYPELOC, META_CLASS, META_CLANG_AST_NODE));

  g.add_named_node(CLASS_OBJCNONLAZYCLASSATTR, "clang::ObjCNonLazyClassAttr");
  g.add_edge((CLASS_OBJCNONLAZYCLASSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCNONLAZYCLASSATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_OMPPARALLELGENERICLOOPDIRECTIVE, "clang::OMPParallelGenericLoopDirective");
  g.add_edge((CLASS_OMPPARALLELGENERICLOOPDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPPARALLELGENERICLOOPDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_EMPTYBASESATTR, "clang::EmptyBasesAttr");
  g.add_edge((CLASS_EMPTYBASESATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_EMPTYBASESATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_WEAKREFATTR, "clang::WeakRefAttr");
  g.add_edge((CLASS_WEAKREFATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_WEAKREFATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_SUPPRESSATTR, "clang::SuppressAttr");
  g.add_edge((CLASS_SUPPRESSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SUPPRESSATTR, META_SUBCLASS, CLASS_DECLORSTMTATTR));

  g.add_named_node(CLASS_DECAYEDTYPELOC, "clang::DecayedTypeLoc");
  g.add_edge((CLASS_DECAYEDTYPELOC, META_CLASS, META_CLANG_AST_NODE));

  g.add_named_node(CLASS_C11NORETURNATTR, "clang::C11NoReturnAttr");
  g.add_edge((CLASS_C11NORETURNATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_C11NORETURNATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CALLEDONCEATTR, "clang::CalledOnceAttr");
  g.add_edge((CLASS_CALLEDONCEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CALLEDONCEATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_SUBSTTEMPLATETYPEPARMPACKTYPELOC, "clang::SubstTemplateTypeParmPackTypeLoc");
  g.add_edge((CLASS_SUBSTTEMPLATETYPEPARMPACKTYPELOC, META_CLASS, META_CLANG_AST_NODE));

  g.add_named_node(CLASS_INDIRECTFIELDDECL, "clang::IndirectFieldDecl");
  g.add_edge((CLASS_INDIRECTFIELDDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_INDIRECTFIELDDECL, META_SUBCLASS, CLASS_VALUEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_CHAIN, "clang::IndirectFieldDecl::chain"),
      add_named_node(&mut g, METHOD_CHAIN_BEGIN, "clang::IndirectFieldDecl::chain_begin"),
      add_named_node(&mut g, METHOD_CHAIN_END, "clang::IndirectFieldDecl::chain_end"),
      add_named_node(&mut g, METHOD_GETCHAININGSIZE, "clang::IndirectFieldDecl::getChainingSize"),
      add_named_node(&mut g, METHOD_GETANONFIELD, "clang::IndirectFieldDecl::getAnonField"),
      add_named_node(&mut g, METHOD_GETVARDECL, "clang::IndirectFieldDecl::getVarDecl"),
      add_named_node(&mut g, METHOD_GETCANONICALDECL_13, "clang::IndirectFieldDecl::getCanonicalDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_INDIRECTFIELDDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ASSERTCAPABILITYATTR, "clang::AssertCapabilityAttr");
  g.add_edge((CLASS_ASSERTCAPABILITYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ASSERTCAPABILITYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_WEBASSEMBLYIMPORTMODULEATTR, "clang::WebAssemblyImportModuleAttr");
  g.add_edge((CLASS_WEBASSEMBLYIMPORTMODULEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_WEBASSEMBLYIMPORTMODULEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OMPTASKYIELDDIRECTIVE, "clang::OMPTaskyieldDirective");
  g.add_edge((CLASS_OMPTASKYIELDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTASKYIELDDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_SWIFTINDIRECTRESULTATTR, "clang::SwiftIndirectResultAttr");
  g.add_edge((CLASS_SWIFTINDIRECTRESULTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTINDIRECTRESULTATTR, META_SUBCLASS, CLASS_PARAMETERABIATTR));

  g.add_named_node(CLASS_MACROQUALIFIEDTYPE, "clang::MacroQualifiedType");
  g.add_edge((CLASS_MACROQUALIFIEDTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MACROQUALIFIEDTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETMACROIDENTIFIER, "clang::MacroQualifiedType::getMacroIdentifier"),
      add_named_node(&mut g, METHOD_GETUNDERLYINGTYPE_1, "clang::MacroQualifiedType::getUnderlyingType"),
      add_named_node(&mut g, METHOD_GETMODIFIEDTYPE_1, "clang::MacroQualifiedType::getModifiedType"),
      add_named_node(&mut g, METHOD_ISSUGARED_26, "clang::MacroQualifiedType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_26, "clang::MacroQualifiedType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MACROQUALIFIEDTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCDESIGNATEDINITIALIZERATTR, "clang::ObjCDesignatedInitializerAttr");
  g.add_edge((CLASS_OBJCDESIGNATEDINITIALIZERATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCDESIGNATEDINITIALIZERATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_STRICTGUARDSTACKCHECKATTR, "clang::StrictGuardStackCheckAttr");
  g.add_edge((CLASS_STRICTGUARDSTACKCHECKATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_STRICTGUARDSTACKCHECKATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_IMAGINARYLITERAL, "clang::ImaginaryLiteral");
  g.add_edge((CLASS_IMAGINARYLITERAL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_IMAGINARYLITERAL, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSUBEXPR_6, "clang::ImaginaryLiteral::getSubExpr"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_83, "clang::ImaginaryLiteral::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_82, "clang::ImaginaryLiteral::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_72, "clang::ImaginaryLiteral::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_IMAGINARYLITERAL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SCOPEDLOCKABLEATTR, "clang::ScopedLockableAttr");
  g.add_edge((CLASS_SCOPEDLOCKABLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SCOPEDLOCKABLEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_RECORDDECL, "clang::RecordDecl");
  g.add_edge((CLASS_RECORDDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RECORDDECL, META_SUBCLASS, CLASS_TAGDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETPREVIOUSDECL_5, "clang::RecordDecl::getPreviousDecl"),
      add_named_node(&mut g, METHOD_GETMOSTRECENTDECL_7, "clang::RecordDecl::getMostRecentDecl"),
      add_named_node(&mut g, METHOD_HASFLEXIBLEARRAYMEMBER, "clang::RecordDecl::hasFlexibleArrayMember"),
      add_named_node(&mut g, METHOD_ISANONYMOUSSTRUCTORUNION_1, "clang::RecordDecl::isAnonymousStructOrUnion"),
      add_named_node(&mut g, METHOD_HASOBJECTMEMBER, "clang::RecordDecl::hasObjectMember"),
      add_named_node(&mut g, METHOD_HASVOLATILEMEMBER, "clang::RecordDecl::hasVolatileMember"),
      add_named_node(&mut g, METHOD_HASLOADEDFIELDSFROMEXTERNALSTORAGE, "clang::RecordDecl::hasLoadedFieldsFromExternalStorage"),
      add_named_node(&mut g, METHOD_ISNONTRIVIALTOPRIMITIVEDEFAULTINITIALIZE, "clang::RecordDecl::isNonTrivialToPrimitiveDefaultInitialize"),
      add_named_node(&mut g, METHOD_ISNONTRIVIALTOPRIMITIVECOPY, "clang::RecordDecl::isNonTrivialToPrimitiveCopy"),
      add_named_node(&mut g, METHOD_ISNONTRIVIALTOPRIMITIVEDESTROY, "clang::RecordDecl::isNonTrivialToPrimitiveDestroy"),
      add_named_node(&mut g, METHOD_HASNONTRIVIALTOPRIMITIVEDEFAULTINITIALIZECUNION, "clang::RecordDecl::hasNonTrivialToPrimitiveDefaultInitializeCUnion"),
      add_named_node(&mut g, METHOD_HASNONTRIVIALTOPRIMITIVEDESTRUCTCUNION, "clang::RecordDecl::hasNonTrivialToPrimitiveDestructCUnion"),
      add_named_node(&mut g, METHOD_HASNONTRIVIALTOPRIMITIVECOPYCUNION, "clang::RecordDecl::hasNonTrivialToPrimitiveCopyCUnion"),
      add_named_node(&mut g, METHOD_CANPASSINREGISTERS, "clang::RecordDecl::canPassInRegisters"),
      add_named_node(&mut g, METHOD_GETARGPASSINGRESTRICTIONS, "clang::RecordDecl::getArgPassingRestrictions"),
      add_named_node(&mut g, METHOD_ISPARAMDESTROYEDINCALLEE, "clang::RecordDecl::isParamDestroyedInCallee"),
      add_named_node(&mut g, METHOD_ISRANDOMIZED, "clang::RecordDecl::isRandomized"),
      add_named_node(&mut g, METHOD_ISINJECTEDCLASSNAME, "clang::RecordDecl::isInjectedClassName"),
      add_named_node(&mut g, METHOD_ISLAMBDA_1, "clang::RecordDecl::isLambda"),
      add_named_node(&mut g, METHOD_ISCAPTUREDRECORD, "clang::RecordDecl::isCapturedRecord"),
      add_named_node(&mut g, METHOD_GETDEFINITION_3, "clang::RecordDecl::getDefinition"),
      add_named_node(&mut g, METHOD_ISORCONTAINSUNION, "clang::RecordDecl::isOrContainsUnion"),
      add_named_node(&mut g, METHOD_FIELDS, "clang::RecordDecl::fields"),
      add_named_node(&mut g, METHOD_FIELD_BEGIN, "clang::RecordDecl::field_begin"),
      add_named_node(&mut g, METHOD_FIELD_END, "clang::RecordDecl::field_end"),
      add_named_node(&mut g, METHOD_FIELD_EMPTY, "clang::RecordDecl::field_empty"),
      add_named_node(&mut g, METHOD_FINDFIRSTNAMEDDATAMEMBER, "clang::RecordDecl::findFirstNamedDataMember"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_RECORDDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_XRAYLOGARGSATTR, "clang::XRayLogArgsAttr");
  g.add_edge((CLASS_XRAYLOGARGSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_XRAYLOGARGSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_SWIFTATTRATTR, "clang::SwiftAttrAttr");
  g.add_edge((CLASS_SWIFTATTRATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTATTRATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_POINTERTYPE, "clang::PointerType");
  g.add_edge((CLASS_POINTERTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_POINTERTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETPOINTEETYPE_4, "clang::PointerType::getPointeeType"),
      add_named_node(&mut g, METHOD_ISSUGARED_32, "clang::PointerType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_32, "clang::PointerType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_POINTERTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ARTIFICIALATTR, "clang::ArtificialAttr");
  g.add_edge((CLASS_ARTIFICIALATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARTIFICIALATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_USINGPACKDECL, "clang::UsingPackDecl");
  g.add_edge((CLASS_USINGPACKDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_USINGPACKDECL, META_SUBCLASS, CLASS_NAMEDDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETINSTANTIATEDFROMUSINGDECL, "clang::UsingPackDecl::getInstantiatedFromUsingDecl"),
      add_named_node(&mut g, METHOD_EXPANSIONS, "clang::UsingPackDecl::expansions"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_35, "clang::UsingPackDecl::getSourceRange"),
      add_named_node(&mut g, METHOD_GETCANONICALDECL_25, "clang::UsingPackDecl::getCanonicalDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_USINGPACKDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NSERRORDOMAINATTR, "clang::NSErrorDomainAttr");
  g.add_edge((CLASS_NSERRORDOMAINATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NSERRORDOMAINATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_RELEASEHANDLEATTR, "clang::ReleaseHandleAttr");
  g.add_edge((CLASS_RELEASEHANDLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RELEASEHANDLEATTR, META_SUBCLASS, CLASS_INHERITABLEPARAMATTR));

  g.add_named_node(CLASS_PARMVARDECL, "clang::ParmVarDecl");
  g.add_edge((CLASS_PARMVARDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PARMVARDECL, META_SUBCLASS, CLASS_VARDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSOURCERANGE_21, "clang::ParmVarDecl::getSourceRange"),
      add_named_node(&mut g, METHOD_ISOBJCMETHODPARAMETER, "clang::ParmVarDecl::isObjCMethodParameter"),
      add_named_node(&mut g, METHOD_ISDESTROYEDINCALLEE, "clang::ParmVarDecl::isDestroyedInCallee"),
      add_named_node(&mut g, METHOD_GETFUNCTIONSCOPEDEPTH, "clang::ParmVarDecl::getFunctionScopeDepth"),
      add_named_node(&mut g, METHOD_GETFUNCTIONSCOPEINDEX, "clang::ParmVarDecl::getFunctionScopeIndex"),
      add_named_node(&mut g, METHOD_GETOBJCDECLQUALIFIER, "clang::ParmVarDecl::getObjCDeclQualifier"),
      add_named_node(&mut g, METHOD_ISKNRPROMOTED, "clang::ParmVarDecl::isKNRPromoted"),
      add_named_node(&mut g, METHOD_ISEXPLICITOBJECTPARAMETER, "clang::ParmVarDecl::isExplicitObjectParameter"),
      add_named_node(&mut g, METHOD_GETEXPLICITOBJECTPARAMTHISLOC, "clang::ParmVarDecl::getExplicitObjectParamThisLoc"),
      add_named_node(&mut g, METHOD_GETDEFAULTARG, "clang::ParmVarDecl::getDefaultArg"),
      add_named_node(&mut g, METHOD_GETDEFAULTARGRANGE, "clang::ParmVarDecl::getDefaultArgRange"),
      add_named_node(&mut g, METHOD_GETUNINSTANTIATEDDEFAULTARG, "clang::ParmVarDecl::getUninstantiatedDefaultArg"),
      add_named_node(&mut g, METHOD_HASDEFAULTARG, "clang::ParmVarDecl::hasDefaultArg"),
      add_named_node(&mut g, METHOD_HASUNPARSEDDEFAULTARG, "clang::ParmVarDecl::hasUnparsedDefaultArg"),
      add_named_node(&mut g, METHOD_HASUNINSTANTIATEDDEFAULTARG, "clang::ParmVarDecl::hasUninstantiatedDefaultArg"),
      add_named_node(&mut g, METHOD_HASINHERITEDDEFAULTARG, "clang::ParmVarDecl::hasInheritedDefaultArg"),
      add_named_node(&mut g, METHOD_GETORIGINALTYPE_1, "clang::ParmVarDecl::getOriginalType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PARMVARDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_VECTYPEHINTATTR, "clang::VecTypeHintAttr");
  g.add_edge((CLASS_VECTYPEHINTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_VECTYPEHINTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_SWIFTASYNCCONTEXTATTR, "clang::SwiftAsyncContextAttr");
  g.add_edge((CLASS_SWIFTASYNCCONTEXTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTASYNCCONTEXTATTR, META_SUBCLASS, CLASS_PARAMETERABIATTR));

  g.add_named_node(CLASS_BTFDECLTAGATTR, "clang::BTFDeclTagAttr");
  g.add_edge((CLASS_BTFDECLTAGATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BTFDECLTAGATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_WARNUNUSEDRESULTATTR, "clang::WarnUnusedResultAttr");
  g.add_edge((CLASS_WARNUNUSEDRESULTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_WARNUNUSEDRESULTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CHARACTERLITERAL, "clang::CharacterLiteral");
  g.add_edge((CLASS_CHARACTERLITERAL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CHARACTERLITERAL, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETLOCATION_6, "clang::CharacterLiteral::getLocation"),
      add_named_node(&mut g, METHOD_GETKIND_3, "clang::CharacterLiteral::getKind"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_51, "clang::CharacterLiteral::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_50, "clang::CharacterLiteral::getEndLoc"),
      add_named_node(&mut g, METHOD_GETVALUE_6, "clang::CharacterLiteral::getValue"),
      add_named_node(&mut g, METHOD_CHILDREN_41, "clang::CharacterLiteral::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CHARACTERLITERAL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPE, "clang::Type");
  g.add_edge((CLASS_TYPE, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTYPECLASS, "clang::Type::getTypeClass"),
      add_named_node(&mut g, METHOD_ISFROMAST, "clang::Type::isFromAST"),
      add_named_node(&mut g, METHOD_CONTAINSUNEXPANDEDPARAMETERPACK, "clang::Type::containsUnexpandedParameterPack"),
      add_named_node(&mut g, METHOD_ISCANONICALUNQUALIFIED, "clang::Type::isCanonicalUnqualified"),
      add_named_node(&mut g, METHOD_GETLOCALLYUNQUALIFIEDSINGLESTEPDESUGAREDTYPE, "clang::Type::getLocallyUnqualifiedSingleStepDesugaredType"),
      add_named_node(&mut g, METHOD_ISSIZELESSTYPE, "clang::Type::isSizelessType"),
      add_named_node(&mut g, METHOD_ISSIZELESSBUILTINTYPE, "clang::Type::isSizelessBuiltinType"),
      add_named_node(&mut g, METHOD_ISSIZELESSVECTORTYPE, "clang::Type::isSizelessVectorType"),
      add_named_node(&mut g, METHOD_ISSVESIZELESSBUILTINTYPE, "clang::Type::isSVESizelessBuiltinType"),
      add_named_node(&mut g, METHOD_ISRVVSIZELESSBUILTINTYPE, "clang::Type::isRVVSizelessBuiltinType"),
      add_named_node(&mut g, METHOD_ISWEBASSEMBLYEXTERNREFTYPE, "clang::Type::isWebAssemblyExternrefType"),
      add_named_node(&mut g, METHOD_ISWEBASSEMBLYTABLETYPE, "clang::Type::isWebAssemblyTableType"),
      add_named_node(&mut g, METHOD_ISSVEVLSBUILTINTYPE, "clang::Type::isSveVLSBuiltinType"),
      add_named_node(&mut g, METHOD_ISRVVVLSBUILTINTYPE, "clang::Type::isRVVVLSBuiltinType"),
      add_named_node(&mut g, METHOD_ISINCOMPLETEOROBJECTTYPE, "clang::Type::isIncompleteOrObjectType"),
      add_named_node(&mut g, METHOD_ISOBJECTTYPE, "clang::Type::isObjectType"),
      add_named_node(&mut g, METHOD_ISSTRUCTURALTYPE, "clang::Type::isStructuralType"),
      add_named_node(&mut g, METHOD_ISSTANDARDLAYOUTTYPE, "clang::Type::isStandardLayoutType"),
      add_named_node(&mut g, METHOD_ISBUILTINTYPE, "clang::Type::isBuiltinType"),
      add_named_node(&mut g, METHOD_ISPLACEHOLDERTYPE_1, "clang::Type::isPlaceholderType"),
      add_named_node(&mut g, METHOD_GETASPLACEHOLDERTYPE, "clang::Type::getAsPlaceholderType"),
      add_named_node(&mut g, METHOD_ISNONOVERLOADPLACEHOLDERTYPE_1, "clang::Type::isNonOverloadPlaceholderType"),
      add_named_node(&mut g, METHOD_ISINTEGERTYPE, "clang::Type::isIntegerType"),
      add_named_node(&mut g, METHOD_ISENUMERALTYPE, "clang::Type::isEnumeralType"),
      add_named_node(&mut g, METHOD_ISSCOPEDENUMERALTYPE, "clang::Type::isScopedEnumeralType"),
      add_named_node(&mut g, METHOD_ISBOOLEANTYPE, "clang::Type::isBooleanType"),
      add_named_node(&mut g, METHOD_ISCHARTYPE, "clang::Type::isCharType"),
      add_named_node(&mut g, METHOD_ISWIDECHARTYPE, "clang::Type::isWideCharType"),
      add_named_node(&mut g, METHOD_ISCHAR8TYPE, "clang::Type::isChar8Type"),
      add_named_node(&mut g, METHOD_ISCHAR16TYPE, "clang::Type::isChar16Type"),
      add_named_node(&mut g, METHOD_ISCHAR32TYPE, "clang::Type::isChar32Type"),
      add_named_node(&mut g, METHOD_ISANYCHARACTERTYPE, "clang::Type::isAnyCharacterType"),
      add_named_node(&mut g, METHOD_ISINTEGRALORENUMERATIONTYPE, "clang::Type::isIntegralOrEnumerationType"),
      add_named_node(&mut g, METHOD_ISINTEGRALORUNSCOPEDENUMERATIONTYPE, "clang::Type::isIntegralOrUnscopedEnumerationType"),
      add_named_node(&mut g, METHOD_ISUNSCOPEDENUMERATIONTYPE, "clang::Type::isUnscopedEnumerationType"),
      add_named_node(&mut g, METHOD_ISREALFLOATINGTYPE, "clang::Type::isRealFloatingType"),
      add_named_node(&mut g, METHOD_ISCOMPLEXTYPE, "clang::Type::isComplexType"),
      add_named_node(&mut g, METHOD_ISANYCOMPLEXTYPE, "clang::Type::isAnyComplexType"),
      add_named_node(&mut g, METHOD_ISFLOATINGTYPE, "clang::Type::isFloatingType"),
      add_named_node(&mut g, METHOD_ISHALFTYPE, "clang::Type::isHalfType"),
      add_named_node(&mut g, METHOD_ISFLOAT16TYPE, "clang::Type::isFloat16Type"),
      add_named_node(&mut g, METHOD_ISBFLOAT16TYPE, "clang::Type::isBFloat16Type"),
      add_named_node(&mut g, METHOD_ISFLOAT128TYPE, "clang::Type::isFloat128Type"),
      add_named_node(&mut g, METHOD_ISIBM128TYPE, "clang::Type::isIbm128Type"),
      add_named_node(&mut g, METHOD_ISREALTYPE, "clang::Type::isRealType"),
      add_named_node(&mut g, METHOD_ISARITHMETICTYPE, "clang::Type::isArithmeticType"),
      add_named_node(&mut g, METHOD_ISVOIDTYPE, "clang::Type::isVoidType"),
      add_named_node(&mut g, METHOD_ISSCALARTYPE, "clang::Type::isScalarType"),
      add_named_node(&mut g, METHOD_ISAGGREGATETYPE, "clang::Type::isAggregateType"),
      add_named_node(&mut g, METHOD_ISFUNDAMENTALTYPE, "clang::Type::isFundamentalType"),
      add_named_node(&mut g, METHOD_ISCOMPOUNDTYPE, "clang::Type::isCompoundType"),
      add_named_node(&mut g, METHOD_ISFUNCTIONTYPE, "clang::Type::isFunctionType"),
      add_named_node(&mut g, METHOD_ISFUNCTIONNOPROTOTYPE, "clang::Type::isFunctionNoProtoType"),
      add_named_node(&mut g, METHOD_ISFUNCTIONPROTOTYPE, "clang::Type::isFunctionProtoType"),
      add_named_node(&mut g, METHOD_ISPOINTERTYPE, "clang::Type::isPointerType"),
      add_named_node(&mut g, METHOD_ISANYPOINTERTYPE, "clang::Type::isAnyPointerType"),
      add_named_node(&mut g, METHOD_ISBLOCKPOINTERTYPE, "clang::Type::isBlockPointerType"),
      add_named_node(&mut g, METHOD_ISVOIDPOINTERTYPE, "clang::Type::isVoidPointerType"),
      add_named_node(&mut g, METHOD_ISREFERENCETYPE, "clang::Type::isReferenceType"),
      add_named_node(&mut g, METHOD_ISLVALUEREFERENCETYPE, "clang::Type::isLValueReferenceType"),
      add_named_node(&mut g, METHOD_ISRVALUEREFERENCETYPE, "clang::Type::isRValueReferenceType"),
      add_named_node(&mut g, METHOD_ISOBJECTPOINTERTYPE, "clang::Type::isObjectPointerType"),
      add_named_node(&mut g, METHOD_ISFUNCTIONPOINTERTYPE_1, "clang::Type::isFunctionPointerType"),
      add_named_node(&mut g, METHOD_ISFUNCTIONREFERENCETYPE, "clang::Type::isFunctionReferenceType"),
      add_named_node(&mut g, METHOD_ISMEMBERPOINTERTYPE, "clang::Type::isMemberPointerType"),
      add_named_node(&mut g, METHOD_ISMEMBERFUNCTIONPOINTERTYPE, "clang::Type::isMemberFunctionPointerType"),
      add_named_node(&mut g, METHOD_ISMEMBERDATAPOINTERTYPE, "clang::Type::isMemberDataPointerType"),
      add_named_node(&mut g, METHOD_ISARRAYTYPE, "clang::Type::isArrayType"),
      add_named_node(&mut g, METHOD_ISCONSTANTARRAYTYPE, "clang::Type::isConstantArrayType"),
      add_named_node(&mut g, METHOD_ISINCOMPLETEARRAYTYPE, "clang::Type::isIncompleteArrayType"),
      add_named_node(&mut g, METHOD_ISVARIABLEARRAYTYPE, "clang::Type::isVariableArrayType"),
      add_named_node(&mut g, METHOD_ISDEPENDENTSIZEDARRAYTYPE, "clang::Type::isDependentSizedArrayType"),
      add_named_node(&mut g, METHOD_ISRECORDTYPE, "clang::Type::isRecordType"),
      add_named_node(&mut g, METHOD_ISCLASSTYPE, "clang::Type::isClassType"),
      add_named_node(&mut g, METHOD_ISSTRUCTURETYPE, "clang::Type::isStructureType"),
      add_named_node(&mut g, METHOD_ISOBJCBOXABLERECORDTYPE, "clang::Type::isObjCBoxableRecordType"),
      add_named_node(&mut g, METHOD_ISINTERFACETYPE, "clang::Type::isInterfaceType"),
      add_named_node(&mut g, METHOD_ISSTRUCTUREORCLASSTYPE, "clang::Type::isStructureOrClassType"),
      add_named_node(&mut g, METHOD_ISUNIONTYPE, "clang::Type::isUnionType"),
      add_named_node(&mut g, METHOD_ISCOMPLEXINTEGERTYPE, "clang::Type::isComplexIntegerType"),
      add_named_node(&mut g, METHOD_ISVECTORTYPE, "clang::Type::isVectorType"),
      add_named_node(&mut g, METHOD_ISEXTVECTORTYPE, "clang::Type::isExtVectorType"),
      add_named_node(&mut g, METHOD_ISEXTVECTORBOOLTYPE, "clang::Type::isExtVectorBoolType"),
      add_named_node(&mut g, METHOD_ISMATRIXTYPE, "clang::Type::isMatrixType"),
      add_named_node(&mut g, METHOD_ISCONSTANTMATRIXTYPE, "clang::Type::isConstantMatrixType"),
      add_named_node(&mut g, METHOD_ISDEPENDENTADDRESSSPACETYPE, "clang::Type::isDependentAddressSpaceType"),
      add_named_node(&mut g, METHOD_ISOBJCOBJECTPOINTERTYPE, "clang::Type::isObjCObjectPointerType"),
      add_named_node(&mut g, METHOD_ISOBJCRETAINABLETYPE, "clang::Type::isObjCRetainableType"),
      add_named_node(&mut g, METHOD_ISOBJCLIFETIMETYPE, "clang::Type::isObjCLifetimeType"),
      add_named_node(&mut g, METHOD_ISOBJCINDIRECTLIFETIMETYPE, "clang::Type::isObjCIndirectLifetimeType"),
      add_named_node(&mut g, METHOD_ISOBJCNSOBJECTTYPE, "clang::Type::isObjCNSObjectType"),
      add_named_node(&mut g, METHOD_ISOBJCINDEPENDENTCLASSTYPE, "clang::Type::isObjCIndependentClassType"),
      add_named_node(&mut g, METHOD_ISOBJCOBJECTTYPE, "clang::Type::isObjCObjectType"),
      add_named_node(&mut g, METHOD_ISOBJCQUALIFIEDINTERFACETYPE, "clang::Type::isObjCQualifiedInterfaceType"),
      add_named_node(&mut g, METHOD_ISOBJCQUALIFIEDIDTYPE, "clang::Type::isObjCQualifiedIdType"),
      add_named_node(&mut g, METHOD_ISOBJCQUALIFIEDCLASSTYPE, "clang::Type::isObjCQualifiedClassType"),
      add_named_node(&mut g, METHOD_ISOBJCOBJECTORINTERFACETYPE, "clang::Type::isObjCObjectOrInterfaceType"),
      add_named_node(&mut g, METHOD_ISOBJCIDTYPE, "clang::Type::isObjCIdType"),
      add_named_node(&mut g, METHOD_ISDECLTYPETYPE, "clang::Type::isDecltypeType"),
      add_named_node(&mut g, METHOD_ISOBJCINERTUNSAFEUNRETAINEDTYPE, "clang::Type::isObjCInertUnsafeUnretainedType"),
      add_named_node(&mut g, METHOD_ISOBJCCLASSTYPE, "clang::Type::isObjCClassType"),
      add_named_node(&mut g, METHOD_ISOBJCCLASSORCLASSKINDOFTYPE, "clang::Type::isObjCClassOrClassKindOfType"),
      add_named_node(&mut g, METHOD_ISOBJCSELTYPE, "clang::Type::isObjCSelType"),
      add_named_node(&mut g, METHOD_ISOBJCBUILTINTYPE, "clang::Type::isObjCBuiltinType"),
      add_named_node(&mut g, METHOD_ISOBJCARCBRIDGABLETYPE, "clang::Type::isObjCARCBridgableType"),
      add_named_node(&mut g, METHOD_ISCARCBRIDGABLETYPE, "clang::Type::isCARCBridgableType"),
      add_named_node(&mut g, METHOD_ISTEMPLATETYPEPARMTYPE, "clang::Type::isTemplateTypeParmType"),
      add_named_node(&mut g, METHOD_ISNULLPTRTYPE, "clang::Type::isNullPtrType"),
      add_named_node(&mut g, METHOD_ISNOTHROWT, "clang::Type::isNothrowT"),
      add_named_node(&mut g, METHOD_ISALIGNVALT, "clang::Type::isAlignValT"),
      add_named_node(&mut g, METHOD_ISSTDBYTETYPE, "clang::Type::isStdByteType"),
      add_named_node(&mut g, METHOD_ISATOMICTYPE, "clang::Type::isAtomicType"),
      add_named_node(&mut g, METHOD_ISUNDEDUCEDAUTOTYPE, "clang::Type::isUndeducedAutoType"),
      add_named_node(&mut g, METHOD_ISTYPEDEFNAMETYPE, "clang::Type::isTypedefNameType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE1DROTYPE, "clang::Type::isOCLImage1dROType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE1DARRAYROTYPE, "clang::Type::isOCLImage1dArrayROType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE1DBUFFERROTYPE, "clang::Type::isOCLImage1dBufferROType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE2DROTYPE, "clang::Type::isOCLImage2dROType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE2DARRAYROTYPE, "clang::Type::isOCLImage2dArrayROType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE2DDEPTHROTYPE, "clang::Type::isOCLImage2dDepthROType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE2DARRAYDEPTHROTYPE, "clang::Type::isOCLImage2dArrayDepthROType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE2DMSAAROTYPE, "clang::Type::isOCLImage2dMSAAROType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE2DARRAYMSAAROTYPE, "clang::Type::isOCLImage2dArrayMSAAROType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE2DMSAADEPTHROTYPE, "clang::Type::isOCLImage2dMSAADepthROType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE2DARRAYMSAADEPTHROTYPE, "clang::Type::isOCLImage2dArrayMSAADepthROType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE3DROTYPE, "clang::Type::isOCLImage3dROType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE1DWOTYPE, "clang::Type::isOCLImage1dWOType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE1DARRAYWOTYPE, "clang::Type::isOCLImage1dArrayWOType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE1DBUFFERWOTYPE, "clang::Type::isOCLImage1dBufferWOType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE2DWOTYPE, "clang::Type::isOCLImage2dWOType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE2DARRAYWOTYPE, "clang::Type::isOCLImage2dArrayWOType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE2DDEPTHWOTYPE, "clang::Type::isOCLImage2dDepthWOType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE2DARRAYDEPTHWOTYPE, "clang::Type::isOCLImage2dArrayDepthWOType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE2DMSAAWOTYPE, "clang::Type::isOCLImage2dMSAAWOType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE2DARRAYMSAAWOTYPE, "clang::Type::isOCLImage2dArrayMSAAWOType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE2DMSAADEPTHWOTYPE, "clang::Type::isOCLImage2dMSAADepthWOType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE2DARRAYMSAADEPTHWOTYPE, "clang::Type::isOCLImage2dArrayMSAADepthWOType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE3DWOTYPE, "clang::Type::isOCLImage3dWOType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE1DRWTYPE, "clang::Type::isOCLImage1dRWType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE1DARRAYRWTYPE, "clang::Type::isOCLImage1dArrayRWType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE1DBUFFERRWTYPE, "clang::Type::isOCLImage1dBufferRWType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE2DRWTYPE, "clang::Type::isOCLImage2dRWType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE2DARRAYRWTYPE, "clang::Type::isOCLImage2dArrayRWType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE2DDEPTHRWTYPE, "clang::Type::isOCLImage2dDepthRWType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE2DARRAYDEPTHRWTYPE, "clang::Type::isOCLImage2dArrayDepthRWType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE2DMSAARWTYPE, "clang::Type::isOCLImage2dMSAARWType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE2DARRAYMSAARWTYPE, "clang::Type::isOCLImage2dArrayMSAARWType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE2DMSAADEPTHRWTYPE, "clang::Type::isOCLImage2dMSAADepthRWType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE2DARRAYMSAADEPTHRWTYPE, "clang::Type::isOCLImage2dArrayMSAADepthRWType"),
      add_named_node(&mut g, METHOD_ISOCLIMAGE3DRWTYPE, "clang::Type::isOCLImage3dRWType"),
      add_named_node(&mut g, METHOD_ISIMAGETYPE, "clang::Type::isImageType"),
      add_named_node(&mut g, METHOD_ISSAMPLERT, "clang::Type::isSamplerT"),
      add_named_node(&mut g, METHOD_ISEVENTT, "clang::Type::isEventT"),
      add_named_node(&mut g, METHOD_ISCLKEVENTT, "clang::Type::isClkEventT"),
      add_named_node(&mut g, METHOD_ISQUEUET, "clang::Type::isQueueT"),
      add_named_node(&mut g, METHOD_ISRESERVEIDT, "clang::Type::isReserveIDT"),
      add_named_node(&mut g, METHOD_ISOCLINTELSUBGROUPAVCMCEPAYLOADTYPE, "clang::Type::isOCLIntelSubgroupAVCMcePayloadType"),
      add_named_node(&mut g, METHOD_ISOCLINTELSUBGROUPAVCIMEPAYLOADTYPE, "clang::Type::isOCLIntelSubgroupAVCImePayloadType"),
      add_named_node(&mut g, METHOD_ISOCLINTELSUBGROUPAVCREFPAYLOADTYPE, "clang::Type::isOCLIntelSubgroupAVCRefPayloadType"),
      add_named_node(&mut g, METHOD_ISOCLINTELSUBGROUPAVCSICPAYLOADTYPE, "clang::Type::isOCLIntelSubgroupAVCSicPayloadType"),
      add_named_node(&mut g, METHOD_ISOCLINTELSUBGROUPAVCMCERESULTTYPE, "clang::Type::isOCLIntelSubgroupAVCMceResultType"),
      add_named_node(&mut g, METHOD_ISOCLINTELSUBGROUPAVCIMERESULTTYPE, "clang::Type::isOCLIntelSubgroupAVCImeResultType"),
      add_named_node(&mut g, METHOD_ISOCLINTELSUBGROUPAVCREFRESULTTYPE, "clang::Type::isOCLIntelSubgroupAVCRefResultType"),
      add_named_node(&mut g, METHOD_ISOCLINTELSUBGROUPAVCSICRESULTTYPE, "clang::Type::isOCLIntelSubgroupAVCSicResultType"),
      add_named_node(&mut g, METHOD_ISOCLINTELSUBGROUPAVCIMERESULTSINGLEREFERENCESTREAMOUTTYPE, "clang::Type::isOCLIntelSubgroupAVCImeResultSingleReferenceStreamoutType"),
      add_named_node(&mut g, METHOD_ISOCLINTELSUBGROUPAVCIMERESULTDUALREFERENCESTREAMOUTTYPE, "clang::Type::isOCLIntelSubgroupAVCImeResultDualReferenceStreamoutType"),
      add_named_node(&mut g, METHOD_ISOCLINTELSUBGROUPAVCIMESINGLEREFERENCESTREAMINTYPE, "clang::Type::isOCLIntelSubgroupAVCImeSingleReferenceStreaminType"),
      add_named_node(&mut g, METHOD_ISOCLINTELSUBGROUPAVCIMEDUALREFERENCESTREAMINTYPE, "clang::Type::isOCLIntelSubgroupAVCImeDualReferenceStreaminType"),
      add_named_node(&mut g, METHOD_ISOCLINTELSUBGROUPAVCTYPE, "clang::Type::isOCLIntelSubgroupAVCType"),
      add_named_node(&mut g, METHOD_ISOCLEXTOPAQUETYPE, "clang::Type::isOCLExtOpaqueType"),
      add_named_node(&mut g, METHOD_ISPIPETYPE, "clang::Type::isPipeType"),
      add_named_node(&mut g, METHOD_ISBITINTTYPE, "clang::Type::isBitIntType"),
      add_named_node(&mut g, METHOD_ISOPENCLSPECIFICTYPE, "clang::Type::isOpenCLSpecificType"),
      add_named_node(&mut g, METHOD_ISOBJCARCIMPLICITLYUNRETAINEDTYPE, "clang::Type::isObjCARCImplicitlyUnretainedType"),
      add_named_node(&mut g, METHOD_ISCUDADEVICEBUILTINSURFACETYPE, "clang::Type::isCUDADeviceBuiltinSurfaceType"),
      add_named_node(&mut g, METHOD_ISCUDADEVICEBUILTINTEXTURETYPE, "clang::Type::isCUDADeviceBuiltinTextureType"),
      add_named_node(&mut g, METHOD_GETOBJCARCIMPLICITLIFETIME, "clang::Type::getObjCARCImplicitLifetime"),
      add_named_node(&mut g, METHOD_GETSCALARTYPEKIND, "clang::Type::getScalarTypeKind"),
      add_named_node(&mut g, METHOD_GETDEPENDENCE, "clang::Type::getDependence"),
      add_named_node(&mut g, METHOD_CONTAINSERRORS, "clang::Type::containsErrors"),
      add_named_node(&mut g, METHOD_ISDEPENDENTTYPE_1, "clang::Type::isDependentType"),
      add_named_node(&mut g, METHOD_ISINSTANTIATIONDEPENDENTTYPE, "clang::Type::isInstantiationDependentType"),
      add_named_node(&mut g, METHOD_ISUNDEDUCEDTYPE, "clang::Type::isUndeducedType"),
      add_named_node(&mut g, METHOD_ISVARIABLYMODIFIEDTYPE, "clang::Type::isVariablyModifiedType"),
      add_named_node(&mut g, METHOD_HASSIZEDVLATYPE, "clang::Type::hasSizedVLAType"),
      add_named_node(&mut g, METHOD_HASUNNAMEDORLOCALTYPE, "clang::Type::hasUnnamedOrLocalType"),
      add_named_node(&mut g, METHOD_ISOVERLOADABLETYPE, "clang::Type::isOverloadableType"),
      add_named_node(&mut g, METHOD_ISELABORATEDTYPESPECIFIER, "clang::Type::isElaboratedTypeSpecifier"),
      add_named_node(&mut g, METHOD_CANDECAYTOPOINTERTYPE, "clang::Type::canDecayToPointerType"),
      add_named_node(&mut g, METHOD_HASPOINTERREPRESENTATION, "clang::Type::hasPointerRepresentation"),
      add_named_node(&mut g, METHOD_HASOBJCPOINTERREPRESENTATION, "clang::Type::hasObjCPointerRepresentation"),
      add_named_node(&mut g, METHOD_HASINTEGERREPRESENTATION, "clang::Type::hasIntegerRepresentation"),
      add_named_node(&mut g, METHOD_HASSIGNEDINTEGERREPRESENTATION, "clang::Type::hasSignedIntegerRepresentation"),
      add_named_node(&mut g, METHOD_HASUNSIGNEDINTEGERREPRESENTATION, "clang::Type::hasUnsignedIntegerRepresentation"),
      add_named_node(&mut g, METHOD_HASFLOATINGREPRESENTATION, "clang::Type::hasFloatingRepresentation"),
      add_named_node(&mut g, METHOD_GETASSTRUCTURETYPE, "clang::Type::getAsStructureType"),
      add_named_node(&mut g, METHOD_GETASUNIONTYPE, "clang::Type::getAsUnionType"),
      add_named_node(&mut g, METHOD_GETASCOMPLEXINTEGERTYPE, "clang::Type::getAsComplexIntegerType"),
      add_named_node(&mut g, METHOD_GETASOBJCINTERFACETYPE, "clang::Type::getAsObjCInterfaceType"),
      add_named_node(&mut g, METHOD_GETASOBJCINTERFACEPOINTERTYPE, "clang::Type::getAsObjCInterfacePointerType"),
      add_named_node(&mut g, METHOD_GETASOBJCQUALIFIEDIDTYPE, "clang::Type::getAsObjCQualifiedIdType"),
      add_named_node(&mut g, METHOD_GETASOBJCQUALIFIEDCLASSTYPE, "clang::Type::getAsObjCQualifiedClassType"),
      add_named_node(&mut g, METHOD_GETASOBJCQUALIFIEDINTERFACETYPE, "clang::Type::getAsObjCQualifiedInterfaceType"),
      add_named_node(&mut g, METHOD_GETASCXXRECORDDECL, "clang::Type::getAsCXXRecordDecl"),
      add_named_node(&mut g, METHOD_GETASRECORDDECL, "clang::Type::getAsRecordDecl"),
      add_named_node(&mut g, METHOD_GETASTAGDECL, "clang::Type::getAsTagDecl"),
      add_named_node(&mut g, METHOD_GETPOINTEECXXRECORDDECL, "clang::Type::getPointeeCXXRecordDecl"),
      add_named_node(&mut g, METHOD_GETCONTAINEDDEDUCEDTYPE, "clang::Type::getContainedDeducedType"),
      add_named_node(&mut g, METHOD_GETCONTAINEDAUTOTYPE, "clang::Type::getContainedAutoType"),
      add_named_node(&mut g, METHOD_HASAUTOFORTRAILINGRETURNTYPE, "clang::Type::hasAutoForTrailingReturnType"),
      add_named_node(&mut g, METHOD_GETASARRAYTYPEUNSAFE, "clang::Type::getAsArrayTypeUnsafe"),
      add_named_node(&mut g, METHOD_CASTASARRAYTYPEUNSAFE, "clang::Type::castAsArrayTypeUnsafe"),
      add_named_node(&mut g, METHOD_GETBASEELEMENTTYPEUNSAFE, "clang::Type::getBaseElementTypeUnsafe"),
      add_named_node(&mut g, METHOD_GETARRAYELEMENTTYPENOTYPEQUAL, "clang::Type::getArrayElementTypeNoTypeQual"),
      add_named_node(&mut g, METHOD_GETPOINTEEORARRAYELEMENTTYPE, "clang::Type::getPointeeOrArrayElementType"),
      add_named_node(&mut g, METHOD_GETPOINTEETYPE_6, "clang::Type::getPointeeType"),
      add_named_node(&mut g, METHOD_GETUNQUALIFIEDDESUGAREDTYPE, "clang::Type::getUnqualifiedDesugaredType"),
      add_named_node(&mut g, METHOD_ISSIGNEDINTEGERTYPE, "clang::Type::isSignedIntegerType"),
      add_named_node(&mut g, METHOD_ISUNSIGNEDINTEGERTYPE, "clang::Type::isUnsignedIntegerType"),
      add_named_node(&mut g, METHOD_ISSIGNEDINTEGERORENUMERATIONTYPE, "clang::Type::isSignedIntegerOrEnumerationType"),
      add_named_node(&mut g, METHOD_ISUNSIGNEDINTEGERORENUMERATIONTYPE, "clang::Type::isUnsignedIntegerOrEnumerationType"),
      add_named_node(&mut g, METHOD_ISFIXEDPOINTTYPE, "clang::Type::isFixedPointType"),
      add_named_node(&mut g, METHOD_ISFIXEDPOINTORINTEGERTYPE, "clang::Type::isFixedPointOrIntegerType"),
      add_named_node(&mut g, METHOD_ISSATURATEDFIXEDPOINTTYPE, "clang::Type::isSaturatedFixedPointType"),
      add_named_node(&mut g, METHOD_ISUNSATURATEDFIXEDPOINTTYPE, "clang::Type::isUnsaturatedFixedPointType"),
      add_named_node(&mut g, METHOD_ISSIGNEDFIXEDPOINTTYPE, "clang::Type::isSignedFixedPointType"),
      add_named_node(&mut g, METHOD_ISUNSIGNEDFIXEDPOINTTYPE, "clang::Type::isUnsignedFixedPointType"),
      add_named_node(&mut g, METHOD_ISCONSTANTSIZETYPE, "clang::Type::isConstantSizeType"),
      add_named_node(&mut g, METHOD_ISSPECIFIERTYPE, "clang::Type::isSpecifierType"),
      add_named_node(&mut g, METHOD_GETLINKAGE, "clang::Type::getLinkage"),
      add_named_node(&mut g, METHOD_GETVISIBILITY, "clang::Type::getVisibility"),
      add_named_node(&mut g, METHOD_ISVISIBILITYEXPLICIT, "clang::Type::isVisibilityExplicit"),
      add_named_node(&mut g, METHOD_GETLINKAGEANDVISIBILITY, "clang::Type::getLinkageAndVisibility"),
      add_named_node(&mut g, METHOD_ISLINKAGEVALID_1, "clang::Type::isLinkageValid"),
      add_named_node(&mut g, METHOD_GETNULLABILITY, "clang::Type::getNullability"),
      add_named_node(&mut g, METHOD_ACCEPTSOBJCTYPEPARAMS, "clang::Type::acceptsObjCTypeParams"),
      add_named_node(&mut g, METHOD_GETTYPECLASSNAME, "clang::Type::getTypeClassName"),
      add_named_node(&mut g, METHOD_GETCANONICALTYPEINTERNAL, "clang::Type::getCanonicalTypeInternal"),
      add_named_node(&mut g, METHOD_GETCANONICALTYPEUNQUALIFIED, "clang::Type::getCanonicalTypeUnqualified"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_INITSEGATTR, "clang::InitSegAttr");
  g.add_edge((CLASS_INITSEGATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_INITSEGATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_OBJCCOMPATIBLEALIASDECL, "clang::ObjCCompatibleAliasDecl");
  g.add_edge((CLASS_OBJCCOMPATIBLEALIASDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCCOMPATIBLEALIASDECL, META_SUBCLASS, CLASS_NAMEDDECL));

  g.add_named_node(CLASS_TRYACQUIRECAPABILITYATTR, "clang::TryAcquireCapabilityAttr");
  g.add_edge((CLASS_TRYACQUIRECAPABILITYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TRYACQUIRECAPABILITYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_TRANSPARENTUNIONATTR, "clang::TransparentUnionAttr");
  g.add_edge((CLASS_TRANSPARENTUNIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TRANSPARENTUNIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_SWIFTERRORATTR, "clang::SwiftErrorAttr");
  g.add_edge((CLASS_SWIFTERRORATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTERRORATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_SWIFTASYNCCALLATTR, "clang::SwiftAsyncCallAttr");
  g.add_edge((CLASS_SWIFTASYNCCALLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTASYNCCALLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_TYPETRAITEXPR, "clang::TypeTraitExpr");
  g.add_edge((CLASS_TYPETRAITEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPETRAITEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTRAIT_2, "clang::TypeTraitExpr::getTrait"),
      add_named_node(&mut g, METHOD_GETVALUE_10, "clang::TypeTraitExpr::getValue"),
      add_named_node(&mut g, METHOD_GETNUMARGS_4, "clang::TypeTraitExpr::getNumArgs"),
      add_named_node(&mut g, METHOD_GETARGS_2, "clang::TypeTraitExpr::getArgs"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_124, "clang::TypeTraitExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_123, "clang::TypeTraitExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_111, "clang::TypeTraitExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPETRAITEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_M68KINTERRUPTATTR, "clang::M68kInterruptAttr");
  g.add_edge((CLASS_M68KINTERRUPTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_M68KINTERRUPTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_SWIFTASYNCATTR, "clang::SwiftAsyncAttr");
  g.add_edge((CLASS_SWIFTASYNCATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTASYNCATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_SECTIONATTR, "clang::SectionAttr");
  g.add_edge((CLASS_SECTIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SECTIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OSRETURNSNOTRETAINEDATTR, "clang::OSReturnsNotRetainedAttr");
  g.add_edge((CLASS_OSRETURNSNOTRETAINEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OSRETURNSNOTRETAINEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CDECLATTR, "clang::CDeclAttr");
  g.add_edge((CLASS_CDECLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CDECLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CONVERGENTATTR, "clang::ConvergentAttr");
  g.add_edge((CLASS_CONVERGENTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONVERGENTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_NOTHROWATTR, "clang::NoThrowAttr");
  g.add_edge((CLASS_NOTHROWATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOTHROWATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_SENTINELATTR, "clang::SentinelAttr");
  g.add_edge((CLASS_SENTINELATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SENTINELATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_NOESCAPEATTR, "clang::NoEscapeAttr");
  g.add_edge((CLASS_NOESCAPEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOESCAPEATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_OBJCPRECISELIFETIMEATTR, "clang::ObjCPreciseLifetimeAttr");
  g.add_edge((CLASS_OBJCPRECISELIFETIMEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCPRECISELIFETIMEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_ALIGNEDATTR, "clang::AlignedAttr");
  g.add_edge((CLASS_ALIGNEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ALIGNEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CONSUMABLEAUTOCASTATTR, "clang::ConsumableAutoCastAttr");
  g.add_edge((CLASS_CONSUMABLEAUTOCASTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONSUMABLEAUTOCASTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CONSTRUCTORATTR, "clang::ConstructorAttr");
  g.add_edge((CLASS_CONSTRUCTORATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONSTRUCTORATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_MATRIXSUBSCRIPTEXPR, "clang::MatrixSubscriptExpr");
  g.add_edge((CLASS_MATRIXSUBSCRIPTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MATRIXSUBSCRIPTEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_ISINCOMPLETE, "clang::MatrixSubscriptExpr::isIncomplete"),
      add_named_node(&mut g, METHOD_GETBASE_5, "clang::MatrixSubscriptExpr::getBase"),
      add_named_node(&mut g, METHOD_GETROWIDX, "clang::MatrixSubscriptExpr::getRowIdx"),
      add_named_node(&mut g, METHOD_GETCOLUMNIDX, "clang::MatrixSubscriptExpr::getColumnIdx"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_96, "clang::MatrixSubscriptExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_95, "clang::MatrixSubscriptExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETEXPRLOC_8, "clang::MatrixSubscriptExpr::getExprLoc"),
      add_named_node(&mut g, METHOD_GETRBRACKETLOC_4, "clang::MatrixSubscriptExpr::getRBracketLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_84, "clang::MatrixSubscriptExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MATRIXSUBSCRIPTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ALIASATTR, "clang::AliasAttr");
  g.add_edge((CLASS_ALIASATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ALIASATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_OBJCCLASSSTUBATTR, "clang::ObjCClassStubAttr");
  g.add_edge((CLASS_OBJCCLASSSTUBATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCCLASSSTUBATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_CXX11NORETURNATTR, "clang::CXX11NoReturnAttr");
  g.add_edge((CLASS_CXX11NORETURNATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXX11NORETURNATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_REGCALLATTR, "clang::RegCallAttr");
  g.add_edge((CLASS_REGCALLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_REGCALLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CXXMEMBERCALLEXPR, "clang::CXXMemberCallExpr");
  g.add_edge((CLASS_CXXMEMBERCALLEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXMEMBERCALLEXPR, META_SUBCLASS, CLASS_CALLEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETIMPLICITOBJECTARGUMENT, "clang::CXXMemberCallExpr::getImplicitObjectArgument"),
      add_named_node(&mut g, METHOD_GETOBJECTTYPE, "clang::CXXMemberCallExpr::getObjectType"),
      add_named_node(&mut g, METHOD_GETMETHODDECL, "clang::CXXMemberCallExpr::getMethodDecl"),
      add_named_node(&mut g, METHOD_GETRECORDDECL, "clang::CXXMemberCallExpr::getRecordDecl"),
      add_named_node(&mut g, METHOD_GETEXPRLOC_3, "clang::CXXMemberCallExpr::getExprLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXMEMBERCALLEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_WARNUNUSEDATTR, "clang::WarnUnusedAttr");
  g.add_edge((CLASS_WARNUNUSEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_WARNUNUSEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_NONNULLATTR, "clang::NonNullAttr");
  g.add_edge((CLASS_NONNULLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NONNULLATTR, META_SUBCLASS, CLASS_INHERITABLEPARAMATTR));

  g.add_named_node(CLASS_MUSTTAILATTR, "clang::MustTailAttr");
  g.add_edge((CLASS_MUSTTAILATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MUSTTAILATTR, META_SUBCLASS, CLASS_STMTATTR));

  g.add_named_node(CLASS_AMDGPUWAVESPEREUATTR, "clang::AMDGPUWavesPerEUAttr");
  g.add_edge((CLASS_AMDGPUWAVESPEREUATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_AMDGPUWAVESPEREUATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CFCONSUMEDATTR, "clang::CFConsumedAttr");
  g.add_edge((CLASS_CFCONSUMEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CFCONSUMEDATTR, META_SUBCLASS, CLASS_INHERITABLEPARAMATTR));

  g.add_named_node(CLASS_OMPTEAMSDIRECTIVE, "clang::OMPTeamsDirective");
  g.add_edge((CLASS_OMPTEAMSDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTEAMSDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_CLASSTEMPLATESPECIALIZATIONDECL, "clang::ClassTemplateSpecializationDecl");
  g.add_edge((CLASS_CLASSTEMPLATESPECIALIZATIONDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CLASSTEMPLATESPECIALIZATIONDECL, META_SUBCLASS, CLASS_CXXRECORDDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSPECIALIZEDTEMPLATE, "clang::ClassTemplateSpecializationDecl::getSpecializedTemplate"),
      add_named_node(&mut g, METHOD_GETTEMPLATEARGS, "clang::ClassTemplateSpecializationDecl::getTemplateArgs"),
      add_named_node(&mut g, METHOD_GETSPECIALIZATIONKIND, "clang::ClassTemplateSpecializationDecl::getSpecializationKind"),
      add_named_node(&mut g, METHOD_ISEXPLICITSPECIALIZATION, "clang::ClassTemplateSpecializationDecl::isExplicitSpecialization"),
      add_named_node(&mut g, METHOD_ISCLASSSCOPEEXPLICITSPECIALIZATION, "clang::ClassTemplateSpecializationDecl::isClassScopeExplicitSpecialization"),
      add_named_node(&mut g, METHOD_ISEXPLICITINSTANTIATIONORSPECIALIZATION, "clang::ClassTemplateSpecializationDecl::isExplicitInstantiationOrSpecialization"),
      add_named_node(&mut g, METHOD_GETPOINTOFINSTANTIATION, "clang::ClassTemplateSpecializationDecl::getPointOfInstantiation"),
      add_named_node(&mut g, METHOD_GETINSTANTIATEDFROM, "clang::ClassTemplateSpecializationDecl::getInstantiatedFrom"),
      add_named_node(&mut g, METHOD_GETSPECIALIZEDTEMPLATEORPARTIAL, "clang::ClassTemplateSpecializationDecl::getSpecializedTemplateOrPartial"),
      add_named_node(&mut g, METHOD_GETTEMPLATEINSTANTIATIONARGS, "clang::ClassTemplateSpecializationDecl::getTemplateInstantiationArgs"),
      add_named_node(&mut g, METHOD_GETTYPEASWRITTEN, "clang::ClassTemplateSpecializationDecl::getTypeAsWritten"),
      add_named_node(&mut g, METHOD_GETEXTERNLOC, "clang::ClassTemplateSpecializationDecl::getExternLoc"),
      add_named_node(&mut g, METHOD_GETTEMPLATEKEYWORDLOC, "clang::ClassTemplateSpecializationDecl::getTemplateKeywordLoc"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_3, "clang::ClassTemplateSpecializationDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CLASSTEMPLATESPECIALIZATIONDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NOINLINEATTR, "clang::NoInlineAttr");
  g.add_edge((CLASS_NOINLINEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOINLINEATTR, META_SUBCLASS, CLASS_DECLORSTMTATTR));

  g.add_named_node(CLASS_OBJCINDEPENDENTCLASSATTR, "clang::ObjCIndependentClassAttr");
  g.add_edge((CLASS_OBJCINDEPENDENTCLASSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCINDEPENDENTCLASSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_PARAMETERABIATTR, "clang::ParameterABIAttr");
  g.add_edge((CLASS_PARAMETERABIATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PARAMETERABIATTR, META_SUBCLASS, CLASS_INHERITABLEPARAMATTR));

  g.add_named_node(CLASS_UPTRATTR, "clang::UPtrAttr");
  g.add_edge((CLASS_UPTRATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UPTRATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_OMPPARALLELFORDIRECTIVE, "clang::OMPParallelForDirective");
  g.add_edge((CLASS_OMPPARALLELFORDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPPARALLELFORDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_ASMLABELATTR, "clang::AsmLabelAttr");
  g.add_edge((CLASS_ASMLABELATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ASMLABELATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_ACQUIREHANDLEATTR, "clang::AcquireHandleAttr");
  g.add_edge((CLASS_ACQUIREHANDLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ACQUIREHANDLEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_SWIFTCONTEXTATTR, "clang::SwiftContextAttr");
  g.add_edge((CLASS_SWIFTCONTEXTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTCONTEXTATTR, META_SUBCLASS, CLASS_PARAMETERABIATTR));

  g.add_named_node(CLASS_AVRSIGNALATTR, "clang::AVRSignalAttr");
  g.add_edge((CLASS_AVRSIGNALATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_AVRSIGNALATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OPENCLKERNELATTR, "clang::OpenCLKernelAttr");
  g.add_edge((CLASS_OPENCLKERNELATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OPENCLKERNELATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_DEPENDENTUNARYTRANSFORMTYPE, "clang::DependentUnaryTransformType");
  g.add_edge((CLASS_DEPENDENTUNARYTRANSFORMTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEPENDENTUNARYTRANSFORMTYPE, META_SUBCLASS, CLASS_UNARYTRANSFORMTYPE));

  g.add_named_node(CLASS_ARMPRESERVESATTR, "clang::ArmPreservesAttr");
  g.add_edge((CLASS_ARMPRESERVESATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARMPRESERVESATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_STRICTFPATTR, "clang::StrictFPAttr");
  g.add_edge((CLASS_STRICTFPATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_STRICTFPATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CFGUARDATTR, "clang::CFGuardAttr");
  g.add_edge((CLASS_CFGUARDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CFGUARDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CUDADEVICEBUILTINTEXTURETYPEATTR, "clang::CUDADeviceBuiltinTextureTypeAttr");
  g.add_edge((CLASS_CUDADEVICEBUILTINTEXTURETYPEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CUDADEVICEBUILTINTEXTURETYPEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_NSCONSUMEDATTR, "clang::NSConsumedAttr");
  g.add_edge((CLASS_NSCONSUMEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NSCONSUMEDATTR, META_SUBCLASS, CLASS_INHERITABLEPARAMATTR));

  g.add_named_node(CLASS_ARMMVESTRICTPOLYMORPHISMATTR, "clang::ArmMveStrictPolymorphismAttr");
  g.add_edge((CLASS_ARMMVESTRICTPOLYMORPHISMATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARMMVESTRICTPOLYMORPHISMATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_SWIFTERRORRESULTATTR, "clang::SwiftErrorResultAttr");
  g.add_edge((CLASS_SWIFTERRORRESULTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTERRORRESULTATTR, META_SUBCLASS, CLASS_PARAMETERABIATTR));

  g.add_named_node(CLASS_CXXUUIDOFEXPR, "clang::CXXUuidofExpr");
  g.add_edge((CLASS_CXXUUIDOFEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXUUIDOFEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_ISTYPEOPERAND_1, "clang::CXXUuidofExpr::isTypeOperand"),
      add_named_node(&mut g, METHOD_GETTYPEOPERANDSOURCEINFO_1, "clang::CXXUuidofExpr::getTypeOperandSourceInfo"),
      add_named_node(&mut g, METHOD_GETEXPROPERAND_1, "clang::CXXUuidofExpr::getExprOperand"),
      add_named_node(&mut g, METHOD_GETGUIDDECL, "clang::CXXUuidofExpr::getGuidDecl"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_47, "clang::CXXUuidofExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_46, "clang::CXXUuidofExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_46, "clang::CXXUuidofExpr::getSourceRange"),
      add_named_node(&mut g, METHOD_CHILDREN_36, "clang::CXXUuidofExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXUUIDOFEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_THREADATTR, "clang::ThreadAttr");
  g.add_edge((CLASS_THREADATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_THREADATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_OBJCGCATTR, "clang::ObjCGCAttr");
  g.add_edge((CLASS_OBJCGCATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCGCATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_TYPEALIASDECL, "clang::TypeAliasDecl");
  g.add_edge((CLASS_TYPEALIASDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPEALIASDECL, META_SUBCLASS, CLASS_TYPEDEFNAMEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSOURCERANGE_28, "clang::TypeAliasDecl::getSourceRange"),
      add_named_node(&mut g, METHOD_GETDESCRIBEDALIASTEMPLATE, "clang::TypeAliasDecl::getDescribedAliasTemplate"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPEALIASDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NSRETURNSAUTORELEASEDATTR, "clang::NSReturnsAutoreleasedAttr");
  g.add_edge((CLASS_NSRETURNSAUTORELEASEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NSRETURNSAUTORELEASEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_ALLOCALIGNATTR, "clang::AllocAlignAttr");
  g.add_edge((CLASS_ALLOCALIGNATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ALLOCALIGNATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OPENCLCONSTANTADDRESSSPACEATTR, "clang::OpenCLConstantAddressSpaceAttr");
  g.add_edge((CLASS_OPENCLCONSTANTADDRESSSPACEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OPENCLCONSTANTADDRESSSPACEATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_OMPREFERENCEDVARATTR, "clang::OMPReferencedVarAttr");
  g.add_edge((CLASS_OMPREFERENCEDVARATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPREFERENCEDVARATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_CFRETURNSNOTRETAINEDATTR, "clang::CFReturnsNotRetainedAttr");
  g.add_edge((CLASS_CFRETURNSNOTRETAINEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CFRETURNSNOTRETAINEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_REQUIRESEXPRBODYDECL, "clang::RequiresExprBodyDecl");
  g.add_edge((CLASS_REQUIRESEXPRBODYDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_REQUIRESEXPRBODYDECL, META_SUBCLASS, CLASS_DECL));

  g.add_named_node(CLASS_THISCALLATTR, "clang::ThisCallAttr");
  g.add_edge((CLASS_THISCALLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_THISCALLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OBJCINTERFACETYPELOC, "clang::ObjCInterfaceTypeLoc");
  g.add_edge((CLASS_OBJCINTERFACETYPELOC, META_CLASS, META_CLANG_AST_NODE));

  g.add_named_node(CLASS_ARMSTREAMINGATTR, "clang::ArmStreamingAttr");
  g.add_edge((CLASS_ARMSTREAMINGATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARMSTREAMINGATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_VECTORTYPELOC, "clang::VectorTypeLoc");
  g.add_edge((CLASS_VECTORTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETNAMELOC_2, "clang::VectorTypeLoc::getNameLoc"),
      add_named_node(&mut g, METHOD_GETLOCALSOURCERANGE, "clang::VectorTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, METHOD_GETELEMENTLOC, "clang::VectorTypeLoc::getElementLoc"),
      add_named_node(&mut g, METHOD_GETINNERTYPE_1, "clang::VectorTypeLoc::getInnerType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_VECTORTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPCAPTURENOINITATTR, "clang::OMPCaptureNoInitAttr");
  g.add_edge((CLASS_OMPCAPTURENOINITATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPCAPTURENOINITATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_SWIFTIMPORTPROPERTYASACCESSORSATTR, "clang::SwiftImportPropertyAsAccessorsAttr");
  g.add_edge((CLASS_SWIFTIMPORTPROPERTYASACCESSORSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTIMPORTPROPERTYASACCESSORSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_ACQUIREDAFTERATTR, "clang::AcquiredAfterAttr");
  g.add_edge((CLASS_ACQUIREDAFTERATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ACQUIREDAFTERATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_ATTRIBUTEDTYPELOC, "clang::AttributedTypeLoc");
  g.add_edge((CLASS_ATTRIBUTEDTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETATTRKIND_1, "clang::AttributedTypeLoc::getAttrKind"),
      add_named_node(&mut g, METHOD_ISQUALIFIER_1, "clang::AttributedTypeLoc::isQualifier"),
      add_named_node(&mut g, METHOD_GETMODIFIEDLOC, "clang::AttributedTypeLoc::getModifiedLoc"),
      add_named_node(&mut g, METHOD_GETATTR_1, "clang::AttributedTypeLoc::getAttr"),
      add_named_node(&mut g, METHOD_GETLOCALSOURCERANGE_1, "clang::AttributedTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, METHOD_GETINNERTYPE_2, "clang::AttributedTypeLoc::getInnerType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ATTRIBUTEDTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCRUNTIMEVISIBLEATTR, "clang::ObjCRuntimeVisibleAttr");
  g.add_edge((CLASS_OBJCRUNTIMEVISIBLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCRUNTIMEVISIBLEATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_TYPENONNULLATTR, "clang::TypeNonNullAttr");
  g.add_edge((CLASS_TYPENONNULLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPENONNULLATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_SYCLSPECIALCLASSATTR, "clang::SYCLSpecialClassAttr");
  g.add_edge((CLASS_SYCLSPECIALCLASSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SYCLSPECIALCLASSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_ARGUMENTWITHTYPETAGATTR, "clang::ArgumentWithTypeTagAttr");
  g.add_edge((CLASS_ARGUMENTWITHTYPETAGATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARGUMENTWITHTYPETAGATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_AARCH64SVEPCSATTR, "clang::AArch64SVEPcsAttr");
  g.add_edge((CLASS_AARCH64SVEPCSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_AARCH64SVEPCSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_PRAGMACLANGRODATASECTIONATTR, "clang::PragmaClangRodataSectionAttr");
  g.add_edge((CLASS_PRAGMACLANGRODATASECTIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PRAGMACLANGRODATASECTIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_DEPENDENTVECTORTYPE, "clang::DependentVectorType");
  g.add_edge((CLASS_DEPENDENTVECTORTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEPENDENTVECTORTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSIZEEXPR_3, "clang::DependentVectorType::getSizeExpr"),
      add_named_node(&mut g, METHOD_GETELEMENTTYPE_3, "clang::DependentVectorType::getElementType"),
      add_named_node(&mut g, METHOD_GETATTRIBUTELOC_3, "clang::DependentVectorType::getAttributeLoc"),
      add_named_node(&mut g, METHOD_GETVECTORKIND, "clang::DependentVectorType::getVectorKind"),
      add_named_node(&mut g, METHOD_ISSUGARED_17, "clang::DependentVectorType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_17, "clang::DependentVectorType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTVECTORTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NODEREFATTR, "clang::NoDerefAttr");
  g.add_edge((CLASS_NODEREFATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NODEREFATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_ARMSTREAMINGCOMPATIBLEATTR, "clang::ArmStreamingCompatibleAttr");
  g.add_edge((CLASS_ARMSTREAMINGCOMPATIBLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARMSTREAMINGCOMPATIBLEATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_ASSUMPTIONATTR, "clang::AssumptionAttr");
  g.add_edge((CLASS_ASSUMPTIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ASSUMPTIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_FASTCALLATTR, "clang::FastCallAttr");
  g.add_edge((CLASS_FASTCALLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FASTCALLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_TYPELOC, "clang::TypeLoc");
  g.add_edge((CLASS_TYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTYPELOCCLASS, "clang::TypeLoc::getTypeLocClass"),
      add_named_node(&mut g, METHOD_ISNULL, "clang::TypeLoc::isNull"),
      add_named_node(&mut g, METHOD_GETTYPE_2, "clang::TypeLoc::getType"),
      add_named_node(&mut g, METHOD_GETTYPEPTR, "clang::TypeLoc::getTypePtr"),
      add_named_node(&mut g, METHOD_GETOPAQUEDATA, "clang::TypeLoc::getOpaqueData"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_134, "clang::TypeLoc::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_133, "clang::TypeLoc::getEndLoc"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_50, "clang::TypeLoc::getSourceRange"),
      add_named_node(&mut g, METHOD_GETLOCALSOURCERANGE_2, "clang::TypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, METHOD_GETFULLDATASIZE, "clang::TypeLoc::getFullDataSize"),
      add_named_node(&mut g, METHOD_GETNEXTTYPELOC, "clang::TypeLoc::getNextTypeLoc"),
      add_named_node(&mut g, METHOD_GETUNQUALIFIEDLOC, "clang::TypeLoc::getUnqualifiedLoc"),
      add_named_node(&mut g, METHOD_IGNOREPARENS_1, "clang::TypeLoc::IgnoreParens"),
      add_named_node(&mut g, METHOD_FINDEXPLICITQUALIFIERLOC, "clang::TypeLoc::findExplicitQualifierLoc"),
      add_named_node(&mut g, METHOD_GETCONTAINEDAUTOTYPELOC, "clang::TypeLoc::getContainedAutoTypeLoc"),
      add_named_node(&mut g, METHOD_FINDNULLABILITYLOC, "clang::TypeLoc::findNullabilityLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OPENCLLOCALADDRESSSPACEATTR, "clang::OpenCLLocalAddressSpaceAttr");
  g.add_edge((CLASS_OPENCLLOCALADDRESSSPACEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OPENCLLOCALADDRESSSPACEATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_ADJUSTEDTYPE, "clang::AdjustedType");
  g.add_edge((CLASS_ADJUSTEDTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ADJUSTEDTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETORIGINALTYPE, "clang::AdjustedType::getOriginalType"),
      add_named_node(&mut g, METHOD_GETADJUSTEDTYPE, "clang::AdjustedType::getAdjustedType"),
      add_named_node(&mut g, METHOD_ISSUGARED, "clang::AdjustedType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR, "clang::AdjustedType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ADJUSTEDTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTSIZEDEXTVECTORTYPE, "clang::DependentSizedExtVectorType");
  g.add_edge((CLASS_DEPENDENTSIZEDEXTVECTORTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEPENDENTSIZEDEXTVECTORTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSIZEEXPR_2, "clang::DependentSizedExtVectorType::getSizeExpr"),
      add_named_node(&mut g, METHOD_GETELEMENTTYPE_2, "clang::DependentSizedExtVectorType::getElementType"),
      add_named_node(&mut g, METHOD_GETATTRIBUTELOC_1, "clang::DependentSizedExtVectorType::getAttributeLoc"),
      add_named_node(&mut g, METHOD_ISSUGARED_15, "clang::DependentSizedExtVectorType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_15, "clang::DependentSizedExtVectorType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTSIZEDEXTVECTORTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCINERTUNSAFEUNRETAINEDATTR, "clang::ObjCInertUnsafeUnretainedAttr");
  g.add_edge((CLASS_OBJCINERTUNSAFEUNRETAINEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCINERTUNSAFEUNRETAINEDATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_ARRAYINITLOOPEXPR, "clang::ArrayInitLoopExpr");
  g.add_edge((CLASS_ARRAYINITLOOPEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARRAYINITLOOPEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETCOMMONEXPR, "clang::ArrayInitLoopExpr::getCommonExpr"),
      add_named_node(&mut g, METHOD_GETSUBEXPR, "clang::ArrayInitLoopExpr::getSubExpr"),
      add_named_node(&mut g, METHOD_GETARRAYSIZE, "clang::ArrayInitLoopExpr::getArraySize"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_6, "clang::ArrayInitLoopExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_5, "clang::ArrayInitLoopExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_2, "clang::ArrayInitLoopExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ARRAYINITLOOPEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_RECORDTYPE, "clang::RecordType");
  g.add_edge((CLASS_RECORDTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RECORDTYPE, META_SUBCLASS, CLASS_TAGTYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETDECL_2, "clang::RecordType::getDecl"),
      add_named_node(&mut g, METHOD_HASCONSTFIELDS, "clang::RecordType::hasConstFields"),
      add_named_node(&mut g, METHOD_ISSUGARED_34, "clang::RecordType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_34, "clang::RecordType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_RECORDTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_USINGIFEXISTSATTR, "clang::UsingIfExistsAttr");
  g.add_edge((CLASS_USINGIFEXISTSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_USINGIFEXISTSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_SHAREDTRYLOCKFUNCTIONATTR, "clang::SharedTrylockFunctionAttr");
  g.add_edge((CLASS_SHAREDTRYLOCKFUNCTIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SHAREDTRYLOCKFUNCTIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_DEDUCEDTEMPLATESPECIALIZATIONTYPE, "clang::DeducedTemplateSpecializationType");
  g.add_edge((CLASS_DEDUCEDTEMPLATESPECIALIZATIONTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEDUCEDTEMPLATESPECIALIZATIONTYPE, META_SUBCLASS, CLASS_DEDUCEDTYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTEMPLATENAME, "clang::DeducedTemplateSpecializationType::getTemplateName"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEDUCEDTEMPLATESPECIALIZATIONTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SWIFTNAMEATTR, "clang::SwiftNameAttr");
  g.add_edge((CLASS_SWIFTNAMEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTNAMEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_ALLOCSIZEATTR, "clang::AllocSizeAttr");
  g.add_edge((CLASS_ALLOCSIZEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ALLOCSIZEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_VARDECL, "clang::VarDecl");
  g.add_edge((CLASS_VARDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_VARDECL, META_SUBCLASS, CLASS_DECLARATORDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSOURCERANGE_36, "clang::VarDecl::getSourceRange"),
      add_named_node(&mut g, METHOD_GETSTORAGECLASS_1, "clang::VarDecl::getStorageClass"),
      add_named_node(&mut g, METHOD_GETTSCSPEC, "clang::VarDecl::getTSCSpec"),
      add_named_node(&mut g, METHOD_GETTLSKIND, "clang::VarDecl::getTLSKind"),
      add_named_node(&mut g, METHOD_HASLOCALSTORAGE, "clang::VarDecl::hasLocalStorage"),
      add_named_node(&mut g, METHOD_ISSTATICLOCAL, "clang::VarDecl::isStaticLocal"),
      add_named_node(&mut g, METHOD_HASEXTERNALSTORAGE, "clang::VarDecl::hasExternalStorage"),
      add_named_node(&mut g, METHOD_HASGLOBALSTORAGE, "clang::VarDecl::hasGlobalStorage"),
      add_named_node(&mut g, METHOD_GETSTORAGEDURATION_1, "clang::VarDecl::getStorageDuration"),
      add_named_node(&mut g, METHOD_GETLANGUAGELINKAGE_1, "clang::VarDecl::getLanguageLinkage"),
      add_named_node(&mut g, METHOD_ISEXTERNC_1, "clang::VarDecl::isExternC"),
      add_named_node(&mut g, METHOD_ISINEXTERNCCONTEXT_1, "clang::VarDecl::isInExternCContext"),
      add_named_node(&mut g, METHOD_ISINEXTERNCXXCONTEXT_1, "clang::VarDecl::isInExternCXXContext"),
      add_named_node(&mut g, METHOD_ISLOCALVARDECL, "clang::VarDecl::isLocalVarDecl"),
      add_named_node(&mut g, METHOD_ISLOCALVARDECLORPARM, "clang::VarDecl::isLocalVarDeclOrParm"),
      add_named_node(&mut g, METHOD_ISFUNCTIONORMETHODVARDECL, "clang::VarDecl::isFunctionOrMethodVarDecl"),
      add_named_node(&mut g, METHOD_ISSTATICDATAMEMBER, "clang::VarDecl::isStaticDataMember"),
      add_named_node(&mut g, METHOD_GETCANONICALDECL_27, "clang::VarDecl::getCanonicalDecl"),
      add_named_node(&mut g, METHOD_ISTHISDECLARATIONADEFINITION_4, "clang::VarDecl::isThisDeclarationADefinition"),
      add_named_node(&mut g, METHOD_HASDEFINITION_1, "clang::VarDecl::hasDefinition"),
      add_named_node(&mut g, METHOD_GETACTINGDEFINITION, "clang::VarDecl::getActingDefinition"),
      add_named_node(&mut g, METHOD_GETDEFINITION_5, "clang::VarDecl::getDefinition"),
      add_named_node(&mut g, METHOD_ISOUTOFLINE_2, "clang::VarDecl::isOutOfLine"),
      add_named_node(&mut g, METHOD_ISFILEVARDECL, "clang::VarDecl::isFileVarDecl"),
      add_named_node(&mut g, METHOD_GETANYINITIALIZER, "clang::VarDecl::getAnyInitializer"),
      add_named_node(&mut g, METHOD_HASINIT, "clang::VarDecl::hasInit"),
      add_named_node(&mut g, METHOD_GETINIT, "clang::VarDecl::getInit"),
      add_named_node(&mut g, METHOD_GETINITIALIZINGDECLARATION, "clang::VarDecl::getInitializingDeclaration"),
      add_named_node(&mut g, METHOD_ENSUREEVALUATEDSTMT, "clang::VarDecl::ensureEvaluatedStmt"),
      add_named_node(&mut g, METHOD_GETEVALUATEDSTMT, "clang::VarDecl::getEvaluatedStmt"),
      add_named_node(&mut g, METHOD_EVALUATEVALUE, "clang::VarDecl::evaluateValue"),
      add_named_node(&mut g, METHOD_GETEVALUATEDVALUE, "clang::VarDecl::getEvaluatedValue"),
      add_named_node(&mut g, METHOD_HASCONSTANTINITIALIZATION, "clang::VarDecl::hasConstantInitialization"),
      add_named_node(&mut g, METHOD_GETINITSTYLE, "clang::VarDecl::getInitStyle"),
      add_named_node(&mut g, METHOD_ISDIRECTINIT, "clang::VarDecl::isDirectInit"),
      add_named_node(&mut g, METHOD_ISTHISDECLARATIONADEMOTEDDEFINITION_1, "clang::VarDecl::isThisDeclarationADemotedDefinition"),
      add_named_node(&mut g, METHOD_ISEXCEPTIONVARIABLE, "clang::VarDecl::isExceptionVariable"),
      add_named_node(&mut g, METHOD_ISNRVOVARIABLE, "clang::VarDecl::isNRVOVariable"),
      add_named_node(&mut g, METHOD_ISCXXFORRANGEDECL, "clang::VarDecl::isCXXForRangeDecl"),
      add_named_node(&mut g, METHOD_ISOBJCFORDECL, "clang::VarDecl::isObjCForDecl"),
      add_named_node(&mut g, METHOD_ISARCPSEUDOSTRONG, "clang::VarDecl::isARCPseudoStrong"),
      add_named_node(&mut g, METHOD_ISINLINE_1, "clang::VarDecl::isInline"),
      add_named_node(&mut g, METHOD_ISINLINESPECIFIED_1, "clang::VarDecl::isInlineSpecified"),
      add_named_node(&mut g, METHOD_ISCONSTEXPR_1, "clang::VarDecl::isConstexpr"),
      add_named_node(&mut g, METHOD_ISINITCAPTURE_1, "clang::VarDecl::isInitCapture"),
      add_named_node(&mut g, METHOD_ISPARAMETERPACK_5, "clang::VarDecl::isParameterPack"),
      add_named_node(&mut g, METHOD_ISPREVIOUSDECLINSAMEBLOCKSCOPE, "clang::VarDecl::isPreviousDeclInSameBlockScope"),
      add_named_node(&mut g, METHOD_ISESCAPINGBYREF, "clang::VarDecl::isEscapingByref"),
      add_named_node(&mut g, METHOD_ISNONESCAPINGBYREF, "clang::VarDecl::isNonEscapingByref"),
      add_named_node(&mut g, METHOD_HASDEPENDENTALIGNMENT, "clang::VarDecl::hasDependentAlignment"),
      add_named_node(&mut g, METHOD_GETTEMPLATEINSTANTIATIONPATTERN_2, "clang::VarDecl::getTemplateInstantiationPattern"),
      add_named_node(&mut g, METHOD_GETINSTANTIATEDFROMSTATICDATAMEMBER, "clang::VarDecl::getInstantiatedFromStaticDataMember"),
      add_named_node(&mut g, METHOD_GETTEMPLATESPECIALIZATIONKIND_3, "clang::VarDecl::getTemplateSpecializationKind"),
      add_named_node(&mut g, METHOD_GETTEMPLATESPECIALIZATIONKINDFORINSTANTIATION_1, "clang::VarDecl::getTemplateSpecializationKindForInstantiation"),
      add_named_node(&mut g, METHOD_GETPOINTOFINSTANTIATION_2, "clang::VarDecl::getPointOfInstantiation"),
      add_named_node(&mut g, METHOD_GETMEMBERSPECIALIZATIONINFO_3, "clang::VarDecl::getMemberSpecializationInfo"),
      add_named_node(&mut g, METHOD_GETDESCRIBEDVARTEMPLATE, "clang::VarDecl::getDescribedVarTemplate"),
      add_named_node(&mut g, METHOD_ISKNOWNTOBEDEFINED, "clang::VarDecl::isKnownToBeDefined"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_VARDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_COROLIFETIMEBOUNDATTR, "clang::CoroLifetimeBoundAttr");
  g.add_edge((CLASS_COROLIFETIMEBOUNDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_COROLIFETIMEBOUNDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OMPTARGETDIRECTIVE, "clang::OMPTargetDirective");
  g.add_edge((CLASS_OMPTARGETDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_PACKEXPANSIONEXPR, "clang::PackExpansionExpr");
  g.add_edge((CLASS_PACKEXPANSIONEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PACKEXPANSIONEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETPATTERN_2, "clang::PackExpansionExpr::getPattern"),
      add_named_node(&mut g, METHOD_GETELLIPSISLOC_6, "clang::PackExpansionExpr::getEllipsisLoc"),
      add_named_node(&mut g, METHOD_GETNUMEXPANSIONS_3, "clang::PackExpansionExpr::getNumExpansions"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_102, "clang::PackExpansionExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_101, "clang::PackExpansionExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_90, "clang::PackExpansionExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PACKEXPANSIONEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TAGTYPE, "clang::TagType");
  g.add_edge((CLASS_TAGTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TAGTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETDECL_3, "clang::TagType::getDecl"),
      add_named_node(&mut g, METHOD_ISBEINGDEFINED, "clang::TagType::isBeingDefined"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TAGTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPEOFTYPE, "clang::TypeOfType");
  g.add_edge((CLASS_TYPEOFTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPEOFTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETUNMODIFIEDTYPE, "clang::TypeOfType::getUnmodifiedType"),
      add_named_node(&mut g, METHOD_DESUGAR_40, "clang::TypeOfType::desugar"),
      add_named_node(&mut g, METHOD_ISSUGARED_40, "clang::TypeOfType::isSugared"),
      add_named_node(&mut g, METHOD_GETKIND_2, "clang::TypeOfType::getKind"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPEOFTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_HLSLSV_GROUPINDEXATTR, "clang::HLSLSV_GroupIndexAttr");
  g.add_edge((CLASS_HLSLSV_GROUPINDEXATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_HLSLSV_GROUPINDEXATTR, META_SUBCLASS, CLASS_HLSLANNOTATIONATTR));

  g.add_named_node(CLASS_OBJCNSOBJECTATTR, "clang::ObjCNSObjectAttr");
  g.add_edge((CLASS_OBJCNSOBJECTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCNSOBJECTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OMPTASKWAITDIRECTIVE, "clang::OMPTaskwaitDirective");
  g.add_edge((CLASS_OMPTASKWAITDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTASKWAITDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_REQUIRESCAPABILITYATTR, "clang::RequiresCapabilityAttr");
  g.add_edge((CLASS_REQUIRESCAPABILITYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_REQUIRESCAPABILITYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CORODISABLELIFETIMEBOUNDATTR, "clang::CoroDisableLifetimeBoundAttr");
  g.add_edge((CLASS_CORODISABLELIFETIMEBOUNDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CORODISABLELIFETIMEBOUNDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CMSENSCALLATTR, "clang::CmseNSCallAttr");
  g.add_edge((CLASS_CMSENSCALLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CMSENSCALLATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_SWIFTBRIDGEATTR, "clang::SwiftBridgeAttr");
  g.add_edge((CLASS_SWIFTBRIDGEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTBRIDGEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_ANALYZERNORETURNATTR, "clang::AnalyzerNoReturnAttr");
  g.add_edge((CLASS_ANALYZERNORETURNATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ANALYZERNORETURNATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_BTFTYPETAGATTR, "clang::BTFTypeTagAttr");
  g.add_edge((CLASS_BTFTYPETAGATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BTFTYPETAGATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_ANYX86INTERRUPTATTR, "clang::AnyX86InterruptAttr");
  g.add_edge((CLASS_ANYX86INTERRUPTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ANYX86INTERRUPTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_TYPENULLABLEATTR, "clang::TypeNullableAttr");
  g.add_edge((CLASS_TYPENULLABLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPENULLABLEATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_SWIFTIMPORTASNONGENERICATTR, "clang::SwiftImportAsNonGenericAttr");
  g.add_edge((CLASS_SWIFTIMPORTASNONGENERICATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTIMPORTASNONGENERICATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_AUTOTYPE, "clang::AutoType");
  g.add_edge((CLASS_AUTOTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_AUTOTYPE, META_SUBCLASS, CLASS_DEDUCEDTYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTYPECONSTRAINTARGUMENTS, "clang::AutoType::getTypeConstraintArguments"),
      add_named_node(&mut g, METHOD_GETTYPECONSTRAINTCONCEPT, "clang::AutoType::getTypeConstraintConcept"),
      add_named_node(&mut g, METHOD_ISCONSTRAINED, "clang::AutoType::isConstrained"),
      add_named_node(&mut g, METHOD_ISDECLTYPEAUTO, "clang::AutoType::isDecltypeAuto"),
      add_named_node(&mut g, METHOD_ISGNUAUTOTYPE, "clang::AutoType::isGNUAutoType"),
      add_named_node(&mut g, METHOD_GETKEYWORD, "clang::AutoType::getKeyword"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_AUTOTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ARMINATTR, "clang::ArmInAttr");
  g.add_edge((CLASS_ARMINATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARMINATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_LIKELYATTR, "clang::LikelyAttr");
  g.add_edge((CLASS_LIKELYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LIKELYATTR, META_SUBCLASS, CLASS_STMTATTR));

  g.add_named_node(CLASS_OPENCLGLOBALADDRESSSPACEATTR, "clang::OpenCLGlobalAddressSpaceAttr");
  g.add_edge((CLASS_OPENCLGLOBALADDRESSSPACEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OPENCLGLOBALADDRESSSPACEATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_CODEALIGNATTR, "clang::CodeAlignAttr");
  g.add_edge((CLASS_CODEALIGNATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CODEALIGNATTR, META_SUBCLASS, CLASS_STMTATTR));

  g.add_named_node(CLASS_CALLABLEWHENATTR, "clang::CallableWhenAttr");
  g.add_edge((CLASS_CALLABLEWHENATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CALLABLEWHENATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OPENCLGLOBALDEVICEADDRESSSPACEATTR, "clang::OpenCLGlobalDeviceAddressSpaceAttr");
  g.add_edge((CLASS_OPENCLGLOBALDEVICEADDRESSSPACEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OPENCLGLOBALDEVICEADDRESSSPACEATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_OBJCMESSAGEEXPR, "clang::ObjCMessageExpr");
  g.add_edge((CLASS_OBJCMESSAGEEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCMESSAGEEXPR, META_SUBCLASS, CLASS_EXPR));

  g.add_named_node(CLASS_CUDASHAREDATTR, "clang::CUDASharedAttr");
  g.add_edge((CLASS_CUDASHAREDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CUDASHAREDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_VAARGEXPR, "clang::VAArgExpr");
  g.add_edge((CLASS_VAARGEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_VAARGEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSUBEXPR_10, "clang::VAArgExpr::getSubExpr"),
      add_named_node(&mut g, METHOD_ISMICROSOFTABI, "clang::VAArgExpr::isMicrosoftABI"),
      add_named_node(&mut g, METHOD_GETWRITTENTYPEINFO, "clang::VAArgExpr::getWrittenTypeInfo"),
      add_named_node(&mut g, METHOD_GETBUILTINLOC_5, "clang::VAArgExpr::getBuiltinLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_27, "clang::VAArgExpr::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_131, "clang::VAArgExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_130, "clang::VAArgExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_117, "clang::VAArgExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_VAARGEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ENUMDECL, "clang::EnumDecl");
  g.add_edge((CLASS_ENUMDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ENUMDECL, META_SUBCLASS, CLASS_TAGDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETCANONICALDECL_9, "clang::EnumDecl::getCanonicalDecl"),
      add_named_node(&mut g, METHOD_GETPREVIOUSDECL_3, "clang::EnumDecl::getPreviousDecl"),
      add_named_node(&mut g, METHOD_GETMOSTRECENTDECL_4, "clang::EnumDecl::getMostRecentDecl"),
      add_named_node(&mut g, METHOD_GETDEFINITION_1, "clang::EnumDecl::getDefinition"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_8, "clang::EnumDecl::getSourceRange"),
      add_named_node(&mut g, METHOD_ENUMERATORS, "clang::EnumDecl::enumerators"),
      add_named_node(&mut g, METHOD_ENUMERATOR_BEGIN, "clang::EnumDecl::enumerator_begin"),
      add_named_node(&mut g, METHOD_ENUMERATOR_END, "clang::EnumDecl::enumerator_end"),
      add_named_node(&mut g, METHOD_GETPROMOTIONTYPE, "clang::EnumDecl::getPromotionType"),
      add_named_node(&mut g, METHOD_GETINTEGERTYPE, "clang::EnumDecl::getIntegerType"),
      add_named_node(&mut g, METHOD_GETINTEGERTYPESOURCEINFO, "clang::EnumDecl::getIntegerTypeSourceInfo"),
      add_named_node(&mut g, METHOD_GETINTEGERTYPERANGE, "clang::EnumDecl::getIntegerTypeRange"),
      add_named_node(&mut g, METHOD_GETNUMPOSITIVEBITS, "clang::EnumDecl::getNumPositiveBits"),
      add_named_node(&mut g, METHOD_GETNUMNEGATIVEBITS, "clang::EnumDecl::getNumNegativeBits"),
      add_named_node(&mut g, METHOD_ISSCOPED, "clang::EnumDecl::isScoped"),
      add_named_node(&mut g, METHOD_ISSCOPEDUSINGCLASSTAG, "clang::EnumDecl::isScopedUsingClassTag"),
      add_named_node(&mut g, METHOD_ISFIXED, "clang::EnumDecl::isFixed"),
      add_named_node(&mut g, METHOD_ISCOMPLETE, "clang::EnumDecl::isComplete"),
      add_named_node(&mut g, METHOD_ISCLOSED, "clang::EnumDecl::isClosed"),
      add_named_node(&mut g, METHOD_ISCLOSEDFLAG, "clang::EnumDecl::isClosedFlag"),
      add_named_node(&mut g, METHOD_ISCLOSEDNONFLAG, "clang::EnumDecl::isClosedNonFlag"),
      add_named_node(&mut g, METHOD_GETTEMPLATEINSTANTIATIONPATTERN_1, "clang::EnumDecl::getTemplateInstantiationPattern"),
      add_named_node(&mut g, METHOD_GETINSTANTIATEDFROMMEMBERENUM, "clang::EnumDecl::getInstantiatedFromMemberEnum"),
      add_named_node(&mut g, METHOD_GETTEMPLATESPECIALIZATIONKIND_1, "clang::EnumDecl::getTemplateSpecializationKind"),
      add_named_node(&mut g, METHOD_GETMEMBERSPECIALIZATIONINFO_1, "clang::EnumDecl::getMemberSpecializationInfo"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ENUMDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_POINTERATTR, "clang::PointerAttr");
  g.add_edge((CLASS_POINTERATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_POINTERATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_AARCH64VECTORPCSATTR, "clang::AArch64VectorPcsAttr");
  g.add_edge((CLASS_AARCH64VECTORPCSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_AARCH64VECTORPCSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_ADJUSTEDTYPELOC, "clang::AdjustedTypeLoc");
  g.add_edge((CLASS_ADJUSTEDTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETORIGINALLOC, "clang::AdjustedTypeLoc::getOriginalLoc"),
      add_named_node(&mut g, METHOD_GETINNERTYPE_3, "clang::AdjustedTypeLoc::getInnerType"),
      add_named_node(&mut g, METHOD_GETLOCALSOURCERANGE_3, "clang::AdjustedTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, METHOD_GETLOCALDATASIZE, "clang::AdjustedTypeLoc::getLocalDataSize"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ADJUSTEDTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MEMBEREXPR, "clang::MemberExpr");
  g.add_edge((CLASS_MEMBEREXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MEMBEREXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETBASE_6, "clang::MemberExpr::getBase"),
      add_named_node(&mut g, METHOD_GETMEMBERDECL, "clang::MemberExpr::getMemberDecl"),
      add_named_node(&mut g, METHOD_GETFOUNDDECL_3, "clang::MemberExpr::getFoundDecl"),
      add_named_node(&mut g, METHOD_HASQUALIFIER_2, "clang::MemberExpr::hasQualifier"),
      add_named_node(&mut g, METHOD_GETQUALIFIERLOC_14, "clang::MemberExpr::getQualifierLoc"),
      add_named_node(&mut g, METHOD_GETQUALIFIER_15, "clang::MemberExpr::getQualifier"),
      add_named_node(&mut g, METHOD_GETTEMPLATEKEYWORDLOC_5, "clang::MemberExpr::getTemplateKeywordLoc"),
      add_named_node(&mut g, METHOD_GETLANGLELOC_3, "clang::MemberExpr::getLAngleLoc"),
      add_named_node(&mut g, METHOD_GETRANGLELOC_3, "clang::MemberExpr::getRAngleLoc"),
      add_named_node(&mut g, METHOD_HASTEMPLATEKEYWORD_3, "clang::MemberExpr::hasTemplateKeyword"),
      add_named_node(&mut g, METHOD_HASEXPLICITTEMPLATEARGS_4, "clang::MemberExpr::hasExplicitTemplateArgs"),
      add_named_node(&mut g, METHOD_GETTEMPLATEARGS_5, "clang::MemberExpr::getTemplateArgs"),
      add_named_node(&mut g, METHOD_GETNUMTEMPLATEARGS_3, "clang::MemberExpr::getNumTemplateArgs"),
      add_named_node(&mut g, METHOD_TEMPLATE_ARGUMENTS_5, "clang::MemberExpr::template_arguments"),
      add_named_node(&mut g, METHOD_GETMEMBERNAMEINFO_1, "clang::MemberExpr::getMemberNameInfo"),
      add_named_node(&mut g, METHOD_GETOPERATORLOC_6, "clang::MemberExpr::getOperatorLoc"),
      add_named_node(&mut g, METHOD_ISARROW_4, "clang::MemberExpr::isArrow"),
      add_named_node(&mut g, METHOD_GETMEMBERLOC_2, "clang::MemberExpr::getMemberLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_97, "clang::MemberExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_96, "clang::MemberExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETEXPRLOC_9, "clang::MemberExpr::getExprLoc"),
      add_named_node(&mut g, METHOD_ISIMPLICITACCESS_2, "clang::MemberExpr::isImplicitAccess"),
      add_named_node(&mut g, METHOD_HADMULTIPLECANDIDATES_2, "clang::MemberExpr::hadMultipleCandidates"),
      add_named_node(&mut g, METHOD_ISNONODRUSE_1, "clang::MemberExpr::isNonOdrUse"),
      add_named_node(&mut g, METHOD_CHILDREN_85, "clang::MemberExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MEMBEREXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_AMDGPUNUMVGPRATTR, "clang::AMDGPUNumVGPRAttr");
  g.add_edge((CLASS_AMDGPUNUMVGPRATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_AMDGPUNUMVGPRATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CXXPARENLISTINITEXPR, "clang::CXXParenListInitExpr");
  g.add_edge((CLASS_CXXPARENLISTINITEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXPARENLISTINITEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETINITEXPRS, "clang::CXXParenListInitExpr::getInitExprs"),
      add_named_node(&mut g, METHOD_GETUSERSPECIFIEDINITEXPRS, "clang::CXXParenListInitExpr::getUserSpecifiedInitExprs"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_36, "clang::CXXParenListInitExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_35, "clang::CXXParenListInitExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETINITLOC, "clang::CXXParenListInitExpr::getInitLoc"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_42, "clang::CXXParenListInitExpr::getSourceRange"),
      add_named_node(&mut g, METHOD_GETARRAYFILLER, "clang::CXXParenListInitExpr::getArrayFiller"),
      add_named_node(&mut g, METHOD_GETINITIALIZEDFIELDINUNION, "clang::CXXParenListInitExpr::getInitializedFieldInUnion"),
      add_named_node(&mut g, METHOD_CHILDREN_27, "clang::CXXParenListInitExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXPARENLISTINITEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEDUCEDTYPE, "clang::DeducedType");
  g.add_edge((CLASS_DEDUCEDTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEDUCEDTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_ISSUGARED_10, "clang::DeducedType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_10, "clang::DeducedType::desugar"),
      add_named_node(&mut g, METHOD_GETDEDUCEDTYPE, "clang::DeducedType::getDeducedType"),
      add_named_node(&mut g, METHOD_ISDEDUCED, "clang::DeducedType::isDeduced"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEDUCEDTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PARAMTYPESTATEATTR, "clang::ParamTypestateAttr");
  g.add_edge((CLASS_PARAMTYPESTATEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PARAMTYPESTATEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_BPFPRESERVESTATICOFFSETATTR, "clang::BPFPreserveStaticOffsetAttr");
  g.add_edge((CLASS_BPFPRESERVESTATICOFFSETATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BPFPRESERVESTATICOFFSETATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OMPCAPTUREDEXPRDECL, "clang::OMPCapturedExprDecl");
  g.add_edge((CLASS_OMPCAPTUREDEXPRDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPCAPTUREDEXPRDECL, META_SUBCLASS, CLASS_VARDECL));

  g.add_named_node(CLASS_MIPS16ATTR, "clang::Mips16Attr");
  g.add_edge((CLASS_MIPS16ATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MIPS16ATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OBJCATSYNCHRONIZEDSTMT, "clang::ObjCAtSynchronizedStmt");
  g.add_edge((CLASS_OBJCATSYNCHRONIZEDSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCATSYNCHRONIZEDSTMT, META_SUBCLASS, CLASS_STMT));

  g.add_named_node(CLASS_ATOMICEXPR, "clang::AtomicExpr");
  g.add_edge((CLASS_ATOMICEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ATOMICEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETPTR, "clang::AtomicExpr::getPtr"),
      add_named_node(&mut g, METHOD_GETORDER, "clang::AtomicExpr::getOrder"),
      add_named_node(&mut g, METHOD_GETSCOPE, "clang::AtomicExpr::getScope"),
      add_named_node(&mut g, METHOD_GETVAL1, "clang::AtomicExpr::getVal1"),
      add_named_node(&mut g, METHOD_GETORDERFAIL, "clang::AtomicExpr::getOrderFail"),
      add_named_node(&mut g, METHOD_GETVAL2, "clang::AtomicExpr::getVal2"),
      add_named_node(&mut g, METHOD_GETWEAK, "clang::AtomicExpr::getWeak"),
      add_named_node(&mut g, METHOD_GETVALUETYPE_1, "clang::AtomicExpr::getValueType"),
      add_named_node(&mut g, METHOD_GETOP, "clang::AtomicExpr::getOp"),
      add_named_node(&mut g, METHOD_GETOPASSTRING, "clang::AtomicExpr::getOpAsString"),
      add_named_node(&mut g, METHOD_GETNUMSUBEXPRS, "clang::AtomicExpr::getNumSubExprs"),
      add_named_node(&mut g, METHOD_GETSUBEXPRS, "clang::AtomicExpr::getSubExprs"),
      add_named_node(&mut g, METHOD_ISVOLATILE_3, "clang::AtomicExpr::isVolatile"),
      add_named_node(&mut g, METHOD_ISCMPXCHG, "clang::AtomicExpr::isCmpXChg"),
      add_named_node(&mut g, METHOD_ISOPENCL, "clang::AtomicExpr::isOpenCL"),
      add_named_node(&mut g, METHOD_GETBUILTINLOC_1, "clang::AtomicExpr::getBuiltinLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_3, "clang::AtomicExpr::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_11, "clang::AtomicExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_10, "clang::AtomicExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_7, "clang::AtomicExpr::children"),
      add_named_node(&mut g, METHOD_GETSCOPEMODEL, "clang::AtomicExpr::getScopeModel"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ATOMICEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DECLORSTMTATTR, "clang::DeclOrStmtAttr");
  g.add_edge((CLASS_DECLORSTMTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DECLORSTMTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_TARGETATTR, "clang::TargetAttr");
  g.add_edge((CLASS_TARGETATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TARGETATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_AMDGPUNUMSGPRATTR, "clang::AMDGPUNumSGPRAttr");
  g.add_edge((CLASS_AMDGPUNUMSGPRATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_AMDGPUNUMSGPRATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_NVPTXKERNELATTR, "clang::NVPTXKernelAttr");
  g.add_edge((CLASS_NVPTXKERNELATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NVPTXKERNELATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_PTR64ATTR, "clang::Ptr64Attr");
  g.add_edge((CLASS_PTR64ATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PTR64ATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_OFFSETOFEXPR, "clang::OffsetOfExpr");
  g.add_edge((CLASS_OFFSETOFEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OFFSETOFEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETOPERATORLOC_7, "clang::OffsetOfExpr::getOperatorLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_19, "clang::OffsetOfExpr::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETTYPESOURCEINFO_7, "clang::OffsetOfExpr::getTypeSourceInfo"),
      add_named_node(&mut g, METHOD_GETNUMCOMPONENTS, "clang::OffsetOfExpr::getNumComponents"),
      add_named_node(&mut g, METHOD_GETNUMEXPRESSIONS, "clang::OffsetOfExpr::getNumExpressions"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_100, "clang::OffsetOfExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_99, "clang::OffsetOfExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_88, "clang::OffsetOfExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OFFSETOFEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_LABELDECL, "clang::LabelDecl");
  g.add_edge((CLASS_LABELDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LABELDECL, META_SUBCLASS, CLASS_NAMEDDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSTMT, "clang::LabelDecl::getStmt"),
      add_named_node(&mut g, METHOD_ISGNULOCAL, "clang::LabelDecl::isGnuLocal"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_16, "clang::LabelDecl::getSourceRange"),
      add_named_node(&mut g, METHOD_ISMSASMLABEL, "clang::LabelDecl::isMSAsmLabel"),
      add_named_node(&mut g, METHOD_ISRESOLVEDMSASMLABEL, "clang::LabelDecl::isResolvedMSAsmLabel"),
      add_named_node(&mut g, METHOD_GETMSASMLABEL, "clang::LabelDecl::getMSAsmLabel"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_LABELDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_VARIABLEARRAYTYPE, "clang::VariableArrayType");
  g.add_edge((CLASS_VARIABLEARRAYTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_VARIABLEARRAYTYPE, META_SUBCLASS, CLASS_ARRAYTYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSIZEEXPR_4, "clang::VariableArrayType::getSizeExpr"),
      add_named_node(&mut g, METHOD_GETBRACKETSRANGE_1, "clang::VariableArrayType::getBracketsRange"),
      add_named_node(&mut g, METHOD_GETLBRACKETLOC_1, "clang::VariableArrayType::getLBracketLoc"),
      add_named_node(&mut g, METHOD_GETRBRACKETLOC_1, "clang::VariableArrayType::getRBracketLoc"),
      add_named_node(&mut g, METHOD_ISSUGARED_45, "clang::VariableArrayType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_45, "clang::VariableArrayType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_VARIABLEARRAYTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OPENCLGLOBALHOSTADDRESSSPACEATTR, "clang::OpenCLGlobalHostAddressSpaceAttr");
  g.add_edge((CLASS_OPENCLGLOBALHOSTADDRESSSPACEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OPENCLGLOBALHOSTADDRESSSPACEATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_ARMNEWATTR, "clang::ArmNewAttr");
  g.add_edge((CLASS_ARMNEWATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARMNEWATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_PIPETYPELOC, "clang::PipeTypeLoc");
  g.add_edge((CLASS_PIPETYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETVALUELOC, "clang::PipeTypeLoc::getValueLoc"),
      add_named_node(&mut g, METHOD_GETLOCALSOURCERANGE_4, "clang::PipeTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, METHOD_GETKWLOC, "clang::PipeTypeLoc::getKWLoc"),
      add_named_node(&mut g, METHOD_GETINNERTYPE_4, "clang::PipeTypeLoc::getInnerType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PIPETYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ACQUIRECAPABILITYATTR, "clang::AcquireCapabilityAttr");
  g.add_edge((CLASS_ACQUIRECAPABILITYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ACQUIRECAPABILITYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CAPTUREDSTMT, "clang::CapturedStmt");
  g.add_edge((CLASS_CAPTUREDSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CAPTUREDSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETCAPTUREDSTMT, "clang::CapturedStmt::getCapturedStmt"),
      add_named_node(&mut g, METHOD_GETCAPTUREDDECL, "clang::CapturedStmt::getCapturedDecl"),
      add_named_node(&mut g, METHOD_GETCAPTUREDREGIONKIND, "clang::CapturedStmt::getCapturedRegionKind"),
      add_named_node(&mut g, METHOD_GETCAPTUREDRECORDDECL, "clang::CapturedStmt::getCapturedRecordDecl"),
      add_named_node(&mut g, METHOD_CAPTURES_2, "clang::CapturedStmt::captures"),
      add_named_node(&mut g, METHOD_CAPTURE_BEGIN_1, "clang::CapturedStmt::capture_begin"),
      add_named_node(&mut g, METHOD_CAPTURE_END_1, "clang::CapturedStmt::capture_end"),
      add_named_node(&mut g, METHOD_CAPTURE_SIZE, "clang::CapturedStmt::capture_size"),
      add_named_node(&mut g, METHOD_CAPTURE_INITS, "clang::CapturedStmt::capture_inits"),
      add_named_node(&mut g, METHOD_CAPTURE_INIT_BEGIN_1, "clang::CapturedStmt::capture_init_begin"),
      add_named_node(&mut g, METHOD_CAPTURE_INIT_END_1, "clang::CapturedStmt::capture_init_end"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_49, "clang::CapturedStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_48, "clang::CapturedStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_47, "clang::CapturedStmt::getSourceRange"),
      add_named_node(&mut g, METHOD_CHILDREN_38, "clang::CapturedStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CAPTUREDSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ARMOUTATTR, "clang::ArmOutAttr");
  g.add_edge((CLASS_ARMOUTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARMOUTATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_ARMINTERRUPTATTR, "clang::ARMInterruptAttr");
  g.add_edge((CLASS_ARMINTERRUPTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARMINTERRUPTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_BUILTINBITCASTEXPR, "clang::BuiltinBitCastExpr");
  g.add_edge((CLASS_BUILTINBITCASTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BUILTINBITCASTEXPR, META_SUBCLASS, CLASS_EXPLICITCASTEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETBEGINLOC_17, "clang::BuiltinBitCastExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_16, "clang::BuiltinBitCastExpr::getEndLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BUILTINBITCASTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CONSTRUCTORUSINGSHADOWDECL, "clang::ConstructorUsingShadowDecl");
  g.add_edge((CLASS_CONSTRUCTORUSINGSHADOWDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONSTRUCTORUSINGSHADOWDECL, META_SUBCLASS, CLASS_USINGSHADOWDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETINTRODUCER, "clang::ConstructorUsingShadowDecl::getIntroducer"),
      add_named_node(&mut g, METHOD_GETPARENT_1, "clang::ConstructorUsingShadowDecl::getParent"),
      add_named_node(&mut g, METHOD_GETNOMINATEDBASECLASSSHADOWDECL, "clang::ConstructorUsingShadowDecl::getNominatedBaseClassShadowDecl"),
      add_named_node(&mut g, METHOD_GETCONSTRUCTEDBASECLASSSHADOWDECL, "clang::ConstructorUsingShadowDecl::getConstructedBaseClassShadowDecl"),
      add_named_node(&mut g, METHOD_GETNOMINATEDBASECLASS, "clang::ConstructorUsingShadowDecl::getNominatedBaseClass"),
      add_named_node(&mut g, METHOD_GETCONSTRUCTEDBASECLASS, "clang::ConstructorUsingShadowDecl::getConstructedBaseClass"),
      add_named_node(&mut g, METHOD_CONSTRUCTSVIRTUALBASE, "clang::ConstructorUsingShadowDecl::constructsVirtualBase"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CONSTRUCTORUSINGSHADOWDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_STMTATTR, "clang::StmtAttr");
  g.add_edge((CLASS_STMTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_STMTATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_NOMERGEATTR, "clang::NoMergeAttr");
  g.add_edge((CLASS_NOMERGEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOMERGEATTR, META_SUBCLASS, CLASS_DECLORSTMTATTR));

  g.add_named_node(CLASS_TAGDECL, "clang::TagDecl");
  g.add_edge((CLASS_TAGDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TAGDECL, META_SUBCLASS, CLASS_TYPEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETBRACERANGE, "clang::TagDecl::getBraceRange"),
      add_named_node(&mut g, METHOD_GETINNERLOCSTART_1, "clang::TagDecl::getInnerLocStart"),
      add_named_node(&mut g, METHOD_GETOUTERLOCSTART_1, "clang::TagDecl::getOuterLocStart"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_23, "clang::TagDecl::getSourceRange"),
      add_named_node(&mut g, METHOD_GETCANONICALDECL_17, "clang::TagDecl::getCanonicalDecl"),
      add_named_node(&mut g, METHOD_ISTHISDECLARATIONADEFINITION_3, "clang::TagDecl::isThisDeclarationADefinition"),
      add_named_node(&mut g, METHOD_ISCOMPLETEDEFINITION, "clang::TagDecl::isCompleteDefinition"),
      add_named_node(&mut g, METHOD_ISCOMPLETEDEFINITIONREQUIRED, "clang::TagDecl::isCompleteDefinitionRequired"),
      add_named_node(&mut g, METHOD_ISBEINGDEFINED_1, "clang::TagDecl::isBeingDefined"),
      add_named_node(&mut g, METHOD_ISEMBEDDEDINDECLARATOR, "clang::TagDecl::isEmbeddedInDeclarator"),
      add_named_node(&mut g, METHOD_ISFREESTANDING, "clang::TagDecl::isFreeStanding"),
      add_named_node(&mut g, METHOD_MAYHAVEOUTOFDATEDEF, "clang::TagDecl::mayHaveOutOfDateDef"),
      add_named_node(&mut g, METHOD_ISDEPENDENTTYPE, "clang::TagDecl::isDependentType"),
      add_named_node(&mut g, METHOD_ISTHISDECLARATIONADEMOTEDDEFINITION, "clang::TagDecl::isThisDeclarationADemotedDefinition"),
      add_named_node(&mut g, METHOD_GETDEFINITION_4, "clang::TagDecl::getDefinition"),
      add_named_node(&mut g, METHOD_GETKINDNAME, "clang::TagDecl::getKindName"),
      add_named_node(&mut g, METHOD_GETTAGKIND, "clang::TagDecl::getTagKind"),
      add_named_node(&mut g, METHOD_ISSTRUCT, "clang::TagDecl::isStruct"),
      add_named_node(&mut g, METHOD_ISINTERFACE, "clang::TagDecl::isInterface"),
      add_named_node(&mut g, METHOD_ISCLASS, "clang::TagDecl::isClass"),
      add_named_node(&mut g, METHOD_ISUNION, "clang::TagDecl::isUnion"),
      add_named_node(&mut g, METHOD_ISENUM, "clang::TagDecl::isEnum"),
      add_named_node(&mut g, METHOD_HASNAMEFORLINKAGE, "clang::TagDecl::hasNameForLinkage"),
      add_named_node(&mut g, METHOD_GETTYPEDEFNAMEFORANONDECL, "clang::TagDecl::getTypedefNameForAnonDecl"),
      add_named_node(&mut g, METHOD_GETQUALIFIER_5, "clang::TagDecl::getQualifier"),
      add_named_node(&mut g, METHOD_GETQUALIFIERLOC_2, "clang::TagDecl::getQualifierLoc"),
      add_named_node(&mut g, METHOD_GETNUMTEMPLATEPARAMETERLISTS_1, "clang::TagDecl::getNumTemplateParameterLists"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TAGDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SWIFTASYNCNAMEATTR, "clang::SwiftAsyncNameAttr");
  g.add_edge((CLASS_SWIFTASYNCNAMEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTASYNCNAMEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_SPECULATIVELOADHARDENINGATTR, "clang::SpeculativeLoadHardeningAttr");
  g.add_edge((CLASS_SPECULATIVELOADHARDENINGATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SPECULATIVELOADHARDENINGATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CUDADEVICEATTR, "clang::CUDADeviceAttr");
  g.add_edge((CLASS_CUDADEVICEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CUDADEVICEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_PASSOBJECTSIZEATTR, "clang::PassObjectSizeAttr");
  g.add_edge((CLASS_PASSOBJECTSIZEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PASSOBJECTSIZEATTR, META_SUBCLASS, CLASS_INHERITABLEPARAMATTR));

  g.add_named_node(CLASS_HLSLANNOTATIONATTR, "clang::HLSLAnnotationAttr");
  g.add_edge((CLASS_HLSLANNOTATIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_HLSLANNOTATIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_BLOCKPOINTERTYPE, "clang::BlockPointerType");
  g.add_edge((CLASS_BLOCKPOINTERTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BLOCKPOINTERTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETPOINTEETYPE, "clang::BlockPointerType::getPointeeType"),
      add_named_node(&mut g, METHOD_ISSUGARED_5, "clang::BlockPointerType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_5, "clang::BlockPointerType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BLOCKPOINTERTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OPENCLACCESSATTR, "clang::OpenCLAccessAttr");
  g.add_edge((CLASS_OPENCLACCESSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OPENCLACCESSATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_TYPEATTR, "clang::TypeAttr");
  g.add_edge((CLASS_TYPEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPEATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_OPENCLUNROLLHINTATTR, "clang::OpenCLUnrollHintAttr");
  g.add_edge((CLASS_OPENCLUNROLLHINTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OPENCLUNROLLHINTATTR, META_SUBCLASS, CLASS_STMTATTR));

  g.add_named_node(CLASS_IMPLICITVALUEINITEXPR, "clang::ImplicitValueInitExpr");
  g.add_edge((CLASS_IMPLICITVALUEINITEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_IMPLICITVALUEINITEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETBEGINLOC_85, "clang::ImplicitValueInitExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_84, "clang::ImplicitValueInitExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_73, "clang::ImplicitValueInitExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_IMPLICITVALUEINITEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ARMBUILTINALIASATTR, "clang::ArmBuiltinAliasAttr");
  g.add_edge((CLASS_ARMBUILTINALIASATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARMBUILTINALIASATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_SELECTANYATTR, "clang::SelectAnyAttr");
  g.add_edge((CLASS_SELECTANYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SELECTANYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_COLDATTR, "clang::ColdAttr");
  g.add_edge((CLASS_COLDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_COLDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OMPPARALLELMASTERDIRECTIVE, "clang::OMPParallelMasterDirective");
  g.add_edge((CLASS_OMPPARALLELMASTERDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPPARALLELMASTERDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_ACQUIREDBEFOREATTR, "clang::AcquiredBeforeAttr");
  g.add_edge((CLASS_ACQUIREDBEFOREATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ACQUIREDBEFOREATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CONSTANTMATRIXTYPE, "clang::ConstantMatrixType");
  g.add_edge((CLASS_CONSTANTMATRIXTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONSTANTMATRIXTYPE, META_SUBCLASS, CLASS_MATRIXTYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETNUMROWS, "clang::ConstantMatrixType::getNumRows"),
      add_named_node(&mut g, METHOD_GETNUMCOLUMNS, "clang::ConstantMatrixType::getNumColumns"),
      add_named_node(&mut g, METHOD_GETNUMELEMENTSFLATTENED, "clang::ConstantMatrixType::getNumElementsFlattened"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CONSTANTMATRIXTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ALIGNMAC68KATTR, "clang::AlignMac68kAttr");
  g.add_edge((CLASS_ALIGNMAC68KATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ALIGNMAC68KATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_BUILTINATTR, "clang::BuiltinAttr");
  g.add_edge((CLASS_BUILTINATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BUILTINATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_NOCOMMONATTR, "clang::NoCommonAttr");
  g.add_edge((CLASS_NOCOMMONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOCOMMONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_PRAGMACLANGDATASECTIONATTR, "clang::PragmaClangDataSectionAttr");
  g.add_edge((CLASS_PRAGMACLANGDATASECTIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PRAGMACLANGDATASECTIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_VISIBILITYATTR, "clang::VisibilityAttr");
  g.add_edge((CLASS_VISIBILITYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_VISIBILITYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OMPTARGETSIMDDIRECTIVE, "clang::OMPTargetSimdDirective");
  g.add_edge((CLASS_OMPTARGETSIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETSIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_CFICANONICALJUMPTABLEATTR, "clang::CFICanonicalJumpTableAttr");
  g.add_edge((CLASS_CFICANONICALJUMPTABLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CFICANONICALJUMPTABLEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CFRETURNSRETAINEDATTR, "clang::CFReturnsRetainedAttr");
  g.add_edge((CLASS_CFRETURNSRETAINEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CFRETURNSRETAINEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_COMPLEXTYPE, "clang::ComplexType");
  g.add_edge((CLASS_COMPLEXTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_COMPLEXTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETELEMENTTYPE_1, "clang::ComplexType::getElementType"),
      add_named_node(&mut g, METHOD_ISSUGARED_7, "clang::ComplexType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_7, "clang::ComplexType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_COMPLEXTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_USEHANDLEATTR, "clang::UseHandleAttr");
  g.add_edge((CLASS_USEHANDLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_USEHANDLEATTR, META_SUBCLASS, CLASS_INHERITABLEPARAMATTR));

  g.add_named_node(CLASS_CXXREINTERPRETCASTEXPR, "clang::CXXReinterpretCastExpr");
  g.add_edge((CLASS_CXXREINTERPRETCASTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXREINTERPRETCASTEXPR, META_SUBCLASS, CLASS_CXXNAMEDCASTEXPR));

  g.add_named_node(CLASS_DEPENDENTSIZEDEXTVECTORTYPELOC, "clang::DependentSizedExtVectorTypeLoc");
  g.add_edge((CLASS_DEPENDENTSIZEDEXTVECTORTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETNAMELOC_3, "clang::DependentSizedExtVectorTypeLoc::getNameLoc"),
      add_named_node(&mut g, METHOD_GETLOCALSOURCERANGE_5, "clang::DependentSizedExtVectorTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, METHOD_GETELEMENTLOC_1, "clang::DependentSizedExtVectorTypeLoc::getElementLoc"),
      add_named_node(&mut g, METHOD_GETINNERTYPE_5, "clang::DependentSizedExtVectorTypeLoc::getInnerType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTSIZEDEXTVECTORTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPDECLARESIMDDECLATTR, "clang::OMPDeclareSimdDeclAttr");
  g.add_edge((CLASS_OMPDECLARESIMDDECLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPDECLARESIMDDECLATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_OBJCROOTCLASSATTR, "clang::ObjCRootClassAttr");
  g.add_edge((CLASS_OBJCROOTCLASSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCROOTCLASSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_TARGETCLONESATTR, "clang::TargetClonesAttr");
  g.add_edge((CLASS_TARGETCLONESATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TARGETCLONESATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_TEMPLATETYPEPARMTYPE, "clang::TemplateTypeParmType");
  g.add_edge((CLASS_TEMPLATETYPEPARMTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TEMPLATETYPEPARMTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETDEPTH, "clang::TemplateTypeParmType::getDepth"),
      add_named_node(&mut g, METHOD_GETINDEX_2, "clang::TemplateTypeParmType::getIndex"),
      add_named_node(&mut g, METHOD_ISPARAMETERPACK, "clang::TemplateTypeParmType::isParameterPack"),
      add_named_node(&mut g, METHOD_GETDECL_4, "clang::TemplateTypeParmType::getDecl"),
      add_named_node(&mut g, METHOD_GETIDENTIFIER_3, "clang::TemplateTypeParmType::getIdentifier"),
      add_named_node(&mut g, METHOD_ISSUGARED_38, "clang::TemplateTypeParmType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_38, "clang::TemplateTypeParmType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TEMPLATETYPEPARMTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_GCCASMSTMT, "clang::GCCAsmStmt");
  g.add_edge((CLASS_GCCASMSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_GCCASMSTMT, META_SUBCLASS, CLASS_ASMSTMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETRPARENLOC_16, "clang::GCCAsmStmt::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETASMSTRING_1, "clang::GCCAsmStmt::getAsmString"),
      add_named_node(&mut g, METHOD_ISASMGOTO, "clang::GCCAsmStmt::isAsmGoto"),
      add_named_node(&mut g, METHOD_GETNUMLABELS, "clang::GCCAsmStmt::getNumLabels"),
      add_named_node(&mut g, METHOD_BEGIN_LABELS, "clang::GCCAsmStmt::begin_labels"),
      add_named_node(&mut g, METHOD_END_LABELS, "clang::GCCAsmStmt::end_labels"),
      add_named_node(&mut g, METHOD_LABELS, "clang::GCCAsmStmt::labels"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_78, "clang::GCCAsmStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_77, "clang::GCCAsmStmt::getEndLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_GCCASMSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PRAGMACLANGRELROSECTIONATTR, "clang::PragmaClangRelroSectionAttr");
  g.add_edge((CLASS_PRAGMACLANGRELROSECTIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PRAGMACLANGRELROSECTIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_AMDGPUFLATWORKGROUPSIZEATTR, "clang::AMDGPUFlatWorkGroupSizeAttr");
  g.add_edge((CLASS_AMDGPUFLATWORKGROUPSIZEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_AMDGPUFLATWORKGROUPSIZEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_RENDERSCRIPTKERNELATTR, "clang::RenderScriptKernelAttr");
  g.add_edge((CLASS_RENDERSCRIPTKERNELATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RENDERSCRIPTKERNELATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_ASSUMEALIGNEDATTR, "clang::AssumeAlignedAttr");
  g.add_edge((CLASS_ASSUMEALIGNEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ASSUMEALIGNEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_POINTERTYPELOC, "clang::PointerTypeLoc");
  g.add_edge((CLASS_POINTERTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSTARLOC_1, "clang::PointerTypeLoc::getStarLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_POINTERTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_AVAILABLEONLYINDEFAULTEVALMETHODATTR, "clang::AvailableOnlyInDefaultEvalMethodAttr");
  g.add_edge((CLASS_AVAILABLEONLYINDEFAULTEVALMETHODATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_AVAILABLEONLYINDEFAULTEVALMETHODATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OBJCPROPERTYDECL, "clang::ObjCPropertyDecl");
  g.add_edge((CLASS_OBJCPROPERTYDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCPROPERTYDECL, META_SUBCLASS, CLASS_NAMEDDECL));

  g.add_named_node(CLASS_PRAGMACLANGTEXTSECTIONATTR, "clang::PragmaClangTextSectionAttr");
  g.add_edge((CLASS_PRAGMACLANGTEXTSECTIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PRAGMACLANGTEXTSECTIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CODESEGATTR, "clang::CodeSegAttr");
  g.add_edge((CLASS_CODESEGATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CODESEGATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OMPPARALLELFORSIMDDIRECTIVE, "clang::OMPParallelForSimdDirective");
  g.add_edge((CLASS_OMPPARALLELFORSIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPPARALLELFORSIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_WEBASSEMBLYFUNCREFATTR, "clang::WebAssemblyFuncrefAttr");
  g.add_edge((CLASS_WEBASSEMBLYFUNCREFATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_WEBASSEMBLYFUNCREFATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_CONSUMABLESETONREADATTR, "clang::ConsumableSetOnReadAttr");
  g.add_edge((CLASS_CONSUMABLESETONREADATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONSUMABLESETONREADATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CASTEXPR, "clang::CastExpr");
  g.add_edge((CLASS_CASTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CASTEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETCASTKIND, "clang::CastExpr::getCastKind"),
      add_named_node(&mut g, METHOD_GETCASTKINDNAME, "clang::CastExpr::getCastKindName"),
      add_named_node(&mut g, METHOD_GETSUBEXPR_4, "clang::CastExpr::getSubExpr"),
      add_named_node(&mut g, METHOD_GETSUBEXPRASWRITTEN, "clang::CastExpr::getSubExprAsWritten"),
      add_named_node(&mut g, METHOD_GETCONVERSIONFUNCTION, "clang::CastExpr::getConversionFunction"),
      add_named_node(&mut g, METHOD_PATH_EMPTY, "clang::CastExpr::path_empty"),
      add_named_node(&mut g, METHOD_PATH_SIZE, "clang::CastExpr::path_size"),
      add_named_node(&mut g, METHOD_PATH_BEGIN, "clang::CastExpr::path_begin"),
      add_named_node(&mut g, METHOD_PATH_END, "clang::CastExpr::path_end"),
      add_named_node(&mut g, METHOD_PATH, "clang::CastExpr::path"),
      add_named_node(&mut g, METHOD_GETTARGETUNIONFIELD, "clang::CastExpr::getTargetUnionField"),
      add_named_node(&mut g, METHOD_HASSTOREDFPFEATURES_2, "clang::CastExpr::hasStoredFPFeatures"),
      add_named_node(&mut g, METHOD_GETSTOREDFPFEATURES_2, "clang::CastExpr::getStoredFPFeatures"),
      add_named_node(&mut g, METHOD_GETFPFEATURES_2, "clang::CastExpr::getFPFeatures"),
      add_named_node(&mut g, METHOD_CHANGESVOLATILEQUALIFICATION, "clang::CastExpr::changesVolatileQualification"),
      add_named_node(&mut g, METHOD_CHILDREN_40, "clang::CastExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CASTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_USEDATTR, "clang::UsedAttr");
  g.add_edge((CLASS_USEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_USEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_MSGUIDDECL, "clang::MSGuidDecl");
  g.add_edge((CLASS_MSGUIDDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MSGUIDDECL, META_SUBCLASS, CLASS_VALUEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETPARTS, "clang::MSGuidDecl::getParts"),
      add_named_node(&mut g, METHOD_GETASAPVALUE, "clang::MSGuidDecl::getAsAPValue"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MSGUIDDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SUBSTTEMPLATETYPEPARMPACKTYPE, "clang::SubstTemplateTypeParmPackType");
  g.add_edge((CLASS_SUBSTTEMPLATETYPEPARMPACKTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SUBSTTEMPLATETYPEPARMPACKTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETIDENTIFIER_2, "clang::SubstTemplateTypeParmPackType::getIdentifier"),
      add_named_node(&mut g, METHOD_GETASSOCIATEDDECL, "clang::SubstTemplateTypeParmPackType::getAssociatedDecl"),
      add_named_node(&mut g, METHOD_GETREPLACEDPARAMETER, "clang::SubstTemplateTypeParmPackType::getReplacedParameter"),
      add_named_node(&mut g, METHOD_GETINDEX, "clang::SubstTemplateTypeParmPackType::getIndex"),
      add_named_node(&mut g, METHOD_GETFINAL, "clang::SubstTemplateTypeParmPackType::getFinal"),
      add_named_node(&mut g, METHOD_GETNUMARGS, "clang::SubstTemplateTypeParmPackType::getNumArgs"),
      add_named_node(&mut g, METHOD_ISSUGARED_35, "clang::SubstTemplateTypeParmPackType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_35, "clang::SubstTemplateTypeParmPackType::desugar"),
      add_named_node(&mut g, METHOD_GETARGUMENTPACK, "clang::SubstTemplateTypeParmPackType::getArgumentPack"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SUBSTTEMPLATETYPEPARMPACKTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DIAGNOSEASBUILTINATTR, "clang::DiagnoseAsBuiltinAttr");
  g.add_edge((CLASS_DIAGNOSEASBUILTINATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DIAGNOSEASBUILTINATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_COROONLYDESTROYWHENCOMPLETEATTR, "clang::CoroOnlyDestroyWhenCompleteAttr");
  g.add_edge((CLASS_COROONLYDESTROYWHENCOMPLETEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_COROONLYDESTROYWHENCOMPLETEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_PRAGMACLANGBSSSECTIONATTR, "clang::PragmaClangBSSSectionAttr");
  g.add_edge((CLASS_PRAGMACLANGBSSSECTIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PRAGMACLANGBSSSECTIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_RETAINATTR, "clang::RetainAttr");
  g.add_edge((CLASS_RETAINATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RETAINATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CALLBACKATTR, "clang::CallbackAttr");
  g.add_edge((CLASS_CALLBACKATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CALLBACKATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CUDALAUNCHBOUNDSATTR, "clang::CUDALaunchBoundsAttr");
  g.add_edge((CLASS_CUDALAUNCHBOUNDSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CUDALAUNCHBOUNDSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_UUIDATTR, "clang::UuidAttr");
  g.add_edge((CLASS_UUIDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UUIDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_PREFERREDTYPEATTR, "clang::PreferredTypeAttr");
  g.add_edge((CLASS_PREFERREDTYPEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PREFERREDTYPEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_SUBSTNONTYPETEMPLATEPARMPACKEXPR, "clang::SubstNonTypeTemplateParmPackExpr");
  g.add_edge((CLASS_SUBSTNONTYPETEMPLATEPARMPACKEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SUBSTNONTYPETEMPLATEPARMPACKEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETASSOCIATEDDECL_3, "clang::SubstNonTypeTemplateParmPackExpr::getAssociatedDecl"),
      add_named_node(&mut g, METHOD_GETINDEX_5, "clang::SubstNonTypeTemplateParmPackExpr::getIndex"),
      add_named_node(&mut g, METHOD_GETPARAMETERPACK_1, "clang::SubstNonTypeTemplateParmPackExpr::getParameterPack"),
      add_named_node(&mut g, METHOD_GETPARAMETERPACKLOCATION_1, "clang::SubstNonTypeTemplateParmPackExpr::getParameterPackLocation"),
      add_named_node(&mut g, METHOD_GETARGUMENTPACK_1, "clang::SubstNonTypeTemplateParmPackExpr::getArgumentPack"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_121, "clang::SubstNonTypeTemplateParmPackExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_120, "clang::SubstNonTypeTemplateParmPackExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_109, "clang::SubstNonTypeTemplateParmPackExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SUBSTNONTYPETEMPLATEPARMPACKEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXPSEUDODESTRUCTOREXPR, "clang::CXXPseudoDestructorExpr");
  g.add_edge((CLASS_CXXPSEUDODESTRUCTOREXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXPSEUDODESTRUCTOREXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETBASE_1, "clang::CXXPseudoDestructorExpr::getBase"),
      add_named_node(&mut g, METHOD_HASQUALIFIER, "clang::CXXPseudoDestructorExpr::hasQualifier"),
      add_named_node(&mut g, METHOD_GETQUALIFIERLOC_9, "clang::CXXPseudoDestructorExpr::getQualifierLoc"),
      add_named_node(&mut g, METHOD_GETQUALIFIER_12, "clang::CXXPseudoDestructorExpr::getQualifier"),
      add_named_node(&mut g, METHOD_ISARROW_1, "clang::CXXPseudoDestructorExpr::isArrow"),
      add_named_node(&mut g, METHOD_GETOPERATORLOC_4, "clang::CXXPseudoDestructorExpr::getOperatorLoc"),
      add_named_node(&mut g, METHOD_GETSCOPETYPEINFO, "clang::CXXPseudoDestructorExpr::getScopeTypeInfo"),
      add_named_node(&mut g, METHOD_GETCOLONCOLONLOC, "clang::CXXPseudoDestructorExpr::getColonColonLoc"),
      add_named_node(&mut g, METHOD_GETTILDELOC, "clang::CXXPseudoDestructorExpr::getTildeLoc"),
      add_named_node(&mut g, METHOD_GETDESTROYEDTYPEINFO, "clang::CXXPseudoDestructorExpr::getDestroyedTypeInfo"),
      add_named_node(&mut g, METHOD_GETDESTROYEDTYPEIDENTIFIER, "clang::CXXPseudoDestructorExpr::getDestroyedTypeIdentifier"),
      add_named_node(&mut g, METHOD_GETDESTROYEDTYPE_1, "clang::CXXPseudoDestructorExpr::getDestroyedType"),
      add_named_node(&mut g, METHOD_GETDESTROYEDTYPELOC, "clang::CXXPseudoDestructorExpr::getDestroyedTypeLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_37, "clang::CXXPseudoDestructorExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_36, "clang::CXXPseudoDestructorExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_28, "clang::CXXPseudoDestructorExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXPSEUDODESTRUCTOREXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PTGUARDEDVARATTR, "clang::PtGuardedVarAttr");
  g.add_edge((CLASS_PTGUARDEDVARATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PTGUARDEDVARATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_IFUNCATTR, "clang::IFuncAttr");
  g.add_edge((CLASS_IFUNCATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_IFUNCATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_FUNCTIONNOPROTOTYPE, "clang::FunctionNoProtoType");
  g.add_edge((CLASS_FUNCTIONNOPROTOTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FUNCTIONNOPROTOTYPE, META_SUBCLASS, CLASS_FUNCTIONTYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_ISSUGARED_21, "clang::FunctionNoProtoType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_21, "clang::FunctionNoProtoType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FUNCTIONNOPROTOTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPMASKEDTASKLOOPDIRECTIVE, "clang::OMPMaskedTaskLoopDirective");
  g.add_edge((CLASS_OMPMASKEDTASKLOOPDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPMASKEDTASKLOOPDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_LVALUEREFERENCETYPELOC, "clang::LValueReferenceTypeLoc");
  g.add_edge((CLASS_LVALUEREFERENCETYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETAMPLOC, "clang::LValueReferenceTypeLoc::getAmpLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_LVALUEREFERENCETYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ASSERTEXCLUSIVELOCKATTR, "clang::AssertExclusiveLockAttr");
  g.add_edge((CLASS_ASSERTEXCLUSIVELOCKATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ASSERTEXCLUSIVELOCKATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_COMMONATTR, "clang::CommonAttr");
  g.add_edge((CLASS_COMMONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_COMMONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_REQDWORKGROUPSIZEATTR, "clang::ReqdWorkGroupSizeAttr");
  g.add_edge((CLASS_REQDWORKGROUPSIZEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_REQDWORKGROUPSIZEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_RETURNSTWICEATTR, "clang::ReturnsTwiceAttr");
  g.add_edge((CLASS_RETURNSTWICEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RETURNSTWICEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_UNRESOLVEDUSINGTYPENAMEDECL, "clang::UnresolvedUsingTypenameDecl");
  g.add_edge((CLASS_UNRESOLVEDUSINGTYPENAMEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNRESOLVEDUSINGTYPENAMEDECL, META_SUBCLASS, CLASS_TYPEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETUSINGLOC, "clang::UnresolvedUsingTypenameDecl::getUsingLoc"),
      add_named_node(&mut g, METHOD_GETTYPENAMELOC, "clang::UnresolvedUsingTypenameDecl::getTypenameLoc"),
      add_named_node(&mut g, METHOD_GETQUALIFIERLOC_3, "clang::UnresolvedUsingTypenameDecl::getQualifierLoc"),
      add_named_node(&mut g, METHOD_GETQUALIFIER_6, "clang::UnresolvedUsingTypenameDecl::getQualifier"),
      add_named_node(&mut g, METHOD_GETNAMEINFO_1, "clang::UnresolvedUsingTypenameDecl::getNameInfo"),
      add_named_node(&mut g, METHOD_ISPACKEXPANSION_3, "clang::UnresolvedUsingTypenameDecl::isPackExpansion"),
      add_named_node(&mut g, METHOD_GETELLIPSISLOC_2, "clang::UnresolvedUsingTypenameDecl::getEllipsisLoc"),
      add_named_node(&mut g, METHOD_GETCANONICALDECL_21, "clang::UnresolvedUsingTypenameDecl::getCanonicalDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNRESOLVEDUSINGTYPENAMEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_INITPRIORITYATTR, "clang::InitPriorityAttr");
  g.add_edge((CLASS_INITPRIORITYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_INITPRIORITYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_RESTRICTATTR, "clang::RestrictAttr");
  g.add_edge((CLASS_RESTRICTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RESTRICTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_DECLTYPETYPELOC, "clang::DecltypeTypeLoc");
  g.add_edge((CLASS_DECLTYPETYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETUNDERLYINGEXPR_2, "clang::DecltypeTypeLoc::getUnderlyingExpr"),
      add_named_node(&mut g, METHOD_GETDECLTYPELOC, "clang::DecltypeTypeLoc::getDecltypeLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_29, "clang::DecltypeTypeLoc::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETLOCALSOURCERANGE_6, "clang::DecltypeTypeLoc::getLocalSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DECLTYPETYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTARGETPARALLELFORDIRECTIVE, "clang::OMPTargetParallelForDirective");
  g.add_edge((CLASS_OMPTARGETPARALLELFORDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETPARALLELFORDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_HLSLGROUPSHAREDADDRESSSPACEATTR, "clang::HLSLGroupSharedAddressSpaceAttr");
  g.add_edge((CLASS_HLSLGROUPSHAREDADDRESSSPACEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_HLSLGROUPSHAREDADDRESSSPACEATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_BPFPRESERVEACCESSINDEXATTR, "clang::BPFPreserveAccessIndexAttr");
  g.add_edge((CLASS_BPFPRESERVEACCESSINDEXATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BPFPRESERVEACCESSINDEXATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_SWIFTASYNCERRORATTR, "clang::SwiftAsyncErrorAttr");
  g.add_edge((CLASS_SWIFTASYNCERRORATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTASYNCERRORATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_PACKEDATTR, "clang::PackedAttr");
  g.add_edge((CLASS_PACKEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PACKEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_ARRAYSUBSCRIPTEXPR, "clang::ArraySubscriptExpr");
  g.add_edge((CLASS_ARRAYSUBSCRIPTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARRAYSUBSCRIPTEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETLHS, "clang::ArraySubscriptExpr::getLHS"),
      add_named_node(&mut g, METHOD_GETRHS, "clang::ArraySubscriptExpr::getRHS"),
      add_named_node(&mut g, METHOD_GETBASE, "clang::ArraySubscriptExpr::getBase"),
      add_named_node(&mut g, METHOD_GETIDX, "clang::ArraySubscriptExpr::getIdx"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_7, "clang::ArraySubscriptExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_6, "clang::ArraySubscriptExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETRBRACKETLOC_2, "clang::ArraySubscriptExpr::getRBracketLoc"),
      add_named_node(&mut g, METHOD_GETEXPRLOC, "clang::ArraySubscriptExpr::getExprLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_3, "clang::ArraySubscriptExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ARRAYSUBSCRIPTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DECLSTMT, "clang::DeclStmt");
  g.add_edge((CLASS_DECLSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DECLSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_ISSINGLEDECL, "clang::DeclStmt::isSingleDecl"),
      add_named_node(&mut g, METHOD_GETSINGLEDECL, "clang::DeclStmt::getSingleDecl"),
      add_named_node(&mut g, METHOD_GETDECLGROUP, "clang::DeclStmt::getDeclGroup"),
      add_named_node(&mut g, METHOD_GETENDLOC_63, "clang::DeclStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_64, "clang::DeclStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_54, "clang::DeclStmt::children"),
      add_named_node(&mut g, METHOD_DECLS, "clang::DeclStmt::decls"),
      add_named_node(&mut g, METHOD_DECL_BEGIN, "clang::DeclStmt::decl_begin"),
      add_named_node(&mut g, METHOD_DECL_END, "clang::DeclStmt::decl_end"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DECLSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ALWAYSINLINEATTR, "clang::AlwaysInlineAttr");
  g.add_edge((CLASS_ALWAYSINLINEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ALWAYSINLINEATTR, META_SUBCLASS, CLASS_DECLORSTMTATTR));

  g.add_named_node(CLASS_GUARDEDBYATTR, "clang::GuardedByAttr");
  g.add_edge((CLASS_GUARDEDBYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_GUARDEDBYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_RETURNSNONNULLATTR, "clang::ReturnsNonNullAttr");
  g.add_edge((CLASS_RETURNSNONNULLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RETURNSNONNULLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_ARCWEAKREFUNAVAILABLEATTR, "clang::ArcWeakrefUnavailableAttr");
  g.add_edge((CLASS_ARCWEAKREFUNAVAILABLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARCWEAKREFUNAVAILABLEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_SYCLKERNELATTR, "clang::SYCLKernelAttr");
  g.add_edge((CLASS_SYCLKERNELATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SYCLKERNELATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_TYPENULLABLERESULTATTR, "clang::TypeNullableResultAttr");
  g.add_edge((CLASS_TYPENULLABLERESULTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPENULLABLERESULTATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_CUDADEVICEBUILTINSURFACETYPEATTR, "clang::CUDADeviceBuiltinSurfaceTypeAttr");
  g.add_edge((CLASS_CUDADEVICEBUILTINSURFACETYPEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CUDADEVICEBUILTINSURFACETYPEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_MIPSINTERRUPTATTR, "clang::MipsInterruptAttr");
  g.add_edge((CLASS_MIPSINTERRUPTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MIPSINTERRUPTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OWNERSHIPATTR, "clang::OwnershipAttr");
  g.add_edge((CLASS_OWNERSHIPATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OWNERSHIPATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_UNARYTRANSFORMTYPE, "clang::UnaryTransformType");
  g.add_edge((CLASS_UNARYTRANSFORMTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNARYTRANSFORMTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_ISSUGARED_42, "clang::UnaryTransformType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_42, "clang::UnaryTransformType::desugar"),
      add_named_node(&mut g, METHOD_GETUNDERLYINGTYPE_2, "clang::UnaryTransformType::getUnderlyingType"),
      add_named_node(&mut g, METHOD_GETBASETYPE, "clang::UnaryTransformType::getBaseType"),
      add_named_node(&mut g, METHOD_GETUTTKIND, "clang::UnaryTransformType::getUTTKind"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNARYTRANSFORMTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NSRETURNSNOTRETAINEDATTR, "clang::NSReturnsNotRetainedAttr");
  g.add_edge((CLASS_NSRETURNSNOTRETAINEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NSRETURNSNOTRETAINEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_RELEASECAPABILITYATTR, "clang::ReleaseCapabilityAttr");
  g.add_edge((CLASS_RELEASECAPABILITYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RELEASECAPABILITYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CHOOSEEXPR, "clang::ChooseExpr");
  g.add_edge((CLASS_CHOOSEEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CHOOSEEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_ISCONDITIONTRUE, "clang::ChooseExpr::isConditionTrue"),
      add_named_node(&mut g, METHOD_ISCONDITIONDEPENDENT, "clang::ChooseExpr::isConditionDependent"),
      add_named_node(&mut g, METHOD_GETCHOSENSUBEXPR, "clang::ChooseExpr::getChosenSubExpr"),
      add_named_node(&mut g, METHOD_GETCOND_3, "clang::ChooseExpr::getCond"),
      add_named_node(&mut g, METHOD_GETLHS_5, "clang::ChooseExpr::getLHS"),
      add_named_node(&mut g, METHOD_GETRHS_5, "clang::ChooseExpr::getRHS"),
      add_named_node(&mut g, METHOD_GETBUILTINLOC_2, "clang::ChooseExpr::getBuiltinLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_12, "clang::ChooseExpr::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_52, "clang::ChooseExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_51, "clang::ChooseExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_42, "clang::ChooseExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CHOOSEEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OPENCLINTELREQDSUBGROUPSIZEATTR, "clang::OpenCLIntelReqdSubGroupSizeAttr");
  g.add_edge((CLASS_OPENCLINTELREQDSUBGROUPSIZEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OPENCLINTELREQDSUBGROUPSIZEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_PUREATTR, "clang::PureAttr");
  g.add_edge((CLASS_PUREATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PUREATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_INHERITABLEATTR, "clang::InheritableAttr");
  g.add_edge((CLASS_INHERITABLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_INHERITABLEATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_OPTIMIZENONEATTR, "clang::OptimizeNoneAttr");
  g.add_edge((CLASS_OPTIMIZENONEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OPTIMIZENONEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_LTOVISIBILITYPUBLICATTR, "clang::LTOVisibilityPublicAttr");
  g.add_edge((CLASS_LTOVISIBILITYPUBLICATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LTOVISIBILITYPUBLICATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OBJCSUBCLASSINGRESTRICTEDATTR, "clang::ObjCSubclassingRestrictedAttr");
  g.add_edge((CLASS_OBJCSUBCLASSINGRESTRICTEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCSUBCLASSINGRESTRICTEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OBJCTYPEPARAMTYPE, "clang::ObjCTypeParamType");
  g.add_edge((CLASS_OBJCTYPEPARAMTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCTYPEPARAMTYPE, META_SUBCLASS, CLASS_TYPE));

  g.add_named_node(CLASS_FINALATTR, "clang::FinalAttr");
  g.add_edge((CLASS_FINALATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FINALATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_DOSTMT, "clang::DoStmt");
  g.add_edge((CLASS_DOSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DOSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETCOND_5, "clang::DoStmt::getCond"),
      add_named_node(&mut g, METHOD_GETBODY_7, "clang::DoStmt::getBody"),
      add_named_node(&mut g, METHOD_GETDOLOC, "clang::DoStmt::getDoLoc"),
      add_named_node(&mut g, METHOD_GETWHILELOC, "clang::DoStmt::getWhileLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_14, "clang::DoStmt::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_70, "clang::DoStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_69, "clang::DoStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_60, "clang::DoStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DOSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCRETURNSINNERPOINTERATTR, "clang::ObjCReturnsInnerPointerAttr");
  g.add_edge((CLASS_OBJCRETURNSINNERPOINTERATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCRETURNSINNERPOINTERATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CPUDISPATCHATTR, "clang::CPUDispatchAttr");
  g.add_edge((CLASS_CPUDISPATCHATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CPUDISPATCHATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_PTGUARDEDBYATTR, "clang::PtGuardedByAttr");
  g.add_edge((CLASS_PTGUARDEDBYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PTGUARDEDBYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OBJCREQUIRESSUPERATTR, "clang::ObjCRequiresSuperAttr");
  g.add_edge((CLASS_OBJCREQUIRESSUPERATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCREQUIRESSUPERATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OMPCRITICALDIRECTIVE, "clang::OMPCriticalDirective");
  g.add_edge((CLASS_OMPCRITICALDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPCRITICALDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_CXXTEMPORARYOBJECTEXPR, "clang::CXXTemporaryObjectExpr");
  g.add_edge((CLASS_CXXTEMPORARYOBJECTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXTEMPORARYOBJECTEXPR, META_SUBCLASS, CLASS_CXXCONSTRUCTEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTYPESOURCEINFO_3, "clang::CXXTemporaryObjectExpr::getTypeSourceInfo"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_41, "clang::CXXTemporaryObjectExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_40, "clang::CXXTemporaryObjectExpr::getEndLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXTEMPORARYOBJECTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXDEDUCTIONGUIDEDECL, "clang::CXXDeductionGuideDecl");
  g.add_edge((CLASS_CXXDEDUCTIONGUIDEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXDEDUCTIONGUIDEDECL, META_SUBCLASS, CLASS_FUNCTIONDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETEXPLICITSPECIFIER_2, "clang::CXXDeductionGuideDecl::getExplicitSpecifier"),
      add_named_node(&mut g, METHOD_ISEXPLICIT_2, "clang::CXXDeductionGuideDecl::isExplicit"),
      add_named_node(&mut g, METHOD_GETDEDUCEDTEMPLATE, "clang::CXXDeductionGuideDecl::getDeducedTemplate"),
      add_named_node(&mut g, METHOD_GETCORRESPONDINGCONSTRUCTOR, "clang::CXXDeductionGuideDecl::getCorrespondingConstructor"),
      add_named_node(&mut g, METHOD_GETDEDUCTIONCANDIDATEKIND, "clang::CXXDeductionGuideDecl::getDeductionCandidateKind"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXDEDUCTIONGUIDEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CFUNKNOWNTRANSFERATTR, "clang::CFUnknownTransferAttr");
  g.add_edge((CLASS_CFUNKNOWNTRANSFERATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CFUNKNOWNTRANSFERATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_PRESERVEMOSTATTR, "clang::PreserveMostAttr");
  g.add_edge((CLASS_PRESERVEMOSTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PRESERVEMOSTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OBJCREQUIRESPROPERTYDEFSATTR, "clang::ObjCRequiresPropertyDefsAttr");
  g.add_edge((CLASS_OBJCREQUIRESPROPERTYDEFSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCREQUIRESPROPERTYDEFSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_NODEBUGATTR, "clang::NoDebugAttr");
  g.add_edge((CLASS_NODEBUGATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NODEBUGATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_PTR32ATTR, "clang::Ptr32Attr");
  g.add_edge((CLASS_PTR32ATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PTR32ATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_HLSLRESOURCEATTR, "clang::HLSLResourceAttr");
  g.add_edge((CLASS_HLSLRESOURCEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_HLSLRESOURCEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_ARMLOCALLYSTREAMINGATTR, "clang::ArmLocallyStreamingAttr");
  g.add_edge((CLASS_ARMLOCALLYSTREAMINGATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARMLOCALLYSTREAMINGATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_TYPEWITHKEYWORD, "clang::TypeWithKeyword");
  g.add_edge((CLASS_TYPEWITHKEYWORD, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPEWITHKEYWORD, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETKEYWORD_1, "clang::TypeWithKeyword::getKeyword"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPEWITHKEYWORD, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCOWNERSHIPATTR, "clang::ObjCOwnershipAttr");
  g.add_edge((CLASS_OBJCOWNERSHIPATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCOWNERSHIPATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_VECTORTYPE, "clang::VectorType");
  g.add_edge((CLASS_VECTORTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_VECTORTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETELEMENTTYPE_6, "clang::VectorType::getElementType"),
      add_named_node(&mut g, METHOD_GETNUMELEMENTS, "clang::VectorType::getNumElements"),
      add_named_node(&mut g, METHOD_ISSUGARED_46, "clang::VectorType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_46, "clang::VectorType::desugar"),
      add_named_node(&mut g, METHOD_GETVECTORKIND_1, "clang::VectorType::getVectorKind"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_VECTORTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MINVECTORWIDTHATTR, "clang::MinVectorWidthAttr");
  g.add_edge((CLASS_MINVECTORWIDTHATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MINVECTORWIDTHATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_ENUMTYPELOC, "clang::EnumTypeLoc");
  g.add_edge((CLASS_ENUMTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETDECL_9, "clang::EnumTypeLoc::getDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ENUMTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OPENCLPRIVATEADDRESSSPACEATTR, "clang::OpenCLPrivateAddressSpaceAttr");
  g.add_edge((CLASS_OPENCLPRIVATEADDRESSSPACEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OPENCLPRIVATEADDRESSSPACEATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_EXCLUSIVETRYLOCKFUNCTIONATTR, "clang::ExclusiveTrylockFunctionAttr");
  g.add_edge((CLASS_EXCLUSIVETRYLOCKFUNCTIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_EXCLUSIVETRYLOCKFUNCTIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OMPTEAMSDISTRIBUTEPARALLELFORDIRECTIVE, "clang::OMPTeamsDistributeParallelForDirective");
  g.add_edge((CLASS_OMPTEAMSDISTRIBUTEPARALLELFORDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTEAMSDISTRIBUTEPARALLELFORDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_NOBUILTINATTR, "clang::NoBuiltinAttr");
  g.add_edge((CLASS_NOBUILTINATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOBUILTINATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_SWIFTPRIVATEATTR, "clang::SwiftPrivateAttr");
  g.add_edge((CLASS_SWIFTPRIVATEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTPRIVATEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CXXTYPEIDEXPR, "clang::CXXTypeidExpr");
  g.add_edge((CLASS_CXXTYPEIDEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXTYPEIDEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_ISPOTENTIALLYEVALUATED, "clang::CXXTypeidExpr::isPotentiallyEvaluated"),
      add_named_node(&mut g, METHOD_ISTYPEOPERAND, "clang::CXXTypeidExpr::isTypeOperand"),
      add_named_node(&mut g, METHOD_GETTYPEOPERANDSOURCEINFO, "clang::CXXTypeidExpr::getTypeOperandSourceInfo"),
      add_named_node(&mut g, METHOD_GETEXPROPERAND, "clang::CXXTypeidExpr::getExprOperand"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_45, "clang::CXXTypeidExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_44, "clang::CXXTypeidExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_45, "clang::CXXTypeidExpr::getSourceRange"),
      add_named_node(&mut g, METHOD_CHILDREN_34, "clang::CXXTypeidExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXTYPEIDEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCBRIDGERELATEDATTR, "clang::ObjCBridgeRelatedAttr");
  g.add_edge((CLASS_OBJCBRIDGERELATEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCBRIDGERELATEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OBJCEXTERNALLYRETAINEDATTR, "clang::ObjCExternallyRetainedAttr");
  g.add_edge((CLASS_OBJCEXTERNALLYRETAINEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCEXTERNALLYRETAINEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_MIPSLONGCALLATTR, "clang::MipsLongCallAttr");
  g.add_edge((CLASS_MIPSLONGCALLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MIPSLONGCALLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OBJCBRIDGEMUTABLEATTR, "clang::ObjCBridgeMutableAttr");
  g.add_edge((CLASS_OBJCBRIDGEMUTABLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCBRIDGEMUTABLEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_ARMINOUTATTR, "clang::ArmInOutAttr");
  g.add_edge((CLASS_ARMINOUTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARMINOUTATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_OMPPARALLELDIRECTIVE, "clang::OMPParallelDirective");
  g.add_edge((CLASS_OMPPARALLELDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPPARALLELDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_COMPOUNDASSIGNOPERATOR, "clang::CompoundAssignOperator");
  g.add_edge((CLASS_COMPOUNDASSIGNOPERATOR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_COMPOUNDASSIGNOPERATOR, META_SUBCLASS, CLASS_BINARYOPERATOR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETCOMPUTATIONLHSTYPE, "clang::CompoundAssignOperator::getComputationLHSType"),
      add_named_node(&mut g, METHOD_GETCOMPUTATIONRESULTTYPE, "clang::CompoundAssignOperator::getComputationResultType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_COMPOUNDASSIGNOPERATOR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OSRETURNSRETAINEDONZEROATTR, "clang::OSReturnsRetainedOnZeroAttr");
  g.add_edge((CLASS_OSRETURNSRETAINEDONZEROATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OSRETURNSRETAINEDONZEROATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OBJCINDIRECTCOPYRESTOREEXPR, "clang::ObjCIndirectCopyRestoreExpr");
  g.add_edge((CLASS_OBJCINDIRECTCOPYRESTOREEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCINDIRECTCOPYRESTOREEXPR, META_SUBCLASS, CLASS_EXPR));

  g.add_named_node(CLASS_OSRETURNSRETAINEDONNONZEROATTR, "clang::OSReturnsRetainedOnNonZeroAttr");
  g.add_edge((CLASS_OSRETURNSRETAINEDONNONZEROATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OSRETURNSRETAINEDONNONZEROATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_SPTRATTR, "clang::SPtrAttr");
  g.add_edge((CLASS_SPTRATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SPTRATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_PRESERVEALLATTR, "clang::PreserveAllAttr");
  g.add_edge((CLASS_PRESERVEALLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PRESERVEALLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OBJCSUBSCRIPTREFEXPR, "clang::ObjCSubscriptRefExpr");
  g.add_edge((CLASS_OBJCSUBSCRIPTREFEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCSUBSCRIPTREFEXPR, META_SUBCLASS, CLASS_EXPR));

  g.add_named_node(CLASS_OMPDECLAREVARIANTATTR, "clang::OMPDeclareVariantAttr");
  g.add_edge((CLASS_OMPDECLAREVARIANTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPDECLAREVARIANTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OMPALLOCATEDECLATTR, "clang::OMPAllocateDeclAttr");
  g.add_edge((CLASS_OMPALLOCATEDECLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPALLOCATEDECLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_MICROMIPSATTR, "clang::MicroMipsAttr");
  g.add_edge((CLASS_MICROMIPSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MICROMIPSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_GENERICSELECTIONEXPR, "clang::GenericSelectionExpr");
  g.add_edge((CLASS_GENERICSELECTIONEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_GENERICSELECTIONEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETNUMASSOCS, "clang::GenericSelectionExpr::getNumAssocs"),
      add_named_node(&mut g, METHOD_GETRESULTINDEX, "clang::GenericSelectionExpr::getResultIndex"),
      add_named_node(&mut g, METHOD_ISRESULTDEPENDENT, "clang::GenericSelectionExpr::isResultDependent"),
      add_named_node(&mut g, METHOD_ISEXPRPREDICATE, "clang::GenericSelectionExpr::isExprPredicate"),
      add_named_node(&mut g, METHOD_ISTYPEPREDICATE, "clang::GenericSelectionExpr::isTypePredicate"),
      add_named_node(&mut g, METHOD_GETCONTROLLINGEXPR, "clang::GenericSelectionExpr::getControllingExpr"),
      add_named_node(&mut g, METHOD_GETCONTROLLINGTYPE, "clang::GenericSelectionExpr::getControllingType"),
      add_named_node(&mut g, METHOD_GETRESULTEXPR, "clang::GenericSelectionExpr::getResultExpr"),
      add_named_node(&mut g, METHOD_GETASSOCEXPRS, "clang::GenericSelectionExpr::getAssocExprs"),
      add_named_node(&mut g, METHOD_GETASSOCTYPESOURCEINFOS, "clang::GenericSelectionExpr::getAssocTypeSourceInfos"),
      add_named_node(&mut g, METHOD_ASSOCIATIONS, "clang::GenericSelectionExpr::associations"),
      add_named_node(&mut g, METHOD_GETGENERICLOC, "clang::GenericSelectionExpr::getGenericLoc"),
      add_named_node(&mut g, METHOD_GETDEFAULTLOC_2, "clang::GenericSelectionExpr::getDefaultLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_17, "clang::GenericSelectionExpr::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_80, "clang::GenericSelectionExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_79, "clang::GenericSelectionExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_69, "clang::GenericSelectionExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_GENERICSELECTIONEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_HIPMANAGEDATTR, "clang::HIPManagedAttr");
  g.add_edge((CLASS_HIPMANAGEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_HIPMANAGEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_ZEROCALLUSEDREGSATTR, "clang::ZeroCallUsedRegsAttr");
  g.add_edge((CLASS_ZEROCALLUSEDREGSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ZEROCALLUSEDREGSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OBJCEXCEPTIONATTR, "clang::ObjCExceptionAttr");
  g.add_edge((CLASS_OBJCEXCEPTIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCEXCEPTIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CLASSTEMPLATEPARTIALSPECIALIZATIONDECL, "clang::ClassTemplatePartialSpecializationDecl");
  g.add_edge((CLASS_CLASSTEMPLATEPARTIALSPECIALIZATIONDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CLASSTEMPLATEPARTIALSPECIALIZATIONDECL, META_SUBCLASS, CLASS_CLASSTEMPLATESPECIALIZATIONDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTEMPLATEPARAMETERS, "clang::ClassTemplatePartialSpecializationDecl::getTemplateParameters"),
      add_named_node(&mut g, METHOD_HASASSOCIATEDCONSTRAINTS, "clang::ClassTemplatePartialSpecializationDecl::hasAssociatedConstraints"),
      add_named_node(&mut g, METHOD_GETTEMPLATEARGSASWRITTEN, "clang::ClassTemplatePartialSpecializationDecl::getTemplateArgsAsWritten"),
      add_named_node(&mut g, METHOD_GETINSTANTIATEDFROMMEMBER, "clang::ClassTemplatePartialSpecializationDecl::getInstantiatedFromMember"),
      add_named_node(&mut g, METHOD_GETINSTANTIATEDFROMMEMBERTEMPLATE_1, "clang::ClassTemplatePartialSpecializationDecl::getInstantiatedFromMemberTemplate"),
      add_named_node(&mut g, METHOD_GETINJECTEDSPECIALIZATIONTYPE_1, "clang::ClassTemplatePartialSpecializationDecl::getInjectedSpecializationType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CLASSTEMPLATEPARTIALSPECIALIZATIONDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NOTTAILCALLEDATTR, "clang::NotTailCalledAttr");
  g.add_edge((CLASS_NOTTAILCALLEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOTTAILCALLEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_ANNOTATEATTR, "clang::AnnotateAttr");
  g.add_edge((CLASS_ANNOTATEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ANNOTATEATTR, META_SUBCLASS, CLASS_INHERITABLEPARAMATTR));

  g.add_named_node(CLASS_MAYBEUNDEFATTR, "clang::MaybeUndefAttr");
  g.add_edge((CLASS_MAYBEUNDEFATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MAYBEUNDEFATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_ARRAYTYPELOC, "clang::ArrayTypeLoc");
  g.add_edge((CLASS_ARRAYTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETLBRACKETLOC_2, "clang::ArrayTypeLoc::getLBracketLoc"),
      add_named_node(&mut g, METHOD_GETRBRACKETLOC_5, "clang::ArrayTypeLoc::getRBracketLoc"),
      add_named_node(&mut g, METHOD_GETBRACKETSRANGE_2, "clang::ArrayTypeLoc::getBracketsRange"),
      add_named_node(&mut g, METHOD_GETSIZEEXPR_5, "clang::ArrayTypeLoc::getSizeExpr"),
      add_named_node(&mut g, METHOD_GETELEMENTLOC_2, "clang::ArrayTypeLoc::getElementLoc"),
      add_named_node(&mut g, METHOD_GETLOCALSOURCERANGE_7, "clang::ArrayTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, METHOD_GETINNERTYPE_6, "clang::ArrayTypeLoc::getInnerType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ARRAYTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPDISTRIBUTEDIRECTIVE, "clang::OMPDistributeDirective");
  g.add_edge((CLASS_OMPDISTRIBUTEDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPDISTRIBUTEDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_CPUSPECIFICATTR, "clang::CPUSpecificAttr");
  g.add_edge((CLASS_CPUSPECIFICATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CPUSPECIFICATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_FRIENDDECL, "clang::FriendDecl");
  g.add_edge((CLASS_FRIENDDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FRIENDDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETFRIENDTYPE, "clang::FriendDecl::getFriendType"),
      add_named_node(&mut g, METHOD_GETFRIENDTYPENUMTEMPLATEPARAMETERLISTS, "clang::FriendDecl::getFriendTypeNumTemplateParameterLists"),
      add_named_node(&mut g, METHOD_GETFRIENDDECL, "clang::FriendDecl::getFriendDecl"),
      add_named_node(&mut g, METHOD_GETFRIENDLOC, "clang::FriendDecl::getFriendLoc"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_12, "clang::FriendDecl::getSourceRange"),
      add_named_node(&mut g, METHOD_ISUNSUPPORTEDFRIEND, "clang::FriendDecl::isUnsupportedFriend"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FRIENDDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TARGETVERSIONATTR, "clang::TargetVersionAttr");
  g.add_edge((CLASS_TARGETVERSIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TARGETVERSIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_NOUWTABLEATTR, "clang::NoUwtableAttr");
  g.add_edge((CLASS_NOUWTABLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOUWTABLEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_NOUNIQUEADDRESSATTR, "clang::NoUniqueAddressAttr");
  g.add_edge((CLASS_NOUNIQUEADDRESSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOUNIQUEADDRESSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_NOTHREADSAFETYANALYSISATTR, "clang::NoThreadSafetyAnalysisAttr");
  g.add_edge((CLASS_NOTHREADSAFETYANALYSISATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOTHREADSAFETYANALYSISATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_MSNOVTABLEATTR, "clang::MSNoVTableAttr");
  g.add_edge((CLASS_MSNOVTABLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MSNOVTABLEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CXXNOEXCEPTEXPR, "clang::CXXNoexceptExpr");
  g.add_edge((CLASS_CXXNOEXCEPTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXNOEXCEPTEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETOPERAND, "clang::CXXNoexceptExpr::getOperand"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_33, "clang::CXXNoexceptExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_32, "clang::CXXNoexceptExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_40, "clang::CXXNoexceptExpr::getSourceRange"),
      add_named_node(&mut g, METHOD_GETVALUE_5, "clang::CXXNoexceptExpr::getValue"),
      add_named_node(&mut g, METHOD_CHILDREN_25, "clang::CXXNoexceptExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXNOEXCEPTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPPARALLELMASKEDDIRECTIVE, "clang::OMPParallelMaskedDirective");
  g.add_edge((CLASS_OMPPARALLELMASKEDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPPARALLELMASKEDDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_NSCONSUMESSELFATTR, "clang::NSConsumesSelfAttr");
  g.add_edge((CLASS_NSCONSUMESSELFATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NSCONSUMESSELFATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_INTELOCLBICCATTR, "clang::IntelOclBiccAttr");
  g.add_edge((CLASS_INTELOCLBICCATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_INTELOCLBICCATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_NOSPECULATIVELOADHARDENINGATTR, "clang::NoSpeculativeLoadHardeningAttr");
  g.add_edge((CLASS_NOSPECULATIVELOADHARDENINGATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOSPECULATIVELOADHARDENINGATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_NOSPLITSTACKATTR, "clang::NoSplitStackAttr");
  g.add_edge((CLASS_NOSPLITSTACKATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOSPLITSTACKATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_BITINTTYPE, "clang::BitIntType");
  g.add_edge((CLASS_BITINTTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BITINTTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_ISUNSIGNED, "clang::BitIntType::isUnsigned"),
      add_named_node(&mut g, METHOD_ISSIGNED, "clang::BitIntType::isSigned"),
      add_named_node(&mut g, METHOD_GETNUMBITS, "clang::BitIntType::getNumBits"),
      add_named_node(&mut g, METHOD_ISSUGARED_4, "clang::BitIntType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_4, "clang::BitIntType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BITINTTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SOURCELOCEXPR, "clang::SourceLocExpr");
  g.add_edge((CLASS_SOURCELOCEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SOURCELOCEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETBUILTINSTR, "clang::SourceLocExpr::getBuiltinStr"),
      add_named_node(&mut g, METHOD_GETIDENTKIND_1, "clang::SourceLocExpr::getIdentKind"),
      add_named_node(&mut g, METHOD_ISINTTYPE, "clang::SourceLocExpr::isIntType"),
      add_named_node(&mut g, METHOD_GETPARENTCONTEXT, "clang::SourceLocExpr::getParentContext"),
      add_named_node(&mut g, METHOD_GETLOCATION_15, "clang::SourceLocExpr::getLocation"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_117, "clang::SourceLocExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_116, "clang::SourceLocExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_104, "clang::SourceLocExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SOURCELOCEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SWIFTNEWTYPEATTR, "clang::SwiftNewTypeAttr");
  g.add_edge((CLASS_SWIFTNEWTYPEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTNEWTYPEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_AVAILABILITYATTR, "clang::AvailabilityAttr");
  g.add_edge((CLASS_AVAILABILITYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_AVAILABILITYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OBJCDIRECTATTR, "clang::ObjCDirectAttr");
  g.add_edge((CLASS_OBJCDIRECTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCDIRECTATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_IBOUTLETATTR, "clang::IBOutletAttr");
  g.add_edge((CLASS_IBOUTLETATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_IBOUTLETATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CORETURNSTMT, "clang::CoreturnStmt");
  g.add_edge((CLASS_CORETURNSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CORETURNSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETKEYWORDLOC, "clang::CoreturnStmt::getKeywordLoc"),
      add_named_node(&mut g, METHOD_GETOPERAND_1, "clang::CoreturnStmt::getOperand"),
      add_named_node(&mut g, METHOD_GETPROMISECALL, "clang::CoreturnStmt::getPromiseCall"),
      add_named_node(&mut g, METHOD_ISIMPLICIT_3, "clang::CoreturnStmt::isImplicit"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_60, "clang::CoreturnStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_59, "clang::CoreturnStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_50, "clang::CoreturnStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CORETURNSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ANYX86NOCALLERSAVEDREGISTERSATTR, "clang::AnyX86NoCallerSavedRegistersAttr");
  g.add_edge((CLASS_ANYX86NOCALLERSAVEDREGISTERSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ANYX86NOCALLERSAVEDREGISTERSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CODEMODELATTR, "clang::CodeModelAttr");
  g.add_edge((CLASS_CODEMODELATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CODEMODELATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_DLLIMPORTATTR, "clang::DLLImportAttr");
  g.add_edge((CLASS_DLLIMPORTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DLLIMPORTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_PRAGMADETECTMISMATCHDECL, "clang::PragmaDetectMismatchDecl");
  g.add_edge((CLASS_PRAGMADETECTMISMATCHDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PRAGMADETECTMISMATCHDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETNAME, "clang::PragmaDetectMismatchDecl::getName"),
      add_named_node(&mut g, METHOD_GETVALUE_1, "clang::PragmaDetectMismatchDecl::getValue"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PRAGMADETECTMISMATCHDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MODEATTR, "clang::ModeAttr");
  g.add_edge((CLASS_MODEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MODEATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_FLATTENATTR, "clang::FlattenAttr");
  g.add_edge((CLASS_FLATTENATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FLATTENATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_NOSANITIZEATTR, "clang::NoSanitizeAttr");
  g.add_edge((CLASS_NOSANITIZEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOSANITIZEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CXXNAMEDCASTEXPR, "clang::CXXNamedCastExpr");
  g.add_edge((CLASS_CXXNAMEDCASTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXNAMEDCASTEXPR, META_SUBCLASS, CLASS_EXPLICITCASTEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETCASTNAME, "clang::CXXNamedCastExpr::getCastName"),
      add_named_node(&mut g, METHOD_GETOPERATORLOC_2, "clang::CXXNamedCastExpr::getOperatorLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_8, "clang::CXXNamedCastExpr::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_31, "clang::CXXNamedCastExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_30, "clang::CXXNamedCastExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETANGLEBRACKETS, "clang::CXXNamedCastExpr::getAngleBrackets"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXNAMEDCASTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NORANDOMIZELAYOUTATTR, "clang::NoRandomizeLayoutAttr");
  g.add_edge((CLASS_NORANDOMIZELAYOUTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NORANDOMIZELAYOUTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_USINGDECL, "clang::UsingDecl");
  g.add_edge((CLASS_USINGDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_USINGDECL, META_SUBCLASS, CLASS_BASEUSINGDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETUSINGLOC_2, "clang::UsingDecl::getUsingLoc"),
      add_named_node(&mut g, METHOD_GETQUALIFIERLOC_5, "clang::UsingDecl::getQualifierLoc"),
      add_named_node(&mut g, METHOD_GETQUALIFIER_8, "clang::UsingDecl::getQualifier"),
      add_named_node(&mut g, METHOD_GETNAMEINFO_3, "clang::UsingDecl::getNameInfo"),
      add_named_node(&mut g, METHOD_ISACCESSDECLARATION_1, "clang::UsingDecl::isAccessDeclaration"),
      add_named_node(&mut g, METHOD_HASTYPENAME, "clang::UsingDecl::hasTypename"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_32, "clang::UsingDecl::getSourceRange"),
      add_named_node(&mut g, METHOD_GETCANONICALDECL_23, "clang::UsingDecl::getCanonicalDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_USINGDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SYSVABIATTR, "clang::SysVABIAttr");
  g.add_edge((CLASS_SYSVABIATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SYSVABIATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_NOMIPS16ATTR, "clang::NoMips16Attr");
  g.add_edge((CLASS_NOMIPS16ATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOMIPS16ATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_NODESTROYATTR, "clang::NoDestroyAttr");
  g.add_edge((CLASS_NODESTROYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NODESTROYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_SWIFTOBJCMEMBERSATTR, "clang::SwiftObjCMembersAttr");
  g.add_edge((CLASS_SWIFTOBJCMEMBERSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTOBJCMEMBERSATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_NOALIASATTR, "clang::NoAliasAttr");
  g.add_edge((CLASS_NOALIASATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOALIASATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_BUILTINTYPE, "clang::BuiltinType");
  g.add_edge((CLASS_BUILTINTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BUILTINTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETKIND, "clang::BuiltinType::getKind"),
      add_named_node(&mut g, METHOD_ISSUGARED_6, "clang::BuiltinType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_6, "clang::BuiltinType::desugar"),
      add_named_node(&mut g, METHOD_ISINTEGER, "clang::BuiltinType::isInteger"),
      add_named_node(&mut g, METHOD_ISSIGNEDINTEGER, "clang::BuiltinType::isSignedInteger"),
      add_named_node(&mut g, METHOD_ISUNSIGNEDINTEGER, "clang::BuiltinType::isUnsignedInteger"),
      add_named_node(&mut g, METHOD_ISFLOATINGPOINT, "clang::BuiltinType::isFloatingPoint"),
      add_named_node(&mut g, METHOD_ISSVEBOOL, "clang::BuiltinType::isSVEBool"),
      add_named_node(&mut g, METHOD_ISSVECOUNT, "clang::BuiltinType::isSVECount"),
      add_named_node(&mut g, METHOD_ISPLACEHOLDERTYPE, "clang::BuiltinType::isPlaceholderType"),
      add_named_node(&mut g, METHOD_ISNONOVERLOADPLACEHOLDERTYPE, "clang::BuiltinType::isNonOverloadPlaceholderType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BUILTINTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PCSATTR, "clang::PcsAttr");
  g.add_edge((CLASS_PCSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PCSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_AMDGPUKERNELCALLATTR, "clang::AMDGPUKernelCallAttr");
  g.add_edge((CLASS_AMDGPUKERNELCALLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_AMDGPUKERNELCALLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_NSRETURNSRETAINEDATTR, "clang::NSReturnsRetainedAttr");
  g.add_edge((CLASS_NSRETURNSRETAINEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NSRETURNSRETAINEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_ANYX86NOCFCHECKATTR, "clang::AnyX86NoCfCheckAttr");
  g.add_edge((CLASS_ANYX86NOCFCHECKATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ANYX86NOCFCHECKATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_LOCKRETURNEDATTR, "clang::LockReturnedAttr");
  g.add_edge((CLASS_LOCKRETURNEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LOCKRETURNEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_AVRINTERRUPTATTR, "clang::AVRInterruptAttr");
  g.add_edge((CLASS_AVRINTERRUPTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_AVRINTERRUPTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_HLSLSHADERATTR, "clang::HLSLShaderAttr");
  g.add_edge((CLASS_HLSLSHADERATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_HLSLSHADERATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_STDCALLATTR, "clang::StdCallAttr");
  g.add_edge((CLASS_STDCALLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_STDCALLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OBJCRUNTIMENAMEATTR, "clang::ObjCRuntimeNameAttr");
  g.add_edge((CLASS_OBJCRUNTIMENAMEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCRUNTIMENAMEATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_ARRAYTYPE, "clang::ArrayType");
  g.add_edge((CLASS_ARRAYTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARRAYTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETELEMENTTYPE, "clang::ArrayType::getElementType"),
      add_named_node(&mut g, METHOD_GETSIZEMODIFIER, "clang::ArrayType::getSizeModifier"),
      add_named_node(&mut g, METHOD_GETINDEXTYPEQUALIFIERS, "clang::ArrayType::getIndexTypeQualifiers"),
      add_named_node(&mut g, METHOD_GETINDEXTYPECVRQUALIFIERS, "clang::ArrayType::getIndexTypeCVRQualifiers"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ARRAYTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DLLEXPORTATTR, "clang::DLLExportAttr");
  g.add_edge((CLASS_DLLEXPORTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DLLEXPORTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_WEAKATTR, "clang::WeakAttr");
  g.add_edge((CLASS_WEAKATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_WEAKATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CONSTINITATTR, "clang::ConstInitAttr");
  g.add_edge((CLASS_CONSTINITATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONSTINITATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_PARENTYPE, "clang::ParenType");
  g.add_edge((CLASS_PARENTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PARENTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETINNERTYPE, "clang::ParenType::getInnerType"),
      add_named_node(&mut g, METHOD_ISSUGARED_30, "clang::ParenType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_30, "clang::ParenType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PARENTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_COMPLEXTYPELOC, "clang::ComplexTypeLoc");
  g.add_edge((CLASS_COMPLEXTYPELOC, META_CLASS, META_CLANG_AST_NODE));

  g.add_named_node(CLASS_MINSIZEATTR, "clang::MinSizeAttr");
  g.add_edge((CLASS_MINSIZEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MINSIZEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_MAYALIASATTR, "clang::MayAliasAttr");
  g.add_edge((CLASS_MAYALIASATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MAYALIASATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_MIPSSHORTCALLATTR, "clang::MipsShortCallAttr");
  g.add_edge((CLASS_MIPSSHORTCALLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MIPSSHORTCALLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_NOPROFILEFUNCTIONATTR, "clang::NoProfileFunctionAttr");
  g.add_edge((CLASS_NOPROFILEFUNCTIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOPROFILEFUNCTIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CMSENSENTRYATTR, "clang::CmseNSEntryAttr");
  g.add_edge((CLASS_CMSENSENTRYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CMSENSENTRYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_DESTRUCTORATTR, "clang::DestructorAttr");
  g.add_edge((CLASS_DESTRUCTORATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DESTRUCTORATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_MSVTORDISPATTR, "clang::MSVtorDispAttr");
  g.add_edge((CLASS_MSVTORDISPATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MSVTORDISPATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_MSSTRUCTATTR, "clang::MSStructAttr");
  g.add_edge((CLASS_MSSTRUCTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MSSTRUCTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OMPSINGLEDIRECTIVE, "clang::OMPSingleDirective");
  g.add_edge((CLASS_OMPSINGLEDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPSINGLEDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_SIZEOFPACKEXPR, "clang::SizeOfPackExpr");
  g.add_edge((CLASS_SIZEOFPACKEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SIZEOFPACKEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETOPERATORLOC_8, "clang::SizeOfPackExpr::getOperatorLoc"),
      add_named_node(&mut g, METHOD_GETPACKLOC, "clang::SizeOfPackExpr::getPackLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_23, "clang::SizeOfPackExpr::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETPACK, "clang::SizeOfPackExpr::getPack"),
      add_named_node(&mut g, METHOD_GETPACKLENGTH, "clang::SizeOfPackExpr::getPackLength"),
      add_named_node(&mut g, METHOD_ISPARTIALLYSUBSTITUTED, "clang::SizeOfPackExpr::isPartiallySubstituted"),
      add_named_node(&mut g, METHOD_GETPARTIALARGUMENTS, "clang::SizeOfPackExpr::getPartialArguments"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_116, "clang::SizeOfPackExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_115, "clang::SizeOfPackExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_103, "clang::SizeOfPackExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SIZEOFPACKEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DECL, "clang::Decl");
  g.add_edge((CLASS_DECL, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSOURCERANGE_5, "clang::Decl::getSourceRange"),
      add_named_node(&mut g, METHOD_GETBEGINLOC, "clang::Decl::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC, "clang::Decl::getEndLoc"),
      add_named_node(&mut g, METHOD_GETLOCATION, "clang::Decl::getLocation"),
      add_named_node(&mut g, METHOD_GETKIND_6, "clang::Decl::getKind"),
      add_named_node(&mut g, METHOD_GETDECLKINDNAME, "clang::Decl::getDeclKindName"),
      add_named_node(&mut g, METHOD_GETNEXTDECLINCONTEXT, "clang::Decl::getNextDeclInContext"),
      add_named_node(&mut g, METHOD_GETDECLCONTEXT, "clang::Decl::getDeclContext"),
      add_named_node(&mut g, METHOD_GETNONTRANSPARENTDECLCONTEXT, "clang::Decl::getNonTransparentDeclContext"),
      add_named_node(&mut g, METHOD_GETNONCLOSURECONTEXT, "clang::Decl::getNonClosureContext"),
      add_named_node(&mut g, METHOD_GETTRANSLATIONUNITDECL, "clang::Decl::getTranslationUnitDecl"),
      add_named_node(&mut g, METHOD_ISINANONYMOUSNAMESPACE, "clang::Decl::isInAnonymousNamespace"),
      add_named_node(&mut g, METHOD_ISINSTDNAMESPACE, "clang::Decl::isInStdNamespace"),
      add_named_node(&mut g, METHOD_ISFILECONTEXTDECL, "clang::Decl::isFileContextDecl"),
      add_named_node(&mut g, METHOD_GETASTCONTEXT_1, "clang::Decl::getASTContext"),
      add_named_node(&mut g, METHOD_GETLANGOPTS, "clang::Decl::getLangOpts"),
      add_named_node(&mut g, METHOD_GETACCESS, "clang::Decl::getAccess"),
      add_named_node(&mut g, METHOD_GETACCESSUNSAFE, "clang::Decl::getAccessUnsafe"),
      add_named_node(&mut g, METHOD_HASATTRS, "clang::Decl::hasAttrs"),
      add_named_node(&mut g, METHOD_GETATTRS, "clang::Decl::getAttrs"),
      add_named_node(&mut g, METHOD_ATTRS, "clang::Decl::attrs"),
      add_named_node(&mut g, METHOD_ATTR_BEGIN, "clang::Decl::attr_begin"),
      add_named_node(&mut g, METHOD_ATTR_END, "clang::Decl::attr_end"),
      add_named_node(&mut g, METHOD_GETMAXALIGNMENT, "clang::Decl::getMaxAlignment"),
      add_named_node(&mut g, METHOD_ISINVALIDDECL, "clang::Decl::isInvalidDecl"),
      add_named_node(&mut g, METHOD_ISIMPLICIT, "clang::Decl::isImplicit"),
      add_named_node(&mut g, METHOD_ISREFERENCED, "clang::Decl::isReferenced"),
      add_named_node(&mut g, METHOD_ISTHISDECLARATIONREFERENCED, "clang::Decl::isThisDeclarationReferenced"),
      add_named_node(&mut g, METHOD_ISTOPLEVELDECLINOBJCCONTAINER, "clang::Decl::isTopLevelDeclInObjCContainer"),
      add_named_node(&mut g, METHOD_GETEXTERNALSOURCESYMBOLATTR, "clang::Decl::getExternalSourceSymbolAttr"),
      add_named_node(&mut g, METHOD_ISMODULEPRIVATE, "clang::Decl::isModulePrivate"),
      add_named_node(&mut g, METHOD_ISINEXPORTDECLCONTEXT, "clang::Decl::isInExportDeclContext"),
      add_named_node(&mut g, METHOD_ISINVISIBLEOUTSIDETHEOWNINGMODULE, "clang::Decl::isInvisibleOutsideTheOwningModule"),
      add_named_node(&mut g, METHOD_ISINANOTHERMODULEUNIT, "clang::Decl::isInAnotherModuleUnit"),
      add_named_node(&mut g, METHOD_ISDISCARDEDINGLOBALMODULEFRAGMENT, "clang::Decl::isDiscardedInGlobalModuleFragment"),
      add_named_node(&mut g, METHOD_SHOULDSKIPCHECKINGODR, "clang::Decl::shouldSkipCheckingODR"),
      add_named_node(&mut g, METHOD_HASDEFININGATTR, "clang::Decl::hasDefiningAttr"),
      add_named_node(&mut g, METHOD_GETDEFININGATTR, "clang::Decl::getDefiningAttr"),
      add_named_node(&mut g, METHOD_GETVERSIONINTRODUCED, "clang::Decl::getVersionIntroduced"),
      add_named_node(&mut g, METHOD_ISWEAKIMPORTED, "clang::Decl::isWeakImported"),
      add_named_node(&mut g, METHOD_ISFROMASTFILE, "clang::Decl::isFromASTFile"),
      add_named_node(&mut g, METHOD_GETGLOBALID, "clang::Decl::getGlobalID"),
      add_named_node(&mut g, METHOD_GETOWNINGMODULEID, "clang::Decl::getOwningModuleID"),
      add_named_node(&mut g, METHOD_GETIMPORTEDOWNINGMODULE, "clang::Decl::getImportedOwningModule"),
      add_named_node(&mut g, METHOD_GETLOCALOWNINGMODULE, "clang::Decl::getLocalOwningModule"),
      add_named_node(&mut g, METHOD_HASOWNINGMODULE, "clang::Decl::hasOwningModule"),
      add_named_node(&mut g, METHOD_GETOWNINGMODULE, "clang::Decl::getOwningModule"),
      add_named_node(&mut g, METHOD_ISUNCONDITIONALLYVISIBLE, "clang::Decl::isUnconditionallyVisible"),
      add_named_node(&mut g, METHOD_ISREACHABLE, "clang::Decl::isReachable"),
      add_named_node(&mut g, METHOD_GETMODULEOWNERSHIPKIND, "clang::Decl::getModuleOwnershipKind"),
      add_named_node(&mut g, METHOD_GETIDENTIFIERNAMESPACE, "clang::Decl::getIdentifierNamespace"),
      add_named_node(&mut g, METHOD_HASTAGIDENTIFIERNAMESPACE, "clang::Decl::hasTagIdentifierNamespace"),
      add_named_node(&mut g, METHOD_GETLEXICALDECLCONTEXT, "clang::Decl::getLexicalDeclContext"),
      add_named_node(&mut g, METHOD_ISOUTOFLINE, "clang::Decl::isOutOfLine"),
      add_named_node(&mut g, METHOD_ISTEMPLATED, "clang::Decl::isTemplated"),
      add_named_node(&mut g, METHOD_GETTEMPLATEDEPTH, "clang::Decl::getTemplateDepth"),
      add_named_node(&mut g, METHOD_ISDEFINEDOUTSIDEFUNCTIONORMETHOD, "clang::Decl::isDefinedOutsideFunctionOrMethod"),
      add_named_node(&mut g, METHOD_ISINLOCALSCOPEFORINSTANTIATION, "clang::Decl::isInLocalScopeForInstantiation"),
      add_named_node(&mut g, METHOD_GETCANONICALDECL_7, "clang::Decl::getCanonicalDecl"),
      add_named_node(&mut g, METHOD_ISCANONICALDECL, "clang::Decl::isCanonicalDecl"),
      add_named_node(&mut g, METHOD_REDECLS, "clang::Decl::redecls"),
      add_named_node(&mut g, METHOD_REDECLS_BEGIN, "clang::Decl::redecls_begin"),
      add_named_node(&mut g, METHOD_REDECLS_END, "clang::Decl::redecls_end"),
      add_named_node(&mut g, METHOD_GETPREVIOUSDECL_2, "clang::Decl::getPreviousDecl"),
      add_named_node(&mut g, METHOD_ISFIRSTDECL, "clang::Decl::isFirstDecl"),
      add_named_node(&mut g, METHOD_GETMOSTRECENTDECL_3, "clang::Decl::getMostRecentDecl"),
      add_named_node(&mut g, METHOD_GETBODY_2, "clang::Decl::getBody"),
      add_named_node(&mut g, METHOD_HASBODY, "clang::Decl::hasBody"),
      add_named_node(&mut g, METHOD_GETBODYRBRACE, "clang::Decl::getBodyRBrace"),
      add_named_node(&mut g, METHOD_ISTEMPLATEPARAMETER, "clang::Decl::isTemplateParameter"),
      add_named_node(&mut g, METHOD_ISTEMPLATEPARAMETERPACK, "clang::Decl::isTemplateParameterPack"),
      add_named_node(&mut g, METHOD_ISPARAMETERPACK_1, "clang::Decl::isParameterPack"),
      add_named_node(&mut g, METHOD_ISTEMPLATEDECL, "clang::Decl::isTemplateDecl"),
      add_named_node(&mut g, METHOD_ISFUNCTIONORFUNCTIONTEMPLATE, "clang::Decl::isFunctionOrFunctionTemplate"),
      add_named_node(&mut g, METHOD_GETDESCRIBEDTEMPLATE, "clang::Decl::getDescribedTemplate"),
      add_named_node(&mut g, METHOD_GETDESCRIBEDTEMPLATEPARAMS, "clang::Decl::getDescribedTemplateParams"),
      add_named_node(&mut g, METHOD_GETASFUNCTION, "clang::Decl::getAsFunction"),
      add_named_node(&mut g, METHOD_ISLOCALEXTERNDECL, "clang::Decl::isLocalExternDecl"),
      add_named_node(&mut g, METHOD_GETFRIENDOBJECTKIND, "clang::Decl::getFriendObjectKind"),
      add_named_node(&mut g, METHOD_GETID, "clang::Decl::getID"),
      add_named_node(&mut g, METHOD_ISFUNCTIONPOINTERTYPE, "clang::Decl::isFunctionPointerType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCTYPEPARAMTYPELOC, "clang::ObjCTypeParamTypeLoc");
  g.add_edge((CLASS_OBJCTYPEPARAMTYPELOC, META_CLASS, META_CLANG_AST_NODE));

  g.add_named_node(CLASS_ANNOTATETYPEATTR, "clang::AnnotateTypeAttr");
  g.add_edge((CLASS_ANNOTATETYPEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ANNOTATETYPEATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_DESIGNATEDINITEXPR, "clang::DesignatedInitExpr");
  g.add_edge((CLASS_DESIGNATEDINITEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DESIGNATEDINITEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_SIZE_1, "clang::DesignatedInitExpr::size"),
      add_named_node(&mut g, METHOD_DESIGNATORS, "clang::DesignatedInitExpr::designators"),
      add_named_node(&mut g, METHOD_GETEQUALORCOLONLOC, "clang::DesignatedInitExpr::getEqualOrColonLoc"),
      add_named_node(&mut g, METHOD_ISDIRECTINIT_1, "clang::DesignatedInitExpr::isDirectInit"),
      add_named_node(&mut g, METHOD_USESGNUSYNTAX, "clang::DesignatedInitExpr::usesGNUSyntax"),
      add_named_node(&mut g, METHOD_GETINIT_3, "clang::DesignatedInitExpr::getInit"),
      add_named_node(&mut g, METHOD_GETNUMSUBEXPRS_1, "clang::DesignatedInitExpr::getNumSubExprs"),
      add_named_node(&mut g, METHOD_GETDESIGNATORSSOURCERANGE, "clang::DesignatedInitExpr::getDesignatorsSourceRange"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_68, "clang::DesignatedInitExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_67, "clang::DesignatedInitExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_58, "clang::DesignatedInitExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DESIGNATEDINITEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_HLSLNUMTHREADSATTR, "clang::HLSLNumThreadsAttr");
  g.add_edge((CLASS_HLSLNUMTHREADSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_HLSLNUMTHREADSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_TRIVIALABIATTR, "clang::TrivialABIAttr");
  g.add_edge((CLASS_TRIVIALABIATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TRIVIALABIATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_MAXFIELDALIGNMENTATTR, "clang::MaxFieldAlignmentAttr");
  g.add_edge((CLASS_MAXFIELDALIGNMENTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MAXFIELDALIGNMENTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_MSP430INTERRUPTATTR, "clang::MSP430InterruptAttr");
  g.add_edge((CLASS_MSP430INTERRUPTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MSP430INTERRUPTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OBJCATCATCHSTMT, "clang::ObjCAtCatchStmt");
  g.add_edge((CLASS_OBJCATCATCHSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCATCATCHSTMT, META_SUBCLASS, CLASS_STMT));

  g.add_named_node(CLASS_INHERITABLEPARAMATTR, "clang::InheritableParamAttr");
  g.add_edge((CLASS_INHERITABLEPARAMATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_INHERITABLEPARAMATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_RETURNTYPESTATEATTR, "clang::ReturnTypestateAttr");
  g.add_edge((CLASS_RETURNTYPESTATEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RETURNTYPESTATEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OMPTEAMSGENERICLOOPDIRECTIVE, "clang::OMPTeamsGenericLoopDirective");
  g.add_edge((CLASS_OMPTEAMSGENERICLOOPDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTEAMSGENERICLOOPDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_CONSTATTR, "clang::ConstAttr");
  g.add_edge((CLASS_CONSTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONSTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_MSINHERITANCEATTR, "clang::MSInheritanceAttr");
  g.add_edge((CLASS_MSINHERITANCEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MSINHERITANCEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OBJCTYPEPARAMDECL, "clang::ObjCTypeParamDecl");
  g.add_edge((CLASS_OBJCTYPEPARAMDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCTYPEPARAMDECL, META_SUBCLASS, CLASS_TYPEDEFNAMEDECL));

  g.add_named_node(CLASS_MSCONSTEXPRATTR, "clang::MSConstexprAttr");
  g.add_edge((CLASS_MSCONSTEXPRATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MSCONSTEXPRATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_SWIFTBRIDGEDTYPEDEFATTR, "clang::SwiftBridgedTypedefAttr");
  g.add_edge((CLASS_SWIFTBRIDGEDTYPEDEFATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTBRIDGEDTYPEDEFATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_HOTATTR, "clang::HotAttr");
  g.add_edge((CLASS_HOTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_HOTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OMPTHREADPRIVATEDECLATTR, "clang::OMPThreadPrivateDeclAttr");
  g.add_edge((CLASS_OMPTHREADPRIVATEDECLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTHREADPRIVATEDECLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CXXINHERITEDCTORINITEXPR, "clang::CXXInheritedCtorInitExpr");
  g.add_edge((CLASS_CXXINHERITEDCTORINITEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXINHERITEDCTORINITEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETCONSTRUCTOR_1, "clang::CXXInheritedCtorInitExpr::getConstructor"),
      add_named_node(&mut g, METHOD_CONSTRUCTSVBASE, "clang::CXXInheritedCtorInitExpr::constructsVBase"),
      add_named_node(&mut g, METHOD_GETCONSTRUCTIONKIND_1, "clang::CXXInheritedCtorInitExpr::getConstructionKind"),
      add_named_node(&mut g, METHOD_INHERITEDFROMVBASE, "clang::CXXInheritedCtorInitExpr::inheritedFromVBase"),
      add_named_node(&mut g, METHOD_GETLOCATION_3, "clang::CXXInheritedCtorInitExpr::getLocation"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_30, "clang::CXXInheritedCtorInitExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_29, "clang::CXXInheritedCtorInitExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_23, "clang::CXXInheritedCtorInitExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXINHERITEDCTORINITEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MSALLOCATORATTR, "clang::MSAllocatorAttr");
  g.add_edge((CLASS_MSALLOCATORATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MSALLOCATORATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_ENUMEXTENSIBILITYATTR, "clang::EnumExtensibilityAttr");
  g.add_edge((CLASS_ENUMEXTENSIBILITYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ENUMEXTENSIBILITYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_SUBSTNONTYPETEMPLATEPARMEXPR, "clang::SubstNonTypeTemplateParmExpr");
  g.add_edge((CLASS_SUBSTNONTYPETEMPLATEPARMEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SUBSTNONTYPETEMPLATEPARMEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETNAMELOC_1, "clang::SubstNonTypeTemplateParmExpr::getNameLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_120, "clang::SubstNonTypeTemplateParmExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_119, "clang::SubstNonTypeTemplateParmExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETREPLACEMENT, "clang::SubstNonTypeTemplateParmExpr::getReplacement"),
      add_named_node(&mut g, METHOD_GETASSOCIATEDDECL_2, "clang::SubstNonTypeTemplateParmExpr::getAssociatedDecl"),
      add_named_node(&mut g, METHOD_GETINDEX_4, "clang::SubstNonTypeTemplateParmExpr::getIndex"),
      add_named_node(&mut g, METHOD_GETPACKINDEX_1, "clang::SubstNonTypeTemplateParmExpr::getPackIndex"),
      add_named_node(&mut g, METHOD_GETPARAMETER, "clang::SubstNonTypeTemplateParmExpr::getParameter"),
      add_named_node(&mut g, METHOD_ISREFERENCEPARAMETER, "clang::SubstNonTypeTemplateParmExpr::isReferenceParameter"),
      add_named_node(&mut g, METHOD_CHILDREN_108, "clang::SubstNonTypeTemplateParmExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SUBSTNONTYPETEMPLATEPARMEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CUDACONSTANTATTR, "clang::CUDAConstantAttr");
  g.add_edge((CLASS_CUDACONSTANTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CUDACONSTANTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_NOSTACKPROTECTORATTR, "clang::NoStackProtectorAttr");
  g.add_edge((CLASS_NOSTACKPROTECTORATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOSTACKPROTECTORATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_DIAGNOSEIFATTR, "clang::DiagnoseIfAttr");
  g.add_edge((CLASS_DIAGNOSEIFATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DIAGNOSEIFATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OBJCIVARDECL, "clang::ObjCIvarDecl");
  g.add_edge((CLASS_OBJCIVARDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCIVARDECL, META_SUBCLASS, CLASS_FIELDDECL));

  g.add_named_node(CLASS_OMPCAPTUREKINDATTR, "clang::OMPCaptureKindAttr");
  g.add_edge((CLASS_OMPCAPTUREKINDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPCAPTUREKINDATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_REFERENCETYPE, "clang::ReferenceType");
  g.add_edge((CLASS_REFERENCETYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_REFERENCETYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_ISSPELLEDASLVALUE, "clang::ReferenceType::isSpelledAsLValue"),
      add_named_node(&mut g, METHOD_ISINNERREF, "clang::ReferenceType::isInnerRef"),
      add_named_node(&mut g, METHOD_GETPOINTEETYPEASWRITTEN, "clang::ReferenceType::getPointeeTypeAsWritten"),
      add_named_node(&mut g, METHOD_GETPOINTEETYPE_5, "clang::ReferenceType::getPointeeType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_REFERENCETYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPETAGFORDATATYPEATTR, "clang::TypeTagForDatatypeAttr");
  g.add_edge((CLASS_TYPETAGFORDATATYPEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPETAGFORDATATYPEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_MSABIATTR, "clang::MSABIAttr");
  g.add_edge((CLASS_MSABIATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MSABIATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_READONLYPLACEMENTATTR, "clang::ReadOnlyPlacementAttr");
  g.add_edge((CLASS_READONLYPLACEMENTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_READONLYPLACEMENTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_COUNTEDBYATTR, "clang::CountedByAttr");
  g.add_edge((CLASS_COUNTEDBYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_COUNTEDBYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_REDECLARABLETEMPLATEDECL, "clang::RedeclarableTemplateDecl");
  g.add_edge((CLASS_REDECLARABLETEMPLATEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_REDECLARABLETEMPLATEDECL, META_SUBCLASS, CLASS_TEMPLATEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETCANONICALDECL_16, "clang::RedeclarableTemplateDecl::getCanonicalDecl"),
      add_named_node(&mut g, METHOD_ISMEMBERSPECIALIZATION, "clang::RedeclarableTemplateDecl::isMemberSpecialization"),
      add_named_node(&mut g, METHOD_GETINSTANTIATEDFROMMEMBERTEMPLATE_3, "clang::RedeclarableTemplateDecl::getInstantiatedFromMemberTemplate"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_REDECLARABLETEMPLATEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PARENTYPELOC, "clang::ParenTypeLoc");
  g.add_edge((CLASS_PARENTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETLPARENLOC_12, "clang::ParenTypeLoc::getLParenLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_30, "clang::ParenTypeLoc::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETLOCALSOURCERANGE_8, "clang::ParenTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, METHOD_GETINNERLOC, "clang::ParenTypeLoc::getInnerLoc"),
      add_named_node(&mut g, METHOD_GETINNERTYPE_7, "clang::ParenTypeLoc::getInnerType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PARENTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MIGSERVERROUTINEATTR, "clang::MIGServerRoutineAttr");
  g.add_edge((CLASS_MIGSERVERROUTINEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MIGSERVERROUTINEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_DEPENDENTDECLTYPETYPE, "clang::DependentDecltypeType");
  g.add_edge((CLASS_DEPENDENTDECLTYPETYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEPENDENTDECLTYPETYPE, META_SUBCLASS, CLASS_DECLTYPETYPE));

  g.add_named_node(CLASS_RANDOMIZELAYOUTATTR, "clang::RandomizeLayoutAttr");
  g.add_edge((CLASS_RANDOMIZELAYOUTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RANDOMIZELAYOUTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_COROWRAPPERATTR, "clang::CoroWrapperAttr");
  g.add_edge((CLASS_COROWRAPPERATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_COROWRAPPERATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CORORETURNTYPEATTR, "clang::CoroReturnTypeAttr");
  g.add_edge((CLASS_CORORETURNTYPEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CORORETURNTYPEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_M68KRTDATTR, "clang::M68kRTDAttr");
  g.add_edge((CLASS_M68KRTDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_M68KRTDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OMPDECLARETARGETDECLATTR, "clang::OMPDeclareTargetDeclAttr");
  g.add_edge((CLASS_OMPDECLARETARGETDECLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPDECLARETARGETDECLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_MSPROPERTYREFEXPR, "clang::MSPropertyRefExpr");
  g.add_edge((CLASS_MSPROPERTYREFEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MSPROPERTYREFEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSOURCERANGE_48, "clang::MSPropertyRefExpr::getSourceRange"),
      add_named_node(&mut g, METHOD_ISIMPLICITACCESS_1, "clang::MSPropertyRefExpr::isImplicitAccess"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_93, "clang::MSPropertyRefExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_92, "clang::MSPropertyRefExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_81, "clang::MSPropertyRefExpr::children"),
      add_named_node(&mut g, METHOD_GETBASEEXPR, "clang::MSPropertyRefExpr::getBaseExpr"),
      add_named_node(&mut g, METHOD_GETPROPERTYDECL, "clang::MSPropertyRefExpr::getPropertyDecl"),
      add_named_node(&mut g, METHOD_ISARROW_3, "clang::MSPropertyRefExpr::isArrow"),
      add_named_node(&mut g, METHOD_GETMEMBERLOC_1, "clang::MSPropertyRefExpr::getMemberLoc"),
      add_named_node(&mut g, METHOD_GETQUALIFIERLOC_13, "clang::MSPropertyRefExpr::getQualifierLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MSPROPERTYREFEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_LIFETIMEBOUNDATTR, "clang::LifetimeBoundAttr");
  g.add_edge((CLASS_LIFETIMEBOUNDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LIFETIMEBOUNDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_DECLARATORDECL, "clang::DeclaratorDecl");
  g.add_edge((CLASS_DECLARATORDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DECLARATORDECL, META_SUBCLASS, CLASS_VALUEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTYPESOURCEINFO, "clang::DeclaratorDecl::getTypeSourceInfo"),
      add_named_node(&mut g, METHOD_GETINNERLOCSTART, "clang::DeclaratorDecl::getInnerLocStart"),
      add_named_node(&mut g, METHOD_GETOUTERLOCSTART, "clang::DeclaratorDecl::getOuterLocStart"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_6, "clang::DeclaratorDecl::getSourceRange"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_1, "clang::DeclaratorDecl::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETQUALIFIER_3, "clang::DeclaratorDecl::getQualifier"),
      add_named_node(&mut g, METHOD_GETQUALIFIERLOC, "clang::DeclaratorDecl::getQualifierLoc"),
      add_named_node(&mut g, METHOD_GETTRAILINGREQUIRESCLAUSE, "clang::DeclaratorDecl::getTrailingRequiresClause"),
      add_named_node(&mut g, METHOD_GETNUMTEMPLATEPARAMETERLISTS, "clang::DeclaratorDecl::getNumTemplateParameterLists"),
      add_named_node(&mut g, METHOD_GETTYPESPECSTARTLOC, "clang::DeclaratorDecl::getTypeSpecStartLoc"),
      add_named_node(&mut g, METHOD_GETTYPESPECENDLOC, "clang::DeclaratorDecl::getTypeSpecEndLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DECLARATORDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_LOOPHINTATTR, "clang::LoopHintAttr");
  g.add_edge((CLASS_LOOPHINTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LOOPHINTATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_NAMESPACEDECL, "clang::NamespaceDecl");
  g.add_edge((CLASS_NAMESPACEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NAMESPACEDECL, META_SUBCLASS, CLASS_NAMEDDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_ISANONYMOUSNAMESPACE, "clang::NamespaceDecl::isAnonymousNamespace"),
      add_named_node(&mut g, METHOD_ISINLINE, "clang::NamespaceDecl::isInline"),
      add_named_node(&mut g, METHOD_ISNESTED, "clang::NamespaceDecl::isNested"),
      add_named_node(&mut g, METHOD_GETORIGINALNAMESPACE, "clang::NamespaceDecl::getOriginalNamespace"),
      add_named_node(&mut g, METHOD_ISORIGINALNAMESPACE, "clang::NamespaceDecl::isOriginalNamespace"),
      add_named_node(&mut g, METHOD_GETANONYMOUSNAMESPACE, "clang::NamespaceDecl::getAnonymousNamespace"),
      add_named_node(&mut g, METHOD_GETCANONICALDECL_15, "clang::NamespaceDecl::getCanonicalDecl"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_19, "clang::NamespaceDecl::getSourceRange"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_2, "clang::NamespaceDecl::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETRBRACELOC_3, "clang::NamespaceDecl::getRBraceLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NAMESPACEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CARRIESDEPENDENCYATTR, "clang::CarriesDependencyAttr");
  g.add_edge((CLASS_CARRIESDEPENDENCYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CARRIESDEPENDENCYATTR, META_SUBCLASS, CLASS_INHERITABLEPARAMATTR));

  g.add_named_node(CLASS_LEAFATTR, "clang::LeafAttr");
  g.add_edge((CLASS_LEAFATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LEAFATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_RISCVINTERRUPTATTR, "clang::RISCVInterruptAttr");
  g.add_edge((CLASS_RISCVINTERRUPTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RISCVINTERRUPTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_INDIRECTGOTOSTMT, "clang::IndirectGotoStmt");
  g.add_edge((CLASS_INDIRECTGOTOSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_INDIRECTGOTOSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETGOTOLOC_1, "clang::IndirectGotoStmt::getGotoLoc"),
      add_named_node(&mut g, METHOD_GETSTARLOC, "clang::IndirectGotoStmt::getStarLoc"),
      add_named_node(&mut g, METHOD_GETTARGET, "clang::IndirectGotoStmt::getTarget"),
      add_named_node(&mut g, METHOD_GETCONSTANTTARGET, "clang::IndirectGotoStmt::getConstantTarget"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_86, "clang::IndirectGotoStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_85, "clang::IndirectGotoStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_74, "clang::IndirectGotoStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_INDIRECTGOTOSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OVERLOADABLEATTR, "clang::OverloadableAttr");
  g.add_edge((CLASS_OVERLOADABLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OVERLOADABLEATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_OBJCOBJECTTYPELOC, "clang::ObjCObjectTypeLoc");
  g.add_edge((CLASS_OBJCOBJECTTYPELOC, META_CLASS, META_CLANG_AST_NODE));

  g.add_named_node(CLASS_LAYOUTVERSIONATTR, "clang::LayoutVersionAttr");
  g.add_edge((CLASS_LAYOUTVERSIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LAYOUTVERSIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_INTERNALLINKAGEATTR, "clang::InternalLinkageAttr");
  g.add_edge((CLASS_INTERNALLINKAGEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_INTERNALLINKAGEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_ALWAYSDESTROYATTR, "clang::AlwaysDestroyAttr");
  g.add_edge((CLASS_ALWAYSDESTROYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ALWAYSDESTROYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_ELABORATEDTYPE, "clang::ElaboratedType");
  g.add_edge((CLASS_ELABORATEDTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ELABORATEDTYPE, META_SUBCLASS, CLASS_TYPEWITHKEYWORD));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETQUALIFIER_2, "clang::ElaboratedType::getQualifier"),
      add_named_node(&mut g, METHOD_GETNAMEDTYPE, "clang::ElaboratedType::getNamedType"),
      add_named_node(&mut g, METHOD_DESUGAR_18, "clang::ElaboratedType::desugar"),
      add_named_node(&mut g, METHOD_ISSUGARED_18, "clang::ElaboratedType::isSugared"),
      add_named_node(&mut g, METHOD_GETOWNEDTAGDECL, "clang::ElaboratedType::getOwnedTagDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ELABORATEDTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCDIRECTMEMBERSATTR, "clang::ObjCDirectMembersAttr");
  g.add_edge((CLASS_OBJCDIRECTMEMBERSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCDIRECTMEMBERSATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_OMPTASKLOOPDIRECTIVE, "clang::OMPTaskLoopDirective");
  g.add_edge((CLASS_OMPTASKLOOPDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTASKLOOPDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_OBJCMETHODDECL, "clang::ObjCMethodDecl");
  g.add_edge((CLASS_OBJCMETHODDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCMETHODDECL, META_SUBCLASS, CLASS_NAMEDDECL));

  g.add_named_node(CLASS_IBOUTLETCOLLECTIONATTR, "clang::IBOutletCollectionAttr");
  g.add_edge((CLASS_IBOUTLETCOLLECTIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_IBOUTLETCOLLECTIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OBJCPROTOCOLDECL, "clang::ObjCProtocolDecl");
  g.add_edge((CLASS_OBJCPROTOCOLDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCPROTOCOLDECL, META_SUBCLASS, CLASS_OBJCCONTAINERDECL));

  g.add_named_node(CLASS_DEPENDENTSIZEDMATRIXTYPE, "clang::DependentSizedMatrixType");
  g.add_edge((CLASS_DEPENDENTSIZEDMATRIXTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEPENDENTSIZEDMATRIXTYPE, META_SUBCLASS, CLASS_MATRIXTYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETROWEXPR, "clang::DependentSizedMatrixType::getRowExpr"),
      add_named_node(&mut g, METHOD_GETCOLUMNEXPR, "clang::DependentSizedMatrixType::getColumnExpr"),
      add_named_node(&mut g, METHOD_GETATTRIBUTELOC_2, "clang::DependentSizedMatrixType::getAttributeLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTSIZEDMATRIXTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DISABLETAILCALLSATTR, "clang::DisableTailCallsAttr");
  g.add_edge((CLASS_DISABLETAILCALLSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DISABLETAILCALLSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_FORMATATTR, "clang::FormatAttr");
  g.add_edge((CLASS_FORMATATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FORMATATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_FALLTHROUGHATTR, "clang::FallThroughAttr");
  g.add_edge((CLASS_FALLTHROUGHATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FALLTHROUGHATTR, META_SUBCLASS, CLASS_STMTATTR));

  g.add_named_node(CLASS_IBACTIONATTR, "clang::IBActionAttr");
  g.add_edge((CLASS_IBACTIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_IBACTIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_FLAGENUMATTR, "clang::FlagEnumAttr");
  g.add_edge((CLASS_FLAGENUMATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FLAGENUMATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CUDAHOSTATTR, "clang::CUDAHostAttr");
  g.add_edge((CLASS_CUDAHOSTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CUDAHOSTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_SEHTRYSTMT, "clang::SEHTryStmt");
  g.add_edge((CLASS_SEHTRYSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SEHTRYSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETBEGINLOC_113, "clang::SEHTryStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETTRYLOC_1, "clang::SEHTryStmt::getTryLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_112, "clang::SEHTryStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_GETISCXXTRY, "clang::SEHTryStmt::getIsCXXTry"),
      add_named_node(&mut g, METHOD_GETTRYBLOCK_1, "clang::SEHTryStmt::getTryBlock"),
      add_named_node(&mut g, METHOD_GETHANDLER, "clang::SEHTryStmt::getHandler"),
      add_named_node(&mut g, METHOD_GETEXCEPTHANDLER, "clang::SEHTryStmt::getExceptHandler"),
      add_named_node(&mut g, METHOD_GETFINALLYHANDLER, "clang::SEHTryStmt::getFinallyHandler"),
      add_named_node(&mut g, METHOD_CHILDREN_100, "clang::SEHTryStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SEHTRYSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FRIENDTEMPLATEDECL, "clang::FriendTemplateDecl");
  g.add_edge((CLASS_FRIENDTEMPLATEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FRIENDTEMPLATEDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETFRIENDTYPE_1, "clang::FriendTemplateDecl::getFriendType"),
      add_named_node(&mut g, METHOD_GETFRIENDDECL_1, "clang::FriendTemplateDecl::getFriendDecl"),
      add_named_node(&mut g, METHOD_GETFRIENDLOC_1, "clang::FriendTemplateDecl::getFriendLoc"),
      add_named_node(&mut g, METHOD_GETNUMTEMPLATEPARAMETERS, "clang::FriendTemplateDecl::getNumTemplateParameters"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FRIENDTEMPLATEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_HLSLRESOURCEBINDINGATTR, "clang::HLSLResourceBindingAttr");
  g.add_edge((CLASS_HLSLRESOURCEBINDINGATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_HLSLRESOURCEBINDINGATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_BUILTINTEMPLATEDECL, "clang::BuiltinTemplateDecl");
  g.add_edge((CLASS_BUILTINTEMPLATEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BUILTINTEMPLATEDECL, META_SUBCLASS, CLASS_TEMPLATEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSOURCERANGE_2, "clang::BuiltinTemplateDecl::getSourceRange"),
      add_named_node(&mut g, METHOD_GETBUILTINTEMPLATEKIND, "clang::BuiltinTemplateDecl::getBuiltinTemplateKind"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BUILTINTEMPLATEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPDECLAREMAPPERDECL, "clang::OMPDeclareMapperDecl");
  g.add_edge((CLASS_OMPDECLAREMAPPERDECL, META_CLASS, META_CLANG_AST_NODE));

  g.add_named_node(CLASS_DISABLESANITIZERINSTRUMENTATIONATTR, "clang::DisableSanitizerInstrumentationAttr");
  g.add_edge((CLASS_DISABLESANITIZERINSTRUMENTATIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DISABLESANITIZERINSTRUMENTATIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_GUARDEDVARATTR, "clang::GuardedVarAttr");
  g.add_edge((CLASS_GUARDEDVARATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_GUARDEDVARATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_TYPEDEFTYPE, "clang::TypedefType");
  g.add_edge((CLASS_TYPEDEFTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPEDEFTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETDECL_5, "clang::TypedefType::getDecl"),
      add_named_node(&mut g, METHOD_ISSUGARED_41, "clang::TypedefType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_41, "clang::TypedefType::desugar"),
      add_named_node(&mut g, METHOD_TYPEMATCHESDECL, "clang::TypedefType::typeMatchesDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPEDEFTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_WEAKIMPORTATTR, "clang::WeakImportAttr");
  g.add_edge((CLASS_WEAKIMPORTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_WEAKIMPORTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_FORMATARGATTR, "clang::FormatArgAttr");
  g.add_edge((CLASS_FORMATARGATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FORMATARGATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_EXTERNALSOURCESYMBOLATTR, "clang::ExternalSourceSymbolAttr");
  g.add_edge((CLASS_EXTERNALSOURCESYMBOLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_EXTERNALSOURCESYMBOLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CXXOPERATORCALLEXPR, "clang::CXXOperatorCallExpr");
  g.add_edge((CLASS_CXXOPERATORCALLEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXOPERATORCALLEXPR, META_SUBCLASS, CLASS_CALLEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETOPERATOR_1, "clang::CXXOperatorCallExpr::getOperator"),
      add_named_node(&mut g, METHOD_ISASSIGNMENTOP_1, "clang::CXXOperatorCallExpr::isAssignmentOp"),
      add_named_node(&mut g, METHOD_ISCOMPARISONOP_1, "clang::CXXOperatorCallExpr::isComparisonOp"),
      add_named_node(&mut g, METHOD_ISINFIXBINARYOP, "clang::CXXOperatorCallExpr::isInfixBinaryOp"),
      add_named_node(&mut g, METHOD_GETOPERATORLOC_3, "clang::CXXOperatorCallExpr::getOperatorLoc"),
      add_named_node(&mut g, METHOD_GETEXPRLOC_4, "clang::CXXOperatorCallExpr::getExprLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_35, "clang::CXXOperatorCallExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_34, "clang::CXXOperatorCallExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_41, "clang::CXXOperatorCallExpr::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXOPERATORCALLEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCPROPERTYIMPLDECL, "clang::ObjCPropertyImplDecl");
  g.add_edge((CLASS_OBJCPROPERTYIMPLDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCPROPERTYIMPLDECL, META_SUBCLASS, CLASS_DECL));

  g.add_named_node(CLASS_ENFORCETCBLEAFATTR, "clang::EnforceTCBLeafAttr");
  g.add_edge((CLASS_ENFORCETCBLEAFATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ENFORCETCBLEAFATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OBJCBRIDGEDCASTEXPR, "clang::ObjCBridgedCastExpr");
  g.add_edge((CLASS_OBJCBRIDGEDCASTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCBRIDGEDCASTEXPR, META_SUBCLASS, CLASS_EXPLICITCASTEXPR));

  g.add_named_node(CLASS_NAMEDDECL, "clang::NamedDecl");
  g.add_edge((CLASS_NAMEDDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NAMEDDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETIDENTIFIER_4, "clang::NamedDecl::getIdentifier"),
      add_named_node(&mut g, METHOD_GETNAME_3, "clang::NamedDecl::getName"),
      add_named_node(&mut g, METHOD_GETNAMEASSTRING, "clang::NamedDecl::getNameAsString"),
      add_named_node(&mut g, METHOD_GETDECLNAME, "clang::NamedDecl::getDeclName"),
      add_named_node(&mut g, METHOD_GETQUALIFIEDNAMEASSTRING, "clang::NamedDecl::getQualifiedNameAsString"),
      add_named_node(&mut g, METHOD_HASLINKAGE, "clang::NamedDecl::hasLinkage"),
      add_named_node(&mut g, METHOD_ISCXXCLASSMEMBER, "clang::NamedDecl::isCXXClassMember"),
      add_named_node(&mut g, METHOD_ISCXXINSTANCEMEMBER, "clang::NamedDecl::isCXXInstanceMember"),
      add_named_node(&mut g, METHOD_GETLINKAGEINTERNAL, "clang::NamedDecl::getLinkageInternal"),
      add_named_node(&mut g, METHOD_GETFORMALLINKAGE, "clang::NamedDecl::getFormalLinkage"),
      add_named_node(&mut g, METHOD_HASEXTERNALFORMALLINKAGE, "clang::NamedDecl::hasExternalFormalLinkage"),
      add_named_node(&mut g, METHOD_ISEXTERNALLYVISIBLE, "clang::NamedDecl::isExternallyVisible"),
      add_named_node(&mut g, METHOD_ISEXTERNALLYDECLARABLE, "clang::NamedDecl::isExternallyDeclarable"),
      add_named_node(&mut g, METHOD_GETVISIBILITY_1, "clang::NamedDecl::getVisibility"),
      add_named_node(&mut g, METHOD_GETLINKAGEANDVISIBILITY_1, "clang::NamedDecl::getLinkageAndVisibility"),
      add_named_node(&mut g, METHOD_ISLINKAGEVALID, "clang::NamedDecl::isLinkageValid"),
      add_named_node(&mut g, METHOD_HASLINKAGEBEENCOMPUTED, "clang::NamedDecl::hasLinkageBeenComputed"),
      add_named_node(&mut g, METHOD_GETUNDERLYINGDECL, "clang::NamedDecl::getUnderlyingDecl"),
      add_named_node(&mut g, METHOD_GETMOSTRECENTDECL_6, "clang::NamedDecl::getMostRecentDecl"),
      add_named_node(&mut g, METHOD_GETOBJCFSTRINGFORMATTINGFAMILY, "clang::NamedDecl::getObjCFStringFormattingFamily"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NAMEDDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_LOCKSEXCLUDEDATTR, "clang::LocksExcludedAttr");
  g.add_edge((CLASS_LOCKSEXCLUDEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LOCKSEXCLUDEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_DEPENDENTTYPEOFEXPRTYPE, "clang::DependentTypeOfExprType");
  g.add_edge((CLASS_DEPENDENTTYPEOFEXPRTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEPENDENTTYPEOFEXPRTYPE, META_SUBCLASS, CLASS_TYPEOFEXPRTYPE));

  g.add_named_node(CLASS_EXCLUDEFROMEXPLICITINSTANTIATIONATTR, "clang::ExcludeFromExplicitInstantiationAttr");
  g.add_edge((CLASS_EXCLUDEFROMEXPLICITINSTANTIATIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_EXCLUDEFROMEXPLICITINSTANTIATIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OBJCEXPLICITPROTOCOLIMPLATTR, "clang::ObjCExplicitProtocolImplAttr");
  g.add_edge((CLASS_OBJCEXPLICITPROTOCOLIMPLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCEXPLICITPROTOCOLIMPLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_DEPENDENTTEMPLATESPECIALIZATIONTYPE, "clang::DependentTemplateSpecializationType");
  g.add_edge((CLASS_DEPENDENTTEMPLATESPECIALIZATIONTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEPENDENTTEMPLATESPECIALIZATIONTYPE, META_SUBCLASS, CLASS_TYPEWITHKEYWORD));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETQUALIFIER_1, "clang::DependentTemplateSpecializationType::getQualifier"),
      add_named_node(&mut g, METHOD_GETIDENTIFIER_1, "clang::DependentTemplateSpecializationType::getIdentifier"),
      add_named_node(&mut g, METHOD_TEMPLATE_ARGUMENTS, "clang::DependentTemplateSpecializationType::template_arguments"),
      add_named_node(&mut g, METHOD_ISSUGARED_16, "clang::DependentTemplateSpecializationType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_16, "clang::DependentTemplateSpecializationType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTTEMPLATESPECIALIZATIONTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SWIFTCALLATTR, "clang::SwiftCallAttr");
  g.add_edge((CLASS_SWIFTCALLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTCALLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_ENABLEIFATTR, "clang::EnableIfAttr");
  g.add_edge((CLASS_ENABLEIFATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ENABLEIFATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_SETTYPESTATEATTR, "clang::SetTypestateAttr");
  g.add_edge((CLASS_SETTYPESTATEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SETTYPESTATEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OBJCARRAYLITERAL, "clang::ObjCArrayLiteral");
  g.add_edge((CLASS_OBJCARRAYLITERAL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCARRAYLITERAL, META_SUBCLASS, CLASS_EXPR));

  g.add_named_node(CLASS_OSRETURNSRETAINEDATTR, "clang::OSReturnsRetainedAttr");
  g.add_edge((CLASS_OSRETURNSRETAINEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OSRETURNSRETAINEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_ATOMICTYPELOC, "clang::AtomicTypeLoc");
  g.add_edge((CLASS_ATOMICTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETVALUELOC_1, "clang::AtomicTypeLoc::getValueLoc"),
      add_named_node(&mut g, METHOD_GETLOCALSOURCERANGE_9, "clang::AtomicTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, METHOD_GETKWLOC_1, "clang::AtomicTypeLoc::getKWLoc"),
      add_named_node(&mut g, METHOD_GETLPARENLOC_13, "clang::AtomicTypeLoc::getLParenLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_31, "clang::AtomicTypeLoc::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETPARENSRANGE, "clang::AtomicTypeLoc::getParensRange"),
      add_named_node(&mut g, METHOD_GETINNERTYPE_8, "clang::AtomicTypeLoc::getInnerType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ATOMICTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OWNERATTR, "clang::OwnerAttr");
  g.add_edge((CLASS_OWNERATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OWNERATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_STMTEXPR, "clang::StmtExpr");
  g.add_edge((CLASS_STMTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_STMTEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSUBSTMT_5, "clang::StmtExpr::getSubStmt"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_118, "clang::StmtExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_117, "clang::StmtExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_GETLPARENLOC_9, "clang::StmtExpr::getLParenLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_24, "clang::StmtExpr::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETTEMPLATEDEPTH_1, "clang::StmtExpr::getTemplateDepth"),
      add_named_node(&mut g, METHOD_CHILDREN_106, "clang::StmtExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_STMTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NOMICROMIPSATTR, "clang::NoMicroMipsAttr");
  g.add_edge((CLASS_NOMICROMIPSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOMICROMIPSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_DEPRECATEDATTR, "clang::DeprecatedAttr");
  g.add_edge((CLASS_DEPRECATEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEPRECATEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_TESTTYPESTATEATTR, "clang::TestTypestateAttr");
  g.add_edge((CLASS_TESTTYPESTATEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TESTTYPESTATEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CFAUDITEDTRANSFERATTR, "clang::CFAuditedTransferAttr");
  g.add_edge((CLASS_CFAUDITEDTRANSFERATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CFAUDITEDTRANSFERATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_HLSLPARAMMODIFIERATTR, "clang::HLSLParamModifierAttr");
  g.add_edge((CLASS_HLSLPARAMMODIFIERATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_HLSLPARAMMODIFIERATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_DLLIMPORTSTATICLOCALATTR, "clang::DLLImportStaticLocalAttr");
  g.add_edge((CLASS_DLLIMPORTSTATICLOCALATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DLLIMPORTSTATICLOCALATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OMPPARALLELMASTERTASKLOOPDIRECTIVE, "clang::OMPParallelMasterTaskLoopDirective");
  g.add_edge((CLASS_OMPPARALLELMASTERTASKLOOPDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPPARALLELMASTERTASKLOOPDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_BLOCKSATTR, "clang::BlocksAttr");
  g.add_edge((CLASS_BLOCKSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BLOCKSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_NODUPLICATEATTR, "clang::NoDuplicateAttr");
  g.add_edge((CLASS_NODUPLICATEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NODUPLICATEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_OPENCLGENERICADDRESSSPACEATTR, "clang::OpenCLGenericAddressSpaceAttr");
  g.add_edge((CLASS_OPENCLGENERICADDRESSSPACEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OPENCLGENERICADDRESSSPACEATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_OSCONSUMEDATTR, "clang::OSConsumedAttr");
  g.add_edge((CLASS_OSCONSUMEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OSCONSUMEDATTR, META_SUBCLASS, CLASS_INHERITABLEPARAMATTR));

  g.add_named_node(CLASS_DLLEXPORTSTATICLOCALATTR, "clang::DLLExportStaticLocalAttr");
  g.add_edge((CLASS_DLLEXPORTSTATICLOCALATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DLLEXPORTSTATICLOCALATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_BTFTAGATTRIBUTEDTYPE, "clang::BTFTagAttributedType");
  g.add_edge((CLASS_BTFTAGATTRIBUTEDTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BTFTAGATTRIBUTEDTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETWRAPPEDTYPE, "clang::BTFTagAttributedType::getWrappedType"),
      add_named_node(&mut g, METHOD_GETATTR, "clang::BTFTagAttributedType::getAttr"),
      add_named_node(&mut g, METHOD_ISSUGARED_3, "clang::BTFTagAttributedType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_3, "clang::BTFTagAttributedType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BTFTAGATTRIBUTEDTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_STANDALONEDEBUGATTR, "clang::StandaloneDebugAttr");
  g.add_edge((CLASS_STANDALONEDEBUGATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_STANDALONEDEBUGATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_DEPENDENTNAMETYPE, "clang::DependentNameType");
  g.add_edge((CLASS_DEPENDENTNAMETYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEPENDENTNAMETYPE, META_SUBCLASS, CLASS_TYPEWITHKEYWORD));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETQUALIFIER, "clang::DependentNameType::getQualifier"),
      add_named_node(&mut g, METHOD_GETIDENTIFIER, "clang::DependentNameType::getIdentifier"),
      add_named_node(&mut g, METHOD_ISSUGARED_13, "clang::DependentNameType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_13, "clang::DependentNameType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTNAMETYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NOINSTRUMENTFUNCTIONATTR, "clang::NoInstrumentFunctionAttr");
  g.add_edge((CLASS_NOINSTRUMENTFUNCTIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOINSTRUMENTFUNCTIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_DECLTYPETYPE, "clang::DecltypeType");
  g.add_edge((CLASS_DECLTYPETYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DECLTYPETYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETUNDERLYINGEXPR, "clang::DecltypeType::getUnderlyingExpr"),
      add_named_node(&mut g, METHOD_GETUNDERLYINGTYPE, "clang::DecltypeType::getUnderlyingType"),
      add_named_node(&mut g, METHOD_DESUGAR_9, "clang::DecltypeType::desugar"),
      add_named_node(&mut g, METHOD_ISSUGARED_9, "clang::DecltypeType::isSugared"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DECLTYPETYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTBITINTTYPE, "clang::DependentBitIntType");
  g.add_edge((CLASS_DEPENDENTBITINTTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEPENDENTBITINTTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_ISUNSIGNED_1, "clang::DependentBitIntType::isUnsigned"),
      add_named_node(&mut g, METHOD_ISSIGNED_1, "clang::DependentBitIntType::isSigned"),
      add_named_node(&mut g, METHOD_GETNUMBITSEXPR, "clang::DependentBitIntType::getNumBitsExpr"),
      add_named_node(&mut g, METHOD_ISSUGARED_12, "clang::DependentBitIntType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_12, "clang::DependentBitIntType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTBITINTTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_UNNAMEDGLOBALCONSTANTDECL, "clang::UnnamedGlobalConstantDecl");
  g.add_edge((CLASS_UNNAMEDGLOBALCONSTANTDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNNAMEDGLOBALCONSTANTDECL, META_SUBCLASS, CLASS_VALUEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETVALUE_3, "clang::UnnamedGlobalConstantDecl::getValue"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNNAMEDGLOBALCONSTANTDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CLEANUPATTR, "clang::CleanupAttr");
  g.add_edge((CLASS_CLEANUPATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CLEANUPATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_TYPEOFEXPRTYPE, "clang::TypeOfExprType");
  g.add_edge((CLASS_TYPEOFEXPRTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPEOFEXPRTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETUNDERLYINGEXPR_1, "clang::TypeOfExprType::getUnderlyingExpr"),
      add_named_node(&mut g, METHOD_GETKIND_1, "clang::TypeOfExprType::getKind"),
      add_named_node(&mut g, METHOD_DESUGAR_39, "clang::TypeOfExprType::desugar"),
      add_named_node(&mut g, METHOD_ISSUGARED_39, "clang::TypeOfExprType::isSugared"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPEOFEXPRTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_UNUSEDATTR, "clang::UnusedAttr");
  g.add_edge((CLASS_UNUSEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNUSEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_INCOMPLETEARRAYTYPE, "clang::IncompleteArrayType");
  g.add_edge((CLASS_INCOMPLETEARRAYTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_INCOMPLETEARRAYTYPE, META_SUBCLASS, CLASS_ARRAYTYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_ISSUGARED_23, "clang::IncompleteArrayType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_23, "clang::IncompleteArrayType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_INCOMPLETEARRAYTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPEDEFNAMEDECL, "clang::TypedefNameDecl");
  g.add_edge((CLASS_TYPEDEFNAMEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPEDEFNAMEDECL, META_SUBCLASS, CLASS_TYPEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_ISMODED, "clang::TypedefNameDecl::isModed"),
      add_named_node(&mut g, METHOD_GETTYPESOURCEINFO_1, "clang::TypedefNameDecl::getTypeSourceInfo"),
      add_named_node(&mut g, METHOD_GETUNDERLYINGTYPE_4, "clang::TypedefNameDecl::getUnderlyingType"),
      add_named_node(&mut g, METHOD_GETCANONICALDECL_20, "clang::TypedefNameDecl::getCanonicalDecl"),
      add_named_node(&mut g, METHOD_ISTRANSPARENTTAG, "clang::TypedefNameDecl::isTransparentTag"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPEDEFNAMEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_XRAYINSTRUMENTATTR, "clang::XRayInstrumentAttr");
  g.add_edge((CLASS_XRAYINSTRUMENTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_XRAYINSTRUMENTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_SUBSTTEMPLATETYPEPARMTYPE, "clang::SubstTemplateTypeParmType");
  g.add_edge((CLASS_SUBSTTEMPLATETYPEPARMTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SUBSTTEMPLATETYPEPARMTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETREPLACEMENTTYPE, "clang::SubstTemplateTypeParmType::getReplacementType"),
      add_named_node(&mut g, METHOD_GETASSOCIATEDDECL_1, "clang::SubstTemplateTypeParmType::getAssociatedDecl"),
      add_named_node(&mut g, METHOD_GETREPLACEDPARAMETER_1, "clang::SubstTemplateTypeParmType::getReplacedParameter"),
      add_named_node(&mut g, METHOD_GETINDEX_1, "clang::SubstTemplateTypeParmType::getIndex"),
      add_named_node(&mut g, METHOD_GETPACKINDEX, "clang::SubstTemplateTypeParmType::getPackIndex"),
      add_named_node(&mut g, METHOD_ISSUGARED_36, "clang::SubstTemplateTypeParmType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_36, "clang::SubstTemplateTypeParmType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SUBSTTEMPLATETYPEPARMTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ATTRIBUTEDTYPE, "clang::AttributedType");
  g.add_edge((CLASS_ATTRIBUTEDTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ATTRIBUTEDTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETATTRKIND, "clang::AttributedType::getAttrKind"),
      add_named_node(&mut g, METHOD_GETMODIFIEDTYPE, "clang::AttributedType::getModifiedType"),
      add_named_node(&mut g, METHOD_GETEQUIVALENTTYPE, "clang::AttributedType::getEquivalentType"),
      add_named_node(&mut g, METHOD_ISSUGARED_2, "clang::AttributedType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_2, "clang::AttributedType::desugar"),
      add_named_node(&mut g, METHOD_ISQUALIFIER, "clang::AttributedType::isQualifier"),
      add_named_node(&mut g, METHOD_ISMSTYPESPEC, "clang::AttributedType::isMSTypeSpec"),
      add_named_node(&mut g, METHOD_ISWEBASSEMBLYFUNCREFSPEC, "clang::AttributedType::isWebAssemblyFuncrefSpec"),
      add_named_node(&mut g, METHOD_ISCALLINGCONV, "clang::AttributedType::isCallingConv"),
      add_named_node(&mut g, METHOD_GETIMMEDIATENULLABILITY, "clang::AttributedType::getImmediateNullability"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ATTRIBUTEDTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BLOCKDECL, "clang::BlockDecl");
  g.add_edge((CLASS_BLOCKDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BLOCKDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETCARETLOCATION, "clang::BlockDecl::getCaretLocation"),
      add_named_node(&mut g, METHOD_ISVARIADIC_1, "clang::BlockDecl::isVariadic"),
      add_named_node(&mut g, METHOD_GETCOMPOUNDBODY, "clang::BlockDecl::getCompoundBody"),
      add_named_node(&mut g, METHOD_GETBODY, "clang::BlockDecl::getBody"),
      add_named_node(&mut g, METHOD_GETSIGNATUREASWRITTEN, "clang::BlockDecl::getSignatureAsWritten"),
      add_named_node(&mut g, METHOD_PARAMETERS, "clang::BlockDecl::parameters"),
      add_named_node(&mut g, METHOD_PARAM_EMPTY, "clang::BlockDecl::param_empty"),
      add_named_node(&mut g, METHOD_PARAM_BEGIN_2, "clang::BlockDecl::param_begin"),
      add_named_node(&mut g, METHOD_PARAM_END_2, "clang::BlockDecl::param_end"),
      add_named_node(&mut g, METHOD_PARAM_SIZE, "clang::BlockDecl::param_size"),
      add_named_node(&mut g, METHOD_GETNUMPARAMS_1, "clang::BlockDecl::getNumParams"),
      add_named_node(&mut g, METHOD_HASCAPTURES, "clang::BlockDecl::hasCaptures"),
      add_named_node(&mut g, METHOD_GETNUMCAPTURES, "clang::BlockDecl::getNumCaptures"),
      add_named_node(&mut g, METHOD_CAPTURES, "clang::BlockDecl::captures"),
      add_named_node(&mut g, METHOD_CAPTURE_BEGIN_2, "clang::BlockDecl::capture_begin"),
      add_named_node(&mut g, METHOD_CAPTURE_END_2, "clang::BlockDecl::capture_end"),
      add_named_node(&mut g, METHOD_CAPTURESCXXTHIS, "clang::BlockDecl::capturesCXXThis"),
      add_named_node(&mut g, METHOD_BLOCKMISSINGRETURNTYPE, "clang::BlockDecl::blockMissingReturnType"),
      add_named_node(&mut g, METHOD_ISCONVERSIONFROMLAMBDA, "clang::BlockDecl::isConversionFromLambda"),
      add_named_node(&mut g, METHOD_DOESNOTESCAPE, "clang::BlockDecl::doesNotEscape"),
      add_named_node(&mut g, METHOD_CANAVOIDCOPYTOHEAP, "clang::BlockDecl::canAvoidCopyToHeap"),
      add_named_node(&mut g, METHOD_GETBLOCKMANGLINGNUMBER, "clang::BlockDecl::getBlockManglingNumber"),
      add_named_node(&mut g, METHOD_GETBLOCKMANGLINGCONTEXTDECL, "clang::BlockDecl::getBlockManglingContextDecl"),
      add_named_node(&mut g, METHOD_GETSOURCERANGE_1, "clang::BlockDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BLOCKDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPEOFEXPRTYPELOC, "clang::TypeOfExprTypeLoc");
  g.add_edge((CLASS_TYPEOFEXPRTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETUNDERLYINGEXPR_3, "clang::TypeOfExprTypeLoc::getUnderlyingExpr"),
      add_named_node(&mut g, METHOD_GETLOCALSOURCERANGE_10, "clang::TypeOfExprTypeLoc::getLocalSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPEOFEXPRTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCMETHODFAMILYATTR, "clang::ObjCMethodFamilyAttr");
  g.add_edge((CLASS_OBJCMETHODFAMILYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCMETHODFAMILYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_PIPETYPE, "clang::PipeType");
  g.add_edge((CLASS_PIPETYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PIPETYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETELEMENTTYPE_5, "clang::PipeType::getElementType"),
      add_named_node(&mut g, METHOD_ISSUGARED_31, "clang::PipeType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_31, "clang::PipeType::desugar"),
      add_named_node(&mut g, METHOD_ISREADONLY, "clang::PipeType::isReadOnly"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PIPETYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BITINTTYPELOC, "clang::BitIntTypeLoc");
  g.add_edge((CLASS_BITINTTYPELOC, META_CLASS, META_CLANG_AST_NODE));

  g.add_named_node(CLASS_LVALUEREFERENCETYPE, "clang::LValueReferenceType");
  g.add_edge((CLASS_LVALUEREFERENCETYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LVALUEREFERENCETYPE, META_SUBCLASS, CLASS_REFERENCETYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_ISSUGARED_25, "clang::LValueReferenceType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_25, "clang::LValueReferenceType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_LVALUEREFERENCETYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXTHISEXPR, "clang::CXXThisExpr");
  g.add_edge((CLASS_CXXTHISEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXTHISEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETLOCATION_5, "clang::CXXThisExpr::getLocation"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_42, "clang::CXXThisExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_41, "clang::CXXThisExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_ISIMPLICIT_1, "clang::CXXThisExpr::isImplicit"),
      add_named_node(&mut g, METHOD_CHILDREN_31, "clang::CXXThisExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXTHISEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCOBJECTTYPEIMPL, "clang::ObjCObjectTypeImpl");
  g.add_edge((CLASS_OBJCOBJECTTYPEIMPL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCOBJECTTYPEIMPL, META_SUBCLASS, CLASS_OBJCOBJECTTYPE));

  g.add_named_node(CLASS_TEMPLATESPECIALIZATIONTYPE, "clang::TemplateSpecializationType");
  g.add_edge((CLASS_TEMPLATESPECIALIZATIONTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TEMPLATESPECIALIZATIONTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_ISCURRENTINSTANTIATION, "clang::TemplateSpecializationType::isCurrentInstantiation"),
      add_named_node(&mut g, METHOD_ISTYPEALIAS, "clang::TemplateSpecializationType::isTypeAlias"),
      add_named_node(&mut g, METHOD_GETALIASEDTYPE, "clang::TemplateSpecializationType::getAliasedType"),
      add_named_node(&mut g, METHOD_GETTEMPLATENAME_2, "clang::TemplateSpecializationType::getTemplateName"),
      add_named_node(&mut g, METHOD_TEMPLATE_ARGUMENTS_1, "clang::TemplateSpecializationType::template_arguments"),
      add_named_node(&mut g, METHOD_ISSUGARED_37, "clang::TemplateSpecializationType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_37, "clang::TemplateSpecializationType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TEMPLATESPECIALIZATIONTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCOBJECTPOINTERTYPE, "clang::ObjCObjectPointerType");
  g.add_edge((CLASS_OBJCOBJECTPOINTERTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCOBJECTPOINTERTYPE, META_SUBCLASS, CLASS_TYPE));

  g.add_named_node(CLASS_CAPTUREDRECORDATTR, "clang::CapturedRecordAttr");
  g.add_edge((CLASS_CAPTUREDRECORDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CAPTUREDRECORDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_PACKEXPANSIONTYPE, "clang::PackExpansionType");
  g.add_edge((CLASS_PACKEXPANSIONTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PACKEXPANSIONTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETPATTERN, "clang::PackExpansionType::getPattern"),
      add_named_node(&mut g, METHOD_GETNUMEXPANSIONS, "clang::PackExpansionType::getNumExpansions"),
      add_named_node(&mut g, METHOD_ISSUGARED_29, "clang::PackExpansionType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_29, "clang::PackExpansionType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PACKEXPANSIONTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTVECTORTYPELOC, "clang::DependentVectorTypeLoc");
  g.add_edge((CLASS_DEPENDENTVECTORTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETNAMELOC_4, "clang::DependentVectorTypeLoc::getNameLoc"),
      add_named_node(&mut g, METHOD_GETLOCALSOURCERANGE_11, "clang::DependentVectorTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, METHOD_GETELEMENTLOC_3, "clang::DependentVectorTypeLoc::getElementLoc"),
      add_named_node(&mut g, METHOD_GETINNERTYPE_9, "clang::DependentVectorTypeLoc::getInnerType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTVECTORTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTSIZEDARRAYTYPE, "clang::DependentSizedArrayType");
  g.add_edge((CLASS_DEPENDENTSIZEDARRAYTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEPENDENTSIZEDARRAYTYPE, META_SUBCLASS, CLASS_ARRAYTYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSIZEEXPR_1, "clang::DependentSizedArrayType::getSizeExpr"),
      add_named_node(&mut g, METHOD_GETBRACKETSRANGE, "clang::DependentSizedArrayType::getBracketsRange"),
      add_named_node(&mut g, METHOD_GETLBRACKETLOC, "clang::DependentSizedArrayType::getLBracketLoc"),
      add_named_node(&mut g, METHOD_GETRBRACKETLOC, "clang::DependentSizedArrayType::getRBracketLoc"),
      add_named_node(&mut g, METHOD_ISSUGARED_14, "clang::DependentSizedArrayType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_14, "clang::DependentSizedArrayType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTSIZEDARRAYTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_INJECTEDCLASSNAMETYPE, "clang::InjectedClassNameType");
  g.add_edge((CLASS_INJECTEDCLASSNAMETYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_INJECTEDCLASSNAMETYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETINJECTEDSPECIALIZATIONTYPE, "clang::InjectedClassNameType::getInjectedSpecializationType"),
      add_named_node(&mut g, METHOD_GETINJECTEDTST, "clang::InjectedClassNameType::getInjectedTST"),
      add_named_node(&mut g, METHOD_GETTEMPLATENAME_1, "clang::InjectedClassNameType::getTemplateName"),
      add_named_node(&mut g, METHOD_GETDECL_1, "clang::InjectedClassNameType::getDecl"),
      add_named_node(&mut g, METHOD_ISSUGARED_24, "clang::InjectedClassNameType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_24, "clang::InjectedClassNameType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_INJECTEDCLASSNAMETYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ADDRESSSPACEATTR, "clang::AddressSpaceAttr");
  g.add_edge((CLASS_ADDRESSSPACEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ADDRESSSPACEATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_OVERRIDEATTR, "clang::OverrideAttr");
  g.add_edge((CLASS_OVERRIDEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OVERRIDEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_USINGTYPE, "clang::UsingType");
  g.add_edge((CLASS_USINGTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_USINGTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETFOUNDDECL, "clang::UsingType::getFoundDecl"),
      add_named_node(&mut g, METHOD_GETUNDERLYINGTYPE_3, "clang::UsingType::getUnderlyingType"),
      add_named_node(&mut g, METHOD_ISSUGARED_44, "clang::UsingType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_44, "clang::UsingType::desugar"),
      add_named_node(&mut g, METHOD_TYPEMATCHESDECL_1, "clang::UsingType::typeMatchesDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_USINGTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FUNCTIONTYPE, "clang::FunctionType");
  g.add_edge((CLASS_FUNCTIONTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FUNCTIONTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETRETURNTYPE, "clang::FunctionType::getReturnType"),
      add_named_node(&mut g, METHOD_GETHASREGPARM, "clang::FunctionType::getHasRegParm"),
      add_named_node(&mut g, METHOD_GETREGPARMTYPE, "clang::FunctionType::getRegParmType"),
      add_named_node(&mut g, METHOD_GETNORETURNATTR, "clang::FunctionType::getNoReturnAttr"),
      add_named_node(&mut g, METHOD_GETCMSENSCALLATTR, "clang::FunctionType::getCmseNSCallAttr"),
      add_named_node(&mut g, METHOD_GETCALLCONV, "clang::FunctionType::getCallConv"),
      add_named_node(&mut g, METHOD_GETEXTINFO, "clang::FunctionType::getExtInfo"),
      add_named_node(&mut g, METHOD_ISCONST, "clang::FunctionType::isConst"),
      add_named_node(&mut g, METHOD_ISVOLATILE, "clang::FunctionType::isVolatile"),
      add_named_node(&mut g, METHOD_ISRESTRICT, "clang::FunctionType::isRestrict"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FUNCTIONTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_EXTVECTORTYPE, "clang::ExtVectorType");
  g.add_edge((CLASS_EXTVECTORTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_EXTVECTORTYPE, META_SUBCLASS, CLASS_VECTORTYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_ISSUGARED_20, "clang::ExtVectorType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_20, "clang::ExtVectorType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_EXTVECTORTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXDYNAMICCASTEXPR, "clang::CXXDynamicCastExpr");
  g.add_edge((CLASS_CXXDYNAMICCASTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXDYNAMICCASTEXPR, META_SUBCLASS, CLASS_CXXNAMEDCASTEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_ISALWAYSNULL, "clang::CXXDynamicCastExpr::isAlwaysNull"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXDYNAMICCASTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTADDRESSSPACETYPE, "clang::DependentAddressSpaceType");
  g.add_edge((CLASS_DEPENDENTADDRESSSPACETYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEPENDENTADDRESSSPACETYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETADDRSPACEEXPR, "clang::DependentAddressSpaceType::getAddrSpaceExpr"),
      add_named_node(&mut g, METHOD_GETPOINTEETYPE_2, "clang::DependentAddressSpaceType::getPointeeType"),
      add_named_node(&mut g, METHOD_GETATTRIBUTELOC, "clang::DependentAddressSpaceType::getAttributeLoc"),
      add_named_node(&mut g, METHOD_ISSUGARED_11, "clang::DependentAddressSpaceType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_11, "clang::DependentAddressSpaceType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTADDRESSSPACETYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_VECRETURNATTR, "clang::VecReturnAttr");
  g.add_edge((CLASS_VECRETURNATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_VECRETURNATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_DECAYEDTYPE, "clang::DecayedType");
  g.add_edge((CLASS_DECAYEDTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DECAYEDTYPE, META_SUBCLASS, CLASS_ADJUSTEDTYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETDECAYEDTYPE, "clang::DecayedType::getDecayedType"),
      add_named_node(&mut g, METHOD_GETPOINTEETYPE_1, "clang::DecayedType::getPointeeType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DECAYEDTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CAPABILITYATTR, "clang::CapabilityAttr");
  g.add_edge((CLASS_CAPABILITYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CAPABILITYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_MEMBERPOINTERTYPE, "clang::MemberPointerType");
  g.add_edge((CLASS_MEMBERPOINTERTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MEMBERPOINTERTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETPOINTEETYPE_3, "clang::MemberPointerType::getPointeeType"),
      add_named_node(&mut g, METHOD_ISMEMBERFUNCTIONPOINTER, "clang::MemberPointerType::isMemberFunctionPointer"),
      add_named_node(&mut g, METHOD_ISMEMBERDATAPOINTER, "clang::MemberPointerType::isMemberDataPointer"),
      add_named_node(&mut g, METHOD_GETCLASS, "clang::MemberPointerType::getClass"),
      add_named_node(&mut g, METHOD_GETMOSTRECENTCXXRECORDDECL, "clang::MemberPointerType::getMostRecentCXXRecordDecl"),
      add_named_node(&mut g, METHOD_ISSUGARED_28, "clang::MemberPointerType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_28, "clang::MemberPointerType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MEMBERPOINTERTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPEDEFTYPELOC, "clang::TypedefTypeLoc");
  g.add_edge((CLASS_TYPEDEFTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTYPEDEFNAMEDECL, "clang::TypedefTypeLoc::getTypedefNameDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPEDEFTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ATTR, "clang::Attr");
  g.add_edge((CLASS_ATTR, META_CLASS, META_CLANG_AST_NODE));

  g.add_named_node(CLASS_INJECTEDCLASSNAMETYPELOC, "clang::InjectedClassNameTypeLoc");
  g.add_edge((CLASS_INJECTEDCLASSNAMETYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETDECL_10, "clang::InjectedClassNameTypeLoc::getDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_INJECTEDCLASSNAMETYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_RVALUEREFERENCETYPE, "clang::RValueReferenceType");
  g.add_edge((CLASS_RVALUEREFERENCETYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RVALUEREFERENCETYPE, META_SUBCLASS, CLASS_REFERENCETYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_ISSUGARED_33, "clang::RValueReferenceType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_33, "clang::RValueReferenceType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_RVALUEREFERENCETYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ENUMTYPE, "clang::EnumType");
  g.add_edge((CLASS_ENUMTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ENUMTYPE, META_SUBCLASS, CLASS_TAGTYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETDECL, "clang::EnumType::getDecl"),
      add_named_node(&mut g, METHOD_ISSUGARED_19, "clang::EnumType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_19, "clang::EnumType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ENUMTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTSIZEDMATRIXTYPELOC, "clang::DependentSizedMatrixTypeLoc");
  g.add_edge((CLASS_DEPENDENTSIZEDMATRIXTYPELOC, META_CLASS, META_CLANG_AST_NODE));

  g.add_named_node(CLASS_ATOMICTYPE, "clang::AtomicType");
  g.add_edge((CLASS_ATOMICTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ATOMICTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETVALUETYPE, "clang::AtomicType::getValueType"),
      add_named_node(&mut g, METHOD_ISSUGARED_1, "clang::AtomicType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_1, "clang::AtomicType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ATOMICTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_USINGTYPELOC, "clang::UsingTypeLoc");
  g.add_edge((CLASS_USINGTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETUNDERLYINGTYPE_5, "clang::UsingTypeLoc::getUnderlyingType"),
      add_named_node(&mut g, METHOD_GETFOUNDDECL_4, "clang::UsingTypeLoc::getFoundDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_USINGTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CONSTANTMATRIXTYPELOC, "clang::ConstantMatrixTypeLoc");
  g.add_edge((CLASS_CONSTANTMATRIXTYPELOC, META_CLASS, META_CLANG_AST_NODE));

  g.add_named_node(CLASS_MACROQUALIFIEDTYPELOC, "clang::MacroQualifiedTypeLoc");
  g.add_edge((CLASS_MACROQUALIFIEDTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETINNERLOC_1, "clang::MacroQualifiedTypeLoc::getInnerLoc"),
      add_named_node(&mut g, METHOD_GETMACROIDENTIFIER_1, "clang::MacroQualifiedTypeLoc::getMacroIdentifier"),
      add_named_node(&mut g, METHOD_GETEXPANSIONLOC, "clang::MacroQualifiedTypeLoc::getExpansionLoc"),
      add_named_node(&mut g, METHOD_GETINNERTYPE_10, "clang::MacroQualifiedTypeLoc::getInnerType"),
      add_named_node(&mut g, METHOD_GETLOCALSOURCERANGE_12, "clang::MacroQualifiedTypeLoc::getLocalSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MACROQUALIFIEDTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTNAMETYPELOC, "clang::DependentNameTypeLoc");
  g.add_edge((CLASS_DEPENDENTNAMETYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETELABORATEDKEYWORDLOC, "clang::DependentNameTypeLoc::getElaboratedKeywordLoc"),
      add_named_node(&mut g, METHOD_GETQUALIFIERLOC_16, "clang::DependentNameTypeLoc::getQualifierLoc"),
      add_named_node(&mut g, METHOD_GETNAMELOC_5, "clang::DependentNameTypeLoc::getNameLoc"),
      add_named_node(&mut g, METHOD_GETLOCALSOURCERANGE_13, "clang::DependentNameTypeLoc::getLocalSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTNAMETYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTADDRESSSPACETYPELOC, "clang::DependentAddressSpaceTypeLoc");
  g.add_edge((CLASS_DEPENDENTADDRESSSPACETYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETATTRNAMELOC, "clang::DependentAddressSpaceTypeLoc::getAttrNameLoc"),
      add_named_node(&mut g, METHOD_GETATTREXPROPERAND, "clang::DependentAddressSpaceTypeLoc::getAttrExprOperand"),
      add_named_node(&mut g, METHOD_GETATTROPERANDPARENSRANGE, "clang::DependentAddressSpaceTypeLoc::getAttrOperandParensRange"),
      add_named_node(&mut g, METHOD_GETLOCALSOURCERANGE_14, "clang::DependentAddressSpaceTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, METHOD_GETINNERTYPE_11, "clang::DependentAddressSpaceTypeLoc::getInnerType"),
      add_named_node(&mut g, METHOD_GETPOINTEETYPELOC, "clang::DependentAddressSpaceTypeLoc::getPointeeTypeLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTADDRESSSPACETYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_GNUINLINEATTR, "clang::GNUInlineAttr");
  g.add_edge((CLASS_GNUINLINEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_GNUINLINEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_BLOCKPOINTERTYPELOC, "clang::BlockPointerTypeLoc");
  g.add_edge((CLASS_BLOCKPOINTERTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETCARETLOC, "clang::BlockPointerTypeLoc::getCaretLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BLOCKPOINTERTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCINTERFACETYPE, "clang::ObjCInterfaceType");
  g.add_edge((CLASS_OBJCINTERFACETYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCINTERFACETYPE, META_SUBCLASS, CLASS_OBJCOBJECTTYPE));

  g.add_named_node(CLASS_OBJCOBJECTPOINTERTYPELOC, "clang::ObjCObjectPointerTypeLoc");
  g.add_edge((CLASS_OBJCOBJECTPOINTERTYPELOC, META_CLASS, META_CLANG_AST_NODE));

  g.add_named_node(CLASS_OBJCBOXABLEATTR, "clang::ObjCBoxableAttr");
  g.add_edge((CLASS_OBJCBOXABLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCBOXABLEATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_FUNCTIONPROTOTYPELOC, "clang::FunctionProtoTypeLoc");
  g.add_edge((CLASS_FUNCTIONPROTOTYPELOC, META_CLASS, META_CLANG_AST_NODE));

  g.add_named_node(CLASS_X86FORCEALIGNARGPOINTERATTR, "clang::X86ForceAlignArgPointerAttr");
  g.add_edge((CLASS_X86FORCEALIGNARGPOINTERATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_X86FORCEALIGNARGPOINTERATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_UNLIKELYATTR, "clang::UnlikelyAttr");
  g.add_edge((CLASS_UNLIKELYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNLIKELYATTR, META_SUBCLASS, CLASS_STMTATTR));

  g.add_named_node(CLASS_UNRESOLVEDUSINGTYPELOC, "clang::UnresolvedUsingTypeLoc");
  g.add_edge((CLASS_UNRESOLVEDUSINGTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETDECL_11, "clang::UnresolvedUsingTypeLoc::getDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNRESOLVEDUSINGTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCNONRUNTIMEPROTOCOLATTR, "clang::ObjCNonRuntimeProtocolAttr");
  g.add_edge((CLASS_OBJCNONRUNTIMEPROTOCOLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCNONRUNTIMEPROTOCOLATTR, META_SUBCLASS, CLASS_ATTR));

  g.add_named_node(CLASS_TEMPLATESPECIALIZATIONTYPELOC, "clang::TemplateSpecializationTypeLoc");
  g.add_edge((CLASS_TEMPLATESPECIALIZATIONTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTEMPLATEKEYWORDLOC_7, "clang::TemplateSpecializationTypeLoc::getTemplateKeywordLoc"),
      add_named_node(&mut g, METHOD_GETLANGLELOC_5, "clang::TemplateSpecializationTypeLoc::getLAngleLoc"),
      add_named_node(&mut g, METHOD_GETRANGLELOC_5, "clang::TemplateSpecializationTypeLoc::getRAngleLoc"),
      add_named_node(&mut g, METHOD_GETNUMARGS_5, "clang::TemplateSpecializationTypeLoc::getNumArgs"),
      add_named_node(&mut g, METHOD_GETTEMPLATENAMELOC, "clang::TemplateSpecializationTypeLoc::getTemplateNameLoc"),
      add_named_node(&mut g, METHOD_GETLOCALSOURCERANGE_15, "clang::TemplateSpecializationTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, METHOD_GETEXTRALOCALDATASIZE, "clang::TemplateSpecializationTypeLoc::getExtraLocalDataSize"),
      add_named_node(&mut g, METHOD_GETEXTRALOCALDATAALIGNMENT, "clang::TemplateSpecializationTypeLoc::getExtraLocalDataAlignment"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TEMPLATESPECIALIZATIONTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FUNCTIONPROTOTYPE, "clang::FunctionProtoType");
  g.add_edge((CLASS_FUNCTIONPROTOTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FUNCTIONPROTOTYPE, META_SUBCLASS, CLASS_FUNCTIONTYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETNUMPARAMS, "clang::FunctionProtoType::getNumParams"),
      add_named_node(&mut g, METHOD_GETPARAMTYPES, "clang::FunctionProtoType::getParamTypes"),
      add_named_node(&mut g, METHOD_GETEXTPROTOINFO, "clang::FunctionProtoType::getExtProtoInfo"),
      add_named_node(&mut g, METHOD_GETEXCEPTIONSPECTYPE, "clang::FunctionProtoType::getExceptionSpecType"),
      add_named_node(&mut g, METHOD_HASEXCEPTIONSPEC, "clang::FunctionProtoType::hasExceptionSpec"),
      add_named_node(&mut g, METHOD_HASDYNAMICEXCEPTIONSPEC, "clang::FunctionProtoType::hasDynamicExceptionSpec"),
      add_named_node(&mut g, METHOD_HASNOEXCEPTEXCEPTIONSPEC, "clang::FunctionProtoType::hasNoexceptExceptionSpec"),
      add_named_node(&mut g, METHOD_HASDEPENDENTEXCEPTIONSPEC, "clang::FunctionProtoType::hasDependentExceptionSpec"),
      add_named_node(&mut g, METHOD_HASINSTANTIATIONDEPENDENTEXCEPTIONSPEC, "clang::FunctionProtoType::hasInstantiationDependentExceptionSpec"),
      add_named_node(&mut g, METHOD_GETEXCEPTIONSPECINFO, "clang::FunctionProtoType::getExceptionSpecInfo"),
      add_named_node(&mut g, METHOD_GETNUMEXCEPTIONS, "clang::FunctionProtoType::getNumExceptions"),
      add_named_node(&mut g, METHOD_GETNOEXCEPTEXPR, "clang::FunctionProtoType::getNoexceptExpr"),
      add_named_node(&mut g, METHOD_GETEXCEPTIONSPECDECL, "clang::FunctionProtoType::getExceptionSpecDecl"),
      add_named_node(&mut g, METHOD_GETEXCEPTIONSPECTEMPLATE, "clang::FunctionProtoType::getExceptionSpecTemplate"),
      add_named_node(&mut g, METHOD_CANTHROW, "clang::FunctionProtoType::canThrow"),
      add_named_node(&mut g, METHOD_ISVARIADIC, "clang::FunctionProtoType::isVariadic"),
      add_named_node(&mut g, METHOD_GETELLIPSISLOC, "clang::FunctionProtoType::getEllipsisLoc"),
      add_named_node(&mut g, METHOD_ISTEMPLATEVARIADIC, "clang::FunctionProtoType::isTemplateVariadic"),
      add_named_node(&mut g, METHOD_HASTRAILINGRETURN, "clang::FunctionProtoType::hasTrailingReturn"),
      add_named_node(&mut g, METHOD_GETMETHODQUALS, "clang::FunctionProtoType::getMethodQuals"),
      add_named_node(&mut g, METHOD_GETREFQUALIFIER, "clang::FunctionProtoType::getRefQualifier"),
      add_named_node(&mut g, METHOD_PARAM_TYPES, "clang::FunctionProtoType::param_types"),
      add_named_node(&mut g, METHOD_PARAM_TYPE_BEGIN, "clang::FunctionProtoType::param_type_begin"),
      add_named_node(&mut g, METHOD_PARAM_TYPE_END, "clang::FunctionProtoType::param_type_end"),
      add_named_node(&mut g, METHOD_EXCEPTIONS, "clang::FunctionProtoType::exceptions"),
      add_named_node(&mut g, METHOD_EXCEPTION_BEGIN, "clang::FunctionProtoType::exception_begin"),
      add_named_node(&mut g, METHOD_EXCEPTION_END, "clang::FunctionProtoType::exception_end"),
      add_named_node(&mut g, METHOD_HASEXTPARAMETERINFOS, "clang::FunctionProtoType::hasExtParameterInfos"),
      add_named_node(&mut g, METHOD_GETEXTPARAMETERINFOS, "clang::FunctionProtoType::getExtParameterInfos"),
      add_named_node(&mut g, METHOD_GETEXTPARAMETERINFOSORNULL, "clang::FunctionProtoType::getExtParameterInfosOrNull"),
      add_named_node(&mut g, METHOD_GETAARCH64SMEATTRIBUTES, "clang::FunctionProtoType::getAArch64SMEAttributes"),
      add_named_node(&mut g, METHOD_ISSUGARED_22, "clang::FunctionProtoType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_22, "clang::FunctionProtoType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FUNCTIONPROTOTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_QUALIFIEDTYPELOC, "clang::QualifiedTypeLoc");
  g.add_edge((CLASS_QUALIFIEDTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_QUALIFIEDTYPELOC, META_SUBCLASS, CLASS_TYPELOC));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETLOCALSOURCERANGE_16, "clang::QualifiedTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, METHOD_GETUNQUALIFIEDLOC_1, "clang::QualifiedTypeLoc::getUnqualifiedLoc"),
      add_named_node(&mut g, METHOD_GETNEXTTYPELOC_1, "clang::QualifiedTypeLoc::getNextTypeLoc"),
      add_named_node(&mut g, METHOD_GETLOCALDATASIZE_1, "clang::QualifiedTypeLoc::getLocalDataSize"),
      add_named_node(&mut g, METHOD_GETLOCALDATAALIGNMENT, "clang::QualifiedTypeLoc::getLocalDataAlignment"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_QUALIFIEDTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BTFTAGATTRIBUTEDTYPELOC, "clang::BTFTagAttributedTypeLoc");
  g.add_edge((CLASS_BTFTAGATTRIBUTEDTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETWRAPPEDLOC, "clang::BTFTagAttributedTypeLoc::getWrappedLoc"),
      add_named_node(&mut g, METHOD_GETATTR_2, "clang::BTFTagAttributedTypeLoc::getAttr"),
      add_named_node(&mut g, METHOD_GETLOCALSOURCERANGE_17, "clang::BTFTagAttributedTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, METHOD_GETINNERTYPE_12, "clang::BTFTagAttributedTypeLoc::getInnerType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BTFTAGATTRIBUTEDTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_WEBASSEMBLYIMPORTNAMEATTR, "clang::WebAssemblyImportNameAttr");
  g.add_edge((CLASS_WEBASSEMBLYIMPORTNAMEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_WEBASSEMBLYIMPORTNAMEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_FUNCTIONTYPELOC, "clang::FunctionTypeLoc");
  g.add_edge((CLASS_FUNCTIONTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETLOCALRANGEBEGIN, "clang::FunctionTypeLoc::getLocalRangeBegin"),
      add_named_node(&mut g, METHOD_GETLOCALRANGEEND, "clang::FunctionTypeLoc::getLocalRangeEnd"),
      add_named_node(&mut g, METHOD_GETLPARENLOC_14, "clang::FunctionTypeLoc::getLParenLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_32, "clang::FunctionTypeLoc::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETPARENSRANGE_1, "clang::FunctionTypeLoc::getParensRange"),
      add_named_node(&mut g, METHOD_GETEXCEPTIONSPECRANGE, "clang::FunctionTypeLoc::getExceptionSpecRange"),
      add_named_node(&mut g, METHOD_GETPARAMS, "clang::FunctionTypeLoc::getParams"),
      add_named_node(&mut g, METHOD_GETPARMARRAY, "clang::FunctionTypeLoc::getParmArray"),
      add_named_node(&mut g, METHOD_GETNUMPARAMS_4, "clang::FunctionTypeLoc::getNumParams"),
      add_named_node(&mut g, METHOD_GETRETURNLOC_1, "clang::FunctionTypeLoc::getReturnLoc"),
      add_named_node(&mut g, METHOD_GETLOCALSOURCERANGE_18, "clang::FunctionTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, METHOD_GETEXTRALOCALDATASIZE_1, "clang::FunctionTypeLoc::getExtraLocalDataSize"),
      add_named_node(&mut g, METHOD_GETEXTRALOCALDATAALIGNMENT_1, "clang::FunctionTypeLoc::getExtraLocalDataAlignment"),
      add_named_node(&mut g, METHOD_GETINNERTYPE_13, "clang::FunctionTypeLoc::getInnerType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FUNCTIONTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPCANCELDIRECTIVE, "clang::OMPCancelDirective");
  g.add_edge((CLASS_OMPCANCELDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPCANCELDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_CONSTANTARRAYTYPE, "clang::ConstantArrayType");
  g.add_edge((CLASS_CONSTANTARRAYTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONSTANTARRAYTYPE, META_SUBCLASS, CLASS_ARRAYTYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSIZE, "clang::ConstantArrayType::getSize"),
      add_named_node(&mut g, METHOD_GETSIZEEXPR, "clang::ConstantArrayType::getSizeExpr"),
      add_named_node(&mut g, METHOD_ISSUGARED_8, "clang::ConstantArrayType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_8, "clang::ConstantArrayType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CONSTANTARRAYTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTSIZEDARRAYTYPELOC, "clang::DependentSizedArrayTypeLoc");
  g.add_edge((CLASS_DEPENDENTSIZEDARRAYTYPELOC, META_CLASS, META_CLANG_AST_NODE));

  g.add_named_node(CLASS_OBJCKINDOFATTR, "clang::ObjCKindOfAttr");
  g.add_edge((CLASS_OBJCKINDOFATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCKINDOFATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_DEPENDENTTEMPLATESPECIALIZATIONTYPELOC, "clang::DependentTemplateSpecializationTypeLoc");
  g.add_edge((CLASS_DEPENDENTTEMPLATESPECIALIZATIONTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETELABORATEDKEYWORDLOC_1, "clang::DependentTemplateSpecializationTypeLoc::getElaboratedKeywordLoc"),
      add_named_node(&mut g, METHOD_GETQUALIFIERLOC_17, "clang::DependentTemplateSpecializationTypeLoc::getQualifierLoc"),
      add_named_node(&mut g, METHOD_GETTEMPLATEKEYWORDLOC_8, "clang::DependentTemplateSpecializationTypeLoc::getTemplateKeywordLoc"),
      add_named_node(&mut g, METHOD_GETTEMPLATENAMELOC_1, "clang::DependentTemplateSpecializationTypeLoc::getTemplateNameLoc"),
      add_named_node(&mut g, METHOD_GETLANGLELOC_6, "clang::DependentTemplateSpecializationTypeLoc::getLAngleLoc"),
      add_named_node(&mut g, METHOD_GETRANGLELOC_6, "clang::DependentTemplateSpecializationTypeLoc::getRAngleLoc"),
      add_named_node(&mut g, METHOD_GETNUMARGS_6, "clang::DependentTemplateSpecializationTypeLoc::getNumArgs"),
      add_named_node(&mut g, METHOD_GETLOCALSOURCERANGE_19, "clang::DependentTemplateSpecializationTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, METHOD_GETEXTRALOCALDATASIZE_2, "clang::DependentTemplateSpecializationTypeLoc::getExtraLocalDataSize"),
      add_named_node(&mut g, METHOD_GETEXTRALOCALDATAALIGNMENT_2, "clang::DependentTemplateSpecializationTypeLoc::getExtraLocalDataAlignment"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTTEMPLATESPECIALIZATIONTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_VALUESTMT, "clang::ValueStmt");
  g.add_edge((CLASS_VALUESTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_VALUESTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETEXPRSTMT, "clang::ValueStmt::getExprStmt"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_VALUESTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPDISPATCHDIRECTIVE, "clang::OMPDispatchDirective");
  g.add_edge((CLASS_OMPDISPATCHDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPDISPATCHDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));

  g.add_named_node(CLASS_HLSLSV_DISPATCHTHREADIDATTR, "clang::HLSLSV_DispatchThreadIDAttr");
  g.add_edge((CLASS_HLSLSV_DISPATCHTHREADIDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_HLSLSV_DISPATCHTHREADIDATTR, META_SUBCLASS, CLASS_HLSLANNOTATIONATTR));

  g.add_named_node(CLASS_VARIABLEARRAYTYPELOC, "clang::VariableArrayTypeLoc");
  g.add_edge((CLASS_VARIABLEARRAYTYPELOC, META_CLASS, META_CLANG_AST_NODE));

  g.add_named_node(CLASS_DEPENDENTSCOPEDECLREFEXPR, "clang::DependentScopeDeclRefExpr");
  g.add_edge((CLASS_DEPENDENTSCOPEDECLREFEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEPENDENTSCOPEDECLREFEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETNAMEINFO_5, "clang::DependentScopeDeclRefExpr::getNameInfo"),
      add_named_node(&mut g, METHOD_GETDECLNAME_1, "clang::DependentScopeDeclRefExpr::getDeclName"),
      add_named_node(&mut g, METHOD_GETLOCATION_8, "clang::DependentScopeDeclRefExpr::getLocation"),
      add_named_node(&mut g, METHOD_GETQUALIFIERLOC_11, "clang::DependentScopeDeclRefExpr::getQualifierLoc"),
      add_named_node(&mut g, METHOD_GETQUALIFIER_14, "clang::DependentScopeDeclRefExpr::getQualifier"),
      add_named_node(&mut g, METHOD_GETTEMPLATEKEYWORDLOC_4, "clang::DependentScopeDeclRefExpr::getTemplateKeywordLoc"),
      add_named_node(&mut g, METHOD_GETLANGLELOC_2, "clang::DependentScopeDeclRefExpr::getLAngleLoc"),
      add_named_node(&mut g, METHOD_GETRANGLELOC_2, "clang::DependentScopeDeclRefExpr::getRAngleLoc"),
      add_named_node(&mut g, METHOD_HASTEMPLATEKEYWORD_2, "clang::DependentScopeDeclRefExpr::hasTemplateKeyword"),
      add_named_node(&mut g, METHOD_HASEXPLICITTEMPLATEARGS_3, "clang::DependentScopeDeclRefExpr::hasExplicitTemplateArgs"),
      add_named_node(&mut g, METHOD_GETTEMPLATEARGS_4, "clang::DependentScopeDeclRefExpr::getTemplateArgs"),
      add_named_node(&mut g, METHOD_GETNUMTEMPLATEARGS_2, "clang::DependentScopeDeclRefExpr::getNumTemplateArgs"),
      add_named_node(&mut g, METHOD_TEMPLATE_ARGUMENTS_4, "clang::DependentScopeDeclRefExpr::template_arguments"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_67, "clang::DependentScopeDeclRefExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_66, "clang::DependentScopeDeclRefExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_57, "clang::DependentScopeDeclRefExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTSCOPEDECLREFEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MATRIXTYPELOC, "clang::MatrixTypeLoc");
  g.add_edge((CLASS_MATRIXTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETATTRNAMELOC_1, "clang::MatrixTypeLoc::getAttrNameLoc"),
      add_named_node(&mut g, METHOD_GETATTRROWOPERAND, "clang::MatrixTypeLoc::getAttrRowOperand"),
      add_named_node(&mut g, METHOD_GETATTRCOLUMNOPERAND, "clang::MatrixTypeLoc::getAttrColumnOperand"),
      add_named_node(&mut g, METHOD_GETATTROPERANDPARENSRANGE_1, "clang::MatrixTypeLoc::getAttrOperandParensRange"),
      add_named_node(&mut g, METHOD_GETLOCALSOURCERANGE_20, "clang::MatrixTypeLoc::getLocalSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MATRIXTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TLSMODELATTR, "clang::TLSModelAttr");
  g.add_edge((CLASS_TLSMODELATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TLSMODELATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_FUNCTIONNOPROTOTYPELOC, "clang::FunctionNoProtoTypeLoc");
  g.add_edge((CLASS_FUNCTIONNOPROTOTYPELOC, META_CLASS, META_CLANG_AST_NODE));

  g.add_named_node(CLASS_MSDEPENDENTEXISTSSTMT, "clang::MSDependentExistsStmt");
  g.add_edge((CLASS_MSDEPENDENTEXISTSSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MSDEPENDENTEXISTSSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETKEYWORDLOC_3, "clang::MSDependentExistsStmt::getKeywordLoc"),
      add_named_node(&mut g, METHOD_ISIFEXISTS, "clang::MSDependentExistsStmt::isIfExists"),
      add_named_node(&mut g, METHOD_ISIFNOTEXISTS, "clang::MSDependentExistsStmt::isIfNotExists"),
      add_named_node(&mut g, METHOD_GETQUALIFIERLOC_12, "clang::MSDependentExistsStmt::getQualifierLoc"),
      add_named_node(&mut g, METHOD_GETNAMEINFO_6, "clang::MSDependentExistsStmt::getNameInfo"),
      add_named_node(&mut g, METHOD_GETSUBSTMT_4, "clang::MSDependentExistsStmt::getSubStmt"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_92, "clang::MSDependentExistsStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_91, "clang::MSDependentExistsStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_80, "clang::MSDependentExistsStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MSDEPENDENTEXISTSSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PATCHABLEFUNCTIONENTRYATTR, "clang::PatchableFunctionEntryAttr");
  g.add_edge((CLASS_PATCHABLEFUNCTIONENTRYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PATCHABLEFUNCTIONENTRYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_UNARYTRANSFORMTYPELOC, "clang::UnaryTransformTypeLoc");
  g.add_edge((CLASS_UNARYTRANSFORMTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETKWLOC_2, "clang::UnaryTransformTypeLoc::getKWLoc"),
      add_named_node(&mut g, METHOD_GETLPARENLOC_15, "clang::UnaryTransformTypeLoc::getLParenLoc"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_33, "clang::UnaryTransformTypeLoc::getRParenLoc"),
      add_named_node(&mut g, METHOD_GETUNDERLYINGTINFO, "clang::UnaryTransformTypeLoc::getUnderlyingTInfo"),
      add_named_node(&mut g, METHOD_GETLOCALSOURCERANGE_21, "clang::UnaryTransformTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, METHOD_GETPARENSRANGE_2, "clang::UnaryTransformTypeLoc::getParensRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNARYTRANSFORMTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_COROUTINESUSPENDEXPR, "clang::CoroutineSuspendExpr");
  g.add_edge((CLASS_COROUTINESUSPENDEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_COROUTINESUSPENDEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETCOMMONEXPR_1, "clang::CoroutineSuspendExpr::getCommonExpr"),
      add_named_node(&mut g, METHOD_GETOPAQUEVALUE_1, "clang::CoroutineSuspendExpr::getOpaqueValue"),
      add_named_node(&mut g, METHOD_GETREADYEXPR, "clang::CoroutineSuspendExpr::getReadyExpr"),
      add_named_node(&mut g, METHOD_GETSUSPENDEXPR, "clang::CoroutineSuspendExpr::getSuspendExpr"),
      add_named_node(&mut g, METHOD_GETRESUMEEXPR, "clang::CoroutineSuspendExpr::getResumeExpr"),
      add_named_node(&mut g, METHOD_GETOPERAND_2, "clang::CoroutineSuspendExpr::getOperand"),
      add_named_node(&mut g, METHOD_GETKEYWORDLOC_1, "clang::CoroutineSuspendExpr::getKeywordLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_62, "clang::CoroutineSuspendExpr::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_61, "clang::CoroutineSuspendExpr::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_52, "clang::CoroutineSuspendExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_COROUTINESUSPENDEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPENULLUNSPECIFIEDATTR, "clang::TypeNullUnspecifiedAttr");
  g.add_edge((CLASS_TYPENULLUNSPECIFIEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPENULLUNSPECIFIEDATTR, META_SUBCLASS, CLASS_TYPEATTR));

  g.add_named_node(CLASS_UNQUALTYPELOC, "clang::UnqualTypeLoc");
  g.add_edge((CLASS_UNQUALTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNQUALTYPELOC, META_SUBCLASS, CLASS_TYPELOC));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTYPEPTR_1, "clang::UnqualTypeLoc::getTypePtr"),
      add_named_node(&mut g, METHOD_GETTYPELOCCLASS_1, "clang::UnqualTypeLoc::getTypeLocClass"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNQUALTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CUDAINVALIDTARGETATTR, "clang::CUDAInvalidTargetAttr");
  g.add_edge((CLASS_CUDAINVALIDTARGETATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CUDAINVALIDTARGETATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_TYPEOFTYPELOC, "clang::TypeOfTypeLoc");
  g.add_edge((CLASS_TYPEOFTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETUNMODIFIEDTYPE_1, "clang::TypeOfTypeLoc::getUnmodifiedType"),
      add_named_node(&mut g, METHOD_GETUNMODIFIEDTINFO, "clang::TypeOfTypeLoc::getUnmodifiedTInfo"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPEOFTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEDUCEDTYPELOC, "clang::DeducedTypeLoc");
  g.add_edge((CLASS_DEDUCEDTYPELOC, META_CLASS, META_CLANG_AST_NODE));

  g.add_named_node(CLASS_NORETURNATTR, "clang::NoReturnAttr");
  g.add_edge((CLASS_NORETURNATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NORETURNATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_INCOMPLETEARRAYTYPELOC, "clang::IncompleteArrayTypeLoc");
  g.add_edge((CLASS_INCOMPLETEARRAYTYPELOC, META_CLASS, META_CLANG_AST_NODE));

  g.add_named_node(CLASS_UNRESOLVEDUSINGTYPE, "clang::UnresolvedUsingType");
  g.add_edge((CLASS_UNRESOLVEDUSINGTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNRESOLVEDUSINGTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETDECL_6, "clang::UnresolvedUsingType::getDecl"),
      add_named_node(&mut g, METHOD_ISSUGARED_43, "clang::UnresolvedUsingType::isSugared"),
      add_named_node(&mut g, METHOD_DESUGAR_43, "clang::UnresolvedUsingType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNRESOLVEDUSINGTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ELABORATEDTYPELOC, "clang::ElaboratedTypeLoc");
  g.add_edge((CLASS_ELABORATEDTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETELABORATEDKEYWORDLOC_2, "clang::ElaboratedTypeLoc::getElaboratedKeywordLoc"),
      add_named_node(&mut g, METHOD_GETQUALIFIERLOC_18, "clang::ElaboratedTypeLoc::getQualifierLoc"),
      add_named_node(&mut g, METHOD_GETLOCALSOURCERANGE_22, "clang::ElaboratedTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, METHOD_GETNAMEDTYPELOC, "clang::ElaboratedTypeLoc::getNamedTypeLoc"),
      add_named_node(&mut g, METHOD_GETINNERTYPE_14, "clang::ElaboratedTypeLoc::getInnerType"),
      add_named_node(&mut g, METHOD_ISEMPTY_1, "clang::ElaboratedTypeLoc::isEmpty"),
      add_named_node(&mut g, METHOD_GETLOCALDATAALIGNMENT_1, "clang::ElaboratedTypeLoc::getLocalDataAlignment"),
      add_named_node(&mut g, METHOD_GETLOCALDATASIZE_2, "clang::ElaboratedTypeLoc::getLocalDataSize"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ELABORATEDTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_REFERENCETYPELOC, "clang::ReferenceTypeLoc");
  g.add_edge((CLASS_REFERENCETYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETINNERTYPE_15, "clang::ReferenceTypeLoc::getInnerType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_REFERENCETYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CONSTANTARRAYTYPELOC, "clang::ConstantArrayTypeLoc");
  g.add_edge((CLASS_CONSTANTARRAYTYPELOC, META_CLASS, META_CLANG_AST_NODE));

  g.add_named_node(CLASS_OMPDISTRIBUTESIMDDIRECTIVE, "clang::OMPDistributeSimdDirective");
  g.add_edge((CLASS_OMPDISTRIBUTESIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPDISTRIBUTESIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));

  g.add_named_node(CLASS_RECORDTYPELOC, "clang::RecordTypeLoc");
  g.add_edge((CLASS_RECORDTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETDECL_12, "clang::RecordTypeLoc::getDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_RECORDTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FUNCTIONRETURNTHUNKSATTR, "clang::FunctionReturnThunksAttr");
  g.add_edge((CLASS_FUNCTIONRETURNTHUNKSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FUNCTIONRETURNTHUNKSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_RVALUEREFERENCETYPELOC, "clang::RValueReferenceTypeLoc");
  g.add_edge((CLASS_RVALUEREFERENCETYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETAMPAMPLOC_1, "clang::RValueReferenceTypeLoc::getAmpAmpLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_RVALUEREFERENCETYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ASSERTSHAREDLOCKATTR, "clang::AssertSharedLockAttr");
  g.add_edge((CLASS_ASSERTSHAREDLOCKATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ASSERTSHAREDLOCKATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_AUTOTYPELOC, "clang::AutoTypeLoc");
  g.add_edge((CLASS_AUTOTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETAUTOKEYWORD, "clang::AutoTypeLoc::getAutoKeyword"),
      add_named_node(&mut g, METHOD_ISDECLTYPEAUTO_1, "clang::AutoTypeLoc::isDecltypeAuto"),
      add_named_node(&mut g, METHOD_GETRPARENLOC_34, "clang::AutoTypeLoc::getRParenLoc"),
      add_named_node(&mut g, METHOD_ISCONSTRAINED_1, "clang::AutoTypeLoc::isConstrained"),
      add_named_node(&mut g, METHOD_GETCONCEPTREFERENCE_1, "clang::AutoTypeLoc::getConceptReference"),
      add_named_node(&mut g, METHOD_GETNESTEDNAMESPECIFIERLOC_1, "clang::AutoTypeLoc::getNestedNameSpecifierLoc"),
      add_named_node(&mut g, METHOD_GETTEMPLATEKWLOC_1, "clang::AutoTypeLoc::getTemplateKWLoc"),
      add_named_node(&mut g, METHOD_GETCONCEPTNAMELOC_1, "clang::AutoTypeLoc::getConceptNameLoc"),
      add_named_node(&mut g, METHOD_GETFOUNDDECL_5, "clang::AutoTypeLoc::getFoundDecl"),
      add_named_node(&mut g, METHOD_GETNAMEDCONCEPT_1, "clang::AutoTypeLoc::getNamedConcept"),
      add_named_node(&mut g, METHOD_GETCONCEPTNAMEINFO_1, "clang::AutoTypeLoc::getConceptNameInfo"),
      add_named_node(&mut g, METHOD_HASEXPLICITTEMPLATEARGS_6, "clang::AutoTypeLoc::hasExplicitTemplateArgs"),
      add_named_node(&mut g, METHOD_GETLANGLELOC_7, "clang::AutoTypeLoc::getLAngleLoc"),
      add_named_node(&mut g, METHOD_GETRANGLELOC_7, "clang::AutoTypeLoc::getRAngleLoc"),
      add_named_node(&mut g, METHOD_GETNUMARGS_7, "clang::AutoTypeLoc::getNumArgs"),
      add_named_node(&mut g, METHOD_GETLOCALSOURCERANGE_23, "clang::AutoTypeLoc::getLocalSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_AUTOTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCFORCOLLECTIONSTMT, "clang::ObjCForCollectionStmt");
  g.add_edge((CLASS_OBJCFORCOLLECTIONSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCFORCOLLECTIONSTMT, META_SUBCLASS, CLASS_STMT));

  g.add_named_node(CLASS_ERRORATTR, "clang::ErrorAttr");
  g.add_edge((CLASS_ERRORATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ERRORATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_TAGTYPELOC, "clang::TagTypeLoc");
  g.add_edge((CLASS_TAGTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETDECL_13, "clang::TagTypeLoc::getDecl"),
      add_named_node(&mut g, METHOD_ISDEFINITION, "clang::TagTypeLoc::isDefinition"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TAGTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCOBJECTTYPE, "clang::ObjCObjectType");
  g.add_edge((CLASS_OBJCOBJECTTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCOBJECTTYPE, META_SUBCLASS, CLASS_TYPE));

  g.add_named_node(CLASS_BUILTINTYPELOC, "clang::BuiltinTypeLoc");
  g.add_edge((CLASS_BUILTINTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETBUILTINLOC_6, "clang::BuiltinTypeLoc::getBuiltinLoc"),
      add_named_node(&mut g, METHOD_GETNAMELOC_6, "clang::BuiltinTypeLoc::getNameLoc"),
      add_named_node(&mut g, METHOD_GETWRITTENBUILTINSPECS, "clang::BuiltinTypeLoc::getWrittenBuiltinSpecs"),
      add_named_node(&mut g, METHOD_NEEDSEXTRALOCALDATA, "clang::BuiltinTypeLoc::needsExtraLocalData"),
      add_named_node(&mut g, METHOD_GETEXTRALOCALDATASIZE_3, "clang::BuiltinTypeLoc::getExtraLocalDataSize"),
      add_named_node(&mut g, METHOD_GETEXTRALOCALDATAALIGNMENT_3, "clang::BuiltinTypeLoc::getExtraLocalDataAlignment"),
      add_named_node(&mut g, METHOD_GETLOCALSOURCERANGE_24, "clang::BuiltinTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, METHOD_GETWRITTENSIGNSPEC, "clang::BuiltinTypeLoc::getWrittenSignSpec"),
      add_named_node(&mut g, METHOD_HASWRITTENSIGNSPEC, "clang::BuiltinTypeLoc::hasWrittenSignSpec"),
      add_named_node(&mut g, METHOD_GETWRITTENWIDTHSPEC, "clang::BuiltinTypeLoc::getWrittenWidthSpec"),
      add_named_node(&mut g, METHOD_HASWRITTENWIDTHSPEC, "clang::BuiltinTypeLoc::hasWrittenWidthSpec"),
      add_named_node(&mut g, METHOD_GETWRITTENTYPESPEC, "clang::BuiltinTypeLoc::getWrittenTypeSpec"),
      add_named_node(&mut g, METHOD_HASWRITTENTYPESPEC, "clang::BuiltinTypeLoc::hasWrittenTypeSpec"),
      add_named_node(&mut g, METHOD_HASMODEATTR, "clang::BuiltinTypeLoc::hasModeAttr"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BUILTINTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPESPECTYPELOC, "clang::TypeSpecTypeLoc");
  g.add_edge((CLASS_TYPESPECTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETNAMELOC_7, "clang::TypeSpecTypeLoc::getNameLoc"),
      add_named_node(&mut g, METHOD_GETLOCALSOURCERANGE_25, "clang::TypeSpecTypeLoc::getLocalSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPESPECTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTBITINTTYPELOC, "clang::DependentBitIntTypeLoc");
  g.add_edge((CLASS_DEPENDENTBITINTTYPELOC, META_CLASS, META_CLANG_AST_NODE));

  g.add_named_node(CLASS_DEDUCEDTEMPLATESPECIALIZATIONTYPELOC, "clang::DeducedTemplateSpecializationTypeLoc");
  g.add_edge((CLASS_DEDUCEDTEMPLATESPECIALIZATIONTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETTEMPLATENAMELOC_2, "clang::DeducedTemplateSpecializationTypeLoc::getTemplateNameLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEDUCEDTEMPLATESPECIALIZATIONTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PACKEXPANSIONTYPELOC, "clang::PackExpansionTypeLoc");
  g.add_edge((CLASS_PACKEXPANSIONTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETELLIPSISLOC_7, "clang::PackExpansionTypeLoc::getEllipsisLoc"),
      add_named_node(&mut g, METHOD_GETLOCALSOURCERANGE_26, "clang::PackExpansionTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, METHOD_GETPATTERNLOC, "clang::PackExpansionTypeLoc::getPatternLoc"),
      add_named_node(&mut g, METHOD_GETINNERTYPE_16, "clang::PackExpansionTypeLoc::getInnerType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PACKEXPANSIONTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TEMPLATETYPEPARMTYPELOC, "clang::TemplateTypeParmTypeLoc");
  g.add_edge((CLASS_TEMPLATETYPEPARMTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETDECL_14, "clang::TemplateTypeParmTypeLoc::getDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TEMPLATETYPEPARMTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_RETURNSTMT, "clang::ReturnStmt");
  g.add_edge((CLASS_RETURNSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RETURNSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETRETVALUE, "clang::ReturnStmt::getRetValue"),
      add_named_node(&mut g, METHOD_GETNRVOCANDIDATE, "clang::ReturnStmt::getNRVOCandidate"),
      add_named_node(&mut g, METHOD_GETRETURNLOC, "clang::ReturnStmt::getReturnLoc"),
      add_named_node(&mut g, METHOD_GETBEGINLOC_109, "clang::ReturnStmt::getBeginLoc"),
      add_named_node(&mut g, METHOD_GETENDLOC_108, "clang::ReturnStmt::getEndLoc"),
      add_named_node(&mut g, METHOD_CHILDREN_96, "clang::ReturnStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_RETURNSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MEMBERPOINTERTYPELOC, "clang::MemberPointerTypeLoc");
  g.add_edge((CLASS_MEMBERPOINTERTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETSTARLOC_2, "clang::MemberPointerTypeLoc::getStarLoc"),
      add_named_node(&mut g, METHOD_GETCLASS_1, "clang::MemberPointerTypeLoc::getClass"),
      add_named_node(&mut g, METHOD_GETCLASSTINFO, "clang::MemberPointerTypeLoc::getClassTInfo"),
      add_named_node(&mut g, METHOD_GETLOCALSOURCERANGE_27, "clang::MemberPointerTypeLoc::getLocalSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MEMBERPOINTERTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SUBSTTEMPLATETYPEPARMTYPELOC, "clang::SubstTemplateTypeParmTypeLoc");
  g.add_edge((CLASS_SUBSTTEMPLATETYPEPARMTYPELOC, META_CLASS, META_CLANG_AST_NODE));

  g.add_named_node(CLASS_WEBASSEMBLYEXPORTNAMEATTR, "clang::WebAssemblyExportNameAttr");
  g.add_edge((CLASS_WEBASSEMBLYEXPORTNAMEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_WEBASSEMBLYEXPORTNAMEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_CUDAGLOBALATTR, "clang::CUDAGlobalAttr");
  g.add_edge((CLASS_CUDAGLOBALATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CUDAGLOBALATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_PRAGMACOMMENTDECL, "clang::PragmaCommentDecl");
  g.add_edge((CLASS_PRAGMACOMMENTDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PRAGMACOMMENTDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETCOMMENTKIND, "clang::PragmaCommentDecl::getCommentKind"),
      add_named_node(&mut g, METHOD_GETARG, "clang::PragmaCommentDecl::getArg"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PRAGMACOMMENTDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_EXTERNCCONTEXTDECL, "clang::ExternCContextDecl");
  g.add_edge((CLASS_EXTERNCCONTEXTDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_EXTERNCCONTEXTDECL, META_SUBCLASS, CLASS_DECL));

  g.add_named_node(CLASS_OBJCPROPERTYREFEXPR, "clang::ObjCPropertyRefExpr");
  g.add_edge((CLASS_OBJCPROPERTYREFEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCPROPERTYREFEXPR, META_SUBCLASS, CLASS_EXPR));

  g.add_named_node(CLASS_OBJCBRIDGEATTR, "clang::ObjCBridgeAttr");
  g.add_edge((CLASS_OBJCBRIDGEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCBRIDGEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));

  g.add_named_node(CLASS_IMPLICITPARAMDECL, "clang::ImplicitParamDecl");
  g.add_edge((CLASS_IMPLICITPARAMDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_IMPLICITPARAMDECL, META_SUBCLASS, CLASS_VARDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_GETPARAMETERKIND, "clang::ImplicitParamDecl::getParameterKind"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_IMPLICITPARAMDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DECOMPOSITIONDECL, "clang::DecompositionDecl");
  g.add_edge((CLASS_DECOMPOSITIONDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DECOMPOSITIONDECL, META_SUBCLASS, CLASS_VARDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, METHOD_BINDINGS, "clang::DecompositionDecl::bindings"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DECOMPOSITIONDECL, META_METHOD, methods));
  }

  g.add_named_node(ENUM_VALUEKIND, "clang::APValue::ValueKind");
  g.add_edge((ENUM_VALUEKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 3968, "clang::APValue::None"),
      add_named_node(&mut g, 3969, "clang::APValue::Indeterminate"),
      add_named_node(&mut g, 3970, "clang::APValue::Int"),
      add_named_node(&mut g, 3971, "clang::APValue::Float"),
      add_named_node(&mut g, 3972, "clang::APValue::FixedPoint"),
      add_named_node(&mut g, 3973, "clang::APValue::ComplexInt"),
      add_named_node(&mut g, 3974, "clang::APValue::ComplexFloat"),
      add_named_node(&mut g, 3975, "clang::APValue::LValue"),
      add_named_node(&mut g, 3976, "clang::APValue::Vector"),
      add_named_node(&mut g, 3977, "clang::APValue::Array"),
      add_named_node(&mut g, 3978, "clang::APValue::Struct"),
      add_named_node(&mut g, 3979, "clang::APValue::Union"),
      add_named_node(&mut g, 3980, "clang::APValue::MemberPointer"),
      add_named_node(&mut g, 3981, "clang::APValue::AddrLabelDiff"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_VALUEKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_ACCESSSPECIFIER, "clang::AccessSpecifier");
  g.add_edge((ENUM_ACCESSSPECIFIER, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 3983, "clang::AS_public"),
      add_named_node(&mut g, 3984, "clang::AS_protected"),
      add_named_node(&mut g, 3985, "clang::AS_private"),
      add_named_node(&mut g, 3986, "clang::AS_none"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_ACCESSSPECIFIER, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_ARRAYSIZEMODIFIER, "clang::ArraySizeModifier");
  g.add_edge((ENUM_ARRAYSIZEMODIFIER, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 3988, "clang::ArraySizeModifier::Normal"),
      add_named_node(&mut g, 3989, "clang::ArraySizeModifier::Static"),
      add_named_node(&mut g, 3990, "clang::ArraySizeModifier::Star"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_ARRAYSIZEMODIFIER, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_ARRAYTYPETRAIT, "clang::ArrayTypeTrait");
  g.add_edge((ENUM_ARRAYTYPETRAIT, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 3992, "clang::ATT_ArrayRank"),
      add_named_node(&mut g, 3993, "clang::ATT_ArrayExtent"),
      add_named_node(&mut g, 3994, "clang::ATT_Last"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_ARRAYTYPETRAIT, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_ATOMICOP, "clang::AtomicExpr::AtomicOp");
  g.add_edge((ENUM_ATOMICOP, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 3996, "clang::AtomicExpr::AO__c11_atomic_init"),
      add_named_node(&mut g, 3997, "clang::AtomicExpr::AO__c11_atomic_load"),
      add_named_node(&mut g, 3998, "clang::AtomicExpr::AO__c11_atomic_store"),
      add_named_node(&mut g, 3999, "clang::AtomicExpr::AO__c11_atomic_exchange"),
      add_named_node(&mut g, 4000, "clang::AtomicExpr::AO__c11_atomic_compare_exchange_strong"),
      add_named_node(&mut g, 4001, "clang::AtomicExpr::AO__c11_atomic_compare_exchange_weak"),
      add_named_node(&mut g, 4002, "clang::AtomicExpr::AO__c11_atomic_fetch_add"),
      add_named_node(&mut g, 4003, "clang::AtomicExpr::AO__c11_atomic_fetch_sub"),
      add_named_node(&mut g, 4004, "clang::AtomicExpr::AO__c11_atomic_fetch_and"),
      add_named_node(&mut g, 4005, "clang::AtomicExpr::AO__c11_atomic_fetch_or"),
      add_named_node(&mut g, 4006, "clang::AtomicExpr::AO__c11_atomic_fetch_xor"),
      add_named_node(&mut g, 4007, "clang::AtomicExpr::AO__c11_atomic_fetch_nand"),
      add_named_node(&mut g, 4008, "clang::AtomicExpr::AO__c11_atomic_fetch_max"),
      add_named_node(&mut g, 4009, "clang::AtomicExpr::AO__c11_atomic_fetch_min"),
      add_named_node(&mut g, 4010, "clang::AtomicExpr::AO__atomic_load"),
      add_named_node(&mut g, 4011, "clang::AtomicExpr::AO__atomic_load_n"),
      add_named_node(&mut g, 4012, "clang::AtomicExpr::AO__atomic_store"),
      add_named_node(&mut g, 4013, "clang::AtomicExpr::AO__atomic_store_n"),
      add_named_node(&mut g, 4014, "clang::AtomicExpr::AO__atomic_exchange"),
      add_named_node(&mut g, 4015, "clang::AtomicExpr::AO__atomic_exchange_n"),
      add_named_node(&mut g, 4016, "clang::AtomicExpr::AO__atomic_compare_exchange"),
      add_named_node(&mut g, 4017, "clang::AtomicExpr::AO__atomic_compare_exchange_n"),
      add_named_node(&mut g, 4018, "clang::AtomicExpr::AO__atomic_fetch_add"),
      add_named_node(&mut g, 4019, "clang::AtomicExpr::AO__atomic_fetch_sub"),
      add_named_node(&mut g, 4020, "clang::AtomicExpr::AO__atomic_fetch_and"),
      add_named_node(&mut g, 4021, "clang::AtomicExpr::AO__atomic_fetch_or"),
      add_named_node(&mut g, 4022, "clang::AtomicExpr::AO__atomic_fetch_xor"),
      add_named_node(&mut g, 4023, "clang::AtomicExpr::AO__atomic_fetch_nand"),
      add_named_node(&mut g, 4024, "clang::AtomicExpr::AO__atomic_add_fetch"),
      add_named_node(&mut g, 4025, "clang::AtomicExpr::AO__atomic_sub_fetch"),
      add_named_node(&mut g, 4026, "clang::AtomicExpr::AO__atomic_and_fetch"),
      add_named_node(&mut g, 4027, "clang::AtomicExpr::AO__atomic_or_fetch"),
      add_named_node(&mut g, 4028, "clang::AtomicExpr::AO__atomic_xor_fetch"),
      add_named_node(&mut g, 4029, "clang::AtomicExpr::AO__atomic_max_fetch"),
      add_named_node(&mut g, 4030, "clang::AtomicExpr::AO__atomic_min_fetch"),
      add_named_node(&mut g, 4031, "clang::AtomicExpr::AO__atomic_nand_fetch"),
      add_named_node(&mut g, 4032, "clang::AtomicExpr::AO__scoped_atomic_load"),
      add_named_node(&mut g, 4033, "clang::AtomicExpr::AO__scoped_atomic_load_n"),
      add_named_node(&mut g, 4034, "clang::AtomicExpr::AO__scoped_atomic_store"),
      add_named_node(&mut g, 4035, "clang::AtomicExpr::AO__scoped_atomic_store_n"),
      add_named_node(&mut g, 4036, "clang::AtomicExpr::AO__scoped_atomic_exchange"),
      add_named_node(&mut g, 4037, "clang::AtomicExpr::AO__scoped_atomic_exchange_n"),
      add_named_node(&mut g, 4038, "clang::AtomicExpr::AO__scoped_atomic_compare_exchange"),
      add_named_node(&mut g, 4039, "clang::AtomicExpr::AO__scoped_atomic_compare_exchange_n"),
      add_named_node(&mut g, 4040, "clang::AtomicExpr::AO__scoped_atomic_fetch_add"),
      add_named_node(&mut g, 4041, "clang::AtomicExpr::AO__scoped_atomic_fetch_sub"),
      add_named_node(&mut g, 4042, "clang::AtomicExpr::AO__scoped_atomic_fetch_and"),
      add_named_node(&mut g, 4043, "clang::AtomicExpr::AO__scoped_atomic_fetch_or"),
      add_named_node(&mut g, 4044, "clang::AtomicExpr::AO__scoped_atomic_fetch_xor"),
      add_named_node(&mut g, 4045, "clang::AtomicExpr::AO__scoped_atomic_fetch_nand"),
      add_named_node(&mut g, 4046, "clang::AtomicExpr::AO__scoped_atomic_add_fetch"),
      add_named_node(&mut g, 4047, "clang::AtomicExpr::AO__scoped_atomic_sub_fetch"),
      add_named_node(&mut g, 4048, "clang::AtomicExpr::AO__scoped_atomic_and_fetch"),
      add_named_node(&mut g, 4049, "clang::AtomicExpr::AO__scoped_atomic_or_fetch"),
      add_named_node(&mut g, 4050, "clang::AtomicExpr::AO__scoped_atomic_xor_fetch"),
      add_named_node(&mut g, 4051, "clang::AtomicExpr::AO__scoped_atomic_max_fetch"),
      add_named_node(&mut g, 4052, "clang::AtomicExpr::AO__scoped_atomic_min_fetch"),
      add_named_node(&mut g, 4053, "clang::AtomicExpr::AO__scoped_atomic_nand_fetch"),
      add_named_node(&mut g, 4054, "clang::AtomicExpr::AO__scoped_atomic_fetch_min"),
      add_named_node(&mut g, 4055, "clang::AtomicExpr::AO__scoped_atomic_fetch_max"),
      add_named_node(&mut g, 4056, "clang::AtomicExpr::AO__opencl_atomic_init"),
      add_named_node(&mut g, 4057, "clang::AtomicExpr::AO__opencl_atomic_load"),
      add_named_node(&mut g, 4058, "clang::AtomicExpr::AO__opencl_atomic_store"),
      add_named_node(&mut g, 4059, "clang::AtomicExpr::AO__opencl_atomic_exchange"),
      add_named_node(&mut g, 4060, "clang::AtomicExpr::AO__opencl_atomic_compare_exchange_strong"),
      add_named_node(&mut g, 4061, "clang::AtomicExpr::AO__opencl_atomic_compare_exchange_weak"),
      add_named_node(&mut g, 4062, "clang::AtomicExpr::AO__opencl_atomic_fetch_add"),
      add_named_node(&mut g, 4063, "clang::AtomicExpr::AO__opencl_atomic_fetch_sub"),
      add_named_node(&mut g, 4064, "clang::AtomicExpr::AO__opencl_atomic_fetch_and"),
      add_named_node(&mut g, 4065, "clang::AtomicExpr::AO__opencl_atomic_fetch_or"),
      add_named_node(&mut g, 4066, "clang::AtomicExpr::AO__opencl_atomic_fetch_xor"),
      add_named_node(&mut g, 4067, "clang::AtomicExpr::AO__opencl_atomic_fetch_min"),
      add_named_node(&mut g, 4068, "clang::AtomicExpr::AO__opencl_atomic_fetch_max"),
      add_named_node(&mut g, 4069, "clang::AtomicExpr::AO__atomic_fetch_min"),
      add_named_node(&mut g, 4070, "clang::AtomicExpr::AO__atomic_fetch_max"),
      add_named_node(&mut g, 4071, "clang::AtomicExpr::AO__hip_atomic_load"),
      add_named_node(&mut g, 4072, "clang::AtomicExpr::AO__hip_atomic_store"),
      add_named_node(&mut g, 4073, "clang::AtomicExpr::AO__hip_atomic_compare_exchange_weak"),
      add_named_node(&mut g, 4074, "clang::AtomicExpr::AO__hip_atomic_compare_exchange_strong"),
      add_named_node(&mut g, 4075, "clang::AtomicExpr::AO__hip_atomic_exchange"),
      add_named_node(&mut g, 4076, "clang::AtomicExpr::AO__hip_atomic_fetch_add"),
      add_named_node(&mut g, 4077, "clang::AtomicExpr::AO__hip_atomic_fetch_sub"),
      add_named_node(&mut g, 4078, "clang::AtomicExpr::AO__hip_atomic_fetch_and"),
      add_named_node(&mut g, 4079, "clang::AtomicExpr::AO__hip_atomic_fetch_or"),
      add_named_node(&mut g, 4080, "clang::AtomicExpr::AO__hip_atomic_fetch_xor"),
      add_named_node(&mut g, 4081, "clang::AtomicExpr::AO__hip_atomic_fetch_min"),
      add_named_node(&mut g, 4082, "clang::AtomicExpr::AO__hip_atomic_fetch_max"),
      add_named_node(&mut g, 4083, "clang::AtomicExpr::BI_First"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_ATOMICOP, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_AUTOTYPEKEYWORD, "clang::AutoTypeKeyword");
  g.add_edge((ENUM_AUTOTYPEKEYWORD, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4085, "clang::AutoTypeKeyword::Auto"),
      add_named_node(&mut g, 4086, "clang::AutoTypeKeyword::DecltypeAuto"),
      add_named_node(&mut g, 4087, "clang::AutoTypeKeyword::GNUAutoType"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_AUTOTYPEKEYWORD, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_BINARYOPERATORKIND, "clang::BinaryOperatorKind");
  g.add_edge((ENUM_BINARYOPERATORKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4089, "clang::BO_PtrMemD"),
      add_named_node(&mut g, 4090, "clang::BO_PtrMemI"),
      add_named_node(&mut g, 4091, "clang::BO_Mul"),
      add_named_node(&mut g, 4092, "clang::BO_Div"),
      add_named_node(&mut g, 4093, "clang::BO_Rem"),
      add_named_node(&mut g, 4094, "clang::BO_Add"),
      add_named_node(&mut g, 4095, "clang::BO_Sub"),
      add_named_node(&mut g, 4096, "clang::BO_Shl"),
      add_named_node(&mut g, 4097, "clang::BO_Shr"),
      add_named_node(&mut g, 4098, "clang::BO_Cmp"),
      add_named_node(&mut g, 4099, "clang::BO_LT"),
      add_named_node(&mut g, 4100, "clang::BO_GT"),
      add_named_node(&mut g, 4101, "clang::BO_LE"),
      add_named_node(&mut g, 4102, "clang::BO_GE"),
      add_named_node(&mut g, 4103, "clang::BO_EQ"),
      add_named_node(&mut g, 4104, "clang::BO_NE"),
      add_named_node(&mut g, 4105, "clang::BO_And"),
      add_named_node(&mut g, 4106, "clang::BO_Xor"),
      add_named_node(&mut g, 4107, "clang::BO_Or"),
      add_named_node(&mut g, 4108, "clang::BO_LAnd"),
      add_named_node(&mut g, 4109, "clang::BO_LOr"),
      add_named_node(&mut g, 4110, "clang::BO_Assign"),
      add_named_node(&mut g, 4111, "clang::BO_MulAssign"),
      add_named_node(&mut g, 4112, "clang::BO_DivAssign"),
      add_named_node(&mut g, 4113, "clang::BO_RemAssign"),
      add_named_node(&mut g, 4114, "clang::BO_AddAssign"),
      add_named_node(&mut g, 4115, "clang::BO_SubAssign"),
      add_named_node(&mut g, 4116, "clang::BO_ShlAssign"),
      add_named_node(&mut g, 4117, "clang::BO_ShrAssign"),
      add_named_node(&mut g, 4118, "clang::BO_AndAssign"),
      add_named_node(&mut g, 4119, "clang::BO_XorAssign"),
      add_named_node(&mut g, 4120, "clang::BO_OrAssign"),
      add_named_node(&mut g, 4121, "clang::BO_Comma"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_BINARYOPERATORKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_BUILTINTEMPLATEKIND, "clang::BuiltinTemplateKind");
  g.add_edge((ENUM_BUILTINTEMPLATEKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4123, "clang::BTK__make_integer_seq"),
      add_named_node(&mut g, 4124, "clang::BTK__type_pack_element"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_BUILTINTEMPLATEKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_KIND, "clang::BuiltinType::Kind");
  g.add_edge((ENUM_KIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4126, "clang::BuiltinType::OCLImage1dRO"),
      add_named_node(&mut g, 4127, "clang::BuiltinType::OCLImage1dArrayRO"),
      add_named_node(&mut g, 4128, "clang::BuiltinType::OCLImage1dBufferRO"),
      add_named_node(&mut g, 4129, "clang::BuiltinType::OCLImage2dRO"),
      add_named_node(&mut g, 4130, "clang::BuiltinType::OCLImage2dArrayRO"),
      add_named_node(&mut g, 4131, "clang::BuiltinType::OCLImage2dDepthRO"),
      add_named_node(&mut g, 4132, "clang::BuiltinType::OCLImage2dArrayDepthRO"),
      add_named_node(&mut g, 4133, "clang::BuiltinType::OCLImage2dMSAARO"),
      add_named_node(&mut g, 4134, "clang::BuiltinType::OCLImage2dArrayMSAARO"),
      add_named_node(&mut g, 4135, "clang::BuiltinType::OCLImage2dMSAADepthRO"),
      add_named_node(&mut g, 4136, "clang::BuiltinType::OCLImage2dArrayMSAADepthRO"),
      add_named_node(&mut g, 4137, "clang::BuiltinType::OCLImage3dRO"),
      add_named_node(&mut g, 4138, "clang::BuiltinType::OCLImage1dWO"),
      add_named_node(&mut g, 4139, "clang::BuiltinType::OCLImage1dArrayWO"),
      add_named_node(&mut g, 4140, "clang::BuiltinType::OCLImage1dBufferWO"),
      add_named_node(&mut g, 4141, "clang::BuiltinType::OCLImage2dWO"),
      add_named_node(&mut g, 4142, "clang::BuiltinType::OCLImage2dArrayWO"),
      add_named_node(&mut g, 4143, "clang::BuiltinType::OCLImage2dDepthWO"),
      add_named_node(&mut g, 4144, "clang::BuiltinType::OCLImage2dArrayDepthWO"),
      add_named_node(&mut g, 4145, "clang::BuiltinType::OCLImage2dMSAAWO"),
      add_named_node(&mut g, 4146, "clang::BuiltinType::OCLImage2dArrayMSAAWO"),
      add_named_node(&mut g, 4147, "clang::BuiltinType::OCLImage2dMSAADepthWO"),
      add_named_node(&mut g, 4148, "clang::BuiltinType::OCLImage2dArrayMSAADepthWO"),
      add_named_node(&mut g, 4149, "clang::BuiltinType::OCLImage3dWO"),
      add_named_node(&mut g, 4150, "clang::BuiltinType::OCLImage1dRW"),
      add_named_node(&mut g, 4151, "clang::BuiltinType::OCLImage1dArrayRW"),
      add_named_node(&mut g, 4152, "clang::BuiltinType::OCLImage1dBufferRW"),
      add_named_node(&mut g, 4153, "clang::BuiltinType::OCLImage2dRW"),
      add_named_node(&mut g, 4154, "clang::BuiltinType::OCLImage2dArrayRW"),
      add_named_node(&mut g, 4155, "clang::BuiltinType::OCLImage2dDepthRW"),
      add_named_node(&mut g, 4156, "clang::BuiltinType::OCLImage2dArrayDepthRW"),
      add_named_node(&mut g, 4157, "clang::BuiltinType::OCLImage2dMSAARW"),
      add_named_node(&mut g, 4158, "clang::BuiltinType::OCLImage2dArrayMSAARW"),
      add_named_node(&mut g, 4159, "clang::BuiltinType::OCLImage2dMSAADepthRW"),
      add_named_node(&mut g, 4160, "clang::BuiltinType::OCLImage2dArrayMSAADepthRW"),
      add_named_node(&mut g, 4161, "clang::BuiltinType::OCLImage3dRW"),
      add_named_node(&mut g, 4162, "clang::BuiltinType::OCLIntelSubgroupAVCMcePayload"),
      add_named_node(&mut g, 4163, "clang::BuiltinType::OCLIntelSubgroupAVCImePayload"),
      add_named_node(&mut g, 4164, "clang::BuiltinType::OCLIntelSubgroupAVCRefPayload"),
      add_named_node(&mut g, 4165, "clang::BuiltinType::OCLIntelSubgroupAVCSicPayload"),
      add_named_node(&mut g, 4166, "clang::BuiltinType::OCLIntelSubgroupAVCMceResult"),
      add_named_node(&mut g, 4167, "clang::BuiltinType::OCLIntelSubgroupAVCImeResult"),
      add_named_node(&mut g, 4168, "clang::BuiltinType::OCLIntelSubgroupAVCRefResult"),
      add_named_node(&mut g, 4169, "clang::BuiltinType::OCLIntelSubgroupAVCSicResult"),
      add_named_node(&mut g, 4170, "clang::BuiltinType::OCLIntelSubgroupAVCImeResultSingleReferenceStreamout"),
      add_named_node(&mut g, 4171, "clang::BuiltinType::OCLIntelSubgroupAVCImeResultDualReferenceStreamout"),
      add_named_node(&mut g, 4172, "clang::BuiltinType::OCLIntelSubgroupAVCImeSingleReferenceStreamin"),
      add_named_node(&mut g, 4173, "clang::BuiltinType::OCLIntelSubgroupAVCImeDualReferenceStreamin"),
      add_named_node(&mut g, 4174, "clang::BuiltinType::SveInt8"),
      add_named_node(&mut g, 4175, "clang::BuiltinType::SveInt16"),
      add_named_node(&mut g, 4176, "clang::BuiltinType::SveInt32"),
      add_named_node(&mut g, 4177, "clang::BuiltinType::SveInt64"),
      add_named_node(&mut g, 4178, "clang::BuiltinType::SveUint8"),
      add_named_node(&mut g, 4179, "clang::BuiltinType::SveUint16"),
      add_named_node(&mut g, 4180, "clang::BuiltinType::SveUint32"),
      add_named_node(&mut g, 4181, "clang::BuiltinType::SveUint64"),
      add_named_node(&mut g, 4182, "clang::BuiltinType::SveFloat16"),
      add_named_node(&mut g, 4183, "clang::BuiltinType::SveFloat32"),
      add_named_node(&mut g, 4184, "clang::BuiltinType::SveFloat64"),
      add_named_node(&mut g, 4185, "clang::BuiltinType::SveBFloat16"),
      add_named_node(&mut g, 4186, "clang::BuiltinType::SveInt8x2"),
      add_named_node(&mut g, 4187, "clang::BuiltinType::SveInt16x2"),
      add_named_node(&mut g, 4188, "clang::BuiltinType::SveInt32x2"),
      add_named_node(&mut g, 4189, "clang::BuiltinType::SveInt64x2"),
      add_named_node(&mut g, 4190, "clang::BuiltinType::SveUint8x2"),
      add_named_node(&mut g, 4191, "clang::BuiltinType::SveUint16x2"),
      add_named_node(&mut g, 4192, "clang::BuiltinType::SveUint32x2"),
      add_named_node(&mut g, 4193, "clang::BuiltinType::SveUint64x2"),
      add_named_node(&mut g, 4194, "clang::BuiltinType::SveFloat16x2"),
      add_named_node(&mut g, 4195, "clang::BuiltinType::SveFloat32x2"),
      add_named_node(&mut g, 4196, "clang::BuiltinType::SveFloat64x2"),
      add_named_node(&mut g, 4197, "clang::BuiltinType::SveBFloat16x2"),
      add_named_node(&mut g, 4198, "clang::BuiltinType::SveInt8x3"),
      add_named_node(&mut g, 4199, "clang::BuiltinType::SveInt16x3"),
      add_named_node(&mut g, 4200, "clang::BuiltinType::SveInt32x3"),
      add_named_node(&mut g, 4201, "clang::BuiltinType::SveInt64x3"),
      add_named_node(&mut g, 4202, "clang::BuiltinType::SveUint8x3"),
      add_named_node(&mut g, 4203, "clang::BuiltinType::SveUint16x3"),
      add_named_node(&mut g, 4204, "clang::BuiltinType::SveUint32x3"),
      add_named_node(&mut g, 4205, "clang::BuiltinType::SveUint64x3"),
      add_named_node(&mut g, 4206, "clang::BuiltinType::SveFloat16x3"),
      add_named_node(&mut g, 4207, "clang::BuiltinType::SveFloat32x3"),
      add_named_node(&mut g, 4208, "clang::BuiltinType::SveFloat64x3"),
      add_named_node(&mut g, 4209, "clang::BuiltinType::SveBFloat16x3"),
      add_named_node(&mut g, 4210, "clang::BuiltinType::SveInt8x4"),
      add_named_node(&mut g, 4211, "clang::BuiltinType::SveInt16x4"),
      add_named_node(&mut g, 4212, "clang::BuiltinType::SveInt32x4"),
      add_named_node(&mut g, 4213, "clang::BuiltinType::SveInt64x4"),
      add_named_node(&mut g, 4214, "clang::BuiltinType::SveUint8x4"),
      add_named_node(&mut g, 4215, "clang::BuiltinType::SveUint16x4"),
      add_named_node(&mut g, 4216, "clang::BuiltinType::SveUint32x4"),
      add_named_node(&mut g, 4217, "clang::BuiltinType::SveUint64x4"),
      add_named_node(&mut g, 4218, "clang::BuiltinType::SveFloat16x4"),
      add_named_node(&mut g, 4219, "clang::BuiltinType::SveFloat32x4"),
      add_named_node(&mut g, 4220, "clang::BuiltinType::SveFloat64x4"),
      add_named_node(&mut g, 4221, "clang::BuiltinType::SveBFloat16x4"),
      add_named_node(&mut g, 4222, "clang::BuiltinType::SveBool"),
      add_named_node(&mut g, 4223, "clang::BuiltinType::SveBoolx2"),
      add_named_node(&mut g, 4224, "clang::BuiltinType::SveBoolx4"),
      add_named_node(&mut g, 4225, "clang::BuiltinType::SveCount"),
      add_named_node(&mut g, 4226, "clang::BuiltinType::VectorQuad"),
      add_named_node(&mut g, 4227, "clang::BuiltinType::VectorPair"),
      add_named_node(&mut g, 4228, "clang::BuiltinType::RvvInt8mf8"),
      add_named_node(&mut g, 4229, "clang::BuiltinType::RvvInt8mf4"),
      add_named_node(&mut g, 4230, "clang::BuiltinType::RvvInt8mf2"),
      add_named_node(&mut g, 4231, "clang::BuiltinType::RvvInt8m1"),
      add_named_node(&mut g, 4232, "clang::BuiltinType::RvvInt8m2"),
      add_named_node(&mut g, 4233, "clang::BuiltinType::RvvInt8m4"),
      add_named_node(&mut g, 4234, "clang::BuiltinType::RvvInt8m8"),
      add_named_node(&mut g, 4235, "clang::BuiltinType::RvvUint8mf8"),
      add_named_node(&mut g, 4236, "clang::BuiltinType::RvvUint8mf4"),
      add_named_node(&mut g, 4237, "clang::BuiltinType::RvvUint8mf2"),
      add_named_node(&mut g, 4238, "clang::BuiltinType::RvvUint8m1"),
      add_named_node(&mut g, 4239, "clang::BuiltinType::RvvUint8m2"),
      add_named_node(&mut g, 4240, "clang::BuiltinType::RvvUint8m4"),
      add_named_node(&mut g, 4241, "clang::BuiltinType::RvvUint8m8"),
      add_named_node(&mut g, 4242, "clang::BuiltinType::RvvInt16mf4"),
      add_named_node(&mut g, 4243, "clang::BuiltinType::RvvInt16mf2"),
      add_named_node(&mut g, 4244, "clang::BuiltinType::RvvInt16m1"),
      add_named_node(&mut g, 4245, "clang::BuiltinType::RvvInt16m2"),
      add_named_node(&mut g, 4246, "clang::BuiltinType::RvvInt16m4"),
      add_named_node(&mut g, 4247, "clang::BuiltinType::RvvInt16m8"),
      add_named_node(&mut g, 4248, "clang::BuiltinType::RvvUint16mf4"),
      add_named_node(&mut g, 4249, "clang::BuiltinType::RvvUint16mf2"),
      add_named_node(&mut g, 4250, "clang::BuiltinType::RvvUint16m1"),
      add_named_node(&mut g, 4251, "clang::BuiltinType::RvvUint16m2"),
      add_named_node(&mut g, 4252, "clang::BuiltinType::RvvUint16m4"),
      add_named_node(&mut g, 4253, "clang::BuiltinType::RvvUint16m8"),
      add_named_node(&mut g, 4254, "clang::BuiltinType::RvvInt32mf2"),
      add_named_node(&mut g, 4255, "clang::BuiltinType::RvvInt32m1"),
      add_named_node(&mut g, 4256, "clang::BuiltinType::RvvInt32m2"),
      add_named_node(&mut g, 4257, "clang::BuiltinType::RvvInt32m4"),
      add_named_node(&mut g, 4258, "clang::BuiltinType::RvvInt32m8"),
      add_named_node(&mut g, 4259, "clang::BuiltinType::RvvUint32mf2"),
      add_named_node(&mut g, 4260, "clang::BuiltinType::RvvUint32m1"),
      add_named_node(&mut g, 4261, "clang::BuiltinType::RvvUint32m2"),
      add_named_node(&mut g, 4262, "clang::BuiltinType::RvvUint32m4"),
      add_named_node(&mut g, 4263, "clang::BuiltinType::RvvUint32m8"),
      add_named_node(&mut g, 4264, "clang::BuiltinType::RvvInt64m1"),
      add_named_node(&mut g, 4265, "clang::BuiltinType::RvvInt64m2"),
      add_named_node(&mut g, 4266, "clang::BuiltinType::RvvInt64m4"),
      add_named_node(&mut g, 4267, "clang::BuiltinType::RvvInt64m8"),
      add_named_node(&mut g, 4268, "clang::BuiltinType::RvvUint64m1"),
      add_named_node(&mut g, 4269, "clang::BuiltinType::RvvUint64m2"),
      add_named_node(&mut g, 4270, "clang::BuiltinType::RvvUint64m4"),
      add_named_node(&mut g, 4271, "clang::BuiltinType::RvvUint64m8"),
      add_named_node(&mut g, 4272, "clang::BuiltinType::RvvFloat16mf4"),
      add_named_node(&mut g, 4273, "clang::BuiltinType::RvvFloat16mf2"),
      add_named_node(&mut g, 4274, "clang::BuiltinType::RvvFloat16m1"),
      add_named_node(&mut g, 4275, "clang::BuiltinType::RvvFloat16m2"),
      add_named_node(&mut g, 4276, "clang::BuiltinType::RvvFloat16m4"),
      add_named_node(&mut g, 4277, "clang::BuiltinType::RvvFloat16m8"),
      add_named_node(&mut g, 4278, "clang::BuiltinType::RvvBFloat16mf4"),
      add_named_node(&mut g, 4279, "clang::BuiltinType::RvvBFloat16mf2"),
      add_named_node(&mut g, 4280, "clang::BuiltinType::RvvBFloat16m1"),
      add_named_node(&mut g, 4281, "clang::BuiltinType::RvvBFloat16m2"),
      add_named_node(&mut g, 4282, "clang::BuiltinType::RvvBFloat16m4"),
      add_named_node(&mut g, 4283, "clang::BuiltinType::RvvBFloat16m8"),
      add_named_node(&mut g, 4284, "clang::BuiltinType::RvvFloat32mf2"),
      add_named_node(&mut g, 4285, "clang::BuiltinType::RvvFloat32m1"),
      add_named_node(&mut g, 4286, "clang::BuiltinType::RvvFloat32m2"),
      add_named_node(&mut g, 4287, "clang::BuiltinType::RvvFloat32m4"),
      add_named_node(&mut g, 4288, "clang::BuiltinType::RvvFloat32m8"),
      add_named_node(&mut g, 4289, "clang::BuiltinType::RvvFloat64m1"),
      add_named_node(&mut g, 4290, "clang::BuiltinType::RvvFloat64m2"),
      add_named_node(&mut g, 4291, "clang::BuiltinType::RvvFloat64m4"),
      add_named_node(&mut g, 4292, "clang::BuiltinType::RvvFloat64m8"),
      add_named_node(&mut g, 4293, "clang::BuiltinType::RvvBool1"),
      add_named_node(&mut g, 4294, "clang::BuiltinType::RvvBool2"),
      add_named_node(&mut g, 4295, "clang::BuiltinType::RvvBool4"),
      add_named_node(&mut g, 4296, "clang::BuiltinType::RvvBool8"),
      add_named_node(&mut g, 4297, "clang::BuiltinType::RvvBool16"),
      add_named_node(&mut g, 4298, "clang::BuiltinType::RvvBool32"),
      add_named_node(&mut g, 4299, "clang::BuiltinType::RvvBool64"),
      add_named_node(&mut g, 4300, "clang::BuiltinType::RvvInt8mf8x2"),
      add_named_node(&mut g, 4301, "clang::BuiltinType::RvvInt8mf8x3"),
      add_named_node(&mut g, 4302, "clang::BuiltinType::RvvInt8mf8x4"),
      add_named_node(&mut g, 4303, "clang::BuiltinType::RvvInt8mf8x5"),
      add_named_node(&mut g, 4304, "clang::BuiltinType::RvvInt8mf8x6"),
      add_named_node(&mut g, 4305, "clang::BuiltinType::RvvInt8mf8x7"),
      add_named_node(&mut g, 4306, "clang::BuiltinType::RvvInt8mf8x8"),
      add_named_node(&mut g, 4307, "clang::BuiltinType::RvvInt8mf4x2"),
      add_named_node(&mut g, 4308, "clang::BuiltinType::RvvInt8mf4x3"),
      add_named_node(&mut g, 4309, "clang::BuiltinType::RvvInt8mf4x4"),
      add_named_node(&mut g, 4310, "clang::BuiltinType::RvvInt8mf4x5"),
      add_named_node(&mut g, 4311, "clang::BuiltinType::RvvInt8mf4x6"),
      add_named_node(&mut g, 4312, "clang::BuiltinType::RvvInt8mf4x7"),
      add_named_node(&mut g, 4313, "clang::BuiltinType::RvvInt8mf4x8"),
      add_named_node(&mut g, 4314, "clang::BuiltinType::RvvInt8mf2x2"),
      add_named_node(&mut g, 4315, "clang::BuiltinType::RvvInt8mf2x3"),
      add_named_node(&mut g, 4316, "clang::BuiltinType::RvvInt8mf2x4"),
      add_named_node(&mut g, 4317, "clang::BuiltinType::RvvInt8mf2x5"),
      add_named_node(&mut g, 4318, "clang::BuiltinType::RvvInt8mf2x6"),
      add_named_node(&mut g, 4319, "clang::BuiltinType::RvvInt8mf2x7"),
      add_named_node(&mut g, 4320, "clang::BuiltinType::RvvInt8mf2x8"),
      add_named_node(&mut g, 4321, "clang::BuiltinType::RvvInt8m1x2"),
      add_named_node(&mut g, 4322, "clang::BuiltinType::RvvInt8m1x3"),
      add_named_node(&mut g, 4323, "clang::BuiltinType::RvvInt8m1x4"),
      add_named_node(&mut g, 4324, "clang::BuiltinType::RvvInt8m1x5"),
      add_named_node(&mut g, 4325, "clang::BuiltinType::RvvInt8m1x6"),
      add_named_node(&mut g, 4326, "clang::BuiltinType::RvvInt8m1x7"),
      add_named_node(&mut g, 4327, "clang::BuiltinType::RvvInt8m1x8"),
      add_named_node(&mut g, 4328, "clang::BuiltinType::RvvInt8m2x2"),
      add_named_node(&mut g, 4329, "clang::BuiltinType::RvvInt8m2x3"),
      add_named_node(&mut g, 4330, "clang::BuiltinType::RvvInt8m2x4"),
      add_named_node(&mut g, 4331, "clang::BuiltinType::RvvInt8m4x2"),
      add_named_node(&mut g, 4332, "clang::BuiltinType::RvvUint8mf8x2"),
      add_named_node(&mut g, 4333, "clang::BuiltinType::RvvUint8mf8x3"),
      add_named_node(&mut g, 4334, "clang::BuiltinType::RvvUint8mf8x4"),
      add_named_node(&mut g, 4335, "clang::BuiltinType::RvvUint8mf8x5"),
      add_named_node(&mut g, 4336, "clang::BuiltinType::RvvUint8mf8x6"),
      add_named_node(&mut g, 4337, "clang::BuiltinType::RvvUint8mf8x7"),
      add_named_node(&mut g, 4338, "clang::BuiltinType::RvvUint8mf8x8"),
      add_named_node(&mut g, 4339, "clang::BuiltinType::RvvUint8mf4x2"),
      add_named_node(&mut g, 4340, "clang::BuiltinType::RvvUint8mf4x3"),
      add_named_node(&mut g, 4341, "clang::BuiltinType::RvvUint8mf4x4"),
      add_named_node(&mut g, 4342, "clang::BuiltinType::RvvUint8mf4x5"),
      add_named_node(&mut g, 4343, "clang::BuiltinType::RvvUint8mf4x6"),
      add_named_node(&mut g, 4344, "clang::BuiltinType::RvvUint8mf4x7"),
      add_named_node(&mut g, 4345, "clang::BuiltinType::RvvUint8mf4x8"),
      add_named_node(&mut g, 4346, "clang::BuiltinType::RvvUint8mf2x2"),
      add_named_node(&mut g, 4347, "clang::BuiltinType::RvvUint8mf2x3"),
      add_named_node(&mut g, 4348, "clang::BuiltinType::RvvUint8mf2x4"),
      add_named_node(&mut g, 4349, "clang::BuiltinType::RvvUint8mf2x5"),
      add_named_node(&mut g, 4350, "clang::BuiltinType::RvvUint8mf2x6"),
      add_named_node(&mut g, 4351, "clang::BuiltinType::RvvUint8mf2x7"),
      add_named_node(&mut g, 4352, "clang::BuiltinType::RvvUint8mf2x8"),
      add_named_node(&mut g, 4353, "clang::BuiltinType::RvvUint8m1x2"),
      add_named_node(&mut g, 4354, "clang::BuiltinType::RvvUint8m1x3"),
      add_named_node(&mut g, 4355, "clang::BuiltinType::RvvUint8m1x4"),
      add_named_node(&mut g, 4356, "clang::BuiltinType::RvvUint8m1x5"),
      add_named_node(&mut g, 4357, "clang::BuiltinType::RvvUint8m1x6"),
      add_named_node(&mut g, 4358, "clang::BuiltinType::RvvUint8m1x7"),
      add_named_node(&mut g, 4359, "clang::BuiltinType::RvvUint8m1x8"),
      add_named_node(&mut g, 4360, "clang::BuiltinType::RvvUint8m2x2"),
      add_named_node(&mut g, 4361, "clang::BuiltinType::RvvUint8m2x3"),
      add_named_node(&mut g, 4362, "clang::BuiltinType::RvvUint8m2x4"),
      add_named_node(&mut g, 4363, "clang::BuiltinType::RvvUint8m4x2"),
      add_named_node(&mut g, 4364, "clang::BuiltinType::RvvInt16mf4x2"),
      add_named_node(&mut g, 4365, "clang::BuiltinType::RvvInt16mf4x3"),
      add_named_node(&mut g, 4366, "clang::BuiltinType::RvvInt16mf4x4"),
      add_named_node(&mut g, 4367, "clang::BuiltinType::RvvInt16mf4x5"),
      add_named_node(&mut g, 4368, "clang::BuiltinType::RvvInt16mf4x6"),
      add_named_node(&mut g, 4369, "clang::BuiltinType::RvvInt16mf4x7"),
      add_named_node(&mut g, 4370, "clang::BuiltinType::RvvInt16mf4x8"),
      add_named_node(&mut g, 4371, "clang::BuiltinType::RvvInt16mf2x2"),
      add_named_node(&mut g, 4372, "clang::BuiltinType::RvvInt16mf2x3"),
      add_named_node(&mut g, 4373, "clang::BuiltinType::RvvInt16mf2x4"),
      add_named_node(&mut g, 4374, "clang::BuiltinType::RvvInt16mf2x5"),
      add_named_node(&mut g, 4375, "clang::BuiltinType::RvvInt16mf2x6"),
      add_named_node(&mut g, 4376, "clang::BuiltinType::RvvInt16mf2x7"),
      add_named_node(&mut g, 4377, "clang::BuiltinType::RvvInt16mf2x8"),
      add_named_node(&mut g, 4378, "clang::BuiltinType::RvvInt16m1x2"),
      add_named_node(&mut g, 4379, "clang::BuiltinType::RvvInt16m1x3"),
      add_named_node(&mut g, 4380, "clang::BuiltinType::RvvInt16m1x4"),
      add_named_node(&mut g, 4381, "clang::BuiltinType::RvvInt16m1x5"),
      add_named_node(&mut g, 4382, "clang::BuiltinType::RvvInt16m1x6"),
      add_named_node(&mut g, 4383, "clang::BuiltinType::RvvInt16m1x7"),
      add_named_node(&mut g, 4384, "clang::BuiltinType::RvvInt16m1x8"),
      add_named_node(&mut g, 4385, "clang::BuiltinType::RvvInt16m2x2"),
      add_named_node(&mut g, 4386, "clang::BuiltinType::RvvInt16m2x3"),
      add_named_node(&mut g, 4387, "clang::BuiltinType::RvvInt16m2x4"),
      add_named_node(&mut g, 4388, "clang::BuiltinType::RvvInt16m4x2"),
      add_named_node(&mut g, 4389, "clang::BuiltinType::RvvUint16mf4x2"),
      add_named_node(&mut g, 4390, "clang::BuiltinType::RvvUint16mf4x3"),
      add_named_node(&mut g, 4391, "clang::BuiltinType::RvvUint16mf4x4"),
      add_named_node(&mut g, 4392, "clang::BuiltinType::RvvUint16mf4x5"),
      add_named_node(&mut g, 4393, "clang::BuiltinType::RvvUint16mf4x6"),
      add_named_node(&mut g, 4394, "clang::BuiltinType::RvvUint16mf4x7"),
      add_named_node(&mut g, 4395, "clang::BuiltinType::RvvUint16mf4x8"),
      add_named_node(&mut g, 4396, "clang::BuiltinType::RvvUint16mf2x2"),
      add_named_node(&mut g, 4397, "clang::BuiltinType::RvvUint16mf2x3"),
      add_named_node(&mut g, 4398, "clang::BuiltinType::RvvUint16mf2x4"),
      add_named_node(&mut g, 4399, "clang::BuiltinType::RvvUint16mf2x5"),
      add_named_node(&mut g, 4400, "clang::BuiltinType::RvvUint16mf2x6"),
      add_named_node(&mut g, 4401, "clang::BuiltinType::RvvUint16mf2x7"),
      add_named_node(&mut g, 4402, "clang::BuiltinType::RvvUint16mf2x8"),
      add_named_node(&mut g, 4403, "clang::BuiltinType::RvvUint16m1x2"),
      add_named_node(&mut g, 4404, "clang::BuiltinType::RvvUint16m1x3"),
      add_named_node(&mut g, 4405, "clang::BuiltinType::RvvUint16m1x4"),
      add_named_node(&mut g, 4406, "clang::BuiltinType::RvvUint16m1x5"),
      add_named_node(&mut g, 4407, "clang::BuiltinType::RvvUint16m1x6"),
      add_named_node(&mut g, 4408, "clang::BuiltinType::RvvUint16m1x7"),
      add_named_node(&mut g, 4409, "clang::BuiltinType::RvvUint16m1x8"),
      add_named_node(&mut g, 4410, "clang::BuiltinType::RvvUint16m2x2"),
      add_named_node(&mut g, 4411, "clang::BuiltinType::RvvUint16m2x3"),
      add_named_node(&mut g, 4412, "clang::BuiltinType::RvvUint16m2x4"),
      add_named_node(&mut g, 4413, "clang::BuiltinType::RvvUint16m4x2"),
      add_named_node(&mut g, 4414, "clang::BuiltinType::RvvInt32mf2x2"),
      add_named_node(&mut g, 4415, "clang::BuiltinType::RvvInt32mf2x3"),
      add_named_node(&mut g, 4416, "clang::BuiltinType::RvvInt32mf2x4"),
      add_named_node(&mut g, 4417, "clang::BuiltinType::RvvInt32mf2x5"),
      add_named_node(&mut g, 4418, "clang::BuiltinType::RvvInt32mf2x6"),
      add_named_node(&mut g, 4419, "clang::BuiltinType::RvvInt32mf2x7"),
      add_named_node(&mut g, 4420, "clang::BuiltinType::RvvInt32mf2x8"),
      add_named_node(&mut g, 4421, "clang::BuiltinType::RvvInt32m1x2"),
      add_named_node(&mut g, 4422, "clang::BuiltinType::RvvInt32m1x3"),
      add_named_node(&mut g, 4423, "clang::BuiltinType::RvvInt32m1x4"),
      add_named_node(&mut g, 4424, "clang::BuiltinType::RvvInt32m1x5"),
      add_named_node(&mut g, 4425, "clang::BuiltinType::RvvInt32m1x6"),
      add_named_node(&mut g, 4426, "clang::BuiltinType::RvvInt32m1x7"),
      add_named_node(&mut g, 4427, "clang::BuiltinType::RvvInt32m1x8"),
      add_named_node(&mut g, 4428, "clang::BuiltinType::RvvInt32m2x2"),
      add_named_node(&mut g, 4429, "clang::BuiltinType::RvvInt32m2x3"),
      add_named_node(&mut g, 4430, "clang::BuiltinType::RvvInt32m2x4"),
      add_named_node(&mut g, 4431, "clang::BuiltinType::RvvInt32m4x2"),
      add_named_node(&mut g, 4432, "clang::BuiltinType::RvvUint32mf2x2"),
      add_named_node(&mut g, 4433, "clang::BuiltinType::RvvUint32mf2x3"),
      add_named_node(&mut g, 4434, "clang::BuiltinType::RvvUint32mf2x4"),
      add_named_node(&mut g, 4435, "clang::BuiltinType::RvvUint32mf2x5"),
      add_named_node(&mut g, 4436, "clang::BuiltinType::RvvUint32mf2x6"),
      add_named_node(&mut g, 4437, "clang::BuiltinType::RvvUint32mf2x7"),
      add_named_node(&mut g, 4438, "clang::BuiltinType::RvvUint32mf2x8"),
      add_named_node(&mut g, 4439, "clang::BuiltinType::RvvUint32m1x2"),
      add_named_node(&mut g, 4440, "clang::BuiltinType::RvvUint32m1x3"),
      add_named_node(&mut g, 4441, "clang::BuiltinType::RvvUint32m1x4"),
      add_named_node(&mut g, 4442, "clang::BuiltinType::RvvUint32m1x5"),
      add_named_node(&mut g, 4443, "clang::BuiltinType::RvvUint32m1x6"),
      add_named_node(&mut g, 4444, "clang::BuiltinType::RvvUint32m1x7"),
      add_named_node(&mut g, 4445, "clang::BuiltinType::RvvUint32m1x8"),
      add_named_node(&mut g, 4446, "clang::BuiltinType::RvvUint32m2x2"),
      add_named_node(&mut g, 4447, "clang::BuiltinType::RvvUint32m2x3"),
      add_named_node(&mut g, 4448, "clang::BuiltinType::RvvUint32m2x4"),
      add_named_node(&mut g, 4449, "clang::BuiltinType::RvvUint32m4x2"),
      add_named_node(&mut g, 4450, "clang::BuiltinType::RvvInt64m1x2"),
      add_named_node(&mut g, 4451, "clang::BuiltinType::RvvInt64m1x3"),
      add_named_node(&mut g, 4452, "clang::BuiltinType::RvvInt64m1x4"),
      add_named_node(&mut g, 4453, "clang::BuiltinType::RvvInt64m1x5"),
      add_named_node(&mut g, 4454, "clang::BuiltinType::RvvInt64m1x6"),
      add_named_node(&mut g, 4455, "clang::BuiltinType::RvvInt64m1x7"),
      add_named_node(&mut g, 4456, "clang::BuiltinType::RvvInt64m1x8"),
      add_named_node(&mut g, 4457, "clang::BuiltinType::RvvInt64m2x2"),
      add_named_node(&mut g, 4458, "clang::BuiltinType::RvvInt64m2x3"),
      add_named_node(&mut g, 4459, "clang::BuiltinType::RvvInt64m2x4"),
      add_named_node(&mut g, 4460, "clang::BuiltinType::RvvInt64m4x2"),
      add_named_node(&mut g, 4461, "clang::BuiltinType::RvvUint64m1x2"),
      add_named_node(&mut g, 4462, "clang::BuiltinType::RvvUint64m1x3"),
      add_named_node(&mut g, 4463, "clang::BuiltinType::RvvUint64m1x4"),
      add_named_node(&mut g, 4464, "clang::BuiltinType::RvvUint64m1x5"),
      add_named_node(&mut g, 4465, "clang::BuiltinType::RvvUint64m1x6"),
      add_named_node(&mut g, 4466, "clang::BuiltinType::RvvUint64m1x7"),
      add_named_node(&mut g, 4467, "clang::BuiltinType::RvvUint64m1x8"),
      add_named_node(&mut g, 4468, "clang::BuiltinType::RvvUint64m2x2"),
      add_named_node(&mut g, 4469, "clang::BuiltinType::RvvUint64m2x3"),
      add_named_node(&mut g, 4470, "clang::BuiltinType::RvvUint64m2x4"),
      add_named_node(&mut g, 4471, "clang::BuiltinType::RvvUint64m4x2"),
      add_named_node(&mut g, 4472, "clang::BuiltinType::RvvFloat16mf4x2"),
      add_named_node(&mut g, 4473, "clang::BuiltinType::RvvFloat16mf4x3"),
      add_named_node(&mut g, 4474, "clang::BuiltinType::RvvFloat16mf4x4"),
      add_named_node(&mut g, 4475, "clang::BuiltinType::RvvFloat16mf4x5"),
      add_named_node(&mut g, 4476, "clang::BuiltinType::RvvFloat16mf4x6"),
      add_named_node(&mut g, 4477, "clang::BuiltinType::RvvFloat16mf4x7"),
      add_named_node(&mut g, 4478, "clang::BuiltinType::RvvFloat16mf4x8"),
      add_named_node(&mut g, 4479, "clang::BuiltinType::RvvFloat16mf2x2"),
      add_named_node(&mut g, 4480, "clang::BuiltinType::RvvFloat16mf2x3"),
      add_named_node(&mut g, 4481, "clang::BuiltinType::RvvFloat16mf2x4"),
      add_named_node(&mut g, 4482, "clang::BuiltinType::RvvFloat16mf2x5"),
      add_named_node(&mut g, 4483, "clang::BuiltinType::RvvFloat16mf2x6"),
      add_named_node(&mut g, 4484, "clang::BuiltinType::RvvFloat16mf2x7"),
      add_named_node(&mut g, 4485, "clang::BuiltinType::RvvFloat16mf2x8"),
      add_named_node(&mut g, 4486, "clang::BuiltinType::RvvFloat16m1x2"),
      add_named_node(&mut g, 4487, "clang::BuiltinType::RvvFloat16m1x3"),
      add_named_node(&mut g, 4488, "clang::BuiltinType::RvvFloat16m1x4"),
      add_named_node(&mut g, 4489, "clang::BuiltinType::RvvFloat16m1x5"),
      add_named_node(&mut g, 4490, "clang::BuiltinType::RvvFloat16m1x6"),
      add_named_node(&mut g, 4491, "clang::BuiltinType::RvvFloat16m1x7"),
      add_named_node(&mut g, 4492, "clang::BuiltinType::RvvFloat16m1x8"),
      add_named_node(&mut g, 4493, "clang::BuiltinType::RvvFloat16m2x2"),
      add_named_node(&mut g, 4494, "clang::BuiltinType::RvvFloat16m2x3"),
      add_named_node(&mut g, 4495, "clang::BuiltinType::RvvFloat16m2x4"),
      add_named_node(&mut g, 4496, "clang::BuiltinType::RvvFloat16m4x2"),
      add_named_node(&mut g, 4497, "clang::BuiltinType::RvvFloat32mf2x2"),
      add_named_node(&mut g, 4498, "clang::BuiltinType::RvvFloat32mf2x3"),
      add_named_node(&mut g, 4499, "clang::BuiltinType::RvvFloat32mf2x4"),
      add_named_node(&mut g, 4500, "clang::BuiltinType::RvvFloat32mf2x5"),
      add_named_node(&mut g, 4501, "clang::BuiltinType::RvvFloat32mf2x6"),
      add_named_node(&mut g, 4502, "clang::BuiltinType::RvvFloat32mf2x7"),
      add_named_node(&mut g, 4503, "clang::BuiltinType::RvvFloat32mf2x8"),
      add_named_node(&mut g, 4504, "clang::BuiltinType::RvvFloat32m1x2"),
      add_named_node(&mut g, 4505, "clang::BuiltinType::RvvFloat32m1x3"),
      add_named_node(&mut g, 4506, "clang::BuiltinType::RvvFloat32m1x4"),
      add_named_node(&mut g, 4507, "clang::BuiltinType::RvvFloat32m1x5"),
      add_named_node(&mut g, 4508, "clang::BuiltinType::RvvFloat32m1x6"),
      add_named_node(&mut g, 4509, "clang::BuiltinType::RvvFloat32m1x7"),
      add_named_node(&mut g, 4510, "clang::BuiltinType::RvvFloat32m1x8"),
      add_named_node(&mut g, 4511, "clang::BuiltinType::RvvFloat32m2x2"),
      add_named_node(&mut g, 4512, "clang::BuiltinType::RvvFloat32m2x3"),
      add_named_node(&mut g, 4513, "clang::BuiltinType::RvvFloat32m2x4"),
      add_named_node(&mut g, 4514, "clang::BuiltinType::RvvFloat32m4x2"),
      add_named_node(&mut g, 4515, "clang::BuiltinType::RvvFloat64m1x2"),
      add_named_node(&mut g, 4516, "clang::BuiltinType::RvvFloat64m1x3"),
      add_named_node(&mut g, 4517, "clang::BuiltinType::RvvFloat64m1x4"),
      add_named_node(&mut g, 4518, "clang::BuiltinType::RvvFloat64m1x5"),
      add_named_node(&mut g, 4519, "clang::BuiltinType::RvvFloat64m1x6"),
      add_named_node(&mut g, 4520, "clang::BuiltinType::RvvFloat64m1x7"),
      add_named_node(&mut g, 4521, "clang::BuiltinType::RvvFloat64m1x8"),
      add_named_node(&mut g, 4522, "clang::BuiltinType::RvvFloat64m2x2"),
      add_named_node(&mut g, 4523, "clang::BuiltinType::RvvFloat64m2x3"),
      add_named_node(&mut g, 4524, "clang::BuiltinType::RvvFloat64m2x4"),
      add_named_node(&mut g, 4525, "clang::BuiltinType::RvvFloat64m4x2"),
      add_named_node(&mut g, 4526, "clang::BuiltinType::RvvBFloat16mf4x2"),
      add_named_node(&mut g, 4527, "clang::BuiltinType::RvvBFloat16mf4x3"),
      add_named_node(&mut g, 4528, "clang::BuiltinType::RvvBFloat16mf4x4"),
      add_named_node(&mut g, 4529, "clang::BuiltinType::RvvBFloat16mf4x5"),
      add_named_node(&mut g, 4530, "clang::BuiltinType::RvvBFloat16mf4x6"),
      add_named_node(&mut g, 4531, "clang::BuiltinType::RvvBFloat16mf4x7"),
      add_named_node(&mut g, 4532, "clang::BuiltinType::RvvBFloat16mf4x8"),
      add_named_node(&mut g, 4533, "clang::BuiltinType::RvvBFloat16mf2x2"),
      add_named_node(&mut g, 4534, "clang::BuiltinType::RvvBFloat16mf2x3"),
      add_named_node(&mut g, 4535, "clang::BuiltinType::RvvBFloat16mf2x4"),
      add_named_node(&mut g, 4536, "clang::BuiltinType::RvvBFloat16mf2x5"),
      add_named_node(&mut g, 4537, "clang::BuiltinType::RvvBFloat16mf2x6"),
      add_named_node(&mut g, 4538, "clang::BuiltinType::RvvBFloat16mf2x7"),
      add_named_node(&mut g, 4539, "clang::BuiltinType::RvvBFloat16mf2x8"),
      add_named_node(&mut g, 4540, "clang::BuiltinType::RvvBFloat16m1x2"),
      add_named_node(&mut g, 4541, "clang::BuiltinType::RvvBFloat16m1x3"),
      add_named_node(&mut g, 4542, "clang::BuiltinType::RvvBFloat16m1x4"),
      add_named_node(&mut g, 4543, "clang::BuiltinType::RvvBFloat16m1x5"),
      add_named_node(&mut g, 4544, "clang::BuiltinType::RvvBFloat16m1x6"),
      add_named_node(&mut g, 4545, "clang::BuiltinType::RvvBFloat16m1x7"),
      add_named_node(&mut g, 4546, "clang::BuiltinType::RvvBFloat16m1x8"),
      add_named_node(&mut g, 4547, "clang::BuiltinType::RvvBFloat16m2x2"),
      add_named_node(&mut g, 4548, "clang::BuiltinType::RvvBFloat16m2x3"),
      add_named_node(&mut g, 4549, "clang::BuiltinType::RvvBFloat16m2x4"),
      add_named_node(&mut g, 4550, "clang::BuiltinType::RvvBFloat16m4x2"),
      add_named_node(&mut g, 4551, "clang::BuiltinType::WasmExternRef"),
      add_named_node(&mut g, 4552, "clang::BuiltinType::Void"),
      add_named_node(&mut g, 4553, "clang::BuiltinType::Bool"),
      add_named_node(&mut g, 4554, "clang::BuiltinType::Char_U"),
      add_named_node(&mut g, 4555, "clang::BuiltinType::UChar"),
      add_named_node(&mut g, 4556, "clang::BuiltinType::WChar_U"),
      add_named_node(&mut g, 4557, "clang::BuiltinType::Char8"),
      add_named_node(&mut g, 4558, "clang::BuiltinType::Char16"),
      add_named_node(&mut g, 4559, "clang::BuiltinType::Char32"),
      add_named_node(&mut g, 4560, "clang::BuiltinType::UShort"),
      add_named_node(&mut g, 4561, "clang::BuiltinType::UInt"),
      add_named_node(&mut g, 4562, "clang::BuiltinType::ULong"),
      add_named_node(&mut g, 4563, "clang::BuiltinType::ULongLong"),
      add_named_node(&mut g, 4564, "clang::BuiltinType::UInt128"),
      add_named_node(&mut g, 4565, "clang::BuiltinType::Char_S"),
      add_named_node(&mut g, 4566, "clang::BuiltinType::SChar"),
      add_named_node(&mut g, 4567, "clang::BuiltinType::WChar_S"),
      add_named_node(&mut g, 4568, "clang::BuiltinType::Short"),
      add_named_node(&mut g, 4569, "clang::BuiltinType::Int"),
      add_named_node(&mut g, 4570, "clang::BuiltinType::Long"),
      add_named_node(&mut g, 4571, "clang::BuiltinType::LongLong"),
      add_named_node(&mut g, 4572, "clang::BuiltinType::Int128"),
      add_named_node(&mut g, 4573, "clang::BuiltinType::ShortAccum"),
      add_named_node(&mut g, 4574, "clang::BuiltinType::Accum"),
      add_named_node(&mut g, 4575, "clang::BuiltinType::LongAccum"),
      add_named_node(&mut g, 4576, "clang::BuiltinType::UShortAccum"),
      add_named_node(&mut g, 4577, "clang::BuiltinType::UAccum"),
      add_named_node(&mut g, 4578, "clang::BuiltinType::ULongAccum"),
      add_named_node(&mut g, 4579, "clang::BuiltinType::ShortFract"),
      add_named_node(&mut g, 4580, "clang::BuiltinType::Fract"),
      add_named_node(&mut g, 4581, "clang::BuiltinType::LongFract"),
      add_named_node(&mut g, 4582, "clang::BuiltinType::UShortFract"),
      add_named_node(&mut g, 4583, "clang::BuiltinType::UFract"),
      add_named_node(&mut g, 4584, "clang::BuiltinType::ULongFract"),
      add_named_node(&mut g, 4585, "clang::BuiltinType::SatShortAccum"),
      add_named_node(&mut g, 4586, "clang::BuiltinType::SatAccum"),
      add_named_node(&mut g, 4587, "clang::BuiltinType::SatLongAccum"),
      add_named_node(&mut g, 4588, "clang::BuiltinType::SatUShortAccum"),
      add_named_node(&mut g, 4589, "clang::BuiltinType::SatUAccum"),
      add_named_node(&mut g, 4590, "clang::BuiltinType::SatULongAccum"),
      add_named_node(&mut g, 4591, "clang::BuiltinType::SatShortFract"),
      add_named_node(&mut g, 4592, "clang::BuiltinType::SatFract"),
      add_named_node(&mut g, 4593, "clang::BuiltinType::SatLongFract"),
      add_named_node(&mut g, 4594, "clang::BuiltinType::SatUShortFract"),
      add_named_node(&mut g, 4595, "clang::BuiltinType::SatUFract"),
      add_named_node(&mut g, 4596, "clang::BuiltinType::SatULongFract"),
      add_named_node(&mut g, 4597, "clang::BuiltinType::Half"),
      add_named_node(&mut g, 4598, "clang::BuiltinType::Float"),
      add_named_node(&mut g, 4599, "clang::BuiltinType::Double"),
      add_named_node(&mut g, 4600, "clang::BuiltinType::LongDouble"),
      add_named_node(&mut g, 4601, "clang::BuiltinType::Float16"),
      add_named_node(&mut g, 4602, "clang::BuiltinType::BFloat16"),
      add_named_node(&mut g, 4603, "clang::BuiltinType::Float128"),
      add_named_node(&mut g, 4604, "clang::BuiltinType::Ibm128"),
      add_named_node(&mut g, 4605, "clang::BuiltinType::NullPtr"),
      add_named_node(&mut g, 4606, "clang::BuiltinType::ObjCId"),
      add_named_node(&mut g, 4607, "clang::BuiltinType::ObjCClass"),
      add_named_node(&mut g, 4608, "clang::BuiltinType::ObjCSel"),
      add_named_node(&mut g, 4609, "clang::BuiltinType::OCLSampler"),
      add_named_node(&mut g, 4610, "clang::BuiltinType::OCLEvent"),
      add_named_node(&mut g, 4611, "clang::BuiltinType::OCLClkEvent"),
      add_named_node(&mut g, 4612, "clang::BuiltinType::OCLQueue"),
      add_named_node(&mut g, 4613, "clang::BuiltinType::OCLReserveID"),
      add_named_node(&mut g, 4614, "clang::BuiltinType::Dependent"),
      add_named_node(&mut g, 4615, "clang::BuiltinType::Overload"),
      add_named_node(&mut g, 4616, "clang::BuiltinType::BoundMember"),
      add_named_node(&mut g, 4617, "clang::BuiltinType::PseudoObject"),
      add_named_node(&mut g, 4618, "clang::BuiltinType::UnknownAny"),
      add_named_node(&mut g, 4619, "clang::BuiltinType::BuiltinFn"),
      add_named_node(&mut g, 4620, "clang::BuiltinType::ARCUnbridgedCast"),
      add_named_node(&mut g, 4621, "clang::BuiltinType::IncompleteMatrixIdx"),
      add_named_node(&mut g, 4622, "clang::BuiltinType::OMPArraySection"),
      add_named_node(&mut g, 4623, "clang::BuiltinType::OMPArrayShaping"),
      add_named_node(&mut g, 4624, "clang::BuiltinType::OMPIterator"),
      add_named_node(&mut g, 4625, "clang::BuiltinType::LastKind"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_KIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_KIND_1, "clang::CFGTerminator::Kind");
  g.add_edge((ENUM_KIND_1, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4627, "clang::CFGTerminator::StmtBranch"),
      add_named_node(&mut g, 4628, "clang::CFGTerminator::TemporaryDtorsBranch"),
      add_named_node(&mut g, 4629, "clang::CFGTerminator::VirtualBaseBranch"),
      add_named_node(&mut g, 4630, "clang::CFGTerminator::NumKindsMinusOne"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_KIND_1, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_CXXCONSTRUCTIONKIND, "clang::CXXConstructionKind");
  g.add_edge((ENUM_CXXCONSTRUCTIONKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4632, "clang::CXXConstructionKind::Complete"),
      add_named_node(&mut g, 4633, "clang::CXXConstructionKind::NonVirtualBase"),
      add_named_node(&mut g, 4634, "clang::CXXConstructionKind::VirtualBase"),
      add_named_node(&mut g, 4635, "clang::CXXConstructionKind::Delegating"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_CXXCONSTRUCTIONKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_CXXNEWINITIALIZATIONSTYLE, "clang::CXXNewInitializationStyle");
  g.add_edge((ENUM_CXXNEWINITIALIZATIONSTYLE, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4637, "clang::CXXNewInitializationStyle::None"),
      add_named_node(&mut g, 4638, "clang::CXXNewInitializationStyle::Parens"),
      add_named_node(&mut g, 4639, "clang::CXXNewInitializationStyle::Braces"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_CXXNEWINITIALIZATIONSTYLE, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_ADLCALLKIND, "clang::CallExpr::ADLCallKind");
  g.add_edge((ENUM_ADLCALLKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4641, "clang::CallExpr::ADLCallKind::NotADL"),
      add_named_node(&mut g, 4642, "clang::CallExpr::ADLCallKind::UsesADL"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_ADLCALLKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_CALLINGCONV, "clang::CallingConv");
  g.add_edge((ENUM_CALLINGCONV, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4644, "clang::CC_C"),
      add_named_node(&mut g, 4645, "clang::CC_X86StdCall"),
      add_named_node(&mut g, 4646, "clang::CC_X86FastCall"),
      add_named_node(&mut g, 4647, "clang::CC_X86ThisCall"),
      add_named_node(&mut g, 4648, "clang::CC_X86VectorCall"),
      add_named_node(&mut g, 4649, "clang::CC_X86Pascal"),
      add_named_node(&mut g, 4650, "clang::CC_Win64"),
      add_named_node(&mut g, 4651, "clang::CC_X86_64SysV"),
      add_named_node(&mut g, 4652, "clang::CC_X86RegCall"),
      add_named_node(&mut g, 4653, "clang::CC_AAPCS"),
      add_named_node(&mut g, 4654, "clang::CC_AAPCS_VFP"),
      add_named_node(&mut g, 4655, "clang::CC_IntelOclBicc"),
      add_named_node(&mut g, 4656, "clang::CC_SpirFunction"),
      add_named_node(&mut g, 4657, "clang::CC_OpenCLKernel"),
      add_named_node(&mut g, 4658, "clang::CC_Swift"),
      add_named_node(&mut g, 4659, "clang::CC_SwiftAsync"),
      add_named_node(&mut g, 4660, "clang::CC_PreserveMost"),
      add_named_node(&mut g, 4661, "clang::CC_PreserveAll"),
      add_named_node(&mut g, 4662, "clang::CC_AArch64VectorCall"),
      add_named_node(&mut g, 4663, "clang::CC_AArch64SVEPCS"),
      add_named_node(&mut g, 4664, "clang::CC_AMDGPUKernelCall"),
      add_named_node(&mut g, 4665, "clang::CC_M68kRTD"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_CALLINGCONV, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_CANTHROWRESULT, "clang::CanThrowResult");
  g.add_edge((ENUM_CANTHROWRESULT, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4667, "clang::CT_Cannot"),
      add_named_node(&mut g, 4668, "clang::CT_Dependent"),
      add_named_node(&mut g, 4669, "clang::CT_Can"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_CANTHROWRESULT, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_CAPTUREDREGIONKIND, "clang::CapturedRegionKind");
  g.add_edge((ENUM_CAPTUREDREGIONKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4671, "clang::CR_Default"),
      add_named_node(&mut g, 4672, "clang::CR_ObjCAtFinally"),
      add_named_node(&mut g, 4673, "clang::CR_OpenMP"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_CAPTUREDREGIONKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_CASTKIND, "clang::CastKind");
  g.add_edge((ENUM_CASTKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4675, "clang::CK_Dependent"),
      add_named_node(&mut g, 4676, "clang::CK_BitCast"),
      add_named_node(&mut g, 4677, "clang::CK_LValueBitCast"),
      add_named_node(&mut g, 4678, "clang::CK_LValueToRValueBitCast"),
      add_named_node(&mut g, 4679, "clang::CK_LValueToRValue"),
      add_named_node(&mut g, 4680, "clang::CK_NoOp"),
      add_named_node(&mut g, 4681, "clang::CK_BaseToDerived"),
      add_named_node(&mut g, 4682, "clang::CK_DerivedToBase"),
      add_named_node(&mut g, 4683, "clang::CK_UncheckedDerivedToBase"),
      add_named_node(&mut g, 4684, "clang::CK_Dynamic"),
      add_named_node(&mut g, 4685, "clang::CK_ToUnion"),
      add_named_node(&mut g, 4686, "clang::CK_ArrayToPointerDecay"),
      add_named_node(&mut g, 4687, "clang::CK_FunctionToPointerDecay"),
      add_named_node(&mut g, 4688, "clang::CK_NullToPointer"),
      add_named_node(&mut g, 4689, "clang::CK_NullToMemberPointer"),
      add_named_node(&mut g, 4690, "clang::CK_BaseToDerivedMemberPointer"),
      add_named_node(&mut g, 4691, "clang::CK_DerivedToBaseMemberPointer"),
      add_named_node(&mut g, 4692, "clang::CK_MemberPointerToBoolean"),
      add_named_node(&mut g, 4693, "clang::CK_ReinterpretMemberPointer"),
      add_named_node(&mut g, 4694, "clang::CK_UserDefinedConversion"),
      add_named_node(&mut g, 4695, "clang::CK_ConstructorConversion"),
      add_named_node(&mut g, 4696, "clang::CK_IntegralToPointer"),
      add_named_node(&mut g, 4697, "clang::CK_PointerToIntegral"),
      add_named_node(&mut g, 4698, "clang::CK_PointerToBoolean"),
      add_named_node(&mut g, 4699, "clang::CK_ToVoid"),
      add_named_node(&mut g, 4700, "clang::CK_MatrixCast"),
      add_named_node(&mut g, 4701, "clang::CK_VectorSplat"),
      add_named_node(&mut g, 4702, "clang::CK_IntegralCast"),
      add_named_node(&mut g, 4703, "clang::CK_IntegralToBoolean"),
      add_named_node(&mut g, 4704, "clang::CK_IntegralToFloating"),
      add_named_node(&mut g, 4705, "clang::CK_FloatingToFixedPoint"),
      add_named_node(&mut g, 4706, "clang::CK_FixedPointToFloating"),
      add_named_node(&mut g, 4707, "clang::CK_FixedPointCast"),
      add_named_node(&mut g, 4708, "clang::CK_FixedPointToIntegral"),
      add_named_node(&mut g, 4709, "clang::CK_IntegralToFixedPoint"),
      add_named_node(&mut g, 4710, "clang::CK_FixedPointToBoolean"),
      add_named_node(&mut g, 4711, "clang::CK_FloatingToIntegral"),
      add_named_node(&mut g, 4712, "clang::CK_FloatingToBoolean"),
      add_named_node(&mut g, 4713, "clang::CK_BooleanToSignedIntegral"),
      add_named_node(&mut g, 4714, "clang::CK_FloatingCast"),
      add_named_node(&mut g, 4715, "clang::CK_CPointerToObjCPointerCast"),
      add_named_node(&mut g, 4716, "clang::CK_BlockPointerToObjCPointerCast"),
      add_named_node(&mut g, 4717, "clang::CK_AnyPointerToBlockPointerCast"),
      add_named_node(&mut g, 4718, "clang::CK_ObjCObjectLValueCast"),
      add_named_node(&mut g, 4719, "clang::CK_FloatingRealToComplex"),
      add_named_node(&mut g, 4720, "clang::CK_FloatingComplexToReal"),
      add_named_node(&mut g, 4721, "clang::CK_FloatingComplexToBoolean"),
      add_named_node(&mut g, 4722, "clang::CK_FloatingComplexCast"),
      add_named_node(&mut g, 4723, "clang::CK_FloatingComplexToIntegralComplex"),
      add_named_node(&mut g, 4724, "clang::CK_IntegralRealToComplex"),
      add_named_node(&mut g, 4725, "clang::CK_IntegralComplexToReal"),
      add_named_node(&mut g, 4726, "clang::CK_IntegralComplexToBoolean"),
      add_named_node(&mut g, 4727, "clang::CK_IntegralComplexCast"),
      add_named_node(&mut g, 4728, "clang::CK_IntegralComplexToFloatingComplex"),
      add_named_node(&mut g, 4729, "clang::CK_ARCProduceObject"),
      add_named_node(&mut g, 4730, "clang::CK_ARCConsumeObject"),
      add_named_node(&mut g, 4731, "clang::CK_ARCReclaimReturnedObject"),
      add_named_node(&mut g, 4732, "clang::CK_ARCExtendBlockObject"),
      add_named_node(&mut g, 4733, "clang::CK_AtomicToNonAtomic"),
      add_named_node(&mut g, 4734, "clang::CK_NonAtomicToAtomic"),
      add_named_node(&mut g, 4735, "clang::CK_CopyAndAutoreleaseBlockObject"),
      add_named_node(&mut g, 4736, "clang::CK_BuiltinFnToFnPtr"),
      add_named_node(&mut g, 4737, "clang::CK_ZeroToOCLOpaqueType"),
      add_named_node(&mut g, 4738, "clang::CK_AddressSpaceConversion"),
      add_named_node(&mut g, 4739, "clang::CK_IntToOCLSampler"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_CASTKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_CHARACTERLITERALKIND, "clang::CharacterLiteralKind");
  g.add_edge((ENUM_CHARACTERLITERALKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4741, "clang::CharacterLiteralKind::Ascii"),
      add_named_node(&mut g, 4742, "clang::CharacterLiteralKind::Wide"),
      add_named_node(&mut g, 4743, "clang::CharacterLiteralKind::UTF8"),
      add_named_node(&mut g, 4744, "clang::CharacterLiteralKind::UTF16"),
      add_named_node(&mut g, 4745, "clang::CharacterLiteralKind::UTF32"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_CHARACTERLITERALKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_CONSTANTRESULTSTORAGEKIND, "clang::ConstantResultStorageKind");
  g.add_edge((ENUM_CONSTANTRESULTSTORAGEKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4747, "clang::ConstantResultStorageKind::None"),
      add_named_node(&mut g, 4748, "clang::ConstantResultStorageKind::Int64"),
      add_named_node(&mut g, 4749, "clang::ConstantResultStorageKind::APValue"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_CONSTANTRESULTSTORAGEKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_CONSTEXPRSPECKIND, "clang::ConstexprSpecKind");
  g.add_edge((ENUM_CONSTEXPRSPECKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4751, "clang::ConstexprSpecKind::Unspecified"),
      add_named_node(&mut g, 4752, "clang::ConstexprSpecKind::Constexpr"),
      add_named_node(&mut g, 4753, "clang::ConstexprSpecKind::Consteval"),
      add_named_node(&mut g, 4754, "clang::ConstexprSpecKind::Constinit"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_CONSTEXPRSPECKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_FRIENDOBJECTKIND, "clang::Decl::FriendObjectKind");
  g.add_edge((ENUM_FRIENDOBJECTKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4756, "clang::Decl::FOK_None"),
      add_named_node(&mut g, 4757, "clang::Decl::FOK_Declared"),
      add_named_node(&mut g, 4758, "clang::Decl::FOK_Undeclared"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_FRIENDOBJECTKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_MODULEOWNERSHIPKIND, "clang::Decl::ModuleOwnershipKind");
  g.add_edge((ENUM_MODULEOWNERSHIPKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4760, "clang::Decl::ModuleOwnershipKind::Unowned"),
      add_named_node(&mut g, 4761, "clang::Decl::ModuleOwnershipKind::Visible"),
      add_named_node(&mut g, 4762, "clang::Decl::ModuleOwnershipKind::VisibleWhenImported"),
      add_named_node(&mut g, 4763, "clang::Decl::ModuleOwnershipKind::ReachableWhenImported"),
      add_named_node(&mut g, 4764, "clang::Decl::ModuleOwnershipKind::ModulePrivate"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_MODULEOWNERSHIPKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_OBJCDECLQUALIFIER, "clang::Decl::ObjCDeclQualifier");
  g.add_edge((ENUM_OBJCDECLQUALIFIER, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4766, "clang::Decl::OBJC_TQ_None"),
      add_named_node(&mut g, 4767, "clang::Decl::OBJC_TQ_In"),
      add_named_node(&mut g, 4768, "clang::Decl::OBJC_TQ_Inout"),
      add_named_node(&mut g, 4769, "clang::Decl::OBJC_TQ_Out"),
      add_named_node(&mut g, 4770, "clang::Decl::OBJC_TQ_Bycopy"),
      add_named_node(&mut g, 4771, "clang::Decl::OBJC_TQ_Byref"),
      add_named_node(&mut g, 4772, "clang::Decl::OBJC_TQ_Oneway"),
      add_named_node(&mut g, 4773, "clang::Decl::OBJC_TQ_CSNullability"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_OBJCDECLQUALIFIER, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_DEDUCTIONCANDIDATE, "clang::DeductionCandidate");
  g.add_edge((ENUM_DEDUCTIONCANDIDATE, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4775, "clang::DeductionCandidate::Normal"),
      add_named_node(&mut g, 4776, "clang::DeductionCandidate::Copy"),
      add_named_node(&mut g, 4777, "clang::DeductionCandidate::Aggregate"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_DEDUCTIONCANDIDATE, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_ELABORATEDTYPEKEYWORD, "clang::ElaboratedTypeKeyword");
  g.add_edge((ENUM_ELABORATEDTYPEKEYWORD, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4779, "clang::ElaboratedTypeKeyword::Struct"),
      add_named_node(&mut g, 4780, "clang::ElaboratedTypeKeyword::Interface"),
      add_named_node(&mut g, 4781, "clang::ElaboratedTypeKeyword::Union"),
      add_named_node(&mut g, 4782, "clang::ElaboratedTypeKeyword::Class"),
      add_named_node(&mut g, 4783, "clang::ElaboratedTypeKeyword::Enum"),
      add_named_node(&mut g, 4784, "clang::ElaboratedTypeKeyword::Typename"),
      add_named_node(&mut g, 4785, "clang::ElaboratedTypeKeyword::None"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_ELABORATEDTYPEKEYWORD, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_EXCEPTIONSPECIFICATIONTYPE, "clang::ExceptionSpecificationType");
  g.add_edge((ENUM_EXCEPTIONSPECIFICATIONTYPE, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4787, "clang::EST_None"),
      add_named_node(&mut g, 4788, "clang::EST_DynamicNone"),
      add_named_node(&mut g, 4789, "clang::EST_Dynamic"),
      add_named_node(&mut g, 4790, "clang::EST_MSAny"),
      add_named_node(&mut g, 4791, "clang::EST_NoThrow"),
      add_named_node(&mut g, 4792, "clang::EST_BasicNoexcept"),
      add_named_node(&mut g, 4793, "clang::EST_DependentNoexcept"),
      add_named_node(&mut g, 4794, "clang::EST_NoexceptFalse"),
      add_named_node(&mut g, 4795, "clang::EST_NoexceptTrue"),
      add_named_node(&mut g, 4796, "clang::EST_Unevaluated"),
      add_named_node(&mut g, 4797, "clang::EST_Uninstantiated"),
      add_named_node(&mut g, 4798, "clang::EST_Unparsed"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_EXCEPTIONSPECIFICATIONTYPE, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_EXPRDEPENDENCE, "clang::ExprDependenceScope::ExprDependence");
  g.add_edge((ENUM_EXPRDEPENDENCE, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4800, "clang::ExprDependenceScope::UnexpandedPack"),
      add_named_node(&mut g, 4801, "clang::ExprDependenceScope::Instantiation"),
      add_named_node(&mut g, 4802, "clang::ExprDependenceScope::Type"),
      add_named_node(&mut g, 4803, "clang::ExprDependenceScope::Value"),
      add_named_node(&mut g, 4804, "clang::ExprDependenceScope::Error"),
      add_named_node(&mut g, 4805, "clang::ExprDependenceScope::None"),
      add_named_node(&mut g, 4806, "clang::ExprDependenceScope::All"),
      add_named_node(&mut g, 4807, "clang::ExprDependenceScope::TypeValue"),
      add_named_node(&mut g, 4808, "clang::ExprDependenceScope::TypeInstantiation"),
      add_named_node(&mut g, 4809, "clang::ExprDependenceScope::ValueInstantiation"),
      add_named_node(&mut g, 4810, "clang::ExprDependenceScope::TypeValueInstantiation"),
      add_named_node(&mut g, 4811, "clang::ExprDependenceScope::ErrorDependent"),
      add_named_node(&mut g, 4812, "clang::ExprDependenceScope::LLVM_BITMASK_LARGEST_ENUMERATOR"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_EXPRDEPENDENCE, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_EXPROBJECTKIND, "clang::ExprObjectKind");
  g.add_edge((ENUM_EXPROBJECTKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4814, "clang::OK_Ordinary"),
      add_named_node(&mut g, 4815, "clang::OK_BitField"),
      add_named_node(&mut g, 4816, "clang::OK_VectorComponent"),
      add_named_node(&mut g, 4817, "clang::OK_ObjCProperty"),
      add_named_node(&mut g, 4818, "clang::OK_ObjCSubscript"),
      add_named_node(&mut g, 4819, "clang::OK_MatrixComponent"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_EXPROBJECTKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_EXPRVALUEKIND, "clang::ExprValueKind");
  g.add_edge((ENUM_EXPRVALUEKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4821, "clang::VK_PRValue"),
      add_named_node(&mut g, 4822, "clang::VK_LValue"),
      add_named_node(&mut g, 4823, "clang::VK_XValue"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_EXPRVALUEKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_EXPRESSIONTRAIT, "clang::ExpressionTrait");
  g.add_edge((ENUM_EXPRESSIONTRAIT, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4825, "clang::ET_IsLValueExpr"),
      add_named_node(&mut g, 4826, "clang::ET_IsRValueExpr"),
      add_named_node(&mut g, 4827, "clang::ET_Last"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_EXPRESSIONTRAIT, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_TEMPLATEDKIND, "clang::FunctionDecl::TemplatedKind");
  g.add_edge((ENUM_TEMPLATEDKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4829, "clang::FunctionDecl::TK_NonTemplate"),
      add_named_node(&mut g, 4830, "clang::FunctionDecl::TK_FunctionTemplate"),
      add_named_node(&mut g, 4831, "clang::FunctionDecl::TK_MemberSpecialization"),
      add_named_node(&mut g, 4832, "clang::FunctionDecl::TK_FunctionTemplateSpecialization"),
      add_named_node(&mut g, 4833, "clang::FunctionDecl::TK_DependentFunctionTemplateSpecialization"),
      add_named_node(&mut g, 4834, "clang::FunctionDecl::TK_DependentNonTemplate"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_TEMPLATEDKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_IFSTATEMENTKIND, "clang::IfStatementKind");
  g.add_edge((ENUM_IFSTATEMENTKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4836, "clang::IfStatementKind::Ordinary"),
      add_named_node(&mut g, 4837, "clang::IfStatementKind::Constexpr"),
      add_named_node(&mut g, 4838, "clang::IfStatementKind::ConstevalNonNegated"),
      add_named_node(&mut g, 4839, "clang::IfStatementKind::ConstevalNegated"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_IFSTATEMENTKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_IMPLICITPARAMKIND, "clang::ImplicitParamKind");
  g.add_edge((ENUM_IMPLICITPARAMKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4841, "clang::ImplicitParamKind::ObjCSelf"),
      add_named_node(&mut g, 4842, "clang::ImplicitParamKind::ObjCCmd"),
      add_named_node(&mut g, 4843, "clang::ImplicitParamKind::CXXThis"),
      add_named_node(&mut g, 4844, "clang::ImplicitParamKind::CXXVTT"),
      add_named_node(&mut g, 4845, "clang::ImplicitParamKind::CapturedContext"),
      add_named_node(&mut g, 4846, "clang::ImplicitParamKind::ThreadPrivateVar"),
      add_named_node(&mut g, 4847, "clang::ImplicitParamKind::Other"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_IMPLICITPARAMKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_INCLASSINITSTYLE, "clang::InClassInitStyle");
  g.add_edge((ENUM_INCLASSINITSTYLE, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4849, "clang::ICIS_NoInit"),
      add_named_node(&mut g, 4850, "clang::ICIS_CopyInit"),
      add_named_node(&mut g, 4851, "clang::ICIS_ListInit"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_INCLASSINITSTYLE, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_LAMBDACAPTUREDEFAULT, "clang::LambdaCaptureDefault");
  g.add_edge((ENUM_LAMBDACAPTUREDEFAULT, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4853, "clang::LCD_None"),
      add_named_node(&mut g, 4854, "clang::LCD_ByCopy"),
      add_named_node(&mut g, 4855, "clang::LCD_ByRef"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_LAMBDACAPTUREDEFAULT, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_LANGUAGELINKAGE, "clang::LanguageLinkage");
  g.add_edge((ENUM_LANGUAGELINKAGE, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4857, "clang::CLanguageLinkage"),
      add_named_node(&mut g, 4858, "clang::CXXLanguageLinkage"),
      add_named_node(&mut g, 4859, "clang::NoLanguageLinkage"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_LANGUAGELINKAGE, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_LINKAGE, "clang::Linkage");
  g.add_edge((ENUM_LINKAGE, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4861, "clang::Linkage::Invalid"),
      add_named_node(&mut g, 4862, "clang::Linkage::None"),
      add_named_node(&mut g, 4863, "clang::Linkage::Internal"),
      add_named_node(&mut g, 4864, "clang::Linkage::UniqueExternal"),
      add_named_node(&mut g, 4865, "clang::Linkage::VisibleNone"),
      add_named_node(&mut g, 4866, "clang::Linkage::Module"),
      add_named_node(&mut g, 4867, "clang::Linkage::External"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_LINKAGE, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_LINKAGESPECLANGUAGEIDS, "clang::LinkageSpecLanguageIDs");
  g.add_edge((ENUM_LINKAGESPECLANGUAGEIDS, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4869, "clang::LinkageSpecLanguageIDs::C"),
      add_named_node(&mut g, 4870, "clang::LinkageSpecLanguageIDs::CXX"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_LINKAGESPECLANGUAGEIDS, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_MSVTORDISPMODE, "clang::MSVtorDispMode");
  g.add_edge((ENUM_MSVTORDISPMODE, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4872, "clang::MSVtorDispMode::Never"),
      add_named_node(&mut g, 4873, "clang::MSVtorDispMode::ForVBaseOverride"),
      add_named_node(&mut g, 4874, "clang::MSVtorDispMode::ForVFTable"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_MSVTORDISPMODE, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_MULTIVERSIONKIND, "clang::MultiVersionKind");
  g.add_edge((ENUM_MULTIVERSIONKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4876, "clang::MultiVersionKind::None"),
      add_named_node(&mut g, 4877, "clang::MultiVersionKind::Target"),
      add_named_node(&mut g, 4878, "clang::MultiVersionKind::CPUSpecific"),
      add_named_node(&mut g, 4879, "clang::MultiVersionKind::CPUDispatch"),
      add_named_node(&mut g, 4880, "clang::MultiVersionKind::TargetClones"),
      add_named_node(&mut g, 4881, "clang::MultiVersionKind::TargetVersion"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_MULTIVERSIONKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_NONODRUSEREASON, "clang::NonOdrUseReason");
  g.add_edge((ENUM_NONODRUSEREASON, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4883, "clang::NOUR_None"),
      add_named_node(&mut g, 4884, "clang::NOUR_Unevaluated"),
      add_named_node(&mut g, 4885, "clang::NOUR_Constant"),
      add_named_node(&mut g, 4886, "clang::NOUR_Discarded"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_NONODRUSEREASON, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_OBJCSTRINGFORMATFAMILY, "clang::ObjCStringFormatFamily");
  g.add_edge((ENUM_OBJCSTRINGFORMATFAMILY, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4888, "clang::SFF_None"),
      add_named_node(&mut g, 4889, "clang::SFF_NSString"),
      add_named_node(&mut g, 4890, "clang::SFF_CFString"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_OBJCSTRINGFORMATFAMILY, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_OVERLOADEDOPERATORKIND, "clang::OverloadedOperatorKind");
  g.add_edge((ENUM_OVERLOADEDOPERATORKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4892, "clang::OO_None"),
      add_named_node(&mut g, 4893, "clang::OO_New"),
      add_named_node(&mut g, 4894, "clang::OO_Delete"),
      add_named_node(&mut g, 4895, "clang::OO_Array_New"),
      add_named_node(&mut g, 4896, "clang::OO_Array_Delete"),
      add_named_node(&mut g, 4897, "clang::OO_Plus"),
      add_named_node(&mut g, 4898, "clang::OO_Minus"),
      add_named_node(&mut g, 4899, "clang::OO_Star"),
      add_named_node(&mut g, 4900, "clang::OO_Slash"),
      add_named_node(&mut g, 4901, "clang::OO_Percent"),
      add_named_node(&mut g, 4902, "clang::OO_Caret"),
      add_named_node(&mut g, 4903, "clang::OO_Amp"),
      add_named_node(&mut g, 4904, "clang::OO_Pipe"),
      add_named_node(&mut g, 4905, "clang::OO_Tilde"),
      add_named_node(&mut g, 4906, "clang::OO_Exclaim"),
      add_named_node(&mut g, 4907, "clang::OO_Equal"),
      add_named_node(&mut g, 4908, "clang::OO_Less"),
      add_named_node(&mut g, 4909, "clang::OO_Greater"),
      add_named_node(&mut g, 4910, "clang::OO_PlusEqual"),
      add_named_node(&mut g, 4911, "clang::OO_MinusEqual"),
      add_named_node(&mut g, 4912, "clang::OO_StarEqual"),
      add_named_node(&mut g, 4913, "clang::OO_SlashEqual"),
      add_named_node(&mut g, 4914, "clang::OO_PercentEqual"),
      add_named_node(&mut g, 4915, "clang::OO_CaretEqual"),
      add_named_node(&mut g, 4916, "clang::OO_AmpEqual"),
      add_named_node(&mut g, 4917, "clang::OO_PipeEqual"),
      add_named_node(&mut g, 4918, "clang::OO_LessLess"),
      add_named_node(&mut g, 4919, "clang::OO_GreaterGreater"),
      add_named_node(&mut g, 4920, "clang::OO_LessLessEqual"),
      add_named_node(&mut g, 4921, "clang::OO_GreaterGreaterEqual"),
      add_named_node(&mut g, 4922, "clang::OO_EqualEqual"),
      add_named_node(&mut g, 4923, "clang::OO_ExclaimEqual"),
      add_named_node(&mut g, 4924, "clang::OO_LessEqual"),
      add_named_node(&mut g, 4925, "clang::OO_GreaterEqual"),
      add_named_node(&mut g, 4926, "clang::OO_Spaceship"),
      add_named_node(&mut g, 4927, "clang::OO_AmpAmp"),
      add_named_node(&mut g, 4928, "clang::OO_PipePipe"),
      add_named_node(&mut g, 4929, "clang::OO_PlusPlus"),
      add_named_node(&mut g, 4930, "clang::OO_MinusMinus"),
      add_named_node(&mut g, 4931, "clang::OO_Comma"),
      add_named_node(&mut g, 4932, "clang::OO_ArrowStar"),
      add_named_node(&mut g, 4933, "clang::OO_Arrow"),
      add_named_node(&mut g, 4934, "clang::OO_Call"),
      add_named_node(&mut g, 4935, "clang::OO_Subscript"),
      add_named_node(&mut g, 4936, "clang::OO_Conditional"),
      add_named_node(&mut g, 4937, "clang::OO_Coawait"),
      add_named_node(&mut g, 4938, "clang::NUM_OVERLOADED_OPERATORS"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_OVERLOADEDOPERATORKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_PRAGMAMSCOMMENTKIND, "clang::PragmaMSCommentKind");
  g.add_edge((ENUM_PRAGMAMSCOMMENTKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4940, "clang::PCK_Unknown"),
      add_named_node(&mut g, 4941, "clang::PCK_Linker"),
      add_named_node(&mut g, 4942, "clang::PCK_Lib"),
      add_named_node(&mut g, 4943, "clang::PCK_Compiler"),
      add_named_node(&mut g, 4944, "clang::PCK_ExeStr"),
      add_named_node(&mut g, 4945, "clang::PCK_User"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_PRAGMAMSCOMMENTKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_PREDEFINEDIDENTKIND, "clang::PredefinedIdentKind");
  g.add_edge((ENUM_PREDEFINEDIDENTKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4947, "clang::PredefinedIdentKind::Func"),
      add_named_node(&mut g, 4948, "clang::PredefinedIdentKind::Function"),
      add_named_node(&mut g, 4949, "clang::PredefinedIdentKind::LFunction"),
      add_named_node(&mut g, 4950, "clang::PredefinedIdentKind::FuncDName"),
      add_named_node(&mut g, 4951, "clang::PredefinedIdentKind::FuncSig"),
      add_named_node(&mut g, 4952, "clang::PredefinedIdentKind::LFuncSig"),
      add_named_node(&mut g, 4953, "clang::PredefinedIdentKind::PrettyFunction"),
      add_named_node(&mut g, 4954, "clang::PredefinedIdentKind::PrettyFunctionNoVirtual"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_PREDEFINEDIDENTKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_OBJCLIFETIME, "clang::Qualifiers::ObjCLifetime");
  g.add_edge((ENUM_OBJCLIFETIME, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4956, "clang::Qualifiers::OCL_None"),
      add_named_node(&mut g, 4957, "clang::Qualifiers::OCL_ExplicitNone"),
      add_named_node(&mut g, 4958, "clang::Qualifiers::OCL_Strong"),
      add_named_node(&mut g, 4959, "clang::Qualifiers::OCL_Weak"),
      add_named_node(&mut g, 4960, "clang::Qualifiers::OCL_Autoreleasing"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_OBJCLIFETIME, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_RECORDARGPASSINGKIND, "clang::RecordArgPassingKind");
  g.add_edge((ENUM_RECORDARGPASSINGKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4962, "clang::RecordArgPassingKind::CanPassInRegs"),
      add_named_node(&mut g, 4963, "clang::RecordArgPassingKind::CannotPassInRegs"),
      add_named_node(&mut g, 4964, "clang::RecordArgPassingKind::CanNeverPassInRegs"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_RECORDARGPASSINGKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_REFQUALIFIERKIND, "clang::RefQualifierKind");
  g.add_edge((ENUM_REFQUALIFIERKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4966, "clang::RQ_None"),
      add_named_node(&mut g, 4967, "clang::RQ_LValue"),
      add_named_node(&mut g, 4968, "clang::RQ_RValue"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_REFQUALIFIERKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_SOURCELOCIDENTKIND, "clang::SourceLocIdentKind");
  g.add_edge((ENUM_SOURCELOCIDENTKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4970, "clang::SourceLocIdentKind::Function"),
      add_named_node(&mut g, 4971, "clang::SourceLocIdentKind::FuncSig"),
      add_named_node(&mut g, 4972, "clang::SourceLocIdentKind::File"),
      add_named_node(&mut g, 4973, "clang::SourceLocIdentKind::FileName"),
      add_named_node(&mut g, 4974, "clang::SourceLocIdentKind::Line"),
      add_named_node(&mut g, 4975, "clang::SourceLocIdentKind::Column"),
      add_named_node(&mut g, 4976, "clang::SourceLocIdentKind::SourceLocStruct"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_SOURCELOCIDENTKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_STORAGECLASS, "clang::StorageClass");
  g.add_edge((ENUM_STORAGECLASS, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4978, "clang::SC_None"),
      add_named_node(&mut g, 4979, "clang::SC_Extern"),
      add_named_node(&mut g, 4980, "clang::SC_Static"),
      add_named_node(&mut g, 4981, "clang::SC_PrivateExtern"),
      add_named_node(&mut g, 4982, "clang::SC_Auto"),
      add_named_node(&mut g, 4983, "clang::SC_Register"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_STORAGECLASS, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_STORAGEDURATION, "clang::StorageDuration");
  g.add_edge((ENUM_STORAGEDURATION, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4985, "clang::SD_FullExpression"),
      add_named_node(&mut g, 4986, "clang::SD_Automatic"),
      add_named_node(&mut g, 4987, "clang::SD_Thread"),
      add_named_node(&mut g, 4988, "clang::SD_Static"),
      add_named_node(&mut g, 4989, "clang::SD_Dynamic"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_STORAGEDURATION, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_STRINGLITERALKIND, "clang::StringLiteralKind");
  g.add_edge((ENUM_STRINGLITERALKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4991, "clang::StringLiteralKind::Ordinary"),
      add_named_node(&mut g, 4992, "clang::StringLiteralKind::Wide"),
      add_named_node(&mut g, 4993, "clang::StringLiteralKind::UTF8"),
      add_named_node(&mut g, 4994, "clang::StringLiteralKind::UTF16"),
      add_named_node(&mut g, 4995, "clang::StringLiteralKind::UTF32"),
      add_named_node(&mut g, 4996, "clang::StringLiteralKind::Unevaluated"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_STRINGLITERALKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_TAGTYPEKIND, "clang::TagTypeKind");
  g.add_edge((ENUM_TAGTYPEKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4998, "clang::TagTypeKind::Struct"),
      add_named_node(&mut g, 4999, "clang::TagTypeKind::Interface"),
      add_named_node(&mut g, 5000, "clang::TagTypeKind::Union"),
      add_named_node(&mut g, 5001, "clang::TagTypeKind::Class"),
      add_named_node(&mut g, 5002, "clang::TagTypeKind::Enum"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_TAGTYPEKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_TEMPLATESPECIALIZATIONKIND, "clang::TemplateSpecializationKind");
  g.add_edge((ENUM_TEMPLATESPECIALIZATIONKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 5004, "clang::TSK_Undeclared"),
      add_named_node(&mut g, 5005, "clang::TSK_ImplicitInstantiation"),
      add_named_node(&mut g, 5006, "clang::TSK_ExplicitSpecialization"),
      add_named_node(&mut g, 5007, "clang::TSK_ExplicitInstantiationDeclaration"),
      add_named_node(&mut g, 5008, "clang::TSK_ExplicitInstantiationDefinition"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_TEMPLATESPECIALIZATIONKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_THREADSTORAGECLASSSPECIFIER, "clang::ThreadStorageClassSpecifier");
  g.add_edge((ENUM_THREADSTORAGECLASSSPECIFIER, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 5010, "clang::TSCS_unspecified"),
      add_named_node(&mut g, 5011, "clang::TSCS___thread"),
      add_named_node(&mut g, 5012, "clang::TSCS_thread_local"),
      add_named_node(&mut g, 5013, "clang::TSCS__Thread_local"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_THREADSTORAGECLASSSPECIFIER, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_TYPEDEPENDENCE, "clang::TypeDependenceScope::TypeDependence");
  g.add_edge((ENUM_TYPEDEPENDENCE, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 5015, "clang::TypeDependenceScope::UnexpandedPack"),
      add_named_node(&mut g, 5016, "clang::TypeDependenceScope::Instantiation"),
      add_named_node(&mut g, 5017, "clang::TypeDependenceScope::Dependent"),
      add_named_node(&mut g, 5018, "clang::TypeDependenceScope::VariablyModified"),
      add_named_node(&mut g, 5019, "clang::TypeDependenceScope::Error"),
      add_named_node(&mut g, 5020, "clang::TypeDependenceScope::None"),
      add_named_node(&mut g, 5021, "clang::TypeDependenceScope::All"),
      add_named_node(&mut g, 5022, "clang::TypeDependenceScope::DependentInstantiation"),
      add_named_node(&mut g, 5023, "clang::TypeDependenceScope::LLVM_BITMASK_LARGEST_ENUMERATOR"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_TYPEDEPENDENCE, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_TYPEOFKIND, "clang::TypeOfKind");
  g.add_edge((ENUM_TYPEOFKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 5025, "clang::TypeOfKind::Qualified"),
      add_named_node(&mut g, 5026, "clang::TypeOfKind::Unqualified"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_TYPEOFKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_TYPETRAIT, "clang::TypeTrait");
  g.add_edge((ENUM_TYPETRAIT, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 5028, "clang::UTT_IsInterfaceClass"),
      add_named_node(&mut g, 5029, "clang::UTT_IsSealed"),
      add_named_node(&mut g, 5030, "clang::UTT_IsDestructible"),
      add_named_node(&mut g, 5031, "clang::UTT_IsTriviallyDestructible"),
      add_named_node(&mut g, 5032, "clang::UTT_IsNothrowDestructible"),
      add_named_node(&mut g, 5033, "clang::UTT_HasNothrowMoveAssign"),
      add_named_node(&mut g, 5034, "clang::UTT_HasTrivialMoveAssign"),
      add_named_node(&mut g, 5035, "clang::UTT_HasTrivialMoveConstructor"),
      add_named_node(&mut g, 5036, "clang::UTT_HasNothrowAssign"),
      add_named_node(&mut g, 5037, "clang::UTT_HasNothrowCopy"),
      add_named_node(&mut g, 5038, "clang::UTT_HasNothrowConstructor"),
      add_named_node(&mut g, 5039, "clang::UTT_HasTrivialAssign"),
      add_named_node(&mut g, 5040, "clang::UTT_HasTrivialCopy"),
      add_named_node(&mut g, 5041, "clang::UTT_HasTrivialDefaultConstructor"),
      add_named_node(&mut g, 5042, "clang::UTT_HasTrivialDestructor"),
      add_named_node(&mut g, 5043, "clang::UTT_HasVirtualDestructor"),
      add_named_node(&mut g, 5044, "clang::UTT_IsAbstract"),
      add_named_node(&mut g, 5045, "clang::UTT_IsAggregate"),
      add_named_node(&mut g, 5046, "clang::UTT_IsClass"),
      add_named_node(&mut g, 5047, "clang::UTT_IsEmpty"),
      add_named_node(&mut g, 5048, "clang::UTT_IsEnum"),
      add_named_node(&mut g, 5049, "clang::UTT_IsFinal"),
      add_named_node(&mut g, 5050, "clang::UTT_IsLiteral"),
      add_named_node(&mut g, 5051, "clang::UTT_IsPOD"),
      add_named_node(&mut g, 5052, "clang::UTT_IsPolymorphic"),
      add_named_node(&mut g, 5053, "clang::UTT_IsStandardLayout"),
      add_named_node(&mut g, 5054, "clang::UTT_IsTrivial"),
      add_named_node(&mut g, 5055, "clang::UTT_IsTriviallyCopyable"),
      add_named_node(&mut g, 5056, "clang::UTT_IsUnion"),
      add_named_node(&mut g, 5057, "clang::UTT_HasUniqueObjectRepresentations"),
      add_named_node(&mut g, 5058, "clang::UTT_IsTriviallyRelocatable"),
      add_named_node(&mut g, 5059, "clang::UTT_IsTriviallyEqualityComparable"),
      add_named_node(&mut g, 5060, "clang::UTT_IsBoundedArray"),
      add_named_node(&mut g, 5061, "clang::UTT_IsUnboundedArray"),
      add_named_node(&mut g, 5062, "clang::UTT_IsNullPointer"),
      add_named_node(&mut g, 5063, "clang::UTT_IsScopedEnum"),
      add_named_node(&mut g, 5064, "clang::UTT_IsReferenceable"),
      add_named_node(&mut g, 5065, "clang::UTT_CanPassInRegs"),
      add_named_node(&mut g, 5066, "clang::UTT_IsArithmetic"),
      add_named_node(&mut g, 5067, "clang::UTT_IsFloatingPoint"),
      add_named_node(&mut g, 5068, "clang::UTT_IsIntegral"),
      add_named_node(&mut g, 5069, "clang::UTT_IsCompleteType"),
      add_named_node(&mut g, 5070, "clang::UTT_IsVoid"),
      add_named_node(&mut g, 5071, "clang::UTT_IsArray"),
      add_named_node(&mut g, 5072, "clang::UTT_IsFunction"),
      add_named_node(&mut g, 5073, "clang::UTT_IsReference"),
      add_named_node(&mut g, 5074, "clang::UTT_IsLvalueReference"),
      add_named_node(&mut g, 5075, "clang::UTT_IsRvalueReference"),
      add_named_node(&mut g, 5076, "clang::UTT_IsFundamental"),
      add_named_node(&mut g, 5077, "clang::UTT_IsObject"),
      add_named_node(&mut g, 5078, "clang::UTT_IsScalar"),
      add_named_node(&mut g, 5079, "clang::UTT_IsCompound"),
      add_named_node(&mut g, 5080, "clang::UTT_IsPointer"),
      add_named_node(&mut g, 5081, "clang::UTT_IsMemberObjectPointer"),
      add_named_node(&mut g, 5082, "clang::UTT_IsMemberFunctionPointer"),
      add_named_node(&mut g, 5083, "clang::UTT_IsMemberPointer"),
      add_named_node(&mut g, 5084, "clang::UTT_IsConst"),
      add_named_node(&mut g, 5085, "clang::UTT_IsVolatile"),
      add_named_node(&mut g, 5086, "clang::UTT_IsSigned"),
      add_named_node(&mut g, 5087, "clang::UTT_IsUnsigned"),
      add_named_node(&mut g, 5088, "clang::UTT_Last"),
      add_named_node(&mut g, 5089, "clang::BTT_TypeCompatible"),
      add_named_node(&mut g, 5090, "clang::BTT_IsNothrowAssignable"),
      add_named_node(&mut g, 5091, "clang::BTT_IsAssignable"),
      add_named_node(&mut g, 5092, "clang::BTT_IsBaseOf"),
      add_named_node(&mut g, 5093, "clang::BTT_IsConvertibleTo"),
      add_named_node(&mut g, 5094, "clang::BTT_IsTriviallyAssignable"),
      add_named_node(&mut g, 5095, "clang::BTT_ReferenceBindsToTemporary"),
      add_named_node(&mut g, 5096, "clang::BTT_ReferenceConstructsFromTemporary"),
      add_named_node(&mut g, 5097, "clang::BTT_IsSame"),
      add_named_node(&mut g, 5098, "clang::BTT_IsConvertible"),
      add_named_node(&mut g, 5099, "clang::BTT_Last"),
      add_named_node(&mut g, 5100, "clang::TT_IsConstructible"),
      add_named_node(&mut g, 5101, "clang::TT_IsNothrowConstructible"),
      add_named_node(&mut g, 5102, "clang::TT_IsTriviallyConstructible"),
      add_named_node(&mut g, 5103, "clang::TT_Last"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_TYPETRAIT, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_UNARYEXPRORTYPETRAIT, "clang::UnaryExprOrTypeTrait");
  g.add_edge((ENUM_UNARYEXPRORTYPETRAIT, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 5105, "clang::UETT_SizeOf"),
      add_named_node(&mut g, 5106, "clang::UETT_DataSizeOf"),
      add_named_node(&mut g, 5107, "clang::UETT_AlignOf"),
      add_named_node(&mut g, 5108, "clang::UETT_PreferredAlignOf"),
      add_named_node(&mut g, 5109, "clang::UETT_VecStep"),
      add_named_node(&mut g, 5110, "clang::UETT_OpenMPRequiredSimdAlign"),
      add_named_node(&mut g, 5111, "clang::UETT_VectorElements"),
      add_named_node(&mut g, 5112, "clang::UETT_Last"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_UNARYEXPRORTYPETRAIT, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_UNARYOPERATORKIND, "clang::UnaryOperatorKind");
  g.add_edge((ENUM_UNARYOPERATORKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 5114, "clang::UO_PostInc"),
      add_named_node(&mut g, 5115, "clang::UO_PostDec"),
      add_named_node(&mut g, 5116, "clang::UO_PreInc"),
      add_named_node(&mut g, 5117, "clang::UO_PreDec"),
      add_named_node(&mut g, 5118, "clang::UO_AddrOf"),
      add_named_node(&mut g, 5119, "clang::UO_Deref"),
      add_named_node(&mut g, 5120, "clang::UO_Plus"),
      add_named_node(&mut g, 5121, "clang::UO_Minus"),
      add_named_node(&mut g, 5122, "clang::UO_Not"),
      add_named_node(&mut g, 5123, "clang::UO_LNot"),
      add_named_node(&mut g, 5124, "clang::UO_Real"),
      add_named_node(&mut g, 5125, "clang::UO_Imag"),
      add_named_node(&mut g, 5126, "clang::UO_Extension"),
      add_named_node(&mut g, 5127, "clang::UO_Coawait"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_UNARYOPERATORKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_UTTKIND, "clang::UnaryTransformType::UTTKind");
  g.add_edge((ENUM_UTTKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 5129, "clang::UnaryTransformType::AddLvalueReference"),
      add_named_node(&mut g, 5130, "clang::UnaryTransformType::AddPointer"),
      add_named_node(&mut g, 5131, "clang::UnaryTransformType::AddRvalueReference"),
      add_named_node(&mut g, 5132, "clang::UnaryTransformType::Decay"),
      add_named_node(&mut g, 5133, "clang::UnaryTransformType::MakeSigned"),
      add_named_node(&mut g, 5134, "clang::UnaryTransformType::MakeUnsigned"),
      add_named_node(&mut g, 5135, "clang::UnaryTransformType::RemoveAllExtents"),
      add_named_node(&mut g, 5136, "clang::UnaryTransformType::RemoveConst"),
      add_named_node(&mut g, 5137, "clang::UnaryTransformType::RemoveCV"),
      add_named_node(&mut g, 5138, "clang::UnaryTransformType::RemoveCVRef"),
      add_named_node(&mut g, 5139, "clang::UnaryTransformType::RemoveExtent"),
      add_named_node(&mut g, 5140, "clang::UnaryTransformType::RemovePointer"),
      add_named_node(&mut g, 5141, "clang::UnaryTransformType::RemoveReference"),
      add_named_node(&mut g, 5142, "clang::UnaryTransformType::RemoveRestrict"),
      add_named_node(&mut g, 5143, "clang::UnaryTransformType::RemoveVolatile"),
      add_named_node(&mut g, 5144, "clang::UnaryTransformType::EnumUnderlyingType"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_UTTKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_LITERALOPERATORKIND, "clang::UserDefinedLiteral::LiteralOperatorKind");
  g.add_edge((ENUM_LITERALOPERATORKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 5146, "clang::UserDefinedLiteral::LOK_Raw"),
      add_named_node(&mut g, 5147, "clang::UserDefinedLiteral::LOK_Template"),
      add_named_node(&mut g, 5148, "clang::UserDefinedLiteral::LOK_Integer"),
      add_named_node(&mut g, 5149, "clang::UserDefinedLiteral::LOK_Floating"),
      add_named_node(&mut g, 5150, "clang::UserDefinedLiteral::LOK_String"),
      add_named_node(&mut g, 5151, "clang::UserDefinedLiteral::LOK_Character"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_LITERALOPERATORKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_DEFINITIONKIND, "clang::VarDecl::DefinitionKind");
  g.add_edge((ENUM_DEFINITIONKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 5153, "clang::VarDecl::DeclarationOnly"),
      add_named_node(&mut g, 5154, "clang::VarDecl::TentativeDefinition"),
      add_named_node(&mut g, 5155, "clang::VarDecl::Definition"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_DEFINITIONKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_INITIALIZATIONSTYLE, "clang::VarDecl::InitializationStyle");
  g.add_edge((ENUM_INITIALIZATIONSTYLE, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 5157, "clang::VarDecl::CInit"),
      add_named_node(&mut g, 5158, "clang::VarDecl::CallInit"),
      add_named_node(&mut g, 5159, "clang::VarDecl::ListInit"),
      add_named_node(&mut g, 5160, "clang::VarDecl::ParenListInit"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_INITIALIZATIONSTYLE, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_TLSKIND, "clang::VarDecl::TLSKind");
  g.add_edge((ENUM_TLSKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 5162, "clang::VarDecl::TLS_None"),
      add_named_node(&mut g, 5163, "clang::VarDecl::TLS_Static"),
      add_named_node(&mut g, 5164, "clang::VarDecl::TLS_Dynamic"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_TLSKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_VECTORKIND, "clang::VectorKind");
  g.add_edge((ENUM_VECTORKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 5166, "clang::VectorKind::Generic"),
      add_named_node(&mut g, 5167, "clang::VectorKind::AltiVecVector"),
      add_named_node(&mut g, 5168, "clang::VectorKind::AltiVecPixel"),
      add_named_node(&mut g, 5169, "clang::VectorKind::AltiVecBool"),
      add_named_node(&mut g, 5170, "clang::VectorKind::Neon"),
      add_named_node(&mut g, 5171, "clang::VectorKind::NeonPoly"),
      add_named_node(&mut g, 5172, "clang::VectorKind::SveFixedLengthData"),
      add_named_node(&mut g, 5173, "clang::VectorKind::SveFixedLengthPredicate"),
      add_named_node(&mut g, 5174, "clang::VectorKind::RVVFixedLengthData"),
      add_named_node(&mut g, 5175, "clang::VectorKind::RVVFixedLengthMask"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_VECTORKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_VISIBILITY, "clang::Visibility");
  g.add_edge((ENUM_VISIBILITY, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 5177, "clang::HiddenVisibility"),
      add_named_node(&mut g, 5178, "clang::ProtectedVisibility"),
      add_named_node(&mut g, 5179, "clang::DefaultVisibility"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_VISIBILITY, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_KIND_2, "clang::attr::Kind");
  g.add_edge((ENUM_KIND_2, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 5181, "clang::attr::AddressSpace"),
      add_named_node(&mut g, 5182, "clang::attr::AnnotateType"),
      add_named_node(&mut g, 5183, "clang::attr::ArmIn"),
      add_named_node(&mut g, 5184, "clang::attr::ArmInOut"),
      add_named_node(&mut g, 5185, "clang::attr::ArmMveStrictPolymorphism"),
      add_named_node(&mut g, 5186, "clang::attr::ArmOut"),
      add_named_node(&mut g, 5187, "clang::attr::ArmPreserves"),
      add_named_node(&mut g, 5188, "clang::attr::ArmStreaming"),
      add_named_node(&mut g, 5189, "clang::attr::ArmStreamingCompatible"),
      add_named_node(&mut g, 5190, "clang::attr::BTFTypeTag"),
      add_named_node(&mut g, 5191, "clang::attr::CmseNSCall"),
      add_named_node(&mut g, 5192, "clang::attr::HLSLGroupSharedAddressSpace"),
      add_named_node(&mut g, 5193, "clang::attr::HLSLParamModifier"),
      add_named_node(&mut g, 5194, "clang::attr::NoDeref"),
      add_named_node(&mut g, 5195, "clang::attr::ObjCGC"),
      add_named_node(&mut g, 5196, "clang::attr::ObjCInertUnsafeUnretained"),
      add_named_node(&mut g, 5197, "clang::attr::ObjCKindOf"),
      add_named_node(&mut g, 5198, "clang::attr::OpenCLConstantAddressSpace"),
      add_named_node(&mut g, 5199, "clang::attr::OpenCLGenericAddressSpace"),
      add_named_node(&mut g, 5200, "clang::attr::OpenCLGlobalAddressSpace"),
      add_named_node(&mut g, 5201, "clang::attr::OpenCLGlobalDeviceAddressSpace"),
      add_named_node(&mut g, 5202, "clang::attr::OpenCLGlobalHostAddressSpace"),
      add_named_node(&mut g, 5203, "clang::attr::OpenCLLocalAddressSpace"),
      add_named_node(&mut g, 5204, "clang::attr::OpenCLPrivateAddressSpace"),
      add_named_node(&mut g, 5205, "clang::attr::Ptr32"),
      add_named_node(&mut g, 5206, "clang::attr::Ptr64"),
      add_named_node(&mut g, 5207, "clang::attr::SPtr"),
      add_named_node(&mut g, 5208, "clang::attr::TypeNonNull"),
      add_named_node(&mut g, 5209, "clang::attr::TypeNullUnspecified"),
      add_named_node(&mut g, 5210, "clang::attr::TypeNullable"),
      add_named_node(&mut g, 5211, "clang::attr::TypeNullableResult"),
      add_named_node(&mut g, 5212, "clang::attr::UPtr"),
      add_named_node(&mut g, 5213, "clang::attr::WebAssemblyFuncref"),
      add_named_node(&mut g, 5214, "clang::attr::CodeAlign"),
      add_named_node(&mut g, 5215, "clang::attr::FallThrough"),
      add_named_node(&mut g, 5216, "clang::attr::Likely"),
      add_named_node(&mut g, 5217, "clang::attr::MustTail"),
      add_named_node(&mut g, 5218, "clang::attr::OpenCLUnrollHint"),
      add_named_node(&mut g, 5219, "clang::attr::Unlikely"),
      add_named_node(&mut g, 5220, "clang::attr::AlwaysInline"),
      add_named_node(&mut g, 5221, "clang::attr::NoInline"),
      add_named_node(&mut g, 5222, "clang::attr::NoMerge"),
      add_named_node(&mut g, 5223, "clang::attr::Suppress"),
      add_named_node(&mut g, 5224, "clang::attr::AArch64SVEPcs"),
      add_named_node(&mut g, 5225, "clang::attr::AArch64VectorPcs"),
      add_named_node(&mut g, 5226, "clang::attr::AMDGPUKernelCall"),
      add_named_node(&mut g, 5227, "clang::attr::AcquireHandle"),
      add_named_node(&mut g, 5228, "clang::attr::AnyX86NoCfCheck"),
      add_named_node(&mut g, 5229, "clang::attr::CDecl"),
      add_named_node(&mut g, 5230, "clang::attr::FastCall"),
      add_named_node(&mut g, 5231, "clang::attr::IntelOclBicc"),
      add_named_node(&mut g, 5232, "clang::attr::LifetimeBound"),
      add_named_node(&mut g, 5233, "clang::attr::M68kRTD"),
      add_named_node(&mut g, 5234, "clang::attr::MSABI"),
      add_named_node(&mut g, 5235, "clang::attr::NSReturnsRetained"),
      add_named_node(&mut g, 5236, "clang::attr::ObjCOwnership"),
      add_named_node(&mut g, 5237, "clang::attr::Pascal"),
      add_named_node(&mut g, 5238, "clang::attr::Pcs"),
      add_named_node(&mut g, 5239, "clang::attr::PreserveAll"),
      add_named_node(&mut g, 5240, "clang::attr::PreserveMost"),
      add_named_node(&mut g, 5241, "clang::attr::RegCall"),
      add_named_node(&mut g, 5242, "clang::attr::StdCall"),
      add_named_node(&mut g, 5243, "clang::attr::SwiftAsyncCall"),
      add_named_node(&mut g, 5244, "clang::attr::SwiftCall"),
      add_named_node(&mut g, 5245, "clang::attr::SysVABI"),
      add_named_node(&mut g, 5246, "clang::attr::ThisCall"),
      add_named_node(&mut g, 5247, "clang::attr::VectorCall"),
      add_named_node(&mut g, 5248, "clang::attr::SwiftAsyncContext"),
      add_named_node(&mut g, 5249, "clang::attr::SwiftContext"),
      add_named_node(&mut g, 5250, "clang::attr::SwiftErrorResult"),
      add_named_node(&mut g, 5251, "clang::attr::SwiftIndirectResult"),
      add_named_node(&mut g, 5252, "clang::attr::Annotate"),
      add_named_node(&mut g, 5253, "clang::attr::CFConsumed"),
      add_named_node(&mut g, 5254, "clang::attr::CarriesDependency"),
      add_named_node(&mut g, 5255, "clang::attr::NSConsumed"),
      add_named_node(&mut g, 5256, "clang::attr::NonNull"),
      add_named_node(&mut g, 5257, "clang::attr::OSConsumed"),
      add_named_node(&mut g, 5258, "clang::attr::PassObjectSize"),
      add_named_node(&mut g, 5259, "clang::attr::ReleaseHandle"),
      add_named_node(&mut g, 5260, "clang::attr::UseHandle"),
      add_named_node(&mut g, 5261, "clang::attr::HLSLSV_DispatchThreadID"),
      add_named_node(&mut g, 5262, "clang::attr::HLSLSV_GroupIndex"),
      add_named_node(&mut g, 5263, "clang::attr::AMDGPUFlatWorkGroupSize"),
      add_named_node(&mut g, 5264, "clang::attr::AMDGPUNumSGPR"),
      add_named_node(&mut g, 5265, "clang::attr::AMDGPUNumVGPR"),
      add_named_node(&mut g, 5266, "clang::attr::AMDGPUWavesPerEU"),
      add_named_node(&mut g, 5267, "clang::attr::ARMInterrupt"),
      add_named_node(&mut g, 5268, "clang::attr::AVRInterrupt"),
      add_named_node(&mut g, 5269, "clang::attr::AVRSignal"),
      add_named_node(&mut g, 5270, "clang::attr::AcquireCapability"),
      add_named_node(&mut g, 5271, "clang::attr::AcquiredAfter"),
      add_named_node(&mut g, 5272, "clang::attr::AcquiredBefore"),
      add_named_node(&mut g, 5273, "clang::attr::AlignMac68k"),
      add_named_node(&mut g, 5274, "clang::attr::AlignNatural"),
      add_named_node(&mut g, 5275, "clang::attr::Aligned"),
      add_named_node(&mut g, 5276, "clang::attr::AllocAlign"),
      add_named_node(&mut g, 5277, "clang::attr::AllocSize"),
      add_named_node(&mut g, 5278, "clang::attr::AlwaysDestroy"),
      add_named_node(&mut g, 5279, "clang::attr::AnalyzerNoReturn"),
      add_named_node(&mut g, 5280, "clang::attr::AnyX86Interrupt"),
      add_named_node(&mut g, 5281, "clang::attr::AnyX86NoCallerSavedRegisters"),
      add_named_node(&mut g, 5282, "clang::attr::ArcWeakrefUnavailable"),
      add_named_node(&mut g, 5283, "clang::attr::ArgumentWithTypeTag"),
      add_named_node(&mut g, 5284, "clang::attr::ArmBuiltinAlias"),
      add_named_node(&mut g, 5285, "clang::attr::ArmLocallyStreaming"),
      add_named_node(&mut g, 5286, "clang::attr::ArmNew"),
      add_named_node(&mut g, 5287, "clang::attr::Artificial"),
      add_named_node(&mut g, 5288, "clang::attr::AsmLabel"),
      add_named_node(&mut g, 5289, "clang::attr::AssertCapability"),
      add_named_node(&mut g, 5290, "clang::attr::AssertExclusiveLock"),
      add_named_node(&mut g, 5291, "clang::attr::AssertSharedLock"),
      add_named_node(&mut g, 5292, "clang::attr::AssumeAligned"),
      add_named_node(&mut g, 5293, "clang::attr::Assumption"),
      add_named_node(&mut g, 5294, "clang::attr::Availability"),
      add_named_node(&mut g, 5295, "clang::attr::AvailableOnlyInDefaultEvalMethod"),
      add_named_node(&mut g, 5296, "clang::attr::BPFPreserveAccessIndex"),
      add_named_node(&mut g, 5297, "clang::attr::BPFPreserveStaticOffset"),
      add_named_node(&mut g, 5298, "clang::attr::BTFDeclTag"),
      add_named_node(&mut g, 5299, "clang::attr::Blocks"),
      add_named_node(&mut g, 5300, "clang::attr::Builtin"),
      add_named_node(&mut g, 5301, "clang::attr::C11NoReturn"),
      add_named_node(&mut g, 5302, "clang::attr::CFAuditedTransfer"),
      add_named_node(&mut g, 5303, "clang::attr::CFGuard"),
      add_named_node(&mut g, 5304, "clang::attr::CFICanonicalJumpTable"),
      add_named_node(&mut g, 5305, "clang::attr::CFReturnsNotRetained"),
      add_named_node(&mut g, 5306, "clang::attr::CFReturnsRetained"),
      add_named_node(&mut g, 5307, "clang::attr::CFUnknownTransfer"),
      add_named_node(&mut g, 5308, "clang::attr::CPUDispatch"),
      add_named_node(&mut g, 5309, "clang::attr::CPUSpecific"),
      add_named_node(&mut g, 5310, "clang::attr::CUDAConstant"),
      add_named_node(&mut g, 5311, "clang::attr::CUDADevice"),
      add_named_node(&mut g, 5312, "clang::attr::CUDADeviceBuiltinSurfaceType"),
      add_named_node(&mut g, 5313, "clang::attr::CUDADeviceBuiltinTextureType"),
      add_named_node(&mut g, 5314, "clang::attr::CUDAGlobal"),
      add_named_node(&mut g, 5315, "clang::attr::CUDAHost"),
      add_named_node(&mut g, 5316, "clang::attr::CUDAInvalidTarget"),
      add_named_node(&mut g, 5317, "clang::attr::CUDALaunchBounds"),
      add_named_node(&mut g, 5318, "clang::attr::CUDAShared"),
      add_named_node(&mut g, 5319, "clang::attr::CXX11NoReturn"),
      add_named_node(&mut g, 5320, "clang::attr::CallableWhen"),
      add_named_node(&mut g, 5321, "clang::attr::Callback"),
      add_named_node(&mut g, 5322, "clang::attr::Capability"),
      add_named_node(&mut g, 5323, "clang::attr::CapturedRecord"),
      add_named_node(&mut g, 5324, "clang::attr::Cleanup"),
      add_named_node(&mut g, 5325, "clang::attr::CmseNSEntry"),
      add_named_node(&mut g, 5326, "clang::attr::CodeModel"),
      add_named_node(&mut g, 5327, "clang::attr::CodeSeg"),
      add_named_node(&mut g, 5328, "clang::attr::Cold"),
      add_named_node(&mut g, 5329, "clang::attr::Common"),
      add_named_node(&mut g, 5330, "clang::attr::Const"),
      add_named_node(&mut g, 5331, "clang::attr::ConstInit"),
      add_named_node(&mut g, 5332, "clang::attr::Constructor"),
      add_named_node(&mut g, 5333, "clang::attr::Consumable"),
      add_named_node(&mut g, 5334, "clang::attr::ConsumableAutoCast"),
      add_named_node(&mut g, 5335, "clang::attr::ConsumableSetOnRead"),
      add_named_node(&mut g, 5336, "clang::attr::Convergent"),
      add_named_node(&mut g, 5337, "clang::attr::CoroDisableLifetimeBound"),
      add_named_node(&mut g, 5338, "clang::attr::CoroLifetimeBound"),
      add_named_node(&mut g, 5339, "clang::attr::CoroOnlyDestroyWhenComplete"),
      add_named_node(&mut g, 5340, "clang::attr::CoroReturnType"),
      add_named_node(&mut g, 5341, "clang::attr::CoroWrapper"),
      add_named_node(&mut g, 5342, "clang::attr::CountedBy"),
      add_named_node(&mut g, 5343, "clang::attr::DLLExport"),
      add_named_node(&mut g, 5344, "clang::attr::DLLExportStaticLocal"),
      add_named_node(&mut g, 5345, "clang::attr::DLLImport"),
      add_named_node(&mut g, 5346, "clang::attr::DLLImportStaticLocal"),
      add_named_node(&mut g, 5347, "clang::attr::Deprecated"),
      add_named_node(&mut g, 5348, "clang::attr::Destructor"),
      add_named_node(&mut g, 5349, "clang::attr::DiagnoseAsBuiltin"),
      add_named_node(&mut g, 5350, "clang::attr::DiagnoseIf"),
      add_named_node(&mut g, 5351, "clang::attr::DisableSanitizerInstrumentation"),
      add_named_node(&mut g, 5352, "clang::attr::DisableTailCalls"),
      add_named_node(&mut g, 5353, "clang::attr::EmptyBases"),
      add_named_node(&mut g, 5354, "clang::attr::EnableIf"),
      add_named_node(&mut g, 5355, "clang::attr::EnforceTCB"),
      add_named_node(&mut g, 5356, "clang::attr::EnforceTCBLeaf"),
      add_named_node(&mut g, 5357, "clang::attr::EnumExtensibility"),
      add_named_node(&mut g, 5358, "clang::attr::Error"),
      add_named_node(&mut g, 5359, "clang::attr::ExcludeFromExplicitInstantiation"),
      add_named_node(&mut g, 5360, "clang::attr::ExclusiveTrylockFunction"),
      add_named_node(&mut g, 5361, "clang::attr::ExternalSourceSymbol"),
      add_named_node(&mut g, 5362, "clang::attr::Final"),
      add_named_node(&mut g, 5363, "clang::attr::FlagEnum"),
      add_named_node(&mut g, 5364, "clang::attr::Flatten"),
      add_named_node(&mut g, 5365, "clang::attr::Format"),
      add_named_node(&mut g, 5366, "clang::attr::FormatArg"),
      add_named_node(&mut g, 5367, "clang::attr::FunctionReturnThunks"),
      add_named_node(&mut g, 5368, "clang::attr::GNUInline"),
      add_named_node(&mut g, 5369, "clang::attr::GuardedBy"),
      add_named_node(&mut g, 5370, "clang::attr::GuardedVar"),
      add_named_node(&mut g, 5371, "clang::attr::HIPManaged"),
      add_named_node(&mut g, 5372, "clang::attr::HLSLNumThreads"),
      add_named_node(&mut g, 5373, "clang::attr::HLSLResource"),
      add_named_node(&mut g, 5374, "clang::attr::HLSLResourceBinding"),
      add_named_node(&mut g, 5375, "clang::attr::HLSLShader"),
      add_named_node(&mut g, 5376, "clang::attr::Hot"),
      add_named_node(&mut g, 5377, "clang::attr::IBAction"),
      add_named_node(&mut g, 5378, "clang::attr::IBOutlet"),
      add_named_node(&mut g, 5379, "clang::attr::IBOutletCollection"),
      add_named_node(&mut g, 5380, "clang::attr::InitPriority"),
      add_named_node(&mut g, 5381, "clang::attr::InternalLinkage"),
      add_named_node(&mut g, 5382, "clang::attr::LTOVisibilityPublic"),
      add_named_node(&mut g, 5383, "clang::attr::LayoutVersion"),
      add_named_node(&mut g, 5384, "clang::attr::Leaf"),
      add_named_node(&mut g, 5385, "clang::attr::LockReturned"),
      add_named_node(&mut g, 5386, "clang::attr::LocksExcluded"),
      add_named_node(&mut g, 5387, "clang::attr::M68kInterrupt"),
      add_named_node(&mut g, 5388, "clang::attr::MIGServerRoutine"),
      add_named_node(&mut g, 5389, "clang::attr::MSAllocator"),
      add_named_node(&mut g, 5390, "clang::attr::MSConstexpr"),
      add_named_node(&mut g, 5391, "clang::attr::MSInheritance"),
      add_named_node(&mut g, 5392, "clang::attr::MSNoVTable"),
      add_named_node(&mut g, 5393, "clang::attr::MSP430Interrupt"),
      add_named_node(&mut g, 5394, "clang::attr::MSStruct"),
      add_named_node(&mut g, 5395, "clang::attr::MSVtorDisp"),
      add_named_node(&mut g, 5396, "clang::attr::MaxFieldAlignment"),
      add_named_node(&mut g, 5397, "clang::attr::MayAlias"),
      add_named_node(&mut g, 5398, "clang::attr::MaybeUndef"),
      add_named_node(&mut g, 5399, "clang::attr::MicroMips"),
      add_named_node(&mut g, 5400, "clang::attr::MinSize"),
      add_named_node(&mut g, 5401, "clang::attr::MinVectorWidth"),
      add_named_node(&mut g, 5402, "clang::attr::Mips16"),
      add_named_node(&mut g, 5403, "clang::attr::MipsInterrupt"),
      add_named_node(&mut g, 5404, "clang::attr::MipsLongCall"),
      add_named_node(&mut g, 5405, "clang::attr::MipsShortCall"),
      add_named_node(&mut g, 5406, "clang::attr::NSConsumesSelf"),
      add_named_node(&mut g, 5407, "clang::attr::NSErrorDomain"),
      add_named_node(&mut g, 5408, "clang::attr::NSReturnsAutoreleased"),
      add_named_node(&mut g, 5409, "clang::attr::NSReturnsNotRetained"),
      add_named_node(&mut g, 5410, "clang::attr::NVPTXKernel"),
      add_named_node(&mut g, 5411, "clang::attr::Naked"),
      add_named_node(&mut g, 5412, "clang::attr::NoAlias"),
      add_named_node(&mut g, 5413, "clang::attr::NoCommon"),
      add_named_node(&mut g, 5414, "clang::attr::NoDebug"),
      add_named_node(&mut g, 5415, "clang::attr::NoDestroy"),
      add_named_node(&mut g, 5416, "clang::attr::NoDuplicate"),
      add_named_node(&mut g, 5417, "clang::attr::NoInstrumentFunction"),
      add_named_node(&mut g, 5418, "clang::attr::NoMicroMips"),
      add_named_node(&mut g, 5419, "clang::attr::NoMips16"),
      add_named_node(&mut g, 5420, "clang::attr::NoProfileFunction"),
      add_named_node(&mut g, 5421, "clang::attr::NoRandomizeLayout"),
      add_named_node(&mut g, 5422, "clang::attr::NoReturn"),
      add_named_node(&mut g, 5423, "clang::attr::NoSanitize"),
      add_named_node(&mut g, 5424, "clang::attr::NoSpeculativeLoadHardening"),
      add_named_node(&mut g, 5425, "clang::attr::NoSplitStack"),
      add_named_node(&mut g, 5426, "clang::attr::NoStackProtector"),
      add_named_node(&mut g, 5427, "clang::attr::NoThreadSafetyAnalysis"),
      add_named_node(&mut g, 5428, "clang::attr::NoThrow"),
      add_named_node(&mut g, 5429, "clang::attr::NoUniqueAddress"),
      add_named_node(&mut g, 5430, "clang::attr::NoUwtable"),
      add_named_node(&mut g, 5431, "clang::attr::NotTailCalled"),
      add_named_node(&mut g, 5432, "clang::attr::OMPAllocateDecl"),
      add_named_node(&mut g, 5433, "clang::attr::OMPCaptureNoInit"),
      add_named_node(&mut g, 5434, "clang::attr::OMPDeclareTargetDecl"),
      add_named_node(&mut g, 5435, "clang::attr::OMPDeclareVariant"),
      add_named_node(&mut g, 5436, "clang::attr::OMPThreadPrivateDecl"),
      add_named_node(&mut g, 5437, "clang::attr::OSConsumesThis"),
      add_named_node(&mut g, 5438, "clang::attr::OSReturnsNotRetained"),
      add_named_node(&mut g, 5439, "clang::attr::OSReturnsRetained"),
      add_named_node(&mut g, 5440, "clang::attr::OSReturnsRetainedOnNonZero"),
      add_named_node(&mut g, 5441, "clang::attr::OSReturnsRetainedOnZero"),
      add_named_node(&mut g, 5442, "clang::attr::ObjCBridge"),
      add_named_node(&mut g, 5443, "clang::attr::ObjCBridgeMutable"),
      add_named_node(&mut g, 5444, "clang::attr::ObjCBridgeRelated"),
      add_named_node(&mut g, 5445, "clang::attr::ObjCException"),
      add_named_node(&mut g, 5446, "clang::attr::ObjCExplicitProtocolImpl"),
      add_named_node(&mut g, 5447, "clang::attr::ObjCExternallyRetained"),
      add_named_node(&mut g, 5448, "clang::attr::ObjCIndependentClass"),
      add_named_node(&mut g, 5449, "clang::attr::ObjCMethodFamily"),
      add_named_node(&mut g, 5450, "clang::attr::ObjCNSObject"),
      add_named_node(&mut g, 5451, "clang::attr::ObjCPreciseLifetime"),
      add_named_node(&mut g, 5452, "clang::attr::ObjCRequiresPropertyDefs"),
      add_named_node(&mut g, 5453, "clang::attr::ObjCRequiresSuper"),
      add_named_node(&mut g, 5454, "clang::attr::ObjCReturnsInnerPointer"),
      add_named_node(&mut g, 5455, "clang::attr::ObjCRootClass"),
      add_named_node(&mut g, 5456, "clang::attr::ObjCSubclassingRestricted"),
      add_named_node(&mut g, 5457, "clang::attr::OpenCLIntelReqdSubGroupSize"),
      add_named_node(&mut g, 5458, "clang::attr::OpenCLKernel"),
      add_named_node(&mut g, 5459, "clang::attr::OptimizeNone"),
      add_named_node(&mut g, 5460, "clang::attr::Override"),
      add_named_node(&mut g, 5461, "clang::attr::Owner"),
      add_named_node(&mut g, 5462, "clang::attr::Ownership"),
      add_named_node(&mut g, 5463, "clang::attr::Packed"),
      add_named_node(&mut g, 5464, "clang::attr::ParamTypestate"),
      add_named_node(&mut g, 5465, "clang::attr::PatchableFunctionEntry"),
      add_named_node(&mut g, 5466, "clang::attr::Pointer"),
      add_named_node(&mut g, 5467, "clang::attr::PragmaClangBSSSection"),
      add_named_node(&mut g, 5468, "clang::attr::PragmaClangDataSection"),
      add_named_node(&mut g, 5469, "clang::attr::PragmaClangRelroSection"),
      add_named_node(&mut g, 5470, "clang::attr::PragmaClangRodataSection"),
      add_named_node(&mut g, 5471, "clang::attr::PragmaClangTextSection"),
      add_named_node(&mut g, 5472, "clang::attr::PreferredName"),
      add_named_node(&mut g, 5473, "clang::attr::PreferredType"),
      add_named_node(&mut g, 5474, "clang::attr::PtGuardedBy"),
      add_named_node(&mut g, 5475, "clang::attr::PtGuardedVar"),
      add_named_node(&mut g, 5476, "clang::attr::Pure"),
      add_named_node(&mut g, 5477, "clang::attr::RISCVInterrupt"),
      add_named_node(&mut g, 5478, "clang::attr::RandomizeLayout"),
      add_named_node(&mut g, 5479, "clang::attr::ReadOnlyPlacement"),
      add_named_node(&mut g, 5480, "clang::attr::Reinitializes"),
      add_named_node(&mut g, 5481, "clang::attr::ReleaseCapability"),
      add_named_node(&mut g, 5482, "clang::attr::ReqdWorkGroupSize"),
      add_named_node(&mut g, 5483, "clang::attr::RequiresCapability"),
      add_named_node(&mut g, 5484, "clang::attr::Restrict"),
      add_named_node(&mut g, 5485, "clang::attr::Retain"),
      add_named_node(&mut g, 5486, "clang::attr::ReturnTypestate"),
      add_named_node(&mut g, 5487, "clang::attr::ReturnsNonNull"),
      add_named_node(&mut g, 5488, "clang::attr::ReturnsTwice"),
      add_named_node(&mut g, 5489, "clang::attr::SYCLKernel"),
      add_named_node(&mut g, 5490, "clang::attr::SYCLSpecialClass"),
      add_named_node(&mut g, 5491, "clang::attr::ScopedLockable"),
      add_named_node(&mut g, 5492, "clang::attr::Section"),
      add_named_node(&mut g, 5493, "clang::attr::SelectAny"),
      add_named_node(&mut g, 5494, "clang::attr::Sentinel"),
      add_named_node(&mut g, 5495, "clang::attr::SetTypestate"),
      add_named_node(&mut g, 5496, "clang::attr::SharedTrylockFunction"),
      add_named_node(&mut g, 5497, "clang::attr::SpeculativeLoadHardening"),
      add_named_node(&mut g, 5498, "clang::attr::StandaloneDebug"),
      add_named_node(&mut g, 5499, "clang::attr::StrictFP"),
      add_named_node(&mut g, 5500, "clang::attr::StrictGuardStackCheck"),
      add_named_node(&mut g, 5501, "clang::attr::SwiftAsync"),
      add_named_node(&mut g, 5502, "clang::attr::SwiftAsyncError"),
      add_named_node(&mut g, 5503, "clang::attr::SwiftAsyncName"),
      add_named_node(&mut g, 5504, "clang::attr::SwiftAttr"),
      add_named_node(&mut g, 5505, "clang::attr::SwiftBridge"),
      add_named_node(&mut g, 5506, "clang::attr::SwiftBridgedTypedef"),
      add_named_node(&mut g, 5507, "clang::attr::SwiftError"),
      add_named_node(&mut g, 5508, "clang::attr::SwiftImportAsNonGeneric"),
      add_named_node(&mut g, 5509, "clang::attr::SwiftImportPropertyAsAccessors"),
      add_named_node(&mut g, 5510, "clang::attr::SwiftName"),
      add_named_node(&mut g, 5511, "clang::attr::SwiftNewType"),
      add_named_node(&mut g, 5512, "clang::attr::SwiftPrivate"),
      add_named_node(&mut g, 5513, "clang::attr::TLSModel"),
      add_named_node(&mut g, 5514, "clang::attr::Target"),
      add_named_node(&mut g, 5515, "clang::attr::TargetClones"),
      add_named_node(&mut g, 5516, "clang::attr::TargetVersion"),
      add_named_node(&mut g, 5517, "clang::attr::TestTypestate"),
      add_named_node(&mut g, 5518, "clang::attr::TransparentUnion"),
      add_named_node(&mut g, 5519, "clang::attr::TrivialABI"),
      add_named_node(&mut g, 5520, "clang::attr::TryAcquireCapability"),
      add_named_node(&mut g, 5521, "clang::attr::TypeTagForDatatype"),
      add_named_node(&mut g, 5522, "clang::attr::TypeVisibility"),
      add_named_node(&mut g, 5523, "clang::attr::Unavailable"),
      add_named_node(&mut g, 5524, "clang::attr::Uninitialized"),
      add_named_node(&mut g, 5525, "clang::attr::UnsafeBufferUsage"),
      add_named_node(&mut g, 5526, "clang::attr::Unused"),
      add_named_node(&mut g, 5527, "clang::attr::Used"),
      add_named_node(&mut g, 5528, "clang::attr::UsingIfExists"),
      add_named_node(&mut g, 5529, "clang::attr::Uuid"),
      add_named_node(&mut g, 5530, "clang::attr::VecReturn"),
      add_named_node(&mut g, 5531, "clang::attr::VecTypeHint"),
      add_named_node(&mut g, 5532, "clang::attr::Visibility"),
      add_named_node(&mut g, 5533, "clang::attr::WarnUnused"),
      add_named_node(&mut g, 5534, "clang::attr::WarnUnusedResult"),
      add_named_node(&mut g, 5535, "clang::attr::Weak"),
      add_named_node(&mut g, 5536, "clang::attr::WeakImport"),
      add_named_node(&mut g, 5537, "clang::attr::WeakRef"),
      add_named_node(&mut g, 5538, "clang::attr::WebAssemblyExportName"),
      add_named_node(&mut g, 5539, "clang::attr::WebAssemblyImportModule"),
      add_named_node(&mut g, 5540, "clang::attr::WebAssemblyImportName"),
      add_named_node(&mut g, 5541, "clang::attr::WorkGroupSizeHint"),
      add_named_node(&mut g, 5542, "clang::attr::X86ForceAlignArgPointer"),
      add_named_node(&mut g, 5543, "clang::attr::XRayInstrument"),
      add_named_node(&mut g, 5544, "clang::attr::XRayLogArgs"),
      add_named_node(&mut g, 5545, "clang::attr::ZeroCallUsedRegs"),
      add_named_node(&mut g, 5546, "clang::attr::AbiTag"),
      add_named_node(&mut g, 5547, "clang::attr::Alias"),
      add_named_node(&mut g, 5548, "clang::attr::AlignValue"),
      add_named_node(&mut g, 5549, "clang::attr::BuiltinAlias"),
      add_named_node(&mut g, 5550, "clang::attr::CalledOnce"),
      add_named_node(&mut g, 5551, "clang::attr::IFunc"),
      add_named_node(&mut g, 5552, "clang::attr::InitSeg"),
      add_named_node(&mut g, 5553, "clang::attr::LoaderUninitialized"),
      add_named_node(&mut g, 5554, "clang::attr::LoopHint"),
      add_named_node(&mut g, 5555, "clang::attr::Mode"),
      add_named_node(&mut g, 5556, "clang::attr::NoBuiltin"),
      add_named_node(&mut g, 5557, "clang::attr::NoEscape"),
      add_named_node(&mut g, 5558, "clang::attr::OMPCaptureKind"),
      add_named_node(&mut g, 5559, "clang::attr::OMPDeclareSimdDecl"),
      add_named_node(&mut g, 5560, "clang::attr::OMPReferencedVar"),
      add_named_node(&mut g, 5561, "clang::attr::ObjCBoxable"),
      add_named_node(&mut g, 5562, "clang::attr::ObjCClassStub"),
      add_named_node(&mut g, 5563, "clang::attr::ObjCDesignatedInitializer"),
      add_named_node(&mut g, 5564, "clang::attr::ObjCDirect"),
      add_named_node(&mut g, 5565, "clang::attr::ObjCDirectMembers"),
      add_named_node(&mut g, 5566, "clang::attr::ObjCNonLazyClass"),
      add_named_node(&mut g, 5567, "clang::attr::ObjCNonRuntimeProtocol"),
      add_named_node(&mut g, 5568, "clang::attr::ObjCRuntimeName"),
      add_named_node(&mut g, 5569, "clang::attr::ObjCRuntimeVisible"),
      add_named_node(&mut g, 5570, "clang::attr::OpenCLAccess"),
      add_named_node(&mut g, 5571, "clang::attr::Overloadable"),
      add_named_node(&mut g, 5572, "clang::attr::RenderScriptKernel"),
      add_named_node(&mut g, 5573, "clang::attr::SwiftObjCMembers"),
      add_named_node(&mut g, 5574, "clang::attr::SwiftVersionedAddition"),
      add_named_node(&mut g, 5575, "clang::attr::SwiftVersionedRemoval"),
      add_named_node(&mut g, 5576, "clang::attr::Thread"),
      add_named_node(&mut g, 5577, "clang::attr::FirstAttr"),
      add_named_node(&mut g, 5578, "clang::attr::LastAttr"),
      add_named_node(&mut g, 5579, "clang::attr::FirstTypeAttr"),
      add_named_node(&mut g, 5580, "clang::attr::LastTypeAttr"),
      add_named_node(&mut g, 5581, "clang::attr::FirstStmtAttr"),
      add_named_node(&mut g, 5582, "clang::attr::LastStmtAttr"),
      add_named_node(&mut g, 5583, "clang::attr::FirstDeclOrStmtAttr"),
      add_named_node(&mut g, 5584, "clang::attr::LastDeclOrStmtAttr"),
      add_named_node(&mut g, 5585, "clang::attr::FirstInheritableAttr"),
      add_named_node(&mut g, 5586, "clang::attr::LastInheritableAttr"),
      add_named_node(&mut g, 5587, "clang::attr::FirstDeclOrTypeAttr"),
      add_named_node(&mut g, 5588, "clang::attr::LastDeclOrTypeAttr"),
      add_named_node(&mut g, 5589, "clang::attr::FirstInheritableParamAttr"),
      add_named_node(&mut g, 5590, "clang::attr::LastInheritableParamAttr"),
      add_named_node(&mut g, 5591, "clang::attr::FirstParameterABIAttr"),
      add_named_node(&mut g, 5592, "clang::attr::LastParameterABIAttr"),
      add_named_node(&mut g, 5593, "clang::attr::FirstHLSLAnnotationAttr"),
      add_named_node(&mut g, 5594, "clang::attr::LastHLSLAnnotationAttr"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_KIND_2, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_SEMANTICS, "llvm::APFloatBase::Semantics");
  g.add_edge((ENUM_SEMANTICS, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 5596, "llvm::APFloatBase::S_IEEEhalf"),
      add_named_node(&mut g, 5597, "llvm::APFloatBase::S_BFloat"),
      add_named_node(&mut g, 5598, "llvm::APFloatBase::S_IEEEsingle"),
      add_named_node(&mut g, 5599, "llvm::APFloatBase::S_IEEEdouble"),
      add_named_node(&mut g, 5600, "llvm::APFloatBase::S_IEEEquad"),
      add_named_node(&mut g, 5601, "llvm::APFloatBase::S_PPCDoubleDouble"),
      add_named_node(&mut g, 5602, "llvm::APFloatBase::S_Float8E5M2"),
      add_named_node(&mut g, 5603, "llvm::APFloatBase::S_Float8E5M2FNUZ"),
      add_named_node(&mut g, 5604, "llvm::APFloatBase::S_Float8E4M3FN"),
      add_named_node(&mut g, 5605, "llvm::APFloatBase::S_Float8E4M3FNUZ"),
      add_named_node(&mut g, 5606, "llvm::APFloatBase::S_Float8E4M3B11FNUZ"),
      add_named_node(&mut g, 5607, "llvm::APFloatBase::S_FloatTF32"),
      add_named_node(&mut g, 5608, "llvm::APFloatBase::S_x87DoubleExtended"),
      add_named_node(&mut g, 5609, "llvm::APFloatBase::S_MaxSemantics"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_SEMANTICS, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g
}

pub const TRUE_: u64 = 0;
pub const FALSE_: u64 = 1;
pub const META_CLASS: u64 = 2;
pub const META_SUBCLASS: u64 = 3;
pub const META_METHOD: u64 = 4;
pub const META_DOMAIN: u64 = 5;
pub const META_RANGE: u64 = 6;
pub const META_CLANG_AST_NODE: u64 = 7;
pub const META_CLANG_AST_METHOD: u64 = 8;
pub const META_CLANG_AST_ENUM: u64 = 9;
pub const META_CLANG_AST_ENUM_ENUMERATORS: u64 = 10;
pub const META_CLANG_AST_ENUM_CONSTANT: u64 = 11;
pub const BUILTIN_STRING_CLASS: u64 = 12;
pub const BUILTIN_U64_CLASS: u64 = 13;
pub const BUILTIN_I64_CLASS: u64 = 14;
pub const BUILTIN_DOUBLE_CLASS: u64 = 15;
pub const BUILTIN_LIST_CLASS: u64 = 16;
pub const BUILTIN_LIST_SIZE: u64 = 17;
pub const BUILTIN_LIST_ITEM_CLASS: u64 = 18;
pub const BUILTIN_SET_CLASS: u64 = 19;
pub const BUILTIN_SET_SIZE: u64 = 20;
pub const BUILTIN_SET_ITEM: u64 = 21;
pub const BUILTIN_SET_ITEM_CLASS: u64 = 22;
pub const INVALID_FILE: u64 = 23;
pub const INVALID_SOURCE_LOCATION: u64 = 24;
pub const FILE_CLASS: u64 = 25;
pub const FILE_NAME: u64 = 26;
pub const FILE_CONTENT: u64 = 27;
pub const SOURCE_LOCATION_CLASS: u64 = 28;
pub const SOURCE_LOCATION_IS_FILE: u64 = 29;
pub const SOURCE_LOCATION_FILE: u64 = 30;
pub const SOURCE_LOCATION_LINE: u64 = 31;
pub const SOURCE_LOCATION_COLUMN: u64 = 32;
pub const SOURCE_LOCATION_EXPANSION_LOC: u64 = 33;
pub const SOURCE_LOCATION_SPELLING_LOC: u64 = 34;
pub const SOURCE_RANGE_CLASS: u64 = 35;
pub const SOURCE_RANGE_BEGIN: u64 = 36;
pub const SOURCE_RANGE_END: u64 = 37;
pub const QUALTYPE_CLASS: u64 = 38;
pub const QUALTYPE_IS_CONST: u64 = 39;
pub const QUALTYPE_IS_VOLATILE: u64 = 40;
pub const QUALTYPE_IS_RESTRICT: u64 = 41;
pub const QUALTYPE_TYPE: u64 = 42;
pub const USR: u64 = 43;
pub const CLASS_CFG: u64 = 44;
pub const CFG: u64 = 45;
pub const CFG_NODES: u64 = 46;
pub const CFG_ENTRY: u64 = 47;
pub const CFG_EXIT: u64 = 48;
pub const CFG_INDIRECT_GOTO_BLOCK: u64 = 49;
pub const CFG_TRY_BLOCKS: u64 = 50;
pub const CFG_IS_LINEAR: u64 = 51;
pub const CLASS_CFGBLOCK: u64 = 52;
pub const CFG_BLOCK_PARENT: u64 = 53;
pub const CFG_BLOCK_SUCCS: u64 = 54;
pub const CFG_BLOCK_PREDS: u64 = 55;
pub const CFG_BLOCK_LABEL: u64 = 56;
pub const CFG_BLOCK_TERMINATOR_STMT: u64 = 57;
pub const CFG_BLOCK_TERMINATOR_KIND: u64 = 58;
pub const CFG_BLOCK_TERMINATOR_CONDITION: u64 = 59;
pub const CFG_BLOCK_LOOP_TARGET: u64 = 60;
pub const CFG_BLOCK_HAS_NO_RETURN_ELEMENT: u64 = 61;
pub const CLASS_CFGELEMENT: u64 = 62;
pub const CLASS_CFGCLEANUPFUNCTION: u64 = 63;
pub const CLASS_CFGIMPLICITDTOR: u64 = 64;
pub const CLASS_CFGINITIALIZER: u64 = 65;
pub const CLASS_CFGLIFETIMEENDS: u64 = 66;
pub const CLASS_CFGLOOPEXIT: u64 = 67;
pub const CLASS_CFGNEWALLOCATOR: u64 = 68;
pub const CLASS_CFGSCOPEBEGIN: u64 = 69;
pub const CLASS_CFGSCOPEEND: u64 = 70;
pub const CLASS_CFGSTMT: u64 = 71;
pub const CLASS_CFGAUTOMATICOBJDTOR: u64 = 72;
pub const CLASS_CFGBASEDTOR: u64 = 73;
pub const CLASS_CFGDELETEDTOR: u64 = 74;
pub const CLASS_CFGMEMBERDTOR: u64 = 75;
pub const CLASS_CFGTEMPORARYDTOR: u64 = 76;
pub const CLASS_CFGCXXRECORDTYPEDCALL: u64 = 77;
pub const CLASS_CFGCONSTRUCTOR: u64 = 78;
pub const CFG_ELEMENT_TRIGGER_STMT: u64 = 79;
pub const CFG_ELEMENT_VAR_DECL: u64 = 80;
pub const CFG_ELEMENT_ALLOC_EXPR: u64 = 81;
pub const CFG_ELEMENT_LOOP_STMT: u64 = 82;
pub const CFG_ELEMENT_STMT: u64 = 83;
pub const CFG_ELEMENT_CTOR_CONTEXT: u64 = 84;
pub const CFG_ELEMENT_DTOR_DECL: u64 = 85;
pub const CFG_ELEMENT_IS_NO_RETURN: u64 = 86;
pub const CFG_ELEMENT_INIT: u64 = 87;
pub const CFG_ELEMENT_CXX_RECORD_DECL: u64 = 88;
pub const CFG_ELEMENT_DELETE_EXPR: u64 = 89;
pub const CFG_ELEMENT_BASE_SPECIFIER: u64 = 90;
pub const CFG_ELEMENT_FIELD_DECL: u64 = 91;
pub const CFG_ELEMENT_BIND_TEMPORARY_EXPR: u64 = 92;
pub const CFG_ELEMENT_FUNCTION_DECL: u64 = 93;
pub const CLASS_CXXSTDINITIALIZERLISTEXPR: u64 = 94;
pub const METHOD_GETSUBEXPR_2: u64 = 95;
pub const METHOD_GETBEGINLOC_40: u64 = 96;
pub const METHOD_GETENDLOC_39: u64 = 97;
pub const METHOD_GETSOURCERANGE_44: u64 = 98;
pub const METHOD_CHILDREN_30: u64 = 99;
pub const CLASS_OBJCPROTOCOLEXPR: u64 = 100;
pub const CLASS_OMPTARGETTEAMSDISTRIBUTESIMDDIRECTIVE: u64 = 101;
pub const CLASS_EXPR: u64 = 102;
pub const METHOD_GETTYPE_1: u64 = 103;
pub const METHOD_GETDEPENDENCE_1: u64 = 104;
pub const METHOD_ISVALUEDEPENDENT: u64 = 105;
pub const METHOD_ISTYPEDEPENDENT: u64 = 106;
pub const METHOD_ISINSTANTIATIONDEPENDENT: u64 = 107;
pub const METHOD_CONTAINSUNEXPANDEDPARAMETERPACK_1: u64 = 108;
pub const METHOD_CONTAINSERRORS_1: u64 = 109;
pub const METHOD_GETEXPRLOC_14: u64 = 110;
pub const METHOD_ISREADIFDISCARDEDINCPLUSPLUS11: u64 = 111;
pub const METHOD_ISLVALUE: u64 = 112;
pub const METHOD_ISPRVALUE: u64 = 113;
pub const METHOD_ISXVALUE: u64 = 114;
pub const METHOD_ISGLVALUE: u64 = 115;
pub const METHOD_GETVALUEKIND: u64 = 116;
pub const METHOD_GETOBJECTKIND: u64 = 117;
pub const METHOD_ISORDINARYORBITFIELDOBJECT: u64 = 118;
pub const METHOD_REFERSTOBITFIELD: u64 = 119;
pub const METHOD_GETSOURCEBITFIELD: u64 = 120;
pub const METHOD_GETREFERENCEDDECLOFCALLEE: u64 = 121;
pub const METHOD_GETOBJCPROPERTY: u64 = 122;
pub const METHOD_ISOBJCSELFEXPR: u64 = 123;
pub const METHOD_REFERSTOVECTORELEMENT: u64 = 124;
pub const METHOD_REFERSTOMATRIXELEMENT: u64 = 125;
pub const METHOD_REFERSTOGLOBALREGISTERVAR: u64 = 126;
pub const METHOD_HASPLACEHOLDERTYPE: u64 = 127;
pub const METHOD_IGNOREUNLESSSPELLEDINSOURCE: u64 = 128;
pub const METHOD_IGNOREIMPCASTS: u64 = 129;
pub const METHOD_IGNORECASTS: u64 = 130;
pub const METHOD_IGNOREIMPLICIT: u64 = 131;
pub const METHOD_IGNOREIMPLICITASWRITTEN: u64 = 132;
pub const METHOD_IGNOREPARENS: u64 = 133;
pub const METHOD_IGNOREPARENIMPCASTS: u64 = 134;
pub const METHOD_IGNOREPARENCASTS: u64 = 135;
pub const METHOD_IGNORECONVERSIONOPERATORSINGLESTEP: u64 = 136;
pub const METHOD_IGNOREPARENLVALUECASTS: u64 = 137;
pub const METHOD_IGNOREPARENBASECASTS: u64 = 138;
pub const METHOD_ISDEFAULTARGUMENT: u64 = 139;
pub const METHOD_ISIMPLICITCXXTHIS: u64 = 140;
pub const METHOD_GETBESTDYNAMICCLASSTYPE: u64 = 141;
pub const METHOD_GETBESTDYNAMICCLASSTYPEEXPR: u64 = 142;
pub const METHOD_SKIPRVALUESUBOBJECTADJUSTMENTS: u64 = 143;
pub const CLASS_OMPPARALLELMASKEDTASKLOOPDIRECTIVE: u64 = 144;
pub const CLASS_GOTOSTMT: u64 = 145;
pub const METHOD_GETLABEL_1: u64 = 146;
pub const METHOD_GETGOTOLOC: u64 = 147;
pub const METHOD_GETLABELLOC_1: u64 = 148;
pub const METHOD_GETBEGINLOC_81: u64 = 149;
pub const METHOD_GETENDLOC_80: u64 = 150;
pub const METHOD_CHILDREN_70: u64 = 151;
pub const CLASS_CONTINUESTMT: u64 = 152;
pub const METHOD_GETCONTINUELOC: u64 = 153;
pub const METHOD_GETBEGINLOC_58: u64 = 154;
pub const METHOD_GETENDLOC_57: u64 = 155;
pub const METHOD_CHILDREN_48: u64 = 156;
pub const CLASS_OBJCSELECTOREXPR: u64 = 157;
pub const CLASS_SWITCHSTMT: u64 = 158;
pub const METHOD_HASINITSTORAGE_1: u64 = 159;
pub const METHOD_HASVARSTORAGE_1: u64 = 160;
pub const METHOD_GETCOND_8: u64 = 161;
pub const METHOD_GETBODY_11: u64 = 162;
pub const METHOD_GETINIT_6: u64 = 163;
pub const METHOD_GETCONDITIONVARIABLE_2: u64 = 164;
pub const METHOD_GETCONDITIONVARIABLEDECLSTMT_2: u64 = 165;
pub const METHOD_GETSWITCHCASELIST: u64 = 166;
pub const METHOD_GETSWITCHLOC: u64 = 167;
pub const METHOD_GETLPARENLOC_10: u64 = 168;
pub const METHOD_GETRPARENLOC_25: u64 = 169;
pub const METHOD_ISALLENUMCASESCOVERED: u64 = 170;
pub const METHOD_GETBEGINLOC_123: u64 = 171;
pub const METHOD_GETENDLOC_122: u64 = 172;
pub const METHOD_CHILDREN_110: u64 = 173;
pub const CLASS_PSEUDOOBJECTEXPR: u64 = 174;
pub const METHOD_GETSYNTACTICFORM_1: u64 = 175;
pub const METHOD_GETRESULTEXPRINDEX: u64 = 176;
pub const METHOD_GETRESULTEXPR_1: u64 = 177;
pub const METHOD_GETNUMSEMANTICEXPRS: u64 = 178;
pub const METHOD_SEMANTICS_BEGIN: u64 = 179;
pub const METHOD_SEMANTICS_END: u64 = 180;
pub const METHOD_SEMANTICS: u64 = 181;
pub const METHOD_GETEXPRLOC_11: u64 = 182;
pub const METHOD_GETBEGINLOC_106: u64 = 183;
pub const METHOD_GETENDLOC_105: u64 = 184;
pub const METHOD_CHILDREN_94: u64 = 185;
pub const CLASS_PARENEXPR: u64 = 186;
pub const METHOD_GETSUBEXPR_8: u64 = 187;
pub const METHOD_GETBEGINLOC_103: u64 = 188;
pub const METHOD_GETENDLOC_102: u64 = 189;
pub const METHOD_GETLPAREN: u64 = 190;
pub const METHOD_GETRPAREN: u64 = 191;
pub const METHOD_CHILDREN_91: u64 = 192;
pub const CLASS_MSASMSTMT: u64 = 193;
pub const METHOD_GETLBRACELOC_2: u64 = 194;
pub const METHOD_GETENDLOC_90: u64 = 195;
pub const METHOD_HASBRACES_2: u64 = 196;
pub const METHOD_GETASMSTRING_2: u64 = 197;
pub const METHOD_GETALLCONSTRAINTS: u64 = 198;
pub const METHOD_GETCLOBBERS: u64 = 199;
pub const METHOD_GETALLEXPRS: u64 = 200;
pub const METHOD_GETBEGINLOC_91: u64 = 201;
pub const METHOD_CHILDREN_79: u64 = 202;
pub const CLASS_SEHEXCEPTSTMT: u64 = 203;
pub const METHOD_GETBEGINLOC_110: u64 = 204;
pub const METHOD_GETEXCEPTLOC: u64 = 205;
pub const METHOD_GETENDLOC_109: u64 = 206;
pub const METHOD_GETFILTEREXPR: u64 = 207;
pub const METHOD_GETBLOCK: u64 = 208;
pub const METHOD_CHILDREN_97: u64 = 209;
pub const CLASS_OMPLOOPBASEDDIRECTIVE: u64 = 210;
pub const CLASS_SEHLEAVESTMT: u64 = 211;
pub const METHOD_GETLEAVELOC: u64 = 212;
pub const METHOD_GETBEGINLOC_112: u64 = 213;
pub const METHOD_GETENDLOC_111: u64 = 214;
pub const METHOD_CHILDREN_99: u64 = 215;
pub const CLASS_UNARYEXPRORTYPETRAITEXPR: u64 = 216;
pub const METHOD_GETKIND_5: u64 = 217;
pub const METHOD_ISARGUMENTTYPE: u64 = 218;
pub const METHOD_GETARGUMENTTYPE: u64 = 219;
pub const METHOD_GETARGUMENTTYPEINFO: u64 = 220;
pub const METHOD_GETARGUMENTEXPR: u64 = 221;
pub const METHOD_GETTYPEOFARGUMENT: u64 = 222;
pub const METHOD_GETOPERATORLOC_9: u64 = 223;
pub const METHOD_GETRPARENLOC_26: u64 = 224;
pub const METHOD_GETBEGINLOC_126: u64 = 225;
pub const METHOD_GETENDLOC_125: u64 = 226;
pub const METHOD_CHILDREN_113: u64 = 227;
pub const CLASS_OBJCENCODEEXPR: u64 = 228;
pub const CLASS_COROUTINEBODYSTMT: u64 = 229;
pub const METHOD_HASDEPENDENTPROMISETYPE: u64 = 230;
pub const METHOD_GETBODY_6: u64 = 231;
pub const METHOD_GETPROMISEDECLSTMT: u64 = 232;
pub const METHOD_GETPROMISEDECL: u64 = 233;
pub const METHOD_GETINITSUSPENDSTMT: u64 = 234;
pub const METHOD_GETFINALSUSPENDSTMT: u64 = 235;
pub const METHOD_GETEXCEPTIONHANDLER: u64 = 236;
pub const METHOD_GETFALLTHROUGHHANDLER: u64 = 237;
pub const METHOD_GETALLOCATE: u64 = 238;
pub const METHOD_GETDEALLOCATE: u64 = 239;
pub const METHOD_GETRESULTDECL: u64 = 240;
pub const METHOD_GETRETURNVALUEINIT: u64 = 241;
pub const METHOD_GETRETURNVALUE: u64 = 242;
pub const METHOD_GETRETURNSTMT: u64 = 243;
pub const METHOD_GETRETURNSTMTONALLOCFAILURE: u64 = 244;
pub const METHOD_GETPARAMMOVES: u64 = 245;
pub const METHOD_GETBEGINLOC_61: u64 = 246;
pub const METHOD_GETENDLOC_60: u64 = 247;
pub const METHOD_CHILDREN_51: u64 = 248;
pub const METHOD_CHILDRENEXCLBODY: u64 = 249;
pub const CLASS_ARRAYTYPETRAITEXPR: u64 = 250;
pub const METHOD_GETBEGINLOC_8: u64 = 251;
pub const METHOD_GETENDLOC_7: u64 = 252;
pub const METHOD_GETTRAIT: u64 = 253;
pub const METHOD_GETQUERIEDTYPE: u64 = 254;
pub const METHOD_GETQUERIEDTYPESOURCEINFO: u64 = 255;
pub const METHOD_GETVALUE_9: u64 = 256;
pub const METHOD_GETDIMENSIONEXPRESSION: u64 = 257;
pub const METHOD_CHILDREN_4: u64 = 258;
pub const CLASS_OBJCAUTORELEASEPOOLSTMT: u64 = 259;
pub const CLASS_OMPCANONICALLOOP: u64 = 260;
pub const CLASS_OMPTILEDIRECTIVE: u64 = 261;
pub const CLASS_CASESTMT: u64 = 262;
pub const METHOD_CASESTMTISGNURANGE: u64 = 263;
pub const METHOD_GETCASELOC: u64 = 264;
pub const METHOD_GETELLIPSISLOC_5: u64 = 265;
pub const METHOD_GETLHS_4: u64 = 266;
pub const METHOD_GETRHS_4: u64 = 267;
pub const METHOD_GETSUBSTMT_1: u64 = 268;
pub const METHOD_GETBEGINLOC_50: u64 = 269;
pub const METHOD_GETENDLOC_49: u64 = 270;
pub const METHOD_CHILDREN_39: u64 = 271;
pub const CLASS_OMPLOOPTRANSFORMATIONDIRECTIVE: u64 = 272;
pub const CLASS_CXXBOOLLITERALEXPR: u64 = 273;
pub const METHOD_GETVALUE_4: u64 = 274;
pub const METHOD_GETBEGINLOC_20: u64 = 275;
pub const METHOD_GETENDLOC_19: u64 = 276;
pub const METHOD_GETLOCATION_1: u64 = 277;
pub const METHOD_CHILDREN_14: u64 = 278;
pub const CLASS_SYCLUNIQUESTABLENAMEEXPR: u64 = 279;
pub const METHOD_GETTYPESOURCEINFO_8: u64 = 280;
pub const METHOD_GETBEGINLOC_114: u64 = 281;
pub const METHOD_GETENDLOC_113: u64 = 282;
pub const METHOD_GETLOCATION_14: u64 = 283;
pub const METHOD_GETLPARENLOCATION: u64 = 284;
pub const METHOD_GETRPARENLOCATION: u64 = 285;
pub const METHOD_CHILDREN_101: u64 = 286;
pub const CLASS_OMPSIMDDIRECTIVE: u64 = 287;
pub const CLASS_ASMSTMT: u64 = 288;
pub const METHOD_GETASMLOC_1: u64 = 289;
pub const METHOD_ISSIMPLE: u64 = 290;
pub const METHOD_ISVOLATILE_2: u64 = 291;
pub const METHOD_GETBEGINLOC_10: u64 = 292;
pub const METHOD_GETENDLOC_9: u64 = 293;
pub const METHOD_GETNUMOUTPUTS: u64 = 294;
pub const METHOD_GETNUMPLUSOPERANDS: u64 = 295;
pub const METHOD_GETNUMINPUTS: u64 = 296;
pub const METHOD_GETNUMCLOBBERS: u64 = 297;
pub const METHOD_BEGIN_INPUTS: u64 = 298;
pub const METHOD_END_INPUTS: u64 = 299;
pub const METHOD_INPUTS: u64 = 300;
pub const METHOD_BEGIN_OUTPUTS: u64 = 301;
pub const METHOD_END_OUTPUTS: u64 = 302;
pub const METHOD_OUTPUTS: u64 = 303;
pub const METHOD_CHILDREN_6: u64 = 304;
pub const CLASS_OMPFORSIMDDIRECTIVE: u64 = 305;
pub const CLASS_OBJCBOXEDEXPR: u64 = 306;
pub const CLASS_OMPPARALLELSECTIONSDIRECTIVE: u64 = 307;
pub const CLASS_OBJCAVAILABILITYCHECKEXPR: u64 = 308;
pub const CLASS_OMPTASKLOOPSIMDDIRECTIVE: u64 = 309;
pub const CLASS_OMPMASTERTASKLOOPSIMDDIRECTIVE: u64 = 310;
pub const CLASS_CXXTRYSTMT: u64 = 311;
pub const METHOD_GETBEGINLOC_44: u64 = 312;
pub const METHOD_GETTRYLOC: u64 = 313;
pub const METHOD_GETENDLOC_43: u64 = 314;
pub const METHOD_GETTRYBLOCK: u64 = 315;
pub const METHOD_GETNUMHANDLERS: u64 = 316;
pub const METHOD_CHILDREN_33: u64 = 317;
pub const CLASS_DEFAULTSTMT: u64 = 318;
pub const METHOD_GETSUBSTMT_2: u64 = 319;
pub const METHOD_GETDEFAULTLOC_1: u64 = 320;
pub const METHOD_GETBEGINLOC_65: u64 = 321;
pub const METHOD_GETENDLOC_64: u64 = 322;
pub const METHOD_CHILDREN_55: u64 = 323;
pub const CLASS_OMPMASKEDTASKLOOPSIMDDIRECTIVE: u64 = 324;
pub const CLASS_OBJCISAEXPR: u64 = 325;
pub const CLASS_OBJCATFINALLYSTMT: u64 = 326;
pub const CLASS_BLOCKEXPR: u64 = 327;
pub const METHOD_GETBLOCKDECL: u64 = 328;
pub const METHOD_GETCARETLOCATION_1: u64 = 329;
pub const METHOD_GETBODY_4: u64 = 330;
pub const METHOD_GETBEGINLOC_15: u64 = 331;
pub const METHOD_GETENDLOC_14: u64 = 332;
pub const METHOD_GETFUNCTIONTYPE: u64 = 333;
pub const METHOD_CHILDREN_11: u64 = 334;
pub const CLASS_OMPMASTERDIRECTIVE: u64 = 335;
pub const CLASS_OBJCIVARREFEXPR: u64 = 336;
pub const CLASS_OMPITERATOREXPR: u64 = 337;
pub const CLASS_OMPTARGETUPDATEDIRECTIVE: u64 = 338;
pub const CLASS_IFSTMT: u64 = 339;
pub const METHOD_HASINITSTORAGE: u64 = 340;
pub const METHOD_HASVARSTORAGE: u64 = 341;
pub const METHOD_HASELSESTORAGE: u64 = 342;
pub const METHOD_GETCOND_7: u64 = 343;
pub const METHOD_GETTHEN: u64 = 344;
pub const METHOD_GETELSE: u64 = 345;
pub const METHOD_GETCONDITIONVARIABLE_1: u64 = 346;
pub const METHOD_GETCONDITIONVARIABLEDECLSTMT_1: u64 = 347;
pub const METHOD_GETINIT_5: u64 = 348;
pub const METHOD_GETIFLOC: u64 = 349;
pub const METHOD_GETELSELOC: u64 = 350;
pub const METHOD_ISCONSTEVAL_1: u64 = 351;
pub const METHOD_ISNONNEGATEDCONSTEVAL: u64 = 352;
pub const METHOD_ISNEGATEDCONSTEVAL: u64 = 353;
pub const METHOD_ISCONSTEXPR_2: u64 = 354;
pub const METHOD_GETSTATEMENTKIND: u64 = 355;
pub const METHOD_ISOBJCAVAILABILITYCHECK: u64 = 356;
pub const METHOD_GETBEGINLOC_82: u64 = 357;
pub const METHOD_GETENDLOC_81: u64 = 358;
pub const METHOD_GETLPARENLOC_6: u64 = 359;
pub const METHOD_GETRPARENLOC_18: u64 = 360;
pub const METHOD_CHILDREN_71: u64 = 361;
pub const CLASS_RECOVERYEXPR: u64 = 362;
pub const METHOD_SUBEXPRESSIONS: u64 = 363;
pub const METHOD_GETBEGINLOC_107: u64 = 364;
pub const METHOD_GETENDLOC_106: u64 = 365;
pub const CLASS_OMPDISTRIBUTEPARALLELFORDIRECTIVE: u64 = 366;
pub const CLASS_OMPTARGETPARALLELDIRECTIVE: u64 = 367;
pub const CLASS_CONSTANTEXPR: u64 = 368;
pub const METHOD_GETBEGINLOC_57: u64 = 369;
pub const METHOD_GETENDLOC_56: u64 = 370;
pub const METHOD_GETRESULTAPVALUEKIND: u64 = 371;
pub const METHOD_GETRESULTSTORAGEKIND: u64 = 372;
pub const METHOD_ISIMMEDIATEINVOCATION: u64 = 373;
pub const METHOD_HASAPVALUERESULT: u64 = 374;
pub const METHOD_GETAPVALUERESULT: u64 = 375;
pub const METHOD_GETRESULTASAPSINT: u64 = 376;
pub const METHOD_CHILDREN_47: u64 = 377;
pub const CLASS_INITLISTEXPR: u64 = 378;
pub const METHOD_GETNUMINITS: u64 = 379;
pub const METHOD_GETINITS: u64 = 380;
pub const METHOD_INITS_1: u64 = 381;
pub const METHOD_GETARRAYFILLER_1: u64 = 382;
pub const METHOD_HASARRAYFILLER: u64 = 383;
pub const METHOD_HASDESIGNATEDINIT: u64 = 384;
pub const METHOD_GETINITIALIZEDFIELDINUNION_1: u64 = 385;
pub const METHOD_ISEXPLICIT_3: u64 = 386;
pub const METHOD_ISSTRINGLITERALINIT: u64 = 387;
pub const METHOD_ISTRANSPARENT: u64 = 388;
pub const METHOD_GETLBRACELOC_1: u64 = 389;
pub const METHOD_GETRBRACELOC_4: u64 = 390;
pub const METHOD_ISSEMANTICFORM: u64 = 391;
pub const METHOD_GETSEMANTICFORM_1: u64 = 392;
pub const METHOD_ISSYNTACTICFORM: u64 = 393;
pub const METHOD_GETSYNTACTICFORM: u64 = 394;
pub const METHOD_HADARRAYRANGEDESIGNATOR: u64 = 395;
pub const METHOD_GETBEGINLOC_87: u64 = 396;
pub const METHOD_GETENDLOC_86: u64 = 397;
pub const METHOD_CHILDREN_75: u64 = 398;
pub const METHOD_BEGIN: u64 = 399;
pub const METHOD_END: u64 = 400;
pub const METHOD_RBEGIN: u64 = 401;
pub const METHOD_REND: u64 = 402;
pub const CLASS_OMPTARGETTEAMSGENERICLOOPDIRECTIVE: u64 = 403;
pub const CLASS_SWITCHCASE: u64 = 404;
pub const METHOD_GETNEXTSWITCHCASE: u64 = 405;
pub const METHOD_GETKEYWORDLOC_4: u64 = 406;
pub const METHOD_GETCOLONLOC_3: u64 = 407;
pub const METHOD_GETSUBSTMT_6: u64 = 408;
pub const METHOD_GETBEGINLOC_122: u64 = 409;
pub const METHOD_GETENDLOC_121: u64 = 410;
pub const CLASS_OMPTARGETDATADIRECTIVE: u64 = 411;
pub const CLASS_OMPTEAMSDISTRIBUTEDIRECTIVE: u64 = 412;
pub const CLASS_OMPTASKGROUPDIRECTIVE: u64 = 413;
pub const CLASS_OMPUNROLLDIRECTIVE: u64 = 414;
pub const CLASS_FUNCTIONPARMPACKEXPR: u64 = 415;
pub const METHOD_GETPARAMETERPACK: u64 = 416;
pub const METHOD_GETPARAMETERPACKLOCATION: u64 = 417;
pub const METHOD_BEGIN_1: u64 = 418;
pub const METHOD_END_1: u64 = 419;
pub const METHOD_GETNUMEXPANSIONS_2: u64 = 420;
pub const METHOD_GETBEGINLOC_77: u64 = 421;
pub const METHOD_GETENDLOC_76: u64 = 422;
pub const METHOD_CHILDREN_67: u64 = 423;
pub const CLASS_OMPTARGETTEAMSDISTRIBUTEDIRECTIVE: u64 = 424;
pub const CLASS_DECLREFEXPR: u64 = 425;
pub const METHOD_GETDECL_7: u64 = 426;
pub const METHOD_GETNAMEINFO_4: u64 = 427;
pub const METHOD_GETLOCATION_7: u64 = 428;
pub const METHOD_GETBEGINLOC_63: u64 = 429;
pub const METHOD_GETENDLOC_62: u64 = 430;
pub const METHOD_HASQUALIFIER_1: u64 = 431;
pub const METHOD_GETQUALIFIERLOC_10: u64 = 432;
pub const METHOD_GETQUALIFIER_13: u64 = 433;
pub const METHOD_GETFOUNDDECL_2: u64 = 434;
pub const METHOD_HASTEMPLATEKWANDARGSINFO: u64 = 435;
pub const METHOD_GETTEMPLATEKEYWORDLOC_3: u64 = 436;
pub const METHOD_GETLANGLELOC_1: u64 = 437;
pub const METHOD_GETRANGLELOC_1: u64 = 438;
pub const METHOD_HASTEMPLATEKEYWORD_1: u64 = 439;
pub const METHOD_HASEXPLICITTEMPLATEARGS_2: u64 = 440;
pub const METHOD_GETTEMPLATEARGS_3: u64 = 441;
pub const METHOD_GETNUMTEMPLATEARGS_1: u64 = 442;
pub const METHOD_TEMPLATE_ARGUMENTS_3: u64 = 443;
pub const METHOD_HADMULTIPLECANDIDATES_1: u64 = 444;
pub const METHOD_ISNONODRUSE: u64 = 445;
pub const METHOD_REFERSTOENCLOSINGVARIABLEORCAPTURE: u64 = 446;
pub const METHOD_ISIMMEDIATEESCALATING_2: u64 = 447;
pub const METHOD_ISCAPTUREDBYCOPYINLAMBDAWITHEXPLICITOBJECTPARAMETER: u64 = 448;
pub const METHOD_CHILDREN_53: u64 = 449;
pub const CLASS_FORSTMT: u64 = 450;
pub const METHOD_GETCONDITIONVARIABLE: u64 = 451;
pub const METHOD_GETCONDITIONVARIABLEDECLSTMT: u64 = 452;
pub const METHOD_GETINIT_4: u64 = 453;
pub const METHOD_GETCOND_6: u64 = 454;
pub const METHOD_GETINC_1: u64 = 455;
pub const METHOD_GETBODY_8: u64 = 456;
pub const METHOD_GETFORLOC_1: u64 = 457;
pub const METHOD_GETLPARENLOC_5: u64 = 458;
pub const METHOD_GETRPARENLOC_15: u64 = 459;
pub const METHOD_GETBEGINLOC_76: u64 = 460;
pub const METHOD_GETENDLOC_75: u64 = 461;
pub const METHOD_CHILDREN_66: u64 = 462;
pub const CLASS_OVERLOADEXPR: u64 = 463;
pub const METHOD_GETNAMINGCLASS: u64 = 464;
pub const METHOD_DECLS_BEGIN: u64 = 465;
pub const METHOD_DECLS_END: u64 = 466;
pub const METHOD_DECLS_1: u64 = 467;
pub const METHOD_GETNUMDECLS: u64 = 468;
pub const METHOD_GETNAMEINFO_7: u64 = 469;
pub const METHOD_GETNAME_2: u64 = 470;
pub const METHOD_GETNAMELOC: u64 = 471;
pub const METHOD_GETQUALIFIER_16: u64 = 472;
pub const METHOD_GETQUALIFIERLOC_15: u64 = 473;
pub const METHOD_GETTEMPLATEKEYWORDLOC_6: u64 = 474;
pub const METHOD_GETLANGLELOC_4: u64 = 475;
pub const METHOD_GETRANGLELOC_4: u64 = 476;
pub const METHOD_HASTEMPLATEKEYWORD_4: u64 = 477;
pub const METHOD_HASEXPLICITTEMPLATEARGS_5: u64 = 478;
pub const METHOD_GETTEMPLATEARGS_6: u64 = 479;
pub const METHOD_GETNUMTEMPLATEARGS_4: u64 = 480;
pub const METHOD_TEMPLATE_ARGUMENTS_6: u64 = 481;
pub const CLASS_OMPCANCELLATIONPOINTDIRECTIVE: u64 = 482;
pub const CLASS_FIXEDPOINTLITERAL: u64 = 483;
pub const METHOD_GETBEGINLOC_74: u64 = 484;
pub const METHOD_GETENDLOC_73: u64 = 485;
pub const METHOD_GETLOCATION_9: u64 = 486;
pub const METHOD_GETSCALE: u64 = 487;
pub const METHOD_CHILDREN_64: u64 = 488;
pub const CLASS_OBJCATTHROWSTMT: u64 = 489;
pub const CLASS_OMPTARGETEXITDATADIRECTIVE: u64 = 490;
pub const CLASS_OMPTARGETENTERDATADIRECTIVE: u64 = 491;
pub const CLASS_OMPTARGETTEAMSDISTRIBUTEPARALLELFORDIRECTIVE: u64 = 492;
pub const CLASS_OMPFLUSHDIRECTIVE: u64 = 493;
pub const CLASS_OMPTARGETTEAMSDIRECTIVE: u64 = 494;
pub const CLASS_OMPSCANDIRECTIVE: u64 = 495;
pub const CLASS_OMPMETADIRECTIVE: u64 = 496;
pub const CLASS_OMPATOMICDIRECTIVE: u64 = 497;
pub const CLASS_NULLSTMT: u64 = 498;
pub const METHOD_GETSEMILOC: u64 = 499;
pub const METHOD_HASLEADINGEMPTYMACRO: u64 = 500;
pub const METHOD_GETBEGINLOC_99: u64 = 501;
pub const METHOD_GETENDLOC_98: u64 = 502;
pub const METHOD_CHILDREN_87: u64 = 503;
pub const CLASS_OMPDISTRIBUTEPARALLELFORSIMDDIRECTIVE: u64 = 504;
pub const CLASS_OMPGENERICLOOPDIRECTIVE: u64 = 505;
pub const CLASS_OMPEXECUTABLEDIRECTIVE: u64 = 506;
pub const CLASS_CXXDEFAULTINITEXPR: u64 = 507;
pub const METHOD_HASREWRITTENINIT_1: u64 = 508;
pub const METHOD_GETFIELD: u64 = 509;
pub const METHOD_GETEXPR_1: u64 = 510;
pub const METHOD_GETREWRITTENEXPR_1: u64 = 511;
pub const METHOD_GETUSEDCONTEXT_1: u64 = 512;
pub const METHOD_GETUSEDLOCATION_1: u64 = 513;
pub const METHOD_GETBEGINLOC_24: u64 = 514;
pub const METHOD_GETENDLOC_23: u64 = 515;
pub const METHOD_CHILDREN_18: u64 = 516;
pub const CLASS_OMPMASTERTASKLOOPDIRECTIVE: u64 = 517;
pub const CLASS_OMPPARALLELMASTERTASKLOOPSIMDDIRECTIVE: u64 = 518;
pub const CLASS_EXPLICITCASTEXPR: u64 = 519;
pub const METHOD_GETTYPEINFOASWRITTEN: u64 = 520;
pub const METHOD_GETTYPEASWRITTEN_3: u64 = 521;
pub const CLASS_OMPSECTIONSDIRECTIVE: u64 = 522;
pub const CLASS_OMPLOOPDIRECTIVE: u64 = 523;
pub const CLASS_OMPARRAYSECTIONEXPR: u64 = 524;
pub const CLASS_OMPMASKEDDIRECTIVE: u64 = 525;
pub const CLASS_CXXUNRESOLVEDCONSTRUCTEXPR: u64 = 526;
pub const METHOD_GETTYPEASWRITTEN_2: u64 = 527;
pub const METHOD_GETTYPESOURCEINFO_4: u64 = 528;
pub const METHOD_GETLPARENLOC_3: u64 = 529;
pub const METHOD_GETRPARENLOC_10: u64 = 530;
pub const METHOD_ISLISTINITIALIZATION_2: u64 = 531;
pub const METHOD_GETNUMARGS_2: u64 = 532;
pub const METHOD_ARG_BEGIN: u64 = 533;
pub const METHOD_ARG_END: u64 = 534;
pub const METHOD_ARGUMENTS_1: u64 = 535;
pub const METHOD_GETBEGINLOC_46: u64 = 536;
pub const METHOD_GETENDLOC_45: u64 = 537;
pub const METHOD_CHILDREN_35: u64 = 538;
pub const CLASS_USERDEFINEDLITERAL: u64 = 539;
pub const METHOD_GETLITERALOPERATORKIND: u64 = 540;
pub const METHOD_GETCOOKEDLITERAL: u64 = 541;
pub const METHOD_GETBEGINLOC_130: u64 = 542;
pub const METHOD_GETENDLOC_129: u64 = 543;
pub const METHOD_GETUDSUFFIXLOC: u64 = 544;
pub const METHOD_GETUDSUFFIX: u64 = 545;
pub const CLASS_CXXCATCHSTMT: u64 = 546;
pub const METHOD_GETBEGINLOC_21: u64 = 547;
pub const METHOD_GETENDLOC_20: u64 = 548;
pub const METHOD_GETCATCHLOC: u64 = 549;
pub const METHOD_GETEXCEPTIONDECL: u64 = 550;
pub const METHOD_GETCAUGHTTYPE: u64 = 551;
pub const METHOD_GETHANDLERBLOCK: u64 = 552;
pub const METHOD_CHILDREN_15: u64 = 553;
pub const CLASS_OMPSCOPEDIRECTIVE: u64 = 554;
pub const CLASS_CXXADDRSPACECASTEXPR: u64 = 555;
pub const CLASS_OMPTEAMSDISTRIBUTEPARALLELFORSIMDDIRECTIVE: u64 = 556;
pub const CLASS_OMPARRAYSHAPINGEXPR: u64 = 557;
pub const CLASS_UNRESOLVEDLOOKUPEXPR: u64 = 558;
pub const METHOD_REQUIRESADL: u64 = 559;
pub const METHOD_ISOVERLOADED: u64 = 560;
pub const METHOD_GETNAMINGCLASS_1: u64 = 561;
pub const METHOD_GETBEGINLOC_128: u64 = 562;
pub const METHOD_GETENDLOC_127: u64 = 563;
pub const METHOD_CHILDREN_115: u64 = 564;
pub const CLASS_OMPTEAMSDISTRIBUTESIMDDIRECTIVE: u64 = 565;
pub const CLASS_ABSTRACTCONDITIONALOPERATOR: u64 = 566;
pub const METHOD_GETCOND: u64 = 567;
pub const METHOD_GETTRUEEXPR: u64 = 568;
pub const METHOD_GETFALSEEXPR: u64 = 569;
pub const METHOD_GETQUESTIONLOC: u64 = 570;
pub const METHOD_GETCOLONLOC_1: u64 = 571;
pub const CLASS_BINARYCONDITIONALOPERATOR: u64 = 572;
pub const METHOD_GETCOMMON: u64 = 573;
pub const METHOD_GETOPAQUEVALUE: u64 = 574;
pub const METHOD_GETCOND_1: u64 = 575;
pub const METHOD_GETTRUEEXPR_1: u64 = 576;
pub const METHOD_GETFALSEEXPR_1: u64 = 577;
pub const METHOD_GETBEGINLOC_13: u64 = 578;
pub const METHOD_GETENDLOC_12: u64 = 579;
pub const METHOD_CHILDREN_9: u64 = 580;
pub const CLASS_OMPBARRIERDIRECTIVE: u64 = 581;
pub const CLASS_OMPTASKDIRECTIVE: u64 = 582;
pub const CLASS_SHUFFLEVECTOREXPR: u64 = 583;
pub const METHOD_GETBUILTINLOC_4: u64 = 584;
pub const METHOD_GETRPARENLOC_22: u64 = 585;
pub const METHOD_GETBEGINLOC_115: u64 = 586;
pub const METHOD_GETENDLOC_114: u64 = 587;
pub const METHOD_GETNUMSUBEXPRS_2: u64 = 588;
pub const METHOD_CHILDREN_102: u64 = 589;
pub const CLASS_OMPTARGETPARALLELFORSIMDDIRECTIVE: u64 = 590;
pub const CLASS_OMPFORDIRECTIVE: u64 = 591;
pub const CLASS_CONCEPTSPECIALIZATIONEXPR: u64 = 592;
pub const METHOD_GETTEMPLATEARGUMENTS_1: u64 = 593;
pub const METHOD_GETCONCEPTREFERENCE: u64 = 594;
pub const METHOD_GETNAMEDCONCEPT: u64 = 595;
pub const METHOD_HASEXPLICITTEMPLATEARGS_1: u64 = 596;
pub const METHOD_GETCONCEPTNAMELOC: u64 = 597;
pub const METHOD_GETTEMPLATEARGSASWRITTEN_2: u64 = 598;
pub const METHOD_GETNESTEDNAMESPECIFIERLOC: u64 = 599;
pub const METHOD_GETTEMPLATEKWLOC: u64 = 600;
pub const METHOD_GETFOUNDDECL_1: u64 = 601;
pub const METHOD_GETCONCEPTNAMEINFO: u64 = 602;
pub const METHOD_GETSPECIALIZATIONDECL: u64 = 603;
pub const METHOD_ISSATISFIED: u64 = 604;
pub const METHOD_GETSATISFACTION: u64 = 605;
pub const METHOD_GETBEGINLOC_55: u64 = 606;
pub const METHOD_GETENDLOC_54: u64 = 607;
pub const METHOD_GETEXPRLOC_6: u64 = 608;
pub const METHOD_CHILDREN_45: u64 = 609;
pub const CLASS_OBJCBOOLLITERALEXPR: u64 = 610;
pub const CLASS_GNUNULLEXPR: u64 = 611;
pub const METHOD_GETTOKENLOCATION: u64 = 612;
pub const METHOD_GETBEGINLOC_79: u64 = 613;
pub const METHOD_GETENDLOC_78: u64 = 614;
pub const METHOD_CHILDREN_68: u64 = 615;
pub const CLASS_ADDRLABELEXPR: u64 = 616;
pub const METHOD_GETAMPAMPLOC: u64 = 617;
pub const METHOD_GETLABELLOC: u64 = 618;
pub const METHOD_GETBEGINLOC_4: u64 = 619;
pub const METHOD_GETENDLOC_3: u64 = 620;
pub const METHOD_GETLABEL: u64 = 621;
pub const METHOD_CHILDREN: u64 = 622;
pub const CLASS_OMPPARALLELMASKEDTASKLOOPSIMDDIRECTIVE: u64 = 623;
pub const CLASS_OBJCSTRINGLITERAL: u64 = 624;
pub const CLASS_OMPINTEROPDIRECTIVE: u64 = 625;
pub const CLASS_REQUIRESEXPR: u64 = 626;
pub const METHOD_GETLOCALPARAMETERS: u64 = 627;
pub const METHOD_GETBODY_10: u64 = 628;
pub const METHOD_GETREQUIREMENTS: u64 = 629;
pub const METHOD_ISSATISFIED_1: u64 = 630;
pub const METHOD_GETREQUIRESKWLOC: u64 = 631;
pub const METHOD_GETLPARENLOC_8: u64 = 632;
pub const METHOD_GETRPARENLOC_21: u64 = 633;
pub const METHOD_GETRBRACELOC_5: u64 = 634;
pub const METHOD_GETBEGINLOC_108: u64 = 635;
pub const METHOD_GETENDLOC_107: u64 = 636;
pub const METHOD_CHILDREN_95: u64 = 637;
pub const CLASS_DEPENDENTCOAWAITEXPR: u64 = 638;
pub const METHOD_GETOPERAND_3: u64 = 639;
pub const METHOD_GETOPERATORCOAWAITLOOKUP: u64 = 640;
pub const METHOD_GETKEYWORDLOC_2: u64 = 641;
pub const METHOD_GETBEGINLOC_66: u64 = 642;
pub const METHOD_GETENDLOC_65: u64 = 643;
pub const METHOD_CHILDREN_56: u64 = 644;
pub const CLASS_LAMBDAEXPR: u64 = 645;
pub const METHOD_GETCAPTUREDEFAULT: u64 = 646;
pub const METHOD_GETCAPTUREDEFAULTLOC: u64 = 647;
pub const METHOD_CAPTURES_3: u64 = 648;
pub const METHOD_CAPTURE_BEGIN: u64 = 649;
pub const METHOD_CAPTURE_END: u64 = 650;
pub const METHOD_CAPTURE_SIZE_1: u64 = 651;
pub const METHOD_EXPLICIT_CAPTURES: u64 = 652;
pub const METHOD_EXPLICIT_CAPTURE_BEGIN: u64 = 653;
pub const METHOD_EXPLICIT_CAPTURE_END: u64 = 654;
pub const METHOD_IMPLICIT_CAPTURES: u64 = 655;
pub const METHOD_IMPLICIT_CAPTURE_BEGIN: u64 = 656;
pub const METHOD_IMPLICIT_CAPTURE_END: u64 = 657;
pub const METHOD_CAPTURE_INITS_1: u64 = 658;
pub const METHOD_CAPTURE_INIT_BEGIN: u64 = 659;
pub const METHOD_CAPTURE_INIT_END: u64 = 660;
pub const METHOD_GETINTRODUCERRANGE: u64 = 661;
pub const METHOD_GETLAMBDACLASS: u64 = 662;
pub const METHOD_GETCALLOPERATOR: u64 = 663;
pub const METHOD_GETDEPENDENTCALLOPERATOR: u64 = 664;
pub const METHOD_GETTEMPLATEPARAMETERLIST: u64 = 665;
pub const METHOD_GETEXPLICITTEMPLATEPARAMETERS: u64 = 666;
pub const METHOD_GETTRAILINGREQUIRESCLAUSE_1: u64 = 667;
pub const METHOD_ISGENERICLAMBDA_1: u64 = 668;
pub const METHOD_GETBODY_9: u64 = 669;
pub const METHOD_GETCOMPOUNDSTMTBODY: u64 = 670;
pub const METHOD_ISMUTABLE_1: u64 = 671;
pub const METHOD_HASEXPLICITPARAMETERS: u64 = 672;
pub const METHOD_HASEXPLICITRESULTTYPE: u64 = 673;
pub const METHOD_GETBEGINLOC_90: u64 = 674;
pub const METHOD_GETENDLOC_89: u64 = 675;
pub const METHOD_CHILDREN_78: u64 = 676;
pub const CLASS_COYIELDEXPR: u64 = 677;
pub const CLASS_COAWAITEXPR: u64 = 678;
pub const METHOD_ISIMPLICIT_2: u64 = 679;
pub const CLASS_CXXFOLDEXPR: u64 = 680;
pub const METHOD_GETCALLEE: u64 = 681;
pub const METHOD_GETLHS_2: u64 = 682;
pub const METHOD_GETRHS_2: u64 = 683;
pub const METHOD_ISRIGHTFOLD: u64 = 684;
pub const METHOD_ISLEFTFOLD: u64 = 685;
pub const METHOD_GETPATTERN_1: u64 = 686;
pub const METHOD_GETINIT_1: u64 = 687;
pub const METHOD_GETLPARENLOC_1: u64 = 688;
pub const METHOD_GETRPARENLOC_5: u64 = 689;
pub const METHOD_GETELLIPSISLOC_4: u64 = 690;
pub const METHOD_GETOPERATOR: u64 = 691;
pub const METHOD_GETNUMEXPANSIONS_1: u64 = 692;
pub const METHOD_GETBEGINLOC_27: u64 = 693;
pub const METHOD_GETENDLOC_26: u64 = 694;
pub const METHOD_CHILDREN_21: u64 = 695;
pub const CLASS_MATERIALIZETEMPORARYEXPR: u64 = 696;
pub const METHOD_GETSUBEXPR_7: u64 = 697;
pub const METHOD_GETSTORAGEDURATION_2: u64 = 698;
pub const METHOD_GETLIFETIMEEXTENDEDTEMPORARYDECL: u64 = 699;
pub const METHOD_GETEXTENDINGDECL_1: u64 = 700;
pub const METHOD_GETMANGLINGNUMBER_1: u64 = 701;
pub const METHOD_ISBOUNDTOLVALUEREFERENCE: u64 = 702;
pub const METHOD_GETBEGINLOC_95: u64 = 703;
pub const METHOD_GETENDLOC_94: u64 = 704;
pub const METHOD_CHILDREN_83: u64 = 705;
pub const CLASS_CXXDEPENDENTSCOPEMEMBEREXPR: u64 = 706;
pub const METHOD_ISIMPLICITACCESS: u64 = 707;
pub const METHOD_GETBASE_7: u64 = 708;
pub const METHOD_GETBASETYPE_1: u64 = 709;
pub const METHOD_ISARROW: u64 = 710;
pub const METHOD_GETOPERATORLOC_1: u64 = 711;
pub const METHOD_GETQUALIFIER_11: u64 = 712;
pub const METHOD_GETQUALIFIERLOC_8: u64 = 713;
pub const METHOD_GETFIRSTQUALIFIERFOUNDINSCOPE: u64 = 714;
pub const METHOD_GETMEMBERNAMEINFO: u64 = 715;
pub const METHOD_GETMEMBER: u64 = 716;
pub const METHOD_GETMEMBERLOC: u64 = 717;
pub const METHOD_GETTEMPLATEKEYWORDLOC_2: u64 = 718;
pub const METHOD_GETLANGLELOC: u64 = 719;
pub const METHOD_GETRANGLELOC: u64 = 720;
pub const METHOD_HASTEMPLATEKEYWORD: u64 = 721;
pub const METHOD_HASEXPLICITTEMPLATEARGS: u64 = 722;
pub const METHOD_GETTEMPLATEARGS_2: u64 = 723;
pub const METHOD_GETNUMTEMPLATEARGS: u64 = 724;
pub const METHOD_TEMPLATE_ARGUMENTS_2: u64 = 725;
pub const METHOD_GETBEGINLOC_26: u64 = 726;
pub const METHOD_GETENDLOC_25: u64 = 727;
pub const METHOD_CHILDREN_20: u64 = 728;
pub const CLASS_OMPDEPOBJDIRECTIVE: u64 = 729;
pub const CLASS_UNRESOLVEDMEMBEREXPR: u64 = 730;
pub const METHOD_ISIMPLICITACCESS_3: u64 = 731;
pub const METHOD_GETBASE_8: u64 = 732;
pub const METHOD_GETBASETYPE_2: u64 = 733;
pub const METHOD_HASUNRESOLVEDUSING: u64 = 734;
pub const METHOD_ISARROW_5: u64 = 735;
pub const METHOD_GETOPERATORLOC_11: u64 = 736;
pub const METHOD_GETNAMINGCLASS_2: u64 = 737;
pub const METHOD_GETMEMBERNAMEINFO_2: u64 = 738;
pub const METHOD_GETMEMBERNAME: u64 = 739;
pub const METHOD_GETMEMBERLOC_3: u64 = 740;
pub const METHOD_GETEXPRLOC_13: u64 = 741;
pub const METHOD_GETBEGINLOC_129: u64 = 742;
pub const METHOD_GETENDLOC_128: u64 = 743;
pub const METHOD_CHILDREN_116: u64 = 744;
pub const CLASS_CXXNEWEXPR: u64 = 745;
pub const METHOD_GETALLOCATEDTYPE: u64 = 746;
pub const METHOD_GETALLOCATEDTYPESOURCEINFO: u64 = 747;
pub const METHOD_SHOULDNULLCHECKALLOCATION: u64 = 748;
pub const METHOD_GETOPERATORNEW: u64 = 749;
pub const METHOD_GETOPERATORDELETE_2: u64 = 750;
pub const METHOD_ISARRAY: u64 = 751;
pub const METHOD_GETARRAYSIZE_1: u64 = 752;
pub const METHOD_GETNUMPLACEMENTARGS: u64 = 753;
pub const METHOD_ISPARENTYPEID: u64 = 754;
pub const METHOD_GETTYPEIDPARENS: u64 = 755;
pub const METHOD_ISGLOBALNEW: u64 = 756;
pub const METHOD_HASINITIALIZER: u64 = 757;
pub const METHOD_GETINITIALIZATIONSTYLE: u64 = 758;
pub const METHOD_GETINITIALIZER: u64 = 759;
pub const METHOD_GETCONSTRUCTEXPR: u64 = 760;
pub const METHOD_PASSALIGNMENT: u64 = 761;
pub const METHOD_DOESUSUALARRAYDELETEWANTSIZE_1: u64 = 762;
pub const METHOD_PLACEMENT_ARGUMENTS: u64 = 763;
pub const METHOD_PLACEMENT_ARG_BEGIN: u64 = 764;
pub const METHOD_PLACEMENT_ARG_END: u64 = 765;
pub const METHOD_RAW_ARG_BEGIN: u64 = 766;
pub const METHOD_RAW_ARG_END: u64 = 767;
pub const METHOD_GETBEGINLOC_32: u64 = 768;
pub const METHOD_GETENDLOC_31: u64 = 769;
pub const METHOD_GETDIRECTINITRANGE: u64 = 770;
pub const METHOD_GETSOURCERANGE_39: u64 = 771;
pub const METHOD_CHILDREN_24: u64 = 772;
pub const CLASS_IMPLICITCASTEXPR: u64 = 773;
pub const METHOD_ISPARTOFEXPLICITCAST: u64 = 774;
pub const METHOD_GETBEGINLOC_84: u64 = 775;
pub const METHOD_GETENDLOC_83: u64 = 776;
pub const CLASS_CXXCONSTRUCTEXPR: u64 = 777;
pub const METHOD_GETCONSTRUCTOR: u64 = 778;
pub const METHOD_GETLOCATION_2: u64 = 779;
pub const METHOD_ISELIDABLE: u64 = 780;
pub const METHOD_HADMULTIPLECANDIDATES: u64 = 781;
pub const METHOD_ISLISTINITIALIZATION: u64 = 782;
pub const METHOD_ISSTDINITLISTINITIALIZATION: u64 = 783;
pub const METHOD_REQUIRESZEROINITIALIZATION: u64 = 784;
pub const METHOD_GETCONSTRUCTIONKIND: u64 = 785;
pub const METHOD_ARGUMENTS: u64 = 786;
pub const METHOD_ARG_BEGIN_1: u64 = 787;
pub const METHOD_ARG_END_1: u64 = 788;
pub const METHOD_GETARGS: u64 = 789;
pub const METHOD_GETNUMARGS_1: u64 = 790;
pub const METHOD_ISIMMEDIATEESCALATING_1: u64 = 791;
pub const METHOD_GETBEGINLOC_22: u64 = 792;
pub const METHOD_GETENDLOC_21: u64 = 793;
pub const METHOD_GETPARENORBRACERANGE: u64 = 794;
pub const METHOD_CHILDREN_16: u64 = 795;
pub const CLASS_COMPOUNDLITERALEXPR: u64 = 796;
pub const METHOD_GETINITIALIZER_1: u64 = 797;
pub const METHOD_ISFILESCOPE: u64 = 798;
pub const METHOD_GETLPARENLOC_4: u64 = 799;
pub const METHOD_GETTYPESOURCEINFO_5: u64 = 800;
pub const METHOD_GETBEGINLOC_53: u64 = 801;
pub const METHOD_GETENDLOC_52: u64 = 802;
pub const METHOD_CHILDREN_43: u64 = 803;
pub const CLASS_CXXBINDTEMPORARYEXPR: u64 = 804;
pub const METHOD_GETTEMPORARY: u64 = 805;
pub const METHOD_GETSUBEXPR_1: u64 = 806;
pub const METHOD_GETBEGINLOC_19: u64 = 807;
pub const METHOD_GETENDLOC_18: u64 = 808;
pub const METHOD_CHILDREN_13: u64 = 809;
pub const CLASS_CXXDEFAULTARGEXPR: u64 = 810;
pub const METHOD_GETPARAM: u64 = 811;
pub const METHOD_HASREWRITTENINIT: u64 = 812;
pub const METHOD_GETEXPR: u64 = 813;
pub const METHOD_GETREWRITTENEXPR: u64 = 814;
pub const METHOD_GETADJUSTEDREWRITTENEXPR: u64 = 815;
pub const METHOD_GETUSEDCONTEXT: u64 = 816;
pub const METHOD_GETUSEDLOCATION: u64 = 817;
pub const METHOD_GETBEGINLOC_23: u64 = 818;
pub const METHOD_GETENDLOC_22: u64 = 819;
pub const METHOD_GETEXPRLOC_2: u64 = 820;
pub const METHOD_CHILDREN_17: u64 = 821;
pub const CLASS_OMPSECTIONDIRECTIVE: u64 = 822;
pub const CLASS_CXXTHROWEXPR: u64 = 823;
pub const METHOD_GETSUBEXPR_3: u64 = 824;
pub const METHOD_GETTHROWLOC: u64 = 825;
pub const METHOD_ISTHROWNVARIABLEINSCOPE: u64 = 826;
pub const METHOD_GETBEGINLOC_43: u64 = 827;
pub const METHOD_GETENDLOC_42: u64 = 828;
pub const METHOD_CHILDREN_32: u64 = 829;
pub const CLASS_MSPROPERTYSUBSCRIPTEXPR: u64 = 830;
pub const METHOD_GETBASE_4: u64 = 831;
pub const METHOD_GETIDX_1: u64 = 832;
pub const METHOD_GETBEGINLOC_94: u64 = 833;
pub const METHOD_GETENDLOC_93: u64 = 834;
pub const METHOD_GETRBRACKETLOC_3: u64 = 835;
pub const METHOD_GETEXPRLOC_7: u64 = 836;
pub const METHOD_CHILDREN_82: u64 = 837;
pub const CLASS_CXXREWRITTENBINARYOPERATOR: u64 = 838;
pub const METHOD_GETSEMANTICFORM: u64 = 839;
pub const METHOD_GETDECOMPOSEDFORM: u64 = 840;
pub const METHOD_ISREVERSED: u64 = 841;
pub const METHOD_GETOPERATOR_2: u64 = 842;
pub const METHOD_GETOPCODE_1: u64 = 843;
pub const METHOD_GETOPCODESTR_1: u64 = 844;
pub const METHOD_ISCOMPARISONOP_2: u64 = 845;
pub const METHOD_ISASSIGNMENTOP_2: u64 = 846;
pub const METHOD_GETLHS_3: u64 = 847;
pub const METHOD_GETRHS_3: u64 = 848;
pub const METHOD_GETOPERATORLOC_5: u64 = 849;
pub const METHOD_GETEXPRLOC_5: u64 = 850;
pub const METHOD_GETBEGINLOC_38: u64 = 851;
pub const METHOD_GETENDLOC_37: u64 = 852;
pub const METHOD_GETSOURCERANGE_43: u64 = 853;
pub const CLASS_CUDAKERNELCALLEXPR: u64 = 854;
pub const METHOD_GETCONFIG: u64 = 855;
pub const CLASS_TYPOEXPR: u64 = 856;
pub const METHOD_CHILDREN_112: u64 = 857;
pub const METHOD_GETBEGINLOC_125: u64 = 858;
pub const METHOD_GETENDLOC_124: u64 = 859;
pub const CLASS_ASTYPEEXPR: u64 = 860;
pub const METHOD_GETSRCEXPR: u64 = 861;
pub const METHOD_GETBUILTINLOC: u64 = 862;
pub const METHOD_GETRPARENLOC_2: u64 = 863;
pub const METHOD_GETBEGINLOC_9: u64 = 864;
pub const METHOD_GETENDLOC_8: u64 = 865;
pub const METHOD_CHILDREN_5: u64 = 866;
pub const CLASS_EXTVECTORELEMENTEXPR: u64 = 867;
pub const METHOD_GETBASE_3: u64 = 868;
pub const METHOD_GETACCESSOR: u64 = 869;
pub const METHOD_GETACCESSORLOC: u64 = 870;
pub const METHOD_GETNUMELEMENTS_1: u64 = 871;
pub const METHOD_CONTAINSDUPLICATEELEMENTS: u64 = 872;
pub const METHOD_GETBEGINLOC_73: u64 = 873;
pub const METHOD_GETENDLOC_72: u64 = 874;
pub const METHOD_ISARROW_2: u64 = 875;
pub const METHOD_CHILDREN_63: u64 = 876;
pub const CLASS_PARENLISTEXPR: u64 = 877;
pub const METHOD_GETNUMEXPRS: u64 = 878;
pub const METHOD_GETLPARENLOC_7: u64 = 879;
pub const METHOD_GETRPARENLOC_20: u64 = 880;
pub const METHOD_GETBEGINLOC_104: u64 = 881;
pub const METHOD_GETENDLOC_103: u64 = 882;
pub const METHOD_CHILDREN_92: u64 = 883;
pub const CLASS_NOINITEXPR: u64 = 884;
pub const METHOD_GETBEGINLOC_98: u64 = 885;
pub const METHOD_GETENDLOC_97: u64 = 886;
pub const METHOD_CHILDREN_86: u64 = 887;
pub const CLASS_ARRAYINITINDEXEXPR: u64 = 888;
pub const METHOD_GETBEGINLOC_5: u64 = 889;
pub const METHOD_GETENDLOC_4: u64 = 890;
pub const METHOD_CHILDREN_1: u64 = 891;
pub const CLASS_CONVERTVECTOREXPR: u64 = 892;
pub const METHOD_GETSRCEXPR_1: u64 = 893;
pub const METHOD_GETTYPESOURCEINFO_6: u64 = 894;
pub const METHOD_GETBUILTINLOC_3: u64 = 895;
pub const METHOD_GETRPARENLOC_13: u64 = 896;
pub const METHOD_GETBEGINLOC_59: u64 = 897;
pub const METHOD_GETENDLOC_58: u64 = 898;
pub const METHOD_CHILDREN_49: u64 = 899;
pub const CLASS_CONDITIONALOPERATOR: u64 = 900;
pub const METHOD_GETCOND_4: u64 = 901;
pub const METHOD_GETTRUEEXPR_2: u64 = 902;
pub const METHOD_GETFALSEEXPR_2: u64 = 903;
pub const METHOD_GETLHS_6: u64 = 904;
pub const METHOD_GETRHS_6: u64 = 905;
pub const METHOD_GETBEGINLOC_56: u64 = 906;
pub const METHOD_GETENDLOC_55: u64 = 907;
pub const METHOD_CHILDREN_46: u64 = 908;
pub const CLASS_BINARYOPERATOR: u64 = 909;
pub const METHOD_GETEXPRLOC_1: u64 = 910;
pub const METHOD_GETOPERATORLOC: u64 = 911;
pub const METHOD_GETOPCODE: u64 = 912;
pub const METHOD_GETLHS_1: u64 = 913;
pub const METHOD_GETRHS_1: u64 = 914;
pub const METHOD_GETBEGINLOC_14: u64 = 915;
pub const METHOD_GETENDLOC_13: u64 = 916;
pub const METHOD_GETOPCODESTR: u64 = 917;
pub const METHOD_ISPTRMEMOP: u64 = 918;
pub const METHOD_ISMULTIPLICATIVEOP: u64 = 919;
pub const METHOD_ISADDITIVEOP: u64 = 920;
pub const METHOD_ISSHIFTOP: u64 = 921;
pub const METHOD_ISBITWISEOP: u64 = 922;
pub const METHOD_ISRELATIONALOP: u64 = 923;
pub const METHOD_ISEQUALITYOP: u64 = 924;
pub const METHOD_ISCOMPARISONOP: u64 = 925;
pub const METHOD_ISCOMMAOP: u64 = 926;
pub const METHOD_ISLOGICALOP: u64 = 927;
pub const METHOD_ISASSIGNMENTOP: u64 = 928;
pub const METHOD_ISCOMPOUNDASSIGNMENTOP: u64 = 929;
pub const METHOD_ISSHIFTASSIGNOP: u64 = 930;
pub const METHOD_CHILDREN_10: u64 = 931;
pub const METHOD_HASSTOREDFPFEATURES: u64 = 932;
pub const METHOD_GETSTOREDFPFEATURES: u64 = 933;
pub const METHOD_GETFPFEATURES: u64 = 934;
pub const CLASS_CXXFUNCTIONALCASTEXPR: u64 = 935;
pub const METHOD_GETLPARENLOC_2: u64 = 936;
pub const METHOD_GETRPARENLOC_7: u64 = 937;
pub const METHOD_ISLISTINITIALIZATION_1: u64 = 938;
pub const METHOD_GETBEGINLOC_29: u64 = 939;
pub const METHOD_GETENDLOC_28: u64 = 940;
pub const CLASS_UNARYOPERATOR: u64 = 941;
pub const METHOD_GETOPCODE_2: u64 = 942;
pub const METHOD_GETSUBEXPR_9: u64 = 943;
pub const METHOD_GETOPERATORLOC_10: u64 = 944;
pub const METHOD_CANOVERFLOW: u64 = 945;
pub const METHOD_ISPREFIX: u64 = 946;
pub const METHOD_ISPOSTFIX: u64 = 947;
pub const METHOD_ISINCREMENTOP: u64 = 948;
pub const METHOD_ISDECREMENTOP: u64 = 949;
pub const METHOD_ISINCREMENTDECREMENTOP: u64 = 950;
pub const METHOD_ISARITHMETICOP: u64 = 951;
pub const METHOD_GETBEGINLOC_127: u64 = 952;
pub const METHOD_GETENDLOC_126: u64 = 953;
pub const METHOD_GETEXPRLOC_12: u64 = 954;
pub const METHOD_CHILDREN_114: u64 = 955;
pub const METHOD_HASSTOREDFPFEATURES_4: u64 = 956;
pub const METHOD_GETSTOREDFPFEATURES_4: u64 = 957;
pub const METHOD_GETFPOPTIONSOVERRIDE: u64 = 958;
pub const CLASS_OBJCDICTIONARYLITERAL: u64 = 959;
pub const CLASS_ATTRIBUTEDSTMT: u64 = 960;
pub const METHOD_GETATTRLOC: u64 = 961;
pub const METHOD_GETATTRS_1: u64 = 962;
pub const METHOD_GETSUBSTMT: u64 = 963;
pub const METHOD_GETBEGINLOC_12: u64 = 964;
pub const METHOD_GETENDLOC_11: u64 = 965;
pub const METHOD_CHILDREN_8: u64 = 966;
pub const CLASS_CXXSTATICCASTEXPR: u64 = 967;
pub const CLASS_WHILESTMT: u64 = 968;
pub const METHOD_HASVARSTORAGE_2: u64 = 969;
pub const METHOD_GETCOND_9: u64 = 970;
pub const METHOD_GETBODY_12: u64 = 971;
pub const METHOD_GETCONDITIONVARIABLE_3: u64 = 972;
pub const METHOD_GETCONDITIONVARIABLEDECLSTMT_3: u64 = 973;
pub const METHOD_GETWHILELOC_1: u64 = 974;
pub const METHOD_GETLPARENLOC_11: u64 = 975;
pub const METHOD_GETRPARENLOC_28: u64 = 976;
pub const METHOD_GETBEGINLOC_132: u64 = 977;
pub const METHOD_GETENDLOC_131: u64 = 978;
pub const METHOD_CHILDREN_118: u64 = 979;
pub const CLASS_CXXSCALARVALUEINITEXPR: u64 = 980;
pub const METHOD_GETTYPESOURCEINFO_2: u64 = 981;
pub const METHOD_GETRPARENLOC_9: u64 = 982;
pub const METHOD_GETBEGINLOC_39: u64 = 983;
pub const METHOD_GETENDLOC_38: u64 = 984;
pub const METHOD_CHILDREN_29: u64 = 985;
pub const CLASS_CSTYLECASTEXPR: u64 = 986;
pub const METHOD_GETLPARENLOC: u64 = 987;
pub const METHOD_GETRPARENLOC_4: u64 = 988;
pub const METHOD_GETBEGINLOC_18: u64 = 989;
pub const METHOD_GETENDLOC_17: u64 = 990;
pub const CLASS_INTEGERLITERAL: u64 = 991;
pub const METHOD_GETBEGINLOC_88: u64 = 992;
pub const METHOD_GETENDLOC_87: u64 = 993;
pub const METHOD_GETLOCATION_11: u64 = 994;
pub const METHOD_CHILDREN_76: u64 = 995;
pub const CLASS_CXXCONSTCASTEXPR: u64 = 996;
pub const CLASS_CALLEXPR: u64 = 997;
pub const METHOD_GETCALLEE_1: u64 = 998;
pub const METHOD_GETADLCALLKIND: u64 = 999;
pub const METHOD_USESADL: u64 = 1000;
pub const METHOD_HASSTOREDFPFEATURES_1: u64 = 1001;
pub const METHOD_GETCALLEEDECL: u64 = 1002;
pub const METHOD_GETDIRECTCALLEE: u64 = 1003;
pub const METHOD_GETNUMARGS_3: u64 = 1004;
pub const METHOD_GETARGS_1: u64 = 1005;
pub const METHOD_ARGUMENTS_2: u64 = 1006;
pub const METHOD_ARG_BEGIN_2: u64 = 1007;
pub const METHOD_ARG_END_2: u64 = 1008;
pub const METHOD_GETSTOREDFPFEATURES_1: u64 = 1009;
pub const METHOD_GETFPFEATURES_1: u64 = 1010;
pub const METHOD_GETBUILTINCALLEE: u64 = 1011;
pub const METHOD_GETRPARENLOC_11: u64 = 1012;
pub const METHOD_GETBEGINLOC_48: u64 = 1013;
pub const METHOD_GETENDLOC_47: u64 = 1014;
pub const METHOD_ISCALLTOSTDMOVE: u64 = 1015;
pub const METHOD_CHILDREN_37: u64 = 1016;
pub const CLASS_PREDEFINEDEXPR: u64 = 1017;
pub const METHOD_GETIDENTKIND: u64 = 1018;
pub const METHOD_ISTRANSPARENT_1: u64 = 1019;
pub const METHOD_GETLOCATION_13: u64 = 1020;
pub const METHOD_GETFUNCTIONNAME: u64 = 1021;
pub const METHOD_GETIDENTKINDNAME: u64 = 1022;
pub const METHOD_GETBEGINLOC_105: u64 = 1023;
pub const METHOD_GETENDLOC_104: u64 = 1024;
pub const METHOD_CHILDREN_93: u64 = 1025;
pub const CLASS_STRINGLITERAL: u64 = 1026;
pub const METHOD_GETSTRING: u64 = 1027;
pub const METHOD_GETBYTES: u64 = 1028;
pub const METHOD_GETBYTELENGTH: u64 = 1029;
pub const METHOD_GETLENGTH: u64 = 1030;
pub const METHOD_GETCHARBYTEWIDTH: u64 = 1031;
pub const METHOD_GETKIND_4: u64 = 1032;
pub const METHOD_ISORDINARY: u64 = 1033;
pub const METHOD_ISWIDE: u64 = 1034;
pub const METHOD_ISUTF8: u64 = 1035;
pub const METHOD_ISUTF16: u64 = 1036;
pub const METHOD_ISUTF32: u64 = 1037;
pub const METHOD_ISUNEVALUATED: u64 = 1038;
pub const METHOD_ISPASCAL: u64 = 1039;
pub const METHOD_CONTAINSNONASCII: u64 = 1040;
pub const METHOD_CONTAINSNONASCIIORNULL: u64 = 1041;
pub const METHOD_GETNUMCONCATENATED: u64 = 1042;
pub const METHOD_TOKLOC_BEGIN: u64 = 1043;
pub const METHOD_TOKLOC_END: u64 = 1044;
pub const METHOD_GETBEGINLOC_119: u64 = 1045;
pub const METHOD_GETENDLOC_118: u64 = 1046;
pub const METHOD_CHILDREN_107: u64 = 1047;
pub const CLASS_FLOATINGLITERAL: u64 = 1048;
pub const METHOD_GETVALUE_8: u64 = 1049;
pub const METHOD_GETRAWSEMANTICS: u64 = 1050;
pub const METHOD_GETSEMANTICS: u64 = 1051;
pub const METHOD_ISEXACT: u64 = 1052;
pub const METHOD_GETVALUEASAPPROXIMATEDOUBLE: u64 = 1053;
pub const METHOD_GETLOCATION_10: u64 = 1054;
pub const METHOD_GETBEGINLOC_75: u64 = 1055;
pub const METHOD_GETENDLOC_74: u64 = 1056;
pub const METHOD_CHILDREN_65: u64 = 1057;
pub const CLASS_OPAQUEVALUEEXPR: u64 = 1058;
pub const METHOD_GETLOCATION_12: u64 = 1059;
pub const METHOD_GETBEGINLOC_101: u64 = 1060;
pub const METHOD_GETENDLOC_100: u64 = 1061;
pub const METHOD_GETEXPRLOC_10: u64 = 1062;
pub const METHOD_CHILDREN_89: u64 = 1063;
pub const METHOD_GETSOURCEEXPR: u64 = 1064;
pub const METHOD_ISUNIQUE: u64 = 1065;
pub const CLASS_DESIGNATEDINITUPDATEEXPR: u64 = 1066;
pub const METHOD_GETBEGINLOC_69: u64 = 1067;
pub const METHOD_GETENDLOC_68: u64 = 1068;
pub const METHOD_GETBASE_2: u64 = 1069;
pub const METHOD_GETUPDATER: u64 = 1070;
pub const METHOD_CHILDREN_59: u64 = 1071;
pub const CLASS_FULLEXPR: u64 = 1072;
pub const METHOD_GETSUBEXPR_5: u64 = 1073;
pub const CLASS_EXPRWITHCLEANUPS: u64 = 1074;
pub const METHOD_GETOBJECTS: u64 = 1075;
pub const METHOD_GETNUMOBJECTS: u64 = 1076;
pub const METHOD_CLEANUPSHAVESIDEEFFECTS: u64 = 1077;
pub const METHOD_GETBEGINLOC_71: u64 = 1078;
pub const METHOD_GETENDLOC_70: u64 = 1079;
pub const METHOD_CHILDREN_61: u64 = 1080;
pub const CLASS_LABELSTMT: u64 = 1081;
pub const METHOD_GETIDENTLOC: u64 = 1082;
pub const METHOD_GETDECL_8: u64 = 1083;
pub const METHOD_GETNAME_1: u64 = 1084;
pub const METHOD_GETSUBSTMT_3: u64 = 1085;
pub const METHOD_GETBEGINLOC_89: u64 = 1086;
pub const METHOD_GETENDLOC_88: u64 = 1087;
pub const METHOD_CHILDREN_77: u64 = 1088;
pub const METHOD_ISSIDEENTRY: u64 = 1089;
pub const CLASS_CXXRECORDDECL: u64 = 1090;
pub const METHOD_GETCANONICALDECL_4: u64 = 1091;
pub const METHOD_GETPREVIOUSDECL: u64 = 1092;
pub const METHOD_GETMOSTRECENTDECL_1: u64 = 1093;
pub const METHOD_GETMOSTRECENTNONINJECTEDDECL: u64 = 1094;
pub const METHOD_GETDEFINITION: u64 = 1095;
pub const METHOD_HASDEFINITION: u64 = 1096;
pub const METHOD_ISDYNAMICCLASS: u64 = 1097;
pub const METHOD_MAYBEDYNAMICCLASS: u64 = 1098;
pub const METHOD_MAYBENONDYNAMICCLASS: u64 = 1099;
pub const METHOD_ISPARSINGBASESPECIFIERS: u64 = 1100;
pub const METHOD_GETODRHASH: u64 = 1101;
pub const METHOD_GETNUMBASES: u64 = 1102;
pub const METHOD_BASES: u64 = 1103;
pub const METHOD_BASES_BEGIN: u64 = 1104;
pub const METHOD_BASES_END: u64 = 1105;
pub const METHOD_GETNUMVBASES: u64 = 1106;
pub const METHOD_VBASES: u64 = 1107;
pub const METHOD_VBASES_BEGIN: u64 = 1108;
pub const METHOD_VBASES_END: u64 = 1109;
pub const METHOD_HASANYDEPENDENTBASES: u64 = 1110;
pub const METHOD_METHODS: u64 = 1111;
pub const METHOD_METHOD_BEGIN: u64 = 1112;
pub const METHOD_METHOD_END: u64 = 1113;
pub const METHOD_CTORS: u64 = 1114;
pub const METHOD_CTOR_BEGIN: u64 = 1115;
pub const METHOD_CTOR_END: u64 = 1116;
pub const METHOD_FRIENDS: u64 = 1117;
pub const METHOD_FRIEND_BEGIN: u64 = 1118;
pub const METHOD_FRIEND_END: u64 = 1119;
pub const METHOD_HASFRIENDS: u64 = 1120;
pub const METHOD_DEFAULTEDCOPYCONSTRUCTORISDELETED: u64 = 1121;
pub const METHOD_DEFAULTEDMOVECONSTRUCTORISDELETED: u64 = 1122;
pub const METHOD_DEFAULTEDDESTRUCTORISDELETED: u64 = 1123;
pub const METHOD_HASSIMPLECOPYCONSTRUCTOR: u64 = 1124;
pub const METHOD_HASSIMPLEMOVECONSTRUCTOR: u64 = 1125;
pub const METHOD_HASSIMPLECOPYASSIGNMENT: u64 = 1126;
pub const METHOD_HASSIMPLEMOVEASSIGNMENT: u64 = 1127;
pub const METHOD_HASSIMPLEDESTRUCTOR: u64 = 1128;
pub const METHOD_HASDEFAULTCONSTRUCTOR: u64 = 1129;
pub const METHOD_NEEDSIMPLICITDEFAULTCONSTRUCTOR: u64 = 1130;
pub const METHOD_HASUSERDECLAREDCONSTRUCTOR: u64 = 1131;
pub const METHOD_HASUSERPROVIDEDDEFAULTCONSTRUCTOR: u64 = 1132;
pub const METHOD_HASUSERDECLAREDCOPYCONSTRUCTOR: u64 = 1133;
pub const METHOD_NEEDSIMPLICITCOPYCONSTRUCTOR: u64 = 1134;
pub const METHOD_NEEDSOVERLOADRESOLUTIONFORCOPYCONSTRUCTOR: u64 = 1135;
pub const METHOD_IMPLICITCOPYCONSTRUCTORHASCONSTPARAM: u64 = 1136;
pub const METHOD_HASCOPYCONSTRUCTORWITHCONSTPARAM: u64 = 1137;
pub const METHOD_HASUSERDECLAREDMOVEOPERATION: u64 = 1138;
pub const METHOD_HASUSERDECLAREDMOVECONSTRUCTOR: u64 = 1139;
pub const METHOD_HASMOVECONSTRUCTOR: u64 = 1140;
pub const METHOD_NEEDSIMPLICITMOVECONSTRUCTOR: u64 = 1141;
pub const METHOD_NEEDSOVERLOADRESOLUTIONFORMOVECONSTRUCTOR: u64 = 1142;
pub const METHOD_HASUSERDECLAREDCOPYASSIGNMENT: u64 = 1143;
pub const METHOD_NEEDSIMPLICITCOPYASSIGNMENT: u64 = 1144;
pub const METHOD_NEEDSOVERLOADRESOLUTIONFORCOPYASSIGNMENT: u64 = 1145;
pub const METHOD_IMPLICITCOPYASSIGNMENTHASCONSTPARAM: u64 = 1146;
pub const METHOD_HASCOPYASSIGNMENTWITHCONSTPARAM: u64 = 1147;
pub const METHOD_HASUSERDECLAREDMOVEASSIGNMENT: u64 = 1148;
pub const METHOD_HASMOVEASSIGNMENT: u64 = 1149;
pub const METHOD_NEEDSIMPLICITMOVEASSIGNMENT: u64 = 1150;
pub const METHOD_NEEDSOVERLOADRESOLUTIONFORMOVEASSIGNMENT: u64 = 1151;
pub const METHOD_HASUSERDECLAREDDESTRUCTOR: u64 = 1152;
pub const METHOD_NEEDSIMPLICITDESTRUCTOR: u64 = 1153;
pub const METHOD_NEEDSOVERLOADRESOLUTIONFORDESTRUCTOR: u64 = 1154;
pub const METHOD_ISLAMBDA: u64 = 1155;
pub const METHOD_ISGENERICLAMBDA: u64 = 1156;
pub const METHOD_LAMBDAISDEFAULTCONSTRUCTIBLEANDASSIGNABLE: u64 = 1157;
pub const METHOD_GETLAMBDACALLOPERATOR: u64 = 1158;
pub const METHOD_GETDEPENDENTLAMBDACALLOPERATOR: u64 = 1159;
pub const METHOD_GETLAMBDASTATICINVOKER: u64 = 1160;
pub const METHOD_GETGENERICLAMBDATEMPLATEPARAMETERLIST: u64 = 1161;
pub const METHOD_GETLAMBDAEXPLICITTEMPLATEPARAMETERS: u64 = 1162;
pub const METHOD_GETLAMBDACAPTUREDEFAULT: u64 = 1163;
pub const METHOD_ISCAPTURELESSLAMBDA: u64 = 1164;
pub const METHOD_CAPTURES_1: u64 = 1165;
pub const METHOD_CAPTURES_BEGIN: u64 = 1166;
pub const METHOD_CAPTURES_END: u64 = 1167;
pub const METHOD_CAPTURE_SIZE_2: u64 = 1168;
pub const METHOD_CONVERSION_BEGIN: u64 = 1169;
pub const METHOD_CONVERSION_END: u64 = 1170;
pub const METHOD_GETVISIBLECONVERSIONFUNCTIONS: u64 = 1171;
pub const METHOD_ISAGGREGATE: u64 = 1172;
pub const METHOD_HASINCLASSINITIALIZER: u64 = 1173;
pub const METHOD_HASUNINITIALIZEDREFERENCEMEMBER: u64 = 1174;
pub const METHOD_ISPOD: u64 = 1175;
pub const METHOD_ISCLIKE: u64 = 1176;
pub const METHOD_ISEMPTY: u64 = 1177;
pub const METHOD_HASINITMETHOD: u64 = 1178;
pub const METHOD_HASPRIVATEFIELDS: u64 = 1179;
pub const METHOD_HASPROTECTEDFIELDS: u64 = 1180;
pub const METHOD_HASDIRECTFIELDS: u64 = 1181;
pub const METHOD_ISPOLYMORPHIC: u64 = 1182;
pub const METHOD_ISABSTRACT: u64 = 1183;
pub const METHOD_ISSTANDARDLAYOUT: u64 = 1184;
pub const METHOD_ISCXX11STANDARDLAYOUT: u64 = 1185;
pub const METHOD_HASMUTABLEFIELDS: u64 = 1186;
pub const METHOD_HASVARIANTMEMBERS: u64 = 1187;
pub const METHOD_HASTRIVIALDEFAULTCONSTRUCTOR: u64 = 1188;
pub const METHOD_HASNONTRIVIALDEFAULTCONSTRUCTOR: u64 = 1189;
pub const METHOD_HASCONSTEXPRNONCOPYMOVECONSTRUCTOR: u64 = 1190;
pub const METHOD_DEFAULTEDDEFAULTCONSTRUCTORISCONSTEXPR: u64 = 1191;
pub const METHOD_HASCONSTEXPRDEFAULTCONSTRUCTOR: u64 = 1192;
pub const METHOD_HASTRIVIALCOPYCONSTRUCTOR: u64 = 1193;
pub const METHOD_HASTRIVIALCOPYCONSTRUCTORFORCALL: u64 = 1194;
pub const METHOD_HASNONTRIVIALCOPYCONSTRUCTOR: u64 = 1195;
pub const METHOD_HASNONTRIVIALCOPYCONSTRUCTORFORCALL: u64 = 1196;
pub const METHOD_HASTRIVIALMOVECONSTRUCTOR: u64 = 1197;
pub const METHOD_HASTRIVIALMOVECONSTRUCTORFORCALL: u64 = 1198;
pub const METHOD_HASNONTRIVIALMOVECONSTRUCTOR: u64 = 1199;
pub const METHOD_HASNONTRIVIALMOVECONSTRUCTORFORCALL: u64 = 1200;
pub const METHOD_HASTRIVIALCOPYASSIGNMENT: u64 = 1201;
pub const METHOD_HASNONTRIVIALCOPYASSIGNMENT: u64 = 1202;
pub const METHOD_HASTRIVIALMOVEASSIGNMENT: u64 = 1203;
pub const METHOD_HASNONTRIVIALMOVEASSIGNMENT: u64 = 1204;
pub const METHOD_DEFAULTEDDESTRUCTORISCONSTEXPR: u64 = 1205;
pub const METHOD_HASCONSTEXPRDESTRUCTOR: u64 = 1206;
pub const METHOD_HASTRIVIALDESTRUCTOR: u64 = 1207;
pub const METHOD_HASTRIVIALDESTRUCTORFORCALL: u64 = 1208;
pub const METHOD_HASNONTRIVIALDESTRUCTOR: u64 = 1209;
pub const METHOD_HASNONTRIVIALDESTRUCTORFORCALL: u64 = 1210;
pub const METHOD_ALLOWCONSTDEFAULTINIT: u64 = 1211;
pub const METHOD_HASIRRELEVANTDESTRUCTOR: u64 = 1212;
pub const METHOD_HASNONLITERALTYPEFIELDSORBASES: u64 = 1213;
pub const METHOD_HASINHERITEDCONSTRUCTOR: u64 = 1214;
pub const METHOD_HASINHERITEDASSIGNMENT: u64 = 1215;
pub const METHOD_ISTRIVIALLYCOPYABLE: u64 = 1216;
pub const METHOD_ISTRIVIALLYCOPYCONSTRUCTIBLE: u64 = 1217;
pub const METHOD_ISTRIVIAL: u64 = 1218;
pub const METHOD_ISLITERAL: u64 = 1219;
pub const METHOD_ISSTRUCTURAL: u64 = 1220;
pub const METHOD_GETINSTANTIATEDFROMMEMBERCLASS: u64 = 1221;
pub const METHOD_GETMEMBERSPECIALIZATIONINFO: u64 = 1222;
pub const METHOD_GETDESCRIBEDCLASSTEMPLATE: u64 = 1223;
pub const METHOD_GETTEMPLATESPECIALIZATIONKIND: u64 = 1224;
pub const METHOD_GETTEMPLATEINSTANTIATIONPATTERN: u64 = 1225;
pub const METHOD_GETDESTRUCTOR: u64 = 1226;
pub const METHOD_ISANYDESTRUCTORNORETURN: u64 = 1227;
pub const METHOD_ISLOCALCLASS: u64 = 1228;
pub const METHOD_MAYBEABSTRACT: u64 = 1229;
pub const METHOD_ISEFFECTIVELYFINAL: u64 = 1230;
pub const METHOD_GETLAMBDAMANGLINGNUMBER: u64 = 1231;
pub const METHOD_HASKNOWNLAMBDAINTERNALLINKAGE: u64 = 1232;
pub const METHOD_GETLAMBDACONTEXTDECL: u64 = 1233;
pub const METHOD_GETLAMBDAINDEXINCONTEXT: u64 = 1234;
pub const METHOD_GETLAMBDANUMBERING: u64 = 1235;
pub const METHOD_GETDEVICELAMBDAMANGLINGNUMBER: u64 = 1236;
pub const METHOD_GETMSINHERITANCEMODEL: u64 = 1237;
pub const METHOD_CALCULATEINHERITANCEMODEL: u64 = 1238;
pub const METHOD_NULLFIELDOFFSETISZERO: u64 = 1239;
pub const METHOD_GETMSVTORDISPMODE: u64 = 1240;
pub const METHOD_ISDEPENDENTLAMBDA: u64 = 1241;
pub const METHOD_ISNEVERDEPENDENTLAMBDA: u64 = 1242;
pub const METHOD_GETLAMBDADEPENDENCYKIND: u64 = 1243;
pub const METHOD_GETLAMBDATYPEINFO: u64 = 1244;
pub const METHOD_ISINTERFACELIKE: u64 = 1245;
pub const CLASS_HLSLBUFFERDECL: u64 = 1246;
pub const METHOD_GETSOURCERANGE_14: u64 = 1247;
pub const METHOD_GETLOCSTART: u64 = 1248;
pub const METHOD_GETLBRACELOC: u64 = 1249;
pub const METHOD_GETRBRACELOC_1: u64 = 1250;
pub const METHOD_ISCBUFFER: u64 = 1251;
pub const CLASS_TEMPLATEPARAMOBJECTDECL: u64 = 1252;
pub const METHOD_GETVALUE_2: u64 = 1253;
pub const METHOD_GETCANONICALDECL_18: u64 = 1254;
pub const CLASS_EXPRESSIONTRAITEXPR: u64 = 1255;
pub const METHOD_GETBEGINLOC_72: u64 = 1256;
pub const METHOD_GETENDLOC_71: u64 = 1257;
pub const METHOD_GETTRAIT_1: u64 = 1258;
pub const METHOD_GETQUERIEDEXPRESSION: u64 = 1259;
pub const METHOD_GETVALUE_7: u64 = 1260;
pub const METHOD_CHILDREN_62: u64 = 1261;
pub const CLASS_MSPROPERTYDECL: u64 = 1262;
pub const METHOD_HASGETTER: u64 = 1263;
pub const METHOD_GETGETTERID: u64 = 1264;
pub const METHOD_HASSETTER: u64 = 1265;
pub const METHOD_GETSETTERID: u64 = 1266;
pub const CLASS_USINGSHADOWDECL: u64 = 1267;
pub const METHOD_GETCANONICALDECL_26: u64 = 1268;
pub const METHOD_GETTARGETDECL: u64 = 1269;
pub const METHOD_GETINTRODUCER_1: u64 = 1270;
pub const METHOD_GETNEXTUSINGSHADOWDECL: u64 = 1271;
pub const CLASS_USINGENUMDECL: u64 = 1272;
pub const METHOD_GETUSINGLOC_4: u64 = 1273;
pub const METHOD_GETENUMLOC: u64 = 1274;
pub const METHOD_GETQUALIFIER_10: u64 = 1275;
pub const METHOD_GETQUALIFIERLOC_7: u64 = 1276;
pub const METHOD_GETENUMTYPELOC: u64 = 1277;
pub const METHOD_GETENUMTYPE: u64 = 1278;
pub const METHOD_GETENUMDECL: u64 = 1279;
pub const METHOD_GETSOURCERANGE_34: u64 = 1280;
pub const METHOD_GETCANONICALDECL_24: u64 = 1281;
pub const CLASS_FUNCTIONDECL: u64 = 1282;
pub const METHOD_GETNAMEINFO: u64 = 1283;
pub const METHOD_GETELLIPSISLOC_1: u64 = 1284;
pub const METHOD_GETSOURCERANGE_13: u64 = 1285;
pub const METHOD_HASBODY_1: u64 = 1286;
pub const METHOD_HASTRIVIALBODY: u64 = 1287;
pub const METHOD_ISDEFINED: u64 = 1288;
pub const METHOD_GETDEFINITION_2: u64 = 1289;
pub const METHOD_GETBODY_3: u64 = 1290;
pub const METHOD_ISTHISDECLARATIONADEFINITION_1: u64 = 1291;
pub const METHOD_ISTHISDECLARATIONINSTANTIATEDFROMAFRIENDDEFINITION: u64 = 1292;
pub const METHOD_DOESTHISDECLARATIONHAVEABODY: u64 = 1293;
pub const METHOD_GETDEFAULTEDFUNCTIONINFO: u64 = 1294;
pub const METHOD_ISVARIADIC_2: u64 = 1295;
pub const METHOD_ISVIRTUALASWRITTEN: u64 = 1296;
pub const METHOD_ISPUREVIRTUAL: u64 = 1297;
pub const METHOD_ISLATETEMPLATEPARSED: u64 = 1298;
pub const METHOD_ISTRIVIAL_1: u64 = 1299;
pub const METHOD_ISTRIVIALFORCALL: u64 = 1300;
pub const METHOD_ISDEFAULTED: u64 = 1301;
pub const METHOD_ISEXPLICITLYDEFAULTED: u64 = 1302;
pub const METHOD_GETDEFAULTLOC: u64 = 1303;
pub const METHOD_ISUSERPROVIDED: u64 = 1304;
pub const METHOD_ISINELIGIBLEORNOTSELECTED: u64 = 1305;
pub const METHOD_HASIMPLICITRETURNZERO: u64 = 1306;
pub const METHOD_HASPROTOTYPE: u64 = 1307;
pub const METHOD_HASWRITTENPROTOTYPE: u64 = 1308;
pub const METHOD_HASINHERITEDPROTOTYPE: u64 = 1309;
pub const METHOD_ISCONSTEXPR: u64 = 1310;
pub const METHOD_GETCONSTEXPRKIND: u64 = 1311;
pub const METHOD_ISCONSTEXPRSPECIFIED: u64 = 1312;
pub const METHOD_ISCONSTEVAL: u64 = 1313;
pub const METHOD_BODYCONTAINSIMMEDIATEESCALATINGEXPRESSIONS: u64 = 1314;
pub const METHOD_ISIMMEDIATEESCALATING: u64 = 1315;
pub const METHOD_ISIMMEDIATEFUNCTION: u64 = 1316;
pub const METHOD_INSTANTIATIONISPENDING: u64 = 1317;
pub const METHOD_USESSEHTRY: u64 = 1318;
pub const METHOD_ISDELETED: u64 = 1319;
pub const METHOD_ISDELETEDASWRITTEN: u64 = 1320;
pub const METHOD_ISMAIN: u64 = 1321;
pub const METHOD_ISMSVCRTENTRYPOINT: u64 = 1322;
pub const METHOD_ISRESERVEDGLOBALPLACEMENTOPERATOR: u64 = 1323;
pub const METHOD_ISINLINEBUILTINDECLARATION: u64 = 1324;
pub const METHOD_ISDESTROYINGOPERATORDELETE: u64 = 1325;
pub const METHOD_GETLANGUAGELINKAGE: u64 = 1326;
pub const METHOD_ISEXTERNC: u64 = 1327;
pub const METHOD_ISINEXTERNCCONTEXT: u64 = 1328;
pub const METHOD_ISINEXTERNCXXCONTEXT: u64 = 1329;
pub const METHOD_ISGLOBAL: u64 = 1330;
pub const METHOD_ISNORETURN: u64 = 1331;
pub const METHOD_HASSKIPPEDBODY: u64 = 1332;
pub const METHOD_WILLHAVEBODY: u64 = 1333;
pub const METHOD_ISMULTIVERSION: u64 = 1334;
pub const METHOD_FRIENDCONSTRAINTREFERSTOENCLOSINGTEMPLATE: u64 = 1335;
pub const METHOD_ISMEMBERLIKECONSTRAINEDFRIEND: u64 = 1336;
pub const METHOD_GETMULTIVERSIONKIND: u64 = 1337;
pub const METHOD_ISCPUDISPATCHMULTIVERSION: u64 = 1338;
pub const METHOD_ISCPUSPECIFICMULTIVERSION: u64 = 1339;
pub const METHOD_ISTARGETMULTIVERSION: u64 = 1340;
pub const METHOD_ISTARGETCLONESMULTIVERSION: u64 = 1341;
pub const METHOD_GETCANONICALDECL_11: u64 = 1342;
pub const METHOD_PARAMETERS_2: u64 = 1343;
pub const METHOD_PARAM_EMPTY_1: u64 = 1344;
pub const METHOD_PARAM_BEGIN: u64 = 1345;
pub const METHOD_PARAM_END: u64 = 1346;
pub const METHOD_PARAM_SIZE_1: u64 = 1347;
pub const METHOD_GETNUMPARAMS_3: u64 = 1348;
pub const METHOD_GETMINREQUIREDARGUMENTS: u64 = 1349;
pub const METHOD_GETMINREQUIREDEXPLICITARGUMENTS: u64 = 1350;
pub const METHOD_HASCXXEXPLICITFUNCTIONOBJECTPARAMETER: u64 = 1351;
pub const METHOD_GETNUMNONOBJECTPARAMS: u64 = 1352;
pub const METHOD_HASONEPARAMORDEFAULTARGS: u64 = 1353;
pub const METHOD_GETFUNCTIONTYPELOC: u64 = 1354;
pub const METHOD_GETRETURNTYPE_1: u64 = 1355;
pub const METHOD_GETRETURNTYPESOURCERANGE: u64 = 1356;
pub const METHOD_GETPARAMETERSSOURCERANGE: u64 = 1357;
pub const METHOD_GETDECLAREDRETURNTYPE: u64 = 1358;
pub const METHOD_GETEXCEPTIONSPECTYPE_1: u64 = 1359;
pub const METHOD_GETEXCEPTIONSPECSOURCERANGE: u64 = 1360;
pub const METHOD_GETCALLRESULTTYPE: u64 = 1361;
pub const METHOD_GETSTORAGECLASS: u64 = 1362;
pub const METHOD_ISINLINESPECIFIED: u64 = 1363;
pub const METHOD_USESFPINTRIN: u64 = 1364;
pub const METHOD_ISINLINED: u64 = 1365;
pub const METHOD_ISINLINEDEFINITIONEXTERNALLYVISIBLE: u64 = 1366;
pub const METHOD_ISMSEXTERNINLINE: u64 = 1367;
pub const METHOD_DOESDECLARATIONFORCEEXTERNALLYVISIBLEDEFINITION: u64 = 1368;
pub const METHOD_ISSTATIC_1: u64 = 1369;
pub const METHOD_ISOVERLOADEDOPERATOR: u64 = 1370;
pub const METHOD_GETOVERLOADEDOPERATOR: u64 = 1371;
pub const METHOD_GETLITERALIDENTIFIER: u64 = 1372;
pub const METHOD_GETINSTANTIATEDFROMMEMBERFUNCTION: u64 = 1373;
pub const METHOD_GETTEMPLATEDKIND: u64 = 1374;
pub const METHOD_GETMEMBERSPECIALIZATIONINFO_2: u64 = 1375;
pub const METHOD_GETINSTANTIATEDFROMDECL: u64 = 1376;
pub const METHOD_GETDESCRIBEDFUNCTIONTEMPLATE: u64 = 1377;
pub const METHOD_ISFUNCTIONTEMPLATESPECIALIZATION: u64 = 1378;
pub const METHOD_GETTEMPLATESPECIALIZATIONINFO: u64 = 1379;
pub const METHOD_ISIMPLICITLYINSTANTIABLE: u64 = 1380;
pub const METHOD_ISTEMPLATEINSTANTIATION: u64 = 1381;
pub const METHOD_GETPRIMARYTEMPLATE: u64 = 1382;
pub const METHOD_GETTEMPLATESPECIALIZATIONARGS: u64 = 1383;
pub const METHOD_GETTEMPLATESPECIALIZATIONARGSASWRITTEN: u64 = 1384;
pub const METHOD_GETDEPENDENTSPECIALIZATIONINFO: u64 = 1385;
pub const METHOD_GETTEMPLATESPECIALIZATIONKIND_2: u64 = 1386;
pub const METHOD_GETTEMPLATESPECIALIZATIONKINDFORINSTANTIATION: u64 = 1387;
pub const METHOD_GETPOINTOFINSTANTIATION_1: u64 = 1388;
pub const METHOD_ISOUTOFLINE_1: u64 = 1389;
pub const METHOD_GETMEMORYFUNCTIONKIND: u64 = 1390;
pub const METHOD_GETODRHASH_1: u64 = 1391;
pub const CLASS_CLASSTEMPLATEDECL: u64 = 1392;
pub const METHOD_GETTEMPLATEDDECL: u64 = 1393;
pub const METHOD_ISTHISDECLARATIONADEFINITION: u64 = 1394;
pub const METHOD_GETCANONICALDECL_5: u64 = 1395;
pub const METHOD_GETPREVIOUSDECL_1: u64 = 1396;
pub const METHOD_GETMOSTRECENTDECL_2: u64 = 1397;
pub const METHOD_GETINSTANTIATEDFROMMEMBERTEMPLATE: u64 = 1398;
pub const METHOD_SPECIALIZATIONS: u64 = 1399;
pub const METHOD_SPEC_BEGIN: u64 = 1400;
pub const METHOD_SPEC_END: u64 = 1401;
pub const CLASS_TEMPLATETEMPLATEPARMDECL: u64 = 1402;
pub const METHOD_ISPARAMETERPACK_3: u64 = 1403;
pub const METHOD_ISPACKEXPANSION_1: u64 = 1404;
pub const METHOD_ISEXPANDEDPARAMETERPACK_1: u64 = 1405;
pub const METHOD_GETNUMEXPANSIONTEMPLATEPARAMETERS: u64 = 1406;
pub const METHOD_GETDEFAULTARGSTORAGE_1: u64 = 1407;
pub const METHOD_HASDEFAULTARGUMENT_1: u64 = 1408;
pub const METHOD_GETDEFAULTARGUMENT_1: u64 = 1409;
pub const METHOD_GETDEFAULTARGUMENTLOC_1: u64 = 1410;
pub const METHOD_DEFAULTARGUMENTWASINHERITED_1: u64 = 1411;
pub const METHOD_GETSOURCERANGE_25: u64 = 1412;
pub const CLASS_CXXFORRANGESTMT: u64 = 1413;
pub const METHOD_GETINIT_2: u64 = 1414;
pub const METHOD_GETLOOPVARIABLE: u64 = 1415;
pub const METHOD_GETRANGEINIT: u64 = 1416;
pub const METHOD_GETRANGESTMT: u64 = 1417;
pub const METHOD_GETBEGINSTMT: u64 = 1418;
pub const METHOD_GETENDSTMT: u64 = 1419;
pub const METHOD_GETCOND_2: u64 = 1420;
pub const METHOD_GETINC: u64 = 1421;
pub const METHOD_GETLOOPVARSTMT: u64 = 1422;
pub const METHOD_GETBODY_5: u64 = 1423;
pub const METHOD_GETFORLOC: u64 = 1424;
pub const METHOD_GETCOAWAITLOC: u64 = 1425;
pub const METHOD_GETCOLONLOC_2: u64 = 1426;
pub const METHOD_GETRPARENLOC_6: u64 = 1427;
pub const METHOD_GETBEGINLOC_28: u64 = 1428;
pub const METHOD_GETENDLOC_27: u64 = 1429;
pub const METHOD_CHILDREN_22: u64 = 1430;
pub const CLASS_CONCEPTDECL: u64 = 1431;
pub const METHOD_GETCONSTRAINTEXPR: u64 = 1432;
pub const METHOD_GETSOURCERANGE_4: u64 = 1433;
pub const METHOD_ISTYPECONCEPT: u64 = 1434;
pub const METHOD_GETCANONICALDECL_6: u64 = 1435;
pub const CLASS_OBJCCONTAINERDECL: u64 = 1436;
pub const CLASS_UNRESOLVEDUSINGIFEXISTSDECL: u64 = 1437;
pub const CLASS_OMPALLOCATEDECL: u64 = 1438;
pub const CLASS_OBJCINTERFACEDECL: u64 = 1439;
pub const CLASS_BREAKSTMT: u64 = 1440;
pub const METHOD_GETBREAKLOC: u64 = 1441;
pub const METHOD_GETBEGINLOC_16: u64 = 1442;
pub const METHOD_GETENDLOC_15: u64 = 1443;
pub const METHOD_CHILDREN_12: u64 = 1444;
pub const CLASS_OMPORDEREDDIRECTIVE: u64 = 1445;
pub const CLASS_CXXDELETEEXPR: u64 = 1446;
pub const METHOD_ISGLOBALDELETE: u64 = 1447;
pub const METHOD_ISARRAYFORM: u64 = 1448;
pub const METHOD_ISARRAYFORMASWRITTEN: u64 = 1449;
pub const METHOD_DOESUSUALARRAYDELETEWANTSIZE: u64 = 1450;
pub const METHOD_GETOPERATORDELETE_1: u64 = 1451;
pub const METHOD_GETARGUMENT: u64 = 1452;
pub const METHOD_GETDESTROYEDTYPE: u64 = 1453;
pub const METHOD_GETBEGINLOC_25: u64 = 1454;
pub const METHOD_GETENDLOC_24: u64 = 1455;
pub const METHOD_CHILDREN_19: u64 = 1456;
pub const CLASS_STATICASSERTDECL: u64 = 1457;
pub const METHOD_GETASSERTEXPR: u64 = 1458;
pub const METHOD_GETMESSAGE: u64 = 1459;
pub const METHOD_ISFAILED: u64 = 1460;
pub const METHOD_GETRPARENLOC_1: u64 = 1461;
pub const METHOD_GETSOURCERANGE_22: u64 = 1462;
pub const CLASS_TOPLEVELSTMTDECL: u64 = 1463;
pub const METHOD_GETSOURCERANGE_27: u64 = 1464;
pub const METHOD_GETSTMT_1: u64 = 1465;
pub const METHOD_ISSEMIMISSING: u64 = 1466;
pub const CLASS_CXXMETHODDECL: u64 = 1467;
pub const METHOD_ISSTATIC: u64 = 1468;
pub const METHOD_ISINSTANCE: u64 = 1469;
pub const METHOD_ISEXPLICITOBJECTMEMBERFUNCTION: u64 = 1470;
pub const METHOD_ISIMPLICITOBJECTMEMBERFUNCTION: u64 = 1471;
pub const METHOD_ISCONST_1: u64 = 1472;
pub const METHOD_ISVOLATILE_1: u64 = 1473;
pub const METHOD_ISVIRTUAL: u64 = 1474;
pub const METHOD_ISCOPYASSIGNMENTOPERATOR: u64 = 1475;
pub const METHOD_ISMOVEASSIGNMENTOPERATOR: u64 = 1476;
pub const METHOD_GETCANONICALDECL_3: u64 = 1477;
pub const METHOD_GETMOSTRECENTDECL: u64 = 1478;
pub const METHOD_BEGIN_OVERRIDDEN_METHODS: u64 = 1479;
pub const METHOD_END_OVERRIDDEN_METHODS: u64 = 1480;
pub const METHOD_SIZE_OVERRIDDEN_METHODS: u64 = 1481;
pub const METHOD_OVERRIDDEN_METHODS: u64 = 1482;
pub const METHOD_GETPARENT: u64 = 1483;
pub const METHOD_GETTHISTYPE: u64 = 1484;
pub const METHOD_GETFUNCTIONOBJECTPARAMETERREFERENCETYPE: u64 = 1485;
pub const METHOD_GETFUNCTIONOBJECTPARAMETERTYPE: u64 = 1486;
pub const METHOD_GETNUMEXPLICITPARAMS: u64 = 1487;
pub const METHOD_GETMETHODQUALIFIERS: u64 = 1488;
pub const METHOD_GETREFQUALIFIER_1: u64 = 1489;
pub const METHOD_HASINLINEBODY: u64 = 1490;
pub const METHOD_ISLAMBDASTATICINVOKER: u64 = 1491;
pub const CLASS_VALUEDECL: u64 = 1492;
pub const METHOD_GETTYPE: u64 = 1493;
pub const METHOD_ISWEAK: u64 = 1494;
pub const METHOD_ISINITCAPTURE: u64 = 1495;
pub const METHOD_GETPOTENTIALLYDECOMPOSEDVARDECL: u64 = 1496;
pub const CLASS_OMPTARGETPARALLELGENERICLOOPDIRECTIVE: u64 = 1497;
pub const CLASS_OMPTHREADPRIVATEDECL: u64 = 1498;
pub const CLASS_BINDINGDECL: u64 = 1499;
pub const METHOD_GETBINDING: u64 = 1500;
pub const METHOD_GETDECOMPOSEDDECL: u64 = 1501;
pub const METHOD_GETHOLDINGVAR: u64 = 1502;
pub const CLASS_USINGDIRECTIVEDECL: u64 = 1503;
pub const METHOD_GETQUALIFIERLOC_6: u64 = 1504;
pub const METHOD_GETQUALIFIER_9: u64 = 1505;
pub const METHOD_GETNOMINATEDNAMESPACEASWRITTEN: u64 = 1506;
pub const METHOD_GETNOMINATEDNAMESPACE: u64 = 1507;
pub const METHOD_GETCOMMONANCESTOR: u64 = 1508;
pub const METHOD_GETUSINGLOC_3: u64 = 1509;
pub const METHOD_GETNAMESPACEKEYLOCATION: u64 = 1510;
pub const METHOD_GETIDENTLOCATION: u64 = 1511;
pub const METHOD_GETSOURCERANGE_33: u64 = 1512;
pub const CLASS_OBJCCATEGORYDECL: u64 = 1513;
pub const CLASS_IMPLICITCONCEPTSPECIALIZATIONDECL: u64 = 1514;
pub const METHOD_GETTEMPLATEARGUMENTS: u64 = 1515;
pub const CLASS_VARTEMPLATESPECIALIZATIONDECL: u64 = 1516;
pub const METHOD_GETSPECIALIZEDTEMPLATE_1: u64 = 1517;
pub const METHOD_GETTEMPLATEARGS_1: u64 = 1518;
pub const METHOD_GETTEMPLATEARGSINFO: u64 = 1519;
pub const METHOD_GETSPECIALIZATIONKIND_1: u64 = 1520;
pub const METHOD_ISEXPLICITSPECIALIZATION_1: u64 = 1521;
pub const METHOD_ISCLASSSCOPEEXPLICITSPECIALIZATION_1: u64 = 1522;
pub const METHOD_ISEXPLICITINSTANTIATIONORSPECIALIZATION_1: u64 = 1523;
pub const METHOD_GETPOINTOFINSTANTIATION_3: u64 = 1524;
pub const METHOD_GETINSTANTIATEDFROM_1: u64 = 1525;
pub const METHOD_GETSPECIALIZEDTEMPLATEORPARTIAL_1: u64 = 1526;
pub const METHOD_GETTEMPLATEINSTANTIATIONARGS_1: u64 = 1527;
pub const METHOD_GETTYPEASWRITTEN_1: u64 = 1528;
pub const METHOD_GETEXTERNLOC_2: u64 = 1529;
pub const METHOD_GETTEMPLATEKEYWORDLOC_1: u64 = 1530;
pub const METHOD_GETSOURCERANGE_38: u64 = 1531;
pub const CLASS_OMPERRORDIRECTIVE: u64 = 1532;
pub const CLASS_BASEUSINGDECL: u64 = 1533;
pub const METHOD_SHADOWS: u64 = 1534;
pub const METHOD_SHADOW_BEGIN: u64 = 1535;
pub const METHOD_SHADOW_END: u64 = 1536;
pub const METHOD_SHADOW_SIZE: u64 = 1537;
pub const CLASS_CXXCONSTRUCTORDECL: u64 = 1538;
pub const METHOD_GETEXPLICITSPECIFIER: u64 = 1539;
pub const METHOD_ISEXPLICIT: u64 = 1540;
pub const METHOD_INITS: u64 = 1541;
pub const METHOD_INIT_BEGIN: u64 = 1542;
pub const METHOD_INIT_END: u64 = 1543;
pub const METHOD_INIT_RBEGIN: u64 = 1544;
pub const METHOD_INIT_REND: u64 = 1545;
pub const METHOD_GETNUMCTORINITIALIZERS: u64 = 1546;
pub const METHOD_ISDELEGATINGCONSTRUCTOR: u64 = 1547;
pub const METHOD_GETTARGETCONSTRUCTOR: u64 = 1548;
pub const METHOD_ISDEFAULTCONSTRUCTOR: u64 = 1549;
pub const METHOD_ISCOPYCONSTRUCTOR: u64 = 1550;
pub const METHOD_ISMOVECONSTRUCTOR: u64 = 1551;
pub const METHOD_ISCOPYORMOVECONSTRUCTOR: u64 = 1552;
pub const METHOD_ISSPECIALIZATIONCOPYINGOBJECT: u64 = 1553;
pub const METHOD_ISINHERITINGCONSTRUCTOR: u64 = 1554;
pub const METHOD_GETINHERITEDCONSTRUCTOR: u64 = 1555;
pub const METHOD_GETCANONICALDECL: u64 = 1556;
pub const CLASS_TEMPLATETYPEPARMDECL: u64 = 1557;
pub const METHOD_WASDECLAREDWITHTYPENAME: u64 = 1558;
pub const METHOD_GETDEFAULTARGSTORAGE_2: u64 = 1559;
pub const METHOD_HASDEFAULTARGUMENT_2: u64 = 1560;
pub const METHOD_GETDEFAULTARGUMENT_2: u64 = 1561;
pub const METHOD_GETDEFAULTARGUMENTINFO: u64 = 1562;
pub const METHOD_GETDEFAULTARGUMENTLOC_2: u64 = 1563;
pub const METHOD_DEFAULTARGUMENTWASINHERITED_2: u64 = 1564;
pub const METHOD_GETDEPTH_1: u64 = 1565;
pub const METHOD_GETINDEX_3: u64 = 1566;
pub const METHOD_ISPARAMETERPACK_4: u64 = 1567;
pub const METHOD_ISPACKEXPANSION_2: u64 = 1568;
pub const METHOD_ISEXPANDEDPARAMETERPACK_2: u64 = 1569;
pub const METHOD_GETNUMEXPANSIONPARAMETERS: u64 = 1570;
pub const METHOD_GETTYPECONSTRAINT: u64 = 1571;
pub const METHOD_HASTYPECONSTRAINT: u64 = 1572;
pub const METHOD_GETSOURCERANGE_26: u64 = 1573;
pub const CLASS_STMT: u64 = 1574;
pub const METHOD_GETSTMTCLASS: u64 = 1575;
pub const METHOD_GETSTMTCLASSNAME: u64 = 1576;
pub const METHOD_GETSOURCERANGE_49: u64 = 1577;
pub const METHOD_GETBEGINLOC_133: u64 = 1578;
pub const METHOD_GETENDLOC_132: u64 = 1579;
pub const METHOD_STRIPLABELLIKESTATEMENTS: u64 = 1580;
pub const METHOD_CHILDREN_105: u64 = 1581;
pub const METHOD_CHILD_BEGIN: u64 = 1582;
pub const METHOD_CHILD_END: u64 = 1583;
pub const CLASS_FUNCTIONTEMPLATEDECL: u64 = 1584;
pub const METHOD_GETTEMPLATEDDECL_1: u64 = 1585;
pub const METHOD_ISTHISDECLARATIONADEFINITION_2: u64 = 1586;
pub const METHOD_GETCANONICALDECL_12: u64 = 1587;
pub const METHOD_GETPREVIOUSDECL_4: u64 = 1588;
pub const METHOD_GETMOSTRECENTDECL_5: u64 = 1589;
pub const METHOD_GETINSTANTIATEDFROMMEMBERTEMPLATE_2: u64 = 1590;
pub const METHOD_SPECIALIZATIONS_1: u64 = 1591;
pub const METHOD_SPEC_BEGIN_1: u64 = 1592;
pub const METHOD_SPEC_END_1: u64 = 1593;
pub const METHOD_ISABBREVIATED: u64 = 1594;
pub const CLASS_ACCESSSPECDECL: u64 = 1595;
pub const METHOD_GETACCESSSPECIFIERLOC: u64 = 1596;
pub const METHOD_GETCOLONLOC: u64 = 1597;
pub const METHOD_GETSOURCERANGE: u64 = 1598;
pub const CLASS_LIFETIMEEXTENDEDTEMPORARYDECL: u64 = 1599;
pub const METHOD_GETEXTENDINGDECL: u64 = 1600;
pub const METHOD_GETSTORAGEDURATION: u64 = 1601;
pub const METHOD_GETTEMPORARYEXPR: u64 = 1602;
pub const METHOD_GETMANGLINGNUMBER: u64 = 1603;
pub const METHOD_GETVALUE: u64 = 1604;
pub const METHOD_CHILDRENEXPR: u64 = 1605;
pub const CLASS_TEMPLATEDECL: u64 = 1606;
pub const METHOD_GETTEMPLATEPARAMETERS_1: u64 = 1607;
pub const METHOD_HASASSOCIATEDCONSTRAINTS_1: u64 = 1608;
pub const METHOD_GETTEMPLATEDDECL_2: u64 = 1609;
pub const METHOD_ISTYPEALIAS_1: u64 = 1610;
pub const METHOD_GETSOURCERANGE_24: u64 = 1611;
pub const CLASS_CAPTUREDDECL: u64 = 1612;
pub const METHOD_GETBODY_1: u64 = 1613;
pub const METHOD_ISNOTHROW: u64 = 1614;
pub const METHOD_GETNUMPARAMS_2: u64 = 1615;
pub const METHOD_PARAMETERS_1: u64 = 1616;
pub const METHOD_GETCONTEXTPARAM: u64 = 1617;
pub const METHOD_GETCONTEXTPARAMPOSITION: u64 = 1618;
pub const METHOD_PARAM_BEGIN_1: u64 = 1619;
pub const METHOD_PARAM_END_1: u64 = 1620;
pub const CLASS_TYPEDEFDECL: u64 = 1621;
pub const METHOD_GETSOURCERANGE_30: u64 = 1622;
pub const CLASS_OMPREQUIRESDECL: u64 = 1623;
pub const CLASS_OBJCIMPLEMENTATIONDECL: u64 = 1624;
pub const CLASS_NAMESPACEALIASDECL: u64 = 1625;
pub const METHOD_GETCANONICALDECL_14: u64 = 1626;
pub const METHOD_GETQUALIFIERLOC_1: u64 = 1627;
pub const METHOD_GETQUALIFIER_4: u64 = 1628;
pub const METHOD_GETNAMESPACE: u64 = 1629;
pub const METHOD_GETALIASLOC: u64 = 1630;
pub const METHOD_GETNAMESPACELOC: u64 = 1631;
pub const METHOD_GETTARGETNAMELOC: u64 = 1632;
pub const METHOD_GETALIASEDNAMESPACE: u64 = 1633;
pub const METHOD_GETSOURCERANGE_18: u64 = 1634;
pub const CLASS_LINKAGESPECDECL: u64 = 1635;
pub const METHOD_GETLANGUAGE: u64 = 1636;
pub const METHOD_HASBRACES_1: u64 = 1637;
pub const METHOD_GETEXTERNLOC_1: u64 = 1638;
pub const METHOD_GETRBRACELOC_2: u64 = 1639;
pub const METHOD_GETENDLOC_2: u64 = 1640;
pub const METHOD_GETSOURCERANGE_17: u64 = 1641;
pub const CLASS_EMPTYDECL: u64 = 1642;
pub const CLASS_IMPORTDECL: u64 = 1643;
pub const METHOD_GETIMPORTEDMODULE: u64 = 1644;
pub const METHOD_GETIDENTIFIERLOCS: u64 = 1645;
pub const METHOD_GETSOURCERANGE_15: u64 = 1646;
pub const CLASS_TRANSLATIONUNITDECL: u64 = 1647;
pub const METHOD_GETASTCONTEXT: u64 = 1648;
pub const METHOD_GETANONYMOUSNAMESPACE_1: u64 = 1649;
pub const CLASS_EXPORTDECL: u64 = 1650;
pub const METHOD_GETEXPORTLOC: u64 = 1651;
pub const METHOD_GETRBRACELOC: u64 = 1652;
pub const METHOD_HASBRACES: u64 = 1653;
pub const METHOD_GETENDLOC_1: u64 = 1654;
pub const METHOD_GETSOURCERANGE_9: u64 = 1655;
pub const CLASS_OBJCIMPLDECL: u64 = 1656;
pub const CLASS_VARTEMPLATEDECL: u64 = 1657;
pub const METHOD_GETTEMPLATEDDECL_4: u64 = 1658;
pub const METHOD_ISTHISDECLARATIONADEFINITION_5: u64 = 1659;
pub const METHOD_GETCANONICALDECL_28: u64 = 1660;
pub const METHOD_GETPREVIOUSDECL_7: u64 = 1661;
pub const METHOD_GETMOSTRECENTDECL_8: u64 = 1662;
pub const METHOD_GETINSTANTIATEDFROMMEMBERTEMPLATE_5: u64 = 1663;
pub const METHOD_SPECIALIZATIONS_2: u64 = 1664;
pub const METHOD_SPEC_BEGIN_2: u64 = 1665;
pub const METHOD_SPEC_END_2: u64 = 1666;
pub const CLASS_FILESCOPEASMDECL: u64 = 1667;
pub const METHOD_GETASMLOC: u64 = 1668;
pub const METHOD_GETRPARENLOC: u64 = 1669;
pub const METHOD_GETSOURCERANGE_11: u64 = 1670;
pub const METHOD_GETASMSTRING: u64 = 1671;
pub const CLASS_FIELDDECL: u64 = 1672;
pub const METHOD_GETFIELDINDEX: u64 = 1673;
pub const METHOD_ISMUTABLE: u64 = 1674;
pub const METHOD_ISBITFIELD: u64 = 1675;
pub const METHOD_ISUNNAMEDBITFIELD: u64 = 1676;
pub const METHOD_ISANONYMOUSSTRUCTORUNION: u64 = 1677;
pub const METHOD_GETBITWIDTH: u64 = 1678;
pub const METHOD_ISPOTENTIALLYOVERLAPPING: u64 = 1679;
pub const METHOD_GETINCLASSINITSTYLE: u64 = 1680;
pub const METHOD_HASINCLASSINITIALIZER_1: u64 = 1681;
pub const METHOD_HASNONNULLINCLASSINITIALIZER: u64 = 1682;
pub const METHOD_GETINCLASSINITIALIZER: u64 = 1683;
pub const METHOD_HASCAPTUREDVLATYPE: u64 = 1684;
pub const METHOD_GETCAPTUREDVLATYPE: u64 = 1685;
pub const METHOD_GETPARENT_2: u64 = 1686;
pub const METHOD_GETSOURCERANGE_10: u64 = 1687;
pub const METHOD_GETCANONICALDECL_10: u64 = 1688;
pub const CLASS_TYPEALIASTEMPLATEDECL: u64 = 1689;
pub const METHOD_GETTEMPLATEDDECL_3: u64 = 1690;
pub const METHOD_GETCANONICALDECL_19: u64 = 1691;
pub const METHOD_GETPREVIOUSDECL_6: u64 = 1692;
pub const METHOD_GETINSTANTIATEDFROMMEMBERTEMPLATE_4: u64 = 1693;
pub const CLASS_OBJCCATEGORYIMPLDECL: u64 = 1694;
pub const CLASS_OMPDECLAREREDUCTIONDECL: u64 = 1695;
pub const CLASS_UNRESOLVEDUSINGVALUEDECL: u64 = 1696;
pub const METHOD_GETUSINGLOC_1: u64 = 1697;
pub const METHOD_ISACCESSDECLARATION: u64 = 1698;
pub const METHOD_GETQUALIFIERLOC_4: u64 = 1699;
pub const METHOD_GETQUALIFIER_7: u64 = 1700;
pub const METHOD_GETNAMEINFO_2: u64 = 1701;
pub const METHOD_ISPACKEXPANSION_4: u64 = 1702;
pub const METHOD_GETELLIPSISLOC_3: u64 = 1703;
pub const METHOD_GETSOURCERANGE_31: u64 = 1704;
pub const METHOD_GETCANONICALDECL_22: u64 = 1705;
pub const CLASS_ENUMCONSTANTDECL: u64 = 1706;
pub const METHOD_GETINITEXPR: u64 = 1707;
pub const METHOD_GETINITVAL: u64 = 1708;
pub const METHOD_GETSOURCERANGE_7: u64 = 1709;
pub const METHOD_GETCANONICALDECL_8: u64 = 1710;
pub const CLASS_NONTYPETEMPLATEPARMDECL: u64 = 1711;
pub const METHOD_GETSOURCERANGE_20: u64 = 1712;
pub const METHOD_GETDEFAULTARGSTORAGE: u64 = 1713;
pub const METHOD_HASDEFAULTARGUMENT: u64 = 1714;
pub const METHOD_GETDEFAULTARGUMENT: u64 = 1715;
pub const METHOD_GETDEFAULTARGUMENTLOC: u64 = 1716;
pub const METHOD_DEFAULTARGUMENTWASINHERITED: u64 = 1717;
pub const METHOD_ISPARAMETERPACK_2: u64 = 1718;
pub const METHOD_ISPACKEXPANSION: u64 = 1719;
pub const METHOD_ISEXPANDEDPARAMETERPACK: u64 = 1720;
pub const METHOD_GETNUMEXPANSIONTYPES: u64 = 1721;
pub const METHOD_GETPLACEHOLDERTYPECONSTRAINT: u64 = 1722;
pub const METHOD_HASPLACEHOLDERTYPECONSTRAINT: u64 = 1723;
pub const CLASS_OBJCATDEFSFIELDDECL: u64 = 1724;
pub const CLASS_CXXCONVERSIONDECL: u64 = 1725;
pub const METHOD_GETEXPLICITSPECIFIER_1: u64 = 1726;
pub const METHOD_ISEXPLICIT_1: u64 = 1727;
pub const METHOD_GETCONVERSIONTYPE: u64 = 1728;
pub const METHOD_ISLAMBDATOBLOCKPOINTERCONVERSION: u64 = 1729;
pub const METHOD_GETCANONICALDECL_1: u64 = 1730;
pub const CLASS_SEHFINALLYSTMT: u64 = 1731;
pub const METHOD_GETBEGINLOC_111: u64 = 1732;
pub const METHOD_GETFINALLYLOC: u64 = 1733;
pub const METHOD_GETENDLOC_110: u64 = 1734;
pub const METHOD_GETBLOCK_1: u64 = 1735;
pub const METHOD_CHILDREN_98: u64 = 1736;
pub const CLASS_CXXDESTRUCTORDECL: u64 = 1737;
pub const METHOD_GETOPERATORDELETE: u64 = 1738;
pub const METHOD_GETOPERATORDELETETHISARG: u64 = 1739;
pub const METHOD_GETCANONICALDECL_2: u64 = 1740;
pub const CLASS_VARTEMPLATEPARTIALSPECIALIZATIONDECL: u64 = 1741;
pub const METHOD_GETTEMPLATEPARAMETERS_2: u64 = 1742;
pub const METHOD_GETTEMPLATEARGSASWRITTEN_1: u64 = 1743;
pub const METHOD_HASASSOCIATEDCONSTRAINTS_2: u64 = 1744;
pub const METHOD_GETINSTANTIATEDFROMMEMBER_1: u64 = 1745;
pub const METHOD_GETSOURCERANGE_37: u64 = 1746;
pub const CLASS_TYPEVISIBILITYATTR: u64 = 1747;
pub const CLASS_REINITIALIZESATTR: u64 = 1748;
pub const CLASS_UNINITIALIZEDATTR: u64 = 1749;
pub const CLASS_VECTORCALLATTR: u64 = 1750;
pub const CLASS_CXXNULLPTRLITERALEXPR: u64 = 1751;
pub const METHOD_GETBEGINLOC_34: u64 = 1752;
pub const METHOD_GETENDLOC_33: u64 = 1753;
pub const METHOD_GETLOCATION_4: u64 = 1754;
pub const METHOD_CHILDREN_26: u64 = 1755;
pub const CLASS_COMPOUNDSTMT: u64 = 1756;
pub const METHOD_BODY_EMPTY: u64 = 1757;
pub const METHOD_SIZE: u64 = 1758;
pub const METHOD_HASSTOREDFPFEATURES_3: u64 = 1759;
pub const METHOD_GETSTOREDFPFEATURES_3: u64 = 1760;
pub const METHOD_BODY: u64 = 1761;
pub const METHOD_BODY_BEGIN: u64 = 1762;
pub const METHOD_BODY_END: u64 = 1763;
pub const METHOD_BODY_FRONT: u64 = 1764;
pub const METHOD_BODY_BACK: u64 = 1765;
pub const METHOD_BODY_RBEGIN: u64 = 1766;
pub const METHOD_BODY_REND: u64 = 1767;
pub const METHOD_GETSTMTEXPRRESULT: u64 = 1768;
pub const METHOD_GETBEGINLOC_54: u64 = 1769;
pub const METHOD_GETENDLOC_53: u64 = 1770;
pub const METHOD_GETLBRACLOC: u64 = 1771;
pub const METHOD_GETRBRACLOC: u64 = 1772;
pub const METHOD_CHILDREN_44: u64 = 1773;
pub const CLASS_WORKGROUPSIZEHINTATTR: u64 = 1774;
pub const CLASS_LOADERUNINITIALIZEDATTR: u64 = 1775;
pub const CLASS_PREFERREDNAMEATTR: u64 = 1776;
pub const CLASS_OSCONSUMESTHISATTR: u64 = 1777;
pub const CLASS_ALIGNNATURALATTR: u64 = 1778;
pub const CLASS_UNAVAILABLEATTR: u64 = 1779;
pub const CLASS_BUILTINALIASATTR: u64 = 1780;
pub const CLASS_CONSUMABLEATTR: u64 = 1781;
pub const CLASS_ABITAGATTR: u64 = 1782;
pub const CLASS_PASCALATTR: u64 = 1783;
pub const CLASS_OBJCATTRYSTMT: u64 = 1784;
pub const CLASS_OMPTARGETTEAMSDISTRIBUTEPARALLELFORSIMDDIRECTIVE: u64 = 1785;
pub const CLASS_ALIGNVALUEATTR: u64 = 1786;
pub const CLASS_TYPEDECL: u64 = 1787;
pub const METHOD_GETTYPEFORDECL: u64 = 1788;
pub const METHOD_GETBEGINLOC_3: u64 = 1789;
pub const METHOD_GETSOURCERANGE_29: u64 = 1790;
pub const CLASS_ENFORCETCBATTR: u64 = 1791;
pub const CLASS_SWIFTVERSIONEDREMOVALATTR: u64 = 1792;
pub const CLASS_MATRIXTYPE: u64 = 1793;
pub const METHOD_GETELEMENTTYPE_4: u64 = 1794;
pub const METHOD_ISSUGARED_27: u64 = 1795;
pub const METHOD_DESUGAR_27: u64 = 1796;
pub const CLASS_NAKEDATTR: u64 = 1797;
pub const CLASS_UNSAFEBUFFERUSAGEATTR: u64 = 1798;
pub const CLASS_SWIFTVERSIONEDADDITIONATTR: u64 = 1799;
pub const CLASS_EXTVECTORTYPELOC: u64 = 1800;
pub const CLASS_OBJCNONLAZYCLASSATTR: u64 = 1801;
pub const CLASS_OMPPARALLELGENERICLOOPDIRECTIVE: u64 = 1802;
pub const CLASS_EMPTYBASESATTR: u64 = 1803;
pub const CLASS_WEAKREFATTR: u64 = 1804;
pub const CLASS_SUPPRESSATTR: u64 = 1805;
pub const CLASS_DECAYEDTYPELOC: u64 = 1806;
pub const CLASS_C11NORETURNATTR: u64 = 1807;
pub const CLASS_CALLEDONCEATTR: u64 = 1808;
pub const CLASS_SUBSTTEMPLATETYPEPARMPACKTYPELOC: u64 = 1809;
pub const CLASS_INDIRECTFIELDDECL: u64 = 1810;
pub const METHOD_CHAIN: u64 = 1811;
pub const METHOD_CHAIN_BEGIN: u64 = 1812;
pub const METHOD_CHAIN_END: u64 = 1813;
pub const METHOD_GETCHAININGSIZE: u64 = 1814;
pub const METHOD_GETANONFIELD: u64 = 1815;
pub const METHOD_GETVARDECL: u64 = 1816;
pub const METHOD_GETCANONICALDECL_13: u64 = 1817;
pub const CLASS_ASSERTCAPABILITYATTR: u64 = 1818;
pub const CLASS_WEBASSEMBLYIMPORTMODULEATTR: u64 = 1819;
pub const CLASS_OMPTASKYIELDDIRECTIVE: u64 = 1820;
pub const CLASS_SWIFTINDIRECTRESULTATTR: u64 = 1821;
pub const CLASS_MACROQUALIFIEDTYPE: u64 = 1822;
pub const METHOD_GETMACROIDENTIFIER: u64 = 1823;
pub const METHOD_GETUNDERLYINGTYPE_1: u64 = 1824;
pub const METHOD_GETMODIFIEDTYPE_1: u64 = 1825;
pub const METHOD_ISSUGARED_26: u64 = 1826;
pub const METHOD_DESUGAR_26: u64 = 1827;
pub const CLASS_OBJCDESIGNATEDINITIALIZERATTR: u64 = 1828;
pub const CLASS_STRICTGUARDSTACKCHECKATTR: u64 = 1829;
pub const CLASS_IMAGINARYLITERAL: u64 = 1830;
pub const METHOD_GETSUBEXPR_6: u64 = 1831;
pub const METHOD_GETBEGINLOC_83: u64 = 1832;
pub const METHOD_GETENDLOC_82: u64 = 1833;
pub const METHOD_CHILDREN_72: u64 = 1834;
pub const CLASS_SCOPEDLOCKABLEATTR: u64 = 1835;
pub const CLASS_RECORDDECL: u64 = 1836;
pub const METHOD_GETPREVIOUSDECL_5: u64 = 1837;
pub const METHOD_GETMOSTRECENTDECL_7: u64 = 1838;
pub const METHOD_HASFLEXIBLEARRAYMEMBER: u64 = 1839;
pub const METHOD_ISANONYMOUSSTRUCTORUNION_1: u64 = 1840;
pub const METHOD_HASOBJECTMEMBER: u64 = 1841;
pub const METHOD_HASVOLATILEMEMBER: u64 = 1842;
pub const METHOD_HASLOADEDFIELDSFROMEXTERNALSTORAGE: u64 = 1843;
pub const METHOD_ISNONTRIVIALTOPRIMITIVEDEFAULTINITIALIZE: u64 = 1844;
pub const METHOD_ISNONTRIVIALTOPRIMITIVECOPY: u64 = 1845;
pub const METHOD_ISNONTRIVIALTOPRIMITIVEDESTROY: u64 = 1846;
pub const METHOD_HASNONTRIVIALTOPRIMITIVEDEFAULTINITIALIZECUNION: u64 = 1847;
pub const METHOD_HASNONTRIVIALTOPRIMITIVEDESTRUCTCUNION: u64 = 1848;
pub const METHOD_HASNONTRIVIALTOPRIMITIVECOPYCUNION: u64 = 1849;
pub const METHOD_CANPASSINREGISTERS: u64 = 1850;
pub const METHOD_GETARGPASSINGRESTRICTIONS: u64 = 1851;
pub const METHOD_ISPARAMDESTROYEDINCALLEE: u64 = 1852;
pub const METHOD_ISRANDOMIZED: u64 = 1853;
pub const METHOD_ISINJECTEDCLASSNAME: u64 = 1854;
pub const METHOD_ISLAMBDA_1: u64 = 1855;
pub const METHOD_ISCAPTUREDRECORD: u64 = 1856;
pub const METHOD_GETDEFINITION_3: u64 = 1857;
pub const METHOD_ISORCONTAINSUNION: u64 = 1858;
pub const METHOD_FIELDS: u64 = 1859;
pub const METHOD_FIELD_BEGIN: u64 = 1860;
pub const METHOD_FIELD_END: u64 = 1861;
pub const METHOD_FIELD_EMPTY: u64 = 1862;
pub const METHOD_FINDFIRSTNAMEDDATAMEMBER: u64 = 1863;
pub const CLASS_XRAYLOGARGSATTR: u64 = 1864;
pub const CLASS_SWIFTATTRATTR: u64 = 1865;
pub const CLASS_POINTERTYPE: u64 = 1866;
pub const METHOD_GETPOINTEETYPE_4: u64 = 1867;
pub const METHOD_ISSUGARED_32: u64 = 1868;
pub const METHOD_DESUGAR_32: u64 = 1869;
pub const CLASS_ARTIFICIALATTR: u64 = 1870;
pub const CLASS_USINGPACKDECL: u64 = 1871;
pub const METHOD_GETINSTANTIATEDFROMUSINGDECL: u64 = 1872;
pub const METHOD_EXPANSIONS: u64 = 1873;
pub const METHOD_GETSOURCERANGE_35: u64 = 1874;
pub const METHOD_GETCANONICALDECL_25: u64 = 1875;
pub const CLASS_NSERRORDOMAINATTR: u64 = 1876;
pub const CLASS_RELEASEHANDLEATTR: u64 = 1877;
pub const CLASS_PARMVARDECL: u64 = 1878;
pub const METHOD_GETSOURCERANGE_21: u64 = 1879;
pub const METHOD_ISOBJCMETHODPARAMETER: u64 = 1880;
pub const METHOD_ISDESTROYEDINCALLEE: u64 = 1881;
pub const METHOD_GETFUNCTIONSCOPEDEPTH: u64 = 1882;
pub const METHOD_GETFUNCTIONSCOPEINDEX: u64 = 1883;
pub const METHOD_GETOBJCDECLQUALIFIER: u64 = 1884;
pub const METHOD_ISKNRPROMOTED: u64 = 1885;
pub const METHOD_ISEXPLICITOBJECTPARAMETER: u64 = 1886;
pub const METHOD_GETEXPLICITOBJECTPARAMTHISLOC: u64 = 1887;
pub const METHOD_GETDEFAULTARG: u64 = 1888;
pub const METHOD_GETDEFAULTARGRANGE: u64 = 1889;
pub const METHOD_GETUNINSTANTIATEDDEFAULTARG: u64 = 1890;
pub const METHOD_HASDEFAULTARG: u64 = 1891;
pub const METHOD_HASUNPARSEDDEFAULTARG: u64 = 1892;
pub const METHOD_HASUNINSTANTIATEDDEFAULTARG: u64 = 1893;
pub const METHOD_HASINHERITEDDEFAULTARG: u64 = 1894;
pub const METHOD_GETORIGINALTYPE_1: u64 = 1895;
pub const CLASS_VECTYPEHINTATTR: u64 = 1896;
pub const CLASS_SWIFTASYNCCONTEXTATTR: u64 = 1897;
pub const CLASS_BTFDECLTAGATTR: u64 = 1898;
pub const CLASS_WARNUNUSEDRESULTATTR: u64 = 1899;
pub const CLASS_CHARACTERLITERAL: u64 = 1900;
pub const METHOD_GETLOCATION_6: u64 = 1901;
pub const METHOD_GETKIND_3: u64 = 1902;
pub const METHOD_GETBEGINLOC_51: u64 = 1903;
pub const METHOD_GETENDLOC_50: u64 = 1904;
pub const METHOD_GETVALUE_6: u64 = 1905;
pub const METHOD_CHILDREN_41: u64 = 1906;
pub const CLASS_TYPE: u64 = 1907;
pub const METHOD_GETTYPECLASS: u64 = 1908;
pub const METHOD_ISFROMAST: u64 = 1909;
pub const METHOD_CONTAINSUNEXPANDEDPARAMETERPACK: u64 = 1910;
pub const METHOD_ISCANONICALUNQUALIFIED: u64 = 1911;
pub const METHOD_GETLOCALLYUNQUALIFIEDSINGLESTEPDESUGAREDTYPE: u64 = 1912;
pub const METHOD_ISSIZELESSTYPE: u64 = 1913;
pub const METHOD_ISSIZELESSBUILTINTYPE: u64 = 1914;
pub const METHOD_ISSIZELESSVECTORTYPE: u64 = 1915;
pub const METHOD_ISSVESIZELESSBUILTINTYPE: u64 = 1916;
pub const METHOD_ISRVVSIZELESSBUILTINTYPE: u64 = 1917;
pub const METHOD_ISWEBASSEMBLYEXTERNREFTYPE: u64 = 1918;
pub const METHOD_ISWEBASSEMBLYTABLETYPE: u64 = 1919;
pub const METHOD_ISSVEVLSBUILTINTYPE: u64 = 1920;
pub const METHOD_ISRVVVLSBUILTINTYPE: u64 = 1921;
pub const METHOD_ISINCOMPLETEOROBJECTTYPE: u64 = 1922;
pub const METHOD_ISOBJECTTYPE: u64 = 1923;
pub const METHOD_ISSTRUCTURALTYPE: u64 = 1924;
pub const METHOD_ISSTANDARDLAYOUTTYPE: u64 = 1925;
pub const METHOD_ISBUILTINTYPE: u64 = 1926;
pub const METHOD_ISPLACEHOLDERTYPE_1: u64 = 1927;
pub const METHOD_GETASPLACEHOLDERTYPE: u64 = 1928;
pub const METHOD_ISNONOVERLOADPLACEHOLDERTYPE_1: u64 = 1929;
pub const METHOD_ISINTEGERTYPE: u64 = 1930;
pub const METHOD_ISENUMERALTYPE: u64 = 1931;
pub const METHOD_ISSCOPEDENUMERALTYPE: u64 = 1932;
pub const METHOD_ISBOOLEANTYPE: u64 = 1933;
pub const METHOD_ISCHARTYPE: u64 = 1934;
pub const METHOD_ISWIDECHARTYPE: u64 = 1935;
pub const METHOD_ISCHAR8TYPE: u64 = 1936;
pub const METHOD_ISCHAR16TYPE: u64 = 1937;
pub const METHOD_ISCHAR32TYPE: u64 = 1938;
pub const METHOD_ISANYCHARACTERTYPE: u64 = 1939;
pub const METHOD_ISINTEGRALORENUMERATIONTYPE: u64 = 1940;
pub const METHOD_ISINTEGRALORUNSCOPEDENUMERATIONTYPE: u64 = 1941;
pub const METHOD_ISUNSCOPEDENUMERATIONTYPE: u64 = 1942;
pub const METHOD_ISREALFLOATINGTYPE: u64 = 1943;
pub const METHOD_ISCOMPLEXTYPE: u64 = 1944;
pub const METHOD_ISANYCOMPLEXTYPE: u64 = 1945;
pub const METHOD_ISFLOATINGTYPE: u64 = 1946;
pub const METHOD_ISHALFTYPE: u64 = 1947;
pub const METHOD_ISFLOAT16TYPE: u64 = 1948;
pub const METHOD_ISBFLOAT16TYPE: u64 = 1949;
pub const METHOD_ISFLOAT128TYPE: u64 = 1950;
pub const METHOD_ISIBM128TYPE: u64 = 1951;
pub const METHOD_ISREALTYPE: u64 = 1952;
pub const METHOD_ISARITHMETICTYPE: u64 = 1953;
pub const METHOD_ISVOIDTYPE: u64 = 1954;
pub const METHOD_ISSCALARTYPE: u64 = 1955;
pub const METHOD_ISAGGREGATETYPE: u64 = 1956;
pub const METHOD_ISFUNDAMENTALTYPE: u64 = 1957;
pub const METHOD_ISCOMPOUNDTYPE: u64 = 1958;
pub const METHOD_ISFUNCTIONTYPE: u64 = 1959;
pub const METHOD_ISFUNCTIONNOPROTOTYPE: u64 = 1960;
pub const METHOD_ISFUNCTIONPROTOTYPE: u64 = 1961;
pub const METHOD_ISPOINTERTYPE: u64 = 1962;
pub const METHOD_ISANYPOINTERTYPE: u64 = 1963;
pub const METHOD_ISBLOCKPOINTERTYPE: u64 = 1964;
pub const METHOD_ISVOIDPOINTERTYPE: u64 = 1965;
pub const METHOD_ISREFERENCETYPE: u64 = 1966;
pub const METHOD_ISLVALUEREFERENCETYPE: u64 = 1967;
pub const METHOD_ISRVALUEREFERENCETYPE: u64 = 1968;
pub const METHOD_ISOBJECTPOINTERTYPE: u64 = 1969;
pub const METHOD_ISFUNCTIONPOINTERTYPE_1: u64 = 1970;
pub const METHOD_ISFUNCTIONREFERENCETYPE: u64 = 1971;
pub const METHOD_ISMEMBERPOINTERTYPE: u64 = 1972;
pub const METHOD_ISMEMBERFUNCTIONPOINTERTYPE: u64 = 1973;
pub const METHOD_ISMEMBERDATAPOINTERTYPE: u64 = 1974;
pub const METHOD_ISARRAYTYPE: u64 = 1975;
pub const METHOD_ISCONSTANTARRAYTYPE: u64 = 1976;
pub const METHOD_ISINCOMPLETEARRAYTYPE: u64 = 1977;
pub const METHOD_ISVARIABLEARRAYTYPE: u64 = 1978;
pub const METHOD_ISDEPENDENTSIZEDARRAYTYPE: u64 = 1979;
pub const METHOD_ISRECORDTYPE: u64 = 1980;
pub const METHOD_ISCLASSTYPE: u64 = 1981;
pub const METHOD_ISSTRUCTURETYPE: u64 = 1982;
pub const METHOD_ISOBJCBOXABLERECORDTYPE: u64 = 1983;
pub const METHOD_ISINTERFACETYPE: u64 = 1984;
pub const METHOD_ISSTRUCTUREORCLASSTYPE: u64 = 1985;
pub const METHOD_ISUNIONTYPE: u64 = 1986;
pub const METHOD_ISCOMPLEXINTEGERTYPE: u64 = 1987;
pub const METHOD_ISVECTORTYPE: u64 = 1988;
pub const METHOD_ISEXTVECTORTYPE: u64 = 1989;
pub const METHOD_ISEXTVECTORBOOLTYPE: u64 = 1990;
pub const METHOD_ISMATRIXTYPE: u64 = 1991;
pub const METHOD_ISCONSTANTMATRIXTYPE: u64 = 1992;
pub const METHOD_ISDEPENDENTADDRESSSPACETYPE: u64 = 1993;
pub const METHOD_ISOBJCOBJECTPOINTERTYPE: u64 = 1994;
pub const METHOD_ISOBJCRETAINABLETYPE: u64 = 1995;
pub const METHOD_ISOBJCLIFETIMETYPE: u64 = 1996;
pub const METHOD_ISOBJCINDIRECTLIFETIMETYPE: u64 = 1997;
pub const METHOD_ISOBJCNSOBJECTTYPE: u64 = 1998;
pub const METHOD_ISOBJCINDEPENDENTCLASSTYPE: u64 = 1999;
pub const METHOD_ISOBJCOBJECTTYPE: u64 = 2000;
pub const METHOD_ISOBJCQUALIFIEDINTERFACETYPE: u64 = 2001;
pub const METHOD_ISOBJCQUALIFIEDIDTYPE: u64 = 2002;
pub const METHOD_ISOBJCQUALIFIEDCLASSTYPE: u64 = 2003;
pub const METHOD_ISOBJCOBJECTORINTERFACETYPE: u64 = 2004;
pub const METHOD_ISOBJCIDTYPE: u64 = 2005;
pub const METHOD_ISDECLTYPETYPE: u64 = 2006;
pub const METHOD_ISOBJCINERTUNSAFEUNRETAINEDTYPE: u64 = 2007;
pub const METHOD_ISOBJCCLASSTYPE: u64 = 2008;
pub const METHOD_ISOBJCCLASSORCLASSKINDOFTYPE: u64 = 2009;
pub const METHOD_ISOBJCSELTYPE: u64 = 2010;
pub const METHOD_ISOBJCBUILTINTYPE: u64 = 2011;
pub const METHOD_ISOBJCARCBRIDGABLETYPE: u64 = 2012;
pub const METHOD_ISCARCBRIDGABLETYPE: u64 = 2013;
pub const METHOD_ISTEMPLATETYPEPARMTYPE: u64 = 2014;
pub const METHOD_ISNULLPTRTYPE: u64 = 2015;
pub const METHOD_ISNOTHROWT: u64 = 2016;
pub const METHOD_ISALIGNVALT: u64 = 2017;
pub const METHOD_ISSTDBYTETYPE: u64 = 2018;
pub const METHOD_ISATOMICTYPE: u64 = 2019;
pub const METHOD_ISUNDEDUCEDAUTOTYPE: u64 = 2020;
pub const METHOD_ISTYPEDEFNAMETYPE: u64 = 2021;
pub const METHOD_ISOCLIMAGE1DROTYPE: u64 = 2022;
pub const METHOD_ISOCLIMAGE1DARRAYROTYPE: u64 = 2023;
pub const METHOD_ISOCLIMAGE1DBUFFERROTYPE: u64 = 2024;
pub const METHOD_ISOCLIMAGE2DROTYPE: u64 = 2025;
pub const METHOD_ISOCLIMAGE2DARRAYROTYPE: u64 = 2026;
pub const METHOD_ISOCLIMAGE2DDEPTHROTYPE: u64 = 2027;
pub const METHOD_ISOCLIMAGE2DARRAYDEPTHROTYPE: u64 = 2028;
pub const METHOD_ISOCLIMAGE2DMSAAROTYPE: u64 = 2029;
pub const METHOD_ISOCLIMAGE2DARRAYMSAAROTYPE: u64 = 2030;
pub const METHOD_ISOCLIMAGE2DMSAADEPTHROTYPE: u64 = 2031;
pub const METHOD_ISOCLIMAGE2DARRAYMSAADEPTHROTYPE: u64 = 2032;
pub const METHOD_ISOCLIMAGE3DROTYPE: u64 = 2033;
pub const METHOD_ISOCLIMAGE1DWOTYPE: u64 = 2034;
pub const METHOD_ISOCLIMAGE1DARRAYWOTYPE: u64 = 2035;
pub const METHOD_ISOCLIMAGE1DBUFFERWOTYPE: u64 = 2036;
pub const METHOD_ISOCLIMAGE2DWOTYPE: u64 = 2037;
pub const METHOD_ISOCLIMAGE2DARRAYWOTYPE: u64 = 2038;
pub const METHOD_ISOCLIMAGE2DDEPTHWOTYPE: u64 = 2039;
pub const METHOD_ISOCLIMAGE2DARRAYDEPTHWOTYPE: u64 = 2040;
pub const METHOD_ISOCLIMAGE2DMSAAWOTYPE: u64 = 2041;
pub const METHOD_ISOCLIMAGE2DARRAYMSAAWOTYPE: u64 = 2042;
pub const METHOD_ISOCLIMAGE2DMSAADEPTHWOTYPE: u64 = 2043;
pub const METHOD_ISOCLIMAGE2DARRAYMSAADEPTHWOTYPE: u64 = 2044;
pub const METHOD_ISOCLIMAGE3DWOTYPE: u64 = 2045;
pub const METHOD_ISOCLIMAGE1DRWTYPE: u64 = 2046;
pub const METHOD_ISOCLIMAGE1DARRAYRWTYPE: u64 = 2047;
pub const METHOD_ISOCLIMAGE1DBUFFERRWTYPE: u64 = 2048;
pub const METHOD_ISOCLIMAGE2DRWTYPE: u64 = 2049;
pub const METHOD_ISOCLIMAGE2DARRAYRWTYPE: u64 = 2050;
pub const METHOD_ISOCLIMAGE2DDEPTHRWTYPE: u64 = 2051;
pub const METHOD_ISOCLIMAGE2DARRAYDEPTHRWTYPE: u64 = 2052;
pub const METHOD_ISOCLIMAGE2DMSAARWTYPE: u64 = 2053;
pub const METHOD_ISOCLIMAGE2DARRAYMSAARWTYPE: u64 = 2054;
pub const METHOD_ISOCLIMAGE2DMSAADEPTHRWTYPE: u64 = 2055;
pub const METHOD_ISOCLIMAGE2DARRAYMSAADEPTHRWTYPE: u64 = 2056;
pub const METHOD_ISOCLIMAGE3DRWTYPE: u64 = 2057;
pub const METHOD_ISIMAGETYPE: u64 = 2058;
pub const METHOD_ISSAMPLERT: u64 = 2059;
pub const METHOD_ISEVENTT: u64 = 2060;
pub const METHOD_ISCLKEVENTT: u64 = 2061;
pub const METHOD_ISQUEUET: u64 = 2062;
pub const METHOD_ISRESERVEIDT: u64 = 2063;
pub const METHOD_ISOCLINTELSUBGROUPAVCMCEPAYLOADTYPE: u64 = 2064;
pub const METHOD_ISOCLINTELSUBGROUPAVCIMEPAYLOADTYPE: u64 = 2065;
pub const METHOD_ISOCLINTELSUBGROUPAVCREFPAYLOADTYPE: u64 = 2066;
pub const METHOD_ISOCLINTELSUBGROUPAVCSICPAYLOADTYPE: u64 = 2067;
pub const METHOD_ISOCLINTELSUBGROUPAVCMCERESULTTYPE: u64 = 2068;
pub const METHOD_ISOCLINTELSUBGROUPAVCIMERESULTTYPE: u64 = 2069;
pub const METHOD_ISOCLINTELSUBGROUPAVCREFRESULTTYPE: u64 = 2070;
pub const METHOD_ISOCLINTELSUBGROUPAVCSICRESULTTYPE: u64 = 2071;
pub const METHOD_ISOCLINTELSUBGROUPAVCIMERESULTSINGLEREFERENCESTREAMOUTTYPE: u64 = 2072;
pub const METHOD_ISOCLINTELSUBGROUPAVCIMERESULTDUALREFERENCESTREAMOUTTYPE: u64 = 2073;
pub const METHOD_ISOCLINTELSUBGROUPAVCIMESINGLEREFERENCESTREAMINTYPE: u64 = 2074;
pub const METHOD_ISOCLINTELSUBGROUPAVCIMEDUALREFERENCESTREAMINTYPE: u64 = 2075;
pub const METHOD_ISOCLINTELSUBGROUPAVCTYPE: u64 = 2076;
pub const METHOD_ISOCLEXTOPAQUETYPE: u64 = 2077;
pub const METHOD_ISPIPETYPE: u64 = 2078;
pub const METHOD_ISBITINTTYPE: u64 = 2079;
pub const METHOD_ISOPENCLSPECIFICTYPE: u64 = 2080;
pub const METHOD_ISOBJCARCIMPLICITLYUNRETAINEDTYPE: u64 = 2081;
pub const METHOD_ISCUDADEVICEBUILTINSURFACETYPE: u64 = 2082;
pub const METHOD_ISCUDADEVICEBUILTINTEXTURETYPE: u64 = 2083;
pub const METHOD_GETOBJCARCIMPLICITLIFETIME: u64 = 2084;
pub const METHOD_GETSCALARTYPEKIND: u64 = 2085;
pub const METHOD_GETDEPENDENCE: u64 = 2086;
pub const METHOD_CONTAINSERRORS: u64 = 2087;
pub const METHOD_ISDEPENDENTTYPE_1: u64 = 2088;
pub const METHOD_ISINSTANTIATIONDEPENDENTTYPE: u64 = 2089;
pub const METHOD_ISUNDEDUCEDTYPE: u64 = 2090;
pub const METHOD_ISVARIABLYMODIFIEDTYPE: u64 = 2091;
pub const METHOD_HASSIZEDVLATYPE: u64 = 2092;
pub const METHOD_HASUNNAMEDORLOCALTYPE: u64 = 2093;
pub const METHOD_ISOVERLOADABLETYPE: u64 = 2094;
pub const METHOD_ISELABORATEDTYPESPECIFIER: u64 = 2095;
pub const METHOD_CANDECAYTOPOINTERTYPE: u64 = 2096;
pub const METHOD_HASPOINTERREPRESENTATION: u64 = 2097;
pub const METHOD_HASOBJCPOINTERREPRESENTATION: u64 = 2098;
pub const METHOD_HASINTEGERREPRESENTATION: u64 = 2099;
pub const METHOD_HASSIGNEDINTEGERREPRESENTATION: u64 = 2100;
pub const METHOD_HASUNSIGNEDINTEGERREPRESENTATION: u64 = 2101;
pub const METHOD_HASFLOATINGREPRESENTATION: u64 = 2102;
pub const METHOD_GETASSTRUCTURETYPE: u64 = 2103;
pub const METHOD_GETASUNIONTYPE: u64 = 2104;
pub const METHOD_GETASCOMPLEXINTEGERTYPE: u64 = 2105;
pub const METHOD_GETASOBJCINTERFACETYPE: u64 = 2106;
pub const METHOD_GETASOBJCINTERFACEPOINTERTYPE: u64 = 2107;
pub const METHOD_GETASOBJCQUALIFIEDIDTYPE: u64 = 2108;
pub const METHOD_GETASOBJCQUALIFIEDCLASSTYPE: u64 = 2109;
pub const METHOD_GETASOBJCQUALIFIEDINTERFACETYPE: u64 = 2110;
pub const METHOD_GETASCXXRECORDDECL: u64 = 2111;
pub const METHOD_GETASRECORDDECL: u64 = 2112;
pub const METHOD_GETASTAGDECL: u64 = 2113;
pub const METHOD_GETPOINTEECXXRECORDDECL: u64 = 2114;
pub const METHOD_GETCONTAINEDDEDUCEDTYPE: u64 = 2115;
pub const METHOD_GETCONTAINEDAUTOTYPE: u64 = 2116;
pub const METHOD_HASAUTOFORTRAILINGRETURNTYPE: u64 = 2117;
pub const METHOD_GETASARRAYTYPEUNSAFE: u64 = 2118;
pub const METHOD_CASTASARRAYTYPEUNSAFE: u64 = 2119;
pub const METHOD_GETBASEELEMENTTYPEUNSAFE: u64 = 2120;
pub const METHOD_GETARRAYELEMENTTYPENOTYPEQUAL: u64 = 2121;
pub const METHOD_GETPOINTEEORARRAYELEMENTTYPE: u64 = 2122;
pub const METHOD_GETPOINTEETYPE_6: u64 = 2123;
pub const METHOD_GETUNQUALIFIEDDESUGAREDTYPE: u64 = 2124;
pub const METHOD_ISSIGNEDINTEGERTYPE: u64 = 2125;
pub const METHOD_ISUNSIGNEDINTEGERTYPE: u64 = 2126;
pub const METHOD_ISSIGNEDINTEGERORENUMERATIONTYPE: u64 = 2127;
pub const METHOD_ISUNSIGNEDINTEGERORENUMERATIONTYPE: u64 = 2128;
pub const METHOD_ISFIXEDPOINTTYPE: u64 = 2129;
pub const METHOD_ISFIXEDPOINTORINTEGERTYPE: u64 = 2130;
pub const METHOD_ISSATURATEDFIXEDPOINTTYPE: u64 = 2131;
pub const METHOD_ISUNSATURATEDFIXEDPOINTTYPE: u64 = 2132;
pub const METHOD_ISSIGNEDFIXEDPOINTTYPE: u64 = 2133;
pub const METHOD_ISUNSIGNEDFIXEDPOINTTYPE: u64 = 2134;
pub const METHOD_ISCONSTANTSIZETYPE: u64 = 2135;
pub const METHOD_ISSPECIFIERTYPE: u64 = 2136;
pub const METHOD_GETLINKAGE: u64 = 2137;
pub const METHOD_GETVISIBILITY: u64 = 2138;
pub const METHOD_ISVISIBILITYEXPLICIT: u64 = 2139;
pub const METHOD_GETLINKAGEANDVISIBILITY: u64 = 2140;
pub const METHOD_ISLINKAGEVALID_1: u64 = 2141;
pub const METHOD_GETNULLABILITY: u64 = 2142;
pub const METHOD_ACCEPTSOBJCTYPEPARAMS: u64 = 2143;
pub const METHOD_GETTYPECLASSNAME: u64 = 2144;
pub const METHOD_GETCANONICALTYPEINTERNAL: u64 = 2145;
pub const METHOD_GETCANONICALTYPEUNQUALIFIED: u64 = 2146;
pub const CLASS_INITSEGATTR: u64 = 2147;
pub const CLASS_OBJCCOMPATIBLEALIASDECL: u64 = 2148;
pub const CLASS_TRYACQUIRECAPABILITYATTR: u64 = 2149;
pub const CLASS_TRANSPARENTUNIONATTR: u64 = 2150;
pub const CLASS_SWIFTERRORATTR: u64 = 2151;
pub const CLASS_SWIFTASYNCCALLATTR: u64 = 2152;
pub const CLASS_TYPETRAITEXPR: u64 = 2153;
pub const METHOD_GETTRAIT_2: u64 = 2154;
pub const METHOD_GETVALUE_10: u64 = 2155;
pub const METHOD_GETNUMARGS_4: u64 = 2156;
pub const METHOD_GETARGS_2: u64 = 2157;
pub const METHOD_GETBEGINLOC_124: u64 = 2158;
pub const METHOD_GETENDLOC_123: u64 = 2159;
pub const METHOD_CHILDREN_111: u64 = 2160;
pub const CLASS_M68KINTERRUPTATTR: u64 = 2161;
pub const CLASS_SWIFTASYNCATTR: u64 = 2162;
pub const CLASS_SECTIONATTR: u64 = 2163;
pub const CLASS_OSRETURNSNOTRETAINEDATTR: u64 = 2164;
pub const CLASS_CDECLATTR: u64 = 2165;
pub const CLASS_CONVERGENTATTR: u64 = 2166;
pub const CLASS_NOTHROWATTR: u64 = 2167;
pub const CLASS_SENTINELATTR: u64 = 2168;
pub const CLASS_NOESCAPEATTR: u64 = 2169;
pub const CLASS_OBJCPRECISELIFETIMEATTR: u64 = 2170;
pub const CLASS_ALIGNEDATTR: u64 = 2171;
pub const CLASS_CONSUMABLEAUTOCASTATTR: u64 = 2172;
pub const CLASS_CONSTRUCTORATTR: u64 = 2173;
pub const CLASS_MATRIXSUBSCRIPTEXPR: u64 = 2174;
pub const METHOD_ISINCOMPLETE: u64 = 2175;
pub const METHOD_GETBASE_5: u64 = 2176;
pub const METHOD_GETROWIDX: u64 = 2177;
pub const METHOD_GETCOLUMNIDX: u64 = 2178;
pub const METHOD_GETBEGINLOC_96: u64 = 2179;
pub const METHOD_GETENDLOC_95: u64 = 2180;
pub const METHOD_GETEXPRLOC_8: u64 = 2181;
pub const METHOD_GETRBRACKETLOC_4: u64 = 2182;
pub const METHOD_CHILDREN_84: u64 = 2183;
pub const CLASS_ALIASATTR: u64 = 2184;
pub const CLASS_OBJCCLASSSTUBATTR: u64 = 2185;
pub const CLASS_CXX11NORETURNATTR: u64 = 2186;
pub const CLASS_REGCALLATTR: u64 = 2187;
pub const CLASS_CXXMEMBERCALLEXPR: u64 = 2188;
pub const METHOD_GETIMPLICITOBJECTARGUMENT: u64 = 2189;
pub const METHOD_GETOBJECTTYPE: u64 = 2190;
pub const METHOD_GETMETHODDECL: u64 = 2191;
pub const METHOD_GETRECORDDECL: u64 = 2192;
pub const METHOD_GETEXPRLOC_3: u64 = 2193;
pub const CLASS_WARNUNUSEDATTR: u64 = 2194;
pub const CLASS_NONNULLATTR: u64 = 2195;
pub const CLASS_MUSTTAILATTR: u64 = 2196;
pub const CLASS_AMDGPUWAVESPEREUATTR: u64 = 2197;
pub const CLASS_CFCONSUMEDATTR: u64 = 2198;
pub const CLASS_OMPTEAMSDIRECTIVE: u64 = 2199;
pub const CLASS_CLASSTEMPLATESPECIALIZATIONDECL: u64 = 2200;
pub const METHOD_GETSPECIALIZEDTEMPLATE: u64 = 2201;
pub const METHOD_GETTEMPLATEARGS: u64 = 2202;
pub const METHOD_GETSPECIALIZATIONKIND: u64 = 2203;
pub const METHOD_ISEXPLICITSPECIALIZATION: u64 = 2204;
pub const METHOD_ISCLASSSCOPEEXPLICITSPECIALIZATION: u64 = 2205;
pub const METHOD_ISEXPLICITINSTANTIATIONORSPECIALIZATION: u64 = 2206;
pub const METHOD_GETPOINTOFINSTANTIATION: u64 = 2207;
pub const METHOD_GETINSTANTIATEDFROM: u64 = 2208;
pub const METHOD_GETSPECIALIZEDTEMPLATEORPARTIAL: u64 = 2209;
pub const METHOD_GETTEMPLATEINSTANTIATIONARGS: u64 = 2210;
pub const METHOD_GETTYPEASWRITTEN: u64 = 2211;
pub const METHOD_GETEXTERNLOC: u64 = 2212;
pub const METHOD_GETTEMPLATEKEYWORDLOC: u64 = 2213;
pub const METHOD_GETSOURCERANGE_3: u64 = 2214;
pub const CLASS_NOINLINEATTR: u64 = 2215;
pub const CLASS_OBJCINDEPENDENTCLASSATTR: u64 = 2216;
pub const CLASS_PARAMETERABIATTR: u64 = 2217;
pub const CLASS_UPTRATTR: u64 = 2218;
pub const CLASS_OMPPARALLELFORDIRECTIVE: u64 = 2219;
pub const CLASS_ASMLABELATTR: u64 = 2220;
pub const CLASS_ACQUIREHANDLEATTR: u64 = 2221;
pub const CLASS_SWIFTCONTEXTATTR: u64 = 2222;
pub const CLASS_AVRSIGNALATTR: u64 = 2223;
pub const CLASS_OPENCLKERNELATTR: u64 = 2224;
pub const CLASS_DEPENDENTUNARYTRANSFORMTYPE: u64 = 2225;
pub const CLASS_ARMPRESERVESATTR: u64 = 2226;
pub const CLASS_STRICTFPATTR: u64 = 2227;
pub const CLASS_CFGUARDATTR: u64 = 2228;
pub const CLASS_CUDADEVICEBUILTINTEXTURETYPEATTR: u64 = 2229;
pub const CLASS_NSCONSUMEDATTR: u64 = 2230;
pub const CLASS_ARMMVESTRICTPOLYMORPHISMATTR: u64 = 2231;
pub const CLASS_SWIFTERRORRESULTATTR: u64 = 2232;
pub const CLASS_CXXUUIDOFEXPR: u64 = 2233;
pub const METHOD_ISTYPEOPERAND_1: u64 = 2234;
pub const METHOD_GETTYPEOPERANDSOURCEINFO_1: u64 = 2235;
pub const METHOD_GETEXPROPERAND_1: u64 = 2236;
pub const METHOD_GETGUIDDECL: u64 = 2237;
pub const METHOD_GETBEGINLOC_47: u64 = 2238;
pub const METHOD_GETENDLOC_46: u64 = 2239;
pub const METHOD_GETSOURCERANGE_46: u64 = 2240;
pub const METHOD_CHILDREN_36: u64 = 2241;
pub const CLASS_THREADATTR: u64 = 2242;
pub const CLASS_OBJCGCATTR: u64 = 2243;
pub const CLASS_TYPEALIASDECL: u64 = 2244;
pub const METHOD_GETSOURCERANGE_28: u64 = 2245;
pub const METHOD_GETDESCRIBEDALIASTEMPLATE: u64 = 2246;
pub const CLASS_NSRETURNSAUTORELEASEDATTR: u64 = 2247;
pub const CLASS_ALLOCALIGNATTR: u64 = 2248;
pub const CLASS_OPENCLCONSTANTADDRESSSPACEATTR: u64 = 2249;
pub const CLASS_OMPREFERENCEDVARATTR: u64 = 2250;
pub const CLASS_CFRETURNSNOTRETAINEDATTR: u64 = 2251;
pub const CLASS_REQUIRESEXPRBODYDECL: u64 = 2252;
pub const CLASS_THISCALLATTR: u64 = 2253;
pub const CLASS_OBJCINTERFACETYPELOC: u64 = 2254;
pub const CLASS_ARMSTREAMINGATTR: u64 = 2255;
pub const CLASS_VECTORTYPELOC: u64 = 2256;
pub const METHOD_GETNAMELOC_2: u64 = 2257;
pub const METHOD_GETLOCALSOURCERANGE: u64 = 2258;
pub const METHOD_GETELEMENTLOC: u64 = 2259;
pub const METHOD_GETINNERTYPE_1: u64 = 2260;
pub const CLASS_OMPCAPTURENOINITATTR: u64 = 2261;
pub const CLASS_SWIFTIMPORTPROPERTYASACCESSORSATTR: u64 = 2262;
pub const CLASS_ACQUIREDAFTERATTR: u64 = 2263;
pub const CLASS_ATTRIBUTEDTYPELOC: u64 = 2264;
pub const METHOD_GETATTRKIND_1: u64 = 2265;
pub const METHOD_ISQUALIFIER_1: u64 = 2266;
pub const METHOD_GETMODIFIEDLOC: u64 = 2267;
pub const METHOD_GETATTR_1: u64 = 2268;
pub const METHOD_GETLOCALSOURCERANGE_1: u64 = 2269;
pub const METHOD_GETINNERTYPE_2: u64 = 2270;
pub const CLASS_OBJCRUNTIMEVISIBLEATTR: u64 = 2271;
pub const CLASS_TYPENONNULLATTR: u64 = 2272;
pub const CLASS_SYCLSPECIALCLASSATTR: u64 = 2273;
pub const CLASS_ARGUMENTWITHTYPETAGATTR: u64 = 2274;
pub const CLASS_AARCH64SVEPCSATTR: u64 = 2275;
pub const CLASS_PRAGMACLANGRODATASECTIONATTR: u64 = 2276;
pub const CLASS_DEPENDENTVECTORTYPE: u64 = 2277;
pub const METHOD_GETSIZEEXPR_3: u64 = 2278;
pub const METHOD_GETELEMENTTYPE_3: u64 = 2279;
pub const METHOD_GETATTRIBUTELOC_3: u64 = 2280;
pub const METHOD_GETVECTORKIND: u64 = 2281;
pub const METHOD_ISSUGARED_17: u64 = 2282;
pub const METHOD_DESUGAR_17: u64 = 2283;
pub const CLASS_NODEREFATTR: u64 = 2284;
pub const CLASS_ARMSTREAMINGCOMPATIBLEATTR: u64 = 2285;
pub const CLASS_ASSUMPTIONATTR: u64 = 2286;
pub const CLASS_FASTCALLATTR: u64 = 2287;
pub const CLASS_TYPELOC: u64 = 2288;
pub const METHOD_GETTYPELOCCLASS: u64 = 2289;
pub const METHOD_ISNULL: u64 = 2290;
pub const METHOD_GETTYPE_2: u64 = 2291;
pub const METHOD_GETTYPEPTR: u64 = 2292;
pub const METHOD_GETOPAQUEDATA: u64 = 2293;
pub const METHOD_GETBEGINLOC_134: u64 = 2294;
pub const METHOD_GETENDLOC_133: u64 = 2295;
pub const METHOD_GETSOURCERANGE_50: u64 = 2296;
pub const METHOD_GETLOCALSOURCERANGE_2: u64 = 2297;
pub const METHOD_GETFULLDATASIZE: u64 = 2298;
pub const METHOD_GETNEXTTYPELOC: u64 = 2299;
pub const METHOD_GETUNQUALIFIEDLOC: u64 = 2300;
pub const METHOD_IGNOREPARENS_1: u64 = 2301;
pub const METHOD_FINDEXPLICITQUALIFIERLOC: u64 = 2302;
pub const METHOD_GETCONTAINEDAUTOTYPELOC: u64 = 2303;
pub const METHOD_FINDNULLABILITYLOC: u64 = 2304;
pub const CLASS_OPENCLLOCALADDRESSSPACEATTR: u64 = 2305;
pub const CLASS_ADJUSTEDTYPE: u64 = 2306;
pub const METHOD_GETORIGINALTYPE: u64 = 2307;
pub const METHOD_GETADJUSTEDTYPE: u64 = 2308;
pub const METHOD_ISSUGARED: u64 = 2309;
pub const METHOD_DESUGAR: u64 = 2310;
pub const CLASS_DEPENDENTSIZEDEXTVECTORTYPE: u64 = 2311;
pub const METHOD_GETSIZEEXPR_2: u64 = 2312;
pub const METHOD_GETELEMENTTYPE_2: u64 = 2313;
pub const METHOD_GETATTRIBUTELOC_1: u64 = 2314;
pub const METHOD_ISSUGARED_15: u64 = 2315;
pub const METHOD_DESUGAR_15: u64 = 2316;
pub const CLASS_OBJCINERTUNSAFEUNRETAINEDATTR: u64 = 2317;
pub const CLASS_ARRAYINITLOOPEXPR: u64 = 2318;
pub const METHOD_GETCOMMONEXPR: u64 = 2319;
pub const METHOD_GETSUBEXPR: u64 = 2320;
pub const METHOD_GETARRAYSIZE: u64 = 2321;
pub const METHOD_GETBEGINLOC_6: u64 = 2322;
pub const METHOD_GETENDLOC_5: u64 = 2323;
pub const METHOD_CHILDREN_2: u64 = 2324;
pub const CLASS_RECORDTYPE: u64 = 2325;
pub const METHOD_GETDECL_2: u64 = 2326;
pub const METHOD_HASCONSTFIELDS: u64 = 2327;
pub const METHOD_ISSUGARED_34: u64 = 2328;
pub const METHOD_DESUGAR_34: u64 = 2329;
pub const CLASS_USINGIFEXISTSATTR: u64 = 2330;
pub const CLASS_SHAREDTRYLOCKFUNCTIONATTR: u64 = 2331;
pub const CLASS_DEDUCEDTEMPLATESPECIALIZATIONTYPE: u64 = 2332;
pub const METHOD_GETTEMPLATENAME: u64 = 2333;
pub const CLASS_SWIFTNAMEATTR: u64 = 2334;
pub const CLASS_ALLOCSIZEATTR: u64 = 2335;
pub const CLASS_VARDECL: u64 = 2336;
pub const METHOD_GETSOURCERANGE_36: u64 = 2337;
pub const METHOD_GETSTORAGECLASS_1: u64 = 2338;
pub const METHOD_GETTSCSPEC: u64 = 2339;
pub const METHOD_GETTLSKIND: u64 = 2340;
pub const METHOD_HASLOCALSTORAGE: u64 = 2341;
pub const METHOD_ISSTATICLOCAL: u64 = 2342;
pub const METHOD_HASEXTERNALSTORAGE: u64 = 2343;
pub const METHOD_HASGLOBALSTORAGE: u64 = 2344;
pub const METHOD_GETSTORAGEDURATION_1: u64 = 2345;
pub const METHOD_GETLANGUAGELINKAGE_1: u64 = 2346;
pub const METHOD_ISEXTERNC_1: u64 = 2347;
pub const METHOD_ISINEXTERNCCONTEXT_1: u64 = 2348;
pub const METHOD_ISINEXTERNCXXCONTEXT_1: u64 = 2349;
pub const METHOD_ISLOCALVARDECL: u64 = 2350;
pub const METHOD_ISLOCALVARDECLORPARM: u64 = 2351;
pub const METHOD_ISFUNCTIONORMETHODVARDECL: u64 = 2352;
pub const METHOD_ISSTATICDATAMEMBER: u64 = 2353;
pub const METHOD_GETCANONICALDECL_27: u64 = 2354;
pub const METHOD_ISTHISDECLARATIONADEFINITION_4: u64 = 2355;
pub const METHOD_HASDEFINITION_1: u64 = 2356;
pub const METHOD_GETACTINGDEFINITION: u64 = 2357;
pub const METHOD_GETDEFINITION_5: u64 = 2358;
pub const METHOD_ISOUTOFLINE_2: u64 = 2359;
pub const METHOD_ISFILEVARDECL: u64 = 2360;
pub const METHOD_GETANYINITIALIZER: u64 = 2361;
pub const METHOD_HASINIT: u64 = 2362;
pub const METHOD_GETINIT: u64 = 2363;
pub const METHOD_GETINITIALIZINGDECLARATION: u64 = 2364;
pub const METHOD_ENSUREEVALUATEDSTMT: u64 = 2365;
pub const METHOD_GETEVALUATEDSTMT: u64 = 2366;
pub const METHOD_EVALUATEVALUE: u64 = 2367;
pub const METHOD_GETEVALUATEDVALUE: u64 = 2368;
pub const METHOD_HASCONSTANTINITIALIZATION: u64 = 2369;
pub const METHOD_GETINITSTYLE: u64 = 2370;
pub const METHOD_ISDIRECTINIT: u64 = 2371;
pub const METHOD_ISTHISDECLARATIONADEMOTEDDEFINITION_1: u64 = 2372;
pub const METHOD_ISEXCEPTIONVARIABLE: u64 = 2373;
pub const METHOD_ISNRVOVARIABLE: u64 = 2374;
pub const METHOD_ISCXXFORRANGEDECL: u64 = 2375;
pub const METHOD_ISOBJCFORDECL: u64 = 2376;
pub const METHOD_ISARCPSEUDOSTRONG: u64 = 2377;
pub const METHOD_ISINLINE_1: u64 = 2378;
pub const METHOD_ISINLINESPECIFIED_1: u64 = 2379;
pub const METHOD_ISCONSTEXPR_1: u64 = 2380;
pub const METHOD_ISINITCAPTURE_1: u64 = 2381;
pub const METHOD_ISPARAMETERPACK_5: u64 = 2382;
pub const METHOD_ISPREVIOUSDECLINSAMEBLOCKSCOPE: u64 = 2383;
pub const METHOD_ISESCAPINGBYREF: u64 = 2384;
pub const METHOD_ISNONESCAPINGBYREF: u64 = 2385;
pub const METHOD_HASDEPENDENTALIGNMENT: u64 = 2386;
pub const METHOD_GETTEMPLATEINSTANTIATIONPATTERN_2: u64 = 2387;
pub const METHOD_GETINSTANTIATEDFROMSTATICDATAMEMBER: u64 = 2388;
pub const METHOD_GETTEMPLATESPECIALIZATIONKIND_3: u64 = 2389;
pub const METHOD_GETTEMPLATESPECIALIZATIONKINDFORINSTANTIATION_1: u64 = 2390;
pub const METHOD_GETPOINTOFINSTANTIATION_2: u64 = 2391;
pub const METHOD_GETMEMBERSPECIALIZATIONINFO_3: u64 = 2392;
pub const METHOD_GETDESCRIBEDVARTEMPLATE: u64 = 2393;
pub const METHOD_ISKNOWNTOBEDEFINED: u64 = 2394;
pub const CLASS_COROLIFETIMEBOUNDATTR: u64 = 2395;
pub const CLASS_OMPTARGETDIRECTIVE: u64 = 2396;
pub const CLASS_PACKEXPANSIONEXPR: u64 = 2397;
pub const METHOD_GETPATTERN_2: u64 = 2398;
pub const METHOD_GETELLIPSISLOC_6: u64 = 2399;
pub const METHOD_GETNUMEXPANSIONS_3: u64 = 2400;
pub const METHOD_GETBEGINLOC_102: u64 = 2401;
pub const METHOD_GETENDLOC_101: u64 = 2402;
pub const METHOD_CHILDREN_90: u64 = 2403;
pub const CLASS_TAGTYPE: u64 = 2404;
pub const METHOD_GETDECL_3: u64 = 2405;
pub const METHOD_ISBEINGDEFINED: u64 = 2406;
pub const CLASS_TYPEOFTYPE: u64 = 2407;
pub const METHOD_GETUNMODIFIEDTYPE: u64 = 2408;
pub const METHOD_DESUGAR_40: u64 = 2409;
pub const METHOD_ISSUGARED_40: u64 = 2410;
pub const METHOD_GETKIND_2: u64 = 2411;
pub const CLASS_HLSLSV_GROUPINDEXATTR: u64 = 2412;
pub const CLASS_OBJCNSOBJECTATTR: u64 = 2413;
pub const CLASS_OMPTASKWAITDIRECTIVE: u64 = 2414;
pub const CLASS_REQUIRESCAPABILITYATTR: u64 = 2415;
pub const CLASS_CORODISABLELIFETIMEBOUNDATTR: u64 = 2416;
pub const CLASS_CMSENSCALLATTR: u64 = 2417;
pub const CLASS_SWIFTBRIDGEATTR: u64 = 2418;
pub const CLASS_ANALYZERNORETURNATTR: u64 = 2419;
pub const CLASS_BTFTYPETAGATTR: u64 = 2420;
pub const CLASS_ANYX86INTERRUPTATTR: u64 = 2421;
pub const CLASS_TYPENULLABLEATTR: u64 = 2422;
pub const CLASS_SWIFTIMPORTASNONGENERICATTR: u64 = 2423;
pub const CLASS_AUTOTYPE: u64 = 2424;
pub const METHOD_GETTYPECONSTRAINTARGUMENTS: u64 = 2425;
pub const METHOD_GETTYPECONSTRAINTCONCEPT: u64 = 2426;
pub const METHOD_ISCONSTRAINED: u64 = 2427;
pub const METHOD_ISDECLTYPEAUTO: u64 = 2428;
pub const METHOD_ISGNUAUTOTYPE: u64 = 2429;
pub const METHOD_GETKEYWORD: u64 = 2430;
pub const CLASS_ARMINATTR: u64 = 2431;
pub const CLASS_LIKELYATTR: u64 = 2432;
pub const CLASS_OPENCLGLOBALADDRESSSPACEATTR: u64 = 2433;
pub const CLASS_CODEALIGNATTR: u64 = 2434;
pub const CLASS_CALLABLEWHENATTR: u64 = 2435;
pub const CLASS_OPENCLGLOBALDEVICEADDRESSSPACEATTR: u64 = 2436;
pub const CLASS_OBJCMESSAGEEXPR: u64 = 2437;
pub const CLASS_CUDASHAREDATTR: u64 = 2438;
pub const CLASS_VAARGEXPR: u64 = 2439;
pub const METHOD_GETSUBEXPR_10: u64 = 2440;
pub const METHOD_ISMICROSOFTABI: u64 = 2441;
pub const METHOD_GETWRITTENTYPEINFO: u64 = 2442;
pub const METHOD_GETBUILTINLOC_5: u64 = 2443;
pub const METHOD_GETRPARENLOC_27: u64 = 2444;
pub const METHOD_GETBEGINLOC_131: u64 = 2445;
pub const METHOD_GETENDLOC_130: u64 = 2446;
pub const METHOD_CHILDREN_117: u64 = 2447;
pub const CLASS_ENUMDECL: u64 = 2448;
pub const METHOD_GETCANONICALDECL_9: u64 = 2449;
pub const METHOD_GETPREVIOUSDECL_3: u64 = 2450;
pub const METHOD_GETMOSTRECENTDECL_4: u64 = 2451;
pub const METHOD_GETDEFINITION_1: u64 = 2452;
pub const METHOD_GETSOURCERANGE_8: u64 = 2453;
pub const METHOD_ENUMERATORS: u64 = 2454;
pub const METHOD_ENUMERATOR_BEGIN: u64 = 2455;
pub const METHOD_ENUMERATOR_END: u64 = 2456;
pub const METHOD_GETPROMOTIONTYPE: u64 = 2457;
pub const METHOD_GETINTEGERTYPE: u64 = 2458;
pub const METHOD_GETINTEGERTYPESOURCEINFO: u64 = 2459;
pub const METHOD_GETINTEGERTYPERANGE: u64 = 2460;
pub const METHOD_GETNUMPOSITIVEBITS: u64 = 2461;
pub const METHOD_GETNUMNEGATIVEBITS: u64 = 2462;
pub const METHOD_ISSCOPED: u64 = 2463;
pub const METHOD_ISSCOPEDUSINGCLASSTAG: u64 = 2464;
pub const METHOD_ISFIXED: u64 = 2465;
pub const METHOD_ISCOMPLETE: u64 = 2466;
pub const METHOD_ISCLOSED: u64 = 2467;
pub const METHOD_ISCLOSEDFLAG: u64 = 2468;
pub const METHOD_ISCLOSEDNONFLAG: u64 = 2469;
pub const METHOD_GETTEMPLATEINSTANTIATIONPATTERN_1: u64 = 2470;
pub const METHOD_GETINSTANTIATEDFROMMEMBERENUM: u64 = 2471;
pub const METHOD_GETTEMPLATESPECIALIZATIONKIND_1: u64 = 2472;
pub const METHOD_GETMEMBERSPECIALIZATIONINFO_1: u64 = 2473;
pub const CLASS_POINTERATTR: u64 = 2474;
pub const CLASS_AARCH64VECTORPCSATTR: u64 = 2475;
pub const CLASS_ADJUSTEDTYPELOC: u64 = 2476;
pub const METHOD_GETORIGINALLOC: u64 = 2477;
pub const METHOD_GETINNERTYPE_3: u64 = 2478;
pub const METHOD_GETLOCALSOURCERANGE_3: u64 = 2479;
pub const METHOD_GETLOCALDATASIZE: u64 = 2480;
pub const CLASS_MEMBEREXPR: u64 = 2481;
pub const METHOD_GETBASE_6: u64 = 2482;
pub const METHOD_GETMEMBERDECL: u64 = 2483;
pub const METHOD_GETFOUNDDECL_3: u64 = 2484;
pub const METHOD_HASQUALIFIER_2: u64 = 2485;
pub const METHOD_GETQUALIFIERLOC_14: u64 = 2486;
pub const METHOD_GETQUALIFIER_15: u64 = 2487;
pub const METHOD_GETTEMPLATEKEYWORDLOC_5: u64 = 2488;
pub const METHOD_GETLANGLELOC_3: u64 = 2489;
pub const METHOD_GETRANGLELOC_3: u64 = 2490;
pub const METHOD_HASTEMPLATEKEYWORD_3: u64 = 2491;
pub const METHOD_HASEXPLICITTEMPLATEARGS_4: u64 = 2492;
pub const METHOD_GETTEMPLATEARGS_5: u64 = 2493;
pub const METHOD_GETNUMTEMPLATEARGS_3: u64 = 2494;
pub const METHOD_TEMPLATE_ARGUMENTS_5: u64 = 2495;
pub const METHOD_GETMEMBERNAMEINFO_1: u64 = 2496;
pub const METHOD_GETOPERATORLOC_6: u64 = 2497;
pub const METHOD_ISARROW_4: u64 = 2498;
pub const METHOD_GETMEMBERLOC_2: u64 = 2499;
pub const METHOD_GETBEGINLOC_97: u64 = 2500;
pub const METHOD_GETENDLOC_96: u64 = 2501;
pub const METHOD_GETEXPRLOC_9: u64 = 2502;
pub const METHOD_ISIMPLICITACCESS_2: u64 = 2503;
pub const METHOD_HADMULTIPLECANDIDATES_2: u64 = 2504;
pub const METHOD_ISNONODRUSE_1: u64 = 2505;
pub const METHOD_CHILDREN_85: u64 = 2506;
pub const CLASS_AMDGPUNUMVGPRATTR: u64 = 2507;
pub const CLASS_CXXPARENLISTINITEXPR: u64 = 2508;
pub const METHOD_GETINITEXPRS: u64 = 2509;
pub const METHOD_GETUSERSPECIFIEDINITEXPRS: u64 = 2510;
pub const METHOD_GETBEGINLOC_36: u64 = 2511;
pub const METHOD_GETENDLOC_35: u64 = 2512;
pub const METHOD_GETINITLOC: u64 = 2513;
pub const METHOD_GETSOURCERANGE_42: u64 = 2514;
pub const METHOD_GETARRAYFILLER: u64 = 2515;
pub const METHOD_GETINITIALIZEDFIELDINUNION: u64 = 2516;
pub const METHOD_CHILDREN_27: u64 = 2517;
pub const CLASS_DEDUCEDTYPE: u64 = 2518;
pub const METHOD_ISSUGARED_10: u64 = 2519;
pub const METHOD_DESUGAR_10: u64 = 2520;
pub const METHOD_GETDEDUCEDTYPE: u64 = 2521;
pub const METHOD_ISDEDUCED: u64 = 2522;
pub const CLASS_PARAMTYPESTATEATTR: u64 = 2523;
pub const CLASS_BPFPRESERVESTATICOFFSETATTR: u64 = 2524;
pub const CLASS_OMPCAPTUREDEXPRDECL: u64 = 2525;
pub const CLASS_MIPS16ATTR: u64 = 2526;
pub const CLASS_OBJCATSYNCHRONIZEDSTMT: u64 = 2527;
pub const CLASS_ATOMICEXPR: u64 = 2528;
pub const METHOD_GETPTR: u64 = 2529;
pub const METHOD_GETORDER: u64 = 2530;
pub const METHOD_GETSCOPE: u64 = 2531;
pub const METHOD_GETVAL1: u64 = 2532;
pub const METHOD_GETORDERFAIL: u64 = 2533;
pub const METHOD_GETVAL2: u64 = 2534;
pub const METHOD_GETWEAK: u64 = 2535;
pub const METHOD_GETVALUETYPE_1: u64 = 2536;
pub const METHOD_GETOP: u64 = 2537;
pub const METHOD_GETOPASSTRING: u64 = 2538;
pub const METHOD_GETNUMSUBEXPRS: u64 = 2539;
pub const METHOD_GETSUBEXPRS: u64 = 2540;
pub const METHOD_ISVOLATILE_3: u64 = 2541;
pub const METHOD_ISCMPXCHG: u64 = 2542;
pub const METHOD_ISOPENCL: u64 = 2543;
pub const METHOD_GETBUILTINLOC_1: u64 = 2544;
pub const METHOD_GETRPARENLOC_3: u64 = 2545;
pub const METHOD_GETBEGINLOC_11: u64 = 2546;
pub const METHOD_GETENDLOC_10: u64 = 2547;
pub const METHOD_CHILDREN_7: u64 = 2548;
pub const METHOD_GETSCOPEMODEL: u64 = 2549;
pub const CLASS_DECLORSTMTATTR: u64 = 2550;
pub const CLASS_TARGETATTR: u64 = 2551;
pub const CLASS_AMDGPUNUMSGPRATTR: u64 = 2552;
pub const CLASS_NVPTXKERNELATTR: u64 = 2553;
pub const CLASS_PTR64ATTR: u64 = 2554;
pub const CLASS_OFFSETOFEXPR: u64 = 2555;
pub const METHOD_GETOPERATORLOC_7: u64 = 2556;
pub const METHOD_GETRPARENLOC_19: u64 = 2557;
pub const METHOD_GETTYPESOURCEINFO_7: u64 = 2558;
pub const METHOD_GETNUMCOMPONENTS: u64 = 2559;
pub const METHOD_GETNUMEXPRESSIONS: u64 = 2560;
pub const METHOD_GETBEGINLOC_100: u64 = 2561;
pub const METHOD_GETENDLOC_99: u64 = 2562;
pub const METHOD_CHILDREN_88: u64 = 2563;
pub const CLASS_LABELDECL: u64 = 2564;
pub const METHOD_GETSTMT: u64 = 2565;
pub const METHOD_ISGNULOCAL: u64 = 2566;
pub const METHOD_GETSOURCERANGE_16: u64 = 2567;
pub const METHOD_ISMSASMLABEL: u64 = 2568;
pub const METHOD_ISRESOLVEDMSASMLABEL: u64 = 2569;
pub const METHOD_GETMSASMLABEL: u64 = 2570;
pub const CLASS_VARIABLEARRAYTYPE: u64 = 2571;
pub const METHOD_GETSIZEEXPR_4: u64 = 2572;
pub const METHOD_GETBRACKETSRANGE_1: u64 = 2573;
pub const METHOD_GETLBRACKETLOC_1: u64 = 2574;
pub const METHOD_GETRBRACKETLOC_1: u64 = 2575;
pub const METHOD_ISSUGARED_45: u64 = 2576;
pub const METHOD_DESUGAR_45: u64 = 2577;
pub const CLASS_OPENCLGLOBALHOSTADDRESSSPACEATTR: u64 = 2578;
pub const CLASS_ARMNEWATTR: u64 = 2579;
pub const CLASS_PIPETYPELOC: u64 = 2580;
pub const METHOD_GETVALUELOC: u64 = 2581;
pub const METHOD_GETLOCALSOURCERANGE_4: u64 = 2582;
pub const METHOD_GETKWLOC: u64 = 2583;
pub const METHOD_GETINNERTYPE_4: u64 = 2584;
pub const CLASS_ACQUIRECAPABILITYATTR: u64 = 2585;
pub const CLASS_CAPTUREDSTMT: u64 = 2586;
pub const METHOD_GETCAPTUREDSTMT: u64 = 2587;
pub const METHOD_GETCAPTUREDDECL: u64 = 2588;
pub const METHOD_GETCAPTUREDREGIONKIND: u64 = 2589;
pub const METHOD_GETCAPTUREDRECORDDECL: u64 = 2590;
pub const METHOD_CAPTURES_2: u64 = 2591;
pub const METHOD_CAPTURE_BEGIN_1: u64 = 2592;
pub const METHOD_CAPTURE_END_1: u64 = 2593;
pub const METHOD_CAPTURE_SIZE: u64 = 2594;
pub const METHOD_CAPTURE_INITS: u64 = 2595;
pub const METHOD_CAPTURE_INIT_BEGIN_1: u64 = 2596;
pub const METHOD_CAPTURE_INIT_END_1: u64 = 2597;
pub const METHOD_GETBEGINLOC_49: u64 = 2598;
pub const METHOD_GETENDLOC_48: u64 = 2599;
pub const METHOD_GETSOURCERANGE_47: u64 = 2600;
pub const METHOD_CHILDREN_38: u64 = 2601;
pub const CLASS_ARMOUTATTR: u64 = 2602;
pub const CLASS_ARMINTERRUPTATTR: u64 = 2603;
pub const CLASS_BUILTINBITCASTEXPR: u64 = 2604;
pub const METHOD_GETBEGINLOC_17: u64 = 2605;
pub const METHOD_GETENDLOC_16: u64 = 2606;
pub const CLASS_CONSTRUCTORUSINGSHADOWDECL: u64 = 2607;
pub const METHOD_GETINTRODUCER: u64 = 2608;
pub const METHOD_GETPARENT_1: u64 = 2609;
pub const METHOD_GETNOMINATEDBASECLASSSHADOWDECL: u64 = 2610;
pub const METHOD_GETCONSTRUCTEDBASECLASSSHADOWDECL: u64 = 2611;
pub const METHOD_GETNOMINATEDBASECLASS: u64 = 2612;
pub const METHOD_GETCONSTRUCTEDBASECLASS: u64 = 2613;
pub const METHOD_CONSTRUCTSVIRTUALBASE: u64 = 2614;
pub const CLASS_STMTATTR: u64 = 2615;
pub const CLASS_NOMERGEATTR: u64 = 2616;
pub const CLASS_TAGDECL: u64 = 2617;
pub const METHOD_GETBRACERANGE: u64 = 2618;
pub const METHOD_GETINNERLOCSTART_1: u64 = 2619;
pub const METHOD_GETOUTERLOCSTART_1: u64 = 2620;
pub const METHOD_GETSOURCERANGE_23: u64 = 2621;
pub const METHOD_GETCANONICALDECL_17: u64 = 2622;
pub const METHOD_ISTHISDECLARATIONADEFINITION_3: u64 = 2623;
pub const METHOD_ISCOMPLETEDEFINITION: u64 = 2624;
pub const METHOD_ISCOMPLETEDEFINITIONREQUIRED: u64 = 2625;
pub const METHOD_ISBEINGDEFINED_1: u64 = 2626;
pub const METHOD_ISEMBEDDEDINDECLARATOR: u64 = 2627;
pub const METHOD_ISFREESTANDING: u64 = 2628;
pub const METHOD_MAYHAVEOUTOFDATEDEF: u64 = 2629;
pub const METHOD_ISDEPENDENTTYPE: u64 = 2630;
pub const METHOD_ISTHISDECLARATIONADEMOTEDDEFINITION: u64 = 2631;
pub const METHOD_GETDEFINITION_4: u64 = 2632;
pub const METHOD_GETKINDNAME: u64 = 2633;
pub const METHOD_GETTAGKIND: u64 = 2634;
pub const METHOD_ISSTRUCT: u64 = 2635;
pub const METHOD_ISINTERFACE: u64 = 2636;
pub const METHOD_ISCLASS: u64 = 2637;
pub const METHOD_ISUNION: u64 = 2638;
pub const METHOD_ISENUM: u64 = 2639;
pub const METHOD_HASNAMEFORLINKAGE: u64 = 2640;
pub const METHOD_GETTYPEDEFNAMEFORANONDECL: u64 = 2641;
pub const METHOD_GETQUALIFIER_5: u64 = 2642;
pub const METHOD_GETQUALIFIERLOC_2: u64 = 2643;
pub const METHOD_GETNUMTEMPLATEPARAMETERLISTS_1: u64 = 2644;
pub const CLASS_SWIFTASYNCNAMEATTR: u64 = 2645;
pub const CLASS_SPECULATIVELOADHARDENINGATTR: u64 = 2646;
pub const CLASS_CUDADEVICEATTR: u64 = 2647;
pub const CLASS_PASSOBJECTSIZEATTR: u64 = 2648;
pub const CLASS_HLSLANNOTATIONATTR: u64 = 2649;
pub const CLASS_BLOCKPOINTERTYPE: u64 = 2650;
pub const METHOD_GETPOINTEETYPE: u64 = 2651;
pub const METHOD_ISSUGARED_5: u64 = 2652;
pub const METHOD_DESUGAR_5: u64 = 2653;
pub const CLASS_OPENCLACCESSATTR: u64 = 2654;
pub const CLASS_TYPEATTR: u64 = 2655;
pub const CLASS_OPENCLUNROLLHINTATTR: u64 = 2656;
pub const CLASS_IMPLICITVALUEINITEXPR: u64 = 2657;
pub const METHOD_GETBEGINLOC_85: u64 = 2658;
pub const METHOD_GETENDLOC_84: u64 = 2659;
pub const METHOD_CHILDREN_73: u64 = 2660;
pub const CLASS_ARMBUILTINALIASATTR: u64 = 2661;
pub const CLASS_SELECTANYATTR: u64 = 2662;
pub const CLASS_COLDATTR: u64 = 2663;
pub const CLASS_OMPPARALLELMASTERDIRECTIVE: u64 = 2664;
pub const CLASS_ACQUIREDBEFOREATTR: u64 = 2665;
pub const CLASS_CONSTANTMATRIXTYPE: u64 = 2666;
pub const METHOD_GETNUMROWS: u64 = 2667;
pub const METHOD_GETNUMCOLUMNS: u64 = 2668;
pub const METHOD_GETNUMELEMENTSFLATTENED: u64 = 2669;
pub const CLASS_ALIGNMAC68KATTR: u64 = 2670;
pub const CLASS_BUILTINATTR: u64 = 2671;
pub const CLASS_NOCOMMONATTR: u64 = 2672;
pub const CLASS_PRAGMACLANGDATASECTIONATTR: u64 = 2673;
pub const CLASS_VISIBILITYATTR: u64 = 2674;
pub const CLASS_OMPTARGETSIMDDIRECTIVE: u64 = 2675;
pub const CLASS_CFICANONICALJUMPTABLEATTR: u64 = 2676;
pub const CLASS_CFRETURNSRETAINEDATTR: u64 = 2677;
pub const CLASS_COMPLEXTYPE: u64 = 2678;
pub const METHOD_GETELEMENTTYPE_1: u64 = 2679;
pub const METHOD_ISSUGARED_7: u64 = 2680;
pub const METHOD_DESUGAR_7: u64 = 2681;
pub const CLASS_USEHANDLEATTR: u64 = 2682;
pub const CLASS_CXXREINTERPRETCASTEXPR: u64 = 2683;
pub const CLASS_DEPENDENTSIZEDEXTVECTORTYPELOC: u64 = 2684;
pub const METHOD_GETNAMELOC_3: u64 = 2685;
pub const METHOD_GETLOCALSOURCERANGE_5: u64 = 2686;
pub const METHOD_GETELEMENTLOC_1: u64 = 2687;
pub const METHOD_GETINNERTYPE_5: u64 = 2688;
pub const CLASS_OMPDECLARESIMDDECLATTR: u64 = 2689;
pub const CLASS_OBJCROOTCLASSATTR: u64 = 2690;
pub const CLASS_TARGETCLONESATTR: u64 = 2691;
pub const CLASS_TEMPLATETYPEPARMTYPE: u64 = 2692;
pub const METHOD_GETDEPTH: u64 = 2693;
pub const METHOD_GETINDEX_2: u64 = 2694;
pub const METHOD_ISPARAMETERPACK: u64 = 2695;
pub const METHOD_GETDECL_4: u64 = 2696;
pub const METHOD_GETIDENTIFIER_3: u64 = 2697;
pub const METHOD_ISSUGARED_38: u64 = 2698;
pub const METHOD_DESUGAR_38: u64 = 2699;
pub const CLASS_GCCASMSTMT: u64 = 2700;
pub const METHOD_GETRPARENLOC_16: u64 = 2701;
pub const METHOD_GETASMSTRING_1: u64 = 2702;
pub const METHOD_ISASMGOTO: u64 = 2703;
pub const METHOD_GETNUMLABELS: u64 = 2704;
pub const METHOD_BEGIN_LABELS: u64 = 2705;
pub const METHOD_END_LABELS: u64 = 2706;
pub const METHOD_LABELS: u64 = 2707;
pub const METHOD_GETBEGINLOC_78: u64 = 2708;
pub const METHOD_GETENDLOC_77: u64 = 2709;
pub const CLASS_PRAGMACLANGRELROSECTIONATTR: u64 = 2710;
pub const CLASS_AMDGPUFLATWORKGROUPSIZEATTR: u64 = 2711;
pub const CLASS_RENDERSCRIPTKERNELATTR: u64 = 2712;
pub const CLASS_ASSUMEALIGNEDATTR: u64 = 2713;
pub const CLASS_POINTERTYPELOC: u64 = 2714;
pub const METHOD_GETSTARLOC_1: u64 = 2715;
pub const CLASS_AVAILABLEONLYINDEFAULTEVALMETHODATTR: u64 = 2716;
pub const CLASS_OBJCPROPERTYDECL: u64 = 2717;
pub const CLASS_PRAGMACLANGTEXTSECTIONATTR: u64 = 2718;
pub const CLASS_CODESEGATTR: u64 = 2719;
pub const CLASS_OMPPARALLELFORSIMDDIRECTIVE: u64 = 2720;
pub const CLASS_WEBASSEMBLYFUNCREFATTR: u64 = 2721;
pub const CLASS_CONSUMABLESETONREADATTR: u64 = 2722;
pub const CLASS_CASTEXPR: u64 = 2723;
pub const METHOD_GETCASTKIND: u64 = 2724;
pub const METHOD_GETCASTKINDNAME: u64 = 2725;
pub const METHOD_GETSUBEXPR_4: u64 = 2726;
pub const METHOD_GETSUBEXPRASWRITTEN: u64 = 2727;
pub const METHOD_GETCONVERSIONFUNCTION: u64 = 2728;
pub const METHOD_PATH_EMPTY: u64 = 2729;
pub const METHOD_PATH_SIZE: u64 = 2730;
pub const METHOD_PATH_BEGIN: u64 = 2731;
pub const METHOD_PATH_END: u64 = 2732;
pub const METHOD_PATH: u64 = 2733;
pub const METHOD_GETTARGETUNIONFIELD: u64 = 2734;
pub const METHOD_HASSTOREDFPFEATURES_2: u64 = 2735;
pub const METHOD_GETSTOREDFPFEATURES_2: u64 = 2736;
pub const METHOD_GETFPFEATURES_2: u64 = 2737;
pub const METHOD_CHANGESVOLATILEQUALIFICATION: u64 = 2738;
pub const METHOD_CHILDREN_40: u64 = 2739;
pub const CLASS_USEDATTR: u64 = 2740;
pub const CLASS_MSGUIDDECL: u64 = 2741;
pub const METHOD_GETPARTS: u64 = 2742;
pub const METHOD_GETASAPVALUE: u64 = 2743;
pub const CLASS_SUBSTTEMPLATETYPEPARMPACKTYPE: u64 = 2744;
pub const METHOD_GETIDENTIFIER_2: u64 = 2745;
pub const METHOD_GETASSOCIATEDDECL: u64 = 2746;
pub const METHOD_GETREPLACEDPARAMETER: u64 = 2747;
pub const METHOD_GETINDEX: u64 = 2748;
pub const METHOD_GETFINAL: u64 = 2749;
pub const METHOD_GETNUMARGS: u64 = 2750;
pub const METHOD_ISSUGARED_35: u64 = 2751;
pub const METHOD_DESUGAR_35: u64 = 2752;
pub const METHOD_GETARGUMENTPACK: u64 = 2753;
pub const CLASS_DIAGNOSEASBUILTINATTR: u64 = 2754;
pub const CLASS_COROONLYDESTROYWHENCOMPLETEATTR: u64 = 2755;
pub const CLASS_PRAGMACLANGBSSSECTIONATTR: u64 = 2756;
pub const CLASS_RETAINATTR: u64 = 2757;
pub const CLASS_CALLBACKATTR: u64 = 2758;
pub const CLASS_CUDALAUNCHBOUNDSATTR: u64 = 2759;
pub const CLASS_UUIDATTR: u64 = 2760;
pub const CLASS_PREFERREDTYPEATTR: u64 = 2761;
pub const CLASS_SUBSTNONTYPETEMPLATEPARMPACKEXPR: u64 = 2762;
pub const METHOD_GETASSOCIATEDDECL_3: u64 = 2763;
pub const METHOD_GETINDEX_5: u64 = 2764;
pub const METHOD_GETPARAMETERPACK_1: u64 = 2765;
pub const METHOD_GETPARAMETERPACKLOCATION_1: u64 = 2766;
pub const METHOD_GETARGUMENTPACK_1: u64 = 2767;
pub const METHOD_GETBEGINLOC_121: u64 = 2768;
pub const METHOD_GETENDLOC_120: u64 = 2769;
pub const METHOD_CHILDREN_109: u64 = 2770;
pub const CLASS_CXXPSEUDODESTRUCTOREXPR: u64 = 2771;
pub const METHOD_GETBASE_1: u64 = 2772;
pub const METHOD_HASQUALIFIER: u64 = 2773;
pub const METHOD_GETQUALIFIERLOC_9: u64 = 2774;
pub const METHOD_GETQUALIFIER_12: u64 = 2775;
pub const METHOD_ISARROW_1: u64 = 2776;
pub const METHOD_GETOPERATORLOC_4: u64 = 2777;
pub const METHOD_GETSCOPETYPEINFO: u64 = 2778;
pub const METHOD_GETCOLONCOLONLOC: u64 = 2779;
pub const METHOD_GETTILDELOC: u64 = 2780;
pub const METHOD_GETDESTROYEDTYPEINFO: u64 = 2781;
pub const METHOD_GETDESTROYEDTYPEIDENTIFIER: u64 = 2782;
pub const METHOD_GETDESTROYEDTYPE_1: u64 = 2783;
pub const METHOD_GETDESTROYEDTYPELOC: u64 = 2784;
pub const METHOD_GETBEGINLOC_37: u64 = 2785;
pub const METHOD_GETENDLOC_36: u64 = 2786;
pub const METHOD_CHILDREN_28: u64 = 2787;
pub const CLASS_PTGUARDEDVARATTR: u64 = 2788;
pub const CLASS_IFUNCATTR: u64 = 2789;
pub const CLASS_FUNCTIONNOPROTOTYPE: u64 = 2790;
pub const METHOD_ISSUGARED_21: u64 = 2791;
pub const METHOD_DESUGAR_21: u64 = 2792;
pub const CLASS_OMPMASKEDTASKLOOPDIRECTIVE: u64 = 2793;
pub const CLASS_LVALUEREFERENCETYPELOC: u64 = 2794;
pub const METHOD_GETAMPLOC: u64 = 2795;
pub const CLASS_ASSERTEXCLUSIVELOCKATTR: u64 = 2796;
pub const CLASS_COMMONATTR: u64 = 2797;
pub const CLASS_REQDWORKGROUPSIZEATTR: u64 = 2798;
pub const CLASS_RETURNSTWICEATTR: u64 = 2799;
pub const CLASS_UNRESOLVEDUSINGTYPENAMEDECL: u64 = 2800;
pub const METHOD_GETUSINGLOC: u64 = 2801;
pub const METHOD_GETTYPENAMELOC: u64 = 2802;
pub const METHOD_GETQUALIFIERLOC_3: u64 = 2803;
pub const METHOD_GETQUALIFIER_6: u64 = 2804;
pub const METHOD_GETNAMEINFO_1: u64 = 2805;
pub const METHOD_ISPACKEXPANSION_3: u64 = 2806;
pub const METHOD_GETELLIPSISLOC_2: u64 = 2807;
pub const METHOD_GETCANONICALDECL_21: u64 = 2808;
pub const CLASS_INITPRIORITYATTR: u64 = 2809;
pub const CLASS_RESTRICTATTR: u64 = 2810;
pub const CLASS_DECLTYPETYPELOC: u64 = 2811;
pub const METHOD_GETUNDERLYINGEXPR_2: u64 = 2812;
pub const METHOD_GETDECLTYPELOC: u64 = 2813;
pub const METHOD_GETRPARENLOC_29: u64 = 2814;
pub const METHOD_GETLOCALSOURCERANGE_6: u64 = 2815;
pub const CLASS_OMPTARGETPARALLELFORDIRECTIVE: u64 = 2816;
pub const CLASS_HLSLGROUPSHAREDADDRESSSPACEATTR: u64 = 2817;
pub const CLASS_BPFPRESERVEACCESSINDEXATTR: u64 = 2818;
pub const CLASS_SWIFTASYNCERRORATTR: u64 = 2819;
pub const CLASS_PACKEDATTR: u64 = 2820;
pub const CLASS_ARRAYSUBSCRIPTEXPR: u64 = 2821;
pub const METHOD_GETLHS: u64 = 2822;
pub const METHOD_GETRHS: u64 = 2823;
pub const METHOD_GETBASE: u64 = 2824;
pub const METHOD_GETIDX: u64 = 2825;
pub const METHOD_GETBEGINLOC_7: u64 = 2826;
pub const METHOD_GETENDLOC_6: u64 = 2827;
pub const METHOD_GETRBRACKETLOC_2: u64 = 2828;
pub const METHOD_GETEXPRLOC: u64 = 2829;
pub const METHOD_CHILDREN_3: u64 = 2830;
pub const CLASS_DECLSTMT: u64 = 2831;
pub const METHOD_ISSINGLEDECL: u64 = 2832;
pub const METHOD_GETSINGLEDECL: u64 = 2833;
pub const METHOD_GETDECLGROUP: u64 = 2834;
pub const METHOD_GETENDLOC_63: u64 = 2835;
pub const METHOD_GETBEGINLOC_64: u64 = 2836;
pub const METHOD_CHILDREN_54: u64 = 2837;
pub const METHOD_DECLS: u64 = 2838;
pub const METHOD_DECL_BEGIN: u64 = 2839;
pub const METHOD_DECL_END: u64 = 2840;
pub const CLASS_ALWAYSINLINEATTR: u64 = 2841;
pub const CLASS_GUARDEDBYATTR: u64 = 2842;
pub const CLASS_RETURNSNONNULLATTR: u64 = 2843;
pub const CLASS_ARCWEAKREFUNAVAILABLEATTR: u64 = 2844;
pub const CLASS_SYCLKERNELATTR: u64 = 2845;
pub const CLASS_TYPENULLABLERESULTATTR: u64 = 2846;
pub const CLASS_CUDADEVICEBUILTINSURFACETYPEATTR: u64 = 2847;
pub const CLASS_MIPSINTERRUPTATTR: u64 = 2848;
pub const CLASS_OWNERSHIPATTR: u64 = 2849;
pub const CLASS_UNARYTRANSFORMTYPE: u64 = 2850;
pub const METHOD_ISSUGARED_42: u64 = 2851;
pub const METHOD_DESUGAR_42: u64 = 2852;
pub const METHOD_GETUNDERLYINGTYPE_2: u64 = 2853;
pub const METHOD_GETBASETYPE: u64 = 2854;
pub const METHOD_GETUTTKIND: u64 = 2855;
pub const CLASS_NSRETURNSNOTRETAINEDATTR: u64 = 2856;
pub const CLASS_RELEASECAPABILITYATTR: u64 = 2857;
pub const CLASS_CHOOSEEXPR: u64 = 2858;
pub const METHOD_ISCONDITIONTRUE: u64 = 2859;
pub const METHOD_ISCONDITIONDEPENDENT: u64 = 2860;
pub const METHOD_GETCHOSENSUBEXPR: u64 = 2861;
pub const METHOD_GETCOND_3: u64 = 2862;
pub const METHOD_GETLHS_5: u64 = 2863;
pub const METHOD_GETRHS_5: u64 = 2864;
pub const METHOD_GETBUILTINLOC_2: u64 = 2865;
pub const METHOD_GETRPARENLOC_12: u64 = 2866;
pub const METHOD_GETBEGINLOC_52: u64 = 2867;
pub const METHOD_GETENDLOC_51: u64 = 2868;
pub const METHOD_CHILDREN_42: u64 = 2869;
pub const CLASS_OPENCLINTELREQDSUBGROUPSIZEATTR: u64 = 2870;
pub const CLASS_PUREATTR: u64 = 2871;
pub const CLASS_INHERITABLEATTR: u64 = 2872;
pub const CLASS_OPTIMIZENONEATTR: u64 = 2873;
pub const CLASS_LTOVISIBILITYPUBLICATTR: u64 = 2874;
pub const CLASS_OBJCSUBCLASSINGRESTRICTEDATTR: u64 = 2875;
pub const CLASS_OBJCTYPEPARAMTYPE: u64 = 2876;
pub const CLASS_FINALATTR: u64 = 2877;
pub const CLASS_DOSTMT: u64 = 2878;
pub const METHOD_GETCOND_5: u64 = 2879;
pub const METHOD_GETBODY_7: u64 = 2880;
pub const METHOD_GETDOLOC: u64 = 2881;
pub const METHOD_GETWHILELOC: u64 = 2882;
pub const METHOD_GETRPARENLOC_14: u64 = 2883;
pub const METHOD_GETBEGINLOC_70: u64 = 2884;
pub const METHOD_GETENDLOC_69: u64 = 2885;
pub const METHOD_CHILDREN_60: u64 = 2886;
pub const CLASS_OBJCRETURNSINNERPOINTERATTR: u64 = 2887;
pub const CLASS_CPUDISPATCHATTR: u64 = 2888;
pub const CLASS_PTGUARDEDBYATTR: u64 = 2889;
pub const CLASS_OBJCREQUIRESSUPERATTR: u64 = 2890;
pub const CLASS_OMPCRITICALDIRECTIVE: u64 = 2891;
pub const CLASS_CXXTEMPORARYOBJECTEXPR: u64 = 2892;
pub const METHOD_GETTYPESOURCEINFO_3: u64 = 2893;
pub const METHOD_GETBEGINLOC_41: u64 = 2894;
pub const METHOD_GETENDLOC_40: u64 = 2895;
pub const CLASS_CXXDEDUCTIONGUIDEDECL: u64 = 2896;
pub const METHOD_GETEXPLICITSPECIFIER_2: u64 = 2897;
pub const METHOD_ISEXPLICIT_2: u64 = 2898;
pub const METHOD_GETDEDUCEDTEMPLATE: u64 = 2899;
pub const METHOD_GETCORRESPONDINGCONSTRUCTOR: u64 = 2900;
pub const METHOD_GETDEDUCTIONCANDIDATEKIND: u64 = 2901;
pub const CLASS_CFUNKNOWNTRANSFERATTR: u64 = 2902;
pub const CLASS_PRESERVEMOSTATTR: u64 = 2903;
pub const CLASS_OBJCREQUIRESPROPERTYDEFSATTR: u64 = 2904;
pub const CLASS_NODEBUGATTR: u64 = 2905;
pub const CLASS_PTR32ATTR: u64 = 2906;
pub const CLASS_HLSLRESOURCEATTR: u64 = 2907;
pub const CLASS_ARMLOCALLYSTREAMINGATTR: u64 = 2908;
pub const CLASS_TYPEWITHKEYWORD: u64 = 2909;
pub const METHOD_GETKEYWORD_1: u64 = 2910;
pub const CLASS_OBJCOWNERSHIPATTR: u64 = 2911;
pub const CLASS_VECTORTYPE: u64 = 2912;
pub const METHOD_GETELEMENTTYPE_6: u64 = 2913;
pub const METHOD_GETNUMELEMENTS: u64 = 2914;
pub const METHOD_ISSUGARED_46: u64 = 2915;
pub const METHOD_DESUGAR_46: u64 = 2916;
pub const METHOD_GETVECTORKIND_1: u64 = 2917;
pub const CLASS_MINVECTORWIDTHATTR: u64 = 2918;
pub const CLASS_ENUMTYPELOC: u64 = 2919;
pub const METHOD_GETDECL_9: u64 = 2920;
pub const CLASS_OPENCLPRIVATEADDRESSSPACEATTR: u64 = 2921;
pub const CLASS_EXCLUSIVETRYLOCKFUNCTIONATTR: u64 = 2922;
pub const CLASS_OMPTEAMSDISTRIBUTEPARALLELFORDIRECTIVE: u64 = 2923;
pub const CLASS_NOBUILTINATTR: u64 = 2924;
pub const CLASS_SWIFTPRIVATEATTR: u64 = 2925;
pub const CLASS_CXXTYPEIDEXPR: u64 = 2926;
pub const METHOD_ISPOTENTIALLYEVALUATED: u64 = 2927;
pub const METHOD_ISTYPEOPERAND: u64 = 2928;
pub const METHOD_GETTYPEOPERANDSOURCEINFO: u64 = 2929;
pub const METHOD_GETEXPROPERAND: u64 = 2930;
pub const METHOD_GETBEGINLOC_45: u64 = 2931;
pub const METHOD_GETENDLOC_44: u64 = 2932;
pub const METHOD_GETSOURCERANGE_45: u64 = 2933;
pub const METHOD_CHILDREN_34: u64 = 2934;
pub const CLASS_OBJCBRIDGERELATEDATTR: u64 = 2935;
pub const CLASS_OBJCEXTERNALLYRETAINEDATTR: u64 = 2936;
pub const CLASS_MIPSLONGCALLATTR: u64 = 2937;
pub const CLASS_OBJCBRIDGEMUTABLEATTR: u64 = 2938;
pub const CLASS_ARMINOUTATTR: u64 = 2939;
pub const CLASS_OMPPARALLELDIRECTIVE: u64 = 2940;
pub const CLASS_COMPOUNDASSIGNOPERATOR: u64 = 2941;
pub const METHOD_GETCOMPUTATIONLHSTYPE: u64 = 2942;
pub const METHOD_GETCOMPUTATIONRESULTTYPE: u64 = 2943;
pub const CLASS_OSRETURNSRETAINEDONZEROATTR: u64 = 2944;
pub const CLASS_OBJCINDIRECTCOPYRESTOREEXPR: u64 = 2945;
pub const CLASS_OSRETURNSRETAINEDONNONZEROATTR: u64 = 2946;
pub const CLASS_SPTRATTR: u64 = 2947;
pub const CLASS_PRESERVEALLATTR: u64 = 2948;
pub const CLASS_OBJCSUBSCRIPTREFEXPR: u64 = 2949;
pub const CLASS_OMPDECLAREVARIANTATTR: u64 = 2950;
pub const CLASS_OMPALLOCATEDECLATTR: u64 = 2951;
pub const CLASS_MICROMIPSATTR: u64 = 2952;
pub const CLASS_GENERICSELECTIONEXPR: u64 = 2953;
pub const METHOD_GETNUMASSOCS: u64 = 2954;
pub const METHOD_GETRESULTINDEX: u64 = 2955;
pub const METHOD_ISRESULTDEPENDENT: u64 = 2956;
pub const METHOD_ISEXPRPREDICATE: u64 = 2957;
pub const METHOD_ISTYPEPREDICATE: u64 = 2958;
pub const METHOD_GETCONTROLLINGEXPR: u64 = 2959;
pub const METHOD_GETCONTROLLINGTYPE: u64 = 2960;
pub const METHOD_GETRESULTEXPR: u64 = 2961;
pub const METHOD_GETASSOCEXPRS: u64 = 2962;
pub const METHOD_GETASSOCTYPESOURCEINFOS: u64 = 2963;
pub const METHOD_ASSOCIATIONS: u64 = 2964;
pub const METHOD_GETGENERICLOC: u64 = 2965;
pub const METHOD_GETDEFAULTLOC_2: u64 = 2966;
pub const METHOD_GETRPARENLOC_17: u64 = 2967;
pub const METHOD_GETBEGINLOC_80: u64 = 2968;
pub const METHOD_GETENDLOC_79: u64 = 2969;
pub const METHOD_CHILDREN_69: u64 = 2970;
pub const CLASS_HIPMANAGEDATTR: u64 = 2971;
pub const CLASS_ZEROCALLUSEDREGSATTR: u64 = 2972;
pub const CLASS_OBJCEXCEPTIONATTR: u64 = 2973;
pub const CLASS_CLASSTEMPLATEPARTIALSPECIALIZATIONDECL: u64 = 2974;
pub const METHOD_GETTEMPLATEPARAMETERS: u64 = 2975;
pub const METHOD_HASASSOCIATEDCONSTRAINTS: u64 = 2976;
pub const METHOD_GETTEMPLATEARGSASWRITTEN: u64 = 2977;
pub const METHOD_GETINSTANTIATEDFROMMEMBER: u64 = 2978;
pub const METHOD_GETINSTANTIATEDFROMMEMBERTEMPLATE_1: u64 = 2979;
pub const METHOD_GETINJECTEDSPECIALIZATIONTYPE_1: u64 = 2980;
pub const CLASS_NOTTAILCALLEDATTR: u64 = 2981;
pub const CLASS_ANNOTATEATTR: u64 = 2982;
pub const CLASS_MAYBEUNDEFATTR: u64 = 2983;
pub const CLASS_ARRAYTYPELOC: u64 = 2984;
pub const METHOD_GETLBRACKETLOC_2: u64 = 2985;
pub const METHOD_GETRBRACKETLOC_5: u64 = 2986;
pub const METHOD_GETBRACKETSRANGE_2: u64 = 2987;
pub const METHOD_GETSIZEEXPR_5: u64 = 2988;
pub const METHOD_GETELEMENTLOC_2: u64 = 2989;
pub const METHOD_GETLOCALSOURCERANGE_7: u64 = 2990;
pub const METHOD_GETINNERTYPE_6: u64 = 2991;
pub const CLASS_OMPDISTRIBUTEDIRECTIVE: u64 = 2992;
pub const CLASS_CPUSPECIFICATTR: u64 = 2993;
pub const CLASS_FRIENDDECL: u64 = 2994;
pub const METHOD_GETFRIENDTYPE: u64 = 2995;
pub const METHOD_GETFRIENDTYPENUMTEMPLATEPARAMETERLISTS: u64 = 2996;
pub const METHOD_GETFRIENDDECL: u64 = 2997;
pub const METHOD_GETFRIENDLOC: u64 = 2998;
pub const METHOD_GETSOURCERANGE_12: u64 = 2999;
pub const METHOD_ISUNSUPPORTEDFRIEND: u64 = 3000;
pub const CLASS_TARGETVERSIONATTR: u64 = 3001;
pub const CLASS_NOUWTABLEATTR: u64 = 3002;
pub const CLASS_NOUNIQUEADDRESSATTR: u64 = 3003;
pub const CLASS_NOTHREADSAFETYANALYSISATTR: u64 = 3004;
pub const CLASS_MSNOVTABLEATTR: u64 = 3005;
pub const CLASS_CXXNOEXCEPTEXPR: u64 = 3006;
pub const METHOD_GETOPERAND: u64 = 3007;
pub const METHOD_GETBEGINLOC_33: u64 = 3008;
pub const METHOD_GETENDLOC_32: u64 = 3009;
pub const METHOD_GETSOURCERANGE_40: u64 = 3010;
pub const METHOD_GETVALUE_5: u64 = 3011;
pub const METHOD_CHILDREN_25: u64 = 3012;
pub const CLASS_OMPPARALLELMASKEDDIRECTIVE: u64 = 3013;
pub const CLASS_NSCONSUMESSELFATTR: u64 = 3014;
pub const CLASS_INTELOCLBICCATTR: u64 = 3015;
pub const CLASS_NOSPECULATIVELOADHARDENINGATTR: u64 = 3016;
pub const CLASS_NOSPLITSTACKATTR: u64 = 3017;
pub const CLASS_BITINTTYPE: u64 = 3018;
pub const METHOD_ISUNSIGNED: u64 = 3019;
pub const METHOD_ISSIGNED: u64 = 3020;
pub const METHOD_GETNUMBITS: u64 = 3021;
pub const METHOD_ISSUGARED_4: u64 = 3022;
pub const METHOD_DESUGAR_4: u64 = 3023;
pub const CLASS_SOURCELOCEXPR: u64 = 3024;
pub const METHOD_GETBUILTINSTR: u64 = 3025;
pub const METHOD_GETIDENTKIND_1: u64 = 3026;
pub const METHOD_ISINTTYPE: u64 = 3027;
pub const METHOD_GETPARENTCONTEXT: u64 = 3028;
pub const METHOD_GETLOCATION_15: u64 = 3029;
pub const METHOD_GETBEGINLOC_117: u64 = 3030;
pub const METHOD_GETENDLOC_116: u64 = 3031;
pub const METHOD_CHILDREN_104: u64 = 3032;
pub const CLASS_SWIFTNEWTYPEATTR: u64 = 3033;
pub const CLASS_AVAILABILITYATTR: u64 = 3034;
pub const CLASS_OBJCDIRECTATTR: u64 = 3035;
pub const CLASS_IBOUTLETATTR: u64 = 3036;
pub const CLASS_CORETURNSTMT: u64 = 3037;
pub const METHOD_GETKEYWORDLOC: u64 = 3038;
pub const METHOD_GETOPERAND_1: u64 = 3039;
pub const METHOD_GETPROMISECALL: u64 = 3040;
pub const METHOD_ISIMPLICIT_3: u64 = 3041;
pub const METHOD_GETBEGINLOC_60: u64 = 3042;
pub const METHOD_GETENDLOC_59: u64 = 3043;
pub const METHOD_CHILDREN_50: u64 = 3044;
pub const CLASS_ANYX86NOCALLERSAVEDREGISTERSATTR: u64 = 3045;
pub const CLASS_CODEMODELATTR: u64 = 3046;
pub const CLASS_DLLIMPORTATTR: u64 = 3047;
pub const CLASS_PRAGMADETECTMISMATCHDECL: u64 = 3048;
pub const METHOD_GETNAME: u64 = 3049;
pub const METHOD_GETVALUE_1: u64 = 3050;
pub const CLASS_MODEATTR: u64 = 3051;
pub const CLASS_FLATTENATTR: u64 = 3052;
pub const CLASS_NOSANITIZEATTR: u64 = 3053;
pub const CLASS_CXXNAMEDCASTEXPR: u64 = 3054;
pub const METHOD_GETCASTNAME: u64 = 3055;
pub const METHOD_GETOPERATORLOC_2: u64 = 3056;
pub const METHOD_GETRPARENLOC_8: u64 = 3057;
pub const METHOD_GETBEGINLOC_31: u64 = 3058;
pub const METHOD_GETENDLOC_30: u64 = 3059;
pub const METHOD_GETANGLEBRACKETS: u64 = 3060;
pub const CLASS_NORANDOMIZELAYOUTATTR: u64 = 3061;
pub const CLASS_USINGDECL: u64 = 3062;
pub const METHOD_GETUSINGLOC_2: u64 = 3063;
pub const METHOD_GETQUALIFIERLOC_5: u64 = 3064;
pub const METHOD_GETQUALIFIER_8: u64 = 3065;
pub const METHOD_GETNAMEINFO_3: u64 = 3066;
pub const METHOD_ISACCESSDECLARATION_1: u64 = 3067;
pub const METHOD_HASTYPENAME: u64 = 3068;
pub const METHOD_GETSOURCERANGE_32: u64 = 3069;
pub const METHOD_GETCANONICALDECL_23: u64 = 3070;
pub const CLASS_SYSVABIATTR: u64 = 3071;
pub const CLASS_NOMIPS16ATTR: u64 = 3072;
pub const CLASS_NODESTROYATTR: u64 = 3073;
pub const CLASS_SWIFTOBJCMEMBERSATTR: u64 = 3074;
pub const CLASS_NOALIASATTR: u64 = 3075;
pub const CLASS_BUILTINTYPE: u64 = 3076;
pub const METHOD_GETKIND: u64 = 3077;
pub const METHOD_ISSUGARED_6: u64 = 3078;
pub const METHOD_DESUGAR_6: u64 = 3079;
pub const METHOD_ISINTEGER: u64 = 3080;
pub const METHOD_ISSIGNEDINTEGER: u64 = 3081;
pub const METHOD_ISUNSIGNEDINTEGER: u64 = 3082;
pub const METHOD_ISFLOATINGPOINT: u64 = 3083;
pub const METHOD_ISSVEBOOL: u64 = 3084;
pub const METHOD_ISSVECOUNT: u64 = 3085;
pub const METHOD_ISPLACEHOLDERTYPE: u64 = 3086;
pub const METHOD_ISNONOVERLOADPLACEHOLDERTYPE: u64 = 3087;
pub const CLASS_PCSATTR: u64 = 3088;
pub const CLASS_AMDGPUKERNELCALLATTR: u64 = 3089;
pub const CLASS_NSRETURNSRETAINEDATTR: u64 = 3090;
pub const CLASS_ANYX86NOCFCHECKATTR: u64 = 3091;
pub const CLASS_LOCKRETURNEDATTR: u64 = 3092;
pub const CLASS_AVRINTERRUPTATTR: u64 = 3093;
pub const CLASS_HLSLSHADERATTR: u64 = 3094;
pub const CLASS_STDCALLATTR: u64 = 3095;
pub const CLASS_OBJCRUNTIMENAMEATTR: u64 = 3096;
pub const CLASS_ARRAYTYPE: u64 = 3097;
pub const METHOD_GETELEMENTTYPE: u64 = 3098;
pub const METHOD_GETSIZEMODIFIER: u64 = 3099;
pub const METHOD_GETINDEXTYPEQUALIFIERS: u64 = 3100;
pub const METHOD_GETINDEXTYPECVRQUALIFIERS: u64 = 3101;
pub const CLASS_DLLEXPORTATTR: u64 = 3102;
pub const CLASS_WEAKATTR: u64 = 3103;
pub const CLASS_CONSTINITATTR: u64 = 3104;
pub const CLASS_PARENTYPE: u64 = 3105;
pub const METHOD_GETINNERTYPE: u64 = 3106;
pub const METHOD_ISSUGARED_30: u64 = 3107;
pub const METHOD_DESUGAR_30: u64 = 3108;
pub const CLASS_COMPLEXTYPELOC: u64 = 3109;
pub const CLASS_MINSIZEATTR: u64 = 3110;
pub const CLASS_MAYALIASATTR: u64 = 3111;
pub const CLASS_MIPSSHORTCALLATTR: u64 = 3112;
pub const CLASS_NOPROFILEFUNCTIONATTR: u64 = 3113;
pub const CLASS_CMSENSENTRYATTR: u64 = 3114;
pub const CLASS_DESTRUCTORATTR: u64 = 3115;
pub const CLASS_MSVTORDISPATTR: u64 = 3116;
pub const CLASS_MSSTRUCTATTR: u64 = 3117;
pub const CLASS_OMPSINGLEDIRECTIVE: u64 = 3118;
pub const CLASS_SIZEOFPACKEXPR: u64 = 3119;
pub const METHOD_GETOPERATORLOC_8: u64 = 3120;
pub const METHOD_GETPACKLOC: u64 = 3121;
pub const METHOD_GETRPARENLOC_23: u64 = 3122;
pub const METHOD_GETPACK: u64 = 3123;
pub const METHOD_GETPACKLENGTH: u64 = 3124;
pub const METHOD_ISPARTIALLYSUBSTITUTED: u64 = 3125;
pub const METHOD_GETPARTIALARGUMENTS: u64 = 3126;
pub const METHOD_GETBEGINLOC_116: u64 = 3127;
pub const METHOD_GETENDLOC_115: u64 = 3128;
pub const METHOD_CHILDREN_103: u64 = 3129;
pub const CLASS_DECL: u64 = 3130;
pub const METHOD_GETSOURCERANGE_5: u64 = 3131;
pub const METHOD_GETBEGINLOC: u64 = 3132;
pub const METHOD_GETENDLOC: u64 = 3133;
pub const METHOD_GETLOCATION: u64 = 3134;
pub const METHOD_GETKIND_6: u64 = 3135;
pub const METHOD_GETDECLKINDNAME: u64 = 3136;
pub const METHOD_GETNEXTDECLINCONTEXT: u64 = 3137;
pub const METHOD_GETDECLCONTEXT: u64 = 3138;
pub const METHOD_GETNONTRANSPARENTDECLCONTEXT: u64 = 3139;
pub const METHOD_GETNONCLOSURECONTEXT: u64 = 3140;
pub const METHOD_GETTRANSLATIONUNITDECL: u64 = 3141;
pub const METHOD_ISINANONYMOUSNAMESPACE: u64 = 3142;
pub const METHOD_ISINSTDNAMESPACE: u64 = 3143;
pub const METHOD_ISFILECONTEXTDECL: u64 = 3144;
pub const METHOD_GETASTCONTEXT_1: u64 = 3145;
pub const METHOD_GETLANGOPTS: u64 = 3146;
pub const METHOD_GETACCESS: u64 = 3147;
pub const METHOD_GETACCESSUNSAFE: u64 = 3148;
pub const METHOD_HASATTRS: u64 = 3149;
pub const METHOD_GETATTRS: u64 = 3150;
pub const METHOD_ATTRS: u64 = 3151;
pub const METHOD_ATTR_BEGIN: u64 = 3152;
pub const METHOD_ATTR_END: u64 = 3153;
pub const METHOD_GETMAXALIGNMENT: u64 = 3154;
pub const METHOD_ISINVALIDDECL: u64 = 3155;
pub const METHOD_ISIMPLICIT: u64 = 3156;
pub const METHOD_ISREFERENCED: u64 = 3157;
pub const METHOD_ISTHISDECLARATIONREFERENCED: u64 = 3158;
pub const METHOD_ISTOPLEVELDECLINOBJCCONTAINER: u64 = 3159;
pub const METHOD_GETEXTERNALSOURCESYMBOLATTR: u64 = 3160;
pub const METHOD_ISMODULEPRIVATE: u64 = 3161;
pub const METHOD_ISINEXPORTDECLCONTEXT: u64 = 3162;
pub const METHOD_ISINVISIBLEOUTSIDETHEOWNINGMODULE: u64 = 3163;
pub const METHOD_ISINANOTHERMODULEUNIT: u64 = 3164;
pub const METHOD_ISDISCARDEDINGLOBALMODULEFRAGMENT: u64 = 3165;
pub const METHOD_SHOULDSKIPCHECKINGODR: u64 = 3166;
pub const METHOD_HASDEFININGATTR: u64 = 3167;
pub const METHOD_GETDEFININGATTR: u64 = 3168;
pub const METHOD_GETVERSIONINTRODUCED: u64 = 3169;
pub const METHOD_ISWEAKIMPORTED: u64 = 3170;
pub const METHOD_ISFROMASTFILE: u64 = 3171;
pub const METHOD_GETGLOBALID: u64 = 3172;
pub const METHOD_GETOWNINGMODULEID: u64 = 3173;
pub const METHOD_GETIMPORTEDOWNINGMODULE: u64 = 3174;
pub const METHOD_GETLOCALOWNINGMODULE: u64 = 3175;
pub const METHOD_HASOWNINGMODULE: u64 = 3176;
pub const METHOD_GETOWNINGMODULE: u64 = 3177;
pub const METHOD_ISUNCONDITIONALLYVISIBLE: u64 = 3178;
pub const METHOD_ISREACHABLE: u64 = 3179;
pub const METHOD_GETMODULEOWNERSHIPKIND: u64 = 3180;
pub const METHOD_GETIDENTIFIERNAMESPACE: u64 = 3181;
pub const METHOD_HASTAGIDENTIFIERNAMESPACE: u64 = 3182;
pub const METHOD_GETLEXICALDECLCONTEXT: u64 = 3183;
pub const METHOD_ISOUTOFLINE: u64 = 3184;
pub const METHOD_ISTEMPLATED: u64 = 3185;
pub const METHOD_GETTEMPLATEDEPTH: u64 = 3186;
pub const METHOD_ISDEFINEDOUTSIDEFUNCTIONORMETHOD: u64 = 3187;
pub const METHOD_ISINLOCALSCOPEFORINSTANTIATION: u64 = 3188;
pub const METHOD_GETCANONICALDECL_7: u64 = 3189;
pub const METHOD_ISCANONICALDECL: u64 = 3190;
pub const METHOD_REDECLS: u64 = 3191;
pub const METHOD_REDECLS_BEGIN: u64 = 3192;
pub const METHOD_REDECLS_END: u64 = 3193;
pub const METHOD_GETPREVIOUSDECL_2: u64 = 3194;
pub const METHOD_ISFIRSTDECL: u64 = 3195;
pub const METHOD_GETMOSTRECENTDECL_3: u64 = 3196;
pub const METHOD_GETBODY_2: u64 = 3197;
pub const METHOD_HASBODY: u64 = 3198;
pub const METHOD_GETBODYRBRACE: u64 = 3199;
pub const METHOD_ISTEMPLATEPARAMETER: u64 = 3200;
pub const METHOD_ISTEMPLATEPARAMETERPACK: u64 = 3201;
pub const METHOD_ISPARAMETERPACK_1: u64 = 3202;
pub const METHOD_ISTEMPLATEDECL: u64 = 3203;
pub const METHOD_ISFUNCTIONORFUNCTIONTEMPLATE: u64 = 3204;
pub const METHOD_GETDESCRIBEDTEMPLATE: u64 = 3205;
pub const METHOD_GETDESCRIBEDTEMPLATEPARAMS: u64 = 3206;
pub const METHOD_GETASFUNCTION: u64 = 3207;
pub const METHOD_ISLOCALEXTERNDECL: u64 = 3208;
pub const METHOD_GETFRIENDOBJECTKIND: u64 = 3209;
pub const METHOD_GETID: u64 = 3210;
pub const METHOD_ISFUNCTIONPOINTERTYPE: u64 = 3211;
pub const CLASS_OBJCTYPEPARAMTYPELOC: u64 = 3212;
pub const CLASS_ANNOTATETYPEATTR: u64 = 3213;
pub const CLASS_DESIGNATEDINITEXPR: u64 = 3214;
pub const METHOD_SIZE_1: u64 = 3215;
pub const METHOD_DESIGNATORS: u64 = 3216;
pub const METHOD_GETEQUALORCOLONLOC: u64 = 3217;
pub const METHOD_ISDIRECTINIT_1: u64 = 3218;
pub const METHOD_USESGNUSYNTAX: u64 = 3219;
pub const METHOD_GETINIT_3: u64 = 3220;
pub const METHOD_GETNUMSUBEXPRS_1: u64 = 3221;
pub const METHOD_GETDESIGNATORSSOURCERANGE: u64 = 3222;
pub const METHOD_GETBEGINLOC_68: u64 = 3223;
pub const METHOD_GETENDLOC_67: u64 = 3224;
pub const METHOD_CHILDREN_58: u64 = 3225;
pub const CLASS_HLSLNUMTHREADSATTR: u64 = 3226;
pub const CLASS_TRIVIALABIATTR: u64 = 3227;
pub const CLASS_MAXFIELDALIGNMENTATTR: u64 = 3228;
pub const CLASS_MSP430INTERRUPTATTR: u64 = 3229;
pub const CLASS_OBJCATCATCHSTMT: u64 = 3230;
pub const CLASS_INHERITABLEPARAMATTR: u64 = 3231;
pub const CLASS_RETURNTYPESTATEATTR: u64 = 3232;
pub const CLASS_OMPTEAMSGENERICLOOPDIRECTIVE: u64 = 3233;
pub const CLASS_CONSTATTR: u64 = 3234;
pub const CLASS_MSINHERITANCEATTR: u64 = 3235;
pub const CLASS_OBJCTYPEPARAMDECL: u64 = 3236;
pub const CLASS_MSCONSTEXPRATTR: u64 = 3237;
pub const CLASS_SWIFTBRIDGEDTYPEDEFATTR: u64 = 3238;
pub const CLASS_HOTATTR: u64 = 3239;
pub const CLASS_OMPTHREADPRIVATEDECLATTR: u64 = 3240;
pub const CLASS_CXXINHERITEDCTORINITEXPR: u64 = 3241;
pub const METHOD_GETCONSTRUCTOR_1: u64 = 3242;
pub const METHOD_CONSTRUCTSVBASE: u64 = 3243;
pub const METHOD_GETCONSTRUCTIONKIND_1: u64 = 3244;
pub const METHOD_INHERITEDFROMVBASE: u64 = 3245;
pub const METHOD_GETLOCATION_3: u64 = 3246;
pub const METHOD_GETBEGINLOC_30: u64 = 3247;
pub const METHOD_GETENDLOC_29: u64 = 3248;
pub const METHOD_CHILDREN_23: u64 = 3249;
pub const CLASS_MSALLOCATORATTR: u64 = 3250;
pub const CLASS_ENUMEXTENSIBILITYATTR: u64 = 3251;
pub const CLASS_SUBSTNONTYPETEMPLATEPARMEXPR: u64 = 3252;
pub const METHOD_GETNAMELOC_1: u64 = 3253;
pub const METHOD_GETBEGINLOC_120: u64 = 3254;
pub const METHOD_GETENDLOC_119: u64 = 3255;
pub const METHOD_GETREPLACEMENT: u64 = 3256;
pub const METHOD_GETASSOCIATEDDECL_2: u64 = 3257;
pub const METHOD_GETINDEX_4: u64 = 3258;
pub const METHOD_GETPACKINDEX_1: u64 = 3259;
pub const METHOD_GETPARAMETER: u64 = 3260;
pub const METHOD_ISREFERENCEPARAMETER: u64 = 3261;
pub const METHOD_CHILDREN_108: u64 = 3262;
pub const CLASS_CUDACONSTANTATTR: u64 = 3263;
pub const CLASS_NOSTACKPROTECTORATTR: u64 = 3264;
pub const CLASS_DIAGNOSEIFATTR: u64 = 3265;
pub const CLASS_OBJCIVARDECL: u64 = 3266;
pub const CLASS_OMPCAPTUREKINDATTR: u64 = 3267;
pub const CLASS_REFERENCETYPE: u64 = 3268;
pub const METHOD_ISSPELLEDASLVALUE: u64 = 3269;
pub const METHOD_ISINNERREF: u64 = 3270;
pub const METHOD_GETPOINTEETYPEASWRITTEN: u64 = 3271;
pub const METHOD_GETPOINTEETYPE_5: u64 = 3272;
pub const CLASS_TYPETAGFORDATATYPEATTR: u64 = 3273;
pub const CLASS_MSABIATTR: u64 = 3274;
pub const CLASS_READONLYPLACEMENTATTR: u64 = 3275;
pub const CLASS_COUNTEDBYATTR: u64 = 3276;
pub const CLASS_REDECLARABLETEMPLATEDECL: u64 = 3277;
pub const METHOD_GETCANONICALDECL_16: u64 = 3278;
pub const METHOD_ISMEMBERSPECIALIZATION: u64 = 3279;
pub const METHOD_GETINSTANTIATEDFROMMEMBERTEMPLATE_3: u64 = 3280;
pub const CLASS_PARENTYPELOC: u64 = 3281;
pub const METHOD_GETLPARENLOC_12: u64 = 3282;
pub const METHOD_GETRPARENLOC_30: u64 = 3283;
pub const METHOD_GETLOCALSOURCERANGE_8: u64 = 3284;
pub const METHOD_GETINNERLOC: u64 = 3285;
pub const METHOD_GETINNERTYPE_7: u64 = 3286;
pub const CLASS_MIGSERVERROUTINEATTR: u64 = 3287;
pub const CLASS_DEPENDENTDECLTYPETYPE: u64 = 3288;
pub const CLASS_RANDOMIZELAYOUTATTR: u64 = 3289;
pub const CLASS_COROWRAPPERATTR: u64 = 3290;
pub const CLASS_CORORETURNTYPEATTR: u64 = 3291;
pub const CLASS_M68KRTDATTR: u64 = 3292;
pub const CLASS_OMPDECLARETARGETDECLATTR: u64 = 3293;
pub const CLASS_MSPROPERTYREFEXPR: u64 = 3294;
pub const METHOD_GETSOURCERANGE_48: u64 = 3295;
pub const METHOD_ISIMPLICITACCESS_1: u64 = 3296;
pub const METHOD_GETBEGINLOC_93: u64 = 3297;
pub const METHOD_GETENDLOC_92: u64 = 3298;
pub const METHOD_CHILDREN_81: u64 = 3299;
pub const METHOD_GETBASEEXPR: u64 = 3300;
pub const METHOD_GETPROPERTYDECL: u64 = 3301;
pub const METHOD_ISARROW_3: u64 = 3302;
pub const METHOD_GETMEMBERLOC_1: u64 = 3303;
pub const METHOD_GETQUALIFIERLOC_13: u64 = 3304;
pub const CLASS_LIFETIMEBOUNDATTR: u64 = 3305;
pub const CLASS_DECLARATORDECL: u64 = 3306;
pub const METHOD_GETTYPESOURCEINFO: u64 = 3307;
pub const METHOD_GETINNERLOCSTART: u64 = 3308;
pub const METHOD_GETOUTERLOCSTART: u64 = 3309;
pub const METHOD_GETSOURCERANGE_6: u64 = 3310;
pub const METHOD_GETBEGINLOC_1: u64 = 3311;
pub const METHOD_GETQUALIFIER_3: u64 = 3312;
pub const METHOD_GETQUALIFIERLOC: u64 = 3313;
pub const METHOD_GETTRAILINGREQUIRESCLAUSE: u64 = 3314;
pub const METHOD_GETNUMTEMPLATEPARAMETERLISTS: u64 = 3315;
pub const METHOD_GETTYPESPECSTARTLOC: u64 = 3316;
pub const METHOD_GETTYPESPECENDLOC: u64 = 3317;
pub const CLASS_LOOPHINTATTR: u64 = 3318;
pub const CLASS_NAMESPACEDECL: u64 = 3319;
pub const METHOD_ISANONYMOUSNAMESPACE: u64 = 3320;
pub const METHOD_ISINLINE: u64 = 3321;
pub const METHOD_ISNESTED: u64 = 3322;
pub const METHOD_GETORIGINALNAMESPACE: u64 = 3323;
pub const METHOD_ISORIGINALNAMESPACE: u64 = 3324;
pub const METHOD_GETANONYMOUSNAMESPACE: u64 = 3325;
pub const METHOD_GETCANONICALDECL_15: u64 = 3326;
pub const METHOD_GETSOURCERANGE_19: u64 = 3327;
pub const METHOD_GETBEGINLOC_2: u64 = 3328;
pub const METHOD_GETRBRACELOC_3: u64 = 3329;
pub const CLASS_CARRIESDEPENDENCYATTR: u64 = 3330;
pub const CLASS_LEAFATTR: u64 = 3331;
pub const CLASS_RISCVINTERRUPTATTR: u64 = 3332;
pub const CLASS_INDIRECTGOTOSTMT: u64 = 3333;
pub const METHOD_GETGOTOLOC_1: u64 = 3334;
pub const METHOD_GETSTARLOC: u64 = 3335;
pub const METHOD_GETTARGET: u64 = 3336;
pub const METHOD_GETCONSTANTTARGET: u64 = 3337;
pub const METHOD_GETBEGINLOC_86: u64 = 3338;
pub const METHOD_GETENDLOC_85: u64 = 3339;
pub const METHOD_CHILDREN_74: u64 = 3340;
pub const CLASS_OVERLOADABLEATTR: u64 = 3341;
pub const CLASS_OBJCOBJECTTYPELOC: u64 = 3342;
pub const CLASS_LAYOUTVERSIONATTR: u64 = 3343;
pub const CLASS_INTERNALLINKAGEATTR: u64 = 3344;
pub const CLASS_ALWAYSDESTROYATTR: u64 = 3345;
pub const CLASS_ELABORATEDTYPE: u64 = 3346;
pub const METHOD_GETQUALIFIER_2: u64 = 3347;
pub const METHOD_GETNAMEDTYPE: u64 = 3348;
pub const METHOD_DESUGAR_18: u64 = 3349;
pub const METHOD_ISSUGARED_18: u64 = 3350;
pub const METHOD_GETOWNEDTAGDECL: u64 = 3351;
pub const CLASS_OBJCDIRECTMEMBERSATTR: u64 = 3352;
pub const CLASS_OMPTASKLOOPDIRECTIVE: u64 = 3353;
pub const CLASS_OBJCMETHODDECL: u64 = 3354;
pub const CLASS_IBOUTLETCOLLECTIONATTR: u64 = 3355;
pub const CLASS_OBJCPROTOCOLDECL: u64 = 3356;
pub const CLASS_DEPENDENTSIZEDMATRIXTYPE: u64 = 3357;
pub const METHOD_GETROWEXPR: u64 = 3358;
pub const METHOD_GETCOLUMNEXPR: u64 = 3359;
pub const METHOD_GETATTRIBUTELOC_2: u64 = 3360;
pub const CLASS_DISABLETAILCALLSATTR: u64 = 3361;
pub const CLASS_FORMATATTR: u64 = 3362;
pub const CLASS_FALLTHROUGHATTR: u64 = 3363;
pub const CLASS_IBACTIONATTR: u64 = 3364;
pub const CLASS_FLAGENUMATTR: u64 = 3365;
pub const CLASS_CUDAHOSTATTR: u64 = 3366;
pub const CLASS_SEHTRYSTMT: u64 = 3367;
pub const METHOD_GETBEGINLOC_113: u64 = 3368;
pub const METHOD_GETTRYLOC_1: u64 = 3369;
pub const METHOD_GETENDLOC_112: u64 = 3370;
pub const METHOD_GETISCXXTRY: u64 = 3371;
pub const METHOD_GETTRYBLOCK_1: u64 = 3372;
pub const METHOD_GETHANDLER: u64 = 3373;
pub const METHOD_GETEXCEPTHANDLER: u64 = 3374;
pub const METHOD_GETFINALLYHANDLER: u64 = 3375;
pub const METHOD_CHILDREN_100: u64 = 3376;
pub const CLASS_FRIENDTEMPLATEDECL: u64 = 3377;
pub const METHOD_GETFRIENDTYPE_1: u64 = 3378;
pub const METHOD_GETFRIENDDECL_1: u64 = 3379;
pub const METHOD_GETFRIENDLOC_1: u64 = 3380;
pub const METHOD_GETNUMTEMPLATEPARAMETERS: u64 = 3381;
pub const CLASS_HLSLRESOURCEBINDINGATTR: u64 = 3382;
pub const CLASS_BUILTINTEMPLATEDECL: u64 = 3383;
pub const METHOD_GETSOURCERANGE_2: u64 = 3384;
pub const METHOD_GETBUILTINTEMPLATEKIND: u64 = 3385;
pub const CLASS_OMPDECLAREMAPPERDECL: u64 = 3386;
pub const CLASS_DISABLESANITIZERINSTRUMENTATIONATTR: u64 = 3387;
pub const CLASS_GUARDEDVARATTR: u64 = 3388;
pub const CLASS_TYPEDEFTYPE: u64 = 3389;
pub const METHOD_GETDECL_5: u64 = 3390;
pub const METHOD_ISSUGARED_41: u64 = 3391;
pub const METHOD_DESUGAR_41: u64 = 3392;
pub const METHOD_TYPEMATCHESDECL: u64 = 3393;
pub const CLASS_WEAKIMPORTATTR: u64 = 3394;
pub const CLASS_FORMATARGATTR: u64 = 3395;
pub const CLASS_EXTERNALSOURCESYMBOLATTR: u64 = 3396;
pub const CLASS_CXXOPERATORCALLEXPR: u64 = 3397;
pub const METHOD_GETOPERATOR_1: u64 = 3398;
pub const METHOD_ISASSIGNMENTOP_1: u64 = 3399;
pub const METHOD_ISCOMPARISONOP_1: u64 = 3400;
pub const METHOD_ISINFIXBINARYOP: u64 = 3401;
pub const METHOD_GETOPERATORLOC_3: u64 = 3402;
pub const METHOD_GETEXPRLOC_4: u64 = 3403;
pub const METHOD_GETBEGINLOC_35: u64 = 3404;
pub const METHOD_GETENDLOC_34: u64 = 3405;
pub const METHOD_GETSOURCERANGE_41: u64 = 3406;
pub const CLASS_OBJCPROPERTYIMPLDECL: u64 = 3407;
pub const CLASS_ENFORCETCBLEAFATTR: u64 = 3408;
pub const CLASS_OBJCBRIDGEDCASTEXPR: u64 = 3409;
pub const CLASS_NAMEDDECL: u64 = 3410;
pub const METHOD_GETIDENTIFIER_4: u64 = 3411;
pub const METHOD_GETNAME_3: u64 = 3412;
pub const METHOD_GETNAMEASSTRING: u64 = 3413;
pub const METHOD_GETDECLNAME: u64 = 3414;
pub const METHOD_GETQUALIFIEDNAMEASSTRING: u64 = 3415;
pub const METHOD_HASLINKAGE: u64 = 3416;
pub const METHOD_ISCXXCLASSMEMBER: u64 = 3417;
pub const METHOD_ISCXXINSTANCEMEMBER: u64 = 3418;
pub const METHOD_GETLINKAGEINTERNAL: u64 = 3419;
pub const METHOD_GETFORMALLINKAGE: u64 = 3420;
pub const METHOD_HASEXTERNALFORMALLINKAGE: u64 = 3421;
pub const METHOD_ISEXTERNALLYVISIBLE: u64 = 3422;
pub const METHOD_ISEXTERNALLYDECLARABLE: u64 = 3423;
pub const METHOD_GETVISIBILITY_1: u64 = 3424;
pub const METHOD_GETLINKAGEANDVISIBILITY_1: u64 = 3425;
pub const METHOD_ISLINKAGEVALID: u64 = 3426;
pub const METHOD_HASLINKAGEBEENCOMPUTED: u64 = 3427;
pub const METHOD_GETUNDERLYINGDECL: u64 = 3428;
pub const METHOD_GETMOSTRECENTDECL_6: u64 = 3429;
pub const METHOD_GETOBJCFSTRINGFORMATTINGFAMILY: u64 = 3430;
pub const CLASS_LOCKSEXCLUDEDATTR: u64 = 3431;
pub const CLASS_DEPENDENTTYPEOFEXPRTYPE: u64 = 3432;
pub const CLASS_EXCLUDEFROMEXPLICITINSTANTIATIONATTR: u64 = 3433;
pub const CLASS_OBJCEXPLICITPROTOCOLIMPLATTR: u64 = 3434;
pub const CLASS_DEPENDENTTEMPLATESPECIALIZATIONTYPE: u64 = 3435;
pub const METHOD_GETQUALIFIER_1: u64 = 3436;
pub const METHOD_GETIDENTIFIER_1: u64 = 3437;
pub const METHOD_TEMPLATE_ARGUMENTS: u64 = 3438;
pub const METHOD_ISSUGARED_16: u64 = 3439;
pub const METHOD_DESUGAR_16: u64 = 3440;
pub const CLASS_SWIFTCALLATTR: u64 = 3441;
pub const CLASS_ENABLEIFATTR: u64 = 3442;
pub const CLASS_SETTYPESTATEATTR: u64 = 3443;
pub const CLASS_OBJCARRAYLITERAL: u64 = 3444;
pub const CLASS_OSRETURNSRETAINEDATTR: u64 = 3445;
pub const CLASS_ATOMICTYPELOC: u64 = 3446;
pub const METHOD_GETVALUELOC_1: u64 = 3447;
pub const METHOD_GETLOCALSOURCERANGE_9: u64 = 3448;
pub const METHOD_GETKWLOC_1: u64 = 3449;
pub const METHOD_GETLPARENLOC_13: u64 = 3450;
pub const METHOD_GETRPARENLOC_31: u64 = 3451;
pub const METHOD_GETPARENSRANGE: u64 = 3452;
pub const METHOD_GETINNERTYPE_8: u64 = 3453;
pub const CLASS_OWNERATTR: u64 = 3454;
pub const CLASS_STMTEXPR: u64 = 3455;
pub const METHOD_GETSUBSTMT_5: u64 = 3456;
pub const METHOD_GETBEGINLOC_118: u64 = 3457;
pub const METHOD_GETENDLOC_117: u64 = 3458;
pub const METHOD_GETLPARENLOC_9: u64 = 3459;
pub const METHOD_GETRPARENLOC_24: u64 = 3460;
pub const METHOD_GETTEMPLATEDEPTH_1: u64 = 3461;
pub const METHOD_CHILDREN_106: u64 = 3462;
pub const CLASS_NOMICROMIPSATTR: u64 = 3463;
pub const CLASS_DEPRECATEDATTR: u64 = 3464;
pub const CLASS_TESTTYPESTATEATTR: u64 = 3465;
pub const CLASS_CFAUDITEDTRANSFERATTR: u64 = 3466;
pub const CLASS_HLSLPARAMMODIFIERATTR: u64 = 3467;
pub const CLASS_DLLIMPORTSTATICLOCALATTR: u64 = 3468;
pub const CLASS_OMPPARALLELMASTERTASKLOOPDIRECTIVE: u64 = 3469;
pub const CLASS_BLOCKSATTR: u64 = 3470;
pub const CLASS_NODUPLICATEATTR: u64 = 3471;
pub const CLASS_OPENCLGENERICADDRESSSPACEATTR: u64 = 3472;
pub const CLASS_OSCONSUMEDATTR: u64 = 3473;
pub const CLASS_DLLEXPORTSTATICLOCALATTR: u64 = 3474;
pub const CLASS_BTFTAGATTRIBUTEDTYPE: u64 = 3475;
pub const METHOD_GETWRAPPEDTYPE: u64 = 3476;
pub const METHOD_GETATTR: u64 = 3477;
pub const METHOD_ISSUGARED_3: u64 = 3478;
pub const METHOD_DESUGAR_3: u64 = 3479;
pub const CLASS_STANDALONEDEBUGATTR: u64 = 3480;
pub const CLASS_DEPENDENTNAMETYPE: u64 = 3481;
pub const METHOD_GETQUALIFIER: u64 = 3482;
pub const METHOD_GETIDENTIFIER: u64 = 3483;
pub const METHOD_ISSUGARED_13: u64 = 3484;
pub const METHOD_DESUGAR_13: u64 = 3485;
pub const CLASS_NOINSTRUMENTFUNCTIONATTR: u64 = 3486;
pub const CLASS_DECLTYPETYPE: u64 = 3487;
pub const METHOD_GETUNDERLYINGEXPR: u64 = 3488;
pub const METHOD_GETUNDERLYINGTYPE: u64 = 3489;
pub const METHOD_DESUGAR_9: u64 = 3490;
pub const METHOD_ISSUGARED_9: u64 = 3491;
pub const CLASS_DEPENDENTBITINTTYPE: u64 = 3492;
pub const METHOD_ISUNSIGNED_1: u64 = 3493;
pub const METHOD_ISSIGNED_1: u64 = 3494;
pub const METHOD_GETNUMBITSEXPR: u64 = 3495;
pub const METHOD_ISSUGARED_12: u64 = 3496;
pub const METHOD_DESUGAR_12: u64 = 3497;
pub const CLASS_UNNAMEDGLOBALCONSTANTDECL: u64 = 3498;
pub const METHOD_GETVALUE_3: u64 = 3499;
pub const CLASS_CLEANUPATTR: u64 = 3500;
pub const CLASS_TYPEOFEXPRTYPE: u64 = 3501;
pub const METHOD_GETUNDERLYINGEXPR_1: u64 = 3502;
pub const METHOD_GETKIND_1: u64 = 3503;
pub const METHOD_DESUGAR_39: u64 = 3504;
pub const METHOD_ISSUGARED_39: u64 = 3505;
pub const CLASS_UNUSEDATTR: u64 = 3506;
pub const CLASS_INCOMPLETEARRAYTYPE: u64 = 3507;
pub const METHOD_ISSUGARED_23: u64 = 3508;
pub const METHOD_DESUGAR_23: u64 = 3509;
pub const CLASS_TYPEDEFNAMEDECL: u64 = 3510;
pub const METHOD_ISMODED: u64 = 3511;
pub const METHOD_GETTYPESOURCEINFO_1: u64 = 3512;
pub const METHOD_GETUNDERLYINGTYPE_4: u64 = 3513;
pub const METHOD_GETCANONICALDECL_20: u64 = 3514;
pub const METHOD_ISTRANSPARENTTAG: u64 = 3515;
pub const CLASS_XRAYINSTRUMENTATTR: u64 = 3516;
pub const CLASS_SUBSTTEMPLATETYPEPARMTYPE: u64 = 3517;
pub const METHOD_GETREPLACEMENTTYPE: u64 = 3518;
pub const METHOD_GETASSOCIATEDDECL_1: u64 = 3519;
pub const METHOD_GETREPLACEDPARAMETER_1: u64 = 3520;
pub const METHOD_GETINDEX_1: u64 = 3521;
pub const METHOD_GETPACKINDEX: u64 = 3522;
pub const METHOD_ISSUGARED_36: u64 = 3523;
pub const METHOD_DESUGAR_36: u64 = 3524;
pub const CLASS_ATTRIBUTEDTYPE: u64 = 3525;
pub const METHOD_GETATTRKIND: u64 = 3526;
pub const METHOD_GETMODIFIEDTYPE: u64 = 3527;
pub const METHOD_GETEQUIVALENTTYPE: u64 = 3528;
pub const METHOD_ISSUGARED_2: u64 = 3529;
pub const METHOD_DESUGAR_2: u64 = 3530;
pub const METHOD_ISQUALIFIER: u64 = 3531;
pub const METHOD_ISMSTYPESPEC: u64 = 3532;
pub const METHOD_ISWEBASSEMBLYFUNCREFSPEC: u64 = 3533;
pub const METHOD_ISCALLINGCONV: u64 = 3534;
pub const METHOD_GETIMMEDIATENULLABILITY: u64 = 3535;
pub const CLASS_BLOCKDECL: u64 = 3536;
pub const METHOD_GETCARETLOCATION: u64 = 3537;
pub const METHOD_ISVARIADIC_1: u64 = 3538;
pub const METHOD_GETCOMPOUNDBODY: u64 = 3539;
pub const METHOD_GETBODY: u64 = 3540;
pub const METHOD_GETSIGNATUREASWRITTEN: u64 = 3541;
pub const METHOD_PARAMETERS: u64 = 3542;
pub const METHOD_PARAM_EMPTY: u64 = 3543;
pub const METHOD_PARAM_BEGIN_2: u64 = 3544;
pub const METHOD_PARAM_END_2: u64 = 3545;
pub const METHOD_PARAM_SIZE: u64 = 3546;
pub const METHOD_GETNUMPARAMS_1: u64 = 3547;
pub const METHOD_HASCAPTURES: u64 = 3548;
pub const METHOD_GETNUMCAPTURES: u64 = 3549;
pub const METHOD_CAPTURES: u64 = 3550;
pub const METHOD_CAPTURE_BEGIN_2: u64 = 3551;
pub const METHOD_CAPTURE_END_2: u64 = 3552;
pub const METHOD_CAPTURESCXXTHIS: u64 = 3553;
pub const METHOD_BLOCKMISSINGRETURNTYPE: u64 = 3554;
pub const METHOD_ISCONVERSIONFROMLAMBDA: u64 = 3555;
pub const METHOD_DOESNOTESCAPE: u64 = 3556;
pub const METHOD_CANAVOIDCOPYTOHEAP: u64 = 3557;
pub const METHOD_GETBLOCKMANGLINGNUMBER: u64 = 3558;
pub const METHOD_GETBLOCKMANGLINGCONTEXTDECL: u64 = 3559;
pub const METHOD_GETSOURCERANGE_1: u64 = 3560;
pub const CLASS_TYPEOFEXPRTYPELOC: u64 = 3561;
pub const METHOD_GETUNDERLYINGEXPR_3: u64 = 3562;
pub const METHOD_GETLOCALSOURCERANGE_10: u64 = 3563;
pub const CLASS_OBJCMETHODFAMILYATTR: u64 = 3564;
pub const CLASS_PIPETYPE: u64 = 3565;
pub const METHOD_GETELEMENTTYPE_5: u64 = 3566;
pub const METHOD_ISSUGARED_31: u64 = 3567;
pub const METHOD_DESUGAR_31: u64 = 3568;
pub const METHOD_ISREADONLY: u64 = 3569;
pub const CLASS_BITINTTYPELOC: u64 = 3570;
pub const CLASS_LVALUEREFERENCETYPE: u64 = 3571;
pub const METHOD_ISSUGARED_25: u64 = 3572;
pub const METHOD_DESUGAR_25: u64 = 3573;
pub const CLASS_CXXTHISEXPR: u64 = 3574;
pub const METHOD_GETLOCATION_5: u64 = 3575;
pub const METHOD_GETBEGINLOC_42: u64 = 3576;
pub const METHOD_GETENDLOC_41: u64 = 3577;
pub const METHOD_ISIMPLICIT_1: u64 = 3578;
pub const METHOD_CHILDREN_31: u64 = 3579;
pub const CLASS_OBJCOBJECTTYPEIMPL: u64 = 3580;
pub const CLASS_TEMPLATESPECIALIZATIONTYPE: u64 = 3581;
pub const METHOD_ISCURRENTINSTANTIATION: u64 = 3582;
pub const METHOD_ISTYPEALIAS: u64 = 3583;
pub const METHOD_GETALIASEDTYPE: u64 = 3584;
pub const METHOD_GETTEMPLATENAME_2: u64 = 3585;
pub const METHOD_TEMPLATE_ARGUMENTS_1: u64 = 3586;
pub const METHOD_ISSUGARED_37: u64 = 3587;
pub const METHOD_DESUGAR_37: u64 = 3588;
pub const CLASS_OBJCOBJECTPOINTERTYPE: u64 = 3589;
pub const CLASS_CAPTUREDRECORDATTR: u64 = 3590;
pub const CLASS_PACKEXPANSIONTYPE: u64 = 3591;
pub const METHOD_GETPATTERN: u64 = 3592;
pub const METHOD_GETNUMEXPANSIONS: u64 = 3593;
pub const METHOD_ISSUGARED_29: u64 = 3594;
pub const METHOD_DESUGAR_29: u64 = 3595;
pub const CLASS_DEPENDENTVECTORTYPELOC: u64 = 3596;
pub const METHOD_GETNAMELOC_4: u64 = 3597;
pub const METHOD_GETLOCALSOURCERANGE_11: u64 = 3598;
pub const METHOD_GETELEMENTLOC_3: u64 = 3599;
pub const METHOD_GETINNERTYPE_9: u64 = 3600;
pub const CLASS_DEPENDENTSIZEDARRAYTYPE: u64 = 3601;
pub const METHOD_GETSIZEEXPR_1: u64 = 3602;
pub const METHOD_GETBRACKETSRANGE: u64 = 3603;
pub const METHOD_GETLBRACKETLOC: u64 = 3604;
pub const METHOD_GETRBRACKETLOC: u64 = 3605;
pub const METHOD_ISSUGARED_14: u64 = 3606;
pub const METHOD_DESUGAR_14: u64 = 3607;
pub const CLASS_INJECTEDCLASSNAMETYPE: u64 = 3608;
pub const METHOD_GETINJECTEDSPECIALIZATIONTYPE: u64 = 3609;
pub const METHOD_GETINJECTEDTST: u64 = 3610;
pub const METHOD_GETTEMPLATENAME_1: u64 = 3611;
pub const METHOD_GETDECL_1: u64 = 3612;
pub const METHOD_ISSUGARED_24: u64 = 3613;
pub const METHOD_DESUGAR_24: u64 = 3614;
pub const CLASS_ADDRESSSPACEATTR: u64 = 3615;
pub const CLASS_OVERRIDEATTR: u64 = 3616;
pub const CLASS_USINGTYPE: u64 = 3617;
pub const METHOD_GETFOUNDDECL: u64 = 3618;
pub const METHOD_GETUNDERLYINGTYPE_3: u64 = 3619;
pub const METHOD_ISSUGARED_44: u64 = 3620;
pub const METHOD_DESUGAR_44: u64 = 3621;
pub const METHOD_TYPEMATCHESDECL_1: u64 = 3622;
pub const CLASS_FUNCTIONTYPE: u64 = 3623;
pub const METHOD_GETRETURNTYPE: u64 = 3624;
pub const METHOD_GETHASREGPARM: u64 = 3625;
pub const METHOD_GETREGPARMTYPE: u64 = 3626;
pub const METHOD_GETNORETURNATTR: u64 = 3627;
pub const METHOD_GETCMSENSCALLATTR: u64 = 3628;
pub const METHOD_GETCALLCONV: u64 = 3629;
pub const METHOD_GETEXTINFO: u64 = 3630;
pub const METHOD_ISCONST: u64 = 3631;
pub const METHOD_ISVOLATILE: u64 = 3632;
pub const METHOD_ISRESTRICT: u64 = 3633;
pub const CLASS_EXTVECTORTYPE: u64 = 3634;
pub const METHOD_ISSUGARED_20: u64 = 3635;
pub const METHOD_DESUGAR_20: u64 = 3636;
pub const CLASS_CXXDYNAMICCASTEXPR: u64 = 3637;
pub const METHOD_ISALWAYSNULL: u64 = 3638;
pub const CLASS_DEPENDENTADDRESSSPACETYPE: u64 = 3639;
pub const METHOD_GETADDRSPACEEXPR: u64 = 3640;
pub const METHOD_GETPOINTEETYPE_2: u64 = 3641;
pub const METHOD_GETATTRIBUTELOC: u64 = 3642;
pub const METHOD_ISSUGARED_11: u64 = 3643;
pub const METHOD_DESUGAR_11: u64 = 3644;
pub const CLASS_VECRETURNATTR: u64 = 3645;
pub const CLASS_DECAYEDTYPE: u64 = 3646;
pub const METHOD_GETDECAYEDTYPE: u64 = 3647;
pub const METHOD_GETPOINTEETYPE_1: u64 = 3648;
pub const CLASS_CAPABILITYATTR: u64 = 3649;
pub const CLASS_MEMBERPOINTERTYPE: u64 = 3650;
pub const METHOD_GETPOINTEETYPE_3: u64 = 3651;
pub const METHOD_ISMEMBERFUNCTIONPOINTER: u64 = 3652;
pub const METHOD_ISMEMBERDATAPOINTER: u64 = 3653;
pub const METHOD_GETCLASS: u64 = 3654;
pub const METHOD_GETMOSTRECENTCXXRECORDDECL: u64 = 3655;
pub const METHOD_ISSUGARED_28: u64 = 3656;
pub const METHOD_DESUGAR_28: u64 = 3657;
pub const CLASS_TYPEDEFTYPELOC: u64 = 3658;
pub const METHOD_GETTYPEDEFNAMEDECL: u64 = 3659;
pub const CLASS_ATTR: u64 = 3660;
pub const CLASS_INJECTEDCLASSNAMETYPELOC: u64 = 3661;
pub const METHOD_GETDECL_10: u64 = 3662;
pub const CLASS_RVALUEREFERENCETYPE: u64 = 3663;
pub const METHOD_ISSUGARED_33: u64 = 3664;
pub const METHOD_DESUGAR_33: u64 = 3665;
pub const CLASS_ENUMTYPE: u64 = 3666;
pub const METHOD_GETDECL: u64 = 3667;
pub const METHOD_ISSUGARED_19: u64 = 3668;
pub const METHOD_DESUGAR_19: u64 = 3669;
pub const CLASS_DEPENDENTSIZEDMATRIXTYPELOC: u64 = 3670;
pub const CLASS_ATOMICTYPE: u64 = 3671;
pub const METHOD_GETVALUETYPE: u64 = 3672;
pub const METHOD_ISSUGARED_1: u64 = 3673;
pub const METHOD_DESUGAR_1: u64 = 3674;
pub const CLASS_USINGTYPELOC: u64 = 3675;
pub const METHOD_GETUNDERLYINGTYPE_5: u64 = 3676;
pub const METHOD_GETFOUNDDECL_4: u64 = 3677;
pub const CLASS_CONSTANTMATRIXTYPELOC: u64 = 3678;
pub const CLASS_MACROQUALIFIEDTYPELOC: u64 = 3679;
pub const METHOD_GETINNERLOC_1: u64 = 3680;
pub const METHOD_GETMACROIDENTIFIER_1: u64 = 3681;
pub const METHOD_GETEXPANSIONLOC: u64 = 3682;
pub const METHOD_GETINNERTYPE_10: u64 = 3683;
pub const METHOD_GETLOCALSOURCERANGE_12: u64 = 3684;
pub const CLASS_DEPENDENTNAMETYPELOC: u64 = 3685;
pub const METHOD_GETELABORATEDKEYWORDLOC: u64 = 3686;
pub const METHOD_GETQUALIFIERLOC_16: u64 = 3687;
pub const METHOD_GETNAMELOC_5: u64 = 3688;
pub const METHOD_GETLOCALSOURCERANGE_13: u64 = 3689;
pub const CLASS_DEPENDENTADDRESSSPACETYPELOC: u64 = 3690;
pub const METHOD_GETATTRNAMELOC: u64 = 3691;
pub const METHOD_GETATTREXPROPERAND: u64 = 3692;
pub const METHOD_GETATTROPERANDPARENSRANGE: u64 = 3693;
pub const METHOD_GETLOCALSOURCERANGE_14: u64 = 3694;
pub const METHOD_GETINNERTYPE_11: u64 = 3695;
pub const METHOD_GETPOINTEETYPELOC: u64 = 3696;
pub const CLASS_GNUINLINEATTR: u64 = 3697;
pub const CLASS_BLOCKPOINTERTYPELOC: u64 = 3698;
pub const METHOD_GETCARETLOC: u64 = 3699;
pub const CLASS_OBJCINTERFACETYPE: u64 = 3700;
pub const CLASS_OBJCOBJECTPOINTERTYPELOC: u64 = 3701;
pub const CLASS_OBJCBOXABLEATTR: u64 = 3702;
pub const CLASS_FUNCTIONPROTOTYPELOC: u64 = 3703;
pub const CLASS_X86FORCEALIGNARGPOINTERATTR: u64 = 3704;
pub const CLASS_UNLIKELYATTR: u64 = 3705;
pub const CLASS_UNRESOLVEDUSINGTYPELOC: u64 = 3706;
pub const METHOD_GETDECL_11: u64 = 3707;
pub const CLASS_OBJCNONRUNTIMEPROTOCOLATTR: u64 = 3708;
pub const CLASS_TEMPLATESPECIALIZATIONTYPELOC: u64 = 3709;
pub const METHOD_GETTEMPLATEKEYWORDLOC_7: u64 = 3710;
pub const METHOD_GETLANGLELOC_5: u64 = 3711;
pub const METHOD_GETRANGLELOC_5: u64 = 3712;
pub const METHOD_GETNUMARGS_5: u64 = 3713;
pub const METHOD_GETTEMPLATENAMELOC: u64 = 3714;
pub const METHOD_GETLOCALSOURCERANGE_15: u64 = 3715;
pub const METHOD_GETEXTRALOCALDATASIZE: u64 = 3716;
pub const METHOD_GETEXTRALOCALDATAALIGNMENT: u64 = 3717;
pub const CLASS_FUNCTIONPROTOTYPE: u64 = 3718;
pub const METHOD_GETNUMPARAMS: u64 = 3719;
pub const METHOD_GETPARAMTYPES: u64 = 3720;
pub const METHOD_GETEXTPROTOINFO: u64 = 3721;
pub const METHOD_GETEXCEPTIONSPECTYPE: u64 = 3722;
pub const METHOD_HASEXCEPTIONSPEC: u64 = 3723;
pub const METHOD_HASDYNAMICEXCEPTIONSPEC: u64 = 3724;
pub const METHOD_HASNOEXCEPTEXCEPTIONSPEC: u64 = 3725;
pub const METHOD_HASDEPENDENTEXCEPTIONSPEC: u64 = 3726;
pub const METHOD_HASINSTANTIATIONDEPENDENTEXCEPTIONSPEC: u64 = 3727;
pub const METHOD_GETEXCEPTIONSPECINFO: u64 = 3728;
pub const METHOD_GETNUMEXCEPTIONS: u64 = 3729;
pub const METHOD_GETNOEXCEPTEXPR: u64 = 3730;
pub const METHOD_GETEXCEPTIONSPECDECL: u64 = 3731;
pub const METHOD_GETEXCEPTIONSPECTEMPLATE: u64 = 3732;
pub const METHOD_CANTHROW: u64 = 3733;
pub const METHOD_ISVARIADIC: u64 = 3734;
pub const METHOD_GETELLIPSISLOC: u64 = 3735;
pub const METHOD_ISTEMPLATEVARIADIC: u64 = 3736;
pub const METHOD_HASTRAILINGRETURN: u64 = 3737;
pub const METHOD_GETMETHODQUALS: u64 = 3738;
pub const METHOD_GETREFQUALIFIER: u64 = 3739;
pub const METHOD_PARAM_TYPES: u64 = 3740;
pub const METHOD_PARAM_TYPE_BEGIN: u64 = 3741;
pub const METHOD_PARAM_TYPE_END: u64 = 3742;
pub const METHOD_EXCEPTIONS: u64 = 3743;
pub const METHOD_EXCEPTION_BEGIN: u64 = 3744;
pub const METHOD_EXCEPTION_END: u64 = 3745;
pub const METHOD_HASEXTPARAMETERINFOS: u64 = 3746;
pub const METHOD_GETEXTPARAMETERINFOS: u64 = 3747;
pub const METHOD_GETEXTPARAMETERINFOSORNULL: u64 = 3748;
pub const METHOD_GETAARCH64SMEATTRIBUTES: u64 = 3749;
pub const METHOD_ISSUGARED_22: u64 = 3750;
pub const METHOD_DESUGAR_22: u64 = 3751;
pub const CLASS_QUALIFIEDTYPELOC: u64 = 3752;
pub const METHOD_GETLOCALSOURCERANGE_16: u64 = 3753;
pub const METHOD_GETUNQUALIFIEDLOC_1: u64 = 3754;
pub const METHOD_GETNEXTTYPELOC_1: u64 = 3755;
pub const METHOD_GETLOCALDATASIZE_1: u64 = 3756;
pub const METHOD_GETLOCALDATAALIGNMENT: u64 = 3757;
pub const CLASS_BTFTAGATTRIBUTEDTYPELOC: u64 = 3758;
pub const METHOD_GETWRAPPEDLOC: u64 = 3759;
pub const METHOD_GETATTR_2: u64 = 3760;
pub const METHOD_GETLOCALSOURCERANGE_17: u64 = 3761;
pub const METHOD_GETINNERTYPE_12: u64 = 3762;
pub const CLASS_WEBASSEMBLYIMPORTNAMEATTR: u64 = 3763;
pub const CLASS_FUNCTIONTYPELOC: u64 = 3764;
pub const METHOD_GETLOCALRANGEBEGIN: u64 = 3765;
pub const METHOD_GETLOCALRANGEEND: u64 = 3766;
pub const METHOD_GETLPARENLOC_14: u64 = 3767;
pub const METHOD_GETRPARENLOC_32: u64 = 3768;
pub const METHOD_GETPARENSRANGE_1: u64 = 3769;
pub const METHOD_GETEXCEPTIONSPECRANGE: u64 = 3770;
pub const METHOD_GETPARAMS: u64 = 3771;
pub const METHOD_GETPARMARRAY: u64 = 3772;
pub const METHOD_GETNUMPARAMS_4: u64 = 3773;
pub const METHOD_GETRETURNLOC_1: u64 = 3774;
pub const METHOD_GETLOCALSOURCERANGE_18: u64 = 3775;
pub const METHOD_GETEXTRALOCALDATASIZE_1: u64 = 3776;
pub const METHOD_GETEXTRALOCALDATAALIGNMENT_1: u64 = 3777;
pub const METHOD_GETINNERTYPE_13: u64 = 3778;
pub const CLASS_OMPCANCELDIRECTIVE: u64 = 3779;
pub const CLASS_CONSTANTARRAYTYPE: u64 = 3780;
pub const METHOD_GETSIZE: u64 = 3781;
pub const METHOD_GETSIZEEXPR: u64 = 3782;
pub const METHOD_ISSUGARED_8: u64 = 3783;
pub const METHOD_DESUGAR_8: u64 = 3784;
pub const CLASS_DEPENDENTSIZEDARRAYTYPELOC: u64 = 3785;
pub const CLASS_OBJCKINDOFATTR: u64 = 3786;
pub const CLASS_DEPENDENTTEMPLATESPECIALIZATIONTYPELOC: u64 = 3787;
pub const METHOD_GETELABORATEDKEYWORDLOC_1: u64 = 3788;
pub const METHOD_GETQUALIFIERLOC_17: u64 = 3789;
pub const METHOD_GETTEMPLATEKEYWORDLOC_8: u64 = 3790;
pub const METHOD_GETTEMPLATENAMELOC_1: u64 = 3791;
pub const METHOD_GETLANGLELOC_6: u64 = 3792;
pub const METHOD_GETRANGLELOC_6: u64 = 3793;
pub const METHOD_GETNUMARGS_6: u64 = 3794;
pub const METHOD_GETLOCALSOURCERANGE_19: u64 = 3795;
pub const METHOD_GETEXTRALOCALDATASIZE_2: u64 = 3796;
pub const METHOD_GETEXTRALOCALDATAALIGNMENT_2: u64 = 3797;
pub const CLASS_VALUESTMT: u64 = 3798;
pub const METHOD_GETEXPRSTMT: u64 = 3799;
pub const CLASS_OMPDISPATCHDIRECTIVE: u64 = 3800;
pub const CLASS_HLSLSV_DISPATCHTHREADIDATTR: u64 = 3801;
pub const CLASS_VARIABLEARRAYTYPELOC: u64 = 3802;
pub const CLASS_DEPENDENTSCOPEDECLREFEXPR: u64 = 3803;
pub const METHOD_GETNAMEINFO_5: u64 = 3804;
pub const METHOD_GETDECLNAME_1: u64 = 3805;
pub const METHOD_GETLOCATION_8: u64 = 3806;
pub const METHOD_GETQUALIFIERLOC_11: u64 = 3807;
pub const METHOD_GETQUALIFIER_14: u64 = 3808;
pub const METHOD_GETTEMPLATEKEYWORDLOC_4: u64 = 3809;
pub const METHOD_GETLANGLELOC_2: u64 = 3810;
pub const METHOD_GETRANGLELOC_2: u64 = 3811;
pub const METHOD_HASTEMPLATEKEYWORD_2: u64 = 3812;
pub const METHOD_HASEXPLICITTEMPLATEARGS_3: u64 = 3813;
pub const METHOD_GETTEMPLATEARGS_4: u64 = 3814;
pub const METHOD_GETNUMTEMPLATEARGS_2: u64 = 3815;
pub const METHOD_TEMPLATE_ARGUMENTS_4: u64 = 3816;
pub const METHOD_GETBEGINLOC_67: u64 = 3817;
pub const METHOD_GETENDLOC_66: u64 = 3818;
pub const METHOD_CHILDREN_57: u64 = 3819;
pub const CLASS_MATRIXTYPELOC: u64 = 3820;
pub const METHOD_GETATTRNAMELOC_1: u64 = 3821;
pub const METHOD_GETATTRROWOPERAND: u64 = 3822;
pub const METHOD_GETATTRCOLUMNOPERAND: u64 = 3823;
pub const METHOD_GETATTROPERANDPARENSRANGE_1: u64 = 3824;
pub const METHOD_GETLOCALSOURCERANGE_20: u64 = 3825;
pub const CLASS_TLSMODELATTR: u64 = 3826;
pub const CLASS_FUNCTIONNOPROTOTYPELOC: u64 = 3827;
pub const CLASS_MSDEPENDENTEXISTSSTMT: u64 = 3828;
pub const METHOD_GETKEYWORDLOC_3: u64 = 3829;
pub const METHOD_ISIFEXISTS: u64 = 3830;
pub const METHOD_ISIFNOTEXISTS: u64 = 3831;
pub const METHOD_GETQUALIFIERLOC_12: u64 = 3832;
pub const METHOD_GETNAMEINFO_6: u64 = 3833;
pub const METHOD_GETSUBSTMT_4: u64 = 3834;
pub const METHOD_GETBEGINLOC_92: u64 = 3835;
pub const METHOD_GETENDLOC_91: u64 = 3836;
pub const METHOD_CHILDREN_80: u64 = 3837;
pub const CLASS_PATCHABLEFUNCTIONENTRYATTR: u64 = 3838;
pub const CLASS_UNARYTRANSFORMTYPELOC: u64 = 3839;
pub const METHOD_GETKWLOC_2: u64 = 3840;
pub const METHOD_GETLPARENLOC_15: u64 = 3841;
pub const METHOD_GETRPARENLOC_33: u64 = 3842;
pub const METHOD_GETUNDERLYINGTINFO: u64 = 3843;
pub const METHOD_GETLOCALSOURCERANGE_21: u64 = 3844;
pub const METHOD_GETPARENSRANGE_2: u64 = 3845;
pub const CLASS_COROUTINESUSPENDEXPR: u64 = 3846;
pub const METHOD_GETCOMMONEXPR_1: u64 = 3847;
pub const METHOD_GETOPAQUEVALUE_1: u64 = 3848;
pub const METHOD_GETREADYEXPR: u64 = 3849;
pub const METHOD_GETSUSPENDEXPR: u64 = 3850;
pub const METHOD_GETRESUMEEXPR: u64 = 3851;
pub const METHOD_GETOPERAND_2: u64 = 3852;
pub const METHOD_GETKEYWORDLOC_1: u64 = 3853;
pub const METHOD_GETBEGINLOC_62: u64 = 3854;
pub const METHOD_GETENDLOC_61: u64 = 3855;
pub const METHOD_CHILDREN_52: u64 = 3856;
pub const CLASS_TYPENULLUNSPECIFIEDATTR: u64 = 3857;
pub const CLASS_UNQUALTYPELOC: u64 = 3858;
pub const METHOD_GETTYPEPTR_1: u64 = 3859;
pub const METHOD_GETTYPELOCCLASS_1: u64 = 3860;
pub const CLASS_CUDAINVALIDTARGETATTR: u64 = 3861;
pub const CLASS_TYPEOFTYPELOC: u64 = 3862;
pub const METHOD_GETUNMODIFIEDTYPE_1: u64 = 3863;
pub const METHOD_GETUNMODIFIEDTINFO: u64 = 3864;
pub const CLASS_DEDUCEDTYPELOC: u64 = 3865;
pub const CLASS_NORETURNATTR: u64 = 3866;
pub const CLASS_INCOMPLETEARRAYTYPELOC: u64 = 3867;
pub const CLASS_UNRESOLVEDUSINGTYPE: u64 = 3868;
pub const METHOD_GETDECL_6: u64 = 3869;
pub const METHOD_ISSUGARED_43: u64 = 3870;
pub const METHOD_DESUGAR_43: u64 = 3871;
pub const CLASS_ELABORATEDTYPELOC: u64 = 3872;
pub const METHOD_GETELABORATEDKEYWORDLOC_2: u64 = 3873;
pub const METHOD_GETQUALIFIERLOC_18: u64 = 3874;
pub const METHOD_GETLOCALSOURCERANGE_22: u64 = 3875;
pub const METHOD_GETNAMEDTYPELOC: u64 = 3876;
pub const METHOD_GETINNERTYPE_14: u64 = 3877;
pub const METHOD_ISEMPTY_1: u64 = 3878;
pub const METHOD_GETLOCALDATAALIGNMENT_1: u64 = 3879;
pub const METHOD_GETLOCALDATASIZE_2: u64 = 3880;
pub const CLASS_REFERENCETYPELOC: u64 = 3881;
pub const METHOD_GETINNERTYPE_15: u64 = 3882;
pub const CLASS_CONSTANTARRAYTYPELOC: u64 = 3883;
pub const CLASS_OMPDISTRIBUTESIMDDIRECTIVE: u64 = 3884;
pub const CLASS_RECORDTYPELOC: u64 = 3885;
pub const METHOD_GETDECL_12: u64 = 3886;
pub const CLASS_FUNCTIONRETURNTHUNKSATTR: u64 = 3887;
pub const CLASS_RVALUEREFERENCETYPELOC: u64 = 3888;
pub const METHOD_GETAMPAMPLOC_1: u64 = 3889;
pub const CLASS_ASSERTSHAREDLOCKATTR: u64 = 3890;
pub const CLASS_AUTOTYPELOC: u64 = 3891;
pub const METHOD_GETAUTOKEYWORD: u64 = 3892;
pub const METHOD_ISDECLTYPEAUTO_1: u64 = 3893;
pub const METHOD_GETRPARENLOC_34: u64 = 3894;
pub const METHOD_ISCONSTRAINED_1: u64 = 3895;
pub const METHOD_GETCONCEPTREFERENCE_1: u64 = 3896;
pub const METHOD_GETNESTEDNAMESPECIFIERLOC_1: u64 = 3897;
pub const METHOD_GETTEMPLATEKWLOC_1: u64 = 3898;
pub const METHOD_GETCONCEPTNAMELOC_1: u64 = 3899;
pub const METHOD_GETFOUNDDECL_5: u64 = 3900;
pub const METHOD_GETNAMEDCONCEPT_1: u64 = 3901;
pub const METHOD_GETCONCEPTNAMEINFO_1: u64 = 3902;
pub const METHOD_HASEXPLICITTEMPLATEARGS_6: u64 = 3903;
pub const METHOD_GETLANGLELOC_7: u64 = 3904;
pub const METHOD_GETRANGLELOC_7: u64 = 3905;
pub const METHOD_GETNUMARGS_7: u64 = 3906;
pub const METHOD_GETLOCALSOURCERANGE_23: u64 = 3907;
pub const CLASS_OBJCFORCOLLECTIONSTMT: u64 = 3908;
pub const CLASS_ERRORATTR: u64 = 3909;
pub const CLASS_TAGTYPELOC: u64 = 3910;
pub const METHOD_GETDECL_13: u64 = 3911;
pub const METHOD_ISDEFINITION: u64 = 3912;
pub const CLASS_OBJCOBJECTTYPE: u64 = 3913;
pub const CLASS_BUILTINTYPELOC: u64 = 3914;
pub const METHOD_GETBUILTINLOC_6: u64 = 3915;
pub const METHOD_GETNAMELOC_6: u64 = 3916;
pub const METHOD_GETWRITTENBUILTINSPECS: u64 = 3917;
pub const METHOD_NEEDSEXTRALOCALDATA: u64 = 3918;
pub const METHOD_GETEXTRALOCALDATASIZE_3: u64 = 3919;
pub const METHOD_GETEXTRALOCALDATAALIGNMENT_3: u64 = 3920;
pub const METHOD_GETLOCALSOURCERANGE_24: u64 = 3921;
pub const METHOD_GETWRITTENSIGNSPEC: u64 = 3922;
pub const METHOD_HASWRITTENSIGNSPEC: u64 = 3923;
pub const METHOD_GETWRITTENWIDTHSPEC: u64 = 3924;
pub const METHOD_HASWRITTENWIDTHSPEC: u64 = 3925;
pub const METHOD_GETWRITTENTYPESPEC: u64 = 3926;
pub const METHOD_HASWRITTENTYPESPEC: u64 = 3927;
pub const METHOD_HASMODEATTR: u64 = 3928;
pub const CLASS_TYPESPECTYPELOC: u64 = 3929;
pub const METHOD_GETNAMELOC_7: u64 = 3930;
pub const METHOD_GETLOCALSOURCERANGE_25: u64 = 3931;
pub const CLASS_DEPENDENTBITINTTYPELOC: u64 = 3932;
pub const CLASS_DEDUCEDTEMPLATESPECIALIZATIONTYPELOC: u64 = 3933;
pub const METHOD_GETTEMPLATENAMELOC_2: u64 = 3934;
pub const CLASS_PACKEXPANSIONTYPELOC: u64 = 3935;
pub const METHOD_GETELLIPSISLOC_7: u64 = 3936;
pub const METHOD_GETLOCALSOURCERANGE_26: u64 = 3937;
pub const METHOD_GETPATTERNLOC: u64 = 3938;
pub const METHOD_GETINNERTYPE_16: u64 = 3939;
pub const CLASS_TEMPLATETYPEPARMTYPELOC: u64 = 3940;
pub const METHOD_GETDECL_14: u64 = 3941;
pub const CLASS_RETURNSTMT: u64 = 3942;
pub const METHOD_GETRETVALUE: u64 = 3943;
pub const METHOD_GETNRVOCANDIDATE: u64 = 3944;
pub const METHOD_GETRETURNLOC: u64 = 3945;
pub const METHOD_GETBEGINLOC_109: u64 = 3946;
pub const METHOD_GETENDLOC_108: u64 = 3947;
pub const METHOD_CHILDREN_96: u64 = 3948;
pub const CLASS_MEMBERPOINTERTYPELOC: u64 = 3949;
pub const METHOD_GETSTARLOC_2: u64 = 3950;
pub const METHOD_GETCLASS_1: u64 = 3951;
pub const METHOD_GETCLASSTINFO: u64 = 3952;
pub const METHOD_GETLOCALSOURCERANGE_27: u64 = 3953;
pub const CLASS_SUBSTTEMPLATETYPEPARMTYPELOC: u64 = 3954;
pub const CLASS_WEBASSEMBLYEXPORTNAMEATTR: u64 = 3955;
pub const CLASS_CUDAGLOBALATTR: u64 = 3956;
pub const CLASS_PRAGMACOMMENTDECL: u64 = 3957;
pub const METHOD_GETCOMMENTKIND: u64 = 3958;
pub const METHOD_GETARG: u64 = 3959;
pub const CLASS_EXTERNCCONTEXTDECL: u64 = 3960;
pub const CLASS_OBJCPROPERTYREFEXPR: u64 = 3961;
pub const CLASS_OBJCBRIDGEATTR: u64 = 3962;
pub const CLASS_IMPLICITPARAMDECL: u64 = 3963;
pub const METHOD_GETPARAMETERKIND: u64 = 3964;
pub const CLASS_DECOMPOSITIONDECL: u64 = 3965;
pub const METHOD_BINDINGS: u64 = 3966;
pub const ENUM_VALUEKIND: u64 = 3967;
pub const ENUM_ACCESSSPECIFIER: u64 = 3982;
pub const ENUM_ARRAYSIZEMODIFIER: u64 = 3987;
pub const ENUM_ARRAYTYPETRAIT: u64 = 3991;
pub const ENUM_ATOMICOP: u64 = 3995;
pub const ENUM_AUTOTYPEKEYWORD: u64 = 4084;
pub const ENUM_BINARYOPERATORKIND: u64 = 4088;
pub const ENUM_BUILTINTEMPLATEKIND: u64 = 4122;
pub const ENUM_KIND: u64 = 4125;
pub const ENUM_KIND_1: u64 = 4626;
pub const ENUM_CXXCONSTRUCTIONKIND: u64 = 4631;
pub const ENUM_CXXNEWINITIALIZATIONSTYLE: u64 = 4636;
pub const ENUM_ADLCALLKIND: u64 = 4640;
pub const ENUM_CALLINGCONV: u64 = 4643;
pub const ENUM_CANTHROWRESULT: u64 = 4666;
pub const ENUM_CAPTUREDREGIONKIND: u64 = 4670;
pub const ENUM_CASTKIND: u64 = 4674;
pub const ENUM_CHARACTERLITERALKIND: u64 = 4740;
pub const ENUM_CONSTANTRESULTSTORAGEKIND: u64 = 4746;
pub const ENUM_CONSTEXPRSPECKIND: u64 = 4750;
pub const ENUM_FRIENDOBJECTKIND: u64 = 4755;
pub const ENUM_MODULEOWNERSHIPKIND: u64 = 4759;
pub const ENUM_OBJCDECLQUALIFIER: u64 = 4765;
pub const ENUM_DEDUCTIONCANDIDATE: u64 = 4774;
pub const ENUM_ELABORATEDTYPEKEYWORD: u64 = 4778;
pub const ENUM_EXCEPTIONSPECIFICATIONTYPE: u64 = 4786;
pub const ENUM_EXPRDEPENDENCE: u64 = 4799;
pub const ENUM_EXPROBJECTKIND: u64 = 4813;
pub const ENUM_EXPRVALUEKIND: u64 = 4820;
pub const ENUM_EXPRESSIONTRAIT: u64 = 4824;
pub const ENUM_TEMPLATEDKIND: u64 = 4828;
pub const ENUM_IFSTATEMENTKIND: u64 = 4835;
pub const ENUM_IMPLICITPARAMKIND: u64 = 4840;
pub const ENUM_INCLASSINITSTYLE: u64 = 4848;
pub const ENUM_LAMBDACAPTUREDEFAULT: u64 = 4852;
pub const ENUM_LANGUAGELINKAGE: u64 = 4856;
pub const ENUM_LINKAGE: u64 = 4860;
pub const ENUM_LINKAGESPECLANGUAGEIDS: u64 = 4868;
pub const ENUM_MSVTORDISPMODE: u64 = 4871;
pub const ENUM_MULTIVERSIONKIND: u64 = 4875;
pub const ENUM_NONODRUSEREASON: u64 = 4882;
pub const ENUM_OBJCSTRINGFORMATFAMILY: u64 = 4887;
pub const ENUM_OVERLOADEDOPERATORKIND: u64 = 4891;
pub const ENUM_PRAGMAMSCOMMENTKIND: u64 = 4939;
pub const ENUM_PREDEFINEDIDENTKIND: u64 = 4946;
pub const ENUM_OBJCLIFETIME: u64 = 4955;
pub const ENUM_RECORDARGPASSINGKIND: u64 = 4961;
pub const ENUM_REFQUALIFIERKIND: u64 = 4965;
pub const ENUM_SOURCELOCIDENTKIND: u64 = 4969;
pub const ENUM_STORAGECLASS: u64 = 4977;
pub const ENUM_STORAGEDURATION: u64 = 4984;
pub const ENUM_STRINGLITERALKIND: u64 = 4990;
pub const ENUM_TAGTYPEKIND: u64 = 4997;
pub const ENUM_TEMPLATESPECIALIZATIONKIND: u64 = 5003;
pub const ENUM_THREADSTORAGECLASSSPECIFIER: u64 = 5009;
pub const ENUM_TYPEDEPENDENCE: u64 = 5014;
pub const ENUM_TYPEOFKIND: u64 = 5024;
pub const ENUM_TYPETRAIT: u64 = 5027;
pub const ENUM_UNARYEXPRORTYPETRAIT: u64 = 5104;
pub const ENUM_UNARYOPERATORKIND: u64 = 5113;
pub const ENUM_UTTKIND: u64 = 5128;
pub const ENUM_LITERALOPERATORKIND: u64 = 5145;
pub const ENUM_DEFINITIONKIND: u64 = 5152;
pub const ENUM_INITIALIZATIONSTYLE: u64 = 5156;
pub const ENUM_TLSKIND: u64 = 5161;
pub const ENUM_VECTORKIND: u64 = 5165;
pub const ENUM_VISIBILITY: u64 = 5176;
pub const ENUM_KIND_2: u64 = 5180;
pub const ENUM_SEMANTICS: u64 = 5595;
////   END ARBORETUM GENERATED CODE ////
