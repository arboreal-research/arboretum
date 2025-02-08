#include "arboretum_ast_visitor.h"

#include "arboretum_ffi.h"

namespace arboretum {

//// BEGIN ARBORETUM GENERATED CODE ////
// Types
bool ArboretumASTVisitor::VisitAdjustedType(clang::AdjustedType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getOriginalType());
  uint64_t c2 = context_.resolve(D->getAdjustedType());
  bool c3 = D->isSugared();
  uint64_t c4 = context_.resolve(D->desugar());
  arboretum_emit_AdjustedType(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitArrayType(clang::ArrayType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getElementType());
  uint64_t c2 = context_.data_model_.resolve(D->getSizeModifier());
  uint32_t c3 = D->getIndexTypeCVRQualifiers();
  arboretum_emit_ArrayType(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitAtomicType(clang::AtomicType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getValueType());
  bool c2 = D->isSugared();
  uint64_t c3 = context_.resolve(D->desugar());
  arboretum_emit_AtomicType(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitAttributedType(clang::AttributedType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.data_model_.resolve(D->getAttrKind());
  uint64_t c2 = context_.resolve(D->getModifiedType());
  uint64_t c3 = context_.resolve(D->getEquivalentType());
  bool c4 = D->isSugared();
  uint64_t c5 = context_.resolve(D->desugar());
  bool c6 = D->isQualifier();
  bool c7 = D->isMSTypeSpec();
  bool c8 = D->isWebAssemblyFuncrefSpec();
  bool c9 = D->isCallingConv();
  arboretum_emit_AttributedType(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9);
  return true;
}

bool ArboretumASTVisitor::VisitAutoType(clang::AutoType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getTypeConstraintConcept());
  bool c2 = D->isConstrained();
  bool c3 = D->isDecltypeAuto();
  bool c4 = D->isGNUAutoType();
  uint64_t c5 = context_.data_model_.resolve(D->getKeyword());
  arboretum_emit_AutoType(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitBTFTagAttributedType(clang::BTFTagAttributedType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getWrappedType());
  uint64_t c2 = context_.resolve(D->getAttr());
  bool c3 = D->isSugared();
  uint64_t c4 = context_.resolve(D->desugar());
  arboretum_emit_BTFTagAttributedType(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitBitIntType(clang::BitIntType* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isUnsigned();
  bool c2 = D->isSigned();
  uint32_t c3 = D->getNumBits();
  bool c4 = D->isSugared();
  uint64_t c5 = context_.resolve(D->desugar());
  arboretum_emit_BitIntType(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitBlockPointerType(clang::BlockPointerType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getPointeeType());
  bool c2 = D->isSugared();
  uint64_t c3 = context_.resolve(D->desugar());
  arboretum_emit_BlockPointerType(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitBuiltinType(clang::BuiltinType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.data_model_.resolve(D->getKind());
  bool c2 = D->isSugared();
  uint64_t c3 = context_.resolve(D->desugar());
  bool c4 = D->isInteger();
  bool c5 = D->isSignedInteger();
  bool c6 = D->isUnsignedInteger();
  bool c7 = D->isFloatingPoint();
  bool c8 = D->isSVEBool();
  bool c9 = D->isSVECount();
  bool c10 = D->isPlaceholderType();
  bool c11 = D->isNonOverloadPlaceholderType();
  arboretum_emit_BuiltinType(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11);
  return true;
}

bool ArboretumASTVisitor::VisitComplexType(clang::ComplexType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getElementType());
  bool c2 = D->isSugared();
  uint64_t c3 = context_.resolve(D->desugar());
  arboretum_emit_ComplexType(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitConstantArrayType(clang::ConstantArrayType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getSizeExpr());
  bool c2 = D->isSugared();
  uint64_t c3 = context_.resolve(D->desugar());
  arboretum_emit_ConstantArrayType(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitConstantMatrixType(clang::ConstantMatrixType* D) {
  uint64_t c0 = context_.resolve(D);
  uint32_t c1 = D->getNumRows();
  uint32_t c2 = D->getNumColumns();
  uint32_t c3 = D->getNumElementsFlattened();
  arboretum_emit_ConstantMatrixType(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitDecayedType(clang::DecayedType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getDecayedType());
  uint64_t c2 = context_.resolve(D->getPointeeType());
  arboretum_emit_DecayedType(c0, c1, c2);
  return true;
}

bool ArboretumASTVisitor::VisitDecltypeType(clang::DecltypeType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getUnderlyingExpr());
  uint64_t c2 = context_.resolve(D->getUnderlyingType());
  uint64_t c3 = context_.resolve(D->desugar());
  bool c4 = D->isSugared();
  arboretum_emit_DecltypeType(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitDeducedTemplateSpecializationType(clang::DeducedTemplateSpecializationType* D) {
  uint64_t c0 = context_.resolve(D);
  arboretum_emit_DeducedTemplateSpecializationType(c0);
  return true;
}

bool ArboretumASTVisitor::VisitDeducedType(clang::DeducedType* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isSugared();
  uint64_t c2 = context_.resolve(D->desugar());
  uint64_t c3 = context_.resolve(D->getDeducedType());
  bool c4 = D->isDeduced();
  arboretum_emit_DeducedType(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitDependentAddressSpaceType(clang::DependentAddressSpaceType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getAddrSpaceExpr());
  uint64_t c2 = context_.resolve(D->getPointeeType());
  uint64_t c3 = context_.source_model_.resolve(D->getAttributeLoc());
  bool c4 = D->isSugared();
  uint64_t c5 = context_.resolve(D->desugar());
  arboretum_emit_DependentAddressSpaceType(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitDependentBitIntType(clang::DependentBitIntType* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isUnsigned();
  bool c2 = D->isSigned();
  uint64_t c3 = context_.resolve(D->getNumBitsExpr());
  bool c4 = D->isSugared();
  uint64_t c5 = context_.resolve(D->desugar());
  arboretum_emit_DependentBitIntType(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitDependentDecltypeType(clang::DependentDecltypeType* D) {
  uint64_t c0 = context_.resolve(D);
  arboretum_emit_DependentDecltypeType(c0);
  return true;
}

bool ArboretumASTVisitor::VisitDependentNameType(clang::DependentNameType* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isSugared();
  uint64_t c2 = context_.resolve(D->desugar());
  arboretum_emit_DependentNameType(c0, c1, c2);
  return true;
}

bool ArboretumASTVisitor::VisitDependentSizedArrayType(clang::DependentSizedArrayType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getSizeExpr());
  uint64_t c2 = context_.source_model_.resolve(D->getBracketsRange());
  uint64_t c3 = context_.source_model_.resolve(D->getLBracketLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getRBracketLoc());
  bool c5 = D->isSugared();
  uint64_t c6 = context_.resolve(D->desugar());
  arboretum_emit_DependentSizedArrayType(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitDependentSizedExtVectorType(clang::DependentSizedExtVectorType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getSizeExpr());
  uint64_t c2 = context_.resolve(D->getElementType());
  uint64_t c3 = context_.source_model_.resolve(D->getAttributeLoc());
  bool c4 = D->isSugared();
  uint64_t c5 = context_.resolve(D->desugar());
  arboretum_emit_DependentSizedExtVectorType(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitDependentSizedMatrixType(clang::DependentSizedMatrixType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getRowExpr());
  uint64_t c2 = context_.resolve(D->getColumnExpr());
  uint64_t c3 = context_.source_model_.resolve(D->getAttributeLoc());
  arboretum_emit_DependentSizedMatrixType(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitDependentTemplateSpecializationType(clang::DependentTemplateSpecializationType* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isSugared();
  uint64_t c2 = context_.resolve(D->desugar());
  arboretum_emit_DependentTemplateSpecializationType(c0, c1, c2);
  return true;
}

bool ArboretumASTVisitor::VisitDependentTypeOfExprType(clang::DependentTypeOfExprType* D) {
  uint64_t c0 = context_.resolve(D);
  arboretum_emit_DependentTypeOfExprType(c0);
  return true;
}

bool ArboretumASTVisitor::VisitDependentUnaryTransformType(clang::DependentUnaryTransformType* D) {
  uint64_t c0 = context_.resolve(D);
  arboretum_emit_DependentUnaryTransformType(c0);
  return true;
}

bool ArboretumASTVisitor::VisitDependentVectorType(clang::DependentVectorType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getSizeExpr());
  uint64_t c2 = context_.resolve(D->getElementType());
  uint64_t c3 = context_.source_model_.resolve(D->getAttributeLoc());
  uint64_t c4 = context_.data_model_.resolve(D->getVectorKind());
  bool c5 = D->isSugared();
  uint64_t c6 = context_.resolve(D->desugar());
  arboretum_emit_DependentVectorType(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitElaboratedType(clang::ElaboratedType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getNamedType());
  uint64_t c2 = context_.resolve(D->desugar());
  bool c3 = D->isSugared();
  uint64_t c4 = context_.resolve(D->getOwnedTagDecl());
  arboretum_emit_ElaboratedType(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitEnumType(clang::EnumType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getDecl());
  bool c2 = D->isSugared();
  uint64_t c3 = context_.resolve(D->desugar());
  arboretum_emit_EnumType(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitExtVectorType(clang::ExtVectorType* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isSugared();
  uint64_t c2 = context_.resolve(D->desugar());
  arboretum_emit_ExtVectorType(c0, c1, c2);
  return true;
}

bool ArboretumASTVisitor::VisitFunctionNoProtoType(clang::FunctionNoProtoType* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isSugared();
  uint64_t c2 = context_.resolve(D->desugar());
  arboretum_emit_FunctionNoProtoType(c0, c1, c2);
  return true;
}

bool ArboretumASTVisitor::VisitFunctionProtoType(clang::FunctionProtoType* D) {
  uint64_t c0 = context_.resolve(D);
  uint32_t c1 = D->getNumParams();
  uint64_t c2 = context_.data_model_.resolve(D->getExceptionSpecType());
  bool c3 = D->hasExceptionSpec();
  bool c4 = D->hasDynamicExceptionSpec();
  bool c5 = D->hasNoexceptExceptionSpec();
  bool c6 = D->hasDependentExceptionSpec();
  bool c7 = D->hasInstantiationDependentExceptionSpec();
  uint32_t c8 = D->getNumExceptions();
  uint64_t c9 = context_.resolve(D->getNoexceptExpr());
  uint64_t c10 = context_.resolve(D->getExceptionSpecDecl());
  uint64_t c11 = context_.resolve(D->getExceptionSpecTemplate());
  uint64_t c12 = context_.data_model_.resolve(D->canThrow());
  bool c13 = D->isVariadic();
  uint64_t c14 = context_.source_model_.resolve(D->getEllipsisLoc());
  bool c15 = D->isTemplateVariadic();
  bool c16 = D->hasTrailingReturn();
  uint64_t c17 = context_.data_model_.resolve(D->getRefQualifier());
  bool c18 = D->hasExtParameterInfos();
  uint32_t c19 = D->getAArch64SMEAttributes();
  bool c20 = D->isSugared();
  uint64_t c21 = context_.resolve(D->desugar());
  {
    uint64_t idx = 0;
    for (const auto& element : D->getParamTypes()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_FunctionProtoType_getParamTypes(c0, idx, element_id);
    }
  }
  {
    uint64_t idx = 0;
    for (const auto& element : D->param_types()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_FunctionProtoType_param_types(c0, idx, element_id);
    }
  }
  {
    uint64_t idx = 0;
    for (const auto& element : D->exceptions()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_FunctionProtoType_exceptions(c0, idx, element_id);
    }
  }
  arboretum_emit_FunctionProtoType(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19, c20, c21);
  return true;
}

bool ArboretumASTVisitor::VisitFunctionType(clang::FunctionType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getReturnType());
  bool c2 = D->getHasRegParm();
  uint32_t c3 = D->getRegParmType();
  bool c4 = D->getNoReturnAttr();
  bool c5 = D->getCmseNSCallAttr();
  uint64_t c6 = context_.data_model_.resolve(D->getCallConv());
  bool c7 = D->isConst();
  bool c8 = D->isVolatile();
  bool c9 = D->isRestrict();
  arboretum_emit_FunctionType(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9);
  return true;
}

bool ArboretumASTVisitor::VisitIncompleteArrayType(clang::IncompleteArrayType* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isSugared();
  uint64_t c2 = context_.resolve(D->desugar());
  arboretum_emit_IncompleteArrayType(c0, c1, c2);
  return true;
}

bool ArboretumASTVisitor::VisitInjectedClassNameType(clang::InjectedClassNameType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getInjectedSpecializationType());
  uint64_t c2 = context_.resolve(D->getInjectedTST());
  uint64_t c3 = context_.resolve(D->getDecl());
  bool c4 = D->isSugared();
  uint64_t c5 = context_.resolve(D->desugar());
  arboretum_emit_InjectedClassNameType(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitLValueReferenceType(clang::LValueReferenceType* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isSugared();
  uint64_t c2 = context_.resolve(D->desugar());
  arboretum_emit_LValueReferenceType(c0, c1, c2);
  return true;
}

bool ArboretumASTVisitor::VisitMacroQualifiedType(clang::MacroQualifiedType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getUnderlyingType());
  uint64_t c2 = context_.resolve(D->getModifiedType());
  bool c3 = D->isSugared();
  uint64_t c4 = context_.resolve(D->desugar());
  arboretum_emit_MacroQualifiedType(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitMatrixType(clang::MatrixType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getElementType());
  bool c2 = D->isSugared();
  uint64_t c3 = context_.resolve(D->desugar());
  arboretum_emit_MatrixType(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitMemberPointerType(clang::MemberPointerType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getPointeeType());
  bool c2 = D->isMemberFunctionPointer();
  bool c3 = D->isMemberDataPointer();
  uint64_t c4 = context_.resolve(D->getClass());
  bool c5 = D->isSugared();
  uint64_t c6 = context_.resolve(D->desugar());
  arboretum_emit_MemberPointerType(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitObjCInterfaceType(clang::ObjCInterfaceType* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCObjectPointerType(clang::ObjCObjectPointerType* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCObjectType(clang::ObjCObjectType* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCObjectTypeImpl(clang::ObjCObjectTypeImpl* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCTypeParamType(clang::ObjCTypeParamType* D) {
  return true;
}

bool ArboretumASTVisitor::VisitPackExpansionType(clang::PackExpansionType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getPattern());
  bool c2 = D->isSugared();
  uint64_t c3 = context_.resolve(D->desugar());
  arboretum_emit_PackExpansionType(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitParenType(clang::ParenType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getInnerType());
  bool c2 = D->isSugared();
  uint64_t c3 = context_.resolve(D->desugar());
  arboretum_emit_ParenType(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitPipeType(clang::PipeType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getElementType());
  bool c2 = D->isSugared();
  uint64_t c3 = context_.resolve(D->desugar());
  bool c4 = D->isReadOnly();
  arboretum_emit_PipeType(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitPointerType(clang::PointerType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getPointeeType());
  bool c2 = D->isSugared();
  uint64_t c3 = context_.resolve(D->desugar());
  arboretum_emit_PointerType(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitRValueReferenceType(clang::RValueReferenceType* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isSugared();
  uint64_t c2 = context_.resolve(D->desugar());
  arboretum_emit_RValueReferenceType(c0, c1, c2);
  return true;
}

bool ArboretumASTVisitor::VisitRecordType(clang::RecordType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getDecl());
  bool c2 = D->hasConstFields();
  bool c3 = D->isSugared();
  uint64_t c4 = context_.resolve(D->desugar());
  arboretum_emit_RecordType(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitReferenceType(clang::ReferenceType* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isSpelledAsLValue();
  bool c2 = D->isInnerRef();
  uint64_t c3 = context_.resolve(D->getPointeeTypeAsWritten());
  uint64_t c4 = context_.resolve(D->getPointeeType());
  arboretum_emit_ReferenceType(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitSubstTemplateTypeParmPackType(clang::SubstTemplateTypeParmPackType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getAssociatedDecl());
  uint64_t c2 = context_.resolve(D->getReplacedParameter());
  uint32_t c3 = D->getIndex();
  bool c4 = D->getFinal();
  uint32_t c5 = D->getNumArgs();
  bool c6 = D->isSugared();
  uint64_t c7 = context_.resolve(D->desugar());
  arboretum_emit_SubstTemplateTypeParmPackType(c0, c1, c2, c3, c4, c5, c6, c7);
  return true;
}

bool ArboretumASTVisitor::VisitSubstTemplateTypeParmType(clang::SubstTemplateTypeParmType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getReplacementType());
  uint64_t c2 = context_.resolve(D->getAssociatedDecl());
  uint64_t c3 = context_.resolve(D->getReplacedParameter());
  uint32_t c4 = D->getIndex();
  bool c5 = D->isSugared();
  uint64_t c6 = context_.resolve(D->desugar());
  arboretum_emit_SubstTemplateTypeParmType(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitTagType(clang::TagType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getDecl());
  bool c2 = D->isBeingDefined();
  arboretum_emit_TagType(c0, c1, c2);
  return true;
}

bool ArboretumASTVisitor::VisitTemplateSpecializationType(clang::TemplateSpecializationType* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isCurrentInstantiation();
  bool c2 = D->isTypeAlias();
  bool c3 = D->isSugared();
  uint64_t c4 = context_.resolve(D->desugar());
  arboretum_emit_TemplateSpecializationType(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitTemplateTypeParmType(clang::TemplateTypeParmType* D) {
  uint64_t c0 = context_.resolve(D);
  uint32_t c1 = D->getDepth();
  uint32_t c2 = D->getIndex();
  bool c3 = D->isParameterPack();
  uint64_t c4 = context_.resolve(D->getDecl());
  bool c5 = D->isSugared();
  uint64_t c6 = context_.resolve(D->desugar());
  arboretum_emit_TemplateTypeParmType(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitType(clang::Type* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->containsUnexpandedParameterPack();
  uint64_t c2 = context_.resolve(D->getLocallyUnqualifiedSingleStepDesugaredType());
  uint64_t c3 = context_.resolve(D->getAsPlaceholderType());
  uint64_t c4 = context_.data_model_.resolve(D->getObjCARCImplicitLifetime());
  uint64_t c5 = context_.data_model_.resolve(D->getDependence());
  bool c6 = D->containsErrors();
  bool c7 = D->hasSizedVLAType();
  bool c8 = D->hasUnnamedOrLocalType();
  bool c9 = D->canDecayToPointerType();
  bool c10 = D->hasPointerRepresentation();
  bool c11 = D->hasObjCPointerRepresentation();
  bool c12 = D->hasIntegerRepresentation();
  bool c13 = D->hasSignedIntegerRepresentation();
  bool c14 = D->hasUnsignedIntegerRepresentation();
  bool c15 = D->hasFloatingRepresentation();
  uint64_t c16 = context_.resolve(D->getAsStructureType());
  uint64_t c17 = context_.resolve(D->getAsUnionType());
  uint64_t c18 = context_.resolve(D->getAsComplexIntegerType());
  uint64_t c19 = context_.resolve(D->getAsObjCInterfaceType());
  uint64_t c20 = context_.resolve(D->getAsObjCInterfacePointerType());
  uint64_t c21 = context_.resolve(D->getAsObjCQualifiedIdType());
  uint64_t c22 = context_.resolve(D->getAsObjCQualifiedClassType());
  uint64_t c23 = context_.resolve(D->getAsObjCQualifiedInterfaceType());
  uint64_t c24 = context_.resolve(D->getAsCXXRecordDecl());
  uint64_t c25 = context_.resolve(D->getAsRecordDecl());
  uint64_t c26 = context_.resolve(D->getAsTagDecl());
  uint64_t c27 = context_.resolve(D->getPointeeCXXRecordDecl());
  uint64_t c28 = context_.resolve(D->getBaseElementTypeUnsafe());
  uint64_t c29 = context_.resolve(D->getArrayElementTypeNoTypeQual());
  uint64_t c30 = context_.resolve(D->getPointeeOrArrayElementType());
  uint64_t c31 = context_.data_model_.resolve(D->getLinkage());
  uint64_t c32 = context_.data_model_.resolve(D->getVisibility());
  bool c33 = D->acceptsObjCTypeParams();
  uint64_t c34 = context_.resolve(D->getCanonicalTypeInternal());
  arboretum_emit_Type(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19, c20, c21, c22, c23, c24, c25, c26, c27, c28, c29, c30, c31, c32, c33, c34);
  return true;
}

bool ArboretumASTVisitor::VisitTypeOfExprType(clang::TypeOfExprType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getUnderlyingExpr());
  uint64_t c2 = context_.data_model_.resolve(D->getKind());
  uint64_t c3 = context_.resolve(D->desugar());
  bool c4 = D->isSugared();
  arboretum_emit_TypeOfExprType(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitTypeOfType(clang::TypeOfType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getUnmodifiedType());
  uint64_t c2 = context_.resolve(D->desugar());
  bool c3 = D->isSugared();
  uint64_t c4 = context_.data_model_.resolve(D->getKind());
  arboretum_emit_TypeOfType(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitTypeWithKeyword(clang::TypeWithKeyword* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.data_model_.resolve(D->getKeyword());
  arboretum_emit_TypeWithKeyword(c0, c1);
  return true;
}

bool ArboretumASTVisitor::VisitTypedefType(clang::TypedefType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getDecl());
  bool c2 = D->isSugared();
  uint64_t c3 = context_.resolve(D->desugar());
  bool c4 = D->typeMatchesDecl();
  arboretum_emit_TypedefType(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitUnaryTransformType(clang::UnaryTransformType* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isSugared();
  uint64_t c2 = context_.resolve(D->desugar());
  uint64_t c3 = context_.resolve(D->getUnderlyingType());
  uint64_t c4 = context_.resolve(D->getBaseType());
  uint64_t c5 = context_.data_model_.resolve(D->getUTTKind());
  arboretum_emit_UnaryTransformType(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedUsingType(clang::UnresolvedUsingType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getDecl());
  bool c2 = D->isSugared();
  uint64_t c3 = context_.resolve(D->desugar());
  arboretum_emit_UnresolvedUsingType(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitUsingType(clang::UsingType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getFoundDecl());
  uint64_t c2 = context_.resolve(D->getUnderlyingType());
  bool c3 = D->isSugared();
  uint64_t c4 = context_.resolve(D->desugar());
  bool c5 = D->typeMatchesDecl();
  arboretum_emit_UsingType(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitVariableArrayType(clang::VariableArrayType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getSizeExpr());
  uint64_t c2 = context_.source_model_.resolve(D->getBracketsRange());
  uint64_t c3 = context_.source_model_.resolve(D->getLBracketLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getRBracketLoc());
  bool c5 = D->isSugared();
  uint64_t c6 = context_.resolve(D->desugar());
  arboretum_emit_VariableArrayType(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitVectorType(clang::VectorType* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getElementType());
  uint32_t c2 = D->getNumElements();
  bool c3 = D->isSugared();
  uint64_t c4 = context_.resolve(D->desugar());
  uint64_t c5 = context_.data_model_.resolve(D->getVectorKind());
  arboretum_emit_VectorType(c0, c1, c2, c3, c4, c5);
  return true;
}


// TypeLocs
bool ArboretumASTVisitor::VisitAdjustedTypeLoc(clang::AdjustedTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitArrayTypeLoc(clang::ArrayTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitAtomicTypeLoc(clang::AtomicTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitAttributedTypeLoc(clang::AttributedTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitAutoTypeLoc(clang::AutoTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitBTFTagAttributedTypeLoc(clang::BTFTagAttributedTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitBitIntTypeLoc(clang::BitIntTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitBlockPointerTypeLoc(clang::BlockPointerTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitBuiltinTypeLoc(clang::BuiltinTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitComplexTypeLoc(clang::ComplexTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitConstantArrayTypeLoc(clang::ConstantArrayTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitConstantMatrixTypeLoc(clang::ConstantMatrixTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitDecayedTypeLoc(clang::DecayedTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitDecltypeTypeLoc(clang::DecltypeTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitDeducedTemplateSpecializationTypeLoc(clang::DeducedTemplateSpecializationTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitDeducedTypeLoc(clang::DeducedTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitDependentAddressSpaceTypeLoc(clang::DependentAddressSpaceTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitDependentBitIntTypeLoc(clang::DependentBitIntTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitDependentNameTypeLoc(clang::DependentNameTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitDependentSizedArrayTypeLoc(clang::DependentSizedArrayTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitDependentSizedExtVectorTypeLoc(clang::DependentSizedExtVectorTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitDependentSizedMatrixTypeLoc(clang::DependentSizedMatrixTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitDependentTemplateSpecializationTypeLoc(clang::DependentTemplateSpecializationTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitDependentVectorTypeLoc(clang::DependentVectorTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitElaboratedTypeLoc(clang::ElaboratedTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitEnumTypeLoc(clang::EnumTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitExtVectorTypeLoc(clang::ExtVectorTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitFunctionNoProtoTypeLoc(clang::FunctionNoProtoTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitFunctionProtoTypeLoc(clang::FunctionProtoTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitFunctionTypeLoc(clang::FunctionTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitIncompleteArrayTypeLoc(clang::IncompleteArrayTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitInjectedClassNameTypeLoc(clang::InjectedClassNameTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitLValueReferenceTypeLoc(clang::LValueReferenceTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitMacroQualifiedTypeLoc(clang::MacroQualifiedTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitMatrixTypeLoc(clang::MatrixTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitMemberPointerTypeLoc(clang::MemberPointerTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCInterfaceTypeLoc(clang::ObjCInterfaceTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCObjectPointerTypeLoc(clang::ObjCObjectPointerTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCObjectTypeLoc(clang::ObjCObjectTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCTypeParamTypeLoc(clang::ObjCTypeParamTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitPackExpansionTypeLoc(clang::PackExpansionTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitParenTypeLoc(clang::ParenTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitPipeTypeLoc(clang::PipeTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitPointerTypeLoc(clang::PointerTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitQualifiedTypeLoc(clang::QualifiedTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitRValueReferenceTypeLoc(clang::RValueReferenceTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitRecordTypeLoc(clang::RecordTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitReferenceTypeLoc(clang::ReferenceTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitSubstTemplateTypeParmPackTypeLoc(clang::SubstTemplateTypeParmPackTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitSubstTemplateTypeParmTypeLoc(clang::SubstTemplateTypeParmTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitTagTypeLoc(clang::TagTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitTemplateSpecializationTypeLoc(clang::TemplateSpecializationTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitTemplateTypeParmTypeLoc(clang::TemplateTypeParmTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitTypeLoc(clang::TypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitTypeOfExprTypeLoc(clang::TypeOfExprTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitTypeOfTypeLoc(clang::TypeOfTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitTypeSpecTypeLoc(clang::TypeSpecTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitTypedefTypeLoc(clang::TypedefTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitUnaryTransformTypeLoc(clang::UnaryTransformTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitUnqualTypeLoc(clang::UnqualTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedUsingTypeLoc(clang::UnresolvedUsingTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitUsingTypeLoc(clang::UsingTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitVariableArrayTypeLoc(clang::VariableArrayTypeLoc D) {
  return true;
}

bool ArboretumASTVisitor::VisitVectorTypeLoc(clang::VectorTypeLoc D) {
  return true;
}


// Decls
bool ArboretumASTVisitor::VisitAccessSpecDecl(clang::AccessSpecDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getAccessSpecifierLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getColonLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getSourceRange());
  arboretum_emit_AccessSpecDecl(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitBaseUsingDecl(clang::BaseUsingDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint32_t c1 = D->shadow_size();
  {
    uint64_t idx = 0;
    for (const auto& element : D->shadows()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_BaseUsingDecl_shadows(c0, idx, element_id);
      ++idx;
    }
  }
  arboretum_emit_BaseUsingDecl(c0, c1);
  return true;
}

bool ArboretumASTVisitor::VisitBindingDecl(clang::BindingDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getBinding());
  uint64_t c2 = context_.resolve(D->getDecomposedDecl());
  uint64_t c3 = context_.resolve(D->getHoldingVar());
  arboretum_emit_BindingDecl(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitBlockDecl(clang::BlockDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getCaretLocation());
  bool c2 = D->isVariadic();
  uint64_t c3 = context_.resolve(D->getCompoundBody());
  uint64_t c4 = context_.resolve(D->getBody());
  bool c5 = D->param_empty();
  uint64_t c6 = D->param_size();
  uint32_t c7 = D->getNumParams();
  bool c8 = D->hasCaptures();
  uint32_t c9 = D->getNumCaptures();
  bool c10 = D->capturesCXXThis();
  bool c11 = D->blockMissingReturnType();
  bool c12 = D->isConversionFromLambda();
  bool c13 = D->doesNotEscape();
  bool c14 = D->canAvoidCopyToHeap();
  uint32_t c15 = D->getBlockManglingNumber();
  uint64_t c16 = context_.resolve(D->getBlockManglingContextDecl());
  uint64_t c17 = context_.source_model_.resolve(D->getSourceRange());
  {
    uint64_t idx = 0;
    for (const auto& element : D->parameters()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_BlockDecl_parameters(c0, idx, element_id);
    }
  }
  arboretum_emit_BlockDecl(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17);
  return true;
}

bool ArboretumASTVisitor::VisitBuiltinTemplateDecl(clang::BuiltinTemplateDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getSourceRange());
  uint64_t c2 = context_.data_model_.resolve(D->getBuiltinTemplateKind());
  arboretum_emit_BuiltinTemplateDecl(c0, c1, c2);
  return true;
}

bool ArboretumASTVisitor::VisitCXXConstructorDecl(clang::CXXConstructorDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isExplicit();
  uint32_t c2 = D->getNumCtorInitializers();
  bool c3 = D->isDelegatingConstructor();
  bool c4 = D->isDefaultConstructor();
  bool c5 = D->isCopyConstructor();
  bool c6 = D->isMoveConstructor();
  bool c7 = D->isCopyOrMoveConstructor();
  bool c8 = D->isSpecializationCopyingObject();
  bool c9 = D->isInheritingConstructor();
  uint64_t c10 = context_.resolve(D->getCanonicalDecl());
  arboretum_emit_CXXConstructorDecl(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10);
  return true;
}

bool ArboretumASTVisitor::VisitCXXConversionDecl(clang::CXXConversionDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isExplicit();
  uint64_t c2 = context_.resolve(D->getConversionType());
  bool c3 = D->isLambdaToBlockPointerConversion();
  uint64_t c4 = context_.resolve(D->getCanonicalDecl());
  arboretum_emit_CXXConversionDecl(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitCXXDeductionGuideDecl(clang::CXXDeductionGuideDecl* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isExplicit();
  uint64_t c2 = context_.resolve(D->getDeducedTemplate());
  uint64_t c3 = context_.resolve(D->getCorrespondingConstructor());
  uint64_t c4 = context_.data_model_.resolve(D->getDeductionCandidateKind());
  arboretum_emit_CXXDeductionGuideDecl(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitCXXDestructorDecl(clang::CXXDestructorDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getOperatorDelete());
  uint64_t c2 = context_.resolve(D->getOperatorDeleteThisArg());
  uint64_t c3 = context_.resolve(D->getCanonicalDecl());
  arboretum_emit_CXXDestructorDecl(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitCXXMethodDecl(clang::CXXMethodDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isStatic();
  bool c2 = D->isInstance();
  bool c3 = D->isExplicitObjectMemberFunction();
  bool c4 = D->isImplicitObjectMemberFunction();
  bool c5 = D->isConst();
  bool c6 = D->isVolatile();
  bool c7 = D->isVirtual();
  bool c8 = D->isCopyAssignmentOperator();
  bool c9 = D->isMoveAssignmentOperator();
  uint64_t c10 = context_.resolve(D->getCanonicalDecl());
  uint64_t c11 = context_.resolve(D->getMostRecentDecl());
  uint32_t c12 = D->size_overridden_methods();
  uint64_t c13 = context_.resolve(D->getParent());
  uint64_t c14 = context_.resolve(D->getThisType());
  uint64_t c15 = context_.resolve(D->getFunctionObjectParameterReferenceType());
  uint64_t c16 = context_.resolve(D->getFunctionObjectParameterType());
  uint32_t c17 = D->getNumExplicitParams();
  uint64_t c18 = context_.data_model_.resolve(D->getRefQualifier());
  bool c19 = D->hasInlineBody();
  bool c20 = D->isLambdaStaticInvoker();
  {
    uint64_t idx = 0;
    for (const auto& element : D->overridden_methods()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_CXXMethodDecl_overridden_methods(c0, idx, element_id);
      ++idx;
    }
  }
  arboretum_emit_CXXMethodDecl(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19, c20);
  return true;
}

bool ArboretumASTVisitor::VisitCXXRecordDecl(clang::CXXRecordDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getCanonicalDecl());
  uint64_t c2 = context_.resolve(D->getPreviousDecl());
  uint64_t c3 = context_.resolve(D->getMostRecentDecl());
  uint64_t c4 = context_.resolve(D->getDefinition());
  bool c5 = D->hasDefinition();
  bool c6 = D->isDynamicClass();
  bool c7 = D->mayBeDynamicClass();
  bool c8 = D->mayBeNonDynamicClass();
  bool c9 = D->isParsingBaseSpecifiers();
  uint32_t c10 = D->getODRHash();
  uint32_t c11 = D->getNumBases();
  uint32_t c12 = D->getNumVBases();
  bool c13 = D->hasAnyDependentBases();
  bool c14 = D->hasFriends();
  bool c15 = D->hasSimpleCopyConstructor();
  bool c16 = D->hasSimpleMoveConstructor();
  bool c17 = D->hasSimpleCopyAssignment();
  bool c18 = D->hasSimpleMoveAssignment();
  bool c19 = D->hasSimpleDestructor();
  bool c20 = D->hasDefaultConstructor();
  bool c21 = D->needsImplicitDefaultConstructor();
  bool c22 = D->hasUserDeclaredConstructor();
  bool c23 = D->hasUserProvidedDefaultConstructor();
  bool c24 = D->hasUserDeclaredCopyConstructor();
  bool c25 = D->needsImplicitCopyConstructor();
  bool c26 = D->needsOverloadResolutionForCopyConstructor();
  bool c27 = D->implicitCopyConstructorHasConstParam();
  bool c28 = D->hasCopyConstructorWithConstParam();
  bool c29 = D->hasUserDeclaredMoveOperation();
  bool c30 = D->hasUserDeclaredMoveConstructor();
  bool c31 = D->hasMoveConstructor();
  bool c32 = D->needsImplicitMoveConstructor();
  bool c33 = D->needsOverloadResolutionForMoveConstructor();
  bool c34 = D->hasUserDeclaredCopyAssignment();
  bool c35 = D->needsImplicitCopyAssignment();
  bool c36 = D->needsOverloadResolutionForCopyAssignment();
  bool c37 = D->implicitCopyAssignmentHasConstParam();
  bool c38 = D->hasCopyAssignmentWithConstParam();
  bool c39 = D->hasUserDeclaredMoveAssignment();
  bool c40 = D->hasMoveAssignment();
  bool c41 = D->needsImplicitMoveAssignment();
  bool c42 = D->needsOverloadResolutionForMoveAssignment();
  bool c43 = D->hasUserDeclaredDestructor();
  bool c44 = D->needsImplicitDestructor();
  bool c45 = D->needsOverloadResolutionForDestructor();
  bool c46 = D->isLambda();
  bool c47 = D->isGenericLambda();
  bool c48 = D->lambdaIsDefaultConstructibleAndAssignable();
  uint64_t c49 = context_.resolve(D->getLambdaCallOperator());
  uint64_t c50 = context_.resolve(D->getDependentLambdaCallOperator());
  bool c51 = D->isCapturelessLambda();
  bool c52 = D->isAggregate();
  bool c53 = D->hasInClassInitializer();
  bool c54 = D->hasUninitializedReferenceMember();
  bool c55 = D->isPOD();
  bool c56 = D->isCLike();
  bool c57 = D->isEmpty();
  bool c58 = D->hasInitMethod();
  bool c59 = D->hasPrivateFields();
  bool c60 = D->hasProtectedFields();
  bool c61 = D->hasDirectFields();
  bool c62 = D->isPolymorphic();
  bool c63 = D->isAbstract();
  bool c64 = D->isStandardLayout();
  bool c65 = D->isCXX11StandardLayout();
  bool c66 = D->hasMutableFields();
  bool c67 = D->hasVariantMembers();
  bool c68 = D->hasTrivialDefaultConstructor();
  bool c69 = D->hasNonTrivialDefaultConstructor();
  bool c70 = D->hasConstexprNonCopyMoveConstructor();
  bool c71 = D->defaultedDefaultConstructorIsConstexpr();
  bool c72 = D->hasConstexprDefaultConstructor();
  bool c73 = D->hasTrivialCopyConstructor();
  bool c74 = D->hasTrivialCopyConstructorForCall();
  bool c75 = D->hasNonTrivialCopyConstructor();
  bool c76 = D->hasNonTrivialCopyConstructorForCall();
  bool c77 = D->hasTrivialMoveConstructor();
  bool c78 = D->hasTrivialMoveConstructorForCall();
  bool c79 = D->hasNonTrivialMoveConstructor();
  bool c80 = D->hasNonTrivialMoveConstructorForCall();
  bool c81 = D->hasTrivialCopyAssignment();
  bool c82 = D->hasNonTrivialCopyAssignment();
  bool c83 = D->hasTrivialMoveAssignment();
  bool c84 = D->hasNonTrivialMoveAssignment();
  bool c85 = D->defaultedDestructorIsConstexpr();
  bool c86 = D->hasConstexprDestructor();
  bool c87 = D->hasTrivialDestructor();
  bool c88 = D->hasTrivialDestructorForCall();
  bool c89 = D->hasNonTrivialDestructor();
  bool c90 = D->hasNonTrivialDestructorForCall();
  bool c91 = D->allowConstDefaultInit();
  bool c92 = D->hasIrrelevantDestructor();
  bool c93 = D->hasNonLiteralTypeFieldsOrBases();
  bool c94 = D->hasInheritedConstructor();
  bool c95 = D->hasInheritedAssignment();
  bool c96 = D->isTriviallyCopyable();
  bool c97 = D->isTriviallyCopyConstructible();
  bool c98 = D->isTrivial();
  bool c99 = D->isLiteral();
  bool c100 = D->isStructural();
  uint64_t c101 = context_.resolve(D->getInstantiatedFromMemberClass());
  uint64_t c102 = context_.resolve(D->getDescribedClassTemplate());
  uint64_t c103 = context_.data_model_.resolve(D->getTemplateSpecializationKind());
  uint64_t c104 = context_.resolve(D->getTemplateInstantiationPattern());
  uint64_t c105 = context_.resolve(D->getDestructor());
  bool c106 = D->isAnyDestructorNoReturn();
  uint64_t c107 = context_.resolve(D->isLocalClass());
  bool c108 = D->mayBeAbstract();
  bool c109 = D->isEffectivelyFinal();
  uint32_t c110 = D->getDeviceLambdaManglingNumber();
  uint64_t c111 = context_.data_model_.resolve(D->getMSVtorDispMode());
  bool c112 = D->isDependentLambda();
  bool c113 = D->isNeverDependentLambda();
  uint32_t c114 = D->getLambdaDependencyKind();
  {
    uint64_t idx = 0;
    for (const auto& element : D->methods()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_CXXRecordDecl_methods(c0, idx, element_id);
      ++idx;
    }
  }
  {
    uint64_t idx = 0;
    for (const auto& element : D->ctors()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_CXXRecordDecl_ctors(c0, idx, element_id);
      ++idx;
    }
  }
  {
    uint64_t idx = 0;
    for (const auto& element : D->friends()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_CXXRecordDecl_friends(c0, idx, element_id);
      ++idx;
    }
  }
  {
    uint64_t idx = 0;
    for (const auto& element : D->getLambdaExplicitTemplateParameters()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_CXXRecordDecl_getLambdaExplicitTemplateParameters(c0, idx, element_id);
    }
  }
  arboretum_emit_CXXRecordDecl(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19, c20, c21, c22, c23, c24, c25, c26, c27, c28, c29, c30, c31, c32, c33, c34, c35, c36, c37, c38, c39, c40, c41, c42, c43, c44, c45, c46, c47, c48, c49, c50, c51, c52, c53, c54, c55, c56, c57, c58, c59, c60, c61, c62, c63, c64, c65, c66, c67, c68, c69, c70, c71, c72, c73, c74, c75, c76, c77, c78, c79, c80, c81, c82, c83, c84, c85, c86, c87, c88, c89, c90, c91, c92, c93, c94, c95, c96, c97, c98, c99, c100, c101, c102, c103, c104, c105, c106, c107, c108, c109, c110, c111, c112, c113, c114);
  return true;
}

bool ArboretumASTVisitor::VisitCapturedDecl(clang::CapturedDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getBody());
  bool c2 = D->isNothrow();
  uint32_t c3 = D->getNumParams();
  uint64_t c4 = context_.resolve(D->getContextParam());
  uint32_t c5 = D->getContextParamPosition();
  {
    uint64_t idx = 0;
    for (const auto& element : D->parameters()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_CapturedDecl_parameters(c0, idx, element_id);
    }
  }
  arboretum_emit_CapturedDecl(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitClassTemplateDecl(clang::ClassTemplateDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getTemplatedDecl());
  bool c2 = D->isThisDeclarationADefinition();
  uint64_t c3 = context_.resolve(D->getCanonicalDecl());
  uint64_t c4 = context_.resolve(D->getPreviousDecl());
  uint64_t c5 = context_.resolve(D->getMostRecentDecl());
  uint64_t c6 = context_.resolve(D->getInstantiatedFromMemberTemplate());
  arboretum_emit_ClassTemplateDecl(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitClassTemplatePartialSpecializationDecl(clang::ClassTemplatePartialSpecializationDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  uint64_t c0 = context_.resolve(D);
  bool c1 = D->hasAssociatedConstraints();
  uint64_t c2 = context_.resolve(D->getInstantiatedFromMember());
  uint64_t c3 = context_.resolve(D->getInstantiatedFromMemberTemplate());
  uint64_t c4 = context_.resolve(D->getInjectedSpecializationType());
  arboretum_emit_ClassTemplatePartialSpecializationDecl(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitClassTemplateSpecializationDecl(clang::ClassTemplateSpecializationDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getSpecializedTemplate());
  uint64_t c2 = context_.data_model_.resolve(D->getSpecializationKind());
  bool c3 = D->isExplicitSpecialization();
  bool c4 = D->isClassScopeExplicitSpecialization();
  bool c5 = D->isExplicitInstantiationOrSpecialization();
  uint64_t c6 = context_.source_model_.resolve(D->getPointOfInstantiation());
  uint64_t c7 = context_.source_model_.resolve(D->getExternLoc());
  uint64_t c8 = context_.source_model_.resolve(D->getTemplateKeywordLoc());
  uint64_t c9 = context_.source_model_.resolve(D->getSourceRange());
  arboretum_emit_ClassTemplateSpecializationDecl(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9);
  return true;
}

bool ArboretumASTVisitor::VisitConceptDecl(clang::ConceptDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getConstraintExpr());
  uint64_t c2 = context_.source_model_.resolve(D->getSourceRange());
  bool c3 = D->isTypeConcept();
  uint64_t c4 = context_.resolve(D->getCanonicalDecl());
  arboretum_emit_ConceptDecl(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitConstructorUsingShadowDecl(clang::ConstructorUsingShadowDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getIntroducer());
  uint64_t c2 = context_.resolve(D->getParent());
  uint64_t c3 = context_.resolve(D->getNominatedBaseClassShadowDecl());
  uint64_t c4 = context_.resolve(D->getConstructedBaseClassShadowDecl());
  uint64_t c5 = context_.resolve(D->getNominatedBaseClass());
  uint64_t c6 = context_.resolve(D->getConstructedBaseClass());
  bool c7 = D->constructsVirtualBase();
  arboretum_emit_ConstructorUsingShadowDecl(c0, c1, c2, c3, c4, c5, c6, c7);
  return true;
}

bool ArboretumASTVisitor::VisitDecl(clang::Decl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getSourceRange());
  uint64_t c2 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getLocation());
  uint64_t c5 = context_.resolve(D->getNextDeclInContext());
  uint64_t c6 = context_.resolve(D->getNonClosureContext());
  uint64_t c7 = context_.resolve(D->getTranslationUnitDecl());
  bool c8 = D->isInAnonymousNamespace();
  bool c9 = D->isInStdNamespace();
  bool c10 = D->isFileContextDecl();
  uint64_t c11 = context_.data_model_.resolve(D->getAccess());
  uint64_t c12 = context_.data_model_.resolve(D->getAccessUnsafe());
  bool c13 = D->hasAttrs();
  bool c14 = D->isInvalidDecl();
  bool c15 = D->isImplicit();
  bool c16 = D->isReferenced();
  bool c17 = D->isThisDeclarationReferenced();
  bool c18 = D->isTopLevelDeclInObjCContainer();
  bool c19 = D->isModulePrivate();
  bool c20 = D->isInExportDeclContext();
  bool c21 = D->isInvisibleOutsideTheOwningModule();
  bool c22 = D->isInAnotherModuleUnit();
  bool c23 = D->isDiscardedInGlobalModuleFragment();
  bool c24 = D->shouldSkipCheckingODR();
  bool c25 = D->hasDefiningAttr();
  uint64_t c26 = context_.resolve(D->getDefiningAttr());
  bool c27 = D->isWeakImported();
  bool c28 = D->isFromASTFile();
  uint32_t c29 = D->getGlobalID();
  uint32_t c30 = D->getOwningModuleID();
  bool c31 = D->hasOwningModule();
  bool c32 = D->isUnconditionallyVisible();
  bool c33 = D->isReachable();
  uint64_t c34 = context_.data_model_.resolve(D->getModuleOwnershipKind());
  uint32_t c35 = D->getIdentifierNamespace();
  bool c36 = D->hasTagIdentifierNamespace();
  bool c37 = D->isOutOfLine();
  bool c38 = D->isTemplated();
  uint32_t c39 = D->getTemplateDepth();
  bool c40 = D->isDefinedOutsideFunctionOrMethod();
  uint64_t c41 = context_.resolve(D->getCanonicalDecl());
  bool c42 = D->isCanonicalDecl();
  uint64_t c43 = context_.resolve(D->getPreviousDecl());
  bool c44 = D->isFirstDecl();
  uint64_t c45 = context_.resolve(D->getMostRecentDecl());
  uint64_t c46 = context_.resolve(D->getBody());
  bool c47 = D->hasBody();
  uint64_t c48 = context_.source_model_.resolve(D->getBodyRBrace());
  bool c49 = D->isTemplateParameter();
  bool c50 = D->isTemplateParameterPack();
  bool c51 = D->isParameterPack();
  bool c52 = D->isTemplateDecl();
  bool c53 = D->isFunctionOrFunctionTemplate();
  uint64_t c54 = context_.resolve(D->getDescribedTemplate());
  uint64_t c55 = context_.resolve(D->getAsFunction());
  bool c56 = D->isLocalExternDecl();
  uint64_t c57 = context_.data_model_.resolve(D->getFriendObjectKind());
  int64_t c58 = D->getID();
  bool c59 = D->isFunctionPointerType();
  {
    uint64_t idx = 0;
    for (const auto& element : D->attrs()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_Decl_attrs(c0, idx, element_id);
      ++idx;
    }
  }
  {
    uint64_t idx = 0;
    for (const auto& element : D->redecls()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_Decl_redecls(c0, idx, element_id);
      ++idx;
    }
  }
  arboretum_emit_Decl(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19, c20, c21, c22, c23, c24, c25, c26, c27, c28, c29, c30, c31, c32, c33, c34, c35, c36, c37, c38, c39, c40, c41, c42, c43, c44, c45, c46, c47, c48, c49, c50, c51, c52, c53, c54, c55, c56, c57, c58, c59);
  return true;
}

bool ArboretumASTVisitor::VisitDeclaratorDecl(clang::DeclaratorDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getInnerLocStart());
  uint64_t c2 = context_.source_model_.resolve(D->getOuterLocStart());
  uint64_t c3 = context_.source_model_.resolve(D->getSourceRange());
  uint64_t c4 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c5 = context_.resolve(D->getTrailingRequiresClause());
  uint32_t c6 = D->getNumTemplateParameterLists();
  uint64_t c7 = context_.source_model_.resolve(D->getTypeSpecStartLoc());
  uint64_t c8 = context_.source_model_.resolve(D->getTypeSpecEndLoc());
  arboretum_emit_DeclaratorDecl(c0, c1, c2, c3, c4, c5, c6, c7, c8);
  return true;
}

bool ArboretumASTVisitor::VisitDecompositionDecl(clang::DecompositionDecl* D) {
  uint64_t c0 = context_.resolve(D);
  {
    uint64_t idx = 0;
    for (const auto& element : D->bindings()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_DecompositionDecl_bindings(c0, idx, element_id);
    }
  }
  arboretum_emit_DecompositionDecl(c0);
  return true;
}

bool ArboretumASTVisitor::VisitEmptyDecl(clang::EmptyDecl* D) {
  uint64_t c0 = context_.resolve(D);
  arboretum_emit_EmptyDecl(c0);
  return true;
}

bool ArboretumASTVisitor::VisitEnumConstantDecl(clang::EnumConstantDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getInitExpr());
  uint64_t c2 = context_.source_model_.resolve(D->getSourceRange());
  uint64_t c3 = context_.resolve(D->getCanonicalDecl());
  arboretum_emit_EnumConstantDecl(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitEnumDecl(clang::EnumDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getCanonicalDecl());
  uint64_t c2 = context_.resolve(D->getPreviousDecl());
  uint64_t c3 = context_.resolve(D->getMostRecentDecl());
  uint64_t c4 = context_.resolve(D->getDefinition());
  uint64_t c5 = context_.source_model_.resolve(D->getSourceRange());
  uint64_t c6 = context_.resolve(D->getPromotionType());
  uint64_t c7 = context_.resolve(D->getIntegerType());
  uint64_t c8 = context_.source_model_.resolve(D->getIntegerTypeRange());
  uint32_t c9 = D->getNumPositiveBits();
  uint32_t c10 = D->getNumNegativeBits();
  bool c11 = D->isScoped();
  bool c12 = D->isScopedUsingClassTag();
  bool c13 = D->isFixed();
  bool c14 = D->isComplete();
  bool c15 = D->isClosed();
  bool c16 = D->isClosedFlag();
  bool c17 = D->isClosedNonFlag();
  uint64_t c18 = context_.resolve(D->getTemplateInstantiationPattern());
  uint64_t c19 = context_.resolve(D->getInstantiatedFromMemberEnum());
  uint64_t c20 = context_.data_model_.resolve(D->getTemplateSpecializationKind());
  {
    uint64_t idx = 0;
    for (const auto& element : D->enumerators()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_EnumDecl_enumerators(c0, idx, element_id);
      ++idx;
    }
  }
  arboretum_emit_EnumDecl(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19, c20);
  return true;
}

bool ArboretumASTVisitor::VisitExportDecl(clang::ExportDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getExportLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getRBraceLoc());
  bool c3 = D->hasBraces();
  uint64_t c4 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getSourceRange());
  arboretum_emit_ExportDecl(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitExternCContextDecl(clang::ExternCContextDecl* D) {
  uint64_t c0 = context_.resolve(D);
  arboretum_emit_ExternCContextDecl(c0);
  return true;
}

bool ArboretumASTVisitor::VisitFieldDecl(clang::FieldDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint32_t c1 = D->getFieldIndex();
  bool c2 = D->isMutable();
  bool c3 = D->isBitField();
  bool c4 = D->isUnnamedBitfield();
  bool c5 = D->isAnonymousStructOrUnion();
  uint64_t c6 = context_.resolve(D->getBitWidth());
  bool c7 = D->isPotentiallyOverlapping();
  uint64_t c8 = context_.data_model_.resolve(D->getInClassInitStyle());
  bool c9 = D->hasInClassInitializer();
  bool c10 = D->hasNonNullInClassInitializer();
  uint64_t c11 = context_.resolve(D->getInClassInitializer());
  bool c12 = D->hasCapturedVLAType();
  uint64_t c13 = context_.resolve(D->getCapturedVLAType());
  uint64_t c14 = context_.resolve(D->getParent());
  uint64_t c15 = context_.source_model_.resolve(D->getSourceRange());
  uint64_t c16 = context_.resolve(D->getCanonicalDecl());
  arboretum_emit_FieldDecl(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16);
  return true;
}

bool ArboretumASTVisitor::VisitFileScopeAsmDecl(clang::FileScopeAsmDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getAsmLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getRParenLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getSourceRange());
  uint64_t c4 = context_.resolve(D->getAsmString());
  arboretum_emit_FileScopeAsmDecl(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitFriendDecl(clang::FriendDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint32_t c1 = D->getFriendTypeNumTemplateParameterLists();
  uint64_t c2 = context_.resolve(D->getFriendDecl());
  uint64_t c3 = context_.source_model_.resolve(D->getFriendLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getSourceRange());
  bool c5 = D->isUnsupportedFriend();
  arboretum_emit_FriendDecl(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitFriendTemplateDecl(clang::FriendTemplateDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getFriendDecl());
  uint64_t c2 = context_.source_model_.resolve(D->getFriendLoc());
  uint32_t c3 = D->getNumTemplateParameters();
  arboretum_emit_FriendTemplateDecl(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitFunctionDecl(clang::FunctionDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getEllipsisLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getSourceRange());
  bool c3 = D->hasBody();
  bool c4 = D->hasTrivialBody();
  bool c5 = D->isDefined();
  uint64_t c6 = context_.resolve(D->getDefinition());
  uint64_t c7 = context_.resolve(D->getBody());
  bool c8 = D->isThisDeclarationADefinition();
  bool c9 = D->isThisDeclarationInstantiatedFromAFriendDefinition();
  bool c10 = D->doesThisDeclarationHaveABody();
  bool c11 = D->isVariadic();
  bool c12 = D->isVirtualAsWritten();
  bool c13 = D->isPureVirtual();
  bool c14 = D->isLateTemplateParsed();
  bool c15 = D->isTrivial();
  bool c16 = D->isTrivialForCall();
  bool c17 = D->isDefaulted();
  bool c18 = D->isExplicitlyDefaulted();
  uint64_t c19 = context_.source_model_.resolve(D->getDefaultLoc());
  bool c20 = D->isUserProvided();
  bool c21 = D->isIneligibleOrNotSelected();
  bool c22 = D->hasImplicitReturnZero();
  bool c23 = D->hasPrototype();
  bool c24 = D->hasWrittenPrototype();
  bool c25 = D->hasInheritedPrototype();
  bool c26 = D->isConstexpr();
  uint64_t c27 = context_.data_model_.resolve(D->getConstexprKind());
  bool c28 = D->isConstexprSpecified();
  bool c29 = D->isConsteval();
  bool c30 = D->BodyContainsImmediateEscalatingExpressions();
  bool c31 = D->isImmediateEscalating();
  bool c32 = D->isImmediateFunction();
  bool c33 = D->instantiationIsPending();
  bool c34 = D->usesSEHTry();
  bool c35 = D->isDeleted();
  bool c36 = D->isDeletedAsWritten();
  bool c37 = D->isMain();
  bool c38 = D->isMSVCRTEntryPoint();
  bool c39 = D->isReservedGlobalPlacementOperator();
  bool c40 = D->isInlineBuiltinDeclaration();
  bool c41 = D->isDestroyingOperatorDelete();
  uint64_t c42 = context_.data_model_.resolve(D->getLanguageLinkage());
  bool c43 = D->isExternC();
  bool c44 = D->isInExternCContext();
  bool c45 = D->isInExternCXXContext();
  bool c46 = D->isGlobal();
  bool c47 = D->isNoReturn();
  bool c48 = D->hasSkippedBody();
  bool c49 = D->willHaveBody();
  bool c50 = D->isMultiVersion();
  bool c51 = D->FriendConstraintRefersToEnclosingTemplate();
  bool c52 = D->isMemberLikeConstrainedFriend();
  uint64_t c53 = context_.data_model_.resolve(D->getMultiVersionKind());
  bool c54 = D->isCPUDispatchMultiVersion();
  bool c55 = D->isCPUSpecificMultiVersion();
  bool c56 = D->isTargetMultiVersion();
  bool c57 = D->isTargetClonesMultiVersion();
  uint64_t c58 = context_.resolve(D->getCanonicalDecl());
  bool c59 = D->param_empty();
  uint64_t c60 = D->param_size();
  uint32_t c61 = D->getNumParams();
  uint32_t c62 = D->getMinRequiredArguments();
  uint32_t c63 = D->getMinRequiredExplicitArguments();
  bool c64 = D->hasCXXExplicitFunctionObjectParameter();
  uint32_t c65 = D->getNumNonObjectParams();
  bool c66 = D->hasOneParamOrDefaultArgs();
  uint64_t c67 = context_.resolve(D->getReturnType());
  uint64_t c68 = context_.source_model_.resolve(D->getReturnTypeSourceRange());
  uint64_t c69 = context_.source_model_.resolve(D->getParametersSourceRange());
  uint64_t c70 = context_.resolve(D->getDeclaredReturnType());
  uint64_t c71 = context_.data_model_.resolve(D->getExceptionSpecType());
  uint64_t c72 = context_.source_model_.resolve(D->getExceptionSpecSourceRange());
  uint64_t c73 = context_.resolve(D->getCallResultType());
  uint64_t c74 = context_.data_model_.resolve(D->getStorageClass());
  bool c75 = D->isInlineSpecified();
  bool c76 = D->UsesFPIntrin();
  bool c77 = D->isInlined();
  bool c78 = D->isInlineDefinitionExternallyVisible();
  bool c79 = D->isMSExternInline();
  bool c80 = D->doesDeclarationForceExternallyVisibleDefinition();
  bool c81 = D->isStatic();
  bool c82 = D->isOverloadedOperator();
  uint64_t c83 = context_.data_model_.resolve(D->getOverloadedOperator());
  uint64_t c84 = context_.resolve(D->getInstantiatedFromMemberFunction());
  uint64_t c85 = context_.data_model_.resolve(D->getTemplatedKind());
  uint64_t c86 = context_.resolve(D->getInstantiatedFromDecl());
  uint64_t c87 = context_.resolve(D->getDescribedFunctionTemplate());
  bool c88 = D->isFunctionTemplateSpecialization();
  bool c89 = D->isImplicitlyInstantiable();
  bool c90 = D->isTemplateInstantiation();
  uint64_t c91 = context_.resolve(D->getPrimaryTemplate());
  uint64_t c92 = context_.data_model_.resolve(D->getTemplateSpecializationKind());
  uint64_t c93 = context_.data_model_.resolve(D->getTemplateSpecializationKindForInstantiation());
  uint64_t c94 = context_.source_model_.resolve(D->getPointOfInstantiation());
  bool c95 = D->isOutOfLine();
  uint32_t c96 = D->getMemoryFunctionKind();
  uint32_t c97 = D->getODRHash();
  {
    uint64_t idx = 0;
    for (const auto& element : D->parameters()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_FunctionDecl_parameters(c0, idx, element_id);
    }
  }
  arboretum_emit_FunctionDecl(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19, c20, c21, c22, c23, c24, c25, c26, c27, c28, c29, c30, c31, c32, c33, c34, c35, c36, c37, c38, c39, c40, c41, c42, c43, c44, c45, c46, c47, c48, c49, c50, c51, c52, c53, c54, c55, c56, c57, c58, c59, c60, c61, c62, c63, c64, c65, c66, c67, c68, c69, c70, c71, c72, c73, c74, c75, c76, c77, c78, c79, c80, c81, c82, c83, c84, c85, c86, c87, c88, c89, c90, c91, c92, c93, c94, c95, c96, c97);
  return true;
}

bool ArboretumASTVisitor::VisitFunctionTemplateDecl(clang::FunctionTemplateDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getTemplatedDecl());
  bool c2 = D->isThisDeclarationADefinition();
  uint64_t c3 = context_.resolve(D->getCanonicalDecl());
  uint64_t c4 = context_.resolve(D->getPreviousDecl());
  uint64_t c5 = context_.resolve(D->getMostRecentDecl());
  uint64_t c6 = context_.resolve(D->getInstantiatedFromMemberTemplate());
  bool c7 = D->isAbbreviated();
  arboretum_emit_FunctionTemplateDecl(c0, c1, c2, c3, c4, c5, c6, c7);
  return true;
}

bool ArboretumASTVisitor::VisitHLSLBufferDecl(clang::HLSLBufferDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getSourceRange());
  uint64_t c2 = context_.source_model_.resolve(D->getLocStart());
  uint64_t c3 = context_.source_model_.resolve(D->getLBraceLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getRBraceLoc());
  bool c5 = D->isCBuffer();
  arboretum_emit_HLSLBufferDecl(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitImplicitConceptSpecializationDecl(clang::ImplicitConceptSpecializationDecl* D) {
  uint64_t c0 = context_.resolve(D);
  arboretum_emit_ImplicitConceptSpecializationDecl(c0);
  return true;
}

bool ArboretumASTVisitor::VisitImplicitParamDecl(clang::ImplicitParamDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.data_model_.resolve(D->getParameterKind());
  arboretum_emit_ImplicitParamDecl(c0, c1);
  return true;
}

bool ArboretumASTVisitor::VisitImportDecl(clang::ImportDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getSourceRange());
  {
    uint64_t idx = 0;
    for (const auto& element : D->getIdentifierLocs()) {
      uint64_t element_id = context_.source_model_.resolve(element);
      arboretum_emit_ImportDecl_getIdentifierLocs(c0, idx, element_id);
    }
  }
  arboretum_emit_ImportDecl(c0, c1);
  return true;
}

bool ArboretumASTVisitor::VisitIndirectFieldDecl(clang::IndirectFieldDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint32_t c1 = D->getChainingSize();
  uint64_t c2 = context_.resolve(D->getAnonField());
  uint64_t c3 = context_.resolve(D->getVarDecl());
  uint64_t c4 = context_.resolve(D->getCanonicalDecl());
  {
    uint64_t idx = 0;
    for (const auto& element : D->chain()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_IndirectFieldDecl_chain(c0, idx, element_id);
    }
  }
  arboretum_emit_IndirectFieldDecl(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitLabelDecl(clang::LabelDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getStmt());
  bool c2 = D->isGnuLocal();
  uint64_t c3 = context_.source_model_.resolve(D->getSourceRange());
  bool c4 = D->isMSAsmLabel();
  bool c5 = D->isResolvedMSAsmLabel();
  const char* c6 = D->getMSAsmLabel().data();
  arboretum_emit_LabelDecl(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitLifetimeExtendedTemporaryDecl(clang::LifetimeExtendedTemporaryDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getExtendingDecl());
  uint64_t c2 = context_.data_model_.resolve(D->getStorageDuration());
  uint64_t c3 = context_.resolve(D->getTemporaryExpr());
  uint32_t c4 = D->getManglingNumber();
  arboretum_emit_LifetimeExtendedTemporaryDecl(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitLinkageSpecDecl(clang::LinkageSpecDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.data_model_.resolve(D->getLanguage());
  bool c2 = D->hasBraces();
  uint64_t c3 = context_.source_model_.resolve(D->getExternLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getRBraceLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c6 = context_.source_model_.resolve(D->getSourceRange());
  arboretum_emit_LinkageSpecDecl(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitMSGuidDecl(clang::MSGuidDecl* D) {
  uint64_t c0 = context_.resolve(D);
  arboretum_emit_MSGuidDecl(c0);
  return true;
}

bool ArboretumASTVisitor::VisitMSPropertyDecl(clang::MSPropertyDecl* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->hasGetter();
  bool c2 = D->hasSetter();
  arboretum_emit_MSPropertyDecl(c0, c1, c2);
  return true;
}

bool ArboretumASTVisitor::VisitNamedDecl(clang::NamedDecl* D) {
  uint64_t c0 = context_.resolve(D);
  std::string c1_str = D->getNameAsString();
  const char* c1 = c1_str.c_str();
  std::string c2_str = D->getQualifiedNameAsString();
  const char* c2 = c2_str.c_str();
  bool c3 = D->hasLinkage();
  bool c4 = D->isCXXClassMember();
  bool c5 = D->isCXXInstanceMember();
  uint64_t c6 = context_.data_model_.resolve(D->getLinkageInternal());
  uint64_t c7 = context_.data_model_.resolve(D->getFormalLinkage());
  bool c8 = D->hasExternalFormalLinkage();
  bool c9 = D->isExternallyVisible();
  bool c10 = D->isExternallyDeclarable();
  bool c11 = D->isLinkageValid();
  bool c12 = D->hasLinkageBeenComputed();
  uint64_t c13 = context_.resolve(D->getUnderlyingDecl());
  uint64_t c14 = context_.resolve(D->getMostRecentDecl());
  uint64_t c15 = context_.data_model_.resolve(D->getObjCFStringFormattingFamily());
  arboretum_emit_NamedDecl(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15);
  return true;
}

bool ArboretumASTVisitor::VisitNamespaceAliasDecl(clang::NamespaceAliasDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getCanonicalDecl());
  uint64_t c2 = context_.resolve(D->getNamespace());
  uint64_t c3 = context_.source_model_.resolve(D->getAliasLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getNamespaceLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getTargetNameLoc());
  uint64_t c6 = context_.resolve(D->getAliasedNamespace());
  uint64_t c7 = context_.source_model_.resolve(D->getSourceRange());
  arboretum_emit_NamespaceAliasDecl(c0, c1, c2, c3, c4, c5, c6, c7);
  return true;
}

bool ArboretumASTVisitor::VisitNamespaceDecl(clang::NamespaceDecl* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isAnonymousNamespace();
  bool c2 = D->isInline();
  bool c3 = D->isNested();
  uint64_t c4 = context_.resolve(D->getOriginalNamespace());
  bool c5 = D->isOriginalNamespace();
  uint64_t c6 = context_.resolve(D->getAnonymousNamespace());
  uint64_t c7 = context_.resolve(D->getCanonicalDecl());
  uint64_t c8 = context_.source_model_.resolve(D->getSourceRange());
  uint64_t c9 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c10 = context_.source_model_.resolve(D->getRBraceLoc());
  arboretum_emit_NamespaceDecl(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10);
  return true;
}

bool ArboretumASTVisitor::VisitNonTypeTemplateParmDecl(clang::NonTypeTemplateParmDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getSourceRange());
  bool c2 = D->hasDefaultArgument();
  uint64_t c3 = context_.resolve(D->getDefaultArgument());
  uint64_t c4 = context_.source_model_.resolve(D->getDefaultArgumentLoc());
  bool c5 = D->defaultArgumentWasInherited();
  bool c6 = D->isParameterPack();
  bool c7 = D->isPackExpansion();
  bool c8 = D->isExpandedParameterPack();
  uint64_t c9 = context_.resolve(D->getPlaceholderTypeConstraint());
  bool c10 = D->hasPlaceholderTypeConstraint();
  arboretum_emit_NonTypeTemplateParmDecl(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10);
  return true;
}

bool ArboretumASTVisitor::VisitOMPAllocateDecl(clang::OMPAllocateDecl* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPCapturedExprDecl(clang::OMPCapturedExprDecl* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPDeclareMapperDecl(clang::OMPDeclareMapperDecl* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPDeclareReductionDecl(clang::OMPDeclareReductionDecl* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPRequiresDecl(clang::OMPRequiresDecl* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPThreadPrivateDecl(clang::OMPThreadPrivateDecl* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCAtDefsFieldDecl(clang::ObjCAtDefsFieldDecl* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCCategoryDecl(clang::ObjCCategoryDecl* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCCategoryImplDecl(clang::ObjCCategoryImplDecl* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCCompatibleAliasDecl(clang::ObjCCompatibleAliasDecl* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCContainerDecl(clang::ObjCContainerDecl* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCImplDecl(clang::ObjCImplDecl* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCImplementationDecl(clang::ObjCImplementationDecl* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCInterfaceDecl(clang::ObjCInterfaceDecl* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCIvarDecl(clang::ObjCIvarDecl* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCMethodDecl(clang::ObjCMethodDecl* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCPropertyDecl(clang::ObjCPropertyDecl* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCPropertyImplDecl(clang::ObjCPropertyImplDecl* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCProtocolDecl(clang::ObjCProtocolDecl* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCTypeParamDecl(clang::ObjCTypeParamDecl* D) {
  return true;
}

bool ArboretumASTVisitor::VisitParmVarDecl(clang::ParmVarDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getSourceRange());
  bool c2 = D->isObjCMethodParameter();
  bool c3 = D->isDestroyedInCallee();
  uint32_t c4 = D->getFunctionScopeDepth();
  uint32_t c5 = D->getFunctionScopeIndex();
  uint64_t c6 = context_.data_model_.resolve(D->getObjCDeclQualifier());
  bool c7 = D->isKNRPromoted();
  bool c8 = D->isExplicitObjectParameter();
  uint64_t c9 = context_.source_model_.resolve(D->getExplicitObjectParamThisLoc());
  uint64_t c10 = context_.resolve(D->getDefaultArg());
  uint64_t c11 = context_.source_model_.resolve(D->getDefaultArgRange());
  uint64_t c12 = context_.resolve(D->getUninstantiatedDefaultArg());
  bool c13 = D->hasDefaultArg();
  bool c14 = D->hasUnparsedDefaultArg();
  bool c15 = D->hasUninstantiatedDefaultArg();
  bool c16 = D->hasInheritedDefaultArg();
  uint64_t c17 = context_.resolve(D->getOriginalType());
  arboretum_emit_ParmVarDecl(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17);
  return true;
}

bool ArboretumASTVisitor::VisitPragmaCommentDecl(clang::PragmaCommentDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.data_model_.resolve(D->getCommentKind());
  const char* c2 = D->getArg().data();
  arboretum_emit_PragmaCommentDecl(c0, c1, c2);
  return true;
}

bool ArboretumASTVisitor::VisitPragmaDetectMismatchDecl(clang::PragmaDetectMismatchDecl* D) {
  uint64_t c0 = context_.resolve(D);
  const char* c1 = D->getName().data();
  const char* c2 = D->getValue().data();
  arboretum_emit_PragmaDetectMismatchDecl(c0, c1, c2);
  return true;
}

bool ArboretumASTVisitor::VisitRecordDecl(clang::RecordDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getPreviousDecl());
  uint64_t c2 = context_.resolve(D->getMostRecentDecl());
  bool c3 = D->hasFlexibleArrayMember();
  bool c4 = D->isAnonymousStructOrUnion();
  bool c5 = D->hasObjectMember();
  bool c6 = D->hasVolatileMember();
  bool c7 = D->hasLoadedFieldsFromExternalStorage();
  bool c8 = D->isNonTrivialToPrimitiveDefaultInitialize();
  bool c9 = D->isNonTrivialToPrimitiveCopy();
  bool c10 = D->isNonTrivialToPrimitiveDestroy();
  bool c11 = D->hasNonTrivialToPrimitiveDefaultInitializeCUnion();
  bool c12 = D->hasNonTrivialToPrimitiveDestructCUnion();
  bool c13 = D->hasNonTrivialToPrimitiveCopyCUnion();
  bool c14 = D->canPassInRegisters();
  uint64_t c15 = context_.data_model_.resolve(D->getArgPassingRestrictions());
  bool c16 = D->isParamDestroyedInCallee();
  bool c17 = D->isRandomized();
  bool c18 = D->isInjectedClassName();
  bool c19 = D->isLambda();
  bool c20 = D->isCapturedRecord();
  uint64_t c21 = context_.resolve(D->getDefinition());
  bool c22 = D->isOrContainsUnion();
  bool c23 = D->field_empty();
  uint64_t c24 = context_.resolve(D->findFirstNamedDataMember());
  {
    uint64_t idx = 0;
    for (const auto& element : D->fields()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_RecordDecl_fields(c0, idx, element_id);
      ++idx;
    }
  }
  arboretum_emit_RecordDecl(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19, c20, c21, c22, c23, c24);
  return true;
}

bool ArboretumASTVisitor::VisitRedeclarableTemplateDecl(clang::RedeclarableTemplateDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getCanonicalDecl());
  bool c2 = D->isMemberSpecialization();
  uint64_t c3 = context_.resolve(D->getInstantiatedFromMemberTemplate());
  arboretum_emit_RedeclarableTemplateDecl(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitRequiresExprBodyDecl(clang::RequiresExprBodyDecl* D) {
  uint64_t c0 = context_.resolve(D);
  arboretum_emit_RequiresExprBodyDecl(c0);
  return true;
}

bool ArboretumASTVisitor::VisitStaticAssertDecl(clang::StaticAssertDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getAssertExpr());
  uint64_t c2 = context_.resolve(D->getMessage());
  bool c3 = D->isFailed();
  uint64_t c4 = context_.source_model_.resolve(D->getRParenLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getSourceRange());
  arboretum_emit_StaticAssertDecl(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitTagDecl(clang::TagDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getBraceRange());
  uint64_t c2 = context_.source_model_.resolve(D->getInnerLocStart());
  uint64_t c3 = context_.source_model_.resolve(D->getOuterLocStart());
  uint64_t c4 = context_.source_model_.resolve(D->getSourceRange());
  uint64_t c5 = context_.resolve(D->getCanonicalDecl());
  bool c6 = D->isThisDeclarationADefinition();
  bool c7 = D->isCompleteDefinition();
  bool c8 = D->isCompleteDefinitionRequired();
  bool c9 = D->isBeingDefined();
  bool c10 = D->isEmbeddedInDeclarator();
  bool c11 = D->isFreeStanding();
  bool c12 = D->mayHaveOutOfDateDef();
  bool c13 = D->isDependentType();
  bool c14 = D->isThisDeclarationADemotedDefinition();
  uint64_t c15 = context_.resolve(D->getDefinition());
  const char* c16 = D->getKindName().data();
  uint64_t c17 = context_.data_model_.resolve(D->getTagKind());
  bool c18 = D->isStruct();
  bool c19 = D->isInterface();
  bool c20 = D->isClass();
  bool c21 = D->isUnion();
  bool c22 = D->isEnum();
  bool c23 = D->hasNameForLinkage();
  uint64_t c24 = context_.resolve(D->getTypedefNameForAnonDecl());
  uint32_t c25 = D->getNumTemplateParameterLists();
  arboretum_emit_TagDecl(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19, c20, c21, c22, c23, c24, c25);
  return true;
}

bool ArboretumASTVisitor::VisitTemplateDecl(clang::TemplateDecl* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->hasAssociatedConstraints();
  uint64_t c2 = context_.resolve(D->getTemplatedDecl());
  bool c3 = D->isTypeAlias();
  uint64_t c4 = context_.source_model_.resolve(D->getSourceRange());
  arboretum_emit_TemplateDecl(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitTemplateParamObjectDecl(clang::TemplateParamObjectDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getCanonicalDecl());
  arboretum_emit_TemplateParamObjectDecl(c0, c1);
  return true;
}

bool ArboretumASTVisitor::VisitTemplateTemplateParmDecl(clang::TemplateTemplateParmDecl* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isParameterPack();
  bool c2 = D->isPackExpansion();
  bool c3 = D->isExpandedParameterPack();
  bool c4 = D->hasDefaultArgument();
  uint64_t c5 = context_.source_model_.resolve(D->getDefaultArgumentLoc());
  bool c6 = D->defaultArgumentWasInherited();
  uint64_t c7 = context_.source_model_.resolve(D->getSourceRange());
  arboretum_emit_TemplateTemplateParmDecl(c0, c1, c2, c3, c4, c5, c6, c7);
  return true;
}

bool ArboretumASTVisitor::VisitTemplateTypeParmDecl(clang::TemplateTypeParmDecl* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->wasDeclaredWithTypename();
  bool c2 = D->hasDefaultArgument();
  uint64_t c3 = context_.source_model_.resolve(D->getDefaultArgumentLoc());
  bool c4 = D->defaultArgumentWasInherited();
  uint32_t c5 = D->getDepth();
  uint32_t c6 = D->getIndex();
  bool c7 = D->isParameterPack();
  bool c8 = D->isPackExpansion();
  bool c9 = D->isExpandedParameterPack();
  bool c10 = D->hasTypeConstraint();
  uint64_t c11 = context_.source_model_.resolve(D->getSourceRange());
  arboretum_emit_TemplateTypeParmDecl(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11);
  return true;
}

bool ArboretumASTVisitor::VisitTopLevelStmtDecl(clang::TopLevelStmtDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getSourceRange());
  uint64_t c2 = context_.resolve(D->getStmt());
  bool c3 = D->isSemiMissing();
  arboretum_emit_TopLevelStmtDecl(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitTranslationUnitDecl(clang::TranslationUnitDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getAnonymousNamespace());
  arboretum_emit_TranslationUnitDecl(c0, c1);
  return true;
}

bool ArboretumASTVisitor::VisitTypeAliasDecl(clang::TypeAliasDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getSourceRange());
  uint64_t c2 = context_.resolve(D->getDescribedAliasTemplate());
  arboretum_emit_TypeAliasDecl(c0, c1, c2);
  return true;
}

bool ArboretumASTVisitor::VisitTypeAliasTemplateDecl(clang::TypeAliasTemplateDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getTemplatedDecl());
  uint64_t c2 = context_.resolve(D->getCanonicalDecl());
  uint64_t c3 = context_.resolve(D->getPreviousDecl());
  uint64_t c4 = context_.resolve(D->getInstantiatedFromMemberTemplate());
  arboretum_emit_TypeAliasTemplateDecl(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitTypeDecl(clang::TypeDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getTypeForDecl());
  uint64_t c2 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getSourceRange());
  arboretum_emit_TypeDecl(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitTypedefDecl(clang::TypedefDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getSourceRange());
  arboretum_emit_TypedefDecl(c0, c1);
  return true;
}

bool ArboretumASTVisitor::VisitTypedefNameDecl(clang::TypedefNameDecl* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isModed();
  uint64_t c2 = context_.resolve(D->getUnderlyingType());
  uint64_t c3 = context_.resolve(D->getCanonicalDecl());
  bool c4 = D->isTransparentTag();
  arboretum_emit_TypedefNameDecl(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitUnnamedGlobalConstantDecl(clang::UnnamedGlobalConstantDecl* D) {
  uint64_t c0 = context_.resolve(D);
  arboretum_emit_UnnamedGlobalConstantDecl(c0);
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedUsingIfExistsDecl(clang::UnresolvedUsingIfExistsDecl* D) {
  uint64_t c0 = context_.resolve(D);
  arboretum_emit_UnresolvedUsingIfExistsDecl(c0);
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedUsingTypenameDecl(clang::UnresolvedUsingTypenameDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getUsingLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getTypenameLoc());
  bool c3 = D->isPackExpansion();
  uint64_t c4 = context_.source_model_.resolve(D->getEllipsisLoc());
  uint64_t c5 = context_.resolve(D->getCanonicalDecl());
  arboretum_emit_UnresolvedUsingTypenameDecl(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedUsingValueDecl(clang::UnresolvedUsingValueDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getUsingLoc());
  bool c2 = D->isAccessDeclaration();
  bool c3 = D->isPackExpansion();
  uint64_t c4 = context_.source_model_.resolve(D->getEllipsisLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getSourceRange());
  uint64_t c6 = context_.resolve(D->getCanonicalDecl());
  arboretum_emit_UnresolvedUsingValueDecl(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitUsingDecl(clang::UsingDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getUsingLoc());
  bool c2 = D->isAccessDeclaration();
  bool c3 = D->hasTypename();
  uint64_t c4 = context_.source_model_.resolve(D->getSourceRange());
  uint64_t c5 = context_.resolve(D->getCanonicalDecl());
  arboretum_emit_UsingDecl(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitUsingDirectiveDecl(clang::UsingDirectiveDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getNominatedNamespaceAsWritten());
  uint64_t c2 = context_.resolve(D->getNominatedNamespace());
  uint64_t c3 = context_.source_model_.resolve(D->getUsingLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getNamespaceKeyLocation());
  uint64_t c5 = context_.source_model_.resolve(D->getIdentLocation());
  uint64_t c6 = context_.source_model_.resolve(D->getSourceRange());
  arboretum_emit_UsingDirectiveDecl(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitUsingEnumDecl(clang::UsingEnumDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getUsingLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getEnumLoc());
  uint64_t c3 = context_.resolve(D->getEnumDecl());
  uint64_t c4 = context_.source_model_.resolve(D->getSourceRange());
  uint64_t c5 = context_.resolve(D->getCanonicalDecl());
  arboretum_emit_UsingEnumDecl(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitUsingPackDecl(clang::UsingPackDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getInstantiatedFromUsingDecl());
  uint64_t c2 = context_.source_model_.resolve(D->getSourceRange());
  uint64_t c3 = context_.resolve(D->getCanonicalDecl());
  {
    uint64_t idx = 0;
    for (const auto& element : D->expansions()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_UsingPackDecl_expansions(c0, idx, element_id);
    }
  }
  arboretum_emit_UsingPackDecl(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitUsingShadowDecl(clang::UsingShadowDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getCanonicalDecl());
  uint64_t c2 = context_.resolve(D->getTargetDecl());
  uint64_t c3 = context_.resolve(D->getIntroducer());
  uint64_t c4 = context_.resolve(D->getNextUsingShadowDecl());
  arboretum_emit_UsingShadowDecl(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitValueDecl(clang::ValueDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getType());
  bool c2 = D->isWeak();
  bool c3 = D->isInitCapture();
  uint64_t c4 = context_.resolve(D->getPotentiallyDecomposedVarDecl());
  arboretum_emit_ValueDecl(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitVarDecl(clang::VarDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getSourceRange());
  uint64_t c2 = context_.data_model_.resolve(D->getStorageClass());
  uint64_t c3 = context_.data_model_.resolve(D->getTSCSpec());
  uint64_t c4 = context_.data_model_.resolve(D->getTLSKind());
  bool c5 = D->hasLocalStorage();
  bool c6 = D->isStaticLocal();
  bool c7 = D->hasExternalStorage();
  bool c8 = D->hasGlobalStorage();
  uint64_t c9 = context_.data_model_.resolve(D->getStorageDuration());
  uint64_t c10 = context_.data_model_.resolve(D->getLanguageLinkage());
  bool c11 = D->isExternC();
  bool c12 = D->isInExternCContext();
  bool c13 = D->isInExternCXXContext();
  bool c14 = D->isLocalVarDecl();
  bool c15 = D->isLocalVarDeclOrParm();
  bool c16 = D->isFunctionOrMethodVarDecl();
  bool c17 = D->isStaticDataMember();
  uint64_t c18 = context_.resolve(D->getCanonicalDecl());
  uint64_t c19 = context_.data_model_.resolve(D->isThisDeclarationADefinition());
  uint64_t c20 = context_.data_model_.resolve(D->hasDefinition());
  uint64_t c21 = context_.resolve(D->getActingDefinition());
  uint64_t c22 = context_.resolve(D->getDefinition());
  bool c23 = D->isOutOfLine();
  bool c24 = D->isFileVarDecl();
  uint64_t c25 = context_.resolve(D->getAnyInitializer());
  bool c26 = D->hasInit();
  uint64_t c27 = context_.resolve(D->getInit());
  uint64_t c28 = context_.resolve(D->getInitializingDeclaration());
  bool c29 = D->hasConstantInitialization();
  uint64_t c30 = context_.data_model_.resolve(D->getInitStyle());
  bool c31 = D->isDirectInit();
  bool c32 = D->isThisDeclarationADemotedDefinition();
  bool c33 = D->isExceptionVariable();
  bool c34 = D->isNRVOVariable();
  bool c35 = D->isCXXForRangeDecl();
  bool c36 = D->isObjCForDecl();
  bool c37 = D->isARCPseudoStrong();
  bool c38 = D->isInline();
  bool c39 = D->isInlineSpecified();
  bool c40 = D->isConstexpr();
  bool c41 = D->isInitCapture();
  bool c42 = D->isParameterPack();
  bool c43 = D->isPreviousDeclInSameBlockScope();
  bool c44 = D->isEscapingByref();
  bool c45 = D->isNonEscapingByref();
  bool c46 = D->hasDependentAlignment();
  uint64_t c47 = context_.resolve(D->getTemplateInstantiationPattern());
  uint64_t c48 = context_.resolve(D->getInstantiatedFromStaticDataMember());
  uint64_t c49 = context_.data_model_.resolve(D->getTemplateSpecializationKind());
  uint64_t c50 = context_.data_model_.resolve(D->getTemplateSpecializationKindForInstantiation());
  uint64_t c51 = context_.source_model_.resolve(D->getPointOfInstantiation());
  uint64_t c52 = context_.resolve(D->getDescribedVarTemplate());
  bool c53 = D->isKnownToBeDefined();
  arboretum_emit_VarDecl(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19, c20, c21, c22, c23, c24, c25, c26, c27, c28, c29, c30, c31, c32, c33, c34, c35, c36, c37, c38, c39, c40, c41, c42, c43, c44, c45, c46, c47, c48, c49, c50, c51, c52, c53);
  return true;
}

bool ArboretumASTVisitor::VisitVarTemplateDecl(clang::VarTemplateDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getTemplatedDecl());
  bool c2 = D->isThisDeclarationADefinition();
  uint64_t c3 = context_.resolve(D->getCanonicalDecl());
  uint64_t c4 = context_.resolve(D->getPreviousDecl());
  uint64_t c5 = context_.resolve(D->getMostRecentDecl());
  uint64_t c6 = context_.resolve(D->getInstantiatedFromMemberTemplate());
  arboretum_emit_VarTemplateDecl(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitVarTemplatePartialSpecializationDecl(clang::VarTemplatePartialSpecializationDecl* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->hasAssociatedConstraints();
  uint64_t c2 = context_.resolve(D->getInstantiatedFromMember());
  uint64_t c3 = context_.source_model_.resolve(D->getSourceRange());
  arboretum_emit_VarTemplatePartialSpecializationDecl(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitVarTemplateSpecializationDecl(clang::VarTemplateSpecializationDecl* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getSpecializedTemplate());
  uint64_t c2 = context_.data_model_.resolve(D->getSpecializationKind());
  bool c3 = D->isExplicitSpecialization();
  bool c4 = D->isClassScopeExplicitSpecialization();
  bool c5 = D->isExplicitInstantiationOrSpecialization();
  uint64_t c6 = context_.source_model_.resolve(D->getPointOfInstantiation());
  uint64_t c7 = context_.source_model_.resolve(D->getExternLoc());
  uint64_t c8 = context_.source_model_.resolve(D->getTemplateKeywordLoc());
  uint64_t c9 = context_.source_model_.resolve(D->getSourceRange());
  arboretum_emit_VarTemplateSpecializationDecl(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9);
  return true;
}


// Stmts
bool ArboretumASTVisitor::VisitAbstractConditionalOperator(clang::AbstractConditionalOperator* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getCond());
  uint64_t c2 = context_.resolve(D->getTrueExpr());
  uint64_t c3 = context_.resolve(D->getFalseExpr());
  uint64_t c4 = context_.source_model_.resolve(D->getQuestionLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getColonLoc());
  arboretum_emit_AbstractConditionalOperator(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitAddrLabelExpr(clang::AddrLabelExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getAmpAmpLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getLabelLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c5 = context_.resolve(D->getLabel());
  arboretum_emit_AddrLabelExpr(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitArrayInitIndexExpr(clang::ArrayInitIndexExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_ArrayInitIndexExpr(c0, c1, c2);
  return true;
}

bool ArboretumASTVisitor::VisitArrayInitLoopExpr(clang::ArrayInitLoopExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getCommonExpr());
  uint64_t c2 = context_.resolve(D->getSubExpr());
  uint64_t c3 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_ArrayInitLoopExpr(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitArraySubscriptExpr(clang::ArraySubscriptExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getLHS());
  uint64_t c2 = context_.resolve(D->getRHS());
  uint64_t c3 = context_.resolve(D->getBase());
  uint64_t c4 = context_.resolve(D->getIdx());
  uint64_t c5 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c6 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c7 = context_.source_model_.resolve(D->getRBracketLoc());
  uint64_t c8 = context_.source_model_.resolve(D->getExprLoc());
  arboretum_emit_ArraySubscriptExpr(c0, c1, c2, c3, c4, c5, c6, c7, c8);
  return true;
}

bool ArboretumASTVisitor::VisitArrayTypeTraitExpr(clang::ArrayTypeTraitExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c3 = context_.data_model_.resolve(D->getTrait());
  uint64_t c4 = context_.resolve(D->getQueriedType());
  uint64_t c5 = context_.resolve(D->getDimensionExpression());
  arboretum_emit_ArrayTypeTraitExpr(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitAsTypeExpr(clang::AsTypeExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getSrcExpr());
  uint64_t c2 = context_.source_model_.resolve(D->getBuiltinLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getRParenLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_AsTypeExpr(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitAsmStmt(clang::AsmStmt* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getAsmLoc());
  bool c2 = D->isSimple();
  bool c3 = D->isVolatile();
  uint64_t c4 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getEndLoc());
  uint32_t c6 = D->getNumOutputs();
  uint32_t c7 = D->getNumPlusOperands();
  uint32_t c8 = D->getNumInputs();
  uint32_t c9 = D->getNumClobbers();
  arboretum_emit_AsmStmt(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9);
  return true;
}

bool ArboretumASTVisitor::VisitAtomicExpr(clang::AtomicExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getPtr());
  uint64_t c2 = context_.resolve(D->getOrder());
  uint64_t c3 = context_.resolve(D->getScope());
  uint64_t c4 = context_.resolve(D->getVal1());
  uint64_t c5 = context_.resolve(D->getOrderFail());
  uint64_t c6 = context_.resolve(D->getVal2());
  uint64_t c7 = context_.resolve(D->getWeak());
  uint64_t c8 = context_.resolve(D->getValueType());
  uint64_t c9 = context_.data_model_.resolve(D->getOp());
  const char* c10 = D->getOpAsString().data();
  uint32_t c11 = D->getNumSubExprs();
  bool c12 = D->isVolatile();
  bool c13 = D->isCmpXChg();
  bool c14 = D->isOpenCL();
  uint64_t c15 = context_.source_model_.resolve(D->getBuiltinLoc());
  uint64_t c16 = context_.source_model_.resolve(D->getRParenLoc());
  uint64_t c17 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c18 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_AtomicExpr(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18);
  return true;
}

bool ArboretumASTVisitor::VisitAttributedStmt(clang::AttributedStmt* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getAttrLoc());
  uint64_t c2 = context_.resolve(D->getSubStmt());
  uint64_t c3 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getEndLoc());
  {
    uint64_t idx = 0;
    for (const auto& element : D->getAttrs()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_AttributedStmt_getAttrs(c0, idx, element_id);
    }
  }
  arboretum_emit_AttributedStmt(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitBinaryConditionalOperator(clang::BinaryConditionalOperator* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getCommon());
  uint64_t c2 = context_.resolve(D->getOpaqueValue());
  uint64_t c3 = context_.resolve(D->getCond());
  uint64_t c4 = context_.resolve(D->getTrueExpr());
  uint64_t c5 = context_.resolve(D->getFalseExpr());
  uint64_t c6 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c7 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_BinaryConditionalOperator(c0, c1, c2, c3, c4, c5, c6, c7);
  return true;
}

bool ArboretumASTVisitor::VisitBinaryOperator(clang::BinaryOperator* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getExprLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getOperatorLoc());
  uint64_t c3 = context_.data_model_.resolve(D->getOpcode());
  uint64_t c4 = context_.resolve(D->getLHS());
  uint64_t c5 = context_.resolve(D->getRHS());
  uint64_t c6 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c7 = context_.source_model_.resolve(D->getEndLoc());
  const char* c8 = D->getOpcodeStr().data();
  bool c9 = D->isPtrMemOp();
  bool c10 = D->isMultiplicativeOp();
  bool c11 = D->isAdditiveOp();
  bool c12 = D->isShiftOp();
  bool c13 = D->isBitwiseOp();
  bool c14 = D->isRelationalOp();
  bool c15 = D->isEqualityOp();
  bool c16 = D->isComparisonOp();
  bool c17 = D->isCommaOp();
  bool c18 = D->isLogicalOp();
  bool c19 = D->isAssignmentOp();
  bool c20 = D->isCompoundAssignmentOp();
  bool c21 = D->isShiftAssignOp();
  bool c22 = D->hasStoredFPFeatures();
  arboretum_emit_BinaryOperator(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19, c20, c21, c22);
  return true;
}

bool ArboretumASTVisitor::VisitBlockExpr(clang::BlockExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getBlockDecl());
  uint64_t c2 = context_.source_model_.resolve(D->getCaretLocation());
  uint64_t c3 = context_.resolve(D->getBody());
  uint64_t c4 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c6 = context_.resolve(D->getFunctionType());
  arboretum_emit_BlockExpr(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitBreakStmt(clang::BreakStmt* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getBreakLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_BreakStmt(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitBuiltinBitCastExpr(clang::BuiltinBitCastExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_BuiltinBitCastExpr(c0, c1, c2);
  return true;
}

bool ArboretumASTVisitor::VisitCStyleCastExpr(clang::CStyleCastExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getLParenLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getRParenLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_CStyleCastExpr(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitCUDAKernelCallExpr(clang::CUDAKernelCallExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getConfig());
  arboretum_emit_CUDAKernelCallExpr(c0, c1);
  return true;
}

bool ArboretumASTVisitor::VisitCXXAddrspaceCastExpr(clang::CXXAddrspaceCastExpr* D) {
  uint64_t c0 = context_.resolve(D);
  arboretum_emit_CXXAddrspaceCastExpr(c0);
  return true;
}

bool ArboretumASTVisitor::VisitCXXBindTemporaryExpr(clang::CXXBindTemporaryExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getSubExpr());
  uint64_t c2 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_CXXBindTemporaryExpr(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitCXXBoolLiteralExpr(clang::CXXBoolLiteralExpr* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->getValue();
  uint64_t c2 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getLocation());
  arboretum_emit_CXXBoolLiteralExpr(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitCXXCatchStmt(clang::CXXCatchStmt* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getCatchLoc());
  uint64_t c4 = context_.resolve(D->getExceptionDecl());
  uint64_t c5 = context_.resolve(D->getCaughtType());
  uint64_t c6 = context_.resolve(D->getHandlerBlock());
  arboretum_emit_CXXCatchStmt(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitCXXConstCastExpr(clang::CXXConstCastExpr* D) {
  uint64_t c0 = context_.resolve(D);
  arboretum_emit_CXXConstCastExpr(c0);
  return true;
}

bool ArboretumASTVisitor::VisitCXXConstructExpr(clang::CXXConstructExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getConstructor());
  uint64_t c2 = context_.source_model_.resolve(D->getLocation());
  bool c3 = D->isElidable();
  bool c4 = D->hadMultipleCandidates();
  bool c5 = D->isListInitialization();
  bool c6 = D->isStdInitListInitialization();
  bool c7 = D->requiresZeroInitialization();
  uint64_t c8 = context_.data_model_.resolve(D->getConstructionKind());
  uint32_t c9 = D->getNumArgs();
  bool c10 = D->isImmediateEscalating();
  uint64_t c11 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c12 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c13 = context_.source_model_.resolve(D->getParenOrBraceRange());
  arboretum_emit_CXXConstructExpr(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13);
  return true;
}

bool ArboretumASTVisitor::VisitCXXDefaultArgExpr(clang::CXXDefaultArgExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getParam());
  bool c2 = D->hasRewrittenInit();
  uint64_t c3 = context_.resolve(D->getExpr());
  uint64_t c4 = context_.resolve(D->getRewrittenExpr());
  uint64_t c5 = context_.resolve(D->getAdjustedRewrittenExpr());
  uint64_t c6 = context_.source_model_.resolve(D->getUsedLocation());
  uint64_t c7 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c8 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c9 = context_.source_model_.resolve(D->getExprLoc());
  arboretum_emit_CXXDefaultArgExpr(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9);
  return true;
}

bool ArboretumASTVisitor::VisitCXXDefaultInitExpr(clang::CXXDefaultInitExpr* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->hasRewrittenInit();
  uint64_t c2 = context_.resolve(D->getField());
  uint64_t c3 = context_.resolve(D->getExpr());
  uint64_t c4 = context_.resolve(D->getRewrittenExpr());
  uint64_t c5 = context_.source_model_.resolve(D->getUsedLocation());
  uint64_t c6 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c7 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_CXXDefaultInitExpr(c0, c1, c2, c3, c4, c5, c6, c7);
  return true;
}

bool ArboretumASTVisitor::VisitCXXDeleteExpr(clang::CXXDeleteExpr* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isGlobalDelete();
  bool c2 = D->isArrayForm();
  bool c3 = D->isArrayFormAsWritten();
  bool c4 = D->doesUsualArrayDeleteWantSize();
  uint64_t c5 = context_.resolve(D->getOperatorDelete());
  uint64_t c6 = context_.resolve(D->getArgument());
  uint64_t c7 = context_.resolve(D->getDestroyedType());
  uint64_t c8 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c9 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_CXXDeleteExpr(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9);
  return true;
}

bool ArboretumASTVisitor::VisitCXXDependentScopeMemberExpr(clang::CXXDependentScopeMemberExpr* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isImplicitAccess();
  uint64_t c2 = context_.resolve(D->getBaseType());
  bool c3 = D->isArrow();
  uint64_t c4 = context_.source_model_.resolve(D->getOperatorLoc());
  uint64_t c5 = context_.resolve(D->getFirstQualifierFoundInScope());
  uint64_t c6 = context_.source_model_.resolve(D->getMemberLoc());
  uint64_t c7 = context_.source_model_.resolve(D->getTemplateKeywordLoc());
  uint64_t c8 = context_.source_model_.resolve(D->getLAngleLoc());
  uint64_t c9 = context_.source_model_.resolve(D->getRAngleLoc());
  bool c10 = D->hasTemplateKeyword();
  bool c11 = D->hasExplicitTemplateArgs();
  uint32_t c12 = D->getNumTemplateArgs();
  uint64_t c13 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c14 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_CXXDependentScopeMemberExpr(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14);
  return true;
}

bool ArboretumASTVisitor::VisitCXXDynamicCastExpr(clang::CXXDynamicCastExpr* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isAlwaysNull();
  arboretum_emit_CXXDynamicCastExpr(c0, c1);
  return true;
}

bool ArboretumASTVisitor::VisitCXXFoldExpr(clang::CXXFoldExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getCallee());
  uint64_t c2 = context_.resolve(D->getLHS());
  uint64_t c3 = context_.resolve(D->getRHS());
  bool c4 = D->isRightFold();
  bool c5 = D->isLeftFold();
  uint64_t c6 = context_.resolve(D->getPattern());
  uint64_t c7 = context_.resolve(D->getInit());
  uint64_t c8 = context_.source_model_.resolve(D->getLParenLoc());
  uint64_t c9 = context_.source_model_.resolve(D->getRParenLoc());
  uint64_t c10 = context_.source_model_.resolve(D->getEllipsisLoc());
  uint64_t c11 = context_.data_model_.resolve(D->getOperator());
  uint64_t c12 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c13 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_CXXFoldExpr(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13);
  return true;
}

bool ArboretumASTVisitor::VisitCXXForRangeStmt(clang::CXXForRangeStmt* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getInit());
  uint64_t c2 = context_.resolve(D->getLoopVariable());
  uint64_t c3 = context_.resolve(D->getRangeInit());
  uint64_t c4 = context_.resolve(D->getRangeStmt());
  uint64_t c5 = context_.resolve(D->getBeginStmt());
  uint64_t c6 = context_.resolve(D->getEndStmt());
  uint64_t c7 = context_.resolve(D->getCond());
  uint64_t c8 = context_.resolve(D->getInc());
  uint64_t c9 = context_.resolve(D->getLoopVarStmt());
  uint64_t c10 = context_.resolve(D->getBody());
  uint64_t c11 = context_.source_model_.resolve(D->getForLoc());
  uint64_t c12 = context_.source_model_.resolve(D->getCoawaitLoc());
  uint64_t c13 = context_.source_model_.resolve(D->getColonLoc());
  uint64_t c14 = context_.source_model_.resolve(D->getRParenLoc());
  uint64_t c15 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c16 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_CXXForRangeStmt(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16);
  return true;
}

bool ArboretumASTVisitor::VisitCXXFunctionalCastExpr(clang::CXXFunctionalCastExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getLParenLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getRParenLoc());
  bool c3 = D->isListInitialization();
  uint64_t c4 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_CXXFunctionalCastExpr(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitCXXInheritedCtorInitExpr(clang::CXXInheritedCtorInitExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getConstructor());
  bool c2 = D->constructsVBase();
  uint64_t c3 = context_.data_model_.resolve(D->getConstructionKind());
  bool c4 = D->inheritedFromVBase();
  uint64_t c5 = context_.source_model_.resolve(D->getLocation());
  uint64_t c6 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c7 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_CXXInheritedCtorInitExpr(c0, c1, c2, c3, c4, c5, c6, c7);
  return true;
}

bool ArboretumASTVisitor::VisitCXXMemberCallExpr(clang::CXXMemberCallExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getImplicitObjectArgument());
  uint64_t c2 = context_.resolve(D->getObjectType());
  uint64_t c3 = context_.resolve(D->getMethodDecl());
  uint64_t c4 = context_.resolve(D->getRecordDecl());
  uint64_t c5 = context_.source_model_.resolve(D->getExprLoc());
  arboretum_emit_CXXMemberCallExpr(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitCXXNamedCastExpr(clang::CXXNamedCastExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getOperatorLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getRParenLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getAngleBrackets());
  arboretum_emit_CXXNamedCastExpr(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitCXXNewExpr(clang::CXXNewExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getAllocatedType());
  uint64_t c2 = context_.resolve(D->getOperatorNew());
  uint64_t c3 = context_.resolve(D->getOperatorDelete());
  bool c4 = D->isArray();
  uint32_t c5 = D->getNumPlacementArgs();
  bool c6 = D->isParenTypeId();
  uint64_t c7 = context_.source_model_.resolve(D->getTypeIdParens());
  bool c8 = D->isGlobalNew();
  bool c9 = D->hasInitializer();
  uint64_t c10 = context_.data_model_.resolve(D->getInitializationStyle());
  uint64_t c11 = context_.resolve(D->getInitializer());
  uint64_t c12 = context_.resolve(D->getConstructExpr());
  bool c13 = D->passAlignment();
  bool c14 = D->doesUsualArrayDeleteWantSize();
  uint64_t c15 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c16 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c17 = context_.source_model_.resolve(D->getDirectInitRange());
  uint64_t c18 = context_.source_model_.resolve(D->getSourceRange());
  arboretum_emit_CXXNewExpr(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18);
  return true;
}

bool ArboretumASTVisitor::VisitCXXNoexceptExpr(clang::CXXNoexceptExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getOperand());
  uint64_t c2 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getSourceRange());
  bool c5 = D->getValue();
  arboretum_emit_CXXNoexceptExpr(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitCXXNullPtrLiteralExpr(clang::CXXNullPtrLiteralExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getLocation());
  arboretum_emit_CXXNullPtrLiteralExpr(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitCXXOperatorCallExpr(clang::CXXOperatorCallExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.data_model_.resolve(D->getOperator());
  bool c2 = D->isAssignmentOp();
  bool c3 = D->isComparisonOp();
  bool c4 = D->isInfixBinaryOp();
  uint64_t c5 = context_.source_model_.resolve(D->getOperatorLoc());
  uint64_t c6 = context_.source_model_.resolve(D->getExprLoc());
  uint64_t c7 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c8 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c9 = context_.source_model_.resolve(D->getSourceRange());
  arboretum_emit_CXXOperatorCallExpr(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9);
  return true;
}

bool ArboretumASTVisitor::VisitCXXParenListInitExpr(clang::CXXParenListInitExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getInitLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getSourceRange());
  uint64_t c5 = context_.resolve(D->getArrayFiller());
  uint64_t c6 = context_.resolve(D->getInitializedFieldInUnion());
  {
    uint64_t idx = 0;
    for (const auto& element : D->getInitExprs()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_CXXParenListInitExpr_getInitExprs(c0, idx, element_id);
    }
  }
  {
    uint64_t idx = 0;
    for (const auto& element : D->getUserSpecifiedInitExprs()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_CXXParenListInitExpr_getUserSpecifiedInitExprs(c0, idx, element_id);
    }
  }
  arboretum_emit_CXXParenListInitExpr(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitCXXPseudoDestructorExpr(clang::CXXPseudoDestructorExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getBase());
  bool c2 = D->hasQualifier();
  bool c3 = D->isArrow();
  uint64_t c4 = context_.source_model_.resolve(D->getOperatorLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getColonColonLoc());
  uint64_t c6 = context_.source_model_.resolve(D->getTildeLoc());
  uint64_t c7 = context_.resolve(D->getDestroyedType());
  uint64_t c8 = context_.source_model_.resolve(D->getDestroyedTypeLoc());
  uint64_t c9 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c10 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_CXXPseudoDestructorExpr(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10);
  return true;
}

bool ArboretumASTVisitor::VisitCXXReinterpretCastExpr(clang::CXXReinterpretCastExpr* D) {
  uint64_t c0 = context_.resolve(D);
  arboretum_emit_CXXReinterpretCastExpr(c0);
  return true;
}

bool ArboretumASTVisitor::VisitCXXRewrittenBinaryOperator(clang::CXXRewrittenBinaryOperator* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getSemanticForm());
  bool c2 = D->isReversed();
  uint64_t c3 = context_.data_model_.resolve(D->getOperator());
  uint64_t c4 = context_.data_model_.resolve(D->getOpcode());
  const char* c5 = D->getOpcodeStr().data();
  bool c6 = D->isComparisonOp();
  bool c7 = D->isAssignmentOp();
  uint64_t c8 = context_.resolve(D->getLHS());
  uint64_t c9 = context_.resolve(D->getRHS());
  uint64_t c10 = context_.source_model_.resolve(D->getOperatorLoc());
  uint64_t c11 = context_.source_model_.resolve(D->getExprLoc());
  uint64_t c12 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c13 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c14 = context_.source_model_.resolve(D->getSourceRange());
  arboretum_emit_CXXRewrittenBinaryOperator(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14);
  return true;
}

bool ArboretumASTVisitor::VisitCXXScalarValueInitExpr(clang::CXXScalarValueInitExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getRParenLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_CXXScalarValueInitExpr(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitCXXStaticCastExpr(clang::CXXStaticCastExpr* D) {
  uint64_t c0 = context_.resolve(D);
  arboretum_emit_CXXStaticCastExpr(c0);
  return true;
}

bool ArboretumASTVisitor::VisitCXXStdInitializerListExpr(clang::CXXStdInitializerListExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getSubExpr());
  uint64_t c2 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getSourceRange());
  arboretum_emit_CXXStdInitializerListExpr(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitCXXTemporaryObjectExpr(clang::CXXTemporaryObjectExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_CXXTemporaryObjectExpr(c0, c1, c2);
  return true;
}

bool ArboretumASTVisitor::VisitCXXThisExpr(clang::CXXThisExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getLocation());
  uint64_t c2 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getEndLoc());
  bool c4 = D->isImplicit();
  arboretum_emit_CXXThisExpr(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitCXXThrowExpr(clang::CXXThrowExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getSubExpr());
  uint64_t c2 = context_.source_model_.resolve(D->getThrowLoc());
  bool c3 = D->isThrownVariableInScope();
  uint64_t c4 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_CXXThrowExpr(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitCXXTryStmt(clang::CXXTryStmt* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getTryLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c4 = context_.resolve(D->getTryBlock());
  uint32_t c5 = D->getNumHandlers();
  arboretum_emit_CXXTryStmt(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitCXXTypeidExpr(clang::CXXTypeidExpr* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isPotentiallyEvaluated();
  bool c2 = D->isTypeOperand();
  uint64_t c3 = context_.resolve(D->getExprOperand());
  uint64_t c4 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c6 = context_.source_model_.resolve(D->getSourceRange());
  arboretum_emit_CXXTypeidExpr(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitCXXUnresolvedConstructExpr(clang::CXXUnresolvedConstructExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getTypeAsWritten());
  uint64_t c2 = context_.source_model_.resolve(D->getLParenLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getRParenLoc());
  bool c4 = D->isListInitialization();
  uint32_t c5 = D->getNumArgs();
  uint64_t c6 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c7 = context_.source_model_.resolve(D->getEndLoc());
  {
    uint64_t idx = 0;
    for (const auto& element : D->arguments()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_CXXUnresolvedConstructExpr_arguments(c0, idx, element_id);
      ++idx;
    }
  }
  arboretum_emit_CXXUnresolvedConstructExpr(c0, c1, c2, c3, c4, c5, c6, c7);
  return true;
}

bool ArboretumASTVisitor::VisitCXXUuidofExpr(clang::CXXUuidofExpr* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isTypeOperand();
  uint64_t c2 = context_.resolve(D->getExprOperand());
  uint64_t c3 = context_.resolve(D->getGuidDecl());
  uint64_t c4 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c6 = context_.source_model_.resolve(D->getSourceRange());
  arboretum_emit_CXXUuidofExpr(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitCallExpr(clang::CallExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getCallee());
  uint64_t c2 = context_.data_model_.resolve(D->getADLCallKind());
  bool c3 = D->usesADL();
  bool c4 = D->hasStoredFPFeatures();
  uint64_t c5 = context_.resolve(D->getCalleeDecl());
  uint64_t c6 = context_.resolve(D->getDirectCallee());
  uint32_t c7 = D->getNumArgs();
  uint32_t c8 = D->getBuiltinCallee();
  uint64_t c9 = context_.source_model_.resolve(D->getRParenLoc());
  uint64_t c10 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c11 = context_.source_model_.resolve(D->getEndLoc());
  bool c12 = D->isCallToStdMove();
  arboretum_emit_CallExpr(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12);
  return true;
}

bool ArboretumASTVisitor::VisitCapturedStmt(clang::CapturedStmt* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getCapturedStmt());
  uint64_t c2 = context_.resolve(D->getCapturedDecl());
  uint64_t c3 = context_.data_model_.resolve(D->getCapturedRegionKind());
  uint64_t c4 = context_.resolve(D->getCapturedRecordDecl());
  uint32_t c5 = D->capture_size();
  uint64_t c6 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c7 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c8 = context_.source_model_.resolve(D->getSourceRange());
  {
    uint64_t idx = 0;
    for (const auto& element : D->capture_inits()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_CapturedStmt_capture_inits(c0, idx, element_id);
      ++idx;
    }
  }
  arboretum_emit_CapturedStmt(c0, c1, c2, c3, c4, c5, c6, c7, c8);
  return true;
}

bool ArboretumASTVisitor::VisitCaseStmt(clang::CaseStmt* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->caseStmtIsGNURange();
  uint64_t c2 = context_.source_model_.resolve(D->getCaseLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getEllipsisLoc());
  uint64_t c4 = context_.resolve(D->getLHS());
  uint64_t c5 = context_.resolve(D->getRHS());
  uint64_t c6 = context_.resolve(D->getSubStmt());
  uint64_t c7 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c8 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_CaseStmt(c0, c1, c2, c3, c4, c5, c6, c7, c8);
  return true;
}

bool ArboretumASTVisitor::VisitCastExpr(clang::CastExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.data_model_.resolve(D->getCastKind());
  uint64_t c2 = context_.resolve(D->getSubExpr());
  uint64_t c3 = context_.resolve(D->getSubExprAsWritten());
  uint64_t c4 = context_.resolve(D->getConversionFunction());
  bool c5 = D->path_empty();
  uint32_t c6 = D->path_size();
  bool c7 = D->hasStoredFPFeatures();
  bool c8 = D->changesVolatileQualification();
  arboretum_emit_CastExpr(c0, c1, c2, c3, c4, c5, c6, c7, c8);
  return true;
}

bool ArboretumASTVisitor::VisitCharacterLiteral(clang::CharacterLiteral* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getLocation());
  uint64_t c2 = context_.data_model_.resolve(D->getKind());
  uint64_t c3 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getEndLoc());
  uint32_t c5 = D->getValue();
  arboretum_emit_CharacterLiteral(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitChooseExpr(clang::ChooseExpr* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isConditionTrue();
  bool c2 = D->isConditionDependent();
  uint64_t c3 = context_.resolve(D->getChosenSubExpr());
  uint64_t c4 = context_.resolve(D->getCond());
  uint64_t c5 = context_.resolve(D->getLHS());
  uint64_t c6 = context_.resolve(D->getRHS());
  uint64_t c7 = context_.source_model_.resolve(D->getBuiltinLoc());
  uint64_t c8 = context_.source_model_.resolve(D->getRParenLoc());
  uint64_t c9 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c10 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_ChooseExpr(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10);
  return true;
}

bool ArboretumASTVisitor::VisitCoawaitExpr(clang::CoawaitExpr* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isImplicit();
  arboretum_emit_CoawaitExpr(c0, c1);
  return true;
}

bool ArboretumASTVisitor::VisitCompoundAssignOperator(clang::CompoundAssignOperator* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getComputationLHSType());
  uint64_t c2 = context_.resolve(D->getComputationResultType());
  arboretum_emit_CompoundAssignOperator(c0, c1, c2);
  return true;
}

bool ArboretumASTVisitor::VisitCompoundLiteralExpr(clang::CompoundLiteralExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getInitializer());
  bool c2 = D->isFileScope();
  uint64_t c3 = context_.source_model_.resolve(D->getLParenLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_CompoundLiteralExpr(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitCompoundStmt(clang::CompoundStmt* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->body_empty();
  uint32_t c2 = D->size();
  bool c3 = D->hasStoredFPFeatures();
  uint64_t c4 = context_.resolve(D->body_front());
  uint64_t c5 = context_.resolve(D->body_back());
  uint64_t c6 = context_.resolve(D->getStmtExprResult());
  uint64_t c7 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c8 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c9 = context_.source_model_.resolve(D->getLBracLoc());
  uint64_t c10 = context_.source_model_.resolve(D->getRBracLoc());
  {
    uint64_t idx = 0;
    for (const auto& element : D->body()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_CompoundStmt_body(c0, idx, element_id);
      ++idx;
    }
  }
  arboretum_emit_CompoundStmt(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10);
  return true;
}

bool ArboretumASTVisitor::VisitConceptSpecializationExpr(clang::ConceptSpecializationExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getNamedConcept());
  bool c2 = D->hasExplicitTemplateArgs();
  uint64_t c3 = context_.source_model_.resolve(D->getConceptNameLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getTemplateKWLoc());
  uint64_t c5 = context_.resolve(D->getFoundDecl());
  uint64_t c6 = context_.resolve(D->getSpecializationDecl());
  uint64_t c7 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c8 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c9 = context_.source_model_.resolve(D->getExprLoc());
  arboretum_emit_ConceptSpecializationExpr(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9);
  return true;
}

bool ArboretumASTVisitor::VisitConditionalOperator(clang::ConditionalOperator* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getCond());
  uint64_t c2 = context_.resolve(D->getTrueExpr());
  uint64_t c3 = context_.resolve(D->getFalseExpr());
  uint64_t c4 = context_.resolve(D->getLHS());
  uint64_t c5 = context_.resolve(D->getRHS());
  uint64_t c6 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c7 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_ConditionalOperator(c0, c1, c2, c3, c4, c5, c6, c7);
  return true;
}

bool ArboretumASTVisitor::VisitConstantExpr(clang::ConstantExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c3 = context_.data_model_.resolve(D->getResultAPValueKind());
  uint64_t c4 = context_.data_model_.resolve(D->getResultStorageKind());
  bool c5 = D->isImmediateInvocation();
  bool c6 = D->hasAPValueResult();
  arboretum_emit_ConstantExpr(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitContinueStmt(clang::ContinueStmt* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getContinueLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_ContinueStmt(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitConvertVectorExpr(clang::ConvertVectorExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getSrcExpr());
  uint64_t c2 = context_.source_model_.resolve(D->getBuiltinLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getRParenLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_ConvertVectorExpr(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitCoreturnStmt(clang::CoreturnStmt* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getKeywordLoc());
  uint64_t c2 = context_.resolve(D->getOperand());
  uint64_t c3 = context_.resolve(D->getPromiseCall());
  bool c4 = D->isImplicit();
  uint64_t c5 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c6 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_CoreturnStmt(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitCoroutineBodyStmt(clang::CoroutineBodyStmt* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->hasDependentPromiseType();
  uint64_t c2 = context_.resolve(D->getBody());
  uint64_t c3 = context_.resolve(D->getPromiseDeclStmt());
  uint64_t c4 = context_.resolve(D->getPromiseDecl());
  uint64_t c5 = context_.resolve(D->getInitSuspendStmt());
  uint64_t c6 = context_.resolve(D->getFinalSuspendStmt());
  uint64_t c7 = context_.resolve(D->getExceptionHandler());
  uint64_t c8 = context_.resolve(D->getFallthroughHandler());
  uint64_t c9 = context_.resolve(D->getAllocate());
  uint64_t c10 = context_.resolve(D->getDeallocate());
  uint64_t c11 = context_.resolve(D->getResultDecl());
  uint64_t c12 = context_.resolve(D->getReturnValueInit());
  uint64_t c13 = context_.resolve(D->getReturnValue());
  uint64_t c14 = context_.resolve(D->getReturnStmt());
  uint64_t c15 = context_.resolve(D->getReturnStmtOnAllocFailure());
  uint64_t c16 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c17 = context_.source_model_.resolve(D->getEndLoc());
  {
    uint64_t idx = 0;
    for (const auto& element : D->getParamMoves()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_CoroutineBodyStmt_getParamMoves(c0, idx, element_id);
    }
  }
  arboretum_emit_CoroutineBodyStmt(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17);
  return true;
}

bool ArboretumASTVisitor::VisitCoroutineSuspendExpr(clang::CoroutineSuspendExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getCommonExpr());
  uint64_t c2 = context_.resolve(D->getOpaqueValue());
  uint64_t c3 = context_.resolve(D->getReadyExpr());
  uint64_t c4 = context_.resolve(D->getSuspendExpr());
  uint64_t c5 = context_.resolve(D->getResumeExpr());
  uint64_t c6 = context_.resolve(D->getOperand());
  uint64_t c7 = context_.source_model_.resolve(D->getKeywordLoc());
  uint64_t c8 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c9 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_CoroutineSuspendExpr(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9);
  return true;
}

bool ArboretumASTVisitor::VisitCoyieldExpr(clang::CoyieldExpr* D) {
  uint64_t c0 = context_.resolve(D);
  arboretum_emit_CoyieldExpr(c0);
  return true;
}

bool ArboretumASTVisitor::VisitDeclRefExpr(clang::DeclRefExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getDecl());
  uint64_t c2 = context_.source_model_.resolve(D->getLocation());
  uint64_t c3 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getEndLoc());
  bool c5 = D->hasQualifier();
  uint64_t c6 = context_.resolve(D->getFoundDecl());
  bool c7 = D->hasTemplateKWAndArgsInfo();
  uint64_t c8 = context_.source_model_.resolve(D->getTemplateKeywordLoc());
  uint64_t c9 = context_.source_model_.resolve(D->getLAngleLoc());
  uint64_t c10 = context_.source_model_.resolve(D->getRAngleLoc());
  bool c11 = D->hasTemplateKeyword();
  bool c12 = D->hasExplicitTemplateArgs();
  uint32_t c13 = D->getNumTemplateArgs();
  bool c14 = D->hadMultipleCandidates();
  uint64_t c15 = context_.data_model_.resolve(D->isNonOdrUse());
  bool c16 = D->refersToEnclosingVariableOrCapture();
  bool c17 = D->isImmediateEscalating();
  bool c18 = D->isCapturedByCopyInLambdaWithExplicitObjectParameter();
  arboretum_emit_DeclRefExpr(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18);
  return true;
}

bool ArboretumASTVisitor::VisitDeclStmt(clang::DeclStmt* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isSingleDecl();
  uint64_t c2 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getBeginLoc());
  {
    uint64_t idx = 0;
    for (const auto& element : D->decls()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_DeclStmt_decls(c0, idx, element_id);
      ++idx;
    }
  }
  arboretum_emit_DeclStmt(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitDefaultStmt(clang::DefaultStmt* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getSubStmt());
  uint64_t c2 = context_.source_model_.resolve(D->getDefaultLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_DefaultStmt(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitDependentCoawaitExpr(clang::DependentCoawaitExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getOperand());
  uint64_t c2 = context_.resolve(D->getOperatorCoawaitLookup());
  uint64_t c3 = context_.source_model_.resolve(D->getKeywordLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_DependentCoawaitExpr(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitDependentScopeDeclRefExpr(clang::DependentScopeDeclRefExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getLocation());
  uint64_t c2 = context_.source_model_.resolve(D->getTemplateKeywordLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getLAngleLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getRAngleLoc());
  bool c5 = D->hasTemplateKeyword();
  bool c6 = D->hasExplicitTemplateArgs();
  uint32_t c7 = D->getNumTemplateArgs();
  uint64_t c8 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c9 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_DependentScopeDeclRefExpr(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9);
  return true;
}

bool ArboretumASTVisitor::VisitDesignatedInitExpr(clang::DesignatedInitExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint32_t c1 = D->size();
  uint64_t c2 = context_.source_model_.resolve(D->getEqualOrColonLoc());
  bool c3 = D->isDirectInit();
  bool c4 = D->usesGNUSyntax();
  uint64_t c5 = context_.resolve(D->getInit());
  uint32_t c6 = D->getNumSubExprs();
  uint64_t c7 = context_.source_model_.resolve(D->getDesignatorsSourceRange());
  uint64_t c8 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c9 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_DesignatedInitExpr(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9);
  return true;
}

bool ArboretumASTVisitor::VisitDesignatedInitUpdateExpr(clang::DesignatedInitUpdateExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c3 = context_.resolve(D->getBase());
  uint64_t c4 = context_.resolve(D->getUpdater());
  arboretum_emit_DesignatedInitUpdateExpr(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitDoStmt(clang::DoStmt* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getCond());
  uint64_t c2 = context_.resolve(D->getBody());
  uint64_t c3 = context_.source_model_.resolve(D->getDoLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getWhileLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getRParenLoc());
  uint64_t c6 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c7 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_DoStmt(c0, c1, c2, c3, c4, c5, c6, c7);
  return true;
}

bool ArboretumASTVisitor::VisitExplicitCastExpr(clang::ExplicitCastExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getTypeAsWritten());
  arboretum_emit_ExplicitCastExpr(c0, c1);
  return true;
}

bool ArboretumASTVisitor::VisitExpr(clang::Expr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getType());
  uint64_t c2 = context_.data_model_.resolve(D->getDependence());
  bool c3 = D->isValueDependent();
  bool c4 = D->isTypeDependent();
  bool c5 = D->isInstantiationDependent();
  bool c6 = D->containsUnexpandedParameterPack();
  bool c7 = D->containsErrors();
  bool c8 = D->isLValue();
  bool c9 = D->isPRValue();
  bool c10 = D->isXValue();
  bool c11 = D->isGLValue();
  uint64_t c12 = context_.data_model_.resolve(D->getValueKind());
  uint64_t c13 = context_.data_model_.resolve(D->getObjectKind());
  bool c14 = D->isOrdinaryOrBitFieldObject();
  bool c15 = D->refersToBitField();
  uint64_t c16 = context_.resolve(D->getSourceBitField());
  uint64_t c17 = context_.resolve(D->getReferencedDeclOfCallee());
  uint64_t c18 = context_.resolve(D->getObjCProperty());
  bool c19 = D->isObjCSelfExpr();
  bool c20 = D->refersToVectorElement();
  bool c21 = D->refersToMatrixElement();
  bool c22 = D->refersToGlobalRegisterVar();
  uint64_t c23 = context_.resolve(D->IgnoreImpCasts());
  uint64_t c24 = context_.resolve(D->IgnoreCasts());
  uint64_t c25 = context_.resolve(D->IgnoreImplicit());
  uint64_t c26 = context_.resolve(D->IgnoreImplicitAsWritten());
  uint64_t c27 = context_.resolve(D->IgnoreParens());
  uint64_t c28 = context_.resolve(D->IgnoreParenImpCasts());
  uint64_t c29 = context_.resolve(D->IgnoreParenCasts());
  uint64_t c30 = context_.resolve(D->IgnoreConversionOperatorSingleStep());
  uint64_t c31 = context_.resolve(D->IgnoreParenLValueCasts());
  uint64_t c32 = context_.resolve(D->IgnoreParenBaseCasts());
  bool c33 = D->isDefaultArgument();
  bool c34 = D->isImplicitCXXThis();
  uint64_t c35 = context_.resolve(D->skipRValueSubobjectAdjustments());
  arboretum_emit_Expr(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19, c20, c21, c22, c23, c24, c25, c26, c27, c28, c29, c30, c31, c32, c33, c34, c35);
  return true;
}

bool ArboretumASTVisitor::VisitExprWithCleanups(clang::ExprWithCleanups* D) {
  uint64_t c0 = context_.resolve(D);
  uint32_t c1 = D->getNumObjects();
  bool c2 = D->cleanupsHaveSideEffects();
  uint64_t c3 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_ExprWithCleanups(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitExpressionTraitExpr(clang::ExpressionTraitExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c3 = context_.data_model_.resolve(D->getTrait());
  uint64_t c4 = context_.resolve(D->getQueriedExpression());
  bool c5 = D->getValue();
  arboretum_emit_ExpressionTraitExpr(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitExtVectorElementExpr(clang::ExtVectorElementExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getBase());
  uint64_t c2 = context_.source_model_.resolve(D->getAccessorLoc());
  uint32_t c3 = D->getNumElements();
  bool c4 = D->containsDuplicateElements();
  uint64_t c5 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c6 = context_.source_model_.resolve(D->getEndLoc());
  bool c7 = D->isArrow();
  arboretum_emit_ExtVectorElementExpr(c0, c1, c2, c3, c4, c5, c6, c7);
  return true;
}

bool ArboretumASTVisitor::VisitFixedPointLiteral(clang::FixedPointLiteral* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getLocation());
  uint32_t c4 = D->getScale();
  arboretum_emit_FixedPointLiteral(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitFloatingLiteral(clang::FloatingLiteral* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.data_model_.resolve(D->getRawSemantics());
  bool c2 = D->isExact();
  double c3 = D->getValueAsApproximateDouble();
  uint64_t c4 = context_.source_model_.resolve(D->getLocation());
  uint64_t c5 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c6 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_FloatingLiteral(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitForStmt(clang::ForStmt* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getConditionVariable());
  uint64_t c2 = context_.resolve(D->getConditionVariableDeclStmt());
  uint64_t c3 = context_.resolve(D->getInit());
  uint64_t c4 = context_.resolve(D->getCond());
  uint64_t c5 = context_.resolve(D->getInc());
  uint64_t c6 = context_.resolve(D->getBody());
  uint64_t c7 = context_.source_model_.resolve(D->getForLoc());
  uint64_t c8 = context_.source_model_.resolve(D->getLParenLoc());
  uint64_t c9 = context_.source_model_.resolve(D->getRParenLoc());
  uint64_t c10 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c11 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_ForStmt(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11);
  return true;
}

bool ArboretumASTVisitor::VisitFullExpr(clang::FullExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getSubExpr());
  arboretum_emit_FullExpr(c0, c1);
  return true;
}

bool ArboretumASTVisitor::VisitFunctionParmPackExpr(clang::FunctionParmPackExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getParameterPack());
  uint64_t c2 = context_.source_model_.resolve(D->getParameterPackLocation());
  uint32_t c3 = D->getNumExpansions();
  uint64_t c4 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_FunctionParmPackExpr(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitGCCAsmStmt(clang::GCCAsmStmt* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getRParenLoc());
  uint64_t c2 = context_.resolve(D->getAsmString());
  bool c3 = D->isAsmGoto();
  uint32_t c4 = D->getNumLabels();
  uint64_t c5 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c6 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_GCCAsmStmt(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitGNUNullExpr(clang::GNUNullExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getTokenLocation());
  uint64_t c2 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_GNUNullExpr(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitGenericSelectionExpr(clang::GenericSelectionExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint32_t c1 = D->getNumAssocs();
  uint32_t c2 = D->getResultIndex();
  bool c3 = D->isResultDependent();
  bool c4 = D->isExprPredicate();
  bool c5 = D->isTypePredicate();
  uint64_t c6 = context_.resolve(D->getControllingExpr());
  uint64_t c7 = context_.resolve(D->getResultExpr());
  uint64_t c8 = context_.source_model_.resolve(D->getGenericLoc());
  uint64_t c9 = context_.source_model_.resolve(D->getDefaultLoc());
  uint64_t c10 = context_.source_model_.resolve(D->getRParenLoc());
  uint64_t c11 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c12 = context_.source_model_.resolve(D->getEndLoc());
  {
    uint64_t idx = 0;
    for (const auto& element : D->getAssocExprs()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_GenericSelectionExpr_getAssocExprs(c0, idx, element_id);
    }
  }
  arboretum_emit_GenericSelectionExpr(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12);
  return true;
}

bool ArboretumASTVisitor::VisitGotoStmt(clang::GotoStmt* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getLabel());
  uint64_t c2 = context_.source_model_.resolve(D->getGotoLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getLabelLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_GotoStmt(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitIfStmt(clang::IfStmt* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->hasInitStorage();
  bool c2 = D->hasVarStorage();
  bool c3 = D->hasElseStorage();
  uint64_t c4 = context_.resolve(D->getCond());
  uint64_t c5 = context_.resolve(D->getThen());
  uint64_t c6 = context_.resolve(D->getElse());
  uint64_t c7 = context_.resolve(D->getConditionVariable());
  uint64_t c8 = context_.resolve(D->getConditionVariableDeclStmt());
  uint64_t c9 = context_.resolve(D->getInit());
  uint64_t c10 = context_.source_model_.resolve(D->getIfLoc());
  uint64_t c11 = context_.source_model_.resolve(D->getElseLoc());
  bool c12 = D->isConsteval();
  bool c13 = D->isNonNegatedConsteval();
  bool c14 = D->isNegatedConsteval();
  bool c15 = D->isConstexpr();
  uint64_t c16 = context_.data_model_.resolve(D->getStatementKind());
  bool c17 = D->isObjCAvailabilityCheck();
  uint64_t c18 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c19 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c20 = context_.source_model_.resolve(D->getLParenLoc());
  uint64_t c21 = context_.source_model_.resolve(D->getRParenLoc());
  arboretum_emit_IfStmt(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19, c20, c21);
  return true;
}

bool ArboretumASTVisitor::VisitImaginaryLiteral(clang::ImaginaryLiteral* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getSubExpr());
  uint64_t c2 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_ImaginaryLiteral(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitImplicitCastExpr(clang::ImplicitCastExpr* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isPartOfExplicitCast();
  uint64_t c2 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_ImplicitCastExpr(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitImplicitValueInitExpr(clang::ImplicitValueInitExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_ImplicitValueInitExpr(c0, c1, c2);
  return true;
}

bool ArboretumASTVisitor::VisitIndirectGotoStmt(clang::IndirectGotoStmt* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getGotoLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getStarLoc());
  uint64_t c3 = context_.resolve(D->getTarget());
  uint64_t c4 = context_.resolve(D->getConstantTarget());
  uint64_t c5 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c6 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_IndirectGotoStmt(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitInitListExpr(clang::InitListExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint32_t c1 = D->getNumInits();
  uint64_t c2 = context_.resolve(D->getArrayFiller());
  bool c3 = D->hasArrayFiller();
  bool c4 = D->hasDesignatedInit();
  uint64_t c5 = context_.resolve(D->getInitializedFieldInUnion());
  bool c6 = D->isExplicit();
  bool c7 = D->isStringLiteralInit();
  bool c8 = D->isTransparent();
  uint64_t c9 = context_.source_model_.resolve(D->getLBraceLoc());
  uint64_t c10 = context_.source_model_.resolve(D->getRBraceLoc());
  bool c11 = D->isSemanticForm();
  uint64_t c12 = context_.resolve(D->getSemanticForm());
  bool c13 = D->isSyntacticForm();
  uint64_t c14 = context_.resolve(D->getSyntacticForm());
  bool c15 = D->hadArrayRangeDesignator();
  uint64_t c16 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c17 = context_.source_model_.resolve(D->getEndLoc());
  {
    uint64_t idx = 0;
    for (const auto& element : D->inits()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_InitListExpr_inits(c0, idx, element_id);
    }
  }
  arboretum_emit_InitListExpr(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17);
  return true;
}

bool ArboretumASTVisitor::VisitIntegerLiteral(clang::IntegerLiteral* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getLocation());
  arboretum_emit_IntegerLiteral(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitLabelStmt(clang::LabelStmt* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getIdentLoc());
  uint64_t c2 = context_.resolve(D->getDecl());
  uint64_t c3 = context_.resolve(D->getSubStmt());
  uint64_t c4 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getEndLoc());
  bool c6 = D->isSideEntry();
  arboretum_emit_LabelStmt(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitLambdaExpr(clang::LambdaExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.data_model_.resolve(D->getCaptureDefault());
  uint64_t c2 = context_.source_model_.resolve(D->getCaptureDefaultLoc());
  uint32_t c3 = D->capture_size();
  uint64_t c4 = context_.source_model_.resolve(D->getIntroducerRange());
  uint64_t c5 = context_.resolve(D->getLambdaClass());
  uint64_t c6 = context_.resolve(D->getCallOperator());
  uint64_t c7 = context_.resolve(D->getDependentCallOperator());
  uint64_t c8 = context_.resolve(D->getTrailingRequiresClause());
  bool c9 = D->isGenericLambda();
  uint64_t c10 = context_.resolve(D->getBody());
  uint64_t c11 = context_.resolve(D->getCompoundStmtBody());
  bool c12 = D->isMutable();
  bool c13 = D->hasExplicitParameters();
  bool c14 = D->hasExplicitResultType();
  uint64_t c15 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c16 = context_.source_model_.resolve(D->getEndLoc());
  {
    uint64_t idx = 0;
    for (const auto& element : D->capture_inits()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_LambdaExpr_capture_inits(c0, idx, element_id);
      ++idx;
    }
  }
  {
    uint64_t idx = 0;
    for (const auto& element : D->getExplicitTemplateParameters()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_LambdaExpr_getExplicitTemplateParameters(c0, idx, element_id);
    }
  }
  arboretum_emit_LambdaExpr(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16);
  return true;
}

bool ArboretumASTVisitor::VisitMSAsmStmt(clang::MSAsmStmt* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getLBraceLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getEndLoc());
  bool c3 = D->hasBraces();
  const char* c4 = D->getAsmString().data();
  uint64_t c5 = context_.source_model_.resolve(D->getBeginLoc());
  {
    uint64_t idx = 0;
    for (const auto& element : D->getAllConstraints()) {
      const char* element_id = element.data();
      arboretum_emit_MSAsmStmt_getAllConstraints(c0, idx, element_id);
    }
  }
  {
    uint64_t idx = 0;
    for (const auto& element : D->getClobbers()) {
      const char* element_id = element.data();
      arboretum_emit_MSAsmStmt_getClobbers(c0, idx, element_id);
    }
  }
  {
    uint64_t idx = 0;
    for (const auto& element : D->getAllExprs()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_MSAsmStmt_getAllExprs(c0, idx, element_id);
    }
  }
  arboretum_emit_MSAsmStmt(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitMSDependentExistsStmt(clang::MSDependentExistsStmt* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getKeywordLoc());
  bool c2 = D->isIfExists();
  bool c3 = D->isIfNotExists();
  uint64_t c4 = context_.resolve(D->getSubStmt());
  uint64_t c5 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c6 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_MSDependentExistsStmt(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitMSPropertyRefExpr(clang::MSPropertyRefExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getSourceRange());
  bool c2 = D->isImplicitAccess();
  uint64_t c3 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c5 = context_.resolve(D->getBaseExpr());
  uint64_t c6 = context_.resolve(D->getPropertyDecl());
  bool c7 = D->isArrow();
  uint64_t c8 = context_.source_model_.resolve(D->getMemberLoc());
  arboretum_emit_MSPropertyRefExpr(c0, c1, c2, c3, c4, c5, c6, c7, c8);
  return true;
}

bool ArboretumASTVisitor::VisitMSPropertySubscriptExpr(clang::MSPropertySubscriptExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getBase());
  uint64_t c2 = context_.resolve(D->getIdx());
  uint64_t c3 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getRBracketLoc());
  uint64_t c6 = context_.source_model_.resolve(D->getExprLoc());
  arboretum_emit_MSPropertySubscriptExpr(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitMaterializeTemporaryExpr(clang::MaterializeTemporaryExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getSubExpr());
  uint64_t c2 = context_.data_model_.resolve(D->getStorageDuration());
  uint64_t c3 = context_.resolve(D->getLifetimeExtendedTemporaryDecl());
  uint64_t c4 = context_.resolve(D->getExtendingDecl());
  uint32_t c5 = D->getManglingNumber();
  bool c6 = D->isBoundToLvalueReference();
  uint64_t c7 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c8 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_MaterializeTemporaryExpr(c0, c1, c2, c3, c4, c5, c6, c7, c8);
  return true;
}

bool ArboretumASTVisitor::VisitMatrixSubscriptExpr(clang::MatrixSubscriptExpr* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isIncomplete();
  uint64_t c2 = context_.resolve(D->getBase());
  uint64_t c3 = context_.resolve(D->getRowIdx());
  uint64_t c4 = context_.resolve(D->getColumnIdx());
  uint64_t c5 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c6 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c7 = context_.source_model_.resolve(D->getExprLoc());
  uint64_t c8 = context_.source_model_.resolve(D->getRBracketLoc());
  arboretum_emit_MatrixSubscriptExpr(c0, c1, c2, c3, c4, c5, c6, c7, c8);
  return true;
}

bool ArboretumASTVisitor::VisitMemberExpr(clang::MemberExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getBase());
  uint64_t c2 = context_.resolve(D->getMemberDecl());
  bool c3 = D->hasQualifier();
  uint64_t c4 = context_.source_model_.resolve(D->getTemplateKeywordLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getLAngleLoc());
  uint64_t c6 = context_.source_model_.resolve(D->getRAngleLoc());
  bool c7 = D->hasTemplateKeyword();
  bool c8 = D->hasExplicitTemplateArgs();
  uint32_t c9 = D->getNumTemplateArgs();
  uint64_t c10 = context_.source_model_.resolve(D->getOperatorLoc());
  bool c11 = D->isArrow();
  uint64_t c12 = context_.source_model_.resolve(D->getMemberLoc());
  uint64_t c13 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c14 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c15 = context_.source_model_.resolve(D->getExprLoc());
  bool c16 = D->isImplicitAccess();
  bool c17 = D->hadMultipleCandidates();
  uint64_t c18 = context_.data_model_.resolve(D->isNonOdrUse());
  arboretum_emit_MemberExpr(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18);
  return true;
}

bool ArboretumASTVisitor::VisitNoInitExpr(clang::NoInitExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_NoInitExpr(c0, c1, c2);
  return true;
}

bool ArboretumASTVisitor::VisitNullStmt(clang::NullStmt* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getSemiLoc());
  bool c2 = D->hasLeadingEmptyMacro();
  uint64_t c3 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_NullStmt(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitOMPArraySectionExpr(clang::OMPArraySectionExpr* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPArrayShapingExpr(clang::OMPArrayShapingExpr* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPAtomicDirective(clang::OMPAtomicDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPBarrierDirective(clang::OMPBarrierDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPCancelDirective(clang::OMPCancelDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPCancellationPointDirective(clang::OMPCancellationPointDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPCanonicalLoop(clang::OMPCanonicalLoop* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPCriticalDirective(clang::OMPCriticalDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPDepobjDirective(clang::OMPDepobjDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPDispatchDirective(clang::OMPDispatchDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPDistributeDirective(clang::OMPDistributeDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPDistributeParallelForDirective(clang::OMPDistributeParallelForDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPDistributeParallelForSimdDirective(clang::OMPDistributeParallelForSimdDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPDistributeSimdDirective(clang::OMPDistributeSimdDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPErrorDirective(clang::OMPErrorDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPExecutableDirective(clang::OMPExecutableDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPFlushDirective(clang::OMPFlushDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPForDirective(clang::OMPForDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPForSimdDirective(clang::OMPForSimdDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPGenericLoopDirective(clang::OMPGenericLoopDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPInteropDirective(clang::OMPInteropDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPIteratorExpr(clang::OMPIteratorExpr* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPLoopBasedDirective(clang::OMPLoopBasedDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPLoopDirective(clang::OMPLoopDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPLoopTransformationDirective(clang::OMPLoopTransformationDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPMaskedDirective(clang::OMPMaskedDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPMaskedTaskLoopDirective(clang::OMPMaskedTaskLoopDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPMaskedTaskLoopSimdDirective(clang::OMPMaskedTaskLoopSimdDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPMasterDirective(clang::OMPMasterDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPMasterTaskLoopDirective(clang::OMPMasterTaskLoopDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPMasterTaskLoopSimdDirective(clang::OMPMasterTaskLoopSimdDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPMetaDirective(clang::OMPMetaDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPOrderedDirective(clang::OMPOrderedDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelDirective(clang::OMPParallelDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelForDirective(clang::OMPParallelForDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelForSimdDirective(clang::OMPParallelForSimdDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelGenericLoopDirective(clang::OMPParallelGenericLoopDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelMaskedDirective(clang::OMPParallelMaskedDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelMaskedTaskLoopDirective(clang::OMPParallelMaskedTaskLoopDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelMaskedTaskLoopSimdDirective(clang::OMPParallelMaskedTaskLoopSimdDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelMasterDirective(clang::OMPParallelMasterDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelMasterTaskLoopDirective(clang::OMPParallelMasterTaskLoopDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelMasterTaskLoopSimdDirective(clang::OMPParallelMasterTaskLoopSimdDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelSectionsDirective(clang::OMPParallelSectionsDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPScanDirective(clang::OMPScanDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPScopeDirective(clang::OMPScopeDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPSectionDirective(clang::OMPSectionDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPSectionsDirective(clang::OMPSectionsDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPSimdDirective(clang::OMPSimdDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPSingleDirective(clang::OMPSingleDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetDataDirective(clang::OMPTargetDataDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetDirective(clang::OMPTargetDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetEnterDataDirective(clang::OMPTargetEnterDataDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetExitDataDirective(clang::OMPTargetExitDataDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetParallelDirective(clang::OMPTargetParallelDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetParallelForDirective(clang::OMPTargetParallelForDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetParallelForSimdDirective(clang::OMPTargetParallelForSimdDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetParallelGenericLoopDirective(clang::OMPTargetParallelGenericLoopDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetSimdDirective(clang::OMPTargetSimdDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetTeamsDirective(clang::OMPTargetTeamsDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetTeamsDistributeDirective(clang::OMPTargetTeamsDistributeDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetTeamsDistributeParallelForDirective(clang::OMPTargetTeamsDistributeParallelForDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetTeamsDistributeParallelForSimdDirective(clang::OMPTargetTeamsDistributeParallelForSimdDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetTeamsDistributeSimdDirective(clang::OMPTargetTeamsDistributeSimdDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetTeamsGenericLoopDirective(clang::OMPTargetTeamsGenericLoopDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetUpdateDirective(clang::OMPTargetUpdateDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTaskDirective(clang::OMPTaskDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTaskLoopDirective(clang::OMPTaskLoopDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTaskLoopSimdDirective(clang::OMPTaskLoopSimdDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTaskgroupDirective(clang::OMPTaskgroupDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTaskwaitDirective(clang::OMPTaskwaitDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTaskyieldDirective(clang::OMPTaskyieldDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTeamsDirective(clang::OMPTeamsDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTeamsDistributeDirective(clang::OMPTeamsDistributeDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTeamsDistributeParallelForDirective(clang::OMPTeamsDistributeParallelForDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTeamsDistributeParallelForSimdDirective(clang::OMPTeamsDistributeParallelForSimdDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTeamsDistributeSimdDirective(clang::OMPTeamsDistributeSimdDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTeamsGenericLoopDirective(clang::OMPTeamsGenericLoopDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPTileDirective(clang::OMPTileDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOMPUnrollDirective(clang::OMPUnrollDirective* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCArrayLiteral(clang::ObjCArrayLiteral* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCAtCatchStmt(clang::ObjCAtCatchStmt* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCAtFinallyStmt(clang::ObjCAtFinallyStmt* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCAtSynchronizedStmt(clang::ObjCAtSynchronizedStmt* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCAtThrowStmt(clang::ObjCAtThrowStmt* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCAtTryStmt(clang::ObjCAtTryStmt* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCAutoreleasePoolStmt(clang::ObjCAutoreleasePoolStmt* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCAvailabilityCheckExpr(clang::ObjCAvailabilityCheckExpr* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCBoolLiteralExpr(clang::ObjCBoolLiteralExpr* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCBoxedExpr(clang::ObjCBoxedExpr* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCBridgedCastExpr(clang::ObjCBridgedCastExpr* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCDictionaryLiteral(clang::ObjCDictionaryLiteral* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCEncodeExpr(clang::ObjCEncodeExpr* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCForCollectionStmt(clang::ObjCForCollectionStmt* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCIndirectCopyRestoreExpr(clang::ObjCIndirectCopyRestoreExpr* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCIsaExpr(clang::ObjCIsaExpr* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCIvarRefExpr(clang::ObjCIvarRefExpr* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCMessageExpr(clang::ObjCMessageExpr* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCPropertyRefExpr(clang::ObjCPropertyRefExpr* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCProtocolExpr(clang::ObjCProtocolExpr* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCSelectorExpr(clang::ObjCSelectorExpr* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCStringLiteral(clang::ObjCStringLiteral* D) {
  return true;
}

bool ArboretumASTVisitor::VisitObjCSubscriptRefExpr(clang::ObjCSubscriptRefExpr* D) {
  return true;
}

bool ArboretumASTVisitor::VisitOffsetOfExpr(clang::OffsetOfExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getOperatorLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getRParenLoc());
  uint32_t c3 = D->getNumComponents();
  uint32_t c4 = D->getNumExpressions();
  uint64_t c5 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c6 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_OffsetOfExpr(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitOpaqueValueExpr(clang::OpaqueValueExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getLocation());
  uint64_t c2 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getExprLoc());
  uint64_t c5 = context_.resolve(D->getSourceExpr());
  bool c6 = D->isUnique();
  arboretum_emit_OpaqueValueExpr(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitOverloadExpr(clang::OverloadExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getNamingClass());
  uint32_t c2 = D->getNumDecls();
  uint64_t c3 = context_.source_model_.resolve(D->getNameLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getTemplateKeywordLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getLAngleLoc());
  uint64_t c6 = context_.source_model_.resolve(D->getRAngleLoc());
  bool c7 = D->hasTemplateKeyword();
  bool c8 = D->hasExplicitTemplateArgs();
  uint32_t c9 = D->getNumTemplateArgs();
  arboretum_emit_OverloadExpr(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9);
  return true;
}

bool ArboretumASTVisitor::VisitPackExpansionExpr(clang::PackExpansionExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getPattern());
  uint64_t c2 = context_.source_model_.resolve(D->getEllipsisLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_PackExpansionExpr(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitParenExpr(clang::ParenExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getSubExpr());
  uint64_t c2 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getLParen());
  uint64_t c5 = context_.source_model_.resolve(D->getRParen());
  arboretum_emit_ParenExpr(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitParenListExpr(clang::ParenListExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint32_t c1 = D->getNumExprs();
  uint64_t c2 = context_.source_model_.resolve(D->getLParenLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getRParenLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_ParenListExpr(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitPredefinedExpr(clang::PredefinedExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.data_model_.resolve(D->getIdentKind());
  bool c2 = D->isTransparent();
  uint64_t c3 = context_.source_model_.resolve(D->getLocation());
  uint64_t c4 = context_.resolve(D->getFunctionName());
  const char* c5 = D->getIdentKindName().data();
  uint64_t c6 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c7 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_PredefinedExpr(c0, c1, c2, c3, c4, c5, c6, c7);
  return true;
}

bool ArboretumASTVisitor::VisitPseudoObjectExpr(clang::PseudoObjectExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getSyntacticForm());
  uint32_t c2 = D->getResultExprIndex();
  uint64_t c3 = context_.resolve(D->getResultExpr());
  uint32_t c4 = D->getNumSemanticExprs();
  uint64_t c5 = context_.source_model_.resolve(D->getExprLoc());
  uint64_t c6 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c7 = context_.source_model_.resolve(D->getEndLoc());
  {
    uint64_t idx = 0;
    for (const auto& element : D->semantics()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_PseudoObjectExpr_semantics(c0, idx, element_id);
    }
  }
  arboretum_emit_PseudoObjectExpr(c0, c1, c2, c3, c4, c5, c6, c7);
  return true;
}

bool ArboretumASTVisitor::VisitRecoveryExpr(clang::RecoveryExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getEndLoc());
  {
    uint64_t idx = 0;
    for (const auto& element : D->subExpressions()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_RecoveryExpr_subExpressions(c0, idx, element_id);
    }
  }
  arboretum_emit_RecoveryExpr(c0, c1, c2);
  return true;
}

bool ArboretumASTVisitor::VisitRequiresExpr(clang::RequiresExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getBody());
  uint64_t c2 = context_.source_model_.resolve(D->getRequiresKWLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getLParenLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getRParenLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getRBraceLoc());
  uint64_t c6 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c7 = context_.source_model_.resolve(D->getEndLoc());
  {
    uint64_t idx = 0;
    for (const auto& element : D->getLocalParameters()) {
      uint64_t element_id = context_.resolve(element);
      arboretum_emit_RequiresExpr_getLocalParameters(c0, idx, element_id);
    }
  }
  arboretum_emit_RequiresExpr(c0, c1, c2, c3, c4, c5, c6, c7);
  return true;
}

bool ArboretumASTVisitor::VisitReturnStmt(clang::ReturnStmt* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getRetValue());
  uint64_t c2 = context_.resolve(D->getNRVOCandidate());
  uint64_t c3 = context_.source_model_.resolve(D->getReturnLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_ReturnStmt(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitSEHExceptStmt(clang::SEHExceptStmt* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getExceptLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c4 = context_.resolve(D->getFilterExpr());
  uint64_t c5 = context_.resolve(D->getBlock());
  arboretum_emit_SEHExceptStmt(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitSEHFinallyStmt(clang::SEHFinallyStmt* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getFinallyLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c4 = context_.resolve(D->getBlock());
  arboretum_emit_SEHFinallyStmt(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitSEHLeaveStmt(clang::SEHLeaveStmt* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getLeaveLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_SEHLeaveStmt(c0, c1, c2, c3);
  return true;
}

bool ArboretumASTVisitor::VisitSEHTryStmt(clang::SEHTryStmt* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getTryLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getEndLoc());
  bool c4 = D->getIsCXXTry();
  uint64_t c5 = context_.resolve(D->getTryBlock());
  uint64_t c6 = context_.resolve(D->getHandler());
  uint64_t c7 = context_.resolve(D->getExceptHandler());
  uint64_t c8 = context_.resolve(D->getFinallyHandler());
  arboretum_emit_SEHTryStmt(c0, c1, c2, c3, c4, c5, c6, c7, c8);
  return true;
}

bool ArboretumASTVisitor::VisitSYCLUniqueStableNameExpr(clang::SYCLUniqueStableNameExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getLocation());
  uint64_t c4 = context_.source_model_.resolve(D->getLParenLocation());
  uint64_t c5 = context_.source_model_.resolve(D->getRParenLocation());
  arboretum_emit_SYCLUniqueStableNameExpr(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitShuffleVectorExpr(clang::ShuffleVectorExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getBuiltinLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getRParenLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getEndLoc());
  uint32_t c5 = D->getNumSubExprs();
  arboretum_emit_ShuffleVectorExpr(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitSizeOfPackExpr(clang::SizeOfPackExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getOperatorLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getPackLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getRParenLoc());
  uint64_t c4 = context_.resolve(D->getPack());
  bool c5 = D->isPartiallySubstituted();
  uint64_t c6 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c7 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_SizeOfPackExpr(c0, c1, c2, c3, c4, c5, c6, c7);
  return true;
}

bool ArboretumASTVisitor::VisitSourceLocExpr(clang::SourceLocExpr* D) {
  uint64_t c0 = context_.resolve(D);
  const char* c1 = D->getBuiltinStr().data();
  uint64_t c2 = context_.data_model_.resolve(D->getIdentKind());
  bool c3 = D->isIntType();
  uint64_t c4 = context_.source_model_.resolve(D->getLocation());
  uint64_t c5 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c6 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_SourceLocExpr(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitStmt(clang::Stmt* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->stripLabelLikeStatements());
  arboretum_emit_Stmt(c0, c1);
  return true;
}

bool ArboretumASTVisitor::VisitStmtExpr(clang::StmtExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getSubStmt());
  uint64_t c2 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getLParenLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getRParenLoc());
  uint32_t c6 = D->getTemplateDepth();
  arboretum_emit_StmtExpr(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitStringLiteral(clang::StringLiteral* D) {
  uint64_t c0 = context_.resolve(D);
  const char* c1 = D->getString().data();
  const char* c2 = D->getBytes().data();
  uint32_t c3 = D->getByteLength();
  uint32_t c4 = D->getLength();
  uint32_t c5 = D->getCharByteWidth();
  uint64_t c6 = context_.data_model_.resolve(D->getKind());
  bool c7 = D->isOrdinary();
  bool c8 = D->isWide();
  bool c9 = D->isUTF8();
  bool c10 = D->isUTF16();
  bool c11 = D->isUTF32();
  bool c12 = D->isUnevaluated();
  bool c13 = D->isPascal();
  bool c14 = D->containsNonAscii();
  bool c15 = D->containsNonAsciiOrNull();
  uint32_t c16 = D->getNumConcatenated();
  uint64_t c17 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c18 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_StringLiteral(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18);
  return true;
}

bool ArboretumASTVisitor::VisitSubstNonTypeTemplateParmExpr(clang::SubstNonTypeTemplateParmExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getNameLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c4 = context_.resolve(D->getReplacement());
  uint64_t c5 = context_.resolve(D->getAssociatedDecl());
  uint32_t c6 = D->getIndex();
  uint64_t c7 = context_.resolve(D->getParameter());
  bool c8 = D->isReferenceParameter();
  arboretum_emit_SubstNonTypeTemplateParmExpr(c0, c1, c2, c3, c4, c5, c6, c7, c8);
  return true;
}

bool ArboretumASTVisitor::VisitSubstNonTypeTemplateParmPackExpr(clang::SubstNonTypeTemplateParmPackExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getAssociatedDecl());
  uint32_t c2 = D->getIndex();
  uint64_t c3 = context_.resolve(D->getParameterPack());
  uint64_t c4 = context_.source_model_.resolve(D->getParameterPackLocation());
  uint64_t c5 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c6 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_SubstNonTypeTemplateParmPackExpr(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitSwitchCase(clang::SwitchCase* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getNextSwitchCase());
  uint64_t c2 = context_.source_model_.resolve(D->getKeywordLoc());
  uint64_t c3 = context_.source_model_.resolve(D->getColonLoc());
  uint64_t c4 = context_.resolve(D->getSubStmt());
  uint64_t c5 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c6 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_SwitchCase(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitSwitchStmt(clang::SwitchStmt* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->hasInitStorage();
  bool c2 = D->hasVarStorage();
  uint64_t c3 = context_.resolve(D->getCond());
  uint64_t c4 = context_.resolve(D->getBody());
  uint64_t c5 = context_.resolve(D->getInit());
  uint64_t c6 = context_.resolve(D->getConditionVariable());
  uint64_t c7 = context_.resolve(D->getConditionVariableDeclStmt());
  uint64_t c8 = context_.resolve(D->getSwitchCaseList());
  uint64_t c9 = context_.source_model_.resolve(D->getSwitchLoc());
  uint64_t c10 = context_.source_model_.resolve(D->getLParenLoc());
  uint64_t c11 = context_.source_model_.resolve(D->getRParenLoc());
  bool c12 = D->isAllEnumCasesCovered();
  uint64_t c13 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c14 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_SwitchStmt(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14);
  return true;
}

bool ArboretumASTVisitor::VisitTypeTraitExpr(clang::TypeTraitExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.data_model_.resolve(D->getTrait());
  uint32_t c2 = D->getNumArgs();
  uint64_t c3 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_TypeTraitExpr(c0, c1, c2, c3, c4);
  return true;
}

bool ArboretumASTVisitor::VisitTypoExpr(clang::TypoExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c2 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_TypoExpr(c0, c1, c2);
  return true;
}

bool ArboretumASTVisitor::VisitUnaryExprOrTypeTraitExpr(clang::UnaryExprOrTypeTraitExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.data_model_.resolve(D->getKind());
  bool c2 = D->isArgumentType();
  uint64_t c3 = context_.resolve(D->getTypeOfArgument());
  uint64_t c4 = context_.source_model_.resolve(D->getOperatorLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getRParenLoc());
  uint64_t c6 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c7 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_UnaryExprOrTypeTraitExpr(c0, c1, c2, c3, c4, c5, c6, c7);
  return true;
}

bool ArboretumASTVisitor::VisitUnaryOperator(clang::UnaryOperator* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.data_model_.resolve(D->getOpcode());
  uint64_t c2 = context_.resolve(D->getSubExpr());
  uint64_t c3 = context_.source_model_.resolve(D->getOperatorLoc());
  bool c4 = D->canOverflow();
  bool c5 = D->isPrefix();
  bool c6 = D->isPostfix();
  bool c7 = D->isIncrementOp();
  bool c8 = D->isDecrementOp();
  bool c9 = D->isIncrementDecrementOp();
  bool c10 = D->isArithmeticOp();
  uint64_t c11 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c12 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c13 = context_.source_model_.resolve(D->getExprLoc());
  bool c14 = D->hasStoredFPFeatures();
  arboretum_emit_UnaryOperator(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14);
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedLookupExpr(clang::UnresolvedLookupExpr* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->requiresADL();
  bool c2 = D->isOverloaded();
  uint64_t c3 = context_.resolve(D->getNamingClass());
  uint64_t c4 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_UnresolvedLookupExpr(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedMemberExpr(clang::UnresolvedMemberExpr* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->isImplicitAccess();
  uint64_t c2 = context_.resolve(D->getBaseType());
  bool c3 = D->hasUnresolvedUsing();
  bool c4 = D->isArrow();
  uint64_t c5 = context_.source_model_.resolve(D->getOperatorLoc());
  uint64_t c6 = context_.resolve(D->getNamingClass());
  uint64_t c7 = context_.source_model_.resolve(D->getMemberLoc());
  uint64_t c8 = context_.source_model_.resolve(D->getExprLoc());
  uint64_t c9 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c10 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_UnresolvedMemberExpr(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10);
  return true;
}

bool ArboretumASTVisitor::VisitUserDefinedLiteral(clang::UserDefinedLiteral* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.data_model_.resolve(D->getLiteralOperatorKind());
  uint64_t c2 = context_.resolve(D->getCookedLiteral());
  uint64_t c3 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getEndLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getUDSuffixLoc());
  arboretum_emit_UserDefinedLiteral(c0, c1, c2, c3, c4, c5);
  return true;
}

bool ArboretumASTVisitor::VisitVAArgExpr(clang::VAArgExpr* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getSubExpr());
  bool c2 = D->isMicrosoftABI();
  uint64_t c3 = context_.source_model_.resolve(D->getBuiltinLoc());
  uint64_t c4 = context_.source_model_.resolve(D->getRParenLoc());
  uint64_t c5 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c6 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_VAArgExpr(c0, c1, c2, c3, c4, c5, c6);
  return true;
}

bool ArboretumASTVisitor::VisitValueStmt(clang::ValueStmt* D) {
  uint64_t c0 = context_.resolve(D);
  uint64_t c1 = context_.resolve(D->getExprStmt());
  arboretum_emit_ValueStmt(c0, c1);
  return true;
}

bool ArboretumASTVisitor::VisitWhileStmt(clang::WhileStmt* D) {
  uint64_t c0 = context_.resolve(D);
  bool c1 = D->hasVarStorage();
  uint64_t c2 = context_.resolve(D->getCond());
  uint64_t c3 = context_.resolve(D->getBody());
  uint64_t c4 = context_.resolve(D->getConditionVariable());
  uint64_t c5 = context_.resolve(D->getConditionVariableDeclStmt());
  uint64_t c6 = context_.source_model_.resolve(D->getWhileLoc());
  uint64_t c7 = context_.source_model_.resolve(D->getLParenLoc());
  uint64_t c8 = context_.source_model_.resolve(D->getRParenLoc());
  uint64_t c9 = context_.source_model_.resolve(D->getBeginLoc());
  uint64_t c10 = context_.source_model_.resolve(D->getEndLoc());
  arboretum_emit_WhileStmt(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10);
  return true;
}

////   END ARBORETUM GENERATED CODE ////
}  // namespace arboretum