use arboretum_graph::{Value, GraphBuffer};

//// BEGIN ARBORETUM GENERATED CODE ////
pub fn build_data_model() -> GraphBuffer {
  let mut g = GraphBuffer::new();

  let mut next_id_value = 4294972851;
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

  g.add_named_node(CLASS_OBJCARRAYLITERAL, "clang::ObjCArrayLiteral");
  g.add_edge((CLASS_OBJCARRAYLITERAL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCARRAYLITERAL, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCARRAYLITERAL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXSTDINITIALIZERLISTEXPR, "clang::CXXStdInitializerListExpr");
  g.add_edge((CLASS_CXXSTDINITIALIZERLISTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXSTDINITIALIZERLISTEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967342, "clang::CXXStdInitializerListExpr::getSubExpr"),
      add_named_node(&mut g, 4294967343, "clang::CXXStdInitializerListExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967344, "clang::CXXStdInitializerListExpr::getEndLoc"),
      add_named_node(&mut g, 4294967345, "clang::CXXStdInitializerListExpr::getSourceRange"),
      add_named_node(&mut g, 4294967346, "clang::CXXStdInitializerListExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXSTDINITIALIZERLISTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCSELECTOREXPR, "clang::ObjCSelectorExpr");
  g.add_edge((CLASS_OBJCSELECTOREXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCSELECTOREXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCSELECTOREXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXTYPEIDEXPR, "clang::CXXTypeidExpr");
  g.add_edge((CLASS_CXXTYPEIDEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXTYPEIDEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967349, "clang::CXXTypeidExpr::isPotentiallyEvaluated"),
      add_named_node(&mut g, 4294967350, "clang::CXXTypeidExpr::isTypeOperand"),
      add_named_node(&mut g, 4294967351, "clang::CXXTypeidExpr::getTypeOperandSourceInfo"),
      add_named_node(&mut g, 4294967352, "clang::CXXTypeidExpr::getExprOperand"),
      add_named_node(&mut g, 4294967353, "clang::CXXTypeidExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967354, "clang::CXXTypeidExpr::getEndLoc"),
      add_named_node(&mut g, 4294967355, "clang::CXXTypeidExpr::getSourceRange"),
      add_named_node(&mut g, 4294967356, "clang::CXXTypeidExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXTYPEIDEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCPROPERTYREFEXPR, "clang::ObjCPropertyRefExpr");
  g.add_edge((CLASS_OBJCPROPERTYREFEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCPROPERTYREFEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCPROPERTYREFEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ASMSTMT, "clang::AsmStmt");
  g.add_edge((CLASS_ASMSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ASMSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967359, "clang::AsmStmt::getAsmLoc"),
      add_named_node(&mut g, 4294967360, "clang::AsmStmt::isSimple"),
      add_named_node(&mut g, 4294967361, "clang::AsmStmt::isVolatile"),
      add_named_node(&mut g, 4294967362, "clang::AsmStmt::getBeginLoc"),
      add_named_node(&mut g, 4294967363, "clang::AsmStmt::getEndLoc"),
      add_named_node(&mut g, 4294967364, "clang::AsmStmt::getNumOutputs"),
      add_named_node(&mut g, 4294967365, "clang::AsmStmt::getNumPlusOperands"),
      add_named_node(&mut g, 4294967366, "clang::AsmStmt::getNumInputs"),
      add_named_node(&mut g, 4294967367, "clang::AsmStmt::getNumClobbers"),
      add_named_node(&mut g, 4294967368, "clang::AsmStmt::begin_inputs"),
      add_named_node(&mut g, 4294967369, "clang::AsmStmt::end_inputs"),
      add_named_node(&mut g, 4294967370, "clang::AsmStmt::inputs"),
      add_named_node(&mut g, 4294967371, "clang::AsmStmt::begin_outputs"),
      add_named_node(&mut g, 4294967372, "clang::AsmStmt::end_outputs"),
      add_named_node(&mut g, 4294967373, "clang::AsmStmt::outputs"),
      add_named_node(&mut g, 4294967374, "clang::AsmStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ASMSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTARGETTEAMSDISTRIBUTESIMDDIRECTIVE, "clang::OMPTargetTeamsDistributeSimdDirective");
  g.add_edge((CLASS_OMPTARGETTEAMSDISTRIBUTESIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETTEAMSDISTRIBUTESIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTARGETTEAMSDISTRIBUTESIMDDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCMESSAGEEXPR, "clang::ObjCMessageExpr");
  g.add_edge((CLASS_OBJCMESSAGEEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCMESSAGEEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCMESSAGEEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXPARENLISTINITEXPR, "clang::CXXParenListInitExpr");
  g.add_edge((CLASS_CXXPARENLISTINITEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXPARENLISTINITEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967378, "clang::CXXParenListInitExpr::getInitExprs"),
      add_named_node(&mut g, 4294967379, "clang::CXXParenListInitExpr::getUserSpecifiedInitExprs"),
      add_named_node(&mut g, 4294967380, "clang::CXXParenListInitExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967381, "clang::CXXParenListInitExpr::getEndLoc"),
      add_named_node(&mut g, 4294967382, "clang::CXXParenListInitExpr::getInitLoc"),
      add_named_node(&mut g, 4294967383, "clang::CXXParenListInitExpr::getSourceRange"),
      add_named_node(&mut g, 4294967384, "clang::CXXParenListInitExpr::getArrayFiller"),
      add_named_node(&mut g, 4294967385, "clang::CXXParenListInitExpr::getInitializedFieldInUnion"),
      add_named_node(&mut g, 4294967386, "clang::CXXParenListInitExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXPARENLISTINITEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_VALUESTMT, "clang::ValueStmt");
  g.add_edge((CLASS_VALUESTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_VALUESTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967388, "clang::ValueStmt::getExprStmt"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_VALUESTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CONVERTVECTOREXPR, "clang::ConvertVectorExpr");
  g.add_edge((CLASS_CONVERTVECTOREXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONVERTVECTOREXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967390, "clang::ConvertVectorExpr::getSrcExpr"),
      add_named_node(&mut g, 4294967391, "clang::ConvertVectorExpr::getTypeSourceInfo"),
      add_named_node(&mut g, 4294967392, "clang::ConvertVectorExpr::getBuiltinLoc"),
      add_named_node(&mut g, 4294967393, "clang::ConvertVectorExpr::getRParenLoc"),
      add_named_node(&mut g, 4294967394, "clang::ConvertVectorExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967395, "clang::ConvertVectorExpr::getEndLoc"),
      add_named_node(&mut g, 4294967396, "clang::ConvertVectorExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CONVERTVECTOREXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_GOTOSTMT, "clang::GotoStmt");
  g.add_edge((CLASS_GOTOSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_GOTOSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967398, "clang::GotoStmt::getLabel"),
      add_named_node(&mut g, 4294967399, "clang::GotoStmt::getGotoLoc"),
      add_named_node(&mut g, 4294967400, "clang::GotoStmt::getLabelLoc"),
      add_named_node(&mut g, 4294967401, "clang::GotoStmt::getBeginLoc"),
      add_named_node(&mut g, 4294967402, "clang::GotoStmt::getEndLoc"),
      add_named_node(&mut g, 4294967403, "clang::GotoStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_GOTOSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BREAKSTMT, "clang::BreakStmt");
  g.add_edge((CLASS_BREAKSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BREAKSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967405, "clang::BreakStmt::getBreakLoc"),
      add_named_node(&mut g, 4294967406, "clang::BreakStmt::getBeginLoc"),
      add_named_node(&mut g, 4294967407, "clang::BreakStmt::getEndLoc"),
      add_named_node(&mut g, 4294967408, "clang::BreakStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BREAKSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXDYNAMICCASTEXPR, "clang::CXXDynamicCastExpr");
  g.add_edge((CLASS_CXXDYNAMICCASTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXDYNAMICCASTEXPR, META_SUBCLASS, CLASS_CXXNAMEDCASTEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967410, "clang::CXXDynamicCastExpr::isAlwaysNull"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXDYNAMICCASTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_COROUTINEBODYSTMT, "clang::CoroutineBodyStmt");
  g.add_edge((CLASS_COROUTINEBODYSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_COROUTINEBODYSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967412, "clang::CoroutineBodyStmt::hasDependentPromiseType"),
      add_named_node(&mut g, 4294967413, "clang::CoroutineBodyStmt::getBody"),
      add_named_node(&mut g, 4294967414, "clang::CoroutineBodyStmt::getPromiseDeclStmt"),
      add_named_node(&mut g, 4294967415, "clang::CoroutineBodyStmt::getPromiseDecl"),
      add_named_node(&mut g, 4294967416, "clang::CoroutineBodyStmt::getInitSuspendStmt"),
      add_named_node(&mut g, 4294967417, "clang::CoroutineBodyStmt::getFinalSuspendStmt"),
      add_named_node(&mut g, 4294967418, "clang::CoroutineBodyStmt::getExceptionHandler"),
      add_named_node(&mut g, 4294967419, "clang::CoroutineBodyStmt::getFallthroughHandler"),
      add_named_node(&mut g, 4294967420, "clang::CoroutineBodyStmt::getAllocate"),
      add_named_node(&mut g, 4294967421, "clang::CoroutineBodyStmt::getDeallocate"),
      add_named_node(&mut g, 4294967422, "clang::CoroutineBodyStmt::getResultDecl"),
      add_named_node(&mut g, 4294967423, "clang::CoroutineBodyStmt::getReturnValueInit"),
      add_named_node(&mut g, 4294967424, "clang::CoroutineBodyStmt::getReturnValue"),
      add_named_node(&mut g, 4294967425, "clang::CoroutineBodyStmt::getReturnStmt"),
      add_named_node(&mut g, 4294967426, "clang::CoroutineBodyStmt::getReturnStmtOnAllocFailure"),
      add_named_node(&mut g, 4294967427, "clang::CoroutineBodyStmt::getParamMoves"),
      add_named_node(&mut g, 4294967428, "clang::CoroutineBodyStmt::getBeginLoc"),
      add_named_node(&mut g, 4294967429, "clang::CoroutineBodyStmt::getEndLoc"),
      add_named_node(&mut g, 4294967430, "clang::CoroutineBodyStmt::children"),
      add_named_node(&mut g, 4294967431, "clang::CoroutineBodyStmt::childrenExclBody"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_COROUTINEBODYSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CORETURNSTMT, "clang::CoreturnStmt");
  g.add_edge((CLASS_CORETURNSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CORETURNSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967433, "clang::CoreturnStmt::getKeywordLoc"),
      add_named_node(&mut g, 4294967434, "clang::CoreturnStmt::getOperand"),
      add_named_node(&mut g, 4294967435, "clang::CoreturnStmt::getPromiseCall"),
      add_named_node(&mut g, 4294967436, "clang::CoreturnStmt::isImplicit"),
      add_named_node(&mut g, 4294967437, "clang::CoreturnStmt::getBeginLoc"),
      add_named_node(&mut g, 4294967438, "clang::CoreturnStmt::getEndLoc"),
      add_named_node(&mut g, 4294967439, "clang::CoreturnStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CORETURNSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ARRAYTYPETRAITEXPR, "clang::ArrayTypeTraitExpr");
  g.add_edge((CLASS_ARRAYTYPETRAITEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARRAYTYPETRAITEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967441, "clang::ArrayTypeTraitExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967442, "clang::ArrayTypeTraitExpr::getEndLoc"),
      add_named_node(&mut g, 4294967443, "clang::ArrayTypeTraitExpr::getTrait"),
      add_named_node(&mut g, 4294967444, "clang::ArrayTypeTraitExpr::getQueriedType"),
      add_named_node(&mut g, 4294967445, "clang::ArrayTypeTraitExpr::getQueriedTypeSourceInfo"),
      add_named_node(&mut g, 4294967446, "clang::ArrayTypeTraitExpr::getValue"),
      add_named_node(&mut g, 4294967447, "clang::ArrayTypeTraitExpr::getDimensionExpression"),
      add_named_node(&mut g, 4294967448, "clang::ArrayTypeTraitExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ARRAYTYPETRAITEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCATCATCHSTMT, "clang::ObjCAtCatchStmt");
  g.add_edge((CLASS_OBJCATCATCHSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCATCATCHSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCATCATCHSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FORSTMT, "clang::ForStmt");
  g.add_edge((CLASS_FORSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FORSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967451, "clang::ForStmt::getConditionVariable"),
      add_named_node(&mut g, 4294967452, "clang::ForStmt::getConditionVariableDeclStmt"),
      add_named_node(&mut g, 4294967453, "clang::ForStmt::getInit"),
      add_named_node(&mut g, 4294967454, "clang::ForStmt::getCond"),
      add_named_node(&mut g, 4294967455, "clang::ForStmt::getInc"),
      add_named_node(&mut g, 4294967456, "clang::ForStmt::getBody"),
      add_named_node(&mut g, 4294967457, "clang::ForStmt::getForLoc"),
      add_named_node(&mut g, 4294967458, "clang::ForStmt::getLParenLoc"),
      add_named_node(&mut g, 4294967459, "clang::ForStmt::getRParenLoc"),
      add_named_node(&mut g, 4294967460, "clang::ForStmt::getBeginLoc"),
      add_named_node(&mut g, 4294967461, "clang::ForStmt::getEndLoc"),
      add_named_node(&mut g, 4294967462, "clang::ForStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FORSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCAUTORELEASEPOOLSTMT, "clang::ObjCAutoreleasePoolStmt");
  g.add_edge((CLASS_OBJCAUTORELEASEPOOLSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCAUTORELEASEPOOLSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCAUTORELEASEPOOLSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCDICTIONARYLITERAL, "clang::ObjCDictionaryLiteral");
  g.add_edge((CLASS_OBJCDICTIONARYLITERAL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCDICTIONARYLITERAL, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCDICTIONARYLITERAL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPLOOPTRANSFORMATIONDIRECTIVE, "clang::OMPLoopTransformationDirective");
  g.add_edge((CLASS_OMPLOOPTRANSFORMATIONDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPLOOPTRANSFORMATIONDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPBASEDDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPLOOPTRANSFORMATIONDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCSUBSCRIPTREFEXPR, "clang::ObjCSubscriptRefExpr");
  g.add_edge((CLASS_OBJCSUBSCRIPTREFEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCSUBSCRIPTREFEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCSUBSCRIPTREFEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SYCLUNIQUESTABLENAMEEXPR, "clang::SYCLUniqueStableNameExpr");
  g.add_edge((CLASS_SYCLUNIQUESTABLENAMEEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SYCLUNIQUESTABLENAMEEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967468, "clang::SYCLUniqueStableNameExpr::getTypeSourceInfo"),
      add_named_node(&mut g, 4294967469, "clang::SYCLUniqueStableNameExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967470, "clang::SYCLUniqueStableNameExpr::getEndLoc"),
      add_named_node(&mut g, 4294967471, "clang::SYCLUniqueStableNameExpr::getLocation"),
      add_named_node(&mut g, 4294967472, "clang::SYCLUniqueStableNameExpr::getLParenLocation"),
      add_named_node(&mut g, 4294967473, "clang::SYCLUniqueStableNameExpr::getRParenLocation"),
      add_named_node(&mut g, 4294967474, "clang::SYCLUniqueStableNameExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SYCLUNIQUESTABLENAMEEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPFORSIMDDIRECTIVE, "clang::OMPForSimdDirective");
  g.add_edge((CLASS_OMPFORSIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPFORSIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPFORSIMDDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPPARALLELSECTIONSDIRECTIVE, "clang::OMPParallelSectionsDirective");
  g.add_edge((CLASS_OMPPARALLELSECTIONSDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPPARALLELSECTIONSDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPPARALLELSECTIONSDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CONDITIONALOPERATOR, "clang::ConditionalOperator");
  g.add_edge((CLASS_CONDITIONALOPERATOR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONDITIONALOPERATOR, META_SUBCLASS, CLASS_ABSTRACTCONDITIONALOPERATOR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967478, "clang::ConditionalOperator::getCond"),
      add_named_node(&mut g, 4294967479, "clang::ConditionalOperator::getTrueExpr"),
      add_named_node(&mut g, 4294967480, "clang::ConditionalOperator::getFalseExpr"),
      add_named_node(&mut g, 4294967481, "clang::ConditionalOperator::getLHS"),
      add_named_node(&mut g, 4294967482, "clang::ConditionalOperator::getRHS"),
      add_named_node(&mut g, 4294967483, "clang::ConditionalOperator::getBeginLoc"),
      add_named_node(&mut g, 4294967484, "clang::ConditionalOperator::getEndLoc"),
      add_named_node(&mut g, 4294967485, "clang::ConditionalOperator::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CONDITIONALOPERATOR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTARGETPARALLELFORDIRECTIVE, "clang::OMPTargetParallelForDirective");
  g.add_edge((CLASS_OMPTARGETPARALLELFORDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETPARALLELFORDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTARGETPARALLELFORDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTASKLOOPSIMDDIRECTIVE, "clang::OMPTaskLoopSimdDirective");
  g.add_edge((CLASS_OMPTASKLOOPSIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTASKLOOPSIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTASKLOOPSIMDDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CASTEXPR, "clang::CastExpr");
  g.add_edge((CLASS_CASTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CASTEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967489, "clang::CastExpr::getCastKind"),
      add_named_node(&mut g, 4294967490, "clang::CastExpr::getCastKindName"),
      add_named_node(&mut g, 4294967491, "clang::CastExpr::getSubExpr"),
      add_named_node(&mut g, 4294967492, "clang::CastExpr::getSubExprAsWritten"),
      add_named_node(&mut g, 4294967493, "clang::CastExpr::getConversionFunction"),
      add_named_node(&mut g, 4294967494, "clang::CastExpr::path_empty"),
      add_named_node(&mut g, 4294967495, "clang::CastExpr::path_size"),
      add_named_node(&mut g, 4294967496, "clang::CastExpr::path_begin"),
      add_named_node(&mut g, 4294967497, "clang::CastExpr::path_end"),
      add_named_node(&mut g, 4294967498, "clang::CastExpr::path"),
      add_named_node(&mut g, 4294967499, "clang::CastExpr::getTargetUnionField"),
      add_named_node(&mut g, 4294967500, "clang::CastExpr::hasStoredFPFeatures"),
      add_named_node(&mut g, 4294967501, "clang::CastExpr::getStoredFPFeatures"),
      add_named_node(&mut g, 4294967502, "clang::CastExpr::getFPFeatures"),
      add_named_node(&mut g, 4294967503, "clang::CastExpr::changesVolatileQualification"),
      add_named_node(&mut g, 4294967504, "clang::CastExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CASTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPMASTERTASKLOOPSIMDDIRECTIVE, "clang::OMPMasterTaskLoopSimdDirective");
  g.add_edge((CLASS_OMPMASTERTASKLOOPSIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPMASTERTASKLOOPSIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPMASTERTASKLOOPSIMDDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXTRYSTMT, "clang::CXXTryStmt");
  g.add_edge((CLASS_CXXTRYSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXTRYSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967507, "clang::CXXTryStmt::getBeginLoc"),
      add_named_node(&mut g, 4294967508, "clang::CXXTryStmt::getTryLoc"),
      add_named_node(&mut g, 4294967509, "clang::CXXTryStmt::getEndLoc"),
      add_named_node(&mut g, 4294967510, "clang::CXXTryStmt::getTryBlock"),
      add_named_node(&mut g, 4294967511, "clang::CXXTryStmt::getNumHandlers"),
      add_named_node(&mut g, 4294967512, "clang::CXXTryStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXTRYSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPMASKEDTASKLOOPSIMDDIRECTIVE, "clang::OMPMaskedTaskLoopSimdDirective");
  g.add_edge((CLASS_OMPMASKEDTASKLOOPSIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPMASKEDTASKLOOPSIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPMASKEDTASKLOOPSIMDDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCISAEXPR, "clang::ObjCIsaExpr");
  g.add_edge((CLASS_OBJCISAEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCISAEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCISAEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCATFINALLYSTMT, "clang::ObjCAtFinallyStmt");
  g.add_edge((CLASS_OBJCATFINALLYSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCATFINALLYSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCATFINALLYSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPPARALLELMASTERTASKLOOPDIRECTIVE, "clang::OMPParallelMasterTaskLoopDirective");
  g.add_edge((CLASS_OMPPARALLELMASTERTASKLOOPDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPPARALLELMASTERTASKLOOPDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPPARALLELMASTERTASKLOOPDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCIVARREFEXPR, "clang::ObjCIvarRefExpr");
  g.add_edge((CLASS_OBJCIVARREFEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCIVARREFEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCIVARREFEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTASKYIELDDIRECTIVE, "clang::OMPTaskyieldDirective");
  g.add_edge((CLASS_OMPTASKYIELDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTASKYIELDDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTASKYIELDDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPITERATOREXPR, "clang::OMPIteratorExpr");
  g.add_edge((CLASS_OMPITERATOREXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPITERATOREXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPITERATOREXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPDISTRIBUTEPARALLELFORDIRECTIVE, "clang::OMPDistributeParallelForDirective");
  g.add_edge((CLASS_OMPDISTRIBUTEPARALLELFORDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPDISTRIBUTEPARALLELFORDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPDISTRIBUTEPARALLELFORDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CONSTANTEXPR, "clang::ConstantExpr");
  g.add_edge((CLASS_CONSTANTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONSTANTEXPR, META_SUBCLASS, CLASS_FULLEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967522, "clang::ConstantExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967523, "clang::ConstantExpr::getEndLoc"),
      add_named_node(&mut g, 4294967524, "clang::ConstantExpr::getResultAPValueKind"),
      add_named_node(&mut g, 4294967525, "clang::ConstantExpr::getResultStorageKind"),
      add_named_node(&mut g, 4294967526, "clang::ConstantExpr::isImmediateInvocation"),
      add_named_node(&mut g, 4294967527, "clang::ConstantExpr::hasAPValueResult"),
      add_named_node(&mut g, 4294967528, "clang::ConstantExpr::getAPValueResult"),
      add_named_node(&mut g, 4294967529, "clang::ConstantExpr::getResultAsAPSInt"),
      add_named_node(&mut g, 4294967530, "clang::ConstantExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CONSTANTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPPARALLELFORSIMDDIRECTIVE, "clang::OMPParallelForSimdDirective");
  g.add_edge((CLASS_OMPPARALLELFORSIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPPARALLELFORSIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPPARALLELFORSIMDDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXNOEXCEPTEXPR, "clang::CXXNoexceptExpr");
  g.add_edge((CLASS_CXXNOEXCEPTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXNOEXCEPTEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967533, "clang::CXXNoexceptExpr::getOperand"),
      add_named_node(&mut g, 4294967534, "clang::CXXNoexceptExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967535, "clang::CXXNoexceptExpr::getEndLoc"),
      add_named_node(&mut g, 4294967536, "clang::CXXNoexceptExpr::getSourceRange"),
      add_named_node(&mut g, 4294967537, "clang::CXXNoexceptExpr::getValue"),
      add_named_node(&mut g, 4294967538, "clang::CXXNoexceptExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXNOEXCEPTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPPARALLELMASKEDDIRECTIVE, "clang::OMPParallelMaskedDirective");
  g.add_edge((CLASS_OMPPARALLELMASKEDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPPARALLELMASKEDDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPPARALLELMASKEDDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_INITLISTEXPR, "clang::InitListExpr");
  g.add_edge((CLASS_INITLISTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_INITLISTEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967541, "clang::InitListExpr::getNumInits"),
      add_named_node(&mut g, 4294967542, "clang::InitListExpr::getInits"),
      add_named_node(&mut g, 4294967543, "clang::InitListExpr::inits"),
      add_named_node(&mut g, 4294967544, "clang::InitListExpr::getArrayFiller"),
      add_named_node(&mut g, 4294967545, "clang::InitListExpr::hasArrayFiller"),
      add_named_node(&mut g, 4294967546, "clang::InitListExpr::hasDesignatedInit"),
      add_named_node(&mut g, 4294967547, "clang::InitListExpr::getInitializedFieldInUnion"),
      add_named_node(&mut g, 4294967548, "clang::InitListExpr::isExplicit"),
      add_named_node(&mut g, 4294967549, "clang::InitListExpr::isStringLiteralInit"),
      add_named_node(&mut g, 4294967550, "clang::InitListExpr::isTransparent"),
      add_named_node(&mut g, 4294967551, "clang::InitListExpr::getLBraceLoc"),
      add_named_node(&mut g, 4294967552, "clang::InitListExpr::getRBraceLoc"),
      add_named_node(&mut g, 4294967553, "clang::InitListExpr::isSemanticForm"),
      add_named_node(&mut g, 4294967554, "clang::InitListExpr::getSemanticForm"),
      add_named_node(&mut g, 4294967555, "clang::InitListExpr::isSyntacticForm"),
      add_named_node(&mut g, 4294967556, "clang::InitListExpr::getSyntacticForm"),
      add_named_node(&mut g, 4294967557, "clang::InitListExpr::hadArrayRangeDesignator"),
      add_named_node(&mut g, 4294967558, "clang::InitListExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967559, "clang::InitListExpr::getEndLoc"),
      add_named_node(&mut g, 4294967560, "clang::InitListExpr::children"),
      add_named_node(&mut g, 4294967561, "clang::InitListExpr::begin"),
      add_named_node(&mut g, 4294967562, "clang::InitListExpr::end"),
      add_named_node(&mut g, 4294967563, "clang::InitListExpr::rbegin"),
      add_named_node(&mut g, 4294967564, "clang::InitListExpr::rend"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_INITLISTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTARGETTEAMSGENERICLOOPDIRECTIVE, "clang::OMPTargetTeamsGenericLoopDirective");
  g.add_edge((CLASS_OMPTARGETTEAMSGENERICLOOPDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETTEAMSGENERICLOOPDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTARGETTEAMSGENERICLOOPDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTARGETDATADIRECTIVE, "clang::OMPTargetDataDirective");
  g.add_edge((CLASS_OMPTARGETDATADIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETDATADIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTARGETDATADIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTASKGROUPDIRECTIVE, "clang::OMPTaskgroupDirective");
  g.add_edge((CLASS_OMPTASKGROUPDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTASKGROUPDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTASKGROUPDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTARGETTEAMSDIRECTIVE, "clang::OMPTargetTeamsDirective");
  g.add_edge((CLASS_OMPTARGETTEAMSDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETTEAMSDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTARGETTEAMSDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPSCANDIRECTIVE, "clang::OMPScanDirective");
  g.add_edge((CLASS_OMPSCANDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPSCANDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPSCANDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPMETADIRECTIVE, "clang::OMPMetaDirective");
  g.add_edge((CLASS_OMPMETADIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPMETADIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPMETADIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPATOMICDIRECTIVE, "clang::OMPAtomicDirective");
  g.add_edge((CLASS_OMPATOMICDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPATOMICDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPATOMICDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_COMPOUNDSTMT, "clang::CompoundStmt");
  g.add_edge((CLASS_COMPOUNDSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_COMPOUNDSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967573, "clang::CompoundStmt::body_empty"),
      add_named_node(&mut g, 4294967574, "clang::CompoundStmt::size"),
      add_named_node(&mut g, 4294967575, "clang::CompoundStmt::hasStoredFPFeatures"),
      add_named_node(&mut g, 4294967576, "clang::CompoundStmt::getStoredFPFeatures"),
      add_named_node(&mut g, 4294967577, "clang::CompoundStmt::body"),
      add_named_node(&mut g, 4294967578, "clang::CompoundStmt::body_begin"),
      add_named_node(&mut g, 4294967579, "clang::CompoundStmt::body_end"),
      add_named_node(&mut g, 4294967580, "clang::CompoundStmt::body_front"),
      add_named_node(&mut g, 4294967581, "clang::CompoundStmt::body_back"),
      add_named_node(&mut g, 4294967582, "clang::CompoundStmt::body_rbegin"),
      add_named_node(&mut g, 4294967583, "clang::CompoundStmt::body_rend"),
      add_named_node(&mut g, 4294967584, "clang::CompoundStmt::getStmtExprResult"),
      add_named_node(&mut g, 4294967585, "clang::CompoundStmt::getBeginLoc"),
      add_named_node(&mut g, 4294967586, "clang::CompoundStmt::getEndLoc"),
      add_named_node(&mut g, 4294967587, "clang::CompoundStmt::getLBracLoc"),
      add_named_node(&mut g, 4294967588, "clang::CompoundStmt::getRBracLoc"),
      add_named_node(&mut g, 4294967589, "clang::CompoundStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_COMPOUNDSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPDISTRIBUTEDIRECTIVE, "clang::OMPDistributeDirective");
  g.add_edge((CLASS_OMPDISTRIBUTEDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPDISTRIBUTEDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPDISTRIBUTEDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPDISTRIBUTEPARALLELFORSIMDDIRECTIVE, "clang::OMPDistributeParallelForSimdDirective");
  g.add_edge((CLASS_OMPDISTRIBUTEPARALLELFORSIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPDISTRIBUTEPARALLELFORSIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPDISTRIBUTEPARALLELFORSIMDDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPGENERICLOOPDIRECTIVE, "clang::OMPGenericLoopDirective");
  g.add_edge((CLASS_OMPGENERICLOOPDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPGENERICLOOPDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPGENERICLOOPDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CAPTUREDSTMT, "clang::CapturedStmt");
  g.add_edge((CLASS_CAPTUREDSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CAPTUREDSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967594, "clang::CapturedStmt::getCapturedStmt"),
      add_named_node(&mut g, 4294967595, "clang::CapturedStmt::getCapturedDecl"),
      add_named_node(&mut g, 4294967596, "clang::CapturedStmt::getCapturedRegionKind"),
      add_named_node(&mut g, 4294967597, "clang::CapturedStmt::getCapturedRecordDecl"),
      add_named_node(&mut g, 4294967598, "clang::CapturedStmt::captures"),
      add_named_node(&mut g, 4294967599, "clang::CapturedStmt::capture_begin"),
      add_named_node(&mut g, 4294967600, "clang::CapturedStmt::capture_end"),
      add_named_node(&mut g, 4294967601, "clang::CapturedStmt::capture_size"),
      add_named_node(&mut g, 4294967602, "clang::CapturedStmt::capture_inits"),
      add_named_node(&mut g, 4294967603, "clang::CapturedStmt::capture_init_begin"),
      add_named_node(&mut g, 4294967604, "clang::CapturedStmt::capture_init_end"),
      add_named_node(&mut g, 4294967605, "clang::CapturedStmt::getBeginLoc"),
      add_named_node(&mut g, 4294967606, "clang::CapturedStmt::getEndLoc"),
      add_named_node(&mut g, 4294967607, "clang::CapturedStmt::getSourceRange"),
      add_named_node(&mut g, 4294967608, "clang::CapturedStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CAPTUREDSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_COYIELDEXPR, "clang::CoyieldExpr");
  g.add_edge((CLASS_COYIELDEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_COYIELDEXPR, META_SUBCLASS, CLASS_COROUTINESUSPENDEXPR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_COYIELDEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCATTHROWSTMT, "clang::ObjCAtThrowStmt");
  g.add_edge((CLASS_OBJCATTHROWSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCATTHROWSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCATTHROWSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTARGETEXITDATADIRECTIVE, "clang::OMPTargetExitDataDirective");
  g.add_edge((CLASS_OMPTARGETEXITDATADIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETEXITDATADIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTARGETEXITDATADIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTARGETENTERDATADIRECTIVE, "clang::OMPTargetEnterDataDirective");
  g.add_edge((CLASS_OMPTARGETENTERDATADIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETENTERDATADIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTARGETENTERDATADIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTARGETTEAMSDISTRIBUTEPARALLELFORDIRECTIVE, "clang::OMPTargetTeamsDistributeParallelForDirective");
  g.add_edge((CLASS_OMPTARGETTEAMSDISTRIBUTEPARALLELFORDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETTEAMSDISTRIBUTEPARALLELFORDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTARGETTEAMSDISTRIBUTEPARALLELFORDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPEXECUTABLEDIRECTIVE, "clang::OMPExecutableDirective");
  g.add_edge((CLASS_OMPEXECUTABLEDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPEXECUTABLEDIRECTIVE, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPEXECUTABLEDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPMASTERTASKLOOPDIRECTIVE, "clang::OMPMasterTaskLoopDirective");
  g.add_edge((CLASS_OMPMASTERTASKLOOPDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPMASTERTASKLOOPDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPMASTERTASKLOOPDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CHOOSEEXPR, "clang::ChooseExpr");
  g.add_edge((CLASS_CHOOSEEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CHOOSEEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967617, "clang::ChooseExpr::isConditionTrue"),
      add_named_node(&mut g, 4294967618, "clang::ChooseExpr::isConditionDependent"),
      add_named_node(&mut g, 4294967619, "clang::ChooseExpr::getChosenSubExpr"),
      add_named_node(&mut g, 4294967620, "clang::ChooseExpr::getCond"),
      add_named_node(&mut g, 4294967621, "clang::ChooseExpr::getLHS"),
      add_named_node(&mut g, 4294967622, "clang::ChooseExpr::getRHS"),
      add_named_node(&mut g, 4294967623, "clang::ChooseExpr::getBuiltinLoc"),
      add_named_node(&mut g, 4294967624, "clang::ChooseExpr::getRParenLoc"),
      add_named_node(&mut g, 4294967625, "clang::ChooseExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967626, "clang::ChooseExpr::getEndLoc"),
      add_named_node(&mut g, 4294967627, "clang::ChooseExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CHOOSEEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_EXPLICITCASTEXPR, "clang::ExplicitCastExpr");
  g.add_edge((CLASS_EXPLICITCASTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_EXPLICITCASTEXPR, META_SUBCLASS, CLASS_CASTEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967629, "clang::ExplicitCastExpr::getTypeInfoAsWritten"),
      add_named_node(&mut g, 4294967630, "clang::ExplicitCastExpr::getTypeAsWritten"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_EXPLICITCASTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPPARALLELGENERICLOOPDIRECTIVE, "clang::OMPParallelGenericLoopDirective");
  g.add_edge((CLASS_OMPPARALLELGENERICLOOPDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPPARALLELGENERICLOOPDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPPARALLELGENERICLOOPDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTEAMSGENERICLOOPDIRECTIVE, "clang::OMPTeamsGenericLoopDirective");
  g.add_edge((CLASS_OMPTEAMSGENERICLOOPDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTEAMSGENERICLOOPDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTEAMSGENERICLOOPDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPMASKEDDIRECTIVE, "clang::OMPMaskedDirective");
  g.add_edge((CLASS_OMPMASKEDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPMASKEDDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPMASKEDDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXUNRESOLVEDCONSTRUCTEXPR, "clang::CXXUnresolvedConstructExpr");
  g.add_edge((CLASS_CXXUNRESOLVEDCONSTRUCTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXUNRESOLVEDCONSTRUCTEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967635, "clang::CXXUnresolvedConstructExpr::getTypeAsWritten"),
      add_named_node(&mut g, 4294967636, "clang::CXXUnresolvedConstructExpr::getTypeSourceInfo"),
      add_named_node(&mut g, 4294967637, "clang::CXXUnresolvedConstructExpr::getLParenLoc"),
      add_named_node(&mut g, 4294967638, "clang::CXXUnresolvedConstructExpr::getRParenLoc"),
      add_named_node(&mut g, 4294967639, "clang::CXXUnresolvedConstructExpr::isListInitialization"),
      add_named_node(&mut g, 4294967640, "clang::CXXUnresolvedConstructExpr::getNumArgs"),
      add_named_node(&mut g, 4294967641, "clang::CXXUnresolvedConstructExpr::arg_begin"),
      add_named_node(&mut g, 4294967642, "clang::CXXUnresolvedConstructExpr::arg_end"),
      add_named_node(&mut g, 4294967643, "clang::CXXUnresolvedConstructExpr::arguments"),
      add_named_node(&mut g, 4294967644, "clang::CXXUnresolvedConstructExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967645, "clang::CXXUnresolvedConstructExpr::getEndLoc"),
      add_named_node(&mut g, 4294967646, "clang::CXXUnresolvedConstructExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXUNRESOLVEDCONSTRUCTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_USERDEFINEDLITERAL, "clang::UserDefinedLiteral");
  g.add_edge((CLASS_USERDEFINEDLITERAL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_USERDEFINEDLITERAL, META_SUBCLASS, CLASS_CALLEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967648, "clang::UserDefinedLiteral::getLiteralOperatorKind"),
      add_named_node(&mut g, 4294967649, "clang::UserDefinedLiteral::getCookedLiteral"),
      add_named_node(&mut g, 4294967650, "clang::UserDefinedLiteral::getBeginLoc"),
      add_named_node(&mut g, 4294967651, "clang::UserDefinedLiteral::getEndLoc"),
      add_named_node(&mut g, 4294967652, "clang::UserDefinedLiteral::getUDSuffixLoc"),
      add_named_node(&mut g, 4294967653, "clang::UserDefinedLiteral::getUDSuffix"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_USERDEFINEDLITERAL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCATTRYSTMT, "clang::ObjCAtTryStmt");
  g.add_edge((CLASS_OBJCATTRYSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCATTRYSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCATTRYSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTARGETTEAMSDISTRIBUTEPARALLELFORSIMDDIRECTIVE, "clang::OMPTargetTeamsDistributeParallelForSimdDirective");
  g.add_edge((CLASS_OMPTARGETTEAMSDISTRIBUTEPARALLELFORSIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETTEAMSDISTRIBUTEPARALLELFORSIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTARGETTEAMSDISTRIBUTEPARALLELFORSIMDDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXADDRSPACECASTEXPR, "clang::CXXAddrspaceCastExpr");
  g.add_edge((CLASS_CXXADDRSPACECASTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXADDRSPACECASTEXPR, META_SUBCLASS, CLASS_CXXNAMEDCASTEXPR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXADDRSPACECASTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FUNCTIONPARMPACKEXPR, "clang::FunctionParmPackExpr");
  g.add_edge((CLASS_FUNCTIONPARMPACKEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FUNCTIONPARMPACKEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967658, "clang::FunctionParmPackExpr::getParameterPack"),
      add_named_node(&mut g, 4294967659, "clang::FunctionParmPackExpr::getParameterPackLocation"),
      add_named_node(&mut g, 4294967660, "clang::FunctionParmPackExpr::begin"),
      add_named_node(&mut g, 4294967661, "clang::FunctionParmPackExpr::end"),
      add_named_node(&mut g, 4294967662, "clang::FunctionParmPackExpr::getNumExpansions"),
      add_named_node(&mut g, 4294967663, "clang::FunctionParmPackExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967664, "clang::FunctionParmPackExpr::getEndLoc"),
      add_named_node(&mut g, 4294967665, "clang::FunctionParmPackExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FUNCTIONPARMPACKEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DECLREFEXPR, "clang::DeclRefExpr");
  g.add_edge((CLASS_DECLREFEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DECLREFEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967667, "clang::DeclRefExpr::getDecl"),
      add_named_node(&mut g, 4294967668, "clang::DeclRefExpr::getNameInfo"),
      add_named_node(&mut g, 4294967669, "clang::DeclRefExpr::getLocation"),
      add_named_node(&mut g, 4294967670, "clang::DeclRefExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967671, "clang::DeclRefExpr::getEndLoc"),
      add_named_node(&mut g, 4294967672, "clang::DeclRefExpr::hasQualifier"),
      add_named_node(&mut g, 4294967673, "clang::DeclRefExpr::getQualifierLoc"),
      add_named_node(&mut g, 4294967674, "clang::DeclRefExpr::getQualifier"),
      add_named_node(&mut g, 4294967675, "clang::DeclRefExpr::getFoundDecl"),
      add_named_node(&mut g, 4294967676, "clang::DeclRefExpr::hasTemplateKWAndArgsInfo"),
      add_named_node(&mut g, 4294967677, "clang::DeclRefExpr::getTemplateKeywordLoc"),
      add_named_node(&mut g, 4294967678, "clang::DeclRefExpr::getLAngleLoc"),
      add_named_node(&mut g, 4294967679, "clang::DeclRefExpr::getRAngleLoc"),
      add_named_node(&mut g, 4294967680, "clang::DeclRefExpr::hasTemplateKeyword"),
      add_named_node(&mut g, 4294967681, "clang::DeclRefExpr::hasExplicitTemplateArgs"),
      add_named_node(&mut g, 4294967682, "clang::DeclRefExpr::getTemplateArgs"),
      add_named_node(&mut g, 4294967683, "clang::DeclRefExpr::getNumTemplateArgs"),
      add_named_node(&mut g, 4294967684, "clang::DeclRefExpr::template_arguments"),
      add_named_node(&mut g, 4294967685, "clang::DeclRefExpr::hadMultipleCandidates"),
      add_named_node(&mut g, 4294967686, "clang::DeclRefExpr::isNonOdrUse"),
      add_named_node(&mut g, 4294967687, "clang::DeclRefExpr::refersToEnclosingVariableOrCapture"),
      add_named_node(&mut g, 4294967688, "clang::DeclRefExpr::isImmediateEscalating"),
      add_named_node(&mut g, 4294967689, "clang::DeclRefExpr::isCapturedByCopyInLambdaWithExplicitObjectParameter"),
      add_named_node(&mut g, 4294967690, "clang::DeclRefExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DECLREFEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTEAMSDISTRIBUTEPARALLELFORSIMDDIRECTIVE, "clang::OMPTeamsDistributeParallelForSimdDirective");
  g.add_edge((CLASS_OMPTEAMSDISTRIBUTEPARALLELFORSIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTEAMSDISTRIBUTEPARALLELFORSIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTEAMSDISTRIBUTEPARALLELFORSIMDDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPARRAYSHAPINGEXPR, "clang::OMPArrayShapingExpr");
  g.add_edge((CLASS_OMPARRAYSHAPINGEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPARRAYSHAPINGEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPARRAYSHAPINGEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_UNRESOLVEDLOOKUPEXPR, "clang::UnresolvedLookupExpr");
  g.add_edge((CLASS_UNRESOLVEDLOOKUPEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNRESOLVEDLOOKUPEXPR, META_SUBCLASS, CLASS_OVERLOADEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967694, "clang::UnresolvedLookupExpr::requiresADL"),
      add_named_node(&mut g, 4294967695, "clang::UnresolvedLookupExpr::isOverloaded"),
      add_named_node(&mut g, 4294967696, "clang::UnresolvedLookupExpr::getNamingClass"),
      add_named_node(&mut g, 4294967697, "clang::UnresolvedLookupExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967698, "clang::UnresolvedLookupExpr::getEndLoc"),
      add_named_node(&mut g, 4294967699, "clang::UnresolvedLookupExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNRESOLVEDLOOKUPEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PSEUDOOBJECTEXPR, "clang::PseudoObjectExpr");
  g.add_edge((CLASS_PSEUDOOBJECTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PSEUDOOBJECTEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967701, "clang::PseudoObjectExpr::getSyntacticForm"),
      add_named_node(&mut g, 4294967702, "clang::PseudoObjectExpr::getResultExprIndex"),
      add_named_node(&mut g, 4294967703, "clang::PseudoObjectExpr::getResultExpr"),
      add_named_node(&mut g, 4294967704, "clang::PseudoObjectExpr::getNumSemanticExprs"),
      add_named_node(&mut g, 4294967705, "clang::PseudoObjectExpr::semantics_begin"),
      add_named_node(&mut g, 4294967706, "clang::PseudoObjectExpr::semantics_end"),
      add_named_node(&mut g, 4294967707, "clang::PseudoObjectExpr::semantics"),
      add_named_node(&mut g, 4294967708, "clang::PseudoObjectExpr::getExprLoc"),
      add_named_node(&mut g, 4294967709, "clang::PseudoObjectExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967710, "clang::PseudoObjectExpr::getEndLoc"),
      add_named_node(&mut g, 4294967711, "clang::PseudoObjectExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PSEUDOOBJECTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PARENEXPR, "clang::ParenExpr");
  g.add_edge((CLASS_PARENEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PARENEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967713, "clang::ParenExpr::getSubExpr"),
      add_named_node(&mut g, 4294967714, "clang::ParenExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967715, "clang::ParenExpr::getEndLoc"),
      add_named_node(&mut g, 4294967716, "clang::ParenExpr::getLParen"),
      add_named_node(&mut g, 4294967717, "clang::ParenExpr::getRParen"),
      add_named_node(&mut g, 4294967718, "clang::ParenExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PARENEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTEAMSDISTRIBUTESIMDDIRECTIVE, "clang::OMPTeamsDistributeSimdDirective");
  g.add_edge((CLASS_OMPTEAMSDISTRIBUTESIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTEAMSDISTRIBUTESIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTEAMSDISTRIBUTESIMDDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ABSTRACTCONDITIONALOPERATOR, "clang::AbstractConditionalOperator");
  g.add_edge((CLASS_ABSTRACTCONDITIONALOPERATOR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ABSTRACTCONDITIONALOPERATOR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967721, "clang::AbstractConditionalOperator::getCond"),
      add_named_node(&mut g, 4294967722, "clang::AbstractConditionalOperator::getTrueExpr"),
      add_named_node(&mut g, 4294967723, "clang::AbstractConditionalOperator::getFalseExpr"),
      add_named_node(&mut g, 4294967724, "clang::AbstractConditionalOperator::getQuestionLoc"),
      add_named_node(&mut g, 4294967725, "clang::AbstractConditionalOperator::getColonLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ABSTRACTCONDITIONALOPERATOR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BINARYCONDITIONALOPERATOR, "clang::BinaryConditionalOperator");
  g.add_edge((CLASS_BINARYCONDITIONALOPERATOR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BINARYCONDITIONALOPERATOR, META_SUBCLASS, CLASS_ABSTRACTCONDITIONALOPERATOR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967727, "clang::BinaryConditionalOperator::getCommon"),
      add_named_node(&mut g, 4294967728, "clang::BinaryConditionalOperator::getOpaqueValue"),
      add_named_node(&mut g, 4294967729, "clang::BinaryConditionalOperator::getCond"),
      add_named_node(&mut g, 4294967730, "clang::BinaryConditionalOperator::getTrueExpr"),
      add_named_node(&mut g, 4294967731, "clang::BinaryConditionalOperator::getFalseExpr"),
      add_named_node(&mut g, 4294967732, "clang::BinaryConditionalOperator::getBeginLoc"),
      add_named_node(&mut g, 4294967733, "clang::BinaryConditionalOperator::getEndLoc"),
      add_named_node(&mut g, 4294967734, "clang::BinaryConditionalOperator::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BINARYCONDITIONALOPERATOR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPBARRIERDIRECTIVE, "clang::OMPBarrierDirective");
  g.add_edge((CLASS_OMPBARRIERDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPBARRIERDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPBARRIERDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTASKDIRECTIVE, "clang::OMPTaskDirective");
  g.add_edge((CLASS_OMPTASKDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTASKDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTASKDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SHUFFLEVECTOREXPR, "clang::ShuffleVectorExpr");
  g.add_edge((CLASS_SHUFFLEVECTOREXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SHUFFLEVECTOREXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967738, "clang::ShuffleVectorExpr::getBuiltinLoc"),
      add_named_node(&mut g, 4294967739, "clang::ShuffleVectorExpr::getRParenLoc"),
      add_named_node(&mut g, 4294967740, "clang::ShuffleVectorExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967741, "clang::ShuffleVectorExpr::getEndLoc"),
      add_named_node(&mut g, 4294967742, "clang::ShuffleVectorExpr::getNumSubExprs"),
      add_named_node(&mut g, 4294967743, "clang::ShuffleVectorExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SHUFFLEVECTOREXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPFORDIRECTIVE, "clang::OMPForDirective");
  g.add_edge((CLASS_OMPFORDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPFORDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPFORDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CONCEPTSPECIALIZATIONEXPR, "clang::ConceptSpecializationExpr");
  g.add_edge((CLASS_CONCEPTSPECIALIZATIONEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONCEPTSPECIALIZATIONEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967746, "clang::ConceptSpecializationExpr::getTemplateArguments"),
      add_named_node(&mut g, 4294967747, "clang::ConceptSpecializationExpr::getConceptReference"),
      add_named_node(&mut g, 4294967748, "clang::ConceptSpecializationExpr::getNamedConcept"),
      add_named_node(&mut g, 4294967749, "clang::ConceptSpecializationExpr::hasExplicitTemplateArgs"),
      add_named_node(&mut g, 4294967750, "clang::ConceptSpecializationExpr::getConceptNameLoc"),
      add_named_node(&mut g, 4294967751, "clang::ConceptSpecializationExpr::getTemplateArgsAsWritten"),
      add_named_node(&mut g, 4294967752, "clang::ConceptSpecializationExpr::getNestedNameSpecifierLoc"),
      add_named_node(&mut g, 4294967753, "clang::ConceptSpecializationExpr::getTemplateKWLoc"),
      add_named_node(&mut g, 4294967754, "clang::ConceptSpecializationExpr::getFoundDecl"),
      add_named_node(&mut g, 4294967755, "clang::ConceptSpecializationExpr::getConceptNameInfo"),
      add_named_node(&mut g, 4294967756, "clang::ConceptSpecializationExpr::getSpecializationDecl"),
      add_named_node(&mut g, 4294967757, "clang::ConceptSpecializationExpr::isSatisfied"),
      add_named_node(&mut g, 4294967758, "clang::ConceptSpecializationExpr::getSatisfaction"),
      add_named_node(&mut g, 4294967759, "clang::ConceptSpecializationExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967760, "clang::ConceptSpecializationExpr::getEndLoc"),
      add_named_node(&mut g, 4294967761, "clang::ConceptSpecializationExpr::getExprLoc"),
      add_named_node(&mut g, 4294967762, "clang::ConceptSpecializationExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CONCEPTSPECIALIZATIONEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPPARALLELMASKEDTASKLOOPSIMDDIRECTIVE, "clang::OMPParallelMaskedTaskLoopSimdDirective");
  g.add_edge((CLASS_OMPPARALLELMASKEDTASKLOOPSIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPPARALLELMASKEDTASKLOOPSIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPPARALLELMASKEDTASKLOOPSIMDDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_LABELSTMT, "clang::LabelStmt");
  g.add_edge((CLASS_LABELSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LABELSTMT, META_SUBCLASS, CLASS_VALUESTMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967765, "clang::LabelStmt::getIdentLoc"),
      add_named_node(&mut g, 4294967766, "clang::LabelStmt::getDecl"),
      add_named_node(&mut g, 4294967767, "clang::LabelStmt::getName"),
      add_named_node(&mut g, 4294967768, "clang::LabelStmt::getSubStmt"),
      add_named_node(&mut g, 4294967769, "clang::LabelStmt::getBeginLoc"),
      add_named_node(&mut g, 4294967770, "clang::LabelStmt::getEndLoc"),
      add_named_node(&mut g, 4294967771, "clang::LabelStmt::children"),
      add_named_node(&mut g, 4294967772, "clang::LabelStmt::isSideEntry"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_LABELSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_GNUNULLEXPR, "clang::GNUNullExpr");
  g.add_edge((CLASS_GNUNULLEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_GNUNULLEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967774, "clang::GNUNullExpr::getTokenLocation"),
      add_named_node(&mut g, 4294967775, "clang::GNUNullExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967776, "clang::GNUNullExpr::getEndLoc"),
      add_named_node(&mut g, 4294967777, "clang::GNUNullExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_GNUNULLEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCBOOLLITERALEXPR, "clang::ObjCBoolLiteralExpr");
  g.add_edge((CLASS_OBJCBOOLLITERALEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCBOOLLITERALEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCBOOLLITERALEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPINTEROPDIRECTIVE, "clang::OMPInteropDirective");
  g.add_edge((CLASS_OMPINTEROPDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPINTEROPDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPINTEROPDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_REQUIRESEXPR, "clang::RequiresExpr");
  g.add_edge((CLASS_REQUIRESEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_REQUIRESEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967781, "clang::RequiresExpr::getLocalParameters"),
      add_named_node(&mut g, 4294967782, "clang::RequiresExpr::getBody"),
      add_named_node(&mut g, 4294967783, "clang::RequiresExpr::getRequirements"),
      add_named_node(&mut g, 4294967784, "clang::RequiresExpr::isSatisfied"),
      add_named_node(&mut g, 4294967785, "clang::RequiresExpr::getRequiresKWLoc"),
      add_named_node(&mut g, 4294967786, "clang::RequiresExpr::getLParenLoc"),
      add_named_node(&mut g, 4294967787, "clang::RequiresExpr::getRParenLoc"),
      add_named_node(&mut g, 4294967788, "clang::RequiresExpr::getRBraceLoc"),
      add_named_node(&mut g, 4294967789, "clang::RequiresExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967790, "clang::RequiresExpr::getEndLoc"),
      add_named_node(&mut g, 4294967791, "clang::RequiresExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_REQUIRESEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_COAWAITEXPR, "clang::CoawaitExpr");
  g.add_edge((CLASS_COAWAITEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_COAWAITEXPR, META_SUBCLASS, CLASS_COROUTINESUSPENDEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967793, "clang::CoawaitExpr::isImplicit"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_COAWAITEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MATERIALIZETEMPORARYEXPR, "clang::MaterializeTemporaryExpr");
  g.add_edge((CLASS_MATERIALIZETEMPORARYEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MATERIALIZETEMPORARYEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967795, "clang::MaterializeTemporaryExpr::getSubExpr"),
      add_named_node(&mut g, 4294967796, "clang::MaterializeTemporaryExpr::getStorageDuration"),
      add_named_node(&mut g, 4294967797, "clang::MaterializeTemporaryExpr::getLifetimeExtendedTemporaryDecl"),
      add_named_node(&mut g, 4294967798, "clang::MaterializeTemporaryExpr::getExtendingDecl"),
      add_named_node(&mut g, 4294967799, "clang::MaterializeTemporaryExpr::getManglingNumber"),
      add_named_node(&mut g, 4294967800, "clang::MaterializeTemporaryExpr::isBoundToLvalueReference"),
      add_named_node(&mut g, 4294967801, "clang::MaterializeTemporaryExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967802, "clang::MaterializeTemporaryExpr::getEndLoc"),
      add_named_node(&mut g, 4294967803, "clang::MaterializeTemporaryExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MATERIALIZETEMPORARYEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTARGETDIRECTIVE, "clang::OMPTargetDirective");
  g.add_edge((CLASS_OMPTARGETDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTARGETDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PACKEXPANSIONEXPR, "clang::PackExpansionExpr");
  g.add_edge((CLASS_PACKEXPANSIONEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PACKEXPANSIONEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967806, "clang::PackExpansionExpr::getPattern"),
      add_named_node(&mut g, 4294967807, "clang::PackExpansionExpr::getEllipsisLoc"),
      add_named_node(&mut g, 4294967808, "clang::PackExpansionExpr::getNumExpansions"),
      add_named_node(&mut g, 4294967809, "clang::PackExpansionExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967810, "clang::PackExpansionExpr::getEndLoc"),
      add_named_node(&mut g, 4294967811, "clang::PackExpansionExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PACKEXPANSIONEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXTHISEXPR, "clang::CXXThisExpr");
  g.add_edge((CLASS_CXXTHISEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXTHISEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967813, "clang::CXXThisExpr::getLocation"),
      add_named_node(&mut g, 4294967814, "clang::CXXThisExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967815, "clang::CXXThisExpr::getEndLoc"),
      add_named_node(&mut g, 4294967816, "clang::CXXThisExpr::isImplicit"),
      add_named_node(&mut g, 4294967817, "clang::CXXThisExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXTHISEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXDEPENDENTSCOPEMEMBEREXPR, "clang::CXXDependentScopeMemberExpr");
  g.add_edge((CLASS_CXXDEPENDENTSCOPEMEMBEREXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXDEPENDENTSCOPEMEMBEREXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967819, "clang::CXXDependentScopeMemberExpr::isImplicitAccess"),
      add_named_node(&mut g, 4294967820, "clang::CXXDependentScopeMemberExpr::getBase"),
      add_named_node(&mut g, 4294967821, "clang::CXXDependentScopeMemberExpr::getBaseType"),
      add_named_node(&mut g, 4294967822, "clang::CXXDependentScopeMemberExpr::isArrow"),
      add_named_node(&mut g, 4294967823, "clang::CXXDependentScopeMemberExpr::getOperatorLoc"),
      add_named_node(&mut g, 4294967824, "clang::CXXDependentScopeMemberExpr::getQualifier"),
      add_named_node(&mut g, 4294967825, "clang::CXXDependentScopeMemberExpr::getQualifierLoc"),
      add_named_node(&mut g, 4294967826, "clang::CXXDependentScopeMemberExpr::getFirstQualifierFoundInScope"),
      add_named_node(&mut g, 4294967827, "clang::CXXDependentScopeMemberExpr::getMemberNameInfo"),
      add_named_node(&mut g, 4294967828, "clang::CXXDependentScopeMemberExpr::getMember"),
      add_named_node(&mut g, 4294967829, "clang::CXXDependentScopeMemberExpr::getMemberLoc"),
      add_named_node(&mut g, 4294967830, "clang::CXXDependentScopeMemberExpr::getTemplateKeywordLoc"),
      add_named_node(&mut g, 4294967831, "clang::CXXDependentScopeMemberExpr::getLAngleLoc"),
      add_named_node(&mut g, 4294967832, "clang::CXXDependentScopeMemberExpr::getRAngleLoc"),
      add_named_node(&mut g, 4294967833, "clang::CXXDependentScopeMemberExpr::hasTemplateKeyword"),
      add_named_node(&mut g, 4294967834, "clang::CXXDependentScopeMemberExpr::hasExplicitTemplateArgs"),
      add_named_node(&mut g, 4294967835, "clang::CXXDependentScopeMemberExpr::getTemplateArgs"),
      add_named_node(&mut g, 4294967836, "clang::CXXDependentScopeMemberExpr::getNumTemplateArgs"),
      add_named_node(&mut g, 4294967837, "clang::CXXDependentScopeMemberExpr::template_arguments"),
      add_named_node(&mut g, 4294967838, "clang::CXXDependentScopeMemberExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967839, "clang::CXXDependentScopeMemberExpr::getEndLoc"),
      add_named_node(&mut g, 4294967840, "clang::CXXDependentScopeMemberExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXDEPENDENTSCOPEMEMBEREXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPETRAITEXPR, "clang::TypeTraitExpr");
  g.add_edge((CLASS_TYPETRAITEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPETRAITEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967842, "clang::TypeTraitExpr::getTrait"),
      add_named_node(&mut g, 4294967843, "clang::TypeTraitExpr::getValue"),
      add_named_node(&mut g, 4294967844, "clang::TypeTraitExpr::getNumArgs"),
      add_named_node(&mut g, 4294967845, "clang::TypeTraitExpr::getArgs"),
      add_named_node(&mut g, 4294967846, "clang::TypeTraitExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967847, "clang::TypeTraitExpr::getEndLoc"),
      add_named_node(&mut g, 4294967848, "clang::TypeTraitExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPETRAITEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXNEWEXPR, "clang::CXXNewExpr");
  g.add_edge((CLASS_CXXNEWEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXNEWEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967850, "clang::CXXNewExpr::getAllocatedType"),
      add_named_node(&mut g, 4294967851, "clang::CXXNewExpr::getAllocatedTypeSourceInfo"),
      add_named_node(&mut g, 4294967852, "clang::CXXNewExpr::shouldNullCheckAllocation"),
      add_named_node(&mut g, 4294967853, "clang::CXXNewExpr::getOperatorNew"),
      add_named_node(&mut g, 4294967854, "clang::CXXNewExpr::getOperatorDelete"),
      add_named_node(&mut g, 4294967855, "clang::CXXNewExpr::isArray"),
      add_named_node(&mut g, 4294967856, "clang::CXXNewExpr::getArraySize"),
      add_named_node(&mut g, 4294967857, "clang::CXXNewExpr::getNumPlacementArgs"),
      add_named_node(&mut g, 4294967858, "clang::CXXNewExpr::isParenTypeId"),
      add_named_node(&mut g, 4294967859, "clang::CXXNewExpr::getTypeIdParens"),
      add_named_node(&mut g, 4294967860, "clang::CXXNewExpr::isGlobalNew"),
      add_named_node(&mut g, 4294967861, "clang::CXXNewExpr::hasInitializer"),
      add_named_node(&mut g, 4294967862, "clang::CXXNewExpr::getInitializationStyle"),
      add_named_node(&mut g, 4294967863, "clang::CXXNewExpr::getInitializer"),
      add_named_node(&mut g, 4294967864, "clang::CXXNewExpr::getConstructExpr"),
      add_named_node(&mut g, 4294967865, "clang::CXXNewExpr::passAlignment"),
      add_named_node(&mut g, 4294967866, "clang::CXXNewExpr::doesUsualArrayDeleteWantSize"),
      add_named_node(&mut g, 4294967867, "clang::CXXNewExpr::placement_arguments"),
      add_named_node(&mut g, 4294967868, "clang::CXXNewExpr::placement_arg_begin"),
      add_named_node(&mut g, 4294967869, "clang::CXXNewExpr::placement_arg_end"),
      add_named_node(&mut g, 4294967870, "clang::CXXNewExpr::raw_arg_begin"),
      add_named_node(&mut g, 4294967871, "clang::CXXNewExpr::raw_arg_end"),
      add_named_node(&mut g, 4294967872, "clang::CXXNewExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967873, "clang::CXXNewExpr::getEndLoc"),
      add_named_node(&mut g, 4294967874, "clang::CXXNewExpr::getDirectInitRange"),
      add_named_node(&mut g, 4294967875, "clang::CXXNewExpr::getSourceRange"),
      add_named_node(&mut g, 4294967876, "clang::CXXNewExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXNEWEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXINHERITEDCTORINITEXPR, "clang::CXXInheritedCtorInitExpr");
  g.add_edge((CLASS_CXXINHERITEDCTORINITEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXINHERITEDCTORINITEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967878, "clang::CXXInheritedCtorInitExpr::getConstructor"),
      add_named_node(&mut g, 4294967879, "clang::CXXInheritedCtorInitExpr::constructsVBase"),
      add_named_node(&mut g, 4294967880, "clang::CXXInheritedCtorInitExpr::getConstructionKind"),
      add_named_node(&mut g, 4294967881, "clang::CXXInheritedCtorInitExpr::inheritedFromVBase"),
      add_named_node(&mut g, 4294967882, "clang::CXXInheritedCtorInitExpr::getLocation"),
      add_named_node(&mut g, 4294967883, "clang::CXXInheritedCtorInitExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967884, "clang::CXXInheritedCtorInitExpr::getEndLoc"),
      add_named_node(&mut g, 4294967885, "clang::CXXInheritedCtorInitExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXINHERITEDCTORINITEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_IMPLICITCASTEXPR, "clang::ImplicitCastExpr");
  g.add_edge((CLASS_IMPLICITCASTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_IMPLICITCASTEXPR, META_SUBCLASS, CLASS_CASTEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967887, "clang::ImplicitCastExpr::isPartOfExplicitCast"),
      add_named_node(&mut g, 4294967888, "clang::ImplicitCastExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967889, "clang::ImplicitCastExpr::getEndLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_IMPLICITCASTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXCONSTRUCTEXPR, "clang::CXXConstructExpr");
  g.add_edge((CLASS_CXXCONSTRUCTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXCONSTRUCTEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967891, "clang::CXXConstructExpr::getConstructor"),
      add_named_node(&mut g, 4294967892, "clang::CXXConstructExpr::getLocation"),
      add_named_node(&mut g, 4294967893, "clang::CXXConstructExpr::isElidable"),
      add_named_node(&mut g, 4294967894, "clang::CXXConstructExpr::hadMultipleCandidates"),
      add_named_node(&mut g, 4294967895, "clang::CXXConstructExpr::isListInitialization"),
      add_named_node(&mut g, 4294967896, "clang::CXXConstructExpr::isStdInitListInitialization"),
      add_named_node(&mut g, 4294967897, "clang::CXXConstructExpr::requiresZeroInitialization"),
      add_named_node(&mut g, 4294967898, "clang::CXXConstructExpr::getConstructionKind"),
      add_named_node(&mut g, 4294967899, "clang::CXXConstructExpr::arguments"),
      add_named_node(&mut g, 4294967900, "clang::CXXConstructExpr::arg_begin"),
      add_named_node(&mut g, 4294967901, "clang::CXXConstructExpr::arg_end"),
      add_named_node(&mut g, 4294967902, "clang::CXXConstructExpr::getArgs"),
      add_named_node(&mut g, 4294967903, "clang::CXXConstructExpr::getNumArgs"),
      add_named_node(&mut g, 4294967904, "clang::CXXConstructExpr::isImmediateEscalating"),
      add_named_node(&mut g, 4294967905, "clang::CXXConstructExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967906, "clang::CXXConstructExpr::getEndLoc"),
      add_named_node(&mut g, 4294967907, "clang::CXXConstructExpr::getParenOrBraceRange"),
      add_named_node(&mut g, 4294967908, "clang::CXXConstructExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXCONSTRUCTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_COMPOUNDLITERALEXPR, "clang::CompoundLiteralExpr");
  g.add_edge((CLASS_COMPOUNDLITERALEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_COMPOUNDLITERALEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967910, "clang::CompoundLiteralExpr::getInitializer"),
      add_named_node(&mut g, 4294967911, "clang::CompoundLiteralExpr::isFileScope"),
      add_named_node(&mut g, 4294967912, "clang::CompoundLiteralExpr::getLParenLoc"),
      add_named_node(&mut g, 4294967913, "clang::CompoundLiteralExpr::getTypeSourceInfo"),
      add_named_node(&mut g, 4294967914, "clang::CompoundLiteralExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967915, "clang::CompoundLiteralExpr::getEndLoc"),
      add_named_node(&mut g, 4294967916, "clang::CompoundLiteralExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_COMPOUNDLITERALEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXDEFAULTINITEXPR, "clang::CXXDefaultInitExpr");
  g.add_edge((CLASS_CXXDEFAULTINITEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXDEFAULTINITEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967918, "clang::CXXDefaultInitExpr::hasRewrittenInit"),
      add_named_node(&mut g, 4294967919, "clang::CXXDefaultInitExpr::getField"),
      add_named_node(&mut g, 4294967920, "clang::CXXDefaultInitExpr::getExpr"),
      add_named_node(&mut g, 4294967921, "clang::CXXDefaultInitExpr::getRewrittenExpr"),
      add_named_node(&mut g, 4294967922, "clang::CXXDefaultInitExpr::getUsedContext"),
      add_named_node(&mut g, 4294967923, "clang::CXXDefaultInitExpr::getUsedLocation"),
      add_named_node(&mut g, 4294967924, "clang::CXXDefaultInitExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967925, "clang::CXXDefaultInitExpr::getEndLoc"),
      add_named_node(&mut g, 4294967926, "clang::CXXDefaultInitExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXDEFAULTINITEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXDEFAULTARGEXPR, "clang::CXXDefaultArgExpr");
  g.add_edge((CLASS_CXXDEFAULTARGEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXDEFAULTARGEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967928, "clang::CXXDefaultArgExpr::getParam"),
      add_named_node(&mut g, 4294967929, "clang::CXXDefaultArgExpr::hasRewrittenInit"),
      add_named_node(&mut g, 4294967930, "clang::CXXDefaultArgExpr::getExpr"),
      add_named_node(&mut g, 4294967931, "clang::CXXDefaultArgExpr::getRewrittenExpr"),
      add_named_node(&mut g, 4294967932, "clang::CXXDefaultArgExpr::getAdjustedRewrittenExpr"),
      add_named_node(&mut g, 4294967933, "clang::CXXDefaultArgExpr::getUsedContext"),
      add_named_node(&mut g, 4294967934, "clang::CXXDefaultArgExpr::getUsedLocation"),
      add_named_node(&mut g, 4294967935, "clang::CXXDefaultArgExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967936, "clang::CXXDefaultArgExpr::getEndLoc"),
      add_named_node(&mut g, 4294967937, "clang::CXXDefaultArgExpr::getExprLoc"),
      add_named_node(&mut g, 4294967938, "clang::CXXDefaultArgExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXDEFAULTARGEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPSECTIONDIRECTIVE, "clang::OMPSectionDirective");
  g.add_edge((CLASS_OMPSECTIONDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPSECTIONDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPSECTIONDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXTHROWEXPR, "clang::CXXThrowExpr");
  g.add_edge((CLASS_CXXTHROWEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXTHROWEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967941, "clang::CXXThrowExpr::getSubExpr"),
      add_named_node(&mut g, 4294967942, "clang::CXXThrowExpr::getThrowLoc"),
      add_named_node(&mut g, 4294967943, "clang::CXXThrowExpr::isThrownVariableInScope"),
      add_named_node(&mut g, 4294967944, "clang::CXXThrowExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967945, "clang::CXXThrowExpr::getEndLoc"),
      add_named_node(&mut g, 4294967946, "clang::CXXThrowExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXTHROWEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MSPROPERTYSUBSCRIPTEXPR, "clang::MSPropertySubscriptExpr");
  g.add_edge((CLASS_MSPROPERTYSUBSCRIPTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MSPROPERTYSUBSCRIPTEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967948, "clang::MSPropertySubscriptExpr::getBase"),
      add_named_node(&mut g, 4294967949, "clang::MSPropertySubscriptExpr::getIdx"),
      add_named_node(&mut g, 4294967950, "clang::MSPropertySubscriptExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967951, "clang::MSPropertySubscriptExpr::getEndLoc"),
      add_named_node(&mut g, 4294967952, "clang::MSPropertySubscriptExpr::getRBracketLoc"),
      add_named_node(&mut g, 4294967953, "clang::MSPropertySubscriptExpr::getExprLoc"),
      add_named_node(&mut g, 4294967954, "clang::MSPropertySubscriptExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MSPROPERTYSUBSCRIPTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXREWRITTENBINARYOPERATOR, "clang::CXXRewrittenBinaryOperator");
  g.add_edge((CLASS_CXXREWRITTENBINARYOPERATOR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXREWRITTENBINARYOPERATOR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967956, "clang::CXXRewrittenBinaryOperator::getSemanticForm"),
      add_named_node(&mut g, 4294967957, "clang::CXXRewrittenBinaryOperator::getDecomposedForm"),
      add_named_node(&mut g, 4294967958, "clang::CXXRewrittenBinaryOperator::isReversed"),
      add_named_node(&mut g, 4294967959, "clang::CXXRewrittenBinaryOperator::getOperator"),
      add_named_node(&mut g, 4294967960, "clang::CXXRewrittenBinaryOperator::getOpcode"),
      add_named_node(&mut g, 4294967961, "clang::CXXRewrittenBinaryOperator::getOpcodeStr"),
      add_named_node(&mut g, 4294967962, "clang::CXXRewrittenBinaryOperator::isComparisonOp"),
      add_named_node(&mut g, 4294967963, "clang::CXXRewrittenBinaryOperator::isAssignmentOp"),
      add_named_node(&mut g, 4294967964, "clang::CXXRewrittenBinaryOperator::getLHS"),
      add_named_node(&mut g, 4294967965, "clang::CXXRewrittenBinaryOperator::getRHS"),
      add_named_node(&mut g, 4294967966, "clang::CXXRewrittenBinaryOperator::getOperatorLoc"),
      add_named_node(&mut g, 4294967967, "clang::CXXRewrittenBinaryOperator::getExprLoc"),
      add_named_node(&mut g, 4294967968, "clang::CXXRewrittenBinaryOperator::getBeginLoc"),
      add_named_node(&mut g, 4294967969, "clang::CXXRewrittenBinaryOperator::getEndLoc"),
      add_named_node(&mut g, 4294967970, "clang::CXXRewrittenBinaryOperator::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXREWRITTENBINARYOPERATOR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CUDAKERNELCALLEXPR, "clang::CUDAKernelCallExpr");
  g.add_edge((CLASS_CUDAKERNELCALLEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CUDAKERNELCALLEXPR, META_SUBCLASS, CLASS_CALLEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967972, "clang::CUDAKernelCallExpr::getConfig"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CUDAKERNELCALLEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPOEXPR, "clang::TypoExpr");
  g.add_edge((CLASS_TYPOEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPOEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967974, "clang::TypoExpr::children"),
      add_named_node(&mut g, 4294967975, "clang::TypoExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967976, "clang::TypoExpr::getEndLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPOEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ASTYPEEXPR, "clang::AsTypeExpr");
  g.add_edge((CLASS_ASTYPEEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ASTYPEEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967978, "clang::AsTypeExpr::getSrcExpr"),
      add_named_node(&mut g, 4294967979, "clang::AsTypeExpr::getBuiltinLoc"),
      add_named_node(&mut g, 4294967980, "clang::AsTypeExpr::getRParenLoc"),
      add_named_node(&mut g, 4294967981, "clang::AsTypeExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967982, "clang::AsTypeExpr::getEndLoc"),
      add_named_node(&mut g, 4294967983, "clang::AsTypeExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ASTYPEEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_EXTVECTORELEMENTEXPR, "clang::ExtVectorElementExpr");
  g.add_edge((CLASS_EXTVECTORELEMENTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_EXTVECTORELEMENTEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967985, "clang::ExtVectorElementExpr::getBase"),
      add_named_node(&mut g, 4294967986, "clang::ExtVectorElementExpr::getAccessor"),
      add_named_node(&mut g, 4294967987, "clang::ExtVectorElementExpr::getAccessorLoc"),
      add_named_node(&mut g, 4294967988, "clang::ExtVectorElementExpr::getNumElements"),
      add_named_node(&mut g, 4294967989, "clang::ExtVectorElementExpr::containsDuplicateElements"),
      add_named_node(&mut g, 4294967990, "clang::ExtVectorElementExpr::getBeginLoc"),
      add_named_node(&mut g, 4294967991, "clang::ExtVectorElementExpr::getEndLoc"),
      add_named_node(&mut g, 4294967992, "clang::ExtVectorElementExpr::isArrow"),
      add_named_node(&mut g, 4294967993, "clang::ExtVectorElementExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_EXTVECTORELEMENTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_GENERICSELECTIONEXPR, "clang::GenericSelectionExpr");
  g.add_edge((CLASS_GENERICSELECTIONEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_GENERICSELECTIONEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294967995, "clang::GenericSelectionExpr::getNumAssocs"),
      add_named_node(&mut g, 4294967996, "clang::GenericSelectionExpr::getResultIndex"),
      add_named_node(&mut g, 4294967997, "clang::GenericSelectionExpr::isResultDependent"),
      add_named_node(&mut g, 4294967998, "clang::GenericSelectionExpr::isExprPredicate"),
      add_named_node(&mut g, 4294967999, "clang::GenericSelectionExpr::isTypePredicate"),
      add_named_node(&mut g, 4294968000, "clang::GenericSelectionExpr::getControllingExpr"),
      add_named_node(&mut g, 4294968001, "clang::GenericSelectionExpr::getControllingType"),
      add_named_node(&mut g, 4294968002, "clang::GenericSelectionExpr::getResultExpr"),
      add_named_node(&mut g, 4294968003, "clang::GenericSelectionExpr::getAssocExprs"),
      add_named_node(&mut g, 4294968004, "clang::GenericSelectionExpr::getAssocTypeSourceInfos"),
      add_named_node(&mut g, 4294968005, "clang::GenericSelectionExpr::associations"),
      add_named_node(&mut g, 4294968006, "clang::GenericSelectionExpr::getGenericLoc"),
      add_named_node(&mut g, 4294968007, "clang::GenericSelectionExpr::getDefaultLoc"),
      add_named_node(&mut g, 4294968008, "clang::GenericSelectionExpr::getRParenLoc"),
      add_named_node(&mut g, 4294968009, "clang::GenericSelectionExpr::getBeginLoc"),
      add_named_node(&mut g, 4294968010, "clang::GenericSelectionExpr::getEndLoc"),
      add_named_node(&mut g, 4294968011, "clang::GenericSelectionExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_GENERICSELECTIONEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_IMPLICITVALUEINITEXPR, "clang::ImplicitValueInitExpr");
  g.add_edge((CLASS_IMPLICITVALUEINITEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_IMPLICITVALUEINITEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968013, "clang::ImplicitValueInitExpr::getBeginLoc"),
      add_named_node(&mut g, 4294968014, "clang::ImplicitValueInitExpr::getEndLoc"),
      add_named_node(&mut g, 4294968015, "clang::ImplicitValueInitExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_IMPLICITVALUEINITEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NOINITEXPR, "clang::NoInitExpr");
  g.add_edge((CLASS_NOINITEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOINITEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968017, "clang::NoInitExpr::getBeginLoc"),
      add_named_node(&mut g, 4294968018, "clang::NoInitExpr::getEndLoc"),
      add_named_node(&mut g, 4294968019, "clang::NoInitExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NOINITEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ARRAYINITINDEXEXPR, "clang::ArrayInitIndexExpr");
  g.add_edge((CLASS_ARRAYINITINDEXEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARRAYINITINDEXEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968021, "clang::ArrayInitIndexExpr::getBeginLoc"),
      add_named_node(&mut g, 4294968022, "clang::ArrayInitIndexExpr::getEndLoc"),
      add_named_node(&mut g, 4294968023, "clang::ArrayInitIndexExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ARRAYINITINDEXEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DESIGNATEDINITEXPR, "clang::DesignatedInitExpr");
  g.add_edge((CLASS_DESIGNATEDINITEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DESIGNATEDINITEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968025, "clang::DesignatedInitExpr::size"),
      add_named_node(&mut g, 4294968026, "clang::DesignatedInitExpr::designators"),
      add_named_node(&mut g, 4294968027, "clang::DesignatedInitExpr::getEqualOrColonLoc"),
      add_named_node(&mut g, 4294968028, "clang::DesignatedInitExpr::isDirectInit"),
      add_named_node(&mut g, 4294968029, "clang::DesignatedInitExpr::usesGNUSyntax"),
      add_named_node(&mut g, 4294968030, "clang::DesignatedInitExpr::getInit"),
      add_named_node(&mut g, 4294968031, "clang::DesignatedInitExpr::getNumSubExprs"),
      add_named_node(&mut g, 4294968032, "clang::DesignatedInitExpr::getDesignatorsSourceRange"),
      add_named_node(&mut g, 4294968033, "clang::DesignatedInitExpr::getBeginLoc"),
      add_named_node(&mut g, 4294968034, "clang::DesignatedInitExpr::getEndLoc"),
      add_named_node(&mut g, 4294968035, "clang::DesignatedInitExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DESIGNATEDINITEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_STMTEXPR, "clang::StmtExpr");
  g.add_edge((CLASS_STMTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_STMTEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968037, "clang::StmtExpr::getSubStmt"),
      add_named_node(&mut g, 4294968038, "clang::StmtExpr::getBeginLoc"),
      add_named_node(&mut g, 4294968039, "clang::StmtExpr::getEndLoc"),
      add_named_node(&mut g, 4294968040, "clang::StmtExpr::getLParenLoc"),
      add_named_node(&mut g, 4294968041, "clang::StmtExpr::getRParenLoc"),
      add_named_node(&mut g, 4294968042, "clang::StmtExpr::getTemplateDepth"),
      add_named_node(&mut g, 4294968043, "clang::StmtExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_STMTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BINARYOPERATOR, "clang::BinaryOperator");
  g.add_edge((CLASS_BINARYOPERATOR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BINARYOPERATOR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968045, "clang::BinaryOperator::getExprLoc"),
      add_named_node(&mut g, 4294968046, "clang::BinaryOperator::getOperatorLoc"),
      add_named_node(&mut g, 4294968047, "clang::BinaryOperator::getOpcode"),
      add_named_node(&mut g, 4294968048, "clang::BinaryOperator::getLHS"),
      add_named_node(&mut g, 4294968049, "clang::BinaryOperator::getRHS"),
      add_named_node(&mut g, 4294968050, "clang::BinaryOperator::getBeginLoc"),
      add_named_node(&mut g, 4294968051, "clang::BinaryOperator::getEndLoc"),
      add_named_node(&mut g, 4294968052, "clang::BinaryOperator::getOpcodeStr"),
      add_named_node(&mut g, 4294968053, "clang::BinaryOperator::isPtrMemOp"),
      add_named_node(&mut g, 4294968054, "clang::BinaryOperator::isMultiplicativeOp"),
      add_named_node(&mut g, 4294968055, "clang::BinaryOperator::isAdditiveOp"),
      add_named_node(&mut g, 4294968056, "clang::BinaryOperator::isShiftOp"),
      add_named_node(&mut g, 4294968057, "clang::BinaryOperator::isBitwiseOp"),
      add_named_node(&mut g, 4294968058, "clang::BinaryOperator::isRelationalOp"),
      add_named_node(&mut g, 4294968059, "clang::BinaryOperator::isEqualityOp"),
      add_named_node(&mut g, 4294968060, "clang::BinaryOperator::isComparisonOp"),
      add_named_node(&mut g, 4294968061, "clang::BinaryOperator::isCommaOp"),
      add_named_node(&mut g, 4294968062, "clang::BinaryOperator::isLogicalOp"),
      add_named_node(&mut g, 4294968063, "clang::BinaryOperator::isAssignmentOp"),
      add_named_node(&mut g, 4294968064, "clang::BinaryOperator::isCompoundAssignmentOp"),
      add_named_node(&mut g, 4294968065, "clang::BinaryOperator::isShiftAssignOp"),
      add_named_node(&mut g, 4294968066, "clang::BinaryOperator::children"),
      add_named_node(&mut g, 4294968067, "clang::BinaryOperator::hasStoredFPFeatures"),
      add_named_node(&mut g, 4294968068, "clang::BinaryOperator::getStoredFPFeatures"),
      add_named_node(&mut g, 4294968069, "clang::BinaryOperator::getFPFeatures"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BINARYOPERATOR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXSTATICCASTEXPR, "clang::CXXStaticCastExpr");
  g.add_edge((CLASS_CXXSTATICCASTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXSTATICCASTEXPR, META_SUBCLASS, CLASS_CXXNAMEDCASTEXPR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXSTATICCASTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MEMBEREXPR, "clang::MemberExpr");
  g.add_edge((CLASS_MEMBEREXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MEMBEREXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968072, "clang::MemberExpr::getBase"),
      add_named_node(&mut g, 4294968073, "clang::MemberExpr::getMemberDecl"),
      add_named_node(&mut g, 4294968074, "clang::MemberExpr::getFoundDecl"),
      add_named_node(&mut g, 4294968075, "clang::MemberExpr::hasQualifier"),
      add_named_node(&mut g, 4294968076, "clang::MemberExpr::getQualifierLoc"),
      add_named_node(&mut g, 4294968077, "clang::MemberExpr::getQualifier"),
      add_named_node(&mut g, 4294968078, "clang::MemberExpr::getTemplateKeywordLoc"),
      add_named_node(&mut g, 4294968079, "clang::MemberExpr::getLAngleLoc"),
      add_named_node(&mut g, 4294968080, "clang::MemberExpr::getRAngleLoc"),
      add_named_node(&mut g, 4294968081, "clang::MemberExpr::hasTemplateKeyword"),
      add_named_node(&mut g, 4294968082, "clang::MemberExpr::hasExplicitTemplateArgs"),
      add_named_node(&mut g, 4294968083, "clang::MemberExpr::getTemplateArgs"),
      add_named_node(&mut g, 4294968084, "clang::MemberExpr::getNumTemplateArgs"),
      add_named_node(&mut g, 4294968085, "clang::MemberExpr::template_arguments"),
      add_named_node(&mut g, 4294968086, "clang::MemberExpr::getMemberNameInfo"),
      add_named_node(&mut g, 4294968087, "clang::MemberExpr::getOperatorLoc"),
      add_named_node(&mut g, 4294968088, "clang::MemberExpr::isArrow"),
      add_named_node(&mut g, 4294968089, "clang::MemberExpr::getMemberLoc"),
      add_named_node(&mut g, 4294968090, "clang::MemberExpr::getBeginLoc"),
      add_named_node(&mut g, 4294968091, "clang::MemberExpr::getEndLoc"),
      add_named_node(&mut g, 4294968092, "clang::MemberExpr::getExprLoc"),
      add_named_node(&mut g, 4294968093, "clang::MemberExpr::isImplicitAccess"),
      add_named_node(&mut g, 4294968094, "clang::MemberExpr::hadMultipleCandidates"),
      add_named_node(&mut g, 4294968095, "clang::MemberExpr::isNonOdrUse"),
      add_named_node(&mut g, 4294968096, "clang::MemberExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MEMBEREXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_UNARYEXPRORTYPETRAITEXPR, "clang::UnaryExprOrTypeTraitExpr");
  g.add_edge((CLASS_UNARYEXPRORTYPETRAITEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNARYEXPRORTYPETRAITEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968098, "clang::UnaryExprOrTypeTraitExpr::getKind"),
      add_named_node(&mut g, 4294968099, "clang::UnaryExprOrTypeTraitExpr::isArgumentType"),
      add_named_node(&mut g, 4294968100, "clang::UnaryExprOrTypeTraitExpr::getArgumentType"),
      add_named_node(&mut g, 4294968101, "clang::UnaryExprOrTypeTraitExpr::getArgumentTypeInfo"),
      add_named_node(&mut g, 4294968102, "clang::UnaryExprOrTypeTraitExpr::getArgumentExpr"),
      add_named_node(&mut g, 4294968103, "clang::UnaryExprOrTypeTraitExpr::getTypeOfArgument"),
      add_named_node(&mut g, 4294968104, "clang::UnaryExprOrTypeTraitExpr::getOperatorLoc"),
      add_named_node(&mut g, 4294968105, "clang::UnaryExprOrTypeTraitExpr::getRParenLoc"),
      add_named_node(&mut g, 4294968106, "clang::UnaryExprOrTypeTraitExpr::getBeginLoc"),
      add_named_node(&mut g, 4294968107, "clang::UnaryExprOrTypeTraitExpr::getEndLoc"),
      add_named_node(&mut g, 4294968108, "clang::UnaryExprOrTypeTraitExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNARYEXPRORTYPETRAITEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OFFSETOFEXPR, "clang::OffsetOfExpr");
  g.add_edge((CLASS_OFFSETOFEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OFFSETOFEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968110, "clang::OffsetOfExpr::getOperatorLoc"),
      add_named_node(&mut g, 4294968111, "clang::OffsetOfExpr::getRParenLoc"),
      add_named_node(&mut g, 4294968112, "clang::OffsetOfExpr::getTypeSourceInfo"),
      add_named_node(&mut g, 4294968113, "clang::OffsetOfExpr::getNumComponents"),
      add_named_node(&mut g, 4294968114, "clang::OffsetOfExpr::getNumExpressions"),
      add_named_node(&mut g, 4294968115, "clang::OffsetOfExpr::getBeginLoc"),
      add_named_node(&mut g, 4294968116, "clang::OffsetOfExpr::getEndLoc"),
      add_named_node(&mut g, 4294968117, "clang::OffsetOfExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OFFSETOFEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PREDEFINEDEXPR, "clang::PredefinedExpr");
  g.add_edge((CLASS_PREDEFINEDEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PREDEFINEDEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968119, "clang::PredefinedExpr::getIdentKind"),
      add_named_node(&mut g, 4294968120, "clang::PredefinedExpr::isTransparent"),
      add_named_node(&mut g, 4294968121, "clang::PredefinedExpr::getLocation"),
      add_named_node(&mut g, 4294968122, "clang::PredefinedExpr::getFunctionName"),
      add_named_node(&mut g, 4294968123, "clang::PredefinedExpr::getIdentKindName"),
      add_named_node(&mut g, 4294968124, "clang::PredefinedExpr::getBeginLoc"),
      add_named_node(&mut g, 4294968125, "clang::PredefinedExpr::getEndLoc"),
      add_named_node(&mut g, 4294968126, "clang::PredefinedExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PREDEFINEDEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CHARACTERLITERAL, "clang::CharacterLiteral");
  g.add_edge((CLASS_CHARACTERLITERAL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CHARACTERLITERAL, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968128, "clang::CharacterLiteral::getLocation"),
      add_named_node(&mut g, 4294968129, "clang::CharacterLiteral::getKind"),
      add_named_node(&mut g, 4294968130, "clang::CharacterLiteral::getBeginLoc"),
      add_named_node(&mut g, 4294968131, "clang::CharacterLiteral::getEndLoc"),
      add_named_node(&mut g, 4294968132, "clang::CharacterLiteral::getValue"),
      add_named_node(&mut g, 4294968133, "clang::CharacterLiteral::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CHARACTERLITERAL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OPAQUEVALUEEXPR, "clang::OpaqueValueExpr");
  g.add_edge((CLASS_OPAQUEVALUEEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OPAQUEVALUEEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968135, "clang::OpaqueValueExpr::getLocation"),
      add_named_node(&mut g, 4294968136, "clang::OpaqueValueExpr::getBeginLoc"),
      add_named_node(&mut g, 4294968137, "clang::OpaqueValueExpr::getEndLoc"),
      add_named_node(&mut g, 4294968138, "clang::OpaqueValueExpr::getExprLoc"),
      add_named_node(&mut g, 4294968139, "clang::OpaqueValueExpr::children"),
      add_named_node(&mut g, 4294968140, "clang::OpaqueValueExpr::getSourceExpr"),
      add_named_node(&mut g, 4294968141, "clang::OpaqueValueExpr::isUnique"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OPAQUEVALUEEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_EXPRWITHCLEANUPS, "clang::ExprWithCleanups");
  g.add_edge((CLASS_EXPRWITHCLEANUPS, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_EXPRWITHCLEANUPS, META_SUBCLASS, CLASS_FULLEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968143, "clang::ExprWithCleanups::getObjects"),
      add_named_node(&mut g, 4294968144, "clang::ExprWithCleanups::getNumObjects"),
      add_named_node(&mut g, 4294968145, "clang::ExprWithCleanups::cleanupsHaveSideEffects"),
      add_named_node(&mut g, 4294968146, "clang::ExprWithCleanups::getBeginLoc"),
      add_named_node(&mut g, 4294968147, "clang::ExprWithCleanups::getEndLoc"),
      add_named_node(&mut g, 4294968148, "clang::ExprWithCleanups::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_EXPRWITHCLEANUPS, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ATTRIBUTEDSTMT, "clang::AttributedStmt");
  g.add_edge((CLASS_ATTRIBUTEDSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ATTRIBUTEDSTMT, META_SUBCLASS, CLASS_VALUESTMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968150, "clang::AttributedStmt::getAttrLoc"),
      add_named_node(&mut g, 4294968151, "clang::AttributedStmt::getAttrs"),
      add_named_node(&mut g, 4294968152, "clang::AttributedStmt::getSubStmt"),
      add_named_node(&mut g, 4294968153, "clang::AttributedStmt::getBeginLoc"),
      add_named_node(&mut g, 4294968154, "clang::AttributedStmt::getEndLoc"),
      add_named_node(&mut g, 4294968155, "clang::AttributedStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ATTRIBUTEDSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DECLSTMT, "clang::DeclStmt");
  g.add_edge((CLASS_DECLSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DECLSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968157, "clang::DeclStmt::isSingleDecl"),
      add_named_node(&mut g, 4294968158, "clang::DeclStmt::getSingleDecl"),
      add_named_node(&mut g, 4294968159, "clang::DeclStmt::getDeclGroup"),
      add_named_node(&mut g, 4294968160, "clang::DeclStmt::getEndLoc"),
      add_named_node(&mut g, 4294968161, "clang::DeclStmt::getBeginLoc"),
      add_named_node(&mut g, 4294968162, "clang::DeclStmt::children"),
      add_named_node(&mut g, 4294968163, "clang::DeclStmt::decls"),
      add_named_node(&mut g, 4294968164, "clang::DeclStmt::decl_begin"),
      add_named_node(&mut g, 4294968165, "clang::DeclStmt::decl_end"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DECLSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXRECORDDECL, "clang::CXXRecordDecl");
  g.add_edge((CLASS_CXXRECORDDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXRECORDDECL, META_SUBCLASS, CLASS_RECORDDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968167, "clang::CXXRecordDecl::getCanonicalDecl"),
      add_named_node(&mut g, 4294968168, "clang::CXXRecordDecl::getPreviousDecl"),
      add_named_node(&mut g, 4294968169, "clang::CXXRecordDecl::getMostRecentDecl"),
      add_named_node(&mut g, 4294968170, "clang::CXXRecordDecl::getMostRecentNonInjectedDecl"),
      add_named_node(&mut g, 4294968171, "clang::CXXRecordDecl::getDefinition"),
      add_named_node(&mut g, 4294968172, "clang::CXXRecordDecl::hasDefinition"),
      add_named_node(&mut g, 4294968173, "clang::CXXRecordDecl::isDynamicClass"),
      add_named_node(&mut g, 4294968174, "clang::CXXRecordDecl::mayBeDynamicClass"),
      add_named_node(&mut g, 4294968175, "clang::CXXRecordDecl::mayBeNonDynamicClass"),
      add_named_node(&mut g, 4294968176, "clang::CXXRecordDecl::isParsingBaseSpecifiers"),
      add_named_node(&mut g, 4294968177, "clang::CXXRecordDecl::getODRHash"),
      add_named_node(&mut g, 4294968178, "clang::CXXRecordDecl::getNumBases"),
      add_named_node(&mut g, 4294968179, "clang::CXXRecordDecl::bases"),
      add_named_node(&mut g, 4294968180, "clang::CXXRecordDecl::bases_begin"),
      add_named_node(&mut g, 4294968181, "clang::CXXRecordDecl::bases_end"),
      add_named_node(&mut g, 4294968182, "clang::CXXRecordDecl::getNumVBases"),
      add_named_node(&mut g, 4294968183, "clang::CXXRecordDecl::vbases"),
      add_named_node(&mut g, 4294968184, "clang::CXXRecordDecl::vbases_begin"),
      add_named_node(&mut g, 4294968185, "clang::CXXRecordDecl::vbases_end"),
      add_named_node(&mut g, 4294968186, "clang::CXXRecordDecl::hasAnyDependentBases"),
      add_named_node(&mut g, 4294968187, "clang::CXXRecordDecl::methods"),
      add_named_node(&mut g, 4294968188, "clang::CXXRecordDecl::method_begin"),
      add_named_node(&mut g, 4294968189, "clang::CXXRecordDecl::method_end"),
      add_named_node(&mut g, 4294968190, "clang::CXXRecordDecl::ctors"),
      add_named_node(&mut g, 4294968191, "clang::CXXRecordDecl::ctor_begin"),
      add_named_node(&mut g, 4294968192, "clang::CXXRecordDecl::ctor_end"),
      add_named_node(&mut g, 4294968193, "clang::CXXRecordDecl::friends"),
      add_named_node(&mut g, 4294968194, "clang::CXXRecordDecl::friend_begin"),
      add_named_node(&mut g, 4294968195, "clang::CXXRecordDecl::friend_end"),
      add_named_node(&mut g, 4294968196, "clang::CXXRecordDecl::hasFriends"),
      add_named_node(&mut g, 4294968197, "clang::CXXRecordDecl::defaultedCopyConstructorIsDeleted"),
      add_named_node(&mut g, 4294968198, "clang::CXXRecordDecl::defaultedMoveConstructorIsDeleted"),
      add_named_node(&mut g, 4294968199, "clang::CXXRecordDecl::defaultedDestructorIsDeleted"),
      add_named_node(&mut g, 4294968200, "clang::CXXRecordDecl::hasSimpleCopyConstructor"),
      add_named_node(&mut g, 4294968201, "clang::CXXRecordDecl::hasSimpleMoveConstructor"),
      add_named_node(&mut g, 4294968202, "clang::CXXRecordDecl::hasSimpleCopyAssignment"),
      add_named_node(&mut g, 4294968203, "clang::CXXRecordDecl::hasSimpleMoveAssignment"),
      add_named_node(&mut g, 4294968204, "clang::CXXRecordDecl::hasSimpleDestructor"),
      add_named_node(&mut g, 4294968205, "clang::CXXRecordDecl::hasDefaultConstructor"),
      add_named_node(&mut g, 4294968206, "clang::CXXRecordDecl::needsImplicitDefaultConstructor"),
      add_named_node(&mut g, 4294968207, "clang::CXXRecordDecl::hasUserDeclaredConstructor"),
      add_named_node(&mut g, 4294968208, "clang::CXXRecordDecl::hasUserProvidedDefaultConstructor"),
      add_named_node(&mut g, 4294968209, "clang::CXXRecordDecl::hasUserDeclaredCopyConstructor"),
      add_named_node(&mut g, 4294968210, "clang::CXXRecordDecl::needsImplicitCopyConstructor"),
      add_named_node(&mut g, 4294968211, "clang::CXXRecordDecl::needsOverloadResolutionForCopyConstructor"),
      add_named_node(&mut g, 4294968212, "clang::CXXRecordDecl::implicitCopyConstructorHasConstParam"),
      add_named_node(&mut g, 4294968213, "clang::CXXRecordDecl::hasCopyConstructorWithConstParam"),
      add_named_node(&mut g, 4294968214, "clang::CXXRecordDecl::hasUserDeclaredMoveOperation"),
      add_named_node(&mut g, 4294968215, "clang::CXXRecordDecl::hasUserDeclaredMoveConstructor"),
      add_named_node(&mut g, 4294968216, "clang::CXXRecordDecl::hasMoveConstructor"),
      add_named_node(&mut g, 4294968217, "clang::CXXRecordDecl::needsImplicitMoveConstructor"),
      add_named_node(&mut g, 4294968218, "clang::CXXRecordDecl::needsOverloadResolutionForMoveConstructor"),
      add_named_node(&mut g, 4294968219, "clang::CXXRecordDecl::hasUserDeclaredCopyAssignment"),
      add_named_node(&mut g, 4294968220, "clang::CXXRecordDecl::needsImplicitCopyAssignment"),
      add_named_node(&mut g, 4294968221, "clang::CXXRecordDecl::needsOverloadResolutionForCopyAssignment"),
      add_named_node(&mut g, 4294968222, "clang::CXXRecordDecl::implicitCopyAssignmentHasConstParam"),
      add_named_node(&mut g, 4294968223, "clang::CXXRecordDecl::hasCopyAssignmentWithConstParam"),
      add_named_node(&mut g, 4294968224, "clang::CXXRecordDecl::hasUserDeclaredMoveAssignment"),
      add_named_node(&mut g, 4294968225, "clang::CXXRecordDecl::hasMoveAssignment"),
      add_named_node(&mut g, 4294968226, "clang::CXXRecordDecl::needsImplicitMoveAssignment"),
      add_named_node(&mut g, 4294968227, "clang::CXXRecordDecl::needsOverloadResolutionForMoveAssignment"),
      add_named_node(&mut g, 4294968228, "clang::CXXRecordDecl::hasUserDeclaredDestructor"),
      add_named_node(&mut g, 4294968229, "clang::CXXRecordDecl::needsImplicitDestructor"),
      add_named_node(&mut g, 4294968230, "clang::CXXRecordDecl::needsOverloadResolutionForDestructor"),
      add_named_node(&mut g, 4294968231, "clang::CXXRecordDecl::isLambda"),
      add_named_node(&mut g, 4294968232, "clang::CXXRecordDecl::isGenericLambda"),
      add_named_node(&mut g, 4294968233, "clang::CXXRecordDecl::lambdaIsDefaultConstructibleAndAssignable"),
      add_named_node(&mut g, 4294968234, "clang::CXXRecordDecl::getLambdaCallOperator"),
      add_named_node(&mut g, 4294968235, "clang::CXXRecordDecl::getDependentLambdaCallOperator"),
      add_named_node(&mut g, 4294968236, "clang::CXXRecordDecl::getLambdaStaticInvoker"),
      add_named_node(&mut g, 4294968237, "clang::CXXRecordDecl::getGenericLambdaTemplateParameterList"),
      add_named_node(&mut g, 4294968238, "clang::CXXRecordDecl::getLambdaExplicitTemplateParameters"),
      add_named_node(&mut g, 4294968239, "clang::CXXRecordDecl::getLambdaCaptureDefault"),
      add_named_node(&mut g, 4294968240, "clang::CXXRecordDecl::isCapturelessLambda"),
      add_named_node(&mut g, 4294968241, "clang::CXXRecordDecl::captures"),
      add_named_node(&mut g, 4294968242, "clang::CXXRecordDecl::captures_begin"),
      add_named_node(&mut g, 4294968243, "clang::CXXRecordDecl::captures_end"),
      add_named_node(&mut g, 4294968244, "clang::CXXRecordDecl::capture_size"),
      add_named_node(&mut g, 4294968245, "clang::CXXRecordDecl::conversion_begin"),
      add_named_node(&mut g, 4294968246, "clang::CXXRecordDecl::conversion_end"),
      add_named_node(&mut g, 4294968247, "clang::CXXRecordDecl::getVisibleConversionFunctions"),
      add_named_node(&mut g, 4294968248, "clang::CXXRecordDecl::isAggregate"),
      add_named_node(&mut g, 4294968249, "clang::CXXRecordDecl::hasInClassInitializer"),
      add_named_node(&mut g, 4294968250, "clang::CXXRecordDecl::hasUninitializedReferenceMember"),
      add_named_node(&mut g, 4294968251, "clang::CXXRecordDecl::isPOD"),
      add_named_node(&mut g, 4294968252, "clang::CXXRecordDecl::isCLike"),
      add_named_node(&mut g, 4294968253, "clang::CXXRecordDecl::isEmpty"),
      add_named_node(&mut g, 4294968254, "clang::CXXRecordDecl::hasInitMethod"),
      add_named_node(&mut g, 4294968255, "clang::CXXRecordDecl::hasPrivateFields"),
      add_named_node(&mut g, 4294968256, "clang::CXXRecordDecl::hasProtectedFields"),
      add_named_node(&mut g, 4294968257, "clang::CXXRecordDecl::hasDirectFields"),
      add_named_node(&mut g, 4294968258, "clang::CXXRecordDecl::isPolymorphic"),
      add_named_node(&mut g, 4294968259, "clang::CXXRecordDecl::isAbstract"),
      add_named_node(&mut g, 4294968260, "clang::CXXRecordDecl::isStandardLayout"),
      add_named_node(&mut g, 4294968261, "clang::CXXRecordDecl::isCXX11StandardLayout"),
      add_named_node(&mut g, 4294968262, "clang::CXXRecordDecl::hasMutableFields"),
      add_named_node(&mut g, 4294968263, "clang::CXXRecordDecl::hasVariantMembers"),
      add_named_node(&mut g, 4294968264, "clang::CXXRecordDecl::hasTrivialDefaultConstructor"),
      add_named_node(&mut g, 4294968265, "clang::CXXRecordDecl::hasNonTrivialDefaultConstructor"),
      add_named_node(&mut g, 4294968266, "clang::CXXRecordDecl::hasConstexprNonCopyMoveConstructor"),
      add_named_node(&mut g, 4294968267, "clang::CXXRecordDecl::defaultedDefaultConstructorIsConstexpr"),
      add_named_node(&mut g, 4294968268, "clang::CXXRecordDecl::hasConstexprDefaultConstructor"),
      add_named_node(&mut g, 4294968269, "clang::CXXRecordDecl::hasTrivialCopyConstructor"),
      add_named_node(&mut g, 4294968270, "clang::CXXRecordDecl::hasTrivialCopyConstructorForCall"),
      add_named_node(&mut g, 4294968271, "clang::CXXRecordDecl::hasNonTrivialCopyConstructor"),
      add_named_node(&mut g, 4294968272, "clang::CXXRecordDecl::hasNonTrivialCopyConstructorForCall"),
      add_named_node(&mut g, 4294968273, "clang::CXXRecordDecl::hasTrivialMoveConstructor"),
      add_named_node(&mut g, 4294968274, "clang::CXXRecordDecl::hasTrivialMoveConstructorForCall"),
      add_named_node(&mut g, 4294968275, "clang::CXXRecordDecl::hasNonTrivialMoveConstructor"),
      add_named_node(&mut g, 4294968276, "clang::CXXRecordDecl::hasNonTrivialMoveConstructorForCall"),
      add_named_node(&mut g, 4294968277, "clang::CXXRecordDecl::hasTrivialCopyAssignment"),
      add_named_node(&mut g, 4294968278, "clang::CXXRecordDecl::hasNonTrivialCopyAssignment"),
      add_named_node(&mut g, 4294968279, "clang::CXXRecordDecl::hasTrivialMoveAssignment"),
      add_named_node(&mut g, 4294968280, "clang::CXXRecordDecl::hasNonTrivialMoveAssignment"),
      add_named_node(&mut g, 4294968281, "clang::CXXRecordDecl::defaultedDestructorIsConstexpr"),
      add_named_node(&mut g, 4294968282, "clang::CXXRecordDecl::hasConstexprDestructor"),
      add_named_node(&mut g, 4294968283, "clang::CXXRecordDecl::hasTrivialDestructor"),
      add_named_node(&mut g, 4294968284, "clang::CXXRecordDecl::hasTrivialDestructorForCall"),
      add_named_node(&mut g, 4294968285, "clang::CXXRecordDecl::hasNonTrivialDestructor"),
      add_named_node(&mut g, 4294968286, "clang::CXXRecordDecl::hasNonTrivialDestructorForCall"),
      add_named_node(&mut g, 4294968287, "clang::CXXRecordDecl::allowConstDefaultInit"),
      add_named_node(&mut g, 4294968288, "clang::CXXRecordDecl::hasIrrelevantDestructor"),
      add_named_node(&mut g, 4294968289, "clang::CXXRecordDecl::hasNonLiteralTypeFieldsOrBases"),
      add_named_node(&mut g, 4294968290, "clang::CXXRecordDecl::hasInheritedConstructor"),
      add_named_node(&mut g, 4294968291, "clang::CXXRecordDecl::hasInheritedAssignment"),
      add_named_node(&mut g, 4294968292, "clang::CXXRecordDecl::isTriviallyCopyable"),
      add_named_node(&mut g, 4294968293, "clang::CXXRecordDecl::isTriviallyCopyConstructible"),
      add_named_node(&mut g, 4294968294, "clang::CXXRecordDecl::isTrivial"),
      add_named_node(&mut g, 4294968295, "clang::CXXRecordDecl::isLiteral"),
      add_named_node(&mut g, 4294968296, "clang::CXXRecordDecl::isStructural"),
      add_named_node(&mut g, 4294968297, "clang::CXXRecordDecl::getInstantiatedFromMemberClass"),
      add_named_node(&mut g, 4294968298, "clang::CXXRecordDecl::getMemberSpecializationInfo"),
      add_named_node(&mut g, 4294968299, "clang::CXXRecordDecl::getDescribedClassTemplate"),
      add_named_node(&mut g, 4294968300, "clang::CXXRecordDecl::getTemplateSpecializationKind"),
      add_named_node(&mut g, 4294968301, "clang::CXXRecordDecl::getTemplateInstantiationPattern"),
      add_named_node(&mut g, 4294968302, "clang::CXXRecordDecl::getDestructor"),
      add_named_node(&mut g, 4294968303, "clang::CXXRecordDecl::isAnyDestructorNoReturn"),
      add_named_node(&mut g, 4294968304, "clang::CXXRecordDecl::isLocalClass"),
      add_named_node(&mut g, 4294968305, "clang::CXXRecordDecl::mayBeAbstract"),
      add_named_node(&mut g, 4294968306, "clang::CXXRecordDecl::isEffectivelyFinal"),
      add_named_node(&mut g, 4294968307, "clang::CXXRecordDecl::getLambdaManglingNumber"),
      add_named_node(&mut g, 4294968308, "clang::CXXRecordDecl::hasKnownLambdaInternalLinkage"),
      add_named_node(&mut g, 4294968309, "clang::CXXRecordDecl::getLambdaContextDecl"),
      add_named_node(&mut g, 4294968310, "clang::CXXRecordDecl::getLambdaIndexInContext"),
      add_named_node(&mut g, 4294968311, "clang::CXXRecordDecl::getLambdaNumbering"),
      add_named_node(&mut g, 4294968312, "clang::CXXRecordDecl::getDeviceLambdaManglingNumber"),
      add_named_node(&mut g, 4294968313, "clang::CXXRecordDecl::getMSInheritanceModel"),
      add_named_node(&mut g, 4294968314, "clang::CXXRecordDecl::calculateInheritanceModel"),
      add_named_node(&mut g, 4294968315, "clang::CXXRecordDecl::nullFieldOffsetIsZero"),
      add_named_node(&mut g, 4294968316, "clang::CXXRecordDecl::getMSVtorDispMode"),
      add_named_node(&mut g, 4294968317, "clang::CXXRecordDecl::isDependentLambda"),
      add_named_node(&mut g, 4294968318, "clang::CXXRecordDecl::isNeverDependentLambda"),
      add_named_node(&mut g, 4294968319, "clang::CXXRecordDecl::getLambdaDependencyKind"),
      add_named_node(&mut g, 4294968320, "clang::CXXRecordDecl::getLambdaTypeInfo"),
      add_named_node(&mut g, 4294968321, "clang::CXXRecordDecl::isInterfaceLike"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXRECORDDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_USINGDECL, "clang::UsingDecl");
  g.add_edge((CLASS_USINGDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_USINGDECL, META_SUBCLASS, CLASS_BASEUSINGDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968323, "clang::UsingDecl::getUsingLoc"),
      add_named_node(&mut g, 4294968324, "clang::UsingDecl::getQualifierLoc"),
      add_named_node(&mut g, 4294968325, "clang::UsingDecl::getQualifier"),
      add_named_node(&mut g, 4294968326, "clang::UsingDecl::getNameInfo"),
      add_named_node(&mut g, 4294968327, "clang::UsingDecl::isAccessDeclaration"),
      add_named_node(&mut g, 4294968328, "clang::UsingDecl::hasTypename"),
      add_named_node(&mut g, 4294968329, "clang::UsingDecl::getSourceRange"),
      add_named_node(&mut g, 4294968330, "clang::UsingDecl::getCanonicalDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_USINGDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCINDIRECTCOPYRESTOREEXPR, "clang::ObjCIndirectCopyRestoreExpr");
  g.add_edge((CLASS_OBJCINDIRECTCOPYRESTOREEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCINDIRECTCOPYRESTOREEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCINDIRECTCOPYRESTOREEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_HLSLBUFFERDECL, "clang::HLSLBufferDecl");
  g.add_edge((CLASS_HLSLBUFFERDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_HLSLBUFFERDECL, META_SUBCLASS, CLASS_NAMEDDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968333, "clang::HLSLBufferDecl::getSourceRange"),
      add_named_node(&mut g, 4294968334, "clang::HLSLBufferDecl::getLocStart"),
      add_named_node(&mut g, 4294968335, "clang::HLSLBufferDecl::getLBraceLoc"),
      add_named_node(&mut g, 4294968336, "clang::HLSLBufferDecl::getRBraceLoc"),
      add_named_node(&mut g, 4294968337, "clang::HLSLBufferDecl::isCBuffer"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_HLSLBUFFERDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEFAULTSTMT, "clang::DefaultStmt");
  g.add_edge((CLASS_DEFAULTSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEFAULTSTMT, META_SUBCLASS, CLASS_SWITCHCASE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968339, "clang::DefaultStmt::getSubStmt"),
      add_named_node(&mut g, 4294968340, "clang::DefaultStmt::getDefaultLoc"),
      add_named_node(&mut g, 4294968341, "clang::DefaultStmt::getBeginLoc"),
      add_named_node(&mut g, 4294968342, "clang::DefaultStmt::getEndLoc"),
      add_named_node(&mut g, 4294968343, "clang::DefaultStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEFAULTSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCCOMPATIBLEALIASDECL, "clang::ObjCCompatibleAliasDecl");
  g.add_edge((CLASS_OBJCCOMPATIBLEALIASDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCCOMPATIBLEALIASDECL, META_SUBCLASS, CLASS_NAMEDDECL));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCCOMPATIBLEALIASDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_EXPRESSIONTRAITEXPR, "clang::ExpressionTraitExpr");
  g.add_edge((CLASS_EXPRESSIONTRAITEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_EXPRESSIONTRAITEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968346, "clang::ExpressionTraitExpr::getBeginLoc"),
      add_named_node(&mut g, 4294968347, "clang::ExpressionTraitExpr::getEndLoc"),
      add_named_node(&mut g, 4294968348, "clang::ExpressionTraitExpr::getTrait"),
      add_named_node(&mut g, 4294968349, "clang::ExpressionTraitExpr::getQueriedExpression"),
      add_named_node(&mut g, 4294968350, "clang::ExpressionTraitExpr::getValue"),
      add_named_node(&mut g, 4294968351, "clang::ExpressionTraitExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_EXPRESSIONTRAITEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MSPROPERTYDECL, "clang::MSPropertyDecl");
  g.add_edge((CLASS_MSPROPERTYDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MSPROPERTYDECL, META_SUBCLASS, CLASS_DECLARATORDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968353, "clang::MSPropertyDecl::hasGetter"),
      add_named_node(&mut g, 4294968354, "clang::MSPropertyDecl::getGetterId"),
      add_named_node(&mut g, 4294968355, "clang::MSPropertyDecl::hasSetter"),
      add_named_node(&mut g, 4294968356, "clang::MSPropertyDecl::getSetterId"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MSPROPERTYDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BUILTINBITCASTEXPR, "clang::BuiltinBitCastExpr");
  g.add_edge((CLASS_BUILTINBITCASTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BUILTINBITCASTEXPR, META_SUBCLASS, CLASS_EXPLICITCASTEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968358, "clang::BuiltinBitCastExpr::getBeginLoc"),
      add_named_node(&mut g, 4294968359, "clang::BuiltinBitCastExpr::getEndLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BUILTINBITCASTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CONSTRUCTORUSINGSHADOWDECL, "clang::ConstructorUsingShadowDecl");
  g.add_edge((CLASS_CONSTRUCTORUSINGSHADOWDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONSTRUCTORUSINGSHADOWDECL, META_SUBCLASS, CLASS_USINGSHADOWDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968361, "clang::ConstructorUsingShadowDecl::getIntroducer"),
      add_named_node(&mut g, 4294968362, "clang::ConstructorUsingShadowDecl::getParent"),
      add_named_node(&mut g, 4294968363, "clang::ConstructorUsingShadowDecl::getNominatedBaseClassShadowDecl"),
      add_named_node(&mut g, 4294968364, "clang::ConstructorUsingShadowDecl::getConstructedBaseClassShadowDecl"),
      add_named_node(&mut g, 4294968365, "clang::ConstructorUsingShadowDecl::getNominatedBaseClass"),
      add_named_node(&mut g, 4294968366, "clang::ConstructorUsingShadowDecl::getConstructedBaseClass"),
      add_named_node(&mut g, 4294968367, "clang::ConstructorUsingShadowDecl::constructsVirtualBase"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CONSTRUCTORUSINGSHADOWDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPEDECL, "clang::TypeDecl");
  g.add_edge((CLASS_TYPEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPEDECL, META_SUBCLASS, CLASS_NAMEDDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968369, "clang::TypeDecl::getTypeForDecl"),
      add_named_node(&mut g, 4294968370, "clang::TypeDecl::getBeginLoc"),
      add_named_node(&mut g, 4294968371, "clang::TypeDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_USINGENUMDECL, "clang::UsingEnumDecl");
  g.add_edge((CLASS_USINGENUMDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_USINGENUMDECL, META_SUBCLASS, CLASS_BASEUSINGDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968373, "clang::UsingEnumDecl::getUsingLoc"),
      add_named_node(&mut g, 4294968374, "clang::UsingEnumDecl::getEnumLoc"),
      add_named_node(&mut g, 4294968375, "clang::UsingEnumDecl::getQualifier"),
      add_named_node(&mut g, 4294968376, "clang::UsingEnumDecl::getQualifierLoc"),
      add_named_node(&mut g, 4294968377, "clang::UsingEnumDecl::getEnumTypeLoc"),
      add_named_node(&mut g, 4294968378, "clang::UsingEnumDecl::getEnumType"),
      add_named_node(&mut g, 4294968379, "clang::UsingEnumDecl::getEnumDecl"),
      add_named_node(&mut g, 4294968380, "clang::UsingEnumDecl::getSourceRange"),
      add_named_node(&mut g, 4294968381, "clang::UsingEnumDecl::getCanonicalDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_USINGENUMDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPERRORDIRECTIVE, "clang::OMPErrorDirective");
  g.add_edge((CLASS_OMPERRORDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPERRORDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPERRORDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BASEUSINGDECL, "clang::BaseUsingDecl");
  g.add_edge((CLASS_BASEUSINGDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BASEUSINGDECL, META_SUBCLASS, CLASS_NAMEDDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968384, "clang::BaseUsingDecl::shadows"),
      add_named_node(&mut g, 4294968385, "clang::BaseUsingDecl::shadow_begin"),
      add_named_node(&mut g, 4294968386, "clang::BaseUsingDecl::shadow_end"),
      add_named_node(&mut g, 4294968387, "clang::BaseUsingDecl::shadow_size"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BASEUSINGDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_RECORDDECL, "clang::RecordDecl");
  g.add_edge((CLASS_RECORDDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RECORDDECL, META_SUBCLASS, CLASS_TAGDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968389, "clang::RecordDecl::getPreviousDecl"),
      add_named_node(&mut g, 4294968390, "clang::RecordDecl::getMostRecentDecl"),
      add_named_node(&mut g, 4294968391, "clang::RecordDecl::hasFlexibleArrayMember"),
      add_named_node(&mut g, 4294968392, "clang::RecordDecl::isAnonymousStructOrUnion"),
      add_named_node(&mut g, 4294968393, "clang::RecordDecl::hasObjectMember"),
      add_named_node(&mut g, 4294968394, "clang::RecordDecl::hasVolatileMember"),
      add_named_node(&mut g, 4294968395, "clang::RecordDecl::hasLoadedFieldsFromExternalStorage"),
      add_named_node(&mut g, 4294968396, "clang::RecordDecl::isNonTrivialToPrimitiveDefaultInitialize"),
      add_named_node(&mut g, 4294968397, "clang::RecordDecl::isNonTrivialToPrimitiveCopy"),
      add_named_node(&mut g, 4294968398, "clang::RecordDecl::isNonTrivialToPrimitiveDestroy"),
      add_named_node(&mut g, 4294968399, "clang::RecordDecl::hasNonTrivialToPrimitiveDefaultInitializeCUnion"),
      add_named_node(&mut g, 4294968400, "clang::RecordDecl::hasNonTrivialToPrimitiveDestructCUnion"),
      add_named_node(&mut g, 4294968401, "clang::RecordDecl::hasNonTrivialToPrimitiveCopyCUnion"),
      add_named_node(&mut g, 4294968402, "clang::RecordDecl::canPassInRegisters"),
      add_named_node(&mut g, 4294968403, "clang::RecordDecl::getArgPassingRestrictions"),
      add_named_node(&mut g, 4294968404, "clang::RecordDecl::isParamDestroyedInCallee"),
      add_named_node(&mut g, 4294968405, "clang::RecordDecl::isRandomized"),
      add_named_node(&mut g, 4294968406, "clang::RecordDecl::isInjectedClassName"),
      add_named_node(&mut g, 4294968407, "clang::RecordDecl::isLambda"),
      add_named_node(&mut g, 4294968408, "clang::RecordDecl::isCapturedRecord"),
      add_named_node(&mut g, 4294968409, "clang::RecordDecl::getDefinition"),
      add_named_node(&mut g, 4294968410, "clang::RecordDecl::isOrContainsUnion"),
      add_named_node(&mut g, 4294968411, "clang::RecordDecl::fields"),
      add_named_node(&mut g, 4294968412, "clang::RecordDecl::field_begin"),
      add_named_node(&mut g, 4294968413, "clang::RecordDecl::field_end"),
      add_named_node(&mut g, 4294968414, "clang::RecordDecl::field_empty"),
      add_named_node(&mut g, 4294968415, "clang::RecordDecl::findFirstNamedDataMember"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_RECORDDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NONTYPETEMPLATEPARMDECL, "clang::NonTypeTemplateParmDecl");
  g.add_edge((CLASS_NONTYPETEMPLATEPARMDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NONTYPETEMPLATEPARMDECL, META_SUBCLASS, CLASS_DECLARATORDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968417, "clang::NonTypeTemplateParmDecl::getSourceRange"),
      add_named_node(&mut g, 4294968418, "clang::NonTypeTemplateParmDecl::getDefaultArgStorage"),
      add_named_node(&mut g, 4294968419, "clang::NonTypeTemplateParmDecl::hasDefaultArgument"),
      add_named_node(&mut g, 4294968420, "clang::NonTypeTemplateParmDecl::getDefaultArgument"),
      add_named_node(&mut g, 4294968421, "clang::NonTypeTemplateParmDecl::getDefaultArgumentLoc"),
      add_named_node(&mut g, 4294968422, "clang::NonTypeTemplateParmDecl::defaultArgumentWasInherited"),
      add_named_node(&mut g, 4294968423, "clang::NonTypeTemplateParmDecl::isParameterPack"),
      add_named_node(&mut g, 4294968424, "clang::NonTypeTemplateParmDecl::isPackExpansion"),
      add_named_node(&mut g, 4294968425, "clang::NonTypeTemplateParmDecl::isExpandedParameterPack"),
      add_named_node(&mut g, 4294968426, "clang::NonTypeTemplateParmDecl::getNumExpansionTypes"),
      add_named_node(&mut g, 4294968427, "clang::NonTypeTemplateParmDecl::getPlaceholderTypeConstraint"),
      add_named_node(&mut g, 4294968428, "clang::NonTypeTemplateParmDecl::hasPlaceholderTypeConstraint"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NONTYPETEMPLATEPARMDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCIVARDECL, "clang::ObjCIvarDecl");
  g.add_edge((CLASS_OBJCIVARDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCIVARDECL, META_SUBCLASS, CLASS_FIELDDECL));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCIVARDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CLASSTEMPLATEDECL, "clang::ClassTemplateDecl");
  g.add_edge((CLASS_CLASSTEMPLATEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CLASSTEMPLATEDECL, META_SUBCLASS, CLASS_REDECLARABLETEMPLATEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968431, "clang::ClassTemplateDecl::getTemplatedDecl"),
      add_named_node(&mut g, 4294968432, "clang::ClassTemplateDecl::isThisDeclarationADefinition"),
      add_named_node(&mut g, 4294968433, "clang::ClassTemplateDecl::getCanonicalDecl"),
      add_named_node(&mut g, 4294968434, "clang::ClassTemplateDecl::getPreviousDecl"),
      add_named_node(&mut g, 4294968435, "clang::ClassTemplateDecl::getMostRecentDecl"),
      add_named_node(&mut g, 4294968436, "clang::ClassTemplateDecl::getInstantiatedFromMemberTemplate"),
      add_named_node(&mut g, 4294968437, "clang::ClassTemplateDecl::specializations"),
      add_named_node(&mut g, 4294968438, "clang::ClassTemplateDecl::spec_begin"),
      add_named_node(&mut g, 4294968439, "clang::ClassTemplateDecl::spec_end"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CLASSTEMPLATEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCTYPEPARAMDECL, "clang::ObjCTypeParamDecl");
  g.add_edge((CLASS_OBJCTYPEPARAMDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCTYPEPARAMDECL, META_SUBCLASS, CLASS_TYPEDEFNAMEDECL));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCTYPEPARAMDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TEMPLATETEMPLATEPARMDECL, "clang::TemplateTemplateParmDecl");
  g.add_edge((CLASS_TEMPLATETEMPLATEPARMDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TEMPLATETEMPLATEPARMDECL, META_SUBCLASS, CLASS_TEMPLATEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968442, "clang::TemplateTemplateParmDecl::isParameterPack"),
      add_named_node(&mut g, 4294968443, "clang::TemplateTemplateParmDecl::isPackExpansion"),
      add_named_node(&mut g, 4294968444, "clang::TemplateTemplateParmDecl::isExpandedParameterPack"),
      add_named_node(&mut g, 4294968445, "clang::TemplateTemplateParmDecl::getNumExpansionTemplateParameters"),
      add_named_node(&mut g, 4294968446, "clang::TemplateTemplateParmDecl::getDefaultArgStorage"),
      add_named_node(&mut g, 4294968447, "clang::TemplateTemplateParmDecl::hasDefaultArgument"),
      add_named_node(&mut g, 4294968448, "clang::TemplateTemplateParmDecl::getDefaultArgument"),
      add_named_node(&mut g, 4294968449, "clang::TemplateTemplateParmDecl::getDefaultArgumentLoc"),
      add_named_node(&mut g, 4294968450, "clang::TemplateTemplateParmDecl::defaultArgumentWasInherited"),
      add_named_node(&mut g, 4294968451, "clang::TemplateTemplateParmDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TEMPLATETEMPLATEPARMDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCCONTAINERDECL, "clang::ObjCContainerDecl");
  g.add_edge((CLASS_OBJCCONTAINERDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCCONTAINERDECL, META_SUBCLASS, CLASS_NAMEDDECL));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCCONTAINERDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCPROPERTYDECL, "clang::ObjCPropertyDecl");
  g.add_edge((CLASS_OBJCPROPERTYDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCPROPERTYDECL, META_SUBCLASS, CLASS_NAMEDDECL));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCPROPERTYDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_UNRESOLVEDUSINGIFEXISTSDECL, "clang::UnresolvedUsingIfExistsDecl");
  g.add_edge((CLASS_UNRESOLVEDUSINGIFEXISTSDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNRESOLVEDUSINGIFEXISTSDECL, META_SUBCLASS, CLASS_NAMEDDECL));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNRESOLVEDUSINGIFEXISTSDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCPROTOCOLDECL, "clang::ObjCProtocolDecl");
  g.add_edge((CLASS_OBJCPROTOCOLDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCPROTOCOLDECL, META_SUBCLASS, CLASS_OBJCCONTAINERDECL));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCPROTOCOLDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPORDEREDDIRECTIVE, "clang::OMPOrderedDirective");
  g.add_edge((CLASS_OMPORDEREDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPORDEREDDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPORDEREDDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXDELETEEXPR, "clang::CXXDeleteExpr");
  g.add_edge((CLASS_CXXDELETEEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXDELETEEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968458, "clang::CXXDeleteExpr::isGlobalDelete"),
      add_named_node(&mut g, 4294968459, "clang::CXXDeleteExpr::isArrayForm"),
      add_named_node(&mut g, 4294968460, "clang::CXXDeleteExpr::isArrayFormAsWritten"),
      add_named_node(&mut g, 4294968461, "clang::CXXDeleteExpr::doesUsualArrayDeleteWantSize"),
      add_named_node(&mut g, 4294968462, "clang::CXXDeleteExpr::getOperatorDelete"),
      add_named_node(&mut g, 4294968463, "clang::CXXDeleteExpr::getArgument"),
      add_named_node(&mut g, 4294968464, "clang::CXXDeleteExpr::getDestroyedType"),
      add_named_node(&mut g, 4294968465, "clang::CXXDeleteExpr::getBeginLoc"),
      add_named_node(&mut g, 4294968466, "clang::CXXDeleteExpr::getEndLoc"),
      add_named_node(&mut g, 4294968467, "clang::CXXDeleteExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXDELETEEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_STATICASSERTDECL, "clang::StaticAssertDecl");
  g.add_edge((CLASS_STATICASSERTDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_STATICASSERTDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968469, "clang::StaticAssertDecl::getAssertExpr"),
      add_named_node(&mut g, 4294968470, "clang::StaticAssertDecl::getMessage"),
      add_named_node(&mut g, 4294968471, "clang::StaticAssertDecl::isFailed"),
      add_named_node(&mut g, 4294968472, "clang::StaticAssertDecl::getRParenLoc"),
      add_named_node(&mut g, 4294968473, "clang::StaticAssertDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_STATICASSERTDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPALLOCATEDECL, "clang::OMPAllocateDecl");
  g.add_edge((CLASS_OMPALLOCATEDECL, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPALLOCATEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FILESCOPEASMDECL, "clang::FileScopeAsmDecl");
  g.add_edge((CLASS_FILESCOPEASMDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FILESCOPEASMDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968476, "clang::FileScopeAsmDecl::getAsmLoc"),
      add_named_node(&mut g, 4294968477, "clang::FileScopeAsmDecl::getRParenLoc"),
      add_named_node(&mut g, 4294968478, "clang::FileScopeAsmDecl::getSourceRange"),
      add_named_node(&mut g, 4294968479, "clang::FileScopeAsmDecl::getAsmString"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FILESCOPEASMDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_VARDECL, "clang::VarDecl");
  g.add_edge((CLASS_VARDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_VARDECL, META_SUBCLASS, CLASS_DECLARATORDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968481, "clang::VarDecl::getSourceRange"),
      add_named_node(&mut g, 4294968482, "clang::VarDecl::getStorageClass"),
      add_named_node(&mut g, 4294968483, "clang::VarDecl::getTSCSpec"),
      add_named_node(&mut g, 4294968484, "clang::VarDecl::getTLSKind"),
      add_named_node(&mut g, 4294968485, "clang::VarDecl::hasLocalStorage"),
      add_named_node(&mut g, 4294968486, "clang::VarDecl::isStaticLocal"),
      add_named_node(&mut g, 4294968487, "clang::VarDecl::hasExternalStorage"),
      add_named_node(&mut g, 4294968488, "clang::VarDecl::hasGlobalStorage"),
      add_named_node(&mut g, 4294968489, "clang::VarDecl::getStorageDuration"),
      add_named_node(&mut g, 4294968490, "clang::VarDecl::getLanguageLinkage"),
      add_named_node(&mut g, 4294968491, "clang::VarDecl::isExternC"),
      add_named_node(&mut g, 4294968492, "clang::VarDecl::isInExternCContext"),
      add_named_node(&mut g, 4294968493, "clang::VarDecl::isInExternCXXContext"),
      add_named_node(&mut g, 4294968494, "clang::VarDecl::isLocalVarDecl"),
      add_named_node(&mut g, 4294968495, "clang::VarDecl::isLocalVarDeclOrParm"),
      add_named_node(&mut g, 4294968496, "clang::VarDecl::isFunctionOrMethodVarDecl"),
      add_named_node(&mut g, 4294968497, "clang::VarDecl::isStaticDataMember"),
      add_named_node(&mut g, 4294968498, "clang::VarDecl::getCanonicalDecl"),
      add_named_node(&mut g, 4294968499, "clang::VarDecl::isThisDeclarationADefinition"),
      add_named_node(&mut g, 4294968500, "clang::VarDecl::hasDefinition"),
      add_named_node(&mut g, 4294968501, "clang::VarDecl::getActingDefinition"),
      add_named_node(&mut g, 4294968502, "clang::VarDecl::getDefinition"),
      add_named_node(&mut g, 4294968503, "clang::VarDecl::isOutOfLine"),
      add_named_node(&mut g, 4294968504, "clang::VarDecl::isFileVarDecl"),
      add_named_node(&mut g, 4294968505, "clang::VarDecl::getAnyInitializer"),
      add_named_node(&mut g, 4294968506, "clang::VarDecl::hasInit"),
      add_named_node(&mut g, 4294968507, "clang::VarDecl::getInit"),
      add_named_node(&mut g, 4294968508, "clang::VarDecl::getInitializingDeclaration"),
      add_named_node(&mut g, 4294968509, "clang::VarDecl::ensureEvaluatedStmt"),
      add_named_node(&mut g, 4294968510, "clang::VarDecl::getEvaluatedStmt"),
      add_named_node(&mut g, 4294968511, "clang::VarDecl::evaluateValue"),
      add_named_node(&mut g, 4294968512, "clang::VarDecl::getEvaluatedValue"),
      add_named_node(&mut g, 4294968513, "clang::VarDecl::hasConstantInitialization"),
      add_named_node(&mut g, 4294968514, "clang::VarDecl::getInitStyle"),
      add_named_node(&mut g, 4294968515, "clang::VarDecl::isDirectInit"),
      add_named_node(&mut g, 4294968516, "clang::VarDecl::isThisDeclarationADemotedDefinition"),
      add_named_node(&mut g, 4294968517, "clang::VarDecl::isExceptionVariable"),
      add_named_node(&mut g, 4294968518, "clang::VarDecl::isNRVOVariable"),
      add_named_node(&mut g, 4294968519, "clang::VarDecl::isCXXForRangeDecl"),
      add_named_node(&mut g, 4294968520, "clang::VarDecl::isObjCForDecl"),
      add_named_node(&mut g, 4294968521, "clang::VarDecl::isARCPseudoStrong"),
      add_named_node(&mut g, 4294968522, "clang::VarDecl::isInline"),
      add_named_node(&mut g, 4294968523, "clang::VarDecl::isInlineSpecified"),
      add_named_node(&mut g, 4294968524, "clang::VarDecl::isConstexpr"),
      add_named_node(&mut g, 4294968525, "clang::VarDecl::isInitCapture"),
      add_named_node(&mut g, 4294968526, "clang::VarDecl::isParameterPack"),
      add_named_node(&mut g, 4294968527, "clang::VarDecl::isPreviousDeclInSameBlockScope"),
      add_named_node(&mut g, 4294968528, "clang::VarDecl::isEscapingByref"),
      add_named_node(&mut g, 4294968529, "clang::VarDecl::isNonEscapingByref"),
      add_named_node(&mut g, 4294968530, "clang::VarDecl::hasDependentAlignment"),
      add_named_node(&mut g, 4294968531, "clang::VarDecl::getTemplateInstantiationPattern"),
      add_named_node(&mut g, 4294968532, "clang::VarDecl::getInstantiatedFromStaticDataMember"),
      add_named_node(&mut g, 4294968533, "clang::VarDecl::getTemplateSpecializationKind"),
      add_named_node(&mut g, 4294968534, "clang::VarDecl::getTemplateSpecializationKindForInstantiation"),
      add_named_node(&mut g, 4294968535, "clang::VarDecl::getPointOfInstantiation"),
      add_named_node(&mut g, 4294968536, "clang::VarDecl::getMemberSpecializationInfo"),
      add_named_node(&mut g, 4294968537, "clang::VarDecl::getDescribedVarTemplate"),
      add_named_node(&mut g, 4294968538, "clang::VarDecl::isKnownToBeDefined"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_VARDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_UNRESOLVEDUSINGTYPENAMEDECL, "clang::UnresolvedUsingTypenameDecl");
  g.add_edge((CLASS_UNRESOLVEDUSINGTYPENAMEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNRESOLVEDUSINGTYPENAMEDECL, META_SUBCLASS, CLASS_TYPEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968540, "clang::UnresolvedUsingTypenameDecl::getUsingLoc"),
      add_named_node(&mut g, 4294968541, "clang::UnresolvedUsingTypenameDecl::getTypenameLoc"),
      add_named_node(&mut g, 4294968542, "clang::UnresolvedUsingTypenameDecl::getQualifierLoc"),
      add_named_node(&mut g, 4294968543, "clang::UnresolvedUsingTypenameDecl::getQualifier"),
      add_named_node(&mut g, 4294968544, "clang::UnresolvedUsingTypenameDecl::getNameInfo"),
      add_named_node(&mut g, 4294968545, "clang::UnresolvedUsingTypenameDecl::isPackExpansion"),
      add_named_node(&mut g, 4294968546, "clang::UnresolvedUsingTypenameDecl::getEllipsisLoc"),
      add_named_node(&mut g, 4294968547, "clang::UnresolvedUsingTypenameDecl::getCanonicalDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNRESOLVEDUSINGTYPENAMEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTARGETPARALLELGENERICLOOPDIRECTIVE, "clang::OMPTargetParallelGenericLoopDirective");
  g.add_edge((CLASS_OMPTARGETPARALLELGENERICLOOPDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETPARALLELGENERICLOOPDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTARGETPARALLELGENERICLOOPDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTHREADPRIVATEDECL, "clang::OMPThreadPrivateDecl");
  g.add_edge((CLASS_OMPTHREADPRIVATEDECL, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTHREADPRIVATEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BINDINGDECL, "clang::BindingDecl");
  g.add_edge((CLASS_BINDINGDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BINDINGDECL, META_SUBCLASS, CLASS_VALUEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968551, "clang::BindingDecl::getBinding"),
      add_named_node(&mut g, 4294968552, "clang::BindingDecl::getDecomposedDecl"),
      add_named_node(&mut g, 4294968553, "clang::BindingDecl::getHoldingVar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BINDINGDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TEMPLATEDECL, "clang::TemplateDecl");
  g.add_edge((CLASS_TEMPLATEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TEMPLATEDECL, META_SUBCLASS, CLASS_NAMEDDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968555, "clang::TemplateDecl::getTemplateParameters"),
      add_named_node(&mut g, 4294968556, "clang::TemplateDecl::hasAssociatedConstraints"),
      add_named_node(&mut g, 4294968557, "clang::TemplateDecl::getTemplatedDecl"),
      add_named_node(&mut g, 4294968558, "clang::TemplateDecl::isTypeAlias"),
      add_named_node(&mut g, 4294968559, "clang::TemplateDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TEMPLATEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FRIENDTEMPLATEDECL, "clang::FriendTemplateDecl");
  g.add_edge((CLASS_FRIENDTEMPLATEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FRIENDTEMPLATEDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968561, "clang::FriendTemplateDecl::getFriendType"),
      add_named_node(&mut g, 4294968562, "clang::FriendTemplateDecl::getFriendDecl"),
      add_named_node(&mut g, 4294968563, "clang::FriendTemplateDecl::getFriendLoc"),
      add_named_node(&mut g, 4294968564, "clang::FriendTemplateDecl::getNumTemplateParameters"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FRIENDTEMPLATEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXOPERATORCALLEXPR, "clang::CXXOperatorCallExpr");
  g.add_edge((CLASS_CXXOPERATORCALLEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXOPERATORCALLEXPR, META_SUBCLASS, CLASS_CALLEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968566, "clang::CXXOperatorCallExpr::getOperator"),
      add_named_node(&mut g, 4294968567, "clang::CXXOperatorCallExpr::isAssignmentOp"),
      add_named_node(&mut g, 4294968568, "clang::CXXOperatorCallExpr::isComparisonOp"),
      add_named_node(&mut g, 4294968569, "clang::CXXOperatorCallExpr::isInfixBinaryOp"),
      add_named_node(&mut g, 4294968570, "clang::CXXOperatorCallExpr::getOperatorLoc"),
      add_named_node(&mut g, 4294968571, "clang::CXXOperatorCallExpr::getExprLoc"),
      add_named_node(&mut g, 4294968572, "clang::CXXOperatorCallExpr::getBeginLoc"),
      add_named_node(&mut g, 4294968573, "clang::CXXOperatorCallExpr::getEndLoc"),
      add_named_node(&mut g, 4294968574, "clang::CXXOperatorCallExpr::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXOPERATORCALLEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCPROPERTYIMPLDECL, "clang::ObjCPropertyImplDecl");
  g.add_edge((CLASS_OBJCPROPERTYIMPLDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCPROPERTYIMPLDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCPROPERTYIMPLDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_EMPTYDECL, "clang::EmptyDecl");
  g.add_edge((CLASS_EMPTYDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_EMPTYDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_EMPTYDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CONTINUESTMT, "clang::ContinueStmt");
  g.add_edge((CLASS_CONTINUESTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONTINUESTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968578, "clang::ContinueStmt::getContinueLoc"),
      add_named_node(&mut g, 4294968579, "clang::ContinueStmt::getBeginLoc"),
      add_named_node(&mut g, 4294968580, "clang::ContinueStmt::getEndLoc"),
      add_named_node(&mut g, 4294968581, "clang::ContinueStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CONTINUESTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_IMPORTDECL, "clang::ImportDecl");
  g.add_edge((CLASS_IMPORTDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_IMPORTDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968583, "clang::ImportDecl::getImportedModule"),
      add_named_node(&mut g, 4294968584, "clang::ImportDecl::getIdentifierLocs"),
      add_named_node(&mut g, 4294968585, "clang::ImportDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_IMPORTDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_USINGDIRECTIVEDECL, "clang::UsingDirectiveDecl");
  g.add_edge((CLASS_USINGDIRECTIVEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_USINGDIRECTIVEDECL, META_SUBCLASS, CLASS_NAMEDDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968587, "clang::UsingDirectiveDecl::getQualifierLoc"),
      add_named_node(&mut g, 4294968588, "clang::UsingDirectiveDecl::getQualifier"),
      add_named_node(&mut g, 4294968589, "clang::UsingDirectiveDecl::getNominatedNamespaceAsWritten"),
      add_named_node(&mut g, 4294968590, "clang::UsingDirectiveDecl::getNominatedNamespace"),
      add_named_node(&mut g, 4294968591, "clang::UsingDirectiveDecl::getCommonAncestor"),
      add_named_node(&mut g, 4294968592, "clang::UsingDirectiveDecl::getUsingLoc"),
      add_named_node(&mut g, 4294968593, "clang::UsingDirectiveDecl::getNamespaceKeyLocation"),
      add_named_node(&mut g, 4294968594, "clang::UsingDirectiveDecl::getIdentLocation"),
      add_named_node(&mut g, 4294968595, "clang::UsingDirectiveDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_USINGDIRECTIVEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_IMPLICITCONCEPTSPECIALIZATIONDECL, "clang::ImplicitConceptSpecializationDecl");
  g.add_edge((CLASS_IMPLICITCONCEPTSPECIALIZATIONDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_IMPLICITCONCEPTSPECIALIZATIONDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968597, "clang::ImplicitConceptSpecializationDecl::getTemplateArguments"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_IMPLICITCONCEPTSPECIALIZATIONDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_VARTEMPLATESPECIALIZATIONDECL, "clang::VarTemplateSpecializationDecl");
  g.add_edge((CLASS_VARTEMPLATESPECIALIZATIONDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_VARTEMPLATESPECIALIZATIONDECL, META_SUBCLASS, CLASS_VARDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968599, "clang::VarTemplateSpecializationDecl::getSpecializedTemplate"),
      add_named_node(&mut g, 4294968600, "clang::VarTemplateSpecializationDecl::getTemplateArgs"),
      add_named_node(&mut g, 4294968601, "clang::VarTemplateSpecializationDecl::getTemplateArgsInfo"),
      add_named_node(&mut g, 4294968602, "clang::VarTemplateSpecializationDecl::getSpecializationKind"),
      add_named_node(&mut g, 4294968603, "clang::VarTemplateSpecializationDecl::isExplicitSpecialization"),
      add_named_node(&mut g, 4294968604, "clang::VarTemplateSpecializationDecl::isClassScopeExplicitSpecialization"),
      add_named_node(&mut g, 4294968605, "clang::VarTemplateSpecializationDecl::isExplicitInstantiationOrSpecialization"),
      add_named_node(&mut g, 4294968606, "clang::VarTemplateSpecializationDecl::getPointOfInstantiation"),
      add_named_node(&mut g, 4294968607, "clang::VarTemplateSpecializationDecl::getInstantiatedFrom"),
      add_named_node(&mut g, 4294968608, "clang::VarTemplateSpecializationDecl::getSpecializedTemplateOrPartial"),
      add_named_node(&mut g, 4294968609, "clang::VarTemplateSpecializationDecl::getTemplateInstantiationArgs"),
      add_named_node(&mut g, 4294968610, "clang::VarTemplateSpecializationDecl::getTypeAsWritten"),
      add_named_node(&mut g, 4294968611, "clang::VarTemplateSpecializationDecl::getExternLoc"),
      add_named_node(&mut g, 4294968612, "clang::VarTemplateSpecializationDecl::getTemplateKeywordLoc"),
      add_named_node(&mut g, 4294968613, "clang::VarTemplateSpecializationDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_VARTEMPLATESPECIALIZATIONDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_REQUIRESEXPRBODYDECL, "clang::RequiresExprBodyDecl");
  g.add_edge((CLASS_REQUIRESEXPRBODYDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_REQUIRESEXPRBODYDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_REQUIRESEXPRBODYDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FRIENDDECL, "clang::FriendDecl");
  g.add_edge((CLASS_FRIENDDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FRIENDDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968616, "clang::FriendDecl::getFriendType"),
      add_named_node(&mut g, 4294968617, "clang::FriendDecl::getFriendTypeNumTemplateParameterLists"),
      add_named_node(&mut g, 4294968618, "clang::FriendDecl::getFriendDecl"),
      add_named_node(&mut g, 4294968619, "clang::FriendDecl::getFriendLoc"),
      add_named_node(&mut g, 4294968620, "clang::FriendDecl::getSourceRange"),
      add_named_node(&mut g, 4294968621, "clang::FriendDecl::isUnsupportedFriend"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FRIENDDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TEMPLATETYPEPARMDECL, "clang::TemplateTypeParmDecl");
  g.add_edge((CLASS_TEMPLATETYPEPARMDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TEMPLATETYPEPARMDECL, META_SUBCLASS, CLASS_TYPEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968623, "clang::TemplateTypeParmDecl::wasDeclaredWithTypename"),
      add_named_node(&mut g, 4294968624, "clang::TemplateTypeParmDecl::getDefaultArgStorage"),
      add_named_node(&mut g, 4294968625, "clang::TemplateTypeParmDecl::hasDefaultArgument"),
      add_named_node(&mut g, 4294968626, "clang::TemplateTypeParmDecl::getDefaultArgument"),
      add_named_node(&mut g, 4294968627, "clang::TemplateTypeParmDecl::getDefaultArgumentInfo"),
      add_named_node(&mut g, 4294968628, "clang::TemplateTypeParmDecl::getDefaultArgumentLoc"),
      add_named_node(&mut g, 4294968629, "clang::TemplateTypeParmDecl::defaultArgumentWasInherited"),
      add_named_node(&mut g, 4294968630, "clang::TemplateTypeParmDecl::getDepth"),
      add_named_node(&mut g, 4294968631, "clang::TemplateTypeParmDecl::getIndex"),
      add_named_node(&mut g, 4294968632, "clang::TemplateTypeParmDecl::isParameterPack"),
      add_named_node(&mut g, 4294968633, "clang::TemplateTypeParmDecl::isPackExpansion"),
      add_named_node(&mut g, 4294968634, "clang::TemplateTypeParmDecl::isExpandedParameterPack"),
      add_named_node(&mut g, 4294968635, "clang::TemplateTypeParmDecl::getNumExpansionParameters"),
      add_named_node(&mut g, 4294968636, "clang::TemplateTypeParmDecl::getTypeConstraint"),
      add_named_node(&mut g, 4294968637, "clang::TemplateTypeParmDecl::hasTypeConstraint"),
      add_named_node(&mut g, 4294968638, "clang::TemplateTypeParmDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TEMPLATETYPEPARMDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ENUMDECL, "clang::EnumDecl");
  g.add_edge((CLASS_ENUMDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ENUMDECL, META_SUBCLASS, CLASS_TAGDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968640, "clang::EnumDecl::getCanonicalDecl"),
      add_named_node(&mut g, 4294968641, "clang::EnumDecl::getPreviousDecl"),
      add_named_node(&mut g, 4294968642, "clang::EnumDecl::getMostRecentDecl"),
      add_named_node(&mut g, 4294968643, "clang::EnumDecl::getDefinition"),
      add_named_node(&mut g, 4294968644, "clang::EnumDecl::getSourceRange"),
      add_named_node(&mut g, 4294968645, "clang::EnumDecl::enumerators"),
      add_named_node(&mut g, 4294968646, "clang::EnumDecl::enumerator_begin"),
      add_named_node(&mut g, 4294968647, "clang::EnumDecl::enumerator_end"),
      add_named_node(&mut g, 4294968648, "clang::EnumDecl::getPromotionType"),
      add_named_node(&mut g, 4294968649, "clang::EnumDecl::getIntegerType"),
      add_named_node(&mut g, 4294968650, "clang::EnumDecl::getIntegerTypeSourceInfo"),
      add_named_node(&mut g, 4294968651, "clang::EnumDecl::getIntegerTypeRange"),
      add_named_node(&mut g, 4294968652, "clang::EnumDecl::getNumPositiveBits"),
      add_named_node(&mut g, 4294968653, "clang::EnumDecl::getNumNegativeBits"),
      add_named_node(&mut g, 4294968654, "clang::EnumDecl::isScoped"),
      add_named_node(&mut g, 4294968655, "clang::EnumDecl::isScopedUsingClassTag"),
      add_named_node(&mut g, 4294968656, "clang::EnumDecl::isFixed"),
      add_named_node(&mut g, 4294968657, "clang::EnumDecl::isComplete"),
      add_named_node(&mut g, 4294968658, "clang::EnumDecl::isClosed"),
      add_named_node(&mut g, 4294968659, "clang::EnumDecl::isClosedFlag"),
      add_named_node(&mut g, 4294968660, "clang::EnumDecl::isClosedNonFlag"),
      add_named_node(&mut g, 4294968661, "clang::EnumDecl::getTemplateInstantiationPattern"),
      add_named_node(&mut g, 4294968662, "clang::EnumDecl::getInstantiatedFromMemberEnum"),
      add_named_node(&mut g, 4294968663, "clang::EnumDecl::getTemplateSpecializationKind"),
      add_named_node(&mut g, 4294968664, "clang::EnumDecl::getMemberSpecializationInfo"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ENUMDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ACCESSSPECDECL, "clang::AccessSpecDecl");
  g.add_edge((CLASS_ACCESSSPECDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ACCESSSPECDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968666, "clang::AccessSpecDecl::getAccessSpecifierLoc"),
      add_named_node(&mut g, 4294968667, "clang::AccessSpecDecl::getColonLoc"),
      add_named_node(&mut g, 4294968668, "clang::AccessSpecDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ACCESSSPECDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCIMPLDECL, "clang::ObjCImplDecl");
  g.add_edge((CLASS_OBJCIMPLDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCIMPLDECL, META_SUBCLASS, CLASS_OBJCCONTAINERDECL));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCIMPLDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MSGUIDDECL, "clang::MSGuidDecl");
  g.add_edge((CLASS_MSGUIDDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MSGUIDDECL, META_SUBCLASS, CLASS_VALUEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968671, "clang::MSGuidDecl::getParts"),
      add_named_node(&mut g, 4294968672, "clang::MSGuidDecl::getAsAPValue"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MSGUIDDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPCAPTUREDEXPRDECL, "clang::OMPCapturedExprDecl");
  g.add_edge((CLASS_OMPCAPTUREDEXPRDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPCAPTUREDEXPRDECL, META_SUBCLASS, CLASS_VARDECL));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPCAPTUREDEXPRDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_USINGPACKDECL, "clang::UsingPackDecl");
  g.add_edge((CLASS_USINGPACKDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_USINGPACKDECL, META_SUBCLASS, CLASS_NAMEDDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968675, "clang::UsingPackDecl::getInstantiatedFromUsingDecl"),
      add_named_node(&mut g, 4294968676, "clang::UsingPackDecl::expansions"),
      add_named_node(&mut g, 4294968677, "clang::UsingPackDecl::getSourceRange"),
      add_named_node(&mut g, 4294968678, "clang::UsingPackDecl::getCanonicalDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_USINGPACKDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPEALIASTEMPLATEDECL, "clang::TypeAliasTemplateDecl");
  g.add_edge((CLASS_TYPEALIASTEMPLATEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPEALIASTEMPLATEDECL, META_SUBCLASS, CLASS_REDECLARABLETEMPLATEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968680, "clang::TypeAliasTemplateDecl::getTemplatedDecl"),
      add_named_node(&mut g, 4294968681, "clang::TypeAliasTemplateDecl::getCanonicalDecl"),
      add_named_node(&mut g, 4294968682, "clang::TypeAliasTemplateDecl::getPreviousDecl"),
      add_named_node(&mut g, 4294968683, "clang::TypeAliasTemplateDecl::getInstantiatedFromMemberTemplate"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPEALIASTEMPLATEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_VALUEDECL, "clang::ValueDecl");
  g.add_edge((CLASS_VALUEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_VALUEDECL, META_SUBCLASS, CLASS_NAMEDDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968685, "clang::ValueDecl::getType"),
      add_named_node(&mut g, 4294968686, "clang::ValueDecl::isWeak"),
      add_named_node(&mut g, 4294968687, "clang::ValueDecl::isInitCapture"),
      add_named_node(&mut g, 4294968688, "clang::ValueDecl::getPotentiallyDecomposedVarDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_VALUEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TOPLEVELSTMTDECL, "clang::TopLevelStmtDecl");
  g.add_edge((CLASS_TOPLEVELSTMTDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TOPLEVELSTMTDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968690, "clang::TopLevelStmtDecl::getSourceRange"),
      add_named_node(&mut g, 4294968691, "clang::TopLevelStmtDecl::getStmt"),
      add_named_node(&mut g, 4294968692, "clang::TopLevelStmtDecl::isSemiMissing"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TOPLEVELSTMTDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXNAMEDCASTEXPR, "clang::CXXNamedCastExpr");
  g.add_edge((CLASS_CXXNAMEDCASTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXNAMEDCASTEXPR, META_SUBCLASS, CLASS_EXPLICITCASTEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968694, "clang::CXXNamedCastExpr::getCastName"),
      add_named_node(&mut g, 4294968695, "clang::CXXNamedCastExpr::getOperatorLoc"),
      add_named_node(&mut g, 4294968696, "clang::CXXNamedCastExpr::getRParenLoc"),
      add_named_node(&mut g, 4294968697, "clang::CXXNamedCastExpr::getBeginLoc"),
      add_named_node(&mut g, 4294968698, "clang::CXXNamedCastExpr::getEndLoc"),
      add_named_node(&mut g, 4294968699, "clang::CXXNamedCastExpr::getAngleBrackets"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXNAMEDCASTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPEDEFNAMEDECL, "clang::TypedefNameDecl");
  g.add_edge((CLASS_TYPEDEFNAMEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPEDEFNAMEDECL, META_SUBCLASS, CLASS_TYPEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968701, "clang::TypedefNameDecl::isModed"),
      add_named_node(&mut g, 4294968702, "clang::TypedefNameDecl::getTypeSourceInfo"),
      add_named_node(&mut g, 4294968703, "clang::TypedefNameDecl::getUnderlyingType"),
      add_named_node(&mut g, 4294968704, "clang::TypedefNameDecl::getCanonicalDecl"),
      add_named_node(&mut g, 4294968705, "clang::TypedefNameDecl::isTransparentTag"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPEDEFNAMEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CAPTUREDDECL, "clang::CapturedDecl");
  g.add_edge((CLASS_CAPTUREDDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CAPTUREDDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968707, "clang::CapturedDecl::getBody"),
      add_named_node(&mut g, 4294968708, "clang::CapturedDecl::isNothrow"),
      add_named_node(&mut g, 4294968709, "clang::CapturedDecl::getNumParams"),
      add_named_node(&mut g, 4294968710, "clang::CapturedDecl::parameters"),
      add_named_node(&mut g, 4294968711, "clang::CapturedDecl::getContextParam"),
      add_named_node(&mut g, 4294968712, "clang::CapturedDecl::getContextParamPosition"),
      add_named_node(&mut g, 4294968713, "clang::CapturedDecl::param_begin"),
      add_named_node(&mut g, 4294968714, "clang::CapturedDecl::param_end"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CAPTUREDDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPEDEFDECL, "clang::TypedefDecl");
  g.add_edge((CLASS_TYPEDEFDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPEDEFDECL, META_SUBCLASS, CLASS_TYPEDEFNAMEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968716, "clang::TypedefDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPEDEFDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPDECLAREREDUCTIONDECL, "clang::OMPDeclareReductionDecl");
  g.add_edge((CLASS_OMPDECLAREREDUCTIONDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPDECLAREREDUCTIONDECL, META_SUBCLASS, CLASS_VALUEDECL));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPDECLAREREDUCTIONDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BUILTINTEMPLATEDECL, "clang::BuiltinTemplateDecl");
  g.add_edge((CLASS_BUILTINTEMPLATEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BUILTINTEMPLATEDECL, META_SUBCLASS, CLASS_TEMPLATEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968719, "clang::BuiltinTemplateDecl::getSourceRange"),
      add_named_node(&mut g, 4294968720, "clang::BuiltinTemplateDecl::getBuiltinTemplateKind"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BUILTINTEMPLATEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPDECLAREMAPPERDECL, "clang::OMPDeclareMapperDecl");
  g.add_edge((CLASS_OMPDECLAREMAPPERDECL, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPDECLAREMAPPERDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_UNRESOLVEDUSINGVALUEDECL, "clang::UnresolvedUsingValueDecl");
  g.add_edge((CLASS_UNRESOLVEDUSINGVALUEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNRESOLVEDUSINGVALUEDECL, META_SUBCLASS, CLASS_VALUEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968723, "clang::UnresolvedUsingValueDecl::getUsingLoc"),
      add_named_node(&mut g, 4294968724, "clang::UnresolvedUsingValueDecl::isAccessDeclaration"),
      add_named_node(&mut g, 4294968725, "clang::UnresolvedUsingValueDecl::getQualifierLoc"),
      add_named_node(&mut g, 4294968726, "clang::UnresolvedUsingValueDecl::getQualifier"),
      add_named_node(&mut g, 4294968727, "clang::UnresolvedUsingValueDecl::getNameInfo"),
      add_named_node(&mut g, 4294968728, "clang::UnresolvedUsingValueDecl::isPackExpansion"),
      add_named_node(&mut g, 4294968729, "clang::UnresolvedUsingValueDecl::getEllipsisLoc"),
      add_named_node(&mut g, 4294968730, "clang::UnresolvedUsingValueDecl::getSourceRange"),
      add_named_node(&mut g, 4294968731, "clang::UnresolvedUsingValueDecl::getCanonicalDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNRESOLVEDUSINGVALUEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_VARTEMPLATEDECL, "clang::VarTemplateDecl");
  g.add_edge((CLASS_VARTEMPLATEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_VARTEMPLATEDECL, META_SUBCLASS, CLASS_REDECLARABLETEMPLATEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968733, "clang::VarTemplateDecl::getTemplatedDecl"),
      add_named_node(&mut g, 4294968734, "clang::VarTemplateDecl::isThisDeclarationADefinition"),
      add_named_node(&mut g, 4294968735, "clang::VarTemplateDecl::getCanonicalDecl"),
      add_named_node(&mut g, 4294968736, "clang::VarTemplateDecl::getPreviousDecl"),
      add_named_node(&mut g, 4294968737, "clang::VarTemplateDecl::getMostRecentDecl"),
      add_named_node(&mut g, 4294968738, "clang::VarTemplateDecl::getInstantiatedFromMemberTemplate"),
      add_named_node(&mut g, 4294968739, "clang::VarTemplateDecl::specializations"),
      add_named_node(&mut g, 4294968740, "clang::VarTemplateDecl::spec_begin"),
      add_named_node(&mut g, 4294968741, "clang::VarTemplateDecl::spec_end"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_VARTEMPLATEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_INDIRECTFIELDDECL, "clang::IndirectFieldDecl");
  g.add_edge((CLASS_INDIRECTFIELDDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_INDIRECTFIELDDECL, META_SUBCLASS, CLASS_VALUEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968743, "clang::IndirectFieldDecl::chain"),
      add_named_node(&mut g, 4294968744, "clang::IndirectFieldDecl::chain_begin"),
      add_named_node(&mut g, 4294968745, "clang::IndirectFieldDecl::chain_end"),
      add_named_node(&mut g, 4294968746, "clang::IndirectFieldDecl::getChainingSize"),
      add_named_node(&mut g, 4294968747, "clang::IndirectFieldDecl::getAnonField"),
      add_named_node(&mut g, 4294968748, "clang::IndirectFieldDecl::getVarDecl"),
      add_named_node(&mut g, 4294968749, "clang::IndirectFieldDecl::getCanonicalDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_INDIRECTFIELDDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FIELDDECL, "clang::FieldDecl");
  g.add_edge((CLASS_FIELDDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FIELDDECL, META_SUBCLASS, CLASS_DECLARATORDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968751, "clang::FieldDecl::getFieldIndex"),
      add_named_node(&mut g, 4294968752, "clang::FieldDecl::isMutable"),
      add_named_node(&mut g, 4294968753, "clang::FieldDecl::isBitField"),
      add_named_node(&mut g, 4294968754, "clang::FieldDecl::isUnnamedBitfield"),
      add_named_node(&mut g, 4294968755, "clang::FieldDecl::isAnonymousStructOrUnion"),
      add_named_node(&mut g, 4294968756, "clang::FieldDecl::getBitWidth"),
      add_named_node(&mut g, 4294968757, "clang::FieldDecl::isPotentiallyOverlapping"),
      add_named_node(&mut g, 4294968758, "clang::FieldDecl::getInClassInitStyle"),
      add_named_node(&mut g, 4294968759, "clang::FieldDecl::hasInClassInitializer"),
      add_named_node(&mut g, 4294968760, "clang::FieldDecl::hasNonNullInClassInitializer"),
      add_named_node(&mut g, 4294968761, "clang::FieldDecl::getInClassInitializer"),
      add_named_node(&mut g, 4294968762, "clang::FieldDecl::hasCapturedVLAType"),
      add_named_node(&mut g, 4294968763, "clang::FieldDecl::getCapturedVLAType"),
      add_named_node(&mut g, 4294968764, "clang::FieldDecl::getParent"),
      add_named_node(&mut g, 4294968765, "clang::FieldDecl::getSourceRange"),
      add_named_node(&mut g, 4294968766, "clang::FieldDecl::getCanonicalDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FIELDDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCATDEFSFIELDDECL, "clang::ObjCAtDefsFieldDecl");
  g.add_edge((CLASS_OBJCATDEFSFIELDDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCATDEFSFIELDDECL, META_SUBCLASS, CLASS_FIELDDECL));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCATDEFSFIELDDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXCONVERSIONDECL, "clang::CXXConversionDecl");
  g.add_edge((CLASS_CXXCONVERSIONDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXCONVERSIONDECL, META_SUBCLASS, CLASS_CXXMETHODDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968769, "clang::CXXConversionDecl::getExplicitSpecifier"),
      add_named_node(&mut g, 4294968770, "clang::CXXConversionDecl::isExplicit"),
      add_named_node(&mut g, 4294968771, "clang::CXXConversionDecl::getConversionType"),
      add_named_node(&mut g, 4294968772, "clang::CXXConversionDecl::isLambdaToBlockPointerConversion"),
      add_named_node(&mut g, 4294968773, "clang::CXXConversionDecl::getCanonicalDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXCONVERSIONDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXCONSTRUCTORDECL, "clang::CXXConstructorDecl");
  g.add_edge((CLASS_CXXCONSTRUCTORDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXCONSTRUCTORDECL, META_SUBCLASS, CLASS_CXXMETHODDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968775, "clang::CXXConstructorDecl::getExplicitSpecifier"),
      add_named_node(&mut g, 4294968776, "clang::CXXConstructorDecl::isExplicit"),
      add_named_node(&mut g, 4294968777, "clang::CXXConstructorDecl::inits"),
      add_named_node(&mut g, 4294968778, "clang::CXXConstructorDecl::init_begin"),
      add_named_node(&mut g, 4294968779, "clang::CXXConstructorDecl::init_end"),
      add_named_node(&mut g, 4294968780, "clang::CXXConstructorDecl::init_rbegin"),
      add_named_node(&mut g, 4294968781, "clang::CXXConstructorDecl::init_rend"),
      add_named_node(&mut g, 4294968782, "clang::CXXConstructorDecl::getNumCtorInitializers"),
      add_named_node(&mut g, 4294968783, "clang::CXXConstructorDecl::isDelegatingConstructor"),
      add_named_node(&mut g, 4294968784, "clang::CXXConstructorDecl::getTargetConstructor"),
      add_named_node(&mut g, 4294968785, "clang::CXXConstructorDecl::isDefaultConstructor"),
      add_named_node(&mut g, 4294968786, "clang::CXXConstructorDecl::isCopyConstructor"),
      add_named_node(&mut g, 4294968787, "clang::CXXConstructorDecl::isMoveConstructor"),
      add_named_node(&mut g, 4294968788, "clang::CXXConstructorDecl::isCopyOrMoveConstructor"),
      add_named_node(&mut g, 4294968789, "clang::CXXConstructorDecl::isSpecializationCopyingObject"),
      add_named_node(&mut g, 4294968790, "clang::CXXConstructorDecl::isInheritingConstructor"),
      add_named_node(&mut g, 4294968791, "clang::CXXConstructorDecl::getInheritedConstructor"),
      add_named_node(&mut g, 4294968792, "clang::CXXConstructorDecl::getCanonicalDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXCONSTRUCTORDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPCRITICALDIRECTIVE, "clang::OMPCriticalDirective");
  g.add_edge((CLASS_OMPCRITICALDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPCRITICALDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPCRITICALDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXTEMPORARYOBJECTEXPR, "clang::CXXTemporaryObjectExpr");
  g.add_edge((CLASS_CXXTEMPORARYOBJECTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXTEMPORARYOBJECTEXPR, META_SUBCLASS, CLASS_CXXCONSTRUCTEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968795, "clang::CXXTemporaryObjectExpr::getTypeSourceInfo"),
      add_named_node(&mut g, 4294968796, "clang::CXXTemporaryObjectExpr::getBeginLoc"),
      add_named_node(&mut g, 4294968797, "clang::CXXTemporaryObjectExpr::getEndLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXTEMPORARYOBJECTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXDEDUCTIONGUIDEDECL, "clang::CXXDeductionGuideDecl");
  g.add_edge((CLASS_CXXDEDUCTIONGUIDEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXDEDUCTIONGUIDEDECL, META_SUBCLASS, CLASS_FUNCTIONDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968799, "clang::CXXDeductionGuideDecl::getExplicitSpecifier"),
      add_named_node(&mut g, 4294968800, "clang::CXXDeductionGuideDecl::isExplicit"),
      add_named_node(&mut g, 4294968801, "clang::CXXDeductionGuideDecl::getDeducedTemplate"),
      add_named_node(&mut g, 4294968802, "clang::CXXDeductionGuideDecl::getCorrespondingConstructor"),
      add_named_node(&mut g, 4294968803, "clang::CXXDeductionGuideDecl::getDeductionCandidateKind"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXDEDUCTIONGUIDEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_UNINITIALIZEDATTR, "clang::UninitializedAttr");
  g.add_edge((CLASS_UNINITIALIZEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNINITIALIZEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNINITIALIZEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PRAGMACLANGRELROSECTIONATTR, "clang::PragmaClangRelroSectionAttr");
  g.add_edge((CLASS_PRAGMACLANGRELROSECTIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PRAGMACLANGRELROSECTIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PRAGMACLANGRELROSECTIONATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXNULLPTRLITERALEXPR, "clang::CXXNullPtrLiteralExpr");
  g.add_edge((CLASS_CXXNULLPTRLITERALEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXNULLPTRLITERALEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968807, "clang::CXXNullPtrLiteralExpr::getBeginLoc"),
      add_named_node(&mut g, 4294968808, "clang::CXXNullPtrLiteralExpr::getEndLoc"),
      add_named_node(&mut g, 4294968809, "clang::CXXNullPtrLiteralExpr::getLocation"),
      add_named_node(&mut g, 4294968810, "clang::CXXNullPtrLiteralExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXNULLPTRLITERALEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCPRECISELIFETIMEATTR, "clang::ObjCPreciseLifetimeAttr");
  g.add_edge((CLASS_OBJCPRECISELIFETIMEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCPRECISELIFETIMEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCPRECISELIFETIMEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_REFERENCETYPELOC, "clang::ReferenceTypeLoc");
  g.add_edge((CLASS_REFERENCETYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968813, "clang::ReferenceTypeLoc::getInnerType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_REFERENCETYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTASKLOOPDIRECTIVE, "clang::OMPTaskLoopDirective");
  g.add_edge((CLASS_OMPTASKLOOPDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTASKLOOPDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTASKLOOPDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCMETHODDECL, "clang::ObjCMethodDecl");
  g.add_edge((CLASS_OBJCMETHODDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCMETHODDECL, META_SUBCLASS, CLASS_NAMEDDECL));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCMETHODDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OSCONSUMESTHISATTR, "clang::OSConsumesThisAttr");
  g.add_edge((CLASS_OSCONSUMESTHISATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OSCONSUMESTHISATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OSCONSUMESTHISATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPPARALLELFORDIRECTIVE, "clang::OMPParallelForDirective");
  g.add_edge((CLASS_OMPPARALLELFORDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPPARALLELFORDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPPARALLELFORDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_VECTORCALLATTR, "clang::VectorCallAttr");
  g.add_edge((CLASS_VECTORCALLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_VECTORCALLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_VECTORCALLATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ANYX86INTERRUPTATTR, "clang::AnyX86InterruptAttr");
  g.add_edge((CLASS_ANYX86INTERRUPTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ANYX86INTERRUPTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ANYX86INTERRUPTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NOINSTRUMENTFUNCTIONATTR, "clang::NoInstrumentFunctionAttr");
  g.add_edge((CLASS_NOINSTRUMENTFUNCTIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOINSTRUMENTFUNCTIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NOINSTRUMENTFUNCTIONATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXFOLDEXPR, "clang::CXXFoldExpr");
  g.add_edge((CLASS_CXXFOLDEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXFOLDEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968822, "clang::CXXFoldExpr::getCallee"),
      add_named_node(&mut g, 4294968823, "clang::CXXFoldExpr::getLHS"),
      add_named_node(&mut g, 4294968824, "clang::CXXFoldExpr::getRHS"),
      add_named_node(&mut g, 4294968825, "clang::CXXFoldExpr::isRightFold"),
      add_named_node(&mut g, 4294968826, "clang::CXXFoldExpr::isLeftFold"),
      add_named_node(&mut g, 4294968827, "clang::CXXFoldExpr::getPattern"),
      add_named_node(&mut g, 4294968828, "clang::CXXFoldExpr::getInit"),
      add_named_node(&mut g, 4294968829, "clang::CXXFoldExpr::getLParenLoc"),
      add_named_node(&mut g, 4294968830, "clang::CXXFoldExpr::getRParenLoc"),
      add_named_node(&mut g, 4294968831, "clang::CXXFoldExpr::getEllipsisLoc"),
      add_named_node(&mut g, 4294968832, "clang::CXXFoldExpr::getOperator"),
      add_named_node(&mut g, 4294968833, "clang::CXXFoldExpr::getNumExpansions"),
      add_named_node(&mut g, 4294968834, "clang::CXXFoldExpr::getBeginLoc"),
      add_named_node(&mut g, 4294968835, "clang::CXXFoldExpr::getEndLoc"),
      add_named_node(&mut g, 4294968836, "clang::CXXFoldExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXFOLDEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BUILTINALIASATTR, "clang::BuiltinAliasAttr");
  g.add_edge((CLASS_BUILTINALIASATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BUILTINALIASATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BUILTINALIASATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CONSUMABLEATTR, "clang::ConsumableAttr");
  g.add_edge((CLASS_CONSUMABLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONSUMABLEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CONSUMABLEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTEAMSDIRECTIVE, "clang::OMPTeamsDirective");
  g.add_edge((CLASS_OMPTEAMSDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTEAMSDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTEAMSDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CLASSTEMPLATESPECIALIZATIONDECL, "clang::ClassTemplateSpecializationDecl");
  g.add_edge((CLASS_CLASSTEMPLATESPECIALIZATIONDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CLASSTEMPLATESPECIALIZATIONDECL, META_SUBCLASS, CLASS_CXXRECORDDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968841, "clang::ClassTemplateSpecializationDecl::getSpecializedTemplate"),
      add_named_node(&mut g, 4294968842, "clang::ClassTemplateSpecializationDecl::getTemplateArgs"),
      add_named_node(&mut g, 4294968843, "clang::ClassTemplateSpecializationDecl::getSpecializationKind"),
      add_named_node(&mut g, 4294968844, "clang::ClassTemplateSpecializationDecl::isExplicitSpecialization"),
      add_named_node(&mut g, 4294968845, "clang::ClassTemplateSpecializationDecl::isClassScopeExplicitSpecialization"),
      add_named_node(&mut g, 4294968846, "clang::ClassTemplateSpecializationDecl::isExplicitInstantiationOrSpecialization"),
      add_named_node(&mut g, 4294968847, "clang::ClassTemplateSpecializationDecl::getPointOfInstantiation"),
      add_named_node(&mut g, 4294968848, "clang::ClassTemplateSpecializationDecl::getInstantiatedFrom"),
      add_named_node(&mut g, 4294968849, "clang::ClassTemplateSpecializationDecl::getSpecializedTemplateOrPartial"),
      add_named_node(&mut g, 4294968850, "clang::ClassTemplateSpecializationDecl::getTemplateInstantiationArgs"),
      add_named_node(&mut g, 4294968851, "clang::ClassTemplateSpecializationDecl::getTypeAsWritten"),
      add_named_node(&mut g, 4294968852, "clang::ClassTemplateSpecializationDecl::getExternLoc"),
      add_named_node(&mut g, 4294968853, "clang::ClassTemplateSpecializationDecl::getTemplateKeywordLoc"),
      add_named_node(&mut g, 4294968854, "clang::ClassTemplateSpecializationDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CLASSTEMPLATESPECIALIZATIONDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ACQUIREHANDLEATTR, "clang::AcquireHandleAttr");
  g.add_edge((CLASS_ACQUIREHANDLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ACQUIREHANDLEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ACQUIREHANDLEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OPENCLACCESSATTR, "clang::OpenCLAccessAttr");
  g.add_edge((CLASS_OPENCLACCESSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OPENCLACCESSATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OPENCLACCESSATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_READONLYPLACEMENTATTR, "clang::ReadOnlyPlacementAttr");
  g.add_edge((CLASS_READONLYPLACEMENTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_READONLYPLACEMENTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_READONLYPLACEMENTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SUBSTNONTYPETEMPLATEPARMEXPR, "clang::SubstNonTypeTemplateParmExpr");
  g.add_edge((CLASS_SUBSTNONTYPETEMPLATEPARMEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SUBSTNONTYPETEMPLATEPARMEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968859, "clang::SubstNonTypeTemplateParmExpr::getNameLoc"),
      add_named_node(&mut g, 4294968860, "clang::SubstNonTypeTemplateParmExpr::getBeginLoc"),
      add_named_node(&mut g, 4294968861, "clang::SubstNonTypeTemplateParmExpr::getEndLoc"),
      add_named_node(&mut g, 4294968862, "clang::SubstNonTypeTemplateParmExpr::getReplacement"),
      add_named_node(&mut g, 4294968863, "clang::SubstNonTypeTemplateParmExpr::getAssociatedDecl"),
      add_named_node(&mut g, 4294968864, "clang::SubstNonTypeTemplateParmExpr::getIndex"),
      add_named_node(&mut g, 4294968865, "clang::SubstNonTypeTemplateParmExpr::getPackIndex"),
      add_named_node(&mut g, 4294968866, "clang::SubstNonTypeTemplateParmExpr::getParameter"),
      add_named_node(&mut g, 4294968867, "clang::SubstNonTypeTemplateParmExpr::isReferenceParameter"),
      add_named_node(&mut g, 4294968868, "clang::SubstNonTypeTemplateParmExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SUBSTNONTYPETEMPLATEPARMEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_COUNTEDBYATTR, "clang::CountedByAttr");
  g.add_edge((CLASS_COUNTEDBYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_COUNTEDBYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_COUNTEDBYATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MSABIATTR, "clang::MSABIAttr");
  g.add_edge((CLASS_MSABIATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MSABIATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MSABIATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPETAGFORDATATYPEATTR, "clang::TypeTagForDatatypeAttr");
  g.add_edge((CLASS_TYPETAGFORDATATYPEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPETAGFORDATATYPEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPETAGFORDATATYPEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ALIASATTR, "clang::AliasAttr");
  g.add_edge((CLASS_ALIASATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ALIASATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ALIASATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ENFORCETCBATTR, "clang::EnforceTCBAttr");
  g.add_edge((CLASS_ENFORCETCBATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ENFORCETCBATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ENFORCETCBATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SWIFTVERSIONEDREMOVALATTR, "clang::SwiftVersionedRemovalAttr");
  g.add_edge((CLASS_SWIFTVERSIONEDREMOVALATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTVERSIONEDREMOVALATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SWIFTVERSIONEDREMOVALATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ALLOCSIZEATTR, "clang::AllocSizeAttr");
  g.add_edge((CLASS_ALLOCSIZEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ALLOCSIZEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ALLOCSIZEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ERRORATTR, "clang::ErrorAttr");
  g.add_edge((CLASS_ERRORATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ERRORATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ERRORATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ASSERTCAPABILITYATTR, "clang::AssertCapabilityAttr");
  g.add_edge((CLASS_ASSERTCAPABILITYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ASSERTCAPABILITYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ASSERTCAPABILITYATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PATCHABLEFUNCTIONENTRYATTR, "clang::PatchableFunctionEntryAttr");
  g.add_edge((CLASS_PATCHABLEFUNCTIONENTRYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PATCHABLEFUNCTIONENTRYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PATCHABLEFUNCTIONENTRYATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ALIGNVALUEATTR, "clang::AlignValueAttr");
  g.add_edge((CLASS_ALIGNVALUEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ALIGNVALUEATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ALIGNVALUEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SWIFTBRIDGEATTR, "clang::SwiftBridgeAttr");
  g.add_edge((CLASS_SWIFTBRIDGEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTBRIDGEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SWIFTBRIDGEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MIPS16ATTR, "clang::Mips16Attr");
  g.add_edge((CLASS_MIPS16ATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MIPS16ATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MIPS16ATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CONSTANTARRAYTYPELOC, "clang::ConstantArrayTypeLoc");
  g.add_edge((CLASS_CONSTANTARRAYTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CONSTANTARRAYTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_IFUNCATTR, "clang::IFuncAttr");
  g.add_edge((CLASS_IFUNCATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_IFUNCATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_IFUNCATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPCAPTURENOINITATTR, "clang::OMPCaptureNoInitAttr");
  g.add_edge((CLASS_OMPCAPTURENOINITATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPCAPTURENOINITATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPCAPTURENOINITATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ARMLOCALLYSTREAMINGATTR, "clang::ArmLocallyStreamingAttr");
  g.add_edge((CLASS_ARMLOCALLYSTREAMINGATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARMLOCALLYSTREAMINGATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ARMLOCALLYSTREAMINGATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_VECRETURNATTR, "clang::VecReturnAttr");
  g.add_edge((CLASS_VECRETURNATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_VECRETURNATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_VECRETURNATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SWIFTIMPORTPROPERTYASACCESSORSATTR, "clang::SwiftImportPropertyAsAccessorsAttr");
  g.add_edge((CLASS_SWIFTIMPORTPROPERTYASACCESSORSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTIMPORTPROPERTYASACCESSORSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SWIFTIMPORTPROPERTYASACCESSORSATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_EMPTYBASESATTR, "clang::EmptyBasesAttr");
  g.add_edge((CLASS_EMPTYBASESATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_EMPTYBASESATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_EMPTYBASESATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SWITCHSTMT, "clang::SwitchStmt");
  g.add_edge((CLASS_SWITCHSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWITCHSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968890, "clang::SwitchStmt::hasInitStorage"),
      add_named_node(&mut g, 4294968891, "clang::SwitchStmt::hasVarStorage"),
      add_named_node(&mut g, 4294968892, "clang::SwitchStmt::getCond"),
      add_named_node(&mut g, 4294968893, "clang::SwitchStmt::getBody"),
      add_named_node(&mut g, 4294968894, "clang::SwitchStmt::getInit"),
      add_named_node(&mut g, 4294968895, "clang::SwitchStmt::getConditionVariable"),
      add_named_node(&mut g, 4294968896, "clang::SwitchStmt::getConditionVariableDeclStmt"),
      add_named_node(&mut g, 4294968897, "clang::SwitchStmt::getSwitchCaseList"),
      add_named_node(&mut g, 4294968898, "clang::SwitchStmt::getSwitchLoc"),
      add_named_node(&mut g, 4294968899, "clang::SwitchStmt::getLParenLoc"),
      add_named_node(&mut g, 4294968900, "clang::SwitchStmt::getRParenLoc"),
      add_named_node(&mut g, 4294968901, "clang::SwitchStmt::isAllEnumCasesCovered"),
      add_named_node(&mut g, 4294968902, "clang::SwitchStmt::getBeginLoc"),
      add_named_node(&mut g, 4294968903, "clang::SwitchStmt::getEndLoc"),
      add_named_node(&mut g, 4294968904, "clang::SwitchStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SWITCHSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXBINDTEMPORARYEXPR, "clang::CXXBindTemporaryExpr");
  g.add_edge((CLASS_CXXBINDTEMPORARYEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXBINDTEMPORARYEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968906, "clang::CXXBindTemporaryExpr::getTemporary"),
      add_named_node(&mut g, 4294968907, "clang::CXXBindTemporaryExpr::getSubExpr"),
      add_named_node(&mut g, 4294968908, "clang::CXXBindTemporaryExpr::getBeginLoc"),
      add_named_node(&mut g, 4294968909, "clang::CXXBindTemporaryExpr::getEndLoc"),
      add_named_node(&mut g, 4294968910, "clang::CXXBindTemporaryExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXBINDTEMPORARYEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ACQUIRECAPABILITYATTR, "clang::AcquireCapabilityAttr");
  g.add_edge((CLASS_ACQUIRECAPABILITYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ACQUIRECAPABILITYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ACQUIRECAPABILITYATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NAKEDATTR, "clang::NakedAttr");
  g.add_edge((CLASS_NAKEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NAKEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NAKEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEDUCEDTYPE, "clang::DeducedType");
  g.add_edge((CLASS_DEDUCEDTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEDUCEDTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968914, "clang::DeducedType::isSugared"),
      add_named_node(&mut g, 4294968915, "clang::DeducedType::desugar"),
      add_named_node(&mut g, 4294968916, "clang::DeducedType::getDeducedType"),
      add_named_node(&mut g, 4294968917, "clang::DeducedType::isDeduced"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEDUCEDTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ATOMICTYPELOC, "clang::AtomicTypeLoc");
  g.add_edge((CLASS_ATOMICTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968919, "clang::AtomicTypeLoc::getValueLoc"),
      add_named_node(&mut g, 4294968920, "clang::AtomicTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, 4294968921, "clang::AtomicTypeLoc::getKWLoc"),
      add_named_node(&mut g, 4294968922, "clang::AtomicTypeLoc::getLParenLoc"),
      add_named_node(&mut g, 4294968923, "clang::AtomicTypeLoc::getRParenLoc"),
      add_named_node(&mut g, 4294968924, "clang::AtomicTypeLoc::getParensRange"),
      add_named_node(&mut g, 4294968925, "clang::AtomicTypeLoc::getInnerType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ATOMICTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DECLTYPETYPELOC, "clang::DecltypeTypeLoc");
  g.add_edge((CLASS_DECLTYPETYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968927, "clang::DecltypeTypeLoc::getUnderlyingExpr"),
      add_named_node(&mut g, 4294968928, "clang::DecltypeTypeLoc::getDecltypeLoc"),
      add_named_node(&mut g, 4294968929, "clang::DecltypeTypeLoc::getRParenLoc"),
      add_named_node(&mut g, 4294968930, "clang::DecltypeTypeLoc::getLocalSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DECLTYPETYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTARGETPARALLELDIRECTIVE, "clang::OMPTargetParallelDirective");
  g.add_edge((CLASS_OMPTARGETPARALLELDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETPARALLELDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTARGETPARALLELDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTNAMETYPE, "clang::DependentNameType");
  g.add_edge((CLASS_DEPENDENTNAMETYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEPENDENTNAMETYPE, META_SUBCLASS, CLASS_TYPEWITHKEYWORD));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968933, "clang::DependentNameType::getQualifier"),
      add_named_node(&mut g, 4294968934, "clang::DependentNameType::getIdentifier"),
      add_named_node(&mut g, 4294968935, "clang::DependentNameType::isSugared"),
      add_named_node(&mut g, 4294968936, "clang::DependentNameType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTNAMETYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NSRETURNSAUTORELEASEDATTR, "clang::NSReturnsAutoreleasedAttr");
  g.add_edge((CLASS_NSRETURNSAUTORELEASEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NSRETURNSAUTORELEASEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NSRETURNSAUTORELEASEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SUPPRESSATTR, "clang::SuppressAttr");
  g.add_edge((CLASS_SUPPRESSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SUPPRESSATTR, META_SUBCLASS, CLASS_DECLORSTMTATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SUPPRESSATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_VECTORTYPELOC, "clang::VectorTypeLoc");
  g.add_edge((CLASS_VECTORTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968940, "clang::VectorTypeLoc::getNameLoc"),
      add_named_node(&mut g, 4294968941, "clang::VectorTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, 4294968942, "clang::VectorTypeLoc::getElementLoc"),
      add_named_node(&mut g, 4294968943, "clang::VectorTypeLoc::getInnerType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_VECTORTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCPROTOCOLEXPR, "clang::ObjCProtocolExpr");
  g.add_edge((CLASS_OBJCPROTOCOLEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCPROTOCOLEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCPROTOCOLEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_C11NORETURNATTR, "clang::C11NoReturnAttr");
  g.add_edge((CLASS_C11NORETURNATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_C11NORETURNATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_C11NORETURNATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SEHTRYSTMT, "clang::SEHTryStmt");
  g.add_edge((CLASS_SEHTRYSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SEHTRYSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968947, "clang::SEHTryStmt::getBeginLoc"),
      add_named_node(&mut g, 4294968948, "clang::SEHTryStmt::getTryLoc"),
      add_named_node(&mut g, 4294968949, "clang::SEHTryStmt::getEndLoc"),
      add_named_node(&mut g, 4294968950, "clang::SEHTryStmt::getIsCXXTry"),
      add_named_node(&mut g, 4294968951, "clang::SEHTryStmt::getTryBlock"),
      add_named_node(&mut g, 4294968952, "clang::SEHTryStmt::getHandler"),
      add_named_node(&mut g, 4294968953, "clang::SEHTryStmt::getExceptHandler"),
      add_named_node(&mut g, 4294968954, "clang::SEHTryStmt::getFinallyHandler"),
      add_named_node(&mut g, 4294968955, "clang::SEHTryStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SEHTRYSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CALLEDONCEATTR, "clang::CalledOnceAttr");
  g.add_edge((CLASS_CALLEDONCEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CALLEDONCEATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CALLEDONCEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTEAMSDISTRIBUTEPARALLELFORDIRECTIVE, "clang::OMPTeamsDistributeParallelForDirective");
  g.add_edge((CLASS_OMPTEAMSDISTRIBUTEPARALLELFORDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTEAMSDISTRIBUTEPARALLELFORDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTEAMSDISTRIBUTEPARALLELFORDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FUNCTIONDECL, "clang::FunctionDecl");
  g.add_edge((CLASS_FUNCTIONDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FUNCTIONDECL, META_SUBCLASS, CLASS_DECLARATORDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294968959, "clang::FunctionDecl::getNameInfo"),
      add_named_node(&mut g, 4294968960, "clang::FunctionDecl::getEllipsisLoc"),
      add_named_node(&mut g, 4294968961, "clang::FunctionDecl::getSourceRange"),
      add_named_node(&mut g, 4294968962, "clang::FunctionDecl::hasBody"),
      add_named_node(&mut g, 4294968963, "clang::FunctionDecl::hasTrivialBody"),
      add_named_node(&mut g, 4294968964, "clang::FunctionDecl::isDefined"),
      add_named_node(&mut g, 4294968965, "clang::FunctionDecl::getDefinition"),
      add_named_node(&mut g, 4294968966, "clang::FunctionDecl::getBody"),
      add_named_node(&mut g, 4294968967, "clang::FunctionDecl::isThisDeclarationADefinition"),
      add_named_node(&mut g, 4294968968, "clang::FunctionDecl::isThisDeclarationInstantiatedFromAFriendDefinition"),
      add_named_node(&mut g, 4294968969, "clang::FunctionDecl::doesThisDeclarationHaveABody"),
      add_named_node(&mut g, 4294968970, "clang::FunctionDecl::getDefaultedFunctionInfo"),
      add_named_node(&mut g, 4294968971, "clang::FunctionDecl::isVariadic"),
      add_named_node(&mut g, 4294968972, "clang::FunctionDecl::isVirtualAsWritten"),
      add_named_node(&mut g, 4294968973, "clang::FunctionDecl::isPureVirtual"),
      add_named_node(&mut g, 4294968974, "clang::FunctionDecl::isLateTemplateParsed"),
      add_named_node(&mut g, 4294968975, "clang::FunctionDecl::isTrivial"),
      add_named_node(&mut g, 4294968976, "clang::FunctionDecl::isTrivialForCall"),
      add_named_node(&mut g, 4294968977, "clang::FunctionDecl::isDefaulted"),
      add_named_node(&mut g, 4294968978, "clang::FunctionDecl::isExplicitlyDefaulted"),
      add_named_node(&mut g, 4294968979, "clang::FunctionDecl::getDefaultLoc"),
      add_named_node(&mut g, 4294968980, "clang::FunctionDecl::isUserProvided"),
      add_named_node(&mut g, 4294968981, "clang::FunctionDecl::isIneligibleOrNotSelected"),
      add_named_node(&mut g, 4294968982, "clang::FunctionDecl::hasImplicitReturnZero"),
      add_named_node(&mut g, 4294968983, "clang::FunctionDecl::hasPrototype"),
      add_named_node(&mut g, 4294968984, "clang::FunctionDecl::hasWrittenPrototype"),
      add_named_node(&mut g, 4294968985, "clang::FunctionDecl::hasInheritedPrototype"),
      add_named_node(&mut g, 4294968986, "clang::FunctionDecl::isConstexpr"),
      add_named_node(&mut g, 4294968987, "clang::FunctionDecl::getConstexprKind"),
      add_named_node(&mut g, 4294968988, "clang::FunctionDecl::isConstexprSpecified"),
      add_named_node(&mut g, 4294968989, "clang::FunctionDecl::isConsteval"),
      add_named_node(&mut g, 4294968990, "clang::FunctionDecl::BodyContainsImmediateEscalatingExpressions"),
      add_named_node(&mut g, 4294968991, "clang::FunctionDecl::isImmediateEscalating"),
      add_named_node(&mut g, 4294968992, "clang::FunctionDecl::isImmediateFunction"),
      add_named_node(&mut g, 4294968993, "clang::FunctionDecl::instantiationIsPending"),
      add_named_node(&mut g, 4294968994, "clang::FunctionDecl::usesSEHTry"),
      add_named_node(&mut g, 4294968995, "clang::FunctionDecl::isDeleted"),
      add_named_node(&mut g, 4294968996, "clang::FunctionDecl::isDeletedAsWritten"),
      add_named_node(&mut g, 4294968997, "clang::FunctionDecl::isMain"),
      add_named_node(&mut g, 4294968998, "clang::FunctionDecl::isMSVCRTEntryPoint"),
      add_named_node(&mut g, 4294968999, "clang::FunctionDecl::isReservedGlobalPlacementOperator"),
      add_named_node(&mut g, 4294969000, "clang::FunctionDecl::isInlineBuiltinDeclaration"),
      add_named_node(&mut g, 4294969001, "clang::FunctionDecl::isDestroyingOperatorDelete"),
      add_named_node(&mut g, 4294969002, "clang::FunctionDecl::getLanguageLinkage"),
      add_named_node(&mut g, 4294969003, "clang::FunctionDecl::isExternC"),
      add_named_node(&mut g, 4294969004, "clang::FunctionDecl::isInExternCContext"),
      add_named_node(&mut g, 4294969005, "clang::FunctionDecl::isInExternCXXContext"),
      add_named_node(&mut g, 4294969006, "clang::FunctionDecl::isGlobal"),
      add_named_node(&mut g, 4294969007, "clang::FunctionDecl::isNoReturn"),
      add_named_node(&mut g, 4294969008, "clang::FunctionDecl::hasSkippedBody"),
      add_named_node(&mut g, 4294969009, "clang::FunctionDecl::willHaveBody"),
      add_named_node(&mut g, 4294969010, "clang::FunctionDecl::isMultiVersion"),
      add_named_node(&mut g, 4294969011, "clang::FunctionDecl::FriendConstraintRefersToEnclosingTemplate"),
      add_named_node(&mut g, 4294969012, "clang::FunctionDecl::isMemberLikeConstrainedFriend"),
      add_named_node(&mut g, 4294969013, "clang::FunctionDecl::getMultiVersionKind"),
      add_named_node(&mut g, 4294969014, "clang::FunctionDecl::isCPUDispatchMultiVersion"),
      add_named_node(&mut g, 4294969015, "clang::FunctionDecl::isCPUSpecificMultiVersion"),
      add_named_node(&mut g, 4294969016, "clang::FunctionDecl::isTargetMultiVersion"),
      add_named_node(&mut g, 4294969017, "clang::FunctionDecl::isTargetClonesMultiVersion"),
      add_named_node(&mut g, 4294969018, "clang::FunctionDecl::getCanonicalDecl"),
      add_named_node(&mut g, 4294969019, "clang::FunctionDecl::parameters"),
      add_named_node(&mut g, 4294969020, "clang::FunctionDecl::param_empty"),
      add_named_node(&mut g, 4294969021, "clang::FunctionDecl::param_begin"),
      add_named_node(&mut g, 4294969022, "clang::FunctionDecl::param_end"),
      add_named_node(&mut g, 4294969023, "clang::FunctionDecl::param_size"),
      add_named_node(&mut g, 4294969024, "clang::FunctionDecl::getNumParams"),
      add_named_node(&mut g, 4294969025, "clang::FunctionDecl::getMinRequiredArguments"),
      add_named_node(&mut g, 4294969026, "clang::FunctionDecl::getMinRequiredExplicitArguments"),
      add_named_node(&mut g, 4294969027, "clang::FunctionDecl::hasCXXExplicitFunctionObjectParameter"),
      add_named_node(&mut g, 4294969028, "clang::FunctionDecl::getNumNonObjectParams"),
      add_named_node(&mut g, 4294969029, "clang::FunctionDecl::hasOneParamOrDefaultArgs"),
      add_named_node(&mut g, 4294969030, "clang::FunctionDecl::getFunctionTypeLoc"),
      add_named_node(&mut g, 4294969031, "clang::FunctionDecl::getReturnType"),
      add_named_node(&mut g, 4294969032, "clang::FunctionDecl::getReturnTypeSourceRange"),
      add_named_node(&mut g, 4294969033, "clang::FunctionDecl::getParametersSourceRange"),
      add_named_node(&mut g, 4294969034, "clang::FunctionDecl::getDeclaredReturnType"),
      add_named_node(&mut g, 4294969035, "clang::FunctionDecl::getExceptionSpecType"),
      add_named_node(&mut g, 4294969036, "clang::FunctionDecl::getExceptionSpecSourceRange"),
      add_named_node(&mut g, 4294969037, "clang::FunctionDecl::getCallResultType"),
      add_named_node(&mut g, 4294969038, "clang::FunctionDecl::getStorageClass"),
      add_named_node(&mut g, 4294969039, "clang::FunctionDecl::isInlineSpecified"),
      add_named_node(&mut g, 4294969040, "clang::FunctionDecl::UsesFPIntrin"),
      add_named_node(&mut g, 4294969041, "clang::FunctionDecl::isInlined"),
      add_named_node(&mut g, 4294969042, "clang::FunctionDecl::isInlineDefinitionExternallyVisible"),
      add_named_node(&mut g, 4294969043, "clang::FunctionDecl::isMSExternInline"),
      add_named_node(&mut g, 4294969044, "clang::FunctionDecl::doesDeclarationForceExternallyVisibleDefinition"),
      add_named_node(&mut g, 4294969045, "clang::FunctionDecl::isStatic"),
      add_named_node(&mut g, 4294969046, "clang::FunctionDecl::isOverloadedOperator"),
      add_named_node(&mut g, 4294969047, "clang::FunctionDecl::getOverloadedOperator"),
      add_named_node(&mut g, 4294969048, "clang::FunctionDecl::getLiteralIdentifier"),
      add_named_node(&mut g, 4294969049, "clang::FunctionDecl::getInstantiatedFromMemberFunction"),
      add_named_node(&mut g, 4294969050, "clang::FunctionDecl::getTemplatedKind"),
      add_named_node(&mut g, 4294969051, "clang::FunctionDecl::getMemberSpecializationInfo"),
      add_named_node(&mut g, 4294969052, "clang::FunctionDecl::getInstantiatedFromDecl"),
      add_named_node(&mut g, 4294969053, "clang::FunctionDecl::getDescribedFunctionTemplate"),
      add_named_node(&mut g, 4294969054, "clang::FunctionDecl::isFunctionTemplateSpecialization"),
      add_named_node(&mut g, 4294969055, "clang::FunctionDecl::getTemplateSpecializationInfo"),
      add_named_node(&mut g, 4294969056, "clang::FunctionDecl::isImplicitlyInstantiable"),
      add_named_node(&mut g, 4294969057, "clang::FunctionDecl::isTemplateInstantiation"),
      add_named_node(&mut g, 4294969058, "clang::FunctionDecl::getPrimaryTemplate"),
      add_named_node(&mut g, 4294969059, "clang::FunctionDecl::getTemplateSpecializationArgs"),
      add_named_node(&mut g, 4294969060, "clang::FunctionDecl::getTemplateSpecializationArgsAsWritten"),
      add_named_node(&mut g, 4294969061, "clang::FunctionDecl::getDependentSpecializationInfo"),
      add_named_node(&mut g, 4294969062, "clang::FunctionDecl::getTemplateSpecializationKind"),
      add_named_node(&mut g, 4294969063, "clang::FunctionDecl::getTemplateSpecializationKindForInstantiation"),
      add_named_node(&mut g, 4294969064, "clang::FunctionDecl::getPointOfInstantiation"),
      add_named_node(&mut g, 4294969065, "clang::FunctionDecl::isOutOfLine"),
      add_named_node(&mut g, 4294969066, "clang::FunctionDecl::getMemoryFunctionKind"),
      add_named_node(&mut g, 4294969067, "clang::FunctionDecl::getODRHash"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FUNCTIONDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_XRAYLOGARGSATTR, "clang::XRayLogArgsAttr");
  g.add_edge((CLASS_XRAYLOGARGSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_XRAYLOGARGSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_XRAYLOGARGSATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_IMPLICITPARAMDECL, "clang::ImplicitParamDecl");
  g.add_edge((CLASS_IMPLICITPARAMDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_IMPLICITPARAMDECL, META_SUBCLASS, CLASS_VARDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969070, "clang::ImplicitParamDecl::getParameterKind"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_IMPLICITPARAMDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCBOXABLEATTR, "clang::ObjCBoxableAttr");
  g.add_edge((CLASS_OBJCBOXABLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCBOXABLEATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCBOXABLEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_WEBASSEMBLYIMPORTMODULEATTR, "clang::WebAssemblyImportModuleAttr");
  g.add_edge((CLASS_WEBASSEMBLYIMPORTMODULEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_WEBASSEMBLYIMPORTMODULEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_WEBASSEMBLYIMPORTMODULEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_UNRESOLVEDMEMBEREXPR, "clang::UnresolvedMemberExpr");
  g.add_edge((CLASS_UNRESOLVEDMEMBEREXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNRESOLVEDMEMBEREXPR, META_SUBCLASS, CLASS_OVERLOADEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969074, "clang::UnresolvedMemberExpr::isImplicitAccess"),
      add_named_node(&mut g, 4294969075, "clang::UnresolvedMemberExpr::getBase"),
      add_named_node(&mut g, 4294969076, "clang::UnresolvedMemberExpr::getBaseType"),
      add_named_node(&mut g, 4294969077, "clang::UnresolvedMemberExpr::hasUnresolvedUsing"),
      add_named_node(&mut g, 4294969078, "clang::UnresolvedMemberExpr::isArrow"),
      add_named_node(&mut g, 4294969079, "clang::UnresolvedMemberExpr::getOperatorLoc"),
      add_named_node(&mut g, 4294969080, "clang::UnresolvedMemberExpr::getNamingClass"),
      add_named_node(&mut g, 4294969081, "clang::UnresolvedMemberExpr::getMemberNameInfo"),
      add_named_node(&mut g, 4294969082, "clang::UnresolvedMemberExpr::getMemberName"),
      add_named_node(&mut g, 4294969083, "clang::UnresolvedMemberExpr::getMemberLoc"),
      add_named_node(&mut g, 4294969084, "clang::UnresolvedMemberExpr::getExprLoc"),
      add_named_node(&mut g, 4294969085, "clang::UnresolvedMemberExpr::getBeginLoc"),
      add_named_node(&mut g, 4294969086, "clang::UnresolvedMemberExpr::getEndLoc"),
      add_named_node(&mut g, 4294969087, "clang::UnresolvedMemberExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNRESOLVEDMEMBEREXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPDEPOBJDIRECTIVE, "clang::OMPDepobjDirective");
  g.add_edge((CLASS_OMPDEPOBJDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPDEPOBJDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPDEPOBJDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCDESIGNATEDINITIALIZERATTR, "clang::ObjCDesignatedInitializerAttr");
  g.add_edge((CLASS_OBJCDESIGNATEDINITIALIZERATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCDESIGNATEDINITIALIZERATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCDESIGNATEDINITIALIZERATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_STRINGLITERAL, "clang::StringLiteral");
  g.add_edge((CLASS_STRINGLITERAL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_STRINGLITERAL, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969091, "clang::StringLiteral::getString"),
      add_named_node(&mut g, 4294969092, "clang::StringLiteral::getBytes"),
      add_named_node(&mut g, 4294969093, "clang::StringLiteral::getByteLength"),
      add_named_node(&mut g, 4294969094, "clang::StringLiteral::getLength"),
      add_named_node(&mut g, 4294969095, "clang::StringLiteral::getCharByteWidth"),
      add_named_node(&mut g, 4294969096, "clang::StringLiteral::getKind"),
      add_named_node(&mut g, 4294969097, "clang::StringLiteral::isOrdinary"),
      add_named_node(&mut g, 4294969098, "clang::StringLiteral::isWide"),
      add_named_node(&mut g, 4294969099, "clang::StringLiteral::isUTF8"),
      add_named_node(&mut g, 4294969100, "clang::StringLiteral::isUTF16"),
      add_named_node(&mut g, 4294969101, "clang::StringLiteral::isUTF32"),
      add_named_node(&mut g, 4294969102, "clang::StringLiteral::isUnevaluated"),
      add_named_node(&mut g, 4294969103, "clang::StringLiteral::isPascal"),
      add_named_node(&mut g, 4294969104, "clang::StringLiteral::containsNonAscii"),
      add_named_node(&mut g, 4294969105, "clang::StringLiteral::containsNonAsciiOrNull"),
      add_named_node(&mut g, 4294969106, "clang::StringLiteral::getNumConcatenated"),
      add_named_node(&mut g, 4294969107, "clang::StringLiteral::tokloc_begin"),
      add_named_node(&mut g, 4294969108, "clang::StringLiteral::tokloc_end"),
      add_named_node(&mut g, 4294969109, "clang::StringLiteral::getBeginLoc"),
      add_named_node(&mut g, 4294969110, "clang::StringLiteral::getEndLoc"),
      add_named_node(&mut g, 4294969111, "clang::StringLiteral::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_STRINGLITERAL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_STRICTGUARDSTACKCHECKATTR, "clang::StrictGuardStackCheckAttr");
  g.add_edge((CLASS_STRICTGUARDSTACKCHECKATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_STRICTGUARDSTACKCHECKATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_STRICTGUARDSTACKCHECKATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPSECTIONSDIRECTIVE, "clang::OMPSectionsDirective");
  g.add_edge((CLASS_OMPSECTIONSDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPSECTIONSDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPSECTIONSDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_HLSLSV_DISPATCHTHREADIDATTR, "clang::HLSLSV_DispatchThreadIDAttr");
  g.add_edge((CLASS_HLSLSV_DISPATCHTHREADIDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_HLSLSV_DISPATCHTHREADIDATTR, META_SUBCLASS, CLASS_HLSLANNOTATIONATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_HLSLSV_DISPATCHTHREADIDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FASTCALLATTR, "clang::FastCallAttr");
  g.add_edge((CLASS_FASTCALLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FASTCALLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FASTCALLATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ARTIFICIALATTR, "clang::ArtificialAttr");
  g.add_edge((CLASS_ARTIFICIALATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARTIFICIALATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ARTIFICIALATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_RELEASEHANDLEATTR, "clang::ReleaseHandleAttr");
  g.add_edge((CLASS_RELEASEHANDLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RELEASEHANDLEATTR, META_SUBCLASS, CLASS_INHERITABLEPARAMATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_RELEASEHANDLEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_REFERENCETYPE, "clang::ReferenceType");
  g.add_edge((CLASS_REFERENCETYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_REFERENCETYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969119, "clang::ReferenceType::isSpelledAsLValue"),
      add_named_node(&mut g, 4294969120, "clang::ReferenceType::isInnerRef"),
      add_named_node(&mut g, 4294969121, "clang::ReferenceType::getPointeeTypeAsWritten"),
      add_named_node(&mut g, 4294969122, "clang::ReferenceType::getPointeeType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_REFERENCETYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_VECTYPEHINTATTR, "clang::VecTypeHintAttr");
  g.add_edge((CLASS_VECTYPEHINTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_VECTYPEHINTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_VECTYPEHINTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SWIFTASYNCCONTEXTATTR, "clang::SwiftAsyncContextAttr");
  g.add_edge((CLASS_SWIFTASYNCCONTEXTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTASYNCCONTEXTATTR, META_SUBCLASS, CLASS_PARAMETERABIATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SWIFTASYNCCONTEXTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MINVECTORWIDTHATTR, "clang::MinVectorWidthAttr");
  g.add_edge((CLASS_MINVECTORWIDTHATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MINVECTORWIDTHATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MINVECTORWIDTHATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BTFDECLTAGATTR, "clang::BTFDeclTagAttr");
  g.add_edge((CLASS_BTFDECLTAGATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BTFDECLTAGATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BTFDECLTAGATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CLEANUPATTR, "clang::CleanupAttr");
  g.add_edge((CLASS_CLEANUPATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CLEANUPATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CLEANUPATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPREFERENCEDVARATTR, "clang::OMPReferencedVarAttr");
  g.add_edge((CLASS_OMPREFERENCEDVARATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPREFERENCEDVARATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPREFERENCEDVARATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTASKWAITDIRECTIVE, "clang::OMPTaskwaitDirective");
  g.add_edge((CLASS_OMPTASKWAITDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTASKWAITDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTASKWAITDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_WARNUNUSEDRESULTATTR, "clang::WarnUnusedResultAttr");
  g.add_edge((CLASS_WARNUNUSEDRESULTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_WARNUNUSEDRESULTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_WARNUNUSEDRESULTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_INITSEGATTR, "clang::InitSegAttr");
  g.add_edge((CLASS_INITSEGATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_INITSEGATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_INITSEGATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TRANSPARENTUNIONATTR, "clang::TransparentUnionAttr");
  g.add_edge((CLASS_TRANSPARENTUNIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TRANSPARENTUNIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TRANSPARENTUNIONATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SWIFTERRORATTR, "clang::SwiftErrorAttr");
  g.add_edge((CLASS_SWIFTERRORATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTERRORATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SWIFTERRORATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PARMVARDECL, "clang::ParmVarDecl");
  g.add_edge((CLASS_PARMVARDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PARMVARDECL, META_SUBCLASS, CLASS_VARDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969135, "clang::ParmVarDecl::getSourceRange"),
      add_named_node(&mut g, 4294969136, "clang::ParmVarDecl::isObjCMethodParameter"),
      add_named_node(&mut g, 4294969137, "clang::ParmVarDecl::isDestroyedInCallee"),
      add_named_node(&mut g, 4294969138, "clang::ParmVarDecl::getFunctionScopeDepth"),
      add_named_node(&mut g, 4294969139, "clang::ParmVarDecl::getFunctionScopeIndex"),
      add_named_node(&mut g, 4294969140, "clang::ParmVarDecl::getObjCDeclQualifier"),
      add_named_node(&mut g, 4294969141, "clang::ParmVarDecl::isKNRPromoted"),
      add_named_node(&mut g, 4294969142, "clang::ParmVarDecl::isExplicitObjectParameter"),
      add_named_node(&mut g, 4294969143, "clang::ParmVarDecl::getExplicitObjectParamThisLoc"),
      add_named_node(&mut g, 4294969144, "clang::ParmVarDecl::getDefaultArg"),
      add_named_node(&mut g, 4294969145, "clang::ParmVarDecl::getDefaultArgRange"),
      add_named_node(&mut g, 4294969146, "clang::ParmVarDecl::getUninstantiatedDefaultArg"),
      add_named_node(&mut g, 4294969147, "clang::ParmVarDecl::hasDefaultArg"),
      add_named_node(&mut g, 4294969148, "clang::ParmVarDecl::hasUnparsedDefaultArg"),
      add_named_node(&mut g, 4294969149, "clang::ParmVarDecl::hasUninstantiatedDefaultArg"),
      add_named_node(&mut g, 4294969150, "clang::ParmVarDecl::hasInheritedDefaultArg"),
      add_named_node(&mut g, 4294969151, "clang::ParmVarDecl::getOriginalType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PARMVARDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FULLEXPR, "clang::FullExpr");
  g.add_edge((CLASS_FULLEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FULLEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969153, "clang::FullExpr::getSubExpr"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FULLEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DESIGNATEDINITUPDATEEXPR, "clang::DesignatedInitUpdateExpr");
  g.add_edge((CLASS_DESIGNATEDINITUPDATEEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DESIGNATEDINITUPDATEEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969155, "clang::DesignatedInitUpdateExpr::getBeginLoc"),
      add_named_node(&mut g, 4294969156, "clang::DesignatedInitUpdateExpr::getEndLoc"),
      add_named_node(&mut g, 4294969157, "clang::DesignatedInitUpdateExpr::getBase"),
      add_named_node(&mut g, 4294969158, "clang::DesignatedInitUpdateExpr::getUpdater"),
      add_named_node(&mut g, 4294969159, "clang::DesignatedInitUpdateExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DESIGNATEDINITUPDATEEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SWIFTIMPORTASNONGENERICATTR, "clang::SwiftImportAsNonGenericAttr");
  g.add_edge((CLASS_SWIFTIMPORTASNONGENERICATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTIMPORTASNONGENERICATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SWIFTIMPORTASNONGENERICATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ARMNEWATTR, "clang::ArmNewAttr");
  g.add_edge((CLASS_ARMNEWATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARMNEWATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ARMNEWATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PRAGMADETECTMISMATCHDECL, "clang::PragmaDetectMismatchDecl");
  g.add_edge((CLASS_PRAGMADETECTMISMATCHDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PRAGMADETECTMISMATCHDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969163, "clang::PragmaDetectMismatchDecl::getName"),
      add_named_node(&mut g, 4294969164, "clang::PragmaDetectMismatchDecl::getValue"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PRAGMADETECTMISMATCHDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SWIFTASYNCCALLATTR, "clang::SwiftAsyncCallAttr");
  g.add_edge((CLASS_SWIFTASYNCCALLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTASYNCCALLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SWIFTASYNCCALLATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DOSTMT, "clang::DoStmt");
  g.add_edge((CLASS_DOSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DOSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969167, "clang::DoStmt::getCond"),
      add_named_node(&mut g, 4294969168, "clang::DoStmt::getBody"),
      add_named_node(&mut g, 4294969169, "clang::DoStmt::getDoLoc"),
      add_named_node(&mut g, 4294969170, "clang::DoStmt::getWhileLoc"),
      add_named_node(&mut g, 4294969171, "clang::DoStmt::getRParenLoc"),
      add_named_node(&mut g, 4294969172, "clang::DoStmt::getBeginLoc"),
      add_named_node(&mut g, 4294969173, "clang::DoStmt::getEndLoc"),
      add_named_node(&mut g, 4294969174, "clang::DoStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DOSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_M68KINTERRUPTATTR, "clang::M68kInterruptAttr");
  g.add_edge((CLASS_M68KINTERRUPTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_M68KINTERRUPTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_M68KINTERRUPTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ACQUIREDAFTERATTR, "clang::AcquiredAfterAttr");
  g.add_edge((CLASS_ACQUIREDAFTERATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ACQUIREDAFTERATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ACQUIREDAFTERATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SWIFTASYNCATTR, "clang::SwiftAsyncAttr");
  g.add_edge((CLASS_SWIFTASYNCATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTASYNCATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SWIFTASYNCATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SECTIONATTR, "clang::SectionAttr");
  g.add_edge((CLASS_SECTIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SECTIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SECTIONATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OSRETURNSNOTRETAINEDATTR, "clang::OSReturnsNotRetainedAttr");
  g.add_edge((CLASS_OSRETURNSNOTRETAINEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OSRETURNSNOTRETAINEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OSRETURNSNOTRETAINEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ANYX86NOCALLERSAVEDREGISTERSATTR, "clang::AnyX86NoCallerSavedRegistersAttr");
  g.add_edge((CLASS_ANYX86NOCALLERSAVEDREGISTERSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ANYX86NOCALLERSAVEDREGISTERSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ANYX86NOCALLERSAVEDREGISTERSATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXUUIDOFEXPR, "clang::CXXUuidofExpr");
  g.add_edge((CLASS_CXXUUIDOFEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXUUIDOFEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969182, "clang::CXXUuidofExpr::isTypeOperand"),
      add_named_node(&mut g, 4294969183, "clang::CXXUuidofExpr::getTypeOperandSourceInfo"),
      add_named_node(&mut g, 4294969184, "clang::CXXUuidofExpr::getExprOperand"),
      add_named_node(&mut g, 4294969185, "clang::CXXUuidofExpr::getGuidDecl"),
      add_named_node(&mut g, 4294969186, "clang::CXXUuidofExpr::getBeginLoc"),
      add_named_node(&mut g, 4294969187, "clang::CXXUuidofExpr::getEndLoc"),
      add_named_node(&mut g, 4294969188, "clang::CXXUuidofExpr::getSourceRange"),
      add_named_node(&mut g, 4294969189, "clang::CXXUuidofExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXUUIDOFEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CDECLATTR, "clang::CDeclAttr");
  g.add_edge((CLASS_CDECLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CDECLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CDECLATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ABITAGATTR, "clang::AbiTagAttr");
  g.add_edge((CLASS_ABITAGATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ABITAGATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ABITAGATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_STANDALONEDEBUGATTR, "clang::StandaloneDebugAttr");
  g.add_edge((CLASS_STANDALONEDEBUGATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_STANDALONEDEBUGATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_STANDALONEDEBUGATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CONVERGENTATTR, "clang::ConvergentAttr");
  g.add_edge((CLASS_CONVERGENTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONVERGENTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CONVERGENTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ASMLABELATTR, "clang::AsmLabelAttr");
  g.add_edge((CLASS_ASMLABELATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ASMLABELATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ASMLABELATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ANALYZERNORETURNATTR, "clang::AnalyzerNoReturnAttr");
  g.add_edge((CLASS_ANALYZERNORETURNATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ANALYZERNORETURNATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ANALYZERNORETURNATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SELECTANYATTR, "clang::SelectAnyAttr");
  g.add_edge((CLASS_SELECTANYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SELECTANYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SELECTANYATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_COLDATTR, "clang::ColdAttr");
  g.add_edge((CLASS_COLDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_COLDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_COLDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BLOCKPOINTERTYPE, "clang::BlockPointerType");
  g.add_edge((CLASS_BLOCKPOINTERTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BLOCKPOINTERTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969199, "clang::BlockPointerType::getPointeeType"),
      add_named_node(&mut g, 4294969200, "clang::BlockPointerType::isSugared"),
      add_named_node(&mut g, 4294969201, "clang::BlockPointerType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BLOCKPOINTERTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCNONLAZYCLASSATTR, "clang::ObjCNonLazyClassAttr");
  g.add_edge((CLASS_OBJCNONLAZYCLASSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCNONLAZYCLASSATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCNONLAZYCLASSATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ALIGNNATURALATTR, "clang::AlignNaturalAttr");
  g.add_edge((CLASS_ALIGNNATURALATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ALIGNNATURALATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ALIGNNATURALATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPLOOPBASEDDIRECTIVE, "clang::OMPLoopBasedDirective");
  g.add_edge((CLASS_OMPLOOPBASEDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPLOOPBASEDDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPLOOPBASEDDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_AMDGPUNUMSGPRATTR, "clang::AMDGPUNumSGPRAttr");
  g.add_edge((CLASS_AMDGPUNUMSGPRATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_AMDGPUNUMSGPRATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_AMDGPUNUMSGPRATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SCOPEDLOCKABLEATTR, "clang::ScopedLockableAttr");
  g.add_edge((CLASS_SCOPEDLOCKABLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SCOPEDLOCKABLEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SCOPEDLOCKABLEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTSIZEDMATRIXTYPELOC, "clang::DependentSizedMatrixTypeLoc");
  g.add_edge((CLASS_DEPENDENTSIZEDMATRIXTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTSIZEDMATRIXTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_AMDGPUFLATWORKGROUPSIZEATTR, "clang::AMDGPUFlatWorkGroupSizeAttr");
  g.add_edge((CLASS_AMDGPUFLATWORKGROUPSIZEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_AMDGPUFLATWORKGROUPSIZEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_AMDGPUFLATWORKGROUPSIZEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OPENCLLOCALADDRESSSPACEATTR, "clang::OpenCLLocalAddressSpaceAttr");
  g.add_edge((CLASS_OPENCLLOCALADDRESSSPACEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OPENCLLOCALADDRESSSPACEATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OPENCLLOCALADDRESSSPACEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DECAYEDTYPELOC, "clang::DecayedTypeLoc");
  g.add_edge((CLASS_DECAYEDTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DECAYEDTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCINERTUNSAFEUNRETAINEDATTR, "clang::ObjCInertUnsafeUnretainedAttr");
  g.add_edge((CLASS_OBJCINERTUNSAFEUNRETAINEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCINERTUNSAFEUNRETAINEDATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCINERTUNSAFEUNRETAINEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXCATCHSTMT, "clang::CXXCatchStmt");
  g.add_edge((CLASS_CXXCATCHSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXCATCHSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969213, "clang::CXXCatchStmt::getBeginLoc"),
      add_named_node(&mut g, 4294969214, "clang::CXXCatchStmt::getEndLoc"),
      add_named_node(&mut g, 4294969215, "clang::CXXCatchStmt::getCatchLoc"),
      add_named_node(&mut g, 4294969216, "clang::CXXCatchStmt::getExceptionDecl"),
      add_named_node(&mut g, 4294969217, "clang::CXXCatchStmt::getCaughtType"),
      add_named_node(&mut g, 4294969218, "clang::CXXCatchStmt::getHandlerBlock"),
      add_named_node(&mut g, 4294969219, "clang::CXXCatchStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXCATCHSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPSCOPEDIRECTIVE, "clang::OMPScopeDirective");
  g.add_edge((CLASS_OMPSCOPEDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPSCOPEDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPSCOPEDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_AARCH64SVEPCSATTR, "clang::AArch64SVEPcsAttr");
  g.add_edge((CLASS_AARCH64SVEPCSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_AARCH64SVEPCSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_AARCH64SVEPCSATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CUDADEVICEBUILTINTEXTURETYPEATTR, "clang::CUDADeviceBuiltinTextureTypeAttr");
  g.add_edge((CLASS_CUDADEVICEBUILTINTEXTURETYPEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CUDADEVICEBUILTINTEXTURETYPEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CUDADEVICEBUILTINTEXTURETYPEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_INJECTEDCLASSNAMETYPE, "clang::InjectedClassNameType");
  g.add_edge((CLASS_INJECTEDCLASSNAMETYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_INJECTEDCLASSNAMETYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969224, "clang::InjectedClassNameType::getInjectedSpecializationType"),
      add_named_node(&mut g, 4294969225, "clang::InjectedClassNameType::getInjectedTST"),
      add_named_node(&mut g, 4294969226, "clang::InjectedClassNameType::getTemplateName"),
      add_named_node(&mut g, 4294969227, "clang::InjectedClassNameType::getDecl"),
      add_named_node(&mut g, 4294969228, "clang::InjectedClassNameType::isSugared"),
      add_named_node(&mut g, 4294969229, "clang::InjectedClassNameType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_INJECTEDCLASSNAMETYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCSTRINGLITERAL, "clang::ObjCStringLiteral");
  g.add_edge((CLASS_OBJCSTRINGLITERAL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCSTRINGLITERAL, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCSTRINGLITERAL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_INHERITABLEPARAMATTR, "clang::InheritableParamAttr");
  g.add_edge((CLASS_INHERITABLEPARAMATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_INHERITABLEPARAMATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_INHERITABLEPARAMATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_WARNUNUSEDATTR, "clang::WarnUnusedAttr");
  g.add_edge((CLASS_WARNUNUSEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_WARNUNUSEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_WARNUNUSEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NONNULLATTR, "clang::NonNullAttr");
  g.add_edge((CLASS_NONNULLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NONNULLATTR, META_SUBCLASS, CLASS_INHERITABLEPARAMATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NONNULLATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MUSTTAILATTR, "clang::MustTailAttr");
  g.add_edge((CLASS_MUSTTAILATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MUSTTAILATTR, META_SUBCLASS, CLASS_STMTATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MUSTTAILATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPCANONICALLOOP, "clang::OMPCanonicalLoop");
  g.add_edge((CLASS_OMPCANONICALLOOP, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPCANONICALLOOP, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPCANONICALLOOP, META_METHOD, methods));
  }

  g.add_named_node(CLASS_WEBASSEMBLYIMPORTNAMEATTR, "clang::WebAssemblyImportNameAttr");
  g.add_edge((CLASS_WEBASSEMBLYIMPORTNAMEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_WEBASSEMBLYIMPORTNAMEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_WEBASSEMBLYIMPORTNAMEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPSIMDDIRECTIVE, "clang::OMPSimdDirective");
  g.add_edge((CLASS_OMPSIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPSIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPSIMDDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CFCONSUMEDATTR, "clang::CFConsumedAttr");
  g.add_edge((CLASS_CFCONSUMEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CFCONSUMEDATTR, META_SUBCLASS, CLASS_INHERITABLEPARAMATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CFCONSUMEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_STMTATTR, "clang::StmtAttr");
  g.add_edge((CLASS_STMTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_STMTATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_STMTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SWIFTCONTEXTATTR, "clang::SwiftContextAttr");
  g.add_edge((CLASS_SWIFTCONTEXTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTCONTEXTATTR, META_SUBCLASS, CLASS_PARAMETERABIATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SWIFTCONTEXTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ARMMVESTRICTPOLYMORPHISMATTR, "clang::ArmMveStrictPolymorphismAttr");
  g.add_edge((CLASS_ARMMVESTRICTPOLYMORPHISMATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARMMVESTRICTPOLYMORPHISMATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ARMMVESTRICTPOLYMORPHISMATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SWIFTERRORRESULTATTR, "clang::SwiftErrorResultAttr");
  g.add_edge((CLASS_SWIFTERRORRESULTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTERRORRESULTATTR, META_SUBCLASS, CLASS_PARAMETERABIATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SWIFTERRORRESULTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTARGETSIMDDIRECTIVE, "clang::OMPTargetSimdDirective");
  g.add_edge((CLASS_OMPTARGETSIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETSIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTARGETSIMDDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_THREADATTR, "clang::ThreadAttr");
  g.add_edge((CLASS_THREADATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_THREADATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_THREADATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCGCATTR, "clang::ObjCGCAttr");
  g.add_edge((CLASS_OBJCGCATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCGCATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCGCATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OPENCLUNROLLHINTATTR, "clang::OpenCLUnrollHintAttr");
  g.add_edge((CLASS_OPENCLUNROLLHINTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OPENCLUNROLLHINTATTR, META_SUBCLASS, CLASS_STMTATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OPENCLUNROLLHINTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTADDRESSSPACETYPELOC, "clang::DependentAddressSpaceTypeLoc");
  g.add_edge((CLASS_DEPENDENTADDRESSSPACETYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969248, "clang::DependentAddressSpaceTypeLoc::getAttrNameLoc"),
      add_named_node(&mut g, 4294969249, "clang::DependentAddressSpaceTypeLoc::getAttrExprOperand"),
      add_named_node(&mut g, 4294969250, "clang::DependentAddressSpaceTypeLoc::getAttrOperandParensRange"),
      add_named_node(&mut g, 4294969251, "clang::DependentAddressSpaceTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, 4294969252, "clang::DependentAddressSpaceTypeLoc::getInnerType"),
      add_named_node(&mut g, 4294969253, "clang::DependentAddressSpaceTypeLoc::getPointeeTypeLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTADDRESSSPACETYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BLOCKPOINTERTYPELOC, "clang::BlockPointerTypeLoc");
  g.add_edge((CLASS_BLOCKPOINTERTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969255, "clang::BlockPointerTypeLoc::getCaretLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BLOCKPOINTERTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPENONNULLATTR, "clang::TypeNonNullAttr");
  g.add_edge((CLASS_TYPENONNULLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPENONNULLATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPENONNULLATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SYCLSPECIALCLASSATTR, "clang::SYCLSpecialClassAttr");
  g.add_edge((CLASS_SYCLSPECIALCLASSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SYCLSPECIALCLASSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SYCLSPECIALCLASSATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ARGUMENTWITHTYPETAGATTR, "clang::ArgumentWithTypeTagAttr");
  g.add_edge((CLASS_ARGUMENTWITHTYPETAGATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARGUMENTWITHTYPETAGATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ARGUMENTWITHTYPETAGATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NODEREFATTR, "clang::NoDerefAttr");
  g.add_edge((CLASS_NODEREFATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NODEREFATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NODEREFATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OVERLOADEXPR, "clang::OverloadExpr");
  g.add_edge((CLASS_OVERLOADEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OVERLOADEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969261, "clang::OverloadExpr::getNamingClass"),
      add_named_node(&mut g, 4294969262, "clang::OverloadExpr::decls_begin"),
      add_named_node(&mut g, 4294969263, "clang::OverloadExpr::decls_end"),
      add_named_node(&mut g, 4294969264, "clang::OverloadExpr::decls"),
      add_named_node(&mut g, 4294969265, "clang::OverloadExpr::getNumDecls"),
      add_named_node(&mut g, 4294969266, "clang::OverloadExpr::getNameInfo"),
      add_named_node(&mut g, 4294969267, "clang::OverloadExpr::getName"),
      add_named_node(&mut g, 4294969268, "clang::OverloadExpr::getNameLoc"),
      add_named_node(&mut g, 4294969269, "clang::OverloadExpr::getQualifier"),
      add_named_node(&mut g, 4294969270, "clang::OverloadExpr::getQualifierLoc"),
      add_named_node(&mut g, 4294969271, "clang::OverloadExpr::getTemplateKeywordLoc"),
      add_named_node(&mut g, 4294969272, "clang::OverloadExpr::getLAngleLoc"),
      add_named_node(&mut g, 4294969273, "clang::OverloadExpr::getRAngleLoc"),
      add_named_node(&mut g, 4294969274, "clang::OverloadExpr::hasTemplateKeyword"),
      add_named_node(&mut g, 4294969275, "clang::OverloadExpr::hasExplicitTemplateArgs"),
      add_named_node(&mut g, 4294969276, "clang::OverloadExpr::getTemplateArgs"),
      add_named_node(&mut g, 4294969277, "clang::OverloadExpr::getNumTemplateArgs"),
      add_named_node(&mut g, 4294969278, "clang::OverloadExpr::template_arguments"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OVERLOADEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_EXCLUDEFROMEXPLICITINSTANTIATIONATTR, "clang::ExcludeFromExplicitInstantiationAttr");
  g.add_edge((CLASS_EXCLUDEFROMEXPLICITINSTANTIATIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_EXCLUDEFROMEXPLICITINSTANTIATIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_EXCLUDEFROMEXPLICITINSTANTIATIONATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ATTR, "clang::Attr");
  g.add_edge((CLASS_ATTR, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_INCOMPLETEARRAYTYPELOC, "clang::IncompleteArrayTypeLoc");
  g.add_edge((CLASS_INCOMPLETEARRAYTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_INCOMPLETEARRAYTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTILEDIRECTIVE, "clang::OMPTileDirective");
  g.add_edge((CLASS_OMPTILEDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTILEDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPTRANSFORMATIONDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTILEDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_INHERITABLEATTR, "clang::InheritableAttr");
  g.add_edge((CLASS_INHERITABLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_INHERITABLEATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_INHERITABLEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PACKEXPANSIONTYPELOC, "clang::PackExpansionTypeLoc");
  g.add_edge((CLASS_PACKEXPANSIONTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969285, "clang::PackExpansionTypeLoc::getEllipsisLoc"),
      add_named_node(&mut g, 4294969286, "clang::PackExpansionTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, 4294969287, "clang::PackExpansionTypeLoc::getPatternLoc"),
      add_named_node(&mut g, 4294969288, "clang::PackExpansionTypeLoc::getInnerType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PACKEXPANSIONTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_AMDGPUKERNELCALLATTR, "clang::AMDGPUKernelCallAttr");
  g.add_edge((CLASS_AMDGPUKERNELCALLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_AMDGPUKERNELCALLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_AMDGPUKERNELCALLATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_VARTEMPLATEPARTIALSPECIALIZATIONDECL, "clang::VarTemplatePartialSpecializationDecl");
  g.add_edge((CLASS_VARTEMPLATEPARTIALSPECIALIZATIONDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_VARTEMPLATEPARTIALSPECIALIZATIONDECL, META_SUBCLASS, CLASS_VARTEMPLATESPECIALIZATIONDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969291, "clang::VarTemplatePartialSpecializationDecl::getTemplateParameters"),
      add_named_node(&mut g, 4294969292, "clang::VarTemplatePartialSpecializationDecl::getTemplateArgsAsWritten"),
      add_named_node(&mut g, 4294969293, "clang::VarTemplatePartialSpecializationDecl::hasAssociatedConstraints"),
      add_named_node(&mut g, 4294969294, "clang::VarTemplatePartialSpecializationDecl::getInstantiatedFromMember"),
      add_named_node(&mut g, 4294969295, "clang::VarTemplatePartialSpecializationDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_VARTEMPLATEPARTIALSPECIALIZATIONDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ALLOCALIGNATTR, "clang::AllocAlignAttr");
  g.add_edge((CLASS_ALLOCALIGNATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ALLOCALIGNATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ALLOCALIGNATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ARMSTREAMINGCOMPATIBLEATTR, "clang::ArmStreamingCompatibleAttr");
  g.add_edge((CLASS_ARMSTREAMINGCOMPATIBLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARMSTREAMINGCOMPATIBLEATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ARMSTREAMINGCOMPATIBLEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ASSUMPTIONATTR, "clang::AssumptionAttr");
  g.add_edge((CLASS_ASSUMPTIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ASSUMPTIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ASSUMPTIONATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_HLSLSV_GROUPINDEXATTR, "clang::HLSLSV_GroupIndexAttr");
  g.add_edge((CLASS_HLSLSV_GROUPINDEXATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_HLSLSV_GROUPINDEXATTR, META_SUBCLASS, CLASS_HLSLANNOTATIONATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_HLSLSV_GROUPINDEXATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_IMAGINARYLITERAL, "clang::ImaginaryLiteral");
  g.add_edge((CLASS_IMAGINARYLITERAL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_IMAGINARYLITERAL, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969301, "clang::ImaginaryLiteral::getSubExpr"),
      add_named_node(&mut g, 4294969302, "clang::ImaginaryLiteral::getBeginLoc"),
      add_named_node(&mut g, 4294969303, "clang::ImaginaryLiteral::getEndLoc"),
      add_named_node(&mut g, 4294969304, "clang::ImaginaryLiteral::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_IMAGINARYLITERAL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCNSOBJECTATTR, "clang::ObjCNSObjectAttr");
  g.add_edge((CLASS_OBJCNSOBJECTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCNSOBJECTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCNSOBJECTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CMSENSCALLATTR, "clang::CmseNSCallAttr");
  g.add_edge((CLASS_CMSENSCALLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CMSENSCALLATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CMSENSCALLATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BTFTYPETAGATTR, "clang::BTFTypeTagAttr");
  g.add_edge((CLASS_BTFTYPETAGATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BTFTYPETAGATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BTFTYPETAGATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPENULLABLEATTR, "clang::TypeNullableAttr");
  g.add_edge((CLASS_TYPENULLABLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPENULLABLEATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPENULLABLEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPUNROLLDIRECTIVE, "clang::OMPUnrollDirective");
  g.add_edge((CLASS_OMPUNROLLDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPUNROLLDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPTRANSFORMATIONDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPUNROLLDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TRYACQUIRECAPABILITYATTR, "clang::TryAcquireCapabilityAttr");
  g.add_edge((CLASS_TRYACQUIRECAPABILITYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TRYACQUIRECAPABILITYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TRYACQUIRECAPABILITYATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_HLSLANNOTATIONATTR, "clang::HLSLAnnotationAttr");
  g.add_edge((CLASS_HLSLANNOTATIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_HLSLANNOTATIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_HLSLANNOTATIONATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ARMINATTR, "clang::ArmInAttr");
  g.add_edge((CLASS_ARMINATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARMINATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ARMINATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXSCALARVALUEINITEXPR, "clang::CXXScalarValueInitExpr");
  g.add_edge((CLASS_CXXSCALARVALUEINITEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXSCALARVALUEINITEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969314, "clang::CXXScalarValueInitExpr::getTypeSourceInfo"),
      add_named_node(&mut g, 4294969315, "clang::CXXScalarValueInitExpr::getRParenLoc"),
      add_named_node(&mut g, 4294969316, "clang::CXXScalarValueInitExpr::getBeginLoc"),
      add_named_node(&mut g, 4294969317, "clang::CXXScalarValueInitExpr::getEndLoc"),
      add_named_node(&mut g, 4294969318, "clang::CXXScalarValueInitExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXSCALARVALUEINITEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CSTYLECASTEXPR, "clang::CStyleCastExpr");
  g.add_edge((CLASS_CSTYLECASTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CSTYLECASTEXPR, META_SUBCLASS, CLASS_EXPLICITCASTEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969320, "clang::CStyleCastExpr::getLParenLoc"),
      add_named_node(&mut g, 4294969321, "clang::CStyleCastExpr::getRParenLoc"),
      add_named_node(&mut g, 4294969322, "clang::CStyleCastExpr::getBeginLoc"),
      add_named_node(&mut g, 4294969323, "clang::CStyleCastExpr::getEndLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CSTYLECASTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_LIKELYATTR, "clang::LikelyAttr");
  g.add_edge((CLASS_LIKELYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LIKELYATTR, META_SUBCLASS, CLASS_STMTATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_LIKELYATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_UNUSEDATTR, "clang::UnusedAttr");
  g.add_edge((CLASS_UNUSEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNUSEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNUSEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ARMBUILTINALIASATTR, "clang::ArmBuiltinAliasAttr");
  g.add_edge((CLASS_ARMBUILTINALIASATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARMBUILTINALIASATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ARMBUILTINALIASATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NOMERGEATTR, "clang::NoMergeAttr");
  g.add_edge((CLASS_NOMERGEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOMERGEATTR, META_SUBCLASS, CLASS_DECLORSTMTATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NOMERGEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCFORCOLLECTIONSTMT, "clang::ObjCForCollectionStmt");
  g.add_edge((CLASS_OBJCFORCOLLECTIONSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCFORCOLLECTIONSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCFORCOLLECTIONSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TAGTYPELOC, "clang::TagTypeLoc");
  g.add_edge((CLASS_TAGTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969330, "clang::TagTypeLoc::getDecl"),
      add_named_node(&mut g, 4294969331, "clang::TagTypeLoc::isDefinition"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TAGTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPCAPTUREKINDATTR, "clang::OMPCaptureKindAttr");
  g.add_edge((CLASS_OMPCAPTUREKINDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPCAPTUREKINDATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPCAPTUREKINDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OPENCLGLOBALADDRESSSPACEATTR, "clang::OpenCLGlobalAddressSpaceAttr");
  g.add_edge((CLASS_OPENCLGLOBALADDRESSSPACEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OPENCLGLOBALADDRESSSPACEATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OPENCLGLOBALADDRESSSPACEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TRANSLATIONUNITDECL, "clang::TranslationUnitDecl");
  g.add_edge((CLASS_TRANSLATIONUNITDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TRANSLATIONUNITDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969335, "clang::TranslationUnitDecl::getASTContext"),
      add_named_node(&mut g, 4294969336, "clang::TranslationUnitDecl::getAnonymousNamespace"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TRANSLATIONUNITDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CODEALIGNATTR, "clang::CodeAlignAttr");
  g.add_edge((CLASS_CODEALIGNATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CODEALIGNATTR, META_SUBCLASS, CLASS_STMTATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CODEALIGNATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CALLABLEWHENATTR, "clang::CallableWhenAttr");
  g.add_edge((CLASS_CALLABLEWHENATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CALLABLEWHENATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CALLABLEWHENATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OPENCLGLOBALDEVICEADDRESSSPACEATTR, "clang::OpenCLGlobalDeviceAddressSpaceAttr");
  g.add_edge((CLASS_OPENCLGLOBALDEVICEADDRESSSPACEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OPENCLGLOBALDEVICEADDRESSSPACEATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OPENCLGLOBALDEVICEADDRESSSPACEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ARRAYTYPELOC, "clang::ArrayTypeLoc");
  g.add_edge((CLASS_ARRAYTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969341, "clang::ArrayTypeLoc::getLBracketLoc"),
      add_named_node(&mut g, 4294969342, "clang::ArrayTypeLoc::getRBracketLoc"),
      add_named_node(&mut g, 4294969343, "clang::ArrayTypeLoc::getBracketsRange"),
      add_named_node(&mut g, 4294969344, "clang::ArrayTypeLoc::getSizeExpr"),
      add_named_node(&mut g, 4294969345, "clang::ArrayTypeLoc::getElementLoc"),
      add_named_node(&mut g, 4294969346, "clang::ArrayTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, 4294969347, "clang::ArrayTypeLoc::getInnerType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ARRAYTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CUDASHAREDATTR, "clang::CUDASharedAttr");
  g.add_edge((CLASS_CUDASHAREDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CUDASHAREDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CUDASHAREDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_EXPORTDECL, "clang::ExportDecl");
  g.add_edge((CLASS_EXPORTDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_EXPORTDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969350, "clang::ExportDecl::getExportLoc"),
      add_named_node(&mut g, 4294969351, "clang::ExportDecl::getRBraceLoc"),
      add_named_node(&mut g, 4294969352, "clang::ExportDecl::hasBraces"),
      add_named_node(&mut g, 4294969353, "clang::ExportDecl::getEndLoc"),
      add_named_node(&mut g, 4294969354, "clang::ExportDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_EXPORTDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PARAMTYPESTATEATTR, "clang::ParamTypestateAttr");
  g.add_edge((CLASS_PARAMTYPESTATEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PARAMTYPESTATEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PARAMTYPESTATEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPLOOPDIRECTIVE, "clang::OMPLoopDirective");
  g.add_edge((CLASS_OMPLOOPDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPLOOPDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPBASEDDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPLOOPDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_IFSTMT, "clang::IfStmt");
  g.add_edge((CLASS_IFSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_IFSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969358, "clang::IfStmt::hasInitStorage"),
      add_named_node(&mut g, 4294969359, "clang::IfStmt::hasVarStorage"),
      add_named_node(&mut g, 4294969360, "clang::IfStmt::hasElseStorage"),
      add_named_node(&mut g, 4294969361, "clang::IfStmt::getCond"),
      add_named_node(&mut g, 4294969362, "clang::IfStmt::getThen"),
      add_named_node(&mut g, 4294969363, "clang::IfStmt::getElse"),
      add_named_node(&mut g, 4294969364, "clang::IfStmt::getConditionVariable"),
      add_named_node(&mut g, 4294969365, "clang::IfStmt::getConditionVariableDeclStmt"),
      add_named_node(&mut g, 4294969366, "clang::IfStmt::getInit"),
      add_named_node(&mut g, 4294969367, "clang::IfStmt::getIfLoc"),
      add_named_node(&mut g, 4294969368, "clang::IfStmt::getElseLoc"),
      add_named_node(&mut g, 4294969369, "clang::IfStmt::isConsteval"),
      add_named_node(&mut g, 4294969370, "clang::IfStmt::isNonNegatedConsteval"),
      add_named_node(&mut g, 4294969371, "clang::IfStmt::isNegatedConsteval"),
      add_named_node(&mut g, 4294969372, "clang::IfStmt::isConstexpr"),
      add_named_node(&mut g, 4294969373, "clang::IfStmt::getStatementKind"),
      add_named_node(&mut g, 4294969374, "clang::IfStmt::isObjCAvailabilityCheck"),
      add_named_node(&mut g, 4294969375, "clang::IfStmt::getBeginLoc"),
      add_named_node(&mut g, 4294969376, "clang::IfStmt::getEndLoc"),
      add_named_node(&mut g, 4294969377, "clang::IfStmt::getLParenLoc"),
      add_named_node(&mut g, 4294969378, "clang::IfStmt::getRParenLoc"),
      add_named_node(&mut g, 4294969379, "clang::IfStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_IFSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BPFPRESERVESTATICOFFSETATTR, "clang::BPFPreserveStaticOffsetAttr");
  g.add_edge((CLASS_BPFPRESERVESTATICOFFSETATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BPFPRESERVESTATICOFFSETATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BPFPRESERVESTATICOFFSETATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ARMSTREAMINGATTR, "clang::ArmStreamingAttr");
  g.add_edge((CLASS_ARMSTREAMINGATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARMSTREAMINGATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ARMSTREAMINGATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OPENCLGLOBALHOSTADDRESSSPACEATTR, "clang::OpenCLGlobalHostAddressSpaceAttr");
  g.add_edge((CLASS_OPENCLGLOBALHOSTADDRESSSPACEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OPENCLGLOBALHOSTADDRESSSPACEATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OPENCLGLOBALHOSTADDRESSSPACEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_RECOVERYEXPR, "clang::RecoveryExpr");
  g.add_edge((CLASS_RECOVERYEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RECOVERYEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969384, "clang::RecoveryExpr::subExpressions"),
      add_named_node(&mut g, 4294969385, "clang::RecoveryExpr::getBeginLoc"),
      add_named_node(&mut g, 4294969386, "clang::RecoveryExpr::getEndLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_RECOVERYEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OPENCLKERNELATTR, "clang::OpenCLKernelAttr");
  g.add_edge((CLASS_OPENCLKERNELATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OPENCLKERNELATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OPENCLKERNELATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_LOOPHINTATTR, "clang::LoopHintAttr");
  g.add_edge((CLASS_LOOPHINTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LOOPHINTATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_LOOPHINTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_EXTERNCCONTEXTDECL, "clang::ExternCContextDecl");
  g.add_edge((CLASS_EXTERNCCONTEXTDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_EXTERNCCONTEXTDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_EXTERNCCONTEXTDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ARMPRESERVESATTR, "clang::ArmPreservesAttr");
  g.add_edge((CLASS_ARMPRESERVESATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARMPRESERVESATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ARMPRESERVESATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCKINDOFATTR, "clang::ObjCKindOfAttr");
  g.add_edge((CLASS_OBJCKINDOFATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCKINDOFATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCKINDOFATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FUNCTIONTEMPLATEDECL, "clang::FunctionTemplateDecl");
  g.add_edge((CLASS_FUNCTIONTEMPLATEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FUNCTIONTEMPLATEDECL, META_SUBCLASS, CLASS_REDECLARABLETEMPLATEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969393, "clang::FunctionTemplateDecl::getTemplatedDecl"),
      add_named_node(&mut g, 4294969394, "clang::FunctionTemplateDecl::isThisDeclarationADefinition"),
      add_named_node(&mut g, 4294969395, "clang::FunctionTemplateDecl::getCanonicalDecl"),
      add_named_node(&mut g, 4294969396, "clang::FunctionTemplateDecl::getPreviousDecl"),
      add_named_node(&mut g, 4294969397, "clang::FunctionTemplateDecl::getMostRecentDecl"),
      add_named_node(&mut g, 4294969398, "clang::FunctionTemplateDecl::getInstantiatedFromMemberTemplate"),
      add_named_node(&mut g, 4294969399, "clang::FunctionTemplateDecl::specializations"),
      add_named_node(&mut g, 4294969400, "clang::FunctionTemplateDecl::spec_begin"),
      add_named_node(&mut g, 4294969401, "clang::FunctionTemplateDecl::spec_end"),
      add_named_node(&mut g, 4294969402, "clang::FunctionTemplateDecl::isAbbreviated"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FUNCTIONTEMPLATEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_STRICTFPATTR, "clang::StrictFPAttr");
  g.add_edge((CLASS_STRICTFPATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_STRICTFPATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_STRICTFPATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CFGUARDATTR, "clang::CFGuardAttr");
  g.add_edge((CLASS_CFGUARDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CFGUARDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CFGUARDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TARGETATTR, "clang::TargetAttr");
  g.add_edge((CLASS_TARGETATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TARGETATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TARGETATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NVPTXKERNELATTR, "clang::NVPTXKernelAttr");
  g.add_edge((CLASS_NVPTXKERNELATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NVPTXKERNELATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NVPTXKERNELATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CUDAGLOBALATTR, "clang::CUDAGlobalAttr");
  g.add_edge((CLASS_CUDAGLOBALATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CUDAGLOBALATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CUDAGLOBALATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_WEBASSEMBLYEXPORTNAMEATTR, "clang::WebAssemblyExportNameAttr");
  g.add_edge((CLASS_WEBASSEMBLYEXPORTNAMEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_WEBASSEMBLYEXPORTNAMEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_WEBASSEMBLYEXPORTNAMEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BUILTINTYPELOC, "clang::BuiltinTypeLoc");
  g.add_edge((CLASS_BUILTINTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969410, "clang::BuiltinTypeLoc::getBuiltinLoc"),
      add_named_node(&mut g, 4294969411, "clang::BuiltinTypeLoc::getNameLoc"),
      add_named_node(&mut g, 4294969412, "clang::BuiltinTypeLoc::getWrittenBuiltinSpecs"),
      add_named_node(&mut g, 4294969413, "clang::BuiltinTypeLoc::needsExtraLocalData"),
      add_named_node(&mut g, 4294969414, "clang::BuiltinTypeLoc::getExtraLocalDataSize"),
      add_named_node(&mut g, 4294969415, "clang::BuiltinTypeLoc::getExtraLocalDataAlignment"),
      add_named_node(&mut g, 4294969416, "clang::BuiltinTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, 4294969417, "clang::BuiltinTypeLoc::getWrittenSignSpec"),
      add_named_node(&mut g, 4294969418, "clang::BuiltinTypeLoc::hasWrittenSignSpec"),
      add_named_node(&mut g, 4294969419, "clang::BuiltinTypeLoc::getWrittenWidthSpec"),
      add_named_node(&mut g, 4294969420, "clang::BuiltinTypeLoc::hasWrittenWidthSpec"),
      add_named_node(&mut g, 4294969421, "clang::BuiltinTypeLoc::getWrittenTypeSpec"),
      add_named_node(&mut g, 4294969422, "clang::BuiltinTypeLoc::hasWrittenTypeSpec"),
      add_named_node(&mut g, 4294969423, "clang::BuiltinTypeLoc::hasModeAttr"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BUILTINTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PTR64ATTR, "clang::Ptr64Attr");
  g.add_edge((CLASS_PTR64ATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PTR64ATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PTR64ATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FUNCTIONNOPROTOTYPE, "clang::FunctionNoProtoType");
  g.add_edge((CLASS_FUNCTIONNOPROTOTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FUNCTIONNOPROTOTYPE, META_SUBCLASS, CLASS_FUNCTIONTYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969426, "clang::FunctionNoProtoType::isSugared"),
      add_named_node(&mut g, 4294969427, "clang::FunctionNoProtoType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FUNCTIONNOPROTOTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_AMDGPUWAVESPEREUATTR, "clang::AMDGPUWavesPerEUAttr");
  g.add_edge((CLASS_AMDGPUWAVESPEREUATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_AMDGPUWAVESPEREUATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_AMDGPUWAVESPEREUATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OPENCLCONSTANTADDRESSSPACEATTR, "clang::OpenCLConstantAddressSpaceAttr");
  g.add_edge((CLASS_OPENCLCONSTANTADDRESSSPACEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OPENCLCONSTANTADDRESSSPACEATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OPENCLCONSTANTADDRESSSPACEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPPARALLELMASKEDTASKLOOPDIRECTIVE, "clang::OMPParallelMaskedTaskLoopDirective");
  g.add_edge((CLASS_OMPPARALLELMASKEDTASKLOOPDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPPARALLELMASKEDTASKLOOPDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPPARALLELMASKEDTASKLOOPDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPEATTR, "clang::TypeAttr");
  g.add_edge((CLASS_TYPEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPEATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PRAGMACLANGTEXTSECTIONATTR, "clang::PragmaClangTextSectionAttr");
  g.add_edge((CLASS_PRAGMACLANGTEXTSECTIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PRAGMACLANGTEXTSECTIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PRAGMACLANGTEXTSECTIONATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PARENTYPE, "clang::ParenType");
  g.add_edge((CLASS_PARENTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PARENTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969434, "clang::ParenType::getInnerType"),
      add_named_node(&mut g, 4294969435, "clang::ParenType::isSugared"),
      add_named_node(&mut g, 4294969436, "clang::ParenType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PARENTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ASSERTSHAREDLOCKATTR, "clang::AssertSharedLockAttr");
  g.add_edge((CLASS_ASSERTSHAREDLOCKATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ASSERTSHAREDLOCKATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ASSERTSHAREDLOCKATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NOESCAPEATTR, "clang::NoEscapeAttr");
  g.add_edge((CLASS_NOESCAPEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOESCAPEATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NOESCAPEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BLOCKDECL, "clang::BlockDecl");
  g.add_edge((CLASS_BLOCKDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BLOCKDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969440, "clang::BlockDecl::getCaretLocation"),
      add_named_node(&mut g, 4294969441, "clang::BlockDecl::isVariadic"),
      add_named_node(&mut g, 4294969442, "clang::BlockDecl::getCompoundBody"),
      add_named_node(&mut g, 4294969443, "clang::BlockDecl::getBody"),
      add_named_node(&mut g, 4294969444, "clang::BlockDecl::getSignatureAsWritten"),
      add_named_node(&mut g, 4294969445, "clang::BlockDecl::parameters"),
      add_named_node(&mut g, 4294969446, "clang::BlockDecl::param_empty"),
      add_named_node(&mut g, 4294969447, "clang::BlockDecl::param_begin"),
      add_named_node(&mut g, 4294969448, "clang::BlockDecl::param_end"),
      add_named_node(&mut g, 4294969449, "clang::BlockDecl::param_size"),
      add_named_node(&mut g, 4294969450, "clang::BlockDecl::getNumParams"),
      add_named_node(&mut g, 4294969451, "clang::BlockDecl::hasCaptures"),
      add_named_node(&mut g, 4294969452, "clang::BlockDecl::getNumCaptures"),
      add_named_node(&mut g, 4294969453, "clang::BlockDecl::captures"),
      add_named_node(&mut g, 4294969454, "clang::BlockDecl::capture_begin"),
      add_named_node(&mut g, 4294969455, "clang::BlockDecl::capture_end"),
      add_named_node(&mut g, 4294969456, "clang::BlockDecl::capturesCXXThis"),
      add_named_node(&mut g, 4294969457, "clang::BlockDecl::blockMissingReturnType"),
      add_named_node(&mut g, 4294969458, "clang::BlockDecl::isConversionFromLambda"),
      add_named_node(&mut g, 4294969459, "clang::BlockDecl::doesNotEscape"),
      add_named_node(&mut g, 4294969460, "clang::BlockDecl::canAvoidCopyToHeap"),
      add_named_node(&mut g, 4294969461, "clang::BlockDecl::getBlockManglingNumber"),
      add_named_node(&mut g, 4294969462, "clang::BlockDecl::getBlockManglingContextDecl"),
      add_named_node(&mut g, 4294969463, "clang::BlockDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BLOCKDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BUILTINATTR, "clang::BuiltinAttr");
  g.add_edge((CLASS_BUILTINATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BUILTINATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BUILTINATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NOCOMMONATTR, "clang::NoCommonAttr");
  g.add_edge((CLASS_NOCOMMONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOCOMMONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NOCOMMONATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SWITCHCASE, "clang::SwitchCase");
  g.add_edge((CLASS_SWITCHCASE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWITCHCASE, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969467, "clang::SwitchCase::getNextSwitchCase"),
      add_named_node(&mut g, 4294969468, "clang::SwitchCase::getKeywordLoc"),
      add_named_node(&mut g, 4294969469, "clang::SwitchCase::getColonLoc"),
      add_named_node(&mut g, 4294969470, "clang::SwitchCase::getSubStmt"),
      add_named_node(&mut g, 4294969471, "clang::SwitchCase::getBeginLoc"),
      add_named_node(&mut g, 4294969472, "clang::SwitchCase::getEndLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SWITCHCASE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_LIFETIMEEXTENDEDTEMPORARYDECL, "clang::LifetimeExtendedTemporaryDecl");
  g.add_edge((CLASS_LIFETIMEEXTENDEDTEMPORARYDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LIFETIMEEXTENDEDTEMPORARYDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969474, "clang::LifetimeExtendedTemporaryDecl::getExtendingDecl"),
      add_named_node(&mut g, 4294969475, "clang::LifetimeExtendedTemporaryDecl::getStorageDuration"),
      add_named_node(&mut g, 4294969476, "clang::LifetimeExtendedTemporaryDecl::getTemporaryExpr"),
      add_named_node(&mut g, 4294969477, "clang::LifetimeExtendedTemporaryDecl::getManglingNumber"),
      add_named_node(&mut g, 4294969478, "clang::LifetimeExtendedTemporaryDecl::getValue"),
      add_named_node(&mut g, 4294969479, "clang::LifetimeExtendedTemporaryDecl::childrenExpr"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_LIFETIMEEXTENDEDTEMPORARYDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PRAGMACLANGDATASECTIONATTR, "clang::PragmaClangDataSectionAttr");
  g.add_edge((CLASS_PRAGMACLANGDATASECTIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PRAGMACLANGDATASECTIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PRAGMACLANGDATASECTIONATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_VISIBILITYATTR, "clang::VisibilityAttr");
  g.add_edge((CLASS_VISIBILITYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_VISIBILITYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_VISIBILITYATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CFICANONICALJUMPTABLEATTR, "clang::CFICanonicalJumpTableAttr");
  g.add_edge((CLASS_CFICANONICALJUMPTABLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CFICANONICALJUMPTABLEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CFICANONICALJUMPTABLEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CFRETURNSRETAINEDATTR, "clang::CFReturnsRetainedAttr");
  g.add_edge((CLASS_CFRETURNSRETAINEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CFRETURNSRETAINEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CFRETURNSRETAINEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_USEHANDLEATTR, "clang::UseHandleAttr");
  g.add_edge((CLASS_USEHANDLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_USEHANDLEATTR, META_SUBCLASS, CLASS_INHERITABLEPARAMATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_USEHANDLEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ARMOUTATTR, "clang::ArmOutAttr");
  g.add_edge((CLASS_ARMOUTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARMOUTATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ARMOUTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTARGETTEAMSDISTRIBUTEDIRECTIVE, "clang::OMPTargetTeamsDistributeDirective");
  g.add_edge((CLASS_OMPTARGETTEAMSDISTRIBUTEDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETTEAMSDISTRIBUTEDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTARGETTEAMSDISTRIBUTEDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_COROLIFETIMEBOUNDATTR, "clang::CoroLifetimeBoundAttr");
  g.add_edge((CLASS_COROLIFETIMEBOUNDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_COROLIFETIMEBOUNDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_COROLIFETIMEBOUNDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SHAREDTRYLOCKFUNCTIONATTR, "clang::SharedTrylockFunctionAttr");
  g.add_edge((CLASS_SHAREDTRYLOCKFUNCTIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SHAREDTRYLOCKFUNCTIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SHAREDTRYLOCKFUNCTIONATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPDECLARESIMDDECLATTR, "clang::OMPDeclareSimdDeclAttr");
  g.add_edge((CLASS_OMPDECLARESIMDDECLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPDECLARESIMDDECLATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPDECLARESIMDDECLATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DECLORSTMTATTR, "clang::DeclOrStmtAttr");
  g.add_edge((CLASS_DECLORSTMTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DECLORSTMTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DECLORSTMTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCROOTCLASSATTR, "clang::ObjCRootClassAttr");
  g.add_edge((CLASS_OBJCROOTCLASSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCROOTCLASSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCROOTCLASSATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TARGETCLONESATTR, "clang::TargetClonesAttr");
  g.add_edge((CLASS_TARGETCLONESATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TARGETCLONESATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TARGETCLONESATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_RENDERSCRIPTKERNELATTR, "clang::RenderScriptKernelAttr");
  g.add_edge((CLASS_RENDERSCRIPTKERNELATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RENDERSCRIPTKERNELATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_RENDERSCRIPTKERNELATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ASSUMEALIGNEDATTR, "clang::AssumeAlignedAttr");
  g.add_edge((CLASS_ASSUMEALIGNEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ASSUMEALIGNEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ASSUMEALIGNEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_AVAILABLEONLYINDEFAULTEVALMETHODATTR, "clang::AvailableOnlyInDefaultEvalMethodAttr");
  g.add_edge((CLASS_AVAILABLEONLYINDEFAULTEVALMETHODATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_AVAILABLEONLYINDEFAULTEVALMETHODATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_AVAILABLEONLYINDEFAULTEVALMETHODATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TEMPLATESPECIALIZATIONTYPELOC, "clang::TemplateSpecializationTypeLoc");
  g.add_edge((CLASS_TEMPLATESPECIALIZATIONTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969497, "clang::TemplateSpecializationTypeLoc::getTemplateKeywordLoc"),
      add_named_node(&mut g, 4294969498, "clang::TemplateSpecializationTypeLoc::getLAngleLoc"),
      add_named_node(&mut g, 4294969499, "clang::TemplateSpecializationTypeLoc::getRAngleLoc"),
      add_named_node(&mut g, 4294969500, "clang::TemplateSpecializationTypeLoc::getNumArgs"),
      add_named_node(&mut g, 4294969501, "clang::TemplateSpecializationTypeLoc::getTemplateNameLoc"),
      add_named_node(&mut g, 4294969502, "clang::TemplateSpecializationTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, 4294969503, "clang::TemplateSpecializationTypeLoc::getExtraLocalDataSize"),
      add_named_node(&mut g, 4294969504, "clang::TemplateSpecializationTypeLoc::getExtraLocalDataAlignment"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TEMPLATESPECIALIZATIONTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SEHFINALLYSTMT, "clang::SEHFinallyStmt");
  g.add_edge((CLASS_SEHFINALLYSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SEHFINALLYSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969506, "clang::SEHFinallyStmt::getBeginLoc"),
      add_named_node(&mut g, 4294969507, "clang::SEHFinallyStmt::getFinallyLoc"),
      add_named_node(&mut g, 4294969508, "clang::SEHFinallyStmt::getEndLoc"),
      add_named_node(&mut g, 4294969509, "clang::SEHFinallyStmt::getBlock"),
      add_named_node(&mut g, 4294969510, "clang::SEHFinallyStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SEHFINALLYSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PASSOBJECTSIZEATTR, "clang::PassObjectSizeAttr");
  g.add_edge((CLASS_PASSOBJECTSIZEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PASSOBJECTSIZEATTR, META_SUBCLASS, CLASS_INHERITABLEPARAMATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PASSOBJECTSIZEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SOURCELOCEXPR, "clang::SourceLocExpr");
  g.add_edge((CLASS_SOURCELOCEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SOURCELOCEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969513, "clang::SourceLocExpr::getBuiltinStr"),
      add_named_node(&mut g, 4294969514, "clang::SourceLocExpr::getIdentKind"),
      add_named_node(&mut g, 4294969515, "clang::SourceLocExpr::isIntType"),
      add_named_node(&mut g, 4294969516, "clang::SourceLocExpr::getParentContext"),
      add_named_node(&mut g, 4294969517, "clang::SourceLocExpr::getLocation"),
      add_named_node(&mut g, 4294969518, "clang::SourceLocExpr::getBeginLoc"),
      add_named_node(&mut g, 4294969519, "clang::SourceLocExpr::getEndLoc"),
      add_named_node(&mut g, 4294969520, "clang::SourceLocExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SOURCELOCEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_X86FORCEALIGNARGPOINTERATTR, "clang::X86ForceAlignArgPointerAttr");
  g.add_edge((CLASS_X86FORCEALIGNARGPOINTERATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_X86FORCEALIGNARGPOINTERATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_X86FORCEALIGNARGPOINTERATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_UNLIKELYATTR, "clang::UnlikelyAttr");
  g.add_edge((CLASS_UNLIKELYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNLIKELYATTR, META_SUBCLASS, CLASS_STMTATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNLIKELYATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CAPTUREDRECORDATTR, "clang::CapturedRecordAttr");
  g.add_edge((CLASS_CAPTUREDRECORDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CAPTUREDRECORDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CAPTUREDRECORDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CONSTANTARRAYTYPE, "clang::ConstantArrayType");
  g.add_edge((CLASS_CONSTANTARRAYTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONSTANTARRAYTYPE, META_SUBCLASS, CLASS_ARRAYTYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969525, "clang::ConstantArrayType::getSize"),
      add_named_node(&mut g, 4294969526, "clang::ConstantArrayType::getSizeExpr"),
      add_named_node(&mut g, 4294969527, "clang::ConstantArrayType::isSugared"),
      add_named_node(&mut g, 4294969528, "clang::ConstantArrayType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CONSTANTARRAYTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CASESTMT, "clang::CaseStmt");
  g.add_edge((CLASS_CASESTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CASESTMT, META_SUBCLASS, CLASS_SWITCHCASE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969530, "clang::CaseStmt::caseStmtIsGNURange"),
      add_named_node(&mut g, 4294969531, "clang::CaseStmt::getCaseLoc"),
      add_named_node(&mut g, 4294969532, "clang::CaseStmt::getEllipsisLoc"),
      add_named_node(&mut g, 4294969533, "clang::CaseStmt::getLHS"),
      add_named_node(&mut g, 4294969534, "clang::CaseStmt::getRHS"),
      add_named_node(&mut g, 4294969535, "clang::CaseStmt::getSubStmt"),
      add_named_node(&mut g, 4294969536, "clang::CaseStmt::getBeginLoc"),
      add_named_node(&mut g, 4294969537, "clang::CaseStmt::getEndLoc"),
      add_named_node(&mut g, 4294969538, "clang::CaseStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CASESTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_COMPLEXTYPE, "clang::ComplexType");
  g.add_edge((CLASS_COMPLEXTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_COMPLEXTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969540, "clang::ComplexType::getElementType"),
      add_named_node(&mut g, 4294969541, "clang::ComplexType::isSugared"),
      add_named_node(&mut g, 4294969542, "clang::ComplexType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_COMPLEXTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_WEAKATTR, "clang::WeakAttr");
  g.add_edge((CLASS_WEAKATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_WEAKATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_WEAKATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CONSTINITATTR, "clang::ConstInitAttr");
  g.add_edge((CLASS_CONSTINITATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONSTINITATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CONSTINITATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CONSTRUCTORATTR, "clang::ConstructorAttr");
  g.add_edge((CLASS_CONSTRUCTORATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONSTRUCTORATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CONSTRUCTORATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_UPTRATTR, "clang::UPtrAttr");
  g.add_edge((CLASS_UPTRATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UPTRATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UPTRATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DECLTYPETYPE, "clang::DecltypeType");
  g.add_edge((CLASS_DECLTYPETYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DECLTYPETYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969548, "clang::DecltypeType::getUnderlyingExpr"),
      add_named_node(&mut g, 4294969549, "clang::DecltypeType::getUnderlyingType"),
      add_named_node(&mut g, 4294969550, "clang::DecltypeType::desugar"),
      add_named_node(&mut g, 4294969551, "clang::DecltypeType::isSugared"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DECLTYPETYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SWIFTVERSIONEDADDITIONATTR, "clang::SwiftVersionedAdditionAttr");
  g.add_edge((CLASS_SWIFTVERSIONEDADDITIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTVERSIONEDADDITIONATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SWIFTVERSIONEDADDITIONATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTVECTORTYPE, "clang::DependentVectorType");
  g.add_edge((CLASS_DEPENDENTVECTORTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEPENDENTVECTORTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969554, "clang::DependentVectorType::getSizeExpr"),
      add_named_node(&mut g, 4294969555, "clang::DependentVectorType::getElementType"),
      add_named_node(&mut g, 4294969556, "clang::DependentVectorType::getAttributeLoc"),
      add_named_node(&mut g, 4294969557, "clang::DependentVectorType::getVectorKind"),
      add_named_node(&mut g, 4294969558, "clang::DependentVectorType::isSugared"),
      add_named_node(&mut g, 4294969559, "clang::DependentVectorType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTVECTORTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXDESTRUCTORDECL, "clang::CXXDestructorDecl");
  g.add_edge((CLASS_CXXDESTRUCTORDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXDESTRUCTORDECL, META_SUBCLASS, CLASS_CXXMETHODDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969561, "clang::CXXDestructorDecl::getOperatorDelete"),
      add_named_node(&mut g, 4294969562, "clang::CXXDestructorDecl::getOperatorDeleteThisArg"),
      add_named_node(&mut g, 4294969563, "clang::CXXDestructorDecl::getCanonicalDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXDESTRUCTORDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ALIGNEDATTR, "clang::AlignedAttr");
  g.add_edge((CLASS_ALIGNEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ALIGNEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ALIGNEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CONSUMABLEAUTOCASTATTR, "clang::ConsumableAutoCastAttr");
  g.add_edge((CLASS_CONSUMABLEAUTOCASTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONSUMABLEAUTOCASTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CONSUMABLEAUTOCASTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ENUMCONSTANTDECL, "clang::EnumConstantDecl");
  g.add_edge((CLASS_ENUMCONSTANTDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ENUMCONSTANTDECL, META_SUBCLASS, CLASS_VALUEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969567, "clang::EnumConstantDecl::getInitExpr"),
      add_named_node(&mut g, 4294969568, "clang::EnumConstantDecl::getInitVal"),
      add_named_node(&mut g, 4294969569, "clang::EnumConstantDecl::getSourceRange"),
      add_named_node(&mut g, 4294969570, "clang::EnumConstantDecl::getCanonicalDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ENUMCONSTANTDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_WEBASSEMBLYFUNCREFATTR, "clang::WebAssemblyFuncrefAttr");
  g.add_edge((CLASS_WEBASSEMBLYFUNCREFATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_WEBASSEMBLYFUNCREFATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_WEBASSEMBLYFUNCREFATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CONSUMABLESETONREADATTR, "clang::ConsumableSetOnReadAttr");
  g.add_edge((CLASS_CONSUMABLESETONREADATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONSUMABLESETONREADATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CONSUMABLESETONREADATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ARRAYINITLOOPEXPR, "clang::ArrayInitLoopExpr");
  g.add_edge((CLASS_ARRAYINITLOOPEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARRAYINITLOOPEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969574, "clang::ArrayInitLoopExpr::getCommonExpr"),
      add_named_node(&mut g, 4294969575, "clang::ArrayInitLoopExpr::getSubExpr"),
      add_named_node(&mut g, 4294969576, "clang::ArrayInitLoopExpr::getArraySize"),
      add_named_node(&mut g, 4294969577, "clang::ArrayInitLoopExpr::getBeginLoc"),
      add_named_node(&mut g, 4294969578, "clang::ArrayInitLoopExpr::getEndLoc"),
      add_named_node(&mut g, 4294969579, "clang::ArrayInitLoopExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ARRAYINITLOOPEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_REQUIRESCAPABILITYATTR, "clang::RequiresCapabilityAttr");
  g.add_edge((CLASS_REQUIRESCAPABILITYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_REQUIRESCAPABILITYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_REQUIRESCAPABILITYATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CORODISABLELIFETIMEBOUNDATTR, "clang::CoroDisableLifetimeBoundAttr");
  g.add_edge((CLASS_CORODISABLELIFETIMEBOUNDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CORODISABLELIFETIMEBOUNDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CORODISABLELIFETIMEBOUNDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTCOAWAITEXPR, "clang::DependentCoawaitExpr");
  g.add_edge((CLASS_DEPENDENTCOAWAITEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEPENDENTCOAWAITEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969583, "clang::DependentCoawaitExpr::getOperand"),
      add_named_node(&mut g, 4294969584, "clang::DependentCoawaitExpr::getOperatorCoawaitLookup"),
      add_named_node(&mut g, 4294969585, "clang::DependentCoawaitExpr::getKeywordLoc"),
      add_named_node(&mut g, 4294969586, "clang::DependentCoawaitExpr::getBeginLoc"),
      add_named_node(&mut g, 4294969587, "clang::DependentCoawaitExpr::getEndLoc"),
      add_named_node(&mut g, 4294969588, "clang::DependentCoawaitExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTCOAWAITEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CODESEGATTR, "clang::CodeSegAttr");
  g.add_edge((CLASS_CODESEGATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CODESEGATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CODESEGATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCTYPEPARAMTYPE, "clang::ObjCTypeParamType");
  g.add_edge((CLASS_OBJCTYPEPARAMTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCTYPEPARAMTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCTYPEPARAMTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPREQUIRESDECL, "clang::OMPRequiresDecl");
  g.add_edge((CLASS_OMPREQUIRESDECL, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPREQUIRESDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_USEDATTR, "clang::UsedAttr");
  g.add_edge((CLASS_USEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_USEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_USEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_REGCALLATTR, "clang::RegCallAttr");
  g.add_edge((CLASS_REGCALLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_REGCALLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_REGCALLATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_EXPR, "clang::Expr");
  g.add_edge((CLASS_EXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_EXPR, META_SUBCLASS, CLASS_VALUESTMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969595, "clang::Expr::getType"),
      add_named_node(&mut g, 4294969596, "clang::Expr::getDependence"),
      add_named_node(&mut g, 4294969597, "clang::Expr::isValueDependent"),
      add_named_node(&mut g, 4294969598, "clang::Expr::isTypeDependent"),
      add_named_node(&mut g, 4294969599, "clang::Expr::isInstantiationDependent"),
      add_named_node(&mut g, 4294969600, "clang::Expr::containsUnexpandedParameterPack"),
      add_named_node(&mut g, 4294969601, "clang::Expr::containsErrors"),
      add_named_node(&mut g, 4294969602, "clang::Expr::getExprLoc"),
      add_named_node(&mut g, 4294969603, "clang::Expr::isReadIfDiscardedInCPlusPlus11"),
      add_named_node(&mut g, 4294969604, "clang::Expr::isLValue"),
      add_named_node(&mut g, 4294969605, "clang::Expr::isPRValue"),
      add_named_node(&mut g, 4294969606, "clang::Expr::isXValue"),
      add_named_node(&mut g, 4294969607, "clang::Expr::isGLValue"),
      add_named_node(&mut g, 4294969608, "clang::Expr::getValueKind"),
      add_named_node(&mut g, 4294969609, "clang::Expr::getObjectKind"),
      add_named_node(&mut g, 4294969610, "clang::Expr::isOrdinaryOrBitFieldObject"),
      add_named_node(&mut g, 4294969611, "clang::Expr::refersToBitField"),
      add_named_node(&mut g, 4294969612, "clang::Expr::getSourceBitField"),
      add_named_node(&mut g, 4294969613, "clang::Expr::getReferencedDeclOfCallee"),
      add_named_node(&mut g, 4294969614, "clang::Expr::getObjCProperty"),
      add_named_node(&mut g, 4294969615, "clang::Expr::isObjCSelfExpr"),
      add_named_node(&mut g, 4294969616, "clang::Expr::refersToVectorElement"),
      add_named_node(&mut g, 4294969617, "clang::Expr::refersToMatrixElement"),
      add_named_node(&mut g, 4294969618, "clang::Expr::refersToGlobalRegisterVar"),
      add_named_node(&mut g, 4294969619, "clang::Expr::hasPlaceholderType"),
      add_named_node(&mut g, 4294969620, "clang::Expr::IgnoreUnlessSpelledInSource"),
      add_named_node(&mut g, 4294969621, "clang::Expr::IgnoreImpCasts"),
      add_named_node(&mut g, 4294969622, "clang::Expr::IgnoreCasts"),
      add_named_node(&mut g, 4294969623, "clang::Expr::IgnoreImplicit"),
      add_named_node(&mut g, 4294969624, "clang::Expr::IgnoreImplicitAsWritten"),
      add_named_node(&mut g, 4294969625, "clang::Expr::IgnoreParens"),
      add_named_node(&mut g, 4294969626, "clang::Expr::IgnoreParenImpCasts"),
      add_named_node(&mut g, 4294969627, "clang::Expr::IgnoreParenCasts"),
      add_named_node(&mut g, 4294969628, "clang::Expr::IgnoreConversionOperatorSingleStep"),
      add_named_node(&mut g, 4294969629, "clang::Expr::IgnoreParenLValueCasts"),
      add_named_node(&mut g, 4294969630, "clang::Expr::IgnoreParenBaseCasts"),
      add_named_node(&mut g, 4294969631, "clang::Expr::isDefaultArgument"),
      add_named_node(&mut g, 4294969632, "clang::Expr::isImplicitCXXThis"),
      add_named_node(&mut g, 4294969633, "clang::Expr::getBestDynamicClassType"),
      add_named_node(&mut g, 4294969634, "clang::Expr::getBestDynamicClassTypeExpr"),
      add_named_node(&mut g, 4294969635, "clang::Expr::skipRValueSubobjectAdjustments"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_EXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NULLSTMT, "clang::NullStmt");
  g.add_edge((CLASS_NULLSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NULLSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969637, "clang::NullStmt::getSemiLoc"),
      add_named_node(&mut g, 4294969638, "clang::NullStmt::hasLeadingEmptyMacro"),
      add_named_node(&mut g, 4294969639, "clang::NullStmt::getBeginLoc"),
      add_named_node(&mut g, 4294969640, "clang::NullStmt::getEndLoc"),
      add_named_node(&mut g, 4294969641, "clang::NullStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NULLSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCCLASSSTUBATTR, "clang::ObjCClassStubAttr");
  g.add_edge((CLASS_OBJCCLASSSTUBATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCCLASSSTUBATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCCLASSSTUBATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXX11NORETURNATTR, "clang::CXX11NoReturnAttr");
  g.add_edge((CLASS_CXX11NORETURNATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXX11NORETURNATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXX11NORETURNATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MSPROPERTYREFEXPR, "clang::MSPropertyRefExpr");
  g.add_edge((CLASS_MSPROPERTYREFEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MSPROPERTYREFEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969645, "clang::MSPropertyRefExpr::getSourceRange"),
      add_named_node(&mut g, 4294969646, "clang::MSPropertyRefExpr::isImplicitAccess"),
      add_named_node(&mut g, 4294969647, "clang::MSPropertyRefExpr::getBeginLoc"),
      add_named_node(&mut g, 4294969648, "clang::MSPropertyRefExpr::getEndLoc"),
      add_named_node(&mut g, 4294969649, "clang::MSPropertyRefExpr::children"),
      add_named_node(&mut g, 4294969650, "clang::MSPropertyRefExpr::getBaseExpr"),
      add_named_node(&mut g, 4294969651, "clang::MSPropertyRefExpr::getPropertyDecl"),
      add_named_node(&mut g, 4294969652, "clang::MSPropertyRefExpr::isArrow"),
      add_named_node(&mut g, 4294969653, "clang::MSPropertyRefExpr::getMemberLoc"),
      add_named_node(&mut g, 4294969654, "clang::MSPropertyRefExpr::getQualifierLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MSPROPERTYREFEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DIAGNOSEASBUILTINATTR, "clang::DiagnoseAsBuiltinAttr");
  g.add_edge((CLASS_DIAGNOSEASBUILTINATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DIAGNOSEASBUILTINATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DIAGNOSEASBUILTINATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_COROONLYDESTROYWHENCOMPLETEATTR, "clang::CoroOnlyDestroyWhenCompleteAttr");
  g.add_edge((CLASS_COROONLYDESTROYWHENCOMPLETEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_COROONLYDESTROYWHENCOMPLETEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_COROONLYDESTROYWHENCOMPLETEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PRAGMACLANGRODATASECTIONATTR, "clang::PragmaClangRodataSectionAttr");
  g.add_edge((CLASS_PRAGMACLANGRODATASECTIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PRAGMACLANGRODATASECTIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PRAGMACLANGRODATASECTIONATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CUDALAUNCHBOUNDSATTR, "clang::CUDALaunchBoundsAttr");
  g.add_edge((CLASS_CUDALAUNCHBOUNDSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CUDALAUNCHBOUNDSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CUDALAUNCHBOUNDSATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_RETAINATTR, "clang::RetainAttr");
  g.add_edge((CLASS_RETAINATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RETAINATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_RETAINATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CALLBACKATTR, "clang::CallbackAttr");
  g.add_edge((CLASS_CALLBACKATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CALLBACKATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CALLBACKATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_UUIDATTR, "clang::UuidAttr");
  g.add_edge((CLASS_UUIDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UUIDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UUIDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPEDEFTYPELOC, "clang::TypedefTypeLoc");
  g.add_edge((CLASS_TYPEDEFTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969663, "clang::TypedefTypeLoc::getTypedefNameDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPEDEFTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PREFERREDTYPEATTR, "clang::PreferredTypeAttr");
  g.add_edge((CLASS_PREFERREDTYPEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PREFERREDTYPEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PREFERREDTYPEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PTGUARDEDVARATTR, "clang::PtGuardedVarAttr");
  g.add_edge((CLASS_PTGUARDEDVARATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PTGUARDEDVARATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PTGUARDEDVARATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ASSERTEXCLUSIVELOCKATTR, "clang::AssertExclusiveLockAttr");
  g.add_edge((CLASS_ASSERTEXCLUSIVELOCKATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ASSERTEXCLUSIVELOCKATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ASSERTEXCLUSIVELOCKATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_COMMONATTR, "clang::CommonAttr");
  g.add_edge((CLASS_COMMONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_COMMONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_COMMONATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_REQDWORKGROUPSIZEATTR, "clang::ReqdWorkGroupSizeAttr");
  g.add_edge((CLASS_REQDWORKGROUPSIZEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_REQDWORKGROUPSIZEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_REQDWORKGROUPSIZEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCRUNTIMEVISIBLEATTR, "clang::ObjCRuntimeVisibleAttr");
  g.add_edge((CLASS_OBJCRUNTIMEVISIBLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCRUNTIMEVISIBLEATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCRUNTIMEVISIBLEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PASCALATTR, "clang::PascalAttr");
  g.add_edge((CLASS_PASCALATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PASCALATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PASCALATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_GCCASMSTMT, "clang::GCCAsmStmt");
  g.add_edge((CLASS_GCCASMSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_GCCASMSTMT, META_SUBCLASS, CLASS_ASMSTMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969672, "clang::GCCAsmStmt::getRParenLoc"),
      add_named_node(&mut g, 4294969673, "clang::GCCAsmStmt::getAsmString"),
      add_named_node(&mut g, 4294969674, "clang::GCCAsmStmt::isAsmGoto"),
      add_named_node(&mut g, 4294969675, "clang::GCCAsmStmt::getNumLabels"),
      add_named_node(&mut g, 4294969676, "clang::GCCAsmStmt::begin_labels"),
      add_named_node(&mut g, 4294969677, "clang::GCCAsmStmt::end_labels"),
      add_named_node(&mut g, 4294969678, "clang::GCCAsmStmt::labels"),
      add_named_node(&mut g, 4294969679, "clang::GCCAsmStmt::getBeginLoc"),
      add_named_node(&mut g, 4294969680, "clang::GCCAsmStmt::getEndLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_GCCASMSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PARAMETERABIATTR, "clang::ParameterABIAttr");
  g.add_edge((CLASS_PARAMETERABIATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PARAMETERABIATTR, META_SUBCLASS, CLASS_INHERITABLEPARAMATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PARAMETERABIATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXMEMBERCALLEXPR, "clang::CXXMemberCallExpr");
  g.add_edge((CLASS_CXXMEMBERCALLEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXMEMBERCALLEXPR, META_SUBCLASS, CLASS_CALLEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969683, "clang::CXXMemberCallExpr::getImplicitObjectArgument"),
      add_named_node(&mut g, 4294969684, "clang::CXXMemberCallExpr::getObjectType"),
      add_named_node(&mut g, 4294969685, "clang::CXXMemberCallExpr::getMethodDecl"),
      add_named_node(&mut g, 4294969686, "clang::CXXMemberCallExpr::getRecordDecl"),
      add_named_node(&mut g, 4294969687, "clang::CXXMemberCallExpr::getExprLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXMEMBERCALLEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_RETURNSTWICEATTR, "clang::ReturnsTwiceAttr");
  g.add_edge((CLASS_RETURNSTWICEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RETURNSTWICEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_RETURNSTWICEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTTYPEOFEXPRTYPE, "clang::DependentTypeOfExprType");
  g.add_edge((CLASS_DEPENDENTTYPEOFEXPRTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEPENDENTTYPEOFEXPRTYPE, META_SUBCLASS, CLASS_TYPEOFEXPRTYPE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTTYPEOFEXPRTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SEHEXCEPTSTMT, "clang::SEHExceptStmt");
  g.add_edge((CLASS_SEHEXCEPTSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SEHEXCEPTSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969691, "clang::SEHExceptStmt::getBeginLoc"),
      add_named_node(&mut g, 4294969692, "clang::SEHExceptStmt::getExceptLoc"),
      add_named_node(&mut g, 4294969693, "clang::SEHExceptStmt::getEndLoc"),
      add_named_node(&mut g, 4294969694, "clang::SEHExceptStmt::getFilterExpr"),
      add_named_node(&mut g, 4294969695, "clang::SEHExceptStmt::getBlock"),
      add_named_node(&mut g, 4294969696, "clang::SEHExceptStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SEHEXCEPTSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_AARCH64VECTORPCSATTR, "clang::AArch64VectorPcsAttr");
  g.add_edge((CLASS_AARCH64VECTORPCSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_AARCH64VECTORPCSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_AARCH64VECTORPCSATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_INITPRIORITYATTR, "clang::InitPriorityAttr");
  g.add_edge((CLASS_INITPRIORITYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_INITPRIORITYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_INITPRIORITYATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_RESTRICTATTR, "clang::RestrictAttr");
  g.add_edge((CLASS_RESTRICTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RESTRICTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_RESTRICTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_HLSLGROUPSHAREDADDRESSSPACEATTR, "clang::HLSLGroupSharedAddressSpaceAttr");
  g.add_edge((CLASS_HLSLGROUPSHAREDADDRESSSPACEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_HLSLGROUPSHAREDADDRESSSPACEATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_HLSLGROUPSHAREDADDRESSSPACEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BPFPRESERVEACCESSINDEXATTR, "clang::BPFPreserveAccessIndexAttr");
  g.add_edge((CLASS_BPFPRESERVEACCESSINDEXATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BPFPRESERVEACCESSINDEXATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BPFPRESERVEACCESSINDEXATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FUNCTIONTYPE, "clang::FunctionType");
  g.add_edge((CLASS_FUNCTIONTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FUNCTIONTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969703, "clang::FunctionType::getReturnType"),
      add_named_node(&mut g, 4294969704, "clang::FunctionType::getHasRegParm"),
      add_named_node(&mut g, 4294969705, "clang::FunctionType::getRegParmType"),
      add_named_node(&mut g, 4294969706, "clang::FunctionType::getNoReturnAttr"),
      add_named_node(&mut g, 4294969707, "clang::FunctionType::getCmseNSCallAttr"),
      add_named_node(&mut g, 4294969708, "clang::FunctionType::getCallConv"),
      add_named_node(&mut g, 4294969709, "clang::FunctionType::getExtInfo"),
      add_named_node(&mut g, 4294969710, "clang::FunctionType::isConst"),
      add_named_node(&mut g, 4294969711, "clang::FunctionType::isVolatile"),
      add_named_node(&mut g, 4294969712, "clang::FunctionType::isRestrict"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FUNCTIONTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ATOMICTYPE, "clang::AtomicType");
  g.add_edge((CLASS_ATOMICTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ATOMICTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969714, "clang::AtomicType::getValueType"),
      add_named_node(&mut g, 4294969715, "clang::AtomicType::isSugared"),
      add_named_node(&mut g, 4294969716, "clang::AtomicType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ATOMICTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SWIFTASYNCERRORATTR, "clang::SwiftAsyncErrorAttr");
  g.add_edge((CLASS_SWIFTASYNCERRORATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTASYNCERRORATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SWIFTASYNCERRORATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FLOATINGLITERAL, "clang::FloatingLiteral");
  g.add_edge((CLASS_FLOATINGLITERAL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FLOATINGLITERAL, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969719, "clang::FloatingLiteral::getValue"),
      add_named_node(&mut g, 4294969720, "clang::FloatingLiteral::getRawSemantics"),
      add_named_node(&mut g, 4294969721, "clang::FloatingLiteral::getSemantics"),
      add_named_node(&mut g, 4294969722, "clang::FloatingLiteral::isExact"),
      add_named_node(&mut g, 4294969723, "clang::FloatingLiteral::getValueAsApproximateDouble"),
      add_named_node(&mut g, 4294969724, "clang::FloatingLiteral::getLocation"),
      add_named_node(&mut g, 4294969725, "clang::FloatingLiteral::getBeginLoc"),
      add_named_node(&mut g, 4294969726, "clang::FloatingLiteral::getEndLoc"),
      add_named_node(&mut g, 4294969727, "clang::FloatingLiteral::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FLOATINGLITERAL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PACKEDATTR, "clang::PackedAttr");
  g.add_edge((CLASS_PACKEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PACKEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PACKEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_GUARDEDBYATTR, "clang::GuardedByAttr");
  g.add_edge((CLASS_GUARDEDBYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_GUARDEDBYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_GUARDEDBYATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_RETURNSNONNULLATTR, "clang::ReturnsNonNullAttr");
  g.add_edge((CLASS_RETURNSNONNULLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RETURNSNONNULLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_RETURNSNONNULLATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_AVRINTERRUPTATTR, "clang::AVRInterruptAttr");
  g.add_edge((CLASS_AVRINTERRUPTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_AVRINTERRUPTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_AVRINTERRUPTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MATRIXSUBSCRIPTEXPR, "clang::MatrixSubscriptExpr");
  g.add_edge((CLASS_MATRIXSUBSCRIPTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MATRIXSUBSCRIPTEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969733, "clang::MatrixSubscriptExpr::isIncomplete"),
      add_named_node(&mut g, 4294969734, "clang::MatrixSubscriptExpr::getBase"),
      add_named_node(&mut g, 4294969735, "clang::MatrixSubscriptExpr::getRowIdx"),
      add_named_node(&mut g, 4294969736, "clang::MatrixSubscriptExpr::getColumnIdx"),
      add_named_node(&mut g, 4294969737, "clang::MatrixSubscriptExpr::getBeginLoc"),
      add_named_node(&mut g, 4294969738, "clang::MatrixSubscriptExpr::getEndLoc"),
      add_named_node(&mut g, 4294969739, "clang::MatrixSubscriptExpr::getExprLoc"),
      add_named_node(&mut g, 4294969740, "clang::MatrixSubscriptExpr::getRBracketLoc"),
      add_named_node(&mut g, 4294969741, "clang::MatrixSubscriptExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MATRIXSUBSCRIPTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SENTINELATTR, "clang::SentinelAttr");
  g.add_edge((CLASS_SENTINELATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SENTINELATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SENTINELATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_UNSAFEBUFFERUSAGEATTR, "clang::UnsafeBufferUsageAttr");
  g.add_edge((CLASS_UNSAFEBUFFERUSAGEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNSAFEBUFFERUSAGEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNSAFEBUFFERUSAGEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FUNCTIONPROTOTYPE, "clang::FunctionProtoType");
  g.add_edge((CLASS_FUNCTIONPROTOTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FUNCTIONPROTOTYPE, META_SUBCLASS, CLASS_FUNCTIONTYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969745, "clang::FunctionProtoType::getNumParams"),
      add_named_node(&mut g, 4294969746, "clang::FunctionProtoType::getParamTypes"),
      add_named_node(&mut g, 4294969747, "clang::FunctionProtoType::getExtProtoInfo"),
      add_named_node(&mut g, 4294969748, "clang::FunctionProtoType::getExceptionSpecType"),
      add_named_node(&mut g, 4294969749, "clang::FunctionProtoType::hasExceptionSpec"),
      add_named_node(&mut g, 4294969750, "clang::FunctionProtoType::hasDynamicExceptionSpec"),
      add_named_node(&mut g, 4294969751, "clang::FunctionProtoType::hasNoexceptExceptionSpec"),
      add_named_node(&mut g, 4294969752, "clang::FunctionProtoType::hasDependentExceptionSpec"),
      add_named_node(&mut g, 4294969753, "clang::FunctionProtoType::hasInstantiationDependentExceptionSpec"),
      add_named_node(&mut g, 4294969754, "clang::FunctionProtoType::getExceptionSpecInfo"),
      add_named_node(&mut g, 4294969755, "clang::FunctionProtoType::getNumExceptions"),
      add_named_node(&mut g, 4294969756, "clang::FunctionProtoType::getNoexceptExpr"),
      add_named_node(&mut g, 4294969757, "clang::FunctionProtoType::getExceptionSpecDecl"),
      add_named_node(&mut g, 4294969758, "clang::FunctionProtoType::getExceptionSpecTemplate"),
      add_named_node(&mut g, 4294969759, "clang::FunctionProtoType::canThrow"),
      add_named_node(&mut g, 4294969760, "clang::FunctionProtoType::isVariadic"),
      add_named_node(&mut g, 4294969761, "clang::FunctionProtoType::getEllipsisLoc"),
      add_named_node(&mut g, 4294969762, "clang::FunctionProtoType::isTemplateVariadic"),
      add_named_node(&mut g, 4294969763, "clang::FunctionProtoType::hasTrailingReturn"),
      add_named_node(&mut g, 4294969764, "clang::FunctionProtoType::getMethodQuals"),
      add_named_node(&mut g, 4294969765, "clang::FunctionProtoType::getRefQualifier"),
      add_named_node(&mut g, 4294969766, "clang::FunctionProtoType::param_types"),
      add_named_node(&mut g, 4294969767, "clang::FunctionProtoType::param_type_begin"),
      add_named_node(&mut g, 4294969768, "clang::FunctionProtoType::param_type_end"),
      add_named_node(&mut g, 4294969769, "clang::FunctionProtoType::exceptions"),
      add_named_node(&mut g, 4294969770, "clang::FunctionProtoType::exception_begin"),
      add_named_node(&mut g, 4294969771, "clang::FunctionProtoType::exception_end"),
      add_named_node(&mut g, 4294969772, "clang::FunctionProtoType::hasExtParameterInfos"),
      add_named_node(&mut g, 4294969773, "clang::FunctionProtoType::getExtParameterInfos"),
      add_named_node(&mut g, 4294969774, "clang::FunctionProtoType::getExtParameterInfosOrNull"),
      add_named_node(&mut g, 4294969775, "clang::FunctionProtoType::getAArch64SMEAttributes"),
      add_named_node(&mut g, 4294969776, "clang::FunctionProtoType::isSugared"),
      add_named_node(&mut g, 4294969777, "clang::FunctionProtoType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FUNCTIONPROTOTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NOTHROWATTR, "clang::NoThrowAttr");
  g.add_edge((CLASS_NOTHROWATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOTHROWATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NOTHROWATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_UNAVAILABLEATTR, "clang::UnavailableAttr");
  g.add_edge((CLASS_UNAVAILABLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNAVAILABLEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNAVAILABLEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BUILTINTYPE, "clang::BuiltinType");
  g.add_edge((CLASS_BUILTINTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BUILTINTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969781, "clang::BuiltinType::getKind"),
      add_named_node(&mut g, 4294969782, "clang::BuiltinType::isSugared"),
      add_named_node(&mut g, 4294969783, "clang::BuiltinType::desugar"),
      add_named_node(&mut g, 4294969784, "clang::BuiltinType::isInteger"),
      add_named_node(&mut g, 4294969785, "clang::BuiltinType::isSignedInteger"),
      add_named_node(&mut g, 4294969786, "clang::BuiltinType::isUnsignedInteger"),
      add_named_node(&mut g, 4294969787, "clang::BuiltinType::isFloatingPoint"),
      add_named_node(&mut g, 4294969788, "clang::BuiltinType::isSVEBool"),
      add_named_node(&mut g, 4294969789, "clang::BuiltinType::isSVECount"),
      add_named_node(&mut g, 4294969790, "clang::BuiltinType::isPlaceholderType"),
      add_named_node(&mut g, 4294969791, "clang::BuiltinType::isNonOverloadPlaceholderType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BUILTINTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCINDEPENDENTCLASSATTR, "clang::ObjCIndependentClassAttr");
  g.add_edge((CLASS_OBJCINDEPENDENTCLASSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCINDEPENDENTCLASSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCINDEPENDENTCLASSATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NOINLINEATTR, "clang::NoInlineAttr");
  g.add_edge((CLASS_NOINLINEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOINLINEATTR, META_SUBCLASS, CLASS_DECLORSTMTATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NOINLINEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ALWAYSINLINEATTR, "clang::AlwaysInlineAttr");
  g.add_edge((CLASS_ALWAYSINLINEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ALWAYSINLINEATTR, META_SUBCLASS, CLASS_DECLORSTMTATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ALWAYSINLINEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CUDADEVICEATTR, "clang::CUDADeviceAttr");
  g.add_edge((CLASS_CUDADEVICEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CUDADEVICEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CUDADEVICEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SPECULATIVELOADHARDENINGATTR, "clang::SpeculativeLoadHardeningAttr");
  g.add_edge((CLASS_SPECULATIVELOADHARDENINGATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SPECULATIVELOADHARDENINGATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SPECULATIVELOADHARDENINGATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CFRETURNSNOTRETAINEDATTR, "clang::CFReturnsNotRetainedAttr");
  g.add_edge((CLASS_CFRETURNSNOTRETAINEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CFRETURNSNOTRETAINEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CFRETURNSNOTRETAINEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTARGETPARALLELFORSIMDDIRECTIVE, "clang::OMPTargetParallelForSimdDirective");
  g.add_edge((CLASS_OMPTARGETPARALLELFORSIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETPARALLELFORSIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTARGETPARALLELFORSIMDDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_THISCALLATTR, "clang::ThisCallAttr");
  g.add_edge((CLASS_THISCALLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_THISCALLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_THISCALLATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TEMPLATEPARAMOBJECTDECL, "clang::TemplateParamObjectDecl");
  g.add_edge((CLASS_TEMPLATEPARAMOBJECTDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TEMPLATEPARAMOBJECTDECL, META_SUBCLASS, CLASS_VALUEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969801, "clang::TemplateParamObjectDecl::getValue"),
      add_named_node(&mut g, 4294969802, "clang::TemplateParamObjectDecl::getCanonicalDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TEMPLATEPARAMOBJECTDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPEVISIBILITYATTR, "clang::TypeVisibilityAttr");
  g.add_edge((CLASS_TYPEVISIBILITYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPEVISIBILITYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPEVISIBILITYATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPENULLABLERESULTATTR, "clang::TypeNullableResultAttr");
  g.add_edge((CLASS_TYPENULLABLERESULTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPENULLABLERESULTATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPENULLABLERESULTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CUDADEVICEBUILTINSURFACETYPEATTR, "clang::CUDADeviceBuiltinSurfaceTypeAttr");
  g.add_edge((CLASS_CUDADEVICEBUILTINSURFACETYPEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CUDADEVICEBUILTINSURFACETYPEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CUDADEVICEBUILTINSURFACETYPEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MIPSINTERRUPTATTR, "clang::MipsInterruptAttr");
  g.add_edge((CLASS_MIPSINTERRUPTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MIPSINTERRUPTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MIPSINTERRUPTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTEAMSDISTRIBUTEDIRECTIVE, "clang::OMPTeamsDistributeDirective");
  g.add_edge((CLASS_OMPTEAMSDISTRIBUTEDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTEAMSDISTRIBUTEDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTEAMSDISTRIBUTEDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OWNERSHIPATTR, "clang::OwnershipAttr");
  g.add_edge((CLASS_OWNERSHIPATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OWNERSHIPATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OWNERSHIPATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NSRETURNSNOTRETAINEDATTR, "clang::NSReturnsNotRetainedAttr");
  g.add_edge((CLASS_NSRETURNSNOTRETAINEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NSRETURNSNOTRETAINEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NSRETURNSNOTRETAINEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_RELEASECAPABILITYATTR, "clang::ReleaseCapabilityAttr");
  g.add_edge((CLASS_RELEASECAPABILITYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RELEASECAPABILITYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_RELEASECAPABILITYATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OPENCLINTELREQDSUBGROUPSIZEATTR, "clang::OpenCLIntelReqdSubGroupSizeAttr");
  g.add_edge((CLASS_OPENCLINTELREQDSUBGROUPSIZEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OPENCLINTELREQDSUBGROUPSIZEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OPENCLINTELREQDSUBGROUPSIZEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PUREATTR, "clang::PureAttr");
  g.add_edge((CLASS_PUREATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PUREATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PUREATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXBOOLLITERALEXPR, "clang::CXXBoolLiteralExpr");
  g.add_edge((CLASS_CXXBOOLLITERALEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXBOOLLITERALEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969814, "clang::CXXBoolLiteralExpr::getValue"),
      add_named_node(&mut g, 4294969815, "clang::CXXBoolLiteralExpr::getBeginLoc"),
      add_named_node(&mut g, 4294969816, "clang::CXXBoolLiteralExpr::getEndLoc"),
      add_named_node(&mut g, 4294969817, "clang::CXXBoolLiteralExpr::getLocation"),
      add_named_node(&mut g, 4294969818, "clang::CXXBoolLiteralExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXBOOLLITERALEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OPTIMIZENONEATTR, "clang::OptimizeNoneAttr");
  g.add_edge((CLASS_OPTIMIZENONEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OPTIMIZENONEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OPTIMIZENONEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_LTOVISIBILITYPUBLICATTR, "clang::LTOVisibilityPublicAttr");
  g.add_edge((CLASS_LTOVISIBILITYPUBLICATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LTOVISIBILITYPUBLICATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_LTOVISIBILITYPUBLICATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCSUBCLASSINGRESTRICTEDATTR, "clang::ObjCSubclassingRestrictedAttr");
  g.add_edge((CLASS_OBJCSUBCLASSINGRESTRICTEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCSUBCLASSINGRESTRICTEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCSUBCLASSINGRESTRICTEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NORETURNATTR, "clang::NoReturnAttr");
  g.add_edge((CLASS_NORETURNATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NORETURNATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NORETURNATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ARRAYTYPE, "clang::ArrayType");
  g.add_edge((CLASS_ARRAYTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARRAYTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969824, "clang::ArrayType::getElementType"),
      add_named_node(&mut g, 4294969825, "clang::ArrayType::getSizeModifier"),
      add_named_node(&mut g, 4294969826, "clang::ArrayType::getIndexTypeQualifiers"),
      add_named_node(&mut g, 4294969827, "clang::ArrayType::getIndexTypeCVRQualifiers"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ARRAYTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NAMESPACEALIASDECL, "clang::NamespaceAliasDecl");
  g.add_edge((CLASS_NAMESPACEALIASDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NAMESPACEALIASDECL, META_SUBCLASS, CLASS_NAMEDDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969829, "clang::NamespaceAliasDecl::getCanonicalDecl"),
      add_named_node(&mut g, 4294969830, "clang::NamespaceAliasDecl::getQualifierLoc"),
      add_named_node(&mut g, 4294969831, "clang::NamespaceAliasDecl::getQualifier"),
      add_named_node(&mut g, 4294969832, "clang::NamespaceAliasDecl::getNamespace"),
      add_named_node(&mut g, 4294969833, "clang::NamespaceAliasDecl::getAliasLoc"),
      add_named_node(&mut g, 4294969834, "clang::NamespaceAliasDecl::getNamespaceLoc"),
      add_named_node(&mut g, 4294969835, "clang::NamespaceAliasDecl::getTargetNameLoc"),
      add_named_node(&mut g, 4294969836, "clang::NamespaceAliasDecl::getAliasedNamespace"),
      add_named_node(&mut g, 4294969837, "clang::NamespaceAliasDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NAMESPACEALIASDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_LINKAGESPECDECL, "clang::LinkageSpecDecl");
  g.add_edge((CLASS_LINKAGESPECDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LINKAGESPECDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969839, "clang::LinkageSpecDecl::getLanguage"),
      add_named_node(&mut g, 4294969840, "clang::LinkageSpecDecl::hasBraces"),
      add_named_node(&mut g, 4294969841, "clang::LinkageSpecDecl::getExternLoc"),
      add_named_node(&mut g, 4294969842, "clang::LinkageSpecDecl::getRBraceLoc"),
      add_named_node(&mut g, 4294969843, "clang::LinkageSpecDecl::getEndLoc"),
      add_named_node(&mut g, 4294969844, "clang::LinkageSpecDecl::getSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_LINKAGESPECDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCNONRUNTIMEPROTOCOLATTR, "clang::ObjCNonRuntimeProtocolAttr");
  g.add_edge((CLASS_OBJCNONRUNTIMEPROTOCOLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCNONRUNTIMEPROTOCOLATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCNONRUNTIMEPROTOCOLATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FINALATTR, "clang::FinalAttr");
  g.add_edge((CLASS_FINALATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FINALATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FINALATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCRETURNSINNERPOINTERATTR, "clang::ObjCReturnsInnerPointerAttr");
  g.add_edge((CLASS_OBJCRETURNSINNERPOINTERATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCRETURNSINNERPOINTERATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCRETURNSINNERPOINTERATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CPUDISPATCHATTR, "clang::CPUDispatchAttr");
  g.add_edge((CLASS_CPUDISPATCHATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CPUDISPATCHATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CPUDISPATCHATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PTGUARDEDBYATTR, "clang::PtGuardedByAttr");
  g.add_edge((CLASS_PTGUARDEDBYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PTGUARDEDBYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PTGUARDEDBYATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CFUNKNOWNTRANSFERATTR, "clang::CFUnknownTransferAttr");
  g.add_edge((CLASS_CFUNKNOWNTRANSFERATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CFUNKNOWNTRANSFERATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CFUNKNOWNTRANSFERATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PRESERVEMOSTATTR, "clang::PreserveMostAttr");
  g.add_edge((CLASS_PRESERVEMOSTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PRESERVEMOSTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PRESERVEMOSTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_EXTVECTORTYPELOC, "clang::ExtVectorTypeLoc");
  g.add_edge((CLASS_EXTVECTORTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_EXTVECTORTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCREQUIRESPROPERTYDEFSATTR, "clang::ObjCRequiresPropertyDefsAttr");
  g.add_edge((CLASS_OBJCREQUIRESPROPERTYDEFSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCREQUIRESPROPERTYDEFSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCREQUIRESPROPERTYDEFSATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_INCOMPLETEARRAYTYPE, "clang::IncompleteArrayType");
  g.add_edge((CLASS_INCOMPLETEARRAYTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_INCOMPLETEARRAYTYPE, META_SUBCLASS, CLASS_ARRAYTYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969855, "clang::IncompleteArrayType::isSugared"),
      add_named_node(&mut g, 4294969856, "clang::IncompleteArrayType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_INCOMPLETEARRAYTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_AMDGPUNUMVGPRATTR, "clang::AMDGPUNumVGPRAttr");
  g.add_edge((CLASS_AMDGPUNUMVGPRATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_AMDGPUNUMVGPRATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_AMDGPUNUMVGPRATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PTR32ATTR, "clang::Ptr32Attr");
  g.add_edge((CLASS_PTR32ATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PTR32ATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PTR32ATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_HLSLRESOURCEATTR, "clang::HLSLResourceAttr");
  g.add_edge((CLASS_HLSLRESOURCEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_HLSLRESOURCEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_HLSLRESOURCEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCOWNERSHIPATTR, "clang::ObjCOwnershipAttr");
  g.add_edge((CLASS_OBJCOWNERSHIPATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCOWNERSHIPATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCOWNERSHIPATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXFUNCTIONALCASTEXPR, "clang::CXXFunctionalCastExpr");
  g.add_edge((CLASS_CXXFUNCTIONALCASTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXFUNCTIONALCASTEXPR, META_SUBCLASS, CLASS_EXPLICITCASTEXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969862, "clang::CXXFunctionalCastExpr::getLParenLoc"),
      add_named_node(&mut g, 4294969863, "clang::CXXFunctionalCastExpr::getRParenLoc"),
      add_named_node(&mut g, 4294969864, "clang::CXXFunctionalCastExpr::isListInitialization"),
      add_named_node(&mut g, 4294969865, "clang::CXXFunctionalCastExpr::getBeginLoc"),
      add_named_node(&mut g, 4294969866, "clang::CXXFunctionalCastExpr::getEndLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXFUNCTIONALCASTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OPENCLPRIVATEADDRESSSPACEATTR, "clang::OpenCLPrivateAddressSpaceAttr");
  g.add_edge((CLASS_OPENCLPRIVATEADDRESSSPACEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OPENCLPRIVATEADDRESSSPACEATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OPENCLPRIVATEADDRESSSPACEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_EXCLUSIVETRYLOCKFUNCTIONATTR, "clang::ExclusiveTrylockFunctionAttr");
  g.add_edge((CLASS_EXCLUSIVETRYLOCKFUNCTIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_EXCLUSIVETRYLOCKFUNCTIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_EXCLUSIVETRYLOCKFUNCTIONATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NOBUILTINATTR, "clang::NoBuiltinAttr");
  g.add_edge((CLASS_NOBUILTINATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOBUILTINATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NOBUILTINATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FIXEDPOINTLITERAL, "clang::FixedPointLiteral");
  g.add_edge((CLASS_FIXEDPOINTLITERAL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FIXEDPOINTLITERAL, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969871, "clang::FixedPointLiteral::getBeginLoc"),
      add_named_node(&mut g, 4294969872, "clang::FixedPointLiteral::getEndLoc"),
      add_named_node(&mut g, 4294969873, "clang::FixedPointLiteral::getLocation"),
      add_named_node(&mut g, 4294969874, "clang::FixedPointLiteral::getScale"),
      add_named_node(&mut g, 4294969875, "clang::FixedPointLiteral::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FIXEDPOINTLITERAL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SWIFTPRIVATEATTR, "clang::SwiftPrivateAttr");
  g.add_edge((CLASS_SWIFTPRIVATEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTPRIVATEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SWIFTPRIVATEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCBRIDGERELATEDATTR, "clang::ObjCBridgeRelatedAttr");
  g.add_edge((CLASS_OBJCBRIDGERELATEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCBRIDGERELATEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCBRIDGERELATEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCEXTERNALLYRETAINEDATTR, "clang::ObjCExternallyRetainedAttr");
  g.add_edge((CLASS_OBJCEXTERNALLYRETAINEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCEXTERNALLYRETAINEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCEXTERNALLYRETAINEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_LAMBDAEXPR, "clang::LambdaExpr");
  g.add_edge((CLASS_LAMBDAEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LAMBDAEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969880, "clang::LambdaExpr::getCaptureDefault"),
      add_named_node(&mut g, 4294969881, "clang::LambdaExpr::getCaptureDefaultLoc"),
      add_named_node(&mut g, 4294969882, "clang::LambdaExpr::captures"),
      add_named_node(&mut g, 4294969883, "clang::LambdaExpr::capture_begin"),
      add_named_node(&mut g, 4294969884, "clang::LambdaExpr::capture_end"),
      add_named_node(&mut g, 4294969885, "clang::LambdaExpr::capture_size"),
      add_named_node(&mut g, 4294969886, "clang::LambdaExpr::explicit_captures"),
      add_named_node(&mut g, 4294969887, "clang::LambdaExpr::explicit_capture_begin"),
      add_named_node(&mut g, 4294969888, "clang::LambdaExpr::explicit_capture_end"),
      add_named_node(&mut g, 4294969889, "clang::LambdaExpr::implicit_captures"),
      add_named_node(&mut g, 4294969890, "clang::LambdaExpr::implicit_capture_begin"),
      add_named_node(&mut g, 4294969891, "clang::LambdaExpr::implicit_capture_end"),
      add_named_node(&mut g, 4294969892, "clang::LambdaExpr::capture_inits"),
      add_named_node(&mut g, 4294969893, "clang::LambdaExpr::capture_init_begin"),
      add_named_node(&mut g, 4294969894, "clang::LambdaExpr::capture_init_end"),
      add_named_node(&mut g, 4294969895, "clang::LambdaExpr::getIntroducerRange"),
      add_named_node(&mut g, 4294969896, "clang::LambdaExpr::getLambdaClass"),
      add_named_node(&mut g, 4294969897, "clang::LambdaExpr::getCallOperator"),
      add_named_node(&mut g, 4294969898, "clang::LambdaExpr::getDependentCallOperator"),
      add_named_node(&mut g, 4294969899, "clang::LambdaExpr::getTemplateParameterList"),
      add_named_node(&mut g, 4294969900, "clang::LambdaExpr::getExplicitTemplateParameters"),
      add_named_node(&mut g, 4294969901, "clang::LambdaExpr::getTrailingRequiresClause"),
      add_named_node(&mut g, 4294969902, "clang::LambdaExpr::isGenericLambda"),
      add_named_node(&mut g, 4294969903, "clang::LambdaExpr::getBody"),
      add_named_node(&mut g, 4294969904, "clang::LambdaExpr::getCompoundStmtBody"),
      add_named_node(&mut g, 4294969905, "clang::LambdaExpr::isMutable"),
      add_named_node(&mut g, 4294969906, "clang::LambdaExpr::hasExplicitParameters"),
      add_named_node(&mut g, 4294969907, "clang::LambdaExpr::hasExplicitResultType"),
      add_named_node(&mut g, 4294969908, "clang::LambdaExpr::getBeginLoc"),
      add_named_node(&mut g, 4294969909, "clang::LambdaExpr::getEndLoc"),
      add_named_node(&mut g, 4294969910, "clang::LambdaExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_LAMBDAEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MIPSLONGCALLATTR, "clang::MipsLongCallAttr");
  g.add_edge((CLASS_MIPSLONGCALLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MIPSLONGCALLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MIPSLONGCALLATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCBRIDGEMUTABLEATTR, "clang::ObjCBridgeMutableAttr");
  g.add_edge((CLASS_OBJCBRIDGEMUTABLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCBRIDGEMUTABLEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCBRIDGEMUTABLEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTBITINTTYPELOC, "clang::DependentBitIntTypeLoc");
  g.add_edge((CLASS_DEPENDENTBITINTTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTBITINTTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ARMINOUTATTR, "clang::ArmInOutAttr");
  g.add_edge((CLASS_ARMINOUTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARMINOUTATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ARMINOUTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SUBSTNONTYPETEMPLATEPARMPACKEXPR, "clang::SubstNonTypeTemplateParmPackExpr");
  g.add_edge((CLASS_SUBSTNONTYPETEMPLATEPARMPACKEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SUBSTNONTYPETEMPLATEPARMPACKEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969916, "clang::SubstNonTypeTemplateParmPackExpr::getAssociatedDecl"),
      add_named_node(&mut g, 4294969917, "clang::SubstNonTypeTemplateParmPackExpr::getIndex"),
      add_named_node(&mut g, 4294969918, "clang::SubstNonTypeTemplateParmPackExpr::getParameterPack"),
      add_named_node(&mut g, 4294969919, "clang::SubstNonTypeTemplateParmPackExpr::getParameterPackLocation"),
      add_named_node(&mut g, 4294969920, "clang::SubstNonTypeTemplateParmPackExpr::getArgumentPack"),
      add_named_node(&mut g, 4294969921, "clang::SubstNonTypeTemplateParmPackExpr::getBeginLoc"),
      add_named_node(&mut g, 4294969922, "clang::SubstNonTypeTemplateParmPackExpr::getEndLoc"),
      add_named_node(&mut g, 4294969923, "clang::SubstNonTypeTemplateParmPackExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SUBSTNONTYPETEMPLATEPARMPACKEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXPSEUDODESTRUCTOREXPR, "clang::CXXPseudoDestructorExpr");
  g.add_edge((CLASS_CXXPSEUDODESTRUCTOREXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXPSEUDODESTRUCTOREXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969925, "clang::CXXPseudoDestructorExpr::getBase"),
      add_named_node(&mut g, 4294969926, "clang::CXXPseudoDestructorExpr::hasQualifier"),
      add_named_node(&mut g, 4294969927, "clang::CXXPseudoDestructorExpr::getQualifierLoc"),
      add_named_node(&mut g, 4294969928, "clang::CXXPseudoDestructorExpr::getQualifier"),
      add_named_node(&mut g, 4294969929, "clang::CXXPseudoDestructorExpr::isArrow"),
      add_named_node(&mut g, 4294969930, "clang::CXXPseudoDestructorExpr::getOperatorLoc"),
      add_named_node(&mut g, 4294969931, "clang::CXXPseudoDestructorExpr::getScopeTypeInfo"),
      add_named_node(&mut g, 4294969932, "clang::CXXPseudoDestructorExpr::getColonColonLoc"),
      add_named_node(&mut g, 4294969933, "clang::CXXPseudoDestructorExpr::getTildeLoc"),
      add_named_node(&mut g, 4294969934, "clang::CXXPseudoDestructorExpr::getDestroyedTypeInfo"),
      add_named_node(&mut g, 4294969935, "clang::CXXPseudoDestructorExpr::getDestroyedTypeIdentifier"),
      add_named_node(&mut g, 4294969936, "clang::CXXPseudoDestructorExpr::getDestroyedType"),
      add_named_node(&mut g, 4294969937, "clang::CXXPseudoDestructorExpr::getDestroyedTypeLoc"),
      add_named_node(&mut g, 4294969938, "clang::CXXPseudoDestructorExpr::getBeginLoc"),
      add_named_node(&mut g, 4294969939, "clang::CXXPseudoDestructorExpr::getEndLoc"),
      add_named_node(&mut g, 4294969940, "clang::CXXPseudoDestructorExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXPSEUDODESTRUCTOREXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OSRETURNSRETAINEDONZEROATTR, "clang::OSReturnsRetainedOnZeroAttr");
  g.add_edge((CLASS_OSRETURNSRETAINEDONZEROATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OSRETURNSRETAINEDONZEROATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OSRETURNSRETAINEDONZEROATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OSRETURNSRETAINEDONNONZEROATTR, "clang::OSReturnsRetainedOnNonZeroAttr");
  g.add_edge((CLASS_OSRETURNSRETAINEDONNONZEROATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OSRETURNSRETAINEDONNONZEROATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OSRETURNSRETAINEDONNONZEROATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SPTRATTR, "clang::SPtrAttr");
  g.add_edge((CLASS_SPTRATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SPTRATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SPTRATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCBRIDGEDCASTEXPR, "clang::ObjCBridgedCastExpr");
  g.add_edge((CLASS_OBJCBRIDGEDCASTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCBRIDGEDCASTEXPR, META_SUBCLASS, CLASS_EXPLICITCASTEXPR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCBRIDGEDCASTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PRESERVEALLATTR, "clang::PreserveAllAttr");
  g.add_edge((CLASS_PRESERVEALLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PRESERVEALLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PRESERVEALLATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCOBJECTTYPELOC, "clang::ObjCObjectTypeLoc");
  g.add_edge((CLASS_OBJCOBJECTTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCOBJECTTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPDECLAREVARIANTATTR, "clang::OMPDeclareVariantAttr");
  g.add_edge((CLASS_OMPDECLAREVARIANTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPDECLAREVARIANTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPDECLAREVARIANTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCEXPLICITPROTOCOLIMPLATTR, "clang::ObjCExplicitProtocolImplAttr");
  g.add_edge((CLASS_OBJCEXPLICITPROTOCOLIMPLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCEXPLICITPROTOCOLIMPLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCEXPLICITPROTOCOLIMPLATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPALLOCATEDECLATTR, "clang::OMPAllocateDeclAttr");
  g.add_edge((CLASS_OMPALLOCATEDECLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPALLOCATEDECLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPALLOCATEDECLATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SWIFTINDIRECTRESULTATTR, "clang::SwiftIndirectResultAttr");
  g.add_edge((CLASS_SWIFTINDIRECTRESULTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTINDIRECTRESULTATTR, META_SUBCLASS, CLASS_PARAMETERABIATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SWIFTINDIRECTRESULTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_COMPLEXTYPELOC, "clang::ComplexTypeLoc");
  g.add_edge((CLASS_COMPLEXTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_COMPLEXTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MACROQUALIFIEDTYPE, "clang::MacroQualifiedType");
  g.add_edge((CLASS_MACROQUALIFIEDTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MACROQUALIFIEDTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969953, "clang::MacroQualifiedType::getMacroIdentifier"),
      add_named_node(&mut g, 4294969954, "clang::MacroQualifiedType::getUnderlyingType"),
      add_named_node(&mut g, 4294969955, "clang::MacroQualifiedType::getModifiedType"),
      add_named_node(&mut g, 4294969956, "clang::MacroQualifiedType::isSugared"),
      add_named_node(&mut g, 4294969957, "clang::MacroQualifiedType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MACROQUALIFIEDTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MICROMIPSATTR, "clang::MicroMipsAttr");
  g.add_edge((CLASS_MICROMIPSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MICROMIPSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MICROMIPSATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_USINGTYPE, "clang::UsingType");
  g.add_edge((CLASS_USINGTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_USINGTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969960, "clang::UsingType::getFoundDecl"),
      add_named_node(&mut g, 4294969961, "clang::UsingType::getUnderlyingType"),
      add_named_node(&mut g, 4294969962, "clang::UsingType::isSugared"),
      add_named_node(&mut g, 4294969963, "clang::UsingType::desugar"),
      add_named_node(&mut g, 4294969964, "clang::UsingType::typeMatchesDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_USINGTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_HIPMANAGEDATTR, "clang::HIPManagedAttr");
  g.add_edge((CLASS_HIPMANAGEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_HIPMANAGEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_HIPMANAGEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MACROQUALIFIEDTYPELOC, "clang::MacroQualifiedTypeLoc");
  g.add_edge((CLASS_MACROQUALIFIEDTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969967, "clang::MacroQualifiedTypeLoc::getInnerLoc"),
      add_named_node(&mut g, 4294969968, "clang::MacroQualifiedTypeLoc::getMacroIdentifier"),
      add_named_node(&mut g, 4294969969, "clang::MacroQualifiedTypeLoc::getExpansionLoc"),
      add_named_node(&mut g, 4294969970, "clang::MacroQualifiedTypeLoc::getInnerType"),
      add_named_node(&mut g, 4294969971, "clang::MacroQualifiedTypeLoc::getLocalSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MACROQUALIFIEDTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ZEROCALLUSEDREGSATTR, "clang::ZeroCallUsedRegsAttr");
  g.add_edge((CLASS_ZEROCALLUSEDREGSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ZEROCALLUSEDREGSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ZEROCALLUSEDREGSATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCEXCEPTIONATTR, "clang::ObjCExceptionAttr");
  g.add_edge((CLASS_OBJCEXCEPTIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCEXCEPTIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCEXCEPTIONATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MAYBEUNDEFATTR, "clang::MaybeUndefAttr");
  g.add_edge((CLASS_MAYBEUNDEFATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MAYBEUNDEFATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MAYBEUNDEFATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPCANCELLATIONPOINTDIRECTIVE, "clang::OMPCancellationPointDirective");
  g.add_edge((CLASS_OMPCANCELLATIONPOINTDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPCANCELLATIONPOINTDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPCANCELLATIONPOINTDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TARGETVERSIONATTR, "clang::TargetVersionAttr");
  g.add_edge((CLASS_TARGETVERSIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TARGETVERSIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TARGETVERSIONATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NOUWTABLEATTR, "clang::NoUwtableAttr");
  g.add_edge((CLASS_NOUWTABLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOUWTABLEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NOUWTABLEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NOUNIQUEADDRESSATTR, "clang::NoUniqueAddressAttr");
  g.add_edge((CLASS_NOUNIQUEADDRESSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOUNIQUEADDRESSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NOUNIQUEADDRESSATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_VAARGEXPR, "clang::VAArgExpr");
  g.add_edge((CLASS_VAARGEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_VAARGEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969980, "clang::VAArgExpr::getSubExpr"),
      add_named_node(&mut g, 4294969981, "clang::VAArgExpr::isMicrosoftABI"),
      add_named_node(&mut g, 4294969982, "clang::VAArgExpr::getWrittenTypeInfo"),
      add_named_node(&mut g, 4294969983, "clang::VAArgExpr::getBuiltinLoc"),
      add_named_node(&mut g, 4294969984, "clang::VAArgExpr::getRParenLoc"),
      add_named_node(&mut g, 4294969985, "clang::VAArgExpr::getBeginLoc"),
      add_named_node(&mut g, 4294969986, "clang::VAArgExpr::getEndLoc"),
      add_named_node(&mut g, 4294969987, "clang::VAArgExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_VAARGEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NOTHREADSAFETYANALYSISATTR, "clang::NoThreadSafetyAnalysisAttr");
  g.add_edge((CLASS_NOTHREADSAFETYANALYSISATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOTHREADSAFETYANALYSISATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NOTHREADSAFETYANALYSISATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NOSPLITSTACKATTR, "clang::NoSplitStackAttr");
  g.add_edge((CLASS_NOSPLITSTACKATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOSPLITSTACKATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NOSPLITSTACKATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ADDRESSSPACEATTR, "clang::AddressSpaceAttr");
  g.add_edge((CLASS_ADDRESSSPACEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ADDRESSSPACEATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ADDRESSSPACEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCAVAILABILITYCHECKEXPR, "clang::ObjCAvailabilityCheckExpr");
  g.add_edge((CLASS_OBJCAVAILABILITYCHECKEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCAVAILABILITYCHECKEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCAVAILABILITYCHECKEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MSNOVTABLEATTR, "clang::MSNoVTableAttr");
  g.add_edge((CLASS_MSNOVTABLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MSNOVTABLEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MSNOVTABLEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NSCONSUMESSELFATTR, "clang::NSConsumesSelfAttr");
  g.add_edge((CLASS_NSCONSUMESSELFATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NSCONSUMESSELFATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NSCONSUMESSELFATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FUNCTIONRETURNTHUNKSATTR, "clang::FunctionReturnThunksAttr");
  g.add_edge((CLASS_FUNCTIONRETURNTHUNKSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FUNCTIONRETURNTHUNKSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FUNCTIONRETURNTHUNKSATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PRAGMACOMMENTDECL, "clang::PragmaCommentDecl");
  g.add_edge((CLASS_PRAGMACOMMENTDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PRAGMACOMMENTDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294969996, "clang::PragmaCommentDecl::getCommentKind"),
      add_named_node(&mut g, 4294969997, "clang::PragmaCommentDecl::getArg"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PRAGMACOMMENTDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_INTELOCLBICCATTR, "clang::IntelOclBiccAttr");
  g.add_edge((CLASS_INTELOCLBICCATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_INTELOCLBICCATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_INTELOCLBICCATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NOSPECULATIVELOADHARDENINGATTR, "clang::NoSpeculativeLoadHardeningAttr");
  g.add_edge((CLASS_NOSPECULATIVELOADHARDENINGATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOSPECULATIVELOADHARDENINGATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NOSPECULATIVELOADHARDENINGATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BLOCKEXPR, "clang::BlockExpr");
  g.add_edge((CLASS_BLOCKEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BLOCKEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970001, "clang::BlockExpr::getBlockDecl"),
      add_named_node(&mut g, 4294970002, "clang::BlockExpr::getCaretLocation"),
      add_named_node(&mut g, 4294970003, "clang::BlockExpr::getBody"),
      add_named_node(&mut g, 4294970004, "clang::BlockExpr::getBeginLoc"),
      add_named_node(&mut g, 4294970005, "clang::BlockExpr::getEndLoc"),
      add_named_node(&mut g, 4294970006, "clang::BlockExpr::getFunctionType"),
      add_named_node(&mut g, 4294970007, "clang::BlockExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BLOCKEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SWIFTNEWTYPEATTR, "clang::SwiftNewTypeAttr");
  g.add_edge((CLASS_SWIFTNEWTYPEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTNEWTYPEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SWIFTNEWTYPEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_AVAILABILITYATTR, "clang::AvailabilityAttr");
  g.add_edge((CLASS_AVAILABILITYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_AVAILABILITYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_AVAILABILITYATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCDIRECTATTR, "clang::ObjCDirectAttr");
  g.add_edge((CLASS_OBJCDIRECTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCDIRECTATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCDIRECTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_IBOUTLETATTR, "clang::IBOutletAttr");
  g.add_edge((CLASS_IBOUTLETATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_IBOUTLETATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_IBOUTLETATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DLLIMPORTATTR, "clang::DLLImportAttr");
  g.add_edge((CLASS_DLLIMPORTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DLLIMPORTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DLLIMPORTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MODEATTR, "clang::ModeAttr");
  g.add_edge((CLASS_MODEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MODEATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MODEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_UNNAMEDGLOBALCONSTANTDECL, "clang::UnnamedGlobalConstantDecl");
  g.add_edge((CLASS_UNNAMEDGLOBALCONSTANTDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNNAMEDGLOBALCONSTANTDECL, META_SUBCLASS, CLASS_VALUEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970015, "clang::UnnamedGlobalConstantDecl::getValue"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNNAMEDGLOBALCONSTANTDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FLATTENATTR, "clang::FlattenAttr");
  g.add_edge((CLASS_FLATTENATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FLATTENATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FLATTENATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NOSANITIZEATTR, "clang::NoSanitizeAttr");
  g.add_edge((CLASS_NOSANITIZEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOSANITIZEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NOSANITIZEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCREQUIRESSUPERATTR, "clang::ObjCRequiresSuperAttr");
  g.add_edge((CLASS_OBJCREQUIRESSUPERATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCREQUIRESSUPERATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCREQUIRESSUPERATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PIPETYPE, "clang::PipeType");
  g.add_edge((CLASS_PIPETYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PIPETYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970020, "clang::PipeType::getElementType"),
      add_named_node(&mut g, 4294970021, "clang::PipeType::isSugared"),
      add_named_node(&mut g, 4294970022, "clang::PipeType::desugar"),
      add_named_node(&mut g, 4294970023, "clang::PipeType::isReadOnly"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PIPETYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SYSVABIATTR, "clang::SysVABIAttr");
  g.add_edge((CLASS_SYSVABIATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SYSVABIATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SYSVABIATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NOMIPS16ATTR, "clang::NoMips16Attr");
  g.add_edge((CLASS_NOMIPS16ATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOMIPS16ATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NOMIPS16ATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NODESTROYATTR, "clang::NoDestroyAttr");
  g.add_edge((CLASS_NODESTROYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NODESTROYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NODESTROYATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPCANCELDIRECTIVE, "clang::OMPCancelDirective");
  g.add_edge((CLASS_OMPCANCELDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPCANCELDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPCANCELDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_STMT, "clang::Stmt");
  g.add_edge((CLASS_STMT, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970029, "clang::Stmt::getStmtClass"),
      add_named_node(&mut g, 4294970030, "clang::Stmt::getStmtClassName"),
      add_named_node(&mut g, 4294970031, "clang::Stmt::getSourceRange"),
      add_named_node(&mut g, 4294970032, "clang::Stmt::getBeginLoc"),
      add_named_node(&mut g, 4294970033, "clang::Stmt::getEndLoc"),
      add_named_node(&mut g, 4294970034, "clang::Stmt::stripLabelLikeStatements"),
      add_named_node(&mut g, 4294970035, "clang::Stmt::children"),
      add_named_node(&mut g, 4294970036, "clang::Stmt::child_begin"),
      add_named_node(&mut g, 4294970037, "clang::Stmt::child_end"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_STMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTSIZEDARRAYTYPELOC, "clang::DependentSizedArrayTypeLoc");
  g.add_edge((CLASS_DEPENDENTSIZEDARRAYTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTSIZEDARRAYTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCATSYNCHRONIZEDSTMT, "clang::ObjCAtSynchronizedStmt");
  g.add_edge((CLASS_OBJCATSYNCHRONIZEDSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCATSYNCHRONIZEDSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCATSYNCHRONIZEDSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ATOMICEXPR, "clang::AtomicExpr");
  g.add_edge((CLASS_ATOMICEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ATOMICEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970041, "clang::AtomicExpr::getPtr"),
      add_named_node(&mut g, 4294970042, "clang::AtomicExpr::getOrder"),
      add_named_node(&mut g, 4294970043, "clang::AtomicExpr::getScope"),
      add_named_node(&mut g, 4294970044, "clang::AtomicExpr::getVal1"),
      add_named_node(&mut g, 4294970045, "clang::AtomicExpr::getOrderFail"),
      add_named_node(&mut g, 4294970046, "clang::AtomicExpr::getVal2"),
      add_named_node(&mut g, 4294970047, "clang::AtomicExpr::getWeak"),
      add_named_node(&mut g, 4294970048, "clang::AtomicExpr::getValueType"),
      add_named_node(&mut g, 4294970049, "clang::AtomicExpr::getOp"),
      add_named_node(&mut g, 4294970050, "clang::AtomicExpr::getOpAsString"),
      add_named_node(&mut g, 4294970051, "clang::AtomicExpr::getNumSubExprs"),
      add_named_node(&mut g, 4294970052, "clang::AtomicExpr::getSubExprs"),
      add_named_node(&mut g, 4294970053, "clang::AtomicExpr::isVolatile"),
      add_named_node(&mut g, 4294970054, "clang::AtomicExpr::isCmpXChg"),
      add_named_node(&mut g, 4294970055, "clang::AtomicExpr::isOpenCL"),
      add_named_node(&mut g, 4294970056, "clang::AtomicExpr::getBuiltinLoc"),
      add_named_node(&mut g, 4294970057, "clang::AtomicExpr::getRParenLoc"),
      add_named_node(&mut g, 4294970058, "clang::AtomicExpr::getBeginLoc"),
      add_named_node(&mut g, 4294970059, "clang::AtomicExpr::getEndLoc"),
      add_named_node(&mut g, 4294970060, "clang::AtomicExpr::children"),
      add_named_node(&mut g, 4294970061, "clang::AtomicExpr::getScopeModel"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ATOMICEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SWIFTOBJCMEMBERSATTR, "clang::SwiftObjCMembersAttr");
  g.add_edge((CLASS_SWIFTOBJCMEMBERSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTOBJCMEMBERSATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SWIFTOBJCMEMBERSATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NOALIASATTR, "clang::NoAliasAttr");
  g.add_edge((CLASS_NOALIASATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOALIASATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NOALIASATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ATTRIBUTEDTYPELOC, "clang::AttributedTypeLoc");
  g.add_edge((CLASS_ATTRIBUTEDTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970065, "clang::AttributedTypeLoc::getAttrKind"),
      add_named_node(&mut g, 4294970066, "clang::AttributedTypeLoc::isQualifier"),
      add_named_node(&mut g, 4294970067, "clang::AttributedTypeLoc::getModifiedLoc"),
      add_named_node(&mut g, 4294970068, "clang::AttributedTypeLoc::getAttr"),
      add_named_node(&mut g, 4294970069, "clang::AttributedTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, 4294970070, "clang::AttributedTypeLoc::getInnerType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ATTRIBUTEDTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PCSATTR, "clang::PcsAttr");
  g.add_edge((CLASS_PCSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PCSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PCSATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NSRETURNSRETAINEDATTR, "clang::NSReturnsRetainedAttr");
  g.add_edge((CLASS_NSRETURNSRETAINEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NSRETURNSRETAINEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NSRETURNSRETAINEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_LOCKRETURNEDATTR, "clang::LockReturnedAttr");
  g.add_edge((CLASS_LOCKRETURNEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LOCKRETURNEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_LOCKRETURNEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_HLSLSHADERATTR, "clang::HLSLShaderAttr");
  g.add_edge((CLASS_HLSLSHADERATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_HLSLSHADERATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_HLSLSHADERATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DLLEXPORTATTR, "clang::DLLExportAttr");
  g.add_edge((CLASS_DLLEXPORTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DLLEXPORTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DLLEXPORTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTSCOPEDECLREFEXPR, "clang::DependentScopeDeclRefExpr");
  g.add_edge((CLASS_DEPENDENTSCOPEDECLREFEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEPENDENTSCOPEDECLREFEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970077, "clang::DependentScopeDeclRefExpr::getNameInfo"),
      add_named_node(&mut g, 4294970078, "clang::DependentScopeDeclRefExpr::getDeclName"),
      add_named_node(&mut g, 4294970079, "clang::DependentScopeDeclRefExpr::getLocation"),
      add_named_node(&mut g, 4294970080, "clang::DependentScopeDeclRefExpr::getQualifierLoc"),
      add_named_node(&mut g, 4294970081, "clang::DependentScopeDeclRefExpr::getQualifier"),
      add_named_node(&mut g, 4294970082, "clang::DependentScopeDeclRefExpr::getTemplateKeywordLoc"),
      add_named_node(&mut g, 4294970083, "clang::DependentScopeDeclRefExpr::getLAngleLoc"),
      add_named_node(&mut g, 4294970084, "clang::DependentScopeDeclRefExpr::getRAngleLoc"),
      add_named_node(&mut g, 4294970085, "clang::DependentScopeDeclRefExpr::hasTemplateKeyword"),
      add_named_node(&mut g, 4294970086, "clang::DependentScopeDeclRefExpr::hasExplicitTemplateArgs"),
      add_named_node(&mut g, 4294970087, "clang::DependentScopeDeclRefExpr::getTemplateArgs"),
      add_named_node(&mut g, 4294970088, "clang::DependentScopeDeclRefExpr::getNumTemplateArgs"),
      add_named_node(&mut g, 4294970089, "clang::DependentScopeDeclRefExpr::template_arguments"),
      add_named_node(&mut g, 4294970090, "clang::DependentScopeDeclRefExpr::getBeginLoc"),
      add_named_node(&mut g, 4294970091, "clang::DependentScopeDeclRefExpr::getEndLoc"),
      add_named_node(&mut g, 4294970092, "clang::DependentScopeDeclRefExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTSCOPEDECLREFEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MATRIXTYPELOC, "clang::MatrixTypeLoc");
  g.add_edge((CLASS_MATRIXTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970094, "clang::MatrixTypeLoc::getAttrNameLoc"),
      add_named_node(&mut g, 4294970095, "clang::MatrixTypeLoc::getAttrRowOperand"),
      add_named_node(&mut g, 4294970096, "clang::MatrixTypeLoc::getAttrColumnOperand"),
      add_named_node(&mut g, 4294970097, "clang::MatrixTypeLoc::getAttrOperandParensRange"),
      add_named_node(&mut g, 4294970098, "clang::MatrixTypeLoc::getLocalSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MATRIXTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MINSIZEATTR, "clang::MinSizeAttr");
  g.add_edge((CLASS_MINSIZEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MINSIZEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MINSIZEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_INDIRECTGOTOSTMT, "clang::IndirectGotoStmt");
  g.add_edge((CLASS_INDIRECTGOTOSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_INDIRECTGOTOSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970101, "clang::IndirectGotoStmt::getGotoLoc"),
      add_named_node(&mut g, 4294970102, "clang::IndirectGotoStmt::getStarLoc"),
      add_named_node(&mut g, 4294970103, "clang::IndirectGotoStmt::getTarget"),
      add_named_node(&mut g, 4294970104, "clang::IndirectGotoStmt::getConstantTarget"),
      add_named_node(&mut g, 4294970105, "clang::IndirectGotoStmt::getBeginLoc"),
      add_named_node(&mut g, 4294970106, "clang::IndirectGotoStmt::getEndLoc"),
      add_named_node(&mut g, 4294970107, "clang::IndirectGotoStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_INDIRECTGOTOSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MAYALIASATTR, "clang::MayAliasAttr");
  g.add_edge((CLASS_MAYALIASATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MAYALIASATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MAYALIASATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SEHLEAVESTMT, "clang::SEHLeaveStmt");
  g.add_edge((CLASS_SEHLEAVESTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SEHLEAVESTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970110, "clang::SEHLeaveStmt::getLeaveLoc"),
      add_named_node(&mut g, 4294970111, "clang::SEHLeaveStmt::getBeginLoc"),
      add_named_node(&mut g, 4294970112, "clang::SEHLeaveStmt::getEndLoc"),
      add_named_node(&mut g, 4294970113, "clang::SEHLeaveStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SEHLEAVESTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MIPSSHORTCALLATTR, "clang::MipsShortCallAttr");
  g.add_edge((CLASS_MIPSSHORTCALLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MIPSSHORTCALLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MIPSSHORTCALLATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NOPROFILEFUNCTIONATTR, "clang::NoProfileFunctionAttr");
  g.add_edge((CLASS_NOPROFILEFUNCTIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOPROFILEFUNCTIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NOPROFILEFUNCTIONATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TLSMODELATTR, "clang::TLSModelAttr");
  g.add_edge((CLASS_TLSMODELATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TLSMODELATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TLSMODELATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CMSENSENTRYATTR, "clang::CmseNSEntryAttr");
  g.add_edge((CLASS_CMSENSENTRYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CMSENSENTRYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CMSENSENTRYATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DESTRUCTORATTR, "clang::DestructorAttr");
  g.add_edge((CLASS_DESTRUCTORATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DESTRUCTORATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DESTRUCTORATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NSCONSUMEDATTR, "clang::NSConsumedAttr");
  g.add_edge((CLASS_NSCONSUMEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NSCONSUMEDATTR, META_SUBCLASS, CLASS_INHERITABLEPARAMATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NSCONSUMEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ANNOTATETYPEATTR, "clang::AnnotateTypeAttr");
  g.add_edge((CLASS_ANNOTATETYPEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ANNOTATETYPEATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ANNOTATETYPEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MSVTORDISPATTR, "clang::MSVtorDispAttr");
  g.add_edge((CLASS_MSVTORDISPATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MSVTORDISPATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MSVTORDISPATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MSSTRUCTATTR, "clang::MSStructAttr");
  g.add_edge((CLASS_MSSTRUCTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MSSTRUCTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MSSTRUCTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DECL, "clang::Decl");
  g.add_edge((CLASS_DECL, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970124, "clang::Decl::getSourceRange"),
      add_named_node(&mut g, 4294970125, "clang::Decl::getBeginLoc"),
      add_named_node(&mut g, 4294970126, "clang::Decl::getEndLoc"),
      add_named_node(&mut g, 4294970127, "clang::Decl::getLocation"),
      add_named_node(&mut g, 4294970128, "clang::Decl::getKind"),
      add_named_node(&mut g, 4294970129, "clang::Decl::getDeclKindName"),
      add_named_node(&mut g, 4294970130, "clang::Decl::getNextDeclInContext"),
      add_named_node(&mut g, 4294970131, "clang::Decl::getDeclContext"),
      add_named_node(&mut g, 4294970132, "clang::Decl::getNonTransparentDeclContext"),
      add_named_node(&mut g, 4294970133, "clang::Decl::getNonClosureContext"),
      add_named_node(&mut g, 4294970134, "clang::Decl::getTranslationUnitDecl"),
      add_named_node(&mut g, 4294970135, "clang::Decl::isInAnonymousNamespace"),
      add_named_node(&mut g, 4294970136, "clang::Decl::isInStdNamespace"),
      add_named_node(&mut g, 4294970137, "clang::Decl::isFileContextDecl"),
      add_named_node(&mut g, 4294970138, "clang::Decl::getASTContext"),
      add_named_node(&mut g, 4294970139, "clang::Decl::getLangOpts"),
      add_named_node(&mut g, 4294970140, "clang::Decl::getAccess"),
      add_named_node(&mut g, 4294970141, "clang::Decl::getAccessUnsafe"),
      add_named_node(&mut g, 4294970142, "clang::Decl::hasAttrs"),
      add_named_node(&mut g, 4294970143, "clang::Decl::getAttrs"),
      add_named_node(&mut g, 4294970144, "clang::Decl::attrs"),
      add_named_node(&mut g, 4294970145, "clang::Decl::attr_begin"),
      add_named_node(&mut g, 4294970146, "clang::Decl::attr_end"),
      add_named_node(&mut g, 4294970147, "clang::Decl::getMaxAlignment"),
      add_named_node(&mut g, 4294970148, "clang::Decl::isInvalidDecl"),
      add_named_node(&mut g, 4294970149, "clang::Decl::isImplicit"),
      add_named_node(&mut g, 4294970150, "clang::Decl::isReferenced"),
      add_named_node(&mut g, 4294970151, "clang::Decl::isThisDeclarationReferenced"),
      add_named_node(&mut g, 4294970152, "clang::Decl::isTopLevelDeclInObjCContainer"),
      add_named_node(&mut g, 4294970153, "clang::Decl::getExternalSourceSymbolAttr"),
      add_named_node(&mut g, 4294970154, "clang::Decl::isModulePrivate"),
      add_named_node(&mut g, 4294970155, "clang::Decl::isInExportDeclContext"),
      add_named_node(&mut g, 4294970156, "clang::Decl::isInvisibleOutsideTheOwningModule"),
      add_named_node(&mut g, 4294970157, "clang::Decl::isInAnotherModuleUnit"),
      add_named_node(&mut g, 4294970158, "clang::Decl::isDiscardedInGlobalModuleFragment"),
      add_named_node(&mut g, 4294970159, "clang::Decl::shouldSkipCheckingODR"),
      add_named_node(&mut g, 4294970160, "clang::Decl::hasDefiningAttr"),
      add_named_node(&mut g, 4294970161, "clang::Decl::getDefiningAttr"),
      add_named_node(&mut g, 4294970162, "clang::Decl::getVersionIntroduced"),
      add_named_node(&mut g, 4294970163, "clang::Decl::isWeakImported"),
      add_named_node(&mut g, 4294970164, "clang::Decl::isFromASTFile"),
      add_named_node(&mut g, 4294970165, "clang::Decl::getGlobalID"),
      add_named_node(&mut g, 4294970166, "clang::Decl::getOwningModuleID"),
      add_named_node(&mut g, 4294970167, "clang::Decl::getImportedOwningModule"),
      add_named_node(&mut g, 4294970168, "clang::Decl::getLocalOwningModule"),
      add_named_node(&mut g, 4294970169, "clang::Decl::hasOwningModule"),
      add_named_node(&mut g, 4294970170, "clang::Decl::getOwningModule"),
      add_named_node(&mut g, 4294970171, "clang::Decl::isUnconditionallyVisible"),
      add_named_node(&mut g, 4294970172, "clang::Decl::isReachable"),
      add_named_node(&mut g, 4294970173, "clang::Decl::getModuleOwnershipKind"),
      add_named_node(&mut g, 4294970174, "clang::Decl::getIdentifierNamespace"),
      add_named_node(&mut g, 4294970175, "clang::Decl::hasTagIdentifierNamespace"),
      add_named_node(&mut g, 4294970176, "clang::Decl::getLexicalDeclContext"),
      add_named_node(&mut g, 4294970177, "clang::Decl::isOutOfLine"),
      add_named_node(&mut g, 4294970178, "clang::Decl::isTemplated"),
      add_named_node(&mut g, 4294970179, "clang::Decl::getTemplateDepth"),
      add_named_node(&mut g, 4294970180, "clang::Decl::isDefinedOutsideFunctionOrMethod"),
      add_named_node(&mut g, 4294970181, "clang::Decl::isInLocalScopeForInstantiation"),
      add_named_node(&mut g, 4294970182, "clang::Decl::getCanonicalDecl"),
      add_named_node(&mut g, 4294970183, "clang::Decl::isCanonicalDecl"),
      add_named_node(&mut g, 4294970184, "clang::Decl::redecls"),
      add_named_node(&mut g, 4294970185, "clang::Decl::redecls_begin"),
      add_named_node(&mut g, 4294970186, "clang::Decl::redecls_end"),
      add_named_node(&mut g, 4294970187, "clang::Decl::getPreviousDecl"),
      add_named_node(&mut g, 4294970188, "clang::Decl::isFirstDecl"),
      add_named_node(&mut g, 4294970189, "clang::Decl::getMostRecentDecl"),
      add_named_node(&mut g, 4294970190, "clang::Decl::getBody"),
      add_named_node(&mut g, 4294970191, "clang::Decl::hasBody"),
      add_named_node(&mut g, 4294970192, "clang::Decl::getBodyRBrace"),
      add_named_node(&mut g, 4294970193, "clang::Decl::isTemplateParameter"),
      add_named_node(&mut g, 4294970194, "clang::Decl::isTemplateParameterPack"),
      add_named_node(&mut g, 4294970195, "clang::Decl::isParameterPack"),
      add_named_node(&mut g, 4294970196, "clang::Decl::isTemplateDecl"),
      add_named_node(&mut g, 4294970197, "clang::Decl::isFunctionOrFunctionTemplate"),
      add_named_node(&mut g, 4294970198, "clang::Decl::getDescribedTemplate"),
      add_named_node(&mut g, 4294970199, "clang::Decl::getDescribedTemplateParams"),
      add_named_node(&mut g, 4294970200, "clang::Decl::getAsFunction"),
      add_named_node(&mut g, 4294970201, "clang::Decl::isLocalExternDecl"),
      add_named_node(&mut g, 4294970202, "clang::Decl::getFriendObjectKind"),
      add_named_node(&mut g, 4294970203, "clang::Decl::getID"),
      add_named_node(&mut g, 4294970204, "clang::Decl::isFunctionPointerType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_HLSLNUMTHREADSATTR, "clang::HLSLNumThreadsAttr");
  g.add_edge((CLASS_HLSLNUMTHREADSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_HLSLNUMTHREADSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_HLSLNUMTHREADSATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TRIVIALABIATTR, "clang::TrivialABIAttr");
  g.add_edge((CLASS_TRIVIALABIATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TRIVIALABIATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TRIVIALABIATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MAXFIELDALIGNMENTATTR, "clang::MaxFieldAlignmentAttr");
  g.add_edge((CLASS_MAXFIELDALIGNMENTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MAXFIELDALIGNMENTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MAXFIELDALIGNMENTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MSP430INTERRUPTATTR, "clang::MSP430InterruptAttr");
  g.add_edge((CLASS_MSP430INTERRUPTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MSP430INTERRUPTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MSP430INTERRUPTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_RETURNTYPESTATEATTR, "clang::ReturnTypestateAttr");
  g.add_edge((CLASS_RETURNTYPESTATEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RETURNTYPESTATEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_RETURNTYPESTATEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCBOXEDEXPR, "clang::ObjCBoxedExpr");
  g.add_edge((CLASS_OBJCBOXEDEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCBOXEDEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCBOXEDEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CONSTATTR, "clang::ConstAttr");
  g.add_edge((CLASS_CONSTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONSTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CONSTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MSINHERITANCEATTR, "clang::MSInheritanceAttr");
  g.add_edge((CLASS_MSINHERITANCEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MSINHERITANCEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MSINHERITANCEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPMASTERDIRECTIVE, "clang::OMPMasterDirective");
  g.add_edge((CLASS_OMPMASTERDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPMASTERDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPMASTERDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCOBJECTTYPEIMPL, "clang::ObjCObjectTypeImpl");
  g.add_edge((CLASS_OBJCOBJECTTYPEIMPL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCOBJECTTYPEIMPL, META_SUBCLASS, CLASS_OBJCOBJECTTYPE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCOBJECTTYPEIMPL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TEMPLATESPECIALIZATIONTYPE, "clang::TemplateSpecializationType");
  g.add_edge((CLASS_TEMPLATESPECIALIZATIONTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TEMPLATESPECIALIZATIONTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970216, "clang::TemplateSpecializationType::isCurrentInstantiation"),
      add_named_node(&mut g, 4294970217, "clang::TemplateSpecializationType::isTypeAlias"),
      add_named_node(&mut g, 4294970218, "clang::TemplateSpecializationType::getAliasedType"),
      add_named_node(&mut g, 4294970219, "clang::TemplateSpecializationType::getTemplateName"),
      add_named_node(&mut g, 4294970220, "clang::TemplateSpecializationType::template_arguments"),
      add_named_node(&mut g, 4294970221, "clang::TemplateSpecializationType::isSugared"),
      add_named_node(&mut g, 4294970222, "clang::TemplateSpecializationType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TEMPLATESPECIALIZATIONTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MSCONSTEXPRATTR, "clang::MSConstexprAttr");
  g.add_edge((CLASS_MSCONSTEXPRATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MSCONSTEXPRATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MSCONSTEXPRATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_RVALUEREFERENCETYPE, "clang::RValueReferenceType");
  g.add_edge((CLASS_RVALUEREFERENCETYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RVALUEREFERENCETYPE, META_SUBCLASS, CLASS_REFERENCETYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970225, "clang::RValueReferenceType::isSugared"),
      add_named_node(&mut g, 4294970226, "clang::RValueReferenceType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_RVALUEREFERENCETYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SWIFTBRIDGEDTYPEDEFATTR, "clang::SwiftBridgedTypedefAttr");
  g.add_edge((CLASS_SWIFTBRIDGEDTYPEDEFATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTBRIDGEDTYPEDEFATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SWIFTBRIDGEDTYPEDEFATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCMETHODFAMILYATTR, "clang::ObjCMethodFamilyAttr");
  g.add_edge((CLASS_OBJCMETHODFAMILYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCMETHODFAMILYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCMETHODFAMILYATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_LOCKSEXCLUDEDATTR, "clang::LocksExcludedAttr");
  g.add_edge((CLASS_LOCKSEXCLUDEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LOCKSEXCLUDEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_LOCKSEXCLUDEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BITINTTYPELOC, "clang::BitIntTypeLoc");
  g.add_edge((CLASS_BITINTTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BITINTTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCENCODEEXPR, "clang::ObjCEncodeExpr");
  g.add_edge((CLASS_OBJCENCODEEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCENCODEEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCENCODEEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_HOTATTR, "clang::HotAttr");
  g.add_edge((CLASS_HOTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_HOTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_HOTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ACQUIREDBEFOREATTR, "clang::AcquiredBeforeAttr");
  g.add_edge((CLASS_ACQUIREDBEFOREATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ACQUIREDBEFOREATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ACQUIREDBEFOREATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTHREADPRIVATEDECLATTR, "clang::OMPThreadPrivateDeclAttr");
  g.add_edge((CLASS_OMPTHREADPRIVATEDECLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTHREADPRIVATEDECLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTHREADPRIVATEDECLATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_INTEGERLITERAL, "clang::IntegerLiteral");
  g.add_edge((CLASS_INTEGERLITERAL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_INTEGERLITERAL, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970236, "clang::IntegerLiteral::getBeginLoc"),
      add_named_node(&mut g, 4294970237, "clang::IntegerLiteral::getEndLoc"),
      add_named_node(&mut g, 4294970238, "clang::IntegerLiteral::getLocation"),
      add_named_node(&mut g, 4294970239, "clang::IntegerLiteral::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_INTEGERLITERAL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MSALLOCATORATTR, "clang::MSAllocatorAttr");
  g.add_edge((CLASS_MSALLOCATORATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MSALLOCATORATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MSALLOCATORATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ENUMEXTENSIBILITYATTR, "clang::EnumExtensibilityAttr");
  g.add_edge((CLASS_ENUMEXTENSIBILITYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ENUMEXTENSIBILITYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ENUMEXTENSIBILITYATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CUDACONSTANTATTR, "clang::CUDAConstantAttr");
  g.add_edge((CLASS_CUDACONSTANTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CUDACONSTANTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CUDACONSTANTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NOSTACKPROTECTORATTR, "clang::NoStackProtectorAttr");
  g.add_edge((CLASS_NOSTACKPROTECTORATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOSTACKPROTECTORATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NOSTACKPROTECTORATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DIAGNOSEIFATTR, "clang::DiagnoseIfAttr");
  g.add_edge((CLASS_DIAGNOSEIFATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DIAGNOSEIFATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DIAGNOSEIFATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_USINGSHADOWDECL, "clang::UsingShadowDecl");
  g.add_edge((CLASS_USINGSHADOWDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_USINGSHADOWDECL, META_SUBCLASS, CLASS_NAMEDDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970246, "clang::UsingShadowDecl::getCanonicalDecl"),
      add_named_node(&mut g, 4294970247, "clang::UsingShadowDecl::getTargetDecl"),
      add_named_node(&mut g, 4294970248, "clang::UsingShadowDecl::getIntroducer"),
      add_named_node(&mut g, 4294970249, "clang::UsingShadowDecl::getNextUsingShadowDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_USINGSHADOWDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ADJUSTEDTYPE, "clang::AdjustedType");
  g.add_edge((CLASS_ADJUSTEDTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ADJUSTEDTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970251, "clang::AdjustedType::getOriginalType"),
      add_named_node(&mut g, 4294970252, "clang::AdjustedType::getAdjustedType"),
      add_named_node(&mut g, 4294970253, "clang::AdjustedType::isSugared"),
      add_named_node(&mut g, 4294970254, "clang::AdjustedType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ADJUSTEDTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTSIZEDEXTVECTORTYPE, "clang::DependentSizedExtVectorType");
  g.add_edge((CLASS_DEPENDENTSIZEDEXTVECTORTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEPENDENTSIZEDEXTVECTORTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970256, "clang::DependentSizedExtVectorType::getSizeExpr"),
      add_named_node(&mut g, 4294970257, "clang::DependentSizedExtVectorType::getElementType"),
      add_named_node(&mut g, 4294970258, "clang::DependentSizedExtVectorType::getAttributeLoc"),
      add_named_node(&mut g, 4294970259, "clang::DependentSizedExtVectorType::isSugared"),
      add_named_node(&mut g, 4294970260, "clang::DependentSizedExtVectorType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTSIZEDEXTVECTORTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MIGSERVERROUTINEATTR, "clang::MIGServerRoutineAttr");
  g.add_edge((CLASS_MIGSERVERROUTINEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MIGSERVERROUTINEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MIGSERVERROUTINEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXMETHODDECL, "clang::CXXMethodDecl");
  g.add_edge((CLASS_CXXMETHODDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXMETHODDECL, META_SUBCLASS, CLASS_FUNCTIONDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970263, "clang::CXXMethodDecl::isStatic"),
      add_named_node(&mut g, 4294970264, "clang::CXXMethodDecl::isInstance"),
      add_named_node(&mut g, 4294970265, "clang::CXXMethodDecl::isExplicitObjectMemberFunction"),
      add_named_node(&mut g, 4294970266, "clang::CXXMethodDecl::isImplicitObjectMemberFunction"),
      add_named_node(&mut g, 4294970267, "clang::CXXMethodDecl::isConst"),
      add_named_node(&mut g, 4294970268, "clang::CXXMethodDecl::isVolatile"),
      add_named_node(&mut g, 4294970269, "clang::CXXMethodDecl::isVirtual"),
      add_named_node(&mut g, 4294970270, "clang::CXXMethodDecl::isCopyAssignmentOperator"),
      add_named_node(&mut g, 4294970271, "clang::CXXMethodDecl::isMoveAssignmentOperator"),
      add_named_node(&mut g, 4294970272, "clang::CXXMethodDecl::getCanonicalDecl"),
      add_named_node(&mut g, 4294970273, "clang::CXXMethodDecl::getMostRecentDecl"),
      add_named_node(&mut g, 4294970274, "clang::CXXMethodDecl::begin_overridden_methods"),
      add_named_node(&mut g, 4294970275, "clang::CXXMethodDecl::end_overridden_methods"),
      add_named_node(&mut g, 4294970276, "clang::CXXMethodDecl::size_overridden_methods"),
      add_named_node(&mut g, 4294970277, "clang::CXXMethodDecl::overridden_methods"),
      add_named_node(&mut g, 4294970278, "clang::CXXMethodDecl::getParent"),
      add_named_node(&mut g, 4294970279, "clang::CXXMethodDecl::getThisType"),
      add_named_node(&mut g, 4294970280, "clang::CXXMethodDecl::getFunctionObjectParameterReferenceType"),
      add_named_node(&mut g, 4294970281, "clang::CXXMethodDecl::getFunctionObjectParameterType"),
      add_named_node(&mut g, 4294970282, "clang::CXXMethodDecl::getNumExplicitParams"),
      add_named_node(&mut g, 4294970283, "clang::CXXMethodDecl::getMethodQualifiers"),
      add_named_node(&mut g, 4294970284, "clang::CXXMethodDecl::getRefQualifier"),
      add_named_node(&mut g, 4294970285, "clang::CXXMethodDecl::hasInlineBody"),
      add_named_node(&mut g, 4294970286, "clang::CXXMethodDecl::isLambdaStaticInvoker"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXMETHODDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_RANDOMIZELAYOUTATTR, "clang::RandomizeLayoutAttr");
  g.add_edge((CLASS_RANDOMIZELAYOUTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RANDOMIZELAYOUTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_RANDOMIZELAYOUTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_COROWRAPPERATTR, "clang::CoroWrapperAttr");
  g.add_edge((CLASS_COROWRAPPERATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_COROWRAPPERATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_COROWRAPPERATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CLASSTEMPLATEPARTIALSPECIALIZATIONDECL, "clang::ClassTemplatePartialSpecializationDecl");
  g.add_edge((CLASS_CLASSTEMPLATEPARTIALSPECIALIZATIONDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CLASSTEMPLATEPARTIALSPECIALIZATIONDECL, META_SUBCLASS, CLASS_CLASSTEMPLATESPECIALIZATIONDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970290, "clang::ClassTemplatePartialSpecializationDecl::getTemplateParameters"),
      add_named_node(&mut g, 4294970291, "clang::ClassTemplatePartialSpecializationDecl::hasAssociatedConstraints"),
      add_named_node(&mut g, 4294970292, "clang::ClassTemplatePartialSpecializationDecl::getTemplateArgsAsWritten"),
      add_named_node(&mut g, 4294970293, "clang::ClassTemplatePartialSpecializationDecl::getInstantiatedFromMember"),
      add_named_node(&mut g, 4294970294, "clang::ClassTemplatePartialSpecializationDecl::getInstantiatedFromMemberTemplate"),
      add_named_node(&mut g, 4294970295, "clang::ClassTemplatePartialSpecializationDecl::getInjectedSpecializationType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CLASSTEMPLATEPARTIALSPECIALIZATIONDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPENULLUNSPECIFIEDATTR, "clang::TypeNullUnspecifiedAttr");
  g.add_edge((CLASS_TYPENULLUNSPECIFIEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPENULLUNSPECIFIEDATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPENULLUNSPECIFIEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_WHILESTMT, "clang::WhileStmt");
  g.add_edge((CLASS_WHILESTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_WHILESTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970298, "clang::WhileStmt::hasVarStorage"),
      add_named_node(&mut g, 4294970299, "clang::WhileStmt::getCond"),
      add_named_node(&mut g, 4294970300, "clang::WhileStmt::getBody"),
      add_named_node(&mut g, 4294970301, "clang::WhileStmt::getConditionVariable"),
      add_named_node(&mut g, 4294970302, "clang::WhileStmt::getConditionVariableDeclStmt"),
      add_named_node(&mut g, 4294970303, "clang::WhileStmt::getWhileLoc"),
      add_named_node(&mut g, 4294970304, "clang::WhileStmt::getLParenLoc"),
      add_named_node(&mut g, 4294970305, "clang::WhileStmt::getRParenLoc"),
      add_named_node(&mut g, 4294970306, "clang::WhileStmt::getBeginLoc"),
      add_named_node(&mut g, 4294970307, "clang::WhileStmt::getEndLoc"),
      add_named_node(&mut g, 4294970308, "clang::WhileStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_WHILESTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CORORETURNTYPEATTR, "clang::CoroReturnTypeAttr");
  g.add_edge((CLASS_CORORETURNTYPEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CORORETURNTYPEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CORORETURNTYPEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ADDRLABELEXPR, "clang::AddrLabelExpr");
  g.add_edge((CLASS_ADDRLABELEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ADDRLABELEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970311, "clang::AddrLabelExpr::getAmpAmpLoc"),
      add_named_node(&mut g, 4294970312, "clang::AddrLabelExpr::getLabelLoc"),
      add_named_node(&mut g, 4294970313, "clang::AddrLabelExpr::getBeginLoc"),
      add_named_node(&mut g, 4294970314, "clang::AddrLabelExpr::getEndLoc"),
      add_named_node(&mut g, 4294970315, "clang::AddrLabelExpr::getLabel"),
      add_named_node(&mut g, 4294970316, "clang::AddrLabelExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ADDRLABELEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CUDAINVALIDTARGETATTR, "clang::CUDAInvalidTargetAttr");
  g.add_edge((CLASS_CUDAINVALIDTARGETATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CUDAINVALIDTARGETATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CUDAINVALIDTARGETATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_M68KRTDATTR, "clang::M68kRTDAttr");
  g.add_edge((CLASS_M68KRTDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_M68KRTDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_M68KRTDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPDECLARETARGETDECLATTR, "clang::OMPDeclareTargetDeclAttr");
  g.add_edge((CLASS_OMPDECLARETARGETDECLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPDECLARETARGETDECLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPDECLARETARGETDECLATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_LIFETIMEBOUNDATTR, "clang::LifetimeBoundAttr");
  g.add_edge((CLASS_LIFETIMEBOUNDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LIFETIMEBOUNDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_LIFETIMEBOUNDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PARENLISTEXPR, "clang::ParenListExpr");
  g.add_edge((CLASS_PARENLISTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PARENLISTEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970322, "clang::ParenListExpr::getNumExprs"),
      add_named_node(&mut g, 4294970323, "clang::ParenListExpr::getLParenLoc"),
      add_named_node(&mut g, 4294970324, "clang::ParenListExpr::getRParenLoc"),
      add_named_node(&mut g, 4294970325, "clang::ParenListExpr::getBeginLoc"),
      add_named_node(&mut g, 4294970326, "clang::ParenListExpr::getEndLoc"),
      add_named_node(&mut g, 4294970327, "clang::ParenListExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PARENLISTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CARRIESDEPENDENCYATTR, "clang::CarriesDependencyAttr");
  g.add_edge((CLASS_CARRIESDEPENDENCYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CARRIESDEPENDENCYATTR, META_SUBCLASS, CLASS_INHERITABLEPARAMATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CARRIESDEPENDENCYATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_LEAFATTR, "clang::LeafAttr");
  g.add_edge((CLASS_LEAFATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LEAFATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_LEAFATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ENUMTYPELOC, "clang::EnumTypeLoc");
  g.add_edge((CLASS_ENUMTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970331, "clang::EnumTypeLoc::getDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ENUMTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_RISCVINTERRUPTATTR, "clang::RISCVInterruptAttr");
  g.add_edge((CLASS_RISCVINTERRUPTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RISCVINTERRUPTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_RISCVINTERRUPTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OVERLOADABLEATTR, "clang::OverloadableAttr");
  g.add_edge((CLASS_OVERLOADABLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OVERLOADABLEATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OVERLOADABLEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_LAYOUTVERSIONATTR, "clang::LayoutVersionAttr");
  g.add_edge((CLASS_LAYOUTVERSIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LAYOUTVERSIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_LAYOUTVERSIONATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_INTERNALLINKAGEATTR, "clang::InternalLinkageAttr");
  g.add_edge((CLASS_INTERNALLINKAGEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_INTERNALLINKAGEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_INTERNALLINKAGEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCDIRECTMEMBERSATTR, "clang::ObjCDirectMembersAttr");
  g.add_edge((CLASS_OBJCDIRECTMEMBERSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCDIRECTMEMBERSATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCDIRECTMEMBERSATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_IBOUTLETCOLLECTIONATTR, "clang::IBOutletCollectionAttr");
  g.add_edge((CLASS_IBOUTLETCOLLECTIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_IBOUTLETCOLLECTIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_IBOUTLETCOLLECTIONATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXFORRANGESTMT, "clang::CXXForRangeStmt");
  g.add_edge((CLASS_CXXFORRANGESTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXFORRANGESTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970339, "clang::CXXForRangeStmt::getInit"),
      add_named_node(&mut g, 4294970340, "clang::CXXForRangeStmt::getLoopVariable"),
      add_named_node(&mut g, 4294970341, "clang::CXXForRangeStmt::getRangeInit"),
      add_named_node(&mut g, 4294970342, "clang::CXXForRangeStmt::getRangeStmt"),
      add_named_node(&mut g, 4294970343, "clang::CXXForRangeStmt::getBeginStmt"),
      add_named_node(&mut g, 4294970344, "clang::CXXForRangeStmt::getEndStmt"),
      add_named_node(&mut g, 4294970345, "clang::CXXForRangeStmt::getCond"),
      add_named_node(&mut g, 4294970346, "clang::CXXForRangeStmt::getInc"),
      add_named_node(&mut g, 4294970347, "clang::CXXForRangeStmt::getLoopVarStmt"),
      add_named_node(&mut g, 4294970348, "clang::CXXForRangeStmt::getBody"),
      add_named_node(&mut g, 4294970349, "clang::CXXForRangeStmt::getForLoc"),
      add_named_node(&mut g, 4294970350, "clang::CXXForRangeStmt::getCoawaitLoc"),
      add_named_node(&mut g, 4294970351, "clang::CXXForRangeStmt::getColonLoc"),
      add_named_node(&mut g, 4294970352, "clang::CXXForRangeStmt::getRParenLoc"),
      add_named_node(&mut g, 4294970353, "clang::CXXForRangeStmt::getBeginLoc"),
      add_named_node(&mut g, 4294970354, "clang::CXXForRangeStmt::getEndLoc"),
      add_named_node(&mut g, 4294970355, "clang::CXXForRangeStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXFORRANGESTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CONCEPTDECL, "clang::ConceptDecl");
  g.add_edge((CLASS_CONCEPTDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONCEPTDECL, META_SUBCLASS, CLASS_TEMPLATEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970357, "clang::ConceptDecl::getConstraintExpr"),
      add_named_node(&mut g, 4294970358, "clang::ConceptDecl::getSourceRange"),
      add_named_node(&mut g, 4294970359, "clang::ConceptDecl::isTypeConcept"),
      add_named_node(&mut g, 4294970360, "clang::ConceptDecl::getCanonicalDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CONCEPTDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DISABLETAILCALLSATTR, "clang::DisableTailCallsAttr");
  g.add_edge((CLASS_DISABLETAILCALLSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DISABLETAILCALLSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DISABLETAILCALLSATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FORMATATTR, "clang::FormatAttr");
  g.add_edge((CLASS_FORMATATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FORMATATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FORMATATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCINTERFACETYPE, "clang::ObjCInterfaceType");
  g.add_edge((CLASS_OBJCINTERFACETYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCINTERFACETYPE, META_SUBCLASS, CLASS_OBJCOBJECTTYPE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCINTERFACETYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_REDECLARABLETEMPLATEDECL, "clang::RedeclarableTemplateDecl");
  g.add_edge((CLASS_REDECLARABLETEMPLATEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_REDECLARABLETEMPLATEDECL, META_SUBCLASS, CLASS_TEMPLATEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970365, "clang::RedeclarableTemplateDecl::getCanonicalDecl"),
      add_named_node(&mut g, 4294970366, "clang::RedeclarableTemplateDecl::isMemberSpecialization"),
      add_named_node(&mut g, 4294970367, "clang::RedeclarableTemplateDecl::getInstantiatedFromMemberTemplate"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_REDECLARABLETEMPLATEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PARENTYPELOC, "clang::ParenTypeLoc");
  g.add_edge((CLASS_PARENTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970369, "clang::ParenTypeLoc::getLParenLoc"),
      add_named_node(&mut g, 4294970370, "clang::ParenTypeLoc::getRParenLoc"),
      add_named_node(&mut g, 4294970371, "clang::ParenTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, 4294970372, "clang::ParenTypeLoc::getInnerLoc"),
      add_named_node(&mut g, 4294970373, "clang::ParenTypeLoc::getInnerType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PARENTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FALLTHROUGHATTR, "clang::FallThroughAttr");
  g.add_edge((CLASS_FALLTHROUGHATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FALLTHROUGHATTR, META_SUBCLASS, CLASS_STMTATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FALLTHROUGHATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_IBACTIONATTR, "clang::IBActionAttr");
  g.add_edge((CLASS_IBACTIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_IBACTIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_IBACTIONATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXCONSTCASTEXPR, "clang::CXXConstCastExpr");
  g.add_edge((CLASS_CXXCONSTCASTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXCONSTCASTEXPR, META_SUBCLASS, CLASS_CXXNAMEDCASTEXPR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXCONSTCASTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CALLEXPR, "clang::CallExpr");
  g.add_edge((CLASS_CALLEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CALLEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970378, "clang::CallExpr::getCallee"),
      add_named_node(&mut g, 4294970379, "clang::CallExpr::getADLCallKind"),
      add_named_node(&mut g, 4294970380, "clang::CallExpr::usesADL"),
      add_named_node(&mut g, 4294970381, "clang::CallExpr::hasStoredFPFeatures"),
      add_named_node(&mut g, 4294970382, "clang::CallExpr::getCalleeDecl"),
      add_named_node(&mut g, 4294970383, "clang::CallExpr::getDirectCallee"),
      add_named_node(&mut g, 4294970384, "clang::CallExpr::getNumArgs"),
      add_named_node(&mut g, 4294970385, "clang::CallExpr::getArgs"),
      add_named_node(&mut g, 4294970386, "clang::CallExpr::arguments"),
      add_named_node(&mut g, 4294970387, "clang::CallExpr::arg_begin"),
      add_named_node(&mut g, 4294970388, "clang::CallExpr::arg_end"),
      add_named_node(&mut g, 4294970389, "clang::CallExpr::getStoredFPFeatures"),
      add_named_node(&mut g, 4294970390, "clang::CallExpr::getFPFeatures"),
      add_named_node(&mut g, 4294970391, "clang::CallExpr::getBuiltinCallee"),
      add_named_node(&mut g, 4294970392, "clang::CallExpr::getRParenLoc"),
      add_named_node(&mut g, 4294970393, "clang::CallExpr::getBeginLoc"),
      add_named_node(&mut g, 4294970394, "clang::CallExpr::getEndLoc"),
      add_named_node(&mut g, 4294970395, "clang::CallExpr::isCallToStdMove"),
      add_named_node(&mut g, 4294970396, "clang::CallExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CALLEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FLAGENUMATTR, "clang::FlagEnumAttr");
  g.add_edge((CLASS_FLAGENUMATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FLAGENUMATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FLAGENUMATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CUDAHOSTATTR, "clang::CUDAHostAttr");
  g.add_edge((CLASS_CUDAHOSTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CUDAHOSTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CUDAHOSTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPARRAYSECTIONEXPR, "clang::OMPArraySectionExpr");
  g.add_edge((CLASS_OMPARRAYSECTIONEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPARRAYSECTIONEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPARRAYSECTIONEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SUBSTTEMPLATETYPEPARMTYPE, "clang::SubstTemplateTypeParmType");
  g.add_edge((CLASS_SUBSTTEMPLATETYPEPARMTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SUBSTTEMPLATETYPEPARMTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970401, "clang::SubstTemplateTypeParmType::getReplacementType"),
      add_named_node(&mut g, 4294970402, "clang::SubstTemplateTypeParmType::getAssociatedDecl"),
      add_named_node(&mut g, 4294970403, "clang::SubstTemplateTypeParmType::getReplacedParameter"),
      add_named_node(&mut g, 4294970404, "clang::SubstTemplateTypeParmType::getIndex"),
      add_named_node(&mut g, 4294970405, "clang::SubstTemplateTypeParmType::getPackIndex"),
      add_named_node(&mut g, 4294970406, "clang::SubstTemplateTypeParmType::isSugared"),
      add_named_node(&mut g, 4294970407, "clang::SubstTemplateTypeParmType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SUBSTTEMPLATETYPEPARMTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_XRAYINSTRUMENTATTR, "clang::XRayInstrumentAttr");
  g.add_edge((CLASS_XRAYINSTRUMENTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_XRAYINSTRUMENTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_XRAYINSTRUMENTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ARRAYSUBSCRIPTEXPR, "clang::ArraySubscriptExpr");
  g.add_edge((CLASS_ARRAYSUBSCRIPTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARRAYSUBSCRIPTEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970410, "clang::ArraySubscriptExpr::getLHS"),
      add_named_node(&mut g, 4294970411, "clang::ArraySubscriptExpr::getRHS"),
      add_named_node(&mut g, 4294970412, "clang::ArraySubscriptExpr::getBase"),
      add_named_node(&mut g, 4294970413, "clang::ArraySubscriptExpr::getIdx"),
      add_named_node(&mut g, 4294970414, "clang::ArraySubscriptExpr::getBeginLoc"),
      add_named_node(&mut g, 4294970415, "clang::ArraySubscriptExpr::getEndLoc"),
      add_named_node(&mut g, 4294970416, "clang::ArraySubscriptExpr::getRBracketLoc"),
      add_named_node(&mut g, 4294970417, "clang::ArraySubscriptExpr::getExprLoc"),
      add_named_node(&mut g, 4294970418, "clang::ArraySubscriptExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ARRAYSUBSCRIPTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DISABLESANITIZERINSTRUMENTATIONATTR, "clang::DisableSanitizerInstrumentationAttr");
  g.add_edge((CLASS_DISABLESANITIZERINSTRUMENTATIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DISABLESANITIZERINSTRUMENTATIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DISABLESANITIZERINSTRUMENTATIONATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_GUARDEDVARATTR, "clang::GuardedVarAttr");
  g.add_edge((CLASS_GUARDEDVARATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_GUARDEDVARATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_GUARDEDVARATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCBRIDGEATTR, "clang::ObjCBridgeAttr");
  g.add_edge((CLASS_OBJCBRIDGEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCBRIDGEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCBRIDGEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_GNUINLINEATTR, "clang::GNUInlineAttr");
  g.add_edge((CLASS_GNUINLINEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_GNUINLINEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_GNUINLINEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_WEAKIMPORTATTR, "clang::WeakImportAttr");
  g.add_edge((CLASS_WEAKIMPORTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_WEAKIMPORTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_WEAKIMPORTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FORMATARGATTR, "clang::FormatArgAttr");
  g.add_edge((CLASS_FORMATARGATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_FORMATARGATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FORMATARGATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPEOFEXPRTYPE, "clang::TypeOfExprType");
  g.add_edge((CLASS_TYPEOFEXPRTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPEOFEXPRTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970426, "clang::TypeOfExprType::getUnderlyingExpr"),
      add_named_node(&mut g, 4294970427, "clang::TypeOfExprType::getKind"),
      add_named_node(&mut g, 4294970428, "clang::TypeOfExprType::desugar"),
      add_named_node(&mut g, 4294970429, "clang::TypeOfExprType::isSugared"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPEOFEXPRTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_USINGTYPELOC, "clang::UsingTypeLoc");
  g.add_edge((CLASS_USINGTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970431, "clang::UsingTypeLoc::getUnderlyingType"),
      add_named_node(&mut g, 4294970432, "clang::UsingTypeLoc::getFoundDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_USINGTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_EXTERNALSOURCESYMBOLATTR, "clang::ExternalSourceSymbolAttr");
  g.add_edge((CLASS_EXTERNALSOURCESYMBOLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_EXTERNALSOURCESYMBOLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_EXTERNALSOURCESYMBOLATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ENFORCETCBLEAFATTR, "clang::EnforceTCBLeafAttr");
  g.add_edge((CLASS_ENFORCETCBLEAFATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ENFORCETCBLEAFATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ENFORCETCBLEAFATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SWIFTCALLATTR, "clang::SwiftCallAttr");
  g.add_edge((CLASS_SWIFTCALLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTCALLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SWIFTCALLATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ENABLEIFATTR, "clang::EnableIfAttr");
  g.add_edge((CLASS_ENABLEIFATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ENABLEIFATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ENABLEIFATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ANYX86NOCFCHECKATTR, "clang::AnyX86NoCfCheckAttr");
  g.add_edge((CLASS_ANYX86NOCFCHECKATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ANYX86NOCFCHECKATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ANYX86NOCFCHECKATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SETTYPESTATEATTR, "clang::SetTypestateAttr");
  g.add_edge((CLASS_SETTYPESTATEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SETTYPESTATEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SETTYPESTATEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OSRETURNSRETAINEDATTR, "clang::OSReturnsRetainedAttr");
  g.add_edge((CLASS_OSRETURNSRETAINEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OSRETURNSRETAINEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OSRETURNSRETAINEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OWNERATTR, "clang::OwnerAttr");
  g.add_edge((CLASS_OWNERATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OWNERATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OWNERATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NOMICROMIPSATTR, "clang::NoMicroMipsAttr");
  g.add_edge((CLASS_NOMICROMIPSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOMICROMIPSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NOMICROMIPSATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPRECATEDATTR, "clang::DeprecatedAttr");
  g.add_edge((CLASS_DEPRECATEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEPRECATEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPRECATEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TESTTYPESTATEATTR, "clang::TestTypestateAttr");
  g.add_edge((CLASS_TESTTYPESTATEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TESTTYPESTATEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TESTTYPESTATEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CFAUDITEDTRANSFERATTR, "clang::CFAuditedTransferAttr");
  g.add_edge((CLASS_CFAUDITEDTRANSFERATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CFAUDITEDTRANSFERATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CFAUDITEDTRANSFERATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_HLSLPARAMMODIFIERATTR, "clang::HLSLParamModifierAttr");
  g.add_edge((CLASS_HLSLPARAMMODIFIERATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_HLSLPARAMMODIFIERATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_HLSLPARAMMODIFIERATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OVERRIDEATTR, "clang::OverrideAttr");
  g.add_edge((CLASS_OVERRIDEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OVERRIDEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OVERRIDEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DLLIMPORTSTATICLOCALATTR, "clang::DLLImportStaticLocalAttr");
  g.add_edge((CLASS_DLLIMPORTSTATICLOCALATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DLLIMPORTSTATICLOCALATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DLLIMPORTSTATICLOCALATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPFLUSHDIRECTIVE, "clang::OMPFlushDirective");
  g.add_edge((CLASS_OMPFLUSHDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPFLUSHDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPFLUSHDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PREFERREDNAMEATTR, "clang::PreferredNameAttr");
  g.add_edge((CLASS_PREFERREDNAMEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PREFERREDNAMEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PREFERREDNAMEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ALWAYSDESTROYATTR, "clang::AlwaysDestroyAttr");
  g.add_edge((CLASS_ALWAYSDESTROYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ALWAYSDESTROYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ALWAYSDESTROYATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BLOCKSATTR, "clang::BlocksAttr");
  g.add_edge((CLASS_BLOCKSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BLOCKSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BLOCKSATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NODUPLICATEATTR, "clang::NoDuplicateAttr");
  g.add_edge((CLASS_NODUPLICATEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NODUPLICATEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NODUPLICATEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OPENCLGENERICADDRESSSPACEATTR, "clang::OpenCLGenericAddressSpaceAttr");
  g.add_edge((CLASS_OPENCLGENERICADDRESSSPACEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OPENCLGENERICADDRESSSPACEATTR, META_SUBCLASS, CLASS_TYPEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OPENCLGENERICADDRESSSPACEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OSCONSUMEDATTR, "clang::OSConsumedAttr");
  g.add_edge((CLASS_OSCONSUMEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OSCONSUMEDATTR, META_SUBCLASS, CLASS_INHERITABLEPARAMATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OSCONSUMEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DLLEXPORTSTATICLOCALATTR, "clang::DLLExportStaticLocalAttr");
  g.add_edge((CLASS_DLLEXPORTSTATICLOCALATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DLLEXPORTSTATICLOCALATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DLLEXPORTSTATICLOCALATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCIMPLEMENTATIONDECL, "clang::ObjCImplementationDecl");
  g.add_edge((CLASS_OBJCIMPLEMENTATIONDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCIMPLEMENTATIONDECL, META_SUBCLASS, CLASS_OBJCIMPLDECL));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCIMPLEMENTATIONDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_WORKGROUPSIZEHINTATTR, "clang::WorkGroupSizeHintAttr");
  g.add_edge((CLASS_WORKGROUPSIZEHINTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_WORKGROUPSIZEHINTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_WORKGROUPSIZEHINTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_POINTERTYPE, "clang::PointerType");
  g.add_edge((CLASS_POINTERTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_POINTERTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970459, "clang::PointerType::getPointeeType"),
      add_named_node(&mut g, 4294970460, "clang::PointerType::isSugared"),
      add_named_node(&mut g, 4294970461, "clang::PointerType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_POINTERTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPPARALLELDIRECTIVE, "clang::OMPParallelDirective");
  g.add_edge((CLASS_OMPPARALLELDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPPARALLELDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPPARALLELDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_COMPOUNDASSIGNOPERATOR, "clang::CompoundAssignOperator");
  g.add_edge((CLASS_COMPOUNDASSIGNOPERATOR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_COMPOUNDASSIGNOPERATOR, META_SUBCLASS, CLASS_BINARYOPERATOR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970464, "clang::CompoundAssignOperator::getComputationLHSType"),
      add_named_node(&mut g, 4294970465, "clang::CompoundAssignOperator::getComputationResultType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_COMPOUNDASSIGNOPERATOR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ENUMTYPE, "clang::EnumType");
  g.add_edge((CLASS_ENUMTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ENUMTYPE, META_SUBCLASS, CLASS_TAGTYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970467, "clang::EnumType::getDecl"),
      add_named_node(&mut g, 4294970468, "clang::EnumType::isSugared"),
      add_named_node(&mut g, 4294970469, "clang::EnumType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ENUMTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_RECORDTYPE, "clang::RecordType");
  g.add_edge((CLASS_RECORDTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RECORDTYPE, META_SUBCLASS, CLASS_TAGTYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970471, "clang::RecordType::getDecl"),
      add_named_node(&mut g, 4294970472, "clang::RecordType::hasConstFields"),
      add_named_node(&mut g, 4294970473, "clang::RecordType::isSugared"),
      add_named_node(&mut g, 4294970474, "clang::RecordType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_RECORDTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPPARALLELMASTERTASKLOOPSIMDDIRECTIVE, "clang::OMPParallelMasterTaskLoopSimdDirective");
  g.add_edge((CLASS_OMPPARALLELMASTERTASKLOOPSIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPPARALLELMASTERTASKLOOPSIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPPARALLELMASTERTASKLOOPSIMDDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NORANDOMIZELAYOUTATTR, "clang::NoRandomizeLayoutAttr");
  g.add_edge((CLASS_NORANDOMIZELAYOUTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NORANDOMIZELAYOUTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NORANDOMIZELAYOUTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ELABORATEDTYPE, "clang::ElaboratedType");
  g.add_edge((CLASS_ELABORATEDTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ELABORATEDTYPE, META_SUBCLASS, CLASS_TYPEWITHKEYWORD));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970478, "clang::ElaboratedType::getQualifier"),
      add_named_node(&mut g, 4294970479, "clang::ElaboratedType::getNamedType"),
      add_named_node(&mut g, 4294970480, "clang::ElaboratedType::desugar"),
      add_named_node(&mut g, 4294970481, "clang::ElaboratedType::isSugared"),
      add_named_node(&mut g, 4294970482, "clang::ElaboratedType::getOwnedTagDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ELABORATEDTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TEMPLATETYPEPARMTYPE, "clang::TemplateTypeParmType");
  g.add_edge((CLASS_TEMPLATETYPEPARMTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TEMPLATETYPEPARMTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970484, "clang::TemplateTypeParmType::getDepth"),
      add_named_node(&mut g, 4294970485, "clang::TemplateTypeParmType::getIndex"),
      add_named_node(&mut g, 4294970486, "clang::TemplateTypeParmType::isParameterPack"),
      add_named_node(&mut g, 4294970487, "clang::TemplateTypeParmType::getDecl"),
      add_named_node(&mut g, 4294970488, "clang::TemplateTypeParmType::getIdentifier"),
      add_named_node(&mut g, 4294970489, "clang::TemplateTypeParmType::isSugared"),
      add_named_node(&mut g, 4294970490, "clang::TemplateTypeParmType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TEMPLATETYPEPARMTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SUBSTTEMPLATETYPEPARMPACKTYPE, "clang::SubstTemplateTypeParmPackType");
  g.add_edge((CLASS_SUBSTTEMPLATETYPEPARMPACKTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SUBSTTEMPLATETYPEPARMPACKTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970492, "clang::SubstTemplateTypeParmPackType::getIdentifier"),
      add_named_node(&mut g, 4294970493, "clang::SubstTemplateTypeParmPackType::getAssociatedDecl"),
      add_named_node(&mut g, 4294970494, "clang::SubstTemplateTypeParmPackType::getReplacedParameter"),
      add_named_node(&mut g, 4294970495, "clang::SubstTemplateTypeParmPackType::getIndex"),
      add_named_node(&mut g, 4294970496, "clang::SubstTemplateTypeParmPackType::getFinal"),
      add_named_node(&mut g, 4294970497, "clang::SubstTemplateTypeParmPackType::getNumArgs"),
      add_named_node(&mut g, 4294970498, "clang::SubstTemplateTypeParmPackType::isSugared"),
      add_named_node(&mut g, 4294970499, "clang::SubstTemplateTypeParmPackType::desugar"),
      add_named_node(&mut g, 4294970500, "clang::SubstTemplateTypeParmPackType::getArgumentPack"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SUBSTTEMPLATETYPEPARMPACKTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPMASKEDTASKLOOPDIRECTIVE, "clang::OMPMaskedTaskLoopDirective");
  g.add_edge((CLASS_OMPMASKEDTASKLOOPDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPMASKEDTASKLOOPDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPMASKEDTASKLOOPDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_LVALUEREFERENCETYPELOC, "clang::LValueReferenceTypeLoc");
  g.add_edge((CLASS_LVALUEREFERENCETYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970503, "clang::LValueReferenceTypeLoc::getAmpLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_LVALUEREFERENCETYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_UNARYTRANSFORMTYPE, "clang::UnaryTransformType");
  g.add_edge((CLASS_UNARYTRANSFORMTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNARYTRANSFORMTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970505, "clang::UnaryTransformType::isSugared"),
      add_named_node(&mut g, 4294970506, "clang::UnaryTransformType::desugar"),
      add_named_node(&mut g, 4294970507, "clang::UnaryTransformType::getUnderlyingType"),
      add_named_node(&mut g, 4294970508, "clang::UnaryTransformType::getBaseType"),
      add_named_node(&mut g, 4294970509, "clang::UnaryTransformType::getUTTKind"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNARYTRANSFORMTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ATTRIBUTEDTYPE, "clang::AttributedType");
  g.add_edge((CLASS_ATTRIBUTEDTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ATTRIBUTEDTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970511, "clang::AttributedType::getAttrKind"),
      add_named_node(&mut g, 4294970512, "clang::AttributedType::getModifiedType"),
      add_named_node(&mut g, 4294970513, "clang::AttributedType::getEquivalentType"),
      add_named_node(&mut g, 4294970514, "clang::AttributedType::isSugared"),
      add_named_node(&mut g, 4294970515, "clang::AttributedType::desugar"),
      add_named_node(&mut g, 4294970516, "clang::AttributedType::isQualifier"),
      add_named_node(&mut g, 4294970517, "clang::AttributedType::isMSTypeSpec"),
      add_named_node(&mut g, 4294970518, "clang::AttributedType::isWebAssemblyFuncrefSpec"),
      add_named_node(&mut g, 4294970519, "clang::AttributedType::isCallingConv"),
      add_named_node(&mut g, 4294970520, "clang::AttributedType::getImmediateNullability"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ATTRIBUTEDTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPTARGETUPDATEDIRECTIVE, "clang::OMPTargetUpdateDirective");
  g.add_edge((CLASS_OMPTARGETUPDATEDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPTARGETUPDATEDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPTARGETUPDATEDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTBITINTTYPE, "clang::DependentBitIntType");
  g.add_edge((CLASS_DEPENDENTBITINTTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEPENDENTBITINTTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970523, "clang::DependentBitIntType::isUnsigned"),
      add_named_node(&mut g, 4294970524, "clang::DependentBitIntType::isSigned"),
      add_named_node(&mut g, 4294970525, "clang::DependentBitIntType::getNumBitsExpr"),
      add_named_node(&mut g, 4294970526, "clang::DependentBitIntType::isSugared"),
      add_named_node(&mut g, 4294970527, "clang::DependentBitIntType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTBITINTTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MATRIXTYPE, "clang::MatrixType");
  g.add_edge((CLASS_MATRIXTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MATRIXTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970529, "clang::MatrixType::getElementType"),
      add_named_node(&mut g, 4294970530, "clang::MatrixType::isSugared"),
      add_named_node(&mut g, 4294970531, "clang::MatrixType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MATRIXTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPPARALLELMASTERDIRECTIVE, "clang::OMPParallelMasterDirective");
  g.add_edge((CLASS_OMPPARALLELMASTERDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPPARALLELMASTERDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPPARALLELMASTERDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCOBJECTPOINTERTYPE, "clang::ObjCObjectPointerType");
  g.add_edge((CLASS_OBJCOBJECTPOINTERTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCOBJECTPOINTERTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCOBJECTPOINTERTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCOBJECTTYPE, "clang::ObjCObjectType");
  g.add_edge((CLASS_OBJCOBJECTTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCOBJECTTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCOBJECTTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCINTERFACEDECL, "clang::ObjCInterfaceDecl");
  g.add_edge((CLASS_OBJCINTERFACEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCINTERFACEDECL, META_SUBCLASS, CLASS_OBJCCONTAINERDECL));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCINTERFACEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPEDEFTYPE, "clang::TypedefType");
  g.add_edge((CLASS_TYPEDEFTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPEDEFTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970537, "clang::TypedefType::getDecl"),
      add_named_node(&mut g, 4294970538, "clang::TypedefType::isSugared"),
      add_named_node(&mut g, 4294970539, "clang::TypedefType::desugar"),
      add_named_node(&mut g, 4294970540, "clang::TypedefType::typeMatchesDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPEDEFTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ADJUSTEDTYPELOC, "clang::AdjustedTypeLoc");
  g.add_edge((CLASS_ADJUSTEDTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970542, "clang::AdjustedTypeLoc::getOriginalLoc"),
      add_named_node(&mut g, 4294970543, "clang::AdjustedTypeLoc::getInnerType"),
      add_named_node(&mut g, 4294970544, "clang::AdjustedTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, 4294970545, "clang::AdjustedTypeLoc::getLocalDataSize"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ADJUSTEDTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_LOADERUNINITIALIZEDATTR, "clang::LoaderUninitializedAttr");
  g.add_edge((CLASS_LOADERUNINITIALIZEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LOADERUNINITIALIZEDATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_LOADERUNINITIALIZEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPE, "clang::Type");
  g.add_edge((CLASS_TYPE, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970548, "clang::Type::getTypeClass"),
      add_named_node(&mut g, 4294970549, "clang::Type::isFromAST"),
      add_named_node(&mut g, 4294970550, "clang::Type::containsUnexpandedParameterPack"),
      add_named_node(&mut g, 4294970551, "clang::Type::isCanonicalUnqualified"),
      add_named_node(&mut g, 4294970552, "clang::Type::getLocallyUnqualifiedSingleStepDesugaredType"),
      add_named_node(&mut g, 4294970553, "clang::Type::isSizelessType"),
      add_named_node(&mut g, 4294970554, "clang::Type::isSizelessBuiltinType"),
      add_named_node(&mut g, 4294970555, "clang::Type::isSizelessVectorType"),
      add_named_node(&mut g, 4294970556, "clang::Type::isSVESizelessBuiltinType"),
      add_named_node(&mut g, 4294970557, "clang::Type::isRVVSizelessBuiltinType"),
      add_named_node(&mut g, 4294970558, "clang::Type::isWebAssemblyExternrefType"),
      add_named_node(&mut g, 4294970559, "clang::Type::isWebAssemblyTableType"),
      add_named_node(&mut g, 4294970560, "clang::Type::isSveVLSBuiltinType"),
      add_named_node(&mut g, 4294970561, "clang::Type::isRVVVLSBuiltinType"),
      add_named_node(&mut g, 4294970562, "clang::Type::isIncompleteOrObjectType"),
      add_named_node(&mut g, 4294970563, "clang::Type::isObjectType"),
      add_named_node(&mut g, 4294970564, "clang::Type::isStructuralType"),
      add_named_node(&mut g, 4294970565, "clang::Type::isStandardLayoutType"),
      add_named_node(&mut g, 4294970566, "clang::Type::isBuiltinType"),
      add_named_node(&mut g, 4294970567, "clang::Type::isPlaceholderType"),
      add_named_node(&mut g, 4294970568, "clang::Type::getAsPlaceholderType"),
      add_named_node(&mut g, 4294970569, "clang::Type::isNonOverloadPlaceholderType"),
      add_named_node(&mut g, 4294970570, "clang::Type::isIntegerType"),
      add_named_node(&mut g, 4294970571, "clang::Type::isEnumeralType"),
      add_named_node(&mut g, 4294970572, "clang::Type::isScopedEnumeralType"),
      add_named_node(&mut g, 4294970573, "clang::Type::isBooleanType"),
      add_named_node(&mut g, 4294970574, "clang::Type::isCharType"),
      add_named_node(&mut g, 4294970575, "clang::Type::isWideCharType"),
      add_named_node(&mut g, 4294970576, "clang::Type::isChar8Type"),
      add_named_node(&mut g, 4294970577, "clang::Type::isChar16Type"),
      add_named_node(&mut g, 4294970578, "clang::Type::isChar32Type"),
      add_named_node(&mut g, 4294970579, "clang::Type::isAnyCharacterType"),
      add_named_node(&mut g, 4294970580, "clang::Type::isIntegralOrEnumerationType"),
      add_named_node(&mut g, 4294970581, "clang::Type::isIntegralOrUnscopedEnumerationType"),
      add_named_node(&mut g, 4294970582, "clang::Type::isUnscopedEnumerationType"),
      add_named_node(&mut g, 4294970583, "clang::Type::isRealFloatingType"),
      add_named_node(&mut g, 4294970584, "clang::Type::isComplexType"),
      add_named_node(&mut g, 4294970585, "clang::Type::isAnyComplexType"),
      add_named_node(&mut g, 4294970586, "clang::Type::isFloatingType"),
      add_named_node(&mut g, 4294970587, "clang::Type::isHalfType"),
      add_named_node(&mut g, 4294970588, "clang::Type::isFloat16Type"),
      add_named_node(&mut g, 4294970589, "clang::Type::isBFloat16Type"),
      add_named_node(&mut g, 4294970590, "clang::Type::isFloat128Type"),
      add_named_node(&mut g, 4294970591, "clang::Type::isIbm128Type"),
      add_named_node(&mut g, 4294970592, "clang::Type::isRealType"),
      add_named_node(&mut g, 4294970593, "clang::Type::isArithmeticType"),
      add_named_node(&mut g, 4294970594, "clang::Type::isVoidType"),
      add_named_node(&mut g, 4294970595, "clang::Type::isScalarType"),
      add_named_node(&mut g, 4294970596, "clang::Type::isAggregateType"),
      add_named_node(&mut g, 4294970597, "clang::Type::isFundamentalType"),
      add_named_node(&mut g, 4294970598, "clang::Type::isCompoundType"),
      add_named_node(&mut g, 4294970599, "clang::Type::isFunctionType"),
      add_named_node(&mut g, 4294970600, "clang::Type::isFunctionNoProtoType"),
      add_named_node(&mut g, 4294970601, "clang::Type::isFunctionProtoType"),
      add_named_node(&mut g, 4294970602, "clang::Type::isPointerType"),
      add_named_node(&mut g, 4294970603, "clang::Type::isAnyPointerType"),
      add_named_node(&mut g, 4294970604, "clang::Type::isBlockPointerType"),
      add_named_node(&mut g, 4294970605, "clang::Type::isVoidPointerType"),
      add_named_node(&mut g, 4294970606, "clang::Type::isReferenceType"),
      add_named_node(&mut g, 4294970607, "clang::Type::isLValueReferenceType"),
      add_named_node(&mut g, 4294970608, "clang::Type::isRValueReferenceType"),
      add_named_node(&mut g, 4294970609, "clang::Type::isObjectPointerType"),
      add_named_node(&mut g, 4294970610, "clang::Type::isFunctionPointerType"),
      add_named_node(&mut g, 4294970611, "clang::Type::isFunctionReferenceType"),
      add_named_node(&mut g, 4294970612, "clang::Type::isMemberPointerType"),
      add_named_node(&mut g, 4294970613, "clang::Type::isMemberFunctionPointerType"),
      add_named_node(&mut g, 4294970614, "clang::Type::isMemberDataPointerType"),
      add_named_node(&mut g, 4294970615, "clang::Type::isArrayType"),
      add_named_node(&mut g, 4294970616, "clang::Type::isConstantArrayType"),
      add_named_node(&mut g, 4294970617, "clang::Type::isIncompleteArrayType"),
      add_named_node(&mut g, 4294970618, "clang::Type::isVariableArrayType"),
      add_named_node(&mut g, 4294970619, "clang::Type::isDependentSizedArrayType"),
      add_named_node(&mut g, 4294970620, "clang::Type::isRecordType"),
      add_named_node(&mut g, 4294970621, "clang::Type::isClassType"),
      add_named_node(&mut g, 4294970622, "clang::Type::isStructureType"),
      add_named_node(&mut g, 4294970623, "clang::Type::isObjCBoxableRecordType"),
      add_named_node(&mut g, 4294970624, "clang::Type::isInterfaceType"),
      add_named_node(&mut g, 4294970625, "clang::Type::isStructureOrClassType"),
      add_named_node(&mut g, 4294970626, "clang::Type::isUnionType"),
      add_named_node(&mut g, 4294970627, "clang::Type::isComplexIntegerType"),
      add_named_node(&mut g, 4294970628, "clang::Type::isVectorType"),
      add_named_node(&mut g, 4294970629, "clang::Type::isExtVectorType"),
      add_named_node(&mut g, 4294970630, "clang::Type::isExtVectorBoolType"),
      add_named_node(&mut g, 4294970631, "clang::Type::isMatrixType"),
      add_named_node(&mut g, 4294970632, "clang::Type::isConstantMatrixType"),
      add_named_node(&mut g, 4294970633, "clang::Type::isDependentAddressSpaceType"),
      add_named_node(&mut g, 4294970634, "clang::Type::isObjCObjectPointerType"),
      add_named_node(&mut g, 4294970635, "clang::Type::isObjCRetainableType"),
      add_named_node(&mut g, 4294970636, "clang::Type::isObjCLifetimeType"),
      add_named_node(&mut g, 4294970637, "clang::Type::isObjCIndirectLifetimeType"),
      add_named_node(&mut g, 4294970638, "clang::Type::isObjCNSObjectType"),
      add_named_node(&mut g, 4294970639, "clang::Type::isObjCIndependentClassType"),
      add_named_node(&mut g, 4294970640, "clang::Type::isObjCObjectType"),
      add_named_node(&mut g, 4294970641, "clang::Type::isObjCQualifiedInterfaceType"),
      add_named_node(&mut g, 4294970642, "clang::Type::isObjCQualifiedIdType"),
      add_named_node(&mut g, 4294970643, "clang::Type::isObjCQualifiedClassType"),
      add_named_node(&mut g, 4294970644, "clang::Type::isObjCObjectOrInterfaceType"),
      add_named_node(&mut g, 4294970645, "clang::Type::isObjCIdType"),
      add_named_node(&mut g, 4294970646, "clang::Type::isDecltypeType"),
      add_named_node(&mut g, 4294970647, "clang::Type::isObjCInertUnsafeUnretainedType"),
      add_named_node(&mut g, 4294970648, "clang::Type::isObjCClassType"),
      add_named_node(&mut g, 4294970649, "clang::Type::isObjCClassOrClassKindOfType"),
      add_named_node(&mut g, 4294970650, "clang::Type::isObjCSelType"),
      add_named_node(&mut g, 4294970651, "clang::Type::isObjCBuiltinType"),
      add_named_node(&mut g, 4294970652, "clang::Type::isObjCARCBridgableType"),
      add_named_node(&mut g, 4294970653, "clang::Type::isCARCBridgableType"),
      add_named_node(&mut g, 4294970654, "clang::Type::isTemplateTypeParmType"),
      add_named_node(&mut g, 4294970655, "clang::Type::isNullPtrType"),
      add_named_node(&mut g, 4294970656, "clang::Type::isNothrowT"),
      add_named_node(&mut g, 4294970657, "clang::Type::isAlignValT"),
      add_named_node(&mut g, 4294970658, "clang::Type::isStdByteType"),
      add_named_node(&mut g, 4294970659, "clang::Type::isAtomicType"),
      add_named_node(&mut g, 4294970660, "clang::Type::isUndeducedAutoType"),
      add_named_node(&mut g, 4294970661, "clang::Type::isTypedefNameType"),
      add_named_node(&mut g, 4294970662, "clang::Type::isOCLImage1dROType"),
      add_named_node(&mut g, 4294970663, "clang::Type::isOCLImage1dArrayROType"),
      add_named_node(&mut g, 4294970664, "clang::Type::isOCLImage1dBufferROType"),
      add_named_node(&mut g, 4294970665, "clang::Type::isOCLImage2dROType"),
      add_named_node(&mut g, 4294970666, "clang::Type::isOCLImage2dArrayROType"),
      add_named_node(&mut g, 4294970667, "clang::Type::isOCLImage2dDepthROType"),
      add_named_node(&mut g, 4294970668, "clang::Type::isOCLImage2dArrayDepthROType"),
      add_named_node(&mut g, 4294970669, "clang::Type::isOCLImage2dMSAAROType"),
      add_named_node(&mut g, 4294970670, "clang::Type::isOCLImage2dArrayMSAAROType"),
      add_named_node(&mut g, 4294970671, "clang::Type::isOCLImage2dMSAADepthROType"),
      add_named_node(&mut g, 4294970672, "clang::Type::isOCLImage2dArrayMSAADepthROType"),
      add_named_node(&mut g, 4294970673, "clang::Type::isOCLImage3dROType"),
      add_named_node(&mut g, 4294970674, "clang::Type::isOCLImage1dWOType"),
      add_named_node(&mut g, 4294970675, "clang::Type::isOCLImage1dArrayWOType"),
      add_named_node(&mut g, 4294970676, "clang::Type::isOCLImage1dBufferWOType"),
      add_named_node(&mut g, 4294970677, "clang::Type::isOCLImage2dWOType"),
      add_named_node(&mut g, 4294970678, "clang::Type::isOCLImage2dArrayWOType"),
      add_named_node(&mut g, 4294970679, "clang::Type::isOCLImage2dDepthWOType"),
      add_named_node(&mut g, 4294970680, "clang::Type::isOCLImage2dArrayDepthWOType"),
      add_named_node(&mut g, 4294970681, "clang::Type::isOCLImage2dMSAAWOType"),
      add_named_node(&mut g, 4294970682, "clang::Type::isOCLImage2dArrayMSAAWOType"),
      add_named_node(&mut g, 4294970683, "clang::Type::isOCLImage2dMSAADepthWOType"),
      add_named_node(&mut g, 4294970684, "clang::Type::isOCLImage2dArrayMSAADepthWOType"),
      add_named_node(&mut g, 4294970685, "clang::Type::isOCLImage3dWOType"),
      add_named_node(&mut g, 4294970686, "clang::Type::isOCLImage1dRWType"),
      add_named_node(&mut g, 4294970687, "clang::Type::isOCLImage1dArrayRWType"),
      add_named_node(&mut g, 4294970688, "clang::Type::isOCLImage1dBufferRWType"),
      add_named_node(&mut g, 4294970689, "clang::Type::isOCLImage2dRWType"),
      add_named_node(&mut g, 4294970690, "clang::Type::isOCLImage2dArrayRWType"),
      add_named_node(&mut g, 4294970691, "clang::Type::isOCLImage2dDepthRWType"),
      add_named_node(&mut g, 4294970692, "clang::Type::isOCLImage2dArrayDepthRWType"),
      add_named_node(&mut g, 4294970693, "clang::Type::isOCLImage2dMSAARWType"),
      add_named_node(&mut g, 4294970694, "clang::Type::isOCLImage2dArrayMSAARWType"),
      add_named_node(&mut g, 4294970695, "clang::Type::isOCLImage2dMSAADepthRWType"),
      add_named_node(&mut g, 4294970696, "clang::Type::isOCLImage2dArrayMSAADepthRWType"),
      add_named_node(&mut g, 4294970697, "clang::Type::isOCLImage3dRWType"),
      add_named_node(&mut g, 4294970698, "clang::Type::isImageType"),
      add_named_node(&mut g, 4294970699, "clang::Type::isSamplerT"),
      add_named_node(&mut g, 4294970700, "clang::Type::isEventT"),
      add_named_node(&mut g, 4294970701, "clang::Type::isClkEventT"),
      add_named_node(&mut g, 4294970702, "clang::Type::isQueueT"),
      add_named_node(&mut g, 4294970703, "clang::Type::isReserveIDT"),
      add_named_node(&mut g, 4294970704, "clang::Type::isOCLIntelSubgroupAVCMcePayloadType"),
      add_named_node(&mut g, 4294970705, "clang::Type::isOCLIntelSubgroupAVCImePayloadType"),
      add_named_node(&mut g, 4294970706, "clang::Type::isOCLIntelSubgroupAVCRefPayloadType"),
      add_named_node(&mut g, 4294970707, "clang::Type::isOCLIntelSubgroupAVCSicPayloadType"),
      add_named_node(&mut g, 4294970708, "clang::Type::isOCLIntelSubgroupAVCMceResultType"),
      add_named_node(&mut g, 4294970709, "clang::Type::isOCLIntelSubgroupAVCImeResultType"),
      add_named_node(&mut g, 4294970710, "clang::Type::isOCLIntelSubgroupAVCRefResultType"),
      add_named_node(&mut g, 4294970711, "clang::Type::isOCLIntelSubgroupAVCSicResultType"),
      add_named_node(&mut g, 4294970712, "clang::Type::isOCLIntelSubgroupAVCImeResultSingleReferenceStreamoutType"),
      add_named_node(&mut g, 4294970713, "clang::Type::isOCLIntelSubgroupAVCImeResultDualReferenceStreamoutType"),
      add_named_node(&mut g, 4294970714, "clang::Type::isOCLIntelSubgroupAVCImeSingleReferenceStreaminType"),
      add_named_node(&mut g, 4294970715, "clang::Type::isOCLIntelSubgroupAVCImeDualReferenceStreaminType"),
      add_named_node(&mut g, 4294970716, "clang::Type::isOCLIntelSubgroupAVCType"),
      add_named_node(&mut g, 4294970717, "clang::Type::isOCLExtOpaqueType"),
      add_named_node(&mut g, 4294970718, "clang::Type::isPipeType"),
      add_named_node(&mut g, 4294970719, "clang::Type::isBitIntType"),
      add_named_node(&mut g, 4294970720, "clang::Type::isOpenCLSpecificType"),
      add_named_node(&mut g, 4294970721, "clang::Type::isObjCARCImplicitlyUnretainedType"),
      add_named_node(&mut g, 4294970722, "clang::Type::isCUDADeviceBuiltinSurfaceType"),
      add_named_node(&mut g, 4294970723, "clang::Type::isCUDADeviceBuiltinTextureType"),
      add_named_node(&mut g, 4294970724, "clang::Type::getObjCARCImplicitLifetime"),
      add_named_node(&mut g, 4294970725, "clang::Type::getScalarTypeKind"),
      add_named_node(&mut g, 4294970726, "clang::Type::getDependence"),
      add_named_node(&mut g, 4294970727, "clang::Type::containsErrors"),
      add_named_node(&mut g, 4294970728, "clang::Type::isDependentType"),
      add_named_node(&mut g, 4294970729, "clang::Type::isInstantiationDependentType"),
      add_named_node(&mut g, 4294970730, "clang::Type::isUndeducedType"),
      add_named_node(&mut g, 4294970731, "clang::Type::isVariablyModifiedType"),
      add_named_node(&mut g, 4294970732, "clang::Type::hasSizedVLAType"),
      add_named_node(&mut g, 4294970733, "clang::Type::hasUnnamedOrLocalType"),
      add_named_node(&mut g, 4294970734, "clang::Type::isOverloadableType"),
      add_named_node(&mut g, 4294970735, "clang::Type::isElaboratedTypeSpecifier"),
      add_named_node(&mut g, 4294970736, "clang::Type::canDecayToPointerType"),
      add_named_node(&mut g, 4294970737, "clang::Type::hasPointerRepresentation"),
      add_named_node(&mut g, 4294970738, "clang::Type::hasObjCPointerRepresentation"),
      add_named_node(&mut g, 4294970739, "clang::Type::hasIntegerRepresentation"),
      add_named_node(&mut g, 4294970740, "clang::Type::hasSignedIntegerRepresentation"),
      add_named_node(&mut g, 4294970741, "clang::Type::hasUnsignedIntegerRepresentation"),
      add_named_node(&mut g, 4294970742, "clang::Type::hasFloatingRepresentation"),
      add_named_node(&mut g, 4294970743, "clang::Type::getAsStructureType"),
      add_named_node(&mut g, 4294970744, "clang::Type::getAsUnionType"),
      add_named_node(&mut g, 4294970745, "clang::Type::getAsComplexIntegerType"),
      add_named_node(&mut g, 4294970746, "clang::Type::getAsObjCInterfaceType"),
      add_named_node(&mut g, 4294970747, "clang::Type::getAsObjCInterfacePointerType"),
      add_named_node(&mut g, 4294970748, "clang::Type::getAsObjCQualifiedIdType"),
      add_named_node(&mut g, 4294970749, "clang::Type::getAsObjCQualifiedClassType"),
      add_named_node(&mut g, 4294970750, "clang::Type::getAsObjCQualifiedInterfaceType"),
      add_named_node(&mut g, 4294970751, "clang::Type::getAsCXXRecordDecl"),
      add_named_node(&mut g, 4294970752, "clang::Type::getAsRecordDecl"),
      add_named_node(&mut g, 4294970753, "clang::Type::getAsTagDecl"),
      add_named_node(&mut g, 4294970754, "clang::Type::getPointeeCXXRecordDecl"),
      add_named_node(&mut g, 4294970755, "clang::Type::getContainedDeducedType"),
      add_named_node(&mut g, 4294970756, "clang::Type::getContainedAutoType"),
      add_named_node(&mut g, 4294970757, "clang::Type::hasAutoForTrailingReturnType"),
      add_named_node(&mut g, 4294970758, "clang::Type::getAsArrayTypeUnsafe"),
      add_named_node(&mut g, 4294970759, "clang::Type::castAsArrayTypeUnsafe"),
      add_named_node(&mut g, 4294970760, "clang::Type::getBaseElementTypeUnsafe"),
      add_named_node(&mut g, 4294970761, "clang::Type::getArrayElementTypeNoTypeQual"),
      add_named_node(&mut g, 4294970762, "clang::Type::getPointeeOrArrayElementType"),
      add_named_node(&mut g, 4294970763, "clang::Type::getPointeeType"),
      add_named_node(&mut g, 4294970764, "clang::Type::getUnqualifiedDesugaredType"),
      add_named_node(&mut g, 4294970765, "clang::Type::isSignedIntegerType"),
      add_named_node(&mut g, 4294970766, "clang::Type::isUnsignedIntegerType"),
      add_named_node(&mut g, 4294970767, "clang::Type::isSignedIntegerOrEnumerationType"),
      add_named_node(&mut g, 4294970768, "clang::Type::isUnsignedIntegerOrEnumerationType"),
      add_named_node(&mut g, 4294970769, "clang::Type::isFixedPointType"),
      add_named_node(&mut g, 4294970770, "clang::Type::isFixedPointOrIntegerType"),
      add_named_node(&mut g, 4294970771, "clang::Type::isSaturatedFixedPointType"),
      add_named_node(&mut g, 4294970772, "clang::Type::isUnsaturatedFixedPointType"),
      add_named_node(&mut g, 4294970773, "clang::Type::isSignedFixedPointType"),
      add_named_node(&mut g, 4294970774, "clang::Type::isUnsignedFixedPointType"),
      add_named_node(&mut g, 4294970775, "clang::Type::isConstantSizeType"),
      add_named_node(&mut g, 4294970776, "clang::Type::isSpecifierType"),
      add_named_node(&mut g, 4294970777, "clang::Type::getLinkage"),
      add_named_node(&mut g, 4294970778, "clang::Type::getVisibility"),
      add_named_node(&mut g, 4294970779, "clang::Type::isVisibilityExplicit"),
      add_named_node(&mut g, 4294970780, "clang::Type::getLinkageAndVisibility"),
      add_named_node(&mut g, 4294970781, "clang::Type::isLinkageValid"),
      add_named_node(&mut g, 4294970782, "clang::Type::getNullability"),
      add_named_node(&mut g, 4294970783, "clang::Type::acceptsObjCTypeParams"),
      add_named_node(&mut g, 4294970784, "clang::Type::getTypeClassName"),
      add_named_node(&mut g, 4294970785, "clang::Type::getCanonicalTypeInternal"),
      add_named_node(&mut g, 4294970786, "clang::Type::getCanonicalTypeUnqualified"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_AVRSIGNALATTR, "clang::AVRSignalAttr");
  g.add_edge((CLASS_AVRSIGNALATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_AVRSIGNALATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_AVRSIGNALATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TAGTYPE, "clang::TagType");
  g.add_edge((CLASS_TAGTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TAGTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970789, "clang::TagType::getDecl"),
      add_named_node(&mut g, 4294970790, "clang::TagType::isBeingDefined"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TAGTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPEOFTYPE, "clang::TypeOfType");
  g.add_edge((CLASS_TYPEOFTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPEOFTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970792, "clang::TypeOfType::getUnmodifiedType"),
      add_named_node(&mut g, 4294970793, "clang::TypeOfType::desugar"),
      add_named_node(&mut g, 4294970794, "clang::TypeOfType::isSugared"),
      add_named_node(&mut g, 4294970795, "clang::TypeOfType::getKind"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPEOFTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BITINTTYPE, "clang::BitIntType");
  g.add_edge((CLASS_BITINTTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BITINTTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970797, "clang::BitIntType::isUnsigned"),
      add_named_node(&mut g, 4294970798, "clang::BitIntType::isSigned"),
      add_named_node(&mut g, 4294970799, "clang::BitIntType::getNumBits"),
      add_named_node(&mut g, 4294970800, "clang::BitIntType::isSugared"),
      add_named_node(&mut g, 4294970801, "clang::BitIntType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BITINTTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPEWITHKEYWORD, "clang::TypeWithKeyword");
  g.add_edge((CLASS_TYPEWITHKEYWORD, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPEWITHKEYWORD, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970803, "clang::TypeWithKeyword::getKeyword"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPEWITHKEYWORD, META_METHOD, methods));
  }

  g.add_named_node(CLASS_REINITIALIZESATTR, "clang::ReinitializesAttr");
  g.add_edge((CLASS_REINITIALIZESATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_REINITIALIZESATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_REINITIALIZESATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_AUTOTYPE, "clang::AutoType");
  g.add_edge((CLASS_AUTOTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_AUTOTYPE, META_SUBCLASS, CLASS_DEDUCEDTYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970806, "clang::AutoType::getTypeConstraintArguments"),
      add_named_node(&mut g, 4294970807, "clang::AutoType::getTypeConstraintConcept"),
      add_named_node(&mut g, 4294970808, "clang::AutoType::isConstrained"),
      add_named_node(&mut g, 4294970809, "clang::AutoType::isDecltypeAuto"),
      add_named_node(&mut g, 4294970810, "clang::AutoType::isGNUAutoType"),
      add_named_node(&mut g, 4294970811, "clang::AutoType::getKeyword"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_AUTOTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTTEMPLATESPECIALIZATIONTYPE, "clang::DependentTemplateSpecializationType");
  g.add_edge((CLASS_DEPENDENTTEMPLATESPECIALIZATIONTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEPENDENTTEMPLATESPECIALIZATIONTYPE, META_SUBCLASS, CLASS_TYPEWITHKEYWORD));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970813, "clang::DependentTemplateSpecializationType::getQualifier"),
      add_named_node(&mut g, 4294970814, "clang::DependentTemplateSpecializationType::getIdentifier"),
      add_named_node(&mut g, 4294970815, "clang::DependentTemplateSpecializationType::template_arguments"),
      add_named_node(&mut g, 4294970816, "clang::DependentTemplateSpecializationType::isSugared"),
      add_named_node(&mut g, 4294970817, "clang::DependentTemplateSpecializationType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTTEMPLATESPECIALIZATIONTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PACKEXPANSIONTYPE, "clang::PackExpansionType");
  g.add_edge((CLASS_PACKEXPANSIONTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PACKEXPANSIONTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970819, "clang::PackExpansionType::getPattern"),
      add_named_node(&mut g, 4294970820, "clang::PackExpansionType::getNumExpansions"),
      add_named_node(&mut g, 4294970821, "clang::PackExpansionType::isSugared"),
      add_named_node(&mut g, 4294970822, "clang::PackExpansionType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PACKEXPANSIONTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_STDCALLATTR, "clang::StdCallAttr");
  g.add_edge((CLASS_STDCALLATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_STDCALLATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_STDCALLATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCRUNTIMENAMEATTR, "clang::ObjCRuntimeNameAttr");
  g.add_edge((CLASS_OBJCRUNTIMENAMEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCRUNTIMENAMEATTR, META_SUBCLASS, CLASS_ATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCRUNTIMENAMEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEDUCEDTEMPLATESPECIALIZATIONTYPE, "clang::DeducedTemplateSpecializationType");
  g.add_edge((CLASS_DEDUCEDTEMPLATESPECIALIZATIONTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEDUCEDTEMPLATESPECIALIZATIONTYPE, META_SUBCLASS, CLASS_DEDUCEDTYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970826, "clang::DeducedTemplateSpecializationType::getTemplateName"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEDUCEDTEMPLATESPECIALIZATIONTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_USINGIFEXISTSATTR, "clang::UsingIfExistsAttr");
  g.add_edge((CLASS_USINGIFEXISTSATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_USINGIFEXISTSATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_USINGIFEXISTSATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTUNARYTRANSFORMTYPE, "clang::DependentUnaryTransformType");
  g.add_edge((CLASS_DEPENDENTUNARYTRANSFORMTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEPENDENTUNARYTRANSFORMTYPE, META_SUBCLASS, CLASS_UNARYTRANSFORMTYPE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTUNARYTRANSFORMTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_UNARYOPERATOR, "clang::UnaryOperator");
  g.add_edge((CLASS_UNARYOPERATOR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNARYOPERATOR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970830, "clang::UnaryOperator::getOpcode"),
      add_named_node(&mut g, 4294970831, "clang::UnaryOperator::getSubExpr"),
      add_named_node(&mut g, 4294970832, "clang::UnaryOperator::getOperatorLoc"),
      add_named_node(&mut g, 4294970833, "clang::UnaryOperator::canOverflow"),
      add_named_node(&mut g, 4294970834, "clang::UnaryOperator::isPrefix"),
      add_named_node(&mut g, 4294970835, "clang::UnaryOperator::isPostfix"),
      add_named_node(&mut g, 4294970836, "clang::UnaryOperator::isIncrementOp"),
      add_named_node(&mut g, 4294970837, "clang::UnaryOperator::isDecrementOp"),
      add_named_node(&mut g, 4294970838, "clang::UnaryOperator::isIncrementDecrementOp"),
      add_named_node(&mut g, 4294970839, "clang::UnaryOperator::isArithmeticOp"),
      add_named_node(&mut g, 4294970840, "clang::UnaryOperator::getBeginLoc"),
      add_named_node(&mut g, 4294970841, "clang::UnaryOperator::getEndLoc"),
      add_named_node(&mut g, 4294970842, "clang::UnaryOperator::getExprLoc"),
      add_named_node(&mut g, 4294970843, "clang::UnaryOperator::children"),
      add_named_node(&mut g, 4294970844, "clang::UnaryOperator::hasStoredFPFeatures"),
      add_named_node(&mut g, 4294970845, "clang::UnaryOperator::getStoredFPFeatures"),
      add_named_node(&mut g, 4294970846, "clang::UnaryOperator::getFPOptionsOverride"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNARYOPERATOR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_EXTVECTORTYPE, "clang::ExtVectorType");
  g.add_edge((CLASS_EXTVECTORTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_EXTVECTORTYPE, META_SUBCLASS, CLASS_VECTORTYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970848, "clang::ExtVectorType::isSugared"),
      add_named_node(&mut g, 4294970849, "clang::ExtVectorType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_EXTVECTORTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTDECLTYPETYPE, "clang::DependentDecltypeType");
  g.add_edge((CLASS_DEPENDENTDECLTYPETYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEPENDENTDECLTYPETYPE, META_SUBCLASS, CLASS_DECLTYPETYPE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTDECLTYPETYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NOTTAILCALLEDATTR, "clang::NotTailCalledAttr");
  g.add_edge((CLASS_NOTTAILCALLEDATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NOTTAILCALLEDATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NOTTAILCALLEDATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_LVALUEREFERENCETYPE, "clang::LValueReferenceType");
  g.add_edge((CLASS_LVALUEREFERENCETYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LVALUEREFERENCETYPE, META_SUBCLASS, CLASS_REFERENCETYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970853, "clang::LValueReferenceType::isSugared"),
      add_named_node(&mut g, 4294970854, "clang::LValueReferenceType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_LVALUEREFERENCETYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_UNRESOLVEDUSINGTYPE, "clang::UnresolvedUsingType");
  g.add_edge((CLASS_UNRESOLVEDUSINGTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNRESOLVEDUSINGTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970856, "clang::UnresolvedUsingType::getDecl"),
      add_named_node(&mut g, 4294970857, "clang::UnresolvedUsingType::isSugared"),
      add_named_node(&mut g, 4294970858, "clang::UnresolvedUsingType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNRESOLVEDUSINGTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTSIZEDMATRIXTYPE, "clang::DependentSizedMatrixType");
  g.add_edge((CLASS_DEPENDENTSIZEDMATRIXTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEPENDENTSIZEDMATRIXTYPE, META_SUBCLASS, CLASS_MATRIXTYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970860, "clang::DependentSizedMatrixType::getRowExpr"),
      add_named_node(&mut g, 4294970861, "clang::DependentSizedMatrixType::getColumnExpr"),
      add_named_node(&mut g, 4294970862, "clang::DependentSizedMatrixType::getAttributeLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTSIZEDMATRIXTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CONSTANTMATRIXTYPE, "clang::ConstantMatrixType");
  g.add_edge((CLASS_CONSTANTMATRIXTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CONSTANTMATRIXTYPE, META_SUBCLASS, CLASS_MATRIXTYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970864, "clang::ConstantMatrixType::getNumRows"),
      add_named_node(&mut g, 4294970865, "clang::ConstantMatrixType::getNumColumns"),
      add_named_node(&mut g, 4294970866, "clang::ConstantMatrixType::getNumElementsFlattened"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CONSTANTMATRIXTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCCATEGORYIMPLDECL, "clang::ObjCCategoryImplDecl");
  g.add_edge((CLASS_OBJCCATEGORYIMPLDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCCATEGORYIMPLDECL, META_SUBCLASS, CLASS_OBJCIMPLDECL));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCCATEGORYIMPLDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_VECTORTYPE, "clang::VectorType");
  g.add_edge((CLASS_VECTORTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_VECTORTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970869, "clang::VectorType::getElementType"),
      add_named_node(&mut g, 4294970870, "clang::VectorType::getNumElements"),
      add_named_node(&mut g, 4294970871, "clang::VectorType::isSugared"),
      add_named_node(&mut g, 4294970872, "clang::VectorType::desugar"),
      add_named_node(&mut g, 4294970873, "clang::VectorType::getVectorKind"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_VECTORTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTADDRESSSPACETYPE, "clang::DependentAddressSpaceType");
  g.add_edge((CLASS_DEPENDENTADDRESSSPACETYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEPENDENTADDRESSSPACETYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970875, "clang::DependentAddressSpaceType::getAddrSpaceExpr"),
      add_named_node(&mut g, 4294970876, "clang::DependentAddressSpaceType::getPointeeType"),
      add_named_node(&mut g, 4294970877, "clang::DependentAddressSpaceType::getAttributeLoc"),
      add_named_node(&mut g, 4294970878, "clang::DependentAddressSpaceType::isSugared"),
      add_named_node(&mut g, 4294970879, "clang::DependentAddressSpaceType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTADDRESSSPACETYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_RETURNSTMT, "clang::ReturnStmt");
  g.add_edge((CLASS_RETURNSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_RETURNSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970881, "clang::ReturnStmt::getRetValue"),
      add_named_node(&mut g, 4294970882, "clang::ReturnStmt::getNRVOCandidate"),
      add_named_node(&mut g, 4294970883, "clang::ReturnStmt::getReturnLoc"),
      add_named_node(&mut g, 4294970884, "clang::ReturnStmt::getBeginLoc"),
      add_named_node(&mut g, 4294970885, "clang::ReturnStmt::getEndLoc"),
      add_named_node(&mut g, 4294970886, "clang::ReturnStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_RETURNSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DECAYEDTYPE, "clang::DecayedType");
  g.add_edge((CLASS_DECAYEDTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DECAYEDTYPE, META_SUBCLASS, CLASS_ADJUSTEDTYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970888, "clang::DecayedType::getDecayedType"),
      add_named_node(&mut g, 4294970889, "clang::DecayedType::getPointeeType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DECAYEDTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCCATEGORYDECL, "clang::ObjCCategoryDecl");
  g.add_edge((CLASS_OBJCCATEGORYDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OBJCCATEGORYDECL, META_SUBCLASS, CLASS_OBJCCONTAINERDECL));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCCATEGORYDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MEMBERPOINTERTYPE, "clang::MemberPointerType");
  g.add_edge((CLASS_MEMBERPOINTERTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MEMBERPOINTERTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970892, "clang::MemberPointerType::getPointeeType"),
      add_named_node(&mut g, 4294970893, "clang::MemberPointerType::isMemberFunctionPointer"),
      add_named_node(&mut g, 4294970894, "clang::MemberPointerType::isMemberDataPointer"),
      add_named_node(&mut g, 4294970895, "clang::MemberPointerType::getClass"),
      add_named_node(&mut g, 4294970896, "clang::MemberPointerType::getMostRecentCXXRecordDecl"),
      add_named_node(&mut g, 4294970897, "clang::MemberPointerType::isSugared"),
      add_named_node(&mut g, 4294970898, "clang::MemberPointerType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MEMBERPOINTERTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_INJECTEDCLASSNAMETYPELOC, "clang::InjectedClassNameTypeLoc");
  g.add_edge((CLASS_INJECTEDCLASSNAMETYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970900, "clang::InjectedClassNameTypeLoc::getDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_INJECTEDCLASSNAMETYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PIPETYPELOC, "clang::PipeTypeLoc");
  g.add_edge((CLASS_PIPETYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970902, "clang::PipeTypeLoc::getValueLoc"),
      add_named_node(&mut g, 4294970903, "clang::PipeTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, 4294970904, "clang::PipeTypeLoc::getKWLoc"),
      add_named_node(&mut g, 4294970905, "clang::PipeTypeLoc::getInnerType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PIPETYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTVECTORTYPELOC, "clang::DependentVectorTypeLoc");
  g.add_edge((CLASS_DEPENDENTVECTORTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970907, "clang::DependentVectorTypeLoc::getNameLoc"),
      add_named_node(&mut g, 4294970908, "clang::DependentVectorTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, 4294970909, "clang::DependentVectorTypeLoc::getElementLoc"),
      add_named_node(&mut g, 4294970910, "clang::DependentVectorTypeLoc::getInnerType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTVECTORTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPEALIASDECL, "clang::TypeAliasDecl");
  g.add_edge((CLASS_TYPEALIASDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TYPEALIASDECL, META_SUBCLASS, CLASS_TYPEDEFNAMEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970912, "clang::TypeAliasDecl::getSourceRange"),
      add_named_node(&mut g, 4294970913, "clang::TypeAliasDecl::getDescribedAliasTemplate"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPEALIASDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CONSTANTMATRIXTYPELOC, "clang::ConstantMatrixTypeLoc");
  g.add_edge((CLASS_CONSTANTMATRIXTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CONSTANTMATRIXTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NSERRORDOMAINATTR, "clang::NSErrorDomainAttr");
  g.add_edge((CLASS_NSERRORDOMAINATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NSERRORDOMAINATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NSERRORDOMAINATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_POINTERTYPELOC, "clang::PointerTypeLoc");
  g.add_edge((CLASS_POINTERTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970917, "clang::PointerTypeLoc::getStarLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_POINTERTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ANNOTATEATTR, "clang::AnnotateAttr");
  g.add_edge((CLASS_ANNOTATEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ANNOTATEATTR, META_SUBCLASS, CLASS_INHERITABLEPARAMATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ANNOTATEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEDUCEDTYPELOC, "clang::DeducedTypeLoc");
  g.add_edge((CLASS_DEDUCEDTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEDUCEDTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPEOFTYPELOC, "clang::TypeOfTypeLoc");
  g.add_edge((CLASS_TYPEOFTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970921, "clang::TypeOfTypeLoc::getUnmodifiedType"),
      add_named_node(&mut g, 4294970922, "clang::TypeOfTypeLoc::getUnmodifiedTInfo"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPEOFTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TAGDECL, "clang::TagDecl");
  g.add_edge((CLASS_TAGDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_TAGDECL, META_SUBCLASS, CLASS_TYPEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970924, "clang::TagDecl::getBraceRange"),
      add_named_node(&mut g, 4294970925, "clang::TagDecl::getInnerLocStart"),
      add_named_node(&mut g, 4294970926, "clang::TagDecl::getOuterLocStart"),
      add_named_node(&mut g, 4294970927, "clang::TagDecl::getSourceRange"),
      add_named_node(&mut g, 4294970928, "clang::TagDecl::getCanonicalDecl"),
      add_named_node(&mut g, 4294970929, "clang::TagDecl::isThisDeclarationADefinition"),
      add_named_node(&mut g, 4294970930, "clang::TagDecl::isCompleteDefinition"),
      add_named_node(&mut g, 4294970931, "clang::TagDecl::isCompleteDefinitionRequired"),
      add_named_node(&mut g, 4294970932, "clang::TagDecl::isBeingDefined"),
      add_named_node(&mut g, 4294970933, "clang::TagDecl::isEmbeddedInDeclarator"),
      add_named_node(&mut g, 4294970934, "clang::TagDecl::isFreeStanding"),
      add_named_node(&mut g, 4294970935, "clang::TagDecl::mayHaveOutOfDateDef"),
      add_named_node(&mut g, 4294970936, "clang::TagDecl::isDependentType"),
      add_named_node(&mut g, 4294970937, "clang::TagDecl::isThisDeclarationADemotedDefinition"),
      add_named_node(&mut g, 4294970938, "clang::TagDecl::getDefinition"),
      add_named_node(&mut g, 4294970939, "clang::TagDecl::getKindName"),
      add_named_node(&mut g, 4294970940, "clang::TagDecl::getTagKind"),
      add_named_node(&mut g, 4294970941, "clang::TagDecl::isStruct"),
      add_named_node(&mut g, 4294970942, "clang::TagDecl::isInterface"),
      add_named_node(&mut g, 4294970943, "clang::TagDecl::isClass"),
      add_named_node(&mut g, 4294970944, "clang::TagDecl::isUnion"),
      add_named_node(&mut g, 4294970945, "clang::TagDecl::isEnum"),
      add_named_node(&mut g, 4294970946, "clang::TagDecl::hasNameForLinkage"),
      add_named_node(&mut g, 4294970947, "clang::TagDecl::getTypedefNameForAnonDecl"),
      add_named_node(&mut g, 4294970948, "clang::TagDecl::getQualifier"),
      add_named_node(&mut g, 4294970949, "clang::TagDecl::getQualifierLoc"),
      add_named_node(&mut g, 4294970950, "clang::TagDecl::getNumTemplateParameterLists"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TAGDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTNAMETYPELOC, "clang::DependentNameTypeLoc");
  g.add_edge((CLASS_DEPENDENTNAMETYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970952, "clang::DependentNameTypeLoc::getElaboratedKeywordLoc"),
      add_named_node(&mut g, 4294970953, "clang::DependentNameTypeLoc::getQualifierLoc"),
      add_named_node(&mut g, 4294970954, "clang::DependentNameTypeLoc::getNameLoc"),
      add_named_node(&mut g, 4294970955, "clang::DependentNameTypeLoc::getLocalSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTNAMETYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CXXREINTERPRETCASTEXPR, "clang::CXXReinterpretCastExpr");
  g.add_edge((CLASS_CXXREINTERPRETCASTEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CXXREINTERPRETCASTEXPR, META_SUBCLASS, CLASS_CXXNAMEDCASTEXPR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CXXREINTERPRETCASTEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ARMINTERRUPTATTR, "clang::ARMInterruptAttr");
  g.add_edge((CLASS_ARMINTERRUPTATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARMINTERRUPTATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ARMINTERRUPTATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTSIZEDEXTVECTORTYPELOC, "clang::DependentSizedExtVectorTypeLoc");
  g.add_edge((CLASS_DEPENDENTSIZEDEXTVECTORTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970959, "clang::DependentSizedExtVectorTypeLoc::getNameLoc"),
      add_named_node(&mut g, 4294970960, "clang::DependentSizedExtVectorTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, 4294970961, "clang::DependentSizedExtVectorTypeLoc::getElementLoc"),
      add_named_node(&mut g, 4294970962, "clang::DependentSizedExtVectorTypeLoc::getInnerType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTSIZEDEXTVECTORTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCOBJECTPOINTERTYPELOC, "clang::ObjCObjectPointerTypeLoc");
  g.add_edge((CLASS_OBJCOBJECTPOINTERTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCOBJECTPOINTERTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FUNCTIONPROTOTYPELOC, "clang::FunctionProtoTypeLoc");
  g.add_edge((CLASS_FUNCTIONPROTOTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FUNCTIONPROTOTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CAPABILITYATTR, "clang::CapabilityAttr");
  g.add_edge((CLASS_CAPABILITYATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CAPABILITYATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CAPABILITYATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_UNRESOLVEDUSINGTYPELOC, "clang::UnresolvedUsingTypeLoc");
  g.add_edge((CLASS_UNRESOLVEDUSINGTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970967, "clang::UnresolvedUsingTypeLoc::getDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNRESOLVEDUSINGTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_QUALIFIEDTYPELOC, "clang::QualifiedTypeLoc");
  g.add_edge((CLASS_QUALIFIEDTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_QUALIFIEDTYPELOC, META_SUBCLASS, CLASS_TYPELOC));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970969, "clang::QualifiedTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, 4294970970, "clang::QualifiedTypeLoc::getUnqualifiedLoc"),
      add_named_node(&mut g, 4294970971, "clang::QualifiedTypeLoc::getNextTypeLoc"),
      add_named_node(&mut g, 4294970972, "clang::QualifiedTypeLoc::getLocalDataSize"),
      add_named_node(&mut g, 4294970973, "clang::QualifiedTypeLoc::getLocalDataAlignment"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_QUALIFIEDTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ALIGNMAC68KATTR, "clang::AlignMac68kAttr");
  g.add_edge((CLASS_ALIGNMAC68KATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ALIGNMAC68KATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ALIGNMAC68KATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BTFTAGATTRIBUTEDTYPELOC, "clang::BTFTagAttributedTypeLoc");
  g.add_edge((CLASS_BTFTAGATTRIBUTEDTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970976, "clang::BTFTagAttributedTypeLoc::getWrappedLoc"),
      add_named_node(&mut g, 4294970977, "clang::BTFTagAttributedTypeLoc::getAttr"),
      add_named_node(&mut g, 4294970978, "clang::BTFTagAttributedTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, 4294970979, "clang::BTFTagAttributedTypeLoc::getInnerType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BTFTAGATTRIBUTEDTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NAMEDDECL, "clang::NamedDecl");
  g.add_edge((CLASS_NAMEDDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NAMEDDECL, META_SUBCLASS, CLASS_DECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294970981, "clang::NamedDecl::getIdentifier"),
      add_named_node(&mut g, 4294970982, "clang::NamedDecl::getName"),
      add_named_node(&mut g, 4294970983, "clang::NamedDecl::getNameAsString"),
      add_named_node(&mut g, 4294970984, "clang::NamedDecl::getDeclName"),
      add_named_node(&mut g, 4294970985, "clang::NamedDecl::getQualifiedNameAsString"),
      add_named_node(&mut g, 4294970986, "clang::NamedDecl::hasLinkage"),
      add_named_node(&mut g, 4294970987, "clang::NamedDecl::isCXXClassMember"),
      add_named_node(&mut g, 4294970988, "clang::NamedDecl::isCXXInstanceMember"),
      add_named_node(&mut g, 4294970989, "clang::NamedDecl::getLinkageInternal"),
      add_named_node(&mut g, 4294970990, "clang::NamedDecl::getFormalLinkage"),
      add_named_node(&mut g, 4294970991, "clang::NamedDecl::hasExternalFormalLinkage"),
      add_named_node(&mut g, 4294970992, "clang::NamedDecl::isExternallyVisible"),
      add_named_node(&mut g, 4294970993, "clang::NamedDecl::isExternallyDeclarable"),
      add_named_node(&mut g, 4294970994, "clang::NamedDecl::getVisibility"),
      add_named_node(&mut g, 4294970995, "clang::NamedDecl::getLinkageAndVisibility"),
      add_named_node(&mut g, 4294970996, "clang::NamedDecl::isLinkageValid"),
      add_named_node(&mut g, 4294970997, "clang::NamedDecl::hasLinkageBeenComputed"),
      add_named_node(&mut g, 4294970998, "clang::NamedDecl::getUnderlyingDecl"),
      add_named_node(&mut g, 4294970999, "clang::NamedDecl::getMostRecentDecl"),
      add_named_node(&mut g, 4294971000, "clang::NamedDecl::getObjCFStringFormattingFamily"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NAMEDDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FUNCTIONTYPELOC, "clang::FunctionTypeLoc");
  g.add_edge((CLASS_FUNCTIONTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294971002, "clang::FunctionTypeLoc::getLocalRangeBegin"),
      add_named_node(&mut g, 4294971003, "clang::FunctionTypeLoc::getLocalRangeEnd"),
      add_named_node(&mut g, 4294971004, "clang::FunctionTypeLoc::getLParenLoc"),
      add_named_node(&mut g, 4294971005, "clang::FunctionTypeLoc::getRParenLoc"),
      add_named_node(&mut g, 4294971006, "clang::FunctionTypeLoc::getParensRange"),
      add_named_node(&mut g, 4294971007, "clang::FunctionTypeLoc::getExceptionSpecRange"),
      add_named_node(&mut g, 4294971008, "clang::FunctionTypeLoc::getParams"),
      add_named_node(&mut g, 4294971009, "clang::FunctionTypeLoc::getParmArray"),
      add_named_node(&mut g, 4294971010, "clang::FunctionTypeLoc::getNumParams"),
      add_named_node(&mut g, 4294971011, "clang::FunctionTypeLoc::getReturnLoc"),
      add_named_node(&mut g, 4294971012, "clang::FunctionTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, 4294971013, "clang::FunctionTypeLoc::getExtraLocalDataSize"),
      add_named_node(&mut g, 4294971014, "clang::FunctionTypeLoc::getExtraLocalDataAlignment"),
      add_named_node(&mut g, 4294971015, "clang::FunctionTypeLoc::getInnerType"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FUNCTIONTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SWIFTASYNCNAMEATTR, "clang::SwiftAsyncNameAttr");
  g.add_edge((CLASS_SWIFTASYNCNAMEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTASYNCNAMEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SWIFTASYNCNAMEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTTEMPLATESPECIALIZATIONTYPELOC, "clang::DependentTemplateSpecializationTypeLoc");
  g.add_edge((CLASS_DEPENDENTTEMPLATESPECIALIZATIONTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294971018, "clang::DependentTemplateSpecializationTypeLoc::getElaboratedKeywordLoc"),
      add_named_node(&mut g, 4294971019, "clang::DependentTemplateSpecializationTypeLoc::getQualifierLoc"),
      add_named_node(&mut g, 4294971020, "clang::DependentTemplateSpecializationTypeLoc::getTemplateKeywordLoc"),
      add_named_node(&mut g, 4294971021, "clang::DependentTemplateSpecializationTypeLoc::getTemplateNameLoc"),
      add_named_node(&mut g, 4294971022, "clang::DependentTemplateSpecializationTypeLoc::getLAngleLoc"),
      add_named_node(&mut g, 4294971023, "clang::DependentTemplateSpecializationTypeLoc::getRAngleLoc"),
      add_named_node(&mut g, 4294971024, "clang::DependentTemplateSpecializationTypeLoc::getNumArgs"),
      add_named_node(&mut g, 4294971025, "clang::DependentTemplateSpecializationTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, 4294971026, "clang::DependentTemplateSpecializationTypeLoc::getExtraLocalDataSize"),
      add_named_node(&mut g, 4294971027, "clang::DependentTemplateSpecializationTypeLoc::getExtraLocalDataAlignment"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTTEMPLATESPECIALIZATIONTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPDISPATCHDIRECTIVE, "clang::OMPDispatchDirective");
  g.add_edge((CLASS_OMPDISPATCHDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPDISPATCHDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPDISPATCHDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SWIFTNAMEATTR, "clang::SwiftNameAttr");
  g.add_edge((CLASS_SWIFTNAMEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTNAMEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SWIFTNAMEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEPENDENTSIZEDARRAYTYPE, "clang::DependentSizedArrayType");
  g.add_edge((CLASS_DEPENDENTSIZEDARRAYTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DEPENDENTSIZEDARRAYTYPE, META_SUBCLASS, CLASS_ARRAYTYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294971031, "clang::DependentSizedArrayType::getSizeExpr"),
      add_named_node(&mut g, 4294971032, "clang::DependentSizedArrayType::getBracketsRange"),
      add_named_node(&mut g, 4294971033, "clang::DependentSizedArrayType::getLBracketLoc"),
      add_named_node(&mut g, 4294971034, "clang::DependentSizedArrayType::getRBracketLoc"),
      add_named_node(&mut g, 4294971035, "clang::DependentSizedArrayType::isSugared"),
      add_named_node(&mut g, 4294971036, "clang::DependentSizedArrayType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEPENDENTSIZEDARRAYTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_VARIABLEARRAYTYPELOC, "clang::VariableArrayTypeLoc");
  g.add_edge((CLASS_VARIABLEARRAYTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_VARIABLEARRAYTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_FUNCTIONNOPROTOTYPELOC, "clang::FunctionNoProtoTypeLoc");
  g.add_edge((CLASS_FUNCTIONNOPROTOTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_FUNCTIONNOPROTOTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MSDEPENDENTEXISTSSTMT, "clang::MSDependentExistsStmt");
  g.add_edge((CLASS_MSDEPENDENTEXISTSSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MSDEPENDENTEXISTSSTMT, META_SUBCLASS, CLASS_STMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294971040, "clang::MSDependentExistsStmt::getKeywordLoc"),
      add_named_node(&mut g, 4294971041, "clang::MSDependentExistsStmt::isIfExists"),
      add_named_node(&mut g, 4294971042, "clang::MSDependentExistsStmt::isIfNotExists"),
      add_named_node(&mut g, 4294971043, "clang::MSDependentExistsStmt::getQualifierLoc"),
      add_named_node(&mut g, 4294971044, "clang::MSDependentExistsStmt::getNameInfo"),
      add_named_node(&mut g, 4294971045, "clang::MSDependentExistsStmt::getSubStmt"),
      add_named_node(&mut g, 4294971046, "clang::MSDependentExistsStmt::getBeginLoc"),
      add_named_node(&mut g, 4294971047, "clang::MSDependentExistsStmt::getEndLoc"),
      add_named_node(&mut g, 4294971048, "clang::MSDependentExistsStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MSDEPENDENTEXISTSSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_UNARYTRANSFORMTYPELOC, "clang::UnaryTransformTypeLoc");
  g.add_edge((CLASS_UNARYTRANSFORMTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294971050, "clang::UnaryTransformTypeLoc::getKWLoc"),
      add_named_node(&mut g, 4294971051, "clang::UnaryTransformTypeLoc::getLParenLoc"),
      add_named_node(&mut g, 4294971052, "clang::UnaryTransformTypeLoc::getRParenLoc"),
      add_named_node(&mut g, 4294971053, "clang::UnaryTransformTypeLoc::getUnderlyingTInfo"),
      add_named_node(&mut g, 4294971054, "clang::UnaryTransformTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, 4294971055, "clang::UnaryTransformTypeLoc::getParensRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNARYTRANSFORMTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_COROUTINESUSPENDEXPR, "clang::CoroutineSuspendExpr");
  g.add_edge((CLASS_COROUTINESUSPENDEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_COROUTINESUSPENDEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294971057, "clang::CoroutineSuspendExpr::getCommonExpr"),
      add_named_node(&mut g, 4294971058, "clang::CoroutineSuspendExpr::getOpaqueValue"),
      add_named_node(&mut g, 4294971059, "clang::CoroutineSuspendExpr::getReadyExpr"),
      add_named_node(&mut g, 4294971060, "clang::CoroutineSuspendExpr::getSuspendExpr"),
      add_named_node(&mut g, 4294971061, "clang::CoroutineSuspendExpr::getResumeExpr"),
      add_named_node(&mut g, 4294971062, "clang::CoroutineSuspendExpr::getOperand"),
      add_named_node(&mut g, 4294971063, "clang::CoroutineSuspendExpr::getKeywordLoc"),
      add_named_node(&mut g, 4294971064, "clang::CoroutineSuspendExpr::getBeginLoc"),
      add_named_node(&mut g, 4294971065, "clang::CoroutineSuspendExpr::getEndLoc"),
      add_named_node(&mut g, 4294971066, "clang::CoroutineSuspendExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_COROUTINESUSPENDEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CODEMODELATTR, "clang::CodeModelAttr");
  g.add_edge((CLASS_CODEMODELATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CODEMODELATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CODEMODELATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_UNQUALTYPELOC, "clang::UnqualTypeLoc");
  g.add_edge((CLASS_UNQUALTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_UNQUALTYPELOC, META_SUBCLASS, CLASS_TYPELOC));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294971069, "clang::UnqualTypeLoc::getTypePtr"),
      add_named_node(&mut g, 4294971070, "clang::UnqualTypeLoc::getTypeLocClass"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_UNQUALTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_WEAKREFATTR, "clang::WeakRefAttr");
  g.add_edge((CLASS_WEAKREFATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_WEAKREFATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_WEAKREFATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPELOC, "clang::TypeLoc");
  g.add_edge((CLASS_TYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294971073, "clang::TypeLoc::getTypeLocClass"),
      add_named_node(&mut g, 4294971074, "clang::TypeLoc::isNull"),
      add_named_node(&mut g, 4294971075, "clang::TypeLoc::getType"),
      add_named_node(&mut g, 4294971076, "clang::TypeLoc::getTypePtr"),
      add_named_node(&mut g, 4294971077, "clang::TypeLoc::getOpaqueData"),
      add_named_node(&mut g, 4294971078, "clang::TypeLoc::getBeginLoc"),
      add_named_node(&mut g, 4294971079, "clang::TypeLoc::getEndLoc"),
      add_named_node(&mut g, 4294971080, "clang::TypeLoc::getSourceRange"),
      add_named_node(&mut g, 4294971081, "clang::TypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, 4294971082, "clang::TypeLoc::getFullDataSize"),
      add_named_node(&mut g, 4294971083, "clang::TypeLoc::getNextTypeLoc"),
      add_named_node(&mut g, 4294971084, "clang::TypeLoc::getUnqualifiedLoc"),
      add_named_node(&mut g, 4294971085, "clang::TypeLoc::IgnoreParens"),
      add_named_node(&mut g, 4294971086, "clang::TypeLoc::findExplicitQualifierLoc"),
      add_named_node(&mut g, 4294971087, "clang::TypeLoc::getContainedAutoTypeLoc"),
      add_named_node(&mut g, 4294971088, "clang::TypeLoc::findNullabilityLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPEOFEXPRTYPELOC, "clang::TypeOfExprTypeLoc");
  g.add_edge((CLASS_TYPEOFEXPRTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294971090, "clang::TypeOfExprTypeLoc::getUnderlyingExpr"),
      add_named_node(&mut g, 4294971091, "clang::TypeOfExprTypeLoc::getLocalSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPEOFEXPRTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ARCWEAKREFUNAVAILABLEATTR, "clang::ArcWeakrefUnavailableAttr");
  g.add_edge((CLASS_ARCWEAKREFUNAVAILABLEATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_ARCWEAKREFUNAVAILABLEATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ARCWEAKREFUNAVAILABLEATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SYCLKERNELATTR, "clang::SYCLKernelAttr");
  g.add_edge((CLASS_SYCLKERNELATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SYCLKERNELATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SYCLKERNELATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_BTFTAGATTRIBUTEDTYPE, "clang::BTFTagAttributedType");
  g.add_edge((CLASS_BTFTAGATTRIBUTEDTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_BTFTAGATTRIBUTEDTYPE, META_SUBCLASS, CLASS_TYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294971095, "clang::BTFTagAttributedType::getWrappedType"),
      add_named_node(&mut g, 4294971096, "clang::BTFTagAttributedType::getAttr"),
      add_named_node(&mut g, 4294971097, "clang::BTFTagAttributedType::isSugared"),
      add_named_node(&mut g, 4294971098, "clang::BTFTagAttributedType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_BTFTAGATTRIBUTEDTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_ELABORATEDTYPELOC, "clang::ElaboratedTypeLoc");
  g.add_edge((CLASS_ELABORATEDTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294971100, "clang::ElaboratedTypeLoc::getElaboratedKeywordLoc"),
      add_named_node(&mut g, 4294971101, "clang::ElaboratedTypeLoc::getQualifierLoc"),
      add_named_node(&mut g, 4294971102, "clang::ElaboratedTypeLoc::getLocalSourceRange"),
      add_named_node(&mut g, 4294971103, "clang::ElaboratedTypeLoc::getNamedTypeLoc"),
      add_named_node(&mut g, 4294971104, "clang::ElaboratedTypeLoc::getInnerType"),
      add_named_node(&mut g, 4294971105, "clang::ElaboratedTypeLoc::isEmpty"),
      add_named_node(&mut g, 4294971106, "clang::ElaboratedTypeLoc::getLocalDataAlignment"),
      add_named_node(&mut g, 4294971107, "clang::ElaboratedTypeLoc::getLocalDataSize"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_ELABORATEDTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPDISTRIBUTESIMDDIRECTIVE, "clang::OMPDistributeSimdDirective");
  g.add_edge((CLASS_OMPDISTRIBUTESIMDDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPDISTRIBUTESIMDDIRECTIVE, META_SUBCLASS, CLASS_OMPLOOPDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPDISTRIBUTESIMDDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_CPUSPECIFICATTR, "clang::CPUSpecificAttr");
  g.add_edge((CLASS_CPUSPECIFICATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_CPUSPECIFICATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_CPUSPECIFICATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_RECORDTYPELOC, "clang::RecordTypeLoc");
  g.add_edge((CLASS_RECORDTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294971111, "clang::RecordTypeLoc::getDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_RECORDTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_RVALUEREFERENCETYPELOC, "clang::RValueReferenceTypeLoc");
  g.add_edge((CLASS_RVALUEREFERENCETYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294971113, "clang::RValueReferenceTypeLoc::getAmpAmpLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_RVALUEREFERENCETYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_PRAGMACLANGBSSSECTIONATTR, "clang::PragmaClangBSSSectionAttr");
  g.add_edge((CLASS_PRAGMACLANGBSSSECTIONATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_PRAGMACLANGBSSSECTIONATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_PRAGMACLANGBSSSECTIONATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCINTERFACETYPELOC, "clang::ObjCInterfaceTypeLoc");
  g.add_edge((CLASS_OBJCINTERFACETYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCINTERFACETYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_AUTOTYPELOC, "clang::AutoTypeLoc");
  g.add_edge((CLASS_AUTOTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294971117, "clang::AutoTypeLoc::getAutoKeyword"),
      add_named_node(&mut g, 4294971118, "clang::AutoTypeLoc::isDecltypeAuto"),
      add_named_node(&mut g, 4294971119, "clang::AutoTypeLoc::getRParenLoc"),
      add_named_node(&mut g, 4294971120, "clang::AutoTypeLoc::isConstrained"),
      add_named_node(&mut g, 4294971121, "clang::AutoTypeLoc::getConceptReference"),
      add_named_node(&mut g, 4294971122, "clang::AutoTypeLoc::getNestedNameSpecifierLoc"),
      add_named_node(&mut g, 4294971123, "clang::AutoTypeLoc::getTemplateKWLoc"),
      add_named_node(&mut g, 4294971124, "clang::AutoTypeLoc::getConceptNameLoc"),
      add_named_node(&mut g, 4294971125, "clang::AutoTypeLoc::getFoundDecl"),
      add_named_node(&mut g, 4294971126, "clang::AutoTypeLoc::getNamedConcept"),
      add_named_node(&mut g, 4294971127, "clang::AutoTypeLoc::getConceptNameInfo"),
      add_named_node(&mut g, 4294971128, "clang::AutoTypeLoc::hasExplicitTemplateArgs"),
      add_named_node(&mut g, 4294971129, "clang::AutoTypeLoc::getLAngleLoc"),
      add_named_node(&mut g, 4294971130, "clang::AutoTypeLoc::getRAngleLoc"),
      add_named_node(&mut g, 4294971131, "clang::AutoTypeLoc::getNumArgs"),
      add_named_node(&mut g, 4294971132, "clang::AutoTypeLoc::getLocalSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_AUTOTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SWIFTATTRATTR, "clang::SwiftAttrAttr");
  g.add_edge((CLASS_SWIFTATTRATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SWIFTATTRATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SWIFTATTRATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TYPESPECTYPELOC, "clang::TypeSpecTypeLoc");
  g.add_edge((CLASS_TYPESPECTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294971135, "clang::TypeSpecTypeLoc::getNameLoc"),
      add_named_node(&mut g, 4294971136, "clang::TypeSpecTypeLoc::getLocalSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TYPESPECTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OMPSINGLEDIRECTIVE, "clang::OMPSingleDirective");
  g.add_edge((CLASS_OMPSINGLEDIRECTIVE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_OMPSINGLEDIRECTIVE, META_SUBCLASS, CLASS_OMPEXECUTABLEDIRECTIVE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OMPSINGLEDIRECTIVE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SIZEOFPACKEXPR, "clang::SizeOfPackExpr");
  g.add_edge((CLASS_SIZEOFPACKEXPR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_SIZEOFPACKEXPR, META_SUBCLASS, CLASS_EXPR));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294971139, "clang::SizeOfPackExpr::getOperatorLoc"),
      add_named_node(&mut g, 4294971140, "clang::SizeOfPackExpr::getPackLoc"),
      add_named_node(&mut g, 4294971141, "clang::SizeOfPackExpr::getRParenLoc"),
      add_named_node(&mut g, 4294971142, "clang::SizeOfPackExpr::getPack"),
      add_named_node(&mut g, 4294971143, "clang::SizeOfPackExpr::getPackLength"),
      add_named_node(&mut g, 4294971144, "clang::SizeOfPackExpr::isPartiallySubstituted"),
      add_named_node(&mut g, 4294971145, "clang::SizeOfPackExpr::getPartialArguments"),
      add_named_node(&mut g, 4294971146, "clang::SizeOfPackExpr::getBeginLoc"),
      add_named_node(&mut g, 4294971147, "clang::SizeOfPackExpr::getEndLoc"),
      add_named_node(&mut g, 4294971148, "clang::SizeOfPackExpr::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SIZEOFPACKEXPR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_OBJCTYPEPARAMTYPELOC, "clang::ObjCTypeParamTypeLoc");
  g.add_edge((CLASS_OBJCTYPEPARAMTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_OBJCTYPEPARAMTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DEDUCEDTEMPLATESPECIALIZATIONTYPELOC, "clang::DeducedTemplateSpecializationTypeLoc");
  g.add_edge((CLASS_DEDUCEDTEMPLATESPECIALIZATIONTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294971151, "clang::DeducedTemplateSpecializationTypeLoc::getTemplateNameLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DEDUCEDTEMPLATESPECIALIZATIONTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_POINTERATTR, "clang::PointerAttr");
  g.add_edge((CLASS_POINTERATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_POINTERATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_POINTERATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SUBSTTEMPLATETYPEPARMPACKTYPELOC, "clang::SubstTemplateTypeParmPackTypeLoc");
  g.add_edge((CLASS_SUBSTTEMPLATETYPEPARMPACKTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SUBSTTEMPLATETYPEPARMPACKTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MSASMSTMT, "clang::MSAsmStmt");
  g.add_edge((CLASS_MSASMSTMT, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_MSASMSTMT, META_SUBCLASS, CLASS_ASMSTMT));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294971155, "clang::MSAsmStmt::getLBraceLoc"),
      add_named_node(&mut g, 4294971156, "clang::MSAsmStmt::getEndLoc"),
      add_named_node(&mut g, 4294971157, "clang::MSAsmStmt::hasBraces"),
      add_named_node(&mut g, 4294971158, "clang::MSAsmStmt::getAsmString"),
      add_named_node(&mut g, 4294971159, "clang::MSAsmStmt::getAllConstraints"),
      add_named_node(&mut g, 4294971160, "clang::MSAsmStmt::getClobbers"),
      add_named_node(&mut g, 4294971161, "clang::MSAsmStmt::getAllExprs"),
      add_named_node(&mut g, 4294971162, "clang::MSAsmStmt::getBeginLoc"),
      add_named_node(&mut g, 4294971163, "clang::MSAsmStmt::children"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MSASMSTMT, META_METHOD, methods));
  }

  g.add_named_node(CLASS_TEMPLATETYPEPARMTYPELOC, "clang::TemplateTypeParmTypeLoc");
  g.add_edge((CLASS_TEMPLATETYPEPARMTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294971165, "clang::TemplateTypeParmTypeLoc::getDecl"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_TEMPLATETYPEPARMTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_MEMBERPOINTERTYPELOC, "clang::MemberPointerTypeLoc");
  g.add_edge((CLASS_MEMBERPOINTERTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294971167, "clang::MemberPointerTypeLoc::getStarLoc"),
      add_named_node(&mut g, 4294971168, "clang::MemberPointerTypeLoc::getClass"),
      add_named_node(&mut g, 4294971169, "clang::MemberPointerTypeLoc::getClassTInfo"),
      add_named_node(&mut g, 4294971170, "clang::MemberPointerTypeLoc::getLocalSourceRange"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_MEMBERPOINTERTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_SUBSTTEMPLATETYPEPARMTYPELOC, "clang::SubstTemplateTypeParmTypeLoc");
  g.add_edge((CLASS_SUBSTTEMPLATETYPEPARMTYPELOC, META_CLASS, META_CLANG_AST_NODE));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_SUBSTTEMPLATETYPEPARMTYPELOC, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NODEBUGATTR, "clang::NoDebugAttr");
  g.add_edge((CLASS_NODEBUGATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NODEBUGATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NODEBUGATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_VARIABLEARRAYTYPE, "clang::VariableArrayType");
  g.add_edge((CLASS_VARIABLEARRAYTYPE, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_VARIABLEARRAYTYPE, META_SUBCLASS, CLASS_ARRAYTYPE));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294971174, "clang::VariableArrayType::getSizeExpr"),
      add_named_node(&mut g, 4294971175, "clang::VariableArrayType::getBracketsRange"),
      add_named_node(&mut g, 4294971176, "clang::VariableArrayType::getLBracketLoc"),
      add_named_node(&mut g, 4294971177, "clang::VariableArrayType::getRBracketLoc"),
      add_named_node(&mut g, 4294971178, "clang::VariableArrayType::isSugared"),
      add_named_node(&mut g, 4294971179, "clang::VariableArrayType::desugar"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_VARIABLEARRAYTYPE, META_METHOD, methods));
  }

  g.add_named_node(CLASS_LABELDECL, "clang::LabelDecl");
  g.add_edge((CLASS_LABELDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_LABELDECL, META_SUBCLASS, CLASS_NAMEDDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294971181, "clang::LabelDecl::getStmt"),
      add_named_node(&mut g, 4294971182, "clang::LabelDecl::isGnuLocal"),
      add_named_node(&mut g, 4294971183, "clang::LabelDecl::getSourceRange"),
      add_named_node(&mut g, 4294971184, "clang::LabelDecl::isMSAsmLabel"),
      add_named_node(&mut g, 4294971185, "clang::LabelDecl::isResolvedMSAsmLabel"),
      add_named_node(&mut g, 4294971186, "clang::LabelDecl::getMSAsmLabel"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_LABELDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DECLARATORDECL, "clang::DeclaratorDecl");
  g.add_edge((CLASS_DECLARATORDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DECLARATORDECL, META_SUBCLASS, CLASS_VALUEDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294971188, "clang::DeclaratorDecl::getTypeSourceInfo"),
      add_named_node(&mut g, 4294971189, "clang::DeclaratorDecl::getInnerLocStart"),
      add_named_node(&mut g, 4294971190, "clang::DeclaratorDecl::getOuterLocStart"),
      add_named_node(&mut g, 4294971191, "clang::DeclaratorDecl::getSourceRange"),
      add_named_node(&mut g, 4294971192, "clang::DeclaratorDecl::getBeginLoc"),
      add_named_node(&mut g, 4294971193, "clang::DeclaratorDecl::getQualifier"),
      add_named_node(&mut g, 4294971194, "clang::DeclaratorDecl::getQualifierLoc"),
      add_named_node(&mut g, 4294971195, "clang::DeclaratorDecl::getTrailingRequiresClause"),
      add_named_node(&mut g, 4294971196, "clang::DeclaratorDecl::getNumTemplateParameterLists"),
      add_named_node(&mut g, 4294971197, "clang::DeclaratorDecl::getTypeSpecStartLoc"),
      add_named_node(&mut g, 4294971198, "clang::DeclaratorDecl::getTypeSpecEndLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DECLARATORDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_NAMESPACEDECL, "clang::NamespaceDecl");
  g.add_edge((CLASS_NAMESPACEDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_NAMESPACEDECL, META_SUBCLASS, CLASS_NAMEDDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294971200, "clang::NamespaceDecl::isAnonymousNamespace"),
      add_named_node(&mut g, 4294971201, "clang::NamespaceDecl::isInline"),
      add_named_node(&mut g, 4294971202, "clang::NamespaceDecl::isNested"),
      add_named_node(&mut g, 4294971203, "clang::NamespaceDecl::getOriginalNamespace"),
      add_named_node(&mut g, 4294971204, "clang::NamespaceDecl::isOriginalNamespace"),
      add_named_node(&mut g, 4294971205, "clang::NamespaceDecl::getAnonymousNamespace"),
      add_named_node(&mut g, 4294971206, "clang::NamespaceDecl::getCanonicalDecl"),
      add_named_node(&mut g, 4294971207, "clang::NamespaceDecl::getSourceRange"),
      add_named_node(&mut g, 4294971208, "clang::NamespaceDecl::getBeginLoc"),
      add_named_node(&mut g, 4294971209, "clang::NamespaceDecl::getRBraceLoc"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_NAMESPACEDECL, META_METHOD, methods));
  }

  g.add_named_node(CLASS_HLSLRESOURCEBINDINGATTR, "clang::HLSLResourceBindingAttr");
  g.add_edge((CLASS_HLSLRESOURCEBINDINGATTR, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_HLSLRESOURCEBINDINGATTR, META_SUBCLASS, CLASS_INHERITABLEATTR));
  {
    let methods = Vec::from([
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_HLSLRESOURCEBINDINGATTR, META_METHOD, methods));
  }

  g.add_named_node(CLASS_DECOMPOSITIONDECL, "clang::DecompositionDecl");
  g.add_edge((CLASS_DECOMPOSITIONDECL, META_CLASS, META_CLANG_AST_NODE));
  g.add_edge((CLASS_DECOMPOSITIONDECL, META_SUBCLASS, CLASS_VARDECL));
  {
    let methods = Vec::from([
      add_named_node(&mut g, 4294971212, "clang::DecompositionDecl::bindings"),
    ]);
    let methods = set_node(&mut g, META_CLANG_AST_METHOD, next_id(), next_id(), methods);
    g.add_edge((CLASS_DECOMPOSITIONDECL, META_METHOD, methods));
  }

  g.add_named_node(ENUM_VALUEKIND, "clang::APValue::ValueKind");
  g.add_edge((ENUM_VALUEKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294971214, "clang::APValue::None"),
      add_named_node(&mut g, 4294971215, "clang::APValue::Indeterminate"),
      add_named_node(&mut g, 4294971216, "clang::APValue::Int"),
      add_named_node(&mut g, 4294971217, "clang::APValue::Float"),
      add_named_node(&mut g, 4294971218, "clang::APValue::FixedPoint"),
      add_named_node(&mut g, 4294971219, "clang::APValue::ComplexInt"),
      add_named_node(&mut g, 4294971220, "clang::APValue::ComplexFloat"),
      add_named_node(&mut g, 4294971221, "clang::APValue::LValue"),
      add_named_node(&mut g, 4294971222, "clang::APValue::Vector"),
      add_named_node(&mut g, 4294971223, "clang::APValue::Array"),
      add_named_node(&mut g, 4294971224, "clang::APValue::Struct"),
      add_named_node(&mut g, 4294971225, "clang::APValue::Union"),
      add_named_node(&mut g, 4294971226, "clang::APValue::MemberPointer"),
      add_named_node(&mut g, 4294971227, "clang::APValue::AddrLabelDiff"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_VALUEKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_ACCESSSPECIFIER, "clang::AccessSpecifier");
  g.add_edge((ENUM_ACCESSSPECIFIER, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294971229, "clang::AS_public"),
      add_named_node(&mut g, 4294971230, "clang::AS_protected"),
      add_named_node(&mut g, 4294971231, "clang::AS_private"),
      add_named_node(&mut g, 4294971232, "clang::AS_none"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_ACCESSSPECIFIER, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_ARRAYSIZEMODIFIER, "clang::ArraySizeModifier");
  g.add_edge((ENUM_ARRAYSIZEMODIFIER, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294971234, "clang::ArraySizeModifier::Normal"),
      add_named_node(&mut g, 4294971235, "clang::ArraySizeModifier::Static"),
      add_named_node(&mut g, 4294971236, "clang::ArraySizeModifier::Star"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_ARRAYSIZEMODIFIER, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_ARRAYTYPETRAIT, "clang::ArrayTypeTrait");
  g.add_edge((ENUM_ARRAYTYPETRAIT, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294971238, "clang::ATT_ArrayRank"),
      add_named_node(&mut g, 4294971239, "clang::ATT_ArrayExtent"),
      add_named_node(&mut g, 4294971240, "clang::ATT_Last"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_ARRAYTYPETRAIT, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_ATOMICOP, "clang::AtomicExpr::AtomicOp");
  g.add_edge((ENUM_ATOMICOP, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294971242, "clang::AtomicExpr::AO__c11_atomic_init"),
      add_named_node(&mut g, 4294971243, "clang::AtomicExpr::AO__c11_atomic_load"),
      add_named_node(&mut g, 4294971244, "clang::AtomicExpr::AO__c11_atomic_store"),
      add_named_node(&mut g, 4294971245, "clang::AtomicExpr::AO__c11_atomic_exchange"),
      add_named_node(&mut g, 4294971246, "clang::AtomicExpr::AO__c11_atomic_compare_exchange_strong"),
      add_named_node(&mut g, 4294971247, "clang::AtomicExpr::AO__c11_atomic_compare_exchange_weak"),
      add_named_node(&mut g, 4294971248, "clang::AtomicExpr::AO__c11_atomic_fetch_add"),
      add_named_node(&mut g, 4294971249, "clang::AtomicExpr::AO__c11_atomic_fetch_sub"),
      add_named_node(&mut g, 4294971250, "clang::AtomicExpr::AO__c11_atomic_fetch_and"),
      add_named_node(&mut g, 4294971251, "clang::AtomicExpr::AO__c11_atomic_fetch_or"),
      add_named_node(&mut g, 4294971252, "clang::AtomicExpr::AO__c11_atomic_fetch_xor"),
      add_named_node(&mut g, 4294971253, "clang::AtomicExpr::AO__c11_atomic_fetch_nand"),
      add_named_node(&mut g, 4294971254, "clang::AtomicExpr::AO__c11_atomic_fetch_max"),
      add_named_node(&mut g, 4294971255, "clang::AtomicExpr::AO__c11_atomic_fetch_min"),
      add_named_node(&mut g, 4294971256, "clang::AtomicExpr::AO__atomic_load"),
      add_named_node(&mut g, 4294971257, "clang::AtomicExpr::AO__atomic_load_n"),
      add_named_node(&mut g, 4294971258, "clang::AtomicExpr::AO__atomic_store"),
      add_named_node(&mut g, 4294971259, "clang::AtomicExpr::AO__atomic_store_n"),
      add_named_node(&mut g, 4294971260, "clang::AtomicExpr::AO__atomic_exchange"),
      add_named_node(&mut g, 4294971261, "clang::AtomicExpr::AO__atomic_exchange_n"),
      add_named_node(&mut g, 4294971262, "clang::AtomicExpr::AO__atomic_compare_exchange"),
      add_named_node(&mut g, 4294971263, "clang::AtomicExpr::AO__atomic_compare_exchange_n"),
      add_named_node(&mut g, 4294971264, "clang::AtomicExpr::AO__atomic_fetch_add"),
      add_named_node(&mut g, 4294971265, "clang::AtomicExpr::AO__atomic_fetch_sub"),
      add_named_node(&mut g, 4294971266, "clang::AtomicExpr::AO__atomic_fetch_and"),
      add_named_node(&mut g, 4294971267, "clang::AtomicExpr::AO__atomic_fetch_or"),
      add_named_node(&mut g, 4294971268, "clang::AtomicExpr::AO__atomic_fetch_xor"),
      add_named_node(&mut g, 4294971269, "clang::AtomicExpr::AO__atomic_fetch_nand"),
      add_named_node(&mut g, 4294971270, "clang::AtomicExpr::AO__atomic_add_fetch"),
      add_named_node(&mut g, 4294971271, "clang::AtomicExpr::AO__atomic_sub_fetch"),
      add_named_node(&mut g, 4294971272, "clang::AtomicExpr::AO__atomic_and_fetch"),
      add_named_node(&mut g, 4294971273, "clang::AtomicExpr::AO__atomic_or_fetch"),
      add_named_node(&mut g, 4294971274, "clang::AtomicExpr::AO__atomic_xor_fetch"),
      add_named_node(&mut g, 4294971275, "clang::AtomicExpr::AO__atomic_max_fetch"),
      add_named_node(&mut g, 4294971276, "clang::AtomicExpr::AO__atomic_min_fetch"),
      add_named_node(&mut g, 4294971277, "clang::AtomicExpr::AO__atomic_nand_fetch"),
      add_named_node(&mut g, 4294971278, "clang::AtomicExpr::AO__scoped_atomic_load"),
      add_named_node(&mut g, 4294971279, "clang::AtomicExpr::AO__scoped_atomic_load_n"),
      add_named_node(&mut g, 4294971280, "clang::AtomicExpr::AO__scoped_atomic_store"),
      add_named_node(&mut g, 4294971281, "clang::AtomicExpr::AO__scoped_atomic_store_n"),
      add_named_node(&mut g, 4294971282, "clang::AtomicExpr::AO__scoped_atomic_exchange"),
      add_named_node(&mut g, 4294971283, "clang::AtomicExpr::AO__scoped_atomic_exchange_n"),
      add_named_node(&mut g, 4294971284, "clang::AtomicExpr::AO__scoped_atomic_compare_exchange"),
      add_named_node(&mut g, 4294971285, "clang::AtomicExpr::AO__scoped_atomic_compare_exchange_n"),
      add_named_node(&mut g, 4294971286, "clang::AtomicExpr::AO__scoped_atomic_fetch_add"),
      add_named_node(&mut g, 4294971287, "clang::AtomicExpr::AO__scoped_atomic_fetch_sub"),
      add_named_node(&mut g, 4294971288, "clang::AtomicExpr::AO__scoped_atomic_fetch_and"),
      add_named_node(&mut g, 4294971289, "clang::AtomicExpr::AO__scoped_atomic_fetch_or"),
      add_named_node(&mut g, 4294971290, "clang::AtomicExpr::AO__scoped_atomic_fetch_xor"),
      add_named_node(&mut g, 4294971291, "clang::AtomicExpr::AO__scoped_atomic_fetch_nand"),
      add_named_node(&mut g, 4294971292, "clang::AtomicExpr::AO__scoped_atomic_add_fetch"),
      add_named_node(&mut g, 4294971293, "clang::AtomicExpr::AO__scoped_atomic_sub_fetch"),
      add_named_node(&mut g, 4294971294, "clang::AtomicExpr::AO__scoped_atomic_and_fetch"),
      add_named_node(&mut g, 4294971295, "clang::AtomicExpr::AO__scoped_atomic_or_fetch"),
      add_named_node(&mut g, 4294971296, "clang::AtomicExpr::AO__scoped_atomic_xor_fetch"),
      add_named_node(&mut g, 4294971297, "clang::AtomicExpr::AO__scoped_atomic_max_fetch"),
      add_named_node(&mut g, 4294971298, "clang::AtomicExpr::AO__scoped_atomic_min_fetch"),
      add_named_node(&mut g, 4294971299, "clang::AtomicExpr::AO__scoped_atomic_nand_fetch"),
      add_named_node(&mut g, 4294971300, "clang::AtomicExpr::AO__scoped_atomic_fetch_min"),
      add_named_node(&mut g, 4294971301, "clang::AtomicExpr::AO__scoped_atomic_fetch_max"),
      add_named_node(&mut g, 4294971302, "clang::AtomicExpr::AO__opencl_atomic_init"),
      add_named_node(&mut g, 4294971303, "clang::AtomicExpr::AO__opencl_atomic_load"),
      add_named_node(&mut g, 4294971304, "clang::AtomicExpr::AO__opencl_atomic_store"),
      add_named_node(&mut g, 4294971305, "clang::AtomicExpr::AO__opencl_atomic_exchange"),
      add_named_node(&mut g, 4294971306, "clang::AtomicExpr::AO__opencl_atomic_compare_exchange_strong"),
      add_named_node(&mut g, 4294971307, "clang::AtomicExpr::AO__opencl_atomic_compare_exchange_weak"),
      add_named_node(&mut g, 4294971308, "clang::AtomicExpr::AO__opencl_atomic_fetch_add"),
      add_named_node(&mut g, 4294971309, "clang::AtomicExpr::AO__opencl_atomic_fetch_sub"),
      add_named_node(&mut g, 4294971310, "clang::AtomicExpr::AO__opencl_atomic_fetch_and"),
      add_named_node(&mut g, 4294971311, "clang::AtomicExpr::AO__opencl_atomic_fetch_or"),
      add_named_node(&mut g, 4294971312, "clang::AtomicExpr::AO__opencl_atomic_fetch_xor"),
      add_named_node(&mut g, 4294971313, "clang::AtomicExpr::AO__opencl_atomic_fetch_min"),
      add_named_node(&mut g, 4294971314, "clang::AtomicExpr::AO__opencl_atomic_fetch_max"),
      add_named_node(&mut g, 4294971315, "clang::AtomicExpr::AO__atomic_fetch_min"),
      add_named_node(&mut g, 4294971316, "clang::AtomicExpr::AO__atomic_fetch_max"),
      add_named_node(&mut g, 4294971317, "clang::AtomicExpr::AO__hip_atomic_load"),
      add_named_node(&mut g, 4294971318, "clang::AtomicExpr::AO__hip_atomic_store"),
      add_named_node(&mut g, 4294971319, "clang::AtomicExpr::AO__hip_atomic_compare_exchange_weak"),
      add_named_node(&mut g, 4294971320, "clang::AtomicExpr::AO__hip_atomic_compare_exchange_strong"),
      add_named_node(&mut g, 4294971321, "clang::AtomicExpr::AO__hip_atomic_exchange"),
      add_named_node(&mut g, 4294971322, "clang::AtomicExpr::AO__hip_atomic_fetch_add"),
      add_named_node(&mut g, 4294971323, "clang::AtomicExpr::AO__hip_atomic_fetch_sub"),
      add_named_node(&mut g, 4294971324, "clang::AtomicExpr::AO__hip_atomic_fetch_and"),
      add_named_node(&mut g, 4294971325, "clang::AtomicExpr::AO__hip_atomic_fetch_or"),
      add_named_node(&mut g, 4294971326, "clang::AtomicExpr::AO__hip_atomic_fetch_xor"),
      add_named_node(&mut g, 4294971327, "clang::AtomicExpr::AO__hip_atomic_fetch_min"),
      add_named_node(&mut g, 4294971328, "clang::AtomicExpr::AO__hip_atomic_fetch_max"),
      add_named_node(&mut g, 4294971329, "clang::AtomicExpr::BI_First"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_ATOMICOP, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_AUTOTYPEKEYWORD, "clang::AutoTypeKeyword");
  g.add_edge((ENUM_AUTOTYPEKEYWORD, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294971331, "clang::AutoTypeKeyword::Auto"),
      add_named_node(&mut g, 4294971332, "clang::AutoTypeKeyword::DecltypeAuto"),
      add_named_node(&mut g, 4294971333, "clang::AutoTypeKeyword::GNUAutoType"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_AUTOTYPEKEYWORD, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_BINARYOPERATORKIND, "clang::BinaryOperatorKind");
  g.add_edge((ENUM_BINARYOPERATORKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294971335, "clang::BO_PtrMemD"),
      add_named_node(&mut g, 4294971336, "clang::BO_PtrMemI"),
      add_named_node(&mut g, 4294971337, "clang::BO_Mul"),
      add_named_node(&mut g, 4294971338, "clang::BO_Div"),
      add_named_node(&mut g, 4294971339, "clang::BO_Rem"),
      add_named_node(&mut g, 4294971340, "clang::BO_Add"),
      add_named_node(&mut g, 4294971341, "clang::BO_Sub"),
      add_named_node(&mut g, 4294971342, "clang::BO_Shl"),
      add_named_node(&mut g, 4294971343, "clang::BO_Shr"),
      add_named_node(&mut g, 4294971344, "clang::BO_Cmp"),
      add_named_node(&mut g, 4294971345, "clang::BO_LT"),
      add_named_node(&mut g, 4294971346, "clang::BO_GT"),
      add_named_node(&mut g, 4294971347, "clang::BO_LE"),
      add_named_node(&mut g, 4294971348, "clang::BO_GE"),
      add_named_node(&mut g, 4294971349, "clang::BO_EQ"),
      add_named_node(&mut g, 4294971350, "clang::BO_NE"),
      add_named_node(&mut g, 4294971351, "clang::BO_And"),
      add_named_node(&mut g, 4294971352, "clang::BO_Xor"),
      add_named_node(&mut g, 4294971353, "clang::BO_Or"),
      add_named_node(&mut g, 4294971354, "clang::BO_LAnd"),
      add_named_node(&mut g, 4294971355, "clang::BO_LOr"),
      add_named_node(&mut g, 4294971356, "clang::BO_Assign"),
      add_named_node(&mut g, 4294971357, "clang::BO_MulAssign"),
      add_named_node(&mut g, 4294971358, "clang::BO_DivAssign"),
      add_named_node(&mut g, 4294971359, "clang::BO_RemAssign"),
      add_named_node(&mut g, 4294971360, "clang::BO_AddAssign"),
      add_named_node(&mut g, 4294971361, "clang::BO_SubAssign"),
      add_named_node(&mut g, 4294971362, "clang::BO_ShlAssign"),
      add_named_node(&mut g, 4294971363, "clang::BO_ShrAssign"),
      add_named_node(&mut g, 4294971364, "clang::BO_AndAssign"),
      add_named_node(&mut g, 4294971365, "clang::BO_XorAssign"),
      add_named_node(&mut g, 4294971366, "clang::BO_OrAssign"),
      add_named_node(&mut g, 4294971367, "clang::BO_Comma"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_BINARYOPERATORKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_BUILTINTEMPLATEKIND, "clang::BuiltinTemplateKind");
  g.add_edge((ENUM_BUILTINTEMPLATEKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294971369, "clang::BTK__make_integer_seq"),
      add_named_node(&mut g, 4294971370, "clang::BTK__type_pack_element"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_BUILTINTEMPLATEKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_KIND, "clang::BuiltinType::Kind");
  g.add_edge((ENUM_KIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294971372, "clang::BuiltinType::OCLImage1dRO"),
      add_named_node(&mut g, 4294971373, "clang::BuiltinType::OCLImage1dArrayRO"),
      add_named_node(&mut g, 4294971374, "clang::BuiltinType::OCLImage1dBufferRO"),
      add_named_node(&mut g, 4294971375, "clang::BuiltinType::OCLImage2dRO"),
      add_named_node(&mut g, 4294971376, "clang::BuiltinType::OCLImage2dArrayRO"),
      add_named_node(&mut g, 4294971377, "clang::BuiltinType::OCLImage2dDepthRO"),
      add_named_node(&mut g, 4294971378, "clang::BuiltinType::OCLImage2dArrayDepthRO"),
      add_named_node(&mut g, 4294971379, "clang::BuiltinType::OCLImage2dMSAARO"),
      add_named_node(&mut g, 4294971380, "clang::BuiltinType::OCLImage2dArrayMSAARO"),
      add_named_node(&mut g, 4294971381, "clang::BuiltinType::OCLImage2dMSAADepthRO"),
      add_named_node(&mut g, 4294971382, "clang::BuiltinType::OCLImage2dArrayMSAADepthRO"),
      add_named_node(&mut g, 4294971383, "clang::BuiltinType::OCLImage3dRO"),
      add_named_node(&mut g, 4294971384, "clang::BuiltinType::OCLImage1dWO"),
      add_named_node(&mut g, 4294971385, "clang::BuiltinType::OCLImage1dArrayWO"),
      add_named_node(&mut g, 4294971386, "clang::BuiltinType::OCLImage1dBufferWO"),
      add_named_node(&mut g, 4294971387, "clang::BuiltinType::OCLImage2dWO"),
      add_named_node(&mut g, 4294971388, "clang::BuiltinType::OCLImage2dArrayWO"),
      add_named_node(&mut g, 4294971389, "clang::BuiltinType::OCLImage2dDepthWO"),
      add_named_node(&mut g, 4294971390, "clang::BuiltinType::OCLImage2dArrayDepthWO"),
      add_named_node(&mut g, 4294971391, "clang::BuiltinType::OCLImage2dMSAAWO"),
      add_named_node(&mut g, 4294971392, "clang::BuiltinType::OCLImage2dArrayMSAAWO"),
      add_named_node(&mut g, 4294971393, "clang::BuiltinType::OCLImage2dMSAADepthWO"),
      add_named_node(&mut g, 4294971394, "clang::BuiltinType::OCLImage2dArrayMSAADepthWO"),
      add_named_node(&mut g, 4294971395, "clang::BuiltinType::OCLImage3dWO"),
      add_named_node(&mut g, 4294971396, "clang::BuiltinType::OCLImage1dRW"),
      add_named_node(&mut g, 4294971397, "clang::BuiltinType::OCLImage1dArrayRW"),
      add_named_node(&mut g, 4294971398, "clang::BuiltinType::OCLImage1dBufferRW"),
      add_named_node(&mut g, 4294971399, "clang::BuiltinType::OCLImage2dRW"),
      add_named_node(&mut g, 4294971400, "clang::BuiltinType::OCLImage2dArrayRW"),
      add_named_node(&mut g, 4294971401, "clang::BuiltinType::OCLImage2dDepthRW"),
      add_named_node(&mut g, 4294971402, "clang::BuiltinType::OCLImage2dArrayDepthRW"),
      add_named_node(&mut g, 4294971403, "clang::BuiltinType::OCLImage2dMSAARW"),
      add_named_node(&mut g, 4294971404, "clang::BuiltinType::OCLImage2dArrayMSAARW"),
      add_named_node(&mut g, 4294971405, "clang::BuiltinType::OCLImage2dMSAADepthRW"),
      add_named_node(&mut g, 4294971406, "clang::BuiltinType::OCLImage2dArrayMSAADepthRW"),
      add_named_node(&mut g, 4294971407, "clang::BuiltinType::OCLImage3dRW"),
      add_named_node(&mut g, 4294971408, "clang::BuiltinType::OCLIntelSubgroupAVCMcePayload"),
      add_named_node(&mut g, 4294971409, "clang::BuiltinType::OCLIntelSubgroupAVCImePayload"),
      add_named_node(&mut g, 4294971410, "clang::BuiltinType::OCLIntelSubgroupAVCRefPayload"),
      add_named_node(&mut g, 4294971411, "clang::BuiltinType::OCLIntelSubgroupAVCSicPayload"),
      add_named_node(&mut g, 4294971412, "clang::BuiltinType::OCLIntelSubgroupAVCMceResult"),
      add_named_node(&mut g, 4294971413, "clang::BuiltinType::OCLIntelSubgroupAVCImeResult"),
      add_named_node(&mut g, 4294971414, "clang::BuiltinType::OCLIntelSubgroupAVCRefResult"),
      add_named_node(&mut g, 4294971415, "clang::BuiltinType::OCLIntelSubgroupAVCSicResult"),
      add_named_node(&mut g, 4294971416, "clang::BuiltinType::OCLIntelSubgroupAVCImeResultSingleReferenceStreamout"),
      add_named_node(&mut g, 4294971417, "clang::BuiltinType::OCLIntelSubgroupAVCImeResultDualReferenceStreamout"),
      add_named_node(&mut g, 4294971418, "clang::BuiltinType::OCLIntelSubgroupAVCImeSingleReferenceStreamin"),
      add_named_node(&mut g, 4294971419, "clang::BuiltinType::OCLIntelSubgroupAVCImeDualReferenceStreamin"),
      add_named_node(&mut g, 4294971420, "clang::BuiltinType::SveInt8"),
      add_named_node(&mut g, 4294971421, "clang::BuiltinType::SveInt16"),
      add_named_node(&mut g, 4294971422, "clang::BuiltinType::SveInt32"),
      add_named_node(&mut g, 4294971423, "clang::BuiltinType::SveInt64"),
      add_named_node(&mut g, 4294971424, "clang::BuiltinType::SveUint8"),
      add_named_node(&mut g, 4294971425, "clang::BuiltinType::SveUint16"),
      add_named_node(&mut g, 4294971426, "clang::BuiltinType::SveUint32"),
      add_named_node(&mut g, 4294971427, "clang::BuiltinType::SveUint64"),
      add_named_node(&mut g, 4294971428, "clang::BuiltinType::SveFloat16"),
      add_named_node(&mut g, 4294971429, "clang::BuiltinType::SveFloat32"),
      add_named_node(&mut g, 4294971430, "clang::BuiltinType::SveFloat64"),
      add_named_node(&mut g, 4294971431, "clang::BuiltinType::SveBFloat16"),
      add_named_node(&mut g, 4294971432, "clang::BuiltinType::SveInt8x2"),
      add_named_node(&mut g, 4294971433, "clang::BuiltinType::SveInt16x2"),
      add_named_node(&mut g, 4294971434, "clang::BuiltinType::SveInt32x2"),
      add_named_node(&mut g, 4294971435, "clang::BuiltinType::SveInt64x2"),
      add_named_node(&mut g, 4294971436, "clang::BuiltinType::SveUint8x2"),
      add_named_node(&mut g, 4294971437, "clang::BuiltinType::SveUint16x2"),
      add_named_node(&mut g, 4294971438, "clang::BuiltinType::SveUint32x2"),
      add_named_node(&mut g, 4294971439, "clang::BuiltinType::SveUint64x2"),
      add_named_node(&mut g, 4294971440, "clang::BuiltinType::SveFloat16x2"),
      add_named_node(&mut g, 4294971441, "clang::BuiltinType::SveFloat32x2"),
      add_named_node(&mut g, 4294971442, "clang::BuiltinType::SveFloat64x2"),
      add_named_node(&mut g, 4294971443, "clang::BuiltinType::SveBFloat16x2"),
      add_named_node(&mut g, 4294971444, "clang::BuiltinType::SveInt8x3"),
      add_named_node(&mut g, 4294971445, "clang::BuiltinType::SveInt16x3"),
      add_named_node(&mut g, 4294971446, "clang::BuiltinType::SveInt32x3"),
      add_named_node(&mut g, 4294971447, "clang::BuiltinType::SveInt64x3"),
      add_named_node(&mut g, 4294971448, "clang::BuiltinType::SveUint8x3"),
      add_named_node(&mut g, 4294971449, "clang::BuiltinType::SveUint16x3"),
      add_named_node(&mut g, 4294971450, "clang::BuiltinType::SveUint32x3"),
      add_named_node(&mut g, 4294971451, "clang::BuiltinType::SveUint64x3"),
      add_named_node(&mut g, 4294971452, "clang::BuiltinType::SveFloat16x3"),
      add_named_node(&mut g, 4294971453, "clang::BuiltinType::SveFloat32x3"),
      add_named_node(&mut g, 4294971454, "clang::BuiltinType::SveFloat64x3"),
      add_named_node(&mut g, 4294971455, "clang::BuiltinType::SveBFloat16x3"),
      add_named_node(&mut g, 4294971456, "clang::BuiltinType::SveInt8x4"),
      add_named_node(&mut g, 4294971457, "clang::BuiltinType::SveInt16x4"),
      add_named_node(&mut g, 4294971458, "clang::BuiltinType::SveInt32x4"),
      add_named_node(&mut g, 4294971459, "clang::BuiltinType::SveInt64x4"),
      add_named_node(&mut g, 4294971460, "clang::BuiltinType::SveUint8x4"),
      add_named_node(&mut g, 4294971461, "clang::BuiltinType::SveUint16x4"),
      add_named_node(&mut g, 4294971462, "clang::BuiltinType::SveUint32x4"),
      add_named_node(&mut g, 4294971463, "clang::BuiltinType::SveUint64x4"),
      add_named_node(&mut g, 4294971464, "clang::BuiltinType::SveFloat16x4"),
      add_named_node(&mut g, 4294971465, "clang::BuiltinType::SveFloat32x4"),
      add_named_node(&mut g, 4294971466, "clang::BuiltinType::SveFloat64x4"),
      add_named_node(&mut g, 4294971467, "clang::BuiltinType::SveBFloat16x4"),
      add_named_node(&mut g, 4294971468, "clang::BuiltinType::SveBool"),
      add_named_node(&mut g, 4294971469, "clang::BuiltinType::SveBoolx2"),
      add_named_node(&mut g, 4294971470, "clang::BuiltinType::SveBoolx4"),
      add_named_node(&mut g, 4294971471, "clang::BuiltinType::SveCount"),
      add_named_node(&mut g, 4294971472, "clang::BuiltinType::VectorQuad"),
      add_named_node(&mut g, 4294971473, "clang::BuiltinType::VectorPair"),
      add_named_node(&mut g, 4294971474, "clang::BuiltinType::RvvInt8mf8"),
      add_named_node(&mut g, 4294971475, "clang::BuiltinType::RvvInt8mf4"),
      add_named_node(&mut g, 4294971476, "clang::BuiltinType::RvvInt8mf2"),
      add_named_node(&mut g, 4294971477, "clang::BuiltinType::RvvInt8m1"),
      add_named_node(&mut g, 4294971478, "clang::BuiltinType::RvvInt8m2"),
      add_named_node(&mut g, 4294971479, "clang::BuiltinType::RvvInt8m4"),
      add_named_node(&mut g, 4294971480, "clang::BuiltinType::RvvInt8m8"),
      add_named_node(&mut g, 4294971481, "clang::BuiltinType::RvvUint8mf8"),
      add_named_node(&mut g, 4294971482, "clang::BuiltinType::RvvUint8mf4"),
      add_named_node(&mut g, 4294971483, "clang::BuiltinType::RvvUint8mf2"),
      add_named_node(&mut g, 4294971484, "clang::BuiltinType::RvvUint8m1"),
      add_named_node(&mut g, 4294971485, "clang::BuiltinType::RvvUint8m2"),
      add_named_node(&mut g, 4294971486, "clang::BuiltinType::RvvUint8m4"),
      add_named_node(&mut g, 4294971487, "clang::BuiltinType::RvvUint8m8"),
      add_named_node(&mut g, 4294971488, "clang::BuiltinType::RvvInt16mf4"),
      add_named_node(&mut g, 4294971489, "clang::BuiltinType::RvvInt16mf2"),
      add_named_node(&mut g, 4294971490, "clang::BuiltinType::RvvInt16m1"),
      add_named_node(&mut g, 4294971491, "clang::BuiltinType::RvvInt16m2"),
      add_named_node(&mut g, 4294971492, "clang::BuiltinType::RvvInt16m4"),
      add_named_node(&mut g, 4294971493, "clang::BuiltinType::RvvInt16m8"),
      add_named_node(&mut g, 4294971494, "clang::BuiltinType::RvvUint16mf4"),
      add_named_node(&mut g, 4294971495, "clang::BuiltinType::RvvUint16mf2"),
      add_named_node(&mut g, 4294971496, "clang::BuiltinType::RvvUint16m1"),
      add_named_node(&mut g, 4294971497, "clang::BuiltinType::RvvUint16m2"),
      add_named_node(&mut g, 4294971498, "clang::BuiltinType::RvvUint16m4"),
      add_named_node(&mut g, 4294971499, "clang::BuiltinType::RvvUint16m8"),
      add_named_node(&mut g, 4294971500, "clang::BuiltinType::RvvInt32mf2"),
      add_named_node(&mut g, 4294971501, "clang::BuiltinType::RvvInt32m1"),
      add_named_node(&mut g, 4294971502, "clang::BuiltinType::RvvInt32m2"),
      add_named_node(&mut g, 4294971503, "clang::BuiltinType::RvvInt32m4"),
      add_named_node(&mut g, 4294971504, "clang::BuiltinType::RvvInt32m8"),
      add_named_node(&mut g, 4294971505, "clang::BuiltinType::RvvUint32mf2"),
      add_named_node(&mut g, 4294971506, "clang::BuiltinType::RvvUint32m1"),
      add_named_node(&mut g, 4294971507, "clang::BuiltinType::RvvUint32m2"),
      add_named_node(&mut g, 4294971508, "clang::BuiltinType::RvvUint32m4"),
      add_named_node(&mut g, 4294971509, "clang::BuiltinType::RvvUint32m8"),
      add_named_node(&mut g, 4294971510, "clang::BuiltinType::RvvInt64m1"),
      add_named_node(&mut g, 4294971511, "clang::BuiltinType::RvvInt64m2"),
      add_named_node(&mut g, 4294971512, "clang::BuiltinType::RvvInt64m4"),
      add_named_node(&mut g, 4294971513, "clang::BuiltinType::RvvInt64m8"),
      add_named_node(&mut g, 4294971514, "clang::BuiltinType::RvvUint64m1"),
      add_named_node(&mut g, 4294971515, "clang::BuiltinType::RvvUint64m2"),
      add_named_node(&mut g, 4294971516, "clang::BuiltinType::RvvUint64m4"),
      add_named_node(&mut g, 4294971517, "clang::BuiltinType::RvvUint64m8"),
      add_named_node(&mut g, 4294971518, "clang::BuiltinType::RvvFloat16mf4"),
      add_named_node(&mut g, 4294971519, "clang::BuiltinType::RvvFloat16mf2"),
      add_named_node(&mut g, 4294971520, "clang::BuiltinType::RvvFloat16m1"),
      add_named_node(&mut g, 4294971521, "clang::BuiltinType::RvvFloat16m2"),
      add_named_node(&mut g, 4294971522, "clang::BuiltinType::RvvFloat16m4"),
      add_named_node(&mut g, 4294971523, "clang::BuiltinType::RvvFloat16m8"),
      add_named_node(&mut g, 4294971524, "clang::BuiltinType::RvvBFloat16mf4"),
      add_named_node(&mut g, 4294971525, "clang::BuiltinType::RvvBFloat16mf2"),
      add_named_node(&mut g, 4294971526, "clang::BuiltinType::RvvBFloat16m1"),
      add_named_node(&mut g, 4294971527, "clang::BuiltinType::RvvBFloat16m2"),
      add_named_node(&mut g, 4294971528, "clang::BuiltinType::RvvBFloat16m4"),
      add_named_node(&mut g, 4294971529, "clang::BuiltinType::RvvBFloat16m8"),
      add_named_node(&mut g, 4294971530, "clang::BuiltinType::RvvFloat32mf2"),
      add_named_node(&mut g, 4294971531, "clang::BuiltinType::RvvFloat32m1"),
      add_named_node(&mut g, 4294971532, "clang::BuiltinType::RvvFloat32m2"),
      add_named_node(&mut g, 4294971533, "clang::BuiltinType::RvvFloat32m4"),
      add_named_node(&mut g, 4294971534, "clang::BuiltinType::RvvFloat32m8"),
      add_named_node(&mut g, 4294971535, "clang::BuiltinType::RvvFloat64m1"),
      add_named_node(&mut g, 4294971536, "clang::BuiltinType::RvvFloat64m2"),
      add_named_node(&mut g, 4294971537, "clang::BuiltinType::RvvFloat64m4"),
      add_named_node(&mut g, 4294971538, "clang::BuiltinType::RvvFloat64m8"),
      add_named_node(&mut g, 4294971539, "clang::BuiltinType::RvvBool1"),
      add_named_node(&mut g, 4294971540, "clang::BuiltinType::RvvBool2"),
      add_named_node(&mut g, 4294971541, "clang::BuiltinType::RvvBool4"),
      add_named_node(&mut g, 4294971542, "clang::BuiltinType::RvvBool8"),
      add_named_node(&mut g, 4294971543, "clang::BuiltinType::RvvBool16"),
      add_named_node(&mut g, 4294971544, "clang::BuiltinType::RvvBool32"),
      add_named_node(&mut g, 4294971545, "clang::BuiltinType::RvvBool64"),
      add_named_node(&mut g, 4294971546, "clang::BuiltinType::RvvInt8mf8x2"),
      add_named_node(&mut g, 4294971547, "clang::BuiltinType::RvvInt8mf8x3"),
      add_named_node(&mut g, 4294971548, "clang::BuiltinType::RvvInt8mf8x4"),
      add_named_node(&mut g, 4294971549, "clang::BuiltinType::RvvInt8mf8x5"),
      add_named_node(&mut g, 4294971550, "clang::BuiltinType::RvvInt8mf8x6"),
      add_named_node(&mut g, 4294971551, "clang::BuiltinType::RvvInt8mf8x7"),
      add_named_node(&mut g, 4294971552, "clang::BuiltinType::RvvInt8mf8x8"),
      add_named_node(&mut g, 4294971553, "clang::BuiltinType::RvvInt8mf4x2"),
      add_named_node(&mut g, 4294971554, "clang::BuiltinType::RvvInt8mf4x3"),
      add_named_node(&mut g, 4294971555, "clang::BuiltinType::RvvInt8mf4x4"),
      add_named_node(&mut g, 4294971556, "clang::BuiltinType::RvvInt8mf4x5"),
      add_named_node(&mut g, 4294971557, "clang::BuiltinType::RvvInt8mf4x6"),
      add_named_node(&mut g, 4294971558, "clang::BuiltinType::RvvInt8mf4x7"),
      add_named_node(&mut g, 4294971559, "clang::BuiltinType::RvvInt8mf4x8"),
      add_named_node(&mut g, 4294971560, "clang::BuiltinType::RvvInt8mf2x2"),
      add_named_node(&mut g, 4294971561, "clang::BuiltinType::RvvInt8mf2x3"),
      add_named_node(&mut g, 4294971562, "clang::BuiltinType::RvvInt8mf2x4"),
      add_named_node(&mut g, 4294971563, "clang::BuiltinType::RvvInt8mf2x5"),
      add_named_node(&mut g, 4294971564, "clang::BuiltinType::RvvInt8mf2x6"),
      add_named_node(&mut g, 4294971565, "clang::BuiltinType::RvvInt8mf2x7"),
      add_named_node(&mut g, 4294971566, "clang::BuiltinType::RvvInt8mf2x8"),
      add_named_node(&mut g, 4294971567, "clang::BuiltinType::RvvInt8m1x2"),
      add_named_node(&mut g, 4294971568, "clang::BuiltinType::RvvInt8m1x3"),
      add_named_node(&mut g, 4294971569, "clang::BuiltinType::RvvInt8m1x4"),
      add_named_node(&mut g, 4294971570, "clang::BuiltinType::RvvInt8m1x5"),
      add_named_node(&mut g, 4294971571, "clang::BuiltinType::RvvInt8m1x6"),
      add_named_node(&mut g, 4294971572, "clang::BuiltinType::RvvInt8m1x7"),
      add_named_node(&mut g, 4294971573, "clang::BuiltinType::RvvInt8m1x8"),
      add_named_node(&mut g, 4294971574, "clang::BuiltinType::RvvInt8m2x2"),
      add_named_node(&mut g, 4294971575, "clang::BuiltinType::RvvInt8m2x3"),
      add_named_node(&mut g, 4294971576, "clang::BuiltinType::RvvInt8m2x4"),
      add_named_node(&mut g, 4294971577, "clang::BuiltinType::RvvInt8m4x2"),
      add_named_node(&mut g, 4294971578, "clang::BuiltinType::RvvUint8mf8x2"),
      add_named_node(&mut g, 4294971579, "clang::BuiltinType::RvvUint8mf8x3"),
      add_named_node(&mut g, 4294971580, "clang::BuiltinType::RvvUint8mf8x4"),
      add_named_node(&mut g, 4294971581, "clang::BuiltinType::RvvUint8mf8x5"),
      add_named_node(&mut g, 4294971582, "clang::BuiltinType::RvvUint8mf8x6"),
      add_named_node(&mut g, 4294971583, "clang::BuiltinType::RvvUint8mf8x7"),
      add_named_node(&mut g, 4294971584, "clang::BuiltinType::RvvUint8mf8x8"),
      add_named_node(&mut g, 4294971585, "clang::BuiltinType::RvvUint8mf4x2"),
      add_named_node(&mut g, 4294971586, "clang::BuiltinType::RvvUint8mf4x3"),
      add_named_node(&mut g, 4294971587, "clang::BuiltinType::RvvUint8mf4x4"),
      add_named_node(&mut g, 4294971588, "clang::BuiltinType::RvvUint8mf4x5"),
      add_named_node(&mut g, 4294971589, "clang::BuiltinType::RvvUint8mf4x6"),
      add_named_node(&mut g, 4294971590, "clang::BuiltinType::RvvUint8mf4x7"),
      add_named_node(&mut g, 4294971591, "clang::BuiltinType::RvvUint8mf4x8"),
      add_named_node(&mut g, 4294971592, "clang::BuiltinType::RvvUint8mf2x2"),
      add_named_node(&mut g, 4294971593, "clang::BuiltinType::RvvUint8mf2x3"),
      add_named_node(&mut g, 4294971594, "clang::BuiltinType::RvvUint8mf2x4"),
      add_named_node(&mut g, 4294971595, "clang::BuiltinType::RvvUint8mf2x5"),
      add_named_node(&mut g, 4294971596, "clang::BuiltinType::RvvUint8mf2x6"),
      add_named_node(&mut g, 4294971597, "clang::BuiltinType::RvvUint8mf2x7"),
      add_named_node(&mut g, 4294971598, "clang::BuiltinType::RvvUint8mf2x8"),
      add_named_node(&mut g, 4294971599, "clang::BuiltinType::RvvUint8m1x2"),
      add_named_node(&mut g, 4294971600, "clang::BuiltinType::RvvUint8m1x3"),
      add_named_node(&mut g, 4294971601, "clang::BuiltinType::RvvUint8m1x4"),
      add_named_node(&mut g, 4294971602, "clang::BuiltinType::RvvUint8m1x5"),
      add_named_node(&mut g, 4294971603, "clang::BuiltinType::RvvUint8m1x6"),
      add_named_node(&mut g, 4294971604, "clang::BuiltinType::RvvUint8m1x7"),
      add_named_node(&mut g, 4294971605, "clang::BuiltinType::RvvUint8m1x8"),
      add_named_node(&mut g, 4294971606, "clang::BuiltinType::RvvUint8m2x2"),
      add_named_node(&mut g, 4294971607, "clang::BuiltinType::RvvUint8m2x3"),
      add_named_node(&mut g, 4294971608, "clang::BuiltinType::RvvUint8m2x4"),
      add_named_node(&mut g, 4294971609, "clang::BuiltinType::RvvUint8m4x2"),
      add_named_node(&mut g, 4294971610, "clang::BuiltinType::RvvInt16mf4x2"),
      add_named_node(&mut g, 4294971611, "clang::BuiltinType::RvvInt16mf4x3"),
      add_named_node(&mut g, 4294971612, "clang::BuiltinType::RvvInt16mf4x4"),
      add_named_node(&mut g, 4294971613, "clang::BuiltinType::RvvInt16mf4x5"),
      add_named_node(&mut g, 4294971614, "clang::BuiltinType::RvvInt16mf4x6"),
      add_named_node(&mut g, 4294971615, "clang::BuiltinType::RvvInt16mf4x7"),
      add_named_node(&mut g, 4294971616, "clang::BuiltinType::RvvInt16mf4x8"),
      add_named_node(&mut g, 4294971617, "clang::BuiltinType::RvvInt16mf2x2"),
      add_named_node(&mut g, 4294971618, "clang::BuiltinType::RvvInt16mf2x3"),
      add_named_node(&mut g, 4294971619, "clang::BuiltinType::RvvInt16mf2x4"),
      add_named_node(&mut g, 4294971620, "clang::BuiltinType::RvvInt16mf2x5"),
      add_named_node(&mut g, 4294971621, "clang::BuiltinType::RvvInt16mf2x6"),
      add_named_node(&mut g, 4294971622, "clang::BuiltinType::RvvInt16mf2x7"),
      add_named_node(&mut g, 4294971623, "clang::BuiltinType::RvvInt16mf2x8"),
      add_named_node(&mut g, 4294971624, "clang::BuiltinType::RvvInt16m1x2"),
      add_named_node(&mut g, 4294971625, "clang::BuiltinType::RvvInt16m1x3"),
      add_named_node(&mut g, 4294971626, "clang::BuiltinType::RvvInt16m1x4"),
      add_named_node(&mut g, 4294971627, "clang::BuiltinType::RvvInt16m1x5"),
      add_named_node(&mut g, 4294971628, "clang::BuiltinType::RvvInt16m1x6"),
      add_named_node(&mut g, 4294971629, "clang::BuiltinType::RvvInt16m1x7"),
      add_named_node(&mut g, 4294971630, "clang::BuiltinType::RvvInt16m1x8"),
      add_named_node(&mut g, 4294971631, "clang::BuiltinType::RvvInt16m2x2"),
      add_named_node(&mut g, 4294971632, "clang::BuiltinType::RvvInt16m2x3"),
      add_named_node(&mut g, 4294971633, "clang::BuiltinType::RvvInt16m2x4"),
      add_named_node(&mut g, 4294971634, "clang::BuiltinType::RvvInt16m4x2"),
      add_named_node(&mut g, 4294971635, "clang::BuiltinType::RvvUint16mf4x2"),
      add_named_node(&mut g, 4294971636, "clang::BuiltinType::RvvUint16mf4x3"),
      add_named_node(&mut g, 4294971637, "clang::BuiltinType::RvvUint16mf4x4"),
      add_named_node(&mut g, 4294971638, "clang::BuiltinType::RvvUint16mf4x5"),
      add_named_node(&mut g, 4294971639, "clang::BuiltinType::RvvUint16mf4x6"),
      add_named_node(&mut g, 4294971640, "clang::BuiltinType::RvvUint16mf4x7"),
      add_named_node(&mut g, 4294971641, "clang::BuiltinType::RvvUint16mf4x8"),
      add_named_node(&mut g, 4294971642, "clang::BuiltinType::RvvUint16mf2x2"),
      add_named_node(&mut g, 4294971643, "clang::BuiltinType::RvvUint16mf2x3"),
      add_named_node(&mut g, 4294971644, "clang::BuiltinType::RvvUint16mf2x4"),
      add_named_node(&mut g, 4294971645, "clang::BuiltinType::RvvUint16mf2x5"),
      add_named_node(&mut g, 4294971646, "clang::BuiltinType::RvvUint16mf2x6"),
      add_named_node(&mut g, 4294971647, "clang::BuiltinType::RvvUint16mf2x7"),
      add_named_node(&mut g, 4294971648, "clang::BuiltinType::RvvUint16mf2x8"),
      add_named_node(&mut g, 4294971649, "clang::BuiltinType::RvvUint16m1x2"),
      add_named_node(&mut g, 4294971650, "clang::BuiltinType::RvvUint16m1x3"),
      add_named_node(&mut g, 4294971651, "clang::BuiltinType::RvvUint16m1x4"),
      add_named_node(&mut g, 4294971652, "clang::BuiltinType::RvvUint16m1x5"),
      add_named_node(&mut g, 4294971653, "clang::BuiltinType::RvvUint16m1x6"),
      add_named_node(&mut g, 4294971654, "clang::BuiltinType::RvvUint16m1x7"),
      add_named_node(&mut g, 4294971655, "clang::BuiltinType::RvvUint16m1x8"),
      add_named_node(&mut g, 4294971656, "clang::BuiltinType::RvvUint16m2x2"),
      add_named_node(&mut g, 4294971657, "clang::BuiltinType::RvvUint16m2x3"),
      add_named_node(&mut g, 4294971658, "clang::BuiltinType::RvvUint16m2x4"),
      add_named_node(&mut g, 4294971659, "clang::BuiltinType::RvvUint16m4x2"),
      add_named_node(&mut g, 4294971660, "clang::BuiltinType::RvvInt32mf2x2"),
      add_named_node(&mut g, 4294971661, "clang::BuiltinType::RvvInt32mf2x3"),
      add_named_node(&mut g, 4294971662, "clang::BuiltinType::RvvInt32mf2x4"),
      add_named_node(&mut g, 4294971663, "clang::BuiltinType::RvvInt32mf2x5"),
      add_named_node(&mut g, 4294971664, "clang::BuiltinType::RvvInt32mf2x6"),
      add_named_node(&mut g, 4294971665, "clang::BuiltinType::RvvInt32mf2x7"),
      add_named_node(&mut g, 4294971666, "clang::BuiltinType::RvvInt32mf2x8"),
      add_named_node(&mut g, 4294971667, "clang::BuiltinType::RvvInt32m1x2"),
      add_named_node(&mut g, 4294971668, "clang::BuiltinType::RvvInt32m1x3"),
      add_named_node(&mut g, 4294971669, "clang::BuiltinType::RvvInt32m1x4"),
      add_named_node(&mut g, 4294971670, "clang::BuiltinType::RvvInt32m1x5"),
      add_named_node(&mut g, 4294971671, "clang::BuiltinType::RvvInt32m1x6"),
      add_named_node(&mut g, 4294971672, "clang::BuiltinType::RvvInt32m1x7"),
      add_named_node(&mut g, 4294971673, "clang::BuiltinType::RvvInt32m1x8"),
      add_named_node(&mut g, 4294971674, "clang::BuiltinType::RvvInt32m2x2"),
      add_named_node(&mut g, 4294971675, "clang::BuiltinType::RvvInt32m2x3"),
      add_named_node(&mut g, 4294971676, "clang::BuiltinType::RvvInt32m2x4"),
      add_named_node(&mut g, 4294971677, "clang::BuiltinType::RvvInt32m4x2"),
      add_named_node(&mut g, 4294971678, "clang::BuiltinType::RvvUint32mf2x2"),
      add_named_node(&mut g, 4294971679, "clang::BuiltinType::RvvUint32mf2x3"),
      add_named_node(&mut g, 4294971680, "clang::BuiltinType::RvvUint32mf2x4"),
      add_named_node(&mut g, 4294971681, "clang::BuiltinType::RvvUint32mf2x5"),
      add_named_node(&mut g, 4294971682, "clang::BuiltinType::RvvUint32mf2x6"),
      add_named_node(&mut g, 4294971683, "clang::BuiltinType::RvvUint32mf2x7"),
      add_named_node(&mut g, 4294971684, "clang::BuiltinType::RvvUint32mf2x8"),
      add_named_node(&mut g, 4294971685, "clang::BuiltinType::RvvUint32m1x2"),
      add_named_node(&mut g, 4294971686, "clang::BuiltinType::RvvUint32m1x3"),
      add_named_node(&mut g, 4294971687, "clang::BuiltinType::RvvUint32m1x4"),
      add_named_node(&mut g, 4294971688, "clang::BuiltinType::RvvUint32m1x5"),
      add_named_node(&mut g, 4294971689, "clang::BuiltinType::RvvUint32m1x6"),
      add_named_node(&mut g, 4294971690, "clang::BuiltinType::RvvUint32m1x7"),
      add_named_node(&mut g, 4294971691, "clang::BuiltinType::RvvUint32m1x8"),
      add_named_node(&mut g, 4294971692, "clang::BuiltinType::RvvUint32m2x2"),
      add_named_node(&mut g, 4294971693, "clang::BuiltinType::RvvUint32m2x3"),
      add_named_node(&mut g, 4294971694, "clang::BuiltinType::RvvUint32m2x4"),
      add_named_node(&mut g, 4294971695, "clang::BuiltinType::RvvUint32m4x2"),
      add_named_node(&mut g, 4294971696, "clang::BuiltinType::RvvInt64m1x2"),
      add_named_node(&mut g, 4294971697, "clang::BuiltinType::RvvInt64m1x3"),
      add_named_node(&mut g, 4294971698, "clang::BuiltinType::RvvInt64m1x4"),
      add_named_node(&mut g, 4294971699, "clang::BuiltinType::RvvInt64m1x5"),
      add_named_node(&mut g, 4294971700, "clang::BuiltinType::RvvInt64m1x6"),
      add_named_node(&mut g, 4294971701, "clang::BuiltinType::RvvInt64m1x7"),
      add_named_node(&mut g, 4294971702, "clang::BuiltinType::RvvInt64m1x8"),
      add_named_node(&mut g, 4294971703, "clang::BuiltinType::RvvInt64m2x2"),
      add_named_node(&mut g, 4294971704, "clang::BuiltinType::RvvInt64m2x3"),
      add_named_node(&mut g, 4294971705, "clang::BuiltinType::RvvInt64m2x4"),
      add_named_node(&mut g, 4294971706, "clang::BuiltinType::RvvInt64m4x2"),
      add_named_node(&mut g, 4294971707, "clang::BuiltinType::RvvUint64m1x2"),
      add_named_node(&mut g, 4294971708, "clang::BuiltinType::RvvUint64m1x3"),
      add_named_node(&mut g, 4294971709, "clang::BuiltinType::RvvUint64m1x4"),
      add_named_node(&mut g, 4294971710, "clang::BuiltinType::RvvUint64m1x5"),
      add_named_node(&mut g, 4294971711, "clang::BuiltinType::RvvUint64m1x6"),
      add_named_node(&mut g, 4294971712, "clang::BuiltinType::RvvUint64m1x7"),
      add_named_node(&mut g, 4294971713, "clang::BuiltinType::RvvUint64m1x8"),
      add_named_node(&mut g, 4294971714, "clang::BuiltinType::RvvUint64m2x2"),
      add_named_node(&mut g, 4294971715, "clang::BuiltinType::RvvUint64m2x3"),
      add_named_node(&mut g, 4294971716, "clang::BuiltinType::RvvUint64m2x4"),
      add_named_node(&mut g, 4294971717, "clang::BuiltinType::RvvUint64m4x2"),
      add_named_node(&mut g, 4294971718, "clang::BuiltinType::RvvFloat16mf4x2"),
      add_named_node(&mut g, 4294971719, "clang::BuiltinType::RvvFloat16mf4x3"),
      add_named_node(&mut g, 4294971720, "clang::BuiltinType::RvvFloat16mf4x4"),
      add_named_node(&mut g, 4294971721, "clang::BuiltinType::RvvFloat16mf4x5"),
      add_named_node(&mut g, 4294971722, "clang::BuiltinType::RvvFloat16mf4x6"),
      add_named_node(&mut g, 4294971723, "clang::BuiltinType::RvvFloat16mf4x7"),
      add_named_node(&mut g, 4294971724, "clang::BuiltinType::RvvFloat16mf4x8"),
      add_named_node(&mut g, 4294971725, "clang::BuiltinType::RvvFloat16mf2x2"),
      add_named_node(&mut g, 4294971726, "clang::BuiltinType::RvvFloat16mf2x3"),
      add_named_node(&mut g, 4294971727, "clang::BuiltinType::RvvFloat16mf2x4"),
      add_named_node(&mut g, 4294971728, "clang::BuiltinType::RvvFloat16mf2x5"),
      add_named_node(&mut g, 4294971729, "clang::BuiltinType::RvvFloat16mf2x6"),
      add_named_node(&mut g, 4294971730, "clang::BuiltinType::RvvFloat16mf2x7"),
      add_named_node(&mut g, 4294971731, "clang::BuiltinType::RvvFloat16mf2x8"),
      add_named_node(&mut g, 4294971732, "clang::BuiltinType::RvvFloat16m1x2"),
      add_named_node(&mut g, 4294971733, "clang::BuiltinType::RvvFloat16m1x3"),
      add_named_node(&mut g, 4294971734, "clang::BuiltinType::RvvFloat16m1x4"),
      add_named_node(&mut g, 4294971735, "clang::BuiltinType::RvvFloat16m1x5"),
      add_named_node(&mut g, 4294971736, "clang::BuiltinType::RvvFloat16m1x6"),
      add_named_node(&mut g, 4294971737, "clang::BuiltinType::RvvFloat16m1x7"),
      add_named_node(&mut g, 4294971738, "clang::BuiltinType::RvvFloat16m1x8"),
      add_named_node(&mut g, 4294971739, "clang::BuiltinType::RvvFloat16m2x2"),
      add_named_node(&mut g, 4294971740, "clang::BuiltinType::RvvFloat16m2x3"),
      add_named_node(&mut g, 4294971741, "clang::BuiltinType::RvvFloat16m2x4"),
      add_named_node(&mut g, 4294971742, "clang::BuiltinType::RvvFloat16m4x2"),
      add_named_node(&mut g, 4294971743, "clang::BuiltinType::RvvFloat32mf2x2"),
      add_named_node(&mut g, 4294971744, "clang::BuiltinType::RvvFloat32mf2x3"),
      add_named_node(&mut g, 4294971745, "clang::BuiltinType::RvvFloat32mf2x4"),
      add_named_node(&mut g, 4294971746, "clang::BuiltinType::RvvFloat32mf2x5"),
      add_named_node(&mut g, 4294971747, "clang::BuiltinType::RvvFloat32mf2x6"),
      add_named_node(&mut g, 4294971748, "clang::BuiltinType::RvvFloat32mf2x7"),
      add_named_node(&mut g, 4294971749, "clang::BuiltinType::RvvFloat32mf2x8"),
      add_named_node(&mut g, 4294971750, "clang::BuiltinType::RvvFloat32m1x2"),
      add_named_node(&mut g, 4294971751, "clang::BuiltinType::RvvFloat32m1x3"),
      add_named_node(&mut g, 4294971752, "clang::BuiltinType::RvvFloat32m1x4"),
      add_named_node(&mut g, 4294971753, "clang::BuiltinType::RvvFloat32m1x5"),
      add_named_node(&mut g, 4294971754, "clang::BuiltinType::RvvFloat32m1x6"),
      add_named_node(&mut g, 4294971755, "clang::BuiltinType::RvvFloat32m1x7"),
      add_named_node(&mut g, 4294971756, "clang::BuiltinType::RvvFloat32m1x8"),
      add_named_node(&mut g, 4294971757, "clang::BuiltinType::RvvFloat32m2x2"),
      add_named_node(&mut g, 4294971758, "clang::BuiltinType::RvvFloat32m2x3"),
      add_named_node(&mut g, 4294971759, "clang::BuiltinType::RvvFloat32m2x4"),
      add_named_node(&mut g, 4294971760, "clang::BuiltinType::RvvFloat32m4x2"),
      add_named_node(&mut g, 4294971761, "clang::BuiltinType::RvvFloat64m1x2"),
      add_named_node(&mut g, 4294971762, "clang::BuiltinType::RvvFloat64m1x3"),
      add_named_node(&mut g, 4294971763, "clang::BuiltinType::RvvFloat64m1x4"),
      add_named_node(&mut g, 4294971764, "clang::BuiltinType::RvvFloat64m1x5"),
      add_named_node(&mut g, 4294971765, "clang::BuiltinType::RvvFloat64m1x6"),
      add_named_node(&mut g, 4294971766, "clang::BuiltinType::RvvFloat64m1x7"),
      add_named_node(&mut g, 4294971767, "clang::BuiltinType::RvvFloat64m1x8"),
      add_named_node(&mut g, 4294971768, "clang::BuiltinType::RvvFloat64m2x2"),
      add_named_node(&mut g, 4294971769, "clang::BuiltinType::RvvFloat64m2x3"),
      add_named_node(&mut g, 4294971770, "clang::BuiltinType::RvvFloat64m2x4"),
      add_named_node(&mut g, 4294971771, "clang::BuiltinType::RvvFloat64m4x2"),
      add_named_node(&mut g, 4294971772, "clang::BuiltinType::RvvBFloat16mf4x2"),
      add_named_node(&mut g, 4294971773, "clang::BuiltinType::RvvBFloat16mf4x3"),
      add_named_node(&mut g, 4294971774, "clang::BuiltinType::RvvBFloat16mf4x4"),
      add_named_node(&mut g, 4294971775, "clang::BuiltinType::RvvBFloat16mf4x5"),
      add_named_node(&mut g, 4294971776, "clang::BuiltinType::RvvBFloat16mf4x6"),
      add_named_node(&mut g, 4294971777, "clang::BuiltinType::RvvBFloat16mf4x7"),
      add_named_node(&mut g, 4294971778, "clang::BuiltinType::RvvBFloat16mf4x8"),
      add_named_node(&mut g, 4294971779, "clang::BuiltinType::RvvBFloat16mf2x2"),
      add_named_node(&mut g, 4294971780, "clang::BuiltinType::RvvBFloat16mf2x3"),
      add_named_node(&mut g, 4294971781, "clang::BuiltinType::RvvBFloat16mf2x4"),
      add_named_node(&mut g, 4294971782, "clang::BuiltinType::RvvBFloat16mf2x5"),
      add_named_node(&mut g, 4294971783, "clang::BuiltinType::RvvBFloat16mf2x6"),
      add_named_node(&mut g, 4294971784, "clang::BuiltinType::RvvBFloat16mf2x7"),
      add_named_node(&mut g, 4294971785, "clang::BuiltinType::RvvBFloat16mf2x8"),
      add_named_node(&mut g, 4294971786, "clang::BuiltinType::RvvBFloat16m1x2"),
      add_named_node(&mut g, 4294971787, "clang::BuiltinType::RvvBFloat16m1x3"),
      add_named_node(&mut g, 4294971788, "clang::BuiltinType::RvvBFloat16m1x4"),
      add_named_node(&mut g, 4294971789, "clang::BuiltinType::RvvBFloat16m1x5"),
      add_named_node(&mut g, 4294971790, "clang::BuiltinType::RvvBFloat16m1x6"),
      add_named_node(&mut g, 4294971791, "clang::BuiltinType::RvvBFloat16m1x7"),
      add_named_node(&mut g, 4294971792, "clang::BuiltinType::RvvBFloat16m1x8"),
      add_named_node(&mut g, 4294971793, "clang::BuiltinType::RvvBFloat16m2x2"),
      add_named_node(&mut g, 4294971794, "clang::BuiltinType::RvvBFloat16m2x3"),
      add_named_node(&mut g, 4294971795, "clang::BuiltinType::RvvBFloat16m2x4"),
      add_named_node(&mut g, 4294971796, "clang::BuiltinType::RvvBFloat16m4x2"),
      add_named_node(&mut g, 4294971797, "clang::BuiltinType::WasmExternRef"),
      add_named_node(&mut g, 4294971798, "clang::BuiltinType::Void"),
      add_named_node(&mut g, 4294971799, "clang::BuiltinType::Bool"),
      add_named_node(&mut g, 4294971800, "clang::BuiltinType::Char_U"),
      add_named_node(&mut g, 4294971801, "clang::BuiltinType::UChar"),
      add_named_node(&mut g, 4294971802, "clang::BuiltinType::WChar_U"),
      add_named_node(&mut g, 4294971803, "clang::BuiltinType::Char8"),
      add_named_node(&mut g, 4294971804, "clang::BuiltinType::Char16"),
      add_named_node(&mut g, 4294971805, "clang::BuiltinType::Char32"),
      add_named_node(&mut g, 4294971806, "clang::BuiltinType::UShort"),
      add_named_node(&mut g, 4294971807, "clang::BuiltinType::UInt"),
      add_named_node(&mut g, 4294971808, "clang::BuiltinType::ULong"),
      add_named_node(&mut g, 4294971809, "clang::BuiltinType::ULongLong"),
      add_named_node(&mut g, 4294971810, "clang::BuiltinType::UInt128"),
      add_named_node(&mut g, 4294971811, "clang::BuiltinType::Char_S"),
      add_named_node(&mut g, 4294971812, "clang::BuiltinType::SChar"),
      add_named_node(&mut g, 4294971813, "clang::BuiltinType::WChar_S"),
      add_named_node(&mut g, 4294971814, "clang::BuiltinType::Short"),
      add_named_node(&mut g, 4294971815, "clang::BuiltinType::Int"),
      add_named_node(&mut g, 4294971816, "clang::BuiltinType::Long"),
      add_named_node(&mut g, 4294971817, "clang::BuiltinType::LongLong"),
      add_named_node(&mut g, 4294971818, "clang::BuiltinType::Int128"),
      add_named_node(&mut g, 4294971819, "clang::BuiltinType::ShortAccum"),
      add_named_node(&mut g, 4294971820, "clang::BuiltinType::Accum"),
      add_named_node(&mut g, 4294971821, "clang::BuiltinType::LongAccum"),
      add_named_node(&mut g, 4294971822, "clang::BuiltinType::UShortAccum"),
      add_named_node(&mut g, 4294971823, "clang::BuiltinType::UAccum"),
      add_named_node(&mut g, 4294971824, "clang::BuiltinType::ULongAccum"),
      add_named_node(&mut g, 4294971825, "clang::BuiltinType::ShortFract"),
      add_named_node(&mut g, 4294971826, "clang::BuiltinType::Fract"),
      add_named_node(&mut g, 4294971827, "clang::BuiltinType::LongFract"),
      add_named_node(&mut g, 4294971828, "clang::BuiltinType::UShortFract"),
      add_named_node(&mut g, 4294971829, "clang::BuiltinType::UFract"),
      add_named_node(&mut g, 4294971830, "clang::BuiltinType::ULongFract"),
      add_named_node(&mut g, 4294971831, "clang::BuiltinType::SatShortAccum"),
      add_named_node(&mut g, 4294971832, "clang::BuiltinType::SatAccum"),
      add_named_node(&mut g, 4294971833, "clang::BuiltinType::SatLongAccum"),
      add_named_node(&mut g, 4294971834, "clang::BuiltinType::SatUShortAccum"),
      add_named_node(&mut g, 4294971835, "clang::BuiltinType::SatUAccum"),
      add_named_node(&mut g, 4294971836, "clang::BuiltinType::SatULongAccum"),
      add_named_node(&mut g, 4294971837, "clang::BuiltinType::SatShortFract"),
      add_named_node(&mut g, 4294971838, "clang::BuiltinType::SatFract"),
      add_named_node(&mut g, 4294971839, "clang::BuiltinType::SatLongFract"),
      add_named_node(&mut g, 4294971840, "clang::BuiltinType::SatUShortFract"),
      add_named_node(&mut g, 4294971841, "clang::BuiltinType::SatUFract"),
      add_named_node(&mut g, 4294971842, "clang::BuiltinType::SatULongFract"),
      add_named_node(&mut g, 4294971843, "clang::BuiltinType::Half"),
      add_named_node(&mut g, 4294971844, "clang::BuiltinType::Float"),
      add_named_node(&mut g, 4294971845, "clang::BuiltinType::Double"),
      add_named_node(&mut g, 4294971846, "clang::BuiltinType::LongDouble"),
      add_named_node(&mut g, 4294971847, "clang::BuiltinType::Float16"),
      add_named_node(&mut g, 4294971848, "clang::BuiltinType::BFloat16"),
      add_named_node(&mut g, 4294971849, "clang::BuiltinType::Float128"),
      add_named_node(&mut g, 4294971850, "clang::BuiltinType::Ibm128"),
      add_named_node(&mut g, 4294971851, "clang::BuiltinType::NullPtr"),
      add_named_node(&mut g, 4294971852, "clang::BuiltinType::ObjCId"),
      add_named_node(&mut g, 4294971853, "clang::BuiltinType::ObjCClass"),
      add_named_node(&mut g, 4294971854, "clang::BuiltinType::ObjCSel"),
      add_named_node(&mut g, 4294971855, "clang::BuiltinType::OCLSampler"),
      add_named_node(&mut g, 4294971856, "clang::BuiltinType::OCLEvent"),
      add_named_node(&mut g, 4294971857, "clang::BuiltinType::OCLClkEvent"),
      add_named_node(&mut g, 4294971858, "clang::BuiltinType::OCLQueue"),
      add_named_node(&mut g, 4294971859, "clang::BuiltinType::OCLReserveID"),
      add_named_node(&mut g, 4294971860, "clang::BuiltinType::Dependent"),
      add_named_node(&mut g, 4294971861, "clang::BuiltinType::Overload"),
      add_named_node(&mut g, 4294971862, "clang::BuiltinType::BoundMember"),
      add_named_node(&mut g, 4294971863, "clang::BuiltinType::PseudoObject"),
      add_named_node(&mut g, 4294971864, "clang::BuiltinType::UnknownAny"),
      add_named_node(&mut g, 4294971865, "clang::BuiltinType::BuiltinFn"),
      add_named_node(&mut g, 4294971866, "clang::BuiltinType::ARCUnbridgedCast"),
      add_named_node(&mut g, 4294971867, "clang::BuiltinType::IncompleteMatrixIdx"),
      add_named_node(&mut g, 4294971868, "clang::BuiltinType::OMPArraySection"),
      add_named_node(&mut g, 4294971869, "clang::BuiltinType::OMPArrayShaping"),
      add_named_node(&mut g, 4294971870, "clang::BuiltinType::OMPIterator"),
      add_named_node(&mut g, 4294971871, "clang::BuiltinType::LastKind"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_KIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_CXXCONSTRUCTIONKIND, "clang::CXXConstructionKind");
  g.add_edge((ENUM_CXXCONSTRUCTIONKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294971873, "clang::CXXConstructionKind::Complete"),
      add_named_node(&mut g, 4294971874, "clang::CXXConstructionKind::NonVirtualBase"),
      add_named_node(&mut g, 4294971875, "clang::CXXConstructionKind::VirtualBase"),
      add_named_node(&mut g, 4294971876, "clang::CXXConstructionKind::Delegating"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_CXXCONSTRUCTIONKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_CXXNEWINITIALIZATIONSTYLE, "clang::CXXNewInitializationStyle");
  g.add_edge((ENUM_CXXNEWINITIALIZATIONSTYLE, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294971878, "clang::CXXNewInitializationStyle::None"),
      add_named_node(&mut g, 4294971879, "clang::CXXNewInitializationStyle::Parens"),
      add_named_node(&mut g, 4294971880, "clang::CXXNewInitializationStyle::Braces"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_CXXNEWINITIALIZATIONSTYLE, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_ADLCALLKIND, "clang::CallExpr::ADLCallKind");
  g.add_edge((ENUM_ADLCALLKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294971882, "clang::CallExpr::ADLCallKind::NotADL"),
      add_named_node(&mut g, 4294971883, "clang::CallExpr::ADLCallKind::UsesADL"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_ADLCALLKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_CALLINGCONV, "clang::CallingConv");
  g.add_edge((ENUM_CALLINGCONV, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294971885, "clang::CC_C"),
      add_named_node(&mut g, 4294971886, "clang::CC_X86StdCall"),
      add_named_node(&mut g, 4294971887, "clang::CC_X86FastCall"),
      add_named_node(&mut g, 4294971888, "clang::CC_X86ThisCall"),
      add_named_node(&mut g, 4294971889, "clang::CC_X86VectorCall"),
      add_named_node(&mut g, 4294971890, "clang::CC_X86Pascal"),
      add_named_node(&mut g, 4294971891, "clang::CC_Win64"),
      add_named_node(&mut g, 4294971892, "clang::CC_X86_64SysV"),
      add_named_node(&mut g, 4294971893, "clang::CC_X86RegCall"),
      add_named_node(&mut g, 4294971894, "clang::CC_AAPCS"),
      add_named_node(&mut g, 4294971895, "clang::CC_AAPCS_VFP"),
      add_named_node(&mut g, 4294971896, "clang::CC_IntelOclBicc"),
      add_named_node(&mut g, 4294971897, "clang::CC_SpirFunction"),
      add_named_node(&mut g, 4294971898, "clang::CC_OpenCLKernel"),
      add_named_node(&mut g, 4294971899, "clang::CC_Swift"),
      add_named_node(&mut g, 4294971900, "clang::CC_SwiftAsync"),
      add_named_node(&mut g, 4294971901, "clang::CC_PreserveMost"),
      add_named_node(&mut g, 4294971902, "clang::CC_PreserveAll"),
      add_named_node(&mut g, 4294971903, "clang::CC_AArch64VectorCall"),
      add_named_node(&mut g, 4294971904, "clang::CC_AArch64SVEPCS"),
      add_named_node(&mut g, 4294971905, "clang::CC_AMDGPUKernelCall"),
      add_named_node(&mut g, 4294971906, "clang::CC_M68kRTD"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_CALLINGCONV, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_CANTHROWRESULT, "clang::CanThrowResult");
  g.add_edge((ENUM_CANTHROWRESULT, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294971908, "clang::CT_Cannot"),
      add_named_node(&mut g, 4294971909, "clang::CT_Dependent"),
      add_named_node(&mut g, 4294971910, "clang::CT_Can"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_CANTHROWRESULT, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_CAPTUREDREGIONKIND, "clang::CapturedRegionKind");
  g.add_edge((ENUM_CAPTUREDREGIONKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294971912, "clang::CR_Default"),
      add_named_node(&mut g, 4294971913, "clang::CR_ObjCAtFinally"),
      add_named_node(&mut g, 4294971914, "clang::CR_OpenMP"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_CAPTUREDREGIONKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_CASTKIND, "clang::CastKind");
  g.add_edge((ENUM_CASTKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294971916, "clang::CK_Dependent"),
      add_named_node(&mut g, 4294971917, "clang::CK_BitCast"),
      add_named_node(&mut g, 4294971918, "clang::CK_LValueBitCast"),
      add_named_node(&mut g, 4294971919, "clang::CK_LValueToRValueBitCast"),
      add_named_node(&mut g, 4294971920, "clang::CK_LValueToRValue"),
      add_named_node(&mut g, 4294971921, "clang::CK_NoOp"),
      add_named_node(&mut g, 4294971922, "clang::CK_BaseToDerived"),
      add_named_node(&mut g, 4294971923, "clang::CK_DerivedToBase"),
      add_named_node(&mut g, 4294971924, "clang::CK_UncheckedDerivedToBase"),
      add_named_node(&mut g, 4294971925, "clang::CK_Dynamic"),
      add_named_node(&mut g, 4294971926, "clang::CK_ToUnion"),
      add_named_node(&mut g, 4294971927, "clang::CK_ArrayToPointerDecay"),
      add_named_node(&mut g, 4294971928, "clang::CK_FunctionToPointerDecay"),
      add_named_node(&mut g, 4294971929, "clang::CK_NullToPointer"),
      add_named_node(&mut g, 4294971930, "clang::CK_NullToMemberPointer"),
      add_named_node(&mut g, 4294971931, "clang::CK_BaseToDerivedMemberPointer"),
      add_named_node(&mut g, 4294971932, "clang::CK_DerivedToBaseMemberPointer"),
      add_named_node(&mut g, 4294971933, "clang::CK_MemberPointerToBoolean"),
      add_named_node(&mut g, 4294971934, "clang::CK_ReinterpretMemberPointer"),
      add_named_node(&mut g, 4294971935, "clang::CK_UserDefinedConversion"),
      add_named_node(&mut g, 4294971936, "clang::CK_ConstructorConversion"),
      add_named_node(&mut g, 4294971937, "clang::CK_IntegralToPointer"),
      add_named_node(&mut g, 4294971938, "clang::CK_PointerToIntegral"),
      add_named_node(&mut g, 4294971939, "clang::CK_PointerToBoolean"),
      add_named_node(&mut g, 4294971940, "clang::CK_ToVoid"),
      add_named_node(&mut g, 4294971941, "clang::CK_MatrixCast"),
      add_named_node(&mut g, 4294971942, "clang::CK_VectorSplat"),
      add_named_node(&mut g, 4294971943, "clang::CK_IntegralCast"),
      add_named_node(&mut g, 4294971944, "clang::CK_IntegralToBoolean"),
      add_named_node(&mut g, 4294971945, "clang::CK_IntegralToFloating"),
      add_named_node(&mut g, 4294971946, "clang::CK_FloatingToFixedPoint"),
      add_named_node(&mut g, 4294971947, "clang::CK_FixedPointToFloating"),
      add_named_node(&mut g, 4294971948, "clang::CK_FixedPointCast"),
      add_named_node(&mut g, 4294971949, "clang::CK_FixedPointToIntegral"),
      add_named_node(&mut g, 4294971950, "clang::CK_IntegralToFixedPoint"),
      add_named_node(&mut g, 4294971951, "clang::CK_FixedPointToBoolean"),
      add_named_node(&mut g, 4294971952, "clang::CK_FloatingToIntegral"),
      add_named_node(&mut g, 4294971953, "clang::CK_FloatingToBoolean"),
      add_named_node(&mut g, 4294971954, "clang::CK_BooleanToSignedIntegral"),
      add_named_node(&mut g, 4294971955, "clang::CK_FloatingCast"),
      add_named_node(&mut g, 4294971956, "clang::CK_CPointerToObjCPointerCast"),
      add_named_node(&mut g, 4294971957, "clang::CK_BlockPointerToObjCPointerCast"),
      add_named_node(&mut g, 4294971958, "clang::CK_AnyPointerToBlockPointerCast"),
      add_named_node(&mut g, 4294971959, "clang::CK_ObjCObjectLValueCast"),
      add_named_node(&mut g, 4294971960, "clang::CK_FloatingRealToComplex"),
      add_named_node(&mut g, 4294971961, "clang::CK_FloatingComplexToReal"),
      add_named_node(&mut g, 4294971962, "clang::CK_FloatingComplexToBoolean"),
      add_named_node(&mut g, 4294971963, "clang::CK_FloatingComplexCast"),
      add_named_node(&mut g, 4294971964, "clang::CK_FloatingComplexToIntegralComplex"),
      add_named_node(&mut g, 4294971965, "clang::CK_IntegralRealToComplex"),
      add_named_node(&mut g, 4294971966, "clang::CK_IntegralComplexToReal"),
      add_named_node(&mut g, 4294971967, "clang::CK_IntegralComplexToBoolean"),
      add_named_node(&mut g, 4294971968, "clang::CK_IntegralComplexCast"),
      add_named_node(&mut g, 4294971969, "clang::CK_IntegralComplexToFloatingComplex"),
      add_named_node(&mut g, 4294971970, "clang::CK_ARCProduceObject"),
      add_named_node(&mut g, 4294971971, "clang::CK_ARCConsumeObject"),
      add_named_node(&mut g, 4294971972, "clang::CK_ARCReclaimReturnedObject"),
      add_named_node(&mut g, 4294971973, "clang::CK_ARCExtendBlockObject"),
      add_named_node(&mut g, 4294971974, "clang::CK_AtomicToNonAtomic"),
      add_named_node(&mut g, 4294971975, "clang::CK_NonAtomicToAtomic"),
      add_named_node(&mut g, 4294971976, "clang::CK_CopyAndAutoreleaseBlockObject"),
      add_named_node(&mut g, 4294971977, "clang::CK_BuiltinFnToFnPtr"),
      add_named_node(&mut g, 4294971978, "clang::CK_ZeroToOCLOpaqueType"),
      add_named_node(&mut g, 4294971979, "clang::CK_AddressSpaceConversion"),
      add_named_node(&mut g, 4294971980, "clang::CK_IntToOCLSampler"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_CASTKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_CHARACTERLITERALKIND, "clang::CharacterLiteralKind");
  g.add_edge((ENUM_CHARACTERLITERALKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294971982, "clang::CharacterLiteralKind::Ascii"),
      add_named_node(&mut g, 4294971983, "clang::CharacterLiteralKind::Wide"),
      add_named_node(&mut g, 4294971984, "clang::CharacterLiteralKind::UTF8"),
      add_named_node(&mut g, 4294971985, "clang::CharacterLiteralKind::UTF16"),
      add_named_node(&mut g, 4294971986, "clang::CharacterLiteralKind::UTF32"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_CHARACTERLITERALKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_CONSTANTRESULTSTORAGEKIND, "clang::ConstantResultStorageKind");
  g.add_edge((ENUM_CONSTANTRESULTSTORAGEKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294971988, "clang::ConstantResultStorageKind::None"),
      add_named_node(&mut g, 4294971989, "clang::ConstantResultStorageKind::Int64"),
      add_named_node(&mut g, 4294971990, "clang::ConstantResultStorageKind::APValue"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_CONSTANTRESULTSTORAGEKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_CONSTEXPRSPECKIND, "clang::ConstexprSpecKind");
  g.add_edge((ENUM_CONSTEXPRSPECKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294971992, "clang::ConstexprSpecKind::Unspecified"),
      add_named_node(&mut g, 4294971993, "clang::ConstexprSpecKind::Constexpr"),
      add_named_node(&mut g, 4294971994, "clang::ConstexprSpecKind::Consteval"),
      add_named_node(&mut g, 4294971995, "clang::ConstexprSpecKind::Constinit"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_CONSTEXPRSPECKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_FRIENDOBJECTKIND, "clang::Decl::FriendObjectKind");
  g.add_edge((ENUM_FRIENDOBJECTKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294971997, "clang::Decl::FOK_None"),
      add_named_node(&mut g, 4294971998, "clang::Decl::FOK_Declared"),
      add_named_node(&mut g, 4294971999, "clang::Decl::FOK_Undeclared"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_FRIENDOBJECTKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_MODULEOWNERSHIPKIND, "clang::Decl::ModuleOwnershipKind");
  g.add_edge((ENUM_MODULEOWNERSHIPKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972001, "clang::Decl::ModuleOwnershipKind::Unowned"),
      add_named_node(&mut g, 4294972002, "clang::Decl::ModuleOwnershipKind::Visible"),
      add_named_node(&mut g, 4294972003, "clang::Decl::ModuleOwnershipKind::VisibleWhenImported"),
      add_named_node(&mut g, 4294972004, "clang::Decl::ModuleOwnershipKind::ReachableWhenImported"),
      add_named_node(&mut g, 4294972005, "clang::Decl::ModuleOwnershipKind::ModulePrivate"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_MODULEOWNERSHIPKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_OBJCDECLQUALIFIER, "clang::Decl::ObjCDeclQualifier");
  g.add_edge((ENUM_OBJCDECLQUALIFIER, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972007, "clang::Decl::OBJC_TQ_None"),
      add_named_node(&mut g, 4294972008, "clang::Decl::OBJC_TQ_In"),
      add_named_node(&mut g, 4294972009, "clang::Decl::OBJC_TQ_Inout"),
      add_named_node(&mut g, 4294972010, "clang::Decl::OBJC_TQ_Out"),
      add_named_node(&mut g, 4294972011, "clang::Decl::OBJC_TQ_Bycopy"),
      add_named_node(&mut g, 4294972012, "clang::Decl::OBJC_TQ_Byref"),
      add_named_node(&mut g, 4294972013, "clang::Decl::OBJC_TQ_Oneway"),
      add_named_node(&mut g, 4294972014, "clang::Decl::OBJC_TQ_CSNullability"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_OBJCDECLQUALIFIER, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_DEDUCTIONCANDIDATE, "clang::DeductionCandidate");
  g.add_edge((ENUM_DEDUCTIONCANDIDATE, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972016, "clang::DeductionCandidate::Normal"),
      add_named_node(&mut g, 4294972017, "clang::DeductionCandidate::Copy"),
      add_named_node(&mut g, 4294972018, "clang::DeductionCandidate::Aggregate"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_DEDUCTIONCANDIDATE, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_ELABORATEDTYPEKEYWORD, "clang::ElaboratedTypeKeyword");
  g.add_edge((ENUM_ELABORATEDTYPEKEYWORD, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972020, "clang::ElaboratedTypeKeyword::Struct"),
      add_named_node(&mut g, 4294972021, "clang::ElaboratedTypeKeyword::Interface"),
      add_named_node(&mut g, 4294972022, "clang::ElaboratedTypeKeyword::Union"),
      add_named_node(&mut g, 4294972023, "clang::ElaboratedTypeKeyword::Class"),
      add_named_node(&mut g, 4294972024, "clang::ElaboratedTypeKeyword::Enum"),
      add_named_node(&mut g, 4294972025, "clang::ElaboratedTypeKeyword::Typename"),
      add_named_node(&mut g, 4294972026, "clang::ElaboratedTypeKeyword::None"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_ELABORATEDTYPEKEYWORD, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_EXCEPTIONSPECIFICATIONTYPE, "clang::ExceptionSpecificationType");
  g.add_edge((ENUM_EXCEPTIONSPECIFICATIONTYPE, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972028, "clang::EST_None"),
      add_named_node(&mut g, 4294972029, "clang::EST_DynamicNone"),
      add_named_node(&mut g, 4294972030, "clang::EST_Dynamic"),
      add_named_node(&mut g, 4294972031, "clang::EST_MSAny"),
      add_named_node(&mut g, 4294972032, "clang::EST_NoThrow"),
      add_named_node(&mut g, 4294972033, "clang::EST_BasicNoexcept"),
      add_named_node(&mut g, 4294972034, "clang::EST_DependentNoexcept"),
      add_named_node(&mut g, 4294972035, "clang::EST_NoexceptFalse"),
      add_named_node(&mut g, 4294972036, "clang::EST_NoexceptTrue"),
      add_named_node(&mut g, 4294972037, "clang::EST_Unevaluated"),
      add_named_node(&mut g, 4294972038, "clang::EST_Uninstantiated"),
      add_named_node(&mut g, 4294972039, "clang::EST_Unparsed"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_EXCEPTIONSPECIFICATIONTYPE, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_EXPRDEPENDENCE, "clang::ExprDependenceScope::ExprDependence");
  g.add_edge((ENUM_EXPRDEPENDENCE, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972041, "clang::ExprDependenceScope::UnexpandedPack"),
      add_named_node(&mut g, 4294972042, "clang::ExprDependenceScope::Instantiation"),
      add_named_node(&mut g, 4294972043, "clang::ExprDependenceScope::Type"),
      add_named_node(&mut g, 4294972044, "clang::ExprDependenceScope::Value"),
      add_named_node(&mut g, 4294972045, "clang::ExprDependenceScope::Error"),
      add_named_node(&mut g, 4294972046, "clang::ExprDependenceScope::None"),
      add_named_node(&mut g, 4294972047, "clang::ExprDependenceScope::All"),
      add_named_node(&mut g, 4294972048, "clang::ExprDependenceScope::TypeValue"),
      add_named_node(&mut g, 4294972049, "clang::ExprDependenceScope::TypeInstantiation"),
      add_named_node(&mut g, 4294972050, "clang::ExprDependenceScope::ValueInstantiation"),
      add_named_node(&mut g, 4294972051, "clang::ExprDependenceScope::TypeValueInstantiation"),
      add_named_node(&mut g, 4294972052, "clang::ExprDependenceScope::ErrorDependent"),
      add_named_node(&mut g, 4294972053, "clang::ExprDependenceScope::LLVM_BITMASK_LARGEST_ENUMERATOR"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_EXPRDEPENDENCE, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_EXPROBJECTKIND, "clang::ExprObjectKind");
  g.add_edge((ENUM_EXPROBJECTKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972055, "clang::OK_Ordinary"),
      add_named_node(&mut g, 4294972056, "clang::OK_BitField"),
      add_named_node(&mut g, 4294972057, "clang::OK_VectorComponent"),
      add_named_node(&mut g, 4294972058, "clang::OK_ObjCProperty"),
      add_named_node(&mut g, 4294972059, "clang::OK_ObjCSubscript"),
      add_named_node(&mut g, 4294972060, "clang::OK_MatrixComponent"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_EXPROBJECTKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_EXPRVALUEKIND, "clang::ExprValueKind");
  g.add_edge((ENUM_EXPRVALUEKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972062, "clang::VK_PRValue"),
      add_named_node(&mut g, 4294972063, "clang::VK_LValue"),
      add_named_node(&mut g, 4294972064, "clang::VK_XValue"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_EXPRVALUEKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_EXPRESSIONTRAIT, "clang::ExpressionTrait");
  g.add_edge((ENUM_EXPRESSIONTRAIT, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972066, "clang::ET_IsLValueExpr"),
      add_named_node(&mut g, 4294972067, "clang::ET_IsRValueExpr"),
      add_named_node(&mut g, 4294972068, "clang::ET_Last"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_EXPRESSIONTRAIT, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_TEMPLATEDKIND, "clang::FunctionDecl::TemplatedKind");
  g.add_edge((ENUM_TEMPLATEDKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972070, "clang::FunctionDecl::TK_NonTemplate"),
      add_named_node(&mut g, 4294972071, "clang::FunctionDecl::TK_FunctionTemplate"),
      add_named_node(&mut g, 4294972072, "clang::FunctionDecl::TK_MemberSpecialization"),
      add_named_node(&mut g, 4294972073, "clang::FunctionDecl::TK_FunctionTemplateSpecialization"),
      add_named_node(&mut g, 4294972074, "clang::FunctionDecl::TK_DependentFunctionTemplateSpecialization"),
      add_named_node(&mut g, 4294972075, "clang::FunctionDecl::TK_DependentNonTemplate"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_TEMPLATEDKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_IFSTATEMENTKIND, "clang::IfStatementKind");
  g.add_edge((ENUM_IFSTATEMENTKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972077, "clang::IfStatementKind::Ordinary"),
      add_named_node(&mut g, 4294972078, "clang::IfStatementKind::Constexpr"),
      add_named_node(&mut g, 4294972079, "clang::IfStatementKind::ConstevalNonNegated"),
      add_named_node(&mut g, 4294972080, "clang::IfStatementKind::ConstevalNegated"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_IFSTATEMENTKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_IMPLICITPARAMKIND, "clang::ImplicitParamKind");
  g.add_edge((ENUM_IMPLICITPARAMKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972082, "clang::ImplicitParamKind::ObjCSelf"),
      add_named_node(&mut g, 4294972083, "clang::ImplicitParamKind::ObjCCmd"),
      add_named_node(&mut g, 4294972084, "clang::ImplicitParamKind::CXXThis"),
      add_named_node(&mut g, 4294972085, "clang::ImplicitParamKind::CXXVTT"),
      add_named_node(&mut g, 4294972086, "clang::ImplicitParamKind::CapturedContext"),
      add_named_node(&mut g, 4294972087, "clang::ImplicitParamKind::ThreadPrivateVar"),
      add_named_node(&mut g, 4294972088, "clang::ImplicitParamKind::Other"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_IMPLICITPARAMKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_INCLASSINITSTYLE, "clang::InClassInitStyle");
  g.add_edge((ENUM_INCLASSINITSTYLE, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972090, "clang::ICIS_NoInit"),
      add_named_node(&mut g, 4294972091, "clang::ICIS_CopyInit"),
      add_named_node(&mut g, 4294972092, "clang::ICIS_ListInit"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_INCLASSINITSTYLE, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_LAMBDACAPTUREDEFAULT, "clang::LambdaCaptureDefault");
  g.add_edge((ENUM_LAMBDACAPTUREDEFAULT, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972094, "clang::LCD_None"),
      add_named_node(&mut g, 4294972095, "clang::LCD_ByCopy"),
      add_named_node(&mut g, 4294972096, "clang::LCD_ByRef"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_LAMBDACAPTUREDEFAULT, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_LANGUAGELINKAGE, "clang::LanguageLinkage");
  g.add_edge((ENUM_LANGUAGELINKAGE, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972098, "clang::CLanguageLinkage"),
      add_named_node(&mut g, 4294972099, "clang::CXXLanguageLinkage"),
      add_named_node(&mut g, 4294972100, "clang::NoLanguageLinkage"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_LANGUAGELINKAGE, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_LINKAGE, "clang::Linkage");
  g.add_edge((ENUM_LINKAGE, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972102, "clang::Linkage::Invalid"),
      add_named_node(&mut g, 4294972103, "clang::Linkage::None"),
      add_named_node(&mut g, 4294972104, "clang::Linkage::Internal"),
      add_named_node(&mut g, 4294972105, "clang::Linkage::UniqueExternal"),
      add_named_node(&mut g, 4294972106, "clang::Linkage::VisibleNone"),
      add_named_node(&mut g, 4294972107, "clang::Linkage::Module"),
      add_named_node(&mut g, 4294972108, "clang::Linkage::External"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_LINKAGE, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_LINKAGESPECLANGUAGEIDS, "clang::LinkageSpecLanguageIDs");
  g.add_edge((ENUM_LINKAGESPECLANGUAGEIDS, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972110, "clang::LinkageSpecLanguageIDs::C"),
      add_named_node(&mut g, 4294972111, "clang::LinkageSpecLanguageIDs::CXX"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_LINKAGESPECLANGUAGEIDS, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_MSVTORDISPMODE, "clang::MSVtorDispMode");
  g.add_edge((ENUM_MSVTORDISPMODE, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972113, "clang::MSVtorDispMode::Never"),
      add_named_node(&mut g, 4294972114, "clang::MSVtorDispMode::ForVBaseOverride"),
      add_named_node(&mut g, 4294972115, "clang::MSVtorDispMode::ForVFTable"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_MSVTORDISPMODE, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_MULTIVERSIONKIND, "clang::MultiVersionKind");
  g.add_edge((ENUM_MULTIVERSIONKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972117, "clang::MultiVersionKind::None"),
      add_named_node(&mut g, 4294972118, "clang::MultiVersionKind::Target"),
      add_named_node(&mut g, 4294972119, "clang::MultiVersionKind::CPUSpecific"),
      add_named_node(&mut g, 4294972120, "clang::MultiVersionKind::CPUDispatch"),
      add_named_node(&mut g, 4294972121, "clang::MultiVersionKind::TargetClones"),
      add_named_node(&mut g, 4294972122, "clang::MultiVersionKind::TargetVersion"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_MULTIVERSIONKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_NONODRUSEREASON, "clang::NonOdrUseReason");
  g.add_edge((ENUM_NONODRUSEREASON, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972124, "clang::NOUR_None"),
      add_named_node(&mut g, 4294972125, "clang::NOUR_Unevaluated"),
      add_named_node(&mut g, 4294972126, "clang::NOUR_Constant"),
      add_named_node(&mut g, 4294972127, "clang::NOUR_Discarded"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_NONODRUSEREASON, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_OBJCSTRINGFORMATFAMILY, "clang::ObjCStringFormatFamily");
  g.add_edge((ENUM_OBJCSTRINGFORMATFAMILY, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972129, "clang::SFF_None"),
      add_named_node(&mut g, 4294972130, "clang::SFF_NSString"),
      add_named_node(&mut g, 4294972131, "clang::SFF_CFString"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_OBJCSTRINGFORMATFAMILY, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_OVERLOADEDOPERATORKIND, "clang::OverloadedOperatorKind");
  g.add_edge((ENUM_OVERLOADEDOPERATORKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972133, "clang::OO_None"),
      add_named_node(&mut g, 4294972134, "clang::OO_New"),
      add_named_node(&mut g, 4294972135, "clang::OO_Delete"),
      add_named_node(&mut g, 4294972136, "clang::OO_Array_New"),
      add_named_node(&mut g, 4294972137, "clang::OO_Array_Delete"),
      add_named_node(&mut g, 4294972138, "clang::OO_Plus"),
      add_named_node(&mut g, 4294972139, "clang::OO_Minus"),
      add_named_node(&mut g, 4294972140, "clang::OO_Star"),
      add_named_node(&mut g, 4294972141, "clang::OO_Slash"),
      add_named_node(&mut g, 4294972142, "clang::OO_Percent"),
      add_named_node(&mut g, 4294972143, "clang::OO_Caret"),
      add_named_node(&mut g, 4294972144, "clang::OO_Amp"),
      add_named_node(&mut g, 4294972145, "clang::OO_Pipe"),
      add_named_node(&mut g, 4294972146, "clang::OO_Tilde"),
      add_named_node(&mut g, 4294972147, "clang::OO_Exclaim"),
      add_named_node(&mut g, 4294972148, "clang::OO_Equal"),
      add_named_node(&mut g, 4294972149, "clang::OO_Less"),
      add_named_node(&mut g, 4294972150, "clang::OO_Greater"),
      add_named_node(&mut g, 4294972151, "clang::OO_PlusEqual"),
      add_named_node(&mut g, 4294972152, "clang::OO_MinusEqual"),
      add_named_node(&mut g, 4294972153, "clang::OO_StarEqual"),
      add_named_node(&mut g, 4294972154, "clang::OO_SlashEqual"),
      add_named_node(&mut g, 4294972155, "clang::OO_PercentEqual"),
      add_named_node(&mut g, 4294972156, "clang::OO_CaretEqual"),
      add_named_node(&mut g, 4294972157, "clang::OO_AmpEqual"),
      add_named_node(&mut g, 4294972158, "clang::OO_PipeEqual"),
      add_named_node(&mut g, 4294972159, "clang::OO_LessLess"),
      add_named_node(&mut g, 4294972160, "clang::OO_GreaterGreater"),
      add_named_node(&mut g, 4294972161, "clang::OO_LessLessEqual"),
      add_named_node(&mut g, 4294972162, "clang::OO_GreaterGreaterEqual"),
      add_named_node(&mut g, 4294972163, "clang::OO_EqualEqual"),
      add_named_node(&mut g, 4294972164, "clang::OO_ExclaimEqual"),
      add_named_node(&mut g, 4294972165, "clang::OO_LessEqual"),
      add_named_node(&mut g, 4294972166, "clang::OO_GreaterEqual"),
      add_named_node(&mut g, 4294972167, "clang::OO_Spaceship"),
      add_named_node(&mut g, 4294972168, "clang::OO_AmpAmp"),
      add_named_node(&mut g, 4294972169, "clang::OO_PipePipe"),
      add_named_node(&mut g, 4294972170, "clang::OO_PlusPlus"),
      add_named_node(&mut g, 4294972171, "clang::OO_MinusMinus"),
      add_named_node(&mut g, 4294972172, "clang::OO_Comma"),
      add_named_node(&mut g, 4294972173, "clang::OO_ArrowStar"),
      add_named_node(&mut g, 4294972174, "clang::OO_Arrow"),
      add_named_node(&mut g, 4294972175, "clang::OO_Call"),
      add_named_node(&mut g, 4294972176, "clang::OO_Subscript"),
      add_named_node(&mut g, 4294972177, "clang::OO_Conditional"),
      add_named_node(&mut g, 4294972178, "clang::OO_Coawait"),
      add_named_node(&mut g, 4294972179, "clang::NUM_OVERLOADED_OPERATORS"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_OVERLOADEDOPERATORKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_PRAGMAMSCOMMENTKIND, "clang::PragmaMSCommentKind");
  g.add_edge((ENUM_PRAGMAMSCOMMENTKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972181, "clang::PCK_Unknown"),
      add_named_node(&mut g, 4294972182, "clang::PCK_Linker"),
      add_named_node(&mut g, 4294972183, "clang::PCK_Lib"),
      add_named_node(&mut g, 4294972184, "clang::PCK_Compiler"),
      add_named_node(&mut g, 4294972185, "clang::PCK_ExeStr"),
      add_named_node(&mut g, 4294972186, "clang::PCK_User"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_PRAGMAMSCOMMENTKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_PREDEFINEDIDENTKIND, "clang::PredefinedIdentKind");
  g.add_edge((ENUM_PREDEFINEDIDENTKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972188, "clang::PredefinedIdentKind::Func"),
      add_named_node(&mut g, 4294972189, "clang::PredefinedIdentKind::Function"),
      add_named_node(&mut g, 4294972190, "clang::PredefinedIdentKind::LFunction"),
      add_named_node(&mut g, 4294972191, "clang::PredefinedIdentKind::FuncDName"),
      add_named_node(&mut g, 4294972192, "clang::PredefinedIdentKind::FuncSig"),
      add_named_node(&mut g, 4294972193, "clang::PredefinedIdentKind::LFuncSig"),
      add_named_node(&mut g, 4294972194, "clang::PredefinedIdentKind::PrettyFunction"),
      add_named_node(&mut g, 4294972195, "clang::PredefinedIdentKind::PrettyFunctionNoVirtual"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_PREDEFINEDIDENTKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_OBJCLIFETIME, "clang::Qualifiers::ObjCLifetime");
  g.add_edge((ENUM_OBJCLIFETIME, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972197, "clang::Qualifiers::OCL_None"),
      add_named_node(&mut g, 4294972198, "clang::Qualifiers::OCL_ExplicitNone"),
      add_named_node(&mut g, 4294972199, "clang::Qualifiers::OCL_Strong"),
      add_named_node(&mut g, 4294972200, "clang::Qualifiers::OCL_Weak"),
      add_named_node(&mut g, 4294972201, "clang::Qualifiers::OCL_Autoreleasing"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_OBJCLIFETIME, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_RECORDARGPASSINGKIND, "clang::RecordArgPassingKind");
  g.add_edge((ENUM_RECORDARGPASSINGKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972203, "clang::RecordArgPassingKind::CanPassInRegs"),
      add_named_node(&mut g, 4294972204, "clang::RecordArgPassingKind::CannotPassInRegs"),
      add_named_node(&mut g, 4294972205, "clang::RecordArgPassingKind::CanNeverPassInRegs"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_RECORDARGPASSINGKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_REFQUALIFIERKIND, "clang::RefQualifierKind");
  g.add_edge((ENUM_REFQUALIFIERKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972207, "clang::RQ_None"),
      add_named_node(&mut g, 4294972208, "clang::RQ_LValue"),
      add_named_node(&mut g, 4294972209, "clang::RQ_RValue"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_REFQUALIFIERKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_SOURCELOCIDENTKIND, "clang::SourceLocIdentKind");
  g.add_edge((ENUM_SOURCELOCIDENTKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972211, "clang::SourceLocIdentKind::Function"),
      add_named_node(&mut g, 4294972212, "clang::SourceLocIdentKind::FuncSig"),
      add_named_node(&mut g, 4294972213, "clang::SourceLocIdentKind::File"),
      add_named_node(&mut g, 4294972214, "clang::SourceLocIdentKind::FileName"),
      add_named_node(&mut g, 4294972215, "clang::SourceLocIdentKind::Line"),
      add_named_node(&mut g, 4294972216, "clang::SourceLocIdentKind::Column"),
      add_named_node(&mut g, 4294972217, "clang::SourceLocIdentKind::SourceLocStruct"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_SOURCELOCIDENTKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_STORAGECLASS, "clang::StorageClass");
  g.add_edge((ENUM_STORAGECLASS, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972219, "clang::SC_None"),
      add_named_node(&mut g, 4294972220, "clang::SC_Extern"),
      add_named_node(&mut g, 4294972221, "clang::SC_Static"),
      add_named_node(&mut g, 4294972222, "clang::SC_PrivateExtern"),
      add_named_node(&mut g, 4294972223, "clang::SC_Auto"),
      add_named_node(&mut g, 4294972224, "clang::SC_Register"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_STORAGECLASS, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_STORAGEDURATION, "clang::StorageDuration");
  g.add_edge((ENUM_STORAGEDURATION, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972226, "clang::SD_FullExpression"),
      add_named_node(&mut g, 4294972227, "clang::SD_Automatic"),
      add_named_node(&mut g, 4294972228, "clang::SD_Thread"),
      add_named_node(&mut g, 4294972229, "clang::SD_Static"),
      add_named_node(&mut g, 4294972230, "clang::SD_Dynamic"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_STORAGEDURATION, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_STRINGLITERALKIND, "clang::StringLiteralKind");
  g.add_edge((ENUM_STRINGLITERALKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972232, "clang::StringLiteralKind::Ordinary"),
      add_named_node(&mut g, 4294972233, "clang::StringLiteralKind::Wide"),
      add_named_node(&mut g, 4294972234, "clang::StringLiteralKind::UTF8"),
      add_named_node(&mut g, 4294972235, "clang::StringLiteralKind::UTF16"),
      add_named_node(&mut g, 4294972236, "clang::StringLiteralKind::UTF32"),
      add_named_node(&mut g, 4294972237, "clang::StringLiteralKind::Unevaluated"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_STRINGLITERALKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_TAGTYPEKIND, "clang::TagTypeKind");
  g.add_edge((ENUM_TAGTYPEKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972239, "clang::TagTypeKind::Struct"),
      add_named_node(&mut g, 4294972240, "clang::TagTypeKind::Interface"),
      add_named_node(&mut g, 4294972241, "clang::TagTypeKind::Union"),
      add_named_node(&mut g, 4294972242, "clang::TagTypeKind::Class"),
      add_named_node(&mut g, 4294972243, "clang::TagTypeKind::Enum"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_TAGTYPEKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_TEMPLATESPECIALIZATIONKIND, "clang::TemplateSpecializationKind");
  g.add_edge((ENUM_TEMPLATESPECIALIZATIONKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972245, "clang::TSK_Undeclared"),
      add_named_node(&mut g, 4294972246, "clang::TSK_ImplicitInstantiation"),
      add_named_node(&mut g, 4294972247, "clang::TSK_ExplicitSpecialization"),
      add_named_node(&mut g, 4294972248, "clang::TSK_ExplicitInstantiationDeclaration"),
      add_named_node(&mut g, 4294972249, "clang::TSK_ExplicitInstantiationDefinition"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_TEMPLATESPECIALIZATIONKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_THREADSTORAGECLASSSPECIFIER, "clang::ThreadStorageClassSpecifier");
  g.add_edge((ENUM_THREADSTORAGECLASSSPECIFIER, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972251, "clang::TSCS_unspecified"),
      add_named_node(&mut g, 4294972252, "clang::TSCS___thread"),
      add_named_node(&mut g, 4294972253, "clang::TSCS_thread_local"),
      add_named_node(&mut g, 4294972254, "clang::TSCS__Thread_local"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_THREADSTORAGECLASSSPECIFIER, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_TYPEDEPENDENCE, "clang::TypeDependenceScope::TypeDependence");
  g.add_edge((ENUM_TYPEDEPENDENCE, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972256, "clang::TypeDependenceScope::UnexpandedPack"),
      add_named_node(&mut g, 4294972257, "clang::TypeDependenceScope::Instantiation"),
      add_named_node(&mut g, 4294972258, "clang::TypeDependenceScope::Dependent"),
      add_named_node(&mut g, 4294972259, "clang::TypeDependenceScope::VariablyModified"),
      add_named_node(&mut g, 4294972260, "clang::TypeDependenceScope::Error"),
      add_named_node(&mut g, 4294972261, "clang::TypeDependenceScope::None"),
      add_named_node(&mut g, 4294972262, "clang::TypeDependenceScope::All"),
      add_named_node(&mut g, 4294972263, "clang::TypeDependenceScope::DependentInstantiation"),
      add_named_node(&mut g, 4294972264, "clang::TypeDependenceScope::LLVM_BITMASK_LARGEST_ENUMERATOR"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_TYPEDEPENDENCE, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_TYPEOFKIND, "clang::TypeOfKind");
  g.add_edge((ENUM_TYPEOFKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972266, "clang::TypeOfKind::Qualified"),
      add_named_node(&mut g, 4294972267, "clang::TypeOfKind::Unqualified"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_TYPEOFKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_TYPETRAIT, "clang::TypeTrait");
  g.add_edge((ENUM_TYPETRAIT, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972269, "clang::UTT_IsInterfaceClass"),
      add_named_node(&mut g, 4294972270, "clang::UTT_IsSealed"),
      add_named_node(&mut g, 4294972271, "clang::UTT_IsDestructible"),
      add_named_node(&mut g, 4294972272, "clang::UTT_IsTriviallyDestructible"),
      add_named_node(&mut g, 4294972273, "clang::UTT_IsNothrowDestructible"),
      add_named_node(&mut g, 4294972274, "clang::UTT_HasNothrowMoveAssign"),
      add_named_node(&mut g, 4294972275, "clang::UTT_HasTrivialMoveAssign"),
      add_named_node(&mut g, 4294972276, "clang::UTT_HasTrivialMoveConstructor"),
      add_named_node(&mut g, 4294972277, "clang::UTT_HasNothrowAssign"),
      add_named_node(&mut g, 4294972278, "clang::UTT_HasNothrowCopy"),
      add_named_node(&mut g, 4294972279, "clang::UTT_HasNothrowConstructor"),
      add_named_node(&mut g, 4294972280, "clang::UTT_HasTrivialAssign"),
      add_named_node(&mut g, 4294972281, "clang::UTT_HasTrivialCopy"),
      add_named_node(&mut g, 4294972282, "clang::UTT_HasTrivialDefaultConstructor"),
      add_named_node(&mut g, 4294972283, "clang::UTT_HasTrivialDestructor"),
      add_named_node(&mut g, 4294972284, "clang::UTT_HasVirtualDestructor"),
      add_named_node(&mut g, 4294972285, "clang::UTT_IsAbstract"),
      add_named_node(&mut g, 4294972286, "clang::UTT_IsAggregate"),
      add_named_node(&mut g, 4294972287, "clang::UTT_IsClass"),
      add_named_node(&mut g, 4294972288, "clang::UTT_IsEmpty"),
      add_named_node(&mut g, 4294972289, "clang::UTT_IsEnum"),
      add_named_node(&mut g, 4294972290, "clang::UTT_IsFinal"),
      add_named_node(&mut g, 4294972291, "clang::UTT_IsLiteral"),
      add_named_node(&mut g, 4294972292, "clang::UTT_IsPOD"),
      add_named_node(&mut g, 4294972293, "clang::UTT_IsPolymorphic"),
      add_named_node(&mut g, 4294972294, "clang::UTT_IsStandardLayout"),
      add_named_node(&mut g, 4294972295, "clang::UTT_IsTrivial"),
      add_named_node(&mut g, 4294972296, "clang::UTT_IsTriviallyCopyable"),
      add_named_node(&mut g, 4294972297, "clang::UTT_IsUnion"),
      add_named_node(&mut g, 4294972298, "clang::UTT_HasUniqueObjectRepresentations"),
      add_named_node(&mut g, 4294972299, "clang::UTT_IsTriviallyRelocatable"),
      add_named_node(&mut g, 4294972300, "clang::UTT_IsTriviallyEqualityComparable"),
      add_named_node(&mut g, 4294972301, "clang::UTT_IsBoundedArray"),
      add_named_node(&mut g, 4294972302, "clang::UTT_IsUnboundedArray"),
      add_named_node(&mut g, 4294972303, "clang::UTT_IsNullPointer"),
      add_named_node(&mut g, 4294972304, "clang::UTT_IsScopedEnum"),
      add_named_node(&mut g, 4294972305, "clang::UTT_IsReferenceable"),
      add_named_node(&mut g, 4294972306, "clang::UTT_CanPassInRegs"),
      add_named_node(&mut g, 4294972307, "clang::UTT_IsArithmetic"),
      add_named_node(&mut g, 4294972308, "clang::UTT_IsFloatingPoint"),
      add_named_node(&mut g, 4294972309, "clang::UTT_IsIntegral"),
      add_named_node(&mut g, 4294972310, "clang::UTT_IsCompleteType"),
      add_named_node(&mut g, 4294972311, "clang::UTT_IsVoid"),
      add_named_node(&mut g, 4294972312, "clang::UTT_IsArray"),
      add_named_node(&mut g, 4294972313, "clang::UTT_IsFunction"),
      add_named_node(&mut g, 4294972314, "clang::UTT_IsReference"),
      add_named_node(&mut g, 4294972315, "clang::UTT_IsLvalueReference"),
      add_named_node(&mut g, 4294972316, "clang::UTT_IsRvalueReference"),
      add_named_node(&mut g, 4294972317, "clang::UTT_IsFundamental"),
      add_named_node(&mut g, 4294972318, "clang::UTT_IsObject"),
      add_named_node(&mut g, 4294972319, "clang::UTT_IsScalar"),
      add_named_node(&mut g, 4294972320, "clang::UTT_IsCompound"),
      add_named_node(&mut g, 4294972321, "clang::UTT_IsPointer"),
      add_named_node(&mut g, 4294972322, "clang::UTT_IsMemberObjectPointer"),
      add_named_node(&mut g, 4294972323, "clang::UTT_IsMemberFunctionPointer"),
      add_named_node(&mut g, 4294972324, "clang::UTT_IsMemberPointer"),
      add_named_node(&mut g, 4294972325, "clang::UTT_IsConst"),
      add_named_node(&mut g, 4294972326, "clang::UTT_IsVolatile"),
      add_named_node(&mut g, 4294972327, "clang::UTT_IsSigned"),
      add_named_node(&mut g, 4294972328, "clang::UTT_IsUnsigned"),
      add_named_node(&mut g, 4294972329, "clang::UTT_Last"),
      add_named_node(&mut g, 4294972330, "clang::BTT_TypeCompatible"),
      add_named_node(&mut g, 4294972331, "clang::BTT_IsNothrowAssignable"),
      add_named_node(&mut g, 4294972332, "clang::BTT_IsAssignable"),
      add_named_node(&mut g, 4294972333, "clang::BTT_IsBaseOf"),
      add_named_node(&mut g, 4294972334, "clang::BTT_IsConvertibleTo"),
      add_named_node(&mut g, 4294972335, "clang::BTT_IsTriviallyAssignable"),
      add_named_node(&mut g, 4294972336, "clang::BTT_ReferenceBindsToTemporary"),
      add_named_node(&mut g, 4294972337, "clang::BTT_ReferenceConstructsFromTemporary"),
      add_named_node(&mut g, 4294972338, "clang::BTT_IsSame"),
      add_named_node(&mut g, 4294972339, "clang::BTT_IsConvertible"),
      add_named_node(&mut g, 4294972340, "clang::BTT_Last"),
      add_named_node(&mut g, 4294972341, "clang::TT_IsConstructible"),
      add_named_node(&mut g, 4294972342, "clang::TT_IsNothrowConstructible"),
      add_named_node(&mut g, 4294972343, "clang::TT_IsTriviallyConstructible"),
      add_named_node(&mut g, 4294972344, "clang::TT_Last"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_TYPETRAIT, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_UNARYEXPRORTYPETRAIT, "clang::UnaryExprOrTypeTrait");
  g.add_edge((ENUM_UNARYEXPRORTYPETRAIT, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972346, "clang::UETT_SizeOf"),
      add_named_node(&mut g, 4294972347, "clang::UETT_DataSizeOf"),
      add_named_node(&mut g, 4294972348, "clang::UETT_AlignOf"),
      add_named_node(&mut g, 4294972349, "clang::UETT_PreferredAlignOf"),
      add_named_node(&mut g, 4294972350, "clang::UETT_VecStep"),
      add_named_node(&mut g, 4294972351, "clang::UETT_OpenMPRequiredSimdAlign"),
      add_named_node(&mut g, 4294972352, "clang::UETT_VectorElements"),
      add_named_node(&mut g, 4294972353, "clang::UETT_Last"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_UNARYEXPRORTYPETRAIT, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_UNARYOPERATORKIND, "clang::UnaryOperatorKind");
  g.add_edge((ENUM_UNARYOPERATORKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972355, "clang::UO_PostInc"),
      add_named_node(&mut g, 4294972356, "clang::UO_PostDec"),
      add_named_node(&mut g, 4294972357, "clang::UO_PreInc"),
      add_named_node(&mut g, 4294972358, "clang::UO_PreDec"),
      add_named_node(&mut g, 4294972359, "clang::UO_AddrOf"),
      add_named_node(&mut g, 4294972360, "clang::UO_Deref"),
      add_named_node(&mut g, 4294972361, "clang::UO_Plus"),
      add_named_node(&mut g, 4294972362, "clang::UO_Minus"),
      add_named_node(&mut g, 4294972363, "clang::UO_Not"),
      add_named_node(&mut g, 4294972364, "clang::UO_LNot"),
      add_named_node(&mut g, 4294972365, "clang::UO_Real"),
      add_named_node(&mut g, 4294972366, "clang::UO_Imag"),
      add_named_node(&mut g, 4294972367, "clang::UO_Extension"),
      add_named_node(&mut g, 4294972368, "clang::UO_Coawait"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_UNARYOPERATORKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_UTTKIND, "clang::UnaryTransformType::UTTKind");
  g.add_edge((ENUM_UTTKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972370, "clang::UnaryTransformType::AddLvalueReference"),
      add_named_node(&mut g, 4294972371, "clang::UnaryTransformType::AddPointer"),
      add_named_node(&mut g, 4294972372, "clang::UnaryTransformType::AddRvalueReference"),
      add_named_node(&mut g, 4294972373, "clang::UnaryTransformType::Decay"),
      add_named_node(&mut g, 4294972374, "clang::UnaryTransformType::MakeSigned"),
      add_named_node(&mut g, 4294972375, "clang::UnaryTransformType::MakeUnsigned"),
      add_named_node(&mut g, 4294972376, "clang::UnaryTransformType::RemoveAllExtents"),
      add_named_node(&mut g, 4294972377, "clang::UnaryTransformType::RemoveConst"),
      add_named_node(&mut g, 4294972378, "clang::UnaryTransformType::RemoveCV"),
      add_named_node(&mut g, 4294972379, "clang::UnaryTransformType::RemoveCVRef"),
      add_named_node(&mut g, 4294972380, "clang::UnaryTransformType::RemoveExtent"),
      add_named_node(&mut g, 4294972381, "clang::UnaryTransformType::RemovePointer"),
      add_named_node(&mut g, 4294972382, "clang::UnaryTransformType::RemoveReference"),
      add_named_node(&mut g, 4294972383, "clang::UnaryTransformType::RemoveRestrict"),
      add_named_node(&mut g, 4294972384, "clang::UnaryTransformType::RemoveVolatile"),
      add_named_node(&mut g, 4294972385, "clang::UnaryTransformType::EnumUnderlyingType"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_UTTKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_LITERALOPERATORKIND, "clang::UserDefinedLiteral::LiteralOperatorKind");
  g.add_edge((ENUM_LITERALOPERATORKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972387, "clang::UserDefinedLiteral::LOK_Raw"),
      add_named_node(&mut g, 4294972388, "clang::UserDefinedLiteral::LOK_Template"),
      add_named_node(&mut g, 4294972389, "clang::UserDefinedLiteral::LOK_Integer"),
      add_named_node(&mut g, 4294972390, "clang::UserDefinedLiteral::LOK_Floating"),
      add_named_node(&mut g, 4294972391, "clang::UserDefinedLiteral::LOK_String"),
      add_named_node(&mut g, 4294972392, "clang::UserDefinedLiteral::LOK_Character"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_LITERALOPERATORKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_DEFINITIONKIND, "clang::VarDecl::DefinitionKind");
  g.add_edge((ENUM_DEFINITIONKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972394, "clang::VarDecl::DeclarationOnly"),
      add_named_node(&mut g, 4294972395, "clang::VarDecl::TentativeDefinition"),
      add_named_node(&mut g, 4294972396, "clang::VarDecl::Definition"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_DEFINITIONKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_INITIALIZATIONSTYLE, "clang::VarDecl::InitializationStyle");
  g.add_edge((ENUM_INITIALIZATIONSTYLE, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972398, "clang::VarDecl::CInit"),
      add_named_node(&mut g, 4294972399, "clang::VarDecl::CallInit"),
      add_named_node(&mut g, 4294972400, "clang::VarDecl::ListInit"),
      add_named_node(&mut g, 4294972401, "clang::VarDecl::ParenListInit"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_INITIALIZATIONSTYLE, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_TLSKIND, "clang::VarDecl::TLSKind");
  g.add_edge((ENUM_TLSKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972403, "clang::VarDecl::TLS_None"),
      add_named_node(&mut g, 4294972404, "clang::VarDecl::TLS_Static"),
      add_named_node(&mut g, 4294972405, "clang::VarDecl::TLS_Dynamic"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_TLSKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_VECTORKIND, "clang::VectorKind");
  g.add_edge((ENUM_VECTORKIND, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972407, "clang::VectorKind::Generic"),
      add_named_node(&mut g, 4294972408, "clang::VectorKind::AltiVecVector"),
      add_named_node(&mut g, 4294972409, "clang::VectorKind::AltiVecPixel"),
      add_named_node(&mut g, 4294972410, "clang::VectorKind::AltiVecBool"),
      add_named_node(&mut g, 4294972411, "clang::VectorKind::Neon"),
      add_named_node(&mut g, 4294972412, "clang::VectorKind::NeonPoly"),
      add_named_node(&mut g, 4294972413, "clang::VectorKind::SveFixedLengthData"),
      add_named_node(&mut g, 4294972414, "clang::VectorKind::SveFixedLengthPredicate"),
      add_named_node(&mut g, 4294972415, "clang::VectorKind::RVVFixedLengthData"),
      add_named_node(&mut g, 4294972416, "clang::VectorKind::RVVFixedLengthMask"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_VECTORKIND, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_VISIBILITY, "clang::Visibility");
  g.add_edge((ENUM_VISIBILITY, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972418, "clang::HiddenVisibility"),
      add_named_node(&mut g, 4294972419, "clang::ProtectedVisibility"),
      add_named_node(&mut g, 4294972420, "clang::DefaultVisibility"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_VISIBILITY, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_KIND_1, "clang::attr::Kind");
  g.add_edge((ENUM_KIND_1, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972422, "clang::attr::AddressSpace"),
      add_named_node(&mut g, 4294972423, "clang::attr::AnnotateType"),
      add_named_node(&mut g, 4294972424, "clang::attr::ArmIn"),
      add_named_node(&mut g, 4294972425, "clang::attr::ArmInOut"),
      add_named_node(&mut g, 4294972426, "clang::attr::ArmMveStrictPolymorphism"),
      add_named_node(&mut g, 4294972427, "clang::attr::ArmOut"),
      add_named_node(&mut g, 4294972428, "clang::attr::ArmPreserves"),
      add_named_node(&mut g, 4294972429, "clang::attr::ArmStreaming"),
      add_named_node(&mut g, 4294972430, "clang::attr::ArmStreamingCompatible"),
      add_named_node(&mut g, 4294972431, "clang::attr::BTFTypeTag"),
      add_named_node(&mut g, 4294972432, "clang::attr::CmseNSCall"),
      add_named_node(&mut g, 4294972433, "clang::attr::HLSLGroupSharedAddressSpace"),
      add_named_node(&mut g, 4294972434, "clang::attr::HLSLParamModifier"),
      add_named_node(&mut g, 4294972435, "clang::attr::NoDeref"),
      add_named_node(&mut g, 4294972436, "clang::attr::ObjCGC"),
      add_named_node(&mut g, 4294972437, "clang::attr::ObjCInertUnsafeUnretained"),
      add_named_node(&mut g, 4294972438, "clang::attr::ObjCKindOf"),
      add_named_node(&mut g, 4294972439, "clang::attr::OpenCLConstantAddressSpace"),
      add_named_node(&mut g, 4294972440, "clang::attr::OpenCLGenericAddressSpace"),
      add_named_node(&mut g, 4294972441, "clang::attr::OpenCLGlobalAddressSpace"),
      add_named_node(&mut g, 4294972442, "clang::attr::OpenCLGlobalDeviceAddressSpace"),
      add_named_node(&mut g, 4294972443, "clang::attr::OpenCLGlobalHostAddressSpace"),
      add_named_node(&mut g, 4294972444, "clang::attr::OpenCLLocalAddressSpace"),
      add_named_node(&mut g, 4294972445, "clang::attr::OpenCLPrivateAddressSpace"),
      add_named_node(&mut g, 4294972446, "clang::attr::Ptr32"),
      add_named_node(&mut g, 4294972447, "clang::attr::Ptr64"),
      add_named_node(&mut g, 4294972448, "clang::attr::SPtr"),
      add_named_node(&mut g, 4294972449, "clang::attr::TypeNonNull"),
      add_named_node(&mut g, 4294972450, "clang::attr::TypeNullUnspecified"),
      add_named_node(&mut g, 4294972451, "clang::attr::TypeNullable"),
      add_named_node(&mut g, 4294972452, "clang::attr::TypeNullableResult"),
      add_named_node(&mut g, 4294972453, "clang::attr::UPtr"),
      add_named_node(&mut g, 4294972454, "clang::attr::WebAssemblyFuncref"),
      add_named_node(&mut g, 4294972455, "clang::attr::CodeAlign"),
      add_named_node(&mut g, 4294972456, "clang::attr::FallThrough"),
      add_named_node(&mut g, 4294972457, "clang::attr::Likely"),
      add_named_node(&mut g, 4294972458, "clang::attr::MustTail"),
      add_named_node(&mut g, 4294972459, "clang::attr::OpenCLUnrollHint"),
      add_named_node(&mut g, 4294972460, "clang::attr::Unlikely"),
      add_named_node(&mut g, 4294972461, "clang::attr::AlwaysInline"),
      add_named_node(&mut g, 4294972462, "clang::attr::NoInline"),
      add_named_node(&mut g, 4294972463, "clang::attr::NoMerge"),
      add_named_node(&mut g, 4294972464, "clang::attr::Suppress"),
      add_named_node(&mut g, 4294972465, "clang::attr::AArch64SVEPcs"),
      add_named_node(&mut g, 4294972466, "clang::attr::AArch64VectorPcs"),
      add_named_node(&mut g, 4294972467, "clang::attr::AMDGPUKernelCall"),
      add_named_node(&mut g, 4294972468, "clang::attr::AcquireHandle"),
      add_named_node(&mut g, 4294972469, "clang::attr::AnyX86NoCfCheck"),
      add_named_node(&mut g, 4294972470, "clang::attr::CDecl"),
      add_named_node(&mut g, 4294972471, "clang::attr::FastCall"),
      add_named_node(&mut g, 4294972472, "clang::attr::IntelOclBicc"),
      add_named_node(&mut g, 4294972473, "clang::attr::LifetimeBound"),
      add_named_node(&mut g, 4294972474, "clang::attr::M68kRTD"),
      add_named_node(&mut g, 4294972475, "clang::attr::MSABI"),
      add_named_node(&mut g, 4294972476, "clang::attr::NSReturnsRetained"),
      add_named_node(&mut g, 4294972477, "clang::attr::ObjCOwnership"),
      add_named_node(&mut g, 4294972478, "clang::attr::Pascal"),
      add_named_node(&mut g, 4294972479, "clang::attr::Pcs"),
      add_named_node(&mut g, 4294972480, "clang::attr::PreserveAll"),
      add_named_node(&mut g, 4294972481, "clang::attr::PreserveMost"),
      add_named_node(&mut g, 4294972482, "clang::attr::RegCall"),
      add_named_node(&mut g, 4294972483, "clang::attr::StdCall"),
      add_named_node(&mut g, 4294972484, "clang::attr::SwiftAsyncCall"),
      add_named_node(&mut g, 4294972485, "clang::attr::SwiftCall"),
      add_named_node(&mut g, 4294972486, "clang::attr::SysVABI"),
      add_named_node(&mut g, 4294972487, "clang::attr::ThisCall"),
      add_named_node(&mut g, 4294972488, "clang::attr::VectorCall"),
      add_named_node(&mut g, 4294972489, "clang::attr::SwiftAsyncContext"),
      add_named_node(&mut g, 4294972490, "clang::attr::SwiftContext"),
      add_named_node(&mut g, 4294972491, "clang::attr::SwiftErrorResult"),
      add_named_node(&mut g, 4294972492, "clang::attr::SwiftIndirectResult"),
      add_named_node(&mut g, 4294972493, "clang::attr::Annotate"),
      add_named_node(&mut g, 4294972494, "clang::attr::CFConsumed"),
      add_named_node(&mut g, 4294972495, "clang::attr::CarriesDependency"),
      add_named_node(&mut g, 4294972496, "clang::attr::NSConsumed"),
      add_named_node(&mut g, 4294972497, "clang::attr::NonNull"),
      add_named_node(&mut g, 4294972498, "clang::attr::OSConsumed"),
      add_named_node(&mut g, 4294972499, "clang::attr::PassObjectSize"),
      add_named_node(&mut g, 4294972500, "clang::attr::ReleaseHandle"),
      add_named_node(&mut g, 4294972501, "clang::attr::UseHandle"),
      add_named_node(&mut g, 4294972502, "clang::attr::HLSLSV_DispatchThreadID"),
      add_named_node(&mut g, 4294972503, "clang::attr::HLSLSV_GroupIndex"),
      add_named_node(&mut g, 4294972504, "clang::attr::AMDGPUFlatWorkGroupSize"),
      add_named_node(&mut g, 4294972505, "clang::attr::AMDGPUNumSGPR"),
      add_named_node(&mut g, 4294972506, "clang::attr::AMDGPUNumVGPR"),
      add_named_node(&mut g, 4294972507, "clang::attr::AMDGPUWavesPerEU"),
      add_named_node(&mut g, 4294972508, "clang::attr::ARMInterrupt"),
      add_named_node(&mut g, 4294972509, "clang::attr::AVRInterrupt"),
      add_named_node(&mut g, 4294972510, "clang::attr::AVRSignal"),
      add_named_node(&mut g, 4294972511, "clang::attr::AcquireCapability"),
      add_named_node(&mut g, 4294972512, "clang::attr::AcquiredAfter"),
      add_named_node(&mut g, 4294972513, "clang::attr::AcquiredBefore"),
      add_named_node(&mut g, 4294972514, "clang::attr::AlignMac68k"),
      add_named_node(&mut g, 4294972515, "clang::attr::AlignNatural"),
      add_named_node(&mut g, 4294972516, "clang::attr::Aligned"),
      add_named_node(&mut g, 4294972517, "clang::attr::AllocAlign"),
      add_named_node(&mut g, 4294972518, "clang::attr::AllocSize"),
      add_named_node(&mut g, 4294972519, "clang::attr::AlwaysDestroy"),
      add_named_node(&mut g, 4294972520, "clang::attr::AnalyzerNoReturn"),
      add_named_node(&mut g, 4294972521, "clang::attr::AnyX86Interrupt"),
      add_named_node(&mut g, 4294972522, "clang::attr::AnyX86NoCallerSavedRegisters"),
      add_named_node(&mut g, 4294972523, "clang::attr::ArcWeakrefUnavailable"),
      add_named_node(&mut g, 4294972524, "clang::attr::ArgumentWithTypeTag"),
      add_named_node(&mut g, 4294972525, "clang::attr::ArmBuiltinAlias"),
      add_named_node(&mut g, 4294972526, "clang::attr::ArmLocallyStreaming"),
      add_named_node(&mut g, 4294972527, "clang::attr::ArmNew"),
      add_named_node(&mut g, 4294972528, "clang::attr::Artificial"),
      add_named_node(&mut g, 4294972529, "clang::attr::AsmLabel"),
      add_named_node(&mut g, 4294972530, "clang::attr::AssertCapability"),
      add_named_node(&mut g, 4294972531, "clang::attr::AssertExclusiveLock"),
      add_named_node(&mut g, 4294972532, "clang::attr::AssertSharedLock"),
      add_named_node(&mut g, 4294972533, "clang::attr::AssumeAligned"),
      add_named_node(&mut g, 4294972534, "clang::attr::Assumption"),
      add_named_node(&mut g, 4294972535, "clang::attr::Availability"),
      add_named_node(&mut g, 4294972536, "clang::attr::AvailableOnlyInDefaultEvalMethod"),
      add_named_node(&mut g, 4294972537, "clang::attr::BPFPreserveAccessIndex"),
      add_named_node(&mut g, 4294972538, "clang::attr::BPFPreserveStaticOffset"),
      add_named_node(&mut g, 4294972539, "clang::attr::BTFDeclTag"),
      add_named_node(&mut g, 4294972540, "clang::attr::Blocks"),
      add_named_node(&mut g, 4294972541, "clang::attr::Builtin"),
      add_named_node(&mut g, 4294972542, "clang::attr::C11NoReturn"),
      add_named_node(&mut g, 4294972543, "clang::attr::CFAuditedTransfer"),
      add_named_node(&mut g, 4294972544, "clang::attr::CFGuard"),
      add_named_node(&mut g, 4294972545, "clang::attr::CFICanonicalJumpTable"),
      add_named_node(&mut g, 4294972546, "clang::attr::CFReturnsNotRetained"),
      add_named_node(&mut g, 4294972547, "clang::attr::CFReturnsRetained"),
      add_named_node(&mut g, 4294972548, "clang::attr::CFUnknownTransfer"),
      add_named_node(&mut g, 4294972549, "clang::attr::CPUDispatch"),
      add_named_node(&mut g, 4294972550, "clang::attr::CPUSpecific"),
      add_named_node(&mut g, 4294972551, "clang::attr::CUDAConstant"),
      add_named_node(&mut g, 4294972552, "clang::attr::CUDADevice"),
      add_named_node(&mut g, 4294972553, "clang::attr::CUDADeviceBuiltinSurfaceType"),
      add_named_node(&mut g, 4294972554, "clang::attr::CUDADeviceBuiltinTextureType"),
      add_named_node(&mut g, 4294972555, "clang::attr::CUDAGlobal"),
      add_named_node(&mut g, 4294972556, "clang::attr::CUDAHost"),
      add_named_node(&mut g, 4294972557, "clang::attr::CUDAInvalidTarget"),
      add_named_node(&mut g, 4294972558, "clang::attr::CUDALaunchBounds"),
      add_named_node(&mut g, 4294972559, "clang::attr::CUDAShared"),
      add_named_node(&mut g, 4294972560, "clang::attr::CXX11NoReturn"),
      add_named_node(&mut g, 4294972561, "clang::attr::CallableWhen"),
      add_named_node(&mut g, 4294972562, "clang::attr::Callback"),
      add_named_node(&mut g, 4294972563, "clang::attr::Capability"),
      add_named_node(&mut g, 4294972564, "clang::attr::CapturedRecord"),
      add_named_node(&mut g, 4294972565, "clang::attr::Cleanup"),
      add_named_node(&mut g, 4294972566, "clang::attr::CmseNSEntry"),
      add_named_node(&mut g, 4294972567, "clang::attr::CodeModel"),
      add_named_node(&mut g, 4294972568, "clang::attr::CodeSeg"),
      add_named_node(&mut g, 4294972569, "clang::attr::Cold"),
      add_named_node(&mut g, 4294972570, "clang::attr::Common"),
      add_named_node(&mut g, 4294972571, "clang::attr::Const"),
      add_named_node(&mut g, 4294972572, "clang::attr::ConstInit"),
      add_named_node(&mut g, 4294972573, "clang::attr::Constructor"),
      add_named_node(&mut g, 4294972574, "clang::attr::Consumable"),
      add_named_node(&mut g, 4294972575, "clang::attr::ConsumableAutoCast"),
      add_named_node(&mut g, 4294972576, "clang::attr::ConsumableSetOnRead"),
      add_named_node(&mut g, 4294972577, "clang::attr::Convergent"),
      add_named_node(&mut g, 4294972578, "clang::attr::CoroDisableLifetimeBound"),
      add_named_node(&mut g, 4294972579, "clang::attr::CoroLifetimeBound"),
      add_named_node(&mut g, 4294972580, "clang::attr::CoroOnlyDestroyWhenComplete"),
      add_named_node(&mut g, 4294972581, "clang::attr::CoroReturnType"),
      add_named_node(&mut g, 4294972582, "clang::attr::CoroWrapper"),
      add_named_node(&mut g, 4294972583, "clang::attr::CountedBy"),
      add_named_node(&mut g, 4294972584, "clang::attr::DLLExport"),
      add_named_node(&mut g, 4294972585, "clang::attr::DLLExportStaticLocal"),
      add_named_node(&mut g, 4294972586, "clang::attr::DLLImport"),
      add_named_node(&mut g, 4294972587, "clang::attr::DLLImportStaticLocal"),
      add_named_node(&mut g, 4294972588, "clang::attr::Deprecated"),
      add_named_node(&mut g, 4294972589, "clang::attr::Destructor"),
      add_named_node(&mut g, 4294972590, "clang::attr::DiagnoseAsBuiltin"),
      add_named_node(&mut g, 4294972591, "clang::attr::DiagnoseIf"),
      add_named_node(&mut g, 4294972592, "clang::attr::DisableSanitizerInstrumentation"),
      add_named_node(&mut g, 4294972593, "clang::attr::DisableTailCalls"),
      add_named_node(&mut g, 4294972594, "clang::attr::EmptyBases"),
      add_named_node(&mut g, 4294972595, "clang::attr::EnableIf"),
      add_named_node(&mut g, 4294972596, "clang::attr::EnforceTCB"),
      add_named_node(&mut g, 4294972597, "clang::attr::EnforceTCBLeaf"),
      add_named_node(&mut g, 4294972598, "clang::attr::EnumExtensibility"),
      add_named_node(&mut g, 4294972599, "clang::attr::Error"),
      add_named_node(&mut g, 4294972600, "clang::attr::ExcludeFromExplicitInstantiation"),
      add_named_node(&mut g, 4294972601, "clang::attr::ExclusiveTrylockFunction"),
      add_named_node(&mut g, 4294972602, "clang::attr::ExternalSourceSymbol"),
      add_named_node(&mut g, 4294972603, "clang::attr::Final"),
      add_named_node(&mut g, 4294972604, "clang::attr::FlagEnum"),
      add_named_node(&mut g, 4294972605, "clang::attr::Flatten"),
      add_named_node(&mut g, 4294972606, "clang::attr::Format"),
      add_named_node(&mut g, 4294972607, "clang::attr::FormatArg"),
      add_named_node(&mut g, 4294972608, "clang::attr::FunctionReturnThunks"),
      add_named_node(&mut g, 4294972609, "clang::attr::GNUInline"),
      add_named_node(&mut g, 4294972610, "clang::attr::GuardedBy"),
      add_named_node(&mut g, 4294972611, "clang::attr::GuardedVar"),
      add_named_node(&mut g, 4294972612, "clang::attr::HIPManaged"),
      add_named_node(&mut g, 4294972613, "clang::attr::HLSLNumThreads"),
      add_named_node(&mut g, 4294972614, "clang::attr::HLSLResource"),
      add_named_node(&mut g, 4294972615, "clang::attr::HLSLResourceBinding"),
      add_named_node(&mut g, 4294972616, "clang::attr::HLSLShader"),
      add_named_node(&mut g, 4294972617, "clang::attr::Hot"),
      add_named_node(&mut g, 4294972618, "clang::attr::IBAction"),
      add_named_node(&mut g, 4294972619, "clang::attr::IBOutlet"),
      add_named_node(&mut g, 4294972620, "clang::attr::IBOutletCollection"),
      add_named_node(&mut g, 4294972621, "clang::attr::InitPriority"),
      add_named_node(&mut g, 4294972622, "clang::attr::InternalLinkage"),
      add_named_node(&mut g, 4294972623, "clang::attr::LTOVisibilityPublic"),
      add_named_node(&mut g, 4294972624, "clang::attr::LayoutVersion"),
      add_named_node(&mut g, 4294972625, "clang::attr::Leaf"),
      add_named_node(&mut g, 4294972626, "clang::attr::LockReturned"),
      add_named_node(&mut g, 4294972627, "clang::attr::LocksExcluded"),
      add_named_node(&mut g, 4294972628, "clang::attr::M68kInterrupt"),
      add_named_node(&mut g, 4294972629, "clang::attr::MIGServerRoutine"),
      add_named_node(&mut g, 4294972630, "clang::attr::MSAllocator"),
      add_named_node(&mut g, 4294972631, "clang::attr::MSConstexpr"),
      add_named_node(&mut g, 4294972632, "clang::attr::MSInheritance"),
      add_named_node(&mut g, 4294972633, "clang::attr::MSNoVTable"),
      add_named_node(&mut g, 4294972634, "clang::attr::MSP430Interrupt"),
      add_named_node(&mut g, 4294972635, "clang::attr::MSStruct"),
      add_named_node(&mut g, 4294972636, "clang::attr::MSVtorDisp"),
      add_named_node(&mut g, 4294972637, "clang::attr::MaxFieldAlignment"),
      add_named_node(&mut g, 4294972638, "clang::attr::MayAlias"),
      add_named_node(&mut g, 4294972639, "clang::attr::MaybeUndef"),
      add_named_node(&mut g, 4294972640, "clang::attr::MicroMips"),
      add_named_node(&mut g, 4294972641, "clang::attr::MinSize"),
      add_named_node(&mut g, 4294972642, "clang::attr::MinVectorWidth"),
      add_named_node(&mut g, 4294972643, "clang::attr::Mips16"),
      add_named_node(&mut g, 4294972644, "clang::attr::MipsInterrupt"),
      add_named_node(&mut g, 4294972645, "clang::attr::MipsLongCall"),
      add_named_node(&mut g, 4294972646, "clang::attr::MipsShortCall"),
      add_named_node(&mut g, 4294972647, "clang::attr::NSConsumesSelf"),
      add_named_node(&mut g, 4294972648, "clang::attr::NSErrorDomain"),
      add_named_node(&mut g, 4294972649, "clang::attr::NSReturnsAutoreleased"),
      add_named_node(&mut g, 4294972650, "clang::attr::NSReturnsNotRetained"),
      add_named_node(&mut g, 4294972651, "clang::attr::NVPTXKernel"),
      add_named_node(&mut g, 4294972652, "clang::attr::Naked"),
      add_named_node(&mut g, 4294972653, "clang::attr::NoAlias"),
      add_named_node(&mut g, 4294972654, "clang::attr::NoCommon"),
      add_named_node(&mut g, 4294972655, "clang::attr::NoDebug"),
      add_named_node(&mut g, 4294972656, "clang::attr::NoDestroy"),
      add_named_node(&mut g, 4294972657, "clang::attr::NoDuplicate"),
      add_named_node(&mut g, 4294972658, "clang::attr::NoInstrumentFunction"),
      add_named_node(&mut g, 4294972659, "clang::attr::NoMicroMips"),
      add_named_node(&mut g, 4294972660, "clang::attr::NoMips16"),
      add_named_node(&mut g, 4294972661, "clang::attr::NoProfileFunction"),
      add_named_node(&mut g, 4294972662, "clang::attr::NoRandomizeLayout"),
      add_named_node(&mut g, 4294972663, "clang::attr::NoReturn"),
      add_named_node(&mut g, 4294972664, "clang::attr::NoSanitize"),
      add_named_node(&mut g, 4294972665, "clang::attr::NoSpeculativeLoadHardening"),
      add_named_node(&mut g, 4294972666, "clang::attr::NoSplitStack"),
      add_named_node(&mut g, 4294972667, "clang::attr::NoStackProtector"),
      add_named_node(&mut g, 4294972668, "clang::attr::NoThreadSafetyAnalysis"),
      add_named_node(&mut g, 4294972669, "clang::attr::NoThrow"),
      add_named_node(&mut g, 4294972670, "clang::attr::NoUniqueAddress"),
      add_named_node(&mut g, 4294972671, "clang::attr::NoUwtable"),
      add_named_node(&mut g, 4294972672, "clang::attr::NotTailCalled"),
      add_named_node(&mut g, 4294972673, "clang::attr::OMPAllocateDecl"),
      add_named_node(&mut g, 4294972674, "clang::attr::OMPCaptureNoInit"),
      add_named_node(&mut g, 4294972675, "clang::attr::OMPDeclareTargetDecl"),
      add_named_node(&mut g, 4294972676, "clang::attr::OMPDeclareVariant"),
      add_named_node(&mut g, 4294972677, "clang::attr::OMPThreadPrivateDecl"),
      add_named_node(&mut g, 4294972678, "clang::attr::OSConsumesThis"),
      add_named_node(&mut g, 4294972679, "clang::attr::OSReturnsNotRetained"),
      add_named_node(&mut g, 4294972680, "clang::attr::OSReturnsRetained"),
      add_named_node(&mut g, 4294972681, "clang::attr::OSReturnsRetainedOnNonZero"),
      add_named_node(&mut g, 4294972682, "clang::attr::OSReturnsRetainedOnZero"),
      add_named_node(&mut g, 4294972683, "clang::attr::ObjCBridge"),
      add_named_node(&mut g, 4294972684, "clang::attr::ObjCBridgeMutable"),
      add_named_node(&mut g, 4294972685, "clang::attr::ObjCBridgeRelated"),
      add_named_node(&mut g, 4294972686, "clang::attr::ObjCException"),
      add_named_node(&mut g, 4294972687, "clang::attr::ObjCExplicitProtocolImpl"),
      add_named_node(&mut g, 4294972688, "clang::attr::ObjCExternallyRetained"),
      add_named_node(&mut g, 4294972689, "clang::attr::ObjCIndependentClass"),
      add_named_node(&mut g, 4294972690, "clang::attr::ObjCMethodFamily"),
      add_named_node(&mut g, 4294972691, "clang::attr::ObjCNSObject"),
      add_named_node(&mut g, 4294972692, "clang::attr::ObjCPreciseLifetime"),
      add_named_node(&mut g, 4294972693, "clang::attr::ObjCRequiresPropertyDefs"),
      add_named_node(&mut g, 4294972694, "clang::attr::ObjCRequiresSuper"),
      add_named_node(&mut g, 4294972695, "clang::attr::ObjCReturnsInnerPointer"),
      add_named_node(&mut g, 4294972696, "clang::attr::ObjCRootClass"),
      add_named_node(&mut g, 4294972697, "clang::attr::ObjCSubclassingRestricted"),
      add_named_node(&mut g, 4294972698, "clang::attr::OpenCLIntelReqdSubGroupSize"),
      add_named_node(&mut g, 4294972699, "clang::attr::OpenCLKernel"),
      add_named_node(&mut g, 4294972700, "clang::attr::OptimizeNone"),
      add_named_node(&mut g, 4294972701, "clang::attr::Override"),
      add_named_node(&mut g, 4294972702, "clang::attr::Owner"),
      add_named_node(&mut g, 4294972703, "clang::attr::Ownership"),
      add_named_node(&mut g, 4294972704, "clang::attr::Packed"),
      add_named_node(&mut g, 4294972705, "clang::attr::ParamTypestate"),
      add_named_node(&mut g, 4294972706, "clang::attr::PatchableFunctionEntry"),
      add_named_node(&mut g, 4294972707, "clang::attr::Pointer"),
      add_named_node(&mut g, 4294972708, "clang::attr::PragmaClangBSSSection"),
      add_named_node(&mut g, 4294972709, "clang::attr::PragmaClangDataSection"),
      add_named_node(&mut g, 4294972710, "clang::attr::PragmaClangRelroSection"),
      add_named_node(&mut g, 4294972711, "clang::attr::PragmaClangRodataSection"),
      add_named_node(&mut g, 4294972712, "clang::attr::PragmaClangTextSection"),
      add_named_node(&mut g, 4294972713, "clang::attr::PreferredName"),
      add_named_node(&mut g, 4294972714, "clang::attr::PreferredType"),
      add_named_node(&mut g, 4294972715, "clang::attr::PtGuardedBy"),
      add_named_node(&mut g, 4294972716, "clang::attr::PtGuardedVar"),
      add_named_node(&mut g, 4294972717, "clang::attr::Pure"),
      add_named_node(&mut g, 4294972718, "clang::attr::RISCVInterrupt"),
      add_named_node(&mut g, 4294972719, "clang::attr::RandomizeLayout"),
      add_named_node(&mut g, 4294972720, "clang::attr::ReadOnlyPlacement"),
      add_named_node(&mut g, 4294972721, "clang::attr::Reinitializes"),
      add_named_node(&mut g, 4294972722, "clang::attr::ReleaseCapability"),
      add_named_node(&mut g, 4294972723, "clang::attr::ReqdWorkGroupSize"),
      add_named_node(&mut g, 4294972724, "clang::attr::RequiresCapability"),
      add_named_node(&mut g, 4294972725, "clang::attr::Restrict"),
      add_named_node(&mut g, 4294972726, "clang::attr::Retain"),
      add_named_node(&mut g, 4294972727, "clang::attr::ReturnTypestate"),
      add_named_node(&mut g, 4294972728, "clang::attr::ReturnsNonNull"),
      add_named_node(&mut g, 4294972729, "clang::attr::ReturnsTwice"),
      add_named_node(&mut g, 4294972730, "clang::attr::SYCLKernel"),
      add_named_node(&mut g, 4294972731, "clang::attr::SYCLSpecialClass"),
      add_named_node(&mut g, 4294972732, "clang::attr::ScopedLockable"),
      add_named_node(&mut g, 4294972733, "clang::attr::Section"),
      add_named_node(&mut g, 4294972734, "clang::attr::SelectAny"),
      add_named_node(&mut g, 4294972735, "clang::attr::Sentinel"),
      add_named_node(&mut g, 4294972736, "clang::attr::SetTypestate"),
      add_named_node(&mut g, 4294972737, "clang::attr::SharedTrylockFunction"),
      add_named_node(&mut g, 4294972738, "clang::attr::SpeculativeLoadHardening"),
      add_named_node(&mut g, 4294972739, "clang::attr::StandaloneDebug"),
      add_named_node(&mut g, 4294972740, "clang::attr::StrictFP"),
      add_named_node(&mut g, 4294972741, "clang::attr::StrictGuardStackCheck"),
      add_named_node(&mut g, 4294972742, "clang::attr::SwiftAsync"),
      add_named_node(&mut g, 4294972743, "clang::attr::SwiftAsyncError"),
      add_named_node(&mut g, 4294972744, "clang::attr::SwiftAsyncName"),
      add_named_node(&mut g, 4294972745, "clang::attr::SwiftAttr"),
      add_named_node(&mut g, 4294972746, "clang::attr::SwiftBridge"),
      add_named_node(&mut g, 4294972747, "clang::attr::SwiftBridgedTypedef"),
      add_named_node(&mut g, 4294972748, "clang::attr::SwiftError"),
      add_named_node(&mut g, 4294972749, "clang::attr::SwiftImportAsNonGeneric"),
      add_named_node(&mut g, 4294972750, "clang::attr::SwiftImportPropertyAsAccessors"),
      add_named_node(&mut g, 4294972751, "clang::attr::SwiftName"),
      add_named_node(&mut g, 4294972752, "clang::attr::SwiftNewType"),
      add_named_node(&mut g, 4294972753, "clang::attr::SwiftPrivate"),
      add_named_node(&mut g, 4294972754, "clang::attr::TLSModel"),
      add_named_node(&mut g, 4294972755, "clang::attr::Target"),
      add_named_node(&mut g, 4294972756, "clang::attr::TargetClones"),
      add_named_node(&mut g, 4294972757, "clang::attr::TargetVersion"),
      add_named_node(&mut g, 4294972758, "clang::attr::TestTypestate"),
      add_named_node(&mut g, 4294972759, "clang::attr::TransparentUnion"),
      add_named_node(&mut g, 4294972760, "clang::attr::TrivialABI"),
      add_named_node(&mut g, 4294972761, "clang::attr::TryAcquireCapability"),
      add_named_node(&mut g, 4294972762, "clang::attr::TypeTagForDatatype"),
      add_named_node(&mut g, 4294972763, "clang::attr::TypeVisibility"),
      add_named_node(&mut g, 4294972764, "clang::attr::Unavailable"),
      add_named_node(&mut g, 4294972765, "clang::attr::Uninitialized"),
      add_named_node(&mut g, 4294972766, "clang::attr::UnsafeBufferUsage"),
      add_named_node(&mut g, 4294972767, "clang::attr::Unused"),
      add_named_node(&mut g, 4294972768, "clang::attr::Used"),
      add_named_node(&mut g, 4294972769, "clang::attr::UsingIfExists"),
      add_named_node(&mut g, 4294972770, "clang::attr::Uuid"),
      add_named_node(&mut g, 4294972771, "clang::attr::VecReturn"),
      add_named_node(&mut g, 4294972772, "clang::attr::VecTypeHint"),
      add_named_node(&mut g, 4294972773, "clang::attr::Visibility"),
      add_named_node(&mut g, 4294972774, "clang::attr::WarnUnused"),
      add_named_node(&mut g, 4294972775, "clang::attr::WarnUnusedResult"),
      add_named_node(&mut g, 4294972776, "clang::attr::Weak"),
      add_named_node(&mut g, 4294972777, "clang::attr::WeakImport"),
      add_named_node(&mut g, 4294972778, "clang::attr::WeakRef"),
      add_named_node(&mut g, 4294972779, "clang::attr::WebAssemblyExportName"),
      add_named_node(&mut g, 4294972780, "clang::attr::WebAssemblyImportModule"),
      add_named_node(&mut g, 4294972781, "clang::attr::WebAssemblyImportName"),
      add_named_node(&mut g, 4294972782, "clang::attr::WorkGroupSizeHint"),
      add_named_node(&mut g, 4294972783, "clang::attr::X86ForceAlignArgPointer"),
      add_named_node(&mut g, 4294972784, "clang::attr::XRayInstrument"),
      add_named_node(&mut g, 4294972785, "clang::attr::XRayLogArgs"),
      add_named_node(&mut g, 4294972786, "clang::attr::ZeroCallUsedRegs"),
      add_named_node(&mut g, 4294972787, "clang::attr::AbiTag"),
      add_named_node(&mut g, 4294972788, "clang::attr::Alias"),
      add_named_node(&mut g, 4294972789, "clang::attr::AlignValue"),
      add_named_node(&mut g, 4294972790, "clang::attr::BuiltinAlias"),
      add_named_node(&mut g, 4294972791, "clang::attr::CalledOnce"),
      add_named_node(&mut g, 4294972792, "clang::attr::IFunc"),
      add_named_node(&mut g, 4294972793, "clang::attr::InitSeg"),
      add_named_node(&mut g, 4294972794, "clang::attr::LoaderUninitialized"),
      add_named_node(&mut g, 4294972795, "clang::attr::LoopHint"),
      add_named_node(&mut g, 4294972796, "clang::attr::Mode"),
      add_named_node(&mut g, 4294972797, "clang::attr::NoBuiltin"),
      add_named_node(&mut g, 4294972798, "clang::attr::NoEscape"),
      add_named_node(&mut g, 4294972799, "clang::attr::OMPCaptureKind"),
      add_named_node(&mut g, 4294972800, "clang::attr::OMPDeclareSimdDecl"),
      add_named_node(&mut g, 4294972801, "clang::attr::OMPReferencedVar"),
      add_named_node(&mut g, 4294972802, "clang::attr::ObjCBoxable"),
      add_named_node(&mut g, 4294972803, "clang::attr::ObjCClassStub"),
      add_named_node(&mut g, 4294972804, "clang::attr::ObjCDesignatedInitializer"),
      add_named_node(&mut g, 4294972805, "clang::attr::ObjCDirect"),
      add_named_node(&mut g, 4294972806, "clang::attr::ObjCDirectMembers"),
      add_named_node(&mut g, 4294972807, "clang::attr::ObjCNonLazyClass"),
      add_named_node(&mut g, 4294972808, "clang::attr::ObjCNonRuntimeProtocol"),
      add_named_node(&mut g, 4294972809, "clang::attr::ObjCRuntimeName"),
      add_named_node(&mut g, 4294972810, "clang::attr::ObjCRuntimeVisible"),
      add_named_node(&mut g, 4294972811, "clang::attr::OpenCLAccess"),
      add_named_node(&mut g, 4294972812, "clang::attr::Overloadable"),
      add_named_node(&mut g, 4294972813, "clang::attr::RenderScriptKernel"),
      add_named_node(&mut g, 4294972814, "clang::attr::SwiftObjCMembers"),
      add_named_node(&mut g, 4294972815, "clang::attr::SwiftVersionedAddition"),
      add_named_node(&mut g, 4294972816, "clang::attr::SwiftVersionedRemoval"),
      add_named_node(&mut g, 4294972817, "clang::attr::Thread"),
      add_named_node(&mut g, 4294972818, "clang::attr::FirstAttr"),
      add_named_node(&mut g, 4294972819, "clang::attr::LastAttr"),
      add_named_node(&mut g, 4294972820, "clang::attr::FirstTypeAttr"),
      add_named_node(&mut g, 4294972821, "clang::attr::LastTypeAttr"),
      add_named_node(&mut g, 4294972822, "clang::attr::FirstStmtAttr"),
      add_named_node(&mut g, 4294972823, "clang::attr::LastStmtAttr"),
      add_named_node(&mut g, 4294972824, "clang::attr::FirstDeclOrStmtAttr"),
      add_named_node(&mut g, 4294972825, "clang::attr::LastDeclOrStmtAttr"),
      add_named_node(&mut g, 4294972826, "clang::attr::FirstInheritableAttr"),
      add_named_node(&mut g, 4294972827, "clang::attr::LastInheritableAttr"),
      add_named_node(&mut g, 4294972828, "clang::attr::FirstDeclOrTypeAttr"),
      add_named_node(&mut g, 4294972829, "clang::attr::LastDeclOrTypeAttr"),
      add_named_node(&mut g, 4294972830, "clang::attr::FirstInheritableParamAttr"),
      add_named_node(&mut g, 4294972831, "clang::attr::LastInheritableParamAttr"),
      add_named_node(&mut g, 4294972832, "clang::attr::FirstParameterABIAttr"),
      add_named_node(&mut g, 4294972833, "clang::attr::LastParameterABIAttr"),
      add_named_node(&mut g, 4294972834, "clang::attr::FirstHLSLAnnotationAttr"),
      add_named_node(&mut g, 4294972835, "clang::attr::LastHLSLAnnotationAttr"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_KIND_1, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g.add_named_node(ENUM_SEMANTICS, "llvm::APFloatBase::Semantics");
  g.add_edge((ENUM_SEMANTICS, META_CLASS, META_CLANG_AST_NODE));
  {
    let enumerators = Vec::from([
      add_named_node(&mut g, 4294972837, "llvm::APFloatBase::S_IEEEhalf"),
      add_named_node(&mut g, 4294972838, "llvm::APFloatBase::S_BFloat"),
      add_named_node(&mut g, 4294972839, "llvm::APFloatBase::S_IEEEsingle"),
      add_named_node(&mut g, 4294972840, "llvm::APFloatBase::S_IEEEdouble"),
      add_named_node(&mut g, 4294972841, "llvm::APFloatBase::S_IEEEquad"),
      add_named_node(&mut g, 4294972842, "llvm::APFloatBase::S_PPCDoubleDouble"),
      add_named_node(&mut g, 4294972843, "llvm::APFloatBase::S_Float8E5M2"),
      add_named_node(&mut g, 4294972844, "llvm::APFloatBase::S_Float8E5M2FNUZ"),
      add_named_node(&mut g, 4294972845, "llvm::APFloatBase::S_Float8E4M3FN"),
      add_named_node(&mut g, 4294972846, "llvm::APFloatBase::S_Float8E4M3FNUZ"),
      add_named_node(&mut g, 4294972847, "llvm::APFloatBase::S_Float8E4M3B11FNUZ"),
      add_named_node(&mut g, 4294972848, "llvm::APFloatBase::S_FloatTF32"),
      add_named_node(&mut g, 4294972849, "llvm::APFloatBase::S_x87DoubleExtended"),
      add_named_node(&mut g, 4294972850, "llvm::APFloatBase::S_MaxSemantics"),
    ]);
    let enumerators = set_node(&mut g, META_CLANG_AST_ENUM_CONSTANT, next_id(), next_id(), enumerators);
    g.add_edge((ENUM_SEMANTICS, META_CLANG_AST_ENUM_ENUMERATORS, enumerators));
  }

  g
}

const TRUE_: u64 = 4294967296;
const FALSE_: u64 = 4294967297;
const META_CLASS: u64 = 4294967298;
const META_SUBCLASS: u64 = 4294967299;
const META_METHOD: u64 = 4294967300;
const META_DOMAIN: u64 = 4294967301;
const META_RANGE: u64 = 4294967302;
const META_CLANG_AST_NODE: u64 = 4294967303;
const META_CLANG_AST_METHOD: u64 = 4294967304;
const META_CLANG_AST_ENUM: u64 = 4294967305;
const META_CLANG_AST_ENUM_ENUMERATORS: u64 = 4294967306;
const META_CLANG_AST_ENUM_CONSTANT: u64 = 4294967307;
const BUILTIN_STRING_CLASS: u64 = 4294967308;
const BUILTIN_U64_CLASS: u64 = 4294967309;
const BUILTIN_I64_CLASS: u64 = 4294967310;
const BUILTIN_DOUBLE_CLASS: u64 = 4294967311;
const BUILTIN_LIST_CLASS: u64 = 4294967312;
const BUILTIN_LIST_SIZE: u64 = 4294967313;
const BUILTIN_LIST_ITEM_CLASS: u64 = 4294967314;
const BUILTIN_SET_CLASS: u64 = 4294967315;
const BUILTIN_SET_SIZE: u64 = 4294967316;
const BUILTIN_SET_ITEM: u64 = 4294967317;
const BUILTIN_SET_ITEM_CLASS: u64 = 4294967318;
const INVALID_FILE: u64 = 4294967319;
const INVALID_SOURCE_LOCATION: u64 = 4294967320;
const FILE_CLASS: u64 = 4294967321;
const FILE_NAME: u64 = 4294967322;
const FILE_CONTENT: u64 = 4294967323;
const SOURCE_LOCATION_CLASS: u64 = 4294967324;
const SOURCE_LOCATION_IS_FILE: u64 = 4294967325;
const SOURCE_LOCATION_FILE: u64 = 4294967326;
const SOURCE_LOCATION_LINE: u64 = 4294967327;
const SOURCE_LOCATION_COLUMN: u64 = 4294967328;
const SOURCE_LOCATION_EXPANSION_LOC: u64 = 4294967329;
const SOURCE_LOCATION_SPELLING_LOC: u64 = 4294967330;
const SOURCE_RANGE_CLASS: u64 = 4294967331;
const SOURCE_RANGE_BEGIN: u64 = 4294967332;
const SOURCE_RANGE_END: u64 = 4294967333;
const QUALTYPE_CLASS: u64 = 4294967334;
const QUALTYPE_IS_CONST: u64 = 4294967335;
const QUALTYPE_IS_VOLATILE: u64 = 4294967336;
const QUALTYPE_IS_RESTRICT: u64 = 4294967337;
const QUALTYPE_TYPE: u64 = 4294967338;
const USR: u64 = 4294967339;
const CLASS_OBJCARRAYLITERAL: u64 = 4294967340;
const CLASS_CXXSTDINITIALIZERLISTEXPR: u64 = 4294967341;
const CLASS_OBJCSELECTOREXPR: u64 = 4294967347;
const CLASS_CXXTYPEIDEXPR: u64 = 4294967348;
const CLASS_OBJCPROPERTYREFEXPR: u64 = 4294967357;
const CLASS_ASMSTMT: u64 = 4294967358;
const CLASS_OMPTARGETTEAMSDISTRIBUTESIMDDIRECTIVE: u64 = 4294967375;
const CLASS_OBJCMESSAGEEXPR: u64 = 4294967376;
const CLASS_CXXPARENLISTINITEXPR: u64 = 4294967377;
const CLASS_VALUESTMT: u64 = 4294967387;
const CLASS_CONVERTVECTOREXPR: u64 = 4294967389;
const CLASS_GOTOSTMT: u64 = 4294967397;
const CLASS_BREAKSTMT: u64 = 4294967404;
const CLASS_CXXDYNAMICCASTEXPR: u64 = 4294967409;
const CLASS_COROUTINEBODYSTMT: u64 = 4294967411;
const CLASS_CORETURNSTMT: u64 = 4294967432;
const CLASS_ARRAYTYPETRAITEXPR: u64 = 4294967440;
const CLASS_OBJCATCATCHSTMT: u64 = 4294967449;
const CLASS_FORSTMT: u64 = 4294967450;
const CLASS_OBJCAUTORELEASEPOOLSTMT: u64 = 4294967463;
const CLASS_OBJCDICTIONARYLITERAL: u64 = 4294967464;
const CLASS_OMPLOOPTRANSFORMATIONDIRECTIVE: u64 = 4294967465;
const CLASS_OBJCSUBSCRIPTREFEXPR: u64 = 4294967466;
const CLASS_SYCLUNIQUESTABLENAMEEXPR: u64 = 4294967467;
const CLASS_OMPFORSIMDDIRECTIVE: u64 = 4294967475;
const CLASS_OMPPARALLELSECTIONSDIRECTIVE: u64 = 4294967476;
const CLASS_CONDITIONALOPERATOR: u64 = 4294967477;
const CLASS_OMPTARGETPARALLELFORDIRECTIVE: u64 = 4294967486;
const CLASS_OMPTASKLOOPSIMDDIRECTIVE: u64 = 4294967487;
const CLASS_CASTEXPR: u64 = 4294967488;
const CLASS_OMPMASTERTASKLOOPSIMDDIRECTIVE: u64 = 4294967505;
const CLASS_CXXTRYSTMT: u64 = 4294967506;
const CLASS_OMPMASKEDTASKLOOPSIMDDIRECTIVE: u64 = 4294967513;
const CLASS_OBJCISAEXPR: u64 = 4294967514;
const CLASS_OBJCATFINALLYSTMT: u64 = 4294967515;
const CLASS_OMPPARALLELMASTERTASKLOOPDIRECTIVE: u64 = 4294967516;
const CLASS_OBJCIVARREFEXPR: u64 = 4294967517;
const CLASS_OMPTASKYIELDDIRECTIVE: u64 = 4294967518;
const CLASS_OMPITERATOREXPR: u64 = 4294967519;
const CLASS_OMPDISTRIBUTEPARALLELFORDIRECTIVE: u64 = 4294967520;
const CLASS_CONSTANTEXPR: u64 = 4294967521;
const CLASS_OMPPARALLELFORSIMDDIRECTIVE: u64 = 4294967531;
const CLASS_CXXNOEXCEPTEXPR: u64 = 4294967532;
const CLASS_OMPPARALLELMASKEDDIRECTIVE: u64 = 4294967539;
const CLASS_INITLISTEXPR: u64 = 4294967540;
const CLASS_OMPTARGETTEAMSGENERICLOOPDIRECTIVE: u64 = 4294967565;
const CLASS_OMPTARGETDATADIRECTIVE: u64 = 4294967566;
const CLASS_OMPTASKGROUPDIRECTIVE: u64 = 4294967567;
const CLASS_OMPTARGETTEAMSDIRECTIVE: u64 = 4294967568;
const CLASS_OMPSCANDIRECTIVE: u64 = 4294967569;
const CLASS_OMPMETADIRECTIVE: u64 = 4294967570;
const CLASS_OMPATOMICDIRECTIVE: u64 = 4294967571;
const CLASS_COMPOUNDSTMT: u64 = 4294967572;
const CLASS_OMPDISTRIBUTEDIRECTIVE: u64 = 4294967590;
const CLASS_OMPDISTRIBUTEPARALLELFORSIMDDIRECTIVE: u64 = 4294967591;
const CLASS_OMPGENERICLOOPDIRECTIVE: u64 = 4294967592;
const CLASS_CAPTUREDSTMT: u64 = 4294967593;
const CLASS_COYIELDEXPR: u64 = 4294967609;
const CLASS_OBJCATTHROWSTMT: u64 = 4294967610;
const CLASS_OMPTARGETEXITDATADIRECTIVE: u64 = 4294967611;
const CLASS_OMPTARGETENTERDATADIRECTIVE: u64 = 4294967612;
const CLASS_OMPTARGETTEAMSDISTRIBUTEPARALLELFORDIRECTIVE: u64 = 4294967613;
const CLASS_OMPEXECUTABLEDIRECTIVE: u64 = 4294967614;
const CLASS_OMPMASTERTASKLOOPDIRECTIVE: u64 = 4294967615;
const CLASS_CHOOSEEXPR: u64 = 4294967616;
const CLASS_EXPLICITCASTEXPR: u64 = 4294967628;
const CLASS_OMPPARALLELGENERICLOOPDIRECTIVE: u64 = 4294967631;
const CLASS_OMPTEAMSGENERICLOOPDIRECTIVE: u64 = 4294967632;
const CLASS_OMPMASKEDDIRECTIVE: u64 = 4294967633;
const CLASS_CXXUNRESOLVEDCONSTRUCTEXPR: u64 = 4294967634;
const CLASS_USERDEFINEDLITERAL: u64 = 4294967647;
const CLASS_OBJCATTRYSTMT: u64 = 4294967654;
const CLASS_OMPTARGETTEAMSDISTRIBUTEPARALLELFORSIMDDIRECTIVE: u64 = 4294967655;
const CLASS_CXXADDRSPACECASTEXPR: u64 = 4294967656;
const CLASS_FUNCTIONPARMPACKEXPR: u64 = 4294967657;
const CLASS_DECLREFEXPR: u64 = 4294967666;
const CLASS_OMPTEAMSDISTRIBUTEPARALLELFORSIMDDIRECTIVE: u64 = 4294967691;
const CLASS_OMPARRAYSHAPINGEXPR: u64 = 4294967692;
const CLASS_UNRESOLVEDLOOKUPEXPR: u64 = 4294967693;
const CLASS_PSEUDOOBJECTEXPR: u64 = 4294967700;
const CLASS_PARENEXPR: u64 = 4294967712;
const CLASS_OMPTEAMSDISTRIBUTESIMDDIRECTIVE: u64 = 4294967719;
const CLASS_ABSTRACTCONDITIONALOPERATOR: u64 = 4294967720;
const CLASS_BINARYCONDITIONALOPERATOR: u64 = 4294967726;
const CLASS_OMPBARRIERDIRECTIVE: u64 = 4294967735;
const CLASS_OMPTASKDIRECTIVE: u64 = 4294967736;
const CLASS_SHUFFLEVECTOREXPR: u64 = 4294967737;
const CLASS_OMPFORDIRECTIVE: u64 = 4294967744;
const CLASS_CONCEPTSPECIALIZATIONEXPR: u64 = 4294967745;
const CLASS_OMPPARALLELMASKEDTASKLOOPSIMDDIRECTIVE: u64 = 4294967763;
const CLASS_LABELSTMT: u64 = 4294967764;
const CLASS_GNUNULLEXPR: u64 = 4294967773;
const CLASS_OBJCBOOLLITERALEXPR: u64 = 4294967778;
const CLASS_OMPINTEROPDIRECTIVE: u64 = 4294967779;
const CLASS_REQUIRESEXPR: u64 = 4294967780;
const CLASS_COAWAITEXPR: u64 = 4294967792;
const CLASS_MATERIALIZETEMPORARYEXPR: u64 = 4294967794;
const CLASS_OMPTARGETDIRECTIVE: u64 = 4294967804;
const CLASS_PACKEXPANSIONEXPR: u64 = 4294967805;
const CLASS_CXXTHISEXPR: u64 = 4294967812;
const CLASS_CXXDEPENDENTSCOPEMEMBEREXPR: u64 = 4294967818;
const CLASS_TYPETRAITEXPR: u64 = 4294967841;
const CLASS_CXXNEWEXPR: u64 = 4294967849;
const CLASS_CXXINHERITEDCTORINITEXPR: u64 = 4294967877;
const CLASS_IMPLICITCASTEXPR: u64 = 4294967886;
const CLASS_CXXCONSTRUCTEXPR: u64 = 4294967890;
const CLASS_COMPOUNDLITERALEXPR: u64 = 4294967909;
const CLASS_CXXDEFAULTINITEXPR: u64 = 4294967917;
const CLASS_CXXDEFAULTARGEXPR: u64 = 4294967927;
const CLASS_OMPSECTIONDIRECTIVE: u64 = 4294967939;
const CLASS_CXXTHROWEXPR: u64 = 4294967940;
const CLASS_MSPROPERTYSUBSCRIPTEXPR: u64 = 4294967947;
const CLASS_CXXREWRITTENBINARYOPERATOR: u64 = 4294967955;
const CLASS_CUDAKERNELCALLEXPR: u64 = 4294967971;
const CLASS_TYPOEXPR: u64 = 4294967973;
const CLASS_ASTYPEEXPR: u64 = 4294967977;
const CLASS_EXTVECTORELEMENTEXPR: u64 = 4294967984;
const CLASS_GENERICSELECTIONEXPR: u64 = 4294967994;
const CLASS_IMPLICITVALUEINITEXPR: u64 = 4294968012;
const CLASS_NOINITEXPR: u64 = 4294968016;
const CLASS_ARRAYINITINDEXEXPR: u64 = 4294968020;
const CLASS_DESIGNATEDINITEXPR: u64 = 4294968024;
const CLASS_STMTEXPR: u64 = 4294968036;
const CLASS_BINARYOPERATOR: u64 = 4294968044;
const CLASS_CXXSTATICCASTEXPR: u64 = 4294968070;
const CLASS_MEMBEREXPR: u64 = 4294968071;
const CLASS_UNARYEXPRORTYPETRAITEXPR: u64 = 4294968097;
const CLASS_OFFSETOFEXPR: u64 = 4294968109;
const CLASS_PREDEFINEDEXPR: u64 = 4294968118;
const CLASS_CHARACTERLITERAL: u64 = 4294968127;
const CLASS_OPAQUEVALUEEXPR: u64 = 4294968134;
const CLASS_EXPRWITHCLEANUPS: u64 = 4294968142;
const CLASS_ATTRIBUTEDSTMT: u64 = 4294968149;
const CLASS_DECLSTMT: u64 = 4294968156;
const CLASS_CXXRECORDDECL: u64 = 4294968166;
const CLASS_USINGDECL: u64 = 4294968322;
const CLASS_OBJCINDIRECTCOPYRESTOREEXPR: u64 = 4294968331;
const CLASS_HLSLBUFFERDECL: u64 = 4294968332;
const CLASS_DEFAULTSTMT: u64 = 4294968338;
const CLASS_OBJCCOMPATIBLEALIASDECL: u64 = 4294968344;
const CLASS_EXPRESSIONTRAITEXPR: u64 = 4294968345;
const CLASS_MSPROPERTYDECL: u64 = 4294968352;
const CLASS_BUILTINBITCASTEXPR: u64 = 4294968357;
const CLASS_CONSTRUCTORUSINGSHADOWDECL: u64 = 4294968360;
const CLASS_TYPEDECL: u64 = 4294968368;
const CLASS_USINGENUMDECL: u64 = 4294968372;
const CLASS_OMPERRORDIRECTIVE: u64 = 4294968382;
const CLASS_BASEUSINGDECL: u64 = 4294968383;
const CLASS_RECORDDECL: u64 = 4294968388;
const CLASS_NONTYPETEMPLATEPARMDECL: u64 = 4294968416;
const CLASS_OBJCIVARDECL: u64 = 4294968429;
const CLASS_CLASSTEMPLATEDECL: u64 = 4294968430;
const CLASS_OBJCTYPEPARAMDECL: u64 = 4294968440;
const CLASS_TEMPLATETEMPLATEPARMDECL: u64 = 4294968441;
const CLASS_OBJCCONTAINERDECL: u64 = 4294968452;
const CLASS_OBJCPROPERTYDECL: u64 = 4294968453;
const CLASS_UNRESOLVEDUSINGIFEXISTSDECL: u64 = 4294968454;
const CLASS_OBJCPROTOCOLDECL: u64 = 4294968455;
const CLASS_OMPORDEREDDIRECTIVE: u64 = 4294968456;
const CLASS_CXXDELETEEXPR: u64 = 4294968457;
const CLASS_STATICASSERTDECL: u64 = 4294968468;
const CLASS_OMPALLOCATEDECL: u64 = 4294968474;
const CLASS_FILESCOPEASMDECL: u64 = 4294968475;
const CLASS_VARDECL: u64 = 4294968480;
const CLASS_UNRESOLVEDUSINGTYPENAMEDECL: u64 = 4294968539;
const CLASS_OMPTARGETPARALLELGENERICLOOPDIRECTIVE: u64 = 4294968548;
const CLASS_OMPTHREADPRIVATEDECL: u64 = 4294968549;
const CLASS_BINDINGDECL: u64 = 4294968550;
const CLASS_TEMPLATEDECL: u64 = 4294968554;
const CLASS_FRIENDTEMPLATEDECL: u64 = 4294968560;
const CLASS_CXXOPERATORCALLEXPR: u64 = 4294968565;
const CLASS_OBJCPROPERTYIMPLDECL: u64 = 4294968575;
const CLASS_EMPTYDECL: u64 = 4294968576;
const CLASS_CONTINUESTMT: u64 = 4294968577;
const CLASS_IMPORTDECL: u64 = 4294968582;
const CLASS_USINGDIRECTIVEDECL: u64 = 4294968586;
const CLASS_IMPLICITCONCEPTSPECIALIZATIONDECL: u64 = 4294968596;
const CLASS_VARTEMPLATESPECIALIZATIONDECL: u64 = 4294968598;
const CLASS_REQUIRESEXPRBODYDECL: u64 = 4294968614;
const CLASS_FRIENDDECL: u64 = 4294968615;
const CLASS_TEMPLATETYPEPARMDECL: u64 = 4294968622;
const CLASS_ENUMDECL: u64 = 4294968639;
const CLASS_ACCESSSPECDECL: u64 = 4294968665;
const CLASS_OBJCIMPLDECL: u64 = 4294968669;
const CLASS_MSGUIDDECL: u64 = 4294968670;
const CLASS_OMPCAPTUREDEXPRDECL: u64 = 4294968673;
const CLASS_USINGPACKDECL: u64 = 4294968674;
const CLASS_TYPEALIASTEMPLATEDECL: u64 = 4294968679;
const CLASS_VALUEDECL: u64 = 4294968684;
const CLASS_TOPLEVELSTMTDECL: u64 = 4294968689;
const CLASS_CXXNAMEDCASTEXPR: u64 = 4294968693;
const CLASS_TYPEDEFNAMEDECL: u64 = 4294968700;
const CLASS_CAPTUREDDECL: u64 = 4294968706;
const CLASS_TYPEDEFDECL: u64 = 4294968715;
const CLASS_OMPDECLAREREDUCTIONDECL: u64 = 4294968717;
const CLASS_BUILTINTEMPLATEDECL: u64 = 4294968718;
const CLASS_OMPDECLAREMAPPERDECL: u64 = 4294968721;
const CLASS_UNRESOLVEDUSINGVALUEDECL: u64 = 4294968722;
const CLASS_VARTEMPLATEDECL: u64 = 4294968732;
const CLASS_INDIRECTFIELDDECL: u64 = 4294968742;
const CLASS_FIELDDECL: u64 = 4294968750;
const CLASS_OBJCATDEFSFIELDDECL: u64 = 4294968767;
const CLASS_CXXCONVERSIONDECL: u64 = 4294968768;
const CLASS_CXXCONSTRUCTORDECL: u64 = 4294968774;
const CLASS_OMPCRITICALDIRECTIVE: u64 = 4294968793;
const CLASS_CXXTEMPORARYOBJECTEXPR: u64 = 4294968794;
const CLASS_CXXDEDUCTIONGUIDEDECL: u64 = 4294968798;
const CLASS_UNINITIALIZEDATTR: u64 = 4294968804;
const CLASS_PRAGMACLANGRELROSECTIONATTR: u64 = 4294968805;
const CLASS_CXXNULLPTRLITERALEXPR: u64 = 4294968806;
const CLASS_OBJCPRECISELIFETIMEATTR: u64 = 4294968811;
const CLASS_REFERENCETYPELOC: u64 = 4294968812;
const CLASS_OMPTASKLOOPDIRECTIVE: u64 = 4294968814;
const CLASS_OBJCMETHODDECL: u64 = 4294968815;
const CLASS_OSCONSUMESTHISATTR: u64 = 4294968816;
const CLASS_OMPPARALLELFORDIRECTIVE: u64 = 4294968817;
const CLASS_VECTORCALLATTR: u64 = 4294968818;
const CLASS_ANYX86INTERRUPTATTR: u64 = 4294968819;
const CLASS_NOINSTRUMENTFUNCTIONATTR: u64 = 4294968820;
const CLASS_CXXFOLDEXPR: u64 = 4294968821;
const CLASS_BUILTINALIASATTR: u64 = 4294968837;
const CLASS_CONSUMABLEATTR: u64 = 4294968838;
const CLASS_OMPTEAMSDIRECTIVE: u64 = 4294968839;
const CLASS_CLASSTEMPLATESPECIALIZATIONDECL: u64 = 4294968840;
const CLASS_ACQUIREHANDLEATTR: u64 = 4294968855;
const CLASS_OPENCLACCESSATTR: u64 = 4294968856;
const CLASS_READONLYPLACEMENTATTR: u64 = 4294968857;
const CLASS_SUBSTNONTYPETEMPLATEPARMEXPR: u64 = 4294968858;
const CLASS_COUNTEDBYATTR: u64 = 4294968869;
const CLASS_MSABIATTR: u64 = 4294968870;
const CLASS_TYPETAGFORDATATYPEATTR: u64 = 4294968871;
const CLASS_ALIASATTR: u64 = 4294968872;
const CLASS_ENFORCETCBATTR: u64 = 4294968873;
const CLASS_SWIFTVERSIONEDREMOVALATTR: u64 = 4294968874;
const CLASS_ALLOCSIZEATTR: u64 = 4294968875;
const CLASS_ERRORATTR: u64 = 4294968876;
const CLASS_ASSERTCAPABILITYATTR: u64 = 4294968877;
const CLASS_PATCHABLEFUNCTIONENTRYATTR: u64 = 4294968878;
const CLASS_ALIGNVALUEATTR: u64 = 4294968879;
const CLASS_SWIFTBRIDGEATTR: u64 = 4294968880;
const CLASS_MIPS16ATTR: u64 = 4294968881;
const CLASS_CONSTANTARRAYTYPELOC: u64 = 4294968882;
const CLASS_IFUNCATTR: u64 = 4294968883;
const CLASS_OMPCAPTURENOINITATTR: u64 = 4294968884;
const CLASS_ARMLOCALLYSTREAMINGATTR: u64 = 4294968885;
const CLASS_VECRETURNATTR: u64 = 4294968886;
const CLASS_SWIFTIMPORTPROPERTYASACCESSORSATTR: u64 = 4294968887;
const CLASS_EMPTYBASESATTR: u64 = 4294968888;
const CLASS_SWITCHSTMT: u64 = 4294968889;
const CLASS_CXXBINDTEMPORARYEXPR: u64 = 4294968905;
const CLASS_ACQUIRECAPABILITYATTR: u64 = 4294968911;
const CLASS_NAKEDATTR: u64 = 4294968912;
const CLASS_DEDUCEDTYPE: u64 = 4294968913;
const CLASS_ATOMICTYPELOC: u64 = 4294968918;
const CLASS_DECLTYPETYPELOC: u64 = 4294968926;
const CLASS_OMPTARGETPARALLELDIRECTIVE: u64 = 4294968931;
const CLASS_DEPENDENTNAMETYPE: u64 = 4294968932;
const CLASS_NSRETURNSAUTORELEASEDATTR: u64 = 4294968937;
const CLASS_SUPPRESSATTR: u64 = 4294968938;
const CLASS_VECTORTYPELOC: u64 = 4294968939;
const CLASS_OBJCPROTOCOLEXPR: u64 = 4294968944;
const CLASS_C11NORETURNATTR: u64 = 4294968945;
const CLASS_SEHTRYSTMT: u64 = 4294968946;
const CLASS_CALLEDONCEATTR: u64 = 4294968956;
const CLASS_OMPTEAMSDISTRIBUTEPARALLELFORDIRECTIVE: u64 = 4294968957;
const CLASS_FUNCTIONDECL: u64 = 4294968958;
const CLASS_XRAYLOGARGSATTR: u64 = 4294969068;
const CLASS_IMPLICITPARAMDECL: u64 = 4294969069;
const CLASS_OBJCBOXABLEATTR: u64 = 4294969071;
const CLASS_WEBASSEMBLYIMPORTMODULEATTR: u64 = 4294969072;
const CLASS_UNRESOLVEDMEMBEREXPR: u64 = 4294969073;
const CLASS_OMPDEPOBJDIRECTIVE: u64 = 4294969088;
const CLASS_OBJCDESIGNATEDINITIALIZERATTR: u64 = 4294969089;
const CLASS_STRINGLITERAL: u64 = 4294969090;
const CLASS_STRICTGUARDSTACKCHECKATTR: u64 = 4294969112;
const CLASS_OMPSECTIONSDIRECTIVE: u64 = 4294969113;
const CLASS_HLSLSV_DISPATCHTHREADIDATTR: u64 = 4294969114;
const CLASS_FASTCALLATTR: u64 = 4294969115;
const CLASS_ARTIFICIALATTR: u64 = 4294969116;
const CLASS_RELEASEHANDLEATTR: u64 = 4294969117;
const CLASS_REFERENCETYPE: u64 = 4294969118;
const CLASS_VECTYPEHINTATTR: u64 = 4294969123;
const CLASS_SWIFTASYNCCONTEXTATTR: u64 = 4294969124;
const CLASS_MINVECTORWIDTHATTR: u64 = 4294969125;
const CLASS_BTFDECLTAGATTR: u64 = 4294969126;
const CLASS_CLEANUPATTR: u64 = 4294969127;
const CLASS_OMPREFERENCEDVARATTR: u64 = 4294969128;
const CLASS_OMPTASKWAITDIRECTIVE: u64 = 4294969129;
const CLASS_WARNUNUSEDRESULTATTR: u64 = 4294969130;
const CLASS_INITSEGATTR: u64 = 4294969131;
const CLASS_TRANSPARENTUNIONATTR: u64 = 4294969132;
const CLASS_SWIFTERRORATTR: u64 = 4294969133;
const CLASS_PARMVARDECL: u64 = 4294969134;
const CLASS_FULLEXPR: u64 = 4294969152;
const CLASS_DESIGNATEDINITUPDATEEXPR: u64 = 4294969154;
const CLASS_SWIFTIMPORTASNONGENERICATTR: u64 = 4294969160;
const CLASS_ARMNEWATTR: u64 = 4294969161;
const CLASS_PRAGMADETECTMISMATCHDECL: u64 = 4294969162;
const CLASS_SWIFTASYNCCALLATTR: u64 = 4294969165;
const CLASS_DOSTMT: u64 = 4294969166;
const CLASS_M68KINTERRUPTATTR: u64 = 4294969175;
const CLASS_ACQUIREDAFTERATTR: u64 = 4294969176;
const CLASS_SWIFTASYNCATTR: u64 = 4294969177;
const CLASS_SECTIONATTR: u64 = 4294969178;
const CLASS_OSRETURNSNOTRETAINEDATTR: u64 = 4294969179;
const CLASS_ANYX86NOCALLERSAVEDREGISTERSATTR: u64 = 4294969180;
const CLASS_CXXUUIDOFEXPR: u64 = 4294969181;
const CLASS_CDECLATTR: u64 = 4294969190;
const CLASS_ABITAGATTR: u64 = 4294969191;
const CLASS_STANDALONEDEBUGATTR: u64 = 4294969192;
const CLASS_CONVERGENTATTR: u64 = 4294969193;
const CLASS_ASMLABELATTR: u64 = 4294969194;
const CLASS_ANALYZERNORETURNATTR: u64 = 4294969195;
const CLASS_SELECTANYATTR: u64 = 4294969196;
const CLASS_COLDATTR: u64 = 4294969197;
const CLASS_BLOCKPOINTERTYPE: u64 = 4294969198;
const CLASS_OBJCNONLAZYCLASSATTR: u64 = 4294969202;
const CLASS_ALIGNNATURALATTR: u64 = 4294969203;
const CLASS_OMPLOOPBASEDDIRECTIVE: u64 = 4294969204;
const CLASS_AMDGPUNUMSGPRATTR: u64 = 4294969205;
const CLASS_SCOPEDLOCKABLEATTR: u64 = 4294969206;
const CLASS_DEPENDENTSIZEDMATRIXTYPELOC: u64 = 4294969207;
const CLASS_AMDGPUFLATWORKGROUPSIZEATTR: u64 = 4294969208;
const CLASS_OPENCLLOCALADDRESSSPACEATTR: u64 = 4294969209;
const CLASS_DECAYEDTYPELOC: u64 = 4294969210;
const CLASS_OBJCINERTUNSAFEUNRETAINEDATTR: u64 = 4294969211;
const CLASS_CXXCATCHSTMT: u64 = 4294969212;
const CLASS_OMPSCOPEDIRECTIVE: u64 = 4294969220;
const CLASS_AARCH64SVEPCSATTR: u64 = 4294969221;
const CLASS_CUDADEVICEBUILTINTEXTURETYPEATTR: u64 = 4294969222;
const CLASS_INJECTEDCLASSNAMETYPE: u64 = 4294969223;
const CLASS_OBJCSTRINGLITERAL: u64 = 4294969230;
const CLASS_INHERITABLEPARAMATTR: u64 = 4294969231;
const CLASS_WARNUNUSEDATTR: u64 = 4294969232;
const CLASS_NONNULLATTR: u64 = 4294969233;
const CLASS_MUSTTAILATTR: u64 = 4294969234;
const CLASS_OMPCANONICALLOOP: u64 = 4294969235;
const CLASS_WEBASSEMBLYIMPORTNAMEATTR: u64 = 4294969236;
const CLASS_OMPSIMDDIRECTIVE: u64 = 4294969237;
const CLASS_CFCONSUMEDATTR: u64 = 4294969238;
const CLASS_STMTATTR: u64 = 4294969239;
const CLASS_SWIFTCONTEXTATTR: u64 = 4294969240;
const CLASS_ARMMVESTRICTPOLYMORPHISMATTR: u64 = 4294969241;
const CLASS_SWIFTERRORRESULTATTR: u64 = 4294969242;
const CLASS_OMPTARGETSIMDDIRECTIVE: u64 = 4294969243;
const CLASS_THREADATTR: u64 = 4294969244;
const CLASS_OBJCGCATTR: u64 = 4294969245;
const CLASS_OPENCLUNROLLHINTATTR: u64 = 4294969246;
const CLASS_DEPENDENTADDRESSSPACETYPELOC: u64 = 4294969247;
const CLASS_BLOCKPOINTERTYPELOC: u64 = 4294969254;
const CLASS_TYPENONNULLATTR: u64 = 4294969256;
const CLASS_SYCLSPECIALCLASSATTR: u64 = 4294969257;
const CLASS_ARGUMENTWITHTYPETAGATTR: u64 = 4294969258;
const CLASS_NODEREFATTR: u64 = 4294969259;
const CLASS_OVERLOADEXPR: u64 = 4294969260;
const CLASS_EXCLUDEFROMEXPLICITINSTANTIATIONATTR: u64 = 4294969279;
const CLASS_ATTR: u64 = 4294969280;
const CLASS_INCOMPLETEARRAYTYPELOC: u64 = 4294969281;
const CLASS_OMPTILEDIRECTIVE: u64 = 4294969282;
const CLASS_INHERITABLEATTR: u64 = 4294969283;
const CLASS_PACKEXPANSIONTYPELOC: u64 = 4294969284;
const CLASS_AMDGPUKERNELCALLATTR: u64 = 4294969289;
const CLASS_VARTEMPLATEPARTIALSPECIALIZATIONDECL: u64 = 4294969290;
const CLASS_ALLOCALIGNATTR: u64 = 4294969296;
const CLASS_ARMSTREAMINGCOMPATIBLEATTR: u64 = 4294969297;
const CLASS_ASSUMPTIONATTR: u64 = 4294969298;
const CLASS_HLSLSV_GROUPINDEXATTR: u64 = 4294969299;
const CLASS_IMAGINARYLITERAL: u64 = 4294969300;
const CLASS_OBJCNSOBJECTATTR: u64 = 4294969305;
const CLASS_CMSENSCALLATTR: u64 = 4294969306;
const CLASS_BTFTYPETAGATTR: u64 = 4294969307;
const CLASS_TYPENULLABLEATTR: u64 = 4294969308;
const CLASS_OMPUNROLLDIRECTIVE: u64 = 4294969309;
const CLASS_TRYACQUIRECAPABILITYATTR: u64 = 4294969310;
const CLASS_HLSLANNOTATIONATTR: u64 = 4294969311;
const CLASS_ARMINATTR: u64 = 4294969312;
const CLASS_CXXSCALARVALUEINITEXPR: u64 = 4294969313;
const CLASS_CSTYLECASTEXPR: u64 = 4294969319;
const CLASS_LIKELYATTR: u64 = 4294969324;
const CLASS_UNUSEDATTR: u64 = 4294969325;
const CLASS_ARMBUILTINALIASATTR: u64 = 4294969326;
const CLASS_NOMERGEATTR: u64 = 4294969327;
const CLASS_OBJCFORCOLLECTIONSTMT: u64 = 4294969328;
const CLASS_TAGTYPELOC: u64 = 4294969329;
const CLASS_OMPCAPTUREKINDATTR: u64 = 4294969332;
const CLASS_OPENCLGLOBALADDRESSSPACEATTR: u64 = 4294969333;
const CLASS_TRANSLATIONUNITDECL: u64 = 4294969334;
const CLASS_CODEALIGNATTR: u64 = 4294969337;
const CLASS_CALLABLEWHENATTR: u64 = 4294969338;
const CLASS_OPENCLGLOBALDEVICEADDRESSSPACEATTR: u64 = 4294969339;
const CLASS_ARRAYTYPELOC: u64 = 4294969340;
const CLASS_CUDASHAREDATTR: u64 = 4294969348;
const CLASS_EXPORTDECL: u64 = 4294969349;
const CLASS_PARAMTYPESTATEATTR: u64 = 4294969355;
const CLASS_OMPLOOPDIRECTIVE: u64 = 4294969356;
const CLASS_IFSTMT: u64 = 4294969357;
const CLASS_BPFPRESERVESTATICOFFSETATTR: u64 = 4294969380;
const CLASS_ARMSTREAMINGATTR: u64 = 4294969381;
const CLASS_OPENCLGLOBALHOSTADDRESSSPACEATTR: u64 = 4294969382;
const CLASS_RECOVERYEXPR: u64 = 4294969383;
const CLASS_OPENCLKERNELATTR: u64 = 4294969387;
const CLASS_LOOPHINTATTR: u64 = 4294969388;
const CLASS_EXTERNCCONTEXTDECL: u64 = 4294969389;
const CLASS_ARMPRESERVESATTR: u64 = 4294969390;
const CLASS_OBJCKINDOFATTR: u64 = 4294969391;
const CLASS_FUNCTIONTEMPLATEDECL: u64 = 4294969392;
const CLASS_STRICTFPATTR: u64 = 4294969403;
const CLASS_CFGUARDATTR: u64 = 4294969404;
const CLASS_TARGETATTR: u64 = 4294969405;
const CLASS_NVPTXKERNELATTR: u64 = 4294969406;
const CLASS_CUDAGLOBALATTR: u64 = 4294969407;
const CLASS_WEBASSEMBLYEXPORTNAMEATTR: u64 = 4294969408;
const CLASS_BUILTINTYPELOC: u64 = 4294969409;
const CLASS_PTR64ATTR: u64 = 4294969424;
const CLASS_FUNCTIONNOPROTOTYPE: u64 = 4294969425;
const CLASS_AMDGPUWAVESPEREUATTR: u64 = 4294969428;
const CLASS_OPENCLCONSTANTADDRESSSPACEATTR: u64 = 4294969429;
const CLASS_OMPPARALLELMASKEDTASKLOOPDIRECTIVE: u64 = 4294969430;
const CLASS_TYPEATTR: u64 = 4294969431;
const CLASS_PRAGMACLANGTEXTSECTIONATTR: u64 = 4294969432;
const CLASS_PARENTYPE: u64 = 4294969433;
const CLASS_ASSERTSHAREDLOCKATTR: u64 = 4294969437;
const CLASS_NOESCAPEATTR: u64 = 4294969438;
const CLASS_BLOCKDECL: u64 = 4294969439;
const CLASS_BUILTINATTR: u64 = 4294969464;
const CLASS_NOCOMMONATTR: u64 = 4294969465;
const CLASS_SWITCHCASE: u64 = 4294969466;
const CLASS_LIFETIMEEXTENDEDTEMPORARYDECL: u64 = 4294969473;
const CLASS_PRAGMACLANGDATASECTIONATTR: u64 = 4294969480;
const CLASS_VISIBILITYATTR: u64 = 4294969481;
const CLASS_CFICANONICALJUMPTABLEATTR: u64 = 4294969482;
const CLASS_CFRETURNSRETAINEDATTR: u64 = 4294969483;
const CLASS_USEHANDLEATTR: u64 = 4294969484;
const CLASS_ARMOUTATTR: u64 = 4294969485;
const CLASS_OMPTARGETTEAMSDISTRIBUTEDIRECTIVE: u64 = 4294969486;
const CLASS_COROLIFETIMEBOUNDATTR: u64 = 4294969487;
const CLASS_SHAREDTRYLOCKFUNCTIONATTR: u64 = 4294969488;
const CLASS_OMPDECLARESIMDDECLATTR: u64 = 4294969489;
const CLASS_DECLORSTMTATTR: u64 = 4294969490;
const CLASS_OBJCROOTCLASSATTR: u64 = 4294969491;
const CLASS_TARGETCLONESATTR: u64 = 4294969492;
const CLASS_RENDERSCRIPTKERNELATTR: u64 = 4294969493;
const CLASS_ASSUMEALIGNEDATTR: u64 = 4294969494;
const CLASS_AVAILABLEONLYINDEFAULTEVALMETHODATTR: u64 = 4294969495;
const CLASS_TEMPLATESPECIALIZATIONTYPELOC: u64 = 4294969496;
const CLASS_SEHFINALLYSTMT: u64 = 4294969505;
const CLASS_PASSOBJECTSIZEATTR: u64 = 4294969511;
const CLASS_SOURCELOCEXPR: u64 = 4294969512;
const CLASS_X86FORCEALIGNARGPOINTERATTR: u64 = 4294969521;
const CLASS_UNLIKELYATTR: u64 = 4294969522;
const CLASS_CAPTUREDRECORDATTR: u64 = 4294969523;
const CLASS_CONSTANTARRAYTYPE: u64 = 4294969524;
const CLASS_CASESTMT: u64 = 4294969529;
const CLASS_COMPLEXTYPE: u64 = 4294969539;
const CLASS_WEAKATTR: u64 = 4294969543;
const CLASS_CONSTINITATTR: u64 = 4294969544;
const CLASS_CONSTRUCTORATTR: u64 = 4294969545;
const CLASS_UPTRATTR: u64 = 4294969546;
const CLASS_DECLTYPETYPE: u64 = 4294969547;
const CLASS_SWIFTVERSIONEDADDITIONATTR: u64 = 4294969552;
const CLASS_DEPENDENTVECTORTYPE: u64 = 4294969553;
const CLASS_CXXDESTRUCTORDECL: u64 = 4294969560;
const CLASS_ALIGNEDATTR: u64 = 4294969564;
const CLASS_CONSUMABLEAUTOCASTATTR: u64 = 4294969565;
const CLASS_ENUMCONSTANTDECL: u64 = 4294969566;
const CLASS_WEBASSEMBLYFUNCREFATTR: u64 = 4294969571;
const CLASS_CONSUMABLESETONREADATTR: u64 = 4294969572;
const CLASS_ARRAYINITLOOPEXPR: u64 = 4294969573;
const CLASS_REQUIRESCAPABILITYATTR: u64 = 4294969580;
const CLASS_CORODISABLELIFETIMEBOUNDATTR: u64 = 4294969581;
const CLASS_DEPENDENTCOAWAITEXPR: u64 = 4294969582;
const CLASS_CODESEGATTR: u64 = 4294969589;
const CLASS_OBJCTYPEPARAMTYPE: u64 = 4294969590;
const CLASS_OMPREQUIRESDECL: u64 = 4294969591;
const CLASS_USEDATTR: u64 = 4294969592;
const CLASS_REGCALLATTR: u64 = 4294969593;
const CLASS_EXPR: u64 = 4294969594;
const CLASS_NULLSTMT: u64 = 4294969636;
const CLASS_OBJCCLASSSTUBATTR: u64 = 4294969642;
const CLASS_CXX11NORETURNATTR: u64 = 4294969643;
const CLASS_MSPROPERTYREFEXPR: u64 = 4294969644;
const CLASS_DIAGNOSEASBUILTINATTR: u64 = 4294969655;
const CLASS_COROONLYDESTROYWHENCOMPLETEATTR: u64 = 4294969656;
const CLASS_PRAGMACLANGRODATASECTIONATTR: u64 = 4294969657;
const CLASS_CUDALAUNCHBOUNDSATTR: u64 = 4294969658;
const CLASS_RETAINATTR: u64 = 4294969659;
const CLASS_CALLBACKATTR: u64 = 4294969660;
const CLASS_UUIDATTR: u64 = 4294969661;
const CLASS_TYPEDEFTYPELOC: u64 = 4294969662;
const CLASS_PREFERREDTYPEATTR: u64 = 4294969664;
const CLASS_PTGUARDEDVARATTR: u64 = 4294969665;
const CLASS_ASSERTEXCLUSIVELOCKATTR: u64 = 4294969666;
const CLASS_COMMONATTR: u64 = 4294969667;
const CLASS_REQDWORKGROUPSIZEATTR: u64 = 4294969668;
const CLASS_OBJCRUNTIMEVISIBLEATTR: u64 = 4294969669;
const CLASS_PASCALATTR: u64 = 4294969670;
const CLASS_GCCASMSTMT: u64 = 4294969671;
const CLASS_PARAMETERABIATTR: u64 = 4294969681;
const CLASS_CXXMEMBERCALLEXPR: u64 = 4294969682;
const CLASS_RETURNSTWICEATTR: u64 = 4294969688;
const CLASS_DEPENDENTTYPEOFEXPRTYPE: u64 = 4294969689;
const CLASS_SEHEXCEPTSTMT: u64 = 4294969690;
const CLASS_AARCH64VECTORPCSATTR: u64 = 4294969697;
const CLASS_INITPRIORITYATTR: u64 = 4294969698;
const CLASS_RESTRICTATTR: u64 = 4294969699;
const CLASS_HLSLGROUPSHAREDADDRESSSPACEATTR: u64 = 4294969700;
const CLASS_BPFPRESERVEACCESSINDEXATTR: u64 = 4294969701;
const CLASS_FUNCTIONTYPE: u64 = 4294969702;
const CLASS_ATOMICTYPE: u64 = 4294969713;
const CLASS_SWIFTASYNCERRORATTR: u64 = 4294969717;
const CLASS_FLOATINGLITERAL: u64 = 4294969718;
const CLASS_PACKEDATTR: u64 = 4294969728;
const CLASS_GUARDEDBYATTR: u64 = 4294969729;
const CLASS_RETURNSNONNULLATTR: u64 = 4294969730;
const CLASS_AVRINTERRUPTATTR: u64 = 4294969731;
const CLASS_MATRIXSUBSCRIPTEXPR: u64 = 4294969732;
const CLASS_SENTINELATTR: u64 = 4294969742;
const CLASS_UNSAFEBUFFERUSAGEATTR: u64 = 4294969743;
const CLASS_FUNCTIONPROTOTYPE: u64 = 4294969744;
const CLASS_NOTHROWATTR: u64 = 4294969778;
const CLASS_UNAVAILABLEATTR: u64 = 4294969779;
const CLASS_BUILTINTYPE: u64 = 4294969780;
const CLASS_OBJCINDEPENDENTCLASSATTR: u64 = 4294969792;
const CLASS_NOINLINEATTR: u64 = 4294969793;
const CLASS_ALWAYSINLINEATTR: u64 = 4294969794;
const CLASS_CUDADEVICEATTR: u64 = 4294969795;
const CLASS_SPECULATIVELOADHARDENINGATTR: u64 = 4294969796;
const CLASS_CFRETURNSNOTRETAINEDATTR: u64 = 4294969797;
const CLASS_OMPTARGETPARALLELFORSIMDDIRECTIVE: u64 = 4294969798;
const CLASS_THISCALLATTR: u64 = 4294969799;
const CLASS_TEMPLATEPARAMOBJECTDECL: u64 = 4294969800;
const CLASS_TYPEVISIBILITYATTR: u64 = 4294969803;
const CLASS_TYPENULLABLERESULTATTR: u64 = 4294969804;
const CLASS_CUDADEVICEBUILTINSURFACETYPEATTR: u64 = 4294969805;
const CLASS_MIPSINTERRUPTATTR: u64 = 4294969806;
const CLASS_OMPTEAMSDISTRIBUTEDIRECTIVE: u64 = 4294969807;
const CLASS_OWNERSHIPATTR: u64 = 4294969808;
const CLASS_NSRETURNSNOTRETAINEDATTR: u64 = 4294969809;
const CLASS_RELEASECAPABILITYATTR: u64 = 4294969810;
const CLASS_OPENCLINTELREQDSUBGROUPSIZEATTR: u64 = 4294969811;
const CLASS_PUREATTR: u64 = 4294969812;
const CLASS_CXXBOOLLITERALEXPR: u64 = 4294969813;
const CLASS_OPTIMIZENONEATTR: u64 = 4294969819;
const CLASS_LTOVISIBILITYPUBLICATTR: u64 = 4294969820;
const CLASS_OBJCSUBCLASSINGRESTRICTEDATTR: u64 = 4294969821;
const CLASS_NORETURNATTR: u64 = 4294969822;
const CLASS_ARRAYTYPE: u64 = 4294969823;
const CLASS_NAMESPACEALIASDECL: u64 = 4294969828;
const CLASS_LINKAGESPECDECL: u64 = 4294969838;
const CLASS_OBJCNONRUNTIMEPROTOCOLATTR: u64 = 4294969845;
const CLASS_FINALATTR: u64 = 4294969846;
const CLASS_OBJCRETURNSINNERPOINTERATTR: u64 = 4294969847;
const CLASS_CPUDISPATCHATTR: u64 = 4294969848;
const CLASS_PTGUARDEDBYATTR: u64 = 4294969849;
const CLASS_CFUNKNOWNTRANSFERATTR: u64 = 4294969850;
const CLASS_PRESERVEMOSTATTR: u64 = 4294969851;
const CLASS_EXTVECTORTYPELOC: u64 = 4294969852;
const CLASS_OBJCREQUIRESPROPERTYDEFSATTR: u64 = 4294969853;
const CLASS_INCOMPLETEARRAYTYPE: u64 = 4294969854;
const CLASS_AMDGPUNUMVGPRATTR: u64 = 4294969857;
const CLASS_PTR32ATTR: u64 = 4294969858;
const CLASS_HLSLRESOURCEATTR: u64 = 4294969859;
const CLASS_OBJCOWNERSHIPATTR: u64 = 4294969860;
const CLASS_CXXFUNCTIONALCASTEXPR: u64 = 4294969861;
const CLASS_OPENCLPRIVATEADDRESSSPACEATTR: u64 = 4294969867;
const CLASS_EXCLUSIVETRYLOCKFUNCTIONATTR: u64 = 4294969868;
const CLASS_NOBUILTINATTR: u64 = 4294969869;
const CLASS_FIXEDPOINTLITERAL: u64 = 4294969870;
const CLASS_SWIFTPRIVATEATTR: u64 = 4294969876;
const CLASS_OBJCBRIDGERELATEDATTR: u64 = 4294969877;
const CLASS_OBJCEXTERNALLYRETAINEDATTR: u64 = 4294969878;
const CLASS_LAMBDAEXPR: u64 = 4294969879;
const CLASS_MIPSLONGCALLATTR: u64 = 4294969911;
const CLASS_OBJCBRIDGEMUTABLEATTR: u64 = 4294969912;
const CLASS_DEPENDENTBITINTTYPELOC: u64 = 4294969913;
const CLASS_ARMINOUTATTR: u64 = 4294969914;
const CLASS_SUBSTNONTYPETEMPLATEPARMPACKEXPR: u64 = 4294969915;
const CLASS_CXXPSEUDODESTRUCTOREXPR: u64 = 4294969924;
const CLASS_OSRETURNSRETAINEDONZEROATTR: u64 = 4294969941;
const CLASS_OSRETURNSRETAINEDONNONZEROATTR: u64 = 4294969942;
const CLASS_SPTRATTR: u64 = 4294969943;
const CLASS_OBJCBRIDGEDCASTEXPR: u64 = 4294969944;
const CLASS_PRESERVEALLATTR: u64 = 4294969945;
const CLASS_OBJCOBJECTTYPELOC: u64 = 4294969946;
const CLASS_OMPDECLAREVARIANTATTR: u64 = 4294969947;
const CLASS_OBJCEXPLICITPROTOCOLIMPLATTR: u64 = 4294969948;
const CLASS_OMPALLOCATEDECLATTR: u64 = 4294969949;
const CLASS_SWIFTINDIRECTRESULTATTR: u64 = 4294969950;
const CLASS_COMPLEXTYPELOC: u64 = 4294969951;
const CLASS_MACROQUALIFIEDTYPE: u64 = 4294969952;
const CLASS_MICROMIPSATTR: u64 = 4294969958;
const CLASS_USINGTYPE: u64 = 4294969959;
const CLASS_HIPMANAGEDATTR: u64 = 4294969965;
const CLASS_MACROQUALIFIEDTYPELOC: u64 = 4294969966;
const CLASS_ZEROCALLUSEDREGSATTR: u64 = 4294969972;
const CLASS_OBJCEXCEPTIONATTR: u64 = 4294969973;
const CLASS_MAYBEUNDEFATTR: u64 = 4294969974;
const CLASS_OMPCANCELLATIONPOINTDIRECTIVE: u64 = 4294969975;
const CLASS_TARGETVERSIONATTR: u64 = 4294969976;
const CLASS_NOUWTABLEATTR: u64 = 4294969977;
const CLASS_NOUNIQUEADDRESSATTR: u64 = 4294969978;
const CLASS_VAARGEXPR: u64 = 4294969979;
const CLASS_NOTHREADSAFETYANALYSISATTR: u64 = 4294969988;
const CLASS_NOSPLITSTACKATTR: u64 = 4294969989;
const CLASS_ADDRESSSPACEATTR: u64 = 4294969990;
const CLASS_OBJCAVAILABILITYCHECKEXPR: u64 = 4294969991;
const CLASS_MSNOVTABLEATTR: u64 = 4294969992;
const CLASS_NSCONSUMESSELFATTR: u64 = 4294969993;
const CLASS_FUNCTIONRETURNTHUNKSATTR: u64 = 4294969994;
const CLASS_PRAGMACOMMENTDECL: u64 = 4294969995;
const CLASS_INTELOCLBICCATTR: u64 = 4294969998;
const CLASS_NOSPECULATIVELOADHARDENINGATTR: u64 = 4294969999;
const CLASS_BLOCKEXPR: u64 = 4294970000;
const CLASS_SWIFTNEWTYPEATTR: u64 = 4294970008;
const CLASS_AVAILABILITYATTR: u64 = 4294970009;
const CLASS_OBJCDIRECTATTR: u64 = 4294970010;
const CLASS_IBOUTLETATTR: u64 = 4294970011;
const CLASS_DLLIMPORTATTR: u64 = 4294970012;
const CLASS_MODEATTR: u64 = 4294970013;
const CLASS_UNNAMEDGLOBALCONSTANTDECL: u64 = 4294970014;
const CLASS_FLATTENATTR: u64 = 4294970016;
const CLASS_NOSANITIZEATTR: u64 = 4294970017;
const CLASS_OBJCREQUIRESSUPERATTR: u64 = 4294970018;
const CLASS_PIPETYPE: u64 = 4294970019;
const CLASS_SYSVABIATTR: u64 = 4294970024;
const CLASS_NOMIPS16ATTR: u64 = 4294970025;
const CLASS_NODESTROYATTR: u64 = 4294970026;
const CLASS_OMPCANCELDIRECTIVE: u64 = 4294970027;
const CLASS_STMT: u64 = 4294970028;
const CLASS_DEPENDENTSIZEDARRAYTYPELOC: u64 = 4294970038;
const CLASS_OBJCATSYNCHRONIZEDSTMT: u64 = 4294970039;
const CLASS_ATOMICEXPR: u64 = 4294970040;
const CLASS_SWIFTOBJCMEMBERSATTR: u64 = 4294970062;
const CLASS_NOALIASATTR: u64 = 4294970063;
const CLASS_ATTRIBUTEDTYPELOC: u64 = 4294970064;
const CLASS_PCSATTR: u64 = 4294970071;
const CLASS_NSRETURNSRETAINEDATTR: u64 = 4294970072;
const CLASS_LOCKRETURNEDATTR: u64 = 4294970073;
const CLASS_HLSLSHADERATTR: u64 = 4294970074;
const CLASS_DLLEXPORTATTR: u64 = 4294970075;
const CLASS_DEPENDENTSCOPEDECLREFEXPR: u64 = 4294970076;
const CLASS_MATRIXTYPELOC: u64 = 4294970093;
const CLASS_MINSIZEATTR: u64 = 4294970099;
const CLASS_INDIRECTGOTOSTMT: u64 = 4294970100;
const CLASS_MAYALIASATTR: u64 = 4294970108;
const CLASS_SEHLEAVESTMT: u64 = 4294970109;
const CLASS_MIPSSHORTCALLATTR: u64 = 4294970114;
const CLASS_NOPROFILEFUNCTIONATTR: u64 = 4294970115;
const CLASS_TLSMODELATTR: u64 = 4294970116;
const CLASS_CMSENSENTRYATTR: u64 = 4294970117;
const CLASS_DESTRUCTORATTR: u64 = 4294970118;
const CLASS_NSCONSUMEDATTR: u64 = 4294970119;
const CLASS_ANNOTATETYPEATTR: u64 = 4294970120;
const CLASS_MSVTORDISPATTR: u64 = 4294970121;
const CLASS_MSSTRUCTATTR: u64 = 4294970122;
const CLASS_DECL: u64 = 4294970123;
const CLASS_HLSLNUMTHREADSATTR: u64 = 4294970205;
const CLASS_TRIVIALABIATTR: u64 = 4294970206;
const CLASS_MAXFIELDALIGNMENTATTR: u64 = 4294970207;
const CLASS_MSP430INTERRUPTATTR: u64 = 4294970208;
const CLASS_RETURNTYPESTATEATTR: u64 = 4294970209;
const CLASS_OBJCBOXEDEXPR: u64 = 4294970210;
const CLASS_CONSTATTR: u64 = 4294970211;
const CLASS_MSINHERITANCEATTR: u64 = 4294970212;
const CLASS_OMPMASTERDIRECTIVE: u64 = 4294970213;
const CLASS_OBJCOBJECTTYPEIMPL: u64 = 4294970214;
const CLASS_TEMPLATESPECIALIZATIONTYPE: u64 = 4294970215;
const CLASS_MSCONSTEXPRATTR: u64 = 4294970223;
const CLASS_RVALUEREFERENCETYPE: u64 = 4294970224;
const CLASS_SWIFTBRIDGEDTYPEDEFATTR: u64 = 4294970227;
const CLASS_OBJCMETHODFAMILYATTR: u64 = 4294970228;
const CLASS_LOCKSEXCLUDEDATTR: u64 = 4294970229;
const CLASS_BITINTTYPELOC: u64 = 4294970230;
const CLASS_OBJCENCODEEXPR: u64 = 4294970231;
const CLASS_HOTATTR: u64 = 4294970232;
const CLASS_ACQUIREDBEFOREATTR: u64 = 4294970233;
const CLASS_OMPTHREADPRIVATEDECLATTR: u64 = 4294970234;
const CLASS_INTEGERLITERAL: u64 = 4294970235;
const CLASS_MSALLOCATORATTR: u64 = 4294970240;
const CLASS_ENUMEXTENSIBILITYATTR: u64 = 4294970241;
const CLASS_CUDACONSTANTATTR: u64 = 4294970242;
const CLASS_NOSTACKPROTECTORATTR: u64 = 4294970243;
const CLASS_DIAGNOSEIFATTR: u64 = 4294970244;
const CLASS_USINGSHADOWDECL: u64 = 4294970245;
const CLASS_ADJUSTEDTYPE: u64 = 4294970250;
const CLASS_DEPENDENTSIZEDEXTVECTORTYPE: u64 = 4294970255;
const CLASS_MIGSERVERROUTINEATTR: u64 = 4294970261;
const CLASS_CXXMETHODDECL: u64 = 4294970262;
const CLASS_RANDOMIZELAYOUTATTR: u64 = 4294970287;
const CLASS_COROWRAPPERATTR: u64 = 4294970288;
const CLASS_CLASSTEMPLATEPARTIALSPECIALIZATIONDECL: u64 = 4294970289;
const CLASS_TYPENULLUNSPECIFIEDATTR: u64 = 4294970296;
const CLASS_WHILESTMT: u64 = 4294970297;
const CLASS_CORORETURNTYPEATTR: u64 = 4294970309;
const CLASS_ADDRLABELEXPR: u64 = 4294970310;
const CLASS_CUDAINVALIDTARGETATTR: u64 = 4294970317;
const CLASS_M68KRTDATTR: u64 = 4294970318;
const CLASS_OMPDECLARETARGETDECLATTR: u64 = 4294970319;
const CLASS_LIFETIMEBOUNDATTR: u64 = 4294970320;
const CLASS_PARENLISTEXPR: u64 = 4294970321;
const CLASS_CARRIESDEPENDENCYATTR: u64 = 4294970328;
const CLASS_LEAFATTR: u64 = 4294970329;
const CLASS_ENUMTYPELOC: u64 = 4294970330;
const CLASS_RISCVINTERRUPTATTR: u64 = 4294970332;
const CLASS_OVERLOADABLEATTR: u64 = 4294970333;
const CLASS_LAYOUTVERSIONATTR: u64 = 4294970334;
const CLASS_INTERNALLINKAGEATTR: u64 = 4294970335;
const CLASS_OBJCDIRECTMEMBERSATTR: u64 = 4294970336;
const CLASS_IBOUTLETCOLLECTIONATTR: u64 = 4294970337;
const CLASS_CXXFORRANGESTMT: u64 = 4294970338;
const CLASS_CONCEPTDECL: u64 = 4294970356;
const CLASS_DISABLETAILCALLSATTR: u64 = 4294970361;
const CLASS_FORMATATTR: u64 = 4294970362;
const CLASS_OBJCINTERFACETYPE: u64 = 4294970363;
const CLASS_REDECLARABLETEMPLATEDECL: u64 = 4294970364;
const CLASS_PARENTYPELOC: u64 = 4294970368;
const CLASS_FALLTHROUGHATTR: u64 = 4294970374;
const CLASS_IBACTIONATTR: u64 = 4294970375;
const CLASS_CXXCONSTCASTEXPR: u64 = 4294970376;
const CLASS_CALLEXPR: u64 = 4294970377;
const CLASS_FLAGENUMATTR: u64 = 4294970397;
const CLASS_CUDAHOSTATTR: u64 = 4294970398;
const CLASS_OMPARRAYSECTIONEXPR: u64 = 4294970399;
const CLASS_SUBSTTEMPLATETYPEPARMTYPE: u64 = 4294970400;
const CLASS_XRAYINSTRUMENTATTR: u64 = 4294970408;
const CLASS_ARRAYSUBSCRIPTEXPR: u64 = 4294970409;
const CLASS_DISABLESANITIZERINSTRUMENTATIONATTR: u64 = 4294970419;
const CLASS_GUARDEDVARATTR: u64 = 4294970420;
const CLASS_OBJCBRIDGEATTR: u64 = 4294970421;
const CLASS_GNUINLINEATTR: u64 = 4294970422;
const CLASS_WEAKIMPORTATTR: u64 = 4294970423;
const CLASS_FORMATARGATTR: u64 = 4294970424;
const CLASS_TYPEOFEXPRTYPE: u64 = 4294970425;
const CLASS_USINGTYPELOC: u64 = 4294970430;
const CLASS_EXTERNALSOURCESYMBOLATTR: u64 = 4294970433;
const CLASS_ENFORCETCBLEAFATTR: u64 = 4294970434;
const CLASS_SWIFTCALLATTR: u64 = 4294970435;
const CLASS_ENABLEIFATTR: u64 = 4294970436;
const CLASS_ANYX86NOCFCHECKATTR: u64 = 4294970437;
const CLASS_SETTYPESTATEATTR: u64 = 4294970438;
const CLASS_OSRETURNSRETAINEDATTR: u64 = 4294970439;
const CLASS_OWNERATTR: u64 = 4294970440;
const CLASS_NOMICROMIPSATTR: u64 = 4294970441;
const CLASS_DEPRECATEDATTR: u64 = 4294970442;
const CLASS_TESTTYPESTATEATTR: u64 = 4294970443;
const CLASS_CFAUDITEDTRANSFERATTR: u64 = 4294970444;
const CLASS_HLSLPARAMMODIFIERATTR: u64 = 4294970445;
const CLASS_OVERRIDEATTR: u64 = 4294970446;
const CLASS_DLLIMPORTSTATICLOCALATTR: u64 = 4294970447;
const CLASS_OMPFLUSHDIRECTIVE: u64 = 4294970448;
const CLASS_PREFERREDNAMEATTR: u64 = 4294970449;
const CLASS_ALWAYSDESTROYATTR: u64 = 4294970450;
const CLASS_BLOCKSATTR: u64 = 4294970451;
const CLASS_NODUPLICATEATTR: u64 = 4294970452;
const CLASS_OPENCLGENERICADDRESSSPACEATTR: u64 = 4294970453;
const CLASS_OSCONSUMEDATTR: u64 = 4294970454;
const CLASS_DLLEXPORTSTATICLOCALATTR: u64 = 4294970455;
const CLASS_OBJCIMPLEMENTATIONDECL: u64 = 4294970456;
const CLASS_WORKGROUPSIZEHINTATTR: u64 = 4294970457;
const CLASS_POINTERTYPE: u64 = 4294970458;
const CLASS_OMPPARALLELDIRECTIVE: u64 = 4294970462;
const CLASS_COMPOUNDASSIGNOPERATOR: u64 = 4294970463;
const CLASS_ENUMTYPE: u64 = 4294970466;
const CLASS_RECORDTYPE: u64 = 4294970470;
const CLASS_OMPPARALLELMASTERTASKLOOPSIMDDIRECTIVE: u64 = 4294970475;
const CLASS_NORANDOMIZELAYOUTATTR: u64 = 4294970476;
const CLASS_ELABORATEDTYPE: u64 = 4294970477;
const CLASS_TEMPLATETYPEPARMTYPE: u64 = 4294970483;
const CLASS_SUBSTTEMPLATETYPEPARMPACKTYPE: u64 = 4294970491;
const CLASS_OMPMASKEDTASKLOOPDIRECTIVE: u64 = 4294970501;
const CLASS_LVALUEREFERENCETYPELOC: u64 = 4294970502;
const CLASS_UNARYTRANSFORMTYPE: u64 = 4294970504;
const CLASS_ATTRIBUTEDTYPE: u64 = 4294970510;
const CLASS_OMPTARGETUPDATEDIRECTIVE: u64 = 4294970521;
const CLASS_DEPENDENTBITINTTYPE: u64 = 4294970522;
const CLASS_MATRIXTYPE: u64 = 4294970528;
const CLASS_OMPPARALLELMASTERDIRECTIVE: u64 = 4294970532;
const CLASS_OBJCOBJECTPOINTERTYPE: u64 = 4294970533;
const CLASS_OBJCOBJECTTYPE: u64 = 4294970534;
const CLASS_OBJCINTERFACEDECL: u64 = 4294970535;
const CLASS_TYPEDEFTYPE: u64 = 4294970536;
const CLASS_ADJUSTEDTYPELOC: u64 = 4294970541;
const CLASS_LOADERUNINITIALIZEDATTR: u64 = 4294970546;
const CLASS_TYPE: u64 = 4294970547;
const CLASS_AVRSIGNALATTR: u64 = 4294970787;
const CLASS_TAGTYPE: u64 = 4294970788;
const CLASS_TYPEOFTYPE: u64 = 4294970791;
const CLASS_BITINTTYPE: u64 = 4294970796;
const CLASS_TYPEWITHKEYWORD: u64 = 4294970802;
const CLASS_REINITIALIZESATTR: u64 = 4294970804;
const CLASS_AUTOTYPE: u64 = 4294970805;
const CLASS_DEPENDENTTEMPLATESPECIALIZATIONTYPE: u64 = 4294970812;
const CLASS_PACKEXPANSIONTYPE: u64 = 4294970818;
const CLASS_STDCALLATTR: u64 = 4294970823;
const CLASS_OBJCRUNTIMENAMEATTR: u64 = 4294970824;
const CLASS_DEDUCEDTEMPLATESPECIALIZATIONTYPE: u64 = 4294970825;
const CLASS_USINGIFEXISTSATTR: u64 = 4294970827;
const CLASS_DEPENDENTUNARYTRANSFORMTYPE: u64 = 4294970828;
const CLASS_UNARYOPERATOR: u64 = 4294970829;
const CLASS_EXTVECTORTYPE: u64 = 4294970847;
const CLASS_DEPENDENTDECLTYPETYPE: u64 = 4294970850;
const CLASS_NOTTAILCALLEDATTR: u64 = 4294970851;
const CLASS_LVALUEREFERENCETYPE: u64 = 4294970852;
const CLASS_UNRESOLVEDUSINGTYPE: u64 = 4294970855;
const CLASS_DEPENDENTSIZEDMATRIXTYPE: u64 = 4294970859;
const CLASS_CONSTANTMATRIXTYPE: u64 = 4294970863;
const CLASS_OBJCCATEGORYIMPLDECL: u64 = 4294970867;
const CLASS_VECTORTYPE: u64 = 4294970868;
const CLASS_DEPENDENTADDRESSSPACETYPE: u64 = 4294970874;
const CLASS_RETURNSTMT: u64 = 4294970880;
const CLASS_DECAYEDTYPE: u64 = 4294970887;
const CLASS_OBJCCATEGORYDECL: u64 = 4294970890;
const CLASS_MEMBERPOINTERTYPE: u64 = 4294970891;
const CLASS_INJECTEDCLASSNAMETYPELOC: u64 = 4294970899;
const CLASS_PIPETYPELOC: u64 = 4294970901;
const CLASS_DEPENDENTVECTORTYPELOC: u64 = 4294970906;
const CLASS_TYPEALIASDECL: u64 = 4294970911;
const CLASS_CONSTANTMATRIXTYPELOC: u64 = 4294970914;
const CLASS_NSERRORDOMAINATTR: u64 = 4294970915;
const CLASS_POINTERTYPELOC: u64 = 4294970916;
const CLASS_ANNOTATEATTR: u64 = 4294970918;
const CLASS_DEDUCEDTYPELOC: u64 = 4294970919;
const CLASS_TYPEOFTYPELOC: u64 = 4294970920;
const CLASS_TAGDECL: u64 = 4294970923;
const CLASS_DEPENDENTNAMETYPELOC: u64 = 4294970951;
const CLASS_CXXREINTERPRETCASTEXPR: u64 = 4294970956;
const CLASS_ARMINTERRUPTATTR: u64 = 4294970957;
const CLASS_DEPENDENTSIZEDEXTVECTORTYPELOC: u64 = 4294970958;
const CLASS_OBJCOBJECTPOINTERTYPELOC: u64 = 4294970963;
const CLASS_FUNCTIONPROTOTYPELOC: u64 = 4294970964;
const CLASS_CAPABILITYATTR: u64 = 4294970965;
const CLASS_UNRESOLVEDUSINGTYPELOC: u64 = 4294970966;
const CLASS_QUALIFIEDTYPELOC: u64 = 4294970968;
const CLASS_ALIGNMAC68KATTR: u64 = 4294970974;
const CLASS_BTFTAGATTRIBUTEDTYPELOC: u64 = 4294970975;
const CLASS_NAMEDDECL: u64 = 4294970980;
const CLASS_FUNCTIONTYPELOC: u64 = 4294971001;
const CLASS_SWIFTASYNCNAMEATTR: u64 = 4294971016;
const CLASS_DEPENDENTTEMPLATESPECIALIZATIONTYPELOC: u64 = 4294971017;
const CLASS_OMPDISPATCHDIRECTIVE: u64 = 4294971028;
const CLASS_SWIFTNAMEATTR: u64 = 4294971029;
const CLASS_DEPENDENTSIZEDARRAYTYPE: u64 = 4294971030;
const CLASS_VARIABLEARRAYTYPELOC: u64 = 4294971037;
const CLASS_FUNCTIONNOPROTOTYPELOC: u64 = 4294971038;
const CLASS_MSDEPENDENTEXISTSSTMT: u64 = 4294971039;
const CLASS_UNARYTRANSFORMTYPELOC: u64 = 4294971049;
const CLASS_COROUTINESUSPENDEXPR: u64 = 4294971056;
const CLASS_CODEMODELATTR: u64 = 4294971067;
const CLASS_UNQUALTYPELOC: u64 = 4294971068;
const CLASS_WEAKREFATTR: u64 = 4294971071;
const CLASS_TYPELOC: u64 = 4294971072;
const CLASS_TYPEOFEXPRTYPELOC: u64 = 4294971089;
const CLASS_ARCWEAKREFUNAVAILABLEATTR: u64 = 4294971092;
const CLASS_SYCLKERNELATTR: u64 = 4294971093;
const CLASS_BTFTAGATTRIBUTEDTYPE: u64 = 4294971094;
const CLASS_ELABORATEDTYPELOC: u64 = 4294971099;
const CLASS_OMPDISTRIBUTESIMDDIRECTIVE: u64 = 4294971108;
const CLASS_CPUSPECIFICATTR: u64 = 4294971109;
const CLASS_RECORDTYPELOC: u64 = 4294971110;
const CLASS_RVALUEREFERENCETYPELOC: u64 = 4294971112;
const CLASS_PRAGMACLANGBSSSECTIONATTR: u64 = 4294971114;
const CLASS_OBJCINTERFACETYPELOC: u64 = 4294971115;
const CLASS_AUTOTYPELOC: u64 = 4294971116;
const CLASS_SWIFTATTRATTR: u64 = 4294971133;
const CLASS_TYPESPECTYPELOC: u64 = 4294971134;
const CLASS_OMPSINGLEDIRECTIVE: u64 = 4294971137;
const CLASS_SIZEOFPACKEXPR: u64 = 4294971138;
const CLASS_OBJCTYPEPARAMTYPELOC: u64 = 4294971149;
const CLASS_DEDUCEDTEMPLATESPECIALIZATIONTYPELOC: u64 = 4294971150;
const CLASS_POINTERATTR: u64 = 4294971152;
const CLASS_SUBSTTEMPLATETYPEPARMPACKTYPELOC: u64 = 4294971153;
const CLASS_MSASMSTMT: u64 = 4294971154;
const CLASS_TEMPLATETYPEPARMTYPELOC: u64 = 4294971164;
const CLASS_MEMBERPOINTERTYPELOC: u64 = 4294971166;
const CLASS_SUBSTTEMPLATETYPEPARMTYPELOC: u64 = 4294971171;
const CLASS_NODEBUGATTR: u64 = 4294971172;
const CLASS_VARIABLEARRAYTYPE: u64 = 4294971173;
const CLASS_LABELDECL: u64 = 4294971180;
const CLASS_DECLARATORDECL: u64 = 4294971187;
const CLASS_NAMESPACEDECL: u64 = 4294971199;
const CLASS_HLSLRESOURCEBINDINGATTR: u64 = 4294971210;
const CLASS_DECOMPOSITIONDECL: u64 = 4294971211;
const ENUM_VALUEKIND: u64 = 4294971213;
const ENUM_ACCESSSPECIFIER: u64 = 4294971228;
const ENUM_ARRAYSIZEMODIFIER: u64 = 4294971233;
const ENUM_ARRAYTYPETRAIT: u64 = 4294971237;
const ENUM_ATOMICOP: u64 = 4294971241;
const ENUM_AUTOTYPEKEYWORD: u64 = 4294971330;
const ENUM_BINARYOPERATORKIND: u64 = 4294971334;
const ENUM_BUILTINTEMPLATEKIND: u64 = 4294971368;
const ENUM_KIND: u64 = 4294971371;
const ENUM_CXXCONSTRUCTIONKIND: u64 = 4294971872;
const ENUM_CXXNEWINITIALIZATIONSTYLE: u64 = 4294971877;
const ENUM_ADLCALLKIND: u64 = 4294971881;
const ENUM_CALLINGCONV: u64 = 4294971884;
const ENUM_CANTHROWRESULT: u64 = 4294971907;
const ENUM_CAPTUREDREGIONKIND: u64 = 4294971911;
const ENUM_CASTKIND: u64 = 4294971915;
const ENUM_CHARACTERLITERALKIND: u64 = 4294971981;
const ENUM_CONSTANTRESULTSTORAGEKIND: u64 = 4294971987;
const ENUM_CONSTEXPRSPECKIND: u64 = 4294971991;
const ENUM_FRIENDOBJECTKIND: u64 = 4294971996;
const ENUM_MODULEOWNERSHIPKIND: u64 = 4294972000;
const ENUM_OBJCDECLQUALIFIER: u64 = 4294972006;
const ENUM_DEDUCTIONCANDIDATE: u64 = 4294972015;
const ENUM_ELABORATEDTYPEKEYWORD: u64 = 4294972019;
const ENUM_EXCEPTIONSPECIFICATIONTYPE: u64 = 4294972027;
const ENUM_EXPRDEPENDENCE: u64 = 4294972040;
const ENUM_EXPROBJECTKIND: u64 = 4294972054;
const ENUM_EXPRVALUEKIND: u64 = 4294972061;
const ENUM_EXPRESSIONTRAIT: u64 = 4294972065;
const ENUM_TEMPLATEDKIND: u64 = 4294972069;
const ENUM_IFSTATEMENTKIND: u64 = 4294972076;
const ENUM_IMPLICITPARAMKIND: u64 = 4294972081;
const ENUM_INCLASSINITSTYLE: u64 = 4294972089;
const ENUM_LAMBDACAPTUREDEFAULT: u64 = 4294972093;
const ENUM_LANGUAGELINKAGE: u64 = 4294972097;
const ENUM_LINKAGE: u64 = 4294972101;
const ENUM_LINKAGESPECLANGUAGEIDS: u64 = 4294972109;
const ENUM_MSVTORDISPMODE: u64 = 4294972112;
const ENUM_MULTIVERSIONKIND: u64 = 4294972116;
const ENUM_NONODRUSEREASON: u64 = 4294972123;
const ENUM_OBJCSTRINGFORMATFAMILY: u64 = 4294972128;
const ENUM_OVERLOADEDOPERATORKIND: u64 = 4294972132;
const ENUM_PRAGMAMSCOMMENTKIND: u64 = 4294972180;
const ENUM_PREDEFINEDIDENTKIND: u64 = 4294972187;
const ENUM_OBJCLIFETIME: u64 = 4294972196;
const ENUM_RECORDARGPASSINGKIND: u64 = 4294972202;
const ENUM_REFQUALIFIERKIND: u64 = 4294972206;
const ENUM_SOURCELOCIDENTKIND: u64 = 4294972210;
const ENUM_STORAGECLASS: u64 = 4294972218;
const ENUM_STORAGEDURATION: u64 = 4294972225;
const ENUM_STRINGLITERALKIND: u64 = 4294972231;
const ENUM_TAGTYPEKIND: u64 = 4294972238;
const ENUM_TEMPLATESPECIALIZATIONKIND: u64 = 4294972244;
const ENUM_THREADSTORAGECLASSSPECIFIER: u64 = 4294972250;
const ENUM_TYPEDEPENDENCE: u64 = 4294972255;
const ENUM_TYPEOFKIND: u64 = 4294972265;
const ENUM_TYPETRAIT: u64 = 4294972268;
const ENUM_UNARYEXPRORTYPETRAIT: u64 = 4294972345;
const ENUM_UNARYOPERATORKIND: u64 = 4294972354;
const ENUM_UTTKIND: u64 = 4294972369;
const ENUM_LITERALOPERATORKIND: u64 = 4294972386;
const ENUM_DEFINITIONKIND: u64 = 4294972393;
const ENUM_INITIALIZATIONSTYLE: u64 = 4294972397;
const ENUM_TLSKIND: u64 = 4294972402;
const ENUM_VECTORKIND: u64 = 4294972406;
const ENUM_VISIBILITY: u64 = 4294972417;
const ENUM_KIND_1: u64 = 4294972421;
const ENUM_SEMANTICS: u64 = 4294972836;
////   END ARBORETUM GENERATED CODE ////
