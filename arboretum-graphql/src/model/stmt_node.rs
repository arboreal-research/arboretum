use juniper::*;

pub mod asm_stmt_node;
pub mod switch_case_node;
pub mod value_stmt_node;

#[derive(GraphQLObject)]
pub struct Stmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DeclStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NullStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CompoundStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct IfStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SwitchStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct WhileStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DoStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ForStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct GotoStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct IndirectGotoStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ContinueStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct BreakStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ReturnStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SEHExceptStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SEHFinallyStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SEHTryStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SEHLeaveStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CapturedStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CXXCatchStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CXXTryStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CXXForRangeStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MSDependentExistsStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CoroutineBodyStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CoreturnStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCForCollectionStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCAtCatchStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCAtFinallyStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCAtTryStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCAtSynchronizedStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCAtThrowStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCAutoreleasePoolStmt {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum StmtNode {
  Stmt(Stmt),
  AbstractConditionalOperator(value_stmt_node::expr_node::abstract_conditional_operator_node::AbstractConditionalOperator),
  AddrLabelExpr(value_stmt_node::expr_node::AddrLabelExpr),
  ArrayInitIndexExpr(value_stmt_node::expr_node::ArrayInitIndexExpr),
  ArrayInitLoopExpr(value_stmt_node::expr_node::ArrayInitLoopExpr),
  ArraySubscriptExpr(value_stmt_node::expr_node::ArraySubscriptExpr),
  ArrayTypeTraitExpr(value_stmt_node::expr_node::ArrayTypeTraitExpr),
  AsTypeExpr(value_stmt_node::expr_node::AsTypeExpr),
  AsmStmt(asm_stmt_node::AsmStmt),
  AtomicExpr(value_stmt_node::expr_node::AtomicExpr),
  AttributedStmt(value_stmt_node::AttributedStmt),
  BinaryConditionalOperator(value_stmt_node::expr_node::abstract_conditional_operator_node::BinaryConditionalOperator),
  BinaryOperator(value_stmt_node::expr_node::binary_operator_node::BinaryOperator),
  BlockExpr(value_stmt_node::expr_node::BlockExpr),
  BreakStmt(BreakStmt),
  BuiltinBitCastExpr(value_stmt_node::expr_node::cast_expr_node::explicit_cast_expr_node::BuiltinBitCastExpr),
  CStyleCastExpr(value_stmt_node::expr_node::cast_expr_node::explicit_cast_expr_node::CStyleCastExpr),
  CUDAKernelCallExpr(value_stmt_node::expr_node::call_expr_node::CUDAKernelCallExpr),
  CXXAddrspaceCastExpr(value_stmt_node::expr_node::cast_expr_node::explicit_cast_expr_node::cxx_named_cast_expr_node::CXXAddrspaceCastExpr),
  CXXBindTemporaryExpr(value_stmt_node::expr_node::CXXBindTemporaryExpr),
  CXXBoolLiteralExpr(value_stmt_node::expr_node::CXXBoolLiteralExpr),
  CXXCatchStmt(CXXCatchStmt),
  CXXConstCastExpr(value_stmt_node::expr_node::cast_expr_node::explicit_cast_expr_node::cxx_named_cast_expr_node::CXXConstCastExpr),
  CXXConstructExpr(value_stmt_node::expr_node::cxx_construct_expr_node::CXXConstructExpr),
  CXXDefaultArgExpr(value_stmt_node::expr_node::CXXDefaultArgExpr),
  CXXDefaultInitExpr(value_stmt_node::expr_node::CXXDefaultInitExpr),
  CXXDeleteExpr(value_stmt_node::expr_node::CXXDeleteExpr),
  CXXDependentScopeMemberExpr(value_stmt_node::expr_node::CXXDependentScopeMemberExpr),
  CXXDynamicCastExpr(value_stmt_node::expr_node::cast_expr_node::explicit_cast_expr_node::cxx_named_cast_expr_node::CXXDynamicCastExpr),
  CXXFoldExpr(value_stmt_node::expr_node::CXXFoldExpr),
  CXXForRangeStmt(CXXForRangeStmt),
  CXXFunctionalCastExpr(value_stmt_node::expr_node::cast_expr_node::explicit_cast_expr_node::CXXFunctionalCastExpr),
  CXXInheritedCtorInitExpr(value_stmt_node::expr_node::CXXInheritedCtorInitExpr),
  CXXMemberCallExpr(value_stmt_node::expr_node::call_expr_node::CXXMemberCallExpr),
  CXXNamedCastExpr(value_stmt_node::expr_node::cast_expr_node::explicit_cast_expr_node::cxx_named_cast_expr_node::CXXNamedCastExpr),
  CXXNewExpr(value_stmt_node::expr_node::CXXNewExpr),
  CXXNoexceptExpr(value_stmt_node::expr_node::CXXNoexceptExpr),
  CXXNullPtrLiteralExpr(value_stmt_node::expr_node::CXXNullPtrLiteralExpr),
  CXXOperatorCallExpr(value_stmt_node::expr_node::call_expr_node::CXXOperatorCallExpr),
  CXXParenListInitExpr(value_stmt_node::expr_node::CXXParenListInitExpr),
  CXXPseudoDestructorExpr(value_stmt_node::expr_node::CXXPseudoDestructorExpr),
  CXXReinterpretCastExpr(value_stmt_node::expr_node::cast_expr_node::explicit_cast_expr_node::cxx_named_cast_expr_node::CXXReinterpretCastExpr),
  CXXRewrittenBinaryOperator(value_stmt_node::expr_node::CXXRewrittenBinaryOperator),
  CXXScalarValueInitExpr(value_stmt_node::expr_node::CXXScalarValueInitExpr),
  CXXStaticCastExpr(value_stmt_node::expr_node::cast_expr_node::explicit_cast_expr_node::cxx_named_cast_expr_node::CXXStaticCastExpr),
  CXXStdInitializerListExpr(value_stmt_node::expr_node::CXXStdInitializerListExpr),
  CXXTemporaryObjectExpr(value_stmt_node::expr_node::cxx_construct_expr_node::CXXTemporaryObjectExpr),
  CXXThisExpr(value_stmt_node::expr_node::CXXThisExpr),
  CXXThrowExpr(value_stmt_node::expr_node::CXXThrowExpr),
  CXXTryStmt(CXXTryStmt),
  CXXTypeidExpr(value_stmt_node::expr_node::CXXTypeidExpr),
  CXXUnresolvedConstructExpr(value_stmt_node::expr_node::CXXUnresolvedConstructExpr),
  CXXUuidofExpr(value_stmt_node::expr_node::CXXUuidofExpr),
  CallExpr(value_stmt_node::expr_node::call_expr_node::CallExpr),
  CapturedStmt(CapturedStmt),
  CaseStmt(switch_case_node::CaseStmt),
  CastExpr(value_stmt_node::expr_node::cast_expr_node::CastExpr),
  CharacterLiteral(value_stmt_node::expr_node::CharacterLiteral),
  ChooseExpr(value_stmt_node::expr_node::ChooseExpr),
  CoawaitExpr(value_stmt_node::expr_node::coroutine_suspend_expr_node::CoawaitExpr),
  CompoundAssignOperator(value_stmt_node::expr_node::binary_operator_node::CompoundAssignOperator),
  CompoundLiteralExpr(value_stmt_node::expr_node::CompoundLiteralExpr),
  CompoundStmt(CompoundStmt),
  ConceptSpecializationExpr(value_stmt_node::expr_node::ConceptSpecializationExpr),
  ConditionalOperator(value_stmt_node::expr_node::abstract_conditional_operator_node::ConditionalOperator),
  ConstantExpr(value_stmt_node::expr_node::full_expr_node::ConstantExpr),
  ContinueStmt(ContinueStmt),
  ConvertVectorExpr(value_stmt_node::expr_node::ConvertVectorExpr),
  CoreturnStmt(CoreturnStmt),
  CoroutineBodyStmt(CoroutineBodyStmt),
  CoroutineSuspendExpr(value_stmt_node::expr_node::coroutine_suspend_expr_node::CoroutineSuspendExpr),
  CoyieldExpr(value_stmt_node::expr_node::coroutine_suspend_expr_node::CoyieldExpr),
  DeclRefExpr(value_stmt_node::expr_node::DeclRefExpr),
  DeclStmt(DeclStmt),
  DefaultStmt(switch_case_node::DefaultStmt),
  DependentCoawaitExpr(value_stmt_node::expr_node::DependentCoawaitExpr),
  DependentScopeDeclRefExpr(value_stmt_node::expr_node::DependentScopeDeclRefExpr),
  DesignatedInitExpr(value_stmt_node::expr_node::DesignatedInitExpr),
  DesignatedInitUpdateExpr(value_stmt_node::expr_node::DesignatedInitUpdateExpr),
  DoStmt(DoStmt),
  ExplicitCastExpr(value_stmt_node::expr_node::cast_expr_node::explicit_cast_expr_node::ExplicitCastExpr),
  Expr(value_stmt_node::expr_node::Expr),
  ExprWithCleanups(value_stmt_node::expr_node::full_expr_node::ExprWithCleanups),
  ExpressionTraitExpr(value_stmt_node::expr_node::ExpressionTraitExpr),
  ExtVectorElementExpr(value_stmt_node::expr_node::ExtVectorElementExpr),
  FixedPointLiteral(value_stmt_node::expr_node::FixedPointLiteral),
  FloatingLiteral(value_stmt_node::expr_node::FloatingLiteral),
  ForStmt(ForStmt),
  FullExpr(value_stmt_node::expr_node::full_expr_node::FullExpr),
  FunctionParmPackExpr(value_stmt_node::expr_node::FunctionParmPackExpr),
  GCCAsmStmt(asm_stmt_node::GCCAsmStmt),
  GNUNullExpr(value_stmt_node::expr_node::GNUNullExpr),
  GenericSelectionExpr(value_stmt_node::expr_node::GenericSelectionExpr),
  GotoStmt(GotoStmt),
  IfStmt(IfStmt),
  ImaginaryLiteral(value_stmt_node::expr_node::ImaginaryLiteral),
  ImplicitCastExpr(value_stmt_node::expr_node::cast_expr_node::ImplicitCastExpr),
  ImplicitValueInitExpr(value_stmt_node::expr_node::ImplicitValueInitExpr),
  IndirectGotoStmt(IndirectGotoStmt),
  InitListExpr(value_stmt_node::expr_node::InitListExpr),
  IntegerLiteral(value_stmt_node::expr_node::IntegerLiteral),
  LabelStmt(value_stmt_node::LabelStmt),
  LambdaExpr(value_stmt_node::expr_node::LambdaExpr),
  MSAsmStmt(asm_stmt_node::MSAsmStmt),
  MSDependentExistsStmt(MSDependentExistsStmt),
  MSPropertyRefExpr(value_stmt_node::expr_node::MSPropertyRefExpr),
  MSPropertySubscriptExpr(value_stmt_node::expr_node::MSPropertySubscriptExpr),
  MaterializeTemporaryExpr(value_stmt_node::expr_node::MaterializeTemporaryExpr),
  MatrixSubscriptExpr(value_stmt_node::expr_node::MatrixSubscriptExpr),
  MemberExpr(value_stmt_node::expr_node::MemberExpr),
  NoInitExpr(value_stmt_node::expr_node::NoInitExpr),
  NullStmt(NullStmt),
  ObjCArrayLiteral(value_stmt_node::expr_node::ObjCArrayLiteral),
  ObjCAtCatchStmt(ObjCAtCatchStmt),
  ObjCAtFinallyStmt(ObjCAtFinallyStmt),
  ObjCAtSynchronizedStmt(ObjCAtSynchronizedStmt),
  ObjCAtThrowStmt(ObjCAtThrowStmt),
  ObjCAtTryStmt(ObjCAtTryStmt),
  ObjCAutoreleasePoolStmt(ObjCAutoreleasePoolStmt),
  ObjCAvailabilityCheckExpr(value_stmt_node::expr_node::ObjCAvailabilityCheckExpr),
  ObjCBoolLiteralExpr(value_stmt_node::expr_node::ObjCBoolLiteralExpr),
  ObjCBoxedExpr(value_stmt_node::expr_node::ObjCBoxedExpr),
  ObjCBridgedCastExpr(value_stmt_node::expr_node::cast_expr_node::explicit_cast_expr_node::ObjCBridgedCastExpr),
  ObjCDictionaryLiteral(value_stmt_node::expr_node::ObjCDictionaryLiteral),
  ObjCEncodeExpr(value_stmt_node::expr_node::ObjCEncodeExpr),
  ObjCForCollectionStmt(ObjCForCollectionStmt),
  ObjCIndirectCopyRestoreExpr(value_stmt_node::expr_node::ObjCIndirectCopyRestoreExpr),
  ObjCIsaExpr(value_stmt_node::expr_node::ObjCIsaExpr),
  ObjCIvarRefExpr(value_stmt_node::expr_node::ObjCIvarRefExpr),
  ObjCMessageExpr(value_stmt_node::expr_node::ObjCMessageExpr),
  ObjCPropertyRefExpr(value_stmt_node::expr_node::ObjCPropertyRefExpr),
  ObjCProtocolExpr(value_stmt_node::expr_node::ObjCProtocolExpr),
  ObjCSelectorExpr(value_stmt_node::expr_node::ObjCSelectorExpr),
  ObjCStringLiteral(value_stmt_node::expr_node::ObjCStringLiteral),
  ObjCSubscriptRefExpr(value_stmt_node::expr_node::ObjCSubscriptRefExpr),
  OffsetOfExpr(value_stmt_node::expr_node::OffsetOfExpr),
  OpaqueValueExpr(value_stmt_node::expr_node::OpaqueValueExpr),
  OverloadExpr(value_stmt_node::expr_node::overload_expr_node::OverloadExpr),
  PackExpansionExpr(value_stmt_node::expr_node::PackExpansionExpr),
  ParenExpr(value_stmt_node::expr_node::ParenExpr),
  ParenListExpr(value_stmt_node::expr_node::ParenListExpr),
  PredefinedExpr(value_stmt_node::expr_node::PredefinedExpr),
  PseudoObjectExpr(value_stmt_node::expr_node::PseudoObjectExpr),
  RecoveryExpr(value_stmt_node::expr_node::RecoveryExpr),
  RequiresExpr(value_stmt_node::expr_node::RequiresExpr),
  ReturnStmt(ReturnStmt),
  SEHExceptStmt(SEHExceptStmt),
  SEHFinallyStmt(SEHFinallyStmt),
  SEHLeaveStmt(SEHLeaveStmt),
  SEHTryStmt(SEHTryStmt),
  SYCLUniqueStableNameExpr(value_stmt_node::expr_node::SYCLUniqueStableNameExpr),
  ShuffleVectorExpr(value_stmt_node::expr_node::ShuffleVectorExpr),
  SizeOfPackExpr(value_stmt_node::expr_node::SizeOfPackExpr),
  SourceLocExpr(value_stmt_node::expr_node::SourceLocExpr),
  StmtExpr(value_stmt_node::expr_node::StmtExpr),
  StringLiteral(value_stmt_node::expr_node::StringLiteral),
  SubstNonTypeTemplateParmExpr(value_stmt_node::expr_node::SubstNonTypeTemplateParmExpr),
  SubstNonTypeTemplateParmPackExpr(value_stmt_node::expr_node::SubstNonTypeTemplateParmPackExpr),
  SwitchCase(switch_case_node::SwitchCase),
  SwitchStmt(SwitchStmt),
  TypeTraitExpr(value_stmt_node::expr_node::TypeTraitExpr),
  TypoExpr(value_stmt_node::expr_node::TypoExpr),
  UnaryExprOrTypeTraitExpr(value_stmt_node::expr_node::UnaryExprOrTypeTraitExpr),
  UnaryOperator(value_stmt_node::expr_node::UnaryOperator),
  UnresolvedLookupExpr(value_stmt_node::expr_node::overload_expr_node::UnresolvedLookupExpr),
  UnresolvedMemberExpr(value_stmt_node::expr_node::overload_expr_node::UnresolvedMemberExpr),
  UserDefinedLiteral(value_stmt_node::expr_node::call_expr_node::UserDefinedLiteral),
  VAArgExpr(value_stmt_node::expr_node::VAArgExpr),
  ValueStmt(value_stmt_node::ValueStmt),
  WhileStmt(WhileStmt),
}

