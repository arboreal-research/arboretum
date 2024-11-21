use juniper::*;

pub mod abstract_conditional_operator_node;
pub mod binary_operator_node;
pub mod call_expr_node;
pub mod cast_expr_node;
pub mod coroutine_suspend_expr_node;
pub mod cxx_construct_expr_node;
pub mod full_expr_node;
pub mod overload_expr_node;

#[derive(GraphQLObject)]
pub struct Expr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct OpaqueValueExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DeclRefExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct IntegerLiteral {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct FixedPointLiteral {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CharacterLiteral {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct FloatingLiteral {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ImaginaryLiteral {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct StringLiteral {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct PredefinedExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SYCLUniqueStableNameExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ParenExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct UnaryOperator {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct OffsetOfExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct UnaryExprOrTypeTraitExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ArraySubscriptExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MatrixSubscriptExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MemberExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CompoundLiteralExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AddrLabelExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct StmtExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ShuffleVectorExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ConvertVectorExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ChooseExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct GNUNullExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct VAArgExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SourceLocExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct InitListExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DesignatedInitExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NoInitExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DesignatedInitUpdateExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ArrayInitLoopExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ArrayInitIndexExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ImplicitValueInitExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ParenListExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct GenericSelectionExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ExtVectorElementExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct BlockExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AsTypeExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct PseudoObjectExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AtomicExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct TypoExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct RecoveryExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CXXRewrittenBinaryOperator {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CXXBoolLiteralExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CXXNullPtrLiteralExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CXXStdInitializerListExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CXXTypeidExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MSPropertyRefExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MSPropertySubscriptExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CXXUuidofExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CXXThisExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CXXThrowExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CXXDefaultArgExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CXXDefaultInitExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CXXBindTemporaryExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CXXInheritedCtorInitExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct LambdaExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CXXScalarValueInitExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CXXNewExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CXXDeleteExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CXXPseudoDestructorExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct TypeTraitExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ArrayTypeTraitExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ExpressionTraitExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DependentScopeDeclRefExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CXXUnresolvedConstructExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CXXDependentScopeMemberExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CXXNoexceptExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct PackExpansionExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SizeOfPackExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SubstNonTypeTemplateParmExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SubstNonTypeTemplateParmPackExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct FunctionParmPackExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MaterializeTemporaryExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CXXFoldExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CXXParenListInitExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DependentCoawaitExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ConceptSpecializationExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct RequiresExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCStringLiteral {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCBoolLiteralExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCBoxedExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCArrayLiteral {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCDictionaryLiteral {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCEncodeExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCSelectorExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCProtocolExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCIvarRefExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCPropertyRefExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCSubscriptRefExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCMessageExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCIsaExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCIndirectCopyRestoreExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCAvailabilityCheckExpr {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum ExprNode {
  Expr(Expr),
  AbstractConditionalOperator(abstract_conditional_operator_node::AbstractConditionalOperator),
  AddrLabelExpr(AddrLabelExpr),
  ArrayInitIndexExpr(ArrayInitIndexExpr),
  ArrayInitLoopExpr(ArrayInitLoopExpr),
  ArraySubscriptExpr(ArraySubscriptExpr),
  ArrayTypeTraitExpr(ArrayTypeTraitExpr),
  AsTypeExpr(AsTypeExpr),
  AtomicExpr(AtomicExpr),
  BinaryConditionalOperator(abstract_conditional_operator_node::BinaryConditionalOperator),
  BinaryOperator(binary_operator_node::BinaryOperator),
  BlockExpr(BlockExpr),
  BuiltinBitCastExpr(cast_expr_node::explicit_cast_expr_node::BuiltinBitCastExpr),
  CStyleCastExpr(cast_expr_node::explicit_cast_expr_node::CStyleCastExpr),
  CUDAKernelCallExpr(call_expr_node::CUDAKernelCallExpr),
  CXXAddrspaceCastExpr(cast_expr_node::explicit_cast_expr_node::cxx_named_cast_expr_node::CXXAddrspaceCastExpr),
  CXXBindTemporaryExpr(CXXBindTemporaryExpr),
  CXXBoolLiteralExpr(CXXBoolLiteralExpr),
  CXXConstCastExpr(cast_expr_node::explicit_cast_expr_node::cxx_named_cast_expr_node::CXXConstCastExpr),
  CXXConstructExpr(cxx_construct_expr_node::CXXConstructExpr),
  CXXDefaultArgExpr(CXXDefaultArgExpr),
  CXXDefaultInitExpr(CXXDefaultInitExpr),
  CXXDeleteExpr(CXXDeleteExpr),
  CXXDependentScopeMemberExpr(CXXDependentScopeMemberExpr),
  CXXDynamicCastExpr(cast_expr_node::explicit_cast_expr_node::cxx_named_cast_expr_node::CXXDynamicCastExpr),
  CXXFoldExpr(CXXFoldExpr),
  CXXFunctionalCastExpr(cast_expr_node::explicit_cast_expr_node::CXXFunctionalCastExpr),
  CXXInheritedCtorInitExpr(CXXInheritedCtorInitExpr),
  CXXMemberCallExpr(call_expr_node::CXXMemberCallExpr),
  CXXNamedCastExpr(cast_expr_node::explicit_cast_expr_node::cxx_named_cast_expr_node::CXXNamedCastExpr),
  CXXNewExpr(CXXNewExpr),
  CXXNoexceptExpr(CXXNoexceptExpr),
  CXXNullPtrLiteralExpr(CXXNullPtrLiteralExpr),
  CXXOperatorCallExpr(call_expr_node::CXXOperatorCallExpr),
  CXXParenListInitExpr(CXXParenListInitExpr),
  CXXPseudoDestructorExpr(CXXPseudoDestructorExpr),
  CXXReinterpretCastExpr(cast_expr_node::explicit_cast_expr_node::cxx_named_cast_expr_node::CXXReinterpretCastExpr),
  CXXRewrittenBinaryOperator(CXXRewrittenBinaryOperator),
  CXXScalarValueInitExpr(CXXScalarValueInitExpr),
  CXXStaticCastExpr(cast_expr_node::explicit_cast_expr_node::cxx_named_cast_expr_node::CXXStaticCastExpr),
  CXXStdInitializerListExpr(CXXStdInitializerListExpr),
  CXXTemporaryObjectExpr(cxx_construct_expr_node::CXXTemporaryObjectExpr),
  CXXThisExpr(CXXThisExpr),
  CXXThrowExpr(CXXThrowExpr),
  CXXTypeidExpr(CXXTypeidExpr),
  CXXUnresolvedConstructExpr(CXXUnresolvedConstructExpr),
  CXXUuidofExpr(CXXUuidofExpr),
  CallExpr(call_expr_node::CallExpr),
  CastExpr(cast_expr_node::CastExpr),
  CharacterLiteral(CharacterLiteral),
  ChooseExpr(ChooseExpr),
  CoawaitExpr(coroutine_suspend_expr_node::CoawaitExpr),
  CompoundAssignOperator(binary_operator_node::CompoundAssignOperator),
  CompoundLiteralExpr(CompoundLiteralExpr),
  ConceptSpecializationExpr(ConceptSpecializationExpr),
  ConditionalOperator(abstract_conditional_operator_node::ConditionalOperator),
  ConstantExpr(full_expr_node::ConstantExpr),
  ConvertVectorExpr(ConvertVectorExpr),
  CoroutineSuspendExpr(coroutine_suspend_expr_node::CoroutineSuspendExpr),
  CoyieldExpr(coroutine_suspend_expr_node::CoyieldExpr),
  DeclRefExpr(DeclRefExpr),
  DependentCoawaitExpr(DependentCoawaitExpr),
  DependentScopeDeclRefExpr(DependentScopeDeclRefExpr),
  DesignatedInitExpr(DesignatedInitExpr),
  DesignatedInitUpdateExpr(DesignatedInitUpdateExpr),
  ExplicitCastExpr(cast_expr_node::explicit_cast_expr_node::ExplicitCastExpr),
  ExprWithCleanups(full_expr_node::ExprWithCleanups),
  ExpressionTraitExpr(ExpressionTraitExpr),
  ExtVectorElementExpr(ExtVectorElementExpr),
  FixedPointLiteral(FixedPointLiteral),
  FloatingLiteral(FloatingLiteral),
  FullExpr(full_expr_node::FullExpr),
  FunctionParmPackExpr(FunctionParmPackExpr),
  GNUNullExpr(GNUNullExpr),
  GenericSelectionExpr(GenericSelectionExpr),
  ImaginaryLiteral(ImaginaryLiteral),
  ImplicitCastExpr(cast_expr_node::ImplicitCastExpr),
  ImplicitValueInitExpr(ImplicitValueInitExpr),
  InitListExpr(InitListExpr),
  IntegerLiteral(IntegerLiteral),
  LambdaExpr(LambdaExpr),
  MSPropertyRefExpr(MSPropertyRefExpr),
  MSPropertySubscriptExpr(MSPropertySubscriptExpr),
  MaterializeTemporaryExpr(MaterializeTemporaryExpr),
  MatrixSubscriptExpr(MatrixSubscriptExpr),
  MemberExpr(MemberExpr),
  NoInitExpr(NoInitExpr),
  ObjCArrayLiteral(ObjCArrayLiteral),
  ObjCAvailabilityCheckExpr(ObjCAvailabilityCheckExpr),
  ObjCBoolLiteralExpr(ObjCBoolLiteralExpr),
  ObjCBoxedExpr(ObjCBoxedExpr),
  ObjCBridgedCastExpr(cast_expr_node::explicit_cast_expr_node::ObjCBridgedCastExpr),
  ObjCDictionaryLiteral(ObjCDictionaryLiteral),
  ObjCEncodeExpr(ObjCEncodeExpr),
  ObjCIndirectCopyRestoreExpr(ObjCIndirectCopyRestoreExpr),
  ObjCIsaExpr(ObjCIsaExpr),
  ObjCIvarRefExpr(ObjCIvarRefExpr),
  ObjCMessageExpr(ObjCMessageExpr),
  ObjCPropertyRefExpr(ObjCPropertyRefExpr),
  ObjCProtocolExpr(ObjCProtocolExpr),
  ObjCSelectorExpr(ObjCSelectorExpr),
  ObjCStringLiteral(ObjCStringLiteral),
  ObjCSubscriptRefExpr(ObjCSubscriptRefExpr),
  OffsetOfExpr(OffsetOfExpr),
  OpaqueValueExpr(OpaqueValueExpr),
  OverloadExpr(overload_expr_node::OverloadExpr),
  PackExpansionExpr(PackExpansionExpr),
  ParenExpr(ParenExpr),
  ParenListExpr(ParenListExpr),
  PredefinedExpr(PredefinedExpr),
  PseudoObjectExpr(PseudoObjectExpr),
  RecoveryExpr(RecoveryExpr),
  RequiresExpr(RequiresExpr),
  SYCLUniqueStableNameExpr(SYCLUniqueStableNameExpr),
  ShuffleVectorExpr(ShuffleVectorExpr),
  SizeOfPackExpr(SizeOfPackExpr),
  SourceLocExpr(SourceLocExpr),
  StmtExpr(StmtExpr),
  StringLiteral(StringLiteral),
  SubstNonTypeTemplateParmExpr(SubstNonTypeTemplateParmExpr),
  SubstNonTypeTemplateParmPackExpr(SubstNonTypeTemplateParmPackExpr),
  TypeTraitExpr(TypeTraitExpr),
  TypoExpr(TypoExpr),
  UnaryExprOrTypeTraitExpr(UnaryExprOrTypeTraitExpr),
  UnaryOperator(UnaryOperator),
  UnresolvedLookupExpr(overload_expr_node::UnresolvedLookupExpr),
  UnresolvedMemberExpr(overload_expr_node::UnresolvedMemberExpr),
  UserDefinedLiteral(call_expr_node::UserDefinedLiteral),
  VAArgExpr(VAArgExpr),
}

