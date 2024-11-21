use juniper::*;

pub mod expr_node;

#[derive(GraphQLObject)]
pub struct ValueStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct LabelStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AttributedStmt {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum ValueStmtNode {
  ValueStmt(ValueStmt),
  AbstractConditionalOperator(expr_node::abstract_conditional_operator_node::AbstractConditionalOperator),
  AddrLabelExpr(expr_node::AddrLabelExpr),
  ArrayInitIndexExpr(expr_node::ArrayInitIndexExpr),
  ArrayInitLoopExpr(expr_node::ArrayInitLoopExpr),
  ArraySubscriptExpr(expr_node::ArraySubscriptExpr),
  ArrayTypeTraitExpr(expr_node::ArrayTypeTraitExpr),
  AsTypeExpr(expr_node::AsTypeExpr),
  AtomicExpr(expr_node::AtomicExpr),
  AttributedStmt(AttributedStmt),
  BinaryConditionalOperator(expr_node::abstract_conditional_operator_node::BinaryConditionalOperator),
  BinaryOperator(expr_node::binary_operator_node::BinaryOperator),
  BlockExpr(expr_node::BlockExpr),
  BuiltinBitCastExpr(expr_node::cast_expr_node::explicit_cast_expr_node::BuiltinBitCastExpr),
  CStyleCastExpr(expr_node::cast_expr_node::explicit_cast_expr_node::CStyleCastExpr),
  CUDAKernelCallExpr(expr_node::call_expr_node::CUDAKernelCallExpr),
  CXXAddrspaceCastExpr(expr_node::cast_expr_node::explicit_cast_expr_node::cxx_named_cast_expr_node::CXXAddrspaceCastExpr),
  CXXBindTemporaryExpr(expr_node::CXXBindTemporaryExpr),
  CXXBoolLiteralExpr(expr_node::CXXBoolLiteralExpr),
  CXXConstCastExpr(expr_node::cast_expr_node::explicit_cast_expr_node::cxx_named_cast_expr_node::CXXConstCastExpr),
  CXXConstructExpr(expr_node::cxx_construct_expr_node::CXXConstructExpr),
  CXXDefaultArgExpr(expr_node::CXXDefaultArgExpr),
  CXXDefaultInitExpr(expr_node::CXXDefaultInitExpr),
  CXXDeleteExpr(expr_node::CXXDeleteExpr),
  CXXDependentScopeMemberExpr(expr_node::CXXDependentScopeMemberExpr),
  CXXDynamicCastExpr(expr_node::cast_expr_node::explicit_cast_expr_node::cxx_named_cast_expr_node::CXXDynamicCastExpr),
  CXXFoldExpr(expr_node::CXXFoldExpr),
  CXXFunctionalCastExpr(expr_node::cast_expr_node::explicit_cast_expr_node::CXXFunctionalCastExpr),
  CXXInheritedCtorInitExpr(expr_node::CXXInheritedCtorInitExpr),
  CXXMemberCallExpr(expr_node::call_expr_node::CXXMemberCallExpr),
  CXXNamedCastExpr(expr_node::cast_expr_node::explicit_cast_expr_node::cxx_named_cast_expr_node::CXXNamedCastExpr),
  CXXNewExpr(expr_node::CXXNewExpr),
  CXXNoexceptExpr(expr_node::CXXNoexceptExpr),
  CXXNullPtrLiteralExpr(expr_node::CXXNullPtrLiteralExpr),
  CXXOperatorCallExpr(expr_node::call_expr_node::CXXOperatorCallExpr),
  CXXParenListInitExpr(expr_node::CXXParenListInitExpr),
  CXXPseudoDestructorExpr(expr_node::CXXPseudoDestructorExpr),
  CXXReinterpretCastExpr(expr_node::cast_expr_node::explicit_cast_expr_node::cxx_named_cast_expr_node::CXXReinterpretCastExpr),
  CXXRewrittenBinaryOperator(expr_node::CXXRewrittenBinaryOperator),
  CXXScalarValueInitExpr(expr_node::CXXScalarValueInitExpr),
  CXXStaticCastExpr(expr_node::cast_expr_node::explicit_cast_expr_node::cxx_named_cast_expr_node::CXXStaticCastExpr),
  CXXStdInitializerListExpr(expr_node::CXXStdInitializerListExpr),
  CXXTemporaryObjectExpr(expr_node::cxx_construct_expr_node::CXXTemporaryObjectExpr),
  CXXThisExpr(expr_node::CXXThisExpr),
  CXXThrowExpr(expr_node::CXXThrowExpr),
  CXXTypeidExpr(expr_node::CXXTypeidExpr),
  CXXUnresolvedConstructExpr(expr_node::CXXUnresolvedConstructExpr),
  CXXUuidofExpr(expr_node::CXXUuidofExpr),
  CallExpr(expr_node::call_expr_node::CallExpr),
  CastExpr(expr_node::cast_expr_node::CastExpr),
  CharacterLiteral(expr_node::CharacterLiteral),
  ChooseExpr(expr_node::ChooseExpr),
  CoawaitExpr(expr_node::coroutine_suspend_expr_node::CoawaitExpr),
  CompoundAssignOperator(expr_node::binary_operator_node::CompoundAssignOperator),
  CompoundLiteralExpr(expr_node::CompoundLiteralExpr),
  ConceptSpecializationExpr(expr_node::ConceptSpecializationExpr),
  ConditionalOperator(expr_node::abstract_conditional_operator_node::ConditionalOperator),
  ConstantExpr(expr_node::full_expr_node::ConstantExpr),
  ConvertVectorExpr(expr_node::ConvertVectorExpr),
  CoroutineSuspendExpr(expr_node::coroutine_suspend_expr_node::CoroutineSuspendExpr),
  CoyieldExpr(expr_node::coroutine_suspend_expr_node::CoyieldExpr),
  DeclRefExpr(expr_node::DeclRefExpr),
  DependentCoawaitExpr(expr_node::DependentCoawaitExpr),
  DependentScopeDeclRefExpr(expr_node::DependentScopeDeclRefExpr),
  DesignatedInitExpr(expr_node::DesignatedInitExpr),
  DesignatedInitUpdateExpr(expr_node::DesignatedInitUpdateExpr),
  ExplicitCastExpr(expr_node::cast_expr_node::explicit_cast_expr_node::ExplicitCastExpr),
  Expr(expr_node::Expr),
  ExprWithCleanups(expr_node::full_expr_node::ExprWithCleanups),
  ExpressionTraitExpr(expr_node::ExpressionTraitExpr),
  ExtVectorElementExpr(expr_node::ExtVectorElementExpr),
  FixedPointLiteral(expr_node::FixedPointLiteral),
  FloatingLiteral(expr_node::FloatingLiteral),
  FullExpr(expr_node::full_expr_node::FullExpr),
  FunctionParmPackExpr(expr_node::FunctionParmPackExpr),
  GNUNullExpr(expr_node::GNUNullExpr),
  GenericSelectionExpr(expr_node::GenericSelectionExpr),
  ImaginaryLiteral(expr_node::ImaginaryLiteral),
  ImplicitCastExpr(expr_node::cast_expr_node::ImplicitCastExpr),
  ImplicitValueInitExpr(expr_node::ImplicitValueInitExpr),
  InitListExpr(expr_node::InitListExpr),
  IntegerLiteral(expr_node::IntegerLiteral),
  LabelStmt(LabelStmt),
  LambdaExpr(expr_node::LambdaExpr),
  MSPropertyRefExpr(expr_node::MSPropertyRefExpr),
  MSPropertySubscriptExpr(expr_node::MSPropertySubscriptExpr),
  MaterializeTemporaryExpr(expr_node::MaterializeTemporaryExpr),
  MatrixSubscriptExpr(expr_node::MatrixSubscriptExpr),
  MemberExpr(expr_node::MemberExpr),
  NoInitExpr(expr_node::NoInitExpr),
  ObjCArrayLiteral(expr_node::ObjCArrayLiteral),
  ObjCAvailabilityCheckExpr(expr_node::ObjCAvailabilityCheckExpr),
  ObjCBoolLiteralExpr(expr_node::ObjCBoolLiteralExpr),
  ObjCBoxedExpr(expr_node::ObjCBoxedExpr),
  ObjCBridgedCastExpr(expr_node::cast_expr_node::explicit_cast_expr_node::ObjCBridgedCastExpr),
  ObjCDictionaryLiteral(expr_node::ObjCDictionaryLiteral),
  ObjCEncodeExpr(expr_node::ObjCEncodeExpr),
  ObjCIndirectCopyRestoreExpr(expr_node::ObjCIndirectCopyRestoreExpr),
  ObjCIsaExpr(expr_node::ObjCIsaExpr),
  ObjCIvarRefExpr(expr_node::ObjCIvarRefExpr),
  ObjCMessageExpr(expr_node::ObjCMessageExpr),
  ObjCPropertyRefExpr(expr_node::ObjCPropertyRefExpr),
  ObjCProtocolExpr(expr_node::ObjCProtocolExpr),
  ObjCSelectorExpr(expr_node::ObjCSelectorExpr),
  ObjCStringLiteral(expr_node::ObjCStringLiteral),
  ObjCSubscriptRefExpr(expr_node::ObjCSubscriptRefExpr),
  OffsetOfExpr(expr_node::OffsetOfExpr),
  OpaqueValueExpr(expr_node::OpaqueValueExpr),
  OverloadExpr(expr_node::overload_expr_node::OverloadExpr),
  PackExpansionExpr(expr_node::PackExpansionExpr),
  ParenExpr(expr_node::ParenExpr),
  ParenListExpr(expr_node::ParenListExpr),
  PredefinedExpr(expr_node::PredefinedExpr),
  PseudoObjectExpr(expr_node::PseudoObjectExpr),
  RecoveryExpr(expr_node::RecoveryExpr),
  RequiresExpr(expr_node::RequiresExpr),
  SYCLUniqueStableNameExpr(expr_node::SYCLUniqueStableNameExpr),
  ShuffleVectorExpr(expr_node::ShuffleVectorExpr),
  SizeOfPackExpr(expr_node::SizeOfPackExpr),
  SourceLocExpr(expr_node::SourceLocExpr),
  StmtExpr(expr_node::StmtExpr),
  StringLiteral(expr_node::StringLiteral),
  SubstNonTypeTemplateParmExpr(expr_node::SubstNonTypeTemplateParmExpr),
  SubstNonTypeTemplateParmPackExpr(expr_node::SubstNonTypeTemplateParmPackExpr),
  TypeTraitExpr(expr_node::TypeTraitExpr),
  TypoExpr(expr_node::TypoExpr),
  UnaryExprOrTypeTraitExpr(expr_node::UnaryExprOrTypeTraitExpr),
  UnaryOperator(expr_node::UnaryOperator),
  UnresolvedLookupExpr(expr_node::overload_expr_node::UnresolvedLookupExpr),
  UnresolvedMemberExpr(expr_node::overload_expr_node::UnresolvedMemberExpr),
  UserDefinedLiteral(expr_node::call_expr_node::UserDefinedLiteral),
  VAArgExpr(expr_node::VAArgExpr),
}
