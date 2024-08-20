#include "arboretum_ast_visitor.h"

#include "arboretum_ffi.h"

namespace arboretum {

//// BEGIN ARBORETUM GENERATED CODE ////
// Types
bool ArboretumASTVisitor::VisitAdjustedType(clang::AdjustedType* D) {
  const Id* obj = context_.resolve(D);
  //getOriginalType
  {
    const Id* other = context_.resolve(D->getOriginalType());
    arboretum_create_edge(obj, context_.data_model_.method_getOriginalType, other);
  }
  //getAdjustedType
  {
    const Id* other = context_.resolve(D->getAdjustedType());
    arboretum_create_edge(obj, context_.data_model_.method_getAdjustedType, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitArrayType(clang::ArrayType* D) {
  const Id* obj = context_.resolve(D);
  //getElementType
  {
    const Id* other = context_.resolve(D->getElementType());
    arboretum_create_edge(obj, context_.data_model_.method_getElementType, other);
  }
  //getSizeModifier
  {
    const Id* enum_value = context_.data_model_.resolve(D->getSizeModifier());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getSizeModifier, enum_value);
    }
  }
  //getIndexTypeQualifiers
  // Qualifiers
  //getIndexTypeCVRQualifiers
  // unsigned int
  return true;
}

bool ArboretumASTVisitor::VisitAtomicType(clang::AtomicType* D) {
  const Id* obj = context_.resolve(D);
  //getValueType
  {
    const Id* other = context_.resolve(D->getValueType());
    arboretum_create_edge(obj, context_.data_model_.method_getValueType, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_1, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_1, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitAttributedType(clang::AttributedType* D) {
  const Id* obj = context_.resolve(D);
  //getAttrKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getAttrKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getAttrKind, enum_value);
    }
  }
  //getModifiedType
  {
    const Id* other = context_.resolve(D->getModifiedType());
    arboretum_create_edge(obj, context_.data_model_.method_getModifiedType, other);
  }
  //getEquivalentType
  {
    const Id* other = context_.resolve(D->getEquivalentType());
    arboretum_create_edge(obj, context_.data_model_.method_getEquivalentType, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_2, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_2, other);
  }
  //isQualifier
  arboretum_create_edge(obj, context_.data_model_.method_isQualifier, context_.data_model_.arboretum_node_for(D->isQualifier()));
  //isMSTypeSpec
  arboretum_create_edge(obj, context_.data_model_.method_isMSTypeSpec, context_.data_model_.arboretum_node_for(D->isMSTypeSpec()));
  //isWebAssemblyFuncrefSpec
  arboretum_create_edge(obj, context_.data_model_.method_isWebAssemblyFuncrefSpec, context_.data_model_.arboretum_node_for(D->isWebAssemblyFuncrefSpec()));
  //isCallingConv
  arboretum_create_edge(obj, context_.data_model_.method_isCallingConv, context_.data_model_.arboretum_node_for(D->isCallingConv()));
  //getImmediateNullability
  // std::optional<NullabilityKind>
  return true;
}

bool ArboretumASTVisitor::VisitAutoType(clang::AutoType* D) {
  const Id* obj = context_.resolve(D);
  //getTypeConstraintArguments
  // ArrayRef<TemplateArgument>
  //getTypeConstraintConcept
  {
    const Id* other = context_.resolve(D->getTypeConstraintConcept());
    arboretum_create_edge(obj, context_.data_model_.method_getTypeConstraintConcept, other);
  }
  //isConstrained
  arboretum_create_edge(obj, context_.data_model_.method_isConstrained, context_.data_model_.arboretum_node_for(D->isConstrained()));
  //isDecltypeAuto
  arboretum_create_edge(obj, context_.data_model_.method_isDecltypeAuto, context_.data_model_.arboretum_node_for(D->isDecltypeAuto()));
  //isGNUAutoType
  arboretum_create_edge(obj, context_.data_model_.method_isGNUAutoType, context_.data_model_.arboretum_node_for(D->isGNUAutoType()));
  //getKeyword
  {
    const Id* enum_value = context_.data_model_.resolve(D->getKeyword());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getKeyword, enum_value);
    }
  }
  return true;
}

bool ArboretumASTVisitor::VisitBTFTagAttributedType(clang::BTFTagAttributedType* D) {
  const Id* obj = context_.resolve(D);
  //getWrappedType
  {
    const Id* other = context_.resolve(D->getWrappedType());
    arboretum_create_edge(obj, context_.data_model_.method_getWrappedType, other);
  }
  //getAttr
  {
    const Id* other = context_.resolve(D->getAttr());
    arboretum_create_edge(obj, context_.data_model_.method_getAttr, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_3, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_3, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitBitIntType(clang::BitIntType* D) {
  const Id* obj = context_.resolve(D);
  //isUnsigned
  arboretum_create_edge(obj, context_.data_model_.method_isUnsigned, context_.data_model_.arboretum_node_for(D->isUnsigned()));
  //isSigned
  arboretum_create_edge(obj, context_.data_model_.method_isSigned, context_.data_model_.arboretum_node_for(D->isSigned()));
  //getNumBits
  // unsigned int
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_4, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_4, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitBlockPointerType(clang::BlockPointerType* D) {
  const Id* obj = context_.resolve(D);
  //getPointeeType
  {
    const Id* other = context_.resolve(D->getPointeeType());
    arboretum_create_edge(obj, context_.data_model_.method_getPointeeType, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_5, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_5, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitBuiltinType(clang::BuiltinType* D) {
  const Id* obj = context_.resolve(D);
  //getKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getKind, enum_value);
    }
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_6, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_6, other);
  }
  //isInteger
  arboretum_create_edge(obj, context_.data_model_.method_isInteger, context_.data_model_.arboretum_node_for(D->isInteger()));
  //isSignedInteger
  arboretum_create_edge(obj, context_.data_model_.method_isSignedInteger, context_.data_model_.arboretum_node_for(D->isSignedInteger()));
  //isUnsignedInteger
  arboretum_create_edge(obj, context_.data_model_.method_isUnsignedInteger, context_.data_model_.arboretum_node_for(D->isUnsignedInteger()));
  //isFloatingPoint
  arboretum_create_edge(obj, context_.data_model_.method_isFloatingPoint, context_.data_model_.arboretum_node_for(D->isFloatingPoint()));
  //isSVEBool
  arboretum_create_edge(obj, context_.data_model_.method_isSVEBool, context_.data_model_.arboretum_node_for(D->isSVEBool()));
  //isSVECount
  arboretum_create_edge(obj, context_.data_model_.method_isSVECount, context_.data_model_.arboretum_node_for(D->isSVECount()));
  //isPlaceholderType
  arboretum_create_edge(obj, context_.data_model_.method_isPlaceholderType, context_.data_model_.arboretum_node_for(D->isPlaceholderType()));
  //isNonOverloadPlaceholderType
  arboretum_create_edge(obj, context_.data_model_.method_isNonOverloadPlaceholderType, context_.data_model_.arboretum_node_for(D->isNonOverloadPlaceholderType()));
  return true;
}

bool ArboretumASTVisitor::VisitComplexType(clang::ComplexType* D) {
  const Id* obj = context_.resolve(D);
  //getElementType
  {
    const Id* other = context_.resolve(D->getElementType());
    arboretum_create_edge(obj, context_.data_model_.method_getElementType_1, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_7, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_7, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitConstantArrayType(clang::ConstantArrayType* D) {
  const Id* obj = context_.resolve(D);
  //getSize
  // const llvm::APInt &
  //getSizeExpr
  {
    const Id* other = context_.resolve(D->getSizeExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getSizeExpr, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_8, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_8, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitConstantMatrixType(clang::ConstantMatrixType* D) {
  const Id* obj = context_.resolve(D);
  //getNumRows
  // unsigned int
  //getNumColumns
  // unsigned int
  //getNumElementsFlattened
  // unsigned int
  return true;
}

bool ArboretumASTVisitor::VisitDecayedType(clang::DecayedType* D) {
  const Id* obj = context_.resolve(D);
  //getDecayedType
  {
    const Id* other = context_.resolve(D->getDecayedType());
    arboretum_create_edge(obj, context_.data_model_.method_getDecayedType, other);
  }
  //getPointeeType
  {
    const Id* other = context_.resolve(D->getPointeeType());
    arboretum_create_edge(obj, context_.data_model_.method_getPointeeType_1, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitDecltypeType(clang::DecltypeType* D) {
  const Id* obj = context_.resolve(D);
  //getUnderlyingExpr
  {
    const Id* other = context_.resolve(D->getUnderlyingExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getUnderlyingExpr, other);
  }
  //getUnderlyingType
  {
    const Id* other = context_.resolve(D->getUnderlyingType());
    arboretum_create_edge(obj, context_.data_model_.method_getUnderlyingType, other);
  }
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_9, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_9, context_.data_model_.arboretum_node_for(D->isSugared()));
  return true;
}

bool ArboretumASTVisitor::VisitDeducedTemplateSpecializationType(clang::DeducedTemplateSpecializationType* D) {
  const Id* obj = context_.resolve(D);
  //getTemplateName
  // TemplateName
  return true;
}

bool ArboretumASTVisitor::VisitDeducedType(clang::DeducedType* D) {
  const Id* obj = context_.resolve(D);
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_10, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_10, other);
  }
  //getDeducedType
  {
    const Id* other = context_.resolve(D->getDeducedType());
    arboretum_create_edge(obj, context_.data_model_.method_getDeducedType, other);
  }
  //isDeduced
  arboretum_create_edge(obj, context_.data_model_.method_isDeduced, context_.data_model_.arboretum_node_for(D->isDeduced()));
  return true;
}

bool ArboretumASTVisitor::VisitDependentAddressSpaceType(clang::DependentAddressSpaceType* D) {
  const Id* obj = context_.resolve(D);
  //getAddrSpaceExpr
  {
    const Id* other = context_.resolve(D->getAddrSpaceExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getAddrSpaceExpr, other);
  }
  //getPointeeType
  {
    const Id* other = context_.resolve(D->getPointeeType());
    arboretum_create_edge(obj, context_.data_model_.method_getPointeeType_2, other);
  }
  //getAttributeLoc
  {
    const Id* other = context_.source_model_.resolve(D->getAttributeLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getAttributeLoc, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_11, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_11, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitDependentBitIntType(clang::DependentBitIntType* D) {
  const Id* obj = context_.resolve(D);
  //isUnsigned
  arboretum_create_edge(obj, context_.data_model_.method_isUnsigned_1, context_.data_model_.arboretum_node_for(D->isUnsigned()));
  //isSigned
  arboretum_create_edge(obj, context_.data_model_.method_isSigned_1, context_.data_model_.arboretum_node_for(D->isSigned()));
  //getNumBitsExpr
  {
    const Id* other = context_.resolve(D->getNumBitsExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getNumBitsExpr, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_12, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_12, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitDependentDecltypeType(clang::DependentDecltypeType* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitDependentNameType(clang::DependentNameType* D) {
  const Id* obj = context_.resolve(D);
  //getQualifier
  //getIdentifier
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_13, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_13, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitDependentSizedArrayType(clang::DependentSizedArrayType* D) {
  const Id* obj = context_.resolve(D);
  //getSizeExpr
  {
    const Id* other = context_.resolve(D->getSizeExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getSizeExpr_1, other);
  }
  //getBracketsRange
  {
    const Id* other = context_.source_model_.resolve(D->getBracketsRange());
    arboretum_create_edge(obj, context_.data_model_.method_getBracketsRange, other);
  }
  //getLBracketLoc
  {
    const Id* other = context_.source_model_.resolve(D->getLBracketLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getLBracketLoc, other);
  }
  //getRBracketLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRBracketLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRBracketLoc, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_14, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_14, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitDependentSizedExtVectorType(clang::DependentSizedExtVectorType* D) {
  const Id* obj = context_.resolve(D);
  //getSizeExpr
  {
    const Id* other = context_.resolve(D->getSizeExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getSizeExpr_2, other);
  }
  //getElementType
  {
    const Id* other = context_.resolve(D->getElementType());
    arboretum_create_edge(obj, context_.data_model_.method_getElementType_2, other);
  }
  //getAttributeLoc
  {
    const Id* other = context_.source_model_.resolve(D->getAttributeLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getAttributeLoc_1, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_15, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_15, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitDependentSizedMatrixType(clang::DependentSizedMatrixType* D) {
  const Id* obj = context_.resolve(D);
  //getRowExpr
  {
    const Id* other = context_.resolve(D->getRowExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getRowExpr, other);
  }
  //getColumnExpr
  {
    const Id* other = context_.resolve(D->getColumnExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getColumnExpr, other);
  }
  //getAttributeLoc
  {
    const Id* other = context_.source_model_.resolve(D->getAttributeLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getAttributeLoc_2, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitDependentTemplateSpecializationType(clang::DependentTemplateSpecializationType* D) {
  const Id* obj = context_.resolve(D);
  //getQualifier
  //getIdentifier
  //template_arguments
  // ArrayRef<TemplateArgument>
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_16, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_16, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitDependentTypeOfExprType(clang::DependentTypeOfExprType* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitDependentUnaryTransformType(clang::DependentUnaryTransformType* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitDependentVectorType(clang::DependentVectorType* D) {
  const Id* obj = context_.resolve(D);
  //getSizeExpr
  {
    const Id* other = context_.resolve(D->getSizeExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getSizeExpr_3, other);
  }
  //getElementType
  {
    const Id* other = context_.resolve(D->getElementType());
    arboretum_create_edge(obj, context_.data_model_.method_getElementType_3, other);
  }
  //getAttributeLoc
  {
    const Id* other = context_.source_model_.resolve(D->getAttributeLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getAttributeLoc_3, other);
  }
  //getVectorKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getVectorKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getVectorKind, enum_value);
    }
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_17, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_17, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitElaboratedType(clang::ElaboratedType* D) {
  const Id* obj = context_.resolve(D);
  //getQualifier
  //getNamedType
  {
    const Id* other = context_.resolve(D->getNamedType());
    arboretum_create_edge(obj, context_.data_model_.method_getNamedType, other);
  }
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_18, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_18, context_.data_model_.arboretum_node_for(D->isSugared()));
  //getOwnedTagDecl
  {
    const Id* other = context_.resolve(D->getOwnedTagDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getOwnedTagDecl, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitEnumType(clang::EnumType* D) {
  const Id* obj = context_.resolve(D);
  //getDecl
  {
    const Id* other = context_.resolve(D->getDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getDecl, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_19, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_19, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitExtVectorType(clang::ExtVectorType* D) {
  const Id* obj = context_.resolve(D);
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_20, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_20, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitFunctionNoProtoType(clang::FunctionNoProtoType* D) {
  const Id* obj = context_.resolve(D);
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_21, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_21, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitFunctionProtoType(clang::FunctionProtoType* D) {
  const Id* obj = context_.resolve(D);
  //getNumParams
  // unsigned int
  //getParamTypes
  // ArrayRef<QualType>
  //getExtProtoInfo
  // ExtProtoInfo
  //getExceptionSpecType
  {
    const Id* enum_value = context_.data_model_.resolve(D->getExceptionSpecType());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getExceptionSpecType, enum_value);
    }
  }
  //hasExceptionSpec
  arboretum_create_edge(obj, context_.data_model_.method_hasExceptionSpec, context_.data_model_.arboretum_node_for(D->hasExceptionSpec()));
  //hasDynamicExceptionSpec
  arboretum_create_edge(obj, context_.data_model_.method_hasDynamicExceptionSpec, context_.data_model_.arboretum_node_for(D->hasDynamicExceptionSpec()));
  //hasNoexceptExceptionSpec
  arboretum_create_edge(obj, context_.data_model_.method_hasNoexceptExceptionSpec, context_.data_model_.arboretum_node_for(D->hasNoexceptExceptionSpec()));
  //hasDependentExceptionSpec
  arboretum_create_edge(obj, context_.data_model_.method_hasDependentExceptionSpec, context_.data_model_.arboretum_node_for(D->hasDependentExceptionSpec()));
  //hasInstantiationDependentExceptionSpec
  arboretum_create_edge(obj, context_.data_model_.method_hasInstantiationDependentExceptionSpec, context_.data_model_.arboretum_node_for(D->hasInstantiationDependentExceptionSpec()));
  //getExceptionSpecInfo
  // ExceptionSpecInfo
  //getNumExceptions
  // unsigned int
  //getNoexceptExpr
  {
    const Id* other = context_.resolve(D->getNoexceptExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getNoexceptExpr, other);
  }
  //getExceptionSpecDecl
  {
    const Id* other = context_.resolve(D->getExceptionSpecDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getExceptionSpecDecl, other);
  }
  //getExceptionSpecTemplate
  {
    const Id* other = context_.resolve(D->getExceptionSpecTemplate());
    arboretum_create_edge(obj, context_.data_model_.method_getExceptionSpecTemplate, other);
  }
  //canThrow
  {
    const Id* enum_value = context_.data_model_.resolve(D->canThrow());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_canThrow, enum_value);
    }
  }
  //isVariadic
  arboretum_create_edge(obj, context_.data_model_.method_isVariadic, context_.data_model_.arboretum_node_for(D->isVariadic()));
  //getEllipsisLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEllipsisLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEllipsisLoc, other);
  }
  //isTemplateVariadic
  arboretum_create_edge(obj, context_.data_model_.method_isTemplateVariadic, context_.data_model_.arboretum_node_for(D->isTemplateVariadic()));
  //hasTrailingReturn
  arboretum_create_edge(obj, context_.data_model_.method_hasTrailingReturn, context_.data_model_.arboretum_node_for(D->hasTrailingReturn()));
  //getMethodQuals
  // Qualifiers
  //getRefQualifier
  {
    const Id* enum_value = context_.data_model_.resolve(D->getRefQualifier());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getRefQualifier, enum_value);
    }
  }
  //param_types
  // ArrayRef<QualType>
  //exceptions
  // ArrayRef<QualType>
  //hasExtParameterInfos
  arboretum_create_edge(obj, context_.data_model_.method_hasExtParameterInfos, context_.data_model_.arboretum_node_for(D->hasExtParameterInfos()));
  //getExtParameterInfos
  // ArrayRef<ExtParameterInfo>
  //getExtParameterInfosOrNull
  //getAArch64SMEAttributes
  // unsigned int
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_22, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_22, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitFunctionType(clang::FunctionType* D) {
  const Id* obj = context_.resolve(D);
  //getReturnType
  {
    const Id* other = context_.resolve(D->getReturnType());
    arboretum_create_edge(obj, context_.data_model_.method_getReturnType, other);
  }
  //getHasRegParm
  arboretum_create_edge(obj, context_.data_model_.method_getHasRegParm, context_.data_model_.arboretum_node_for(D->getHasRegParm()));
  //getRegParmType
  // unsigned int
  //getNoReturnAttr
  arboretum_create_edge(obj, context_.data_model_.method_getNoReturnAttr, context_.data_model_.arboretum_node_for(D->getNoReturnAttr()));
  //getCmseNSCallAttr
  arboretum_create_edge(obj, context_.data_model_.method_getCmseNSCallAttr, context_.data_model_.arboretum_node_for(D->getCmseNSCallAttr()));
  //getCallConv
  {
    const Id* enum_value = context_.data_model_.resolve(D->getCallConv());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getCallConv, enum_value);
    }
  }
  //getExtInfo
  // ExtInfo
  //isConst
  arboretum_create_edge(obj, context_.data_model_.method_isConst, context_.data_model_.arboretum_node_for(D->isConst()));
  //isVolatile
  arboretum_create_edge(obj, context_.data_model_.method_isVolatile, context_.data_model_.arboretum_node_for(D->isVolatile()));
  //isRestrict
  arboretum_create_edge(obj, context_.data_model_.method_isRestrict, context_.data_model_.arboretum_node_for(D->isRestrict()));
  return true;
}

bool ArboretumASTVisitor::VisitIncompleteArrayType(clang::IncompleteArrayType* D) {
  const Id* obj = context_.resolve(D);
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_23, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_23, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitInjectedClassNameType(clang::InjectedClassNameType* D) {
  const Id* obj = context_.resolve(D);
  //getInjectedSpecializationType
  {
    const Id* other = context_.resolve(D->getInjectedSpecializationType());
    arboretum_create_edge(obj, context_.data_model_.method_getInjectedSpecializationType, other);
  }
  //getInjectedTST
  {
    const Id* other = context_.resolve(D->getInjectedTST());
    arboretum_create_edge(obj, context_.data_model_.method_getInjectedTST, other);
  }
  //getTemplateName
  // TemplateName
  //getDecl
  {
    const Id* other = context_.resolve(D->getDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getDecl_1, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_24, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_24, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitLValueReferenceType(clang::LValueReferenceType* D) {
  const Id* obj = context_.resolve(D);
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_25, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_25, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitMacroQualifiedType(clang::MacroQualifiedType* D) {
  const Id* obj = context_.resolve(D);
  //getMacroIdentifier
  //getUnderlyingType
  {
    const Id* other = context_.resolve(D->getUnderlyingType());
    arboretum_create_edge(obj, context_.data_model_.method_getUnderlyingType_1, other);
  }
  //getModifiedType
  {
    const Id* other = context_.resolve(D->getModifiedType());
    arboretum_create_edge(obj, context_.data_model_.method_getModifiedType_1, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_26, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_26, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitMatrixType(clang::MatrixType* D) {
  const Id* obj = context_.resolve(D);
  //getElementType
  {
    const Id* other = context_.resolve(D->getElementType());
    arboretum_create_edge(obj, context_.data_model_.method_getElementType_4, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_27, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_27, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitMemberPointerType(clang::MemberPointerType* D) {
  const Id* obj = context_.resolve(D);
  //getPointeeType
  {
    const Id* other = context_.resolve(D->getPointeeType());
    arboretum_create_edge(obj, context_.data_model_.method_getPointeeType_3, other);
  }
  //isMemberFunctionPointer
  arboretum_create_edge(obj, context_.data_model_.method_isMemberFunctionPointer, context_.data_model_.arboretum_node_for(D->isMemberFunctionPointer()));
  //isMemberDataPointer
  arboretum_create_edge(obj, context_.data_model_.method_isMemberDataPointer, context_.data_model_.arboretum_node_for(D->isMemberDataPointer()));
  //getClass
  {
    const Id* other = context_.resolve(D->getClass());
    arboretum_create_edge(obj, context_.data_model_.method_getClass, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_28, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_28, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitObjCInterfaceType(clang::ObjCInterfaceType* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCObjectPointerType(clang::ObjCObjectPointerType* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCObjectType(clang::ObjCObjectType* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCObjectTypeImpl(clang::ObjCObjectTypeImpl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCTypeParamType(clang::ObjCTypeParamType* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitPackExpansionType(clang::PackExpansionType* D) {
  const Id* obj = context_.resolve(D);
  //getPattern
  {
    const Id* other = context_.resolve(D->getPattern());
    arboretum_create_edge(obj, context_.data_model_.method_getPattern, other);
  }
  //getNumExpansions
  // std::optional<unsigned int>
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_29, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_29, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitParenType(clang::ParenType* D) {
  const Id* obj = context_.resolve(D);
  //getInnerType
  {
    const Id* other = context_.resolve(D->getInnerType());
    arboretum_create_edge(obj, context_.data_model_.method_getInnerType, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_30, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_30, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitPipeType(clang::PipeType* D) {
  const Id* obj = context_.resolve(D);
  //getElementType
  {
    const Id* other = context_.resolve(D->getElementType());
    arboretum_create_edge(obj, context_.data_model_.method_getElementType_5, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_31, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_31, other);
  }
  //isReadOnly
  arboretum_create_edge(obj, context_.data_model_.method_isReadOnly, context_.data_model_.arboretum_node_for(D->isReadOnly()));
  return true;
}

bool ArboretumASTVisitor::VisitPointerType(clang::PointerType* D) {
  const Id* obj = context_.resolve(D);
  //getPointeeType
  {
    const Id* other = context_.resolve(D->getPointeeType());
    arboretum_create_edge(obj, context_.data_model_.method_getPointeeType_4, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_32, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_32, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitRValueReferenceType(clang::RValueReferenceType* D) {
  const Id* obj = context_.resolve(D);
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_33, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_33, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitRecordType(clang::RecordType* D) {
  const Id* obj = context_.resolve(D);
  //getDecl
  {
    const Id* other = context_.resolve(D->getDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getDecl_2, other);
  }
  //hasConstFields
  arboretum_create_edge(obj, context_.data_model_.method_hasConstFields, context_.data_model_.arboretum_node_for(D->hasConstFields()));
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_34, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_34, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitReferenceType(clang::ReferenceType* D) {
  const Id* obj = context_.resolve(D);
  //isSpelledAsLValue
  arboretum_create_edge(obj, context_.data_model_.method_isSpelledAsLValue, context_.data_model_.arboretum_node_for(D->isSpelledAsLValue()));
  //isInnerRef
  arboretum_create_edge(obj, context_.data_model_.method_isInnerRef, context_.data_model_.arboretum_node_for(D->isInnerRef()));
  //getPointeeTypeAsWritten
  {
    const Id* other = context_.resolve(D->getPointeeTypeAsWritten());
    arboretum_create_edge(obj, context_.data_model_.method_getPointeeTypeAsWritten, other);
  }
  //getPointeeType
  {
    const Id* other = context_.resolve(D->getPointeeType());
    arboretum_create_edge(obj, context_.data_model_.method_getPointeeType_5, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitSubstTemplateTypeParmPackType(clang::SubstTemplateTypeParmPackType* D) {
  const Id* obj = context_.resolve(D);
  //getIdentifier
  //getAssociatedDecl
  {
    const Id* other = context_.resolve(D->getAssociatedDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getAssociatedDecl, other);
  }
  //getReplacedParameter
  {
    const Id* other = context_.resolve(D->getReplacedParameter());
    arboretum_create_edge(obj, context_.data_model_.method_getReplacedParameter, other);
  }
  //getIndex
  // unsigned int
  //getFinal
  arboretum_create_edge(obj, context_.data_model_.method_getFinal, context_.data_model_.arboretum_node_for(D->getFinal()));
  //getNumArgs
  // unsigned int
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_35, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_35, other);
  }
  //getArgumentPack
  // TemplateArgument
  return true;
}

bool ArboretumASTVisitor::VisitSubstTemplateTypeParmType(clang::SubstTemplateTypeParmType* D) {
  const Id* obj = context_.resolve(D);
  //getReplacementType
  {
    const Id* other = context_.resolve(D->getReplacementType());
    arboretum_create_edge(obj, context_.data_model_.method_getReplacementType, other);
  }
  //getAssociatedDecl
  {
    const Id* other = context_.resolve(D->getAssociatedDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getAssociatedDecl_1, other);
  }
  //getReplacedParameter
  {
    const Id* other = context_.resolve(D->getReplacedParameter());
    arboretum_create_edge(obj, context_.data_model_.method_getReplacedParameter_1, other);
  }
  //getIndex
  // unsigned int
  //getPackIndex
  // std::optional<unsigned int>
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_36, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_36, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitTagType(clang::TagType* D) {
  const Id* obj = context_.resolve(D);
  //getDecl
  {
    const Id* other = context_.resolve(D->getDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getDecl_3, other);
  }
  //isBeingDefined
  arboretum_create_edge(obj, context_.data_model_.method_isBeingDefined, context_.data_model_.arboretum_node_for(D->isBeingDefined()));
  return true;
}

bool ArboretumASTVisitor::VisitTemplateSpecializationType(clang::TemplateSpecializationType* D) {
  const Id* obj = context_.resolve(D);
  //isCurrentInstantiation
  arboretum_create_edge(obj, context_.data_model_.method_isCurrentInstantiation, context_.data_model_.arboretum_node_for(D->isCurrentInstantiation()));
  //isTypeAlias
  arboretum_create_edge(obj, context_.data_model_.method_isTypeAlias, context_.data_model_.arboretum_node_for(D->isTypeAlias()));
  //getTemplateName
  // TemplateName
  //template_arguments
  // ArrayRef<TemplateArgument>
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_37, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_37, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitTemplateTypeParmType(clang::TemplateTypeParmType* D) {
  const Id* obj = context_.resolve(D);
  //getDepth
  // unsigned int
  //getIndex
  // unsigned int
  //isParameterPack
  arboretum_create_edge(obj, context_.data_model_.method_isParameterPack, context_.data_model_.arboretum_node_for(D->isParameterPack()));
  //getDecl
  {
    const Id* other = context_.resolve(D->getDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getDecl_4, other);
  }
  //getIdentifier
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_38, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_38, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitType(clang::Type* D) {
  const Id* obj = context_.resolve(D);
  //containsUnexpandedParameterPack
  arboretum_create_edge(obj, context_.data_model_.method_containsUnexpandedParameterPack, context_.data_model_.arboretum_node_for(D->containsUnexpandedParameterPack()));
  //getLocallyUnqualifiedSingleStepDesugaredType
  {
    const Id* other = context_.resolve(D->getLocallyUnqualifiedSingleStepDesugaredType());
    arboretum_create_edge(obj, context_.data_model_.method_getLocallyUnqualifiedSingleStepDesugaredType, other);
  }
  //getAsPlaceholderType
  {
    const Id* other = context_.resolve(D->getAsPlaceholderType());
    arboretum_create_edge(obj, context_.data_model_.method_getAsPlaceholderType, other);
  }
  //getObjCARCImplicitLifetime
  {
    const Id* enum_value = context_.data_model_.resolve(D->getObjCARCImplicitLifetime());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getObjCARCImplicitLifetime, enum_value);
    }
  }
  //getDependence
  {
    const Id* enum_value = context_.data_model_.resolve(D->getDependence());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getDependence, enum_value);
    }
  }
  //containsErrors
  arboretum_create_edge(obj, context_.data_model_.method_containsErrors, context_.data_model_.arboretum_node_for(D->containsErrors()));
  //hasSizedVLAType
  arboretum_create_edge(obj, context_.data_model_.method_hasSizedVLAType, context_.data_model_.arboretum_node_for(D->hasSizedVLAType()));
  //hasUnnamedOrLocalType
  arboretum_create_edge(obj, context_.data_model_.method_hasUnnamedOrLocalType, context_.data_model_.arboretum_node_for(D->hasUnnamedOrLocalType()));
  //canDecayToPointerType
  arboretum_create_edge(obj, context_.data_model_.method_canDecayToPointerType, context_.data_model_.arboretum_node_for(D->canDecayToPointerType()));
  //hasPointerRepresentation
  arboretum_create_edge(obj, context_.data_model_.method_hasPointerRepresentation, context_.data_model_.arboretum_node_for(D->hasPointerRepresentation()));
  //hasObjCPointerRepresentation
  arboretum_create_edge(obj, context_.data_model_.method_hasObjCPointerRepresentation, context_.data_model_.arboretum_node_for(D->hasObjCPointerRepresentation()));
  //hasIntegerRepresentation
  arboretum_create_edge(obj, context_.data_model_.method_hasIntegerRepresentation, context_.data_model_.arboretum_node_for(D->hasIntegerRepresentation()));
  //hasSignedIntegerRepresentation
  arboretum_create_edge(obj, context_.data_model_.method_hasSignedIntegerRepresentation, context_.data_model_.arboretum_node_for(D->hasSignedIntegerRepresentation()));
  //hasUnsignedIntegerRepresentation
  arboretum_create_edge(obj, context_.data_model_.method_hasUnsignedIntegerRepresentation, context_.data_model_.arboretum_node_for(D->hasUnsignedIntegerRepresentation()));
  //hasFloatingRepresentation
  arboretum_create_edge(obj, context_.data_model_.method_hasFloatingRepresentation, context_.data_model_.arboretum_node_for(D->hasFloatingRepresentation()));
  //getAsStructureType
  {
    const Id* other = context_.resolve(D->getAsStructureType());
    arboretum_create_edge(obj, context_.data_model_.method_getAsStructureType, other);
  }
  //getAsUnionType
  {
    const Id* other = context_.resolve(D->getAsUnionType());
    arboretum_create_edge(obj, context_.data_model_.method_getAsUnionType, other);
  }
  //getAsComplexIntegerType
  {
    const Id* other = context_.resolve(D->getAsComplexIntegerType());
    arboretum_create_edge(obj, context_.data_model_.method_getAsComplexIntegerType, other);
  }
  //getAsObjCInterfaceType
  {
    const Id* other = context_.resolve(D->getAsObjCInterfaceType());
    arboretum_create_edge(obj, context_.data_model_.method_getAsObjCInterfaceType, other);
  }
  //getAsObjCInterfacePointerType
  {
    const Id* other = context_.resolve(D->getAsObjCInterfacePointerType());
    arboretum_create_edge(obj, context_.data_model_.method_getAsObjCInterfacePointerType, other);
  }
  //getAsObjCQualifiedIdType
  {
    const Id* other = context_.resolve(D->getAsObjCQualifiedIdType());
    arboretum_create_edge(obj, context_.data_model_.method_getAsObjCQualifiedIdType, other);
  }
  //getAsObjCQualifiedClassType
  {
    const Id* other = context_.resolve(D->getAsObjCQualifiedClassType());
    arboretum_create_edge(obj, context_.data_model_.method_getAsObjCQualifiedClassType, other);
  }
  //getAsObjCQualifiedInterfaceType
  {
    const Id* other = context_.resolve(D->getAsObjCQualifiedInterfaceType());
    arboretum_create_edge(obj, context_.data_model_.method_getAsObjCQualifiedInterfaceType, other);
  }
  //getAsCXXRecordDecl
  {
    const Id* other = context_.resolve(D->getAsCXXRecordDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getAsCXXRecordDecl, other);
  }
  //getAsRecordDecl
  {
    const Id* other = context_.resolve(D->getAsRecordDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getAsRecordDecl, other);
  }
  //getAsTagDecl
  {
    const Id* other = context_.resolve(D->getAsTagDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getAsTagDecl, other);
  }
  //getPointeeCXXRecordDecl
  {
    const Id* other = context_.resolve(D->getPointeeCXXRecordDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getPointeeCXXRecordDecl, other);
  }
  //getBaseElementTypeUnsafe
  {
    const Id* other = context_.resolve(D->getBaseElementTypeUnsafe());
    arboretum_create_edge(obj, context_.data_model_.method_getBaseElementTypeUnsafe, other);
  }
  //getArrayElementTypeNoTypeQual
  {
    const Id* other = context_.resolve(D->getArrayElementTypeNoTypeQual());
    arboretum_create_edge(obj, context_.data_model_.method_getArrayElementTypeNoTypeQual, other);
  }
  //getPointeeOrArrayElementType
  {
    const Id* other = context_.resolve(D->getPointeeOrArrayElementType());
    arboretum_create_edge(obj, context_.data_model_.method_getPointeeOrArrayElementType, other);
  }
  //getLinkage
  {
    const Id* enum_value = context_.data_model_.resolve(D->getLinkage());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getLinkage, enum_value);
    }
  }
  //getVisibility
  {
    const Id* enum_value = context_.data_model_.resolve(D->getVisibility());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getVisibility, enum_value);
    }
  }
  //getLinkageAndVisibility
  // LinkageInfo
  //getNullability
  // std::optional<NullabilityKind>
  //acceptsObjCTypeParams
  arboretum_create_edge(obj, context_.data_model_.method_acceptsObjCTypeParams, context_.data_model_.arboretum_node_for(D->acceptsObjCTypeParams()));
  //getTypeClassName
  //getCanonicalTypeInternal
  {
    const Id* other = context_.resolve(D->getCanonicalTypeInternal());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalTypeInternal, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitTypeOfExprType(clang::TypeOfExprType* D) {
  const Id* obj = context_.resolve(D);
  //getUnderlyingExpr
  {
    const Id* other = context_.resolve(D->getUnderlyingExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getUnderlyingExpr_1, other);
  }
  //getKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getKind_1, enum_value);
    }
  }
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_39, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_39, context_.data_model_.arboretum_node_for(D->isSugared()));
  return true;
}

bool ArboretumASTVisitor::VisitTypeOfType(clang::TypeOfType* D) {
  const Id* obj = context_.resolve(D);
  //getUnmodifiedType
  {
    const Id* other = context_.resolve(D->getUnmodifiedType());
    arboretum_create_edge(obj, context_.data_model_.method_getUnmodifiedType, other);
  }
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_40, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_40, context_.data_model_.arboretum_node_for(D->isSugared()));
  //getKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getKind_2, enum_value);
    }
  }
  return true;
}

bool ArboretumASTVisitor::VisitTypeWithKeyword(clang::TypeWithKeyword* D) {
  const Id* obj = context_.resolve(D);
  //getKeyword
  {
    const Id* enum_value = context_.data_model_.resolve(D->getKeyword());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getKeyword_1, enum_value);
    }
  }
  return true;
}

bool ArboretumASTVisitor::VisitTypedefType(clang::TypedefType* D) {
  const Id* obj = context_.resolve(D);
  //getDecl
  {
    const Id* other = context_.resolve(D->getDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getDecl_5, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_41, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_41, other);
  }
  //typeMatchesDecl
  arboretum_create_edge(obj, context_.data_model_.method_typeMatchesDecl, context_.data_model_.arboretum_node_for(D->typeMatchesDecl()));
  return true;
}

bool ArboretumASTVisitor::VisitUnaryTransformType(clang::UnaryTransformType* D) {
  const Id* obj = context_.resolve(D);
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_42, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_42, other);
  }
  //getUnderlyingType
  {
    const Id* other = context_.resolve(D->getUnderlyingType());
    arboretum_create_edge(obj, context_.data_model_.method_getUnderlyingType_2, other);
  }
  //getBaseType
  {
    const Id* other = context_.resolve(D->getBaseType());
    arboretum_create_edge(obj, context_.data_model_.method_getBaseType, other);
  }
  //getUTTKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getUTTKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getUTTKind, enum_value);
    }
  }
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedUsingType(clang::UnresolvedUsingType* D) {
  const Id* obj = context_.resolve(D);
  //getDecl
  {
    const Id* other = context_.resolve(D->getDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getDecl_6, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_43, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_43, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitUsingType(clang::UsingType* D) {
  const Id* obj = context_.resolve(D);
  //getFoundDecl
  {
    const Id* other = context_.resolve(D->getFoundDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getFoundDecl, other);
  }
  //getUnderlyingType
  {
    const Id* other = context_.resolve(D->getUnderlyingType());
    arboretum_create_edge(obj, context_.data_model_.method_getUnderlyingType_3, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_44, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_44, other);
  }
  //typeMatchesDecl
  arboretum_create_edge(obj, context_.data_model_.method_typeMatchesDecl_1, context_.data_model_.arboretum_node_for(D->typeMatchesDecl()));
  return true;
}

bool ArboretumASTVisitor::VisitVariableArrayType(clang::VariableArrayType* D) {
  const Id* obj = context_.resolve(D);
  //getSizeExpr
  {
    const Id* other = context_.resolve(D->getSizeExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getSizeExpr_4, other);
  }
  //getBracketsRange
  {
    const Id* other = context_.source_model_.resolve(D->getBracketsRange());
    arboretum_create_edge(obj, context_.data_model_.method_getBracketsRange_1, other);
  }
  //getLBracketLoc
  {
    const Id* other = context_.source_model_.resolve(D->getLBracketLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getLBracketLoc_1, other);
  }
  //getRBracketLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRBracketLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRBracketLoc_1, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_45, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_45, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitVectorType(clang::VectorType* D) {
  const Id* obj = context_.resolve(D);
  //getElementType
  {
    const Id* other = context_.resolve(D->getElementType());
    arboretum_create_edge(obj, context_.data_model_.method_getElementType_6, other);
  }
  //getNumElements
  // unsigned int
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_46, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Id* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.method_desugar_46, other);
  }
  //getVectorKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getVectorKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getVectorKind_1, enum_value);
    }
  }
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
  const Id* obj = context_.resolve(D);
  //getAccessSpecifierLoc
  {
    const Id* other = context_.source_model_.resolve(D->getAccessSpecifierLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getAccessSpecifierLoc, other);
  }
  //getColonLoc
  {
    const Id* other = context_.source_model_.resolve(D->getColonLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getColonLoc, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitBaseUsingDecl(clang::BaseUsingDecl* D) {
  const Id* obj = context_.resolve(D);
  //shadows
  // shadow_range
  //shadow_size
  // unsigned int
  return true;
}

bool ArboretumASTVisitor::VisitBindingDecl(clang::BindingDecl* D) {
  const Id* obj = context_.resolve(D);
  //getBinding
  {
    const Id* other = context_.resolve(D->getBinding());
    arboretum_create_edge(obj, context_.data_model_.method_getBinding, other);
  }
  //getDecomposedDecl
  {
    const Id* other = context_.resolve(D->getDecomposedDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getDecomposedDecl, other);
  }
  //getHoldingVar
  {
    const Id* other = context_.resolve(D->getHoldingVar());
    arboretum_create_edge(obj, context_.data_model_.method_getHoldingVar, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitBlockDecl(clang::BlockDecl* D) {
  const Id* obj = context_.resolve(D);
  //getCaretLocation
  {
    const Id* other = context_.source_model_.resolve(D->getCaretLocation());
    arboretum_create_edge(obj, context_.data_model_.method_getCaretLocation, other);
  }
  //isVariadic
  arboretum_create_edge(obj, context_.data_model_.method_isVariadic_1, context_.data_model_.arboretum_node_for(D->isVariadic()));
  //getCompoundBody
  {
    const Id* other = context_.resolve(D->getCompoundBody());
    arboretum_create_edge(obj, context_.data_model_.method_getCompoundBody, other);
  }
  //getBody
  {
    const Id* other = context_.resolve(D->getBody());
    arboretum_create_edge(obj, context_.data_model_.method_getBody, other);
  }
  //getSignatureAsWritten
  //parameters
  // ArrayRef<ParmVarDecl *>
  //param_empty
  arboretum_create_edge(obj, context_.data_model_.method_param_empty, context_.data_model_.arboretum_node_for(D->param_empty()));
  //param_size
  // size_t
  //getNumParams
  // unsigned int
  //hasCaptures
  arboretum_create_edge(obj, context_.data_model_.method_hasCaptures, context_.data_model_.arboretum_node_for(D->hasCaptures()));
  //getNumCaptures
  // unsigned int
  //captures
  // ArrayRef<Capture>
  //capturesCXXThis
  arboretum_create_edge(obj, context_.data_model_.method_capturesCXXThis, context_.data_model_.arboretum_node_for(D->capturesCXXThis()));
  //blockMissingReturnType
  arboretum_create_edge(obj, context_.data_model_.method_blockMissingReturnType, context_.data_model_.arboretum_node_for(D->blockMissingReturnType()));
  //isConversionFromLambda
  arboretum_create_edge(obj, context_.data_model_.method_isConversionFromLambda, context_.data_model_.arboretum_node_for(D->isConversionFromLambda()));
  //doesNotEscape
  arboretum_create_edge(obj, context_.data_model_.method_doesNotEscape, context_.data_model_.arboretum_node_for(D->doesNotEscape()));
  //canAvoidCopyToHeap
  arboretum_create_edge(obj, context_.data_model_.method_canAvoidCopyToHeap, context_.data_model_.arboretum_node_for(D->canAvoidCopyToHeap()));
  //getBlockManglingNumber
  // unsigned int
  //getBlockManglingContextDecl
  {
    const Id* other = context_.resolve(D->getBlockManglingContextDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getBlockManglingContextDecl, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_1, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitBuiltinTemplateDecl(clang::BuiltinTemplateDecl* D) {
  const Id* obj = context_.resolve(D);
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_2, other);
  }
  //getBuiltinTemplateKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getBuiltinTemplateKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getBuiltinTemplateKind, enum_value);
    }
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXConstructorDecl(clang::CXXConstructorDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Id* obj = context_.resolve(D);
  //getExplicitSpecifier
  // const ExplicitSpecifier
  //isExplicit
  arboretum_create_edge(obj, context_.data_model_.method_isExplicit, context_.data_model_.arboretum_node_for(D->isExplicit()));
  //inits
  // init_const_range
  //getNumCtorInitializers
  // unsigned int
  //isDelegatingConstructor
  arboretum_create_edge(obj, context_.data_model_.method_isDelegatingConstructor, context_.data_model_.arboretum_node_for(D->isDelegatingConstructor()));
  //isDefaultConstructor
  arboretum_create_edge(obj, context_.data_model_.method_isDefaultConstructor, context_.data_model_.arboretum_node_for(D->isDefaultConstructor()));
  //isCopyConstructor
  arboretum_create_edge(obj, context_.data_model_.method_isCopyConstructor, context_.data_model_.arboretum_node_for(D->isCopyConstructor()));
  //isMoveConstructor
  arboretum_create_edge(obj, context_.data_model_.method_isMoveConstructor, context_.data_model_.arboretum_node_for(D->isMoveConstructor()));
  //isCopyOrMoveConstructor
  arboretum_create_edge(obj, context_.data_model_.method_isCopyOrMoveConstructor, context_.data_model_.arboretum_node_for(D->isCopyOrMoveConstructor()));
  //isSpecializationCopyingObject
  arboretum_create_edge(obj, context_.data_model_.method_isSpecializationCopyingObject, context_.data_model_.arboretum_node_for(D->isSpecializationCopyingObject()));
  //isInheritingConstructor
  arboretum_create_edge(obj, context_.data_model_.method_isInheritingConstructor, context_.data_model_.arboretum_node_for(D->isInheritingConstructor()));
  //getInheritedConstructor
  // InheritedConstructor
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXConversionDecl(clang::CXXConversionDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Id* obj = context_.resolve(D);
  //getExplicitSpecifier
  // const ExplicitSpecifier
  //isExplicit
  arboretum_create_edge(obj, context_.data_model_.method_isExplicit_1, context_.data_model_.arboretum_node_for(D->isExplicit()));
  //getConversionType
  {
    const Id* other = context_.resolve(D->getConversionType());
    arboretum_create_edge(obj, context_.data_model_.method_getConversionType, other);
  }
  //isLambdaToBlockPointerConversion
  arboretum_create_edge(obj, context_.data_model_.method_isLambdaToBlockPointerConversion, context_.data_model_.arboretum_node_for(D->isLambdaToBlockPointerConversion()));
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_1, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXDeductionGuideDecl(clang::CXXDeductionGuideDecl* D) {
  const Id* obj = context_.resolve(D);
  //getExplicitSpecifier
  // const ExplicitSpecifier
  //isExplicit
  arboretum_create_edge(obj, context_.data_model_.method_isExplicit_2, context_.data_model_.arboretum_node_for(D->isExplicit()));
  //getDeducedTemplate
  {
    const Id* other = context_.resolve(D->getDeducedTemplate());
    arboretum_create_edge(obj, context_.data_model_.method_getDeducedTemplate, other);
  }
  //getCorrespondingConstructor
  {
    const Id* other = context_.resolve(D->getCorrespondingConstructor());
    arboretum_create_edge(obj, context_.data_model_.method_getCorrespondingConstructor, other);
  }
  //getDeductionCandidateKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getDeductionCandidateKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getDeductionCandidateKind, enum_value);
    }
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXDestructorDecl(clang::CXXDestructorDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Id* obj = context_.resolve(D);
  //getOperatorDelete
  {
    const Id* other = context_.resolve(D->getOperatorDelete());
    arboretum_create_edge(obj, context_.data_model_.method_getOperatorDelete, other);
  }
  //getOperatorDeleteThisArg
  {
    const Id* other = context_.resolve(D->getOperatorDeleteThisArg());
    arboretum_create_edge(obj, context_.data_model_.method_getOperatorDeleteThisArg, other);
  }
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_2, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXMethodDecl(clang::CXXMethodDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Id* obj = context_.resolve(D);
  //isStatic
  arboretum_create_edge(obj, context_.data_model_.method_isStatic, context_.data_model_.arboretum_node_for(D->isStatic()));
  //isInstance
  arboretum_create_edge(obj, context_.data_model_.method_isInstance, context_.data_model_.arboretum_node_for(D->isInstance()));
  //isExplicitObjectMemberFunction
  arboretum_create_edge(obj, context_.data_model_.method_isExplicitObjectMemberFunction, context_.data_model_.arboretum_node_for(D->isExplicitObjectMemberFunction()));
  //isImplicitObjectMemberFunction
  arboretum_create_edge(obj, context_.data_model_.method_isImplicitObjectMemberFunction, context_.data_model_.arboretum_node_for(D->isImplicitObjectMemberFunction()));
  //isConst
  arboretum_create_edge(obj, context_.data_model_.method_isConst_1, context_.data_model_.arboretum_node_for(D->isConst()));
  //isVolatile
  arboretum_create_edge(obj, context_.data_model_.method_isVolatile_1, context_.data_model_.arboretum_node_for(D->isVolatile()));
  //isVirtual
  arboretum_create_edge(obj, context_.data_model_.method_isVirtual, context_.data_model_.arboretum_node_for(D->isVirtual()));
  //isCopyAssignmentOperator
  arboretum_create_edge(obj, context_.data_model_.method_isCopyAssignmentOperator, context_.data_model_.arboretum_node_for(D->isCopyAssignmentOperator()));
  //isMoveAssignmentOperator
  arboretum_create_edge(obj, context_.data_model_.method_isMoveAssignmentOperator, context_.data_model_.arboretum_node_for(D->isMoveAssignmentOperator()));
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_3, other);
  }
  //getMostRecentDecl
  {
    const Id* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getMostRecentDecl, other);
  }
  //size_overridden_methods
  // unsigned int
  //overridden_methods
  // overridden_method_range
  //getParent
  {
    const Id* other = context_.resolve(D->getParent());
    arboretum_create_edge(obj, context_.data_model_.method_getParent, other);
  }
  //getThisType
  {
    const Id* other = context_.resolve(D->getThisType());
    arboretum_create_edge(obj, context_.data_model_.method_getThisType, other);
  }
  //getFunctionObjectParameterReferenceType
  {
    const Id* other = context_.resolve(D->getFunctionObjectParameterReferenceType());
    arboretum_create_edge(obj, context_.data_model_.method_getFunctionObjectParameterReferenceType, other);
  }
  //getFunctionObjectParameterType
  {
    const Id* other = context_.resolve(D->getFunctionObjectParameterType());
    arboretum_create_edge(obj, context_.data_model_.method_getFunctionObjectParameterType, other);
  }
  //getNumExplicitParams
  // unsigned int
  //getMethodQualifiers
  // Qualifiers
  //getRefQualifier
  {
    const Id* enum_value = context_.data_model_.resolve(D->getRefQualifier());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getRefQualifier_1, enum_value);
    }
  }
  //hasInlineBody
  arboretum_create_edge(obj, context_.data_model_.method_hasInlineBody, context_.data_model_.arboretum_node_for(D->hasInlineBody()));
  //isLambdaStaticInvoker
  arboretum_create_edge(obj, context_.data_model_.method_isLambdaStaticInvoker, context_.data_model_.arboretum_node_for(D->isLambdaStaticInvoker()));
  return true;
}

bool ArboretumASTVisitor::VisitCXXRecordDecl(clang::CXXRecordDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Id* obj = context_.resolve(D);
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_4, other);
  }
  //getPreviousDecl
  {
    const Id* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getPreviousDecl, other);
  }
  //getMostRecentDecl
  {
    const Id* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getMostRecentDecl_1, other);
  }
  //getDefinition
  {
    const Id* other = context_.resolve(D->getDefinition());
    arboretum_create_edge(obj, context_.data_model_.method_getDefinition, other);
  }
  //hasDefinition
  arboretum_create_edge(obj, context_.data_model_.method_hasDefinition, context_.data_model_.arboretum_node_for(D->hasDefinition()));
  //isDynamicClass
  arboretum_create_edge(obj, context_.data_model_.method_isDynamicClass, context_.data_model_.arboretum_node_for(D->isDynamicClass()));
  //mayBeDynamicClass
  arboretum_create_edge(obj, context_.data_model_.method_mayBeDynamicClass, context_.data_model_.arboretum_node_for(D->mayBeDynamicClass()));
  //mayBeNonDynamicClass
  arboretum_create_edge(obj, context_.data_model_.method_mayBeNonDynamicClass, context_.data_model_.arboretum_node_for(D->mayBeNonDynamicClass()));
  //isParsingBaseSpecifiers
  arboretum_create_edge(obj, context_.data_model_.method_isParsingBaseSpecifiers, context_.data_model_.arboretum_node_for(D->isParsingBaseSpecifiers()));
  //getODRHash
  // unsigned int
  //getNumBases
  // unsigned int
  //bases
  // base_class_const_range
  //getNumVBases
  // unsigned int
  //vbases
  // base_class_const_range
  //hasAnyDependentBases
  arboretum_create_edge(obj, context_.data_model_.method_hasAnyDependentBases, context_.data_model_.arboretum_node_for(D->hasAnyDependentBases()));
  //methods
  // method_range
  //ctors
  // ctor_range
  //friends
  // friend_range
  //hasFriends
  arboretum_create_edge(obj, context_.data_model_.method_hasFriends, context_.data_model_.arboretum_node_for(D->hasFriends()));
  //hasSimpleCopyConstructor
  arboretum_create_edge(obj, context_.data_model_.method_hasSimpleCopyConstructor, context_.data_model_.arboretum_node_for(D->hasSimpleCopyConstructor()));
  //hasSimpleMoveConstructor
  arboretum_create_edge(obj, context_.data_model_.method_hasSimpleMoveConstructor, context_.data_model_.arboretum_node_for(D->hasSimpleMoveConstructor()));
  //hasSimpleCopyAssignment
  arboretum_create_edge(obj, context_.data_model_.method_hasSimpleCopyAssignment, context_.data_model_.arboretum_node_for(D->hasSimpleCopyAssignment()));
  //hasSimpleMoveAssignment
  arboretum_create_edge(obj, context_.data_model_.method_hasSimpleMoveAssignment, context_.data_model_.arboretum_node_for(D->hasSimpleMoveAssignment()));
  //hasSimpleDestructor
  arboretum_create_edge(obj, context_.data_model_.method_hasSimpleDestructor, context_.data_model_.arboretum_node_for(D->hasSimpleDestructor()));
  //hasDefaultConstructor
  arboretum_create_edge(obj, context_.data_model_.method_hasDefaultConstructor, context_.data_model_.arboretum_node_for(D->hasDefaultConstructor()));
  //needsImplicitDefaultConstructor
  arboretum_create_edge(obj, context_.data_model_.method_needsImplicitDefaultConstructor, context_.data_model_.arboretum_node_for(D->needsImplicitDefaultConstructor()));
  //hasUserDeclaredConstructor
  arboretum_create_edge(obj, context_.data_model_.method_hasUserDeclaredConstructor, context_.data_model_.arboretum_node_for(D->hasUserDeclaredConstructor()));
  //hasUserProvidedDefaultConstructor
  arboretum_create_edge(obj, context_.data_model_.method_hasUserProvidedDefaultConstructor, context_.data_model_.arboretum_node_for(D->hasUserProvidedDefaultConstructor()));
  //hasUserDeclaredCopyConstructor
  arboretum_create_edge(obj, context_.data_model_.method_hasUserDeclaredCopyConstructor, context_.data_model_.arboretum_node_for(D->hasUserDeclaredCopyConstructor()));
  //needsImplicitCopyConstructor
  arboretum_create_edge(obj, context_.data_model_.method_needsImplicitCopyConstructor, context_.data_model_.arboretum_node_for(D->needsImplicitCopyConstructor()));
  //needsOverloadResolutionForCopyConstructor
  arboretum_create_edge(obj, context_.data_model_.method_needsOverloadResolutionForCopyConstructor, context_.data_model_.arboretum_node_for(D->needsOverloadResolutionForCopyConstructor()));
  //implicitCopyConstructorHasConstParam
  arboretum_create_edge(obj, context_.data_model_.method_implicitCopyConstructorHasConstParam, context_.data_model_.arboretum_node_for(D->implicitCopyConstructorHasConstParam()));
  //hasCopyConstructorWithConstParam
  arboretum_create_edge(obj, context_.data_model_.method_hasCopyConstructorWithConstParam, context_.data_model_.arboretum_node_for(D->hasCopyConstructorWithConstParam()));
  //hasUserDeclaredMoveOperation
  arboretum_create_edge(obj, context_.data_model_.method_hasUserDeclaredMoveOperation, context_.data_model_.arboretum_node_for(D->hasUserDeclaredMoveOperation()));
  //hasUserDeclaredMoveConstructor
  arboretum_create_edge(obj, context_.data_model_.method_hasUserDeclaredMoveConstructor, context_.data_model_.arboretum_node_for(D->hasUserDeclaredMoveConstructor()));
  //hasMoveConstructor
  arboretum_create_edge(obj, context_.data_model_.method_hasMoveConstructor, context_.data_model_.arboretum_node_for(D->hasMoveConstructor()));
  //needsImplicitMoveConstructor
  arboretum_create_edge(obj, context_.data_model_.method_needsImplicitMoveConstructor, context_.data_model_.arboretum_node_for(D->needsImplicitMoveConstructor()));
  //needsOverloadResolutionForMoveConstructor
  arboretum_create_edge(obj, context_.data_model_.method_needsOverloadResolutionForMoveConstructor, context_.data_model_.arboretum_node_for(D->needsOverloadResolutionForMoveConstructor()));
  //hasUserDeclaredCopyAssignment
  arboretum_create_edge(obj, context_.data_model_.method_hasUserDeclaredCopyAssignment, context_.data_model_.arboretum_node_for(D->hasUserDeclaredCopyAssignment()));
  //needsImplicitCopyAssignment
  arboretum_create_edge(obj, context_.data_model_.method_needsImplicitCopyAssignment, context_.data_model_.arboretum_node_for(D->needsImplicitCopyAssignment()));
  //needsOverloadResolutionForCopyAssignment
  arboretum_create_edge(obj, context_.data_model_.method_needsOverloadResolutionForCopyAssignment, context_.data_model_.arboretum_node_for(D->needsOverloadResolutionForCopyAssignment()));
  //implicitCopyAssignmentHasConstParam
  arboretum_create_edge(obj, context_.data_model_.method_implicitCopyAssignmentHasConstParam, context_.data_model_.arboretum_node_for(D->implicitCopyAssignmentHasConstParam()));
  //hasCopyAssignmentWithConstParam
  arboretum_create_edge(obj, context_.data_model_.method_hasCopyAssignmentWithConstParam, context_.data_model_.arboretum_node_for(D->hasCopyAssignmentWithConstParam()));
  //hasUserDeclaredMoveAssignment
  arboretum_create_edge(obj, context_.data_model_.method_hasUserDeclaredMoveAssignment, context_.data_model_.arboretum_node_for(D->hasUserDeclaredMoveAssignment()));
  //hasMoveAssignment
  arboretum_create_edge(obj, context_.data_model_.method_hasMoveAssignment, context_.data_model_.arboretum_node_for(D->hasMoveAssignment()));
  //needsImplicitMoveAssignment
  arboretum_create_edge(obj, context_.data_model_.method_needsImplicitMoveAssignment, context_.data_model_.arboretum_node_for(D->needsImplicitMoveAssignment()));
  //needsOverloadResolutionForMoveAssignment
  arboretum_create_edge(obj, context_.data_model_.method_needsOverloadResolutionForMoveAssignment, context_.data_model_.arboretum_node_for(D->needsOverloadResolutionForMoveAssignment()));
  //hasUserDeclaredDestructor
  arboretum_create_edge(obj, context_.data_model_.method_hasUserDeclaredDestructor, context_.data_model_.arboretum_node_for(D->hasUserDeclaredDestructor()));
  //needsImplicitDestructor
  arboretum_create_edge(obj, context_.data_model_.method_needsImplicitDestructor, context_.data_model_.arboretum_node_for(D->needsImplicitDestructor()));
  //needsOverloadResolutionForDestructor
  arboretum_create_edge(obj, context_.data_model_.method_needsOverloadResolutionForDestructor, context_.data_model_.arboretum_node_for(D->needsOverloadResolutionForDestructor()));
  //isLambda
  arboretum_create_edge(obj, context_.data_model_.method_isLambda, context_.data_model_.arboretum_node_for(D->isLambda()));
  //isGenericLambda
  arboretum_create_edge(obj, context_.data_model_.method_isGenericLambda, context_.data_model_.arboretum_node_for(D->isGenericLambda()));
  //lambdaIsDefaultConstructibleAndAssignable
  arboretum_create_edge(obj, context_.data_model_.method_lambdaIsDefaultConstructibleAndAssignable, context_.data_model_.arboretum_node_for(D->lambdaIsDefaultConstructibleAndAssignable()));
  //getLambdaCallOperator
  {
    const Id* other = context_.resolve(D->getLambdaCallOperator());
    arboretum_create_edge(obj, context_.data_model_.method_getLambdaCallOperator, other);
  }
  //getDependentLambdaCallOperator
  {
    const Id* other = context_.resolve(D->getDependentLambdaCallOperator());
    arboretum_create_edge(obj, context_.data_model_.method_getDependentLambdaCallOperator, other);
  }
  //getGenericLambdaTemplateParameterList
  //getLambdaExplicitTemplateParameters
  // ArrayRef<NamedDecl *>
  //isCapturelessLambda
  arboretum_create_edge(obj, context_.data_model_.method_isCapturelessLambda, context_.data_model_.arboretum_node_for(D->isCapturelessLambda()));
  //captures
  // capture_const_range
  //capture_size
  // unsigned int
  //getVisibleConversionFunctions
  // llvm::iterator_range<conversion_iterator>
  //isAggregate
  arboretum_create_edge(obj, context_.data_model_.method_isAggregate, context_.data_model_.arboretum_node_for(D->isAggregate()));
  //hasInClassInitializer
  arboretum_create_edge(obj, context_.data_model_.method_hasInClassInitializer, context_.data_model_.arboretum_node_for(D->hasInClassInitializer()));
  //hasUninitializedReferenceMember
  arboretum_create_edge(obj, context_.data_model_.method_hasUninitializedReferenceMember, context_.data_model_.arboretum_node_for(D->hasUninitializedReferenceMember()));
  //isPOD
  arboretum_create_edge(obj, context_.data_model_.method_isPOD, context_.data_model_.arboretum_node_for(D->isPOD()));
  //isCLike
  arboretum_create_edge(obj, context_.data_model_.method_isCLike, context_.data_model_.arboretum_node_for(D->isCLike()));
  //isEmpty
  arboretum_create_edge(obj, context_.data_model_.method_isEmpty, context_.data_model_.arboretum_node_for(D->isEmpty()));
  //hasInitMethod
  arboretum_create_edge(obj, context_.data_model_.method_hasInitMethod, context_.data_model_.arboretum_node_for(D->hasInitMethod()));
  //hasPrivateFields
  arboretum_create_edge(obj, context_.data_model_.method_hasPrivateFields, context_.data_model_.arboretum_node_for(D->hasPrivateFields()));
  //hasProtectedFields
  arboretum_create_edge(obj, context_.data_model_.method_hasProtectedFields, context_.data_model_.arboretum_node_for(D->hasProtectedFields()));
  //hasDirectFields
  arboretum_create_edge(obj, context_.data_model_.method_hasDirectFields, context_.data_model_.arboretum_node_for(D->hasDirectFields()));
  //isPolymorphic
  arboretum_create_edge(obj, context_.data_model_.method_isPolymorphic, context_.data_model_.arboretum_node_for(D->isPolymorphic()));
  //isAbstract
  arboretum_create_edge(obj, context_.data_model_.method_isAbstract, context_.data_model_.arboretum_node_for(D->isAbstract()));
  //isStandardLayout
  arboretum_create_edge(obj, context_.data_model_.method_isStandardLayout, context_.data_model_.arboretum_node_for(D->isStandardLayout()));
  //isCXX11StandardLayout
  arboretum_create_edge(obj, context_.data_model_.method_isCXX11StandardLayout, context_.data_model_.arboretum_node_for(D->isCXX11StandardLayout()));
  //hasMutableFields
  arboretum_create_edge(obj, context_.data_model_.method_hasMutableFields, context_.data_model_.arboretum_node_for(D->hasMutableFields()));
  //hasVariantMembers
  arboretum_create_edge(obj, context_.data_model_.method_hasVariantMembers, context_.data_model_.arboretum_node_for(D->hasVariantMembers()));
  //hasTrivialDefaultConstructor
  arboretum_create_edge(obj, context_.data_model_.method_hasTrivialDefaultConstructor, context_.data_model_.arboretum_node_for(D->hasTrivialDefaultConstructor()));
  //hasNonTrivialDefaultConstructor
  arboretum_create_edge(obj, context_.data_model_.method_hasNonTrivialDefaultConstructor, context_.data_model_.arboretum_node_for(D->hasNonTrivialDefaultConstructor()));
  //hasConstexprNonCopyMoveConstructor
  arboretum_create_edge(obj, context_.data_model_.method_hasConstexprNonCopyMoveConstructor, context_.data_model_.arboretum_node_for(D->hasConstexprNonCopyMoveConstructor()));
  //defaultedDefaultConstructorIsConstexpr
  arboretum_create_edge(obj, context_.data_model_.method_defaultedDefaultConstructorIsConstexpr, context_.data_model_.arboretum_node_for(D->defaultedDefaultConstructorIsConstexpr()));
  //hasConstexprDefaultConstructor
  arboretum_create_edge(obj, context_.data_model_.method_hasConstexprDefaultConstructor, context_.data_model_.arboretum_node_for(D->hasConstexprDefaultConstructor()));
  //hasTrivialCopyConstructor
  arboretum_create_edge(obj, context_.data_model_.method_hasTrivialCopyConstructor, context_.data_model_.arboretum_node_for(D->hasTrivialCopyConstructor()));
  //hasTrivialCopyConstructorForCall
  arboretum_create_edge(obj, context_.data_model_.method_hasTrivialCopyConstructorForCall, context_.data_model_.arboretum_node_for(D->hasTrivialCopyConstructorForCall()));
  //hasNonTrivialCopyConstructor
  arboretum_create_edge(obj, context_.data_model_.method_hasNonTrivialCopyConstructor, context_.data_model_.arboretum_node_for(D->hasNonTrivialCopyConstructor()));
  //hasNonTrivialCopyConstructorForCall
  arboretum_create_edge(obj, context_.data_model_.method_hasNonTrivialCopyConstructorForCall, context_.data_model_.arboretum_node_for(D->hasNonTrivialCopyConstructorForCall()));
  //hasTrivialMoveConstructor
  arboretum_create_edge(obj, context_.data_model_.method_hasTrivialMoveConstructor, context_.data_model_.arboretum_node_for(D->hasTrivialMoveConstructor()));
  //hasTrivialMoveConstructorForCall
  arboretum_create_edge(obj, context_.data_model_.method_hasTrivialMoveConstructorForCall, context_.data_model_.arboretum_node_for(D->hasTrivialMoveConstructorForCall()));
  //hasNonTrivialMoveConstructor
  arboretum_create_edge(obj, context_.data_model_.method_hasNonTrivialMoveConstructor, context_.data_model_.arboretum_node_for(D->hasNonTrivialMoveConstructor()));
  //hasNonTrivialMoveConstructorForCall
  arboretum_create_edge(obj, context_.data_model_.method_hasNonTrivialMoveConstructorForCall, context_.data_model_.arboretum_node_for(D->hasNonTrivialMoveConstructorForCall()));
  //hasTrivialCopyAssignment
  arboretum_create_edge(obj, context_.data_model_.method_hasTrivialCopyAssignment, context_.data_model_.arboretum_node_for(D->hasTrivialCopyAssignment()));
  //hasNonTrivialCopyAssignment
  arboretum_create_edge(obj, context_.data_model_.method_hasNonTrivialCopyAssignment, context_.data_model_.arboretum_node_for(D->hasNonTrivialCopyAssignment()));
  //hasTrivialMoveAssignment
  arboretum_create_edge(obj, context_.data_model_.method_hasTrivialMoveAssignment, context_.data_model_.arboretum_node_for(D->hasTrivialMoveAssignment()));
  //hasNonTrivialMoveAssignment
  arboretum_create_edge(obj, context_.data_model_.method_hasNonTrivialMoveAssignment, context_.data_model_.arboretum_node_for(D->hasNonTrivialMoveAssignment()));
  //defaultedDestructorIsConstexpr
  arboretum_create_edge(obj, context_.data_model_.method_defaultedDestructorIsConstexpr, context_.data_model_.arboretum_node_for(D->defaultedDestructorIsConstexpr()));
  //hasConstexprDestructor
  arboretum_create_edge(obj, context_.data_model_.method_hasConstexprDestructor, context_.data_model_.arboretum_node_for(D->hasConstexprDestructor()));
  //hasTrivialDestructor
  arboretum_create_edge(obj, context_.data_model_.method_hasTrivialDestructor, context_.data_model_.arboretum_node_for(D->hasTrivialDestructor()));
  //hasTrivialDestructorForCall
  arboretum_create_edge(obj, context_.data_model_.method_hasTrivialDestructorForCall, context_.data_model_.arboretum_node_for(D->hasTrivialDestructorForCall()));
  //hasNonTrivialDestructor
  arboretum_create_edge(obj, context_.data_model_.method_hasNonTrivialDestructor, context_.data_model_.arboretum_node_for(D->hasNonTrivialDestructor()));
  //hasNonTrivialDestructorForCall
  arboretum_create_edge(obj, context_.data_model_.method_hasNonTrivialDestructorForCall, context_.data_model_.arboretum_node_for(D->hasNonTrivialDestructorForCall()));
  //allowConstDefaultInit
  arboretum_create_edge(obj, context_.data_model_.method_allowConstDefaultInit, context_.data_model_.arboretum_node_for(D->allowConstDefaultInit()));
  //hasIrrelevantDestructor
  arboretum_create_edge(obj, context_.data_model_.method_hasIrrelevantDestructor, context_.data_model_.arboretum_node_for(D->hasIrrelevantDestructor()));
  //hasNonLiteralTypeFieldsOrBases
  arboretum_create_edge(obj, context_.data_model_.method_hasNonLiteralTypeFieldsOrBases, context_.data_model_.arboretum_node_for(D->hasNonLiteralTypeFieldsOrBases()));
  //hasInheritedConstructor
  arboretum_create_edge(obj, context_.data_model_.method_hasInheritedConstructor, context_.data_model_.arboretum_node_for(D->hasInheritedConstructor()));
  //hasInheritedAssignment
  arboretum_create_edge(obj, context_.data_model_.method_hasInheritedAssignment, context_.data_model_.arboretum_node_for(D->hasInheritedAssignment()));
  //isTriviallyCopyable
  arboretum_create_edge(obj, context_.data_model_.method_isTriviallyCopyable, context_.data_model_.arboretum_node_for(D->isTriviallyCopyable()));
  //isTriviallyCopyConstructible
  arboretum_create_edge(obj, context_.data_model_.method_isTriviallyCopyConstructible, context_.data_model_.arboretum_node_for(D->isTriviallyCopyConstructible()));
  //isTrivial
  arboretum_create_edge(obj, context_.data_model_.method_isTrivial, context_.data_model_.arboretum_node_for(D->isTrivial()));
  //isLiteral
  arboretum_create_edge(obj, context_.data_model_.method_isLiteral, context_.data_model_.arboretum_node_for(D->isLiteral()));
  //isStructural
  arboretum_create_edge(obj, context_.data_model_.method_isStructural, context_.data_model_.arboretum_node_for(D->isStructural()));
  //getInstantiatedFromMemberClass
  {
    const Id* other = context_.resolve(D->getInstantiatedFromMemberClass());
    arboretum_create_edge(obj, context_.data_model_.method_getInstantiatedFromMemberClass, other);
  }
  //getMemberSpecializationInfo
  //getDescribedClassTemplate
  {
    const Id* other = context_.resolve(D->getDescribedClassTemplate());
    arboretum_create_edge(obj, context_.data_model_.method_getDescribedClassTemplate, other);
  }
  //getTemplateSpecializationKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getTemplateSpecializationKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getTemplateSpecializationKind, enum_value);
    }
  }
  //getTemplateInstantiationPattern
  {
    const Id* other = context_.resolve(D->getTemplateInstantiationPattern());
    arboretum_create_edge(obj, context_.data_model_.method_getTemplateInstantiationPattern, other);
  }
  //getDestructor
  {
    const Id* other = context_.resolve(D->getDestructor());
    arboretum_create_edge(obj, context_.data_model_.method_getDestructor, other);
  }
  //isAnyDestructorNoReturn
  arboretum_create_edge(obj, context_.data_model_.method_isAnyDestructorNoReturn, context_.data_model_.arboretum_node_for(D->isAnyDestructorNoReturn()));
  //isLocalClass
  {
    const Id* other = context_.resolve(D->isLocalClass());
    arboretum_create_edge(obj, context_.data_model_.method_isLocalClass, other);
  }
  //mayBeAbstract
  arboretum_create_edge(obj, context_.data_model_.method_mayBeAbstract, context_.data_model_.arboretum_node_for(D->mayBeAbstract()));
  //isEffectivelyFinal
  arboretum_create_edge(obj, context_.data_model_.method_isEffectivelyFinal, context_.data_model_.arboretum_node_for(D->isEffectivelyFinal()));
  //getLambdaManglingNumber
  // unsigned int
  //getLambdaIndexInContext
  // unsigned int
  //getLambdaNumbering
  // LambdaNumbering
  //getDeviceLambdaManglingNumber
  // unsigned int
  //getMSVtorDispMode
  {
    const Id* enum_value = context_.data_model_.resolve(D->getMSVtorDispMode());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getMSVtorDispMode, enum_value);
    }
  }
  //isDependentLambda
  arboretum_create_edge(obj, context_.data_model_.method_isDependentLambda, context_.data_model_.arboretum_node_for(D->isDependentLambda()));
  //isNeverDependentLambda
  arboretum_create_edge(obj, context_.data_model_.method_isNeverDependentLambda, context_.data_model_.arboretum_node_for(D->isNeverDependentLambda()));
  //getLambdaDependencyKind
  // unsigned int
  //getLambdaTypeInfo
  return true;
}

bool ArboretumASTVisitor::VisitCapturedDecl(clang::CapturedDecl* D) {
  const Id* obj = context_.resolve(D);
  //getBody
  {
    const Id* other = context_.resolve(D->getBody());
    arboretum_create_edge(obj, context_.data_model_.method_getBody_1, other);
  }
  //isNothrow
  arboretum_create_edge(obj, context_.data_model_.method_isNothrow, context_.data_model_.arboretum_node_for(D->isNothrow()));
  //getNumParams
  // unsigned int
  //parameters
  // ArrayRef<ImplicitParamDecl *>
  //getContextParam
  {
    const Id* other = context_.resolve(D->getContextParam());
    arboretum_create_edge(obj, context_.data_model_.method_getContextParam, other);
  }
  //getContextParamPosition
  // unsigned int
  return true;
}

bool ArboretumASTVisitor::VisitClassTemplateDecl(clang::ClassTemplateDecl* D) {
  const Id* obj = context_.resolve(D);
  //getTemplatedDecl
  {
    const Id* other = context_.resolve(D->getTemplatedDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getTemplatedDecl, other);
  }
  //isThisDeclarationADefinition
  arboretum_create_edge(obj, context_.data_model_.method_isThisDeclarationADefinition, context_.data_model_.arboretum_node_for(D->isThisDeclarationADefinition()));
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_5, other);
  }
  //getPreviousDecl
  {
    const Id* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getPreviousDecl_1, other);
  }
  //getMostRecentDecl
  {
    const Id* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getMostRecentDecl_2, other);
  }
  //getInstantiatedFromMemberTemplate
  {
    const Id* other = context_.resolve(D->getInstantiatedFromMemberTemplate());
    arboretum_create_edge(obj, context_.data_model_.method_getInstantiatedFromMemberTemplate, other);
  }
  //specializations
  // spec_range
  return true;
}

bool ArboretumASTVisitor::VisitClassTemplatePartialSpecializationDecl(clang::ClassTemplatePartialSpecializationDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Id* obj = context_.resolve(D);
  //getTemplateParameters
  //hasAssociatedConstraints
  arboretum_create_edge(obj, context_.data_model_.method_hasAssociatedConstraints, context_.data_model_.arboretum_node_for(D->hasAssociatedConstraints()));
  //getTemplateArgsAsWritten
  //getInstantiatedFromMember
  {
    const Id* other = context_.resolve(D->getInstantiatedFromMember());
    arboretum_create_edge(obj, context_.data_model_.method_getInstantiatedFromMember, other);
  }
  //getInstantiatedFromMemberTemplate
  {
    const Id* other = context_.resolve(D->getInstantiatedFromMemberTemplate());
    arboretum_create_edge(obj, context_.data_model_.method_getInstantiatedFromMemberTemplate_1, other);
  }
  //getInjectedSpecializationType
  {
    const Id* other = context_.resolve(D->getInjectedSpecializationType());
    arboretum_create_edge(obj, context_.data_model_.method_getInjectedSpecializationType_1, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitClassTemplateSpecializationDecl(clang::ClassTemplateSpecializationDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Id* obj = context_.resolve(D);
  //getSpecializedTemplate
  {
    const Id* other = context_.resolve(D->getSpecializedTemplate());
    arboretum_create_edge(obj, context_.data_model_.method_getSpecializedTemplate, other);
  }
  //getTemplateArgs
  // const TemplateArgumentList &
  //getSpecializationKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getSpecializationKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getSpecializationKind, enum_value);
    }
  }
  //isExplicitSpecialization
  arboretum_create_edge(obj, context_.data_model_.method_isExplicitSpecialization, context_.data_model_.arboretum_node_for(D->isExplicitSpecialization()));
  //isClassScopeExplicitSpecialization
  arboretum_create_edge(obj, context_.data_model_.method_isClassScopeExplicitSpecialization, context_.data_model_.arboretum_node_for(D->isClassScopeExplicitSpecialization()));
  //isExplicitInstantiationOrSpecialization
  arboretum_create_edge(obj, context_.data_model_.method_isExplicitInstantiationOrSpecialization, context_.data_model_.arboretum_node_for(D->isExplicitInstantiationOrSpecialization()));
  //getPointOfInstantiation
  {
    const Id* other = context_.source_model_.resolve(D->getPointOfInstantiation());
    arboretum_create_edge(obj, context_.data_model_.method_getPointOfInstantiation, other);
  }
  //getInstantiatedFrom
  // llvm::PointerUnion<ClassTemplateDecl *, ClassTemplatePartialSpecializationDecl *>
  //getSpecializedTemplateOrPartial
  // llvm::PointerUnion<ClassTemplateDecl *, ClassTemplatePartialSpecializationDecl *>
  //getTemplateInstantiationArgs
  // const TemplateArgumentList &
  //getTypeAsWritten
  //getExternLoc
  {
    const Id* other = context_.source_model_.resolve(D->getExternLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getExternLoc, other);
  }
  //getTemplateKeywordLoc
  {
    const Id* other = context_.source_model_.resolve(D->getTemplateKeywordLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getTemplateKeywordLoc, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_3, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitConceptDecl(clang::ConceptDecl* D) {
  const Id* obj = context_.resolve(D);
  //getConstraintExpr
  {
    const Id* other = context_.resolve(D->getConstraintExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getConstraintExpr, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_4, other);
  }
  //isTypeConcept
  arboretum_create_edge(obj, context_.data_model_.method_isTypeConcept, context_.data_model_.arboretum_node_for(D->isTypeConcept()));
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_6, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitConstructorUsingShadowDecl(clang::ConstructorUsingShadowDecl* D) {
  const Id* obj = context_.resolve(D);
  //getIntroducer
  {
    const Id* other = context_.resolve(D->getIntroducer());
    arboretum_create_edge(obj, context_.data_model_.method_getIntroducer, other);
  }
  //getParent
  {
    const Id* other = context_.resolve(D->getParent());
    arboretum_create_edge(obj, context_.data_model_.method_getParent_1, other);
  }
  //getNominatedBaseClassShadowDecl
  {
    const Id* other = context_.resolve(D->getNominatedBaseClassShadowDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getNominatedBaseClassShadowDecl, other);
  }
  //getConstructedBaseClassShadowDecl
  {
    const Id* other = context_.resolve(D->getConstructedBaseClassShadowDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getConstructedBaseClassShadowDecl, other);
  }
  //getNominatedBaseClass
  {
    const Id* other = context_.resolve(D->getNominatedBaseClass());
    arboretum_create_edge(obj, context_.data_model_.method_getNominatedBaseClass, other);
  }
  //getConstructedBaseClass
  {
    const Id* other = context_.resolve(D->getConstructedBaseClass());
    arboretum_create_edge(obj, context_.data_model_.method_getConstructedBaseClass, other);
  }
  //constructsVirtualBase
  arboretum_create_edge(obj, context_.data_model_.method_constructsVirtualBase, context_.data_model_.arboretum_node_for(D->constructsVirtualBase()));
  return true;
}

bool ArboretumASTVisitor::VisitDecl(clang::Decl* D) {
  const Id* obj = context_.resolve(D);
  switch(D->getKind()) {
    case clang::Decl::CXXRecord: {
      assert(context_.data_model_.class_CXXRecordDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXRecordDecl); 
    } break;
    case clang::Decl::Using: {
      assert(context_.data_model_.class_UsingDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UsingDecl); 
    } break;
    case clang::Decl::HLSLBuffer: {
      assert(context_.data_model_.class_HLSLBufferDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_HLSLBufferDecl); 
    } break;
    case clang::Decl::ObjCCompatibleAlias: {
      assert(context_.data_model_.class_ObjCCompatibleAliasDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCCompatibleAliasDecl); 
    } break;
    case clang::Decl::ClassTemplateSpecialization: {
      assert(context_.data_model_.class_ClassTemplateSpecializationDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ClassTemplateSpecializationDecl); 
    } break;
    case clang::Decl::NamespaceAlias: {
      assert(context_.data_model_.class_NamespaceAliasDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_NamespaceAliasDecl); 
    } break;
    case clang::Decl::TemplateParamObject: {
      assert(context_.data_model_.class_TemplateParamObjectDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_TemplateParamObjectDecl); 
    } break;
    case clang::Decl::MSProperty: {
      assert(context_.data_model_.class_MSPropertyDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_MSPropertyDecl); 
    } break;
    case clang::Decl::ConstructorUsingShadow: {
      assert(context_.data_model_.class_ConstructorUsingShadowDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ConstructorUsingShadowDecl); 
    } break;
    case clang::Decl::UsingShadow: {
      assert(context_.data_model_.class_UsingShadowDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UsingShadowDecl); 
    } break;
    case clang::Decl::UsingEnum: {
      assert(context_.data_model_.class_UsingEnumDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UsingEnumDecl); 
    } break;
    case clang::Decl::Record: {
      assert(context_.data_model_.class_RecordDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_RecordDecl); 
    } break;
    case clang::Decl::NonTypeTemplateParm: {
      assert(context_.data_model_.class_NonTypeTemplateParmDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_NonTypeTemplateParmDecl); 
    } break;
    case clang::Decl::ObjCIvar: {
      assert(context_.data_model_.class_ObjCIvarDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCIvarDecl); 
    } break;
    case clang::Decl::ClassTemplate: {
      assert(context_.data_model_.class_ClassTemplateDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ClassTemplateDecl); 
    } break;
    case clang::Decl::ObjCTypeParam: {
      assert(context_.data_model_.class_ObjCTypeParamDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCTypeParamDecl); 
    } break;
    case clang::Decl::TemplateTemplateParm: {
      assert(context_.data_model_.class_TemplateTemplateParmDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_TemplateTemplateParmDecl); 
    } break;
    case clang::Decl::Concept: {
      assert(context_.data_model_.class_ConceptDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ConceptDecl); 
    } break;
    case clang::Decl::ObjCMethod: {
      assert(context_.data_model_.class_ObjCMethodDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCMethodDecl); 
    } break;
    case clang::Decl::ObjCProperty: {
      assert(context_.data_model_.class_ObjCPropertyDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCPropertyDecl); 
    } break;
    case clang::Decl::UnresolvedUsingIfExists: {
      assert(context_.data_model_.class_UnresolvedUsingIfExistsDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UnresolvedUsingIfExistsDecl); 
    } break;
    case clang::Decl::ObjCProtocol: {
      assert(context_.data_model_.class_ObjCProtocolDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCProtocolDecl); 
    } break;
    case clang::Decl::OMPAllocate: {
      assert(context_.data_model_.class_OMPAllocateDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPAllocateDecl); 
    } break;
    case clang::Decl::ObjCInterface: {
      assert(context_.data_model_.class_ObjCInterfaceDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCInterfaceDecl); 
    } break;
    case clang::Decl::StaticAssert: {
      assert(context_.data_model_.class_StaticAssertDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_StaticAssertDecl); 
    } break;
    case clang::Decl::FileScopeAsm: {
      assert(context_.data_model_.class_FileScopeAsmDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_FileScopeAsmDecl); 
    } break;
    case clang::Decl::Var: {
      assert(context_.data_model_.class_VarDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_VarDecl); 
    } break;
    case clang::Decl::ObjCCategoryImpl: {
      assert(context_.data_model_.class_ObjCCategoryImplDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCCategoryImplDecl); 
    } break;
    case clang::Decl::BuiltinTemplate: {
      assert(context_.data_model_.class_BuiltinTemplateDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_BuiltinTemplateDecl); 
    } break;
    case clang::Decl::TypeAlias: {
      assert(context_.data_model_.class_TypeAliasDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_TypeAliasDecl); 
    } break;
    case clang::Decl::Typedef: {
      assert(context_.data_model_.class_TypedefDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_TypedefDecl); 
    } break;
    case clang::Decl::OMPDeclareMapper: {
      assert(context_.data_model_.class_OMPDeclareMapperDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPDeclareMapperDecl); 
    } break;
    case clang::Decl::PragmaDetectMismatch: {
      assert(context_.data_model_.class_PragmaDetectMismatchDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_PragmaDetectMismatchDecl); 
    } break;
    case clang::Decl::UnnamedGlobalConstant: {
      assert(context_.data_model_.class_UnnamedGlobalConstantDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UnnamedGlobalConstantDecl); 
    } break;
    case clang::Decl::UnresolvedUsingValue: {
      assert(context_.data_model_.class_UnresolvedUsingValueDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UnresolvedUsingValueDecl); 
    } break;
    case clang::Decl::CXXConstructor: {
      assert(context_.data_model_.class_CXXConstructorDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXConstructorDecl); 
    } break;
    case clang::Decl::Field: {
      assert(context_.data_model_.class_FieldDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_FieldDecl); 
    } break;
    case clang::Decl::Captured: {
      assert(context_.data_model_.class_CapturedDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CapturedDecl); 
    } break;
    case clang::Decl::ObjCAtDefsField: {
      assert(context_.data_model_.class_ObjCAtDefsFieldDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCAtDefsFieldDecl); 
    } break;
    case clang::Decl::Block: {
      assert(context_.data_model_.class_BlockDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_BlockDecl); 
    } break;
    case clang::Decl::OMPDeclareReduction: {
      assert(context_.data_model_.class_OMPDeclareReductionDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPDeclareReductionDecl); 
    } break;
    case clang::Decl::Label: {
      assert(context_.data_model_.class_LabelDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_LabelDecl); 
    } break;
    case clang::Decl::AccessSpec: {
      assert(context_.data_model_.class_AccessSpecDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_AccessSpecDecl); 
    } break;
    case clang::Decl::FunctionTemplate: {
      assert(context_.data_model_.class_FunctionTemplateDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_FunctionTemplateDecl); 
    } break;
    case clang::Decl::PragmaComment: {
      assert(context_.data_model_.class_PragmaCommentDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_PragmaCommentDecl); 
    } break;
    case clang::Decl::Function: {
      assert(context_.data_model_.class_FunctionDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_FunctionDecl); 
    } break;
    case clang::Decl::CXXDeductionGuide: {
      assert(context_.data_model_.class_CXXDeductionGuideDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXDeductionGuideDecl); 
    } break;
    case clang::Decl::Namespace: {
      assert(context_.data_model_.class_NamespaceDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_NamespaceDecl); 
    } break;
    case clang::Decl::Decomposition: {
      assert(context_.data_model_.class_DecompositionDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DecompositionDecl); 
    } break;
    case clang::Decl::OMPCapturedExpr: {
      assert(context_.data_model_.class_OMPCapturedExprDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPCapturedExprDecl); 
    } break;
    case clang::Decl::ClassTemplatePartialSpecialization: {
      assert(context_.data_model_.class_ClassTemplatePartialSpecializationDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ClassTemplatePartialSpecializationDecl); 
    } break;
    case clang::Decl::TopLevelStmt: {
      assert(context_.data_model_.class_TopLevelStmtDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_TopLevelStmtDecl); 
    } break;
    case clang::Decl::CXXDestructor: {
      assert(context_.data_model_.class_CXXDestructorDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXDestructorDecl); 
    } break;
    case clang::Decl::VarTemplate: {
      assert(context_.data_model_.class_VarTemplateDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_VarTemplateDecl); 
    } break;
    case clang::Decl::ImplicitParam: {
      assert(context_.data_model_.class_ImplicitParamDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ImplicitParamDecl); 
    } break;
    case clang::Decl::UsingDirective: {
      assert(context_.data_model_.class_UsingDirectiveDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UsingDirectiveDecl); 
    } break;
    case clang::Decl::ObjCCategory: {
      assert(context_.data_model_.class_ObjCCategoryDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCCategoryDecl); 
    } break;
    case clang::Decl::ParmVar: {
      assert(context_.data_model_.class_ParmVarDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ParmVarDecl); 
    } break;
    case clang::Decl::VarTemplatePartialSpecialization: {
      assert(context_.data_model_.class_VarTemplatePartialSpecializationDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_VarTemplatePartialSpecializationDecl); 
    } break;
    case clang::Decl::OMPRequires: {
      assert(context_.data_model_.class_OMPRequiresDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPRequiresDecl); 
    } break;
    case clang::Decl::EnumConstant: {
      assert(context_.data_model_.class_EnumConstantDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_EnumConstantDecl); 
    } break;
    case clang::Decl::IndirectField: {
      assert(context_.data_model_.class_IndirectFieldDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_IndirectFieldDecl); 
    } break;
    case clang::Decl::CXXConversion: {
      assert(context_.data_model_.class_CXXConversionDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXConversionDecl); 
    } break;
    case clang::Decl::TypeAliasTemplate: {
      assert(context_.data_model_.class_TypeAliasTemplateDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_TypeAliasTemplateDecl); 
    } break;
    case clang::Decl::UsingPack: {
      assert(context_.data_model_.class_UsingPackDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UsingPackDecl); 
    } break;
    case clang::Decl::Friend: {
      assert(context_.data_model_.class_FriendDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_FriendDecl); 
    } break;
    case clang::Decl::RequiresExprBody: {
      assert(context_.data_model_.class_RequiresExprBodyDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_RequiresExprBodyDecl); 
    } break;
    case clang::Decl::MSGuid: {
      assert(context_.data_model_.class_MSGuidDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_MSGuidDecl); 
    } break;
    case clang::Decl::Export: {
      assert(context_.data_model_.class_ExportDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ExportDecl); 
    } break;
    case clang::Decl::LifetimeExtendedTemporary: {
      assert(context_.data_model_.class_LifetimeExtendedTemporaryDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_LifetimeExtendedTemporaryDecl); 
    } break;
    case clang::Decl::TemplateTypeParm: {
      assert(context_.data_model_.class_TemplateTypeParmDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_TemplateTypeParmDecl); 
    } break;
    case clang::Decl::Enum: {
      assert(context_.data_model_.class_EnumDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_EnumDecl); 
    } break;
    case clang::Decl::ImplicitConceptSpecialization: {
      assert(context_.data_model_.class_ImplicitConceptSpecializationDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ImplicitConceptSpecializationDecl); 
    } break;
    case clang::Decl::VarTemplateSpecialization: {
      assert(context_.data_model_.class_VarTemplateSpecializationDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_VarTemplateSpecializationDecl); 
    } break;
    case clang::Decl::TranslationUnit: {
      assert(context_.data_model_.class_TranslationUnitDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_TranslationUnitDecl); 
    } break;
    case clang::Decl::Import: {
      assert(context_.data_model_.class_ImportDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ImportDecl); 
    } break;
    case clang::Decl::Empty: {
      assert(context_.data_model_.class_EmptyDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_EmptyDecl); 
    } break;
    case clang::Decl::ObjCPropertyImpl: {
      assert(context_.data_model_.class_ObjCPropertyImplDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCPropertyImplDecl); 
    } break;
    case clang::Decl::CXXMethod: {
      assert(context_.data_model_.class_CXXMethodDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXMethodDecl); 
    } break;
    case clang::Decl::FriendTemplate: {
      assert(context_.data_model_.class_FriendTemplateDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_FriendTemplateDecl); 
    } break;
    case clang::Decl::ExternCContext: {
      assert(context_.data_model_.class_ExternCContextDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ExternCContextDecl); 
    } break;
    case clang::Decl::Binding: {
      assert(context_.data_model_.class_BindingDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_BindingDecl); 
    } break;
    case clang::Decl::OMPThreadPrivate: {
      assert(context_.data_model_.class_OMPThreadPrivateDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPThreadPrivateDecl); 
    } break;
    case clang::Decl::LinkageSpec: {
      assert(context_.data_model_.class_LinkageSpecDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_LinkageSpecDecl); 
    } break;
    case clang::Decl::ObjCImplementation: {
      assert(context_.data_model_.class_ObjCImplementationDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCImplementationDecl); 
    } break;
    case clang::Decl::UnresolvedUsingTypename: {
      assert(context_.data_model_.class_UnresolvedUsingTypenameDecl != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UnresolvedUsingTypenameDecl); 
    } break;
    default: break;
  }

  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_5, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc, other);
  }
  //getLocation
  {
    const Id* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.method_getLocation, other);
  }
  //getNextDeclInContext
  {
    const Id* other = context_.resolve(D->getNextDeclInContext());
    arboretum_create_edge(obj, context_.data_model_.method_getNextDeclInContext, other);
  }
  //getDeclContext
  //getNonTransparentDeclContext
  //getNonClosureContext
  {
    const Id* other = context_.resolve(D->getNonClosureContext());
    arboretum_create_edge(obj, context_.data_model_.method_getNonClosureContext, other);
  }
  //getTranslationUnitDecl
  {
    const Id* other = context_.resolve(D->getTranslationUnitDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getTranslationUnitDecl, other);
  }
  //isInAnonymousNamespace
  arboretum_create_edge(obj, context_.data_model_.method_isInAnonymousNamespace, context_.data_model_.arboretum_node_for(D->isInAnonymousNamespace()));
  //isInStdNamespace
  arboretum_create_edge(obj, context_.data_model_.method_isInStdNamespace, context_.data_model_.arboretum_node_for(D->isInStdNamespace()));
  //isFileContextDecl
  arboretum_create_edge(obj, context_.data_model_.method_isFileContextDecl, context_.data_model_.arboretum_node_for(D->isFileContextDecl()));
  //getLangOpts
  // const LangOptions &
  //getAccess
  {
    const Id* enum_value = context_.data_model_.resolve(D->getAccess());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getAccess, enum_value);
    }
  }
  //getAccessUnsafe
  {
    const Id* enum_value = context_.data_model_.resolve(D->getAccessUnsafe());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getAccessUnsafe, enum_value);
    }
  }
  //hasAttrs
  arboretum_create_edge(obj, context_.data_model_.method_hasAttrs, context_.data_model_.arboretum_node_for(D->hasAttrs()));
  //getAttrs
  // const AttrVec &
  //attrs
  // attr_range
  //getMaxAlignment
  // unsigned int
  //isInvalidDecl
  arboretum_create_edge(obj, context_.data_model_.method_isInvalidDecl, context_.data_model_.arboretum_node_for(D->isInvalidDecl()));
  //isImplicit
  arboretum_create_edge(obj, context_.data_model_.method_isImplicit, context_.data_model_.arboretum_node_for(D->isImplicit()));
  //isReferenced
  arboretum_create_edge(obj, context_.data_model_.method_isReferenced, context_.data_model_.arboretum_node_for(D->isReferenced()));
  //isThisDeclarationReferenced
  arboretum_create_edge(obj, context_.data_model_.method_isThisDeclarationReferenced, context_.data_model_.arboretum_node_for(D->isThisDeclarationReferenced()));
  //isTopLevelDeclInObjCContainer
  arboretum_create_edge(obj, context_.data_model_.method_isTopLevelDeclInObjCContainer, context_.data_model_.arboretum_node_for(D->isTopLevelDeclInObjCContainer()));
  //isModulePrivate
  arboretum_create_edge(obj, context_.data_model_.method_isModulePrivate, context_.data_model_.arboretum_node_for(D->isModulePrivate()));
  //isInExportDeclContext
  arboretum_create_edge(obj, context_.data_model_.method_isInExportDeclContext, context_.data_model_.arboretum_node_for(D->isInExportDeclContext()));
  //isInvisibleOutsideTheOwningModule
  arboretum_create_edge(obj, context_.data_model_.method_isInvisibleOutsideTheOwningModule, context_.data_model_.arboretum_node_for(D->isInvisibleOutsideTheOwningModule()));
  //isInAnotherModuleUnit
  arboretum_create_edge(obj, context_.data_model_.method_isInAnotherModuleUnit, context_.data_model_.arboretum_node_for(D->isInAnotherModuleUnit()));
  //isDiscardedInGlobalModuleFragment
  arboretum_create_edge(obj, context_.data_model_.method_isDiscardedInGlobalModuleFragment, context_.data_model_.arboretum_node_for(D->isDiscardedInGlobalModuleFragment()));
  //shouldSkipCheckingODR
  arboretum_create_edge(obj, context_.data_model_.method_shouldSkipCheckingODR, context_.data_model_.arboretum_node_for(D->shouldSkipCheckingODR()));
  //hasDefiningAttr
  arboretum_create_edge(obj, context_.data_model_.method_hasDefiningAttr, context_.data_model_.arboretum_node_for(D->hasDefiningAttr()));
  //getDefiningAttr
  {
    const Id* other = context_.resolve(D->getDefiningAttr());
    arboretum_create_edge(obj, context_.data_model_.method_getDefiningAttr, other);
  }
  //getVersionIntroduced
  // VersionTuple
  //isWeakImported
  arboretum_create_edge(obj, context_.data_model_.method_isWeakImported, context_.data_model_.arboretum_node_for(D->isWeakImported()));
  //isFromASTFile
  arboretum_create_edge(obj, context_.data_model_.method_isFromASTFile, context_.data_model_.arboretum_node_for(D->isFromASTFile()));
  //getGlobalID
  // unsigned int
  //getOwningModuleID
  // unsigned int
  //getImportedOwningModule
  //getLocalOwningModule
  //hasOwningModule
  arboretum_create_edge(obj, context_.data_model_.method_hasOwningModule, context_.data_model_.arboretum_node_for(D->hasOwningModule()));
  //getOwningModule
  //isUnconditionallyVisible
  arboretum_create_edge(obj, context_.data_model_.method_isUnconditionallyVisible, context_.data_model_.arboretum_node_for(D->isUnconditionallyVisible()));
  //isReachable
  arboretum_create_edge(obj, context_.data_model_.method_isReachable, context_.data_model_.arboretum_node_for(D->isReachable()));
  //getModuleOwnershipKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getModuleOwnershipKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getModuleOwnershipKind, enum_value);
    }
  }
  //getIdentifierNamespace
  // unsigned int
  //hasTagIdentifierNamespace
  arboretum_create_edge(obj, context_.data_model_.method_hasTagIdentifierNamespace, context_.data_model_.arboretum_node_for(D->hasTagIdentifierNamespace()));
  //getLexicalDeclContext
  //isOutOfLine
  arboretum_create_edge(obj, context_.data_model_.method_isOutOfLine, context_.data_model_.arboretum_node_for(D->isOutOfLine()));
  //isTemplated
  arboretum_create_edge(obj, context_.data_model_.method_isTemplated, context_.data_model_.arboretum_node_for(D->isTemplated()));
  //getTemplateDepth
  // unsigned int
  //isDefinedOutsideFunctionOrMethod
  arboretum_create_edge(obj, context_.data_model_.method_isDefinedOutsideFunctionOrMethod, context_.data_model_.arboretum_node_for(D->isDefinedOutsideFunctionOrMethod()));
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_7, other);
  }
  //isCanonicalDecl
  arboretum_create_edge(obj, context_.data_model_.method_isCanonicalDecl, context_.data_model_.arboretum_node_for(D->isCanonicalDecl()));
  //redecls
  // redecl_range
  //getPreviousDecl
  {
    const Id* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getPreviousDecl_2, other);
  }
  //isFirstDecl
  arboretum_create_edge(obj, context_.data_model_.method_isFirstDecl, context_.data_model_.arboretum_node_for(D->isFirstDecl()));
  //getMostRecentDecl
  {
    const Id* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getMostRecentDecl_3, other);
  }
  //getBody
  {
    const Id* other = context_.resolve(D->getBody());
    arboretum_create_edge(obj, context_.data_model_.method_getBody_2, other);
  }
  //hasBody
  arboretum_create_edge(obj, context_.data_model_.method_hasBody, context_.data_model_.arboretum_node_for(D->hasBody()));
  //getBodyRBrace
  {
    const Id* other = context_.source_model_.resolve(D->getBodyRBrace());
    arboretum_create_edge(obj, context_.data_model_.method_getBodyRBrace, other);
  }
  //isTemplateParameter
  arboretum_create_edge(obj, context_.data_model_.method_isTemplateParameter, context_.data_model_.arboretum_node_for(D->isTemplateParameter()));
  //isTemplateParameterPack
  arboretum_create_edge(obj, context_.data_model_.method_isTemplateParameterPack, context_.data_model_.arboretum_node_for(D->isTemplateParameterPack()));
  //isParameterPack
  arboretum_create_edge(obj, context_.data_model_.method_isParameterPack_1, context_.data_model_.arboretum_node_for(D->isParameterPack()));
  //isTemplateDecl
  arboretum_create_edge(obj, context_.data_model_.method_isTemplateDecl, context_.data_model_.arboretum_node_for(D->isTemplateDecl()));
  //isFunctionOrFunctionTemplate
  arboretum_create_edge(obj, context_.data_model_.method_isFunctionOrFunctionTemplate, context_.data_model_.arboretum_node_for(D->isFunctionOrFunctionTemplate()));
  //getDescribedTemplate
  {
    const Id* other = context_.resolve(D->getDescribedTemplate());
    arboretum_create_edge(obj, context_.data_model_.method_getDescribedTemplate, other);
  }
  //getDescribedTemplateParams
  //getAsFunction
  {
    const Id* other = context_.resolve(D->getAsFunction());
    arboretum_create_edge(obj, context_.data_model_.method_getAsFunction, other);
  }
  //isLocalExternDecl
  arboretum_create_edge(obj, context_.data_model_.method_isLocalExternDecl, context_.data_model_.arboretum_node_for(D->isLocalExternDecl()));
  //getFriendObjectKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getFriendObjectKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getFriendObjectKind, enum_value);
    }
  }
  //getID
  // int64_t
  //isFunctionPointerType
  arboretum_create_edge(obj, context_.data_model_.method_isFunctionPointerType, context_.data_model_.arboretum_node_for(D->isFunctionPointerType()));
  return true;
}

bool ArboretumASTVisitor::VisitDeclaratorDecl(clang::DeclaratorDecl* D) {
  const Id* obj = context_.resolve(D);
  //getTypeSourceInfo
  //getInnerLocStart
  {
    const Id* other = context_.source_model_.resolve(D->getInnerLocStart());
    arboretum_create_edge(obj, context_.data_model_.method_getInnerLocStart, other);
  }
  //getOuterLocStart
  {
    const Id* other = context_.source_model_.resolve(D->getOuterLocStart());
    arboretum_create_edge(obj, context_.data_model_.method_getOuterLocStart, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_6, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_1, other);
  }
  //getQualifier
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getTrailingRequiresClause
  {
    const Id* other = context_.resolve(D->getTrailingRequiresClause());
    arboretum_create_edge(obj, context_.data_model_.method_getTrailingRequiresClause, other);
  }
  //getNumTemplateParameterLists
  // unsigned int
  //getTypeSpecStartLoc
  {
    const Id* other = context_.source_model_.resolve(D->getTypeSpecStartLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getTypeSpecStartLoc, other);
  }
  //getTypeSpecEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getTypeSpecEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getTypeSpecEndLoc, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitDecompositionDecl(clang::DecompositionDecl* D) {
  const Id* obj = context_.resolve(D);
  //bindings
  // ArrayRef<BindingDecl *>
  return true;
}

bool ArboretumASTVisitor::VisitEmptyDecl(clang::EmptyDecl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitEnumConstantDecl(clang::EnumConstantDecl* D) {
  const Id* obj = context_.resolve(D);
  //getInitExpr
  {
    const Id* other = context_.resolve(D->getInitExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getInitExpr, other);
  }
  //getInitVal
  // llvm::APSInt
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_7, other);
  }
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_8, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitEnumDecl(clang::EnumDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Id* obj = context_.resolve(D);
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_9, other);
  }
  //getPreviousDecl
  {
    const Id* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getPreviousDecl_3, other);
  }
  //getMostRecentDecl
  {
    const Id* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getMostRecentDecl_4, other);
  }
  //getDefinition
  {
    const Id* other = context_.resolve(D->getDefinition());
    arboretum_create_edge(obj, context_.data_model_.method_getDefinition_1, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_8, other);
  }
  //enumerators
  // enumerator_range
  //getPromotionType
  {
    const Id* other = context_.resolve(D->getPromotionType());
    arboretum_create_edge(obj, context_.data_model_.method_getPromotionType, other);
  }
  //getIntegerType
  {
    const Id* other = context_.resolve(D->getIntegerType());
    arboretum_create_edge(obj, context_.data_model_.method_getIntegerType, other);
  }
  //getIntegerTypeSourceInfo
  //getIntegerTypeRange
  {
    const Id* other = context_.source_model_.resolve(D->getIntegerTypeRange());
    arboretum_create_edge(obj, context_.data_model_.method_getIntegerTypeRange, other);
  }
  //getNumPositiveBits
  // unsigned int
  //getNumNegativeBits
  // unsigned int
  //isScoped
  arboretum_create_edge(obj, context_.data_model_.method_isScoped, context_.data_model_.arboretum_node_for(D->isScoped()));
  //isScopedUsingClassTag
  arboretum_create_edge(obj, context_.data_model_.method_isScopedUsingClassTag, context_.data_model_.arboretum_node_for(D->isScopedUsingClassTag()));
  //isFixed
  arboretum_create_edge(obj, context_.data_model_.method_isFixed, context_.data_model_.arboretum_node_for(D->isFixed()));
  //isComplete
  arboretum_create_edge(obj, context_.data_model_.method_isComplete, context_.data_model_.arboretum_node_for(D->isComplete()));
  //isClosed
  arboretum_create_edge(obj, context_.data_model_.method_isClosed, context_.data_model_.arboretum_node_for(D->isClosed()));
  //isClosedFlag
  arboretum_create_edge(obj, context_.data_model_.method_isClosedFlag, context_.data_model_.arboretum_node_for(D->isClosedFlag()));
  //isClosedNonFlag
  arboretum_create_edge(obj, context_.data_model_.method_isClosedNonFlag, context_.data_model_.arboretum_node_for(D->isClosedNonFlag()));
  //getTemplateInstantiationPattern
  {
    const Id* other = context_.resolve(D->getTemplateInstantiationPattern());
    arboretum_create_edge(obj, context_.data_model_.method_getTemplateInstantiationPattern_1, other);
  }
  //getInstantiatedFromMemberEnum
  {
    const Id* other = context_.resolve(D->getInstantiatedFromMemberEnum());
    arboretum_create_edge(obj, context_.data_model_.method_getInstantiatedFromMemberEnum, other);
  }
  //getTemplateSpecializationKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getTemplateSpecializationKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getTemplateSpecializationKind_1, enum_value);
    }
  }
  //getMemberSpecializationInfo
  return true;
}

bool ArboretumASTVisitor::VisitExportDecl(clang::ExportDecl* D) {
  const Id* obj = context_.resolve(D);
  //getExportLoc
  {
    const Id* other = context_.source_model_.resolve(D->getExportLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getExportLoc, other);
  }
  //getRBraceLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRBraceLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRBraceLoc, other);
  }
  //hasBraces
  arboretum_create_edge(obj, context_.data_model_.method_hasBraces, context_.data_model_.arboretum_node_for(D->hasBraces()));
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_1, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_9, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitExternCContextDecl(clang::ExternCContextDecl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitFieldDecl(clang::FieldDecl* D) {
  const Id* obj = context_.resolve(D);
  //getFieldIndex
  // unsigned int
  //isMutable
  arboretum_create_edge(obj, context_.data_model_.method_isMutable, context_.data_model_.arboretum_node_for(D->isMutable()));
  //isBitField
  arboretum_create_edge(obj, context_.data_model_.method_isBitField, context_.data_model_.arboretum_node_for(D->isBitField()));
  //isUnnamedBitfield
  arboretum_create_edge(obj, context_.data_model_.method_isUnnamedBitfield, context_.data_model_.arboretum_node_for(D->isUnnamedBitfield()));
  //isAnonymousStructOrUnion
  arboretum_create_edge(obj, context_.data_model_.method_isAnonymousStructOrUnion, context_.data_model_.arboretum_node_for(D->isAnonymousStructOrUnion()));
  //getBitWidth
  {
    const Id* other = context_.resolve(D->getBitWidth());
    arboretum_create_edge(obj, context_.data_model_.method_getBitWidth, other);
  }
  //isPotentiallyOverlapping
  arboretum_create_edge(obj, context_.data_model_.method_isPotentiallyOverlapping, context_.data_model_.arboretum_node_for(D->isPotentiallyOverlapping()));
  //getInClassInitStyle
  {
    const Id* enum_value = context_.data_model_.resolve(D->getInClassInitStyle());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getInClassInitStyle, enum_value);
    }
  }
  //hasInClassInitializer
  arboretum_create_edge(obj, context_.data_model_.method_hasInClassInitializer_1, context_.data_model_.arboretum_node_for(D->hasInClassInitializer()));
  //hasNonNullInClassInitializer
  arboretum_create_edge(obj, context_.data_model_.method_hasNonNullInClassInitializer, context_.data_model_.arboretum_node_for(D->hasNonNullInClassInitializer()));
  //getInClassInitializer
  {
    const Id* other = context_.resolve(D->getInClassInitializer());
    arboretum_create_edge(obj, context_.data_model_.method_getInClassInitializer, other);
  }
  //hasCapturedVLAType
  arboretum_create_edge(obj, context_.data_model_.method_hasCapturedVLAType, context_.data_model_.arboretum_node_for(D->hasCapturedVLAType()));
  //getCapturedVLAType
  {
    const Id* other = context_.resolve(D->getCapturedVLAType());
    arboretum_create_edge(obj, context_.data_model_.method_getCapturedVLAType, other);
  }
  //getParent
  {
    const Id* other = context_.resolve(D->getParent());
    arboretum_create_edge(obj, context_.data_model_.method_getParent_2, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_10, other);
  }
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_10, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitFileScopeAsmDecl(clang::FileScopeAsmDecl* D) {
  const Id* obj = context_.resolve(D);
  //getAsmLoc
  {
    const Id* other = context_.source_model_.resolve(D->getAsmLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getAsmLoc, other);
  }
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_11, other);
  }
  //getAsmString
  {
    const Id* other = context_.resolve(D->getAsmString());
    arboretum_create_edge(obj, context_.data_model_.method_getAsmString, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitFriendDecl(clang::FriendDecl* D) {
  const Id* obj = context_.resolve(D);
  //getFriendType
  //getFriendTypeNumTemplateParameterLists
  // unsigned int
  //getFriendDecl
  {
    const Id* other = context_.resolve(D->getFriendDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getFriendDecl, other);
  }
  //getFriendLoc
  {
    const Id* other = context_.source_model_.resolve(D->getFriendLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getFriendLoc, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_12, other);
  }
  //isUnsupportedFriend
  arboretum_create_edge(obj, context_.data_model_.method_isUnsupportedFriend, context_.data_model_.arboretum_node_for(D->isUnsupportedFriend()));
  return true;
}

bool ArboretumASTVisitor::VisitFriendTemplateDecl(clang::FriendTemplateDecl* D) {
  const Id* obj = context_.resolve(D);
  //getFriendType
  //getFriendDecl
  {
    const Id* other = context_.resolve(D->getFriendDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getFriendDecl_1, other);
  }
  //getFriendLoc
  {
    const Id* other = context_.source_model_.resolve(D->getFriendLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getFriendLoc_1, other);
  }
  //getNumTemplateParameters
  // unsigned int
  return true;
}

bool ArboretumASTVisitor::VisitFunctionDecl(clang::FunctionDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Id* obj = context_.resolve(D);
  //getNameInfo
  // DeclarationNameInfo
  //getEllipsisLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEllipsisLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEllipsisLoc_1, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_13, other);
  }
  //hasBody
  arboretum_create_edge(obj, context_.data_model_.method_hasBody_1, context_.data_model_.arboretum_node_for(D->hasBody()));
  //hasTrivialBody
  arboretum_create_edge(obj, context_.data_model_.method_hasTrivialBody, context_.data_model_.arboretum_node_for(D->hasTrivialBody()));
  //isDefined
  arboretum_create_edge(obj, context_.data_model_.method_isDefined, context_.data_model_.arboretum_node_for(D->isDefined()));
  //getDefinition
  {
    const Id* other = context_.resolve(D->getDefinition());
    arboretum_create_edge(obj, context_.data_model_.method_getDefinition_2, other);
  }
  //getBody
  {
    const Id* other = context_.resolve(D->getBody());
    arboretum_create_edge(obj, context_.data_model_.method_getBody_3, other);
  }
  //isThisDeclarationADefinition
  arboretum_create_edge(obj, context_.data_model_.method_isThisDeclarationADefinition_1, context_.data_model_.arboretum_node_for(D->isThisDeclarationADefinition()));
  //isThisDeclarationInstantiatedFromAFriendDefinition
  arboretum_create_edge(obj, context_.data_model_.method_isThisDeclarationInstantiatedFromAFriendDefinition, context_.data_model_.arboretum_node_for(D->isThisDeclarationInstantiatedFromAFriendDefinition()));
  //doesThisDeclarationHaveABody
  arboretum_create_edge(obj, context_.data_model_.method_doesThisDeclarationHaveABody, context_.data_model_.arboretum_node_for(D->doesThisDeclarationHaveABody()));
  //getDefaultedFunctionInfo
  //isVariadic
  arboretum_create_edge(obj, context_.data_model_.method_isVariadic_2, context_.data_model_.arboretum_node_for(D->isVariadic()));
  //isVirtualAsWritten
  arboretum_create_edge(obj, context_.data_model_.method_isVirtualAsWritten, context_.data_model_.arboretum_node_for(D->isVirtualAsWritten()));
  //isPureVirtual
  arboretum_create_edge(obj, context_.data_model_.method_isPureVirtual, context_.data_model_.arboretum_node_for(D->isPureVirtual()));
  //isLateTemplateParsed
  arboretum_create_edge(obj, context_.data_model_.method_isLateTemplateParsed, context_.data_model_.arboretum_node_for(D->isLateTemplateParsed()));
  //isTrivial
  arboretum_create_edge(obj, context_.data_model_.method_isTrivial_1, context_.data_model_.arboretum_node_for(D->isTrivial()));
  //isTrivialForCall
  arboretum_create_edge(obj, context_.data_model_.method_isTrivialForCall, context_.data_model_.arboretum_node_for(D->isTrivialForCall()));
  //isDefaulted
  arboretum_create_edge(obj, context_.data_model_.method_isDefaulted, context_.data_model_.arboretum_node_for(D->isDefaulted()));
  //isExplicitlyDefaulted
  arboretum_create_edge(obj, context_.data_model_.method_isExplicitlyDefaulted, context_.data_model_.arboretum_node_for(D->isExplicitlyDefaulted()));
  //getDefaultLoc
  {
    const Id* other = context_.source_model_.resolve(D->getDefaultLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getDefaultLoc, other);
  }
  //isUserProvided
  arboretum_create_edge(obj, context_.data_model_.method_isUserProvided, context_.data_model_.arboretum_node_for(D->isUserProvided()));
  //isIneligibleOrNotSelected
  arboretum_create_edge(obj, context_.data_model_.method_isIneligibleOrNotSelected, context_.data_model_.arboretum_node_for(D->isIneligibleOrNotSelected()));
  //hasImplicitReturnZero
  arboretum_create_edge(obj, context_.data_model_.method_hasImplicitReturnZero, context_.data_model_.arboretum_node_for(D->hasImplicitReturnZero()));
  //hasPrototype
  arboretum_create_edge(obj, context_.data_model_.method_hasPrototype, context_.data_model_.arboretum_node_for(D->hasPrototype()));
  //hasWrittenPrototype
  arboretum_create_edge(obj, context_.data_model_.method_hasWrittenPrototype, context_.data_model_.arboretum_node_for(D->hasWrittenPrototype()));
  //hasInheritedPrototype
  arboretum_create_edge(obj, context_.data_model_.method_hasInheritedPrototype, context_.data_model_.arboretum_node_for(D->hasInheritedPrototype()));
  //isConstexpr
  arboretum_create_edge(obj, context_.data_model_.method_isConstexpr, context_.data_model_.arboretum_node_for(D->isConstexpr()));
  //getConstexprKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getConstexprKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getConstexprKind, enum_value);
    }
  }
  //isConstexprSpecified
  arboretum_create_edge(obj, context_.data_model_.method_isConstexprSpecified, context_.data_model_.arboretum_node_for(D->isConstexprSpecified()));
  //isConsteval
  arboretum_create_edge(obj, context_.data_model_.method_isConsteval, context_.data_model_.arboretum_node_for(D->isConsteval()));
  //BodyContainsImmediateEscalatingExpressions
  arboretum_create_edge(obj, context_.data_model_.method_BodyContainsImmediateEscalatingExpressions, context_.data_model_.arboretum_node_for(D->BodyContainsImmediateEscalatingExpressions()));
  //isImmediateEscalating
  arboretum_create_edge(obj, context_.data_model_.method_isImmediateEscalating, context_.data_model_.arboretum_node_for(D->isImmediateEscalating()));
  //isImmediateFunction
  arboretum_create_edge(obj, context_.data_model_.method_isImmediateFunction, context_.data_model_.arboretum_node_for(D->isImmediateFunction()));
  //instantiationIsPending
  arboretum_create_edge(obj, context_.data_model_.method_instantiationIsPending, context_.data_model_.arboretum_node_for(D->instantiationIsPending()));
  //usesSEHTry
  arboretum_create_edge(obj, context_.data_model_.method_usesSEHTry, context_.data_model_.arboretum_node_for(D->usesSEHTry()));
  //isDeleted
  arboretum_create_edge(obj, context_.data_model_.method_isDeleted, context_.data_model_.arboretum_node_for(D->isDeleted()));
  //isDeletedAsWritten
  arboretum_create_edge(obj, context_.data_model_.method_isDeletedAsWritten, context_.data_model_.arboretum_node_for(D->isDeletedAsWritten()));
  //isMain
  arboretum_create_edge(obj, context_.data_model_.method_isMain, context_.data_model_.arboretum_node_for(D->isMain()));
  //isMSVCRTEntryPoint
  arboretum_create_edge(obj, context_.data_model_.method_isMSVCRTEntryPoint, context_.data_model_.arboretum_node_for(D->isMSVCRTEntryPoint()));
  //isReservedGlobalPlacementOperator
  arboretum_create_edge(obj, context_.data_model_.method_isReservedGlobalPlacementOperator, context_.data_model_.arboretum_node_for(D->isReservedGlobalPlacementOperator()));
  //isInlineBuiltinDeclaration
  arboretum_create_edge(obj, context_.data_model_.method_isInlineBuiltinDeclaration, context_.data_model_.arboretum_node_for(D->isInlineBuiltinDeclaration()));
  //isDestroyingOperatorDelete
  arboretum_create_edge(obj, context_.data_model_.method_isDestroyingOperatorDelete, context_.data_model_.arboretum_node_for(D->isDestroyingOperatorDelete()));
  //getLanguageLinkage
  {
    const Id* enum_value = context_.data_model_.resolve(D->getLanguageLinkage());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getLanguageLinkage, enum_value);
    }
  }
  //isExternC
  arboretum_create_edge(obj, context_.data_model_.method_isExternC, context_.data_model_.arboretum_node_for(D->isExternC()));
  //isInExternCContext
  arboretum_create_edge(obj, context_.data_model_.method_isInExternCContext, context_.data_model_.arboretum_node_for(D->isInExternCContext()));
  //isInExternCXXContext
  arboretum_create_edge(obj, context_.data_model_.method_isInExternCXXContext, context_.data_model_.arboretum_node_for(D->isInExternCXXContext()));
  //isGlobal
  arboretum_create_edge(obj, context_.data_model_.method_isGlobal, context_.data_model_.arboretum_node_for(D->isGlobal()));
  //isNoReturn
  arboretum_create_edge(obj, context_.data_model_.method_isNoReturn, context_.data_model_.arboretum_node_for(D->isNoReturn()));
  //hasSkippedBody
  arboretum_create_edge(obj, context_.data_model_.method_hasSkippedBody, context_.data_model_.arboretum_node_for(D->hasSkippedBody()));
  //willHaveBody
  arboretum_create_edge(obj, context_.data_model_.method_willHaveBody, context_.data_model_.arboretum_node_for(D->willHaveBody()));
  //isMultiVersion
  arboretum_create_edge(obj, context_.data_model_.method_isMultiVersion, context_.data_model_.arboretum_node_for(D->isMultiVersion()));
  //FriendConstraintRefersToEnclosingTemplate
  arboretum_create_edge(obj, context_.data_model_.method_FriendConstraintRefersToEnclosingTemplate, context_.data_model_.arboretum_node_for(D->FriendConstraintRefersToEnclosingTemplate()));
  //isMemberLikeConstrainedFriend
  arboretum_create_edge(obj, context_.data_model_.method_isMemberLikeConstrainedFriend, context_.data_model_.arboretum_node_for(D->isMemberLikeConstrainedFriend()));
  //getMultiVersionKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getMultiVersionKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getMultiVersionKind, enum_value);
    }
  }
  //isCPUDispatchMultiVersion
  arboretum_create_edge(obj, context_.data_model_.method_isCPUDispatchMultiVersion, context_.data_model_.arboretum_node_for(D->isCPUDispatchMultiVersion()));
  //isCPUSpecificMultiVersion
  arboretum_create_edge(obj, context_.data_model_.method_isCPUSpecificMultiVersion, context_.data_model_.arboretum_node_for(D->isCPUSpecificMultiVersion()));
  //isTargetMultiVersion
  arboretum_create_edge(obj, context_.data_model_.method_isTargetMultiVersion, context_.data_model_.arboretum_node_for(D->isTargetMultiVersion()));
  //isTargetClonesMultiVersion
  arboretum_create_edge(obj, context_.data_model_.method_isTargetClonesMultiVersion, context_.data_model_.arboretum_node_for(D->isTargetClonesMultiVersion()));
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_11, other);
  }
  //parameters
  // ArrayRef<ParmVarDecl *>
  //param_empty
  arboretum_create_edge(obj, context_.data_model_.method_param_empty_1, context_.data_model_.arboretum_node_for(D->param_empty()));
  //param_size
  // size_t
  //getNumParams
  // unsigned int
  //getMinRequiredArguments
  // unsigned int
  //getMinRequiredExplicitArguments
  // unsigned int
  //hasCXXExplicitFunctionObjectParameter
  arboretum_create_edge(obj, context_.data_model_.method_hasCXXExplicitFunctionObjectParameter, context_.data_model_.arboretum_node_for(D->hasCXXExplicitFunctionObjectParameter()));
  //getNumNonObjectParams
  // unsigned int
  //hasOneParamOrDefaultArgs
  arboretum_create_edge(obj, context_.data_model_.method_hasOneParamOrDefaultArgs, context_.data_model_.arboretum_node_for(D->hasOneParamOrDefaultArgs()));
  //getFunctionTypeLoc
  // FunctionTypeLoc
  //getReturnType
  {
    const Id* other = context_.resolve(D->getReturnType());
    arboretum_create_edge(obj, context_.data_model_.method_getReturnType_1, other);
  }
  //getReturnTypeSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getReturnTypeSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getReturnTypeSourceRange, other);
  }
  //getParametersSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getParametersSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getParametersSourceRange, other);
  }
  //getDeclaredReturnType
  {
    const Id* other = context_.resolve(D->getDeclaredReturnType());
    arboretum_create_edge(obj, context_.data_model_.method_getDeclaredReturnType, other);
  }
  //getExceptionSpecType
  {
    const Id* enum_value = context_.data_model_.resolve(D->getExceptionSpecType());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getExceptionSpecType_1, enum_value);
    }
  }
  //getExceptionSpecSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getExceptionSpecSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getExceptionSpecSourceRange, other);
  }
  //getCallResultType
  {
    const Id* other = context_.resolve(D->getCallResultType());
    arboretum_create_edge(obj, context_.data_model_.method_getCallResultType, other);
  }
  //getStorageClass
  {
    const Id* enum_value = context_.data_model_.resolve(D->getStorageClass());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getStorageClass, enum_value);
    }
  }
  //isInlineSpecified
  arboretum_create_edge(obj, context_.data_model_.method_isInlineSpecified, context_.data_model_.arboretum_node_for(D->isInlineSpecified()));
  //UsesFPIntrin
  arboretum_create_edge(obj, context_.data_model_.method_UsesFPIntrin, context_.data_model_.arboretum_node_for(D->UsesFPIntrin()));
  //isInlined
  arboretum_create_edge(obj, context_.data_model_.method_isInlined, context_.data_model_.arboretum_node_for(D->isInlined()));
  //isInlineDefinitionExternallyVisible
  arboretum_create_edge(obj, context_.data_model_.method_isInlineDefinitionExternallyVisible, context_.data_model_.arboretum_node_for(D->isInlineDefinitionExternallyVisible()));
  //isMSExternInline
  arboretum_create_edge(obj, context_.data_model_.method_isMSExternInline, context_.data_model_.arboretum_node_for(D->isMSExternInline()));
  //doesDeclarationForceExternallyVisibleDefinition
  arboretum_create_edge(obj, context_.data_model_.method_doesDeclarationForceExternallyVisibleDefinition, context_.data_model_.arboretum_node_for(D->doesDeclarationForceExternallyVisibleDefinition()));
  //isStatic
  arboretum_create_edge(obj, context_.data_model_.method_isStatic_1, context_.data_model_.arboretum_node_for(D->isStatic()));
  //isOverloadedOperator
  arboretum_create_edge(obj, context_.data_model_.method_isOverloadedOperator, context_.data_model_.arboretum_node_for(D->isOverloadedOperator()));
  //getOverloadedOperator
  {
    const Id* enum_value = context_.data_model_.resolve(D->getOverloadedOperator());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getOverloadedOperator, enum_value);
    }
  }
  //getLiteralIdentifier
  //getInstantiatedFromMemberFunction
  {
    const Id* other = context_.resolve(D->getInstantiatedFromMemberFunction());
    arboretum_create_edge(obj, context_.data_model_.method_getInstantiatedFromMemberFunction, other);
  }
  //getTemplatedKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getTemplatedKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getTemplatedKind, enum_value);
    }
  }
  //getMemberSpecializationInfo
  //getInstantiatedFromDecl
  {
    const Id* other = context_.resolve(D->getInstantiatedFromDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getInstantiatedFromDecl, other);
  }
  //getDescribedFunctionTemplate
  {
    const Id* other = context_.resolve(D->getDescribedFunctionTemplate());
    arboretum_create_edge(obj, context_.data_model_.method_getDescribedFunctionTemplate, other);
  }
  //isFunctionTemplateSpecialization
  arboretum_create_edge(obj, context_.data_model_.method_isFunctionTemplateSpecialization, context_.data_model_.arboretum_node_for(D->isFunctionTemplateSpecialization()));
  //getTemplateSpecializationInfo
  //isImplicitlyInstantiable
  arboretum_create_edge(obj, context_.data_model_.method_isImplicitlyInstantiable, context_.data_model_.arboretum_node_for(D->isImplicitlyInstantiable()));
  //isTemplateInstantiation
  arboretum_create_edge(obj, context_.data_model_.method_isTemplateInstantiation, context_.data_model_.arboretum_node_for(D->isTemplateInstantiation()));
  //getPrimaryTemplate
  {
    const Id* other = context_.resolve(D->getPrimaryTemplate());
    arboretum_create_edge(obj, context_.data_model_.method_getPrimaryTemplate, other);
  }
  //getTemplateSpecializationArgs
  //getTemplateSpecializationArgsAsWritten
  //getDependentSpecializationInfo
  //getTemplateSpecializationKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getTemplateSpecializationKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getTemplateSpecializationKind_2, enum_value);
    }
  }
  //getTemplateSpecializationKindForInstantiation
  {
    const Id* enum_value = context_.data_model_.resolve(D->getTemplateSpecializationKindForInstantiation());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getTemplateSpecializationKindForInstantiation, enum_value);
    }
  }
  //getPointOfInstantiation
  {
    const Id* other = context_.source_model_.resolve(D->getPointOfInstantiation());
    arboretum_create_edge(obj, context_.data_model_.method_getPointOfInstantiation_1, other);
  }
  //isOutOfLine
  arboretum_create_edge(obj, context_.data_model_.method_isOutOfLine_1, context_.data_model_.arboretum_node_for(D->isOutOfLine()));
  //getMemoryFunctionKind
  // unsigned int
  //getODRHash
  // unsigned int
  return true;
}

bool ArboretumASTVisitor::VisitFunctionTemplateDecl(clang::FunctionTemplateDecl* D) {
  const Id* obj = context_.resolve(D);
  //getTemplatedDecl
  {
    const Id* other = context_.resolve(D->getTemplatedDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getTemplatedDecl_1, other);
  }
  //isThisDeclarationADefinition
  arboretum_create_edge(obj, context_.data_model_.method_isThisDeclarationADefinition_2, context_.data_model_.arboretum_node_for(D->isThisDeclarationADefinition()));
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_12, other);
  }
  //getPreviousDecl
  {
    const Id* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getPreviousDecl_4, other);
  }
  //getMostRecentDecl
  {
    const Id* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getMostRecentDecl_5, other);
  }
  //getInstantiatedFromMemberTemplate
  {
    const Id* other = context_.resolve(D->getInstantiatedFromMemberTemplate());
    arboretum_create_edge(obj, context_.data_model_.method_getInstantiatedFromMemberTemplate_2, other);
  }
  //specializations
  // spec_range
  //isAbbreviated
  arboretum_create_edge(obj, context_.data_model_.method_isAbbreviated, context_.data_model_.arboretum_node_for(D->isAbbreviated()));
  return true;
}

bool ArboretumASTVisitor::VisitHLSLBufferDecl(clang::HLSLBufferDecl* D) {
  const Id* obj = context_.resolve(D);
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_14, other);
  }
  //getLocStart
  {
    const Id* other = context_.source_model_.resolve(D->getLocStart());
    arboretum_create_edge(obj, context_.data_model_.method_getLocStart, other);
  }
  //getLBraceLoc
  {
    const Id* other = context_.source_model_.resolve(D->getLBraceLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getLBraceLoc, other);
  }
  //getRBraceLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRBraceLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRBraceLoc_1, other);
  }
  //isCBuffer
  arboretum_create_edge(obj, context_.data_model_.method_isCBuffer, context_.data_model_.arboretum_node_for(D->isCBuffer()));
  return true;
}

bool ArboretumASTVisitor::VisitImplicitConceptSpecializationDecl(clang::ImplicitConceptSpecializationDecl* D) {
  const Id* obj = context_.resolve(D);
  //getTemplateArguments
  // ArrayRef<TemplateArgument>
  return true;
}

bool ArboretumASTVisitor::VisitImplicitParamDecl(clang::ImplicitParamDecl* D) {
  const Id* obj = context_.resolve(D);
  //getParameterKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getParameterKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getParameterKind, enum_value);
    }
  }
  return true;
}

bool ArboretumASTVisitor::VisitImportDecl(clang::ImportDecl* D) {
  const Id* obj = context_.resolve(D);
  //getImportedModule
  //getIdentifierLocs
  // ArrayRef<SourceLocation>
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_15, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitIndirectFieldDecl(clang::IndirectFieldDecl* D) {
  const Id* obj = context_.resolve(D);
  //chain
  // ArrayRef<NamedDecl *>
  //getChainingSize
  // unsigned int
  //getAnonField
  {
    const Id* other = context_.resolve(D->getAnonField());
    arboretum_create_edge(obj, context_.data_model_.method_getAnonField, other);
  }
  //getVarDecl
  {
    const Id* other = context_.resolve(D->getVarDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getVarDecl, other);
  }
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_13, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitLabelDecl(clang::LabelDecl* D) {
  const Id* obj = context_.resolve(D);
  //getStmt
  {
    const Id* other = context_.resolve(D->getStmt());
    arboretum_create_edge(obj, context_.data_model_.method_getStmt, other);
  }
  //isGnuLocal
  arboretum_create_edge(obj, context_.data_model_.method_isGnuLocal, context_.data_model_.arboretum_node_for(D->isGnuLocal()));
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_16, other);
  }
  //isMSAsmLabel
  arboretum_create_edge(obj, context_.data_model_.method_isMSAsmLabel, context_.data_model_.arboretum_node_for(D->isMSAsmLabel()));
  //isResolvedMSAsmLabel
  arboretum_create_edge(obj, context_.data_model_.method_isResolvedMSAsmLabel, context_.data_model_.arboretum_node_for(D->isResolvedMSAsmLabel()));
  //getMSAsmLabel
  // StringRef
  return true;
}

bool ArboretumASTVisitor::VisitLifetimeExtendedTemporaryDecl(clang::LifetimeExtendedTemporaryDecl* D) {
  const Id* obj = context_.resolve(D);
  //getExtendingDecl
  {
    const Id* other = context_.resolve(D->getExtendingDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getExtendingDecl, other);
  }
  //getStorageDuration
  {
    const Id* enum_value = context_.data_model_.resolve(D->getStorageDuration());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getStorageDuration, enum_value);
    }
  }
  //getTemporaryExpr
  {
    const Id* other = context_.resolve(D->getTemporaryExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getTemporaryExpr, other);
  }
  //getManglingNumber
  // unsigned int
  //getValue
  //childrenExpr
  // class Stmt::const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitLinkageSpecDecl(clang::LinkageSpecDecl* D) {
  const Id* obj = context_.resolve(D);
  //getLanguage
  {
    const Id* enum_value = context_.data_model_.resolve(D->getLanguage());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getLanguage, enum_value);
    }
  }
  //hasBraces
  arboretum_create_edge(obj, context_.data_model_.method_hasBraces_1, context_.data_model_.arboretum_node_for(D->hasBraces()));
  //getExternLoc
  {
    const Id* other = context_.source_model_.resolve(D->getExternLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getExternLoc_1, other);
  }
  //getRBraceLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRBraceLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRBraceLoc_2, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_2, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_17, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitMSGuidDecl(clang::MSGuidDecl* D) {
  const Id* obj = context_.resolve(D);
  //getParts
  // Parts
  //getAsAPValue
  // APValue &
  return true;
}

bool ArboretumASTVisitor::VisitMSPropertyDecl(clang::MSPropertyDecl* D) {
  const Id* obj = context_.resolve(D);
  //hasGetter
  arboretum_create_edge(obj, context_.data_model_.method_hasGetter, context_.data_model_.arboretum_node_for(D->hasGetter()));
  //getGetterId
  //hasSetter
  arboretum_create_edge(obj, context_.data_model_.method_hasSetter, context_.data_model_.arboretum_node_for(D->hasSetter()));
  //getSetterId
  return true;
}

bool ArboretumASTVisitor::VisitNamedDecl(clang::NamedDecl* D) {
  const Id* obj = context_.resolve(D);
  //getIdentifier
  //getName
  // StringRef
  //getNameAsString
  // std::string
  //getDeclName
  // DeclarationName
  //getQualifiedNameAsString
  // std::string
  //hasLinkage
  arboretum_create_edge(obj, context_.data_model_.method_hasLinkage, context_.data_model_.arboretum_node_for(D->hasLinkage()));
  //isCXXClassMember
  arboretum_create_edge(obj, context_.data_model_.method_isCXXClassMember, context_.data_model_.arboretum_node_for(D->isCXXClassMember()));
  //isCXXInstanceMember
  arboretum_create_edge(obj, context_.data_model_.method_isCXXInstanceMember, context_.data_model_.arboretum_node_for(D->isCXXInstanceMember()));
  //getLinkageInternal
  {
    const Id* enum_value = context_.data_model_.resolve(D->getLinkageInternal());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getLinkageInternal, enum_value);
    }
  }
  //getFormalLinkage
  {
    const Id* enum_value = context_.data_model_.resolve(D->getFormalLinkage());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getFormalLinkage, enum_value);
    }
  }
  //hasExternalFormalLinkage
  arboretum_create_edge(obj, context_.data_model_.method_hasExternalFormalLinkage, context_.data_model_.arboretum_node_for(D->hasExternalFormalLinkage()));
  //isExternallyVisible
  arboretum_create_edge(obj, context_.data_model_.method_isExternallyVisible, context_.data_model_.arboretum_node_for(D->isExternallyVisible()));
  //isExternallyDeclarable
  arboretum_create_edge(obj, context_.data_model_.method_isExternallyDeclarable, context_.data_model_.arboretum_node_for(D->isExternallyDeclarable()));
  //isLinkageValid
  arboretum_create_edge(obj, context_.data_model_.method_isLinkageValid, context_.data_model_.arboretum_node_for(D->isLinkageValid()));
  //hasLinkageBeenComputed
  arboretum_create_edge(obj, context_.data_model_.method_hasLinkageBeenComputed, context_.data_model_.arboretum_node_for(D->hasLinkageBeenComputed()));
  //getUnderlyingDecl
  {
    const Id* other = context_.resolve(D->getUnderlyingDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getUnderlyingDecl, other);
  }
  //getMostRecentDecl
  {
    const Id* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getMostRecentDecl_6, other);
  }
  //getObjCFStringFormattingFamily
  {
    const Id* enum_value = context_.data_model_.resolve(D->getObjCFStringFormattingFamily());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getObjCFStringFormattingFamily, enum_value);
    }
  }
  return true;
}

bool ArboretumASTVisitor::VisitNamespaceAliasDecl(clang::NamespaceAliasDecl* D) {
  const Id* obj = context_.resolve(D);
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_14, other);
  }
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getQualifier
  //getNamespace
  {
    const Id* other = context_.resolve(D->getNamespace());
    arboretum_create_edge(obj, context_.data_model_.method_getNamespace, other);
  }
  //getAliasLoc
  {
    const Id* other = context_.source_model_.resolve(D->getAliasLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getAliasLoc, other);
  }
  //getNamespaceLoc
  {
    const Id* other = context_.source_model_.resolve(D->getNamespaceLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getNamespaceLoc, other);
  }
  //getTargetNameLoc
  {
    const Id* other = context_.source_model_.resolve(D->getTargetNameLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getTargetNameLoc, other);
  }
  //getAliasedNamespace
  {
    const Id* other = context_.resolve(D->getAliasedNamespace());
    arboretum_create_edge(obj, context_.data_model_.method_getAliasedNamespace, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_18, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitNamespaceDecl(clang::NamespaceDecl* D) {
  const Id* obj = context_.resolve(D);
  //isAnonymousNamespace
  arboretum_create_edge(obj, context_.data_model_.method_isAnonymousNamespace, context_.data_model_.arboretum_node_for(D->isAnonymousNamespace()));
  //isInline
  arboretum_create_edge(obj, context_.data_model_.method_isInline, context_.data_model_.arboretum_node_for(D->isInline()));
  //isNested
  arboretum_create_edge(obj, context_.data_model_.method_isNested, context_.data_model_.arboretum_node_for(D->isNested()));
  //getOriginalNamespace
  {
    const Id* other = context_.resolve(D->getOriginalNamespace());
    arboretum_create_edge(obj, context_.data_model_.method_getOriginalNamespace, other);
  }
  //isOriginalNamespace
  arboretum_create_edge(obj, context_.data_model_.method_isOriginalNamespace, context_.data_model_.arboretum_node_for(D->isOriginalNamespace()));
  //getAnonymousNamespace
  {
    const Id* other = context_.resolve(D->getAnonymousNamespace());
    arboretum_create_edge(obj, context_.data_model_.method_getAnonymousNamespace, other);
  }
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_15, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_19, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_2, other);
  }
  //getRBraceLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRBraceLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRBraceLoc_3, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitNonTypeTemplateParmDecl(clang::NonTypeTemplateParmDecl* D) {
  const Id* obj = context_.resolve(D);
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_20, other);
  }
  //getDefaultArgStorage
  // const DefArgStorage &
  //hasDefaultArgument
  arboretum_create_edge(obj, context_.data_model_.method_hasDefaultArgument, context_.data_model_.arboretum_node_for(D->hasDefaultArgument()));
  //getDefaultArgument
  {
    const Id* other = context_.resolve(D->getDefaultArgument());
    arboretum_create_edge(obj, context_.data_model_.method_getDefaultArgument, other);
  }
  //getDefaultArgumentLoc
  {
    const Id* other = context_.source_model_.resolve(D->getDefaultArgumentLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getDefaultArgumentLoc, other);
  }
  //defaultArgumentWasInherited
  arboretum_create_edge(obj, context_.data_model_.method_defaultArgumentWasInherited, context_.data_model_.arboretum_node_for(D->defaultArgumentWasInherited()));
  //isParameterPack
  arboretum_create_edge(obj, context_.data_model_.method_isParameterPack_2, context_.data_model_.arboretum_node_for(D->isParameterPack()));
  //isPackExpansion
  arboretum_create_edge(obj, context_.data_model_.method_isPackExpansion, context_.data_model_.arboretum_node_for(D->isPackExpansion()));
  //isExpandedParameterPack
  arboretum_create_edge(obj, context_.data_model_.method_isExpandedParameterPack, context_.data_model_.arboretum_node_for(D->isExpandedParameterPack()));
  //getNumExpansionTypes
  // unsigned int
  //getPlaceholderTypeConstraint
  {
    const Id* other = context_.resolve(D->getPlaceholderTypeConstraint());
    arboretum_create_edge(obj, context_.data_model_.method_getPlaceholderTypeConstraint, other);
  }
  //hasPlaceholderTypeConstraint
  arboretum_create_edge(obj, context_.data_model_.method_hasPlaceholderTypeConstraint, context_.data_model_.arboretum_node_for(D->hasPlaceholderTypeConstraint()));
  return true;
}

bool ArboretumASTVisitor::VisitOMPAllocateDecl(clang::OMPAllocateDecl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPCapturedExprDecl(clang::OMPCapturedExprDecl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPDeclareMapperDecl(clang::OMPDeclareMapperDecl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPDeclareReductionDecl(clang::OMPDeclareReductionDecl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPRequiresDecl(clang::OMPRequiresDecl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPThreadPrivateDecl(clang::OMPThreadPrivateDecl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCAtDefsFieldDecl(clang::ObjCAtDefsFieldDecl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCCategoryDecl(clang::ObjCCategoryDecl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCCategoryImplDecl(clang::ObjCCategoryImplDecl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCCompatibleAliasDecl(clang::ObjCCompatibleAliasDecl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCContainerDecl(clang::ObjCContainerDecl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCImplDecl(clang::ObjCImplDecl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCImplementationDecl(clang::ObjCImplementationDecl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCInterfaceDecl(clang::ObjCInterfaceDecl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCIvarDecl(clang::ObjCIvarDecl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCMethodDecl(clang::ObjCMethodDecl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCPropertyDecl(clang::ObjCPropertyDecl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCPropertyImplDecl(clang::ObjCPropertyImplDecl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCProtocolDecl(clang::ObjCProtocolDecl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCTypeParamDecl(clang::ObjCTypeParamDecl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitParmVarDecl(clang::ParmVarDecl* D) {
  const Id* obj = context_.resolve(D);
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_21, other);
  }
  //isObjCMethodParameter
  arboretum_create_edge(obj, context_.data_model_.method_isObjCMethodParameter, context_.data_model_.arboretum_node_for(D->isObjCMethodParameter()));
  //isDestroyedInCallee
  arboretum_create_edge(obj, context_.data_model_.method_isDestroyedInCallee, context_.data_model_.arboretum_node_for(D->isDestroyedInCallee()));
  //getFunctionScopeDepth
  // unsigned int
  //getFunctionScopeIndex
  // unsigned int
  //getObjCDeclQualifier
  {
    const Id* enum_value = context_.data_model_.resolve(D->getObjCDeclQualifier());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getObjCDeclQualifier, enum_value);
    }
  }
  //isKNRPromoted
  arboretum_create_edge(obj, context_.data_model_.method_isKNRPromoted, context_.data_model_.arboretum_node_for(D->isKNRPromoted()));
  //isExplicitObjectParameter
  arboretum_create_edge(obj, context_.data_model_.method_isExplicitObjectParameter, context_.data_model_.arboretum_node_for(D->isExplicitObjectParameter()));
  //getExplicitObjectParamThisLoc
  {
    const Id* other = context_.source_model_.resolve(D->getExplicitObjectParamThisLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getExplicitObjectParamThisLoc, other);
  }
  //getDefaultArg
  {
    const Id* other = context_.resolve(D->getDefaultArg());
    arboretum_create_edge(obj, context_.data_model_.method_getDefaultArg, other);
  }
  //getDefaultArgRange
  {
    const Id* other = context_.source_model_.resolve(D->getDefaultArgRange());
    arboretum_create_edge(obj, context_.data_model_.method_getDefaultArgRange, other);
  }
  //getUninstantiatedDefaultArg
  {
    const Id* other = context_.resolve(D->getUninstantiatedDefaultArg());
    arboretum_create_edge(obj, context_.data_model_.method_getUninstantiatedDefaultArg, other);
  }
  //hasDefaultArg
  arboretum_create_edge(obj, context_.data_model_.method_hasDefaultArg, context_.data_model_.arboretum_node_for(D->hasDefaultArg()));
  //hasUnparsedDefaultArg
  arboretum_create_edge(obj, context_.data_model_.method_hasUnparsedDefaultArg, context_.data_model_.arboretum_node_for(D->hasUnparsedDefaultArg()));
  //hasUninstantiatedDefaultArg
  arboretum_create_edge(obj, context_.data_model_.method_hasUninstantiatedDefaultArg, context_.data_model_.arboretum_node_for(D->hasUninstantiatedDefaultArg()));
  //hasInheritedDefaultArg
  arboretum_create_edge(obj, context_.data_model_.method_hasInheritedDefaultArg, context_.data_model_.arboretum_node_for(D->hasInheritedDefaultArg()));
  //getOriginalType
  {
    const Id* other = context_.resolve(D->getOriginalType());
    arboretum_create_edge(obj, context_.data_model_.method_getOriginalType_1, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitPragmaCommentDecl(clang::PragmaCommentDecl* D) {
  const Id* obj = context_.resolve(D);
  //getCommentKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getCommentKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getCommentKind, enum_value);
    }
  }
  //getArg
  // StringRef
  return true;
}

bool ArboretumASTVisitor::VisitPragmaDetectMismatchDecl(clang::PragmaDetectMismatchDecl* D) {
  const Id* obj = context_.resolve(D);
  //getName
  // StringRef
  //getValue
  // StringRef
  return true;
}

bool ArboretumASTVisitor::VisitRecordDecl(clang::RecordDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Id* obj = context_.resolve(D);
  //getPreviousDecl
  {
    const Id* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getPreviousDecl_5, other);
  }
  //getMostRecentDecl
  {
    const Id* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getMostRecentDecl_7, other);
  }
  //hasFlexibleArrayMember
  arboretum_create_edge(obj, context_.data_model_.method_hasFlexibleArrayMember, context_.data_model_.arboretum_node_for(D->hasFlexibleArrayMember()));
  //isAnonymousStructOrUnion
  arboretum_create_edge(obj, context_.data_model_.method_isAnonymousStructOrUnion_1, context_.data_model_.arboretum_node_for(D->isAnonymousStructOrUnion()));
  //hasObjectMember
  arboretum_create_edge(obj, context_.data_model_.method_hasObjectMember, context_.data_model_.arboretum_node_for(D->hasObjectMember()));
  //hasVolatileMember
  arboretum_create_edge(obj, context_.data_model_.method_hasVolatileMember, context_.data_model_.arboretum_node_for(D->hasVolatileMember()));
  //hasLoadedFieldsFromExternalStorage
  arboretum_create_edge(obj, context_.data_model_.method_hasLoadedFieldsFromExternalStorage, context_.data_model_.arboretum_node_for(D->hasLoadedFieldsFromExternalStorage()));
  //isNonTrivialToPrimitiveDefaultInitialize
  arboretum_create_edge(obj, context_.data_model_.method_isNonTrivialToPrimitiveDefaultInitialize, context_.data_model_.arboretum_node_for(D->isNonTrivialToPrimitiveDefaultInitialize()));
  //isNonTrivialToPrimitiveCopy
  arboretum_create_edge(obj, context_.data_model_.method_isNonTrivialToPrimitiveCopy, context_.data_model_.arboretum_node_for(D->isNonTrivialToPrimitiveCopy()));
  //isNonTrivialToPrimitiveDestroy
  arboretum_create_edge(obj, context_.data_model_.method_isNonTrivialToPrimitiveDestroy, context_.data_model_.arboretum_node_for(D->isNonTrivialToPrimitiveDestroy()));
  //hasNonTrivialToPrimitiveDefaultInitializeCUnion
  arboretum_create_edge(obj, context_.data_model_.method_hasNonTrivialToPrimitiveDefaultInitializeCUnion, context_.data_model_.arboretum_node_for(D->hasNonTrivialToPrimitiveDefaultInitializeCUnion()));
  //hasNonTrivialToPrimitiveDestructCUnion
  arboretum_create_edge(obj, context_.data_model_.method_hasNonTrivialToPrimitiveDestructCUnion, context_.data_model_.arboretum_node_for(D->hasNonTrivialToPrimitiveDestructCUnion()));
  //hasNonTrivialToPrimitiveCopyCUnion
  arboretum_create_edge(obj, context_.data_model_.method_hasNonTrivialToPrimitiveCopyCUnion, context_.data_model_.arboretum_node_for(D->hasNonTrivialToPrimitiveCopyCUnion()));
  //canPassInRegisters
  arboretum_create_edge(obj, context_.data_model_.method_canPassInRegisters, context_.data_model_.arboretum_node_for(D->canPassInRegisters()));
  //getArgPassingRestrictions
  {
    const Id* enum_value = context_.data_model_.resolve(D->getArgPassingRestrictions());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getArgPassingRestrictions, enum_value);
    }
  }
  //isParamDestroyedInCallee
  arboretum_create_edge(obj, context_.data_model_.method_isParamDestroyedInCallee, context_.data_model_.arboretum_node_for(D->isParamDestroyedInCallee()));
  //isRandomized
  arboretum_create_edge(obj, context_.data_model_.method_isRandomized, context_.data_model_.arboretum_node_for(D->isRandomized()));
  //isInjectedClassName
  arboretum_create_edge(obj, context_.data_model_.method_isInjectedClassName, context_.data_model_.arboretum_node_for(D->isInjectedClassName()));
  //isLambda
  arboretum_create_edge(obj, context_.data_model_.method_isLambda_1, context_.data_model_.arboretum_node_for(D->isLambda()));
  //isCapturedRecord
  arboretum_create_edge(obj, context_.data_model_.method_isCapturedRecord, context_.data_model_.arboretum_node_for(D->isCapturedRecord()));
  //getDefinition
  {
    const Id* other = context_.resolve(D->getDefinition());
    arboretum_create_edge(obj, context_.data_model_.method_getDefinition_3, other);
  }
  //isOrContainsUnion
  arboretum_create_edge(obj, context_.data_model_.method_isOrContainsUnion, context_.data_model_.arboretum_node_for(D->isOrContainsUnion()));
  //fields
  // field_range
  //field_empty
  arboretum_create_edge(obj, context_.data_model_.method_field_empty, context_.data_model_.arboretum_node_for(D->field_empty()));
  //findFirstNamedDataMember
  {
    const Id* other = context_.resolve(D->findFirstNamedDataMember());
    arboretum_create_edge(obj, context_.data_model_.method_findFirstNamedDataMember, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitRedeclarableTemplateDecl(clang::RedeclarableTemplateDecl* D) {
  const Id* obj = context_.resolve(D);
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_16, other);
  }
  //isMemberSpecialization
  arboretum_create_edge(obj, context_.data_model_.method_isMemberSpecialization, context_.data_model_.arboretum_node_for(D->isMemberSpecialization()));
  //getInstantiatedFromMemberTemplate
  {
    const Id* other = context_.resolve(D->getInstantiatedFromMemberTemplate());
    arboretum_create_edge(obj, context_.data_model_.method_getInstantiatedFromMemberTemplate_3, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitRequiresExprBodyDecl(clang::RequiresExprBodyDecl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitStaticAssertDecl(clang::StaticAssertDecl* D) {
  const Id* obj = context_.resolve(D);
  //getAssertExpr
  {
    const Id* other = context_.resolve(D->getAssertExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getAssertExpr, other);
  }
  //getMessage
  {
    const Id* other = context_.resolve(D->getMessage());
    arboretum_create_edge(obj, context_.data_model_.method_getMessage, other);
  }
  //isFailed
  arboretum_create_edge(obj, context_.data_model_.method_isFailed, context_.data_model_.arboretum_node_for(D->isFailed()));
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_1, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_22, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitTagDecl(clang::TagDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Id* obj = context_.resolve(D);
  //getBraceRange
  {
    const Id* other = context_.source_model_.resolve(D->getBraceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getBraceRange, other);
  }
  //getInnerLocStart
  {
    const Id* other = context_.source_model_.resolve(D->getInnerLocStart());
    arboretum_create_edge(obj, context_.data_model_.method_getInnerLocStart_1, other);
  }
  //getOuterLocStart
  {
    const Id* other = context_.source_model_.resolve(D->getOuterLocStart());
    arboretum_create_edge(obj, context_.data_model_.method_getOuterLocStart_1, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_23, other);
  }
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_17, other);
  }
  //isThisDeclarationADefinition
  arboretum_create_edge(obj, context_.data_model_.method_isThisDeclarationADefinition_3, context_.data_model_.arboretum_node_for(D->isThisDeclarationADefinition()));
  //isCompleteDefinition
  arboretum_create_edge(obj, context_.data_model_.method_isCompleteDefinition, context_.data_model_.arboretum_node_for(D->isCompleteDefinition()));
  //isCompleteDefinitionRequired
  arboretum_create_edge(obj, context_.data_model_.method_isCompleteDefinitionRequired, context_.data_model_.arboretum_node_for(D->isCompleteDefinitionRequired()));
  //isBeingDefined
  arboretum_create_edge(obj, context_.data_model_.method_isBeingDefined_1, context_.data_model_.arboretum_node_for(D->isBeingDefined()));
  //isEmbeddedInDeclarator
  arboretum_create_edge(obj, context_.data_model_.method_isEmbeddedInDeclarator, context_.data_model_.arboretum_node_for(D->isEmbeddedInDeclarator()));
  //isFreeStanding
  arboretum_create_edge(obj, context_.data_model_.method_isFreeStanding, context_.data_model_.arboretum_node_for(D->isFreeStanding()));
  //mayHaveOutOfDateDef
  arboretum_create_edge(obj, context_.data_model_.method_mayHaveOutOfDateDef, context_.data_model_.arboretum_node_for(D->mayHaveOutOfDateDef()));
  //isDependentType
  arboretum_create_edge(obj, context_.data_model_.method_isDependentType, context_.data_model_.arboretum_node_for(D->isDependentType()));
  //isThisDeclarationADemotedDefinition
  arboretum_create_edge(obj, context_.data_model_.method_isThisDeclarationADemotedDefinition, context_.data_model_.arboretum_node_for(D->isThisDeclarationADemotedDefinition()));
  //getDefinition
  {
    const Id* other = context_.resolve(D->getDefinition());
    arboretum_create_edge(obj, context_.data_model_.method_getDefinition_4, other);
  }
  //getKindName
  // StringRef
  //getTagKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getTagKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getTagKind, enum_value);
    }
  }
  //isStruct
  arboretum_create_edge(obj, context_.data_model_.method_isStruct, context_.data_model_.arboretum_node_for(D->isStruct()));
  //isInterface
  arboretum_create_edge(obj, context_.data_model_.method_isInterface, context_.data_model_.arboretum_node_for(D->isInterface()));
  //isClass
  arboretum_create_edge(obj, context_.data_model_.method_isClass, context_.data_model_.arboretum_node_for(D->isClass()));
  //isUnion
  arboretum_create_edge(obj, context_.data_model_.method_isUnion, context_.data_model_.arboretum_node_for(D->isUnion()));
  //isEnum
  arboretum_create_edge(obj, context_.data_model_.method_isEnum, context_.data_model_.arboretum_node_for(D->isEnum()));
  //hasNameForLinkage
  arboretum_create_edge(obj, context_.data_model_.method_hasNameForLinkage, context_.data_model_.arboretum_node_for(D->hasNameForLinkage()));
  //getTypedefNameForAnonDecl
  {
    const Id* other = context_.resolve(D->getTypedefNameForAnonDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getTypedefNameForAnonDecl, other);
  }
  //getQualifier
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getNumTemplateParameterLists
  // unsigned int
  return true;
}

bool ArboretumASTVisitor::VisitTemplateDecl(clang::TemplateDecl* D) {
  const Id* obj = context_.resolve(D);
  //getTemplateParameters
  //hasAssociatedConstraints
  arboretum_create_edge(obj, context_.data_model_.method_hasAssociatedConstraints_1, context_.data_model_.arboretum_node_for(D->hasAssociatedConstraints()));
  //getTemplatedDecl
  {
    const Id* other = context_.resolve(D->getTemplatedDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getTemplatedDecl_2, other);
  }
  //isTypeAlias
  arboretum_create_edge(obj, context_.data_model_.method_isTypeAlias_1, context_.data_model_.arboretum_node_for(D->isTypeAlias()));
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_24, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitTemplateParamObjectDecl(clang::TemplateParamObjectDecl* D) {
  const Id* obj = context_.resolve(D);
  //getValue
  // const APValue &
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_18, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitTemplateTemplateParmDecl(clang::TemplateTemplateParmDecl* D) {
  const Id* obj = context_.resolve(D);
  //isParameterPack
  arboretum_create_edge(obj, context_.data_model_.method_isParameterPack_3, context_.data_model_.arboretum_node_for(D->isParameterPack()));
  //isPackExpansion
  arboretum_create_edge(obj, context_.data_model_.method_isPackExpansion_1, context_.data_model_.arboretum_node_for(D->isPackExpansion()));
  //isExpandedParameterPack
  arboretum_create_edge(obj, context_.data_model_.method_isExpandedParameterPack_1, context_.data_model_.arboretum_node_for(D->isExpandedParameterPack()));
  //getNumExpansionTemplateParameters
  // unsigned int
  //getDefaultArgStorage
  // const DefArgStorage &
  //hasDefaultArgument
  arboretum_create_edge(obj, context_.data_model_.method_hasDefaultArgument_1, context_.data_model_.arboretum_node_for(D->hasDefaultArgument()));
  //getDefaultArgument
  // const TemplateArgumentLoc &
  //getDefaultArgumentLoc
  {
    const Id* other = context_.source_model_.resolve(D->getDefaultArgumentLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getDefaultArgumentLoc_1, other);
  }
  //defaultArgumentWasInherited
  arboretum_create_edge(obj, context_.data_model_.method_defaultArgumentWasInherited_1, context_.data_model_.arboretum_node_for(D->defaultArgumentWasInherited()));
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_25, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitTemplateTypeParmDecl(clang::TemplateTypeParmDecl* D) {
  const Id* obj = context_.resolve(D);
  //wasDeclaredWithTypename
  arboretum_create_edge(obj, context_.data_model_.method_wasDeclaredWithTypename, context_.data_model_.arboretum_node_for(D->wasDeclaredWithTypename()));
  //getDefaultArgStorage
  // const DefArgStorage &
  //hasDefaultArgument
  arboretum_create_edge(obj, context_.data_model_.method_hasDefaultArgument_2, context_.data_model_.arboretum_node_for(D->hasDefaultArgument()));
  //getDefaultArgumentInfo
  //getDefaultArgumentLoc
  {
    const Id* other = context_.source_model_.resolve(D->getDefaultArgumentLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getDefaultArgumentLoc_2, other);
  }
  //defaultArgumentWasInherited
  arboretum_create_edge(obj, context_.data_model_.method_defaultArgumentWasInherited_2, context_.data_model_.arboretum_node_for(D->defaultArgumentWasInherited()));
  //getDepth
  // unsigned int
  //getIndex
  // unsigned int
  //isParameterPack
  arboretum_create_edge(obj, context_.data_model_.method_isParameterPack_4, context_.data_model_.arboretum_node_for(D->isParameterPack()));
  //isPackExpansion
  arboretum_create_edge(obj, context_.data_model_.method_isPackExpansion_2, context_.data_model_.arboretum_node_for(D->isPackExpansion()));
  //isExpandedParameterPack
  arboretum_create_edge(obj, context_.data_model_.method_isExpandedParameterPack_2, context_.data_model_.arboretum_node_for(D->isExpandedParameterPack()));
  //getNumExpansionParameters
  // unsigned int
  //getTypeConstraint
  //hasTypeConstraint
  arboretum_create_edge(obj, context_.data_model_.method_hasTypeConstraint, context_.data_model_.arboretum_node_for(D->hasTypeConstraint()));
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_26, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitTopLevelStmtDecl(clang::TopLevelStmtDecl* D) {
  const Id* obj = context_.resolve(D);
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_27, other);
  }
  //getStmt
  {
    const Id* other = context_.resolve(D->getStmt());
    arboretum_create_edge(obj, context_.data_model_.method_getStmt_1, other);
  }
  //isSemiMissing
  arboretum_create_edge(obj, context_.data_model_.method_isSemiMissing, context_.data_model_.arboretum_node_for(D->isSemiMissing()));
  return true;
}

bool ArboretumASTVisitor::VisitTranslationUnitDecl(clang::TranslationUnitDecl* D) {
  const Id* obj = context_.resolve(D);
  //getAnonymousNamespace
  {
    const Id* other = context_.resolve(D->getAnonymousNamespace());
    arboretum_create_edge(obj, context_.data_model_.method_getAnonymousNamespace_1, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitTypeAliasDecl(clang::TypeAliasDecl* D) {
  const Id* obj = context_.resolve(D);
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_28, other);
  }
  //getDescribedAliasTemplate
  {
    const Id* other = context_.resolve(D->getDescribedAliasTemplate());
    arboretum_create_edge(obj, context_.data_model_.method_getDescribedAliasTemplate, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitTypeAliasTemplateDecl(clang::TypeAliasTemplateDecl* D) {
  const Id* obj = context_.resolve(D);
  //getTemplatedDecl
  {
    const Id* other = context_.resolve(D->getTemplatedDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getTemplatedDecl_3, other);
  }
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_19, other);
  }
  //getPreviousDecl
  {
    const Id* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getPreviousDecl_6, other);
  }
  //getInstantiatedFromMemberTemplate
  {
    const Id* other = context_.resolve(D->getInstantiatedFromMemberTemplate());
    arboretum_create_edge(obj, context_.data_model_.method_getInstantiatedFromMemberTemplate_4, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitTypeDecl(clang::TypeDecl* D) {
  const Id* obj = context_.resolve(D);
  //getTypeForDecl
  {
    const Id* other = context_.resolve(D->getTypeForDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getTypeForDecl, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_3, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_29, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitTypedefDecl(clang::TypedefDecl* D) {
  const Id* obj = context_.resolve(D);
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_30, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitTypedefNameDecl(clang::TypedefNameDecl* D) {
  const Id* obj = context_.resolve(D);
  //isModed
  arboretum_create_edge(obj, context_.data_model_.method_isModed, context_.data_model_.arboretum_node_for(D->isModed()));
  //getTypeSourceInfo
  //getUnderlyingType
  {
    const Id* other = context_.resolve(D->getUnderlyingType());
    arboretum_create_edge(obj, context_.data_model_.method_getUnderlyingType_4, other);
  }
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_20, other);
  }
  //isTransparentTag
  arboretum_create_edge(obj, context_.data_model_.method_isTransparentTag, context_.data_model_.arboretum_node_for(D->isTransparentTag()));
  return true;
}

bool ArboretumASTVisitor::VisitUnnamedGlobalConstantDecl(clang::UnnamedGlobalConstantDecl* D) {
  const Id* obj = context_.resolve(D);
  //getValue
  // const APValue &
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedUsingIfExistsDecl(clang::UnresolvedUsingIfExistsDecl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedUsingTypenameDecl(clang::UnresolvedUsingTypenameDecl* D) {
  const Id* obj = context_.resolve(D);
  //getUsingLoc
  {
    const Id* other = context_.source_model_.resolve(D->getUsingLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getUsingLoc, other);
  }
  //getTypenameLoc
  {
    const Id* other = context_.source_model_.resolve(D->getTypenameLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getTypenameLoc, other);
  }
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getQualifier
  //getNameInfo
  // DeclarationNameInfo
  //isPackExpansion
  arboretum_create_edge(obj, context_.data_model_.method_isPackExpansion_3, context_.data_model_.arboretum_node_for(D->isPackExpansion()));
  //getEllipsisLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEllipsisLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEllipsisLoc_2, other);
  }
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_21, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedUsingValueDecl(clang::UnresolvedUsingValueDecl* D) {
  const Id* obj = context_.resolve(D);
  //getUsingLoc
  {
    const Id* other = context_.source_model_.resolve(D->getUsingLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getUsingLoc_1, other);
  }
  //isAccessDeclaration
  arboretum_create_edge(obj, context_.data_model_.method_isAccessDeclaration, context_.data_model_.arboretum_node_for(D->isAccessDeclaration()));
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getQualifier
  //getNameInfo
  // DeclarationNameInfo
  //isPackExpansion
  arboretum_create_edge(obj, context_.data_model_.method_isPackExpansion_4, context_.data_model_.arboretum_node_for(D->isPackExpansion()));
  //getEllipsisLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEllipsisLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEllipsisLoc_3, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_31, other);
  }
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_22, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitUsingDecl(clang::UsingDecl* D) {
  const Id* obj = context_.resolve(D);
  //getUsingLoc
  {
    const Id* other = context_.source_model_.resolve(D->getUsingLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getUsingLoc_2, other);
  }
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getQualifier
  //getNameInfo
  // DeclarationNameInfo
  //isAccessDeclaration
  arboretum_create_edge(obj, context_.data_model_.method_isAccessDeclaration_1, context_.data_model_.arboretum_node_for(D->isAccessDeclaration()));
  //hasTypename
  arboretum_create_edge(obj, context_.data_model_.method_hasTypename, context_.data_model_.arboretum_node_for(D->hasTypename()));
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_32, other);
  }
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_23, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitUsingDirectiveDecl(clang::UsingDirectiveDecl* D) {
  const Id* obj = context_.resolve(D);
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getQualifier
  //getNominatedNamespaceAsWritten
  {
    const Id* other = context_.resolve(D->getNominatedNamespaceAsWritten());
    arboretum_create_edge(obj, context_.data_model_.method_getNominatedNamespaceAsWritten, other);
  }
  //getNominatedNamespace
  {
    const Id* other = context_.resolve(D->getNominatedNamespace());
    arboretum_create_edge(obj, context_.data_model_.method_getNominatedNamespace, other);
  }
  //getCommonAncestor
  //getUsingLoc
  {
    const Id* other = context_.source_model_.resolve(D->getUsingLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getUsingLoc_3, other);
  }
  //getNamespaceKeyLocation
  {
    const Id* other = context_.source_model_.resolve(D->getNamespaceKeyLocation());
    arboretum_create_edge(obj, context_.data_model_.method_getNamespaceKeyLocation, other);
  }
  //getIdentLocation
  {
    const Id* other = context_.source_model_.resolve(D->getIdentLocation());
    arboretum_create_edge(obj, context_.data_model_.method_getIdentLocation, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_33, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitUsingEnumDecl(clang::UsingEnumDecl* D) {
  const Id* obj = context_.resolve(D);
  //getUsingLoc
  {
    const Id* other = context_.source_model_.resolve(D->getUsingLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getUsingLoc_4, other);
  }
  //getEnumLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEnumLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEnumLoc, other);
  }
  //getQualifier
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getEnumTypeLoc
  // TypeLoc
  //getEnumType
  //getEnumDecl
  {
    const Id* other = context_.resolve(D->getEnumDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getEnumDecl, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_34, other);
  }
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_24, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitUsingPackDecl(clang::UsingPackDecl* D) {
  const Id* obj = context_.resolve(D);
  //getInstantiatedFromUsingDecl
  {
    const Id* other = context_.resolve(D->getInstantiatedFromUsingDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getInstantiatedFromUsingDecl, other);
  }
  //expansions
  // ArrayRef<NamedDecl *>
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_35, other);
  }
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_25, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitUsingShadowDecl(clang::UsingShadowDecl* D) {
  const Id* obj = context_.resolve(D);
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_26, other);
  }
  //getTargetDecl
  {
    const Id* other = context_.resolve(D->getTargetDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getTargetDecl, other);
  }
  //getIntroducer
  {
    const Id* other = context_.resolve(D->getIntroducer());
    arboretum_create_edge(obj, context_.data_model_.method_getIntroducer_1, other);
  }
  //getNextUsingShadowDecl
  {
    const Id* other = context_.resolve(D->getNextUsingShadowDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getNextUsingShadowDecl, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitValueDecl(clang::ValueDecl* D) {
  const Id* obj = context_.resolve(D);
  //getType
  {
    const Id* other = context_.resolve(D->getType());
    arboretum_create_edge(obj, context_.data_model_.method_getType, other);
  }
  //isWeak
  arboretum_create_edge(obj, context_.data_model_.method_isWeak, context_.data_model_.arboretum_node_for(D->isWeak()));
  //isInitCapture
  arboretum_create_edge(obj, context_.data_model_.method_isInitCapture, context_.data_model_.arboretum_node_for(D->isInitCapture()));
  //getPotentiallyDecomposedVarDecl
  {
    const Id* other = context_.resolve(D->getPotentiallyDecomposedVarDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getPotentiallyDecomposedVarDecl, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitVarDecl(clang::VarDecl* D) {
  const Id* obj = context_.resolve(D);
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_36, other);
  }
  //getStorageClass
  {
    const Id* enum_value = context_.data_model_.resolve(D->getStorageClass());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getStorageClass_1, enum_value);
    }
  }
  //getTSCSpec
  {
    const Id* enum_value = context_.data_model_.resolve(D->getTSCSpec());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getTSCSpec, enum_value);
    }
  }
  //getTLSKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getTLSKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getTLSKind, enum_value);
    }
  }
  //hasLocalStorage
  arboretum_create_edge(obj, context_.data_model_.method_hasLocalStorage, context_.data_model_.arboretum_node_for(D->hasLocalStorage()));
  //isStaticLocal
  arboretum_create_edge(obj, context_.data_model_.method_isStaticLocal, context_.data_model_.arboretum_node_for(D->isStaticLocal()));
  //hasExternalStorage
  arboretum_create_edge(obj, context_.data_model_.method_hasExternalStorage, context_.data_model_.arboretum_node_for(D->hasExternalStorage()));
  //hasGlobalStorage
  arboretum_create_edge(obj, context_.data_model_.method_hasGlobalStorage, context_.data_model_.arboretum_node_for(D->hasGlobalStorage()));
  //getStorageDuration
  {
    const Id* enum_value = context_.data_model_.resolve(D->getStorageDuration());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getStorageDuration_1, enum_value);
    }
  }
  //getLanguageLinkage
  {
    const Id* enum_value = context_.data_model_.resolve(D->getLanguageLinkage());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getLanguageLinkage_1, enum_value);
    }
  }
  //isExternC
  arboretum_create_edge(obj, context_.data_model_.method_isExternC_1, context_.data_model_.arboretum_node_for(D->isExternC()));
  //isInExternCContext
  arboretum_create_edge(obj, context_.data_model_.method_isInExternCContext_1, context_.data_model_.arboretum_node_for(D->isInExternCContext()));
  //isInExternCXXContext
  arboretum_create_edge(obj, context_.data_model_.method_isInExternCXXContext_1, context_.data_model_.arboretum_node_for(D->isInExternCXXContext()));
  //isLocalVarDecl
  arboretum_create_edge(obj, context_.data_model_.method_isLocalVarDecl, context_.data_model_.arboretum_node_for(D->isLocalVarDecl()));
  //isLocalVarDeclOrParm
  arboretum_create_edge(obj, context_.data_model_.method_isLocalVarDeclOrParm, context_.data_model_.arboretum_node_for(D->isLocalVarDeclOrParm()));
  //isFunctionOrMethodVarDecl
  arboretum_create_edge(obj, context_.data_model_.method_isFunctionOrMethodVarDecl, context_.data_model_.arboretum_node_for(D->isFunctionOrMethodVarDecl()));
  //isStaticDataMember
  arboretum_create_edge(obj, context_.data_model_.method_isStaticDataMember, context_.data_model_.arboretum_node_for(D->isStaticDataMember()));
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_27, other);
  }
  //isThisDeclarationADefinition
  {
    const Id* enum_value = context_.data_model_.resolve(D->isThisDeclarationADefinition());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_isThisDeclarationADefinition_4, enum_value);
    }
  }
  //hasDefinition
  {
    const Id* enum_value = context_.data_model_.resolve(D->hasDefinition());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_hasDefinition_1, enum_value);
    }
  }
  //getActingDefinition
  {
    const Id* other = context_.resolve(D->getActingDefinition());
    arboretum_create_edge(obj, context_.data_model_.method_getActingDefinition, other);
  }
  //getDefinition
  {
    const Id* other = context_.resolve(D->getDefinition());
    arboretum_create_edge(obj, context_.data_model_.method_getDefinition_5, other);
  }
  //isOutOfLine
  arboretum_create_edge(obj, context_.data_model_.method_isOutOfLine_2, context_.data_model_.arboretum_node_for(D->isOutOfLine()));
  //isFileVarDecl
  arboretum_create_edge(obj, context_.data_model_.method_isFileVarDecl, context_.data_model_.arboretum_node_for(D->isFileVarDecl()));
  //getAnyInitializer
  {
    const Id* other = context_.resolve(D->getAnyInitializer());
    arboretum_create_edge(obj, context_.data_model_.method_getAnyInitializer, other);
  }
  //hasInit
  arboretum_create_edge(obj, context_.data_model_.method_hasInit, context_.data_model_.arboretum_node_for(D->hasInit()));
  //getInit
  {
    const Id* other = context_.resolve(D->getInit());
    arboretum_create_edge(obj, context_.data_model_.method_getInit, other);
  }
  //getInitializingDeclaration
  {
    const Id* other = context_.resolve(D->getInitializingDeclaration());
    arboretum_create_edge(obj, context_.data_model_.method_getInitializingDeclaration, other);
  }
  //ensureEvaluatedStmt
  //getEvaluatedStmt
  //evaluateValue
  //getEvaluatedValue
  //hasConstantInitialization
  arboretum_create_edge(obj, context_.data_model_.method_hasConstantInitialization, context_.data_model_.arboretum_node_for(D->hasConstantInitialization()));
  //getInitStyle
  {
    const Id* enum_value = context_.data_model_.resolve(D->getInitStyle());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getInitStyle, enum_value);
    }
  }
  //isDirectInit
  arboretum_create_edge(obj, context_.data_model_.method_isDirectInit, context_.data_model_.arboretum_node_for(D->isDirectInit()));
  //isThisDeclarationADemotedDefinition
  arboretum_create_edge(obj, context_.data_model_.method_isThisDeclarationADemotedDefinition_1, context_.data_model_.arboretum_node_for(D->isThisDeclarationADemotedDefinition()));
  //isExceptionVariable
  arboretum_create_edge(obj, context_.data_model_.method_isExceptionVariable, context_.data_model_.arboretum_node_for(D->isExceptionVariable()));
  //isNRVOVariable
  arboretum_create_edge(obj, context_.data_model_.method_isNRVOVariable, context_.data_model_.arboretum_node_for(D->isNRVOVariable()));
  //isCXXForRangeDecl
  arboretum_create_edge(obj, context_.data_model_.method_isCXXForRangeDecl, context_.data_model_.arboretum_node_for(D->isCXXForRangeDecl()));
  //isObjCForDecl
  arboretum_create_edge(obj, context_.data_model_.method_isObjCForDecl, context_.data_model_.arboretum_node_for(D->isObjCForDecl()));
  //isARCPseudoStrong
  arboretum_create_edge(obj, context_.data_model_.method_isARCPseudoStrong, context_.data_model_.arboretum_node_for(D->isARCPseudoStrong()));
  //isInline
  arboretum_create_edge(obj, context_.data_model_.method_isInline_1, context_.data_model_.arboretum_node_for(D->isInline()));
  //isInlineSpecified
  arboretum_create_edge(obj, context_.data_model_.method_isInlineSpecified_1, context_.data_model_.arboretum_node_for(D->isInlineSpecified()));
  //isConstexpr
  arboretum_create_edge(obj, context_.data_model_.method_isConstexpr_1, context_.data_model_.arboretum_node_for(D->isConstexpr()));
  //isInitCapture
  arboretum_create_edge(obj, context_.data_model_.method_isInitCapture_1, context_.data_model_.arboretum_node_for(D->isInitCapture()));
  //isParameterPack
  arboretum_create_edge(obj, context_.data_model_.method_isParameterPack_5, context_.data_model_.arboretum_node_for(D->isParameterPack()));
  //isPreviousDeclInSameBlockScope
  arboretum_create_edge(obj, context_.data_model_.method_isPreviousDeclInSameBlockScope, context_.data_model_.arboretum_node_for(D->isPreviousDeclInSameBlockScope()));
  //isEscapingByref
  arboretum_create_edge(obj, context_.data_model_.method_isEscapingByref, context_.data_model_.arboretum_node_for(D->isEscapingByref()));
  //isNonEscapingByref
  arboretum_create_edge(obj, context_.data_model_.method_isNonEscapingByref, context_.data_model_.arboretum_node_for(D->isNonEscapingByref()));
  //hasDependentAlignment
  arboretum_create_edge(obj, context_.data_model_.method_hasDependentAlignment, context_.data_model_.arboretum_node_for(D->hasDependentAlignment()));
  //getTemplateInstantiationPattern
  {
    const Id* other = context_.resolve(D->getTemplateInstantiationPattern());
    arboretum_create_edge(obj, context_.data_model_.method_getTemplateInstantiationPattern_2, other);
  }
  //getInstantiatedFromStaticDataMember
  {
    const Id* other = context_.resolve(D->getInstantiatedFromStaticDataMember());
    arboretum_create_edge(obj, context_.data_model_.method_getInstantiatedFromStaticDataMember, other);
  }
  //getTemplateSpecializationKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getTemplateSpecializationKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getTemplateSpecializationKind_3, enum_value);
    }
  }
  //getTemplateSpecializationKindForInstantiation
  {
    const Id* enum_value = context_.data_model_.resolve(D->getTemplateSpecializationKindForInstantiation());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getTemplateSpecializationKindForInstantiation_1, enum_value);
    }
  }
  //getPointOfInstantiation
  {
    const Id* other = context_.source_model_.resolve(D->getPointOfInstantiation());
    arboretum_create_edge(obj, context_.data_model_.method_getPointOfInstantiation_2, other);
  }
  //getMemberSpecializationInfo
  //getDescribedVarTemplate
  {
    const Id* other = context_.resolve(D->getDescribedVarTemplate());
    arboretum_create_edge(obj, context_.data_model_.method_getDescribedVarTemplate, other);
  }
  //isKnownToBeDefined
  arboretum_create_edge(obj, context_.data_model_.method_isKnownToBeDefined, context_.data_model_.arboretum_node_for(D->isKnownToBeDefined()));
  return true;
}

bool ArboretumASTVisitor::VisitVarTemplateDecl(clang::VarTemplateDecl* D) {
  const Id* obj = context_.resolve(D);
  //getTemplatedDecl
  {
    const Id* other = context_.resolve(D->getTemplatedDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getTemplatedDecl_4, other);
  }
  //isThisDeclarationADefinition
  arboretum_create_edge(obj, context_.data_model_.method_isThisDeclarationADefinition_5, context_.data_model_.arboretum_node_for(D->isThisDeclarationADefinition()));
  //getCanonicalDecl
  {
    const Id* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_28, other);
  }
  //getPreviousDecl
  {
    const Id* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getPreviousDecl_7, other);
  }
  //getMostRecentDecl
  {
    const Id* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getMostRecentDecl_8, other);
  }
  //getInstantiatedFromMemberTemplate
  {
    const Id* other = context_.resolve(D->getInstantiatedFromMemberTemplate());
    arboretum_create_edge(obj, context_.data_model_.method_getInstantiatedFromMemberTemplate_5, other);
  }
  //specializations
  // spec_range
  return true;
}

bool ArboretumASTVisitor::VisitVarTemplatePartialSpecializationDecl(clang::VarTemplatePartialSpecializationDecl* D) {
  const Id* obj = context_.resolve(D);
  //getTemplateParameters
  //getTemplateArgsAsWritten
  //hasAssociatedConstraints
  arboretum_create_edge(obj, context_.data_model_.method_hasAssociatedConstraints_2, context_.data_model_.arboretum_node_for(D->hasAssociatedConstraints()));
  //getInstantiatedFromMember
  {
    const Id* other = context_.resolve(D->getInstantiatedFromMember());
    arboretum_create_edge(obj, context_.data_model_.method_getInstantiatedFromMember_1, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_37, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitVarTemplateSpecializationDecl(clang::VarTemplateSpecializationDecl* D) {
  const Id* obj = context_.resolve(D);
  //getSpecializedTemplate
  {
    const Id* other = context_.resolve(D->getSpecializedTemplate());
    arboretum_create_edge(obj, context_.data_model_.method_getSpecializedTemplate_1, other);
  }
  //getTemplateArgs
  // const TemplateArgumentList &
  //getTemplateArgsInfo
  //getSpecializationKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getSpecializationKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getSpecializationKind_1, enum_value);
    }
  }
  //isExplicitSpecialization
  arboretum_create_edge(obj, context_.data_model_.method_isExplicitSpecialization_1, context_.data_model_.arboretum_node_for(D->isExplicitSpecialization()));
  //isClassScopeExplicitSpecialization
  arboretum_create_edge(obj, context_.data_model_.method_isClassScopeExplicitSpecialization_1, context_.data_model_.arboretum_node_for(D->isClassScopeExplicitSpecialization()));
  //isExplicitInstantiationOrSpecialization
  arboretum_create_edge(obj, context_.data_model_.method_isExplicitInstantiationOrSpecialization_1, context_.data_model_.arboretum_node_for(D->isExplicitInstantiationOrSpecialization()));
  //getPointOfInstantiation
  {
    const Id* other = context_.source_model_.resolve(D->getPointOfInstantiation());
    arboretum_create_edge(obj, context_.data_model_.method_getPointOfInstantiation_3, other);
  }
  //getInstantiatedFrom
  // llvm::PointerUnion<VarTemplateDecl *, VarTemplatePartialSpecializationDecl *>
  //getSpecializedTemplateOrPartial
  // llvm::PointerUnion<VarTemplateDecl *, VarTemplatePartialSpecializationDecl *>
  //getTemplateInstantiationArgs
  // const TemplateArgumentList &
  //getTypeAsWritten
  //getExternLoc
  {
    const Id* other = context_.source_model_.resolve(D->getExternLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getExternLoc_2, other);
  }
  //getTemplateKeywordLoc
  {
    const Id* other = context_.source_model_.resolve(D->getTemplateKeywordLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getTemplateKeywordLoc_1, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_38, other);
  }
  return true;
}


// Stmts
bool ArboretumASTVisitor::VisitAbstractConditionalOperator(clang::AbstractConditionalOperator* D) {
  const Id* obj = context_.resolve(D);
  //getCond
  {
    const Id* other = context_.resolve(D->getCond());
    arboretum_create_edge(obj, context_.data_model_.method_getCond, other);
  }
  //getTrueExpr
  {
    const Id* other = context_.resolve(D->getTrueExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getTrueExpr, other);
  }
  //getFalseExpr
  {
    const Id* other = context_.resolve(D->getFalseExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getFalseExpr, other);
  }
  //getQuestionLoc
  {
    const Id* other = context_.source_model_.resolve(D->getQuestionLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getQuestionLoc, other);
  }
  //getColonLoc
  {
    const Id* other = context_.source_model_.resolve(D->getColonLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getColonLoc_1, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitAddrLabelExpr(clang::AddrLabelExpr* D) {
  const Id* obj = context_.resolve(D);
  //getAmpAmpLoc
  {
    const Id* other = context_.source_model_.resolve(D->getAmpAmpLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getAmpAmpLoc, other);
  }
  //getLabelLoc
  {
    const Id* other = context_.source_model_.resolve(D->getLabelLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getLabelLoc, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_4, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_3, other);
  }
  //getLabel
  {
    const Id* other = context_.resolve(D->getLabel());
    arboretum_create_edge(obj, context_.data_model_.method_getLabel, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitArrayInitIndexExpr(clang::ArrayInitIndexExpr* D) {
  const Id* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_5, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_4, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitArrayInitLoopExpr(clang::ArrayInitLoopExpr* D) {
  const Id* obj = context_.resolve(D);
  //getCommonExpr
  {
    const Id* other = context_.resolve(D->getCommonExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getCommonExpr, other);
  }
  //getSubExpr
  {
    const Id* other = context_.resolve(D->getSubExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getSubExpr, other);
  }
  //getArraySize
  // llvm::APInt
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_6, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_5, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitArraySubscriptExpr(clang::ArraySubscriptExpr* D) {
  const Id* obj = context_.resolve(D);
  //getLHS
  {
    const Id* other = context_.resolve(D->getLHS());
    arboretum_create_edge(obj, context_.data_model_.method_getLHS, other);
  }
  //getRHS
  {
    const Id* other = context_.resolve(D->getRHS());
    arboretum_create_edge(obj, context_.data_model_.method_getRHS, other);
  }
  //getBase
  {
    const Id* other = context_.resolve(D->getBase());
    arboretum_create_edge(obj, context_.data_model_.method_getBase, other);
  }
  //getIdx
  {
    const Id* other = context_.resolve(D->getIdx());
    arboretum_create_edge(obj, context_.data_model_.method_getIdx, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_7, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_6, other);
  }
  //getRBracketLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRBracketLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRBracketLoc_2, other);
  }
  //getExprLoc
  {
    const Id* other = context_.source_model_.resolve(D->getExprLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getExprLoc, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitArrayTypeTraitExpr(clang::ArrayTypeTraitExpr* D) {
  const Id* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_8, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_7, other);
  }
  //getTrait
  {
    const Id* enum_value = context_.data_model_.resolve(D->getTrait());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getTrait, enum_value);
    }
  }
  //getQueriedType
  {
    const Id* other = context_.resolve(D->getQueriedType());
    arboretum_create_edge(obj, context_.data_model_.method_getQueriedType, other);
  }
  //getQueriedTypeSourceInfo
  //getDimensionExpression
  {
    const Id* other = context_.resolve(D->getDimensionExpression());
    arboretum_create_edge(obj, context_.data_model_.method_getDimensionExpression, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitAsTypeExpr(clang::AsTypeExpr* D) {
  const Id* obj = context_.resolve(D);
  //getSrcExpr
  {
    const Id* other = context_.resolve(D->getSrcExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getSrcExpr, other);
  }
  //getBuiltinLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBuiltinLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBuiltinLoc, other);
  }
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_2, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_9, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_8, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitAsmStmt(clang::AsmStmt* D) {
  const Id* obj = context_.resolve(D);
  //getAsmLoc
  {
    const Id* other = context_.source_model_.resolve(D->getAsmLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getAsmLoc_1, other);
  }
  //isSimple
  arboretum_create_edge(obj, context_.data_model_.method_isSimple, context_.data_model_.arboretum_node_for(D->isSimple()));
  //isVolatile
  arboretum_create_edge(obj, context_.data_model_.method_isVolatile_2, context_.data_model_.arboretum_node_for(D->isVolatile()));
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_10, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_9, other);
  }
  //getNumOutputs
  // unsigned int
  //getNumPlusOperands
  // unsigned int
  //getNumInputs
  // unsigned int
  //getNumClobbers
  // unsigned int
  //inputs
  // inputs_const_range
  //outputs
  // outputs_const_range
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitAtomicExpr(clang::AtomicExpr* D) {
  const Id* obj = context_.resolve(D);
  //getPtr
  {
    const Id* other = context_.resolve(D->getPtr());
    arboretum_create_edge(obj, context_.data_model_.method_getPtr, other);
  }
  //getOrder
  {
    const Id* other = context_.resolve(D->getOrder());
    arboretum_create_edge(obj, context_.data_model_.method_getOrder, other);
  }
  //getScope
  {
    const Id* other = context_.resolve(D->getScope());
    arboretum_create_edge(obj, context_.data_model_.method_getScope, other);
  }
  //getVal1
  {
    const Id* other = context_.resolve(D->getVal1());
    arboretum_create_edge(obj, context_.data_model_.method_getVal1, other);
  }
  //getOrderFail
  {
    const Id* other = context_.resolve(D->getOrderFail());
    arboretum_create_edge(obj, context_.data_model_.method_getOrderFail, other);
  }
  //getVal2
  {
    const Id* other = context_.resolve(D->getVal2());
    arboretum_create_edge(obj, context_.data_model_.method_getVal2, other);
  }
  //getWeak
  {
    const Id* other = context_.resolve(D->getWeak());
    arboretum_create_edge(obj, context_.data_model_.method_getWeak, other);
  }
  //getValueType
  {
    const Id* other = context_.resolve(D->getValueType());
    arboretum_create_edge(obj, context_.data_model_.method_getValueType_1, other);
  }
  //getOp
  {
    const Id* enum_value = context_.data_model_.resolve(D->getOp());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getOp, enum_value);
    }
  }
  //getOpAsString
  // StringRef
  //getNumSubExprs
  // unsigned int
  //getSubExprs
  //isVolatile
  arboretum_create_edge(obj, context_.data_model_.method_isVolatile_3, context_.data_model_.arboretum_node_for(D->isVolatile()));
  //isCmpXChg
  arboretum_create_edge(obj, context_.data_model_.method_isCmpXChg, context_.data_model_.arboretum_node_for(D->isCmpXChg()));
  //isOpenCL
  arboretum_create_edge(obj, context_.data_model_.method_isOpenCL, context_.data_model_.arboretum_node_for(D->isOpenCL()));
  //getBuiltinLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBuiltinLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBuiltinLoc_1, other);
  }
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_3, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_11, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_10, other);
  }
  //children
  // const_child_range
  //getScopeModel
  // std::unique_ptr<AtomicScopeModel>
  return true;
}

bool ArboretumASTVisitor::VisitAttributedStmt(clang::AttributedStmt* D) {
  const Id* obj = context_.resolve(D);
  //getAttrLoc
  {
    const Id* other = context_.source_model_.resolve(D->getAttrLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getAttrLoc, other);
  }
  //getAttrs
  // ArrayRef<const Attr *>
  //getSubStmt
  {
    const Id* other = context_.resolve(D->getSubStmt());
    arboretum_create_edge(obj, context_.data_model_.method_getSubStmt, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_12, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_11, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitBinaryConditionalOperator(clang::BinaryConditionalOperator* D) {
  const Id* obj = context_.resolve(D);
  //getCommon
  {
    const Id* other = context_.resolve(D->getCommon());
    arboretum_create_edge(obj, context_.data_model_.method_getCommon, other);
  }
  //getOpaqueValue
  {
    const Id* other = context_.resolve(D->getOpaqueValue());
    arboretum_create_edge(obj, context_.data_model_.method_getOpaqueValue, other);
  }
  //getCond
  {
    const Id* other = context_.resolve(D->getCond());
    arboretum_create_edge(obj, context_.data_model_.method_getCond_1, other);
  }
  //getTrueExpr
  {
    const Id* other = context_.resolve(D->getTrueExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getTrueExpr_1, other);
  }
  //getFalseExpr
  {
    const Id* other = context_.resolve(D->getFalseExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getFalseExpr_1, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_13, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_12, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitBinaryOperator(clang::BinaryOperator* D) {
  const Id* obj = context_.resolve(D);
  //getExprLoc
  {
    const Id* other = context_.source_model_.resolve(D->getExprLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getExprLoc_1, other);
  }
  //getOperatorLoc
  {
    const Id* other = context_.source_model_.resolve(D->getOperatorLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getOperatorLoc, other);
  }
  //getOpcode
  {
    const Id* enum_value = context_.data_model_.resolve(D->getOpcode());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getOpcode, enum_value);
    }
  }
  //getLHS
  {
    const Id* other = context_.resolve(D->getLHS());
    arboretum_create_edge(obj, context_.data_model_.method_getLHS_1, other);
  }
  //getRHS
  {
    const Id* other = context_.resolve(D->getRHS());
    arboretum_create_edge(obj, context_.data_model_.method_getRHS_1, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_14, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_13, other);
  }
  //getOpcodeStr
  // StringRef
  //isPtrMemOp
  arboretum_create_edge(obj, context_.data_model_.method_isPtrMemOp, context_.data_model_.arboretum_node_for(D->isPtrMemOp()));
  //isMultiplicativeOp
  arboretum_create_edge(obj, context_.data_model_.method_isMultiplicativeOp, context_.data_model_.arboretum_node_for(D->isMultiplicativeOp()));
  //isAdditiveOp
  arboretum_create_edge(obj, context_.data_model_.method_isAdditiveOp, context_.data_model_.arboretum_node_for(D->isAdditiveOp()));
  //isShiftOp
  arboretum_create_edge(obj, context_.data_model_.method_isShiftOp, context_.data_model_.arboretum_node_for(D->isShiftOp()));
  //isBitwiseOp
  arboretum_create_edge(obj, context_.data_model_.method_isBitwiseOp, context_.data_model_.arboretum_node_for(D->isBitwiseOp()));
  //isRelationalOp
  arboretum_create_edge(obj, context_.data_model_.method_isRelationalOp, context_.data_model_.arboretum_node_for(D->isRelationalOp()));
  //isEqualityOp
  arboretum_create_edge(obj, context_.data_model_.method_isEqualityOp, context_.data_model_.arboretum_node_for(D->isEqualityOp()));
  //isComparisonOp
  arboretum_create_edge(obj, context_.data_model_.method_isComparisonOp, context_.data_model_.arboretum_node_for(D->isComparisonOp()));
  //isCommaOp
  arboretum_create_edge(obj, context_.data_model_.method_isCommaOp, context_.data_model_.arboretum_node_for(D->isCommaOp()));
  //isLogicalOp
  arboretum_create_edge(obj, context_.data_model_.method_isLogicalOp, context_.data_model_.arboretum_node_for(D->isLogicalOp()));
  //isAssignmentOp
  arboretum_create_edge(obj, context_.data_model_.method_isAssignmentOp, context_.data_model_.arboretum_node_for(D->isAssignmentOp()));
  //isCompoundAssignmentOp
  arboretum_create_edge(obj, context_.data_model_.method_isCompoundAssignmentOp, context_.data_model_.arboretum_node_for(D->isCompoundAssignmentOp()));
  //isShiftAssignOp
  arboretum_create_edge(obj, context_.data_model_.method_isShiftAssignOp, context_.data_model_.arboretum_node_for(D->isShiftAssignOp()));
  //children
  // const_child_range
  //hasStoredFPFeatures
  arboretum_create_edge(obj, context_.data_model_.method_hasStoredFPFeatures, context_.data_model_.arboretum_node_for(D->hasStoredFPFeatures()));
  //getStoredFPFeatures
  // FPOptionsOverride
  //getFPFeatures
  // FPOptionsOverride
  return true;
}

bool ArboretumASTVisitor::VisitBlockExpr(clang::BlockExpr* D) {
  const Id* obj = context_.resolve(D);
  //getBlockDecl
  {
    const Id* other = context_.resolve(D->getBlockDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getBlockDecl, other);
  }
  //getCaretLocation
  {
    const Id* other = context_.source_model_.resolve(D->getCaretLocation());
    arboretum_create_edge(obj, context_.data_model_.method_getCaretLocation_1, other);
  }
  //getBody
  {
    const Id* other = context_.resolve(D->getBody());
    arboretum_create_edge(obj, context_.data_model_.method_getBody_4, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_15, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_14, other);
  }
  //getFunctionType
  {
    const Id* other = context_.resolve(D->getFunctionType());
    arboretum_create_edge(obj, context_.data_model_.method_getFunctionType, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitBreakStmt(clang::BreakStmt* D) {
  const Id* obj = context_.resolve(D);
  //getBreakLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBreakLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBreakLoc, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_16, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_15, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitBuiltinBitCastExpr(clang::BuiltinBitCastExpr* D) {
  const Id* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_17, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_16, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitCStyleCastExpr(clang::CStyleCastExpr* D) {
  const Id* obj = context_.resolve(D);
  //getLParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getLParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getLParenLoc, other);
  }
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_4, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_18, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_17, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitCUDAKernelCallExpr(clang::CUDAKernelCallExpr* D) {
  const Id* obj = context_.resolve(D);
  //getConfig
  {
    const Id* other = context_.resolve(D->getConfig());
    arboretum_create_edge(obj, context_.data_model_.method_getConfig, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXAddrspaceCastExpr(clang::CXXAddrspaceCastExpr* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCXXBindTemporaryExpr(clang::CXXBindTemporaryExpr* D) {
  const Id* obj = context_.resolve(D);
  //getTemporary
  //getSubExpr
  {
    const Id* other = context_.resolve(D->getSubExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getSubExpr_1, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_19, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_18, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXBoolLiteralExpr(clang::CXXBoolLiteralExpr* D) {
  const Id* obj = context_.resolve(D);
  //getValue
  arboretum_create_edge(obj, context_.data_model_.method_getValue_4, context_.data_model_.arboretum_node_for(D->getValue()));
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_20, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_19, other);
  }
  //getLocation
  {
    const Id* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.method_getLocation_1, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXCatchStmt(clang::CXXCatchStmt* D) {
  const Id* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_21, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_20, other);
  }
  //getCatchLoc
  {
    const Id* other = context_.source_model_.resolve(D->getCatchLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getCatchLoc, other);
  }
  //getExceptionDecl
  {
    const Id* other = context_.resolve(D->getExceptionDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getExceptionDecl, other);
  }
  //getCaughtType
  {
    const Id* other = context_.resolve(D->getCaughtType());
    arboretum_create_edge(obj, context_.data_model_.method_getCaughtType, other);
  }
  //getHandlerBlock
  {
    const Id* other = context_.resolve(D->getHandlerBlock());
    arboretum_create_edge(obj, context_.data_model_.method_getHandlerBlock, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXConstCastExpr(clang::CXXConstCastExpr* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCXXConstructExpr(clang::CXXConstructExpr* D) {
  const Id* obj = context_.resolve(D);
  //getConstructor
  {
    const Id* other = context_.resolve(D->getConstructor());
    arboretum_create_edge(obj, context_.data_model_.method_getConstructor, other);
  }
  //getLocation
  {
    const Id* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.method_getLocation_2, other);
  }
  //isElidable
  arboretum_create_edge(obj, context_.data_model_.method_isElidable, context_.data_model_.arboretum_node_for(D->isElidable()));
  //hadMultipleCandidates
  arboretum_create_edge(obj, context_.data_model_.method_hadMultipleCandidates, context_.data_model_.arboretum_node_for(D->hadMultipleCandidates()));
  //isListInitialization
  arboretum_create_edge(obj, context_.data_model_.method_isListInitialization, context_.data_model_.arboretum_node_for(D->isListInitialization()));
  //isStdInitListInitialization
  arboretum_create_edge(obj, context_.data_model_.method_isStdInitListInitialization, context_.data_model_.arboretum_node_for(D->isStdInitListInitialization()));
  //requiresZeroInitialization
  arboretum_create_edge(obj, context_.data_model_.method_requiresZeroInitialization, context_.data_model_.arboretum_node_for(D->requiresZeroInitialization()));
  //getConstructionKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getConstructionKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getConstructionKind, enum_value);
    }
  }
  //arguments
  // const_arg_range
  //getArgs
  //getNumArgs
  // unsigned int
  //isImmediateEscalating
  arboretum_create_edge(obj, context_.data_model_.method_isImmediateEscalating_1, context_.data_model_.arboretum_node_for(D->isImmediateEscalating()));
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_22, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_21, other);
  }
  //getParenOrBraceRange
  {
    const Id* other = context_.source_model_.resolve(D->getParenOrBraceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getParenOrBraceRange, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXDefaultArgExpr(clang::CXXDefaultArgExpr* D) {
  const Id* obj = context_.resolve(D);
  //getParam
  {
    const Id* other = context_.resolve(D->getParam());
    arboretum_create_edge(obj, context_.data_model_.method_getParam, other);
  }
  //hasRewrittenInit
  arboretum_create_edge(obj, context_.data_model_.method_hasRewrittenInit, context_.data_model_.arboretum_node_for(D->hasRewrittenInit()));
  //getExpr
  {
    const Id* other = context_.resolve(D->getExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getExpr, other);
  }
  //getRewrittenExpr
  {
    const Id* other = context_.resolve(D->getRewrittenExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getRewrittenExpr, other);
  }
  //getAdjustedRewrittenExpr
  {
    const Id* other = context_.resolve(D->getAdjustedRewrittenExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getAdjustedRewrittenExpr, other);
  }
  //getUsedContext
  //getUsedLocation
  {
    const Id* other = context_.source_model_.resolve(D->getUsedLocation());
    arboretum_create_edge(obj, context_.data_model_.method_getUsedLocation, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_23, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_22, other);
  }
  //getExprLoc
  {
    const Id* other = context_.source_model_.resolve(D->getExprLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getExprLoc_2, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXDefaultInitExpr(clang::CXXDefaultInitExpr* D) {
  const Id* obj = context_.resolve(D);
  //hasRewrittenInit
  arboretum_create_edge(obj, context_.data_model_.method_hasRewrittenInit_1, context_.data_model_.arboretum_node_for(D->hasRewrittenInit()));
  //getField
  {
    const Id* other = context_.resolve(D->getField());
    arboretum_create_edge(obj, context_.data_model_.method_getField, other);
  }
  //getExpr
  {
    const Id* other = context_.resolve(D->getExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getExpr_1, other);
  }
  //getRewrittenExpr
  {
    const Id* other = context_.resolve(D->getRewrittenExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getRewrittenExpr_1, other);
  }
  //getUsedContext
  //getUsedLocation
  {
    const Id* other = context_.source_model_.resolve(D->getUsedLocation());
    arboretum_create_edge(obj, context_.data_model_.method_getUsedLocation_1, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_24, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_23, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXDeleteExpr(clang::CXXDeleteExpr* D) {
  const Id* obj = context_.resolve(D);
  //isGlobalDelete
  arboretum_create_edge(obj, context_.data_model_.method_isGlobalDelete, context_.data_model_.arboretum_node_for(D->isGlobalDelete()));
  //isArrayForm
  arboretum_create_edge(obj, context_.data_model_.method_isArrayForm, context_.data_model_.arboretum_node_for(D->isArrayForm()));
  //isArrayFormAsWritten
  arboretum_create_edge(obj, context_.data_model_.method_isArrayFormAsWritten, context_.data_model_.arboretum_node_for(D->isArrayFormAsWritten()));
  //doesUsualArrayDeleteWantSize
  arboretum_create_edge(obj, context_.data_model_.method_doesUsualArrayDeleteWantSize, context_.data_model_.arboretum_node_for(D->doesUsualArrayDeleteWantSize()));
  //getOperatorDelete
  {
    const Id* other = context_.resolve(D->getOperatorDelete());
    arboretum_create_edge(obj, context_.data_model_.method_getOperatorDelete_1, other);
  }
  //getArgument
  {
    const Id* other = context_.resolve(D->getArgument());
    arboretum_create_edge(obj, context_.data_model_.method_getArgument, other);
  }
  //getDestroyedType
  {
    const Id* other = context_.resolve(D->getDestroyedType());
    arboretum_create_edge(obj, context_.data_model_.method_getDestroyedType, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_25, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_24, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXDependentScopeMemberExpr(clang::CXXDependentScopeMemberExpr* D) {
  const Id* obj = context_.resolve(D);
  //isImplicitAccess
  arboretum_create_edge(obj, context_.data_model_.method_isImplicitAccess, context_.data_model_.arboretum_node_for(D->isImplicitAccess()));
  //getBaseType
  {
    const Id* other = context_.resolve(D->getBaseType());
    arboretum_create_edge(obj, context_.data_model_.method_getBaseType_1, other);
  }
  //isArrow
  arboretum_create_edge(obj, context_.data_model_.method_isArrow, context_.data_model_.arboretum_node_for(D->isArrow()));
  //getOperatorLoc
  {
    const Id* other = context_.source_model_.resolve(D->getOperatorLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getOperatorLoc_1, other);
  }
  //getQualifier
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getFirstQualifierFoundInScope
  {
    const Id* other = context_.resolve(D->getFirstQualifierFoundInScope());
    arboretum_create_edge(obj, context_.data_model_.method_getFirstQualifierFoundInScope, other);
  }
  //getMemberNameInfo
  // const DeclarationNameInfo &
  //getMember
  // DeclarationName
  //getMemberLoc
  {
    const Id* other = context_.source_model_.resolve(D->getMemberLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getMemberLoc, other);
  }
  //getTemplateKeywordLoc
  {
    const Id* other = context_.source_model_.resolve(D->getTemplateKeywordLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getTemplateKeywordLoc_2, other);
  }
  //getLAngleLoc
  {
    const Id* other = context_.source_model_.resolve(D->getLAngleLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getLAngleLoc, other);
  }
  //getRAngleLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRAngleLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRAngleLoc, other);
  }
  //hasTemplateKeyword
  arboretum_create_edge(obj, context_.data_model_.method_hasTemplateKeyword, context_.data_model_.arboretum_node_for(D->hasTemplateKeyword()));
  //hasExplicitTemplateArgs
  arboretum_create_edge(obj, context_.data_model_.method_hasExplicitTemplateArgs, context_.data_model_.arboretum_node_for(D->hasExplicitTemplateArgs()));
  //getTemplateArgs
  //getNumTemplateArgs
  // unsigned int
  //template_arguments
  // ArrayRef<TemplateArgumentLoc>
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_26, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_25, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXDynamicCastExpr(clang::CXXDynamicCastExpr* D) {
  const Id* obj = context_.resolve(D);
  //isAlwaysNull
  arboretum_create_edge(obj, context_.data_model_.method_isAlwaysNull, context_.data_model_.arboretum_node_for(D->isAlwaysNull()));
  return true;
}

bool ArboretumASTVisitor::VisitCXXFoldExpr(clang::CXXFoldExpr* D) {
  const Id* obj = context_.resolve(D);
  //getCallee
  {
    const Id* other = context_.resolve(D->getCallee());
    arboretum_create_edge(obj, context_.data_model_.method_getCallee, other);
  }
  //getLHS
  {
    const Id* other = context_.resolve(D->getLHS());
    arboretum_create_edge(obj, context_.data_model_.method_getLHS_2, other);
  }
  //getRHS
  {
    const Id* other = context_.resolve(D->getRHS());
    arboretum_create_edge(obj, context_.data_model_.method_getRHS_2, other);
  }
  //isRightFold
  arboretum_create_edge(obj, context_.data_model_.method_isRightFold, context_.data_model_.arboretum_node_for(D->isRightFold()));
  //isLeftFold
  arboretum_create_edge(obj, context_.data_model_.method_isLeftFold, context_.data_model_.arboretum_node_for(D->isLeftFold()));
  //getPattern
  {
    const Id* other = context_.resolve(D->getPattern());
    arboretum_create_edge(obj, context_.data_model_.method_getPattern_1, other);
  }
  //getInit
  {
    const Id* other = context_.resolve(D->getInit());
    arboretum_create_edge(obj, context_.data_model_.method_getInit_1, other);
  }
  //getLParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getLParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getLParenLoc_1, other);
  }
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_5, other);
  }
  //getEllipsisLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEllipsisLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEllipsisLoc_4, other);
  }
  //getOperator
  {
    const Id* enum_value = context_.data_model_.resolve(D->getOperator());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getOperator, enum_value);
    }
  }
  //getNumExpansions
  // std::optional<unsigned int>
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_27, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_26, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXForRangeStmt(clang::CXXForRangeStmt* D) {
  const Id* obj = context_.resolve(D);
  //getInit
  {
    const Id* other = context_.resolve(D->getInit());
    arboretum_create_edge(obj, context_.data_model_.method_getInit_2, other);
  }
  //getLoopVariable
  {
    const Id* other = context_.resolve(D->getLoopVariable());
    arboretum_create_edge(obj, context_.data_model_.method_getLoopVariable, other);
  }
  //getRangeInit
  {
    const Id* other = context_.resolve(D->getRangeInit());
    arboretum_create_edge(obj, context_.data_model_.method_getRangeInit, other);
  }
  //getRangeStmt
  {
    const Id* other = context_.resolve(D->getRangeStmt());
    arboretum_create_edge(obj, context_.data_model_.method_getRangeStmt, other);
  }
  //getBeginStmt
  {
    const Id* other = context_.resolve(D->getBeginStmt());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginStmt, other);
  }
  //getEndStmt
  {
    const Id* other = context_.resolve(D->getEndStmt());
    arboretum_create_edge(obj, context_.data_model_.method_getEndStmt, other);
  }
  //getCond
  {
    const Id* other = context_.resolve(D->getCond());
    arboretum_create_edge(obj, context_.data_model_.method_getCond_2, other);
  }
  //getInc
  {
    const Id* other = context_.resolve(D->getInc());
    arboretum_create_edge(obj, context_.data_model_.method_getInc, other);
  }
  //getLoopVarStmt
  {
    const Id* other = context_.resolve(D->getLoopVarStmt());
    arboretum_create_edge(obj, context_.data_model_.method_getLoopVarStmt, other);
  }
  //getBody
  {
    const Id* other = context_.resolve(D->getBody());
    arboretum_create_edge(obj, context_.data_model_.method_getBody_5, other);
  }
  //getForLoc
  {
    const Id* other = context_.source_model_.resolve(D->getForLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getForLoc, other);
  }
  //getCoawaitLoc
  {
    const Id* other = context_.source_model_.resolve(D->getCoawaitLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getCoawaitLoc, other);
  }
  //getColonLoc
  {
    const Id* other = context_.source_model_.resolve(D->getColonLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getColonLoc_2, other);
  }
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_6, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_28, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_27, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXFunctionalCastExpr(clang::CXXFunctionalCastExpr* D) {
  const Id* obj = context_.resolve(D);
  //getLParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getLParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getLParenLoc_2, other);
  }
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_7, other);
  }
  //isListInitialization
  arboretum_create_edge(obj, context_.data_model_.method_isListInitialization_1, context_.data_model_.arboretum_node_for(D->isListInitialization()));
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_29, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_28, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXInheritedCtorInitExpr(clang::CXXInheritedCtorInitExpr* D) {
  const Id* obj = context_.resolve(D);
  //getConstructor
  {
    const Id* other = context_.resolve(D->getConstructor());
    arboretum_create_edge(obj, context_.data_model_.method_getConstructor_1, other);
  }
  //constructsVBase
  arboretum_create_edge(obj, context_.data_model_.method_constructsVBase, context_.data_model_.arboretum_node_for(D->constructsVBase()));
  //getConstructionKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getConstructionKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getConstructionKind_1, enum_value);
    }
  }
  //inheritedFromVBase
  arboretum_create_edge(obj, context_.data_model_.method_inheritedFromVBase, context_.data_model_.arboretum_node_for(D->inheritedFromVBase()));
  //getLocation
  {
    const Id* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.method_getLocation_3, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_30, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_29, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXMemberCallExpr(clang::CXXMemberCallExpr* D) {
  const Id* obj = context_.resolve(D);
  //getImplicitObjectArgument
  {
    const Id* other = context_.resolve(D->getImplicitObjectArgument());
    arboretum_create_edge(obj, context_.data_model_.method_getImplicitObjectArgument, other);
  }
  //getObjectType
  {
    const Id* other = context_.resolve(D->getObjectType());
    arboretum_create_edge(obj, context_.data_model_.method_getObjectType, other);
  }
  //getMethodDecl
  {
    const Id* other = context_.resolve(D->getMethodDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getMethodDecl, other);
  }
  //getRecordDecl
  {
    const Id* other = context_.resolve(D->getRecordDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getRecordDecl, other);
  }
  //getExprLoc
  {
    const Id* other = context_.source_model_.resolve(D->getExprLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getExprLoc_3, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXNamedCastExpr(clang::CXXNamedCastExpr* D) {
  const Id* obj = context_.resolve(D);
  //getCastName
  //getOperatorLoc
  {
    const Id* other = context_.source_model_.resolve(D->getOperatorLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getOperatorLoc_2, other);
  }
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_8, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_31, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_30, other);
  }
  //getAngleBrackets
  {
    const Id* other = context_.source_model_.resolve(D->getAngleBrackets());
    arboretum_create_edge(obj, context_.data_model_.method_getAngleBrackets, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXNewExpr(clang::CXXNewExpr* D) {
  const Id* obj = context_.resolve(D);
  //getAllocatedType
  {
    const Id* other = context_.resolve(D->getAllocatedType());
    arboretum_create_edge(obj, context_.data_model_.method_getAllocatedType, other);
  }
  //getAllocatedTypeSourceInfo
  //getOperatorNew
  {
    const Id* other = context_.resolve(D->getOperatorNew());
    arboretum_create_edge(obj, context_.data_model_.method_getOperatorNew, other);
  }
  //getOperatorDelete
  {
    const Id* other = context_.resolve(D->getOperatorDelete());
    arboretum_create_edge(obj, context_.data_model_.method_getOperatorDelete_2, other);
  }
  //isArray
  arboretum_create_edge(obj, context_.data_model_.method_isArray, context_.data_model_.arboretum_node_for(D->isArray()));
  //getArraySize
  // std::optional<const Expr *>
  //getNumPlacementArgs
  // unsigned int
  //isParenTypeId
  arboretum_create_edge(obj, context_.data_model_.method_isParenTypeId, context_.data_model_.arboretum_node_for(D->isParenTypeId()));
  //getTypeIdParens
  {
    const Id* other = context_.source_model_.resolve(D->getTypeIdParens());
    arboretum_create_edge(obj, context_.data_model_.method_getTypeIdParens, other);
  }
  //isGlobalNew
  arboretum_create_edge(obj, context_.data_model_.method_isGlobalNew, context_.data_model_.arboretum_node_for(D->isGlobalNew()));
  //hasInitializer
  arboretum_create_edge(obj, context_.data_model_.method_hasInitializer, context_.data_model_.arboretum_node_for(D->hasInitializer()));
  //getInitializationStyle
  {
    const Id* enum_value = context_.data_model_.resolve(D->getInitializationStyle());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getInitializationStyle, enum_value);
    }
  }
  //getInitializer
  {
    const Id* other = context_.resolve(D->getInitializer());
    arboretum_create_edge(obj, context_.data_model_.method_getInitializer, other);
  }
  //getConstructExpr
  {
    const Id* other = context_.resolve(D->getConstructExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getConstructExpr, other);
  }
  //passAlignment
  arboretum_create_edge(obj, context_.data_model_.method_passAlignment, context_.data_model_.arboretum_node_for(D->passAlignment()));
  //doesUsualArrayDeleteWantSize
  arboretum_create_edge(obj, context_.data_model_.method_doesUsualArrayDeleteWantSize_1, context_.data_model_.arboretum_node_for(D->doesUsualArrayDeleteWantSize()));
  //placement_arguments
  // llvm::iterator_range<const_arg_iterator>
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_32, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_31, other);
  }
  //getDirectInitRange
  {
    const Id* other = context_.source_model_.resolve(D->getDirectInitRange());
    arboretum_create_edge(obj, context_.data_model_.method_getDirectInitRange, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_39, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXNoexceptExpr(clang::CXXNoexceptExpr* D) {
  const Id* obj = context_.resolve(D);
  //getOperand
  {
    const Id* other = context_.resolve(D->getOperand());
    arboretum_create_edge(obj, context_.data_model_.method_getOperand, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_33, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_32, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_40, other);
  }
  //getValue
  arboretum_create_edge(obj, context_.data_model_.method_getValue_5, context_.data_model_.arboretum_node_for(D->getValue()));
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXNullPtrLiteralExpr(clang::CXXNullPtrLiteralExpr* D) {
  const Id* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_34, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_33, other);
  }
  //getLocation
  {
    const Id* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.method_getLocation_4, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXOperatorCallExpr(clang::CXXOperatorCallExpr* D) {
  const Id* obj = context_.resolve(D);
  //getOperator
  {
    const Id* enum_value = context_.data_model_.resolve(D->getOperator());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getOperator_1, enum_value);
    }
  }
  //isAssignmentOp
  arboretum_create_edge(obj, context_.data_model_.method_isAssignmentOp_1, context_.data_model_.arboretum_node_for(D->isAssignmentOp()));
  //isComparisonOp
  arboretum_create_edge(obj, context_.data_model_.method_isComparisonOp_1, context_.data_model_.arboretum_node_for(D->isComparisonOp()));
  //isInfixBinaryOp
  arboretum_create_edge(obj, context_.data_model_.method_isInfixBinaryOp, context_.data_model_.arboretum_node_for(D->isInfixBinaryOp()));
  //getOperatorLoc
  {
    const Id* other = context_.source_model_.resolve(D->getOperatorLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getOperatorLoc_3, other);
  }
  //getExprLoc
  {
    const Id* other = context_.source_model_.resolve(D->getExprLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getExprLoc_4, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_35, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_34, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_41, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXParenListInitExpr(clang::CXXParenListInitExpr* D) {
  const Id* obj = context_.resolve(D);
  //getInitExprs
  // const ArrayRef<Expr *>
  //getUserSpecifiedInitExprs
  // const ArrayRef<Expr *>
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_36, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_35, other);
  }
  //getInitLoc
  {
    const Id* other = context_.source_model_.resolve(D->getInitLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getInitLoc, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_42, other);
  }
  //getArrayFiller
  {
    const Id* other = context_.resolve(D->getArrayFiller());
    arboretum_create_edge(obj, context_.data_model_.method_getArrayFiller, other);
  }
  //getInitializedFieldInUnion
  {
    const Id* other = context_.resolve(D->getInitializedFieldInUnion());
    arboretum_create_edge(obj, context_.data_model_.method_getInitializedFieldInUnion, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXPseudoDestructorExpr(clang::CXXPseudoDestructorExpr* D) {
  const Id* obj = context_.resolve(D);
  //getBase
  {
    const Id* other = context_.resolve(D->getBase());
    arboretum_create_edge(obj, context_.data_model_.method_getBase_1, other);
  }
  //hasQualifier
  arboretum_create_edge(obj, context_.data_model_.method_hasQualifier, context_.data_model_.arboretum_node_for(D->hasQualifier()));
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getQualifier
  //isArrow
  arboretum_create_edge(obj, context_.data_model_.method_isArrow_1, context_.data_model_.arboretum_node_for(D->isArrow()));
  //getOperatorLoc
  {
    const Id* other = context_.source_model_.resolve(D->getOperatorLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getOperatorLoc_4, other);
  }
  //getScopeTypeInfo
  //getColonColonLoc
  {
    const Id* other = context_.source_model_.resolve(D->getColonColonLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getColonColonLoc, other);
  }
  //getTildeLoc
  {
    const Id* other = context_.source_model_.resolve(D->getTildeLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getTildeLoc, other);
  }
  //getDestroyedTypeInfo
  //getDestroyedTypeIdentifier
  //getDestroyedType
  {
    const Id* other = context_.resolve(D->getDestroyedType());
    arboretum_create_edge(obj, context_.data_model_.method_getDestroyedType_1, other);
  }
  //getDestroyedTypeLoc
  {
    const Id* other = context_.source_model_.resolve(D->getDestroyedTypeLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getDestroyedTypeLoc, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_37, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_36, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXReinterpretCastExpr(clang::CXXReinterpretCastExpr* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCXXRewrittenBinaryOperator(clang::CXXRewrittenBinaryOperator* D) {
  const Id* obj = context_.resolve(D);
  //getSemanticForm
  {
    const Id* other = context_.resolve(D->getSemanticForm());
    arboretum_create_edge(obj, context_.data_model_.method_getSemanticForm, other);
  }
  //getDecomposedForm
  // DecomposedForm
  //isReversed
  arboretum_create_edge(obj, context_.data_model_.method_isReversed, context_.data_model_.arboretum_node_for(D->isReversed()));
  //getOperator
  {
    const Id* enum_value = context_.data_model_.resolve(D->getOperator());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getOperator_2, enum_value);
    }
  }
  //getOpcode
  {
    const Id* enum_value = context_.data_model_.resolve(D->getOpcode());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getOpcode_1, enum_value);
    }
  }
  //getOpcodeStr
  // StringRef
  //isComparisonOp
  arboretum_create_edge(obj, context_.data_model_.method_isComparisonOp_2, context_.data_model_.arboretum_node_for(D->isComparisonOp()));
  //isAssignmentOp
  arboretum_create_edge(obj, context_.data_model_.method_isAssignmentOp_2, context_.data_model_.arboretum_node_for(D->isAssignmentOp()));
  //getLHS
  {
    const Id* other = context_.resolve(D->getLHS());
    arboretum_create_edge(obj, context_.data_model_.method_getLHS_3, other);
  }
  //getRHS
  {
    const Id* other = context_.resolve(D->getRHS());
    arboretum_create_edge(obj, context_.data_model_.method_getRHS_3, other);
  }
  //getOperatorLoc
  {
    const Id* other = context_.source_model_.resolve(D->getOperatorLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getOperatorLoc_5, other);
  }
  //getExprLoc
  {
    const Id* other = context_.source_model_.resolve(D->getExprLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getExprLoc_5, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_38, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_37, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_43, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXScalarValueInitExpr(clang::CXXScalarValueInitExpr* D) {
  const Id* obj = context_.resolve(D);
  //getTypeSourceInfo
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_9, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_39, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_38, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXStaticCastExpr(clang::CXXStaticCastExpr* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCXXStdInitializerListExpr(clang::CXXStdInitializerListExpr* D) {
  const Id* obj = context_.resolve(D);
  //getSubExpr
  {
    const Id* other = context_.resolve(D->getSubExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getSubExpr_2, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_40, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_39, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_44, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXTemporaryObjectExpr(clang::CXXTemporaryObjectExpr* D) {
  const Id* obj = context_.resolve(D);
  //getTypeSourceInfo
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_41, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_40, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXThisExpr(clang::CXXThisExpr* D) {
  const Id* obj = context_.resolve(D);
  //getLocation
  {
    const Id* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.method_getLocation_5, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_42, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_41, other);
  }
  //isImplicit
  arboretum_create_edge(obj, context_.data_model_.method_isImplicit_1, context_.data_model_.arboretum_node_for(D->isImplicit()));
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXThrowExpr(clang::CXXThrowExpr* D) {
  const Id* obj = context_.resolve(D);
  //getSubExpr
  {
    const Id* other = context_.resolve(D->getSubExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getSubExpr_3, other);
  }
  //getThrowLoc
  {
    const Id* other = context_.source_model_.resolve(D->getThrowLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getThrowLoc, other);
  }
  //isThrownVariableInScope
  arboretum_create_edge(obj, context_.data_model_.method_isThrownVariableInScope, context_.data_model_.arboretum_node_for(D->isThrownVariableInScope()));
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_43, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_42, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXTryStmt(clang::CXXTryStmt* D) {
  const Id* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_44, other);
  }
  //getTryLoc
  {
    const Id* other = context_.source_model_.resolve(D->getTryLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getTryLoc, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_43, other);
  }
  //getTryBlock
  {
    const Id* other = context_.resolve(D->getTryBlock());
    arboretum_create_edge(obj, context_.data_model_.method_getTryBlock, other);
  }
  //getNumHandlers
  // unsigned int
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXTypeidExpr(clang::CXXTypeidExpr* D) {
  const Id* obj = context_.resolve(D);
  //isPotentiallyEvaluated
  arboretum_create_edge(obj, context_.data_model_.method_isPotentiallyEvaluated, context_.data_model_.arboretum_node_for(D->isPotentiallyEvaluated()));
  //isTypeOperand
  arboretum_create_edge(obj, context_.data_model_.method_isTypeOperand, context_.data_model_.arboretum_node_for(D->isTypeOperand()));
  //getTypeOperandSourceInfo
  //getExprOperand
  {
    const Id* other = context_.resolve(D->getExprOperand());
    arboretum_create_edge(obj, context_.data_model_.method_getExprOperand, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_45, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_44, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_45, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXUnresolvedConstructExpr(clang::CXXUnresolvedConstructExpr* D) {
  const Id* obj = context_.resolve(D);
  //getTypeAsWritten
  {
    const Id* other = context_.resolve(D->getTypeAsWritten());
    arboretum_create_edge(obj, context_.data_model_.method_getTypeAsWritten_2, other);
  }
  //getTypeSourceInfo
  //getLParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getLParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getLParenLoc_3, other);
  }
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_10, other);
  }
  //isListInitialization
  arboretum_create_edge(obj, context_.data_model_.method_isListInitialization_2, context_.data_model_.arboretum_node_for(D->isListInitialization()));
  //getNumArgs
  // unsigned int
  //arguments
  // const_arg_range
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_46, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_45, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXUuidofExpr(clang::CXXUuidofExpr* D) {
  const Id* obj = context_.resolve(D);
  //isTypeOperand
  arboretum_create_edge(obj, context_.data_model_.method_isTypeOperand_1, context_.data_model_.arboretum_node_for(D->isTypeOperand()));
  //getTypeOperandSourceInfo
  //getExprOperand
  {
    const Id* other = context_.resolve(D->getExprOperand());
    arboretum_create_edge(obj, context_.data_model_.method_getExprOperand_1, other);
  }
  //getGuidDecl
  {
    const Id* other = context_.resolve(D->getGuidDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getGuidDecl, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_47, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_46, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_46, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCallExpr(clang::CallExpr* D) {
  const Id* obj = context_.resolve(D);
  //getCallee
  {
    const Id* other = context_.resolve(D->getCallee());
    arboretum_create_edge(obj, context_.data_model_.method_getCallee_1, other);
  }
  //getADLCallKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getADLCallKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getADLCallKind, enum_value);
    }
  }
  //usesADL
  arboretum_create_edge(obj, context_.data_model_.method_usesADL, context_.data_model_.arboretum_node_for(D->usesADL()));
  //hasStoredFPFeatures
  arboretum_create_edge(obj, context_.data_model_.method_hasStoredFPFeatures_1, context_.data_model_.arboretum_node_for(D->hasStoredFPFeatures()));
  //getCalleeDecl
  {
    const Id* other = context_.resolve(D->getCalleeDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCalleeDecl, other);
  }
  //getDirectCallee
  {
    const Id* other = context_.resolve(D->getDirectCallee());
    arboretum_create_edge(obj, context_.data_model_.method_getDirectCallee, other);
  }
  //getNumArgs
  // unsigned int
  //getArgs
  //arguments
  // const_arg_range
  //getStoredFPFeatures
  // FPOptionsOverride
  //getFPFeatures
  // FPOptionsOverride
  //getBuiltinCallee
  // unsigned int
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_11, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_48, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_47, other);
  }
  //isCallToStdMove
  arboretum_create_edge(obj, context_.data_model_.method_isCallToStdMove, context_.data_model_.arboretum_node_for(D->isCallToStdMove()));
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCapturedStmt(clang::CapturedStmt* D) {
  const Id* obj = context_.resolve(D);
  //getCapturedStmt
  {
    const Id* other = context_.resolve(D->getCapturedStmt());
    arboretum_create_edge(obj, context_.data_model_.method_getCapturedStmt, other);
  }
  //getCapturedDecl
  {
    const Id* other = context_.resolve(D->getCapturedDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCapturedDecl, other);
  }
  //getCapturedRegionKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getCapturedRegionKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getCapturedRegionKind, enum_value);
    }
  }
  //getCapturedRecordDecl
  {
    const Id* other = context_.resolve(D->getCapturedRecordDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getCapturedRecordDecl, other);
  }
  //captures
  // capture_const_range
  //capture_size
  // unsigned int
  //capture_inits
  // const_capture_init_range
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_49, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_48, other);
  }
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_47, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCaseStmt(clang::CaseStmt* D) {
  const Id* obj = context_.resolve(D);
  //caseStmtIsGNURange
  arboretum_create_edge(obj, context_.data_model_.method_caseStmtIsGNURange, context_.data_model_.arboretum_node_for(D->caseStmtIsGNURange()));
  //getCaseLoc
  {
    const Id* other = context_.source_model_.resolve(D->getCaseLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getCaseLoc, other);
  }
  //getEllipsisLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEllipsisLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEllipsisLoc_5, other);
  }
  //getLHS
  {
    const Id* other = context_.resolve(D->getLHS());
    arboretum_create_edge(obj, context_.data_model_.method_getLHS_4, other);
  }
  //getRHS
  {
    const Id* other = context_.resolve(D->getRHS());
    arboretum_create_edge(obj, context_.data_model_.method_getRHS_4, other);
  }
  //getSubStmt
  {
    const Id* other = context_.resolve(D->getSubStmt());
    arboretum_create_edge(obj, context_.data_model_.method_getSubStmt_1, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_50, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_49, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCastExpr(clang::CastExpr* D) {
  const Id* obj = context_.resolve(D);
  //getCastKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getCastKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getCastKind, enum_value);
    }
  }
  //getCastKindName
  //getSubExpr
  {
    const Id* other = context_.resolve(D->getSubExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getSubExpr_4, other);
  }
  //getSubExprAsWritten
  {
    const Id* other = context_.resolve(D->getSubExprAsWritten());
    arboretum_create_edge(obj, context_.data_model_.method_getSubExprAsWritten, other);
  }
  //getConversionFunction
  {
    const Id* other = context_.resolve(D->getConversionFunction());
    arboretum_create_edge(obj, context_.data_model_.method_getConversionFunction, other);
  }
  //path_empty
  arboretum_create_edge(obj, context_.data_model_.method_path_empty, context_.data_model_.arboretum_node_for(D->path_empty()));
  //path_size
  // unsigned int
  //path
  // llvm::iterator_range<path_const_iterator>
  //hasStoredFPFeatures
  arboretum_create_edge(obj, context_.data_model_.method_hasStoredFPFeatures_2, context_.data_model_.arboretum_node_for(D->hasStoredFPFeatures()));
  //getStoredFPFeatures
  // FPOptionsOverride
  //getFPFeatures
  // FPOptionsOverride
  //changesVolatileQualification
  arboretum_create_edge(obj, context_.data_model_.method_changesVolatileQualification, context_.data_model_.arboretum_node_for(D->changesVolatileQualification()));
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCharacterLiteral(clang::CharacterLiteral* D) {
  const Id* obj = context_.resolve(D);
  //getLocation
  {
    const Id* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.method_getLocation_6, other);
  }
  //getKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getKind_3, enum_value);
    }
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_51, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_50, other);
  }
  //getValue
  // unsigned int
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitChooseExpr(clang::ChooseExpr* D) {
  const Id* obj = context_.resolve(D);
  //isConditionTrue
  arboretum_create_edge(obj, context_.data_model_.method_isConditionTrue, context_.data_model_.arboretum_node_for(D->isConditionTrue()));
  //isConditionDependent
  arboretum_create_edge(obj, context_.data_model_.method_isConditionDependent, context_.data_model_.arboretum_node_for(D->isConditionDependent()));
  //getChosenSubExpr
  {
    const Id* other = context_.resolve(D->getChosenSubExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getChosenSubExpr, other);
  }
  //getCond
  {
    const Id* other = context_.resolve(D->getCond());
    arboretum_create_edge(obj, context_.data_model_.method_getCond_3, other);
  }
  //getLHS
  {
    const Id* other = context_.resolve(D->getLHS());
    arboretum_create_edge(obj, context_.data_model_.method_getLHS_5, other);
  }
  //getRHS
  {
    const Id* other = context_.resolve(D->getRHS());
    arboretum_create_edge(obj, context_.data_model_.method_getRHS_5, other);
  }
  //getBuiltinLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBuiltinLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBuiltinLoc_2, other);
  }
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_12, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_52, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_51, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCoawaitExpr(clang::CoawaitExpr* D) {
  const Id* obj = context_.resolve(D);
  //isImplicit
  arboretum_create_edge(obj, context_.data_model_.method_isImplicit_2, context_.data_model_.arboretum_node_for(D->isImplicit()));
  return true;
}

bool ArboretumASTVisitor::VisitCompoundAssignOperator(clang::CompoundAssignOperator* D) {
  const Id* obj = context_.resolve(D);
  //getComputationLHSType
  {
    const Id* other = context_.resolve(D->getComputationLHSType());
    arboretum_create_edge(obj, context_.data_model_.method_getComputationLHSType, other);
  }
  //getComputationResultType
  {
    const Id* other = context_.resolve(D->getComputationResultType());
    arboretum_create_edge(obj, context_.data_model_.method_getComputationResultType, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitCompoundLiteralExpr(clang::CompoundLiteralExpr* D) {
  const Id* obj = context_.resolve(D);
  //getInitializer
  {
    const Id* other = context_.resolve(D->getInitializer());
    arboretum_create_edge(obj, context_.data_model_.method_getInitializer_1, other);
  }
  //isFileScope
  arboretum_create_edge(obj, context_.data_model_.method_isFileScope, context_.data_model_.arboretum_node_for(D->isFileScope()));
  //getLParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getLParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getLParenLoc_4, other);
  }
  //getTypeSourceInfo
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_53, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_52, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCompoundStmt(clang::CompoundStmt* D) {
  const Id* obj = context_.resolve(D);
  //body_empty
  arboretum_create_edge(obj, context_.data_model_.method_body_empty, context_.data_model_.arboretum_node_for(D->body_empty()));
  //size
  // unsigned int
  //hasStoredFPFeatures
  arboretum_create_edge(obj, context_.data_model_.method_hasStoredFPFeatures_3, context_.data_model_.arboretum_node_for(D->hasStoredFPFeatures()));
  //getStoredFPFeatures
  // FPOptionsOverride
  //body
  // body_const_range
  //body_front
  {
    const Id* other = context_.resolve(D->body_front());
    arboretum_create_edge(obj, context_.data_model_.method_body_front, other);
  }
  //body_back
  {
    const Id* other = context_.resolve(D->body_back());
    arboretum_create_edge(obj, context_.data_model_.method_body_back, other);
  }
  //getStmtExprResult
  {
    const Id* other = context_.resolve(D->getStmtExprResult());
    arboretum_create_edge(obj, context_.data_model_.method_getStmtExprResult, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_54, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_53, other);
  }
  //getLBracLoc
  {
    const Id* other = context_.source_model_.resolve(D->getLBracLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getLBracLoc, other);
  }
  //getRBracLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRBracLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRBracLoc, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitConceptSpecializationExpr(clang::ConceptSpecializationExpr* D) {
  const Id* obj = context_.resolve(D);
  //getTemplateArguments
  // ArrayRef<TemplateArgument>
  //getConceptReference
  //getNamedConcept
  {
    const Id* other = context_.resolve(D->getNamedConcept());
    arboretum_create_edge(obj, context_.data_model_.method_getNamedConcept, other);
  }
  //hasExplicitTemplateArgs
  arboretum_create_edge(obj, context_.data_model_.method_hasExplicitTemplateArgs_1, context_.data_model_.arboretum_node_for(D->hasExplicitTemplateArgs()));
  //getConceptNameLoc
  {
    const Id* other = context_.source_model_.resolve(D->getConceptNameLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getConceptNameLoc, other);
  }
  //getTemplateArgsAsWritten
  //getNestedNameSpecifierLoc
  // const NestedNameSpecifierLoc &
  //getTemplateKWLoc
  {
    const Id* other = context_.source_model_.resolve(D->getTemplateKWLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getTemplateKWLoc, other);
  }
  //getFoundDecl
  {
    const Id* other = context_.resolve(D->getFoundDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getFoundDecl_1, other);
  }
  //getConceptNameInfo
  // const DeclarationNameInfo &
  //getSpecializationDecl
  {
    const Id* other = context_.resolve(D->getSpecializationDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getSpecializationDecl, other);
  }
  //getSatisfaction
  // const ASTConstraintSatisfaction &
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_55, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_54, other);
  }
  //getExprLoc
  {
    const Id* other = context_.source_model_.resolve(D->getExprLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getExprLoc_6, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitConditionalOperator(clang::ConditionalOperator* D) {
  const Id* obj = context_.resolve(D);
  //getCond
  {
    const Id* other = context_.resolve(D->getCond());
    arboretum_create_edge(obj, context_.data_model_.method_getCond_4, other);
  }
  //getTrueExpr
  {
    const Id* other = context_.resolve(D->getTrueExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getTrueExpr_2, other);
  }
  //getFalseExpr
  {
    const Id* other = context_.resolve(D->getFalseExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getFalseExpr_2, other);
  }
  //getLHS
  {
    const Id* other = context_.resolve(D->getLHS());
    arboretum_create_edge(obj, context_.data_model_.method_getLHS_6, other);
  }
  //getRHS
  {
    const Id* other = context_.resolve(D->getRHS());
    arboretum_create_edge(obj, context_.data_model_.method_getRHS_6, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_56, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_55, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitConstantExpr(clang::ConstantExpr* D) {
  const Id* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_57, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_56, other);
  }
  //getResultAPValueKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getResultAPValueKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getResultAPValueKind, enum_value);
    }
  }
  //getResultStorageKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getResultStorageKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getResultStorageKind, enum_value);
    }
  }
  //isImmediateInvocation
  arboretum_create_edge(obj, context_.data_model_.method_isImmediateInvocation, context_.data_model_.arboretum_node_for(D->isImmediateInvocation()));
  //hasAPValueResult
  arboretum_create_edge(obj, context_.data_model_.method_hasAPValueResult, context_.data_model_.arboretum_node_for(D->hasAPValueResult()));
  //getAPValueResult
  // APValue
  //getResultAsAPSInt
  // llvm::APSInt
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitContinueStmt(clang::ContinueStmt* D) {
  const Id* obj = context_.resolve(D);
  //getContinueLoc
  {
    const Id* other = context_.source_model_.resolve(D->getContinueLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getContinueLoc, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_58, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_57, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitConvertVectorExpr(clang::ConvertVectorExpr* D) {
  const Id* obj = context_.resolve(D);
  //getSrcExpr
  {
    const Id* other = context_.resolve(D->getSrcExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getSrcExpr_1, other);
  }
  //getTypeSourceInfo
  //getBuiltinLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBuiltinLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBuiltinLoc_3, other);
  }
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_13, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_59, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_58, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCoreturnStmt(clang::CoreturnStmt* D) {
  const Id* obj = context_.resolve(D);
  //getKeywordLoc
  {
    const Id* other = context_.source_model_.resolve(D->getKeywordLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getKeywordLoc, other);
  }
  //getOperand
  {
    const Id* other = context_.resolve(D->getOperand());
    arboretum_create_edge(obj, context_.data_model_.method_getOperand_1, other);
  }
  //getPromiseCall
  {
    const Id* other = context_.resolve(D->getPromiseCall());
    arboretum_create_edge(obj, context_.data_model_.method_getPromiseCall, other);
  }
  //isImplicit
  arboretum_create_edge(obj, context_.data_model_.method_isImplicit_3, context_.data_model_.arboretum_node_for(D->isImplicit()));
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_60, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_59, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCoroutineBodyStmt(clang::CoroutineBodyStmt* D) {
  const Id* obj = context_.resolve(D);
  //hasDependentPromiseType
  arboretum_create_edge(obj, context_.data_model_.method_hasDependentPromiseType, context_.data_model_.arboretum_node_for(D->hasDependentPromiseType()));
  //getBody
  {
    const Id* other = context_.resolve(D->getBody());
    arboretum_create_edge(obj, context_.data_model_.method_getBody_6, other);
  }
  //getPromiseDeclStmt
  {
    const Id* other = context_.resolve(D->getPromiseDeclStmt());
    arboretum_create_edge(obj, context_.data_model_.method_getPromiseDeclStmt, other);
  }
  //getPromiseDecl
  {
    const Id* other = context_.resolve(D->getPromiseDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getPromiseDecl, other);
  }
  //getInitSuspendStmt
  {
    const Id* other = context_.resolve(D->getInitSuspendStmt());
    arboretum_create_edge(obj, context_.data_model_.method_getInitSuspendStmt, other);
  }
  //getFinalSuspendStmt
  {
    const Id* other = context_.resolve(D->getFinalSuspendStmt());
    arboretum_create_edge(obj, context_.data_model_.method_getFinalSuspendStmt, other);
  }
  //getExceptionHandler
  {
    const Id* other = context_.resolve(D->getExceptionHandler());
    arboretum_create_edge(obj, context_.data_model_.method_getExceptionHandler, other);
  }
  //getFallthroughHandler
  {
    const Id* other = context_.resolve(D->getFallthroughHandler());
    arboretum_create_edge(obj, context_.data_model_.method_getFallthroughHandler, other);
  }
  //getAllocate
  {
    const Id* other = context_.resolve(D->getAllocate());
    arboretum_create_edge(obj, context_.data_model_.method_getAllocate, other);
  }
  //getDeallocate
  {
    const Id* other = context_.resolve(D->getDeallocate());
    arboretum_create_edge(obj, context_.data_model_.method_getDeallocate, other);
  }
  //getResultDecl
  {
    const Id* other = context_.resolve(D->getResultDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getResultDecl, other);
  }
  //getReturnValueInit
  {
    const Id* other = context_.resolve(D->getReturnValueInit());
    arboretum_create_edge(obj, context_.data_model_.method_getReturnValueInit, other);
  }
  //getReturnValue
  {
    const Id* other = context_.resolve(D->getReturnValue());
    arboretum_create_edge(obj, context_.data_model_.method_getReturnValue, other);
  }
  //getReturnStmt
  {
    const Id* other = context_.resolve(D->getReturnStmt());
    arboretum_create_edge(obj, context_.data_model_.method_getReturnStmt, other);
  }
  //getReturnStmtOnAllocFailure
  {
    const Id* other = context_.resolve(D->getReturnStmtOnAllocFailure());
    arboretum_create_edge(obj, context_.data_model_.method_getReturnStmtOnAllocFailure, other);
  }
  //getParamMoves
  // ArrayRef<const Stmt *>
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_61, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_60, other);
  }
  //children
  // const_child_range
  //childrenExclBody
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCoroutineSuspendExpr(clang::CoroutineSuspendExpr* D) {
  const Id* obj = context_.resolve(D);
  //getCommonExpr
  {
    const Id* other = context_.resolve(D->getCommonExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getCommonExpr_1, other);
  }
  //getOpaqueValue
  {
    const Id* other = context_.resolve(D->getOpaqueValue());
    arboretum_create_edge(obj, context_.data_model_.method_getOpaqueValue_1, other);
  }
  //getReadyExpr
  {
    const Id* other = context_.resolve(D->getReadyExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getReadyExpr, other);
  }
  //getSuspendExpr
  {
    const Id* other = context_.resolve(D->getSuspendExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getSuspendExpr, other);
  }
  //getResumeExpr
  {
    const Id* other = context_.resolve(D->getResumeExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getResumeExpr, other);
  }
  //getOperand
  {
    const Id* other = context_.resolve(D->getOperand());
    arboretum_create_edge(obj, context_.data_model_.method_getOperand_2, other);
  }
  //getKeywordLoc
  {
    const Id* other = context_.source_model_.resolve(D->getKeywordLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getKeywordLoc_1, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_62, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_61, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCoyieldExpr(clang::CoyieldExpr* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitDeclRefExpr(clang::DeclRefExpr* D) {
  const Id* obj = context_.resolve(D);
  //getDecl
  {
    const Id* other = context_.resolve(D->getDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getDecl_7, other);
  }
  //getNameInfo
  // DeclarationNameInfo
  //getLocation
  {
    const Id* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.method_getLocation_7, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_63, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_62, other);
  }
  //hasQualifier
  arboretum_create_edge(obj, context_.data_model_.method_hasQualifier_1, context_.data_model_.arboretum_node_for(D->hasQualifier()));
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getQualifier
  //getFoundDecl
  {
    const Id* other = context_.resolve(D->getFoundDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getFoundDecl_2, other);
  }
  //hasTemplateKWAndArgsInfo
  arboretum_create_edge(obj, context_.data_model_.method_hasTemplateKWAndArgsInfo, context_.data_model_.arboretum_node_for(D->hasTemplateKWAndArgsInfo()));
  //getTemplateKeywordLoc
  {
    const Id* other = context_.source_model_.resolve(D->getTemplateKeywordLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getTemplateKeywordLoc_3, other);
  }
  //getLAngleLoc
  {
    const Id* other = context_.source_model_.resolve(D->getLAngleLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getLAngleLoc_1, other);
  }
  //getRAngleLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRAngleLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRAngleLoc_1, other);
  }
  //hasTemplateKeyword
  arboretum_create_edge(obj, context_.data_model_.method_hasTemplateKeyword_1, context_.data_model_.arboretum_node_for(D->hasTemplateKeyword()));
  //hasExplicitTemplateArgs
  arboretum_create_edge(obj, context_.data_model_.method_hasExplicitTemplateArgs_2, context_.data_model_.arboretum_node_for(D->hasExplicitTemplateArgs()));
  //getTemplateArgs
  //getNumTemplateArgs
  // unsigned int
  //template_arguments
  // ArrayRef<TemplateArgumentLoc>
  //hadMultipleCandidates
  arboretum_create_edge(obj, context_.data_model_.method_hadMultipleCandidates_1, context_.data_model_.arboretum_node_for(D->hadMultipleCandidates()));
  //isNonOdrUse
  {
    const Id* enum_value = context_.data_model_.resolve(D->isNonOdrUse());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_isNonOdrUse, enum_value);
    }
  }
  //refersToEnclosingVariableOrCapture
  arboretum_create_edge(obj, context_.data_model_.method_refersToEnclosingVariableOrCapture, context_.data_model_.arboretum_node_for(D->refersToEnclosingVariableOrCapture()));
  //isImmediateEscalating
  arboretum_create_edge(obj, context_.data_model_.method_isImmediateEscalating_2, context_.data_model_.arboretum_node_for(D->isImmediateEscalating()));
  //isCapturedByCopyInLambdaWithExplicitObjectParameter
  arboretum_create_edge(obj, context_.data_model_.method_isCapturedByCopyInLambdaWithExplicitObjectParameter, context_.data_model_.arboretum_node_for(D->isCapturedByCopyInLambdaWithExplicitObjectParameter()));
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitDeclStmt(clang::DeclStmt* D) {
  const Id* obj = context_.resolve(D);
  //isSingleDecl
  arboretum_create_edge(obj, context_.data_model_.method_isSingleDecl, context_.data_model_.arboretum_node_for(D->isSingleDecl()));
  //getDeclGroup
  // const DeclGroupRef
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_63, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_64, other);
  }
  //children
  // const_child_range
  //decls
  // decl_const_range
  return true;
}

bool ArboretumASTVisitor::VisitDefaultStmt(clang::DefaultStmt* D) {
  const Id* obj = context_.resolve(D);
  //getSubStmt
  {
    const Id* other = context_.resolve(D->getSubStmt());
    arboretum_create_edge(obj, context_.data_model_.method_getSubStmt_2, other);
  }
  //getDefaultLoc
  {
    const Id* other = context_.source_model_.resolve(D->getDefaultLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getDefaultLoc_1, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_65, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_64, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitDependentCoawaitExpr(clang::DependentCoawaitExpr* D) {
  const Id* obj = context_.resolve(D);
  //getOperand
  {
    const Id* other = context_.resolve(D->getOperand());
    arboretum_create_edge(obj, context_.data_model_.method_getOperand_3, other);
  }
  //getOperatorCoawaitLookup
  {
    const Id* other = context_.resolve(D->getOperatorCoawaitLookup());
    arboretum_create_edge(obj, context_.data_model_.method_getOperatorCoawaitLookup, other);
  }
  //getKeywordLoc
  {
    const Id* other = context_.source_model_.resolve(D->getKeywordLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getKeywordLoc_2, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_66, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_65, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitDependentScopeDeclRefExpr(clang::DependentScopeDeclRefExpr* D) {
  const Id* obj = context_.resolve(D);
  //getNameInfo
  // const DeclarationNameInfo &
  //getDeclName
  // DeclarationName
  //getLocation
  {
    const Id* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.method_getLocation_8, other);
  }
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getQualifier
  //getTemplateKeywordLoc
  {
    const Id* other = context_.source_model_.resolve(D->getTemplateKeywordLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getTemplateKeywordLoc_4, other);
  }
  //getLAngleLoc
  {
    const Id* other = context_.source_model_.resolve(D->getLAngleLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getLAngleLoc_2, other);
  }
  //getRAngleLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRAngleLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRAngleLoc_2, other);
  }
  //hasTemplateKeyword
  arboretum_create_edge(obj, context_.data_model_.method_hasTemplateKeyword_2, context_.data_model_.arboretum_node_for(D->hasTemplateKeyword()));
  //hasExplicitTemplateArgs
  arboretum_create_edge(obj, context_.data_model_.method_hasExplicitTemplateArgs_3, context_.data_model_.arboretum_node_for(D->hasExplicitTemplateArgs()));
  //getTemplateArgs
  //getNumTemplateArgs
  // unsigned int
  //template_arguments
  // ArrayRef<TemplateArgumentLoc>
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_67, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_66, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitDesignatedInitExpr(clang::DesignatedInitExpr* D) {
  const Id* obj = context_.resolve(D);
  //size
  // unsigned int
  //designators
  // llvm::ArrayRef<Designator>
  //getEqualOrColonLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEqualOrColonLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEqualOrColonLoc, other);
  }
  //isDirectInit
  arboretum_create_edge(obj, context_.data_model_.method_isDirectInit_1, context_.data_model_.arboretum_node_for(D->isDirectInit()));
  //usesGNUSyntax
  arboretum_create_edge(obj, context_.data_model_.method_usesGNUSyntax, context_.data_model_.arboretum_node_for(D->usesGNUSyntax()));
  //getInit
  {
    const Id* other = context_.resolve(D->getInit());
    arboretum_create_edge(obj, context_.data_model_.method_getInit_3, other);
  }
  //getNumSubExprs
  // unsigned int
  //getDesignatorsSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getDesignatorsSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getDesignatorsSourceRange, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_68, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_67, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitDesignatedInitUpdateExpr(clang::DesignatedInitUpdateExpr* D) {
  const Id* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_69, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_68, other);
  }
  //getBase
  {
    const Id* other = context_.resolve(D->getBase());
    arboretum_create_edge(obj, context_.data_model_.method_getBase_2, other);
  }
  //getUpdater
  {
    const Id* other = context_.resolve(D->getUpdater());
    arboretum_create_edge(obj, context_.data_model_.method_getUpdater, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitDoStmt(clang::DoStmt* D) {
  const Id* obj = context_.resolve(D);
  //getCond
  {
    const Id* other = context_.resolve(D->getCond());
    arboretum_create_edge(obj, context_.data_model_.method_getCond_5, other);
  }
  //getBody
  {
    const Id* other = context_.resolve(D->getBody());
    arboretum_create_edge(obj, context_.data_model_.method_getBody_7, other);
  }
  //getDoLoc
  {
    const Id* other = context_.source_model_.resolve(D->getDoLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getDoLoc, other);
  }
  //getWhileLoc
  {
    const Id* other = context_.source_model_.resolve(D->getWhileLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getWhileLoc, other);
  }
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_14, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_70, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_69, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitExplicitCastExpr(clang::ExplicitCastExpr* D) {
  const Id* obj = context_.resolve(D);
  //getTypeInfoAsWritten
  //getTypeAsWritten
  {
    const Id* other = context_.resolve(D->getTypeAsWritten());
    arboretum_create_edge(obj, context_.data_model_.method_getTypeAsWritten_3, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitExpr(clang::Expr* D) {
  const Id* obj = context_.resolve(D);
  //getType
  {
    const Id* other = context_.resolve(D->getType());
    arboretum_create_edge(obj, context_.data_model_.method_getType_1, other);
  }
  //getDependence
  {
    const Id* enum_value = context_.data_model_.resolve(D->getDependence());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getDependence_1, enum_value);
    }
  }
  //isValueDependent
  arboretum_create_edge(obj, context_.data_model_.method_isValueDependent, context_.data_model_.arboretum_node_for(D->isValueDependent()));
  //isTypeDependent
  arboretum_create_edge(obj, context_.data_model_.method_isTypeDependent, context_.data_model_.arboretum_node_for(D->isTypeDependent()));
  //isInstantiationDependent
  arboretum_create_edge(obj, context_.data_model_.method_isInstantiationDependent, context_.data_model_.arboretum_node_for(D->isInstantiationDependent()));
  //containsUnexpandedParameterPack
  arboretum_create_edge(obj, context_.data_model_.method_containsUnexpandedParameterPack_1, context_.data_model_.arboretum_node_for(D->containsUnexpandedParameterPack()));
  //containsErrors
  arboretum_create_edge(obj, context_.data_model_.method_containsErrors_1, context_.data_model_.arboretum_node_for(D->containsErrors()));
  //isLValue
  arboretum_create_edge(obj, context_.data_model_.method_isLValue, context_.data_model_.arboretum_node_for(D->isLValue()));
  //isPRValue
  arboretum_create_edge(obj, context_.data_model_.method_isPRValue, context_.data_model_.arboretum_node_for(D->isPRValue()));
  //isXValue
  arboretum_create_edge(obj, context_.data_model_.method_isXValue, context_.data_model_.arboretum_node_for(D->isXValue()));
  //isGLValue
  arboretum_create_edge(obj, context_.data_model_.method_isGLValue, context_.data_model_.arboretum_node_for(D->isGLValue()));
  //getValueKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getValueKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getValueKind, enum_value);
    }
  }
  //getObjectKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getObjectKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getObjectKind, enum_value);
    }
  }
  //isOrdinaryOrBitFieldObject
  arboretum_create_edge(obj, context_.data_model_.method_isOrdinaryOrBitFieldObject, context_.data_model_.arboretum_node_for(D->isOrdinaryOrBitFieldObject()));
  //refersToBitField
  arboretum_create_edge(obj, context_.data_model_.method_refersToBitField, context_.data_model_.arboretum_node_for(D->refersToBitField()));
  //getSourceBitField
  {
    const Id* other = context_.resolve(D->getSourceBitField());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceBitField, other);
  }
  //getReferencedDeclOfCallee
  {
    const Id* other = context_.resolve(D->getReferencedDeclOfCallee());
    arboretum_create_edge(obj, context_.data_model_.method_getReferencedDeclOfCallee, other);
  }
  //getObjCProperty
  {
    const Id* other = context_.resolve(D->getObjCProperty());
    arboretum_create_edge(obj, context_.data_model_.method_getObjCProperty, other);
  }
  //isObjCSelfExpr
  arboretum_create_edge(obj, context_.data_model_.method_isObjCSelfExpr, context_.data_model_.arboretum_node_for(D->isObjCSelfExpr()));
  //refersToVectorElement
  arboretum_create_edge(obj, context_.data_model_.method_refersToVectorElement, context_.data_model_.arboretum_node_for(D->refersToVectorElement()));
  //refersToMatrixElement
  arboretum_create_edge(obj, context_.data_model_.method_refersToMatrixElement, context_.data_model_.arboretum_node_for(D->refersToMatrixElement()));
  //refersToGlobalRegisterVar
  arboretum_create_edge(obj, context_.data_model_.method_refersToGlobalRegisterVar, context_.data_model_.arboretum_node_for(D->refersToGlobalRegisterVar()));
  //IgnoreImpCasts
  {
    const Id* other = context_.resolve(D->IgnoreImpCasts());
    arboretum_create_edge(obj, context_.data_model_.method_IgnoreImpCasts, other);
  }
  //IgnoreCasts
  {
    const Id* other = context_.resolve(D->IgnoreCasts());
    arboretum_create_edge(obj, context_.data_model_.method_IgnoreCasts, other);
  }
  //IgnoreImplicit
  {
    const Id* other = context_.resolve(D->IgnoreImplicit());
    arboretum_create_edge(obj, context_.data_model_.method_IgnoreImplicit, other);
  }
  //IgnoreImplicitAsWritten
  {
    const Id* other = context_.resolve(D->IgnoreImplicitAsWritten());
    arboretum_create_edge(obj, context_.data_model_.method_IgnoreImplicitAsWritten, other);
  }
  //IgnoreParens
  {
    const Id* other = context_.resolve(D->IgnoreParens());
    arboretum_create_edge(obj, context_.data_model_.method_IgnoreParens, other);
  }
  //IgnoreParenImpCasts
  {
    const Id* other = context_.resolve(D->IgnoreParenImpCasts());
    arboretum_create_edge(obj, context_.data_model_.method_IgnoreParenImpCasts, other);
  }
  //IgnoreParenCasts
  {
    const Id* other = context_.resolve(D->IgnoreParenCasts());
    arboretum_create_edge(obj, context_.data_model_.method_IgnoreParenCasts, other);
  }
  //IgnoreConversionOperatorSingleStep
  {
    const Id* other = context_.resolve(D->IgnoreConversionOperatorSingleStep());
    arboretum_create_edge(obj, context_.data_model_.method_IgnoreConversionOperatorSingleStep, other);
  }
  //IgnoreParenLValueCasts
  {
    const Id* other = context_.resolve(D->IgnoreParenLValueCasts());
    arboretum_create_edge(obj, context_.data_model_.method_IgnoreParenLValueCasts, other);
  }
  //IgnoreParenBaseCasts
  {
    const Id* other = context_.resolve(D->IgnoreParenBaseCasts());
    arboretum_create_edge(obj, context_.data_model_.method_IgnoreParenBaseCasts, other);
  }
  //isDefaultArgument
  arboretum_create_edge(obj, context_.data_model_.method_isDefaultArgument, context_.data_model_.arboretum_node_for(D->isDefaultArgument()));
  //isImplicitCXXThis
  arboretum_create_edge(obj, context_.data_model_.method_isImplicitCXXThis, context_.data_model_.arboretum_node_for(D->isImplicitCXXThis()));
  //skipRValueSubobjectAdjustments
  {
    const Id* other = context_.resolve(D->skipRValueSubobjectAdjustments());
    arboretum_create_edge(obj, context_.data_model_.method_skipRValueSubobjectAdjustments, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitExprWithCleanups(clang::ExprWithCleanups* D) {
  const Id* obj = context_.resolve(D);
  //getObjects
  // ArrayRef<CleanupObject>
  //getNumObjects
  // unsigned int
  //cleanupsHaveSideEffects
  arboretum_create_edge(obj, context_.data_model_.method_cleanupsHaveSideEffects, context_.data_model_.arboretum_node_for(D->cleanupsHaveSideEffects()));
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_71, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_70, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitExpressionTraitExpr(clang::ExpressionTraitExpr* D) {
  const Id* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_72, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_71, other);
  }
  //getTrait
  {
    const Id* enum_value = context_.data_model_.resolve(D->getTrait());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getTrait_1, enum_value);
    }
  }
  //getQueriedExpression
  {
    const Id* other = context_.resolve(D->getQueriedExpression());
    arboretum_create_edge(obj, context_.data_model_.method_getQueriedExpression, other);
  }
  //getValue
  arboretum_create_edge(obj, context_.data_model_.method_getValue_7, context_.data_model_.arboretum_node_for(D->getValue()));
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitExtVectorElementExpr(clang::ExtVectorElementExpr* D) {
  const Id* obj = context_.resolve(D);
  //getBase
  {
    const Id* other = context_.resolve(D->getBase());
    arboretum_create_edge(obj, context_.data_model_.method_getBase_3, other);
  }
  //getAccessor
  // IdentifierInfo &
  //getAccessorLoc
  {
    const Id* other = context_.source_model_.resolve(D->getAccessorLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getAccessorLoc, other);
  }
  //getNumElements
  // unsigned int
  //containsDuplicateElements
  arboretum_create_edge(obj, context_.data_model_.method_containsDuplicateElements, context_.data_model_.arboretum_node_for(D->containsDuplicateElements()));
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_73, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_72, other);
  }
  //isArrow
  arboretum_create_edge(obj, context_.data_model_.method_isArrow_2, context_.data_model_.arboretum_node_for(D->isArrow()));
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitFixedPointLiteral(clang::FixedPointLiteral* D) {
  const Id* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_74, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_73, other);
  }
  //getLocation
  {
    const Id* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.method_getLocation_9, other);
  }
  //getScale
  // unsigned int
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitFloatingLiteral(clang::FloatingLiteral* D) {
  const Id* obj = context_.resolve(D);
  //getValue
  // llvm::APFloat
  //getRawSemantics
  {
    const Id* enum_value = context_.data_model_.resolve(D->getRawSemantics());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getRawSemantics, enum_value);
    }
  }
  //getSemantics
  // const llvm::fltSemantics &
  //isExact
  arboretum_create_edge(obj, context_.data_model_.method_isExact, context_.data_model_.arboretum_node_for(D->isExact()));
  //getValueAsApproximateDouble
  // double
  //getLocation
  {
    const Id* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.method_getLocation_10, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_75, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_74, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitForStmt(clang::ForStmt* D) {
  const Id* obj = context_.resolve(D);
  //getConditionVariable
  {
    const Id* other = context_.resolve(D->getConditionVariable());
    arboretum_create_edge(obj, context_.data_model_.method_getConditionVariable, other);
  }
  //getConditionVariableDeclStmt
  {
    const Id* other = context_.resolve(D->getConditionVariableDeclStmt());
    arboretum_create_edge(obj, context_.data_model_.method_getConditionVariableDeclStmt, other);
  }
  //getInit
  {
    const Id* other = context_.resolve(D->getInit());
    arboretum_create_edge(obj, context_.data_model_.method_getInit_4, other);
  }
  //getCond
  {
    const Id* other = context_.resolve(D->getCond());
    arboretum_create_edge(obj, context_.data_model_.method_getCond_6, other);
  }
  //getInc
  {
    const Id* other = context_.resolve(D->getInc());
    arboretum_create_edge(obj, context_.data_model_.method_getInc_1, other);
  }
  //getBody
  {
    const Id* other = context_.resolve(D->getBody());
    arboretum_create_edge(obj, context_.data_model_.method_getBody_8, other);
  }
  //getForLoc
  {
    const Id* other = context_.source_model_.resolve(D->getForLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getForLoc_1, other);
  }
  //getLParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getLParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getLParenLoc_5, other);
  }
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_15, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_76, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_75, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitFullExpr(clang::FullExpr* D) {
  const Id* obj = context_.resolve(D);
  //getSubExpr
  {
    const Id* other = context_.resolve(D->getSubExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getSubExpr_5, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitFunctionParmPackExpr(clang::FunctionParmPackExpr* D) {
  const Id* obj = context_.resolve(D);
  //getParameterPack
  {
    const Id* other = context_.resolve(D->getParameterPack());
    arboretum_create_edge(obj, context_.data_model_.method_getParameterPack, other);
  }
  //getParameterPackLocation
  {
    const Id* other = context_.source_model_.resolve(D->getParameterPackLocation());
    arboretum_create_edge(obj, context_.data_model_.method_getParameterPackLocation, other);
  }
  //getNumExpansions
  // unsigned int
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_77, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_76, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitGCCAsmStmt(clang::GCCAsmStmt* D) {
  const Id* obj = context_.resolve(D);
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_16, other);
  }
  //getAsmString
  {
    const Id* other = context_.resolve(D->getAsmString());
    arboretum_create_edge(obj, context_.data_model_.method_getAsmString_1, other);
  }
  //isAsmGoto
  arboretum_create_edge(obj, context_.data_model_.method_isAsmGoto, context_.data_model_.arboretum_node_for(D->isAsmGoto()));
  //getNumLabels
  // unsigned int
  //labels
  // labels_const_range
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_78, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_77, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitGNUNullExpr(clang::GNUNullExpr* D) {
  const Id* obj = context_.resolve(D);
  //getTokenLocation
  {
    const Id* other = context_.source_model_.resolve(D->getTokenLocation());
    arboretum_create_edge(obj, context_.data_model_.method_getTokenLocation, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_79, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_78, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitGenericSelectionExpr(clang::GenericSelectionExpr* D) {
  const Id* obj = context_.resolve(D);
  //getNumAssocs
  // unsigned int
  //getResultIndex
  // unsigned int
  //isResultDependent
  arboretum_create_edge(obj, context_.data_model_.method_isResultDependent, context_.data_model_.arboretum_node_for(D->isResultDependent()));
  //isExprPredicate
  arboretum_create_edge(obj, context_.data_model_.method_isExprPredicate, context_.data_model_.arboretum_node_for(D->isExprPredicate()));
  //isTypePredicate
  arboretum_create_edge(obj, context_.data_model_.method_isTypePredicate, context_.data_model_.arboretum_node_for(D->isTypePredicate()));
  //getControllingExpr
  {
    const Id* other = context_.resolve(D->getControllingExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getControllingExpr, other);
  }
  //getControllingType
  //getResultExpr
  {
    const Id* other = context_.resolve(D->getResultExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getResultExpr, other);
  }
  //getAssocExprs
  // ArrayRef<Expr *>
  //getAssocTypeSourceInfos
  // ArrayRef<TypeSourceInfo *>
  //associations
  // const_association_range
  //getGenericLoc
  {
    const Id* other = context_.source_model_.resolve(D->getGenericLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getGenericLoc, other);
  }
  //getDefaultLoc
  {
    const Id* other = context_.source_model_.resolve(D->getDefaultLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getDefaultLoc_2, other);
  }
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_17, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_80, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_79, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitGotoStmt(clang::GotoStmt* D) {
  const Id* obj = context_.resolve(D);
  //getLabel
  {
    const Id* other = context_.resolve(D->getLabel());
    arboretum_create_edge(obj, context_.data_model_.method_getLabel_1, other);
  }
  //getGotoLoc
  {
    const Id* other = context_.source_model_.resolve(D->getGotoLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getGotoLoc, other);
  }
  //getLabelLoc
  {
    const Id* other = context_.source_model_.resolve(D->getLabelLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getLabelLoc_1, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_81, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_80, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitIfStmt(clang::IfStmt* D) {
  const Id* obj = context_.resolve(D);
  //hasInitStorage
  arboretum_create_edge(obj, context_.data_model_.method_hasInitStorage, context_.data_model_.arboretum_node_for(D->hasInitStorage()));
  //hasVarStorage
  arboretum_create_edge(obj, context_.data_model_.method_hasVarStorage, context_.data_model_.arboretum_node_for(D->hasVarStorage()));
  //hasElseStorage
  arboretum_create_edge(obj, context_.data_model_.method_hasElseStorage, context_.data_model_.arboretum_node_for(D->hasElseStorage()));
  //getCond
  {
    const Id* other = context_.resolve(D->getCond());
    arboretum_create_edge(obj, context_.data_model_.method_getCond_7, other);
  }
  //getThen
  {
    const Id* other = context_.resolve(D->getThen());
    arboretum_create_edge(obj, context_.data_model_.method_getThen, other);
  }
  //getElse
  {
    const Id* other = context_.resolve(D->getElse());
    arboretum_create_edge(obj, context_.data_model_.method_getElse, other);
  }
  //getConditionVariable
  {
    const Id* other = context_.resolve(D->getConditionVariable());
    arboretum_create_edge(obj, context_.data_model_.method_getConditionVariable_1, other);
  }
  //getConditionVariableDeclStmt
  {
    const Id* other = context_.resolve(D->getConditionVariableDeclStmt());
    arboretum_create_edge(obj, context_.data_model_.method_getConditionVariableDeclStmt_1, other);
  }
  //getInit
  {
    const Id* other = context_.resolve(D->getInit());
    arboretum_create_edge(obj, context_.data_model_.method_getInit_5, other);
  }
  //getIfLoc
  {
    const Id* other = context_.source_model_.resolve(D->getIfLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getIfLoc, other);
  }
  //getElseLoc
  {
    const Id* other = context_.source_model_.resolve(D->getElseLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getElseLoc, other);
  }
  //isConsteval
  arboretum_create_edge(obj, context_.data_model_.method_isConsteval_1, context_.data_model_.arboretum_node_for(D->isConsteval()));
  //isNonNegatedConsteval
  arboretum_create_edge(obj, context_.data_model_.method_isNonNegatedConsteval, context_.data_model_.arboretum_node_for(D->isNonNegatedConsteval()));
  //isNegatedConsteval
  arboretum_create_edge(obj, context_.data_model_.method_isNegatedConsteval, context_.data_model_.arboretum_node_for(D->isNegatedConsteval()));
  //isConstexpr
  arboretum_create_edge(obj, context_.data_model_.method_isConstexpr_2, context_.data_model_.arboretum_node_for(D->isConstexpr()));
  //getStatementKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getStatementKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getStatementKind, enum_value);
    }
  }
  //isObjCAvailabilityCheck
  arboretum_create_edge(obj, context_.data_model_.method_isObjCAvailabilityCheck, context_.data_model_.arboretum_node_for(D->isObjCAvailabilityCheck()));
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_82, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_81, other);
  }
  //getLParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getLParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getLParenLoc_6, other);
  }
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_18, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitImaginaryLiteral(clang::ImaginaryLiteral* D) {
  const Id* obj = context_.resolve(D);
  //getSubExpr
  {
    const Id* other = context_.resolve(D->getSubExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getSubExpr_6, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_83, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_82, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitImplicitCastExpr(clang::ImplicitCastExpr* D) {
  const Id* obj = context_.resolve(D);
  //isPartOfExplicitCast
  arboretum_create_edge(obj, context_.data_model_.method_isPartOfExplicitCast, context_.data_model_.arboretum_node_for(D->isPartOfExplicitCast()));
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_84, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_83, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitImplicitValueInitExpr(clang::ImplicitValueInitExpr* D) {
  const Id* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_85, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_84, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitIndirectGotoStmt(clang::IndirectGotoStmt* D) {
  const Id* obj = context_.resolve(D);
  //getGotoLoc
  {
    const Id* other = context_.source_model_.resolve(D->getGotoLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getGotoLoc_1, other);
  }
  //getStarLoc
  {
    const Id* other = context_.source_model_.resolve(D->getStarLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getStarLoc, other);
  }
  //getTarget
  {
    const Id* other = context_.resolve(D->getTarget());
    arboretum_create_edge(obj, context_.data_model_.method_getTarget, other);
  }
  //getConstantTarget
  {
    const Id* other = context_.resolve(D->getConstantTarget());
    arboretum_create_edge(obj, context_.data_model_.method_getConstantTarget, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_86, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_85, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitInitListExpr(clang::InitListExpr* D) {
  const Id* obj = context_.resolve(D);
  //getNumInits
  // unsigned int
  //inits
  // ArrayRef<Expr *>
  //getArrayFiller
  {
    const Id* other = context_.resolve(D->getArrayFiller());
    arboretum_create_edge(obj, context_.data_model_.method_getArrayFiller_1, other);
  }
  //hasArrayFiller
  arboretum_create_edge(obj, context_.data_model_.method_hasArrayFiller, context_.data_model_.arboretum_node_for(D->hasArrayFiller()));
  //hasDesignatedInit
  arboretum_create_edge(obj, context_.data_model_.method_hasDesignatedInit, context_.data_model_.arboretum_node_for(D->hasDesignatedInit()));
  //getInitializedFieldInUnion
  {
    const Id* other = context_.resolve(D->getInitializedFieldInUnion());
    arboretum_create_edge(obj, context_.data_model_.method_getInitializedFieldInUnion_1, other);
  }
  //isExplicit
  arboretum_create_edge(obj, context_.data_model_.method_isExplicit_3, context_.data_model_.arboretum_node_for(D->isExplicit()));
  //isStringLiteralInit
  arboretum_create_edge(obj, context_.data_model_.method_isStringLiteralInit, context_.data_model_.arboretum_node_for(D->isStringLiteralInit()));
  //isTransparent
  arboretum_create_edge(obj, context_.data_model_.method_isTransparent, context_.data_model_.arboretum_node_for(D->isTransparent()));
  //getLBraceLoc
  {
    const Id* other = context_.source_model_.resolve(D->getLBraceLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getLBraceLoc_1, other);
  }
  //getRBraceLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRBraceLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRBraceLoc_4, other);
  }
  //isSemanticForm
  arboretum_create_edge(obj, context_.data_model_.method_isSemanticForm, context_.data_model_.arboretum_node_for(D->isSemanticForm()));
  //getSemanticForm
  {
    const Id* other = context_.resolve(D->getSemanticForm());
    arboretum_create_edge(obj, context_.data_model_.method_getSemanticForm_1, other);
  }
  //isSyntacticForm
  arboretum_create_edge(obj, context_.data_model_.method_isSyntacticForm, context_.data_model_.arboretum_node_for(D->isSyntacticForm()));
  //getSyntacticForm
  {
    const Id* other = context_.resolve(D->getSyntacticForm());
    arboretum_create_edge(obj, context_.data_model_.method_getSyntacticForm, other);
  }
  //hadArrayRangeDesignator
  arboretum_create_edge(obj, context_.data_model_.method_hadArrayRangeDesignator, context_.data_model_.arboretum_node_for(D->hadArrayRangeDesignator()));
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_87, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_86, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitIntegerLiteral(clang::IntegerLiteral* D) {
  const Id* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_88, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_87, other);
  }
  //getLocation
  {
    const Id* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.method_getLocation_11, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitLabelStmt(clang::LabelStmt* D) {
  const Id* obj = context_.resolve(D);
  //getIdentLoc
  {
    const Id* other = context_.source_model_.resolve(D->getIdentLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getIdentLoc, other);
  }
  //getDecl
  {
    const Id* other = context_.resolve(D->getDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getDecl_8, other);
  }
  //getName
  //getSubStmt
  {
    const Id* other = context_.resolve(D->getSubStmt());
    arboretum_create_edge(obj, context_.data_model_.method_getSubStmt_3, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_89, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_88, other);
  }
  //children
  // const_child_range
  //isSideEntry
  arboretum_create_edge(obj, context_.data_model_.method_isSideEntry, context_.data_model_.arboretum_node_for(D->isSideEntry()));
  return true;
}

bool ArboretumASTVisitor::VisitLambdaExpr(clang::LambdaExpr* D) {
  const Id* obj = context_.resolve(D);
  //getCaptureDefault
  {
    const Id* enum_value = context_.data_model_.resolve(D->getCaptureDefault());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getCaptureDefault, enum_value);
    }
  }
  //getCaptureDefaultLoc
  {
    const Id* other = context_.source_model_.resolve(D->getCaptureDefaultLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getCaptureDefaultLoc, other);
  }
  //captures
  // capture_range
  //capture_size
  // unsigned int
  //explicit_captures
  // capture_range
  //implicit_captures
  // capture_range
  //capture_inits
  // llvm::iterator_range<const_capture_init_iterator>
  //getIntroducerRange
  {
    const Id* other = context_.source_model_.resolve(D->getIntroducerRange());
    arboretum_create_edge(obj, context_.data_model_.method_getIntroducerRange, other);
  }
  //getLambdaClass
  {
    const Id* other = context_.resolve(D->getLambdaClass());
    arboretum_create_edge(obj, context_.data_model_.method_getLambdaClass, other);
  }
  //getCallOperator
  {
    const Id* other = context_.resolve(D->getCallOperator());
    arboretum_create_edge(obj, context_.data_model_.method_getCallOperator, other);
  }
  //getDependentCallOperator
  {
    const Id* other = context_.resolve(D->getDependentCallOperator());
    arboretum_create_edge(obj, context_.data_model_.method_getDependentCallOperator, other);
  }
  //getTemplateParameterList
  //getExplicitTemplateParameters
  // ArrayRef<NamedDecl *>
  //getTrailingRequiresClause
  {
    const Id* other = context_.resolve(D->getTrailingRequiresClause());
    arboretum_create_edge(obj, context_.data_model_.method_getTrailingRequiresClause_1, other);
  }
  //isGenericLambda
  arboretum_create_edge(obj, context_.data_model_.method_isGenericLambda_1, context_.data_model_.arboretum_node_for(D->isGenericLambda()));
  //getBody
  {
    const Id* other = context_.resolve(D->getBody());
    arboretum_create_edge(obj, context_.data_model_.method_getBody_9, other);
  }
  //getCompoundStmtBody
  {
    const Id* other = context_.resolve(D->getCompoundStmtBody());
    arboretum_create_edge(obj, context_.data_model_.method_getCompoundStmtBody, other);
  }
  //isMutable
  arboretum_create_edge(obj, context_.data_model_.method_isMutable_1, context_.data_model_.arboretum_node_for(D->isMutable()));
  //hasExplicitParameters
  arboretum_create_edge(obj, context_.data_model_.method_hasExplicitParameters, context_.data_model_.arboretum_node_for(D->hasExplicitParameters()));
  //hasExplicitResultType
  arboretum_create_edge(obj, context_.data_model_.method_hasExplicitResultType, context_.data_model_.arboretum_node_for(D->hasExplicitResultType()));
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_90, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_89, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitMSAsmStmt(clang::MSAsmStmt* D) {
  const Id* obj = context_.resolve(D);
  //getLBraceLoc
  {
    const Id* other = context_.source_model_.resolve(D->getLBraceLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getLBraceLoc_2, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_90, other);
  }
  //hasBraces
  arboretum_create_edge(obj, context_.data_model_.method_hasBraces_2, context_.data_model_.arboretum_node_for(D->hasBraces()));
  //getAsmString
  // StringRef
  //getAllConstraints
  // ArrayRef<StringRef>
  //getClobbers
  // ArrayRef<StringRef>
  //getAllExprs
  // ArrayRef<Expr *>
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_91, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitMSDependentExistsStmt(clang::MSDependentExistsStmt* D) {
  const Id* obj = context_.resolve(D);
  //getKeywordLoc
  {
    const Id* other = context_.source_model_.resolve(D->getKeywordLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getKeywordLoc_3, other);
  }
  //isIfExists
  arboretum_create_edge(obj, context_.data_model_.method_isIfExists, context_.data_model_.arboretum_node_for(D->isIfExists()));
  //isIfNotExists
  arboretum_create_edge(obj, context_.data_model_.method_isIfNotExists, context_.data_model_.arboretum_node_for(D->isIfNotExists()));
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getNameInfo
  // DeclarationNameInfo
  //getSubStmt
  {
    const Id* other = context_.resolve(D->getSubStmt());
    arboretum_create_edge(obj, context_.data_model_.method_getSubStmt_4, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_92, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_91, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitMSPropertyRefExpr(clang::MSPropertyRefExpr* D) {
  const Id* obj = context_.resolve(D);
  //getSourceRange
  {
    const Id* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_48, other);
  }
  //isImplicitAccess
  arboretum_create_edge(obj, context_.data_model_.method_isImplicitAccess_1, context_.data_model_.arboretum_node_for(D->isImplicitAccess()));
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_93, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_92, other);
  }
  //children
  // const_child_range
  //getBaseExpr
  {
    const Id* other = context_.resolve(D->getBaseExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getBaseExpr, other);
  }
  //getPropertyDecl
  {
    const Id* other = context_.resolve(D->getPropertyDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getPropertyDecl, other);
  }
  //isArrow
  arboretum_create_edge(obj, context_.data_model_.method_isArrow_3, context_.data_model_.arboretum_node_for(D->isArrow()));
  //getMemberLoc
  {
    const Id* other = context_.source_model_.resolve(D->getMemberLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getMemberLoc_1, other);
  }
  //getQualifierLoc
  // NestedNameSpecifierLoc
  return true;
}

bool ArboretumASTVisitor::VisitMSPropertySubscriptExpr(clang::MSPropertySubscriptExpr* D) {
  const Id* obj = context_.resolve(D);
  //getBase
  {
    const Id* other = context_.resolve(D->getBase());
    arboretum_create_edge(obj, context_.data_model_.method_getBase_4, other);
  }
  //getIdx
  {
    const Id* other = context_.resolve(D->getIdx());
    arboretum_create_edge(obj, context_.data_model_.method_getIdx_1, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_94, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_93, other);
  }
  //getRBracketLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRBracketLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRBracketLoc_3, other);
  }
  //getExprLoc
  {
    const Id* other = context_.source_model_.resolve(D->getExprLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getExprLoc_7, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitMaterializeTemporaryExpr(clang::MaterializeTemporaryExpr* D) {
  const Id* obj = context_.resolve(D);
  //getSubExpr
  {
    const Id* other = context_.resolve(D->getSubExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getSubExpr_7, other);
  }
  //getStorageDuration
  {
    const Id* enum_value = context_.data_model_.resolve(D->getStorageDuration());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getStorageDuration_2, enum_value);
    }
  }
  //getLifetimeExtendedTemporaryDecl
  {
    const Id* other = context_.resolve(D->getLifetimeExtendedTemporaryDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getLifetimeExtendedTemporaryDecl, other);
  }
  //getExtendingDecl
  {
    const Id* other = context_.resolve(D->getExtendingDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getExtendingDecl_1, other);
  }
  //getManglingNumber
  // unsigned int
  //isBoundToLvalueReference
  arboretum_create_edge(obj, context_.data_model_.method_isBoundToLvalueReference, context_.data_model_.arboretum_node_for(D->isBoundToLvalueReference()));
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_95, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_94, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitMatrixSubscriptExpr(clang::MatrixSubscriptExpr* D) {
  const Id* obj = context_.resolve(D);
  //isIncomplete
  arboretum_create_edge(obj, context_.data_model_.method_isIncomplete, context_.data_model_.arboretum_node_for(D->isIncomplete()));
  //getBase
  {
    const Id* other = context_.resolve(D->getBase());
    arboretum_create_edge(obj, context_.data_model_.method_getBase_5, other);
  }
  //getRowIdx
  {
    const Id* other = context_.resolve(D->getRowIdx());
    arboretum_create_edge(obj, context_.data_model_.method_getRowIdx, other);
  }
  //getColumnIdx
  {
    const Id* other = context_.resolve(D->getColumnIdx());
    arboretum_create_edge(obj, context_.data_model_.method_getColumnIdx, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_96, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_95, other);
  }
  //getExprLoc
  {
    const Id* other = context_.source_model_.resolve(D->getExprLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getExprLoc_8, other);
  }
  //getRBracketLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRBracketLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRBracketLoc_4, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitMemberExpr(clang::MemberExpr* D) {
  const Id* obj = context_.resolve(D);
  //getBase
  {
    const Id* other = context_.resolve(D->getBase());
    arboretum_create_edge(obj, context_.data_model_.method_getBase_6, other);
  }
  //getMemberDecl
  {
    const Id* other = context_.resolve(D->getMemberDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getMemberDecl, other);
  }
  //getFoundDecl
  // DeclAccessPair
  //hasQualifier
  arboretum_create_edge(obj, context_.data_model_.method_hasQualifier_2, context_.data_model_.arboretum_node_for(D->hasQualifier()));
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getQualifier
  //getTemplateKeywordLoc
  {
    const Id* other = context_.source_model_.resolve(D->getTemplateKeywordLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getTemplateKeywordLoc_5, other);
  }
  //getLAngleLoc
  {
    const Id* other = context_.source_model_.resolve(D->getLAngleLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getLAngleLoc_3, other);
  }
  //getRAngleLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRAngleLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRAngleLoc_3, other);
  }
  //hasTemplateKeyword
  arboretum_create_edge(obj, context_.data_model_.method_hasTemplateKeyword_3, context_.data_model_.arboretum_node_for(D->hasTemplateKeyword()));
  //hasExplicitTemplateArgs
  arboretum_create_edge(obj, context_.data_model_.method_hasExplicitTemplateArgs_4, context_.data_model_.arboretum_node_for(D->hasExplicitTemplateArgs()));
  //getTemplateArgs
  //getNumTemplateArgs
  // unsigned int
  //template_arguments
  // ArrayRef<TemplateArgumentLoc>
  //getMemberNameInfo
  // DeclarationNameInfo
  //getOperatorLoc
  {
    const Id* other = context_.source_model_.resolve(D->getOperatorLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getOperatorLoc_6, other);
  }
  //isArrow
  arboretum_create_edge(obj, context_.data_model_.method_isArrow_4, context_.data_model_.arboretum_node_for(D->isArrow()));
  //getMemberLoc
  {
    const Id* other = context_.source_model_.resolve(D->getMemberLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getMemberLoc_2, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_97, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_96, other);
  }
  //getExprLoc
  {
    const Id* other = context_.source_model_.resolve(D->getExprLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getExprLoc_9, other);
  }
  //isImplicitAccess
  arboretum_create_edge(obj, context_.data_model_.method_isImplicitAccess_2, context_.data_model_.arboretum_node_for(D->isImplicitAccess()));
  //hadMultipleCandidates
  arboretum_create_edge(obj, context_.data_model_.method_hadMultipleCandidates_2, context_.data_model_.arboretum_node_for(D->hadMultipleCandidates()));
  //isNonOdrUse
  {
    const Id* enum_value = context_.data_model_.resolve(D->isNonOdrUse());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_isNonOdrUse_1, enum_value);
    }
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitNoInitExpr(clang::NoInitExpr* D) {
  const Id* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_98, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_97, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitNullStmt(clang::NullStmt* D) {
  const Id* obj = context_.resolve(D);
  //getSemiLoc
  {
    const Id* other = context_.source_model_.resolve(D->getSemiLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getSemiLoc, other);
  }
  //hasLeadingEmptyMacro
  arboretum_create_edge(obj, context_.data_model_.method_hasLeadingEmptyMacro, context_.data_model_.arboretum_node_for(D->hasLeadingEmptyMacro()));
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_99, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_98, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitOMPArraySectionExpr(clang::OMPArraySectionExpr* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPArrayShapingExpr(clang::OMPArrayShapingExpr* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPAtomicDirective(clang::OMPAtomicDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPBarrierDirective(clang::OMPBarrierDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPCancelDirective(clang::OMPCancelDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPCancellationPointDirective(clang::OMPCancellationPointDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPCanonicalLoop(clang::OMPCanonicalLoop* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPCriticalDirective(clang::OMPCriticalDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPDepobjDirective(clang::OMPDepobjDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPDispatchDirective(clang::OMPDispatchDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPDistributeDirective(clang::OMPDistributeDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPDistributeParallelForDirective(clang::OMPDistributeParallelForDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPDistributeParallelForSimdDirective(clang::OMPDistributeParallelForSimdDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPDistributeSimdDirective(clang::OMPDistributeSimdDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPErrorDirective(clang::OMPErrorDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPExecutableDirective(clang::OMPExecutableDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPFlushDirective(clang::OMPFlushDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPForDirective(clang::OMPForDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPForSimdDirective(clang::OMPForSimdDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPGenericLoopDirective(clang::OMPGenericLoopDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPInteropDirective(clang::OMPInteropDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPIteratorExpr(clang::OMPIteratorExpr* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPLoopBasedDirective(clang::OMPLoopBasedDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPLoopDirective(clang::OMPLoopDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPLoopTransformationDirective(clang::OMPLoopTransformationDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPMaskedDirective(clang::OMPMaskedDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPMaskedTaskLoopDirective(clang::OMPMaskedTaskLoopDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPMaskedTaskLoopSimdDirective(clang::OMPMaskedTaskLoopSimdDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPMasterDirective(clang::OMPMasterDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPMasterTaskLoopDirective(clang::OMPMasterTaskLoopDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPMasterTaskLoopSimdDirective(clang::OMPMasterTaskLoopSimdDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPMetaDirective(clang::OMPMetaDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPOrderedDirective(clang::OMPOrderedDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelDirective(clang::OMPParallelDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelForDirective(clang::OMPParallelForDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelForSimdDirective(clang::OMPParallelForSimdDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelGenericLoopDirective(clang::OMPParallelGenericLoopDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelMaskedDirective(clang::OMPParallelMaskedDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelMaskedTaskLoopDirective(clang::OMPParallelMaskedTaskLoopDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelMaskedTaskLoopSimdDirective(clang::OMPParallelMaskedTaskLoopSimdDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelMasterDirective(clang::OMPParallelMasterDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelMasterTaskLoopDirective(clang::OMPParallelMasterTaskLoopDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelMasterTaskLoopSimdDirective(clang::OMPParallelMasterTaskLoopSimdDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelSectionsDirective(clang::OMPParallelSectionsDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPScanDirective(clang::OMPScanDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPScopeDirective(clang::OMPScopeDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPSectionDirective(clang::OMPSectionDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPSectionsDirective(clang::OMPSectionsDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPSimdDirective(clang::OMPSimdDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPSingleDirective(clang::OMPSingleDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetDataDirective(clang::OMPTargetDataDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetDirective(clang::OMPTargetDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetEnterDataDirective(clang::OMPTargetEnterDataDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetExitDataDirective(clang::OMPTargetExitDataDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetParallelDirective(clang::OMPTargetParallelDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetParallelForDirective(clang::OMPTargetParallelForDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetParallelForSimdDirective(clang::OMPTargetParallelForSimdDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetParallelGenericLoopDirective(clang::OMPTargetParallelGenericLoopDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetSimdDirective(clang::OMPTargetSimdDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetTeamsDirective(clang::OMPTargetTeamsDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetTeamsDistributeDirective(clang::OMPTargetTeamsDistributeDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetTeamsDistributeParallelForDirective(clang::OMPTargetTeamsDistributeParallelForDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetTeamsDistributeParallelForSimdDirective(clang::OMPTargetTeamsDistributeParallelForSimdDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetTeamsDistributeSimdDirective(clang::OMPTargetTeamsDistributeSimdDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetTeamsGenericLoopDirective(clang::OMPTargetTeamsGenericLoopDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetUpdateDirective(clang::OMPTargetUpdateDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTaskDirective(clang::OMPTaskDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTaskLoopDirective(clang::OMPTaskLoopDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTaskLoopSimdDirective(clang::OMPTaskLoopSimdDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTaskgroupDirective(clang::OMPTaskgroupDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTaskwaitDirective(clang::OMPTaskwaitDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTaskyieldDirective(clang::OMPTaskyieldDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTeamsDirective(clang::OMPTeamsDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTeamsDistributeDirective(clang::OMPTeamsDistributeDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTeamsDistributeParallelForDirective(clang::OMPTeamsDistributeParallelForDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTeamsDistributeParallelForSimdDirective(clang::OMPTeamsDistributeParallelForSimdDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTeamsDistributeSimdDirective(clang::OMPTeamsDistributeSimdDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTeamsGenericLoopDirective(clang::OMPTeamsGenericLoopDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTileDirective(clang::OMPTileDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPUnrollDirective(clang::OMPUnrollDirective* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCArrayLiteral(clang::ObjCArrayLiteral* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCAtCatchStmt(clang::ObjCAtCatchStmt* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCAtFinallyStmt(clang::ObjCAtFinallyStmt* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCAtSynchronizedStmt(clang::ObjCAtSynchronizedStmt* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCAtThrowStmt(clang::ObjCAtThrowStmt* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCAtTryStmt(clang::ObjCAtTryStmt* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCAutoreleasePoolStmt(clang::ObjCAutoreleasePoolStmt* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCAvailabilityCheckExpr(clang::ObjCAvailabilityCheckExpr* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCBoolLiteralExpr(clang::ObjCBoolLiteralExpr* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCBoxedExpr(clang::ObjCBoxedExpr* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCBridgedCastExpr(clang::ObjCBridgedCastExpr* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCDictionaryLiteral(clang::ObjCDictionaryLiteral* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCEncodeExpr(clang::ObjCEncodeExpr* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCForCollectionStmt(clang::ObjCForCollectionStmt* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCIndirectCopyRestoreExpr(clang::ObjCIndirectCopyRestoreExpr* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCIsaExpr(clang::ObjCIsaExpr* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCIvarRefExpr(clang::ObjCIvarRefExpr* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCMessageExpr(clang::ObjCMessageExpr* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCPropertyRefExpr(clang::ObjCPropertyRefExpr* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCProtocolExpr(clang::ObjCProtocolExpr* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCSelectorExpr(clang::ObjCSelectorExpr* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCStringLiteral(clang::ObjCStringLiteral* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCSubscriptRefExpr(clang::ObjCSubscriptRefExpr* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOffsetOfExpr(clang::OffsetOfExpr* D) {
  const Id* obj = context_.resolve(D);
  //getOperatorLoc
  {
    const Id* other = context_.source_model_.resolve(D->getOperatorLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getOperatorLoc_7, other);
  }
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_19, other);
  }
  //getTypeSourceInfo
  //getNumComponents
  // unsigned int
  //getNumExpressions
  // unsigned int
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_100, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_99, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitOpaqueValueExpr(clang::OpaqueValueExpr* D) {
  const Id* obj = context_.resolve(D);
  //getLocation
  {
    const Id* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.method_getLocation_12, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_101, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_100, other);
  }
  //getExprLoc
  {
    const Id* other = context_.source_model_.resolve(D->getExprLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getExprLoc_10, other);
  }
  //children
  // const_child_range
  //getSourceExpr
  {
    const Id* other = context_.resolve(D->getSourceExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getSourceExpr, other);
  }
  //isUnique
  arboretum_create_edge(obj, context_.data_model_.method_isUnique, context_.data_model_.arboretum_node_for(D->isUnique()));
  return true;
}

bool ArboretumASTVisitor::VisitOverloadExpr(clang::OverloadExpr* D) {
  const Id* obj = context_.resolve(D);
  //getNamingClass
  {
    const Id* other = context_.resolve(D->getNamingClass());
    arboretum_create_edge(obj, context_.data_model_.method_getNamingClass, other);
  }
  //decls
  // llvm::iterator_range<decls_iterator>
  //getNumDecls
  // unsigned int
  //getNameInfo
  // const DeclarationNameInfo &
  //getName
  // DeclarationName
  //getNameLoc
  {
    const Id* other = context_.source_model_.resolve(D->getNameLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getNameLoc, other);
  }
  //getQualifier
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getTemplateKeywordLoc
  {
    const Id* other = context_.source_model_.resolve(D->getTemplateKeywordLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getTemplateKeywordLoc_6, other);
  }
  //getLAngleLoc
  {
    const Id* other = context_.source_model_.resolve(D->getLAngleLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getLAngleLoc_4, other);
  }
  //getRAngleLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRAngleLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRAngleLoc_4, other);
  }
  //hasTemplateKeyword
  arboretum_create_edge(obj, context_.data_model_.method_hasTemplateKeyword_4, context_.data_model_.arboretum_node_for(D->hasTemplateKeyword()));
  //hasExplicitTemplateArgs
  arboretum_create_edge(obj, context_.data_model_.method_hasExplicitTemplateArgs_5, context_.data_model_.arboretum_node_for(D->hasExplicitTemplateArgs()));
  //getTemplateArgs
  //getNumTemplateArgs
  // unsigned int
  //template_arguments
  // ArrayRef<TemplateArgumentLoc>
  return true;
}

bool ArboretumASTVisitor::VisitPackExpansionExpr(clang::PackExpansionExpr* D) {
  const Id* obj = context_.resolve(D);
  //getPattern
  {
    const Id* other = context_.resolve(D->getPattern());
    arboretum_create_edge(obj, context_.data_model_.method_getPattern_2, other);
  }
  //getEllipsisLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEllipsisLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEllipsisLoc_6, other);
  }
  //getNumExpansions
  // std::optional<unsigned int>
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_102, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_101, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitParenExpr(clang::ParenExpr* D) {
  const Id* obj = context_.resolve(D);
  //getSubExpr
  {
    const Id* other = context_.resolve(D->getSubExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getSubExpr_8, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_103, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_102, other);
  }
  //getLParen
  {
    const Id* other = context_.source_model_.resolve(D->getLParen());
    arboretum_create_edge(obj, context_.data_model_.method_getLParen, other);
  }
  //getRParen
  {
    const Id* other = context_.source_model_.resolve(D->getRParen());
    arboretum_create_edge(obj, context_.data_model_.method_getRParen, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitParenListExpr(clang::ParenListExpr* D) {
  const Id* obj = context_.resolve(D);
  //getNumExprs
  // unsigned int
  //getLParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getLParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getLParenLoc_7, other);
  }
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_20, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_104, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_103, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitPredefinedExpr(clang::PredefinedExpr* D) {
  const Id* obj = context_.resolve(D);
  //getIdentKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getIdentKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getIdentKind, enum_value);
    }
  }
  //isTransparent
  arboretum_create_edge(obj, context_.data_model_.method_isTransparent_1, context_.data_model_.arboretum_node_for(D->isTransparent()));
  //getLocation
  {
    const Id* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.method_getLocation_13, other);
  }
  //getFunctionName
  {
    const Id* other = context_.resolve(D->getFunctionName());
    arboretum_create_edge(obj, context_.data_model_.method_getFunctionName, other);
  }
  //getIdentKindName
  // StringRef
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_105, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_104, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitPseudoObjectExpr(clang::PseudoObjectExpr* D) {
  const Id* obj = context_.resolve(D);
  //getSyntacticForm
  {
    const Id* other = context_.resolve(D->getSyntacticForm());
    arboretum_create_edge(obj, context_.data_model_.method_getSyntacticForm_1, other);
  }
  //getResultExprIndex
  // unsigned int
  //getResultExpr
  {
    const Id* other = context_.resolve(D->getResultExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getResultExpr_1, other);
  }
  //getNumSemanticExprs
  // unsigned int
  //semantics
  // ArrayRef<const Expr *>
  //getExprLoc
  {
    const Id* other = context_.source_model_.resolve(D->getExprLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getExprLoc_11, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_106, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_105, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitRecoveryExpr(clang::RecoveryExpr* D) {
  const Id* obj = context_.resolve(D);
  //subExpressions
  // ArrayRef<const Expr *>
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_107, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_106, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitRequiresExpr(clang::RequiresExpr* D) {
  const Id* obj = context_.resolve(D);
  //getLocalParameters
  // ArrayRef<ParmVarDecl *>
  //getBody
  {
    const Id* other = context_.resolve(D->getBody());
    arboretum_create_edge(obj, context_.data_model_.method_getBody_10, other);
  }
  //getRequirements
  // ArrayRef<concepts::Requirement *>
  //getRequiresKWLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRequiresKWLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRequiresKWLoc, other);
  }
  //getLParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getLParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getLParenLoc_8, other);
  }
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_21, other);
  }
  //getRBraceLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRBraceLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRBraceLoc_5, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_108, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_107, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitReturnStmt(clang::ReturnStmt* D) {
  const Id* obj = context_.resolve(D);
  //getRetValue
  {
    const Id* other = context_.resolve(D->getRetValue());
    arboretum_create_edge(obj, context_.data_model_.method_getRetValue, other);
  }
  //getNRVOCandidate
  {
    const Id* other = context_.resolve(D->getNRVOCandidate());
    arboretum_create_edge(obj, context_.data_model_.method_getNRVOCandidate, other);
  }
  //getReturnLoc
  {
    const Id* other = context_.source_model_.resolve(D->getReturnLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getReturnLoc, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_109, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_108, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitSEHExceptStmt(clang::SEHExceptStmt* D) {
  const Id* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_110, other);
  }
  //getExceptLoc
  {
    const Id* other = context_.source_model_.resolve(D->getExceptLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getExceptLoc, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_109, other);
  }
  //getFilterExpr
  {
    const Id* other = context_.resolve(D->getFilterExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getFilterExpr, other);
  }
  //getBlock
  {
    const Id* other = context_.resolve(D->getBlock());
    arboretum_create_edge(obj, context_.data_model_.method_getBlock, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitSEHFinallyStmt(clang::SEHFinallyStmt* D) {
  const Id* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_111, other);
  }
  //getFinallyLoc
  {
    const Id* other = context_.source_model_.resolve(D->getFinallyLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getFinallyLoc, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_110, other);
  }
  //getBlock
  {
    const Id* other = context_.resolve(D->getBlock());
    arboretum_create_edge(obj, context_.data_model_.method_getBlock_1, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitSEHLeaveStmt(clang::SEHLeaveStmt* D) {
  const Id* obj = context_.resolve(D);
  //getLeaveLoc
  {
    const Id* other = context_.source_model_.resolve(D->getLeaveLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getLeaveLoc, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_112, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_111, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitSEHTryStmt(clang::SEHTryStmt* D) {
  const Id* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_113, other);
  }
  //getTryLoc
  {
    const Id* other = context_.source_model_.resolve(D->getTryLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getTryLoc_1, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_112, other);
  }
  //getIsCXXTry
  arboretum_create_edge(obj, context_.data_model_.method_getIsCXXTry, context_.data_model_.arboretum_node_for(D->getIsCXXTry()));
  //getTryBlock
  {
    const Id* other = context_.resolve(D->getTryBlock());
    arboretum_create_edge(obj, context_.data_model_.method_getTryBlock_1, other);
  }
  //getHandler
  {
    const Id* other = context_.resolve(D->getHandler());
    arboretum_create_edge(obj, context_.data_model_.method_getHandler, other);
  }
  //getExceptHandler
  {
    const Id* other = context_.resolve(D->getExceptHandler());
    arboretum_create_edge(obj, context_.data_model_.method_getExceptHandler, other);
  }
  //getFinallyHandler
  {
    const Id* other = context_.resolve(D->getFinallyHandler());
    arboretum_create_edge(obj, context_.data_model_.method_getFinallyHandler, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitSYCLUniqueStableNameExpr(clang::SYCLUniqueStableNameExpr* D) {
  const Id* obj = context_.resolve(D);
  //getTypeSourceInfo
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_114, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_113, other);
  }
  //getLocation
  {
    const Id* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.method_getLocation_14, other);
  }
  //getLParenLocation
  {
    const Id* other = context_.source_model_.resolve(D->getLParenLocation());
    arboretum_create_edge(obj, context_.data_model_.method_getLParenLocation, other);
  }
  //getRParenLocation
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLocation());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLocation, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitShuffleVectorExpr(clang::ShuffleVectorExpr* D) {
  const Id* obj = context_.resolve(D);
  //getBuiltinLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBuiltinLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBuiltinLoc_4, other);
  }
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_22, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_115, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_114, other);
  }
  //getNumSubExprs
  // unsigned int
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitSizeOfPackExpr(clang::SizeOfPackExpr* D) {
  const Id* obj = context_.resolve(D);
  //getOperatorLoc
  {
    const Id* other = context_.source_model_.resolve(D->getOperatorLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getOperatorLoc_8, other);
  }
  //getPackLoc
  {
    const Id* other = context_.source_model_.resolve(D->getPackLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getPackLoc, other);
  }
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_23, other);
  }
  //getPack
  {
    const Id* other = context_.resolve(D->getPack());
    arboretum_create_edge(obj, context_.data_model_.method_getPack, other);
  }
  //getPackLength
  // unsigned int
  //isPartiallySubstituted
  arboretum_create_edge(obj, context_.data_model_.method_isPartiallySubstituted, context_.data_model_.arboretum_node_for(D->isPartiallySubstituted()));
  //getPartialArguments
  // ArrayRef<TemplateArgument>
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_116, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_115, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitSourceLocExpr(clang::SourceLocExpr* D) {
  const Id* obj = context_.resolve(D);
  //getBuiltinStr
  // StringRef
  //getIdentKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getIdentKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getIdentKind_1, enum_value);
    }
  }
  //isIntType
  arboretum_create_edge(obj, context_.data_model_.method_isIntType, context_.data_model_.arboretum_node_for(D->isIntType()));
  //getParentContext
  //getLocation
  {
    const Id* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.method_getLocation_15, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_117, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_116, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitStmt(clang::Stmt* D) {
  const Id* obj = context_.resolve(D);
  switch(D->getStmtClass()) {
    case clang::Stmt::ObjCArrayLiteralClass: {
      assert(context_.data_model_.class_ObjCArrayLiteral != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCArrayLiteral); 
    } break;
    case clang::Stmt::WhileStmtClass: {
      assert(context_.data_model_.class_WhileStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_WhileStmt); 
    } break;
    case clang::Stmt::CXXStdInitializerListExprClass: {
      assert(context_.data_model_.class_CXXStdInitializerListExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXStdInitializerListExpr); 
    } break;
    case clang::Stmt::ImplicitCastExprClass: {
      assert(context_.data_model_.class_ImplicitCastExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ImplicitCastExpr); 
    } break;
    case clang::Stmt::ObjCSelectorExprClass: {
      assert(context_.data_model_.class_ObjCSelectorExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCSelectorExpr); 
    } break;
    case clang::Stmt::ObjCAtTryStmtClass: {
      assert(context_.data_model_.class_ObjCAtTryStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCAtTryStmt); 
    } break;
    case clang::Stmt::ObjCProtocolExprClass: {
      assert(context_.data_model_.class_ObjCProtocolExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCProtocolExpr); 
    } break;
    case clang::Stmt::CXXTypeidExprClass: {
      assert(context_.data_model_.class_CXXTypeidExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXTypeidExpr); 
    } break;
    case clang::Stmt::ObjCPropertyRefExprClass: {
      assert(context_.data_model_.class_ObjCPropertyRefExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCPropertyRefExpr); 
    } break;
    case clang::Stmt::OMPTargetTeamsDistributeSimdDirectiveClass: {
      assert(context_.data_model_.class_OMPTargetTeamsDistributeSimdDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetTeamsDistributeSimdDirective); 
    } break;
    case clang::Stmt::ObjCMessageExprClass: {
      assert(context_.data_model_.class_ObjCMessageExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCMessageExpr); 
    } break;
    case clang::Stmt::CXXParenListInitExprClass: {
      assert(context_.data_model_.class_CXXParenListInitExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXParenListInitExpr); 
    } break;
    case clang::Stmt::ConvertVectorExprClass: {
      assert(context_.data_model_.class_ConvertVectorExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ConvertVectorExpr); 
    } break;
    case clang::Stmt::GotoStmtClass: {
      assert(context_.data_model_.class_GotoStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_GotoStmt); 
    } break;
    case clang::Stmt::BreakStmtClass: {
      assert(context_.data_model_.class_BreakStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_BreakStmt); 
    } break;
    case clang::Stmt::ReturnStmtClass: {
      assert(context_.data_model_.class_ReturnStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ReturnStmt); 
    } break;
    case clang::Stmt::OMPParallelMaskedTaskLoopDirectiveClass: {
      assert(context_.data_model_.class_OMPParallelMaskedTaskLoopDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPParallelMaskedTaskLoopDirective); 
    } break;
    case clang::Stmt::ObjCIsaExprClass: {
      assert(context_.data_model_.class_ObjCIsaExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCIsaExpr); 
    } break;
    case clang::Stmt::GCCAsmStmtClass: {
      assert(context_.data_model_.class_GCCAsmStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_GCCAsmStmt); 
    } break;
    case clang::Stmt::CXXDynamicCastExprClass: {
      assert(context_.data_model_.class_CXXDynamicCastExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXDynamicCastExpr); 
    } break;
    case clang::Stmt::SEHExceptStmtClass: {
      assert(context_.data_model_.class_SEHExceptStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_SEHExceptStmt); 
    } break;
    case clang::Stmt::SEHLeaveStmtClass: {
      assert(context_.data_model_.class_SEHLeaveStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_SEHLeaveStmt); 
    } break;
    case clang::Stmt::CXXForRangeStmtClass: {
      assert(context_.data_model_.class_CXXForRangeStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXForRangeStmt); 
    } break;
    case clang::Stmt::ObjCEncodeExprClass: {
      assert(context_.data_model_.class_ObjCEncodeExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCEncodeExpr); 
    } break;
    case clang::Stmt::SubstNonTypeTemplateParmExprClass: {
      assert(context_.data_model_.class_SubstNonTypeTemplateParmExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_SubstNonTypeTemplateParmExpr); 
    } break;
    case clang::Stmt::ObjCIndirectCopyRestoreExprClass: {
      assert(context_.data_model_.class_ObjCIndirectCopyRestoreExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCIndirectCopyRestoreExpr); 
    } break;
    case clang::Stmt::CoroutineBodyStmtClass: {
      assert(context_.data_model_.class_CoroutineBodyStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CoroutineBodyStmt); 
    } break;
    case clang::Stmt::CoreturnStmtClass: {
      assert(context_.data_model_.class_CoreturnStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CoreturnStmt); 
    } break;
    case clang::Stmt::ArrayTypeTraitExprClass: {
      assert(context_.data_model_.class_ArrayTypeTraitExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ArrayTypeTraitExpr); 
    } break;
    case clang::Stmt::CXXUuidofExprClass: {
      assert(context_.data_model_.class_CXXUuidofExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXUuidofExpr); 
    } break;
    case clang::Stmt::ObjCForCollectionStmtClass: {
      assert(context_.data_model_.class_ObjCForCollectionStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCForCollectionStmt); 
    } break;
    case clang::Stmt::ObjCAtCatchStmtClass: {
      assert(context_.data_model_.class_ObjCAtCatchStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCAtCatchStmt); 
    } break;
    case clang::Stmt::ForStmtClass: {
      assert(context_.data_model_.class_ForStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ForStmt); 
    } break;
    case clang::Stmt::ObjCAtSynchronizedStmtClass: {
      assert(context_.data_model_.class_ObjCAtSynchronizedStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCAtSynchronizedStmt); 
    } break;
    case clang::Stmt::ObjCAtThrowStmtClass: {
      assert(context_.data_model_.class_ObjCAtThrowStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCAtThrowStmt); 
    } break;
    case clang::Stmt::OMPParallelMasterDirectiveClass: {
      assert(context_.data_model_.class_OMPParallelMasterDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPParallelMasterDirective); 
    } break;
    case clang::Stmt::ImaginaryLiteralClass: {
      assert(context_.data_model_.class_ImaginaryLiteral != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ImaginaryLiteral); 
    } break;
    case clang::Stmt::ObjCAutoreleasePoolStmtClass: {
      assert(context_.data_model_.class_ObjCAutoreleasePoolStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCAutoreleasePoolStmt); 
    } break;
    case clang::Stmt::OMPCanonicalLoopClass: {
      assert(context_.data_model_.class_OMPCanonicalLoop != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPCanonicalLoop); 
    } break;
    case clang::Stmt::OMPParallelDirectiveClass: {
      assert(context_.data_model_.class_OMPParallelDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPParallelDirective); 
    } break;
    case clang::Stmt::OMPTileDirectiveClass: {
      assert(context_.data_model_.class_OMPTileDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTileDirective); 
    } break;
    case clang::Stmt::MSPropertyRefExprClass: {
      assert(context_.data_model_.class_MSPropertyRefExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_MSPropertyRefExpr); 
    } break;
    case clang::Stmt::ObjCDictionaryLiteralClass: {
      assert(context_.data_model_.class_ObjCDictionaryLiteral != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCDictionaryLiteral); 
    } break;
    case clang::Stmt::OMPInteropDirectiveClass: {
      assert(context_.data_model_.class_OMPInteropDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPInteropDirective); 
    } break;
    case clang::Stmt::ObjCSubscriptRefExprClass: {
      assert(context_.data_model_.class_ObjCSubscriptRefExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCSubscriptRefExpr); 
    } break;
    case clang::Stmt::CXXBoolLiteralExprClass: {
      assert(context_.data_model_.class_CXXBoolLiteralExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXBoolLiteralExpr); 
    } break;
    case clang::Stmt::SYCLUniqueStableNameExprClass: {
      assert(context_.data_model_.class_SYCLUniqueStableNameExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_SYCLUniqueStableNameExpr); 
    } break;
    case clang::Stmt::OMPSimdDirectiveClass: {
      assert(context_.data_model_.class_OMPSimdDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPSimdDirective); 
    } break;
    case clang::Stmt::OMPForSimdDirectiveClass: {
      assert(context_.data_model_.class_OMPForSimdDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPForSimdDirective); 
    } break;
    case clang::Stmt::ObjCBoxedExprClass: {
      assert(context_.data_model_.class_ObjCBoxedExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCBoxedExpr); 
    } break;
    case clang::Stmt::OMPParallelSectionsDirectiveClass: {
      assert(context_.data_model_.class_OMPParallelSectionsDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPParallelSectionsDirective); 
    } break;
    case clang::Stmt::IfStmtClass: {
      assert(context_.data_model_.class_IfStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_IfStmt); 
    } break;
    case clang::Stmt::ObjCAvailabilityCheckExprClass: {
      assert(context_.data_model_.class_ObjCAvailabilityCheckExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCAvailabilityCheckExpr); 
    } break;
    case clang::Stmt::OMPParallelForDirectiveClass: {
      assert(context_.data_model_.class_OMPParallelForDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPParallelForDirective); 
    } break;
    case clang::Stmt::DoStmtClass: {
      assert(context_.data_model_.class_DoStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DoStmt); 
    } break;
    case clang::Stmt::ConditionalOperatorClass: {
      assert(context_.data_model_.class_ConditionalOperator != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ConditionalOperator); 
    } break;
    case clang::Stmt::OMPTargetParallelForDirectiveClass: {
      assert(context_.data_model_.class_OMPTargetParallelForDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetParallelForDirective); 
    } break;
    case clang::Stmt::OMPErrorDirectiveClass: {
      assert(context_.data_model_.class_OMPErrorDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPErrorDirective); 
    } break;
    case clang::Stmt::OMPTaskLoopSimdDirectiveClass: {
      assert(context_.data_model_.class_OMPTaskLoopSimdDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTaskLoopSimdDirective); 
    } break;
    case clang::Stmt::OMPMaskedTaskLoopDirectiveClass: {
      assert(context_.data_model_.class_OMPMaskedTaskLoopDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPMaskedTaskLoopDirective); 
    } break;
    case clang::Stmt::OMPMasterTaskLoopSimdDirectiveClass: {
      assert(context_.data_model_.class_OMPMasterTaskLoopSimdDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPMasterTaskLoopSimdDirective); 
    } break;
    case clang::Stmt::IndirectGotoStmtClass: {
      assert(context_.data_model_.class_IndirectGotoStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_IndirectGotoStmt); 
    } break;
    case clang::Stmt::OMPForDirectiveClass: {
      assert(context_.data_model_.class_OMPForDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPForDirective); 
    } break;
    case clang::Stmt::CXXTryStmtClass: {
      assert(context_.data_model_.class_CXXTryStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXTryStmt); 
    } break;
    case clang::Stmt::OMPMaskedTaskLoopSimdDirectiveClass: {
      assert(context_.data_model_.class_OMPMaskedTaskLoopSimdDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPMaskedTaskLoopSimdDirective); 
    } break;
    case clang::Stmt::CompoundAssignOperatorClass: {
      assert(context_.data_model_.class_CompoundAssignOperator != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CompoundAssignOperator); 
    } break;
    case clang::Stmt::ObjCAtFinallyStmtClass: {
      assert(context_.data_model_.class_ObjCAtFinallyStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCAtFinallyStmt); 
    } break;
    case clang::Stmt::OMPParallelMasterTaskLoopDirectiveClass: {
      assert(context_.data_model_.class_OMPParallelMasterTaskLoopDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPParallelMasterTaskLoopDirective); 
    } break;
    case clang::Stmt::BlockExprClass: {
      assert(context_.data_model_.class_BlockExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_BlockExpr); 
    } break;
    case clang::Stmt::OMPMasterDirectiveClass: {
      assert(context_.data_model_.class_OMPMasterDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPMasterDirective); 
    } break;
    case clang::Stmt::CXXNoexceptExprClass: {
      assert(context_.data_model_.class_CXXNoexceptExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXNoexceptExpr); 
    } break;
    case clang::Stmt::SEHTryStmtClass: {
      assert(context_.data_model_.class_SEHTryStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_SEHTryStmt); 
    } break;
    case clang::Stmt::UnresolvedMemberExprClass: {
      assert(context_.data_model_.class_UnresolvedMemberExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UnresolvedMemberExpr); 
    } break;
    case clang::Stmt::OMPSingleDirectiveClass: {
      assert(context_.data_model_.class_OMPSingleDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPSingleDirective); 
    } break;
    case clang::Stmt::ObjCIvarRefExprClass: {
      assert(context_.data_model_.class_ObjCIvarRefExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCIvarRefExpr); 
    } break;
    case clang::Stmt::CUDAKernelCallExprClass: {
      assert(context_.data_model_.class_CUDAKernelCallExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CUDAKernelCallExpr); 
    } break;
    case clang::Stmt::OMPTaskyieldDirectiveClass: {
      assert(context_.data_model_.class_OMPTaskyieldDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTaskyieldDirective); 
    } break;
    case clang::Stmt::OMPIteratorExprClass: {
      assert(context_.data_model_.class_OMPIteratorExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPIteratorExpr); 
    } break;
    case clang::Stmt::MSDependentExistsStmtClass: {
      assert(context_.data_model_.class_MSDependentExistsStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_MSDependentExistsStmt); 
    } break;
    case clang::Stmt::OMPTargetUpdateDirectiveClass: {
      assert(context_.data_model_.class_OMPTargetUpdateDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetUpdateDirective); 
    } break;
    case clang::Stmt::OMPCancelDirectiveClass: {
      assert(context_.data_model_.class_OMPCancelDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPCancelDirective); 
    } break;
    case clang::Stmt::RecoveryExprClass: {
      assert(context_.data_model_.class_RecoveryExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_RecoveryExpr); 
    } break;
    case clang::Stmt::OMPDistributeParallelForDirectiveClass: {
      assert(context_.data_model_.class_OMPDistributeParallelForDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPDistributeParallelForDirective); 
    } break;
    case clang::Stmt::OMPTaskwaitDirectiveClass: {
      assert(context_.data_model_.class_OMPTaskwaitDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTaskwaitDirective); 
    } break;
    case clang::Stmt::OMPTargetParallelDirectiveClass: {
      assert(context_.data_model_.class_OMPTargetParallelDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetParallelDirective); 
    } break;
    case clang::Stmt::OMPCriticalDirectiveClass: {
      assert(context_.data_model_.class_OMPCriticalDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPCriticalDirective); 
    } break;
    case clang::Stmt::SwitchStmtClass: {
      assert(context_.data_model_.class_SwitchStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_SwitchStmt); 
    } break;
    case clang::Stmt::CXXNullPtrLiteralExprClass: {
      assert(context_.data_model_.class_CXXNullPtrLiteralExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXNullPtrLiteralExpr); 
    } break;
    case clang::Stmt::MSAsmStmtClass: {
      assert(context_.data_model_.class_MSAsmStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_MSAsmStmt); 
    } break;
    case clang::Stmt::OMPTargetSimdDirectiveClass: {
      assert(context_.data_model_.class_OMPTargetSimdDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetSimdDirective); 
    } break;
    case clang::Stmt::OMPDepobjDirectiveClass: {
      assert(context_.data_model_.class_OMPDepobjDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPDepobjDirective); 
    } break;
    case clang::Stmt::ConstantExprClass: {
      assert(context_.data_model_.class_ConstantExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ConstantExpr); 
    } break;
    case clang::Stmt::OMPParallelForSimdDirectiveClass: {
      assert(context_.data_model_.class_OMPParallelForSimdDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPParallelForSimdDirective); 
    } break;
    case clang::Stmt::OMPParallelMaskedDirectiveClass: {
      assert(context_.data_model_.class_OMPParallelMaskedDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPParallelMaskedDirective); 
    } break;
    case clang::Stmt::InitListExprClass: {
      assert(context_.data_model_.class_InitListExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_InitListExpr); 
    } break;
    case clang::Stmt::OMPTargetTeamsGenericLoopDirectiveClass: {
      assert(context_.data_model_.class_OMPTargetTeamsGenericLoopDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetTeamsGenericLoopDirective); 
    } break;
    case clang::Stmt::CXXUnresolvedConstructExprClass: {
      assert(context_.data_model_.class_CXXUnresolvedConstructExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXUnresolvedConstructExpr); 
    } break;
    case clang::Stmt::OMPTargetDirectiveClass: {
      assert(context_.data_model_.class_OMPTargetDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetDirective); 
    } break;
    case clang::Stmt::OMPTargetDataDirectiveClass: {
      assert(context_.data_model_.class_OMPTargetDataDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetDataDirective); 
    } break;
    case clang::Stmt::OMPTargetEnterDataDirectiveClass: {
      assert(context_.data_model_.class_OMPTargetEnterDataDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetEnterDataDirective); 
    } break;
    case clang::Stmt::OMPTeamsDistributeDirectiveClass: {
      assert(context_.data_model_.class_OMPTeamsDistributeDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTeamsDistributeDirective); 
    } break;
    case clang::Stmt::OMPOrderedDirectiveClass: {
      assert(context_.data_model_.class_OMPOrderedDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPOrderedDirective); 
    } break;
    case clang::Stmt::OMPTaskgroupDirectiveClass: {
      assert(context_.data_model_.class_OMPTaskgroupDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTaskgroupDirective); 
    } break;
    case clang::Stmt::OMPTargetTeamsDirectiveClass: {
      assert(context_.data_model_.class_OMPTargetTeamsDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetTeamsDirective); 
    } break;
    case clang::Stmt::OMPFlushDirectiveClass: {
      assert(context_.data_model_.class_OMPFlushDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPFlushDirective); 
    } break;
    case clang::Stmt::OMPScanDirectiveClass: {
      assert(context_.data_model_.class_OMPScanDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPScanDirective); 
    } break;
    case clang::Stmt::ExpressionTraitExprClass: {
      assert(context_.data_model_.class_ExpressionTraitExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ExpressionTraitExpr); 
    } break;
    case clang::Stmt::GenericSelectionExprClass: {
      assert(context_.data_model_.class_GenericSelectionExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_GenericSelectionExpr); 
    } break;
    case clang::Stmt::OMPTeamsDirectiveClass: {
      assert(context_.data_model_.class_OMPTeamsDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTeamsDirective); 
    } break;
    case clang::Stmt::CXXInheritedCtorInitExprClass: {
      assert(context_.data_model_.class_CXXInheritedCtorInitExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXInheritedCtorInitExpr); 
    } break;
    case clang::Stmt::OMPTeamsDistributeParallelForDirectiveClass: {
      assert(context_.data_model_.class_OMPTeamsDistributeParallelForDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTeamsDistributeParallelForDirective); 
    } break;
    case clang::Stmt::CXXThrowExprClass: {
      assert(context_.data_model_.class_CXXThrowExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXThrowExpr); 
    } break;
    case clang::Stmt::CXXDefaultInitExprClass: {
      assert(context_.data_model_.class_CXXDefaultInitExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXDefaultInitExpr); 
    } break;
    case clang::Stmt::OMPBarrierDirectiveClass: {
      assert(context_.data_model_.class_OMPBarrierDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPBarrierDirective); 
    } break;
    case clang::Stmt::CXXRewrittenBinaryOperatorClass: {
      assert(context_.data_model_.class_CXXRewrittenBinaryOperator != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXRewrittenBinaryOperator); 
    } break;
    case clang::Stmt::MatrixSubscriptExprClass: {
      assert(context_.data_model_.class_MatrixSubscriptExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_MatrixSubscriptExpr); 
    } break;
    case clang::Stmt::ArraySubscriptExprClass: {
      assert(context_.data_model_.class_ArraySubscriptExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ArraySubscriptExpr); 
    } break;
    case clang::Stmt::AtomicExprClass: {
      assert(context_.data_model_.class_AtomicExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_AtomicExpr); 
    } break;
    case clang::Stmt::CXXOperatorCallExprClass: {
      assert(context_.data_model_.class_CXXOperatorCallExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXOperatorCallExpr); 
    } break;
    case clang::Stmt::CXXBindTemporaryExprClass: {
      assert(context_.data_model_.class_CXXBindTemporaryExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXBindTemporaryExpr); 
    } break;
    case clang::Stmt::CapturedStmtClass: {
      assert(context_.data_model_.class_CapturedStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CapturedStmt); 
    } break;
    case clang::Stmt::CoyieldExprClass: {
      assert(context_.data_model_.class_CoyieldExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CoyieldExpr); 
    } break;
    case clang::Stmt::VAArgExprClass: {
      assert(context_.data_model_.class_VAArgExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_VAArgExpr); 
    } break;
    case clang::Stmt::AsTypeExprClass: {
      assert(context_.data_model_.class_AsTypeExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_AsTypeExpr); 
    } break;
    case clang::Stmt::TypeTraitExprClass: {
      assert(context_.data_model_.class_TypeTraitExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_TypeTraitExpr); 
    } break;
    case clang::Stmt::ExtVectorElementExprClass: {
      assert(context_.data_model_.class_ExtVectorElementExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ExtVectorElementExpr); 
    } break;
    case clang::Stmt::OMPUnrollDirectiveClass: {
      assert(context_.data_model_.class_OMPUnrollDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPUnrollDirective); 
    } break;
    case clang::Stmt::FunctionParmPackExprClass: {
      assert(context_.data_model_.class_FunctionParmPackExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_FunctionParmPackExpr); 
    } break;
    case clang::Stmt::ArrayInitIndexExprClass: {
      assert(context_.data_model_.class_ArrayInitIndexExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ArrayInitIndexExpr); 
    } break;
    case clang::Stmt::ArrayInitLoopExprClass: {
      assert(context_.data_model_.class_ArrayInitLoopExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ArrayInitLoopExpr); 
    } break;
    case clang::Stmt::CXXPseudoDestructorExprClass: {
      assert(context_.data_model_.class_CXXPseudoDestructorExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXPseudoDestructorExpr); 
    } break;
    case clang::Stmt::StringLiteralClass: {
      assert(context_.data_model_.class_StringLiteral != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_StringLiteral); 
    } break;
    case clang::Stmt::CaseStmtClass: {
      assert(context_.data_model_.class_CaseStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CaseStmt); 
    } break;
    case clang::Stmt::CXXFunctionalCastExprClass: {
      assert(context_.data_model_.class_CXXFunctionalCastExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXFunctionalCastExpr); 
    } break;
    case clang::Stmt::UnaryExprOrTypeTraitExprClass: {
      assert(context_.data_model_.class_UnaryExprOrTypeTraitExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UnaryExprOrTypeTraitExpr); 
    } break;
    case clang::Stmt::CXXScalarValueInitExprClass: {
      assert(context_.data_model_.class_CXXScalarValueInitExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXScalarValueInitExpr); 
    } break;
    case clang::Stmt::CXXDeleteExprClass: {
      assert(context_.data_model_.class_CXXDeleteExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXDeleteExpr); 
    } break;
    case clang::Stmt::FloatingLiteralClass: {
      assert(context_.data_model_.class_FloatingLiteral != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_FloatingLiteral); 
    } break;
    case clang::Stmt::UnaryOperatorClass: {
      assert(context_.data_model_.class_UnaryOperator != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UnaryOperator); 
    } break;
    case clang::Stmt::NullStmtClass: {
      assert(context_.data_model_.class_NullStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_NullStmt); 
    } break;
    case clang::Stmt::CXXDefaultArgExprClass: {
      assert(context_.data_model_.class_CXXDefaultArgExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXDefaultArgExpr); 
    } break;
    case clang::Stmt::PredefinedExprClass: {
      assert(context_.data_model_.class_PredefinedExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_PredefinedExpr); 
    } break;
    case clang::Stmt::IntegerLiteralClass: {
      assert(context_.data_model_.class_IntegerLiteral != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_IntegerLiteral); 
    } break;
    case clang::Stmt::CStyleCastExprClass: {
      assert(context_.data_model_.class_CStyleCastExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CStyleCastExpr); 
    } break;
    case clang::Stmt::BinaryOperatorClass: {
      assert(context_.data_model_.class_BinaryOperator != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_BinaryOperator); 
    } break;
    case clang::Stmt::CoawaitExprClass: {
      assert(context_.data_model_.class_CoawaitExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CoawaitExpr); 
    } break;
    case clang::Stmt::CompoundLiteralExprClass: {
      assert(context_.data_model_.class_CompoundLiteralExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CompoundLiteralExpr); 
    } break;
    case clang::Stmt::CXXConstructExprClass: {
      assert(context_.data_model_.class_CXXConstructExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXConstructExpr); 
    } break;
    case clang::Stmt::ExprWithCleanupsClass: {
      assert(context_.data_model_.class_ExprWithCleanups != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ExprWithCleanups); 
    } break;
    case clang::Stmt::ObjCBridgedCastExprClass: {
      assert(context_.data_model_.class_ObjCBridgedCastExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCBridgedCastExpr); 
    } break;
    case clang::Stmt::CXXReinterpretCastExprClass: {
      assert(context_.data_model_.class_CXXReinterpretCastExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXReinterpretCastExpr); 
    } break;
    case clang::Stmt::DefaultStmtClass: {
      assert(context_.data_model_.class_DefaultStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DefaultStmt); 
    } break;
    case clang::Stmt::DesignatedInitExprClass: {
      assert(context_.data_model_.class_DesignatedInitExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DesignatedInitExpr); 
    } break;
    case clang::Stmt::MSPropertySubscriptExprClass: {
      assert(context_.data_model_.class_MSPropertySubscriptExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_MSPropertySubscriptExpr); 
    } break;
    case clang::Stmt::OffsetOfExprClass: {
      assert(context_.data_model_.class_OffsetOfExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OffsetOfExpr); 
    } break;
    case clang::Stmt::CharacterLiteralClass: {
      assert(context_.data_model_.class_CharacterLiteral != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CharacterLiteral); 
    } break;
    case clang::Stmt::CXXStaticCastExprClass: {
      assert(context_.data_model_.class_CXXStaticCastExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXStaticCastExpr); 
    } break;
    case clang::Stmt::SourceLocExprClass: {
      assert(context_.data_model_.class_SourceLocExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_SourceLocExpr); 
    } break;
    case clang::Stmt::CXXNewExprClass: {
      assert(context_.data_model_.class_CXXNewExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXNewExpr); 
    } break;
    case clang::Stmt::ParenListExprClass: {
      assert(context_.data_model_.class_ParenListExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ParenListExpr); 
    } break;
    case clang::Stmt::BuiltinBitCastExprClass: {
      assert(context_.data_model_.class_BuiltinBitCastExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_BuiltinBitCastExpr); 
    } break;
    case clang::Stmt::TypoExprClass: {
      assert(context_.data_model_.class_TypoExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_TypoExpr); 
    } break;
    case clang::Stmt::DesignatedInitUpdateExprClass: {
      assert(context_.data_model_.class_DesignatedInitUpdateExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DesignatedInitUpdateExpr); 
    } break;
    case clang::Stmt::OpaqueValueExprClass: {
      assert(context_.data_model_.class_OpaqueValueExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OpaqueValueExpr); 
    } break;
    case clang::Stmt::OMPTargetParallelGenericLoopDirectiveClass: {
      assert(context_.data_model_.class_OMPTargetParallelGenericLoopDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetParallelGenericLoopDirective); 
    } break;
    case clang::Stmt::DeclStmtClass: {
      assert(context_.data_model_.class_DeclStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DeclStmt); 
    } break;
    case clang::Stmt::CXXMemberCallExprClass: {
      assert(context_.data_model_.class_CXXMemberCallExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXMemberCallExpr); 
    } break;
    case clang::Stmt::MemberExprClass: {
      assert(context_.data_model_.class_MemberExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_MemberExpr); 
    } break;
    case clang::Stmt::AttributedStmtClass: {
      assert(context_.data_model_.class_AttributedStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_AttributedStmt); 
    } break;
    case clang::Stmt::CXXConstCastExprClass: {
      assert(context_.data_model_.class_CXXConstCastExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXConstCastExpr); 
    } break;
    case clang::Stmt::StmtExprClass: {
      assert(context_.data_model_.class_StmtExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_StmtExpr); 
    } break;
    case clang::Stmt::ImplicitValueInitExprClass: {
      assert(context_.data_model_.class_ImplicitValueInitExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ImplicitValueInitExpr); 
    } break;
    case clang::Stmt::NoInitExprClass: {
      assert(context_.data_model_.class_NoInitExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_NoInitExpr); 
    } break;
    case clang::Stmt::CallExprClass: {
      assert(context_.data_model_.class_CallExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CallExpr); 
    } break;
    case clang::Stmt::OMPScopeDirectiveClass: {
      assert(context_.data_model_.class_OMPScopeDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPScopeDirective); 
    } break;
    case clang::Stmt::DependentScopeDeclRefExprClass: {
      assert(context_.data_model_.class_DependentScopeDeclRefExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DependentScopeDeclRefExpr); 
    } break;
    case clang::Stmt::CXXDependentScopeMemberExprClass: {
      assert(context_.data_model_.class_CXXDependentScopeMemberExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXDependentScopeMemberExpr); 
    } break;
    case clang::Stmt::CXXThisExprClass: {
      assert(context_.data_model_.class_CXXThisExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXThisExpr); 
    } break;
    case clang::Stmt::PackExpansionExprClass: {
      assert(context_.data_model_.class_PackExpansionExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_PackExpansionExpr); 
    } break;
    case clang::Stmt::MaterializeTemporaryExprClass: {
      assert(context_.data_model_.class_MaterializeTemporaryExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_MaterializeTemporaryExpr); 
    } break;
    case clang::Stmt::CXXFoldExprClass: {
      assert(context_.data_model_.class_CXXFoldExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXFoldExpr); 
    } break;
    case clang::Stmt::OMPParallelMaskedTaskLoopSimdDirectiveClass: {
      assert(context_.data_model_.class_OMPParallelMaskedTaskLoopSimdDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPParallelMaskedTaskLoopSimdDirective); 
    } break;
    case clang::Stmt::LabelStmtClass: {
      assert(context_.data_model_.class_LabelStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_LabelStmt); 
    } break;
    case clang::Stmt::LambdaExprClass: {
      assert(context_.data_model_.class_LambdaExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_LambdaExpr); 
    } break;
    case clang::Stmt::DependentCoawaitExprClass: {
      assert(context_.data_model_.class_DependentCoawaitExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DependentCoawaitExpr); 
    } break;
    case clang::Stmt::SizeOfPackExprClass: {
      assert(context_.data_model_.class_SizeOfPackExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_SizeOfPackExpr); 
    } break;
    case clang::Stmt::RequiresExprClass: {
      assert(context_.data_model_.class_RequiresExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_RequiresExpr); 
    } break;
    case clang::Stmt::ObjCStringLiteralClass: {
      assert(context_.data_model_.class_ObjCStringLiteral != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCStringLiteral); 
    } break;
    case clang::Stmt::ObjCBoolLiteralExprClass: {
      assert(context_.data_model_.class_ObjCBoolLiteralExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCBoolLiteralExpr); 
    } break;
    case clang::Stmt::GNUNullExprClass: {
      assert(context_.data_model_.class_GNUNullExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_GNUNullExpr); 
    } break;
    case clang::Stmt::OMPTargetParallelForSimdDirectiveClass: {
      assert(context_.data_model_.class_OMPTargetParallelForSimdDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetParallelForSimdDirective); 
    } break;
    case clang::Stmt::AddrLabelExprClass: {
      assert(context_.data_model_.class_AddrLabelExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_AddrLabelExpr); 
    } break;
    case clang::Stmt::ConceptSpecializationExprClass: {
      assert(context_.data_model_.class_ConceptSpecializationExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ConceptSpecializationExpr); 
    } break;
    case clang::Stmt::OMPMetaDirectiveClass: {
      assert(context_.data_model_.class_OMPMetaDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPMetaDirective); 
    } break;
    case clang::Stmt::OMPAtomicDirectiveClass: {
      assert(context_.data_model_.class_OMPAtomicDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPAtomicDirective); 
    } break;
    case clang::Stmt::OMPTaskDirectiveClass: {
      assert(context_.data_model_.class_OMPTaskDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTaskDirective); 
    } break;
    case clang::Stmt::ShuffleVectorExprClass: {
      assert(context_.data_model_.class_ShuffleVectorExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ShuffleVectorExpr); 
    } break;
    case clang::Stmt::BinaryConditionalOperatorClass: {
      assert(context_.data_model_.class_BinaryConditionalOperator != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_BinaryConditionalOperator); 
    } break;
    case clang::Stmt::OMPTeamsDistributeSimdDirectiveClass: {
      assert(context_.data_model_.class_OMPTeamsDistributeSimdDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTeamsDistributeSimdDirective); 
    } break;
    case clang::Stmt::ParenExprClass: {
      assert(context_.data_model_.class_ParenExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ParenExpr); 
    } break;
    case clang::Stmt::PseudoObjectExprClass: {
      assert(context_.data_model_.class_PseudoObjectExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_PseudoObjectExpr); 
    } break;
    case clang::Stmt::SEHFinallyStmtClass: {
      assert(context_.data_model_.class_SEHFinallyStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_SEHFinallyStmt); 
    } break;
    case clang::Stmt::OMPArrayShapingExprClass: {
      assert(context_.data_model_.class_OMPArrayShapingExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPArrayShapingExpr); 
    } break;
    case clang::Stmt::UnresolvedLookupExprClass: {
      assert(context_.data_model_.class_UnresolvedLookupExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UnresolvedLookupExpr); 
    } break;
    case clang::Stmt::OMPTeamsDistributeParallelForSimdDirectiveClass: {
      assert(context_.data_model_.class_OMPTeamsDistributeParallelForSimdDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTeamsDistributeParallelForSimdDirective); 
    } break;
    case clang::Stmt::OMPTargetTeamsDistributeDirectiveClass: {
      assert(context_.data_model_.class_OMPTargetTeamsDistributeDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetTeamsDistributeDirective); 
    } break;
    case clang::Stmt::SubstNonTypeTemplateParmPackExprClass: {
      assert(context_.data_model_.class_SubstNonTypeTemplateParmPackExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_SubstNonTypeTemplateParmPackExpr); 
    } break;
    case clang::Stmt::DeclRefExprClass: {
      assert(context_.data_model_.class_DeclRefExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DeclRefExpr); 
    } break;
    case clang::Stmt::OMPTargetTeamsDistributeParallelForSimdDirectiveClass: {
      assert(context_.data_model_.class_OMPTargetTeamsDistributeParallelForSimdDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetTeamsDistributeParallelForSimdDirective); 
    } break;
    case clang::Stmt::CXXAddrspaceCastExprClass: {
      assert(context_.data_model_.class_CXXAddrspaceCastExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXAddrspaceCastExpr); 
    } break;
    case clang::Stmt::OMPGenericLoopDirectiveClass: {
      assert(context_.data_model_.class_OMPGenericLoopDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPGenericLoopDirective); 
    } break;
    case clang::Stmt::OMPDistributeParallelForSimdDirectiveClass: {
      assert(context_.data_model_.class_OMPDistributeParallelForSimdDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPDistributeParallelForSimdDirective); 
    } break;
    case clang::Stmt::OMPDispatchDirectiveClass: {
      assert(context_.data_model_.class_OMPDispatchDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPDispatchDirective); 
    } break;
    case clang::Stmt::OMPTargetTeamsDistributeParallelForDirectiveClass: {
      assert(context_.data_model_.class_OMPTargetTeamsDistributeParallelForDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetTeamsDistributeParallelForDirective); 
    } break;
    case clang::Stmt::UserDefinedLiteralClass: {
      assert(context_.data_model_.class_UserDefinedLiteral != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UserDefinedLiteral); 
    } break;
    case clang::Stmt::OMPMaskedDirectiveClass: {
      assert(context_.data_model_.class_OMPMaskedDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPMaskedDirective); 
    } break;
    case clang::Stmt::OMPTeamsGenericLoopDirectiveClass: {
      assert(context_.data_model_.class_OMPTeamsGenericLoopDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTeamsGenericLoopDirective); 
    } break;
    case clang::Stmt::OMPArraySectionExprClass: {
      assert(context_.data_model_.class_OMPArraySectionExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPArraySectionExpr); 
    } break;
    case clang::Stmt::OMPParallelGenericLoopDirectiveClass: {
      assert(context_.data_model_.class_OMPParallelGenericLoopDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPParallelGenericLoopDirective); 
    } break;
    case clang::Stmt::CXXTemporaryObjectExprClass: {
      assert(context_.data_model_.class_CXXTemporaryObjectExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXTemporaryObjectExpr); 
    } break;
    case clang::Stmt::OMPSectionsDirectiveClass: {
      assert(context_.data_model_.class_OMPSectionsDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPSectionsDirective); 
    } break;
    case clang::Stmt::ChooseExprClass: {
      assert(context_.data_model_.class_ChooseExpr != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ChooseExpr); 
    } break;
    case clang::Stmt::OMPDistributeSimdDirectiveClass: {
      assert(context_.data_model_.class_OMPDistributeSimdDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPDistributeSimdDirective); 
    } break;
    case clang::Stmt::OMPParallelMasterTaskLoopSimdDirectiveClass: {
      assert(context_.data_model_.class_OMPParallelMasterTaskLoopSimdDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPParallelMasterTaskLoopSimdDirective); 
    } break;
    case clang::Stmt::OMPMasterTaskLoopDirectiveClass: {
      assert(context_.data_model_.class_OMPMasterTaskLoopDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPMasterTaskLoopDirective); 
    } break;
    case clang::Stmt::OMPSectionDirectiveClass: {
      assert(context_.data_model_.class_OMPSectionDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPSectionDirective); 
    } break;
    case clang::Stmt::OMPTargetExitDataDirectiveClass: {
      assert(context_.data_model_.class_OMPTargetExitDataDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetExitDataDirective); 
    } break;
    case clang::Stmt::CXXCatchStmtClass: {
      assert(context_.data_model_.class_CXXCatchStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXCatchStmt); 
    } break;
    case clang::Stmt::OMPCancellationPointDirectiveClass: {
      assert(context_.data_model_.class_OMPCancellationPointDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPCancellationPointDirective); 
    } break;
    case clang::Stmt::FixedPointLiteralClass: {
      assert(context_.data_model_.class_FixedPointLiteral != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_FixedPointLiteral); 
    } break;
    case clang::Stmt::OMPDistributeDirectiveClass: {
      assert(context_.data_model_.class_OMPDistributeDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPDistributeDirective); 
    } break;
    case clang::Stmt::CompoundStmtClass: {
      assert(context_.data_model_.class_CompoundStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CompoundStmt); 
    } break;
    case clang::Stmt::OMPTaskLoopDirectiveClass: {
      assert(context_.data_model_.class_OMPTaskLoopDirective != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTaskLoopDirective); 
    } break;
    case clang::Stmt::ContinueStmtClass: {
      assert(context_.data_model_.class_ContinueStmt != nullptr);
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ContinueStmt); 
    } break;
    default: break;
  }

  //stripLabelLikeStatements
  {
    const Id* other = context_.resolve(D->stripLabelLikeStatements());
    arboretum_create_edge(obj, context_.data_model_.method_stripLabelLikeStatements, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitStmtExpr(clang::StmtExpr* D) {
  const Id* obj = context_.resolve(D);
  //getSubStmt
  {
    const Id* other = context_.resolve(D->getSubStmt());
    arboretum_create_edge(obj, context_.data_model_.method_getSubStmt_5, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_118, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_117, other);
  }
  //getLParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getLParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getLParenLoc_9, other);
  }
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_24, other);
  }
  //getTemplateDepth
  // unsigned int
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitStringLiteral(clang::StringLiteral* D) {
  const Id* obj = context_.resolve(D);
  //getString
  // StringRef
  //getBytes
  // StringRef
  //getByteLength
  // unsigned int
  //getLength
  // unsigned int
  //getCharByteWidth
  // unsigned int
  //getKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getKind_4, enum_value);
    }
  }
  //isOrdinary
  arboretum_create_edge(obj, context_.data_model_.method_isOrdinary, context_.data_model_.arboretum_node_for(D->isOrdinary()));
  //isWide
  arboretum_create_edge(obj, context_.data_model_.method_isWide, context_.data_model_.arboretum_node_for(D->isWide()));
  //isUTF8
  arboretum_create_edge(obj, context_.data_model_.method_isUTF8, context_.data_model_.arboretum_node_for(D->isUTF8()));
  //isUTF16
  arboretum_create_edge(obj, context_.data_model_.method_isUTF16, context_.data_model_.arboretum_node_for(D->isUTF16()));
  //isUTF32
  arboretum_create_edge(obj, context_.data_model_.method_isUTF32, context_.data_model_.arboretum_node_for(D->isUTF32()));
  //isUnevaluated
  arboretum_create_edge(obj, context_.data_model_.method_isUnevaluated, context_.data_model_.arboretum_node_for(D->isUnevaluated()));
  //isPascal
  arboretum_create_edge(obj, context_.data_model_.method_isPascal, context_.data_model_.arboretum_node_for(D->isPascal()));
  //containsNonAscii
  arboretum_create_edge(obj, context_.data_model_.method_containsNonAscii, context_.data_model_.arboretum_node_for(D->containsNonAscii()));
  //containsNonAsciiOrNull
  arboretum_create_edge(obj, context_.data_model_.method_containsNonAsciiOrNull, context_.data_model_.arboretum_node_for(D->containsNonAsciiOrNull()));
  //getNumConcatenated
  // unsigned int
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_119, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_118, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitSubstNonTypeTemplateParmExpr(clang::SubstNonTypeTemplateParmExpr* D) {
  const Id* obj = context_.resolve(D);
  //getNameLoc
  {
    const Id* other = context_.source_model_.resolve(D->getNameLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getNameLoc_1, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_120, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_119, other);
  }
  //getReplacement
  {
    const Id* other = context_.resolve(D->getReplacement());
    arboretum_create_edge(obj, context_.data_model_.method_getReplacement, other);
  }
  //getAssociatedDecl
  {
    const Id* other = context_.resolve(D->getAssociatedDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getAssociatedDecl_2, other);
  }
  //getIndex
  // unsigned int
  //getPackIndex
  // std::optional<unsigned int>
  //getParameter
  {
    const Id* other = context_.resolve(D->getParameter());
    arboretum_create_edge(obj, context_.data_model_.method_getParameter, other);
  }
  //isReferenceParameter
  arboretum_create_edge(obj, context_.data_model_.method_isReferenceParameter, context_.data_model_.arboretum_node_for(D->isReferenceParameter()));
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitSubstNonTypeTemplateParmPackExpr(clang::SubstNonTypeTemplateParmPackExpr* D) {
  const Id* obj = context_.resolve(D);
  //getAssociatedDecl
  {
    const Id* other = context_.resolve(D->getAssociatedDecl());
    arboretum_create_edge(obj, context_.data_model_.method_getAssociatedDecl_3, other);
  }
  //getIndex
  // unsigned int
  //getParameterPack
  {
    const Id* other = context_.resolve(D->getParameterPack());
    arboretum_create_edge(obj, context_.data_model_.method_getParameterPack_1, other);
  }
  //getParameterPackLocation
  {
    const Id* other = context_.source_model_.resolve(D->getParameterPackLocation());
    arboretum_create_edge(obj, context_.data_model_.method_getParameterPackLocation_1, other);
  }
  //getArgumentPack
  // TemplateArgument
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_121, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_120, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitSwitchCase(clang::SwitchCase* D) {
  const Id* obj = context_.resolve(D);
  //getNextSwitchCase
  {
    const Id* other = context_.resolve(D->getNextSwitchCase());
    arboretum_create_edge(obj, context_.data_model_.method_getNextSwitchCase, other);
  }
  //getKeywordLoc
  {
    const Id* other = context_.source_model_.resolve(D->getKeywordLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getKeywordLoc_4, other);
  }
  //getColonLoc
  {
    const Id* other = context_.source_model_.resolve(D->getColonLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getColonLoc_3, other);
  }
  //getSubStmt
  {
    const Id* other = context_.resolve(D->getSubStmt());
    arboretum_create_edge(obj, context_.data_model_.method_getSubStmt_6, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_122, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_121, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitSwitchStmt(clang::SwitchStmt* D) {
  const Id* obj = context_.resolve(D);
  //hasInitStorage
  arboretum_create_edge(obj, context_.data_model_.method_hasInitStorage_1, context_.data_model_.arboretum_node_for(D->hasInitStorage()));
  //hasVarStorage
  arboretum_create_edge(obj, context_.data_model_.method_hasVarStorage_1, context_.data_model_.arboretum_node_for(D->hasVarStorage()));
  //getCond
  {
    const Id* other = context_.resolve(D->getCond());
    arboretum_create_edge(obj, context_.data_model_.method_getCond_8, other);
  }
  //getBody
  {
    const Id* other = context_.resolve(D->getBody());
    arboretum_create_edge(obj, context_.data_model_.method_getBody_11, other);
  }
  //getInit
  {
    const Id* other = context_.resolve(D->getInit());
    arboretum_create_edge(obj, context_.data_model_.method_getInit_6, other);
  }
  //getConditionVariable
  {
    const Id* other = context_.resolve(D->getConditionVariable());
    arboretum_create_edge(obj, context_.data_model_.method_getConditionVariable_2, other);
  }
  //getConditionVariableDeclStmt
  {
    const Id* other = context_.resolve(D->getConditionVariableDeclStmt());
    arboretum_create_edge(obj, context_.data_model_.method_getConditionVariableDeclStmt_2, other);
  }
  //getSwitchCaseList
  {
    const Id* other = context_.resolve(D->getSwitchCaseList());
    arboretum_create_edge(obj, context_.data_model_.method_getSwitchCaseList, other);
  }
  //getSwitchLoc
  {
    const Id* other = context_.source_model_.resolve(D->getSwitchLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getSwitchLoc, other);
  }
  //getLParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getLParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getLParenLoc_10, other);
  }
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_25, other);
  }
  //isAllEnumCasesCovered
  arboretum_create_edge(obj, context_.data_model_.method_isAllEnumCasesCovered, context_.data_model_.arboretum_node_for(D->isAllEnumCasesCovered()));
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_123, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_122, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitTypeTraitExpr(clang::TypeTraitExpr* D) {
  const Id* obj = context_.resolve(D);
  //getTrait
  {
    const Id* enum_value = context_.data_model_.resolve(D->getTrait());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getTrait_2, enum_value);
    }
  }
  //getNumArgs
  // unsigned int
  //getArgs
  // ArrayRef<TypeSourceInfo *>
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_124, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_123, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitTypoExpr(clang::TypoExpr* D) {
  const Id* obj = context_.resolve(D);
  //children
  // const_child_range
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_125, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_124, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitUnaryExprOrTypeTraitExpr(clang::UnaryExprOrTypeTraitExpr* D) {
  const Id* obj = context_.resolve(D);
  //getKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getKind_5, enum_value);
    }
  }
  //isArgumentType
  arboretum_create_edge(obj, context_.data_model_.method_isArgumentType, context_.data_model_.arboretum_node_for(D->isArgumentType()));
  //getTypeOfArgument
  {
    const Id* other = context_.resolve(D->getTypeOfArgument());
    arboretum_create_edge(obj, context_.data_model_.method_getTypeOfArgument, other);
  }
  //getOperatorLoc
  {
    const Id* other = context_.source_model_.resolve(D->getOperatorLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getOperatorLoc_9, other);
  }
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_26, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_126, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_125, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitUnaryOperator(clang::UnaryOperator* D) {
  const Id* obj = context_.resolve(D);
  //getOpcode
  {
    const Id* enum_value = context_.data_model_.resolve(D->getOpcode());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getOpcode_2, enum_value);
    }
  }
  //getSubExpr
  {
    const Id* other = context_.resolve(D->getSubExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getSubExpr_9, other);
  }
  //getOperatorLoc
  {
    const Id* other = context_.source_model_.resolve(D->getOperatorLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getOperatorLoc_10, other);
  }
  //canOverflow
  arboretum_create_edge(obj, context_.data_model_.method_canOverflow, context_.data_model_.arboretum_node_for(D->canOverflow()));
  //isPrefix
  arboretum_create_edge(obj, context_.data_model_.method_isPrefix, context_.data_model_.arboretum_node_for(D->isPrefix()));
  //isPostfix
  arboretum_create_edge(obj, context_.data_model_.method_isPostfix, context_.data_model_.arboretum_node_for(D->isPostfix()));
  //isIncrementOp
  arboretum_create_edge(obj, context_.data_model_.method_isIncrementOp, context_.data_model_.arboretum_node_for(D->isIncrementOp()));
  //isDecrementOp
  arboretum_create_edge(obj, context_.data_model_.method_isDecrementOp, context_.data_model_.arboretum_node_for(D->isDecrementOp()));
  //isIncrementDecrementOp
  arboretum_create_edge(obj, context_.data_model_.method_isIncrementDecrementOp, context_.data_model_.arboretum_node_for(D->isIncrementDecrementOp()));
  //isArithmeticOp
  arboretum_create_edge(obj, context_.data_model_.method_isArithmeticOp, context_.data_model_.arboretum_node_for(D->isArithmeticOp()));
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_127, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_126, other);
  }
  //getExprLoc
  {
    const Id* other = context_.source_model_.resolve(D->getExprLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getExprLoc_12, other);
  }
  //children
  // const_child_range
  //hasStoredFPFeatures
  arboretum_create_edge(obj, context_.data_model_.method_hasStoredFPFeatures_4, context_.data_model_.arboretum_node_for(D->hasStoredFPFeatures()));
  //getStoredFPFeatures
  // FPOptionsOverride
  //getFPOptionsOverride
  // FPOptionsOverride
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedLookupExpr(clang::UnresolvedLookupExpr* D) {
  const Id* obj = context_.resolve(D);
  //requiresADL
  arboretum_create_edge(obj, context_.data_model_.method_requiresADL, context_.data_model_.arboretum_node_for(D->requiresADL()));
  //isOverloaded
  arboretum_create_edge(obj, context_.data_model_.method_isOverloaded, context_.data_model_.arboretum_node_for(D->isOverloaded()));
  //getNamingClass
  {
    const Id* other = context_.resolve(D->getNamingClass());
    arboretum_create_edge(obj, context_.data_model_.method_getNamingClass_1, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_128, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_127, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedMemberExpr(clang::UnresolvedMemberExpr* D) {
  const Id* obj = context_.resolve(D);
  //isImplicitAccess
  arboretum_create_edge(obj, context_.data_model_.method_isImplicitAccess_3, context_.data_model_.arboretum_node_for(D->isImplicitAccess()));
  //getBaseType
  {
    const Id* other = context_.resolve(D->getBaseType());
    arboretum_create_edge(obj, context_.data_model_.method_getBaseType_2, other);
  }
  //hasUnresolvedUsing
  arboretum_create_edge(obj, context_.data_model_.method_hasUnresolvedUsing, context_.data_model_.arboretum_node_for(D->hasUnresolvedUsing()));
  //isArrow
  arboretum_create_edge(obj, context_.data_model_.method_isArrow_5, context_.data_model_.arboretum_node_for(D->isArrow()));
  //getOperatorLoc
  {
    const Id* other = context_.source_model_.resolve(D->getOperatorLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getOperatorLoc_11, other);
  }
  //getNamingClass
  {
    const Id* other = context_.resolve(D->getNamingClass());
    arboretum_create_edge(obj, context_.data_model_.method_getNamingClass_2, other);
  }
  //getMemberNameInfo
  // const DeclarationNameInfo &
  //getMemberName
  // DeclarationName
  //getMemberLoc
  {
    const Id* other = context_.source_model_.resolve(D->getMemberLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getMemberLoc_3, other);
  }
  //getExprLoc
  {
    const Id* other = context_.source_model_.resolve(D->getExprLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getExprLoc_13, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_129, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_128, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitUserDefinedLiteral(clang::UserDefinedLiteral* D) {
  const Id* obj = context_.resolve(D);
  //getLiteralOperatorKind
  {
    const Id* enum_value = context_.data_model_.resolve(D->getLiteralOperatorKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.method_getLiteralOperatorKind, enum_value);
    }
  }
  //getCookedLiteral
  {
    const Id* other = context_.resolve(D->getCookedLiteral());
    arboretum_create_edge(obj, context_.data_model_.method_getCookedLiteral, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_130, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_129, other);
  }
  //getUDSuffixLoc
  {
    const Id* other = context_.source_model_.resolve(D->getUDSuffixLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getUDSuffixLoc, other);
  }
  //getUDSuffix
  return true;
}

bool ArboretumASTVisitor::VisitVAArgExpr(clang::VAArgExpr* D) {
  const Id* obj = context_.resolve(D);
  //getSubExpr
  {
    const Id* other = context_.resolve(D->getSubExpr());
    arboretum_create_edge(obj, context_.data_model_.method_getSubExpr_10, other);
  }
  //isMicrosoftABI
  arboretum_create_edge(obj, context_.data_model_.method_isMicrosoftABI, context_.data_model_.arboretum_node_for(D->isMicrosoftABI()));
  //getWrittenTypeInfo
  //getBuiltinLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBuiltinLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBuiltinLoc_5, other);
  }
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_27, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_131, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_130, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitValueStmt(clang::ValueStmt* D) {
  const Id* obj = context_.resolve(D);
  //getExprStmt
  {
    const Id* other = context_.resolve(D->getExprStmt());
    arboretum_create_edge(obj, context_.data_model_.method_getExprStmt, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitWhileStmt(clang::WhileStmt* D) {
  const Id* obj = context_.resolve(D);
  //hasVarStorage
  arboretum_create_edge(obj, context_.data_model_.method_hasVarStorage_2, context_.data_model_.arboretum_node_for(D->hasVarStorage()));
  //getCond
  {
    const Id* other = context_.resolve(D->getCond());
    arboretum_create_edge(obj, context_.data_model_.method_getCond_9, other);
  }
  //getBody
  {
    const Id* other = context_.resolve(D->getBody());
    arboretum_create_edge(obj, context_.data_model_.method_getBody_12, other);
  }
  //getConditionVariable
  {
    const Id* other = context_.resolve(D->getConditionVariable());
    arboretum_create_edge(obj, context_.data_model_.method_getConditionVariable_3, other);
  }
  //getConditionVariableDeclStmt
  {
    const Id* other = context_.resolve(D->getConditionVariableDeclStmt());
    arboretum_create_edge(obj, context_.data_model_.method_getConditionVariableDeclStmt_3, other);
  }
  //getWhileLoc
  {
    const Id* other = context_.source_model_.resolve(D->getWhileLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getWhileLoc_1, other);
  }
  //getLParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getLParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getLParenLoc_11, other);
  }
  //getRParenLoc
  {
    const Id* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_28, other);
  }
  //getBeginLoc
  {
    const Id* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_132, other);
  }
  //getEndLoc
  {
    const Id* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_131, other);
  }
  //children
  // const_child_range
  return true;
}

////   END ARBORETUM GENERATED CODE ////
} // namespace arboretum