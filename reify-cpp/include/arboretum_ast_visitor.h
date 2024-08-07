#pragma once

#include <clang/AST/RecursiveASTVisitor.h>

#include "arboretum_context.h"

namespace arboretum {

struct ArboretumASTVisitor
    : public clang::RecursiveASTVisitor<ArboretumASTVisitor> {
  ArboretumASTVisitor(ArboretumContext &context) : context_(context) {}

  bool shouldVisitTemplateInstantiations() const { return true; }

  bool shouldVisitImplicitCode() const { return true; }

  //// BEGIN ARBORETUM GENERATED CODE ////
  // Types
  bool VisitAdjustedType(clang::AdjustedType* T);
  bool VisitArrayType(clang::ArrayType* T);
  bool VisitAtomicType(clang::AtomicType* T);
  bool VisitAttributedType(clang::AttributedType* T);
  bool VisitAutoType(clang::AutoType* T);
  bool VisitBTFTagAttributedType(clang::BTFTagAttributedType* T);
  bool VisitBitIntType(clang::BitIntType* T);
  bool VisitBlockPointerType(clang::BlockPointerType* T);
  bool VisitBuiltinType(clang::BuiltinType* T);
  bool VisitComplexType(clang::ComplexType* T);
  bool VisitConstantArrayType(clang::ConstantArrayType* T);
  bool VisitConstantMatrixType(clang::ConstantMatrixType* T);
  bool VisitDecayedType(clang::DecayedType* T);
  bool VisitDecltypeType(clang::DecltypeType* T);
  bool VisitDeducedTemplateSpecializationType(clang::DeducedTemplateSpecializationType* T);
  bool VisitDeducedType(clang::DeducedType* T);
  bool VisitDependentAddressSpaceType(clang::DependentAddressSpaceType* T);
  bool VisitDependentBitIntType(clang::DependentBitIntType* T);
  bool VisitDependentDecltypeType(clang::DependentDecltypeType* T);
  bool VisitDependentNameType(clang::DependentNameType* T);
  bool VisitDependentSizedArrayType(clang::DependentSizedArrayType* T);
  bool VisitDependentSizedExtVectorType(clang::DependentSizedExtVectorType* T);
  bool VisitDependentSizedMatrixType(clang::DependentSizedMatrixType* T);
  bool VisitDependentTemplateSpecializationType(clang::DependentTemplateSpecializationType* T);
  bool VisitDependentTypeOfExprType(clang::DependentTypeOfExprType* T);
  bool VisitDependentUnaryTransformType(clang::DependentUnaryTransformType* T);
  bool VisitDependentVectorType(clang::DependentVectorType* T);
  bool VisitElaboratedType(clang::ElaboratedType* T);
  bool VisitEnumType(clang::EnumType* T);
  bool VisitExtVectorType(clang::ExtVectorType* T);
  bool VisitFunctionNoProtoType(clang::FunctionNoProtoType* T);
  bool VisitFunctionProtoType(clang::FunctionProtoType* T);
  bool VisitFunctionType(clang::FunctionType* T);
  bool VisitIncompleteArrayType(clang::IncompleteArrayType* T);
  bool VisitInjectedClassNameType(clang::InjectedClassNameType* T);
  bool VisitLValueReferenceType(clang::LValueReferenceType* T);
  bool VisitMacroQualifiedType(clang::MacroQualifiedType* T);
  bool VisitMatrixType(clang::MatrixType* T);
  bool VisitMemberPointerType(clang::MemberPointerType* T);
  bool VisitObjCInterfaceType(clang::ObjCInterfaceType* T);
  bool VisitObjCObjectPointerType(clang::ObjCObjectPointerType* T);
  bool VisitObjCObjectType(clang::ObjCObjectType* T);
  bool VisitObjCObjectTypeImpl(clang::ObjCObjectTypeImpl* T);
  bool VisitObjCTypeParamType(clang::ObjCTypeParamType* T);
  bool VisitPackExpansionType(clang::PackExpansionType* T);
  bool VisitParenType(clang::ParenType* T);
  bool VisitPipeType(clang::PipeType* T);
  bool VisitPointerType(clang::PointerType* T);
  bool VisitRValueReferenceType(clang::RValueReferenceType* T);
  bool VisitRecordType(clang::RecordType* T);
  bool VisitReferenceType(clang::ReferenceType* T);
  bool VisitSubstTemplateTypeParmPackType(clang::SubstTemplateTypeParmPackType* T);
  bool VisitSubstTemplateTypeParmType(clang::SubstTemplateTypeParmType* T);
  bool VisitTagType(clang::TagType* T);
  bool VisitTemplateSpecializationType(clang::TemplateSpecializationType* T);
  bool VisitTemplateTypeParmType(clang::TemplateTypeParmType* T);
  bool VisitType(clang::Type* T);
  bool VisitTypeOfExprType(clang::TypeOfExprType* T);
  bool VisitTypeOfType(clang::TypeOfType* T);
  bool VisitTypeWithKeyword(clang::TypeWithKeyword* T);
  bool VisitTypedefType(clang::TypedefType* T);
  bool VisitUnaryTransformType(clang::UnaryTransformType* T);
  bool VisitUnresolvedUsingType(clang::UnresolvedUsingType* T);
  bool VisitUsingType(clang::UsingType* T);
  bool VisitVariableArrayType(clang::VariableArrayType* T);
  bool VisitVectorType(clang::VectorType* T);

  // TypeLocs
  bool VisitAdjustedTypeLoc(clang::AdjustedTypeLoc TL);
   bool VisitArrayTypeLoc(clang::ArrayTypeLoc TL);
   bool VisitAtomicTypeLoc(clang::AtomicTypeLoc TL);
   bool VisitAttributedTypeLoc(clang::AttributedTypeLoc TL);
   bool VisitAutoTypeLoc(clang::AutoTypeLoc TL);
   bool VisitBTFTagAttributedTypeLoc(clang::BTFTagAttributedTypeLoc TL);
   bool VisitBitIntTypeLoc(clang::BitIntTypeLoc TL);
   bool VisitBlockPointerTypeLoc(clang::BlockPointerTypeLoc TL);
   bool VisitBuiltinTypeLoc(clang::BuiltinTypeLoc TL);
   bool VisitComplexTypeLoc(clang::ComplexTypeLoc TL);
   bool VisitConstantArrayTypeLoc(clang::ConstantArrayTypeLoc TL);
   bool VisitConstantMatrixTypeLoc(clang::ConstantMatrixTypeLoc TL);
   bool VisitDecayedTypeLoc(clang::DecayedTypeLoc TL);
   bool VisitDecltypeTypeLoc(clang::DecltypeTypeLoc TL);
   bool VisitDeducedTemplateSpecializationTypeLoc(clang::DeducedTemplateSpecializationTypeLoc TL);
   bool VisitDeducedTypeLoc(clang::DeducedTypeLoc TL);
   bool VisitDependentAddressSpaceTypeLoc(clang::DependentAddressSpaceTypeLoc TL);
   bool VisitDependentBitIntTypeLoc(clang::DependentBitIntTypeLoc TL);
   bool VisitDependentNameTypeLoc(clang::DependentNameTypeLoc TL);
   bool VisitDependentSizedArrayTypeLoc(clang::DependentSizedArrayTypeLoc TL);
   bool VisitDependentSizedExtVectorTypeLoc(clang::DependentSizedExtVectorTypeLoc TL);
   bool VisitDependentSizedMatrixTypeLoc(clang::DependentSizedMatrixTypeLoc TL);
   bool VisitDependentTemplateSpecializationTypeLoc(clang::DependentTemplateSpecializationTypeLoc TL);
   bool VisitDependentVectorTypeLoc(clang::DependentVectorTypeLoc TL);
   bool VisitElaboratedTypeLoc(clang::ElaboratedTypeLoc TL);
   bool VisitEnumTypeLoc(clang::EnumTypeLoc TL);
   bool VisitExtVectorTypeLoc(clang::ExtVectorTypeLoc TL);
   bool VisitFunctionNoProtoTypeLoc(clang::FunctionNoProtoTypeLoc TL);
   bool VisitFunctionProtoTypeLoc(clang::FunctionProtoTypeLoc TL);
   bool VisitFunctionTypeLoc(clang::FunctionTypeLoc TL);
   bool VisitIncompleteArrayTypeLoc(clang::IncompleteArrayTypeLoc TL);
   bool VisitInjectedClassNameTypeLoc(clang::InjectedClassNameTypeLoc TL);
   bool VisitLValueReferenceTypeLoc(clang::LValueReferenceTypeLoc TL);
   bool VisitMacroQualifiedTypeLoc(clang::MacroQualifiedTypeLoc TL);
   bool VisitMatrixTypeLoc(clang::MatrixTypeLoc TL);
   bool VisitMemberPointerTypeLoc(clang::MemberPointerTypeLoc TL);
   bool VisitObjCInterfaceTypeLoc(clang::ObjCInterfaceTypeLoc TL);
   bool VisitObjCObjectPointerTypeLoc(clang::ObjCObjectPointerTypeLoc TL);
   bool VisitObjCObjectTypeLoc(clang::ObjCObjectTypeLoc TL);
   bool VisitObjCTypeParamTypeLoc(clang::ObjCTypeParamTypeLoc TL);
   bool VisitPackExpansionTypeLoc(clang::PackExpansionTypeLoc TL);
   bool VisitParenTypeLoc(clang::ParenTypeLoc TL);
   bool VisitPipeTypeLoc(clang::PipeTypeLoc TL);
   bool VisitPointerTypeLoc(clang::PointerTypeLoc TL);
   bool VisitQualifiedTypeLoc(clang::QualifiedTypeLoc TL);
   bool VisitRValueReferenceTypeLoc(clang::RValueReferenceTypeLoc TL);
   bool VisitRecordTypeLoc(clang::RecordTypeLoc TL);
   bool VisitReferenceTypeLoc(clang::ReferenceTypeLoc TL);
   bool VisitSubstTemplateTypeParmPackTypeLoc(clang::SubstTemplateTypeParmPackTypeLoc TL);
   bool VisitSubstTemplateTypeParmTypeLoc(clang::SubstTemplateTypeParmTypeLoc TL);
   bool VisitTagTypeLoc(clang::TagTypeLoc TL);
   bool VisitTemplateSpecializationTypeLoc(clang::TemplateSpecializationTypeLoc TL);
   bool VisitTemplateTypeParmTypeLoc(clang::TemplateTypeParmTypeLoc TL);
   bool VisitTypeLoc(clang::TypeLoc TL);
   bool VisitTypeOfExprTypeLoc(clang::TypeOfExprTypeLoc TL);
   bool VisitTypeOfTypeLoc(clang::TypeOfTypeLoc TL);
   bool VisitTypeSpecTypeLoc(clang::TypeSpecTypeLoc TL);
   bool VisitTypedefTypeLoc(clang::TypedefTypeLoc TL);
   bool VisitUnaryTransformTypeLoc(clang::UnaryTransformTypeLoc TL);
   bool VisitUnqualTypeLoc(clang::UnqualTypeLoc TL);
   bool VisitUnresolvedUsingTypeLoc(clang::UnresolvedUsingTypeLoc TL);
   bool VisitUsingTypeLoc(clang::UsingTypeLoc TL);
   bool VisitVariableArrayTypeLoc(clang::VariableArrayTypeLoc TL);
   bool VisitVectorTypeLoc(clang::VectorTypeLoc TL);
 
  // Decls
  bool VisitAccessSpecDecl(clang::AccessSpecDecl* D);
  bool VisitBaseUsingDecl(clang::BaseUsingDecl* D);
  bool VisitBindingDecl(clang::BindingDecl* D);
  bool VisitBlockDecl(clang::BlockDecl* D);
  bool VisitBuiltinTemplateDecl(clang::BuiltinTemplateDecl* D);
  bool VisitCXXConstructorDecl(clang::CXXConstructorDecl* D);
  bool VisitCXXConversionDecl(clang::CXXConversionDecl* D);
  bool VisitCXXDeductionGuideDecl(clang::CXXDeductionGuideDecl* D);
  bool VisitCXXDestructorDecl(clang::CXXDestructorDecl* D);
  bool VisitCXXMethodDecl(clang::CXXMethodDecl* D);
  bool VisitCXXRecordDecl(clang::CXXRecordDecl* D);
  bool VisitCapturedDecl(clang::CapturedDecl* D);
  bool VisitClassTemplateDecl(clang::ClassTemplateDecl* D);
  bool VisitClassTemplatePartialSpecializationDecl(clang::ClassTemplatePartialSpecializationDecl* D);
  bool VisitClassTemplateSpecializationDecl(clang::ClassTemplateSpecializationDecl* D);
  bool VisitConceptDecl(clang::ConceptDecl* D);
  bool VisitConstructorUsingShadowDecl(clang::ConstructorUsingShadowDecl* D);
  bool VisitDecl(clang::Decl* D);
  bool VisitDeclaratorDecl(clang::DeclaratorDecl* D);
  bool VisitDecompositionDecl(clang::DecompositionDecl* D);
  bool VisitEmptyDecl(clang::EmptyDecl* D);
  bool VisitEnumConstantDecl(clang::EnumConstantDecl* D);
  bool VisitEnumDecl(clang::EnumDecl* D);
  bool VisitExportDecl(clang::ExportDecl* D);
  bool VisitExternCContextDecl(clang::ExternCContextDecl* D);
  bool VisitFieldDecl(clang::FieldDecl* D);
  bool VisitFileScopeAsmDecl(clang::FileScopeAsmDecl* D);
  bool VisitFriendDecl(clang::FriendDecl* D);
  bool VisitFriendTemplateDecl(clang::FriendTemplateDecl* D);
  bool VisitFunctionDecl(clang::FunctionDecl* D);
  bool VisitFunctionTemplateDecl(clang::FunctionTemplateDecl* D);
  bool VisitHLSLBufferDecl(clang::HLSLBufferDecl* D);
  bool VisitImplicitConceptSpecializationDecl(clang::ImplicitConceptSpecializationDecl* D);
  bool VisitImplicitParamDecl(clang::ImplicitParamDecl* D);
  bool VisitImportDecl(clang::ImportDecl* D);
  bool VisitIndirectFieldDecl(clang::IndirectFieldDecl* D);
  bool VisitLabelDecl(clang::LabelDecl* D);
  bool VisitLifetimeExtendedTemporaryDecl(clang::LifetimeExtendedTemporaryDecl* D);
  bool VisitLinkageSpecDecl(clang::LinkageSpecDecl* D);
  bool VisitMSGuidDecl(clang::MSGuidDecl* D);
  bool VisitMSPropertyDecl(clang::MSPropertyDecl* D);
  bool VisitNamedDecl(clang::NamedDecl* D);
  bool VisitNamespaceAliasDecl(clang::NamespaceAliasDecl* D);
  bool VisitNamespaceDecl(clang::NamespaceDecl* D);
  bool VisitNonTypeTemplateParmDecl(clang::NonTypeTemplateParmDecl* D);
  bool VisitOMPAllocateDecl(clang::OMPAllocateDecl* D);
  bool VisitOMPCapturedExprDecl(clang::OMPCapturedExprDecl* D);
  bool VisitOMPDeclareMapperDecl(clang::OMPDeclareMapperDecl* D);
  bool VisitOMPDeclareReductionDecl(clang::OMPDeclareReductionDecl* D);
  bool VisitOMPRequiresDecl(clang::OMPRequiresDecl* D);
  bool VisitOMPThreadPrivateDecl(clang::OMPThreadPrivateDecl* D);
  bool VisitObjCAtDefsFieldDecl(clang::ObjCAtDefsFieldDecl* D);
  bool VisitObjCCategoryDecl(clang::ObjCCategoryDecl* D);
  bool VisitObjCCategoryImplDecl(clang::ObjCCategoryImplDecl* D);
  bool VisitObjCCompatibleAliasDecl(clang::ObjCCompatibleAliasDecl* D);
  bool VisitObjCContainerDecl(clang::ObjCContainerDecl* D);
  bool VisitObjCImplDecl(clang::ObjCImplDecl* D);
  bool VisitObjCImplementationDecl(clang::ObjCImplementationDecl* D);
  bool VisitObjCInterfaceDecl(clang::ObjCInterfaceDecl* D);
  bool VisitObjCIvarDecl(clang::ObjCIvarDecl* D);
  bool VisitObjCMethodDecl(clang::ObjCMethodDecl* D);
  bool VisitObjCPropertyDecl(clang::ObjCPropertyDecl* D);
  bool VisitObjCPropertyImplDecl(clang::ObjCPropertyImplDecl* D);
  bool VisitObjCProtocolDecl(clang::ObjCProtocolDecl* D);
  bool VisitObjCTypeParamDecl(clang::ObjCTypeParamDecl* D);
  bool VisitParmVarDecl(clang::ParmVarDecl* D);
  bool VisitPragmaCommentDecl(clang::PragmaCommentDecl* D);
  bool VisitPragmaDetectMismatchDecl(clang::PragmaDetectMismatchDecl* D);
  bool VisitRecordDecl(clang::RecordDecl* D);
  bool VisitRedeclarableTemplateDecl(clang::RedeclarableTemplateDecl* D);
  bool VisitRequiresExprBodyDecl(clang::RequiresExprBodyDecl* D);
  bool VisitStaticAssertDecl(clang::StaticAssertDecl* D);
  bool VisitTagDecl(clang::TagDecl* D);
  bool VisitTemplateDecl(clang::TemplateDecl* D);
  bool VisitTemplateParamObjectDecl(clang::TemplateParamObjectDecl* D);
  bool VisitTemplateTemplateParmDecl(clang::TemplateTemplateParmDecl* D);
  bool VisitTemplateTypeParmDecl(clang::TemplateTypeParmDecl* D);
  bool VisitTopLevelStmtDecl(clang::TopLevelStmtDecl* D);
  bool VisitTranslationUnitDecl(clang::TranslationUnitDecl* D);
  bool VisitTypeAliasDecl(clang::TypeAliasDecl* D);
  bool VisitTypeAliasTemplateDecl(clang::TypeAliasTemplateDecl* D);
  bool VisitTypeDecl(clang::TypeDecl* D);
  bool VisitTypedefDecl(clang::TypedefDecl* D);
  bool VisitTypedefNameDecl(clang::TypedefNameDecl* D);
  bool VisitUnnamedGlobalConstantDecl(clang::UnnamedGlobalConstantDecl* D);
  bool VisitUnresolvedUsingIfExistsDecl(clang::UnresolvedUsingIfExistsDecl* D);
  bool VisitUnresolvedUsingTypenameDecl(clang::UnresolvedUsingTypenameDecl* D);
  bool VisitUnresolvedUsingValueDecl(clang::UnresolvedUsingValueDecl* D);
  bool VisitUsingDecl(clang::UsingDecl* D);
  bool VisitUsingDirectiveDecl(clang::UsingDirectiveDecl* D);
  bool VisitUsingEnumDecl(clang::UsingEnumDecl* D);
  bool VisitUsingPackDecl(clang::UsingPackDecl* D);
  bool VisitUsingShadowDecl(clang::UsingShadowDecl* D);
  bool VisitValueDecl(clang::ValueDecl* D);
  bool VisitVarDecl(clang::VarDecl* D);
  bool VisitVarTemplateDecl(clang::VarTemplateDecl* D);
  bool VisitVarTemplatePartialSpecializationDecl(clang::VarTemplatePartialSpecializationDecl* D);
  bool VisitVarTemplateSpecializationDecl(clang::VarTemplateSpecializationDecl* D);

  // Stmts
  bool VisitAbstractConditionalOperator(clang::AbstractConditionalOperator* S);
  bool VisitAddrLabelExpr(clang::AddrLabelExpr* S);
  bool VisitArrayInitIndexExpr(clang::ArrayInitIndexExpr* S);
  bool VisitArrayInitLoopExpr(clang::ArrayInitLoopExpr* S);
  bool VisitArraySubscriptExpr(clang::ArraySubscriptExpr* S);
  bool VisitArrayTypeTraitExpr(clang::ArrayTypeTraitExpr* S);
  bool VisitAsTypeExpr(clang::AsTypeExpr* S);
  bool VisitAsmStmt(clang::AsmStmt* S);
  bool VisitAtomicExpr(clang::AtomicExpr* S);
  bool VisitAttributedStmt(clang::AttributedStmt* S);
  bool VisitBinaryConditionalOperator(clang::BinaryConditionalOperator* S);
  bool VisitBinaryOperator(clang::BinaryOperator* S);
  bool VisitBlockExpr(clang::BlockExpr* S);
  bool VisitBreakStmt(clang::BreakStmt* S);
  bool VisitBuiltinBitCastExpr(clang::BuiltinBitCastExpr* S);
  bool VisitCStyleCastExpr(clang::CStyleCastExpr* S);
  bool VisitCUDAKernelCallExpr(clang::CUDAKernelCallExpr* S);
  bool VisitCXXAddrspaceCastExpr(clang::CXXAddrspaceCastExpr* S);
  bool VisitCXXBindTemporaryExpr(clang::CXXBindTemporaryExpr* S);
  bool VisitCXXBoolLiteralExpr(clang::CXXBoolLiteralExpr* S);
  bool VisitCXXCatchStmt(clang::CXXCatchStmt* S);
  bool VisitCXXConstCastExpr(clang::CXXConstCastExpr* S);
  bool VisitCXXConstructExpr(clang::CXXConstructExpr* S);
  bool VisitCXXDefaultArgExpr(clang::CXXDefaultArgExpr* S);
  bool VisitCXXDefaultInitExpr(clang::CXXDefaultInitExpr* S);
  bool VisitCXXDeleteExpr(clang::CXXDeleteExpr* S);
  bool VisitCXXDependentScopeMemberExpr(clang::CXXDependentScopeMemberExpr* S);
  bool VisitCXXDynamicCastExpr(clang::CXXDynamicCastExpr* S);
  bool VisitCXXFoldExpr(clang::CXXFoldExpr* S);
  bool VisitCXXForRangeStmt(clang::CXXForRangeStmt* S);
  bool VisitCXXFunctionalCastExpr(clang::CXXFunctionalCastExpr* S);
  bool VisitCXXInheritedCtorInitExpr(clang::CXXInheritedCtorInitExpr* S);
  bool VisitCXXMemberCallExpr(clang::CXXMemberCallExpr* S);
  bool VisitCXXNamedCastExpr(clang::CXXNamedCastExpr* S);
  bool VisitCXXNewExpr(clang::CXXNewExpr* S);
  bool VisitCXXNoexceptExpr(clang::CXXNoexceptExpr* S);
  bool VisitCXXNullPtrLiteralExpr(clang::CXXNullPtrLiteralExpr* S);
  bool VisitCXXOperatorCallExpr(clang::CXXOperatorCallExpr* S);
  bool VisitCXXParenListInitExpr(clang::CXXParenListInitExpr* S);
  bool VisitCXXPseudoDestructorExpr(clang::CXXPseudoDestructorExpr* S);
  bool VisitCXXReinterpretCastExpr(clang::CXXReinterpretCastExpr* S);
  bool VisitCXXRewrittenBinaryOperator(clang::CXXRewrittenBinaryOperator* S);
  bool VisitCXXScalarValueInitExpr(clang::CXXScalarValueInitExpr* S);
  bool VisitCXXStaticCastExpr(clang::CXXStaticCastExpr* S);
  bool VisitCXXStdInitializerListExpr(clang::CXXStdInitializerListExpr* S);
  bool VisitCXXTemporaryObjectExpr(clang::CXXTemporaryObjectExpr* S);
  bool VisitCXXThisExpr(clang::CXXThisExpr* S);
  bool VisitCXXThrowExpr(clang::CXXThrowExpr* S);
  bool VisitCXXTryStmt(clang::CXXTryStmt* S);
  bool VisitCXXTypeidExpr(clang::CXXTypeidExpr* S);
  bool VisitCXXUnresolvedConstructExpr(clang::CXXUnresolvedConstructExpr* S);
  bool VisitCXXUuidofExpr(clang::CXXUuidofExpr* S);
  bool VisitCallExpr(clang::CallExpr* S);
  bool VisitCapturedStmt(clang::CapturedStmt* S);
  bool VisitCaseStmt(clang::CaseStmt* S);
  bool VisitCastExpr(clang::CastExpr* S);
  bool VisitCharacterLiteral(clang::CharacterLiteral* S);
  bool VisitChooseExpr(clang::ChooseExpr* S);
  bool VisitCoawaitExpr(clang::CoawaitExpr* S);
  bool VisitCompoundAssignOperator(clang::CompoundAssignOperator* S);
  bool VisitCompoundLiteralExpr(clang::CompoundLiteralExpr* S);
  bool VisitCompoundStmt(clang::CompoundStmt* S);
  bool VisitConceptSpecializationExpr(clang::ConceptSpecializationExpr* S);
  bool VisitConditionalOperator(clang::ConditionalOperator* S);
  bool VisitConstantExpr(clang::ConstantExpr* S);
  bool VisitContinueStmt(clang::ContinueStmt* S);
  bool VisitConvertVectorExpr(clang::ConvertVectorExpr* S);
  bool VisitCoreturnStmt(clang::CoreturnStmt* S);
  bool VisitCoroutineBodyStmt(clang::CoroutineBodyStmt* S);
  bool VisitCoroutineSuspendExpr(clang::CoroutineSuspendExpr* S);
  bool VisitCoyieldExpr(clang::CoyieldExpr* S);
  bool VisitDeclRefExpr(clang::DeclRefExpr* S);
  bool VisitDeclStmt(clang::DeclStmt* S);
  bool VisitDefaultStmt(clang::DefaultStmt* S);
  bool VisitDependentCoawaitExpr(clang::DependentCoawaitExpr* S);
  bool VisitDependentScopeDeclRefExpr(clang::DependentScopeDeclRefExpr* S);
  bool VisitDesignatedInitExpr(clang::DesignatedInitExpr* S);
  bool VisitDesignatedInitUpdateExpr(clang::DesignatedInitUpdateExpr* S);
  bool VisitDoStmt(clang::DoStmt* S);
  bool VisitExplicitCastExpr(clang::ExplicitCastExpr* S);
  bool VisitExpr(clang::Expr* S);
  bool VisitExprWithCleanups(clang::ExprWithCleanups* S);
  bool VisitExpressionTraitExpr(clang::ExpressionTraitExpr* S);
  bool VisitExtVectorElementExpr(clang::ExtVectorElementExpr* S);
  bool VisitFixedPointLiteral(clang::FixedPointLiteral* S);
  bool VisitFloatingLiteral(clang::FloatingLiteral* S);
  bool VisitForStmt(clang::ForStmt* S);
  bool VisitFullExpr(clang::FullExpr* S);
  bool VisitFunctionParmPackExpr(clang::FunctionParmPackExpr* S);
  bool VisitGCCAsmStmt(clang::GCCAsmStmt* S);
  bool VisitGNUNullExpr(clang::GNUNullExpr* S);
  bool VisitGenericSelectionExpr(clang::GenericSelectionExpr* S);
  bool VisitGotoStmt(clang::GotoStmt* S);
  bool VisitIfStmt(clang::IfStmt* S);
  bool VisitImaginaryLiteral(clang::ImaginaryLiteral* S);
  bool VisitImplicitCastExpr(clang::ImplicitCastExpr* S);
  bool VisitImplicitValueInitExpr(clang::ImplicitValueInitExpr* S);
  bool VisitIndirectGotoStmt(clang::IndirectGotoStmt* S);
  bool VisitInitListExpr(clang::InitListExpr* S);
  bool VisitIntegerLiteral(clang::IntegerLiteral* S);
  bool VisitLabelStmt(clang::LabelStmt* S);
  bool VisitLambdaExpr(clang::LambdaExpr* S);
  bool VisitMSAsmStmt(clang::MSAsmStmt* S);
  bool VisitMSDependentExistsStmt(clang::MSDependentExistsStmt* S);
  bool VisitMSPropertyRefExpr(clang::MSPropertyRefExpr* S);
  bool VisitMSPropertySubscriptExpr(clang::MSPropertySubscriptExpr* S);
  bool VisitMaterializeTemporaryExpr(clang::MaterializeTemporaryExpr* S);
  bool VisitMatrixSubscriptExpr(clang::MatrixSubscriptExpr* S);
  bool VisitMemberExpr(clang::MemberExpr* S);
  bool VisitNoInitExpr(clang::NoInitExpr* S);
  bool VisitNullStmt(clang::NullStmt* S);
  bool VisitOMPArraySectionExpr(clang::OMPArraySectionExpr* S);
  bool VisitOMPArrayShapingExpr(clang::OMPArrayShapingExpr* S);
  bool VisitOMPAtomicDirective(clang::OMPAtomicDirective* S);
  bool VisitOMPBarrierDirective(clang::OMPBarrierDirective* S);
  bool VisitOMPCancelDirective(clang::OMPCancelDirective* S);
  bool VisitOMPCancellationPointDirective(clang::OMPCancellationPointDirective* S);
  bool VisitOMPCanonicalLoop(clang::OMPCanonicalLoop* S);
  bool VisitOMPCriticalDirective(clang::OMPCriticalDirective* S);
  bool VisitOMPDepobjDirective(clang::OMPDepobjDirective* S);
  bool VisitOMPDispatchDirective(clang::OMPDispatchDirective* S);
  bool VisitOMPDistributeDirective(clang::OMPDistributeDirective* S);
  bool VisitOMPDistributeParallelForDirective(clang::OMPDistributeParallelForDirective* S);
  bool VisitOMPDistributeParallelForSimdDirective(clang::OMPDistributeParallelForSimdDirective* S);
  bool VisitOMPDistributeSimdDirective(clang::OMPDistributeSimdDirective* S);
  bool VisitOMPErrorDirective(clang::OMPErrorDirective* S);
  bool VisitOMPExecutableDirective(clang::OMPExecutableDirective* S);
  bool VisitOMPFlushDirective(clang::OMPFlushDirective* S);
  bool VisitOMPForDirective(clang::OMPForDirective* S);
  bool VisitOMPForSimdDirective(clang::OMPForSimdDirective* S);
  bool VisitOMPGenericLoopDirective(clang::OMPGenericLoopDirective* S);
  bool VisitOMPInteropDirective(clang::OMPInteropDirective* S);
  bool VisitOMPIteratorExpr(clang::OMPIteratorExpr* S);
  bool VisitOMPLoopBasedDirective(clang::OMPLoopBasedDirective* S);
  bool VisitOMPLoopDirective(clang::OMPLoopDirective* S);
  bool VisitOMPLoopTransformationDirective(clang::OMPLoopTransformationDirective* S);
  bool VisitOMPMaskedDirective(clang::OMPMaskedDirective* S);
  bool VisitOMPMaskedTaskLoopDirective(clang::OMPMaskedTaskLoopDirective* S);
  bool VisitOMPMaskedTaskLoopSimdDirective(clang::OMPMaskedTaskLoopSimdDirective* S);
  bool VisitOMPMasterDirective(clang::OMPMasterDirective* S);
  bool VisitOMPMasterTaskLoopDirective(clang::OMPMasterTaskLoopDirective* S);
  bool VisitOMPMasterTaskLoopSimdDirective(clang::OMPMasterTaskLoopSimdDirective* S);
  bool VisitOMPMetaDirective(clang::OMPMetaDirective* S);
  bool VisitOMPOrderedDirective(clang::OMPOrderedDirective* S);
  bool VisitOMPParallelDirective(clang::OMPParallelDirective* S);
  bool VisitOMPParallelForDirective(clang::OMPParallelForDirective* S);
  bool VisitOMPParallelForSimdDirective(clang::OMPParallelForSimdDirective* S);
  bool VisitOMPParallelGenericLoopDirective(clang::OMPParallelGenericLoopDirective* S);
  bool VisitOMPParallelMaskedDirective(clang::OMPParallelMaskedDirective* S);
  bool VisitOMPParallelMaskedTaskLoopDirective(clang::OMPParallelMaskedTaskLoopDirective* S);
  bool VisitOMPParallelMaskedTaskLoopSimdDirective(clang::OMPParallelMaskedTaskLoopSimdDirective* S);
  bool VisitOMPParallelMasterDirective(clang::OMPParallelMasterDirective* S);
  bool VisitOMPParallelMasterTaskLoopDirective(clang::OMPParallelMasterTaskLoopDirective* S);
  bool VisitOMPParallelMasterTaskLoopSimdDirective(clang::OMPParallelMasterTaskLoopSimdDirective* S);
  bool VisitOMPParallelSectionsDirective(clang::OMPParallelSectionsDirective* S);
  bool VisitOMPScanDirective(clang::OMPScanDirective* S);
  bool VisitOMPScopeDirective(clang::OMPScopeDirective* S);
  bool VisitOMPSectionDirective(clang::OMPSectionDirective* S);
  bool VisitOMPSectionsDirective(clang::OMPSectionsDirective* S);
  bool VisitOMPSimdDirective(clang::OMPSimdDirective* S);
  bool VisitOMPSingleDirective(clang::OMPSingleDirective* S);
  bool VisitOMPTargetDataDirective(clang::OMPTargetDataDirective* S);
  bool VisitOMPTargetDirective(clang::OMPTargetDirective* S);
  bool VisitOMPTargetEnterDataDirective(clang::OMPTargetEnterDataDirective* S);
  bool VisitOMPTargetExitDataDirective(clang::OMPTargetExitDataDirective* S);
  bool VisitOMPTargetParallelDirective(clang::OMPTargetParallelDirective* S);
  bool VisitOMPTargetParallelForDirective(clang::OMPTargetParallelForDirective* S);
  bool VisitOMPTargetParallelForSimdDirective(clang::OMPTargetParallelForSimdDirective* S);
  bool VisitOMPTargetParallelGenericLoopDirective(clang::OMPTargetParallelGenericLoopDirective* S);
  bool VisitOMPTargetSimdDirective(clang::OMPTargetSimdDirective* S);
  bool VisitOMPTargetTeamsDirective(clang::OMPTargetTeamsDirective* S);
  bool VisitOMPTargetTeamsDistributeDirective(clang::OMPTargetTeamsDistributeDirective* S);
  bool VisitOMPTargetTeamsDistributeParallelForDirective(clang::OMPTargetTeamsDistributeParallelForDirective* S);
  bool VisitOMPTargetTeamsDistributeParallelForSimdDirective(clang::OMPTargetTeamsDistributeParallelForSimdDirective* S);
  bool VisitOMPTargetTeamsDistributeSimdDirective(clang::OMPTargetTeamsDistributeSimdDirective* S);
  bool VisitOMPTargetTeamsGenericLoopDirective(clang::OMPTargetTeamsGenericLoopDirective* S);
  bool VisitOMPTargetUpdateDirective(clang::OMPTargetUpdateDirective* S);
  bool VisitOMPTaskDirective(clang::OMPTaskDirective* S);
  bool VisitOMPTaskLoopDirective(clang::OMPTaskLoopDirective* S);
  bool VisitOMPTaskLoopSimdDirective(clang::OMPTaskLoopSimdDirective* S);
  bool VisitOMPTaskgroupDirective(clang::OMPTaskgroupDirective* S);
  bool VisitOMPTaskwaitDirective(clang::OMPTaskwaitDirective* S);
  bool VisitOMPTaskyieldDirective(clang::OMPTaskyieldDirective* S);
  bool VisitOMPTeamsDirective(clang::OMPTeamsDirective* S);
  bool VisitOMPTeamsDistributeDirective(clang::OMPTeamsDistributeDirective* S);
  bool VisitOMPTeamsDistributeParallelForDirective(clang::OMPTeamsDistributeParallelForDirective* S);
  bool VisitOMPTeamsDistributeParallelForSimdDirective(clang::OMPTeamsDistributeParallelForSimdDirective* S);
  bool VisitOMPTeamsDistributeSimdDirective(clang::OMPTeamsDistributeSimdDirective* S);
  bool VisitOMPTeamsGenericLoopDirective(clang::OMPTeamsGenericLoopDirective* S);
  bool VisitOMPTileDirective(clang::OMPTileDirective* S);
  bool VisitOMPUnrollDirective(clang::OMPUnrollDirective* S);
  bool VisitObjCArrayLiteral(clang::ObjCArrayLiteral* S);
  bool VisitObjCAtCatchStmt(clang::ObjCAtCatchStmt* S);
  bool VisitObjCAtFinallyStmt(clang::ObjCAtFinallyStmt* S);
  bool VisitObjCAtSynchronizedStmt(clang::ObjCAtSynchronizedStmt* S);
  bool VisitObjCAtThrowStmt(clang::ObjCAtThrowStmt* S);
  bool VisitObjCAtTryStmt(clang::ObjCAtTryStmt* S);
  bool VisitObjCAutoreleasePoolStmt(clang::ObjCAutoreleasePoolStmt* S);
  bool VisitObjCAvailabilityCheckExpr(clang::ObjCAvailabilityCheckExpr* S);
  bool VisitObjCBoolLiteralExpr(clang::ObjCBoolLiteralExpr* S);
  bool VisitObjCBoxedExpr(clang::ObjCBoxedExpr* S);
  bool VisitObjCBridgedCastExpr(clang::ObjCBridgedCastExpr* S);
  bool VisitObjCDictionaryLiteral(clang::ObjCDictionaryLiteral* S);
  bool VisitObjCEncodeExpr(clang::ObjCEncodeExpr* S);
  bool VisitObjCForCollectionStmt(clang::ObjCForCollectionStmt* S);
  bool VisitObjCIndirectCopyRestoreExpr(clang::ObjCIndirectCopyRestoreExpr* S);
  bool VisitObjCIsaExpr(clang::ObjCIsaExpr* S);
  bool VisitObjCIvarRefExpr(clang::ObjCIvarRefExpr* S);
  bool VisitObjCMessageExpr(clang::ObjCMessageExpr* S);
  bool VisitObjCPropertyRefExpr(clang::ObjCPropertyRefExpr* S);
  bool VisitObjCProtocolExpr(clang::ObjCProtocolExpr* S);
  bool VisitObjCSelectorExpr(clang::ObjCSelectorExpr* S);
  bool VisitObjCStringLiteral(clang::ObjCStringLiteral* S);
  bool VisitObjCSubscriptRefExpr(clang::ObjCSubscriptRefExpr* S);
  bool VisitOffsetOfExpr(clang::OffsetOfExpr* S);
  bool VisitOpaqueValueExpr(clang::OpaqueValueExpr* S);
  bool VisitOverloadExpr(clang::OverloadExpr* S);
  bool VisitPackExpansionExpr(clang::PackExpansionExpr* S);
  bool VisitParenExpr(clang::ParenExpr* S);
  bool VisitParenListExpr(clang::ParenListExpr* S);
  bool VisitPredefinedExpr(clang::PredefinedExpr* S);
  bool VisitPseudoObjectExpr(clang::PseudoObjectExpr* S);
  bool VisitRecoveryExpr(clang::RecoveryExpr* S);
  bool VisitRequiresExpr(clang::RequiresExpr* S);
  bool VisitReturnStmt(clang::ReturnStmt* S);
  bool VisitSEHExceptStmt(clang::SEHExceptStmt* S);
  bool VisitSEHFinallyStmt(clang::SEHFinallyStmt* S);
  bool VisitSEHLeaveStmt(clang::SEHLeaveStmt* S);
  bool VisitSEHTryStmt(clang::SEHTryStmt* S);
  bool VisitSYCLUniqueStableNameExpr(clang::SYCLUniqueStableNameExpr* S);
  bool VisitShuffleVectorExpr(clang::ShuffleVectorExpr* S);
  bool VisitSizeOfPackExpr(clang::SizeOfPackExpr* S);
  bool VisitSourceLocExpr(clang::SourceLocExpr* S);
  bool VisitStmt(clang::Stmt* S);
  bool VisitStmtExpr(clang::StmtExpr* S);
  bool VisitStringLiteral(clang::StringLiteral* S);
  bool VisitSubstNonTypeTemplateParmExpr(clang::SubstNonTypeTemplateParmExpr* S);
  bool VisitSubstNonTypeTemplateParmPackExpr(clang::SubstNonTypeTemplateParmPackExpr* S);
  bool VisitSwitchCase(clang::SwitchCase* S);
  bool VisitSwitchStmt(clang::SwitchStmt* S);
  bool VisitTypeTraitExpr(clang::TypeTraitExpr* S);
  bool VisitTypoExpr(clang::TypoExpr* S);
  bool VisitUnaryExprOrTypeTraitExpr(clang::UnaryExprOrTypeTraitExpr* S);
  bool VisitUnaryOperator(clang::UnaryOperator* S);
  bool VisitUnresolvedLookupExpr(clang::UnresolvedLookupExpr* S);
  bool VisitUnresolvedMemberExpr(clang::UnresolvedMemberExpr* S);
  bool VisitUserDefinedLiteral(clang::UserDefinedLiteral* S);
  bool VisitVAArgExpr(clang::VAArgExpr* S);
  bool VisitValueStmt(clang::ValueStmt* S);
  bool VisitWhileStmt(clang::WhileStmt* S);
  ////   END ARBORETUM GENERATED CODE ////

  ArboretumContext &context_;
};

} // namespace arboretum