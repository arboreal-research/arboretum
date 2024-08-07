#include "arboretum_ast_visitor.h"

#include "arboretum_ffi.h"

namespace arboretum {

//// BEGIN ARBORETUM GENERATED CODE ////
// Types
bool ArboretumASTVisitor::VisitAdjustedType(clang::AdjustedType* D) {
  const Entity* obj = context_.resolve(D);
  //getOriginalType
  {
    const Entity* other = context_.resolve(D->getOriginalType());
    arboretum_create_edge(obj, context_.data_model_.getOriginalType, other);
  }
  //getAdjustedType
  {
    const Entity* other = context_.resolve(D->getAdjustedType());
    arboretum_create_edge(obj, context_.data_model_.getAdjustedType, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared38, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar38, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitArrayType(clang::ArrayType* D) {
  const Entity* obj = context_.resolve(D);
  //getElementType
  {
    const Entity* other = context_.resolve(D->getElementType());
    arboretum_create_edge(obj, context_.data_model_.getElementType2, other);
  }
  //getSizeModifier
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getSizeModifier());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getSizeModifier, enum_value);
    }
  }
  //getIndexTypeQualifiers
  // Qualifiers
  //getIndexTypeCVRQualifiers
  // unsigned int
  return true;
}

bool ArboretumASTVisitor::VisitAtomicType(clang::AtomicType* D) {
  const Entity* obj = context_.resolve(D);
  //getValueType
  {
    const Entity* other = context_.resolve(D->getValueType());
    arboretum_create_edge(obj, context_.data_model_.getValueType1, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared19, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar19, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitAttributedType(clang::AttributedType* D) {
  const Entity* obj = context_.resolve(D);
  //getAttrKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getAttrKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getAttrKind1, enum_value);
    }
  }
  //getModifiedType
  {
    const Entity* other = context_.resolve(D->getModifiedType());
    arboretum_create_edge(obj, context_.data_model_.getModifiedType1, other);
  }
  //getEquivalentType
  {
    const Entity* other = context_.resolve(D->getEquivalentType());
    arboretum_create_edge(obj, context_.data_model_.getEquivalentType, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared21, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar21, other);
  }
  //isQualifier
  arboretum_create_edge(obj, context_.data_model_.isQualifier1, context_.data_model_.arboretum_node_for(D->isQualifier()));
  //isMSTypeSpec
  arboretum_create_edge(obj, context_.data_model_.isMSTypeSpec, context_.data_model_.arboretum_node_for(D->isMSTypeSpec()));
  //isWebAssemblyFuncrefSpec
  arboretum_create_edge(obj, context_.data_model_.isWebAssemblyFuncrefSpec, context_.data_model_.arboretum_node_for(D->isWebAssemblyFuncrefSpec()));
  //isCallingConv
  arboretum_create_edge(obj, context_.data_model_.isCallingConv, context_.data_model_.arboretum_node_for(D->isCallingConv()));
  //getImmediateNullability
  // std::optional<NullabilityKind>
  return true;
}

bool ArboretumASTVisitor::VisitAutoType(clang::AutoType* D) {
  const Entity* obj = context_.resolve(D);
  //getTypeConstraintArguments
  // ArrayRef<TemplateArgument>
  //getTypeConstraintConcept
  {
    const Entity* other = context_.resolve(D->getTypeConstraintConcept());
    arboretum_create_edge(obj, context_.data_model_.getTypeConstraintConcept, other);
  }
  //isConstrained
  arboretum_create_edge(obj, context_.data_model_.isConstrained, context_.data_model_.arboretum_node_for(D->isConstrained()));
  //isDecltypeAuto
  arboretum_create_edge(obj, context_.data_model_.isDecltypeAuto, context_.data_model_.arboretum_node_for(D->isDecltypeAuto()));
  //isGNUAutoType
  arboretum_create_edge(obj, context_.data_model_.isGNUAutoType, context_.data_model_.arboretum_node_for(D->isGNUAutoType()));
  //getKeyword
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getKeyword());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getKeyword1, enum_value);
    }
  }
  return true;
}

bool ArboretumASTVisitor::VisitBTFTagAttributedType(clang::BTFTagAttributedType* D) {
  const Entity* obj = context_.resolve(D);
  //getWrappedType
  {
    const Entity* other = context_.resolve(D->getWrappedType());
    arboretum_create_edge(obj, context_.data_model_.getWrappedType, other);
  }
  //getAttr
  {
    const Entity* other = context_.resolve(D->getAttr());
    arboretum_create_edge(obj, context_.data_model_.getAttr1, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared15, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar15, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitBitIntType(clang::BitIntType* D) {
  const Entity* obj = context_.resolve(D);
  //isUnsigned
  arboretum_create_edge(obj, context_.data_model_.isUnsigned, context_.data_model_.arboretum_node_for(D->isUnsigned()));
  //isSigned
  arboretum_create_edge(obj, context_.data_model_.isSigned, context_.data_model_.arboretum_node_for(D->isSigned()));
  //getNumBits
  // unsigned int
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared4, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar4, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitBlockPointerType(clang::BlockPointerType* D) {
  const Entity* obj = context_.resolve(D);
  //getPointeeType
  {
    const Entity* other = context_.resolve(D->getPointeeType());
    arboretum_create_edge(obj, context_.data_model_.getPointeeType1, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared5, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar5, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitBuiltinType(clang::BuiltinType* D) {
  const Entity* obj = context_.resolve(D);
  //getKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getKind8, enum_value);
    }
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared18, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar18, other);
  }
  //isInteger
  arboretum_create_edge(obj, context_.data_model_.isInteger, context_.data_model_.arboretum_node_for(D->isInteger()));
  //isSignedInteger
  arboretum_create_edge(obj, context_.data_model_.isSignedInteger, context_.data_model_.arboretum_node_for(D->isSignedInteger()));
  //isUnsignedInteger
  arboretum_create_edge(obj, context_.data_model_.isUnsignedInteger, context_.data_model_.arboretum_node_for(D->isUnsignedInteger()));
  //isFloatingPoint
  arboretum_create_edge(obj, context_.data_model_.isFloatingPoint, context_.data_model_.arboretum_node_for(D->isFloatingPoint()));
  //isSVEBool
  arboretum_create_edge(obj, context_.data_model_.isSVEBool, context_.data_model_.arboretum_node_for(D->isSVEBool()));
  //isSVECount
  arboretum_create_edge(obj, context_.data_model_.isSVECount, context_.data_model_.arboretum_node_for(D->isSVECount()));
  //isPlaceholderType
  arboretum_create_edge(obj, context_.data_model_.isPlaceholderType1, context_.data_model_.arboretum_node_for(D->isPlaceholderType()));
  //isNonOverloadPlaceholderType
  arboretum_create_edge(obj, context_.data_model_.isNonOverloadPlaceholderType1, context_.data_model_.arboretum_node_for(D->isNonOverloadPlaceholderType()));
  return true;
}

bool ArboretumASTVisitor::VisitComplexType(clang::ComplexType* D) {
  const Entity* obj = context_.resolve(D);
  //getElementType
  {
    const Entity* other = context_.resolve(D->getElementType());
    arboretum_create_edge(obj, context_.data_model_.getElementType, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared2, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar2, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitConstantArrayType(clang::ConstantArrayType* D) {
  const Entity* obj = context_.resolve(D);
  //getSize
  // const llvm::APInt &
  //getSizeExpr
  {
    const Entity* other = context_.resolve(D->getSizeExpr());
    arboretum_create_edge(obj, context_.data_model_.getSizeExpr1, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared34, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar34, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitConstantMatrixType(clang::ConstantMatrixType* D) {
  const Entity* obj = context_.resolve(D);
  //getNumRows
  // unsigned int
  //getNumColumns
  // unsigned int
  //getNumElementsFlattened
  // unsigned int
  return true;
}

bool ArboretumASTVisitor::VisitDecayedType(clang::DecayedType* D) {
  const Entity* obj = context_.resolve(D);
  //getDecayedType
  {
    const Entity* other = context_.resolve(D->getDecayedType());
    arboretum_create_edge(obj, context_.data_model_.getDecayedType, other);
  }
  //getPointeeType
  {
    const Entity* other = context_.resolve(D->getPointeeType());
    arboretum_create_edge(obj, context_.data_model_.getPointeeType4, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitDecltypeType(clang::DecltypeType* D) {
  const Entity* obj = context_.resolve(D);
  //getUnderlyingExpr
  {
    const Entity* other = context_.resolve(D->getUnderlyingExpr());
    arboretum_create_edge(obj, context_.data_model_.getUnderlyingExpr1, other);
  }
  //getUnderlyingType
  {
    const Entity* other = context_.resolve(D->getUnderlyingType());
    arboretum_create_edge(obj, context_.data_model_.getUnderlyingType2, other);
  }
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar17, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared17, context_.data_model_.arboretum_node_for(D->isSugared()));
  return true;
}

bool ArboretumASTVisitor::VisitDeducedTemplateSpecializationType(clang::DeducedTemplateSpecializationType* D) {
  const Entity* obj = context_.resolve(D);
  //getTemplateName
  // TemplateName
  return true;
}

bool ArboretumASTVisitor::VisitDeducedType(clang::DeducedType* D) {
  const Entity* obj = context_.resolve(D);
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared43, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar43, other);
  }
  //getDeducedType
  {
    const Entity* other = context_.resolve(D->getDeducedType());
    arboretum_create_edge(obj, context_.data_model_.getDeducedType, other);
  }
  //isDeduced
  arboretum_create_edge(obj, context_.data_model_.isDeduced, context_.data_model_.arboretum_node_for(D->isDeduced()));
  return true;
}

bool ArboretumASTVisitor::VisitDependentAddressSpaceType(clang::DependentAddressSpaceType* D) {
  const Entity* obj = context_.resolve(D);
  //getAddrSpaceExpr
  {
    const Entity* other = context_.resolve(D->getAddrSpaceExpr());
    arboretum_create_edge(obj, context_.data_model_.getAddrSpaceExpr, other);
  }
  //getPointeeType
  {
    const Entity* other = context_.resolve(D->getPointeeType());
    arboretum_create_edge(obj, context_.data_model_.getPointeeType6, other);
  }
  //getAttributeLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getAttributeLoc());
    arboretum_create_edge(obj, context_.data_model_.getAttributeLoc3, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared42, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar42, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitDependentBitIntType(clang::DependentBitIntType* D) {
  const Entity* obj = context_.resolve(D);
  //isUnsigned
  arboretum_create_edge(obj, context_.data_model_.isUnsigned1, context_.data_model_.arboretum_node_for(D->isUnsigned()));
  //isSigned
  arboretum_create_edge(obj, context_.data_model_.isSigned1, context_.data_model_.arboretum_node_for(D->isSigned()));
  //getNumBitsExpr
  {
    const Entity* other = context_.resolve(D->getNumBitsExpr());
    arboretum_create_edge(obj, context_.data_model_.getNumBitsExpr, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared28, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar28, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitDependentDecltypeType(clang::DependentDecltypeType* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitDependentNameType(clang::DependentNameType* D) {
  const Entity* obj = context_.resolve(D);
  //getQualifier
  //getIdentifier
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared33, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar33, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitDependentSizedArrayType(clang::DependentSizedArrayType* D) {
  const Entity* obj = context_.resolve(D);
  //getSizeExpr
  {
    const Entity* other = context_.resolve(D->getSizeExpr());
    arboretum_create_edge(obj, context_.data_model_.getSizeExpr4, other);
  }
  //getBracketsRange
  {
    const Entity* other = context_.source_model_.resolve(D->getBracketsRange());
    arboretum_create_edge(obj, context_.data_model_.getBracketsRange1, other);
  }
  //getLBracketLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getLBracketLoc());
    arboretum_create_edge(obj, context_.data_model_.getLBracketLoc1, other);
  }
  //getRBracketLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRBracketLoc());
    arboretum_create_edge(obj, context_.data_model_.getRBracketLoc5, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared44, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar44, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitDependentSizedExtVectorType(clang::DependentSizedExtVectorType* D) {
  const Entity* obj = context_.resolve(D);
  //getSizeExpr
  {
    const Entity* other = context_.resolve(D->getSizeExpr());
    arboretum_create_edge(obj, context_.data_model_.getSizeExpr2, other);
  }
  //getElementType
  {
    const Entity* other = context_.resolve(D->getElementType());
    arboretum_create_edge(obj, context_.data_model_.getElementType4, other);
  }
  //getAttributeLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getAttributeLoc());
    arboretum_create_edge(obj, context_.data_model_.getAttributeLoc1, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared39, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar39, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitDependentSizedMatrixType(clang::DependentSizedMatrixType* D) {
  const Entity* obj = context_.resolve(D);
  //getRowExpr
  {
    const Entity* other = context_.resolve(D->getRowExpr());
    arboretum_create_edge(obj, context_.data_model_.getRowExpr, other);
  }
  //getColumnExpr
  {
    const Entity* other = context_.resolve(D->getColumnExpr());
    arboretum_create_edge(obj, context_.data_model_.getColumnExpr, other);
  }
  //getAttributeLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getAttributeLoc());
    arboretum_create_edge(obj, context_.data_model_.getAttributeLoc, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitDependentTemplateSpecializationType(clang::DependentTemplateSpecializationType* D) {
  const Entity* obj = context_.resolve(D);
  //getQualifier
  //getIdentifier
  //template_arguments
  // ArrayRef<TemplateArgument>
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared7, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar7, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitDependentTypeOfExprType(clang::DependentTypeOfExprType* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitDependentUnaryTransformType(clang::DependentUnaryTransformType* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitDependentVectorType(clang::DependentVectorType* D) {
  const Entity* obj = context_.resolve(D);
  //getSizeExpr
  {
    const Entity* other = context_.resolve(D->getSizeExpr());
    arboretum_create_edge(obj, context_.data_model_.getSizeExpr3, other);
  }
  //getElementType
  {
    const Entity* other = context_.resolve(D->getElementType());
    arboretum_create_edge(obj, context_.data_model_.getElementType5, other);
  }
  //getAttributeLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getAttributeLoc());
    arboretum_create_edge(obj, context_.data_model_.getAttributeLoc2, other);
  }
  //getVectorKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getVectorKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getVectorKind1, enum_value);
    }
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared41, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar41, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitElaboratedType(clang::ElaboratedType* D) {
  const Entity* obj = context_.resolve(D);
  //getQualifier
  //getNamedType
  {
    const Entity* other = context_.resolve(D->getNamedType());
    arboretum_create_edge(obj, context_.data_model_.getNamedType, other);
  }
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar27, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared27, context_.data_model_.arboretum_node_for(D->isSugared()));
  //getOwnedTagDecl
  {
    const Entity* other = context_.resolve(D->getOwnedTagDecl());
    arboretum_create_edge(obj, context_.data_model_.getOwnedTagDecl, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitEnumType(clang::EnumType* D) {
  const Entity* obj = context_.resolve(D);
  //getDecl
  {
    const Entity* other = context_.resolve(D->getDecl());
    arboretum_create_edge(obj, context_.data_model_.getDecl11, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared31, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar31, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitExtVectorType(clang::ExtVectorType* D) {
  const Entity* obj = context_.resolve(D);
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared9, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar9, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitFunctionNoProtoType(clang::FunctionNoProtoType* D) {
  const Entity* obj = context_.resolve(D);
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared20, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar20, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitFunctionProtoType(clang::FunctionProtoType* D) {
  const Entity* obj = context_.resolve(D);
  //getNumParams
  // unsigned int
  //getParamTypes
  // ArrayRef<QualType>
  //getExtProtoInfo
  // ExtProtoInfo
  //getExceptionSpecType
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getExceptionSpecType());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getExceptionSpecType1, enum_value);
    }
  }
  //hasExceptionSpec
  arboretum_create_edge(obj, context_.data_model_.hasExceptionSpec, context_.data_model_.arboretum_node_for(D->hasExceptionSpec()));
  //hasDynamicExceptionSpec
  arboretum_create_edge(obj, context_.data_model_.hasDynamicExceptionSpec, context_.data_model_.arboretum_node_for(D->hasDynamicExceptionSpec()));
  //hasNoexceptExceptionSpec
  arboretum_create_edge(obj, context_.data_model_.hasNoexceptExceptionSpec, context_.data_model_.arboretum_node_for(D->hasNoexceptExceptionSpec()));
  //hasDependentExceptionSpec
  arboretum_create_edge(obj, context_.data_model_.hasDependentExceptionSpec, context_.data_model_.arboretum_node_for(D->hasDependentExceptionSpec()));
  //hasInstantiationDependentExceptionSpec
  arboretum_create_edge(obj, context_.data_model_.hasInstantiationDependentExceptionSpec, context_.data_model_.arboretum_node_for(D->hasInstantiationDependentExceptionSpec()));
  //getExceptionSpecInfo
  // ExceptionSpecInfo
  //getNumExceptions
  // unsigned int
  //getNoexceptExpr
  {
    const Entity* other = context_.resolve(D->getNoexceptExpr());
    arboretum_create_edge(obj, context_.data_model_.getNoexceptExpr, other);
  }
  //getExceptionSpecDecl
  {
    const Entity* other = context_.resolve(D->getExceptionSpecDecl());
    arboretum_create_edge(obj, context_.data_model_.getExceptionSpecDecl, other);
  }
  //getExceptionSpecTemplate
  {
    const Entity* other = context_.resolve(D->getExceptionSpecTemplate());
    arboretum_create_edge(obj, context_.data_model_.getExceptionSpecTemplate, other);
  }
  //canThrow
  {
    const Entity* enum_value = context_.data_model_.resolve(D->canThrow());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.canThrow, enum_value);
    }
  }
  //isVariadic
  arboretum_create_edge(obj, context_.data_model_.isVariadic2, context_.data_model_.arboretum_node_for(D->isVariadic()));
  //getEllipsisLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEllipsisLoc());
    arboretum_create_edge(obj, context_.data_model_.getEllipsisLoc5, other);
  }
  //isTemplateVariadic
  arboretum_create_edge(obj, context_.data_model_.isTemplateVariadic, context_.data_model_.arboretum_node_for(D->isTemplateVariadic()));
  //hasTrailingReturn
  arboretum_create_edge(obj, context_.data_model_.hasTrailingReturn, context_.data_model_.arboretum_node_for(D->hasTrailingReturn()));
  //getMethodQuals
  // Qualifiers
  //getRefQualifier
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getRefQualifier());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getRefQualifier1, enum_value);
    }
  }
  //param_types
  // ArrayRef<QualType>
  //exceptions
  // ArrayRef<QualType>
  //hasExtParameterInfos
  arboretum_create_edge(obj, context_.data_model_.hasExtParameterInfos, context_.data_model_.arboretum_node_for(D->hasExtParameterInfos()));
  //getExtParameterInfos
  // ArrayRef<ExtParameterInfo>
  //getExtParameterInfosOrNull
  //getAArch64SMEAttributes
  // unsigned int
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared1, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar1, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitFunctionType(clang::FunctionType* D) {
  const Entity* obj = context_.resolve(D);
  //getReturnType
  {
    const Entity* other = context_.resolve(D->getReturnType());
    arboretum_create_edge(obj, context_.data_model_.getReturnType2, other);
  }
  //getHasRegParm
  arboretum_create_edge(obj, context_.data_model_.getHasRegParm, context_.data_model_.arboretum_node_for(D->getHasRegParm()));
  //getRegParmType
  // unsigned int
  //getNoReturnAttr
  arboretum_create_edge(obj, context_.data_model_.getNoReturnAttr, context_.data_model_.arboretum_node_for(D->getNoReturnAttr()));
  //getCmseNSCallAttr
  arboretum_create_edge(obj, context_.data_model_.getCmseNSCallAttr, context_.data_model_.arboretum_node_for(D->getCmseNSCallAttr()));
  //getCallConv
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getCallConv());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getCallConv, enum_value);
    }
  }
  //getExtInfo
  // ExtInfo
  //isConst
  arboretum_create_edge(obj, context_.data_model_.isConst1, context_.data_model_.arboretum_node_for(D->isConst()));
  //isVolatile
  arboretum_create_edge(obj, context_.data_model_.isVolatile3, context_.data_model_.arboretum_node_for(D->isVolatile()));
  //isRestrict
  arboretum_create_edge(obj, context_.data_model_.isRestrict, context_.data_model_.arboretum_node_for(D->isRestrict()));
  return true;
}

bool ArboretumASTVisitor::VisitIncompleteArrayType(clang::IncompleteArrayType* D) {
  const Entity* obj = context_.resolve(D);
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared45, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar45, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitInjectedClassNameType(clang::InjectedClassNameType* D) {
  const Entity* obj = context_.resolve(D);
  //getInjectedSpecializationType
  {
    const Entity* other = context_.resolve(D->getInjectedSpecializationType());
    arboretum_create_edge(obj, context_.data_model_.getInjectedSpecializationType, other);
  }
  //getInjectedTST
  {
    const Entity* other = context_.resolve(D->getInjectedTST());
    arboretum_create_edge(obj, context_.data_model_.getInjectedTST, other);
  }
  //getTemplateName
  // TemplateName
  //getDecl
  {
    const Entity* other = context_.resolve(D->getDecl());
    arboretum_create_edge(obj, context_.data_model_.getDecl7, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared16, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar16, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitLValueReferenceType(clang::LValueReferenceType* D) {
  const Entity* obj = context_.resolve(D);
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared12, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar12, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitMacroQualifiedType(clang::MacroQualifiedType* D) {
  const Entity* obj = context_.resolve(D);
  //getMacroIdentifier
  //getUnderlyingType
  {
    const Entity* other = context_.resolve(D->getUnderlyingType());
    arboretum_create_edge(obj, context_.data_model_.getUnderlyingType1, other);
  }
  //getModifiedType
  {
    const Entity* other = context_.resolve(D->getModifiedType());
    arboretum_create_edge(obj, context_.data_model_.getModifiedType, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared10, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar10, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitMatrixType(clang::MatrixType* D) {
  const Entity* obj = context_.resolve(D);
  //getElementType
  {
    const Entity* other = context_.resolve(D->getElementType());
    arboretum_create_edge(obj, context_.data_model_.getElementType1, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared3, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar3, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitMemberPointerType(clang::MemberPointerType* D) {
  const Entity* obj = context_.resolve(D);
  //getPointeeType
  {
    const Entity* other = context_.resolve(D->getPointeeType());
    arboretum_create_edge(obj, context_.data_model_.getPointeeType3, other);
  }
  //isMemberFunctionPointer
  arboretum_create_edge(obj, context_.data_model_.isMemberFunctionPointer, context_.data_model_.arboretum_node_for(D->isMemberFunctionPointer()));
  //isMemberDataPointer
  arboretum_create_edge(obj, context_.data_model_.isMemberDataPointer, context_.data_model_.arboretum_node_for(D->isMemberDataPointer()));
  //getClass
  {
    const Entity* other = context_.resolve(D->getClass());
    arboretum_create_edge(obj, context_.data_model_.getClass, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared14, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar14, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitObjCInterfaceType(clang::ObjCInterfaceType* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCObjectPointerType(clang::ObjCObjectPointerType* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCObjectType(clang::ObjCObjectType* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCObjectTypeImpl(clang::ObjCObjectTypeImpl* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCTypeParamType(clang::ObjCTypeParamType* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitPackExpansionType(clang::PackExpansionType* D) {
  const Entity* obj = context_.resolve(D);
  //getPattern
  {
    const Entity* other = context_.resolve(D->getPattern());
    arboretum_create_edge(obj, context_.data_model_.getPattern3, other);
  }
  //getNumExpansions
  // std::optional<unsigned int>
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared32, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar32, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitParenType(clang::ParenType* D) {
  const Entity* obj = context_.resolve(D);
  //getInnerType
  {
    const Entity* other = context_.resolve(D->getInnerType());
    arboretum_create_edge(obj, context_.data_model_.getInnerType10, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared23, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar23, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitPipeType(clang::PipeType* D) {
  const Entity* obj = context_.resolve(D);
  //getElementType
  {
    const Entity* other = context_.resolve(D->getElementType());
    arboretum_create_edge(obj, context_.data_model_.getElementType6, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared48, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar48, other);
  }
  //isReadOnly
  arboretum_create_edge(obj, context_.data_model_.isReadOnly2, context_.data_model_.arboretum_node_for(D->isReadOnly()));
  return true;
}

bool ArboretumASTVisitor::VisitPointerType(clang::PointerType* D) {
  const Entity* obj = context_.resolve(D);
  //getPointeeType
  {
    const Entity* other = context_.resolve(D->getPointeeType());
    arboretum_create_edge(obj, context_.data_model_.getPointeeType, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitRValueReferenceType(clang::RValueReferenceType* D) {
  const Entity* obj = context_.resolve(D);
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared46, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar46, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitRecordType(clang::RecordType* D) {
  const Entity* obj = context_.resolve(D);
  //getDecl
  {
    const Entity* other = context_.resolve(D->getDecl());
    arboretum_create_edge(obj, context_.data_model_.getDecl16, other);
  }
  //hasConstFields
  arboretum_create_edge(obj, context_.data_model_.hasConstFields, context_.data_model_.arboretum_node_for(D->hasConstFields()));
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared49, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar49, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitReferenceType(clang::ReferenceType* D) {
  const Entity* obj = context_.resolve(D);
  //isSpelledAsLValue
  arboretum_create_edge(obj, context_.data_model_.isSpelledAsLValue, context_.data_model_.arboretum_node_for(D->isSpelledAsLValue()));
  //isInnerRef
  arboretum_create_edge(obj, context_.data_model_.isInnerRef, context_.data_model_.arboretum_node_for(D->isInnerRef()));
  //getPointeeTypeAsWritten
  {
    const Entity* other = context_.resolve(D->getPointeeTypeAsWritten());
    arboretum_create_edge(obj, context_.data_model_.getPointeeTypeAsWritten, other);
  }
  //getPointeeType
  {
    const Entity* other = context_.resolve(D->getPointeeType());
    arboretum_create_edge(obj, context_.data_model_.getPointeeType7, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitSubstTemplateTypeParmPackType(clang::SubstTemplateTypeParmPackType* D) {
  const Entity* obj = context_.resolve(D);
  //getIdentifier
  //getAssociatedDecl
  {
    const Entity* other = context_.resolve(D->getAssociatedDecl());
    arboretum_create_edge(obj, context_.data_model_.getAssociatedDecl3, other);
  }
  //getReplacedParameter
  {
    const Entity* other = context_.resolve(D->getReplacedParameter());
    arboretum_create_edge(obj, context_.data_model_.getReplacedParameter1, other);
  }
  //getIndex
  // unsigned int
  //getFinal
  arboretum_create_edge(obj, context_.data_model_.getFinal, context_.data_model_.arboretum_node_for(D->getFinal()));
  //getNumArgs
  // unsigned int
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared26, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar26, other);
  }
  //getArgumentPack
  // TemplateArgument
  return true;
}

bool ArboretumASTVisitor::VisitSubstTemplateTypeParmType(clang::SubstTemplateTypeParmType* D) {
  const Entity* obj = context_.resolve(D);
  //getReplacementType
  {
    const Entity* other = context_.resolve(D->getReplacementType());
    arboretum_create_edge(obj, context_.data_model_.getReplacementType, other);
  }
  //getAssociatedDecl
  {
    const Entity* other = context_.resolve(D->getAssociatedDecl());
    arboretum_create_edge(obj, context_.data_model_.getAssociatedDecl, other);
  }
  //getReplacedParameter
  {
    const Entity* other = context_.resolve(D->getReplacedParameter());
    arboretum_create_edge(obj, context_.data_model_.getReplacedParameter, other);
  }
  //getIndex
  // unsigned int
  //getPackIndex
  // std::optional<unsigned int>
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared6, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar6, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitTagType(clang::TagType* D) {
  const Entity* obj = context_.resolve(D);
  //getDecl
  {
    const Entity* other = context_.resolve(D->getDecl());
    arboretum_create_edge(obj, context_.data_model_.getDecl10, other);
  }
  //isBeingDefined
  arboretum_create_edge(obj, context_.data_model_.isBeingDefined1, context_.data_model_.arboretum_node_for(D->isBeingDefined()));
  return true;
}

bool ArboretumASTVisitor::VisitTemplateSpecializationType(clang::TemplateSpecializationType* D) {
  const Entity* obj = context_.resolve(D);
  //isCurrentInstantiation
  arboretum_create_edge(obj, context_.data_model_.isCurrentInstantiation1, context_.data_model_.arboretum_node_for(D->isCurrentInstantiation()));
  //isTypeAlias
  arboretum_create_edge(obj, context_.data_model_.isTypeAlias1, context_.data_model_.arboretum_node_for(D->isTypeAlias()));
  //getTemplateName
  // TemplateName
  //template_arguments
  // ArrayRef<TemplateArgument>
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared25, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar25, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitTemplateTypeParmType(clang::TemplateTypeParmType* D) {
  const Entity* obj = context_.resolve(D);
  //getDepth
  // unsigned int
  //getIndex
  // unsigned int
  //isParameterPack
  arboretum_create_edge(obj, context_.data_model_.isParameterPack5, context_.data_model_.arboretum_node_for(D->isParameterPack()));
  //getDecl
  {
    const Entity* other = context_.resolve(D->getDecl());
    arboretum_create_edge(obj, context_.data_model_.getDecl5, other);
  }
  //getIdentifier
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared8, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar8, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitType(clang::Type* D) {
  const Entity* obj = context_.resolve(D);
  //containsUnexpandedParameterPack
  arboretum_create_edge(obj, context_.data_model_.containsUnexpandedParameterPack1, context_.data_model_.arboretum_node_for(D->containsUnexpandedParameterPack()));
  //getLocallyUnqualifiedSingleStepDesugaredType
  {
    const Entity* other = context_.resolve(D->getLocallyUnqualifiedSingleStepDesugaredType());
    arboretum_create_edge(obj, context_.data_model_.getLocallyUnqualifiedSingleStepDesugaredType, other);
  }
  //getAsPlaceholderType
  {
    const Entity* other = context_.resolve(D->getAsPlaceholderType());
    arboretum_create_edge(obj, context_.data_model_.getAsPlaceholderType, other);
  }
  //getObjCARCImplicitLifetime
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getObjCARCImplicitLifetime());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getObjCARCImplicitLifetime, enum_value);
    }
  }
  //getDependence
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getDependence());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getDependence1, enum_value);
    }
  }
  //containsErrors
  arboretum_create_edge(obj, context_.data_model_.containsErrors1, context_.data_model_.arboretum_node_for(D->containsErrors()));
  //hasSizedVLAType
  arboretum_create_edge(obj, context_.data_model_.hasSizedVLAType, context_.data_model_.arboretum_node_for(D->hasSizedVLAType()));
  //hasUnnamedOrLocalType
  arboretum_create_edge(obj, context_.data_model_.hasUnnamedOrLocalType, context_.data_model_.arboretum_node_for(D->hasUnnamedOrLocalType()));
  //canDecayToPointerType
  arboretum_create_edge(obj, context_.data_model_.canDecayToPointerType, context_.data_model_.arboretum_node_for(D->canDecayToPointerType()));
  //hasPointerRepresentation
  arboretum_create_edge(obj, context_.data_model_.hasPointerRepresentation, context_.data_model_.arboretum_node_for(D->hasPointerRepresentation()));
  //hasObjCPointerRepresentation
  arboretum_create_edge(obj, context_.data_model_.hasObjCPointerRepresentation, context_.data_model_.arboretum_node_for(D->hasObjCPointerRepresentation()));
  //hasIntegerRepresentation
  arboretum_create_edge(obj, context_.data_model_.hasIntegerRepresentation, context_.data_model_.arboretum_node_for(D->hasIntegerRepresentation()));
  //hasSignedIntegerRepresentation
  arboretum_create_edge(obj, context_.data_model_.hasSignedIntegerRepresentation, context_.data_model_.arboretum_node_for(D->hasSignedIntegerRepresentation()));
  //hasUnsignedIntegerRepresentation
  arboretum_create_edge(obj, context_.data_model_.hasUnsignedIntegerRepresentation, context_.data_model_.arboretum_node_for(D->hasUnsignedIntegerRepresentation()));
  //hasFloatingRepresentation
  arboretum_create_edge(obj, context_.data_model_.hasFloatingRepresentation, context_.data_model_.arboretum_node_for(D->hasFloatingRepresentation()));
  //getAsStructureType
  {
    const Entity* other = context_.resolve(D->getAsStructureType());
    arboretum_create_edge(obj, context_.data_model_.getAsStructureType, other);
  }
  //getAsUnionType
  {
    const Entity* other = context_.resolve(D->getAsUnionType());
    arboretum_create_edge(obj, context_.data_model_.getAsUnionType, other);
  }
  //getAsComplexIntegerType
  {
    const Entity* other = context_.resolve(D->getAsComplexIntegerType());
    arboretum_create_edge(obj, context_.data_model_.getAsComplexIntegerType, other);
  }
  //getAsObjCInterfaceType
  {
    const Entity* other = context_.resolve(D->getAsObjCInterfaceType());
    arboretum_create_edge(obj, context_.data_model_.getAsObjCInterfaceType, other);
  }
  //getAsObjCInterfacePointerType
  {
    const Entity* other = context_.resolve(D->getAsObjCInterfacePointerType());
    arboretum_create_edge(obj, context_.data_model_.getAsObjCInterfacePointerType, other);
  }
  //getAsObjCQualifiedIdType
  {
    const Entity* other = context_.resolve(D->getAsObjCQualifiedIdType());
    arboretum_create_edge(obj, context_.data_model_.getAsObjCQualifiedIdType, other);
  }
  //getAsObjCQualifiedClassType
  {
    const Entity* other = context_.resolve(D->getAsObjCQualifiedClassType());
    arboretum_create_edge(obj, context_.data_model_.getAsObjCQualifiedClassType, other);
  }
  //getAsObjCQualifiedInterfaceType
  {
    const Entity* other = context_.resolve(D->getAsObjCQualifiedInterfaceType());
    arboretum_create_edge(obj, context_.data_model_.getAsObjCQualifiedInterfaceType, other);
  }
  //getAsCXXRecordDecl
  {
    const Entity* other = context_.resolve(D->getAsCXXRecordDecl());
    arboretum_create_edge(obj, context_.data_model_.getAsCXXRecordDecl, other);
  }
  //getAsRecordDecl
  {
    const Entity* other = context_.resolve(D->getAsRecordDecl());
    arboretum_create_edge(obj, context_.data_model_.getAsRecordDecl, other);
  }
  //getAsTagDecl
  {
    const Entity* other = context_.resolve(D->getAsTagDecl());
    arboretum_create_edge(obj, context_.data_model_.getAsTagDecl, other);
  }
  //getPointeeCXXRecordDecl
  {
    const Entity* other = context_.resolve(D->getPointeeCXXRecordDecl());
    arboretum_create_edge(obj, context_.data_model_.getPointeeCXXRecordDecl, other);
  }
  //getBaseElementTypeUnsafe
  {
    const Entity* other = context_.resolve(D->getBaseElementTypeUnsafe());
    arboretum_create_edge(obj, context_.data_model_.getBaseElementTypeUnsafe, other);
  }
  //getArrayElementTypeNoTypeQual
  {
    const Entity* other = context_.resolve(D->getArrayElementTypeNoTypeQual());
    arboretum_create_edge(obj, context_.data_model_.getArrayElementTypeNoTypeQual, other);
  }
  //getPointeeOrArrayElementType
  {
    const Entity* other = context_.resolve(D->getPointeeOrArrayElementType());
    arboretum_create_edge(obj, context_.data_model_.getPointeeOrArrayElementType, other);
  }
  //getLinkage
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getLinkage());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getLinkage, enum_value);
    }
  }
  //getVisibility
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getVisibility());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getVisibility1, enum_value);
    }
  }
  //getLinkageAndVisibility
  // LinkageInfo
  //getNullability
  // std::optional<NullabilityKind>
  //acceptsObjCTypeParams
  arboretum_create_edge(obj, context_.data_model_.acceptsObjCTypeParams, context_.data_model_.arboretum_node_for(D->acceptsObjCTypeParams()));
  //getTypeClassName
  //getCanonicalTypeInternal
  {
    const Entity* other = context_.resolve(D->getCanonicalTypeInternal());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalTypeInternal, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitTypeOfExprType(clang::TypeOfExprType* D) {
  const Entity* obj = context_.resolve(D);
  //getUnderlyingExpr
  {
    const Entity* other = context_.resolve(D->getUnderlyingExpr());
    arboretum_create_edge(obj, context_.data_model_.getUnderlyingExpr2, other);
  }
  //getKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getKind10, enum_value);
    }
  }
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar35, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared35, context_.data_model_.arboretum_node_for(D->isSugared()));
  return true;
}

bool ArboretumASTVisitor::VisitTypeOfType(clang::TypeOfType* D) {
  const Entity* obj = context_.resolve(D);
  //getUnmodifiedType
  {
    const Entity* other = context_.resolve(D->getUnmodifiedType());
    arboretum_create_edge(obj, context_.data_model_.getUnmodifiedType, other);
  }
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar24, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared24, context_.data_model_.arboretum_node_for(D->isSugared()));
  //getKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getKind9, enum_value);
    }
  }
  return true;
}

bool ArboretumASTVisitor::VisitTypeWithKeyword(clang::TypeWithKeyword* D) {
  const Entity* obj = context_.resolve(D);
  //getKeyword
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getKeyword());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getKeyword, enum_value);
    }
  }
  return true;
}

bool ArboretumASTVisitor::VisitTypedefType(clang::TypedefType* D) {
  const Entity* obj = context_.resolve(D);
  //getDecl
  {
    const Entity* other = context_.resolve(D->getDecl());
    arboretum_create_edge(obj, context_.data_model_.getDecl13, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared40, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar40, other);
  }
  //typeMatchesDecl
  arboretum_create_edge(obj, context_.data_model_.typeMatchesDecl1, context_.data_model_.arboretum_node_for(D->typeMatchesDecl()));
  return true;
}

bool ArboretumASTVisitor::VisitUnaryTransformType(clang::UnaryTransformType* D) {
  const Entity* obj = context_.resolve(D);
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared30, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar30, other);
  }
  //getUnderlyingType
  {
    const Entity* other = context_.resolve(D->getUnderlyingType());
    arboretum_create_edge(obj, context_.data_model_.getUnderlyingType3, other);
  }
  //getBaseType
  {
    const Entity* other = context_.resolve(D->getBaseType());
    arboretum_create_edge(obj, context_.data_model_.getBaseType3, other);
  }
  //getUTTKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getUTTKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getUTTKind, enum_value);
    }
  }
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedUsingType(clang::UnresolvedUsingType* D) {
  const Entity* obj = context_.resolve(D);
  //getDecl
  {
    const Entity* other = context_.resolve(D->getDecl());
    arboretum_create_edge(obj, context_.data_model_.getDecl6, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared13, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar13, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitUsingType(clang::UsingType* D) {
  const Entity* obj = context_.resolve(D);
  //getFoundDecl
  {
    const Entity* other = context_.resolve(D->getFoundDecl());
    arboretum_create_edge(obj, context_.data_model_.getFoundDecl4, other);
  }
  //getUnderlyingType
  {
    const Entity* other = context_.resolve(D->getUnderlyingType());
    arboretum_create_edge(obj, context_.data_model_.getUnderlyingType4, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared37, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar37, other);
  }
  //typeMatchesDecl
  arboretum_create_edge(obj, context_.data_model_.typeMatchesDecl, context_.data_model_.arboretum_node_for(D->typeMatchesDecl()));
  return true;
}

bool ArboretumASTVisitor::VisitVariableArrayType(clang::VariableArrayType* D) {
  const Entity* obj = context_.resolve(D);
  //getSizeExpr
  {
    const Entity* other = context_.resolve(D->getSizeExpr());
    arboretum_create_edge(obj, context_.data_model_.getSizeExpr5, other);
  }
  //getBracketsRange
  {
    const Entity* other = context_.source_model_.resolve(D->getBracketsRange());
    arboretum_create_edge(obj, context_.data_model_.getBracketsRange2, other);
  }
  //getLBracketLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getLBracketLoc());
    arboretum_create_edge(obj, context_.data_model_.getLBracketLoc2, other);
  }
  //getRBracketLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRBracketLoc());
    arboretum_create_edge(obj, context_.data_model_.getRBracketLoc6, other);
  }
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared47, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar47, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitVectorType(clang::VectorType* D) {
  const Entity* obj = context_.resolve(D);
  //getElementType
  {
    const Entity* other = context_.resolve(D->getElementType());
    arboretum_create_edge(obj, context_.data_model_.getElementType3, other);
  }
  //getNumElements
  // unsigned int
  //isSugared
  arboretum_create_edge(obj, context_.data_model_.isSugared22, context_.data_model_.arboretum_node_for(D->isSugared()));
  //desugar
  {
    const Entity* other = context_.resolve(D->desugar());
    arboretum_create_edge(obj, context_.data_model_.desugar22, other);
  }
  //getVectorKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getVectorKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getVectorKind, enum_value);
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
  const Entity* obj = context_.resolve(D);
  //getAccessSpecifierLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getAccessSpecifierLoc());
    arboretum_create_edge(obj, context_.data_model_.getAccessSpecifierLoc, other);
  }
  //getColonLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getColonLoc());
    arboretum_create_edge(obj, context_.data_model_.getColonLoc2, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange20, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitBaseUsingDecl(clang::BaseUsingDecl* D) {
  const Entity* obj = context_.resolve(D);
  //shadows
  // shadow_range
  //shadow_size
  // unsigned int
  return true;
}

bool ArboretumASTVisitor::VisitBindingDecl(clang::BindingDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getBinding
  {
    const Entity* other = context_.resolve(D->getBinding());
    arboretum_create_edge(obj, context_.data_model_.getBinding, other);
  }
  //getDecomposedDecl
  {
    const Entity* other = context_.resolve(D->getDecomposedDecl());
    arboretum_create_edge(obj, context_.data_model_.getDecomposedDecl, other);
  }
  //getHoldingVar
  {
    const Entity* other = context_.resolve(D->getHoldingVar());
    arboretum_create_edge(obj, context_.data_model_.getHoldingVar, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitBlockDecl(clang::BlockDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getCaretLocation
  {
    const Entity* other = context_.source_model_.resolve(D->getCaretLocation());
    arboretum_create_edge(obj, context_.data_model_.getCaretLocation1, other);
  }
  //isVariadic
  arboretum_create_edge(obj, context_.data_model_.isVariadic1, context_.data_model_.arboretum_node_for(D->isVariadic()));
  //getCompoundBody
  {
    const Entity* other = context_.resolve(D->getCompoundBody());
    arboretum_create_edge(obj, context_.data_model_.getCompoundBody, other);
  }
  //getBody
  {
    const Entity* other = context_.resolve(D->getBody());
    arboretum_create_edge(obj, context_.data_model_.getBody16, other);
  }
  //getSignatureAsWritten
  //parameters
  // ArrayRef<ParmVarDecl *>
  //param_empty
  arboretum_create_edge(obj, context_.data_model_.param_empty1, context_.data_model_.arboretum_node_for(D->param_empty()));
  //param_size
  // size_t
  //getNumParams
  // unsigned int
  //hasCaptures
  arboretum_create_edge(obj, context_.data_model_.hasCaptures, context_.data_model_.arboretum_node_for(D->hasCaptures()));
  //getNumCaptures
  // unsigned int
  //captures
  // ArrayRef<Capture>
  //capturesCXXThis
  arboretum_create_edge(obj, context_.data_model_.capturesCXXThis, context_.data_model_.arboretum_node_for(D->capturesCXXThis()));
  //blockMissingReturnType
  arboretum_create_edge(obj, context_.data_model_.blockMissingReturnType, context_.data_model_.arboretum_node_for(D->blockMissingReturnType()));
  //isConversionFromLambda
  arboretum_create_edge(obj, context_.data_model_.isConversionFromLambda, context_.data_model_.arboretum_node_for(D->isConversionFromLambda()));
  //doesNotEscape
  arboretum_create_edge(obj, context_.data_model_.doesNotEscape, context_.data_model_.arboretum_node_for(D->doesNotEscape()));
  //canAvoidCopyToHeap
  arboretum_create_edge(obj, context_.data_model_.canAvoidCopyToHeap, context_.data_model_.arboretum_node_for(D->canAvoidCopyToHeap()));
  //getBlockManglingNumber
  // unsigned int
  //getBlockManglingContextDecl
  {
    const Entity* other = context_.resolve(D->getBlockManglingContextDecl());
    arboretum_create_edge(obj, context_.data_model_.getBlockManglingContextDecl, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange32, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitBuiltinTemplateDecl(clang::BuiltinTemplateDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange54, other);
  }
  //getBuiltinTemplateKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getBuiltinTemplateKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getBuiltinTemplateKind, enum_value);
    }
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXConstructorDecl(clang::CXXConstructorDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Entity* obj = context_.resolve(D);
  //getExplicitSpecifier
  // const ExplicitSpecifier
  //isExplicit
  arboretum_create_edge(obj, context_.data_model_.isExplicit3, context_.data_model_.arboretum_node_for(D->isExplicit()));
  //inits
  // init_const_range
  //getNumCtorInitializers
  // unsigned int
  //isDelegatingConstructor
  arboretum_create_edge(obj, context_.data_model_.isDelegatingConstructor, context_.data_model_.arboretum_node_for(D->isDelegatingConstructor()));
  //isDefaultConstructor
  arboretum_create_edge(obj, context_.data_model_.isDefaultConstructor, context_.data_model_.arboretum_node_for(D->isDefaultConstructor()));
  //isCopyConstructor
  arboretum_create_edge(obj, context_.data_model_.isCopyConstructor1, context_.data_model_.arboretum_node_for(D->isCopyConstructor()));
  //isMoveConstructor
  arboretum_create_edge(obj, context_.data_model_.isMoveConstructor1, context_.data_model_.arboretum_node_for(D->isMoveConstructor()));
  //isCopyOrMoveConstructor
  arboretum_create_edge(obj, context_.data_model_.isCopyOrMoveConstructor1, context_.data_model_.arboretum_node_for(D->isCopyOrMoveConstructor()));
  //isSpecializationCopyingObject
  arboretum_create_edge(obj, context_.data_model_.isSpecializationCopyingObject, context_.data_model_.arboretum_node_for(D->isSpecializationCopyingObject()));
  //isInheritingConstructor
  arboretum_create_edge(obj, context_.data_model_.isInheritingConstructor, context_.data_model_.arboretum_node_for(D->isInheritingConstructor()));
  //getInheritedConstructor
  // InheritedConstructor
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl63, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXConversionDecl(clang::CXXConversionDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Entity* obj = context_.resolve(D);
  //getExplicitSpecifier
  // const ExplicitSpecifier
  //isExplicit
  arboretum_create_edge(obj, context_.data_model_.isExplicit1, context_.data_model_.arboretum_node_for(D->isExplicit()));
  //getConversionType
  {
    const Entity* other = context_.resolve(D->getConversionType());
    arboretum_create_edge(obj, context_.data_model_.getConversionType, other);
  }
  //isLambdaToBlockPointerConversion
  arboretum_create_edge(obj, context_.data_model_.isLambdaToBlockPointerConversion, context_.data_model_.arboretum_node_for(D->isLambdaToBlockPointerConversion()));
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl39, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXDeductionGuideDecl(clang::CXXDeductionGuideDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getExplicitSpecifier
  // const ExplicitSpecifier
  //isExplicit
  arboretum_create_edge(obj, context_.data_model_.isExplicit2, context_.data_model_.arboretum_node_for(D->isExplicit()));
  //getDeducedTemplate
  {
    const Entity* other = context_.resolve(D->getDeducedTemplate());
    arboretum_create_edge(obj, context_.data_model_.getDeducedTemplate, other);
  }
  //getCorrespondingConstructor
  {
    const Entity* other = context_.resolve(D->getCorrespondingConstructor());
    arboretum_create_edge(obj, context_.data_model_.getCorrespondingConstructor, other);
  }
  //getDeductionCandidateKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getDeductionCandidateKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getDeductionCandidateKind, enum_value);
    }
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXDestructorDecl(clang::CXXDestructorDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Entity* obj = context_.resolve(D);
  //getOperatorDelete
  {
    const Entity* other = context_.resolve(D->getOperatorDelete());
    arboretum_create_edge(obj, context_.data_model_.getOperatorDelete2, other);
  }
  //getOperatorDeleteThisArg
  {
    const Entity* other = context_.resolve(D->getOperatorDeleteThisArg());
    arboretum_create_edge(obj, context_.data_model_.getOperatorDeleteThisArg, other);
  }
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl41, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXMethodDecl(clang::CXXMethodDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Entity* obj = context_.resolve(D);
  //isStatic
  arboretum_create_edge(obj, context_.data_model_.isStatic, context_.data_model_.arboretum_node_for(D->isStatic()));
  //isInstance
  arboretum_create_edge(obj, context_.data_model_.isInstance, context_.data_model_.arboretum_node_for(D->isInstance()));
  //isExplicitObjectMemberFunction
  arboretum_create_edge(obj, context_.data_model_.isExplicitObjectMemberFunction, context_.data_model_.arboretum_node_for(D->isExplicitObjectMemberFunction()));
  //isImplicitObjectMemberFunction
  arboretum_create_edge(obj, context_.data_model_.isImplicitObjectMemberFunction, context_.data_model_.arboretum_node_for(D->isImplicitObjectMemberFunction()));
  //isConst
  arboretum_create_edge(obj, context_.data_model_.isConst, context_.data_model_.arboretum_node_for(D->isConst()));
  //isVolatile
  arboretum_create_edge(obj, context_.data_model_.isVolatile2, context_.data_model_.arboretum_node_for(D->isVolatile()));
  //isVirtual
  arboretum_create_edge(obj, context_.data_model_.isVirtual, context_.data_model_.arboretum_node_for(D->isVirtual()));
  //isCopyAssignmentOperator
  arboretum_create_edge(obj, context_.data_model_.isCopyAssignmentOperator, context_.data_model_.arboretum_node_for(D->isCopyAssignmentOperator()));
  //isMoveAssignmentOperator
  arboretum_create_edge(obj, context_.data_model_.isMoveAssignmentOperator, context_.data_model_.arboretum_node_for(D->isMoveAssignmentOperator()));
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl17, other);
  }
  //getMostRecentDecl
  {
    const Entity* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_edge(obj, context_.data_model_.getMostRecentDecl5, other);
  }
  //size_overridden_methods
  // unsigned int
  //overridden_methods
  // overridden_method_range
  //getParent
  {
    const Entity* other = context_.resolve(D->getParent());
    arboretum_create_edge(obj, context_.data_model_.getParent, other);
  }
  //getThisType
  {
    const Entity* other = context_.resolve(D->getThisType());
    arboretum_create_edge(obj, context_.data_model_.getThisType, other);
  }
  //getFunctionObjectParameterReferenceType
  {
    const Entity* other = context_.resolve(D->getFunctionObjectParameterReferenceType());
    arboretum_create_edge(obj, context_.data_model_.getFunctionObjectParameterReferenceType, other);
  }
  //getFunctionObjectParameterType
  {
    const Entity* other = context_.resolve(D->getFunctionObjectParameterType());
    arboretum_create_edge(obj, context_.data_model_.getFunctionObjectParameterType, other);
  }
  //getNumExplicitParams
  // unsigned int
  //getMethodQualifiers
  // Qualifiers
  //getRefQualifier
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getRefQualifier());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getRefQualifier, enum_value);
    }
  }
  //hasInlineBody
  arboretum_create_edge(obj, context_.data_model_.hasInlineBody, context_.data_model_.arboretum_node_for(D->hasInlineBody()));
  //isLambdaStaticInvoker
  arboretum_create_edge(obj, context_.data_model_.isLambdaStaticInvoker, context_.data_model_.arboretum_node_for(D->isLambdaStaticInvoker()));
  return true;
}

bool ArboretumASTVisitor::VisitCXXRecordDecl(clang::CXXRecordDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Entity* obj = context_.resolve(D);
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl1, other);
  }
  //getPreviousDecl
  {
    const Entity* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_edge(obj, context_.data_model_.getPreviousDecl1, other);
  }
  //getMostRecentDecl
  {
    const Entity* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_edge(obj, context_.data_model_.getMostRecentDecl1, other);
  }
  //getDefinition
  {
    const Entity* other = context_.resolve(D->getDefinition());
    arboretum_create_edge(obj, context_.data_model_.getDefinition, other);
  }
  //hasDefinition
  arboretum_create_edge(obj, context_.data_model_.hasDefinition, context_.data_model_.arboretum_node_for(D->hasDefinition()));
  //isDynamicClass
  arboretum_create_edge(obj, context_.data_model_.isDynamicClass, context_.data_model_.arboretum_node_for(D->isDynamicClass()));
  //mayBeDynamicClass
  arboretum_create_edge(obj, context_.data_model_.mayBeDynamicClass, context_.data_model_.arboretum_node_for(D->mayBeDynamicClass()));
  //mayBeNonDynamicClass
  arboretum_create_edge(obj, context_.data_model_.mayBeNonDynamicClass, context_.data_model_.arboretum_node_for(D->mayBeNonDynamicClass()));
  //isParsingBaseSpecifiers
  arboretum_create_edge(obj, context_.data_model_.isParsingBaseSpecifiers, context_.data_model_.arboretum_node_for(D->isParsingBaseSpecifiers()));
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
  arboretum_create_edge(obj, context_.data_model_.hasAnyDependentBases, context_.data_model_.arboretum_node_for(D->hasAnyDependentBases()));
  //methods
  // method_range
  //ctors
  // ctor_range
  //friends
  // friend_range
  //hasFriends
  arboretum_create_edge(obj, context_.data_model_.hasFriends, context_.data_model_.arboretum_node_for(D->hasFriends()));
  //hasSimpleCopyConstructor
  arboretum_create_edge(obj, context_.data_model_.hasSimpleCopyConstructor, context_.data_model_.arboretum_node_for(D->hasSimpleCopyConstructor()));
  //hasSimpleMoveConstructor
  arboretum_create_edge(obj, context_.data_model_.hasSimpleMoveConstructor, context_.data_model_.arboretum_node_for(D->hasSimpleMoveConstructor()));
  //hasSimpleCopyAssignment
  arboretum_create_edge(obj, context_.data_model_.hasSimpleCopyAssignment, context_.data_model_.arboretum_node_for(D->hasSimpleCopyAssignment()));
  //hasSimpleMoveAssignment
  arboretum_create_edge(obj, context_.data_model_.hasSimpleMoveAssignment, context_.data_model_.arboretum_node_for(D->hasSimpleMoveAssignment()));
  //hasSimpleDestructor
  arboretum_create_edge(obj, context_.data_model_.hasSimpleDestructor, context_.data_model_.arboretum_node_for(D->hasSimpleDestructor()));
  //hasDefaultConstructor
  arboretum_create_edge(obj, context_.data_model_.hasDefaultConstructor, context_.data_model_.arboretum_node_for(D->hasDefaultConstructor()));
  //needsImplicitDefaultConstructor
  arboretum_create_edge(obj, context_.data_model_.needsImplicitDefaultConstructor, context_.data_model_.arboretum_node_for(D->needsImplicitDefaultConstructor()));
  //hasUserDeclaredConstructor
  arboretum_create_edge(obj, context_.data_model_.hasUserDeclaredConstructor, context_.data_model_.arboretum_node_for(D->hasUserDeclaredConstructor()));
  //hasUserProvidedDefaultConstructor
  arboretum_create_edge(obj, context_.data_model_.hasUserProvidedDefaultConstructor, context_.data_model_.arboretum_node_for(D->hasUserProvidedDefaultConstructor()));
  //hasUserDeclaredCopyConstructor
  arboretum_create_edge(obj, context_.data_model_.hasUserDeclaredCopyConstructor, context_.data_model_.arboretum_node_for(D->hasUserDeclaredCopyConstructor()));
  //needsImplicitCopyConstructor
  arboretum_create_edge(obj, context_.data_model_.needsImplicitCopyConstructor, context_.data_model_.arboretum_node_for(D->needsImplicitCopyConstructor()));
  //needsOverloadResolutionForCopyConstructor
  arboretum_create_edge(obj, context_.data_model_.needsOverloadResolutionForCopyConstructor, context_.data_model_.arboretum_node_for(D->needsOverloadResolutionForCopyConstructor()));
  //implicitCopyConstructorHasConstParam
  arboretum_create_edge(obj, context_.data_model_.implicitCopyConstructorHasConstParam, context_.data_model_.arboretum_node_for(D->implicitCopyConstructorHasConstParam()));
  //hasCopyConstructorWithConstParam
  arboretum_create_edge(obj, context_.data_model_.hasCopyConstructorWithConstParam, context_.data_model_.arboretum_node_for(D->hasCopyConstructorWithConstParam()));
  //hasUserDeclaredMoveOperation
  arboretum_create_edge(obj, context_.data_model_.hasUserDeclaredMoveOperation, context_.data_model_.arboretum_node_for(D->hasUserDeclaredMoveOperation()));
  //hasUserDeclaredMoveConstructor
  arboretum_create_edge(obj, context_.data_model_.hasUserDeclaredMoveConstructor, context_.data_model_.arboretum_node_for(D->hasUserDeclaredMoveConstructor()));
  //hasMoveConstructor
  arboretum_create_edge(obj, context_.data_model_.hasMoveConstructor, context_.data_model_.arboretum_node_for(D->hasMoveConstructor()));
  //needsImplicitMoveConstructor
  arboretum_create_edge(obj, context_.data_model_.needsImplicitMoveConstructor, context_.data_model_.arboretum_node_for(D->needsImplicitMoveConstructor()));
  //needsOverloadResolutionForMoveConstructor
  arboretum_create_edge(obj, context_.data_model_.needsOverloadResolutionForMoveConstructor, context_.data_model_.arboretum_node_for(D->needsOverloadResolutionForMoveConstructor()));
  //hasUserDeclaredCopyAssignment
  arboretum_create_edge(obj, context_.data_model_.hasUserDeclaredCopyAssignment, context_.data_model_.arboretum_node_for(D->hasUserDeclaredCopyAssignment()));
  //needsImplicitCopyAssignment
  arboretum_create_edge(obj, context_.data_model_.needsImplicitCopyAssignment, context_.data_model_.arboretum_node_for(D->needsImplicitCopyAssignment()));
  //needsOverloadResolutionForCopyAssignment
  arboretum_create_edge(obj, context_.data_model_.needsOverloadResolutionForCopyAssignment, context_.data_model_.arboretum_node_for(D->needsOverloadResolutionForCopyAssignment()));
  //implicitCopyAssignmentHasConstParam
  arboretum_create_edge(obj, context_.data_model_.implicitCopyAssignmentHasConstParam, context_.data_model_.arboretum_node_for(D->implicitCopyAssignmentHasConstParam()));
  //hasCopyAssignmentWithConstParam
  arboretum_create_edge(obj, context_.data_model_.hasCopyAssignmentWithConstParam, context_.data_model_.arboretum_node_for(D->hasCopyAssignmentWithConstParam()));
  //hasUserDeclaredMoveAssignment
  arboretum_create_edge(obj, context_.data_model_.hasUserDeclaredMoveAssignment, context_.data_model_.arboretum_node_for(D->hasUserDeclaredMoveAssignment()));
  //hasMoveAssignment
  arboretum_create_edge(obj, context_.data_model_.hasMoveAssignment, context_.data_model_.arboretum_node_for(D->hasMoveAssignment()));
  //needsImplicitMoveAssignment
  arboretum_create_edge(obj, context_.data_model_.needsImplicitMoveAssignment, context_.data_model_.arboretum_node_for(D->needsImplicitMoveAssignment()));
  //needsOverloadResolutionForMoveAssignment
  arboretum_create_edge(obj, context_.data_model_.needsOverloadResolutionForMoveAssignment, context_.data_model_.arboretum_node_for(D->needsOverloadResolutionForMoveAssignment()));
  //hasUserDeclaredDestructor
  arboretum_create_edge(obj, context_.data_model_.hasUserDeclaredDestructor, context_.data_model_.arboretum_node_for(D->hasUserDeclaredDestructor()));
  //needsImplicitDestructor
  arboretum_create_edge(obj, context_.data_model_.needsImplicitDestructor, context_.data_model_.arboretum_node_for(D->needsImplicitDestructor()));
  //needsOverloadResolutionForDestructor
  arboretum_create_edge(obj, context_.data_model_.needsOverloadResolutionForDestructor, context_.data_model_.arboretum_node_for(D->needsOverloadResolutionForDestructor()));
  //isLambda
  arboretum_create_edge(obj, context_.data_model_.isLambda, context_.data_model_.arboretum_node_for(D->isLambda()));
  //isGenericLambda
  arboretum_create_edge(obj, context_.data_model_.isGenericLambda1, context_.data_model_.arboretum_node_for(D->isGenericLambda()));
  //lambdaIsDefaultConstructibleAndAssignable
  arboretum_create_edge(obj, context_.data_model_.lambdaIsDefaultConstructibleAndAssignable, context_.data_model_.arboretum_node_for(D->lambdaIsDefaultConstructibleAndAssignable()));
  //getLambdaCallOperator
  {
    const Entity* other = context_.resolve(D->getLambdaCallOperator());
    arboretum_create_edge(obj, context_.data_model_.getLambdaCallOperator, other);
  }
  //getDependentLambdaCallOperator
  {
    const Entity* other = context_.resolve(D->getDependentLambdaCallOperator());
    arboretum_create_edge(obj, context_.data_model_.getDependentLambdaCallOperator, other);
  }
  //getGenericLambdaTemplateParameterList
  //getLambdaExplicitTemplateParameters
  // ArrayRef<NamedDecl *>
  //isCapturelessLambda
  arboretum_create_edge(obj, context_.data_model_.isCapturelessLambda, context_.data_model_.arboretum_node_for(D->isCapturelessLambda()));
  //captures
  // capture_const_range
  //capture_size
  // unsigned int
  //getVisibleConversionFunctions
  // llvm::iterator_range<conversion_iterator>
  //isAggregate
  arboretum_create_edge(obj, context_.data_model_.isAggregate, context_.data_model_.arboretum_node_for(D->isAggregate()));
  //hasInClassInitializer
  arboretum_create_edge(obj, context_.data_model_.hasInClassInitializer, context_.data_model_.arboretum_node_for(D->hasInClassInitializer()));
  //hasUninitializedReferenceMember
  arboretum_create_edge(obj, context_.data_model_.hasUninitializedReferenceMember, context_.data_model_.arboretum_node_for(D->hasUninitializedReferenceMember()));
  //isPOD
  arboretum_create_edge(obj, context_.data_model_.isPOD, context_.data_model_.arboretum_node_for(D->isPOD()));
  //isCLike
  arboretum_create_edge(obj, context_.data_model_.isCLike, context_.data_model_.arboretum_node_for(D->isCLike()));
  //isEmpty
  arboretum_create_edge(obj, context_.data_model_.isEmpty, context_.data_model_.arboretum_node_for(D->isEmpty()));
  //hasInitMethod
  arboretum_create_edge(obj, context_.data_model_.hasInitMethod, context_.data_model_.arboretum_node_for(D->hasInitMethod()));
  //hasPrivateFields
  arboretum_create_edge(obj, context_.data_model_.hasPrivateFields, context_.data_model_.arboretum_node_for(D->hasPrivateFields()));
  //hasProtectedFields
  arboretum_create_edge(obj, context_.data_model_.hasProtectedFields, context_.data_model_.arboretum_node_for(D->hasProtectedFields()));
  //hasDirectFields
  arboretum_create_edge(obj, context_.data_model_.hasDirectFields, context_.data_model_.arboretum_node_for(D->hasDirectFields()));
  //isPolymorphic
  arboretum_create_edge(obj, context_.data_model_.isPolymorphic, context_.data_model_.arboretum_node_for(D->isPolymorphic()));
  //isAbstract
  arboretum_create_edge(obj, context_.data_model_.isAbstract, context_.data_model_.arboretum_node_for(D->isAbstract()));
  //isStandardLayout
  arboretum_create_edge(obj, context_.data_model_.isStandardLayout, context_.data_model_.arboretum_node_for(D->isStandardLayout()));
  //isCXX11StandardLayout
  arboretum_create_edge(obj, context_.data_model_.isCXX11StandardLayout, context_.data_model_.arboretum_node_for(D->isCXX11StandardLayout()));
  //hasMutableFields
  arboretum_create_edge(obj, context_.data_model_.hasMutableFields, context_.data_model_.arboretum_node_for(D->hasMutableFields()));
  //hasVariantMembers
  arboretum_create_edge(obj, context_.data_model_.hasVariantMembers, context_.data_model_.arboretum_node_for(D->hasVariantMembers()));
  //hasTrivialDefaultConstructor
  arboretum_create_edge(obj, context_.data_model_.hasTrivialDefaultConstructor, context_.data_model_.arboretum_node_for(D->hasTrivialDefaultConstructor()));
  //hasNonTrivialDefaultConstructor
  arboretum_create_edge(obj, context_.data_model_.hasNonTrivialDefaultConstructor, context_.data_model_.arboretum_node_for(D->hasNonTrivialDefaultConstructor()));
  //hasConstexprNonCopyMoveConstructor
  arboretum_create_edge(obj, context_.data_model_.hasConstexprNonCopyMoveConstructor, context_.data_model_.arboretum_node_for(D->hasConstexprNonCopyMoveConstructor()));
  //defaultedDefaultConstructorIsConstexpr
  arboretum_create_edge(obj, context_.data_model_.defaultedDefaultConstructorIsConstexpr, context_.data_model_.arboretum_node_for(D->defaultedDefaultConstructorIsConstexpr()));
  //hasConstexprDefaultConstructor
  arboretum_create_edge(obj, context_.data_model_.hasConstexprDefaultConstructor, context_.data_model_.arboretum_node_for(D->hasConstexprDefaultConstructor()));
  //hasTrivialCopyConstructor
  arboretum_create_edge(obj, context_.data_model_.hasTrivialCopyConstructor, context_.data_model_.arboretum_node_for(D->hasTrivialCopyConstructor()));
  //hasTrivialCopyConstructorForCall
  arboretum_create_edge(obj, context_.data_model_.hasTrivialCopyConstructorForCall, context_.data_model_.arboretum_node_for(D->hasTrivialCopyConstructorForCall()));
  //hasNonTrivialCopyConstructor
  arboretum_create_edge(obj, context_.data_model_.hasNonTrivialCopyConstructor, context_.data_model_.arboretum_node_for(D->hasNonTrivialCopyConstructor()));
  //hasNonTrivialCopyConstructorForCall
  arboretum_create_edge(obj, context_.data_model_.hasNonTrivialCopyConstructorForCall, context_.data_model_.arboretum_node_for(D->hasNonTrivialCopyConstructorForCall()));
  //hasTrivialMoveConstructor
  arboretum_create_edge(obj, context_.data_model_.hasTrivialMoveConstructor, context_.data_model_.arboretum_node_for(D->hasTrivialMoveConstructor()));
  //hasTrivialMoveConstructorForCall
  arboretum_create_edge(obj, context_.data_model_.hasTrivialMoveConstructorForCall, context_.data_model_.arboretum_node_for(D->hasTrivialMoveConstructorForCall()));
  //hasNonTrivialMoveConstructor
  arboretum_create_edge(obj, context_.data_model_.hasNonTrivialMoveConstructor, context_.data_model_.arboretum_node_for(D->hasNonTrivialMoveConstructor()));
  //hasNonTrivialMoveConstructorForCall
  arboretum_create_edge(obj, context_.data_model_.hasNonTrivialMoveConstructorForCall, context_.data_model_.arboretum_node_for(D->hasNonTrivialMoveConstructorForCall()));
  //hasTrivialCopyAssignment
  arboretum_create_edge(obj, context_.data_model_.hasTrivialCopyAssignment, context_.data_model_.arboretum_node_for(D->hasTrivialCopyAssignment()));
  //hasNonTrivialCopyAssignment
  arboretum_create_edge(obj, context_.data_model_.hasNonTrivialCopyAssignment, context_.data_model_.arboretum_node_for(D->hasNonTrivialCopyAssignment()));
  //hasTrivialMoveAssignment
  arboretum_create_edge(obj, context_.data_model_.hasTrivialMoveAssignment, context_.data_model_.arboretum_node_for(D->hasTrivialMoveAssignment()));
  //hasNonTrivialMoveAssignment
  arboretum_create_edge(obj, context_.data_model_.hasNonTrivialMoveAssignment, context_.data_model_.arboretum_node_for(D->hasNonTrivialMoveAssignment()));
  //defaultedDestructorIsConstexpr
  arboretum_create_edge(obj, context_.data_model_.defaultedDestructorIsConstexpr, context_.data_model_.arboretum_node_for(D->defaultedDestructorIsConstexpr()));
  //hasConstexprDestructor
  arboretum_create_edge(obj, context_.data_model_.hasConstexprDestructor, context_.data_model_.arboretum_node_for(D->hasConstexprDestructor()));
  //hasTrivialDestructor
  arboretum_create_edge(obj, context_.data_model_.hasTrivialDestructor, context_.data_model_.arboretum_node_for(D->hasTrivialDestructor()));
  //hasTrivialDestructorForCall
  arboretum_create_edge(obj, context_.data_model_.hasTrivialDestructorForCall, context_.data_model_.arboretum_node_for(D->hasTrivialDestructorForCall()));
  //hasNonTrivialDestructor
  arboretum_create_edge(obj, context_.data_model_.hasNonTrivialDestructor, context_.data_model_.arboretum_node_for(D->hasNonTrivialDestructor()));
  //hasNonTrivialDestructorForCall
  arboretum_create_edge(obj, context_.data_model_.hasNonTrivialDestructorForCall, context_.data_model_.arboretum_node_for(D->hasNonTrivialDestructorForCall()));
  //allowConstDefaultInit
  arboretum_create_edge(obj, context_.data_model_.allowConstDefaultInit, context_.data_model_.arboretum_node_for(D->allowConstDefaultInit()));
  //hasIrrelevantDestructor
  arboretum_create_edge(obj, context_.data_model_.hasIrrelevantDestructor, context_.data_model_.arboretum_node_for(D->hasIrrelevantDestructor()));
  //hasNonLiteralTypeFieldsOrBases
  arboretum_create_edge(obj, context_.data_model_.hasNonLiteralTypeFieldsOrBases, context_.data_model_.arboretum_node_for(D->hasNonLiteralTypeFieldsOrBases()));
  //hasInheritedConstructor
  arboretum_create_edge(obj, context_.data_model_.hasInheritedConstructor, context_.data_model_.arboretum_node_for(D->hasInheritedConstructor()));
  //hasInheritedAssignment
  arboretum_create_edge(obj, context_.data_model_.hasInheritedAssignment, context_.data_model_.arboretum_node_for(D->hasInheritedAssignment()));
  //isTriviallyCopyable
  arboretum_create_edge(obj, context_.data_model_.isTriviallyCopyable, context_.data_model_.arboretum_node_for(D->isTriviallyCopyable()));
  //isTriviallyCopyConstructible
  arboretum_create_edge(obj, context_.data_model_.isTriviallyCopyConstructible, context_.data_model_.arboretum_node_for(D->isTriviallyCopyConstructible()));
  //isTrivial
  arboretum_create_edge(obj, context_.data_model_.isTrivial, context_.data_model_.arboretum_node_for(D->isTrivial()));
  //isLiteral
  arboretum_create_edge(obj, context_.data_model_.isLiteral, context_.data_model_.arboretum_node_for(D->isLiteral()));
  //isStructural
  arboretum_create_edge(obj, context_.data_model_.isStructural, context_.data_model_.arboretum_node_for(D->isStructural()));
  //getInstantiatedFromMemberClass
  {
    const Entity* other = context_.resolve(D->getInstantiatedFromMemberClass());
    arboretum_create_edge(obj, context_.data_model_.getInstantiatedFromMemberClass, other);
  }
  //getMemberSpecializationInfo
  //getDescribedClassTemplate
  {
    const Entity* other = context_.resolve(D->getDescribedClassTemplate());
    arboretum_create_edge(obj, context_.data_model_.getDescribedClassTemplate, other);
  }
  //getTemplateSpecializationKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getTemplateSpecializationKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getTemplateSpecializationKind, enum_value);
    }
  }
  //getTemplateInstantiationPattern
  {
    const Entity* other = context_.resolve(D->getTemplateInstantiationPattern());
    arboretum_create_edge(obj, context_.data_model_.getTemplateInstantiationPattern, other);
  }
  //getDestructor
  {
    const Entity* other = context_.resolve(D->getDestructor());
    arboretum_create_edge(obj, context_.data_model_.getDestructor, other);
  }
  //isAnyDestructorNoReturn
  arboretum_create_edge(obj, context_.data_model_.isAnyDestructorNoReturn, context_.data_model_.arboretum_node_for(D->isAnyDestructorNoReturn()));
  //isLocalClass
  {
    const Entity* other = context_.resolve(D->isLocalClass());
    arboretum_create_edge(obj, context_.data_model_.isLocalClass, other);
  }
  //mayBeAbstract
  arboretum_create_edge(obj, context_.data_model_.mayBeAbstract, context_.data_model_.arboretum_node_for(D->mayBeAbstract()));
  //isEffectivelyFinal
  arboretum_create_edge(obj, context_.data_model_.isEffectivelyFinal, context_.data_model_.arboretum_node_for(D->isEffectivelyFinal()));
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
    const Entity* enum_value = context_.data_model_.resolve(D->getMSVtorDispMode());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getMSVtorDispMode, enum_value);
    }
  }
  //isDependentLambda
  arboretum_create_edge(obj, context_.data_model_.isDependentLambda, context_.data_model_.arboretum_node_for(D->isDependentLambda()));
  //isNeverDependentLambda
  arboretum_create_edge(obj, context_.data_model_.isNeverDependentLambda, context_.data_model_.arboretum_node_for(D->isNeverDependentLambda()));
  //getLambdaDependencyKind
  // unsigned int
  //getLambdaTypeInfo
  return true;
}

bool ArboretumASTVisitor::VisitCapturedDecl(clang::CapturedDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getBody
  {
    const Entity* other = context_.resolve(D->getBody());
    arboretum_create_edge(obj, context_.data_model_.getBody17, other);
  }
  //isNothrow
  arboretum_create_edge(obj, context_.data_model_.isNothrow1, context_.data_model_.arboretum_node_for(D->isNothrow()));
  //getNumParams
  // unsigned int
  //parameters
  // ArrayRef<ImplicitParamDecl *>
  //getContextParam
  {
    const Entity* other = context_.resolve(D->getContextParam());
    arboretum_create_edge(obj, context_.data_model_.getContextParam, other);
  }
  //getContextParamPosition
  // unsigned int
  return true;
}

bool ArboretumASTVisitor::VisitClassTemplateDecl(clang::ClassTemplateDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getTemplatedDecl
  {
    const Entity* other = context_.resolve(D->getTemplatedDecl());
    arboretum_create_edge(obj, context_.data_model_.getTemplatedDecl, other);
  }
  //isThisDeclarationADefinition
  arboretum_create_edge(obj, context_.data_model_.isThisDeclarationADefinition1, context_.data_model_.arboretum_node_for(D->isThisDeclarationADefinition()));
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl11, other);
  }
  //getPreviousDecl
  {
    const Entity* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_edge(obj, context_.data_model_.getPreviousDecl3, other);
  }
  //getMostRecentDecl
  {
    const Entity* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_edge(obj, context_.data_model_.getMostRecentDecl3, other);
  }
  //getInstantiatedFromMemberTemplate
  {
    const Entity* other = context_.resolve(D->getInstantiatedFromMemberTemplate());
    arboretum_create_edge(obj, context_.data_model_.getInstantiatedFromMemberTemplate, other);
  }
  //specializations
  // spec_range
  return true;
}

bool ArboretumASTVisitor::VisitClassTemplatePartialSpecializationDecl(clang::ClassTemplatePartialSpecializationDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Entity* obj = context_.resolve(D);
  //getTemplateParameters
  //hasAssociatedConstraints
  arboretum_create_edge(obj, context_.data_model_.hasAssociatedConstraints2, context_.data_model_.arboretum_node_for(D->hasAssociatedConstraints()));
  //getTemplateArgsAsWritten
  //getInstantiatedFromMember
  {
    const Entity* other = context_.resolve(D->getInstantiatedFromMember());
    arboretum_create_edge(obj, context_.data_model_.getInstantiatedFromMember1, other);
  }
  //getInstantiatedFromMemberTemplate
  {
    const Entity* other = context_.resolve(D->getInstantiatedFromMemberTemplate());
    arboretum_create_edge(obj, context_.data_model_.getInstantiatedFromMemberTemplate4, other);
  }
  //getInjectedSpecializationType
  {
    const Entity* other = context_.resolve(D->getInjectedSpecializationType());
    arboretum_create_edge(obj, context_.data_model_.getInjectedSpecializationType1, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitClassTemplateSpecializationDecl(clang::ClassTemplateSpecializationDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Entity* obj = context_.resolve(D);
  //getSpecializedTemplate
  {
    const Entity* other = context_.resolve(D->getSpecializedTemplate());
    arboretum_create_edge(obj, context_.data_model_.getSpecializedTemplate1, other);
  }
  //getTemplateArgs
  // const TemplateArgumentList &
  //getSpecializationKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getSpecializationKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getSpecializationKind1, enum_value);
    }
  }
  //isExplicitSpecialization
  arboretum_create_edge(obj, context_.data_model_.isExplicitSpecialization1, context_.data_model_.arboretum_node_for(D->isExplicitSpecialization()));
  //isClassScopeExplicitSpecialization
  arboretum_create_edge(obj, context_.data_model_.isClassScopeExplicitSpecialization1, context_.data_model_.arboretum_node_for(D->isClassScopeExplicitSpecialization()));
  //isExplicitInstantiationOrSpecialization
  arboretum_create_edge(obj, context_.data_model_.isExplicitInstantiationOrSpecialization1, context_.data_model_.arboretum_node_for(D->isExplicitInstantiationOrSpecialization()));
  //getPointOfInstantiation
  {
    const Entity* other = context_.source_model_.resolve(D->getPointOfInstantiation());
    arboretum_create_edge(obj, context_.data_model_.getPointOfInstantiation3, other);
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
    const Entity* other = context_.source_model_.resolve(D->getExternLoc());
    arboretum_create_edge(obj, context_.data_model_.getExternLoc1, other);
  }
  //getTemplateKeywordLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getTemplateKeywordLoc());
    arboretum_create_edge(obj, context_.data_model_.getTemplateKeywordLoc5, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange37, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitConceptDecl(clang::ConceptDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getConstraintExpr
  {
    const Entity* other = context_.resolve(D->getConstraintExpr());
    arboretum_create_edge(obj, context_.data_model_.getConstraintExpr, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange60, other);
  }
  //isTypeConcept
  arboretum_create_edge(obj, context_.data_model_.isTypeConcept, context_.data_model_.arboretum_node_for(D->isTypeConcept()));
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl65, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitConstructorUsingShadowDecl(clang::ConstructorUsingShadowDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getIntroducer
  {
    const Entity* other = context_.resolve(D->getIntroducer());
    arboretum_create_edge(obj, context_.data_model_.getIntroducer1, other);
  }
  //getParent
  {
    const Entity* other = context_.resolve(D->getParent());
    arboretum_create_edge(obj, context_.data_model_.getParent4, other);
  }
  //getNominatedBaseClassShadowDecl
  {
    const Entity* other = context_.resolve(D->getNominatedBaseClassShadowDecl());
    arboretum_create_edge(obj, context_.data_model_.getNominatedBaseClassShadowDecl, other);
  }
  //getConstructedBaseClassShadowDecl
  {
    const Entity* other = context_.resolve(D->getConstructedBaseClassShadowDecl());
    arboretum_create_edge(obj, context_.data_model_.getConstructedBaseClassShadowDecl, other);
  }
  //getNominatedBaseClass
  {
    const Entity* other = context_.resolve(D->getNominatedBaseClass());
    arboretum_create_edge(obj, context_.data_model_.getNominatedBaseClass, other);
  }
  //getConstructedBaseClass
  {
    const Entity* other = context_.resolve(D->getConstructedBaseClass());
    arboretum_create_edge(obj, context_.data_model_.getConstructedBaseClass, other);
  }
  //constructsVirtualBase
  arboretum_create_edge(obj, context_.data_model_.constructsVirtualBase, context_.data_model_.arboretum_node_for(D->constructsVirtualBase()));
  return true;
}

bool ArboretumASTVisitor::VisitDecl(clang::Decl* D) {
  const Entity* obj = context_.resolve(D);
  switch(D->getKind()) {
    case clang::Decl::CXXRecord: {
assert(context_.data_model_.CXXRecordDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXRecordDecl); 
    } break;
    case clang::Decl::Using: {
assert(context_.data_model_.UsingDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.UsingDecl); 
    } break;
    case clang::Decl::Empty: {
assert(context_.data_model_.EmptyDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.EmptyDecl); 
    } break;
    case clang::Decl::HLSLBuffer: {
assert(context_.data_model_.HLSLBufferDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.HLSLBufferDecl); 
    } break;
    case clang::Decl::ClassTemplateSpecialization: {
assert(context_.data_model_.ClassTemplateSpecializationDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ClassTemplateSpecializationDecl); 
    } break;
    case clang::Decl::NamespaceAlias: {
assert(context_.data_model_.NamespaceAliasDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.NamespaceAliasDecl); 
    } break;
    case clang::Decl::TemplateParamObject: {
assert(context_.data_model_.TemplateParamObjectDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.TemplateParamObjectDecl); 
    } break;
    case clang::Decl::MSProperty: {
assert(context_.data_model_.MSPropertyDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.MSPropertyDecl); 
    } break;
    case clang::Decl::ConstructorUsingShadow: {
assert(context_.data_model_.ConstructorUsingShadowDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ConstructorUsingShadowDecl); 
    } break;
    case clang::Decl::UsingShadow: {
assert(context_.data_model_.UsingShadowDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.UsingShadowDecl); 
    } break;
    case clang::Decl::TypeAlias: {
assert(context_.data_model_.TypeAliasDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.TypeAliasDecl); 
    } break;
    case clang::Decl::UsingEnum: {
assert(context_.data_model_.UsingEnumDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.UsingEnumDecl); 
    } break;
    case clang::Decl::ClassTemplate: {
assert(context_.data_model_.ClassTemplateDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ClassTemplateDecl); 
    } break;
    case clang::Decl::ObjCTypeParam: {
assert(context_.data_model_.ObjCTypeParamDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCTypeParamDecl); 
    } break;
    case clang::Decl::TemplateTemplateParm: {
assert(context_.data_model_.TemplateTemplateParmDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.TemplateTemplateParmDecl); 
    } break;
    case clang::Decl::Concept: {
assert(context_.data_model_.ConceptDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ConceptDecl); 
    } break;
    case clang::Decl::ObjCMethod: {
assert(context_.data_model_.ObjCMethodDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCMethodDecl); 
    } break;
    case clang::Decl::UnresolvedUsingIfExists: {
assert(context_.data_model_.UnresolvedUsingIfExistsDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.UnresolvedUsingIfExistsDecl); 
    } break;
    case clang::Decl::ObjCProtocol: {
assert(context_.data_model_.ObjCProtocolDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCProtocolDecl); 
    } break;
    case clang::Decl::OMPAllocate: {
assert(context_.data_model_.OMPAllocateDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPAllocateDecl); 
    } break;
    case clang::Decl::ObjCInterface: {
assert(context_.data_model_.ObjCInterfaceDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCInterfaceDecl); 
    } break;
    case clang::Decl::StaticAssert: {
assert(context_.data_model_.StaticAssertDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.StaticAssertDecl); 
    } break;
    case clang::Decl::ObjCPropertyImpl: {
assert(context_.data_model_.ObjCPropertyImplDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCPropertyImplDecl); 
    } break;
    case clang::Decl::CXXMethod: {
assert(context_.data_model_.CXXMethodDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXMethodDecl); 
    } break;
    case clang::Decl::Block: {
assert(context_.data_model_.BlockDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.BlockDecl); 
    } break;
    case clang::Decl::OMPThreadPrivate: {
assert(context_.data_model_.OMPThreadPrivateDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPThreadPrivateDecl); 
    } break;
    case clang::Decl::Binding: {
assert(context_.data_model_.BindingDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.BindingDecl); 
    } break;
    case clang::Decl::TypeAliasTemplate: {
assert(context_.data_model_.TypeAliasTemplateDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.TypeAliasTemplateDecl); 
    } break;
    case clang::Decl::UsingPack: {
assert(context_.data_model_.UsingPackDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.UsingPackDecl); 
    } break;
    case clang::Decl::Record: {
assert(context_.data_model_.RecordDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.RecordDecl); 
    } break;
    case clang::Decl::Typedef: {
assert(context_.data_model_.TypedefDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.TypedefDecl); 
    } break;
    case clang::Decl::ObjCAtDefsField: {
assert(context_.data_model_.ObjCAtDefsFieldDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCAtDefsFieldDecl); 
    } break;
    case clang::Decl::OMPDeclareReduction: {
assert(context_.data_model_.OMPDeclareReductionDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPDeclareReductionDecl); 
    } break;
    case clang::Decl::OMPDeclareMapper: {
assert(context_.data_model_.OMPDeclareMapperDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPDeclareMapperDecl); 
    } break;
    case clang::Decl::ObjCProperty: {
assert(context_.data_model_.ObjCPropertyDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCPropertyDecl); 
    } break;
    case clang::Decl::MSGuid: {
assert(context_.data_model_.MSGuidDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.MSGuidDecl); 
    } break;
    case clang::Decl::Friend: {
assert(context_.data_model_.FriendDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.FriendDecl); 
    } break;
    case clang::Decl::RequiresExprBody: {
assert(context_.data_model_.RequiresExprBodyDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.RequiresExprBodyDecl); 
    } break;
    case clang::Decl::CXXConstructor: {
assert(context_.data_model_.CXXConstructorDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXConstructorDecl); 
    } break;
    case clang::Decl::EnumConstant: {
assert(context_.data_model_.EnumConstantDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.EnumConstantDecl); 
    } break;
    case clang::Decl::IndirectField: {
assert(context_.data_model_.IndirectFieldDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.IndirectFieldDecl); 
    } break;
    case clang::Decl::UnnamedGlobalConstant: {
assert(context_.data_model_.UnnamedGlobalConstantDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.UnnamedGlobalConstantDecl); 
    } break;
    case clang::Decl::Label: {
assert(context_.data_model_.LabelDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.LabelDecl); 
    } break;
    case clang::Decl::PragmaDetectMismatch: {
assert(context_.data_model_.PragmaDetectMismatchDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.PragmaDetectMismatchDecl); 
    } break;
    case clang::Decl::ImplicitParam: {
assert(context_.data_model_.ImplicitParamDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ImplicitParamDecl); 
    } break;
    case clang::Decl::ParmVar: {
assert(context_.data_model_.ParmVarDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ParmVarDecl); 
    } break;
    case clang::Decl::CXXConversion: {
assert(context_.data_model_.CXXConversionDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXConversionDecl); 
    } break;
    case clang::Decl::Enum: {
assert(context_.data_model_.EnumDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.EnumDecl); 
    } break;
    case clang::Decl::CXXDestructor: {
assert(context_.data_model_.CXXDestructorDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXDestructorDecl); 
    } break;
    case clang::Decl::VarTemplatePartialSpecialization: {
assert(context_.data_model_.VarTemplatePartialSpecializationDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.VarTemplatePartialSpecializationDecl); 
    } break;
    case clang::Decl::OMPRequires: {
assert(context_.data_model_.OMPRequiresDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPRequiresDecl); 
    } break;
    case clang::Decl::CXXDeductionGuide: {
assert(context_.data_model_.CXXDeductionGuideDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXDeductionGuideDecl); 
    } break;
    case clang::Decl::ObjCIvar: {
assert(context_.data_model_.ObjCIvarDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCIvarDecl); 
    } break;
    case clang::Decl::NonTypeTemplateParm: {
assert(context_.data_model_.NonTypeTemplateParmDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.NonTypeTemplateParmDecl); 
    } break;
    case clang::Decl::ObjCCompatibleAlias: {
assert(context_.data_model_.ObjCCompatibleAliasDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCCompatibleAliasDecl); 
    } break;
    case clang::Decl::UnresolvedUsingValue: {
assert(context_.data_model_.UnresolvedUsingValueDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.UnresolvedUsingValueDecl); 
    } break;
    case clang::Decl::Namespace: {
assert(context_.data_model_.NamespaceDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.NamespaceDecl); 
    } break;
    case clang::Decl::ImplicitConceptSpecialization: {
assert(context_.data_model_.ImplicitConceptSpecializationDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ImplicitConceptSpecializationDecl); 
    } break;
    case clang::Decl::VarTemplateSpecialization: {
assert(context_.data_model_.VarTemplateSpecializationDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.VarTemplateSpecializationDecl); 
    } break;
    case clang::Decl::Decomposition: {
assert(context_.data_model_.DecompositionDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.DecompositionDecl); 
    } break;
    case clang::Decl::Field: {
assert(context_.data_model_.FieldDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.FieldDecl); 
    } break;
    case clang::Decl::Captured: {
assert(context_.data_model_.CapturedDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CapturedDecl); 
    } break;
    case clang::Decl::OMPCapturedExpr: {
assert(context_.data_model_.OMPCapturedExprDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPCapturedExprDecl); 
    } break;
    case clang::Decl::ClassTemplatePartialSpecialization: {
assert(context_.data_model_.ClassTemplatePartialSpecializationDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ClassTemplatePartialSpecializationDecl); 
    } break;
    case clang::Decl::PragmaComment: {
assert(context_.data_model_.PragmaCommentDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.PragmaCommentDecl); 
    } break;
    case clang::Decl::Function: {
assert(context_.data_model_.FunctionDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.FunctionDecl); 
    } break;
    case clang::Decl::ObjCCategoryImpl: {
assert(context_.data_model_.ObjCCategoryImplDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCCategoryImplDecl); 
    } break;
    case clang::Decl::BuiltinTemplate: {
assert(context_.data_model_.BuiltinTemplateDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.BuiltinTemplateDecl); 
    } break;
    case clang::Decl::TopLevelStmt: {
assert(context_.data_model_.TopLevelStmtDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.TopLevelStmtDecl); 
    } break;
    case clang::Decl::VarTemplate: {
assert(context_.data_model_.VarTemplateDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.VarTemplateDecl); 
    } break;
    case clang::Decl::Export: {
assert(context_.data_model_.ExportDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ExportDecl); 
    } break;
    case clang::Decl::AccessSpec: {
assert(context_.data_model_.AccessSpecDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.AccessSpecDecl); 
    } break;
    case clang::Decl::FunctionTemplate: {
assert(context_.data_model_.FunctionTemplateDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.FunctionTemplateDecl); 
    } break;
    case clang::Decl::TemplateTypeParm: {
assert(context_.data_model_.TemplateTypeParmDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.TemplateTypeParmDecl); 
    } break;
    case clang::Decl::ExternCContext: {
assert(context_.data_model_.ExternCContextDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ExternCContextDecl); 
    } break;
    case clang::Decl::LinkageSpec: {
assert(context_.data_model_.LinkageSpecDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.LinkageSpecDecl); 
    } break;
    case clang::Decl::ObjCImplementation: {
assert(context_.data_model_.ObjCImplementationDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCImplementationDecl); 
    } break;
    case clang::Decl::UnresolvedUsingTypename: {
assert(context_.data_model_.UnresolvedUsingTypenameDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.UnresolvedUsingTypenameDecl); 
    } break;
    case clang::Decl::FileScopeAsm: {
assert(context_.data_model_.FileScopeAsmDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.FileScopeAsmDecl); 
    } break;
    case clang::Decl::Var: {
assert(context_.data_model_.VarDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.VarDecl); 
    } break;
    case clang::Decl::LifetimeExtendedTemporary: {
assert(context_.data_model_.LifetimeExtendedTemporaryDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.LifetimeExtendedTemporaryDecl); 
    } break;
    case clang::Decl::UsingDirective: {
assert(context_.data_model_.UsingDirectiveDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.UsingDirectiveDecl); 
    } break;
    case clang::Decl::Import: {
assert(context_.data_model_.ImportDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ImportDecl); 
    } break;
    case clang::Decl::ObjCCategory: {
assert(context_.data_model_.ObjCCategoryDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCCategoryDecl); 
    } break;
    case clang::Decl::TranslationUnit: {
assert(context_.data_model_.TranslationUnitDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.TranslationUnitDecl); 
    } break;
    case clang::Decl::FriendTemplate: {
assert(context_.data_model_.FriendTemplateDecl != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.FriendTemplateDecl); 
    } break;
    default: break;
  }

  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange25, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc109, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc110, other);
  }
  //getLocation
  {
    const Entity* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.getLocation13, other);
  }
  //getNextDeclInContext
  {
    const Entity* other = context_.resolve(D->getNextDeclInContext());
    arboretum_create_edge(obj, context_.data_model_.getNextDeclInContext1, other);
  }
  //getDeclContext
  //getNonTransparentDeclContext
  //getNonClosureContext
  {
    const Entity* other = context_.resolve(D->getNonClosureContext());
    arboretum_create_edge(obj, context_.data_model_.getNonClosureContext1, other);
  }
  //getTranslationUnitDecl
  {
    const Entity* other = context_.resolve(D->getTranslationUnitDecl());
    arboretum_create_edge(obj, context_.data_model_.getTranslationUnitDecl1, other);
  }
  //isInAnonymousNamespace
  arboretum_create_edge(obj, context_.data_model_.isInAnonymousNamespace, context_.data_model_.arboretum_node_for(D->isInAnonymousNamespace()));
  //isInStdNamespace
  arboretum_create_edge(obj, context_.data_model_.isInStdNamespace, context_.data_model_.arboretum_node_for(D->isInStdNamespace()));
  //isFileContextDecl
  arboretum_create_edge(obj, context_.data_model_.isFileContextDecl, context_.data_model_.arboretum_node_for(D->isFileContextDecl()));
  //getLangOpts
  // const LangOptions &
  //getAccess
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getAccess());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getAccess, enum_value);
    }
  }
  //getAccessUnsafe
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getAccessUnsafe());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getAccessUnsafe, enum_value);
    }
  }
  //hasAttrs
  arboretum_create_edge(obj, context_.data_model_.hasAttrs, context_.data_model_.arboretum_node_for(D->hasAttrs()));
  //getAttrs
  // const AttrVec &
  //attrs
  // attr_range
  //getMaxAlignment
  // unsigned int
  //isInvalidDecl
  arboretum_create_edge(obj, context_.data_model_.isInvalidDecl, context_.data_model_.arboretum_node_for(D->isInvalidDecl()));
  //isImplicit
  arboretum_create_edge(obj, context_.data_model_.isImplicit2, context_.data_model_.arboretum_node_for(D->isImplicit()));
  //isReferenced
  arboretum_create_edge(obj, context_.data_model_.isReferenced, context_.data_model_.arboretum_node_for(D->isReferenced()));
  //isThisDeclarationReferenced
  arboretum_create_edge(obj, context_.data_model_.isThisDeclarationReferenced, context_.data_model_.arboretum_node_for(D->isThisDeclarationReferenced()));
  //isTopLevelDeclInObjCContainer
  arboretum_create_edge(obj, context_.data_model_.isTopLevelDeclInObjCContainer, context_.data_model_.arboretum_node_for(D->isTopLevelDeclInObjCContainer()));
  //isModulePrivate
  arboretum_create_edge(obj, context_.data_model_.isModulePrivate, context_.data_model_.arboretum_node_for(D->isModulePrivate()));
  //isInExportDeclContext
  arboretum_create_edge(obj, context_.data_model_.isInExportDeclContext, context_.data_model_.arboretum_node_for(D->isInExportDeclContext()));
  //isInvisibleOutsideTheOwningModule
  arboretum_create_edge(obj, context_.data_model_.isInvisibleOutsideTheOwningModule, context_.data_model_.arboretum_node_for(D->isInvisibleOutsideTheOwningModule()));
  //isInAnotherModuleUnit
  arboretum_create_edge(obj, context_.data_model_.isInAnotherModuleUnit, context_.data_model_.arboretum_node_for(D->isInAnotherModuleUnit()));
  //isDiscardedInGlobalModuleFragment
  arboretum_create_edge(obj, context_.data_model_.isDiscardedInGlobalModuleFragment, context_.data_model_.arboretum_node_for(D->isDiscardedInGlobalModuleFragment()));
  //shouldSkipCheckingODR
  arboretum_create_edge(obj, context_.data_model_.shouldSkipCheckingODR, context_.data_model_.arboretum_node_for(D->shouldSkipCheckingODR()));
  //hasDefiningAttr
  arboretum_create_edge(obj, context_.data_model_.hasDefiningAttr, context_.data_model_.arboretum_node_for(D->hasDefiningAttr()));
  //getDefiningAttr
  {
    const Entity* other = context_.resolve(D->getDefiningAttr());
    arboretum_create_edge(obj, context_.data_model_.getDefiningAttr, other);
  }
  //getVersionIntroduced
  // VersionTuple
  //isWeakImported
  arboretum_create_edge(obj, context_.data_model_.isWeakImported, context_.data_model_.arboretum_node_for(D->isWeakImported()));
  //isFromASTFile
  arboretum_create_edge(obj, context_.data_model_.isFromASTFile, context_.data_model_.arboretum_node_for(D->isFromASTFile()));
  //getGlobalID
  // unsigned int
  //getOwningModuleID
  // unsigned int
  //getImportedOwningModule
  //getLocalOwningModule
  //hasOwningModule
  arboretum_create_edge(obj, context_.data_model_.hasOwningModule, context_.data_model_.arboretum_node_for(D->hasOwningModule()));
  //getOwningModule
  //isUnconditionallyVisible
  arboretum_create_edge(obj, context_.data_model_.isUnconditionallyVisible, context_.data_model_.arboretum_node_for(D->isUnconditionallyVisible()));
  //isReachable
  arboretum_create_edge(obj, context_.data_model_.isReachable, context_.data_model_.arboretum_node_for(D->isReachable()));
  //getModuleOwnershipKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getModuleOwnershipKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getModuleOwnershipKind, enum_value);
    }
  }
  //getIdentifierNamespace
  // unsigned int
  //hasTagIdentifierNamespace
  arboretum_create_edge(obj, context_.data_model_.hasTagIdentifierNamespace, context_.data_model_.arboretum_node_for(D->hasTagIdentifierNamespace()));
  //getLexicalDeclContext
  //isOutOfLine
  arboretum_create_edge(obj, context_.data_model_.isOutOfLine1, context_.data_model_.arboretum_node_for(D->isOutOfLine()));
  //isTemplated
  arboretum_create_edge(obj, context_.data_model_.isTemplated, context_.data_model_.arboretum_node_for(D->isTemplated()));
  //getTemplateDepth
  // unsigned int
  //isDefinedOutsideFunctionOrMethod
  arboretum_create_edge(obj, context_.data_model_.isDefinedOutsideFunctionOrMethod, context_.data_model_.arboretum_node_for(D->isDefinedOutsideFunctionOrMethod()));
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl25, other);
  }
  //isCanonicalDecl
  arboretum_create_edge(obj, context_.data_model_.isCanonicalDecl, context_.data_model_.arboretum_node_for(D->isCanonicalDecl()));
  //redecls
  // redecl_range
  //getPreviousDecl
  {
    const Entity* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_edge(obj, context_.data_model_.getPreviousDecl9, other);
  }
  //isFirstDecl
  arboretum_create_edge(obj, context_.data_model_.isFirstDecl, context_.data_model_.arboretum_node_for(D->isFirstDecl()));
  //getMostRecentDecl
  {
    const Entity* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_edge(obj, context_.data_model_.getMostRecentDecl12, other);
  }
  //getBody
  {
    const Entity* other = context_.resolve(D->getBody());
    arboretum_create_edge(obj, context_.data_model_.getBody13, other);
  }
  //hasBody
  arboretum_create_edge(obj, context_.data_model_.hasBody, context_.data_model_.arboretum_node_for(D->hasBody()));
  //getBodyRBrace
  {
    const Entity* other = context_.source_model_.resolve(D->getBodyRBrace());
    arboretum_create_edge(obj, context_.data_model_.getBodyRBrace, other);
  }
  //isTemplateParameter
  arboretum_create_edge(obj, context_.data_model_.isTemplateParameter, context_.data_model_.arboretum_node_for(D->isTemplateParameter()));
  //isTemplateParameterPack
  arboretum_create_edge(obj, context_.data_model_.isTemplateParameterPack, context_.data_model_.arboretum_node_for(D->isTemplateParameterPack()));
  //isParameterPack
  arboretum_create_edge(obj, context_.data_model_.isParameterPack3, context_.data_model_.arboretum_node_for(D->isParameterPack()));
  //isTemplateDecl
  arboretum_create_edge(obj, context_.data_model_.isTemplateDecl, context_.data_model_.arboretum_node_for(D->isTemplateDecl()));
  //isFunctionOrFunctionTemplate
  arboretum_create_edge(obj, context_.data_model_.isFunctionOrFunctionTemplate, context_.data_model_.arboretum_node_for(D->isFunctionOrFunctionTemplate()));
  //getDescribedTemplate
  {
    const Entity* other = context_.resolve(D->getDescribedTemplate());
    arboretum_create_edge(obj, context_.data_model_.getDescribedTemplate, other);
  }
  //getDescribedTemplateParams
  //getAsFunction
  {
    const Entity* other = context_.resolve(D->getAsFunction());
    arboretum_create_edge(obj, context_.data_model_.getAsFunction1, other);
  }
  //isLocalExternDecl
  arboretum_create_edge(obj, context_.data_model_.isLocalExternDecl, context_.data_model_.arboretum_node_for(D->isLocalExternDecl()));
  //getFriendObjectKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getFriendObjectKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getFriendObjectKind, enum_value);
    }
  }
  //getID
  // int64_t
  //isFunctionPointerType
  arboretum_create_edge(obj, context_.data_model_.isFunctionPointerType, context_.data_model_.arboretum_node_for(D->isFunctionPointerType()));
  return true;
}

bool ArboretumASTVisitor::VisitDeclaratorDecl(clang::DeclaratorDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getTypeSourceInfo
  //getInnerLocStart
  {
    const Entity* other = context_.source_model_.resolve(D->getInnerLocStart());
    arboretum_create_edge(obj, context_.data_model_.getInnerLocStart1, other);
  }
  //getOuterLocStart
  {
    const Entity* other = context_.source_model_.resolve(D->getOuterLocStart());
    arboretum_create_edge(obj, context_.data_model_.getOuterLocStart1, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange45, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc128, other);
  }
  //getQualifier
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getTrailingRequiresClause
  {
    const Entity* other = context_.resolve(D->getTrailingRequiresClause());
    arboretum_create_edge(obj, context_.data_model_.getTrailingRequiresClause2, other);
  }
  //getNumTemplateParameterLists
  // unsigned int
  //getTypeSpecStartLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getTypeSpecStartLoc());
    arboretum_create_edge(obj, context_.data_model_.getTypeSpecStartLoc, other);
  }
  //getTypeSpecEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getTypeSpecEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getTypeSpecEndLoc, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitDecompositionDecl(clang::DecompositionDecl* D) {
  const Entity* obj = context_.resolve(D);
  //bindings
  // ArrayRef<BindingDecl *>
  return true;
}

bool ArboretumASTVisitor::VisitEmptyDecl(clang::EmptyDecl* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitEnumConstantDecl(clang::EnumConstantDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getInitExpr
  {
    const Entity* other = context_.resolve(D->getInitExpr());
    arboretum_create_edge(obj, context_.data_model_.getInitExpr, other);
  }
  //getInitVal
  // llvm::APSInt
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange43, other);
  }
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl51, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitEnumDecl(clang::EnumDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Entity* obj = context_.resolve(D);
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl31, other);
  }
  //getPreviousDecl
  {
    const Entity* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_edge(obj, context_.data_model_.getPreviousDecl15, other);
  }
  //getMostRecentDecl
  {
    const Entity* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_edge(obj, context_.data_model_.getMostRecentDecl18, other);
  }
  //getDefinition
  {
    const Entity* other = context_.resolve(D->getDefinition());
    arboretum_create_edge(obj, context_.data_model_.getDefinition14, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange27, other);
  }
  //enumerators
  // enumerator_range
  //getPromotionType
  {
    const Entity* other = context_.resolve(D->getPromotionType());
    arboretum_create_edge(obj, context_.data_model_.getPromotionType, other);
  }
  //getIntegerType
  {
    const Entity* other = context_.resolve(D->getIntegerType());
    arboretum_create_edge(obj, context_.data_model_.getIntegerType, other);
  }
  //getIntegerTypeSourceInfo
  //getIntegerTypeRange
  {
    const Entity* other = context_.source_model_.resolve(D->getIntegerTypeRange());
    arboretum_create_edge(obj, context_.data_model_.getIntegerTypeRange, other);
  }
  //getNumPositiveBits
  // unsigned int
  //getNumNegativeBits
  // unsigned int
  //isScoped
  arboretum_create_edge(obj, context_.data_model_.isScoped, context_.data_model_.arboretum_node_for(D->isScoped()));
  //isScopedUsingClassTag
  arboretum_create_edge(obj, context_.data_model_.isScopedUsingClassTag, context_.data_model_.arboretum_node_for(D->isScopedUsingClassTag()));
  //isFixed
  arboretum_create_edge(obj, context_.data_model_.isFixed, context_.data_model_.arboretum_node_for(D->isFixed()));
  //isComplete
  arboretum_create_edge(obj, context_.data_model_.isComplete, context_.data_model_.arboretum_node_for(D->isComplete()));
  //isClosed
  arboretum_create_edge(obj, context_.data_model_.isClosed, context_.data_model_.arboretum_node_for(D->isClosed()));
  //isClosedFlag
  arboretum_create_edge(obj, context_.data_model_.isClosedFlag, context_.data_model_.arboretum_node_for(D->isClosedFlag()));
  //isClosedNonFlag
  arboretum_create_edge(obj, context_.data_model_.isClosedNonFlag, context_.data_model_.arboretum_node_for(D->isClosedNonFlag()));
  //getTemplateInstantiationPattern
  {
    const Entity* other = context_.resolve(D->getTemplateInstantiationPattern());
    arboretum_create_edge(obj, context_.data_model_.getTemplateInstantiationPattern4, other);
  }
  //getInstantiatedFromMemberEnum
  {
    const Entity* other = context_.resolve(D->getInstantiatedFromMemberEnum());
    arboretum_create_edge(obj, context_.data_model_.getInstantiatedFromMemberEnum, other);
  }
  //getTemplateSpecializationKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getTemplateSpecializationKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getTemplateSpecializationKind3, enum_value);
    }
  }
  //getMemberSpecializationInfo
  return true;
}

bool ArboretumASTVisitor::VisitExportDecl(clang::ExportDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getExportLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getExportLoc());
    arboretum_create_edge(obj, context_.data_model_.getExportLoc, other);
  }
  //getRBraceLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRBraceLoc());
    arboretum_create_edge(obj, context_.data_model_.getRBraceLoc2, other);
  }
  //hasBraces
  arboretum_create_edge(obj, context_.data_model_.hasBraces1, context_.data_model_.arboretum_node_for(D->hasBraces()));
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc109, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange23, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitExternCContextDecl(clang::ExternCContextDecl* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitFieldDecl(clang::FieldDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getFieldIndex
  // unsigned int
  //isMutable
  arboretum_create_edge(obj, context_.data_model_.isMutable1, context_.data_model_.arboretum_node_for(D->isMutable()));
  //isBitField
  arboretum_create_edge(obj, context_.data_model_.isBitField, context_.data_model_.arboretum_node_for(D->isBitField()));
  //isUnnamedBitfield
  arboretum_create_edge(obj, context_.data_model_.isUnnamedBitfield, context_.data_model_.arboretum_node_for(D->isUnnamedBitfield()));
  //isAnonymousStructOrUnion
  arboretum_create_edge(obj, context_.data_model_.isAnonymousStructOrUnion1, context_.data_model_.arboretum_node_for(D->isAnonymousStructOrUnion()));
  //getBitWidth
  {
    const Entity* other = context_.resolve(D->getBitWidth());
    arboretum_create_edge(obj, context_.data_model_.getBitWidth, other);
  }
  //isPotentiallyOverlapping
  arboretum_create_edge(obj, context_.data_model_.isPotentiallyOverlapping, context_.data_model_.arboretum_node_for(D->isPotentiallyOverlapping()));
  //getInClassInitStyle
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getInClassInitStyle());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getInClassInitStyle, enum_value);
    }
  }
  //hasInClassInitializer
  arboretum_create_edge(obj, context_.data_model_.hasInClassInitializer1, context_.data_model_.arboretum_node_for(D->hasInClassInitializer()));
  //hasNonNullInClassInitializer
  arboretum_create_edge(obj, context_.data_model_.hasNonNullInClassInitializer, context_.data_model_.arboretum_node_for(D->hasNonNullInClassInitializer()));
  //getInClassInitializer
  {
    const Entity* other = context_.resolve(D->getInClassInitializer());
    arboretum_create_edge(obj, context_.data_model_.getInClassInitializer, other);
  }
  //hasCapturedVLAType
  arboretum_create_edge(obj, context_.data_model_.hasCapturedVLAType, context_.data_model_.arboretum_node_for(D->hasCapturedVLAType()));
  //getCapturedVLAType
  {
    const Entity* other = context_.resolve(D->getCapturedVLAType());
    arboretum_create_edge(obj, context_.data_model_.getCapturedVLAType, other);
  }
  //getParent
  {
    const Entity* other = context_.resolve(D->getParent());
    arboretum_create_edge(obj, context_.data_model_.getParent2, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange30, other);
  }
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl37, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitFileScopeAsmDecl(clang::FileScopeAsmDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getAsmLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getAsmLoc());
    arboretum_create_edge(obj, context_.data_model_.getAsmLoc1, other);
  }
  //getRParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc22, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange21, other);
  }
  //getAsmString
  {
    const Entity* other = context_.resolve(D->getAsmString());
    arboretum_create_edge(obj, context_.data_model_.getAsmString3, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitFriendDecl(clang::FriendDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getFriendType
  //getFriendTypeNumTemplateParameterLists
  // unsigned int
  //getFriendDecl
  {
    const Entity* other = context_.resolve(D->getFriendDecl());
    arboretum_create_edge(obj, context_.data_model_.getFriendDecl, other);
  }
  //getFriendLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getFriendLoc());
    arboretum_create_edge(obj, context_.data_model_.getFriendLoc, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange47, other);
  }
  //isUnsupportedFriend
  arboretum_create_edge(obj, context_.data_model_.isUnsupportedFriend, context_.data_model_.arboretum_node_for(D->isUnsupportedFriend()));
  return true;
}

bool ArboretumASTVisitor::VisitFriendTemplateDecl(clang::FriendTemplateDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getFriendType
  //getFriendDecl
  {
    const Entity* other = context_.resolve(D->getFriendDecl());
    arboretum_create_edge(obj, context_.data_model_.getFriendDecl1, other);
  }
  //getFriendLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getFriendLoc());
    arboretum_create_edge(obj, context_.data_model_.getFriendLoc1, other);
  }
  //getNumTemplateParameters
  // unsigned int
  return true;
}

bool ArboretumASTVisitor::VisitFunctionDecl(clang::FunctionDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Entity* obj = context_.resolve(D);
  //getNameInfo
  // DeclarationNameInfo
  //getEllipsisLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEllipsisLoc());
    arboretum_create_edge(obj, context_.data_model_.getEllipsisLoc3, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange26, other);
  }
  //hasBody
  arboretum_create_edge(obj, context_.data_model_.hasBody2, context_.data_model_.arboretum_node_for(D->hasBody()));
  //hasTrivialBody
  arboretum_create_edge(obj, context_.data_model_.hasTrivialBody, context_.data_model_.arboretum_node_for(D->hasTrivialBody()));
  //isDefined
  arboretum_create_edge(obj, context_.data_model_.isDefined1, context_.data_model_.arboretum_node_for(D->isDefined()));
  //getDefinition
  {
    const Entity* other = context_.resolve(D->getDefinition());
    arboretum_create_edge(obj, context_.data_model_.getDefinition12, other);
  }
  //getBody
  {
    const Entity* other = context_.resolve(D->getBody());
    arboretum_create_edge(obj, context_.data_model_.getBody15, other);
  }
  //isThisDeclarationADefinition
  arboretum_create_edge(obj, context_.data_model_.isThisDeclarationADefinition8, context_.data_model_.arboretum_node_for(D->isThisDeclarationADefinition()));
  //isThisDeclarationInstantiatedFromAFriendDefinition
  arboretum_create_edge(obj, context_.data_model_.isThisDeclarationInstantiatedFromAFriendDefinition, context_.data_model_.arboretum_node_for(D->isThisDeclarationInstantiatedFromAFriendDefinition()));
  //doesThisDeclarationHaveABody
  arboretum_create_edge(obj, context_.data_model_.doesThisDeclarationHaveABody, context_.data_model_.arboretum_node_for(D->doesThisDeclarationHaveABody()));
  //getDefaultedFunctionInfo
  //isVariadic
  arboretum_create_edge(obj, context_.data_model_.isVariadic, context_.data_model_.arboretum_node_for(D->isVariadic()));
  //isVirtualAsWritten
  arboretum_create_edge(obj, context_.data_model_.isVirtualAsWritten, context_.data_model_.arboretum_node_for(D->isVirtualAsWritten()));
  //isPureVirtual
  arboretum_create_edge(obj, context_.data_model_.isPureVirtual, context_.data_model_.arboretum_node_for(D->isPureVirtual()));
  //isLateTemplateParsed
  arboretum_create_edge(obj, context_.data_model_.isLateTemplateParsed, context_.data_model_.arboretum_node_for(D->isLateTemplateParsed()));
  //isTrivial
  arboretum_create_edge(obj, context_.data_model_.isTrivial1, context_.data_model_.arboretum_node_for(D->isTrivial()));
  //isTrivialForCall
  arboretum_create_edge(obj, context_.data_model_.isTrivialForCall, context_.data_model_.arboretum_node_for(D->isTrivialForCall()));
  //isDefaulted
  arboretum_create_edge(obj, context_.data_model_.isDefaulted, context_.data_model_.arboretum_node_for(D->isDefaulted()));
  //isExplicitlyDefaulted
  arboretum_create_edge(obj, context_.data_model_.isExplicitlyDefaulted, context_.data_model_.arboretum_node_for(D->isExplicitlyDefaulted()));
  //getDefaultLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getDefaultLoc());
    arboretum_create_edge(obj, context_.data_model_.getDefaultLoc1, other);
  }
  //isUserProvided
  arboretum_create_edge(obj, context_.data_model_.isUserProvided, context_.data_model_.arboretum_node_for(D->isUserProvided()));
  //isIneligibleOrNotSelected
  arboretum_create_edge(obj, context_.data_model_.isIneligibleOrNotSelected, context_.data_model_.arboretum_node_for(D->isIneligibleOrNotSelected()));
  //hasImplicitReturnZero
  arboretum_create_edge(obj, context_.data_model_.hasImplicitReturnZero, context_.data_model_.arboretum_node_for(D->hasImplicitReturnZero()));
  //hasPrototype
  arboretum_create_edge(obj, context_.data_model_.hasPrototype, context_.data_model_.arboretum_node_for(D->hasPrototype()));
  //hasWrittenPrototype
  arboretum_create_edge(obj, context_.data_model_.hasWrittenPrototype, context_.data_model_.arboretum_node_for(D->hasWrittenPrototype()));
  //hasInheritedPrototype
  arboretum_create_edge(obj, context_.data_model_.hasInheritedPrototype, context_.data_model_.arboretum_node_for(D->hasInheritedPrototype()));
  //isConstexpr
  arboretum_create_edge(obj, context_.data_model_.isConstexpr1, context_.data_model_.arboretum_node_for(D->isConstexpr()));
  //getConstexprKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getConstexprKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getConstexprKind, enum_value);
    }
  }
  //isConstexprSpecified
  arboretum_create_edge(obj, context_.data_model_.isConstexprSpecified, context_.data_model_.arboretum_node_for(D->isConstexprSpecified()));
  //isConsteval
  arboretum_create_edge(obj, context_.data_model_.isConsteval, context_.data_model_.arboretum_node_for(D->isConsteval()));
  //BodyContainsImmediateEscalatingExpressions
  arboretum_create_edge(obj, context_.data_model_.BodyContainsImmediateEscalatingExpressions, context_.data_model_.arboretum_node_for(D->BodyContainsImmediateEscalatingExpressions()));
  //isImmediateEscalating
  arboretum_create_edge(obj, context_.data_model_.isImmediateEscalating2, context_.data_model_.arboretum_node_for(D->isImmediateEscalating()));
  //isImmediateFunction
  arboretum_create_edge(obj, context_.data_model_.isImmediateFunction, context_.data_model_.arboretum_node_for(D->isImmediateFunction()));
  //instantiationIsPending
  arboretum_create_edge(obj, context_.data_model_.instantiationIsPending, context_.data_model_.arboretum_node_for(D->instantiationIsPending()));
  //usesSEHTry
  arboretum_create_edge(obj, context_.data_model_.usesSEHTry, context_.data_model_.arboretum_node_for(D->usesSEHTry()));
  //isDeleted
  arboretum_create_edge(obj, context_.data_model_.isDeleted, context_.data_model_.arboretum_node_for(D->isDeleted()));
  //isDeletedAsWritten
  arboretum_create_edge(obj, context_.data_model_.isDeletedAsWritten, context_.data_model_.arboretum_node_for(D->isDeletedAsWritten()));
  //isMain
  arboretum_create_edge(obj, context_.data_model_.isMain, context_.data_model_.arboretum_node_for(D->isMain()));
  //isMSVCRTEntryPoint
  arboretum_create_edge(obj, context_.data_model_.isMSVCRTEntryPoint, context_.data_model_.arboretum_node_for(D->isMSVCRTEntryPoint()));
  //isReservedGlobalPlacementOperator
  arboretum_create_edge(obj, context_.data_model_.isReservedGlobalPlacementOperator, context_.data_model_.arboretum_node_for(D->isReservedGlobalPlacementOperator()));
  //isInlineBuiltinDeclaration
  arboretum_create_edge(obj, context_.data_model_.isInlineBuiltinDeclaration, context_.data_model_.arboretum_node_for(D->isInlineBuiltinDeclaration()));
  //isDestroyingOperatorDelete
  arboretum_create_edge(obj, context_.data_model_.isDestroyingOperatorDelete, context_.data_model_.arboretum_node_for(D->isDestroyingOperatorDelete()));
  //getLanguageLinkage
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getLanguageLinkage());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getLanguageLinkage1, enum_value);
    }
  }
  //isExternC
  arboretum_create_edge(obj, context_.data_model_.isExternC1, context_.data_model_.arboretum_node_for(D->isExternC()));
  //isInExternCContext
  arboretum_create_edge(obj, context_.data_model_.isInExternCContext1, context_.data_model_.arboretum_node_for(D->isInExternCContext()));
  //isInExternCXXContext
  arboretum_create_edge(obj, context_.data_model_.isInExternCXXContext1, context_.data_model_.arboretum_node_for(D->isInExternCXXContext()));
  //isGlobal
  arboretum_create_edge(obj, context_.data_model_.isGlobal, context_.data_model_.arboretum_node_for(D->isGlobal()));
  //isNoReturn
  arboretum_create_edge(obj, context_.data_model_.isNoReturn, context_.data_model_.arboretum_node_for(D->isNoReturn()));
  //hasSkippedBody
  arboretum_create_edge(obj, context_.data_model_.hasSkippedBody, context_.data_model_.arboretum_node_for(D->hasSkippedBody()));
  //willHaveBody
  arboretum_create_edge(obj, context_.data_model_.willHaveBody, context_.data_model_.arboretum_node_for(D->willHaveBody()));
  //isMultiVersion
  arboretum_create_edge(obj, context_.data_model_.isMultiVersion, context_.data_model_.arboretum_node_for(D->isMultiVersion()));
  //FriendConstraintRefersToEnclosingTemplate
  arboretum_create_edge(obj, context_.data_model_.FriendConstraintRefersToEnclosingTemplate, context_.data_model_.arboretum_node_for(D->FriendConstraintRefersToEnclosingTemplate()));
  //isMemberLikeConstrainedFriend
  arboretum_create_edge(obj, context_.data_model_.isMemberLikeConstrainedFriend, context_.data_model_.arboretum_node_for(D->isMemberLikeConstrainedFriend()));
  //getMultiVersionKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getMultiVersionKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getMultiVersionKind, enum_value);
    }
  }
  //isCPUDispatchMultiVersion
  arboretum_create_edge(obj, context_.data_model_.isCPUDispatchMultiVersion, context_.data_model_.arboretum_node_for(D->isCPUDispatchMultiVersion()));
  //isCPUSpecificMultiVersion
  arboretum_create_edge(obj, context_.data_model_.isCPUSpecificMultiVersion, context_.data_model_.arboretum_node_for(D->isCPUSpecificMultiVersion()));
  //isTargetMultiVersion
  arboretum_create_edge(obj, context_.data_model_.isTargetMultiVersion, context_.data_model_.arboretum_node_for(D->isTargetMultiVersion()));
  //isTargetClonesMultiVersion
  arboretum_create_edge(obj, context_.data_model_.isTargetClonesMultiVersion, context_.data_model_.arboretum_node_for(D->isTargetClonesMultiVersion()));
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl27, other);
  }
  //parameters
  // ArrayRef<ParmVarDecl *>
  //param_empty
  arboretum_create_edge(obj, context_.data_model_.param_empty, context_.data_model_.arboretum_node_for(D->param_empty()));
  //param_size
  // size_t
  //getNumParams
  // unsigned int
  //getMinRequiredArguments
  // unsigned int
  //getMinRequiredExplicitArguments
  // unsigned int
  //hasCXXExplicitFunctionObjectParameter
  arboretum_create_edge(obj, context_.data_model_.hasCXXExplicitFunctionObjectParameter, context_.data_model_.arboretum_node_for(D->hasCXXExplicitFunctionObjectParameter()));
  //getNumNonObjectParams
  // unsigned int
  //hasOneParamOrDefaultArgs
  arboretum_create_edge(obj, context_.data_model_.hasOneParamOrDefaultArgs, context_.data_model_.arboretum_node_for(D->hasOneParamOrDefaultArgs()));
  //getFunctionTypeLoc
  // FunctionTypeLoc
  //getReturnType
  {
    const Entity* other = context_.resolve(D->getReturnType());
    arboretum_create_edge(obj, context_.data_model_.getReturnType, other);
  }
  //getReturnTypeSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getReturnTypeSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getReturnTypeSourceRange, other);
  }
  //getParametersSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getParametersSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getParametersSourceRange, other);
  }
  //getDeclaredReturnType
  {
    const Entity* other = context_.resolve(D->getDeclaredReturnType());
    arboretum_create_edge(obj, context_.data_model_.getDeclaredReturnType, other);
  }
  //getExceptionSpecType
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getExceptionSpecType());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getExceptionSpecType, enum_value);
    }
  }
  //getExceptionSpecSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getExceptionSpecSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getExceptionSpecSourceRange, other);
  }
  //getCallResultType
  {
    const Entity* other = context_.resolve(D->getCallResultType());
    arboretum_create_edge(obj, context_.data_model_.getCallResultType, other);
  }
  //getStorageClass
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getStorageClass());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getStorageClass1, enum_value);
    }
  }
  //isInlineSpecified
  arboretum_create_edge(obj, context_.data_model_.isInlineSpecified1, context_.data_model_.arboretum_node_for(D->isInlineSpecified()));
  //UsesFPIntrin
  arboretum_create_edge(obj, context_.data_model_.UsesFPIntrin, context_.data_model_.arboretum_node_for(D->UsesFPIntrin()));
  //isInlined
  arboretum_create_edge(obj, context_.data_model_.isInlined, context_.data_model_.arboretum_node_for(D->isInlined()));
  //isInlineDefinitionExternallyVisible
  arboretum_create_edge(obj, context_.data_model_.isInlineDefinitionExternallyVisible, context_.data_model_.arboretum_node_for(D->isInlineDefinitionExternallyVisible()));
  //isMSExternInline
  arboretum_create_edge(obj, context_.data_model_.isMSExternInline, context_.data_model_.arboretum_node_for(D->isMSExternInline()));
  //doesDeclarationForceExternallyVisibleDefinition
  arboretum_create_edge(obj, context_.data_model_.doesDeclarationForceExternallyVisibleDefinition, context_.data_model_.arboretum_node_for(D->doesDeclarationForceExternallyVisibleDefinition()));
  //isStatic
  arboretum_create_edge(obj, context_.data_model_.isStatic1, context_.data_model_.arboretum_node_for(D->isStatic()));
  //isOverloadedOperator
  arboretum_create_edge(obj, context_.data_model_.isOverloadedOperator, context_.data_model_.arboretum_node_for(D->isOverloadedOperator()));
  //getOverloadedOperator
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getOverloadedOperator());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getOverloadedOperator2, enum_value);
    }
  }
  //getLiteralIdentifier
  //getInstantiatedFromMemberFunction
  {
    const Entity* other = context_.resolve(D->getInstantiatedFromMemberFunction());
    arboretum_create_edge(obj, context_.data_model_.getInstantiatedFromMemberFunction, other);
  }
  //getTemplatedKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getTemplatedKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getTemplatedKind, enum_value);
    }
  }
  //getMemberSpecializationInfo
  //getInstantiatedFromDecl
  {
    const Entity* other = context_.resolve(D->getInstantiatedFromDecl());
    arboretum_create_edge(obj, context_.data_model_.getInstantiatedFromDecl, other);
  }
  //getDescribedFunctionTemplate
  {
    const Entity* other = context_.resolve(D->getDescribedFunctionTemplate());
    arboretum_create_edge(obj, context_.data_model_.getDescribedFunctionTemplate, other);
  }
  //isFunctionTemplateSpecialization
  arboretum_create_edge(obj, context_.data_model_.isFunctionTemplateSpecialization, context_.data_model_.arboretum_node_for(D->isFunctionTemplateSpecialization()));
  //getTemplateSpecializationInfo
  //isImplicitlyInstantiable
  arboretum_create_edge(obj, context_.data_model_.isImplicitlyInstantiable, context_.data_model_.arboretum_node_for(D->isImplicitlyInstantiable()));
  //isTemplateInstantiation
  arboretum_create_edge(obj, context_.data_model_.isTemplateInstantiation, context_.data_model_.arboretum_node_for(D->isTemplateInstantiation()));
  //getPrimaryTemplate
  {
    const Entity* other = context_.resolve(D->getPrimaryTemplate());
    arboretum_create_edge(obj, context_.data_model_.getPrimaryTemplate, other);
  }
  //getTemplateSpecializationArgs
  //getTemplateSpecializationArgsAsWritten
  //getDependentSpecializationInfo
  //getTemplateSpecializationKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getTemplateSpecializationKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getTemplateSpecializationKind2, enum_value);
    }
  }
  //getTemplateSpecializationKindForInstantiation
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getTemplateSpecializationKindForInstantiation());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getTemplateSpecializationKindForInstantiation1, enum_value);
    }
  }
  //getPointOfInstantiation
  {
    const Entity* other = context_.source_model_.resolve(D->getPointOfInstantiation());
    arboretum_create_edge(obj, context_.data_model_.getPointOfInstantiation2, other);
  }
  //isOutOfLine
  arboretum_create_edge(obj, context_.data_model_.isOutOfLine2, context_.data_model_.arboretum_node_for(D->isOutOfLine()));
  //getMemoryFunctionKind
  // unsigned int
  //getODRHash
  // unsigned int
  return true;
}

bool ArboretumASTVisitor::VisitFunctionTemplateDecl(clang::FunctionTemplateDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getTemplatedDecl
  {
    const Entity* other = context_.resolve(D->getTemplatedDecl());
    arboretum_create_edge(obj, context_.data_model_.getTemplatedDecl2, other);
  }
  //isThisDeclarationADefinition
  arboretum_create_edge(obj, context_.data_model_.isThisDeclarationADefinition4, context_.data_model_.arboretum_node_for(D->isThisDeclarationADefinition()));
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl19, other);
  }
  //getPreviousDecl
  {
    const Entity* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_edge(obj, context_.data_model_.getPreviousDecl5, other);
  }
  //getMostRecentDecl
  {
    const Entity* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_edge(obj, context_.data_model_.getMostRecentDecl8, other);
  }
  //getInstantiatedFromMemberTemplate
  {
    const Entity* other = context_.resolve(D->getInstantiatedFromMemberTemplate());
    arboretum_create_edge(obj, context_.data_model_.getInstantiatedFromMemberTemplate1, other);
  }
  //specializations
  // spec_range
  //isAbbreviated
  arboretum_create_edge(obj, context_.data_model_.isAbbreviated, context_.data_model_.arboretum_node_for(D->isAbbreviated()));
  return true;
}

bool ArboretumASTVisitor::VisitHLSLBufferDecl(clang::HLSLBufferDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange59, other);
  }
  //getLocStart
  {
    const Entity* other = context_.source_model_.resolve(D->getLocStart());
    arboretum_create_edge(obj, context_.data_model_.getLocStart, other);
  }
  //getLBraceLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getLBraceLoc());
    arboretum_create_edge(obj, context_.data_model_.getLBraceLoc2, other);
  }
  //getRBraceLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRBraceLoc());
    arboretum_create_edge(obj, context_.data_model_.getRBraceLoc5, other);
  }
  //isCBuffer
  arboretum_create_edge(obj, context_.data_model_.isCBuffer, context_.data_model_.arboretum_node_for(D->isCBuffer()));
  return true;
}

bool ArboretumASTVisitor::VisitImplicitConceptSpecializationDecl(clang::ImplicitConceptSpecializationDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getTemplateArguments
  // ArrayRef<TemplateArgument>
  return true;
}

bool ArboretumASTVisitor::VisitImplicitParamDecl(clang::ImplicitParamDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getParameterKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getParameterKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getParameterKind, enum_value);
    }
  }
  return true;
}

bool ArboretumASTVisitor::VisitImportDecl(clang::ImportDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getImportedModule
  //getIdentifierLocs
  // ArrayRef<SourceLocation>
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange33, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitIndirectFieldDecl(clang::IndirectFieldDecl* D) {
  const Entity* obj = context_.resolve(D);
  //chain
  // ArrayRef<NamedDecl *>
  //getChainingSize
  // unsigned int
  //getAnonField
  {
    const Entity* other = context_.resolve(D->getAnonField());
    arboretum_create_edge(obj, context_.data_model_.getAnonField, other);
  }
  //getVarDecl
  {
    const Entity* other = context_.resolve(D->getVarDecl());
    arboretum_create_edge(obj, context_.data_model_.getVarDecl, other);
  }
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl45, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitLabelDecl(clang::LabelDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getStmt
  {
    const Entity* other = context_.resolve(D->getStmt());
    arboretum_create_edge(obj, context_.data_model_.getStmt2, other);
  }
  //isGnuLocal
  arboretum_create_edge(obj, context_.data_model_.isGnuLocal, context_.data_model_.arboretum_node_for(D->isGnuLocal()));
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange61, other);
  }
  //isMSAsmLabel
  arboretum_create_edge(obj, context_.data_model_.isMSAsmLabel, context_.data_model_.arboretum_node_for(D->isMSAsmLabel()));
  //isResolvedMSAsmLabel
  arboretum_create_edge(obj, context_.data_model_.isResolvedMSAsmLabel, context_.data_model_.arboretum_node_for(D->isResolvedMSAsmLabel()));
  //getMSAsmLabel
  // StringRef
  return true;
}

bool ArboretumASTVisitor::VisitLifetimeExtendedTemporaryDecl(clang::LifetimeExtendedTemporaryDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getExtendingDecl
  {
    const Entity* other = context_.resolve(D->getExtendingDecl());
    arboretum_create_edge(obj, context_.data_model_.getExtendingDecl3, other);
  }
  //getStorageDuration
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getStorageDuration());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getStorageDuration1, enum_value);
    }
  }
  //getTemporaryExpr
  {
    const Entity* other = context_.resolve(D->getTemporaryExpr());
    arboretum_create_edge(obj, context_.data_model_.getTemporaryExpr1, other);
  }
  //getManglingNumber
  // unsigned int
  //getValue
  //childrenExpr
  // class Stmt::const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitLinkageSpecDecl(clang::LinkageSpecDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getLanguage
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getLanguage());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getLanguage, enum_value);
    }
  }
  //hasBraces
  arboretum_create_edge(obj, context_.data_model_.hasBraces2, context_.data_model_.arboretum_node_for(D->hasBraces()));
  //getExternLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getExternLoc());
    arboretum_create_edge(obj, context_.data_model_.getExternLoc2, other);
  }
  //getRBraceLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRBraceLoc());
    arboretum_create_edge(obj, context_.data_model_.getRBraceLoc3, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc118, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange39, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitMSGuidDecl(clang::MSGuidDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getParts
  // Parts
  //getAsAPValue
  // APValue &
  return true;
}

bool ArboretumASTVisitor::VisitMSPropertyDecl(clang::MSPropertyDecl* D) {
  const Entity* obj = context_.resolve(D);
  //hasGetter
  arboretum_create_edge(obj, context_.data_model_.hasGetter, context_.data_model_.arboretum_node_for(D->hasGetter()));
  //getGetterId
  //hasSetter
  arboretum_create_edge(obj, context_.data_model_.hasSetter, context_.data_model_.arboretum_node_for(D->hasSetter()));
  //getSetterId
  return true;
}

bool ArboretumASTVisitor::VisitNamedDecl(clang::NamedDecl* D) {
  const Entity* obj = context_.resolve(D);
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
  arboretum_create_edge(obj, context_.data_model_.hasLinkage, context_.data_model_.arboretum_node_for(D->hasLinkage()));
  //isCXXClassMember
  arboretum_create_edge(obj, context_.data_model_.isCXXClassMember, context_.data_model_.arboretum_node_for(D->isCXXClassMember()));
  //isCXXInstanceMember
  arboretum_create_edge(obj, context_.data_model_.isCXXInstanceMember, context_.data_model_.arboretum_node_for(D->isCXXInstanceMember()));
  //getLinkageInternal
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getLinkageInternal());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getLinkageInternal, enum_value);
    }
  }
  //getFormalLinkage
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getFormalLinkage());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getFormalLinkage, enum_value);
    }
  }
  //hasExternalFormalLinkage
  arboretum_create_edge(obj, context_.data_model_.hasExternalFormalLinkage, context_.data_model_.arboretum_node_for(D->hasExternalFormalLinkage()));
  //isExternallyVisible
  arboretum_create_edge(obj, context_.data_model_.isExternallyVisible, context_.data_model_.arboretum_node_for(D->isExternallyVisible()));
  //isExternallyDeclarable
  arboretum_create_edge(obj, context_.data_model_.isExternallyDeclarable, context_.data_model_.arboretum_node_for(D->isExternallyDeclarable()));
  //isLinkageValid
  arboretum_create_edge(obj, context_.data_model_.isLinkageValid, context_.data_model_.arboretum_node_for(D->isLinkageValid()));
  //hasLinkageBeenComputed
  arboretum_create_edge(obj, context_.data_model_.hasLinkageBeenComputed, context_.data_model_.arboretum_node_for(D->hasLinkageBeenComputed()));
  //getUnderlyingDecl
  {
    const Entity* other = context_.resolve(D->getUnderlyingDecl());
    arboretum_create_edge(obj, context_.data_model_.getUnderlyingDecl1, other);
  }
  //getMostRecentDecl
  {
    const Entity* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_edge(obj, context_.data_model_.getMostRecentDecl14, other);
  }
  //getObjCFStringFormattingFamily
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getObjCFStringFormattingFamily());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getObjCFStringFormattingFamily, enum_value);
    }
  }
  return true;
}

bool ArboretumASTVisitor::VisitNamespaceAliasDecl(clang::NamespaceAliasDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl47, other);
  }
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getQualifier
  //getNamespace
  {
    const Entity* other = context_.resolve(D->getNamespace());
    arboretum_create_edge(obj, context_.data_model_.getNamespace1, other);
  }
  //getAliasLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getAliasLoc());
    arboretum_create_edge(obj, context_.data_model_.getAliasLoc, other);
  }
  //getNamespaceLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getNamespaceLoc());
    arboretum_create_edge(obj, context_.data_model_.getNamespaceLoc, other);
  }
  //getTargetNameLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getTargetNameLoc());
    arboretum_create_edge(obj, context_.data_model_.getTargetNameLoc, other);
  }
  //getAliasedNamespace
  {
    const Entity* other = context_.resolve(D->getAliasedNamespace());
    arboretum_create_edge(obj, context_.data_model_.getAliasedNamespace, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange38, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitNamespaceDecl(clang::NamespaceDecl* D) {
  const Entity* obj = context_.resolve(D);
  //isAnonymousNamespace
  arboretum_create_edge(obj, context_.data_model_.isAnonymousNamespace, context_.data_model_.arboretum_node_for(D->isAnonymousNamespace()));
  //isInline
  arboretum_create_edge(obj, context_.data_model_.isInline1, context_.data_model_.arboretum_node_for(D->isInline()));
  //isNested
  arboretum_create_edge(obj, context_.data_model_.isNested, context_.data_model_.arboretum_node_for(D->isNested()));
  //getOriginalNamespace
  {
    const Entity* other = context_.resolve(D->getOriginalNamespace());
    arboretum_create_edge(obj, context_.data_model_.getOriginalNamespace1, other);
  }
  //isOriginalNamespace
  arboretum_create_edge(obj, context_.data_model_.isOriginalNamespace, context_.data_model_.arboretum_node_for(D->isOriginalNamespace()));
  //getAnonymousNamespace
  {
    const Entity* other = context_.resolve(D->getAnonymousNamespace());
    arboretum_create_edge(obj, context_.data_model_.getAnonymousNamespace1, other);
  }
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl55, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange44, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc127, other);
  }
  //getRBraceLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRBraceLoc());
    arboretum_create_edge(obj, context_.data_model_.getRBraceLoc4, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitNonTypeTemplateParmDecl(clang::NonTypeTemplateParmDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange29, other);
  }
  //getDefaultArgStorage
  // const DefArgStorage &
  //hasDefaultArgument
  arboretum_create_edge(obj, context_.data_model_.hasDefaultArgument2, context_.data_model_.arboretum_node_for(D->hasDefaultArgument()));
  //getDefaultArgument
  {
    const Entity* other = context_.resolve(D->getDefaultArgument());
    arboretum_create_edge(obj, context_.data_model_.getDefaultArgument2, other);
  }
  //getDefaultArgumentLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getDefaultArgumentLoc());
    arboretum_create_edge(obj, context_.data_model_.getDefaultArgumentLoc2, other);
  }
  //defaultArgumentWasInherited
  arboretum_create_edge(obj, context_.data_model_.defaultArgumentWasInherited2, context_.data_model_.arboretum_node_for(D->defaultArgumentWasInherited()));
  //isParameterPack
  arboretum_create_edge(obj, context_.data_model_.isParameterPack4, context_.data_model_.arboretum_node_for(D->isParameterPack()));
  //isPackExpansion
  arboretum_create_edge(obj, context_.data_model_.isPackExpansion3, context_.data_model_.arboretum_node_for(D->isPackExpansion()));
  //isExpandedParameterPack
  arboretum_create_edge(obj, context_.data_model_.isExpandedParameterPack2, context_.data_model_.arboretum_node_for(D->isExpandedParameterPack()));
  //getNumExpansionTypes
  // unsigned int
  //getPlaceholderTypeConstraint
  {
    const Entity* other = context_.resolve(D->getPlaceholderTypeConstraint());
    arboretum_create_edge(obj, context_.data_model_.getPlaceholderTypeConstraint, other);
  }
  //hasPlaceholderTypeConstraint
  arboretum_create_edge(obj, context_.data_model_.hasPlaceholderTypeConstraint, context_.data_model_.arboretum_node_for(D->hasPlaceholderTypeConstraint()));
  return true;
}

bool ArboretumASTVisitor::VisitOMPAllocateDecl(clang::OMPAllocateDecl* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPCapturedExprDecl(clang::OMPCapturedExprDecl* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPDeclareMapperDecl(clang::OMPDeclareMapperDecl* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPDeclareReductionDecl(clang::OMPDeclareReductionDecl* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPRequiresDecl(clang::OMPRequiresDecl* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPThreadPrivateDecl(clang::OMPThreadPrivateDecl* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCAtDefsFieldDecl(clang::ObjCAtDefsFieldDecl* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCCategoryDecl(clang::ObjCCategoryDecl* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCCategoryImplDecl(clang::ObjCCategoryImplDecl* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCCompatibleAliasDecl(clang::ObjCCompatibleAliasDecl* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCContainerDecl(clang::ObjCContainerDecl* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCImplDecl(clang::ObjCImplDecl* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCImplementationDecl(clang::ObjCImplementationDecl* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCInterfaceDecl(clang::ObjCInterfaceDecl* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCIvarDecl(clang::ObjCIvarDecl* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCMethodDecl(clang::ObjCMethodDecl* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCPropertyDecl(clang::ObjCPropertyDecl* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCPropertyImplDecl(clang::ObjCPropertyImplDecl* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCProtocolDecl(clang::ObjCProtocolDecl* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCTypeParamDecl(clang::ObjCTypeParamDecl* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitParmVarDecl(clang::ParmVarDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange62, other);
  }
  //isObjCMethodParameter
  arboretum_create_edge(obj, context_.data_model_.isObjCMethodParameter, context_.data_model_.arboretum_node_for(D->isObjCMethodParameter()));
  //isDestroyedInCallee
  arboretum_create_edge(obj, context_.data_model_.isDestroyedInCallee, context_.data_model_.arboretum_node_for(D->isDestroyedInCallee()));
  //getFunctionScopeDepth
  // unsigned int
  //getFunctionScopeIndex
  // unsigned int
  //getObjCDeclQualifier
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getObjCDeclQualifier());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getObjCDeclQualifier1, enum_value);
    }
  }
  //isKNRPromoted
  arboretum_create_edge(obj, context_.data_model_.isKNRPromoted, context_.data_model_.arboretum_node_for(D->isKNRPromoted()));
  //isExplicitObjectParameter
  arboretum_create_edge(obj, context_.data_model_.isExplicitObjectParameter, context_.data_model_.arboretum_node_for(D->isExplicitObjectParameter()));
  //getExplicitObjectParamThisLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getExplicitObjectParamThisLoc());
    arboretum_create_edge(obj, context_.data_model_.getExplicitObjectParamThisLoc, other);
  }
  //getDefaultArg
  {
    const Entity* other = context_.resolve(D->getDefaultArg());
    arboretum_create_edge(obj, context_.data_model_.getDefaultArg1, other);
  }
  //getDefaultArgRange
  {
    const Entity* other = context_.source_model_.resolve(D->getDefaultArgRange());
    arboretum_create_edge(obj, context_.data_model_.getDefaultArgRange, other);
  }
  //getUninstantiatedDefaultArg
  {
    const Entity* other = context_.resolve(D->getUninstantiatedDefaultArg());
    arboretum_create_edge(obj, context_.data_model_.getUninstantiatedDefaultArg1, other);
  }
  //hasDefaultArg
  arboretum_create_edge(obj, context_.data_model_.hasDefaultArg, context_.data_model_.arboretum_node_for(D->hasDefaultArg()));
  //hasUnparsedDefaultArg
  arboretum_create_edge(obj, context_.data_model_.hasUnparsedDefaultArg, context_.data_model_.arboretum_node_for(D->hasUnparsedDefaultArg()));
  //hasUninstantiatedDefaultArg
  arboretum_create_edge(obj, context_.data_model_.hasUninstantiatedDefaultArg, context_.data_model_.arboretum_node_for(D->hasUninstantiatedDefaultArg()));
  //hasInheritedDefaultArg
  arboretum_create_edge(obj, context_.data_model_.hasInheritedDefaultArg, context_.data_model_.arboretum_node_for(D->hasInheritedDefaultArg()));
  //getOriginalType
  {
    const Entity* other = context_.resolve(D->getOriginalType());
    arboretum_create_edge(obj, context_.data_model_.getOriginalType1, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitPragmaCommentDecl(clang::PragmaCommentDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getCommentKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getCommentKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getCommentKind, enum_value);
    }
  }
  //getArg
  // StringRef
  return true;
}

bool ArboretumASTVisitor::VisitPragmaDetectMismatchDecl(clang::PragmaDetectMismatchDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getName
  // StringRef
  //getValue
  // StringRef
  return true;
}

bool ArboretumASTVisitor::VisitRecordDecl(clang::RecordDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Entity* obj = context_.resolve(D);
  //getPreviousDecl
  {
    const Entity* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_edge(obj, context_.data_model_.getPreviousDecl13, other);
  }
  //getMostRecentDecl
  {
    const Entity* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_edge(obj, context_.data_model_.getMostRecentDecl16, other);
  }
  //hasFlexibleArrayMember
  arboretum_create_edge(obj, context_.data_model_.hasFlexibleArrayMember, context_.data_model_.arboretum_node_for(D->hasFlexibleArrayMember()));
  //isAnonymousStructOrUnion
  arboretum_create_edge(obj, context_.data_model_.isAnonymousStructOrUnion, context_.data_model_.arboretum_node_for(D->isAnonymousStructOrUnion()));
  //hasObjectMember
  arboretum_create_edge(obj, context_.data_model_.hasObjectMember, context_.data_model_.arboretum_node_for(D->hasObjectMember()));
  //hasVolatileMember
  arboretum_create_edge(obj, context_.data_model_.hasVolatileMember, context_.data_model_.arboretum_node_for(D->hasVolatileMember()));
  //hasLoadedFieldsFromExternalStorage
  arboretum_create_edge(obj, context_.data_model_.hasLoadedFieldsFromExternalStorage, context_.data_model_.arboretum_node_for(D->hasLoadedFieldsFromExternalStorage()));
  //isNonTrivialToPrimitiveDefaultInitialize
  arboretum_create_edge(obj, context_.data_model_.isNonTrivialToPrimitiveDefaultInitialize, context_.data_model_.arboretum_node_for(D->isNonTrivialToPrimitiveDefaultInitialize()));
  //isNonTrivialToPrimitiveCopy
  arboretum_create_edge(obj, context_.data_model_.isNonTrivialToPrimitiveCopy, context_.data_model_.arboretum_node_for(D->isNonTrivialToPrimitiveCopy()));
  //isNonTrivialToPrimitiveDestroy
  arboretum_create_edge(obj, context_.data_model_.isNonTrivialToPrimitiveDestroy, context_.data_model_.arboretum_node_for(D->isNonTrivialToPrimitiveDestroy()));
  //hasNonTrivialToPrimitiveDefaultInitializeCUnion
  arboretum_create_edge(obj, context_.data_model_.hasNonTrivialToPrimitiveDefaultInitializeCUnion, context_.data_model_.arboretum_node_for(D->hasNonTrivialToPrimitiveDefaultInitializeCUnion()));
  //hasNonTrivialToPrimitiveDestructCUnion
  arboretum_create_edge(obj, context_.data_model_.hasNonTrivialToPrimitiveDestructCUnion, context_.data_model_.arboretum_node_for(D->hasNonTrivialToPrimitiveDestructCUnion()));
  //hasNonTrivialToPrimitiveCopyCUnion
  arboretum_create_edge(obj, context_.data_model_.hasNonTrivialToPrimitiveCopyCUnion, context_.data_model_.arboretum_node_for(D->hasNonTrivialToPrimitiveCopyCUnion()));
  //canPassInRegisters
  arboretum_create_edge(obj, context_.data_model_.canPassInRegisters, context_.data_model_.arboretum_node_for(D->canPassInRegisters()));
  //getArgPassingRestrictions
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getArgPassingRestrictions());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getArgPassingRestrictions, enum_value);
    }
  }
  //isParamDestroyedInCallee
  arboretum_create_edge(obj, context_.data_model_.isParamDestroyedInCallee, context_.data_model_.arboretum_node_for(D->isParamDestroyedInCallee()));
  //isRandomized
  arboretum_create_edge(obj, context_.data_model_.isRandomized, context_.data_model_.arboretum_node_for(D->isRandomized()));
  //isInjectedClassName
  arboretum_create_edge(obj, context_.data_model_.isInjectedClassName, context_.data_model_.arboretum_node_for(D->isInjectedClassName()));
  //isLambda
  arboretum_create_edge(obj, context_.data_model_.isLambda1, context_.data_model_.arboretum_node_for(D->isLambda()));
  //isCapturedRecord
  arboretum_create_edge(obj, context_.data_model_.isCapturedRecord, context_.data_model_.arboretum_node_for(D->isCapturedRecord()));
  //getDefinition
  {
    const Entity* other = context_.resolve(D->getDefinition());
    arboretum_create_edge(obj, context_.data_model_.getDefinition13, other);
  }
  //isOrContainsUnion
  arboretum_create_edge(obj, context_.data_model_.isOrContainsUnion, context_.data_model_.arboretum_node_for(D->isOrContainsUnion()));
  //fields
  // field_range
  //field_empty
  arboretum_create_edge(obj, context_.data_model_.field_empty, context_.data_model_.arboretum_node_for(D->field_empty()));
  //findFirstNamedDataMember
  {
    const Entity* other = context_.resolve(D->findFirstNamedDataMember());
    arboretum_create_edge(obj, context_.data_model_.findFirstNamedDataMember, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitRedeclarableTemplateDecl(clang::RedeclarableTemplateDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl59, other);
  }
  //isMemberSpecialization
  arboretum_create_edge(obj, context_.data_model_.isMemberSpecialization2, context_.data_model_.arboretum_node_for(D->isMemberSpecialization()));
  //getInstantiatedFromMemberTemplate
  {
    const Entity* other = context_.resolve(D->getInstantiatedFromMemberTemplate());
    arboretum_create_edge(obj, context_.data_model_.getInstantiatedFromMemberTemplate5, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitRequiresExprBodyDecl(clang::RequiresExprBodyDecl* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitStaticAssertDecl(clang::StaticAssertDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getAssertExpr
  {
    const Entity* other = context_.resolve(D->getAssertExpr());
    arboretum_create_edge(obj, context_.data_model_.getAssertExpr1, other);
  }
  //getMessage
  {
    const Entity* other = context_.resolve(D->getMessage());
    arboretum_create_edge(obj, context_.data_model_.getMessage1, other);
  }
  //isFailed
  arboretum_create_edge(obj, context_.data_model_.isFailed, context_.data_model_.arboretum_node_for(D->isFailed()));
  //getRParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc21, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange15, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitTagDecl(clang::TagDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Entity* obj = context_.resolve(D);
  //getBraceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getBraceRange());
    arboretum_create_edge(obj, context_.data_model_.getBraceRange, other);
  }
  //getInnerLocStart
  {
    const Entity* other = context_.source_model_.resolve(D->getInnerLocStart());
    arboretum_create_edge(obj, context_.data_model_.getInnerLocStart, other);
  }
  //getOuterLocStart
  {
    const Entity* other = context_.source_model_.resolve(D->getOuterLocStart());
    arboretum_create_edge(obj, context_.data_model_.getOuterLocStart, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange8, other);
  }
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl3, other);
  }
  //isThisDeclarationADefinition
  arboretum_create_edge(obj, context_.data_model_.isThisDeclarationADefinition, context_.data_model_.arboretum_node_for(D->isThisDeclarationADefinition()));
  //isCompleteDefinition
  arboretum_create_edge(obj, context_.data_model_.isCompleteDefinition, context_.data_model_.arboretum_node_for(D->isCompleteDefinition()));
  //isCompleteDefinitionRequired
  arboretum_create_edge(obj, context_.data_model_.isCompleteDefinitionRequired, context_.data_model_.arboretum_node_for(D->isCompleteDefinitionRequired()));
  //isBeingDefined
  arboretum_create_edge(obj, context_.data_model_.isBeingDefined, context_.data_model_.arboretum_node_for(D->isBeingDefined()));
  //isEmbeddedInDeclarator
  arboretum_create_edge(obj, context_.data_model_.isEmbeddedInDeclarator, context_.data_model_.arboretum_node_for(D->isEmbeddedInDeclarator()));
  //isFreeStanding
  arboretum_create_edge(obj, context_.data_model_.isFreeStanding, context_.data_model_.arboretum_node_for(D->isFreeStanding()));
  //mayHaveOutOfDateDef
  arboretum_create_edge(obj, context_.data_model_.mayHaveOutOfDateDef, context_.data_model_.arboretum_node_for(D->mayHaveOutOfDateDef()));
  //isDependentType
  arboretum_create_edge(obj, context_.data_model_.isDependentType, context_.data_model_.arboretum_node_for(D->isDependentType()));
  //isThisDeclarationADemotedDefinition
  arboretum_create_edge(obj, context_.data_model_.isThisDeclarationADemotedDefinition, context_.data_model_.arboretum_node_for(D->isThisDeclarationADemotedDefinition()));
  //getDefinition
  {
    const Entity* other = context_.resolve(D->getDefinition());
    arboretum_create_edge(obj, context_.data_model_.getDefinition1, other);
  }
  //getKindName
  // StringRef
  //getTagKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getTagKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getTagKind, enum_value);
    }
  }
  //isStruct
  arboretum_create_edge(obj, context_.data_model_.isStruct, context_.data_model_.arboretum_node_for(D->isStruct()));
  //isInterface
  arboretum_create_edge(obj, context_.data_model_.isInterface, context_.data_model_.arboretum_node_for(D->isInterface()));
  //isClass
  arboretum_create_edge(obj, context_.data_model_.isClass, context_.data_model_.arboretum_node_for(D->isClass()));
  //isUnion
  arboretum_create_edge(obj, context_.data_model_.isUnion, context_.data_model_.arboretum_node_for(D->isUnion()));
  //isEnum
  arboretum_create_edge(obj, context_.data_model_.isEnum, context_.data_model_.arboretum_node_for(D->isEnum()));
  //hasNameForLinkage
  arboretum_create_edge(obj, context_.data_model_.hasNameForLinkage, context_.data_model_.arboretum_node_for(D->hasNameForLinkage()));
  //getTypedefNameForAnonDecl
  {
    const Entity* other = context_.resolve(D->getTypedefNameForAnonDecl());
    arboretum_create_edge(obj, context_.data_model_.getTypedefNameForAnonDecl, other);
  }
  //getQualifier
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getNumTemplateParameterLists
  // unsigned int
  return true;
}

bool ArboretumASTVisitor::VisitTemplateDecl(clang::TemplateDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getTemplateParameters
  //hasAssociatedConstraints
  arboretum_create_edge(obj, context_.data_model_.hasAssociatedConstraints, context_.data_model_.arboretum_node_for(D->hasAssociatedConstraints()));
  //getTemplatedDecl
  {
    const Entity* other = context_.resolve(D->getTemplatedDecl());
    arboretum_create_edge(obj, context_.data_model_.getTemplatedDecl1, other);
  }
  //isTypeAlias
  arboretum_create_edge(obj, context_.data_model_.isTypeAlias, context_.data_model_.arboretum_node_for(D->isTypeAlias()));
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange12, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitTemplateParamObjectDecl(clang::TemplateParamObjectDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getValue
  // const APValue &
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl5, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitTemplateTemplateParmDecl(clang::TemplateTemplateParmDecl* D) {
  const Entity* obj = context_.resolve(D);
  //isParameterPack
  arboretum_create_edge(obj, context_.data_model_.isParameterPack, context_.data_model_.arboretum_node_for(D->isParameterPack()));
  //isPackExpansion
  arboretum_create_edge(obj, context_.data_model_.isPackExpansion, context_.data_model_.arboretum_node_for(D->isPackExpansion()));
  //isExpandedParameterPack
  arboretum_create_edge(obj, context_.data_model_.isExpandedParameterPack, context_.data_model_.arboretum_node_for(D->isExpandedParameterPack()));
  //getNumExpansionTemplateParameters
  // unsigned int
  //getDefaultArgStorage
  // const DefArgStorage &
  //hasDefaultArgument
  arboretum_create_edge(obj, context_.data_model_.hasDefaultArgument, context_.data_model_.arboretum_node_for(D->hasDefaultArgument()));
  //getDefaultArgument
  // const TemplateArgumentLoc &
  //getDefaultArgumentLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getDefaultArgumentLoc());
    arboretum_create_edge(obj, context_.data_model_.getDefaultArgumentLoc, other);
  }
  //defaultArgumentWasInherited
  arboretum_create_edge(obj, context_.data_model_.defaultArgumentWasInherited, context_.data_model_.arboretum_node_for(D->defaultArgumentWasInherited()));
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange10, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitTemplateTypeParmDecl(clang::TemplateTypeParmDecl* D) {
  const Entity* obj = context_.resolve(D);
  //wasDeclaredWithTypename
  arboretum_create_edge(obj, context_.data_model_.wasDeclaredWithTypename, context_.data_model_.arboretum_node_for(D->wasDeclaredWithTypename()));
  //getDefaultArgStorage
  // const DefArgStorage &
  //hasDefaultArgument
  arboretum_create_edge(obj, context_.data_model_.hasDefaultArgument1, context_.data_model_.arboretum_node_for(D->hasDefaultArgument()));
  //getDefaultArgumentInfo
  //getDefaultArgumentLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getDefaultArgumentLoc());
    arboretum_create_edge(obj, context_.data_model_.getDefaultArgumentLoc1, other);
  }
  //defaultArgumentWasInherited
  arboretum_create_edge(obj, context_.data_model_.defaultArgumentWasInherited1, context_.data_model_.arboretum_node_for(D->defaultArgumentWasInherited()));
  //getDepth
  // unsigned int
  //getIndex
  // unsigned int
  //isParameterPack
  arboretum_create_edge(obj, context_.data_model_.isParameterPack1, context_.data_model_.arboretum_node_for(D->isParameterPack()));
  //isPackExpansion
  arboretum_create_edge(obj, context_.data_model_.isPackExpansion1, context_.data_model_.arboretum_node_for(D->isPackExpansion()));
  //isExpandedParameterPack
  arboretum_create_edge(obj, context_.data_model_.isExpandedParameterPack1, context_.data_model_.arboretum_node_for(D->isExpandedParameterPack()));
  //getNumExpansionParameters
  // unsigned int
  //getTypeConstraint
  //hasTypeConstraint
  arboretum_create_edge(obj, context_.data_model_.hasTypeConstraint, context_.data_model_.arboretum_node_for(D->hasTypeConstraint()));
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange18, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitTopLevelStmtDecl(clang::TopLevelStmtDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange24, other);
  }
  //getStmt
  {
    const Entity* other = context_.resolve(D->getStmt());
    arboretum_create_edge(obj, context_.data_model_.getStmt1, other);
  }
  //isSemiMissing
  arboretum_create_edge(obj, context_.data_model_.isSemiMissing, context_.data_model_.arboretum_node_for(D->isSemiMissing()));
  return true;
}

bool ArboretumASTVisitor::VisitTranslationUnitDecl(clang::TranslationUnitDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getAnonymousNamespace
  {
    const Entity* other = context_.resolve(D->getAnonymousNamespace());
    arboretum_create_edge(obj, context_.data_model_.getAnonymousNamespace, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitTypeAliasDecl(clang::TypeAliasDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange31, other);
  }
  //getDescribedAliasTemplate
  {
    const Entity* other = context_.resolve(D->getDescribedAliasTemplate());
    arboretum_create_edge(obj, context_.data_model_.getDescribedAliasTemplate, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitTypeAliasTemplateDecl(clang::TypeAliasTemplateDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getTemplatedDecl
  {
    const Entity* other = context_.resolve(D->getTemplatedDecl());
    arboretum_create_edge(obj, context_.data_model_.getTemplatedDecl4, other);
  }
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl29, other);
  }
  //getPreviousDecl
  {
    const Entity* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_edge(obj, context_.data_model_.getPreviousDecl11, other);
  }
  //getInstantiatedFromMemberTemplate
  {
    const Entity* other = context_.resolve(D->getInstantiatedFromMemberTemplate());
    arboretum_create_edge(obj, context_.data_model_.getInstantiatedFromMemberTemplate3, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitTypeDecl(clang::TypeDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getTypeForDecl
  {
    const Entity* other = context_.resolve(D->getTypeForDecl());
    arboretum_create_edge(obj, context_.data_model_.getTypeForDecl1, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc143, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange50, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitTypedefDecl(clang::TypedefDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange42, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitTypedefNameDecl(clang::TypedefNameDecl* D) {
  const Entity* obj = context_.resolve(D);
  //isModed
  arboretum_create_edge(obj, context_.data_model_.isModed, context_.data_model_.arboretum_node_for(D->isModed()));
  //getTypeSourceInfo
  //getUnderlyingType
  {
    const Entity* other = context_.resolve(D->getUnderlyingType());
    arboretum_create_edge(obj, context_.data_model_.getUnderlyingType, other);
  }
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl33, other);
  }
  //isTransparentTag
  arboretum_create_edge(obj, context_.data_model_.isTransparentTag, context_.data_model_.arboretum_node_for(D->isTransparentTag()));
  return true;
}

bool ArboretumASTVisitor::VisitUnnamedGlobalConstantDecl(clang::UnnamedGlobalConstantDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getValue
  // const APValue &
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedUsingIfExistsDecl(clang::UnresolvedUsingIfExistsDecl* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedUsingTypenameDecl(clang::UnresolvedUsingTypenameDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getUsingLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getUsingLoc());
    arboretum_create_edge(obj, context_.data_model_.getUsingLoc3, other);
  }
  //getTypenameLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getTypenameLoc());
    arboretum_create_edge(obj, context_.data_model_.getTypenameLoc, other);
  }
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getQualifier
  //getNameInfo
  // DeclarationNameInfo
  //isPackExpansion
  arboretum_create_edge(obj, context_.data_model_.isPackExpansion5, context_.data_model_.arboretum_node_for(D->isPackExpansion()));
  //getEllipsisLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEllipsisLoc());
    arboretum_create_edge(obj, context_.data_model_.getEllipsisLoc6, other);
  }
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl53, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedUsingValueDecl(clang::UnresolvedUsingValueDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getUsingLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getUsingLoc());
    arboretum_create_edge(obj, context_.data_model_.getUsingLoc2, other);
  }
  //isAccessDeclaration
  arboretum_create_edge(obj, context_.data_model_.isAccessDeclaration, context_.data_model_.arboretum_node_for(D->isAccessDeclaration()));
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getQualifier
  //getNameInfo
  // DeclarationNameInfo
  //isPackExpansion
  arboretum_create_edge(obj, context_.data_model_.isPackExpansion2, context_.data_model_.arboretum_node_for(D->isPackExpansion()));
  //getEllipsisLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEllipsisLoc());
    arboretum_create_edge(obj, context_.data_model_.getEllipsisLoc4, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange28, other);
  }
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl35, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitUsingDecl(clang::UsingDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getUsingLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getUsingLoc());
    arboretum_create_edge(obj, context_.data_model_.getUsingLoc4, other);
  }
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getQualifier
  //getNameInfo
  // DeclarationNameInfo
  //isAccessDeclaration
  arboretum_create_edge(obj, context_.data_model_.isAccessDeclaration1, context_.data_model_.arboretum_node_for(D->isAccessDeclaration()));
  //hasTypename
  arboretum_create_edge(obj, context_.data_model_.hasTypename, context_.data_model_.arboretum_node_for(D->hasTypename()));
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange49, other);
  }
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl57, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitUsingDirectiveDecl(clang::UsingDirectiveDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getQualifier
  //getNominatedNamespaceAsWritten
  {
    const Entity* other = context_.resolve(D->getNominatedNamespaceAsWritten());
    arboretum_create_edge(obj, context_.data_model_.getNominatedNamespaceAsWritten1, other);
  }
  //getNominatedNamespace
  {
    const Entity* other = context_.resolve(D->getNominatedNamespace());
    arboretum_create_edge(obj, context_.data_model_.getNominatedNamespace1, other);
  }
  //getCommonAncestor
  //getUsingLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getUsingLoc());
    arboretum_create_edge(obj, context_.data_model_.getUsingLoc1, other);
  }
  //getNamespaceKeyLocation
  {
    const Entity* other = context_.source_model_.resolve(D->getNamespaceKeyLocation());
    arboretum_create_edge(obj, context_.data_model_.getNamespaceKeyLocation, other);
  }
  //getIdentLocation
  {
    const Entity* other = context_.source_model_.resolve(D->getIdentLocation());
    arboretum_create_edge(obj, context_.data_model_.getIdentLocation, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange16, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitUsingEnumDecl(clang::UsingEnumDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getUsingLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getUsingLoc());
    arboretum_create_edge(obj, context_.data_model_.getUsingLoc, other);
  }
  //getEnumLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEnumLoc());
    arboretum_create_edge(obj, context_.data_model_.getEnumLoc, other);
  }
  //getQualifier
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getEnumTypeLoc
  // TypeLoc
  //getEnumType
  //getEnumDecl
  {
    const Entity* other = context_.resolve(D->getEnumDecl());
    arboretum_create_edge(obj, context_.data_model_.getEnumDecl, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange9, other);
  }
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl9, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitUsingPackDecl(clang::UsingPackDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getInstantiatedFromUsingDecl
  {
    const Entity* other = context_.resolve(D->getInstantiatedFromUsingDecl());
    arboretum_create_edge(obj, context_.data_model_.getInstantiatedFromUsingDecl, other);
  }
  //expansions
  // ArrayRef<NamedDecl *>
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange36, other);
  }
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl43, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitUsingShadowDecl(clang::UsingShadowDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl7, other);
  }
  //getTargetDecl
  {
    const Entity* other = context_.resolve(D->getTargetDecl());
    arboretum_create_edge(obj, context_.data_model_.getTargetDecl, other);
  }
  //getIntroducer
  {
    const Entity* other = context_.resolve(D->getIntroducer());
    arboretum_create_edge(obj, context_.data_model_.getIntroducer, other);
  }
  //getNextUsingShadowDecl
  {
    const Entity* other = context_.resolve(D->getNextUsingShadowDecl());
    arboretum_create_edge(obj, context_.data_model_.getNextUsingShadowDecl, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitValueDecl(clang::ValueDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getType
  {
    const Entity* other = context_.resolve(D->getType());
    arboretum_create_edge(obj, context_.data_model_.getType1, other);
  }
  //isWeak
  arboretum_create_edge(obj, context_.data_model_.isWeak, context_.data_model_.arboretum_node_for(D->isWeak()));
  //isInitCapture
  arboretum_create_edge(obj, context_.data_model_.isInitCapture2, context_.data_model_.arboretum_node_for(D->isInitCapture()));
  //getPotentiallyDecomposedVarDecl
  {
    const Entity* other = context_.resolve(D->getPotentiallyDecomposedVarDecl());
    arboretum_create_edge(obj, context_.data_model_.getPotentiallyDecomposedVarDecl1, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitVarDecl(clang::VarDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange22, other);
  }
  //getStorageClass
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getStorageClass());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getStorageClass, enum_value);
    }
  }
  //getTSCSpec
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getTSCSpec());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getTSCSpec, enum_value);
    }
  }
  //getTLSKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getTLSKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getTLSKind, enum_value);
    }
  }
  //hasLocalStorage
  arboretum_create_edge(obj, context_.data_model_.hasLocalStorage, context_.data_model_.arboretum_node_for(D->hasLocalStorage()));
  //isStaticLocal
  arboretum_create_edge(obj, context_.data_model_.isStaticLocal, context_.data_model_.arboretum_node_for(D->isStaticLocal()));
  //hasExternalStorage
  arboretum_create_edge(obj, context_.data_model_.hasExternalStorage, context_.data_model_.arboretum_node_for(D->hasExternalStorage()));
  //hasGlobalStorage
  arboretum_create_edge(obj, context_.data_model_.hasGlobalStorage, context_.data_model_.arboretum_node_for(D->hasGlobalStorage()));
  //getStorageDuration
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getStorageDuration());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getStorageDuration2, enum_value);
    }
  }
  //getLanguageLinkage
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getLanguageLinkage());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getLanguageLinkage, enum_value);
    }
  }
  //isExternC
  arboretum_create_edge(obj, context_.data_model_.isExternC, context_.data_model_.arboretum_node_for(D->isExternC()));
  //isInExternCContext
  arboretum_create_edge(obj, context_.data_model_.isInExternCContext, context_.data_model_.arboretum_node_for(D->isInExternCContext()));
  //isInExternCXXContext
  arboretum_create_edge(obj, context_.data_model_.isInExternCXXContext, context_.data_model_.arboretum_node_for(D->isInExternCXXContext()));
  //isLocalVarDecl
  arboretum_create_edge(obj, context_.data_model_.isLocalVarDecl, context_.data_model_.arboretum_node_for(D->isLocalVarDecl()));
  //isLocalVarDeclOrParm
  arboretum_create_edge(obj, context_.data_model_.isLocalVarDeclOrParm, context_.data_model_.arboretum_node_for(D->isLocalVarDeclOrParm()));
  //isFunctionOrMethodVarDecl
  arboretum_create_edge(obj, context_.data_model_.isFunctionOrMethodVarDecl, context_.data_model_.arboretum_node_for(D->isFunctionOrMethodVarDecl()));
  //isStaticDataMember
  arboretum_create_edge(obj, context_.data_model_.isStaticDataMember, context_.data_model_.arboretum_node_for(D->isStaticDataMember()));
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl21, other);
  }
  //isThisDeclarationADefinition
  {
    const Entity* enum_value = context_.data_model_.resolve(D->isThisDeclarationADefinition());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.isThisDeclarationADefinition6, enum_value);
    }
  }
  //hasDefinition
  {
    const Entity* enum_value = context_.data_model_.resolve(D->hasDefinition());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.hasDefinition4, enum_value);
    }
  }
  //getActingDefinition
  {
    const Entity* other = context_.resolve(D->getActingDefinition());
    arboretum_create_edge(obj, context_.data_model_.getActingDefinition1, other);
  }
  //getDefinition
  {
    const Entity* other = context_.resolve(D->getDefinition());
    arboretum_create_edge(obj, context_.data_model_.getDefinition9, other);
  }
  //isOutOfLine
  arboretum_create_edge(obj, context_.data_model_.isOutOfLine, context_.data_model_.arboretum_node_for(D->isOutOfLine()));
  //isFileVarDecl
  arboretum_create_edge(obj, context_.data_model_.isFileVarDecl, context_.data_model_.arboretum_node_for(D->isFileVarDecl()));
  //getAnyInitializer
  {
    const Entity* other = context_.resolve(D->getAnyInitializer());
    arboretum_create_edge(obj, context_.data_model_.getAnyInitializer, other);
  }
  //hasInit
  arboretum_create_edge(obj, context_.data_model_.hasInit, context_.data_model_.arboretum_node_for(D->hasInit()));
  //getInit
  {
    const Entity* other = context_.resolve(D->getInit());
    arboretum_create_edge(obj, context_.data_model_.getInit8, other);
  }
  //getInitializingDeclaration
  {
    const Entity* other = context_.resolve(D->getInitializingDeclaration());
    arboretum_create_edge(obj, context_.data_model_.getInitializingDeclaration1, other);
  }
  //ensureEvaluatedStmt
  //getEvaluatedStmt
  //evaluateValue
  //getEvaluatedValue
  //hasConstantInitialization
  arboretum_create_edge(obj, context_.data_model_.hasConstantInitialization, context_.data_model_.arboretum_node_for(D->hasConstantInitialization()));
  //getInitStyle
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getInitStyle());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getInitStyle, enum_value);
    }
  }
  //isDirectInit
  arboretum_create_edge(obj, context_.data_model_.isDirectInit, context_.data_model_.arboretum_node_for(D->isDirectInit()));
  //isThisDeclarationADemotedDefinition
  arboretum_create_edge(obj, context_.data_model_.isThisDeclarationADemotedDefinition1, context_.data_model_.arboretum_node_for(D->isThisDeclarationADemotedDefinition()));
  //isExceptionVariable
  arboretum_create_edge(obj, context_.data_model_.isExceptionVariable, context_.data_model_.arboretum_node_for(D->isExceptionVariable()));
  //isNRVOVariable
  arboretum_create_edge(obj, context_.data_model_.isNRVOVariable, context_.data_model_.arboretum_node_for(D->isNRVOVariable()));
  //isCXXForRangeDecl
  arboretum_create_edge(obj, context_.data_model_.isCXXForRangeDecl, context_.data_model_.arboretum_node_for(D->isCXXForRangeDecl()));
  //isObjCForDecl
  arboretum_create_edge(obj, context_.data_model_.isObjCForDecl, context_.data_model_.arboretum_node_for(D->isObjCForDecl()));
  //isARCPseudoStrong
  arboretum_create_edge(obj, context_.data_model_.isARCPseudoStrong, context_.data_model_.arboretum_node_for(D->isARCPseudoStrong()));
  //isInline
  arboretum_create_edge(obj, context_.data_model_.isInline, context_.data_model_.arboretum_node_for(D->isInline()));
  //isInlineSpecified
  arboretum_create_edge(obj, context_.data_model_.isInlineSpecified, context_.data_model_.arboretum_node_for(D->isInlineSpecified()));
  //isConstexpr
  arboretum_create_edge(obj, context_.data_model_.isConstexpr, context_.data_model_.arboretum_node_for(D->isConstexpr()));
  //isInitCapture
  arboretum_create_edge(obj, context_.data_model_.isInitCapture1, context_.data_model_.arboretum_node_for(D->isInitCapture()));
  //isParameterPack
  arboretum_create_edge(obj, context_.data_model_.isParameterPack2, context_.data_model_.arboretum_node_for(D->isParameterPack()));
  //isPreviousDeclInSameBlockScope
  arboretum_create_edge(obj, context_.data_model_.isPreviousDeclInSameBlockScope, context_.data_model_.arboretum_node_for(D->isPreviousDeclInSameBlockScope()));
  //isEscapingByref
  arboretum_create_edge(obj, context_.data_model_.isEscapingByref, context_.data_model_.arboretum_node_for(D->isEscapingByref()));
  //isNonEscapingByref
  arboretum_create_edge(obj, context_.data_model_.isNonEscapingByref, context_.data_model_.arboretum_node_for(D->isNonEscapingByref()));
  //hasDependentAlignment
  arboretum_create_edge(obj, context_.data_model_.hasDependentAlignment, context_.data_model_.arboretum_node_for(D->hasDependentAlignment()));
  //getTemplateInstantiationPattern
  {
    const Entity* other = context_.resolve(D->getTemplateInstantiationPattern());
    arboretum_create_edge(obj, context_.data_model_.getTemplateInstantiationPattern2, other);
  }
  //getInstantiatedFromStaticDataMember
  {
    const Entity* other = context_.resolve(D->getInstantiatedFromStaticDataMember());
    arboretum_create_edge(obj, context_.data_model_.getInstantiatedFromStaticDataMember, other);
  }
  //getTemplateSpecializationKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getTemplateSpecializationKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getTemplateSpecializationKind1, enum_value);
    }
  }
  //getTemplateSpecializationKindForInstantiation
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getTemplateSpecializationKindForInstantiation());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getTemplateSpecializationKindForInstantiation, enum_value);
    }
  }
  //getPointOfInstantiation
  {
    const Entity* other = context_.source_model_.resolve(D->getPointOfInstantiation());
    arboretum_create_edge(obj, context_.data_model_.getPointOfInstantiation1, other);
  }
  //getMemberSpecializationInfo
  //getDescribedVarTemplate
  {
    const Entity* other = context_.resolve(D->getDescribedVarTemplate());
    arboretum_create_edge(obj, context_.data_model_.getDescribedVarTemplate, other);
  }
  //isKnownToBeDefined
  arboretum_create_edge(obj, context_.data_model_.isKnownToBeDefined, context_.data_model_.arboretum_node_for(D->isKnownToBeDefined()));
  return true;
}

bool ArboretumASTVisitor::VisitVarTemplateDecl(clang::VarTemplateDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getTemplatedDecl
  {
    const Entity* other = context_.resolve(D->getTemplatedDecl());
    arboretum_create_edge(obj, context_.data_model_.getTemplatedDecl3, other);
  }
  //isThisDeclarationADefinition
  arboretum_create_edge(obj, context_.data_model_.isThisDeclarationADefinition7, context_.data_model_.arboretum_node_for(D->isThisDeclarationADefinition()));
  //getCanonicalDecl
  {
    const Entity* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_edge(obj, context_.data_model_.getCanonicalDecl23, other);
  }
  //getPreviousDecl
  {
    const Entity* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_edge(obj, context_.data_model_.getPreviousDecl7, other);
  }
  //getMostRecentDecl
  {
    const Entity* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_edge(obj, context_.data_model_.getMostRecentDecl10, other);
  }
  //getInstantiatedFromMemberTemplate
  {
    const Entity* other = context_.resolve(D->getInstantiatedFromMemberTemplate());
    arboretum_create_edge(obj, context_.data_model_.getInstantiatedFromMemberTemplate2, other);
  }
  //specializations
  // spec_range
  return true;
}

bool ArboretumASTVisitor::VisitVarTemplatePartialSpecializationDecl(clang::VarTemplatePartialSpecializationDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getTemplateParameters
  //getTemplateArgsAsWritten
  //hasAssociatedConstraints
  arboretum_create_edge(obj, context_.data_model_.hasAssociatedConstraints1, context_.data_model_.arboretum_node_for(D->hasAssociatedConstraints()));
  //getInstantiatedFromMember
  {
    const Entity* other = context_.resolve(D->getInstantiatedFromMember());
    arboretum_create_edge(obj, context_.data_model_.getInstantiatedFromMember, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange41, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitVarTemplateSpecializationDecl(clang::VarTemplateSpecializationDecl* D) {
  const Entity* obj = context_.resolve(D);
  //getSpecializedTemplate
  {
    const Entity* other = context_.resolve(D->getSpecializedTemplate());
    arboretum_create_edge(obj, context_.data_model_.getSpecializedTemplate, other);
  }
  //getTemplateArgs
  // const TemplateArgumentList &
  //getTemplateArgsInfo
  //getSpecializationKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getSpecializationKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getSpecializationKind, enum_value);
    }
  }
  //isExplicitSpecialization
  arboretum_create_edge(obj, context_.data_model_.isExplicitSpecialization, context_.data_model_.arboretum_node_for(D->isExplicitSpecialization()));
  //isClassScopeExplicitSpecialization
  arboretum_create_edge(obj, context_.data_model_.isClassScopeExplicitSpecialization, context_.data_model_.arboretum_node_for(D->isClassScopeExplicitSpecialization()));
  //isExplicitInstantiationOrSpecialization
  arboretum_create_edge(obj, context_.data_model_.isExplicitInstantiationOrSpecialization, context_.data_model_.arboretum_node_for(D->isExplicitInstantiationOrSpecialization()));
  //getPointOfInstantiation
  {
    const Entity* other = context_.source_model_.resolve(D->getPointOfInstantiation());
    arboretum_create_edge(obj, context_.data_model_.getPointOfInstantiation, other);
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
    const Entity* other = context_.source_model_.resolve(D->getExternLoc());
    arboretum_create_edge(obj, context_.data_model_.getExternLoc, other);
  }
  //getTemplateKeywordLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getTemplateKeywordLoc());
    arboretum_create_edge(obj, context_.data_model_.getTemplateKeywordLoc4, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange17, other);
  }
  return true;
}


// Stmts
bool ArboretumASTVisitor::VisitAbstractConditionalOperator(clang::AbstractConditionalOperator* D) {
  const Entity* obj = context_.resolve(D);
  //getCond
  {
    const Entity* other = context_.resolve(D->getCond());
    arboretum_create_edge(obj, context_.data_model_.getCond7, other);
  }
  //getTrueExpr
  {
    const Entity* other = context_.resolve(D->getTrueExpr());
    arboretum_create_edge(obj, context_.data_model_.getTrueExpr, other);
  }
  //getFalseExpr
  {
    const Entity* other = context_.resolve(D->getFalseExpr());
    arboretum_create_edge(obj, context_.data_model_.getFalseExpr, other);
  }
  //getQuestionLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getQuestionLoc());
    arboretum_create_edge(obj, context_.data_model_.getQuestionLoc, other);
  }
  //getColonLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getColonLoc());
    arboretum_create_edge(obj, context_.data_model_.getColonLoc1, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitAddrLabelExpr(clang::AddrLabelExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getAmpAmpLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getAmpAmpLoc());
    arboretum_create_edge(obj, context_.data_model_.getAmpAmpLoc, other);
  }
  //getLabelLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getLabelLoc());
    arboretum_create_edge(obj, context_.data_model_.getLabelLoc, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc52, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc52, other);
  }
  //getLabel
  {
    const Entity* other = context_.resolve(D->getLabel());
    arboretum_create_edge(obj, context_.data_model_.getLabel, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitArrayInitIndexExpr(clang::ArrayInitIndexExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc84, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc84, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitArrayInitLoopExpr(clang::ArrayInitLoopExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getCommonExpr
  {
    const Entity* other = context_.resolve(D->getCommonExpr());
    arboretum_create_edge(obj, context_.data_model_.getCommonExpr, other);
  }
  //getSubExpr
  {
    const Entity* other = context_.resolve(D->getSubExpr());
    arboretum_create_edge(obj, context_.data_model_.getSubExpr11, other);
  }
  //getArraySize
  // llvm::APInt
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc85, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc85, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitArraySubscriptExpr(clang::ArraySubscriptExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getLHS
  {
    const Entity* other = context_.resolve(D->getLHS());
    arboretum_create_edge(obj, context_.data_model_.getLHS7, other);
  }
  //getRHS
  {
    const Entity* other = context_.resolve(D->getRHS());
    arboretum_create_edge(obj, context_.data_model_.getRHS7, other);
  }
  //getBase
  {
    const Entity* other = context_.resolve(D->getBase());
    arboretum_create_edge(obj, context_.data_model_.getBase19, other);
  }
  //getIdx
  {
    const Entity* other = context_.resolve(D->getIdx());
    arboretum_create_edge(obj, context_.data_model_.getIdx3, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc104, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc104, other);
  }
  //getRBracketLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRBracketLoc());
    arboretum_create_edge(obj, context_.data_model_.getRBracketLoc3, other);
  }
  //getExprLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getExprLoc());
    arboretum_create_edge(obj, context_.data_model_.getExprLoc14, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitArrayTypeTraitExpr(clang::ArrayTypeTraitExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc12, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc12, other);
  }
  //getTrait
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getTrait());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getTrait, enum_value);
    }
  }
  //getQueriedType
  {
    const Entity* other = context_.resolve(D->getQueriedType());
    arboretum_create_edge(obj, context_.data_model_.getQueriedType, other);
  }
  //getQueriedTypeSourceInfo
  //getDimensionExpression
  {
    const Entity* other = context_.resolve(D->getDimensionExpression());
    arboretum_create_edge(obj, context_.data_model_.getDimensionExpression, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitAsTypeExpr(clang::AsTypeExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getSrcExpr
  {
    const Entity* other = context_.resolve(D->getSrcExpr());
    arboretum_create_edge(obj, context_.data_model_.getSrcExpr, other);
  }
  //getBuiltinLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBuiltinLoc());
    arboretum_create_edge(obj, context_.data_model_.getBuiltinLoc2, other);
  }
  //getRParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc12, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc80, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc80, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitAsmStmt(clang::AsmStmt* D) {
  const Entity* obj = context_.resolve(D);
  //getAsmLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getAsmLoc());
    arboretum_create_edge(obj, context_.data_model_.getAsmLoc, other);
  }
  //isSimple
  arboretum_create_edge(obj, context_.data_model_.isSimple, context_.data_model_.arboretum_node_for(D->isSimple()));
  //isVolatile
  arboretum_create_edge(obj, context_.data_model_.isVolatile, context_.data_model_.arboretum_node_for(D->isVolatile()));
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc6, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc6, other);
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
  const Entity* obj = context_.resolve(D);
  //getPtr
  {
    const Entity* other = context_.resolve(D->getPtr());
    arboretum_create_edge(obj, context_.data_model_.getPtr, other);
  }
  //getOrder
  {
    const Entity* other = context_.resolve(D->getOrder());
    arboretum_create_edge(obj, context_.data_model_.getOrder, other);
  }
  //getScope
  {
    const Entity* other = context_.resolve(D->getScope());
    arboretum_create_edge(obj, context_.data_model_.getScope, other);
  }
  //getVal1
  {
    const Entity* other = context_.resolve(D->getVal1());
    arboretum_create_edge(obj, context_.data_model_.getVal1, other);
  }
  //getOrderFail
  {
    const Entity* other = context_.resolve(D->getOrderFail());
    arboretum_create_edge(obj, context_.data_model_.getOrderFail, other);
  }
  //getVal2
  {
    const Entity* other = context_.resolve(D->getVal2());
    arboretum_create_edge(obj, context_.data_model_.getVal2, other);
  }
  //getWeak
  {
    const Entity* other = context_.resolve(D->getWeak());
    arboretum_create_edge(obj, context_.data_model_.getWeak, other);
  }
  //getValueType
  {
    const Entity* other = context_.resolve(D->getValueType());
    arboretum_create_edge(obj, context_.data_model_.getValueType, other);
  }
  //getOp
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getOp());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getOp, enum_value);
    }
  }
  //getOpAsString
  // StringRef
  //getNumSubExprs
  // unsigned int
  //getSubExprs
  //isVolatile
  arboretum_create_edge(obj, context_.data_model_.isVolatile1, context_.data_model_.arboretum_node_for(D->isVolatile()));
  //isCmpXChg
  arboretum_create_edge(obj, context_.data_model_.isCmpXChg, context_.data_model_.arboretum_node_for(D->isCmpXChg()));
  //isOpenCL
  arboretum_create_edge(obj, context_.data_model_.isOpenCL, context_.data_model_.arboretum_node_for(D->isOpenCL()));
  //getBuiltinLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBuiltinLoc());
    arboretum_create_edge(obj, context_.data_model_.getBuiltinLoc1, other);
  }
  //getRParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc11, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc77, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc77, other);
  }
  //children
  // const_child_range
  //getScopeModel
  // std::unique_ptr<AtomicScopeModel>
  return true;
}

bool ArboretumASTVisitor::VisitAttributedStmt(clang::AttributedStmt* D) {
  const Entity* obj = context_.resolve(D);
  //getAttrLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getAttrLoc());
    arboretum_create_edge(obj, context_.data_model_.getAttrLoc, other);
  }
  //getAttrs
  // ArrayRef<const Attr *>
  //getSubStmt
  {
    const Entity* other = context_.resolve(D->getSubStmt());
    arboretum_create_edge(obj, context_.data_model_.getSubStmt9, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc112, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc113, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitBinaryConditionalOperator(clang::BinaryConditionalOperator* D) {
  const Entity* obj = context_.resolve(D);
  //getCommon
  {
    const Entity* other = context_.resolve(D->getCommon());
    arboretum_create_edge(obj, context_.data_model_.getCommon, other);
  }
  //getOpaqueValue
  {
    const Entity* other = context_.resolve(D->getOpaqueValue());
    arboretum_create_edge(obj, context_.data_model_.getOpaqueValue, other);
  }
  //getCond
  {
    const Entity* other = context_.resolve(D->getCond());
    arboretum_create_edge(obj, context_.data_model_.getCond8, other);
  }
  //getTrueExpr
  {
    const Entity* other = context_.resolve(D->getTrueExpr());
    arboretum_create_edge(obj, context_.data_model_.getTrueExpr1, other);
  }
  //getFalseExpr
  {
    const Entity* other = context_.resolve(D->getFalseExpr());
    arboretum_create_edge(obj, context_.data_model_.getFalseExpr1, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc49, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc49, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitBinaryOperator(clang::BinaryOperator* D) {
  const Entity* obj = context_.resolve(D);
  //getExprLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getExprLoc());
    arboretum_create_edge(obj, context_.data_model_.getExprLoc11, other);
  }
  //getOperatorLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getOperatorLoc());
    arboretum_create_edge(obj, context_.data_model_.getOperatorLoc4, other);
  }
  //getOpcode
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getOpcode());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getOpcode1, enum_value);
    }
  }
  //getLHS
  {
    const Entity* other = context_.resolve(D->getLHS());
    arboretum_create_edge(obj, context_.data_model_.getLHS5, other);
  }
  //getRHS
  {
    const Entity* other = context_.resolve(D->getRHS());
    arboretum_create_edge(obj, context_.data_model_.getRHS5, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc88, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc88, other);
  }
  //getOpcodeStr
  // StringRef
  //isPtrMemOp
  arboretum_create_edge(obj, context_.data_model_.isPtrMemOp1, context_.data_model_.arboretum_node_for(D->isPtrMemOp()));
  //isMultiplicativeOp
  arboretum_create_edge(obj, context_.data_model_.isMultiplicativeOp1, context_.data_model_.arboretum_node_for(D->isMultiplicativeOp()));
  //isAdditiveOp
  arboretum_create_edge(obj, context_.data_model_.isAdditiveOp1, context_.data_model_.arboretum_node_for(D->isAdditiveOp()));
  //isShiftOp
  arboretum_create_edge(obj, context_.data_model_.isShiftOp1, context_.data_model_.arboretum_node_for(D->isShiftOp()));
  //isBitwiseOp
  arboretum_create_edge(obj, context_.data_model_.isBitwiseOp1, context_.data_model_.arboretum_node_for(D->isBitwiseOp()));
  //isRelationalOp
  arboretum_create_edge(obj, context_.data_model_.isRelationalOp1, context_.data_model_.arboretum_node_for(D->isRelationalOp()));
  //isEqualityOp
  arboretum_create_edge(obj, context_.data_model_.isEqualityOp1, context_.data_model_.arboretum_node_for(D->isEqualityOp()));
  //isComparisonOp
  arboretum_create_edge(obj, context_.data_model_.isComparisonOp2, context_.data_model_.arboretum_node_for(D->isComparisonOp()));
  //isCommaOp
  arboretum_create_edge(obj, context_.data_model_.isCommaOp1, context_.data_model_.arboretum_node_for(D->isCommaOp()));
  //isLogicalOp
  arboretum_create_edge(obj, context_.data_model_.isLogicalOp1, context_.data_model_.arboretum_node_for(D->isLogicalOp()));
  //isAssignmentOp
  arboretum_create_edge(obj, context_.data_model_.isAssignmentOp2, context_.data_model_.arboretum_node_for(D->isAssignmentOp()));
  //isCompoundAssignmentOp
  arboretum_create_edge(obj, context_.data_model_.isCompoundAssignmentOp1, context_.data_model_.arboretum_node_for(D->isCompoundAssignmentOp()));
  //isShiftAssignOp
  arboretum_create_edge(obj, context_.data_model_.isShiftAssignOp1, context_.data_model_.arboretum_node_for(D->isShiftAssignOp()));
  //children
  // const_child_range
  //hasStoredFPFeatures
  arboretum_create_edge(obj, context_.data_model_.hasStoredFPFeatures1, context_.data_model_.arboretum_node_for(D->hasStoredFPFeatures()));
  //getStoredFPFeatures
  // FPOptionsOverride
  //getFPFeatures
  // FPOptionsOverride
  return true;
}

bool ArboretumASTVisitor::VisitBlockExpr(clang::BlockExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getBlockDecl
  {
    const Entity* other = context_.resolve(D->getBlockDecl());
    arboretum_create_edge(obj, context_.data_model_.getBlockDecl, other);
  }
  //getCaretLocation
  {
    const Entity* other = context_.source_model_.resolve(D->getCaretLocation());
    arboretum_create_edge(obj, context_.data_model_.getCaretLocation, other);
  }
  //getBody
  {
    const Entity* other = context_.resolve(D->getBody());
    arboretum_create_edge(obj, context_.data_model_.getBody3, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc23, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc23, other);
  }
  //getFunctionType
  {
    const Entity* other = context_.resolve(D->getFunctionType());
    arboretum_create_edge(obj, context_.data_model_.getFunctionType, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitBreakStmt(clang::BreakStmt* D) {
  const Entity* obj = context_.resolve(D);
  //getBreakLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBreakLoc());
    arboretum_create_edge(obj, context_.data_model_.getBreakLoc, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc155, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc153, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitBuiltinBitCastExpr(clang::BuiltinBitCastExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc122, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc124, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitCStyleCastExpr(clang::CStyleCastExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getLParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getLParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getLParenLoc11, other);
  }
  //getRParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc17, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc92, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc92, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitCUDAKernelCallExpr(clang::CUDAKernelCallExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getConfig
  {
    const Entity* other = context_.resolve(D->getConfig());
    arboretum_create_edge(obj, context_.data_model_.getConfig, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXAddrspaceCastExpr(clang::CXXAddrspaceCastExpr* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCXXBindTemporaryExpr(clang::CXXBindTemporaryExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getTemporary
  //getSubExpr
  {
    const Entity* other = context_.resolve(D->getSubExpr());
    arboretum_create_edge(obj, context_.data_model_.getSubExpr2, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc7, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc7, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXBoolLiteralExpr(clang::CXXBoolLiteralExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getValue
  arboretum_create_edge(obj, context_.data_model_.getValue1, context_.data_model_.arboretum_node_for(D->getValue()));
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc16, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc16, other);
  }
  //getLocation
  {
    const Entity* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.getLocation, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXCatchStmt(clang::CXXCatchStmt* D) {
  const Entity* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc44, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc44, other);
  }
  //getCatchLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getCatchLoc());
    arboretum_create_edge(obj, context_.data_model_.getCatchLoc, other);
  }
  //getExceptionDecl
  {
    const Entity* other = context_.resolve(D->getExceptionDecl());
    arboretum_create_edge(obj, context_.data_model_.getExceptionDecl, other);
  }
  //getCaughtType
  {
    const Entity* other = context_.resolve(D->getCaughtType());
    arboretum_create_edge(obj, context_.data_model_.getCaughtType, other);
  }
  //getHandlerBlock
  {
    const Entity* other = context_.resolve(D->getHandlerBlock());
    arboretum_create_edge(obj, context_.data_model_.getHandlerBlock, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXConstCastExpr(clang::CXXConstCastExpr* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCXXConstructExpr(clang::CXXConstructExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getConstructor
  {
    const Entity* other = context_.resolve(D->getConstructor());
    arboretum_create_edge(obj, context_.data_model_.getConstructor, other);
  }
  //getLocation
  {
    const Entity* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.getLocation8, other);
  }
  //isElidable
  arboretum_create_edge(obj, context_.data_model_.isElidable, context_.data_model_.arboretum_node_for(D->isElidable()));
  //hadMultipleCandidates
  arboretum_create_edge(obj, context_.data_model_.hadMultipleCandidates2, context_.data_model_.arboretum_node_for(D->hadMultipleCandidates()));
  //isListInitialization
  arboretum_create_edge(obj, context_.data_model_.isListInitialization1, context_.data_model_.arboretum_node_for(D->isListInitialization()));
  //isStdInitListInitialization
  arboretum_create_edge(obj, context_.data_model_.isStdInitListInitialization, context_.data_model_.arboretum_node_for(D->isStdInitListInitialization()));
  //requiresZeroInitialization
  arboretum_create_edge(obj, context_.data_model_.requiresZeroInitialization, context_.data_model_.arboretum_node_for(D->requiresZeroInitialization()));
  //getConstructionKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getConstructionKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getConstructionKind, enum_value);
    }
  }
  //arguments
  // const_arg_range
  //getArgs
  //getNumArgs
  // unsigned int
  //isImmediateEscalating
  arboretum_create_edge(obj, context_.data_model_.isImmediateEscalating1, context_.data_model_.arboretum_node_for(D->isImmediateEscalating()));
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc68, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc68, other);
  }
  //getParenOrBraceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getParenOrBraceRange());
    arboretum_create_edge(obj, context_.data_model_.getParenOrBraceRange, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXDefaultArgExpr(clang::CXXDefaultArgExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getParam
  {
    const Entity* other = context_.resolve(D->getParam());
    arboretum_create_edge(obj, context_.data_model_.getParam, other);
  }
  //hasRewrittenInit
  arboretum_create_edge(obj, context_.data_model_.hasRewrittenInit1, context_.data_model_.arboretum_node_for(D->hasRewrittenInit()));
  //getExpr
  {
    const Entity* other = context_.resolve(D->getExpr());
    arboretum_create_edge(obj, context_.data_model_.getExpr7, other);
  }
  //getRewrittenExpr
  {
    const Entity* other = context_.resolve(D->getRewrittenExpr());
    arboretum_create_edge(obj, context_.data_model_.getRewrittenExpr3, other);
  }
  //getAdjustedRewrittenExpr
  {
    const Entity* other = context_.resolve(D->getAdjustedRewrittenExpr());
    arboretum_create_edge(obj, context_.data_model_.getAdjustedRewrittenExpr1, other);
  }
  //getUsedContext
  //getUsedLocation
  {
    const Entity* other = context_.source_model_.resolve(D->getUsedLocation());
    arboretum_create_edge(obj, context_.data_model_.getUsedLocation1, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc71, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc71, other);
  }
  //getExprLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getExprLoc());
    arboretum_create_edge(obj, context_.data_model_.getExprLoc6, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXDefaultInitExpr(clang::CXXDefaultInitExpr* D) {
  const Entity* obj = context_.resolve(D);
  //hasRewrittenInit
  arboretum_create_edge(obj, context_.data_model_.hasRewrittenInit, context_.data_model_.arboretum_node_for(D->hasRewrittenInit()));
  //getField
  {
    const Entity* other = context_.resolve(D->getField());
    arboretum_create_edge(obj, context_.data_model_.getField1, other);
  }
  //getExpr
  {
    const Entity* other = context_.resolve(D->getExpr());
    arboretum_create_edge(obj, context_.data_model_.getExpr5, other);
  }
  //getRewrittenExpr
  {
    const Entity* other = context_.resolve(D->getRewrittenExpr());
    arboretum_create_edge(obj, context_.data_model_.getRewrittenExpr, other);
  }
  //getUsedContext
  //getUsedLocation
  {
    const Entity* other = context_.source_model_.resolve(D->getUsedLocation());
    arboretum_create_edge(obj, context_.data_model_.getUsedLocation, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc70, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc70, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXDeleteExpr(clang::CXXDeleteExpr* D) {
  const Entity* obj = context_.resolve(D);
  //isGlobalDelete
  arboretum_create_edge(obj, context_.data_model_.isGlobalDelete, context_.data_model_.arboretum_node_for(D->isGlobalDelete()));
  //isArrayForm
  arboretum_create_edge(obj, context_.data_model_.isArrayForm, context_.data_model_.arboretum_node_for(D->isArrayForm()));
  //isArrayFormAsWritten
  arboretum_create_edge(obj, context_.data_model_.isArrayFormAsWritten, context_.data_model_.arboretum_node_for(D->isArrayFormAsWritten()));
  //doesUsualArrayDeleteWantSize
  arboretum_create_edge(obj, context_.data_model_.doesUsualArrayDeleteWantSize1, context_.data_model_.arboretum_node_for(D->doesUsualArrayDeleteWantSize()));
  //getOperatorDelete
  {
    const Entity* other = context_.resolve(D->getOperatorDelete());
    arboretum_create_edge(obj, context_.data_model_.getOperatorDelete1, other);
  }
  //getArgument
  {
    const Entity* other = context_.resolve(D->getArgument());
    arboretum_create_edge(obj, context_.data_model_.getArgument1, other);
  }
  //getDestroyedType
  {
    const Entity* other = context_.resolve(D->getDestroyedType());
    arboretum_create_edge(obj, context_.data_model_.getDestroyedType, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc107, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc107, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXDependentScopeMemberExpr(clang::CXXDependentScopeMemberExpr* D) {
  const Entity* obj = context_.resolve(D);
  //isImplicitAccess
  arboretum_create_edge(obj, context_.data_model_.isImplicitAccess1, context_.data_model_.arboretum_node_for(D->isImplicitAccess()));
  //getBaseType
  {
    const Entity* other = context_.resolve(D->getBaseType());
    arboretum_create_edge(obj, context_.data_model_.getBaseType, other);
  }
  //isArrow
  arboretum_create_edge(obj, context_.data_model_.isArrow3, context_.data_model_.arboretum_node_for(D->isArrow()));
  //getOperatorLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getOperatorLoc());
    arboretum_create_edge(obj, context_.data_model_.getOperatorLoc1, other);
  }
  //getQualifier
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getFirstQualifierFoundInScope
  {
    const Entity* other = context_.resolve(D->getFirstQualifierFoundInScope());
    arboretum_create_edge(obj, context_.data_model_.getFirstQualifierFoundInScope, other);
  }
  //getMemberNameInfo
  // const DeclarationNameInfo &
  //getMember
  // DeclarationName
  //getMemberLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getMemberLoc());
    arboretum_create_edge(obj, context_.data_model_.getMemberLoc1, other);
  }
  //getTemplateKeywordLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getTemplateKeywordLoc());
    arboretum_create_edge(obj, context_.data_model_.getTemplateKeywordLoc2, other);
  }
  //getLAngleLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getLAngleLoc());
    arboretum_create_edge(obj, context_.data_model_.getLAngleLoc2, other);
  }
  //getRAngleLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRAngleLoc());
    arboretum_create_edge(obj, context_.data_model_.getRAngleLoc2, other);
  }
  //hasTemplateKeyword
  arboretum_create_edge(obj, context_.data_model_.hasTemplateKeyword2, context_.data_model_.arboretum_node_for(D->hasTemplateKeyword()));
  //hasExplicitTemplateArgs
  arboretum_create_edge(obj, context_.data_model_.hasExplicitTemplateArgs3, context_.data_model_.arboretum_node_for(D->hasExplicitTemplateArgs()));
  //getTemplateArgs
  //getNumTemplateArgs
  // unsigned int
  //template_arguments
  // ArrayRef<TemplateArgumentLoc>
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc63, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc63, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXDynamicCastExpr(clang::CXXDynamicCastExpr* D) {
  const Entity* obj = context_.resolve(D);
  //isAlwaysNull
  arboretum_create_edge(obj, context_.data_model_.isAlwaysNull, context_.data_model_.arboretum_node_for(D->isAlwaysNull()));
  return true;
}

bool ArboretumASTVisitor::VisitCXXFoldExpr(clang::CXXFoldExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getCallee
  {
    const Entity* other = context_.resolve(D->getCallee());
    arboretum_create_edge(obj, context_.data_model_.getCallee, other);
  }
  //getLHS
  {
    const Entity* other = context_.resolve(D->getLHS());
    arboretum_create_edge(obj, context_.data_model_.getLHS2, other);
  }
  //getRHS
  {
    const Entity* other = context_.resolve(D->getRHS());
    arboretum_create_edge(obj, context_.data_model_.getRHS2, other);
  }
  //isRightFold
  arboretum_create_edge(obj, context_.data_model_.isRightFold, context_.data_model_.arboretum_node_for(D->isRightFold()));
  //isLeftFold
  arboretum_create_edge(obj, context_.data_model_.isLeftFold, context_.data_model_.arboretum_node_for(D->isLeftFold()));
  //getPattern
  {
    const Entity* other = context_.resolve(D->getPattern());
    arboretum_create_edge(obj, context_.data_model_.getPattern, other);
  }
  //getInit
  {
    const Entity* other = context_.resolve(D->getInit());
    arboretum_create_edge(obj, context_.data_model_.getInit5, other);
  }
  //getLParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getLParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getLParenLoc6, other);
  }
  //getRParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc9, other);
  }
  //getEllipsisLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEllipsisLoc());
    arboretum_create_edge(obj, context_.data_model_.getEllipsisLoc1, other);
  }
  //getOperator
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getOperator());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getOperator, enum_value);
    }
  }
  //getNumExpansions
  // std::optional<unsigned int>
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc53, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc53, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXForRangeStmt(clang::CXXForRangeStmt* D) {
  const Entity* obj = context_.resolve(D);
  //getInit
  {
    const Entity* other = context_.resolve(D->getInit());
    arboretum_create_edge(obj, context_.data_model_.getInit14, other);
  }
  //getLoopVariable
  {
    const Entity* other = context_.resolve(D->getLoopVariable());
    arboretum_create_edge(obj, context_.data_model_.getLoopVariable1, other);
  }
  //getRangeInit
  {
    const Entity* other = context_.resolve(D->getRangeInit());
    arboretum_create_edge(obj, context_.data_model_.getRangeInit1, other);
  }
  //getRangeStmt
  {
    const Entity* other = context_.resolve(D->getRangeStmt());
    arboretum_create_edge(obj, context_.data_model_.getRangeStmt1, other);
  }
  //getBeginStmt
  {
    const Entity* other = context_.resolve(D->getBeginStmt());
    arboretum_create_edge(obj, context_.data_model_.getBeginStmt1, other);
  }
  //getEndStmt
  {
    const Entity* other = context_.resolve(D->getEndStmt());
    arboretum_create_edge(obj, context_.data_model_.getEndStmt1, other);
  }
  //getCond
  {
    const Entity* other = context_.resolve(D->getCond());
    arboretum_create_edge(obj, context_.data_model_.getCond18, other);
  }
  //getInc
  {
    const Entity* other = context_.resolve(D->getInc());
    arboretum_create_edge(obj, context_.data_model_.getInc4, other);
  }
  //getLoopVarStmt
  {
    const Entity* other = context_.resolve(D->getLoopVarStmt());
    arboretum_create_edge(obj, context_.data_model_.getLoopVarStmt1, other);
  }
  //getBody
  {
    const Entity* other = context_.resolve(D->getBody());
    arboretum_create_edge(obj, context_.data_model_.getBody22, other);
  }
  //getForLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getForLoc());
    arboretum_create_edge(obj, context_.data_model_.getForLoc1, other);
  }
  //getCoawaitLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getCoawaitLoc());
    arboretum_create_edge(obj, context_.data_model_.getCoawaitLoc, other);
  }
  //getColonLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getColonLoc());
    arboretum_create_edge(obj, context_.data_model_.getColonLoc4, other);
  }
  //getRParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc36, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc156, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc154, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXFunctionalCastExpr(clang::CXXFunctionalCastExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getLParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getLParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getLParenLoc10, other);
  }
  //getRParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc15, other);
  }
  //isListInitialization
  arboretum_create_edge(obj, context_.data_model_.isListInitialization2, context_.data_model_.arboretum_node_for(D->isListInitialization()));
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc89, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc89, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXInheritedCtorInitExpr(clang::CXXInheritedCtorInitExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getConstructor
  {
    const Entity* other = context_.resolve(D->getConstructor());
    arboretum_create_edge(obj, context_.data_model_.getConstructor1, other);
  }
  //constructsVBase
  arboretum_create_edge(obj, context_.data_model_.constructsVBase, context_.data_model_.arboretum_node_for(D->constructsVBase()));
  //getConstructionKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getConstructionKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getConstructionKind1, enum_value);
    }
  }
  //inheritedFromVBase
  arboretum_create_edge(obj, context_.data_model_.inheritedFromVBase, context_.data_model_.arboretum_node_for(D->inheritedFromVBase()));
  //getLocation
  {
    const Entity* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.getLocation16, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc145, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc144, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXMemberCallExpr(clang::CXXMemberCallExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getImplicitObjectArgument
  {
    const Entity* other = context_.resolve(D->getImplicitObjectArgument());
    arboretum_create_edge(obj, context_.data_model_.getImplicitObjectArgument, other);
  }
  //getObjectType
  {
    const Entity* other = context_.resolve(D->getObjectType());
    arboretum_create_edge(obj, context_.data_model_.getObjectType, other);
  }
  //getMethodDecl
  {
    const Entity* other = context_.resolve(D->getMethodDecl());
    arboretum_create_edge(obj, context_.data_model_.getMethodDecl, other);
  }
  //getRecordDecl
  {
    const Entity* other = context_.resolve(D->getRecordDecl());
    arboretum_create_edge(obj, context_.data_model_.getRecordDecl, other);
  }
  //getExprLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getExprLoc());
    arboretum_create_edge(obj, context_.data_model_.getExprLoc15, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXNamedCastExpr(clang::CXXNamedCastExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getCastName
  //getOperatorLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getOperatorLoc());
    arboretum_create_edge(obj, context_.data_model_.getOperatorLoc9, other);
  }
  //getRParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc30, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc139, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc139, other);
  }
  //getAngleBrackets
  {
    const Entity* other = context_.source_model_.resolve(D->getAngleBrackets());
    arboretum_create_edge(obj, context_.data_model_.getAngleBrackets, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXNewExpr(clang::CXXNewExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getAllocatedType
  {
    const Entity* other = context_.resolve(D->getAllocatedType());
    arboretum_create_edge(obj, context_.data_model_.getAllocatedType, other);
  }
  //getAllocatedTypeSourceInfo
  //getOperatorNew
  {
    const Entity* other = context_.resolve(D->getOperatorNew());
    arboretum_create_edge(obj, context_.data_model_.getOperatorNew, other);
  }
  //getOperatorDelete
  {
    const Entity* other = context_.resolve(D->getOperatorDelete());
    arboretum_create_edge(obj, context_.data_model_.getOperatorDelete, other);
  }
  //isArray
  arboretum_create_edge(obj, context_.data_model_.isArray, context_.data_model_.arboretum_node_for(D->isArray()));
  //getArraySize
  // std::optional<const Expr *>
  //getNumPlacementArgs
  // unsigned int
  //isParenTypeId
  arboretum_create_edge(obj, context_.data_model_.isParenTypeId, context_.data_model_.arboretum_node_for(D->isParenTypeId()));
  //getTypeIdParens
  {
    const Entity* other = context_.source_model_.resolve(D->getTypeIdParens());
    arboretum_create_edge(obj, context_.data_model_.getTypeIdParens, other);
  }
  //isGlobalNew
  arboretum_create_edge(obj, context_.data_model_.isGlobalNew, context_.data_model_.arboretum_node_for(D->isGlobalNew()));
  //hasInitializer
  arboretum_create_edge(obj, context_.data_model_.hasInitializer, context_.data_model_.arboretum_node_for(D->hasInitializer()));
  //getInitializationStyle
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getInitializationStyle());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getInitializationStyle, enum_value);
    }
  }
  //getInitializer
  {
    const Entity* other = context_.resolve(D->getInitializer());
    arboretum_create_edge(obj, context_.data_model_.getInitializer1, other);
  }
  //getConstructExpr
  {
    const Entity* other = context_.resolve(D->getConstructExpr());
    arboretum_create_edge(obj, context_.data_model_.getConstructExpr, other);
  }
  //passAlignment
  arboretum_create_edge(obj, context_.data_model_.passAlignment, context_.data_model_.arboretum_node_for(D->passAlignment()));
  //doesUsualArrayDeleteWantSize
  arboretum_create_edge(obj, context_.data_model_.doesUsualArrayDeleteWantSize, context_.data_model_.arboretum_node_for(D->doesUsualArrayDeleteWantSize()));
  //placement_arguments
  // llvm::iterator_range<const_arg_iterator>
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc65, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc65, other);
  }
  //getDirectInitRange
  {
    const Entity* other = context_.source_model_.resolve(D->getDirectInitRange());
    arboretum_create_edge(obj, context_.data_model_.getDirectInitRange, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange5, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXNoexceptExpr(clang::CXXNoexceptExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getOperand
  {
    const Entity* other = context_.resolve(D->getOperand());
    arboretum_create_edge(obj, context_.data_model_.getOperand1, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc136, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc136, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange48, other);
  }
  //getValue
  arboretum_create_edge(obj, context_.data_model_.getValue11, context_.data_model_.arboretum_node_for(D->getValue()));
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXNullPtrLiteralExpr(clang::CXXNullPtrLiteralExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc158, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc156, other);
  }
  //getLocation
  {
    const Entity* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.getLocation19, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXOperatorCallExpr(clang::CXXOperatorCallExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getOperator
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getOperator());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getOperator2, enum_value);
    }
  }
  //isAssignmentOp
  arboretum_create_edge(obj, context_.data_model_.isAssignmentOp4, context_.data_model_.arboretum_node_for(D->isAssignmentOp()));
  //isComparisonOp
  arboretum_create_edge(obj, context_.data_model_.isComparisonOp4, context_.data_model_.arboretum_node_for(D->isComparisonOp()));
  //isInfixBinaryOp
  arboretum_create_edge(obj, context_.data_model_.isInfixBinaryOp, context_.data_model_.arboretum_node_for(D->isInfixBinaryOp()));
  //getOperatorLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getOperatorLoc());
    arboretum_create_edge(obj, context_.data_model_.getOperatorLoc11, other);
  }
  //getExprLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getExprLoc());
    arboretum_create_edge(obj, context_.data_model_.getExprLoc17, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc151, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc150, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange55, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXParenListInitExpr(clang::CXXParenListInitExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getInitExprs
  // const ArrayRef<Expr *>
  //getUserSpecifiedInitExprs
  // const ArrayRef<Expr *>
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc2, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc2, other);
  }
  //getInitLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getInitLoc());
    arboretum_create_edge(obj, context_.data_model_.getInitLoc, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange1, other);
  }
  //getArrayFiller
  {
    const Entity* other = context_.resolve(D->getArrayFiller());
    arboretum_create_edge(obj, context_.data_model_.getArrayFiller1, other);
  }
  //getInitializedFieldInUnion
  {
    const Entity* other = context_.resolve(D->getInitializedFieldInUnion());
    arboretum_create_edge(obj, context_.data_model_.getInitializedFieldInUnion1, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXPseudoDestructorExpr(clang::CXXPseudoDestructorExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getBase
  {
    const Entity* other = context_.resolve(D->getBase());
    arboretum_create_edge(obj, context_.data_model_.getBase20, other);
  }
  //hasQualifier
  arboretum_create_edge(obj, context_.data_model_.hasQualifier2, context_.data_model_.arboretum_node_for(D->hasQualifier()));
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getQualifier
  //isArrow
  arboretum_create_edge(obj, context_.data_model_.isArrow6, context_.data_model_.arboretum_node_for(D->isArrow()));
  //getOperatorLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getOperatorLoc());
    arboretum_create_edge(obj, context_.data_model_.getOperatorLoc8, other);
  }
  //getScopeTypeInfo
  //getColonColonLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getColonColonLoc());
    arboretum_create_edge(obj, context_.data_model_.getColonColonLoc, other);
  }
  //getTildeLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getTildeLoc());
    arboretum_create_edge(obj, context_.data_model_.getTildeLoc, other);
  }
  //getDestroyedTypeInfo
  //getDestroyedTypeIdentifier
  //getDestroyedType
  {
    const Entity* other = context_.resolve(D->getDestroyedType());
    arboretum_create_edge(obj, context_.data_model_.getDestroyedType1, other);
  }
  //getDestroyedTypeLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getDestroyedTypeLoc());
    arboretum_create_edge(obj, context_.data_model_.getDestroyedTypeLoc, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc126, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc128, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXReinterpretCastExpr(clang::CXXReinterpretCastExpr* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCXXRewrittenBinaryOperator(clang::CXXRewrittenBinaryOperator* D) {
  const Entity* obj = context_.resolve(D);
  //getSemanticForm
  {
    const Entity* other = context_.resolve(D->getSemanticForm());
    arboretum_create_edge(obj, context_.data_model_.getSemanticForm1, other);
  }
  //getDecomposedForm
  // DecomposedForm
  //isReversed
  arboretum_create_edge(obj, context_.data_model_.isReversed, context_.data_model_.arboretum_node_for(D->isReversed()));
  //getOperator
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getOperator());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getOperator1, enum_value);
    }
  }
  //getOpcode
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getOpcode());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getOpcode, enum_value);
    }
  }
  //getOpcodeStr
  // StringRef
  //isComparisonOp
  arboretum_create_edge(obj, context_.data_model_.isComparisonOp, context_.data_model_.arboretum_node_for(D->isComparisonOp()));
  //isAssignmentOp
  arboretum_create_edge(obj, context_.data_model_.isAssignmentOp, context_.data_model_.arboretum_node_for(D->isAssignmentOp()));
  //getLHS
  {
    const Entity* other = context_.resolve(D->getLHS());
    arboretum_create_edge(obj, context_.data_model_.getLHS3, other);
  }
  //getRHS
  {
    const Entity* other = context_.resolve(D->getRHS());
    arboretum_create_edge(obj, context_.data_model_.getRHS3, other);
  }
  //getOperatorLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getOperatorLoc());
    arboretum_create_edge(obj, context_.data_model_.getOperatorLoc3, other);
  }
  //getExprLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getExprLoc());
    arboretum_create_edge(obj, context_.data_model_.getExprLoc8, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc74, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc74, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange7, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXScalarValueInitExpr(clang::CXXScalarValueInitExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getTypeSourceInfo
  //getRParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc16, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc91, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc91, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXStaticCastExpr(clang::CXXStaticCastExpr* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCXXStdInitializerListExpr(clang::CXXStdInitializerListExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getSubExpr
  {
    const Entity* other = context_.resolve(D->getSubExpr());
    arboretum_create_edge(obj, context_.data_model_.getSubExpr1, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXTemporaryObjectExpr(clang::CXXTemporaryObjectExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getTypeSourceInfo
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc130, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc130, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXThisExpr(clang::CXXThisExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getLocation
  {
    const Entity* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.getLocation7, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc62, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc62, other);
  }
  //isImplicit
  arboretum_create_edge(obj, context_.data_model_.isImplicit1, context_.data_model_.arboretum_node_for(D->isImplicit()));
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXThrowExpr(clang::CXXThrowExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getSubExpr
  {
    const Entity* other = context_.resolve(D->getSubExpr());
    arboretum_create_edge(obj, context_.data_model_.getSubExpr7, other);
  }
  //getThrowLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getThrowLoc());
    arboretum_create_edge(obj, context_.data_model_.getThrowLoc1, other);
  }
  //isThrownVariableInScope
  arboretum_create_edge(obj, context_.data_model_.isThrownVariableInScope, context_.data_model_.arboretum_node_for(D->isThrownVariableInScope()));
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc72, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc72, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXTryStmt(clang::CXXTryStmt* D) {
  const Entity* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc20, other);
  }
  //getTryLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getTryLoc());
    arboretum_create_edge(obj, context_.data_model_.getTryLoc1, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc20, other);
  }
  //getTryBlock
  {
    const Entity* other = context_.resolve(D->getTryBlock());
    arboretum_create_edge(obj, context_.data_model_.getTryBlock2, other);
  }
  //getNumHandlers
  // unsigned int
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXTypeidExpr(clang::CXXTypeidExpr* D) {
  const Entity* obj = context_.resolve(D);
  //isPotentiallyEvaluated
  arboretum_create_edge(obj, context_.data_model_.isPotentiallyEvaluated, context_.data_model_.arboretum_node_for(D->isPotentiallyEvaluated()));
  //isTypeOperand
  arboretum_create_edge(obj, context_.data_model_.isTypeOperand1, context_.data_model_.arboretum_node_for(D->isTypeOperand()));
  //getTypeOperandSourceInfo
  //getExprOperand
  {
    const Entity* other = context_.resolve(D->getExprOperand());
    arboretum_create_edge(obj, context_.data_model_.getExprOperand1, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc131, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc131, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange46, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXUnresolvedConstructExpr(clang::CXXUnresolvedConstructExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getTypeAsWritten
  {
    const Entity* other = context_.resolve(D->getTypeAsWritten());
    arboretum_create_edge(obj, context_.data_model_.getTypeAsWritten1, other);
  }
  //getTypeSourceInfo
  //getLParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getLParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getLParenLoc4, other);
  }
  //getRParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc6, other);
  }
  //isListInitialization
  arboretum_create_edge(obj, context_.data_model_.isListInitialization, context_.data_model_.arboretum_node_for(D->isListInitialization()));
  //getNumArgs
  // unsigned int
  //arguments
  // const_arg_range
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc42, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc42, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCXXUuidofExpr(clang::CXXUuidofExpr* D) {
  const Entity* obj = context_.resolve(D);
  //isTypeOperand
  arboretum_create_edge(obj, context_.data_model_.isTypeOperand, context_.data_model_.arboretum_node_for(D->isTypeOperand()));
  //getTypeOperandSourceInfo
  //getExprOperand
  {
    const Entity* other = context_.resolve(D->getExprOperand());
    arboretum_create_edge(obj, context_.data_model_.getExprOperand, other);
  }
  //getGuidDecl
  {
    const Entity* other = context_.resolve(D->getGuidDecl());
    arboretum_create_edge(obj, context_.data_model_.getGuidDecl, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc120, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc122, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange40, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCallExpr(clang::CallExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getCallee
  {
    const Entity* other = context_.resolve(D->getCallee());
    arboretum_create_edge(obj, context_.data_model_.getCallee2, other);
  }
  //getADLCallKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getADLCallKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getADLCallKind, enum_value);
    }
  }
  //usesADL
  arboretum_create_edge(obj, context_.data_model_.usesADL, context_.data_model_.arboretum_node_for(D->usesADL()));
  //hasStoredFPFeatures
  arboretum_create_edge(obj, context_.data_model_.hasStoredFPFeatures3, context_.data_model_.arboretum_node_for(D->hasStoredFPFeatures()));
  //getCalleeDecl
  {
    const Entity* other = context_.resolve(D->getCalleeDecl());
    arboretum_create_edge(obj, context_.data_model_.getCalleeDecl1, other);
  }
  //getDirectCallee
  {
    const Entity* other = context_.resolve(D->getDirectCallee());
    arboretum_create_edge(obj, context_.data_model_.getDirectCallee1, other);
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
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc18, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc94, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc94, other);
  }
  //isCallToStdMove
  arboretum_create_edge(obj, context_.data_model_.isCallToStdMove, context_.data_model_.arboretum_node_for(D->isCallToStdMove()));
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCapturedStmt(clang::CapturedStmt* D) {
  const Entity* obj = context_.resolve(D);
  //getCapturedStmt
  {
    const Entity* other = context_.resolve(D->getCapturedStmt());
    arboretum_create_edge(obj, context_.data_model_.getCapturedStmt2, other);
  }
  //getCapturedDecl
  {
    const Entity* other = context_.resolve(D->getCapturedDecl());
    arboretum_create_edge(obj, context_.data_model_.getCapturedDecl1, other);
  }
  //getCapturedRegionKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getCapturedRegionKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getCapturedRegionKind, enum_value);
    }
  }
  //getCapturedRecordDecl
  {
    const Entity* other = context_.resolve(D->getCapturedRecordDecl());
    arboretum_create_edge(obj, context_.data_model_.getCapturedRecordDecl, other);
  }
  //captures
  // capture_const_range
  //capture_size
  // unsigned int
  //capture_inits
  // const_capture_init_range
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc67, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc67, other);
  }
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange6, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCaseStmt(clang::CaseStmt* D) {
  const Entity* obj = context_.resolve(D);
  //caseStmtIsGNURange
  arboretum_create_edge(obj, context_.data_model_.caseStmtIsGNURange, context_.data_model_.arboretum_node_for(D->caseStmtIsGNURange()));
  //getCaseLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getCaseLoc());
    arboretum_create_edge(obj, context_.data_model_.getCaseLoc, other);
  }
  //getEllipsisLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEllipsisLoc());
    arboretum_create_edge(obj, context_.data_model_.getEllipsisLoc, other);
  }
  //getLHS
  {
    const Entity* other = context_.resolve(D->getLHS());
    arboretum_create_edge(obj, context_.data_model_.getLHS1, other);
  }
  //getRHS
  {
    const Entity* other = context_.resolve(D->getRHS());
    arboretum_create_edge(obj, context_.data_model_.getRHS1, other);
  }
  //getSubStmt
  {
    const Entity* other = context_.resolve(D->getSubStmt());
    arboretum_create_edge(obj, context_.data_model_.getSubStmt5, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc38, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc38, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCastExpr(clang::CastExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getCastKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getCastKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getCastKind, enum_value);
    }
  }
  //getCastKindName
  //getSubExpr
  {
    const Entity* other = context_.resolve(D->getSubExpr());
    arboretum_create_edge(obj, context_.data_model_.getSubExpr20, other);
  }
  //getSubExprAsWritten
  {
    const Entity* other = context_.resolve(D->getSubExprAsWritten());
    arboretum_create_edge(obj, context_.data_model_.getSubExprAsWritten1, other);
  }
  //getConversionFunction
  {
    const Entity* other = context_.resolve(D->getConversionFunction());
    arboretum_create_edge(obj, context_.data_model_.getConversionFunction, other);
  }
  //path_empty
  arboretum_create_edge(obj, context_.data_model_.path_empty, context_.data_model_.arboretum_node_for(D->path_empty()));
  //path_size
  // unsigned int
  //path
  // llvm::iterator_range<path_const_iterator>
  //hasStoredFPFeatures
  arboretum_create_edge(obj, context_.data_model_.hasStoredFPFeatures4, context_.data_model_.arboretum_node_for(D->hasStoredFPFeatures()));
  //getStoredFPFeatures
  // FPOptionsOverride
  //getFPFeatures
  // FPOptionsOverride
  //changesVolatileQualification
  arboretum_create_edge(obj, context_.data_model_.changesVolatileQualification, context_.data_model_.arboretum_node_for(D->changesVolatileQualification()));
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCharacterLiteral(clang::CharacterLiteral* D) {
  const Entity* obj = context_.resolve(D);
  //getLocation
  {
    const Entity* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.getLocation5, other);
  }
  //getKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getKind, enum_value);
    }
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc46, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc46, other);
  }
  //getValue
  // unsigned int
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitChooseExpr(clang::ChooseExpr* D) {
  const Entity* obj = context_.resolve(D);
  //isConditionTrue
  arboretum_create_edge(obj, context_.data_model_.isConditionTrue, context_.data_model_.arboretum_node_for(D->isConditionTrue()));
  //isConditionDependent
  arboretum_create_edge(obj, context_.data_model_.isConditionDependent, context_.data_model_.arboretum_node_for(D->isConditionDependent()));
  //getChosenSubExpr
  {
    const Entity* other = context_.resolve(D->getChosenSubExpr());
    arboretum_create_edge(obj, context_.data_model_.getChosenSubExpr, other);
  }
  //getCond
  {
    const Entity* other = context_.resolve(D->getCond());
    arboretum_create_edge(obj, context_.data_model_.getCond12, other);
  }
  //getLHS
  {
    const Entity* other = context_.resolve(D->getLHS());
    arboretum_create_edge(obj, context_.data_model_.getLHS8, other);
  }
  //getRHS
  {
    const Entity* other = context_.resolve(D->getRHS());
    arboretum_create_edge(obj, context_.data_model_.getRHS8, other);
  }
  //getBuiltinLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBuiltinLoc());
    arboretum_create_edge(obj, context_.data_model_.getBuiltinLoc5, other);
  }
  //getRParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc27, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc129, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc129, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCoawaitExpr(clang::CoawaitExpr* D) {
  const Entity* obj = context_.resolve(D);
  //isImplicit
  arboretum_create_edge(obj, context_.data_model_.isImplicit, context_.data_model_.arboretum_node_for(D->isImplicit()));
  return true;
}

bool ArboretumASTVisitor::VisitCompoundAssignOperator(clang::CompoundAssignOperator* D) {
  const Entity* obj = context_.resolve(D);
  //getComputationLHSType
  {
    const Entity* other = context_.resolve(D->getComputationLHSType());
    arboretum_create_edge(obj, context_.data_model_.getComputationLHSType, other);
  }
  //getComputationResultType
  {
    const Entity* other = context_.resolve(D->getComputationResultType());
    arboretum_create_edge(obj, context_.data_model_.getComputationResultType, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitCompoundLiteralExpr(clang::CompoundLiteralExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getInitializer
  {
    const Entity* other = context_.resolve(D->getInitializer());
    arboretum_create_edge(obj, context_.data_model_.getInitializer2, other);
  }
  //isFileScope
  arboretum_create_edge(obj, context_.data_model_.isFileScope, context_.data_model_.arboretum_node_for(D->isFileScope()));
  //getLParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getLParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getLParenLoc8, other);
  }
  //getTypeSourceInfo
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc69, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc69, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCompoundStmt(clang::CompoundStmt* D) {
  const Entity* obj = context_.resolve(D);
  //body_empty
  arboretum_create_edge(obj, context_.data_model_.body_empty, context_.data_model_.arboretum_node_for(D->body_empty()));
  //size
  // unsigned int
  //hasStoredFPFeatures
  arboretum_create_edge(obj, context_.data_model_.hasStoredFPFeatures, context_.data_model_.arboretum_node_for(D->hasStoredFPFeatures()));
  //getStoredFPFeatures
  // FPOptionsOverride
  //body
  // body_const_range
  //body_front
  {
    const Entity* other = context_.resolve(D->body_front());
    arboretum_create_edge(obj, context_.data_model_.body_front1, other);
  }
  //body_back
  {
    const Entity* other = context_.resolve(D->body_back());
    arboretum_create_edge(obj, context_.data_model_.body_back1, other);
  }
  //getStmtExprResult
  {
    const Entity* other = context_.resolve(D->getStmtExprResult());
    arboretum_create_edge(obj, context_.data_model_.getStmtExprResult1, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc37, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc37, other);
  }
  //getLBracLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getLBracLoc());
    arboretum_create_edge(obj, context_.data_model_.getLBracLoc, other);
  }
  //getRBracLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRBracLoc());
    arboretum_create_edge(obj, context_.data_model_.getRBracLoc, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitConceptSpecializationExpr(clang::ConceptSpecializationExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getTemplateArguments
  // ArrayRef<TemplateArgument>
  //getConceptReference
  //getNamedConcept
  {
    const Entity* other = context_.resolve(D->getNamedConcept());
    arboretum_create_edge(obj, context_.data_model_.getNamedConcept, other);
  }
  //hasExplicitTemplateArgs
  arboretum_create_edge(obj, context_.data_model_.hasExplicitTemplateArgs2, context_.data_model_.arboretum_node_for(D->hasExplicitTemplateArgs()));
  //getConceptNameLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getConceptNameLoc());
    arboretum_create_edge(obj, context_.data_model_.getConceptNameLoc, other);
  }
  //getTemplateArgsAsWritten
  //getNestedNameSpecifierLoc
  // const NestedNameSpecifierLoc &
  //getTemplateKWLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getTemplateKWLoc());
    arboretum_create_edge(obj, context_.data_model_.getTemplateKWLoc, other);
  }
  //getFoundDecl
  {
    const Entity* other = context_.resolve(D->getFoundDecl());
    arboretum_create_edge(obj, context_.data_model_.getFoundDecl3, other);
  }
  //getConceptNameInfo
  // const DeclarationNameInfo &
  //getSpecializationDecl
  {
    const Entity* other = context_.resolve(D->getSpecializationDecl());
    arboretum_create_edge(obj, context_.data_model_.getSpecializationDecl, other);
  }
  //getSatisfaction
  // const ASTConstraintSatisfaction &
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc51, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc51, other);
  }
  //getExprLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getExprLoc());
    arboretum_create_edge(obj, context_.data_model_.getExprLoc4, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitConditionalOperator(clang::ConditionalOperator* D) {
  const Entity* obj = context_.resolve(D);
  //getCond
  {
    const Entity* other = context_.resolve(D->getCond());
    arboretum_create_edge(obj, context_.data_model_.getCond9, other);
  }
  //getTrueExpr
  {
    const Entity* other = context_.resolve(D->getTrueExpr());
    arboretum_create_edge(obj, context_.data_model_.getTrueExpr2, other);
  }
  //getFalseExpr
  {
    const Entity* other = context_.resolve(D->getFalseExpr());
    arboretum_create_edge(obj, context_.data_model_.getFalseExpr2, other);
  }
  //getLHS
  {
    const Entity* other = context_.resolve(D->getLHS());
    arboretum_create_edge(obj, context_.data_model_.getLHS4, other);
  }
  //getRHS
  {
    const Entity* other = context_.resolve(D->getRHS());
    arboretum_create_edge(obj, context_.data_model_.getRHS4, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc87, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc87, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitConstantExpr(clang::ConstantExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc30, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc30, other);
  }
  //getResultAPValueKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getResultAPValueKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getResultAPValueKind, enum_value);
    }
  }
  //getResultStorageKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getResultStorageKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getResultStorageKind, enum_value);
    }
  }
  //isImmediateInvocation
  arboretum_create_edge(obj, context_.data_model_.isImmediateInvocation, context_.data_model_.arboretum_node_for(D->isImmediateInvocation()));
  //hasAPValueResult
  arboretum_create_edge(obj, context_.data_model_.hasAPValueResult, context_.data_model_.arboretum_node_for(D->hasAPValueResult()));
  //getAPValueResult
  // APValue
  //getResultAsAPSInt
  // llvm::APSInt
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitContinueStmt(clang::ContinueStmt* D) {
  const Entity* obj = context_.resolve(D);
  //getContinueLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getContinueLoc());
    arboretum_create_edge(obj, context_.data_model_.getContinueLoc, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc32, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc32, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitConvertVectorExpr(clang::ConvertVectorExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getSrcExpr
  {
    const Entity* other = context_.resolve(D->getSrcExpr());
    arboretum_create_edge(obj, context_.data_model_.getSrcExpr1, other);
  }
  //getTypeSourceInfo
  //getBuiltinLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBuiltinLoc());
    arboretum_create_edge(obj, context_.data_model_.getBuiltinLoc3, other);
  }
  //getRParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc14, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc86, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc86, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCoreturnStmt(clang::CoreturnStmt* D) {
  const Entity* obj = context_.resolve(D);
  //getKeywordLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getKeywordLoc());
    arboretum_create_edge(obj, context_.data_model_.getKeywordLoc1, other);
  }
  //getOperand
  {
    const Entity* other = context_.resolve(D->getOperand());
    arboretum_create_edge(obj, context_.data_model_.getOperand2, other);
  }
  //getPromiseCall
  {
    const Entity* other = context_.resolve(D->getPromiseCall());
    arboretum_create_edge(obj, context_.data_model_.getPromiseCall, other);
  }
  //isImplicit
  arboretum_create_edge(obj, context_.data_model_.isImplicit5, context_.data_model_.arboretum_node_for(D->isImplicit()));
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc138, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc138, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCoroutineBodyStmt(clang::CoroutineBodyStmt* D) {
  const Entity* obj = context_.resolve(D);
  //hasDependentPromiseType
  arboretum_create_edge(obj, context_.data_model_.hasDependentPromiseType, context_.data_model_.arboretum_node_for(D->hasDependentPromiseType()));
  //getBody
  {
    const Entity* other = context_.resolve(D->getBody());
    arboretum_create_edge(obj, context_.data_model_.getBody2, other);
  }
  //getPromiseDeclStmt
  {
    const Entity* other = context_.resolve(D->getPromiseDeclStmt());
    arboretum_create_edge(obj, context_.data_model_.getPromiseDeclStmt, other);
  }
  //getPromiseDecl
  {
    const Entity* other = context_.resolve(D->getPromiseDecl());
    arboretum_create_edge(obj, context_.data_model_.getPromiseDecl, other);
  }
  //getInitSuspendStmt
  {
    const Entity* other = context_.resolve(D->getInitSuspendStmt());
    arboretum_create_edge(obj, context_.data_model_.getInitSuspendStmt, other);
  }
  //getFinalSuspendStmt
  {
    const Entity* other = context_.resolve(D->getFinalSuspendStmt());
    arboretum_create_edge(obj, context_.data_model_.getFinalSuspendStmt, other);
  }
  //getExceptionHandler
  {
    const Entity* other = context_.resolve(D->getExceptionHandler());
    arboretum_create_edge(obj, context_.data_model_.getExceptionHandler, other);
  }
  //getFallthroughHandler
  {
    const Entity* other = context_.resolve(D->getFallthroughHandler());
    arboretum_create_edge(obj, context_.data_model_.getFallthroughHandler, other);
  }
  //getAllocate
  {
    const Entity* other = context_.resolve(D->getAllocate());
    arboretum_create_edge(obj, context_.data_model_.getAllocate, other);
  }
  //getDeallocate
  {
    const Entity* other = context_.resolve(D->getDeallocate());
    arboretum_create_edge(obj, context_.data_model_.getDeallocate, other);
  }
  //getResultDecl
  {
    const Entity* other = context_.resolve(D->getResultDecl());
    arboretum_create_edge(obj, context_.data_model_.getResultDecl, other);
  }
  //getReturnValueInit
  {
    const Entity* other = context_.resolve(D->getReturnValueInit());
    arboretum_create_edge(obj, context_.data_model_.getReturnValueInit, other);
  }
  //getReturnValue
  {
    const Entity* other = context_.resolve(D->getReturnValue());
    arboretum_create_edge(obj, context_.data_model_.getReturnValue, other);
  }
  //getReturnStmt
  {
    const Entity* other = context_.resolve(D->getReturnStmt());
    arboretum_create_edge(obj, context_.data_model_.getReturnStmt, other);
  }
  //getReturnStmtOnAllocFailure
  {
    const Entity* other = context_.resolve(D->getReturnStmtOnAllocFailure());
    arboretum_create_edge(obj, context_.data_model_.getReturnStmtOnAllocFailure, other);
  }
  //getParamMoves
  // ArrayRef<const Stmt *>
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc11, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc11, other);
  }
  //children
  // const_child_range
  //childrenExclBody
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCoroutineSuspendExpr(clang::CoroutineSuspendExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getCommonExpr
  {
    const Entity* other = context_.resolve(D->getCommonExpr());
    arboretum_create_edge(obj, context_.data_model_.getCommonExpr1, other);
  }
  //getOpaqueValue
  {
    const Entity* other = context_.resolve(D->getOpaqueValue());
    arboretum_create_edge(obj, context_.data_model_.getOpaqueValue1, other);
  }
  //getReadyExpr
  {
    const Entity* other = context_.resolve(D->getReadyExpr());
    arboretum_create_edge(obj, context_.data_model_.getReadyExpr, other);
  }
  //getSuspendExpr
  {
    const Entity* other = context_.resolve(D->getSuspendExpr());
    arboretum_create_edge(obj, context_.data_model_.getSuspendExpr, other);
  }
  //getResumeExpr
  {
    const Entity* other = context_.resolve(D->getResumeExpr());
    arboretum_create_edge(obj, context_.data_model_.getResumeExpr, other);
  }
  //getOperand
  {
    const Entity* other = context_.resolve(D->getOperand());
    arboretum_create_edge(obj, context_.data_model_.getOperand3, other);
  }
  //getKeywordLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getKeywordLoc());
    arboretum_create_edge(obj, context_.data_model_.getKeywordLoc4, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc162, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc160, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitCoyieldExpr(clang::CoyieldExpr* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitDeclRefExpr(clang::DeclRefExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getDecl
  {
    const Entity* other = context_.resolve(D->getDecl());
    arboretum_create_edge(obj, context_.data_model_.getDecl4, other);
  }
  //getNameInfo
  // DeclarationNameInfo
  //getLocation
  {
    const Entity* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.getLocation3, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc34, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc34, other);
  }
  //hasQualifier
  arboretum_create_edge(obj, context_.data_model_.hasQualifier1, context_.data_model_.arboretum_node_for(D->hasQualifier()));
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getQualifier
  //getFoundDecl
  {
    const Entity* other = context_.resolve(D->getFoundDecl());
    arboretum_create_edge(obj, context_.data_model_.getFoundDecl2, other);
  }
  //hasTemplateKWAndArgsInfo
  arboretum_create_edge(obj, context_.data_model_.hasTemplateKWAndArgsInfo1, context_.data_model_.arboretum_node_for(D->hasTemplateKWAndArgsInfo()));
  //getTemplateKeywordLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getTemplateKeywordLoc());
    arboretum_create_edge(obj, context_.data_model_.getTemplateKeywordLoc1, other);
  }
  //getLAngleLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getLAngleLoc());
    arboretum_create_edge(obj, context_.data_model_.getLAngleLoc1, other);
  }
  //getRAngleLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRAngleLoc());
    arboretum_create_edge(obj, context_.data_model_.getRAngleLoc1, other);
  }
  //hasTemplateKeyword
  arboretum_create_edge(obj, context_.data_model_.hasTemplateKeyword1, context_.data_model_.arboretum_node_for(D->hasTemplateKeyword()));
  //hasExplicitTemplateArgs
  arboretum_create_edge(obj, context_.data_model_.hasExplicitTemplateArgs1, context_.data_model_.arboretum_node_for(D->hasExplicitTemplateArgs()));
  //getTemplateArgs
  //getNumTemplateArgs
  // unsigned int
  //template_arguments
  // ArrayRef<TemplateArgumentLoc>
  //hadMultipleCandidates
  arboretum_create_edge(obj, context_.data_model_.hadMultipleCandidates1, context_.data_model_.arboretum_node_for(D->hadMultipleCandidates()));
  //isNonOdrUse
  {
    const Entity* enum_value = context_.data_model_.resolve(D->isNonOdrUse());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.isNonOdrUse1, enum_value);
    }
  }
  //refersToEnclosingVariableOrCapture
  arboretum_create_edge(obj, context_.data_model_.refersToEnclosingVariableOrCapture, context_.data_model_.arboretum_node_for(D->refersToEnclosingVariableOrCapture()));
  //isImmediateEscalating
  arboretum_create_edge(obj, context_.data_model_.isImmediateEscalating, context_.data_model_.arboretum_node_for(D->isImmediateEscalating()));
  //isCapturedByCopyInLambdaWithExplicitObjectParameter
  arboretum_create_edge(obj, context_.data_model_.isCapturedByCopyInLambdaWithExplicitObjectParameter, context_.data_model_.arboretum_node_for(D->isCapturedByCopyInLambdaWithExplicitObjectParameter()));
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitDeclStmt(clang::DeclStmt* D) {
  const Entity* obj = context_.resolve(D);
  //isSingleDecl
  arboretum_create_edge(obj, context_.data_model_.isSingleDecl, context_.data_model_.arboretum_node_for(D->isSingleDecl()));
  //getDeclGroup
  // const DeclGroupRef
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc105, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc105, other);
  }
  //children
  // const_child_range
  //decls
  // decl_const_range
  return true;
}

bool ArboretumASTVisitor::VisitDefaultStmt(clang::DefaultStmt* D) {
  const Entity* obj = context_.resolve(D);
  //getSubStmt
  {
    const Entity* other = context_.resolve(D->getSubStmt());
    arboretum_create_edge(obj, context_.data_model_.getSubStmt7, other);
  }
  //getDefaultLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getDefaultLoc());
    arboretum_create_edge(obj, context_.data_model_.getDefaultLoc, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc103, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc103, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitDependentCoawaitExpr(clang::DependentCoawaitExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getOperand
  {
    const Entity* other = context_.resolve(D->getOperand());
    arboretum_create_edge(obj, context_.data_model_.getOperand, other);
  }
  //getOperatorCoawaitLookup
  {
    const Entity* other = context_.resolve(D->getOperatorCoawaitLookup());
    arboretum_create_edge(obj, context_.data_model_.getOperatorCoawaitLookup, other);
  }
  //getKeywordLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getKeywordLoc());
    arboretum_create_edge(obj, context_.data_model_.getKeywordLoc, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc58, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc58, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitDependentScopeDeclRefExpr(clang::DependentScopeDeclRefExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getNameInfo
  // const DeclarationNameInfo &
  //getDeclName
  // DeclarationName
  //getLocation
  {
    const Entity* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.getLocation20, other);
  }
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getQualifier
  //getTemplateKeywordLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getTemplateKeywordLoc());
    arboretum_create_edge(obj, context_.data_model_.getTemplateKeywordLoc8, other);
  }
  //getLAngleLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getLAngleLoc());
    arboretum_create_edge(obj, context_.data_model_.getLAngleLoc6, other);
  }
  //getRAngleLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRAngleLoc());
    arboretum_create_edge(obj, context_.data_model_.getRAngleLoc6, other);
  }
  //hasTemplateKeyword
  arboretum_create_edge(obj, context_.data_model_.hasTemplateKeyword4, context_.data_model_.arboretum_node_for(D->hasTemplateKeyword()));
  //hasExplicitTemplateArgs
  arboretum_create_edge(obj, context_.data_model_.hasExplicitTemplateArgs5, context_.data_model_.arboretum_node_for(D->hasExplicitTemplateArgs()));
  //getTemplateArgs
  //getNumTemplateArgs
  // unsigned int
  //template_arguments
  // ArrayRef<TemplateArgumentLoc>
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc160, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc158, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitDesignatedInitExpr(clang::DesignatedInitExpr* D) {
  const Entity* obj = context_.resolve(D);
  //size
  // unsigned int
  //designators
  // llvm::ArrayRef<Designator>
  //getEqualOrColonLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEqualOrColonLoc());
    arboretum_create_edge(obj, context_.data_model_.getEqualOrColonLoc, other);
  }
  //isDirectInit
  arboretum_create_edge(obj, context_.data_model_.isDirectInit1, context_.data_model_.arboretum_node_for(D->isDirectInit()));
  //usesGNUSyntax
  arboretum_create_edge(obj, context_.data_model_.usesGNUSyntax, context_.data_model_.arboretum_node_for(D->usesGNUSyntax()));
  //getInit
  {
    const Entity* other = context_.resolve(D->getInit());
    arboretum_create_edge(obj, context_.data_model_.getInit12, other);
  }
  //getNumSubExprs
  // unsigned int
  //getDesignatorsSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getDesignatorsSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getDesignatorsSourceRange, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc141, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc141, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitDesignatedInitUpdateExpr(clang::DesignatedInitUpdateExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc101, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc101, other);
  }
  //getBase
  {
    const Entity* other = context_.resolve(D->getBase());
    arboretum_create_edge(obj, context_.data_model_.getBase17, other);
  }
  //getUpdater
  {
    const Entity* other = context_.resolve(D->getUpdater());
    arboretum_create_edge(obj, context_.data_model_.getUpdater, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitDoStmt(clang::DoStmt* D) {
  const Entity* obj = context_.resolve(D);
  //getCond
  {
    const Entity* other = context_.resolve(D->getCond());
    arboretum_create_edge(obj, context_.data_model_.getCond14, other);
  }
  //getBody
  {
    const Entity* other = context_.resolve(D->getBody());
    arboretum_create_edge(obj, context_.data_model_.getBody19, other);
  }
  //getDoLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getDoLoc());
    arboretum_create_edge(obj, context_.data_model_.getDoLoc, other);
  }
  //getWhileLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getWhileLoc());
    arboretum_create_edge(obj, context_.data_model_.getWhileLoc1, other);
  }
  //getRParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc29, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc135, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc135, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitExplicitCastExpr(clang::ExplicitCastExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getTypeInfoAsWritten
  //getTypeAsWritten
  {
    const Entity* other = context_.resolve(D->getTypeAsWritten());
    arboretum_create_edge(obj, context_.data_model_.getTypeAsWritten, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitExpr(clang::Expr* D) {
  const Entity* obj = context_.resolve(D);
  //getType
  {
    const Entity* other = context_.resolve(D->getType());
    arboretum_create_edge(obj, context_.data_model_.getType, other);
  }
  //getDependence
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getDependence());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getDependence, enum_value);
    }
  }
  //isValueDependent
  arboretum_create_edge(obj, context_.data_model_.isValueDependent, context_.data_model_.arboretum_node_for(D->isValueDependent()));
  //isTypeDependent
  arboretum_create_edge(obj, context_.data_model_.isTypeDependent, context_.data_model_.arboretum_node_for(D->isTypeDependent()));
  //isInstantiationDependent
  arboretum_create_edge(obj, context_.data_model_.isInstantiationDependent, context_.data_model_.arboretum_node_for(D->isInstantiationDependent()));
  //containsUnexpandedParameterPack
  arboretum_create_edge(obj, context_.data_model_.containsUnexpandedParameterPack, context_.data_model_.arboretum_node_for(D->containsUnexpandedParameterPack()));
  //containsErrors
  arboretum_create_edge(obj, context_.data_model_.containsErrors, context_.data_model_.arboretum_node_for(D->containsErrors()));
  //isLValue
  arboretum_create_edge(obj, context_.data_model_.isLValue, context_.data_model_.arboretum_node_for(D->isLValue()));
  //isPRValue
  arboretum_create_edge(obj, context_.data_model_.isPRValue, context_.data_model_.arboretum_node_for(D->isPRValue()));
  //isXValue
  arboretum_create_edge(obj, context_.data_model_.isXValue, context_.data_model_.arboretum_node_for(D->isXValue()));
  //isGLValue
  arboretum_create_edge(obj, context_.data_model_.isGLValue, context_.data_model_.arboretum_node_for(D->isGLValue()));
  //getValueKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getValueKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getValueKind, enum_value);
    }
  }
  //getObjectKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getObjectKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getObjectKind, enum_value);
    }
  }
  //isOrdinaryOrBitFieldObject
  arboretum_create_edge(obj, context_.data_model_.isOrdinaryOrBitFieldObject, context_.data_model_.arboretum_node_for(D->isOrdinaryOrBitFieldObject()));
  //refersToBitField
  arboretum_create_edge(obj, context_.data_model_.refersToBitField, context_.data_model_.arboretum_node_for(D->refersToBitField()));
  //getSourceBitField
  {
    const Entity* other = context_.resolve(D->getSourceBitField());
    arboretum_create_edge(obj, context_.data_model_.getSourceBitField1, other);
  }
  //getReferencedDeclOfCallee
  {
    const Entity* other = context_.resolve(D->getReferencedDeclOfCallee());
    arboretum_create_edge(obj, context_.data_model_.getReferencedDeclOfCallee1, other);
  }
  //getObjCProperty
  {
    const Entity* other = context_.resolve(D->getObjCProperty());
    arboretum_create_edge(obj, context_.data_model_.getObjCProperty, other);
  }
  //isObjCSelfExpr
  arboretum_create_edge(obj, context_.data_model_.isObjCSelfExpr, context_.data_model_.arboretum_node_for(D->isObjCSelfExpr()));
  //refersToVectorElement
  arboretum_create_edge(obj, context_.data_model_.refersToVectorElement, context_.data_model_.arboretum_node_for(D->refersToVectorElement()));
  //refersToMatrixElement
  arboretum_create_edge(obj, context_.data_model_.refersToMatrixElement, context_.data_model_.arboretum_node_for(D->refersToMatrixElement()));
  //refersToGlobalRegisterVar
  arboretum_create_edge(obj, context_.data_model_.refersToGlobalRegisterVar, context_.data_model_.arboretum_node_for(D->refersToGlobalRegisterVar()));
  //IgnoreImpCasts
  {
    const Entity* other = context_.resolve(D->IgnoreImpCasts());
    arboretum_create_edge(obj, context_.data_model_.IgnoreImpCasts1, other);
  }
  //IgnoreCasts
  {
    const Entity* other = context_.resolve(D->IgnoreCasts());
    arboretum_create_edge(obj, context_.data_model_.IgnoreCasts1, other);
  }
  //IgnoreImplicit
  {
    const Entity* other = context_.resolve(D->IgnoreImplicit());
    arboretum_create_edge(obj, context_.data_model_.IgnoreImplicit1, other);
  }
  //IgnoreImplicitAsWritten
  {
    const Entity* other = context_.resolve(D->IgnoreImplicitAsWritten());
    arboretum_create_edge(obj, context_.data_model_.IgnoreImplicitAsWritten1, other);
  }
  //IgnoreParens
  {
    const Entity* other = context_.resolve(D->IgnoreParens());
    arboretum_create_edge(obj, context_.data_model_.IgnoreParens1, other);
  }
  //IgnoreParenImpCasts
  {
    const Entity* other = context_.resolve(D->IgnoreParenImpCasts());
    arboretum_create_edge(obj, context_.data_model_.IgnoreParenImpCasts1, other);
  }
  //IgnoreParenCasts
  {
    const Entity* other = context_.resolve(D->IgnoreParenCasts());
    arboretum_create_edge(obj, context_.data_model_.IgnoreParenCasts1, other);
  }
  //IgnoreConversionOperatorSingleStep
  {
    const Entity* other = context_.resolve(D->IgnoreConversionOperatorSingleStep());
    arboretum_create_edge(obj, context_.data_model_.IgnoreConversionOperatorSingleStep1, other);
  }
  //IgnoreParenLValueCasts
  {
    const Entity* other = context_.resolve(D->IgnoreParenLValueCasts());
    arboretum_create_edge(obj, context_.data_model_.IgnoreParenLValueCasts1, other);
  }
  //IgnoreParenBaseCasts
  {
    const Entity* other = context_.resolve(D->IgnoreParenBaseCasts());
    arboretum_create_edge(obj, context_.data_model_.IgnoreParenBaseCasts1, other);
  }
  //isDefaultArgument
  arboretum_create_edge(obj, context_.data_model_.isDefaultArgument, context_.data_model_.arboretum_node_for(D->isDefaultArgument()));
  //isImplicitCXXThis
  arboretum_create_edge(obj, context_.data_model_.isImplicitCXXThis, context_.data_model_.arboretum_node_for(D->isImplicitCXXThis()));
  //skipRValueSubobjectAdjustments
  {
    const Entity* other = context_.resolve(D->skipRValueSubobjectAdjustments());
    arboretum_create_edge(obj, context_.data_model_.skipRValueSubobjectAdjustments1, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitExprWithCleanups(clang::ExprWithCleanups* D) {
  const Entity* obj = context_.resolve(D);
  //getObjects
  // ArrayRef<CleanupObject>
  //getNumObjects
  // unsigned int
  //cleanupsHaveSideEffects
  arboretum_create_edge(obj, context_.data_model_.cleanupsHaveSideEffects, context_.data_model_.arboretum_node_for(D->cleanupsHaveSideEffects()));
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc102, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc102, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitExpressionTraitExpr(clang::ExpressionTraitExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc115, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc116, other);
  }
  //getTrait
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getTrait());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getTrait2, enum_value);
    }
  }
  //getQueriedExpression
  {
    const Entity* other = context_.resolve(D->getQueriedExpression());
    arboretum_create_edge(obj, context_.data_model_.getQueriedExpression, other);
  }
  //getValue
  arboretum_create_edge(obj, context_.data_model_.getValue9, context_.data_model_.arboretum_node_for(D->getValue()));
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitExtVectorElementExpr(clang::ExtVectorElementExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getBase
  {
    const Entity* other = context_.resolve(D->getBase());
    arboretum_create_edge(obj, context_.data_model_.getBase15, other);
  }
  //getAccessor
  // IdentifierInfo &
  //getAccessorLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getAccessorLoc());
    arboretum_create_edge(obj, context_.data_model_.getAccessorLoc, other);
  }
  //getNumElements
  // unsigned int
  //containsDuplicateElements
  arboretum_create_edge(obj, context_.data_model_.containsDuplicateElements, context_.data_model_.arboretum_node_for(D->containsDuplicateElements()));
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc81, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc81, other);
  }
  //isArrow
  arboretum_create_edge(obj, context_.data_model_.isArrow5, context_.data_model_.arboretum_node_for(D->isArrow()));
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitFixedPointLiteral(clang::FixedPointLiteral* D) {
  const Entity* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc35, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc35, other);
  }
  //getLocation
  {
    const Entity* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.getLocation4, other);
  }
  //getScale
  // unsigned int
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitFloatingLiteral(clang::FloatingLiteral* D) {
  const Entity* obj = context_.resolve(D);
  //getValue
  // llvm::APFloat
  //getRawSemantics
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getRawSemantics());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getRawSemantics, enum_value);
    }
  }
  //getSemantics
  // const llvm::fltSemantics &
  //isExact
  arboretum_create_edge(obj, context_.data_model_.isExact, context_.data_model_.arboretum_node_for(D->isExact()));
  //getValueAsApproximateDouble
  // double
  //getLocation
  {
    const Entity* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.getLocation11, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc99, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc99, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitForStmt(clang::ForStmt* D) {
  const Entity* obj = context_.resolve(D);
  //getConditionVariable
  {
    const Entity* other = context_.resolve(D->getConditionVariable());
    arboretum_create_edge(obj, context_.data_model_.getConditionVariable, other);
  }
  //getConditionVariableDeclStmt
  {
    const Entity* other = context_.resolve(D->getConditionVariableDeclStmt());
    arboretum_create_edge(obj, context_.data_model_.getConditionVariableDeclStmt1, other);
  }
  //getInit
  {
    const Entity* other = context_.resolve(D->getInit());
    arboretum_create_edge(obj, context_.data_model_.getInit1, other);
  }
  //getCond
  {
    const Entity* other = context_.resolve(D->getCond());
    arboretum_create_edge(obj, context_.data_model_.getCond1, other);
  }
  //getInc
  {
    const Entity* other = context_.resolve(D->getInc());
    arboretum_create_edge(obj, context_.data_model_.getInc1, other);
  }
  //getBody
  {
    const Entity* other = context_.resolve(D->getBody());
    arboretum_create_edge(obj, context_.data_model_.getBody1, other);
  }
  //getForLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getForLoc());
    arboretum_create_edge(obj, context_.data_model_.getForLoc, other);
  }
  //getLParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getLParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getLParenLoc, other);
  }
  //getRParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc1, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc3, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc3, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitFullExpr(clang::FullExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getSubExpr
  {
    const Entity* other = context_.resolve(D->getSubExpr());
    arboretum_create_edge(obj, context_.data_model_.getSubExpr13, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitFunctionParmPackExpr(clang::FunctionParmPackExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getParameterPack
  {
    const Entity* other = context_.resolve(D->getParameterPack());
    arboretum_create_edge(obj, context_.data_model_.getParameterPack, other);
  }
  //getParameterPackLocation
  {
    const Entity* other = context_.source_model_.resolve(D->getParameterPackLocation());
    arboretum_create_edge(obj, context_.data_model_.getParameterPackLocation, other);
  }
  //getNumExpansions
  // unsigned int
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc33, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc33, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitGCCAsmStmt(clang::GCCAsmStmt* D) {
  const Entity* obj = context_.resolve(D);
  //getRParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc2, other);
  }
  //getAsmString
  {
    const Entity* other = context_.resolve(D->getAsmString());
    arboretum_create_edge(obj, context_.data_model_.getAsmString, other);
  }
  //isAsmGoto
  arboretum_create_edge(obj, context_.data_model_.isAsmGoto, context_.data_model_.arboretum_node_for(D->isAsmGoto()));
  //getNumLabels
  // unsigned int
  //labels
  // labels_const_range
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc4, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc4, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitGNUNullExpr(clang::GNUNullExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getTokenLocation
  {
    const Entity* other = context_.source_model_.resolve(D->getTokenLocation());
    arboretum_create_edge(obj, context_.data_model_.getTokenLocation, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc54, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc54, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitGenericSelectionExpr(clang::GenericSelectionExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getNumAssocs
  // unsigned int
  //getResultIndex
  // unsigned int
  //isResultDependent
  arboretum_create_edge(obj, context_.data_model_.isResultDependent, context_.data_model_.arboretum_node_for(D->isResultDependent()));
  //isExprPredicate
  arboretum_create_edge(obj, context_.data_model_.isExprPredicate, context_.data_model_.arboretum_node_for(D->isExprPredicate()));
  //isTypePredicate
  arboretum_create_edge(obj, context_.data_model_.isTypePredicate, context_.data_model_.arboretum_node_for(D->isTypePredicate()));
  //getControllingExpr
  {
    const Entity* other = context_.resolve(D->getControllingExpr());
    arboretum_create_edge(obj, context_.data_model_.getControllingExpr1, other);
  }
  //getControllingType
  //getResultExpr
  {
    const Entity* other = context_.resolve(D->getResultExpr());
    arboretum_create_edge(obj, context_.data_model_.getResultExpr3, other);
  }
  //getAssocExprs
  // ArrayRef<Expr *>
  //getAssocTypeSourceInfos
  // ArrayRef<TypeSourceInfo *>
  //associations
  // const_association_range
  //getGenericLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getGenericLoc());
    arboretum_create_edge(obj, context_.data_model_.getGenericLoc, other);
  }
  //getDefaultLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getDefaultLoc());
    arboretum_create_edge(obj, context_.data_model_.getDefaultLoc2, other);
  }
  //getRParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc28, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc134, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc134, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitGotoStmt(clang::GotoStmt* D) {
  const Entity* obj = context_.resolve(D);
  //getLabel
  {
    const Entity* other = context_.resolve(D->getLabel());
    arboretum_create_edge(obj, context_.data_model_.getLabel2, other);
  }
  //getGotoLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getGotoLoc());
    arboretum_create_edge(obj, context_.data_model_.getGotoLoc, other);
  }
  //getLabelLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getLabelLoc());
    arboretum_create_edge(obj, context_.data_model_.getLabelLoc1, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc123, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc125, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitIfStmt(clang::IfStmt* D) {
  const Entity* obj = context_.resolve(D);
  //hasInitStorage
  arboretum_create_edge(obj, context_.data_model_.hasInitStorage1, context_.data_model_.arboretum_node_for(D->hasInitStorage()));
  //hasVarStorage
  arboretum_create_edge(obj, context_.data_model_.hasVarStorage2, context_.data_model_.arboretum_node_for(D->hasVarStorage()));
  //hasElseStorage
  arboretum_create_edge(obj, context_.data_model_.hasElseStorage, context_.data_model_.arboretum_node_for(D->hasElseStorage()));
  //getCond
  {
    const Entity* other = context_.resolve(D->getCond());
    arboretum_create_edge(obj, context_.data_model_.getCond11, other);
  }
  //getThen
  {
    const Entity* other = context_.resolve(D->getThen());
    arboretum_create_edge(obj, context_.data_model_.getThen1, other);
  }
  //getElse
  {
    const Entity* other = context_.resolve(D->getElse());
    arboretum_create_edge(obj, context_.data_model_.getElse1, other);
  }
  //getConditionVariable
  {
    const Entity* other = context_.resolve(D->getConditionVariable());
    arboretum_create_edge(obj, context_.data_model_.getConditionVariable6, other);
  }
  //getConditionVariableDeclStmt
  {
    const Entity* other = context_.resolve(D->getConditionVariableDeclStmt());
    arboretum_create_edge(obj, context_.data_model_.getConditionVariableDeclStmt7, other);
  }
  //getInit
  {
    const Entity* other = context_.resolve(D->getInit());
    arboretum_create_edge(obj, context_.data_model_.getInit11, other);
  }
  //getIfLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getIfLoc());
    arboretum_create_edge(obj, context_.data_model_.getIfLoc, other);
  }
  //getElseLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getElseLoc());
    arboretum_create_edge(obj, context_.data_model_.getElseLoc, other);
  }
  //isConsteval
  arboretum_create_edge(obj, context_.data_model_.isConsteval1, context_.data_model_.arboretum_node_for(D->isConsteval()));
  //isNonNegatedConsteval
  arboretum_create_edge(obj, context_.data_model_.isNonNegatedConsteval, context_.data_model_.arboretum_node_for(D->isNonNegatedConsteval()));
  //isNegatedConsteval
  arboretum_create_edge(obj, context_.data_model_.isNegatedConsteval, context_.data_model_.arboretum_node_for(D->isNegatedConsteval()));
  //isConstexpr
  arboretum_create_edge(obj, context_.data_model_.isConstexpr2, context_.data_model_.arboretum_node_for(D->isConstexpr()));
  //getStatementKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getStatementKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getStatementKind, enum_value);
    }
  }
  //isObjCAvailabilityCheck
  arboretum_create_edge(obj, context_.data_model_.isObjCAvailabilityCheck, context_.data_model_.arboretum_node_for(D->isObjCAvailabilityCheck()));
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc117, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc119, other);
  }
  //getLParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getLParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getLParenLoc12, other);
  }
  //getRParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc24, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitImaginaryLiteral(clang::ImaginaryLiteral* D) {
  const Entity* obj = context_.resolve(D);
  //getSubExpr
  {
    const Entity* other = context_.resolve(D->getSubExpr());
    arboretum_create_edge(obj, context_.data_model_.getSubExpr15, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc110, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc111, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitImplicitCastExpr(clang::ImplicitCastExpr* D) {
  const Entity* obj = context_.resolve(D);
  //isPartOfExplicitCast
  arboretum_create_edge(obj, context_.data_model_.isPartOfExplicitCast, context_.data_model_.arboretum_node_for(D->isPartOfExplicitCast()));
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc66, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc66, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitImplicitValueInitExpr(clang::ImplicitValueInitExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc116, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc117, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitIndirectGotoStmt(clang::IndirectGotoStmt* D) {
  const Entity* obj = context_.resolve(D);
  //getGotoLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getGotoLoc());
    arboretum_create_edge(obj, context_.data_model_.getGotoLoc1, other);
  }
  //getStarLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getStarLoc());
    arboretum_create_edge(obj, context_.data_model_.getStarLoc1, other);
  }
  //getTarget
  {
    const Entity* other = context_.resolve(D->getTarget());
    arboretum_create_edge(obj, context_.data_model_.getTarget1, other);
  }
  //getConstantTarget
  {
    const Entity* other = context_.resolve(D->getConstantTarget());
    arboretum_create_edge(obj, context_.data_model_.getConstantTarget1, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc150, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc149, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitInitListExpr(clang::InitListExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getNumInits
  // unsigned int
  //inits
  // ArrayRef<Expr *>
  //getArrayFiller
  {
    const Entity* other = context_.resolve(D->getArrayFiller());
    arboretum_create_edge(obj, context_.data_model_.getArrayFiller3, other);
  }
  //hasArrayFiller
  arboretum_create_edge(obj, context_.data_model_.hasArrayFiller, context_.data_model_.arboretum_node_for(D->hasArrayFiller()));
  //hasDesignatedInit
  arboretum_create_edge(obj, context_.data_model_.hasDesignatedInit, context_.data_model_.arboretum_node_for(D->hasDesignatedInit()));
  //getInitializedFieldInUnion
  {
    const Entity* other = context_.resolve(D->getInitializedFieldInUnion());
    arboretum_create_edge(obj, context_.data_model_.getInitializedFieldInUnion3, other);
  }
  //isExplicit
  arboretum_create_edge(obj, context_.data_model_.isExplicit, context_.data_model_.arboretum_node_for(D->isExplicit()));
  //isStringLiteralInit
  arboretum_create_edge(obj, context_.data_model_.isStringLiteralInit, context_.data_model_.arboretum_node_for(D->isStringLiteralInit()));
  //isTransparent
  arboretum_create_edge(obj, context_.data_model_.isTransparent1, context_.data_model_.arboretum_node_for(D->isTransparent()));
  //getLBraceLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getLBraceLoc());
    arboretum_create_edge(obj, context_.data_model_.getLBraceLoc1, other);
  }
  //getRBraceLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRBraceLoc());
    arboretum_create_edge(obj, context_.data_model_.getRBraceLoc1, other);
  }
  //isSemanticForm
  arboretum_create_edge(obj, context_.data_model_.isSemanticForm, context_.data_model_.arboretum_node_for(D->isSemanticForm()));
  //getSemanticForm
  {
    const Entity* other = context_.resolve(D->getSemanticForm());
    arboretum_create_edge(obj, context_.data_model_.getSemanticForm2, other);
  }
  //isSyntacticForm
  arboretum_create_edge(obj, context_.data_model_.isSyntacticForm, context_.data_model_.arboretum_node_for(D->isSyntacticForm()));
  //getSyntacticForm
  {
    const Entity* other = context_.resolve(D->getSyntacticForm());
    arboretum_create_edge(obj, context_.data_model_.getSyntacticForm2, other);
  }
  //hadArrayRangeDesignator
  arboretum_create_edge(obj, context_.data_model_.hadArrayRangeDesignator, context_.data_model_.arboretum_node_for(D->hadArrayRangeDesignator()));
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc106, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc106, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitIntegerLiteral(clang::IntegerLiteral* D) {
  const Entity* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc93, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc93, other);
  }
  //getLocation
  {
    const Entity* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.getLocation9, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitLabelStmt(clang::LabelStmt* D) {
  const Entity* obj = context_.resolve(D);
  //getIdentLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getIdentLoc());
    arboretum_create_edge(obj, context_.data_model_.getIdentLoc, other);
  }
  //getDecl
  {
    const Entity* other = context_.resolve(D->getDecl());
    arboretum_create_edge(obj, context_.data_model_.getDecl2, other);
  }
  //getName
  //getSubStmt
  {
    const Entity* other = context_.resolve(D->getSubStmt());
    arboretum_create_edge(obj, context_.data_model_.getSubStmt3, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc31, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc31, other);
  }
  //children
  // const_child_range
  //isSideEntry
  arboretum_create_edge(obj, context_.data_model_.isSideEntry, context_.data_model_.arboretum_node_for(D->isSideEntry()));
  return true;
}

bool ArboretumASTVisitor::VisitLambdaExpr(clang::LambdaExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getCaptureDefault
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getCaptureDefault());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getCaptureDefault, enum_value);
    }
  }
  //getCaptureDefaultLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getCaptureDefaultLoc());
    arboretum_create_edge(obj, context_.data_model_.getCaptureDefaultLoc, other);
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
    const Entity* other = context_.source_model_.resolve(D->getIntroducerRange());
    arboretum_create_edge(obj, context_.data_model_.getIntroducerRange, other);
  }
  //getLambdaClass
  {
    const Entity* other = context_.resolve(D->getLambdaClass());
    arboretum_create_edge(obj, context_.data_model_.getLambdaClass, other);
  }
  //getCallOperator
  {
    const Entity* other = context_.resolve(D->getCallOperator());
    arboretum_create_edge(obj, context_.data_model_.getCallOperator, other);
  }
  //getDependentCallOperator
  {
    const Entity* other = context_.resolve(D->getDependentCallOperator());
    arboretum_create_edge(obj, context_.data_model_.getDependentCallOperator, other);
  }
  //getTemplateParameterList
  //getExplicitTemplateParameters
  // ArrayRef<NamedDecl *>
  //getTrailingRequiresClause
  {
    const Entity* other = context_.resolve(D->getTrailingRequiresClause());
    arboretum_create_edge(obj, context_.data_model_.getTrailingRequiresClause, other);
  }
  //isGenericLambda
  arboretum_create_edge(obj, context_.data_model_.isGenericLambda, context_.data_model_.arboretum_node_for(D->isGenericLambda()));
  //getBody
  {
    const Entity* other = context_.resolve(D->getBody());
    arboretum_create_edge(obj, context_.data_model_.getBody12, other);
  }
  //getCompoundStmtBody
  {
    const Entity* other = context_.resolve(D->getCompoundStmtBody());
    arboretum_create_edge(obj, context_.data_model_.getCompoundStmtBody, other);
  }
  //isMutable
  arboretum_create_edge(obj, context_.data_model_.isMutable, context_.data_model_.arboretum_node_for(D->isMutable()));
  //hasExplicitParameters
  arboretum_create_edge(obj, context_.data_model_.hasExplicitParameters, context_.data_model_.arboretum_node_for(D->hasExplicitParameters()));
  //hasExplicitResultType
  arboretum_create_edge(obj, context_.data_model_.hasExplicitResultType, context_.data_model_.arboretum_node_for(D->hasExplicitResultType()));
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc59, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc59, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitMSAsmStmt(clang::MSAsmStmt* D) {
  const Entity* obj = context_.resolve(D);
  //getLBraceLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getLBraceLoc());
    arboretum_create_edge(obj, context_.data_model_.getLBraceLoc, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc5, other);
  }
  //hasBraces
  arboretum_create_edge(obj, context_.data_model_.hasBraces, context_.data_model_.arboretum_node_for(D->hasBraces()));
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
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc5, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitMSDependentExistsStmt(clang::MSDependentExistsStmt* D) {
  const Entity* obj = context_.resolve(D);
  //getKeywordLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getKeywordLoc());
    arboretum_create_edge(obj, context_.data_model_.getKeywordLoc3, other);
  }
  //isIfExists
  arboretum_create_edge(obj, context_.data_model_.isIfExists, context_.data_model_.arboretum_node_for(D->isIfExists()));
  //isIfNotExists
  arboretum_create_edge(obj, context_.data_model_.isIfNotExists, context_.data_model_.arboretum_node_for(D->isIfNotExists()));
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getNameInfo
  // DeclarationNameInfo
  //getSubStmt
  {
    const Entity* other = context_.resolve(D->getSubStmt());
    arboretum_create_edge(obj, context_.data_model_.getSubStmt14, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc161, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc159, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitMSPropertyRefExpr(clang::MSPropertyRefExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getSourceRange
  {
    const Entity* other = context_.source_model_.resolve(D->getSourceRange());
    arboretum_create_edge(obj, context_.data_model_.getSourceRange52, other);
  }
  //isImplicitAccess
  arboretum_create_edge(obj, context_.data_model_.isImplicitAccess3, context_.data_model_.arboretum_node_for(D->isImplicitAccess()));
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc147, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc146, other);
  }
  //children
  // const_child_range
  //getBaseExpr
  {
    const Entity* other = context_.resolve(D->getBaseExpr());
    arboretum_create_edge(obj, context_.data_model_.getBaseExpr1, other);
  }
  //getPropertyDecl
  {
    const Entity* other = context_.resolve(D->getPropertyDecl());
    arboretum_create_edge(obj, context_.data_model_.getPropertyDecl, other);
  }
  //isArrow
  arboretum_create_edge(obj, context_.data_model_.isArrow7, context_.data_model_.arboretum_node_for(D->isArrow()));
  //getMemberLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getMemberLoc());
    arboretum_create_edge(obj, context_.data_model_.getMemberLoc3, other);
  }
  //getQualifierLoc
  // NestedNameSpecifierLoc
  return true;
}

bool ArboretumASTVisitor::VisitMSPropertySubscriptExpr(clang::MSPropertySubscriptExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getBase
  {
    const Entity* other = context_.resolve(D->getBase());
    arboretum_create_edge(obj, context_.data_model_.getBase12, other);
  }
  //getIdx
  {
    const Entity* other = context_.resolve(D->getIdx());
    arboretum_create_edge(obj, context_.data_model_.getIdx1, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc73, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc73, other);
  }
  //getRBracketLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRBracketLoc());
    arboretum_create_edge(obj, context_.data_model_.getRBracketLoc1, other);
  }
  //getExprLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getExprLoc());
    arboretum_create_edge(obj, context_.data_model_.getExprLoc7, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitMaterializeTemporaryExpr(clang::MaterializeTemporaryExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getSubExpr
  {
    const Entity* other = context_.resolve(D->getSubExpr());
    arboretum_create_edge(obj, context_.data_model_.getSubExpr6, other);
  }
  //getStorageDuration
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getStorageDuration());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getStorageDuration, enum_value);
    }
  }
  //getLifetimeExtendedTemporaryDecl
  {
    const Entity* other = context_.resolve(D->getLifetimeExtendedTemporaryDecl());
    arboretum_create_edge(obj, context_.data_model_.getLifetimeExtendedTemporaryDecl1, other);
  }
  //getExtendingDecl
  {
    const Entity* other = context_.resolve(D->getExtendingDecl());
    arboretum_create_edge(obj, context_.data_model_.getExtendingDecl1, other);
  }
  //getManglingNumber
  // unsigned int
  //isBoundToLvalueReference
  arboretum_create_edge(obj, context_.data_model_.isBoundToLvalueReference, context_.data_model_.arboretum_node_for(D->isBoundToLvalueReference()));
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc60, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc60, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitMatrixSubscriptExpr(clang::MatrixSubscriptExpr* D) {
  const Entity* obj = context_.resolve(D);
  //isIncomplete
  arboretum_create_edge(obj, context_.data_model_.isIncomplete, context_.data_model_.arboretum_node_for(D->isIncomplete()));
  //getBase
  {
    const Entity* other = context_.resolve(D->getBase());
    arboretum_create_edge(obj, context_.data_model_.getBase14, other);
  }
  //getRowIdx
  {
    const Entity* other = context_.resolve(D->getRowIdx());
    arboretum_create_edge(obj, context_.data_model_.getRowIdx1, other);
  }
  //getColumnIdx
  {
    const Entity* other = context_.resolve(D->getColumnIdx());
    arboretum_create_edge(obj, context_.data_model_.getColumnIdx1, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc75, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc75, other);
  }
  //getExprLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getExprLoc());
    arboretum_create_edge(obj, context_.data_model_.getExprLoc9, other);
  }
  //getRBracketLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRBracketLoc());
    arboretum_create_edge(obj, context_.data_model_.getRBracketLoc2, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitMemberExpr(clang::MemberExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getBase
  {
    const Entity* other = context_.resolve(D->getBase());
    arboretum_create_edge(obj, context_.data_model_.getBase, other);
  }
  //getMemberDecl
  {
    const Entity* other = context_.resolve(D->getMemberDecl());
    arboretum_create_edge(obj, context_.data_model_.getMemberDecl, other);
  }
  //getFoundDecl
  // DeclAccessPair
  //hasQualifier
  arboretum_create_edge(obj, context_.data_model_.hasQualifier, context_.data_model_.arboretum_node_for(D->hasQualifier()));
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getQualifier
  //getTemplateKeywordLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getTemplateKeywordLoc());
    arboretum_create_edge(obj, context_.data_model_.getTemplateKeywordLoc, other);
  }
  //getLAngleLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getLAngleLoc());
    arboretum_create_edge(obj, context_.data_model_.getLAngleLoc, other);
  }
  //getRAngleLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRAngleLoc());
    arboretum_create_edge(obj, context_.data_model_.getRAngleLoc, other);
  }
  //hasTemplateKeyword
  arboretum_create_edge(obj, context_.data_model_.hasTemplateKeyword, context_.data_model_.arboretum_node_for(D->hasTemplateKeyword()));
  //hasExplicitTemplateArgs
  arboretum_create_edge(obj, context_.data_model_.hasExplicitTemplateArgs, context_.data_model_.arboretum_node_for(D->hasExplicitTemplateArgs()));
  //getTemplateArgs
  //getNumTemplateArgs
  // unsigned int
  //template_arguments
  // ArrayRef<TemplateArgumentLoc>
  //getMemberNameInfo
  // DeclarationNameInfo
  //getOperatorLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getOperatorLoc());
    arboretum_create_edge(obj, context_.data_model_.getOperatorLoc, other);
  }
  //isArrow
  arboretum_create_edge(obj, context_.data_model_.isArrow, context_.data_model_.arboretum_node_for(D->isArrow()));
  //getMemberLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getMemberLoc());
    arboretum_create_edge(obj, context_.data_model_.getMemberLoc, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc9, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc9, other);
  }
  //getExprLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getExprLoc());
    arboretum_create_edge(obj, context_.data_model_.getExprLoc1, other);
  }
  //isImplicitAccess
  arboretum_create_edge(obj, context_.data_model_.isImplicitAccess, context_.data_model_.arboretum_node_for(D->isImplicitAccess()));
  //hadMultipleCandidates
  arboretum_create_edge(obj, context_.data_model_.hadMultipleCandidates, context_.data_model_.arboretum_node_for(D->hadMultipleCandidates()));
  //isNonOdrUse
  {
    const Entity* enum_value = context_.data_model_.resolve(D->isNonOdrUse());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.isNonOdrUse, enum_value);
    }
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitNoInitExpr(clang::NoInitExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc83, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc83, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitNullStmt(clang::NullStmt* D) {
  const Entity* obj = context_.resolve(D);
  //getSemiLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getSemiLoc());
    arboretum_create_edge(obj, context_.data_model_.getSemiLoc, other);
  }
  //hasLeadingEmptyMacro
  arboretum_create_edge(obj, context_.data_model_.hasLeadingEmptyMacro, context_.data_model_.arboretum_node_for(D->hasLeadingEmptyMacro()));
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc118, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc120, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitOMPArraySectionExpr(clang::OMPArraySectionExpr* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPArrayShapingExpr(clang::OMPArrayShapingExpr* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPAtomicDirective(clang::OMPAtomicDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPBarrierDirective(clang::OMPBarrierDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPCancelDirective(clang::OMPCancelDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPCancellationPointDirective(clang::OMPCancellationPointDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPCanonicalLoop(clang::OMPCanonicalLoop* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPCriticalDirective(clang::OMPCriticalDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPDepobjDirective(clang::OMPDepobjDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPDispatchDirective(clang::OMPDispatchDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPDistributeDirective(clang::OMPDistributeDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPDistributeParallelForDirective(clang::OMPDistributeParallelForDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPDistributeParallelForSimdDirective(clang::OMPDistributeParallelForSimdDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPDistributeSimdDirective(clang::OMPDistributeSimdDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPErrorDirective(clang::OMPErrorDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPExecutableDirective(clang::OMPExecutableDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPFlushDirective(clang::OMPFlushDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPForDirective(clang::OMPForDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPForSimdDirective(clang::OMPForSimdDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPGenericLoopDirective(clang::OMPGenericLoopDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPInteropDirective(clang::OMPInteropDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPIteratorExpr(clang::OMPIteratorExpr* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPLoopBasedDirective(clang::OMPLoopBasedDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPLoopDirective(clang::OMPLoopDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPLoopTransformationDirective(clang::OMPLoopTransformationDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPMaskedDirective(clang::OMPMaskedDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPMaskedTaskLoopDirective(clang::OMPMaskedTaskLoopDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPMaskedTaskLoopSimdDirective(clang::OMPMaskedTaskLoopSimdDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPMasterDirective(clang::OMPMasterDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPMasterTaskLoopDirective(clang::OMPMasterTaskLoopDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPMasterTaskLoopSimdDirective(clang::OMPMasterTaskLoopSimdDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPMetaDirective(clang::OMPMetaDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPOrderedDirective(clang::OMPOrderedDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelDirective(clang::OMPParallelDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelForDirective(clang::OMPParallelForDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelForSimdDirective(clang::OMPParallelForSimdDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelGenericLoopDirective(clang::OMPParallelGenericLoopDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelMaskedDirective(clang::OMPParallelMaskedDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelMaskedTaskLoopDirective(clang::OMPParallelMaskedTaskLoopDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelMaskedTaskLoopSimdDirective(clang::OMPParallelMaskedTaskLoopSimdDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelMasterDirective(clang::OMPParallelMasterDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelMasterTaskLoopDirective(clang::OMPParallelMasterTaskLoopDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelMasterTaskLoopSimdDirective(clang::OMPParallelMasterTaskLoopSimdDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelSectionsDirective(clang::OMPParallelSectionsDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPScanDirective(clang::OMPScanDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPScopeDirective(clang::OMPScopeDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPSectionDirective(clang::OMPSectionDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPSectionsDirective(clang::OMPSectionsDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPSimdDirective(clang::OMPSimdDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPSingleDirective(clang::OMPSingleDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetDataDirective(clang::OMPTargetDataDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetDirective(clang::OMPTargetDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetEnterDataDirective(clang::OMPTargetEnterDataDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetExitDataDirective(clang::OMPTargetExitDataDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetParallelDirective(clang::OMPTargetParallelDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetParallelForDirective(clang::OMPTargetParallelForDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetParallelForSimdDirective(clang::OMPTargetParallelForSimdDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetParallelGenericLoopDirective(clang::OMPTargetParallelGenericLoopDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetSimdDirective(clang::OMPTargetSimdDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetTeamsDirective(clang::OMPTargetTeamsDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetTeamsDistributeDirective(clang::OMPTargetTeamsDistributeDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetTeamsDistributeParallelForDirective(clang::OMPTargetTeamsDistributeParallelForDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetTeamsDistributeParallelForSimdDirective(clang::OMPTargetTeamsDistributeParallelForSimdDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetTeamsDistributeSimdDirective(clang::OMPTargetTeamsDistributeSimdDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetTeamsGenericLoopDirective(clang::OMPTargetTeamsGenericLoopDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetUpdateDirective(clang::OMPTargetUpdateDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTaskDirective(clang::OMPTaskDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTaskLoopDirective(clang::OMPTaskLoopDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTaskLoopSimdDirective(clang::OMPTaskLoopSimdDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTaskgroupDirective(clang::OMPTaskgroupDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTaskwaitDirective(clang::OMPTaskwaitDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTaskyieldDirective(clang::OMPTaskyieldDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTeamsDirective(clang::OMPTeamsDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTeamsDistributeDirective(clang::OMPTeamsDistributeDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTeamsDistributeParallelForDirective(clang::OMPTeamsDistributeParallelForDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTeamsDistributeParallelForSimdDirective(clang::OMPTeamsDistributeParallelForSimdDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTeamsDistributeSimdDirective(clang::OMPTeamsDistributeSimdDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTeamsGenericLoopDirective(clang::OMPTeamsGenericLoopDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTileDirective(clang::OMPTileDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPUnrollDirective(clang::OMPUnrollDirective* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCArrayLiteral(clang::ObjCArrayLiteral* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCAtCatchStmt(clang::ObjCAtCatchStmt* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCAtFinallyStmt(clang::ObjCAtFinallyStmt* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCAtSynchronizedStmt(clang::ObjCAtSynchronizedStmt* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCAtThrowStmt(clang::ObjCAtThrowStmt* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCAtTryStmt(clang::ObjCAtTryStmt* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCAutoreleasePoolStmt(clang::ObjCAutoreleasePoolStmt* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCAvailabilityCheckExpr(clang::ObjCAvailabilityCheckExpr* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCBoolLiteralExpr(clang::ObjCBoolLiteralExpr* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCBoxedExpr(clang::ObjCBoxedExpr* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCBridgedCastExpr(clang::ObjCBridgedCastExpr* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCDictionaryLiteral(clang::ObjCDictionaryLiteral* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCEncodeExpr(clang::ObjCEncodeExpr* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCForCollectionStmt(clang::ObjCForCollectionStmt* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCIndirectCopyRestoreExpr(clang::ObjCIndirectCopyRestoreExpr* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCIsaExpr(clang::ObjCIsaExpr* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCIvarRefExpr(clang::ObjCIvarRefExpr* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCMessageExpr(clang::ObjCMessageExpr* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCPropertyRefExpr(clang::ObjCPropertyRefExpr* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCProtocolExpr(clang::ObjCProtocolExpr* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCSelectorExpr(clang::ObjCSelectorExpr* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCStringLiteral(clang::ObjCStringLiteral* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCSubscriptRefExpr(clang::ObjCSubscriptRefExpr* D) {
  const Entity* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOffsetOfExpr(clang::OffsetOfExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getOperatorLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getOperatorLoc());
    arboretum_create_edge(obj, context_.data_model_.getOperatorLoc7, other);
  }
  //getRParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc20, other);
  }
  //getTypeSourceInfo
  //getNumComponents
  // unsigned int
  //getNumExpressions
  // unsigned int
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc96, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc96, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitOpaqueValueExpr(clang::OpaqueValueExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getLocation
  {
    const Entity* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.getLocation12, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc100, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc100, other);
  }
  //getExprLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getExprLoc());
    arboretum_create_edge(obj, context_.data_model_.getExprLoc13, other);
  }
  //children
  // const_child_range
  //getSourceExpr
  {
    const Entity* other = context_.resolve(D->getSourceExpr());
    arboretum_create_edge(obj, context_.data_model_.getSourceExpr, other);
  }
  //isUnique
  arboretum_create_edge(obj, context_.data_model_.isUnique, context_.data_model_.arboretum_node_for(D->isUnique()));
  return true;
}

bool ArboretumASTVisitor::VisitOverloadExpr(clang::OverloadExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getNamingClass
  {
    const Entity* other = context_.resolve(D->getNamingClass());
    arboretum_create_edge(obj, context_.data_model_.getNamingClass3, other);
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
    const Entity* other = context_.source_model_.resolve(D->getNameLoc());
    arboretum_create_edge(obj, context_.data_model_.getNameLoc, other);
  }
  //getQualifier
  //getQualifierLoc
  // NestedNameSpecifierLoc
  //getTemplateKeywordLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getTemplateKeywordLoc());
    arboretum_create_edge(obj, context_.data_model_.getTemplateKeywordLoc3, other);
  }
  //getLAngleLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getLAngleLoc());
    arboretum_create_edge(obj, context_.data_model_.getLAngleLoc3, other);
  }
  //getRAngleLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRAngleLoc());
    arboretum_create_edge(obj, context_.data_model_.getRAngleLoc3, other);
  }
  //hasTemplateKeyword
  arboretum_create_edge(obj, context_.data_model_.hasTemplateKeyword3, context_.data_model_.arboretum_node_for(D->hasTemplateKeyword()));
  //hasExplicitTemplateArgs
  arboretum_create_edge(obj, context_.data_model_.hasExplicitTemplateArgs4, context_.data_model_.arboretum_node_for(D->hasExplicitTemplateArgs()));
  //getTemplateArgs
  //getNumTemplateArgs
  // unsigned int
  //template_arguments
  // ArrayRef<TemplateArgumentLoc>
  return true;
}

bool ArboretumASTVisitor::VisitPackExpansionExpr(clang::PackExpansionExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getPattern
  {
    const Entity* other = context_.resolve(D->getPattern());
    arboretum_create_edge(obj, context_.data_model_.getPattern2, other);
  }
  //getEllipsisLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEllipsisLoc());
    arboretum_create_edge(obj, context_.data_model_.getEllipsisLoc2, other);
  }
  //getNumExpansions
  // std::optional<unsigned int>
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc61, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc61, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitParenExpr(clang::ParenExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getSubExpr
  {
    const Entity* other = context_.resolve(D->getSubExpr());
    arboretum_create_edge(obj, context_.data_model_.getSubExpr9, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc79, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc79, other);
  }
  //getLParen
  {
    const Entity* other = context_.source_model_.resolve(D->getLParen());
    arboretum_create_edge(obj, context_.data_model_.getLParen, other);
  }
  //getRParen
  {
    const Entity* other = context_.source_model_.resolve(D->getRParen());
    arboretum_create_edge(obj, context_.data_model_.getRParen, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitParenListExpr(clang::ParenListExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getNumExprs
  // unsigned int
  //getLParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getLParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getLParenLoc9, other);
  }
  //getRParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc13, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc82, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc82, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitPredefinedExpr(clang::PredefinedExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getIdentKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getIdentKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getIdentKind, enum_value);
    }
  }
  //isTransparent
  arboretum_create_edge(obj, context_.data_model_.isTransparent, context_.data_model_.arboretum_node_for(D->isTransparent()));
  //getLocation
  {
    const Entity* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.getLocation10, other);
  }
  //getFunctionName
  {
    const Entity* other = context_.resolve(D->getFunctionName());
    arboretum_create_edge(obj, context_.data_model_.getFunctionName1, other);
  }
  //getIdentKindName
  // StringRef
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc97, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc97, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitPseudoObjectExpr(clang::PseudoObjectExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getSyntacticForm
  {
    const Entity* other = context_.resolve(D->getSyntacticForm());
    arboretum_create_edge(obj, context_.data_model_.getSyntacticForm1, other);
  }
  //getResultExprIndex
  // unsigned int
  //getResultExpr
  {
    const Entity* other = context_.resolve(D->getResultExpr());
    arboretum_create_edge(obj, context_.data_model_.getResultExpr1, other);
  }
  //getNumSemanticExprs
  // unsigned int
  //semantics
  // ArrayRef<const Expr *>
  //getExprLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getExprLoc());
    arboretum_create_edge(obj, context_.data_model_.getExprLoc10, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc78, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc78, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitRecoveryExpr(clang::RecoveryExpr* D) {
  const Entity* obj = context_.resolve(D);
  //subExpressions
  // ArrayRef<const Expr *>
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc28, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc28, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitRequiresExpr(clang::RequiresExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getLocalParameters
  // ArrayRef<ParmVarDecl *>
  //getBody
  {
    const Entity* other = context_.resolve(D->getBody());
    arboretum_create_edge(obj, context_.data_model_.getBody11, other);
  }
  //getRequirements
  // ArrayRef<concepts::Requirement *>
  //getRequiresKWLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRequiresKWLoc());
    arboretum_create_edge(obj, context_.data_model_.getRequiresKWLoc, other);
  }
  //getLParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getLParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getLParenLoc7, other);
  }
  //getRParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc10, other);
  }
  //getRBraceLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRBraceLoc());
    arboretum_create_edge(obj, context_.data_model_.getRBraceLoc, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc57, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc57, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitReturnStmt(clang::ReturnStmt* D) {
  const Entity* obj = context_.resolve(D);
  //getRetValue
  {
    const Entity* other = context_.resolve(D->getRetValue());
    arboretum_create_edge(obj, context_.data_model_.getRetValue1, other);
  }
  //getNRVOCandidate
  {
    const Entity* other = context_.resolve(D->getNRVOCandidate());
    arboretum_create_edge(obj, context_.data_model_.getNRVOCandidate, other);
  }
  //getReturnLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getReturnLoc());
    arboretum_create_edge(obj, context_.data_model_.getReturnLoc, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc40, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc40, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitSEHExceptStmt(clang::SEHExceptStmt* D) {
  const Entity* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc8, other);
  }
  //getExceptLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getExceptLoc());
    arboretum_create_edge(obj, context_.data_model_.getExceptLoc, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc8, other);
  }
  //getFilterExpr
  {
    const Entity* other = context_.resolve(D->getFilterExpr());
    arboretum_create_edge(obj, context_.data_model_.getFilterExpr, other);
  }
  //getBlock
  {
    const Entity* other = context_.resolve(D->getBlock());
    arboretum_create_edge(obj, context_.data_model_.getBlock, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitSEHFinallyStmt(clang::SEHFinallyStmt* D) {
  const Entity* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc164, other);
  }
  //getFinallyLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getFinallyLoc());
    arboretum_create_edge(obj, context_.data_model_.getFinallyLoc, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc162, other);
  }
  //getBlock
  {
    const Entity* other = context_.resolve(D->getBlock());
    arboretum_create_edge(obj, context_.data_model_.getBlock1, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitSEHLeaveStmt(clang::SEHLeaveStmt* D) {
  const Entity* obj = context_.resolve(D);
  //getLeaveLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getLeaveLoc());
    arboretum_create_edge(obj, context_.data_model_.getLeaveLoc, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc121, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc123, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitSEHTryStmt(clang::SEHTryStmt* D) {
  const Entity* obj = context_.resolve(D);
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc10, other);
  }
  //getTryLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getTryLoc());
    arboretum_create_edge(obj, context_.data_model_.getTryLoc, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc10, other);
  }
  //getIsCXXTry
  arboretum_create_edge(obj, context_.data_model_.getIsCXXTry, context_.data_model_.arboretum_node_for(D->getIsCXXTry()));
  //getTryBlock
  {
    const Entity* other = context_.resolve(D->getTryBlock());
    arboretum_create_edge(obj, context_.data_model_.getTryBlock, other);
  }
  //getHandler
  {
    const Entity* other = context_.resolve(D->getHandler());
    arboretum_create_edge(obj, context_.data_model_.getHandler, other);
  }
  //getExceptHandler
  {
    const Entity* other = context_.resolve(D->getExceptHandler());
    arboretum_create_edge(obj, context_.data_model_.getExceptHandler, other);
  }
  //getFinallyHandler
  {
    const Entity* other = context_.resolve(D->getFinallyHandler());
    arboretum_create_edge(obj, context_.data_model_.getFinallyHandler, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitSYCLUniqueStableNameExpr(clang::SYCLUniqueStableNameExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getTypeSourceInfo
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc17, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc17, other);
  }
  //getLocation
  {
    const Entity* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.getLocation1, other);
  }
  //getLParenLocation
  {
    const Entity* other = context_.source_model_.resolve(D->getLParenLocation());
    arboretum_create_edge(obj, context_.data_model_.getLParenLocation, other);
  }
  //getRParenLocation
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLocation());
    arboretum_create_edge(obj, context_.data_model_.getRParenLocation, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitShuffleVectorExpr(clang::ShuffleVectorExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getBuiltinLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBuiltinLoc());
    arboretum_create_edge(obj, context_.data_model_.getBuiltinLoc, other);
  }
  //getRParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc8, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc50, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc50, other);
  }
  //getNumSubExprs
  // unsigned int
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitSizeOfPackExpr(clang::SizeOfPackExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getOperatorLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getOperatorLoc());
    arboretum_create_edge(obj, context_.data_model_.getOperatorLoc10, other);
  }
  //getPackLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getPackLoc());
    arboretum_create_edge(obj, context_.data_model_.getPackLoc, other);
  }
  //getRParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc31, other);
  }
  //getPack
  {
    const Entity* other = context_.resolve(D->getPack());
    arboretum_create_edge(obj, context_.data_model_.getPack, other);
  }
  //getPackLength
  // unsigned int
  //isPartiallySubstituted
  arboretum_create_edge(obj, context_.data_model_.isPartiallySubstituted, context_.data_model_.arboretum_node_for(D->isPartiallySubstituted()));
  //getPartialArguments
  // ArrayRef<TemplateArgument>
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc140, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc140, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitSourceLocExpr(clang::SourceLocExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getBuiltinStr
  // StringRef
  //getIdentKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getIdentKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getIdentKind1, enum_value);
    }
  }
  //isIntType
  arboretum_create_edge(obj, context_.data_model_.isIntType, context_.data_model_.arboretum_node_for(D->isIntType()));
  //getParentContext
  //getLocation
  {
    const Entity* other = context_.source_model_.resolve(D->getLocation());
    arboretum_create_edge(obj, context_.data_model_.getLocation15, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc137, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc137, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitStmt(clang::Stmt* D) {
  const Entity* obj = context_.resolve(D);
  switch(D->getStmtClass()) {
    case clang::Stmt::ObjCArrayLiteralClass: {
assert(context_.data_model_.ObjCArrayLiteral != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCArrayLiteral); 
    } break;
    case clang::Stmt::SwitchStmtClass: {
assert(context_.data_model_.SwitchStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.SwitchStmt); 
    } break;
    case clang::Stmt::CXXStdInitializerListExprClass: {
assert(context_.data_model_.CXXStdInitializerListExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXStdInitializerListExpr); 
    } break;
    case clang::Stmt::ImplicitCastExprClass: {
assert(context_.data_model_.ImplicitCastExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ImplicitCastExpr); 
    } break;
    case clang::Stmt::ObjCSelectorExprClass: {
assert(context_.data_model_.ObjCSelectorExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCSelectorExpr); 
    } break;
    case clang::Stmt::ObjCAtTryStmtClass: {
assert(context_.data_model_.ObjCAtTryStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCAtTryStmt); 
    } break;
    case clang::Stmt::ObjCProtocolExprClass: {
assert(context_.data_model_.ObjCProtocolExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCProtocolExpr); 
    } break;
    case clang::Stmt::CXXTypeidExprClass: {
assert(context_.data_model_.CXXTypeidExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXTypeidExpr); 
    } break;
    case clang::Stmt::ObjCPropertyRefExprClass: {
assert(context_.data_model_.ObjCPropertyRefExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCPropertyRefExpr); 
    } break;
    case clang::Stmt::OMPTargetTeamsDistributeSimdDirectiveClass: {
assert(context_.data_model_.OMPTargetTeamsDistributeSimdDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTargetTeamsDistributeSimdDirective); 
    } break;
    case clang::Stmt::ObjCMessageExprClass: {
assert(context_.data_model_.ObjCMessageExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCMessageExpr); 
    } break;
    case clang::Stmt::CXXParenListInitExprClass: {
assert(context_.data_model_.CXXParenListInitExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXParenListInitExpr); 
    } break;
    case clang::Stmt::ObjCIsaExprClass: {
assert(context_.data_model_.ObjCIsaExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCIsaExpr); 
    } break;
    case clang::Stmt::ForStmtClass: {
assert(context_.data_model_.ForStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ForStmt); 
    } break;
    case clang::Stmt::OMPTaskgroupDirectiveClass: {
assert(context_.data_model_.OMPTaskgroupDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTaskgroupDirective); 
    } break;
    case clang::Stmt::IndirectGotoStmtClass: {
assert(context_.data_model_.IndirectGotoStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.IndirectGotoStmt); 
    } break;
    case clang::Stmt::OMPParallelMaskedTaskLoopSimdDirectiveClass: {
assert(context_.data_model_.OMPParallelMaskedTaskLoopSimdDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPParallelMaskedTaskLoopSimdDirective); 
    } break;
    case clang::Stmt::GCCAsmStmtClass: {
assert(context_.data_model_.GCCAsmStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.GCCAsmStmt); 
    } break;
    case clang::Stmt::TypoExprClass: {
assert(context_.data_model_.TypoExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.TypoExpr); 
    } break;
    case clang::Stmt::MSAsmStmtClass: {
assert(context_.data_model_.MSAsmStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.MSAsmStmt); 
    } break;
    case clang::Stmt::CXXBindTemporaryExprClass: {
assert(context_.data_model_.CXXBindTemporaryExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXBindTemporaryExpr); 
    } break;
    case clang::Stmt::CXXOperatorCallExprClass: {
assert(context_.data_model_.CXXOperatorCallExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXOperatorCallExpr); 
    } break;
    case clang::Stmt::SEHExceptStmtClass: {
assert(context_.data_model_.SEHExceptStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.SEHExceptStmt); 
    } break;
    case clang::Stmt::MemberExprClass: {
assert(context_.data_model_.MemberExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.MemberExpr); 
    } break;
    case clang::Stmt::SEHTryStmtClass: {
assert(context_.data_model_.SEHTryStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.SEHTryStmt); 
    } break;
    case clang::Stmt::CXXForRangeStmtClass: {
assert(context_.data_model_.CXXForRangeStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXForRangeStmt); 
    } break;
    case clang::Stmt::ObjCEncodeExprClass: {
assert(context_.data_model_.ObjCEncodeExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCEncodeExpr); 
    } break;
    case clang::Stmt::SubstNonTypeTemplateParmExprClass: {
assert(context_.data_model_.SubstNonTypeTemplateParmExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.SubstNonTypeTemplateParmExpr); 
    } break;
    case clang::Stmt::ObjCIndirectCopyRestoreExprClass: {
assert(context_.data_model_.ObjCIndirectCopyRestoreExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCIndirectCopyRestoreExpr); 
    } break;
    case clang::Stmt::CoroutineBodyStmtClass: {
assert(context_.data_model_.CoroutineBodyStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CoroutineBodyStmt); 
    } break;
    case clang::Stmt::CoreturnStmtClass: {
assert(context_.data_model_.CoreturnStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CoreturnStmt); 
    } break;
    case clang::Stmt::ArrayTypeTraitExprClass: {
assert(context_.data_model_.ArrayTypeTraitExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ArrayTypeTraitExpr); 
    } break;
    case clang::Stmt::CXXUuidofExprClass: {
assert(context_.data_model_.CXXUuidofExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXUuidofExpr); 
    } break;
    case clang::Stmt::ObjCForCollectionStmtClass: {
assert(context_.data_model_.ObjCForCollectionStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCForCollectionStmt); 
    } break;
    case clang::Stmt::ObjCAtCatchStmtClass: {
assert(context_.data_model_.ObjCAtCatchStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCAtCatchStmt); 
    } break;
    case clang::Stmt::ObjCAtSynchronizedStmtClass: {
assert(context_.data_model_.ObjCAtSynchronizedStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCAtSynchronizedStmt); 
    } break;
    case clang::Stmt::ObjCAtThrowStmtClass: {
assert(context_.data_model_.ObjCAtThrowStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCAtThrowStmt); 
    } break;
    case clang::Stmt::OMPParallelMasterDirectiveClass: {
assert(context_.data_model_.OMPParallelMasterDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPParallelMasterDirective); 
    } break;
    case clang::Stmt::ImaginaryLiteralClass: {
assert(context_.data_model_.ImaginaryLiteral != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ImaginaryLiteral); 
    } break;
    case clang::Stmt::NullStmtClass: {
assert(context_.data_model_.NullStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.NullStmt); 
    } break;
    case clang::Stmt::ObjCAutoreleasePoolStmtClass: {
assert(context_.data_model_.ObjCAutoreleasePoolStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCAutoreleasePoolStmt); 
    } break;
    case clang::Stmt::OMPCanonicalLoopClass: {
assert(context_.data_model_.OMPCanonicalLoop != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPCanonicalLoop); 
    } break;
    case clang::Stmt::BreakStmtClass: {
assert(context_.data_model_.BreakStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.BreakStmt); 
    } break;
    case clang::Stmt::OMPParallelDirectiveClass: {
assert(context_.data_model_.OMPParallelDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPParallelDirective); 
    } break;
    case clang::Stmt::OMPTileDirectiveClass: {
assert(context_.data_model_.OMPTileDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTileDirective); 
    } break;
    case clang::Stmt::MSPropertyRefExprClass: {
assert(context_.data_model_.MSPropertyRefExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.MSPropertyRefExpr); 
    } break;
    case clang::Stmt::ObjCDictionaryLiteralClass: {
assert(context_.data_model_.ObjCDictionaryLiteral != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCDictionaryLiteral); 
    } break;
    case clang::Stmt::OMPInteropDirectiveClass: {
assert(context_.data_model_.OMPInteropDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPInteropDirective); 
    } break;
    case clang::Stmt::ObjCSubscriptRefExprClass: {
assert(context_.data_model_.ObjCSubscriptRefExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCSubscriptRefExpr); 
    } break;
    case clang::Stmt::CXXBoolLiteralExprClass: {
assert(context_.data_model_.CXXBoolLiteralExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXBoolLiteralExpr); 
    } break;
    case clang::Stmt::SYCLUniqueStableNameExprClass: {
assert(context_.data_model_.SYCLUniqueStableNameExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.SYCLUniqueStableNameExpr); 
    } break;
    case clang::Stmt::OMPSimdDirectiveClass: {
assert(context_.data_model_.OMPSimdDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPSimdDirective); 
    } break;
    case clang::Stmt::OMPForSimdDirectiveClass: {
assert(context_.data_model_.OMPForSimdDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPForSimdDirective); 
    } break;
    case clang::Stmt::ObjCBoxedExprClass: {
assert(context_.data_model_.ObjCBoxedExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCBoxedExpr); 
    } break;
    case clang::Stmt::OMPParallelSectionsDirectiveClass: {
assert(context_.data_model_.OMPParallelSectionsDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPParallelSectionsDirective); 
    } break;
    case clang::Stmt::ObjCAvailabilityCheckExprClass: {
assert(context_.data_model_.ObjCAvailabilityCheckExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCAvailabilityCheckExpr); 
    } break;
    case clang::Stmt::OMPParallelForDirectiveClass: {
assert(context_.data_model_.OMPParallelForDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPParallelForDirective); 
    } break;
    case clang::Stmt::OMPTargetParallelForDirectiveClass: {
assert(context_.data_model_.OMPTargetParallelForDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTargetParallelForDirective); 
    } break;
    case clang::Stmt::OMPErrorDirectiveClass: {
assert(context_.data_model_.OMPErrorDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPErrorDirective); 
    } break;
    case clang::Stmt::OMPTaskLoopSimdDirectiveClass: {
assert(context_.data_model_.OMPTaskLoopSimdDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTaskLoopSimdDirective); 
    } break;
    case clang::Stmt::OMPMaskedTaskLoopDirectiveClass: {
assert(context_.data_model_.OMPMaskedTaskLoopDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPMaskedTaskLoopDirective); 
    } break;
    case clang::Stmt::OMPMasterTaskLoopSimdDirectiveClass: {
assert(context_.data_model_.OMPMasterTaskLoopSimdDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPMasterTaskLoopSimdDirective); 
    } break;
    case clang::Stmt::CXXTryStmtClass: {
assert(context_.data_model_.CXXTryStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXTryStmt); 
    } break;
    case clang::Stmt::OMPMaskedTaskLoopSimdDirectiveClass: {
assert(context_.data_model_.OMPMaskedTaskLoopSimdDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPMaskedTaskLoopSimdDirective); 
    } break;
    case clang::Stmt::CompoundAssignOperatorClass: {
assert(context_.data_model_.CompoundAssignOperator != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CompoundAssignOperator); 
    } break;
    case clang::Stmt::SEHFinallyStmtClass: {
assert(context_.data_model_.SEHFinallyStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.SEHFinallyStmt); 
    } break;
    case clang::Stmt::OMPParallelMaskedTaskLoopDirectiveClass: {
assert(context_.data_model_.OMPParallelMaskedTaskLoopDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPParallelMaskedTaskLoopDirective); 
    } break;
    case clang::Stmt::ObjCAtFinallyStmtClass: {
assert(context_.data_model_.ObjCAtFinallyStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCAtFinallyStmt); 
    } break;
    case clang::Stmt::OMPParallelMasterTaskLoopDirectiveClass: {
assert(context_.data_model_.OMPParallelMasterTaskLoopDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPParallelMasterTaskLoopDirective); 
    } break;
    case clang::Stmt::BlockExprClass: {
assert(context_.data_model_.BlockExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.BlockExpr); 
    } break;
    case clang::Stmt::OMPMasterDirectiveClass: {
assert(context_.data_model_.OMPMasterDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPMasterDirective); 
    } break;
    case clang::Stmt::CXXNoexceptExprClass: {
assert(context_.data_model_.CXXNoexceptExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXNoexceptExpr); 
    } break;
    case clang::Stmt::WhileStmtClass: {
assert(context_.data_model_.WhileStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.WhileStmt); 
    } break;
    case clang::Stmt::OMPSingleDirectiveClass: {
assert(context_.data_model_.OMPSingleDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPSingleDirective); 
    } break;
    case clang::Stmt::ObjCIvarRefExprClass: {
assert(context_.data_model_.ObjCIvarRefExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCIvarRefExpr); 
    } break;
    case clang::Stmt::CUDAKernelCallExprClass: {
assert(context_.data_model_.CUDAKernelCallExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CUDAKernelCallExpr); 
    } break;
    case clang::Stmt::OMPTaskyieldDirectiveClass: {
assert(context_.data_model_.OMPTaskyieldDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTaskyieldDirective); 
    } break;
    case clang::Stmt::OMPIteratorExprClass: {
assert(context_.data_model_.OMPIteratorExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPIteratorExpr); 
    } break;
    case clang::Stmt::MSDependentExistsStmtClass: {
assert(context_.data_model_.MSDependentExistsStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.MSDependentExistsStmt); 
    } break;
    case clang::Stmt::OMPTargetUpdateDirectiveClass: {
assert(context_.data_model_.OMPTargetUpdateDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTargetUpdateDirective); 
    } break;
    case clang::Stmt::OMPCancelDirectiveClass: {
assert(context_.data_model_.OMPCancelDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPCancelDirective); 
    } break;
    case clang::Stmt::RecoveryExprClass: {
assert(context_.data_model_.RecoveryExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.RecoveryExpr); 
    } break;
    case clang::Stmt::OMPDistributeParallelForDirectiveClass: {
assert(context_.data_model_.OMPDistributeParallelForDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPDistributeParallelForDirective); 
    } break;
    case clang::Stmt::OMPTaskwaitDirectiveClass: {
assert(context_.data_model_.OMPTaskwaitDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTaskwaitDirective); 
    } break;
    case clang::Stmt::OMPTargetParallelDirectiveClass: {
assert(context_.data_model_.OMPTargetParallelDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTargetParallelDirective); 
    } break;
    case clang::Stmt::OMPCriticalDirectiveClass: {
assert(context_.data_model_.OMPCriticalDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPCriticalDirective); 
    } break;
    case clang::Stmt::IfStmtClass: {
assert(context_.data_model_.IfStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.IfStmt); 
    } break;
    case clang::Stmt::OMPDistributeParallelForSimdDirectiveClass: {
assert(context_.data_model_.OMPDistributeParallelForSimdDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPDistributeParallelForSimdDirective); 
    } break;
    case clang::Stmt::OMPGenericLoopDirectiveClass: {
assert(context_.data_model_.OMPGenericLoopDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPGenericLoopDirective); 
    } break;
    case clang::Stmt::CapturedStmtClass: {
assert(context_.data_model_.CapturedStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CapturedStmt); 
    } break;
    case clang::Stmt::OMPForDirectiveClass: {
assert(context_.data_model_.OMPForDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPForDirective); 
    } break;
    case clang::Stmt::OMPTargetSimdDirectiveClass: {
assert(context_.data_model_.OMPTargetSimdDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTargetSimdDirective); 
    } break;
    case clang::Stmt::OMPDepobjDirectiveClass: {
assert(context_.data_model_.OMPDepobjDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPDepobjDirective); 
    } break;
    case clang::Stmt::ConstantExprClass: {
assert(context_.data_model_.ConstantExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ConstantExpr); 
    } break;
    case clang::Stmt::LabelStmtClass: {
assert(context_.data_model_.LabelStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.LabelStmt); 
    } break;
    case clang::Stmt::OMPParallelForSimdDirectiveClass: {
assert(context_.data_model_.OMPParallelForSimdDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPParallelForSimdDirective); 
    } break;
    case clang::Stmt::OMPParallelMaskedDirectiveClass: {
assert(context_.data_model_.OMPParallelMaskedDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPParallelMaskedDirective); 
    } break;
    case clang::Stmt::InitListExprClass: {
assert(context_.data_model_.InitListExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.InitListExpr); 
    } break;
    case clang::Stmt::OMPTargetTeamsGenericLoopDirectiveClass: {
assert(context_.data_model_.OMPTargetTeamsGenericLoopDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTargetTeamsGenericLoopDirective); 
    } break;
    case clang::Stmt::CXXUnresolvedConstructExprClass: {
assert(context_.data_model_.CXXUnresolvedConstructExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXUnresolvedConstructExpr); 
    } break;
    case clang::Stmt::OMPTargetDirectiveClass: {
assert(context_.data_model_.OMPTargetDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTargetDirective); 
    } break;
    case clang::Stmt::OMPTargetDataDirectiveClass: {
assert(context_.data_model_.OMPTargetDataDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTargetDataDirective); 
    } break;
    case clang::Stmt::OMPTargetEnterDataDirectiveClass: {
assert(context_.data_model_.OMPTargetEnterDataDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTargetEnterDataDirective); 
    } break;
    case clang::Stmt::OMPTeamsDistributeDirectiveClass: {
assert(context_.data_model_.OMPTeamsDistributeDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTeamsDistributeDirective); 
    } break;
    case clang::Stmt::OMPOrderedDirectiveClass: {
assert(context_.data_model_.OMPOrderedDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPOrderedDirective); 
    } break;
    case clang::Stmt::ContinueStmtClass: {
assert(context_.data_model_.ContinueStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ContinueStmt); 
    } break;
    case clang::Stmt::CXXDependentScopeMemberExprClass: {
assert(context_.data_model_.CXXDependentScopeMemberExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXDependentScopeMemberExpr); 
    } break;
    case clang::Stmt::ExpressionTraitExprClass: {
assert(context_.data_model_.ExpressionTraitExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ExpressionTraitExpr); 
    } break;
    case clang::Stmt::CXXNullPtrLiteralExprClass: {
assert(context_.data_model_.CXXNullPtrLiteralExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXNullPtrLiteralExpr); 
    } break;
    case clang::Stmt::GenericSelectionExprClass: {
assert(context_.data_model_.GenericSelectionExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.GenericSelectionExpr); 
    } break;
    case clang::Stmt::OMPTeamsDirectiveClass: {
assert(context_.data_model_.OMPTeamsDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTeamsDirective); 
    } break;
    case clang::Stmt::CXXInheritedCtorInitExprClass: {
assert(context_.data_model_.CXXInheritedCtorInitExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXInheritedCtorInitExpr); 
    } break;
    case clang::Stmt::OMPTeamsDistributeParallelForDirectiveClass: {
assert(context_.data_model_.OMPTeamsDistributeParallelForDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTeamsDistributeParallelForDirective); 
    } break;
    case clang::Stmt::MSPropertySubscriptExprClass: {
assert(context_.data_model_.MSPropertySubscriptExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.MSPropertySubscriptExpr); 
    } break;
    case clang::Stmt::DesignatedInitExprClass: {
assert(context_.data_model_.DesignatedInitExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.DesignatedInitExpr); 
    } break;
    case clang::Stmt::ArraySubscriptExprClass: {
assert(context_.data_model_.ArraySubscriptExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ArraySubscriptExpr); 
    } break;
    case clang::Stmt::AtomicExprClass: {
assert(context_.data_model_.AtomicExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.AtomicExpr); 
    } break;
    case clang::Stmt::CoyieldExprClass: {
assert(context_.data_model_.CoyieldExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CoyieldExpr); 
    } break;
    case clang::Stmt::CXXScalarValueInitExprClass: {
assert(context_.data_model_.CXXScalarValueInitExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXScalarValueInitExpr); 
    } break;
    case clang::Stmt::CXXDeleteExprClass: {
assert(context_.data_model_.CXXDeleteExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXDeleteExpr); 
    } break;
    case clang::Stmt::CXXThrowExprClass: {
assert(context_.data_model_.CXXThrowExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXThrowExpr); 
    } break;
    case clang::Stmt::PseudoObjectExprClass: {
assert(context_.data_model_.PseudoObjectExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.PseudoObjectExpr); 
    } break;
    case clang::Stmt::ParenExprClass: {
assert(context_.data_model_.ParenExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ParenExpr); 
    } break;
    case clang::Stmt::ExtVectorElementExprClass: {
assert(context_.data_model_.ExtVectorElementExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ExtVectorElementExpr); 
    } break;
    case clang::Stmt::OMPUnrollDirectiveClass: {
assert(context_.data_model_.OMPUnrollDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPUnrollDirective); 
    } break;
    case clang::Stmt::FunctionParmPackExprClass: {
assert(context_.data_model_.FunctionParmPackExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.FunctionParmPackExpr); 
    } break;
    case clang::Stmt::VAArgExprClass: {
assert(context_.data_model_.VAArgExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.VAArgExpr); 
    } break;
    case clang::Stmt::AsTypeExprClass: {
assert(context_.data_model_.AsTypeExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.AsTypeExpr); 
    } break;
    case clang::Stmt::OMPFlushDirectiveClass: {
assert(context_.data_model_.OMPFlushDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPFlushDirective); 
    } break;
    case clang::Stmt::TypeTraitExprClass: {
assert(context_.data_model_.TypeTraitExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.TypeTraitExpr); 
    } break;
    case clang::Stmt::CXXMemberCallExprClass: {
assert(context_.data_model_.CXXMemberCallExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXMemberCallExpr); 
    } break;
    case clang::Stmt::UnaryExprOrTypeTraitExprClass: {
assert(context_.data_model_.UnaryExprOrTypeTraitExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.UnaryExprOrTypeTraitExpr); 
    } break;
    case clang::Stmt::CXXFunctionalCastExprClass: {
assert(context_.data_model_.CXXFunctionalCastExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXFunctionalCastExpr); 
    } break;
    case clang::Stmt::OffsetOfExprClass: {
assert(context_.data_model_.OffsetOfExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OffsetOfExpr); 
    } break;
    case clang::Stmt::DefaultStmtClass: {
assert(context_.data_model_.DefaultStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.DefaultStmt); 
    } break;
    case clang::Stmt::CXXPseudoDestructorExprClass: {
assert(context_.data_model_.CXXPseudoDestructorExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXPseudoDestructorExpr); 
    } break;
    case clang::Stmt::StringLiteralClass: {
assert(context_.data_model_.StringLiteral != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.StringLiteral); 
    } break;
    case clang::Stmt::FloatingLiteralClass: {
assert(context_.data_model_.FloatingLiteral != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.FloatingLiteral); 
    } break;
    case clang::Stmt::UnaryOperatorClass: {
assert(context_.data_model_.UnaryOperator != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.UnaryOperator); 
    } break;
    case clang::Stmt::OMPTargetTeamsDirectiveClass: {
assert(context_.data_model_.OMPTargetTeamsDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTargetTeamsDirective); 
    } break;
    case clang::Stmt::CXXReinterpretCastExprClass: {
assert(context_.data_model_.CXXReinterpretCastExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXReinterpretCastExpr); 
    } break;
    case clang::Stmt::SourceLocExprClass: {
assert(context_.data_model_.SourceLocExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.SourceLocExpr); 
    } break;
    case clang::Stmt::CXXNewExprClass: {
assert(context_.data_model_.CXXNewExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXNewExpr); 
    } break;
    case clang::Stmt::ParenListExprClass: {
assert(context_.data_model_.ParenListExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ParenListExpr); 
    } break;
    case clang::Stmt::BuiltinBitCastExprClass: {
assert(context_.data_model_.BuiltinBitCastExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.BuiltinBitCastExpr); 
    } break;
    case clang::Stmt::PredefinedExprClass: {
assert(context_.data_model_.PredefinedExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.PredefinedExpr); 
    } break;
    case clang::Stmt::CXXDefaultArgExprClass: {
assert(context_.data_model_.CXXDefaultArgExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXDefaultArgExpr); 
    } break;
    case clang::Stmt::OpaqueValueExprClass: {
assert(context_.data_model_.OpaqueValueExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OpaqueValueExpr); 
    } break;
    case clang::Stmt::OMPTargetParallelGenericLoopDirectiveClass: {
assert(context_.data_model_.OMPTargetParallelGenericLoopDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTargetParallelGenericLoopDirective); 
    } break;
    case clang::Stmt::ReturnStmtClass: {
assert(context_.data_model_.ReturnStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ReturnStmt); 
    } break;
    case clang::Stmt::ArrayInitIndexExprClass: {
assert(context_.data_model_.ArrayInitIndexExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ArrayInitIndexExpr); 
    } break;
    case clang::Stmt::IntegerLiteralClass: {
assert(context_.data_model_.IntegerLiteral != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.IntegerLiteral); 
    } break;
    case clang::Stmt::CStyleCastExprClass: {
assert(context_.data_model_.CStyleCastExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CStyleCastExpr); 
    } break;
    case clang::Stmt::UnresolvedMemberExprClass: {
assert(context_.data_model_.UnresolvedMemberExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.UnresolvedMemberExpr); 
    } break;
    case clang::Stmt::CompoundLiteralExprClass: {
assert(context_.data_model_.CompoundLiteralExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CompoundLiteralExpr); 
    } break;
    case clang::Stmt::CXXConstructExprClass: {
assert(context_.data_model_.CXXConstructExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXConstructExpr); 
    } break;
    case clang::Stmt::ExprWithCleanupsClass: {
assert(context_.data_model_.ExprWithCleanups != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ExprWithCleanups); 
    } break;
    case clang::Stmt::ObjCBridgedCastExprClass: {
assert(context_.data_model_.ObjCBridgedCastExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCBridgedCastExpr); 
    } break;
    case clang::Stmt::DeclStmtClass: {
assert(context_.data_model_.DeclStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.DeclStmt); 
    } break;
    case clang::Stmt::BinaryOperatorClass: {
assert(context_.data_model_.BinaryOperator != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.BinaryOperator); 
    } break;
    case clang::Stmt::CoawaitExprClass: {
assert(context_.data_model_.CoawaitExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CoawaitExpr); 
    } break;
    case clang::Stmt::CXXStaticCastExprClass: {
assert(context_.data_model_.CXXStaticCastExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXStaticCastExpr); 
    } break;
    case clang::Stmt::CXXDynamicCastExprClass: {
assert(context_.data_model_.CXXDynamicCastExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXDynamicCastExpr); 
    } break;
    case clang::Stmt::CXXConstCastExprClass: {
assert(context_.data_model_.CXXConstCastExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXConstCastExpr); 
    } break;
    case clang::Stmt::StmtExprClass: {
assert(context_.data_model_.StmtExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.StmtExpr); 
    } break;
    case clang::Stmt::CXXDefaultInitExprClass: {
assert(context_.data_model_.CXXDefaultInitExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXDefaultInitExpr); 
    } break;
    case clang::Stmt::MatrixSubscriptExprClass: {
assert(context_.data_model_.MatrixSubscriptExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.MatrixSubscriptExpr); 
    } break;
    case clang::Stmt::CXXRewrittenBinaryOperatorClass: {
assert(context_.data_model_.CXXRewrittenBinaryOperator != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXRewrittenBinaryOperator); 
    } break;
    case clang::Stmt::OMPBarrierDirectiveClass: {
assert(context_.data_model_.OMPBarrierDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPBarrierDirective); 
    } break;
    case clang::Stmt::OMPScanDirectiveClass: {
assert(context_.data_model_.OMPScanDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPScanDirective); 
    } break;
    case clang::Stmt::AttributedStmtClass: {
assert(context_.data_model_.AttributedStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.AttributedStmt); 
    } break;
    case clang::Stmt::ConvertVectorExprClass: {
assert(context_.data_model_.ConvertVectorExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ConvertVectorExpr); 
    } break;
    case clang::Stmt::ImplicitValueInitExprClass: {
assert(context_.data_model_.ImplicitValueInitExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ImplicitValueInitExpr); 
    } break;
    case clang::Stmt::NoInitExprClass: {
assert(context_.data_model_.NoInitExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.NoInitExpr); 
    } break;
    case clang::Stmt::CallExprClass: {
assert(context_.data_model_.CallExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CallExpr); 
    } break;
    case clang::Stmt::OMPScopeDirectiveClass: {
assert(context_.data_model_.OMPScopeDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPScopeDirective); 
    } break;
    case clang::Stmt::ConditionalOperatorClass: {
assert(context_.data_model_.ConditionalOperator != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ConditionalOperator); 
    } break;
    case clang::Stmt::DesignatedInitUpdateExprClass: {
assert(context_.data_model_.DesignatedInitUpdateExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.DesignatedInitUpdateExpr); 
    } break;
    case clang::Stmt::ArrayInitLoopExprClass: {
assert(context_.data_model_.ArrayInitLoopExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ArrayInitLoopExpr); 
    } break;
    case clang::Stmt::CXXThisExprClass: {
assert(context_.data_model_.CXXThisExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXThisExpr); 
    } break;
    case clang::Stmt::PackExpansionExprClass: {
assert(context_.data_model_.PackExpansionExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.PackExpansionExpr); 
    } break;
    case clang::Stmt::MaterializeTemporaryExprClass: {
assert(context_.data_model_.MaterializeTemporaryExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.MaterializeTemporaryExpr); 
    } break;
    case clang::Stmt::LambdaExprClass: {
assert(context_.data_model_.LambdaExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.LambdaExpr); 
    } break;
    case clang::Stmt::DependentCoawaitExprClass: {
assert(context_.data_model_.DependentCoawaitExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.DependentCoawaitExpr); 
    } break;
    case clang::Stmt::SizeOfPackExprClass: {
assert(context_.data_model_.SizeOfPackExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.SizeOfPackExpr); 
    } break;
    case clang::Stmt::RequiresExprClass: {
assert(context_.data_model_.RequiresExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.RequiresExpr); 
    } break;
    case clang::Stmt::ObjCStringLiteralClass: {
assert(context_.data_model_.ObjCStringLiteral != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCStringLiteral); 
    } break;
    case clang::Stmt::ObjCBoolLiteralExprClass: {
assert(context_.data_model_.ObjCBoolLiteralExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ObjCBoolLiteralExpr); 
    } break;
    case clang::Stmt::CXXFoldExprClass: {
assert(context_.data_model_.CXXFoldExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXFoldExpr); 
    } break;
    case clang::Stmt::DoStmtClass: {
assert(context_.data_model_.DoStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.DoStmt); 
    } break;
    case clang::Stmt::OMPMetaDirectiveClass: {
assert(context_.data_model_.OMPMetaDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPMetaDirective); 
    } break;
    case clang::Stmt::OMPAtomicDirectiveClass: {
assert(context_.data_model_.OMPAtomicDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPAtomicDirective); 
    } break;
    case clang::Stmt::AddrLabelExprClass: {
assert(context_.data_model_.AddrLabelExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.AddrLabelExpr); 
    } break;
    case clang::Stmt::GNUNullExprClass: {
assert(context_.data_model_.GNUNullExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.GNUNullExpr); 
    } break;
    case clang::Stmt::ConceptSpecializationExprClass: {
assert(context_.data_model_.ConceptSpecializationExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ConceptSpecializationExpr); 
    } break;
    case clang::Stmt::OMPTargetParallelForSimdDirectiveClass: {
assert(context_.data_model_.OMPTargetParallelForSimdDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTargetParallelForSimdDirective); 
    } break;
    case clang::Stmt::OMPTaskDirectiveClass: {
assert(context_.data_model_.OMPTaskDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTaskDirective); 
    } break;
    case clang::Stmt::ShuffleVectorExprClass: {
assert(context_.data_model_.ShuffleVectorExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ShuffleVectorExpr); 
    } break;
    case clang::Stmt::BinaryConditionalOperatorClass: {
assert(context_.data_model_.BinaryConditionalOperator != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.BinaryConditionalOperator); 
    } break;
    case clang::Stmt::OMPTeamsDistributeSimdDirectiveClass: {
assert(context_.data_model_.OMPTeamsDistributeSimdDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTeamsDistributeSimdDirective); 
    } break;
    case clang::Stmt::OMPArrayShapingExprClass: {
assert(context_.data_model_.OMPArrayShapingExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPArrayShapingExpr); 
    } break;
    case clang::Stmt::UnresolvedLookupExprClass: {
assert(context_.data_model_.UnresolvedLookupExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.UnresolvedLookupExpr); 
    } break;
    case clang::Stmt::OMPTeamsDistributeParallelForSimdDirectiveClass: {
assert(context_.data_model_.OMPTeamsDistributeParallelForSimdDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTeamsDistributeParallelForSimdDirective); 
    } break;
    case clang::Stmt::CharacterLiteralClass: {
assert(context_.data_model_.CharacterLiteral != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CharacterLiteral); 
    } break;
    case clang::Stmt::SEHLeaveStmtClass: {
assert(context_.data_model_.SEHLeaveStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.SEHLeaveStmt); 
    } break;
    case clang::Stmt::OMPTargetTeamsDistributeParallelForSimdDirectiveClass: {
assert(context_.data_model_.OMPTargetTeamsDistributeParallelForSimdDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTargetTeamsDistributeParallelForSimdDirective); 
    } break;
    case clang::Stmt::CXXAddrspaceCastExprClass: {
assert(context_.data_model_.CXXAddrspaceCastExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXAddrspaceCastExpr); 
    } break;
    case clang::Stmt::OMPDispatchDirectiveClass: {
assert(context_.data_model_.OMPDispatchDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPDispatchDirective); 
    } break;
    case clang::Stmt::CompoundStmtClass: {
assert(context_.data_model_.CompoundStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CompoundStmt); 
    } break;
    case clang::Stmt::OMPTargetTeamsDistributeParallelForDirectiveClass: {
assert(context_.data_model_.OMPTargetTeamsDistributeParallelForDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTargetTeamsDistributeParallelForDirective); 
    } break;
    case clang::Stmt::UserDefinedLiteralClass: {
assert(context_.data_model_.UserDefinedLiteral != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.UserDefinedLiteral); 
    } break;
    case clang::Stmt::OMPMaskedDirectiveClass: {
assert(context_.data_model_.OMPMaskedDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPMaskedDirective); 
    } break;
    case clang::Stmt::OMPTeamsGenericLoopDirectiveClass: {
assert(context_.data_model_.OMPTeamsGenericLoopDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTeamsGenericLoopDirective); 
    } break;
    case clang::Stmt::OMPArraySectionExprClass: {
assert(context_.data_model_.OMPArraySectionExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPArraySectionExpr); 
    } break;
    case clang::Stmt::OMPParallelGenericLoopDirectiveClass: {
assert(context_.data_model_.OMPParallelGenericLoopDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPParallelGenericLoopDirective); 
    } break;
    case clang::Stmt::CXXTemporaryObjectExprClass: {
assert(context_.data_model_.CXXTemporaryObjectExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXTemporaryObjectExpr); 
    } break;
    case clang::Stmt::OMPSectionsDirectiveClass: {
assert(context_.data_model_.OMPSectionsDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPSectionsDirective); 
    } break;
    case clang::Stmt::ChooseExprClass: {
assert(context_.data_model_.ChooseExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.ChooseExpr); 
    } break;
    case clang::Stmt::OMPDistributeSimdDirectiveClass: {
assert(context_.data_model_.OMPDistributeSimdDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPDistributeSimdDirective); 
    } break;
    case clang::Stmt::OMPParallelMasterTaskLoopSimdDirectiveClass: {
assert(context_.data_model_.OMPParallelMasterTaskLoopSimdDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPParallelMasterTaskLoopSimdDirective); 
    } break;
    case clang::Stmt::OMPMasterTaskLoopDirectiveClass: {
assert(context_.data_model_.OMPMasterTaskLoopDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPMasterTaskLoopDirective); 
    } break;
    case clang::Stmt::OMPSectionDirectiveClass: {
assert(context_.data_model_.OMPSectionDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPSectionDirective); 
    } break;
    case clang::Stmt::OMPDistributeDirectiveClass: {
assert(context_.data_model_.OMPDistributeDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPDistributeDirective); 
    } break;
    case clang::Stmt::CaseStmtClass: {
assert(context_.data_model_.CaseStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CaseStmt); 
    } break;
    case clang::Stmt::OMPTaskLoopDirectiveClass: {
assert(context_.data_model_.OMPTaskLoopDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTaskLoopDirective); 
    } break;
    case clang::Stmt::OMPTargetExitDataDirectiveClass: {
assert(context_.data_model_.OMPTargetExitDataDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTargetExitDataDirective); 
    } break;
    case clang::Stmt::CXXCatchStmtClass: {
assert(context_.data_model_.CXXCatchStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.CXXCatchStmt); 
    } break;
    case clang::Stmt::GotoStmtClass: {
assert(context_.data_model_.GotoStmt != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.GotoStmt); 
    } break;
    case clang::Stmt::DependentScopeDeclRefExprClass: {
assert(context_.data_model_.DependentScopeDeclRefExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.DependentScopeDeclRefExpr); 
    } break;
    case clang::Stmt::FixedPointLiteralClass: {
assert(context_.data_model_.FixedPointLiteral != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.FixedPointLiteral); 
    } break;
    case clang::Stmt::OMPCancellationPointDirectiveClass: {
assert(context_.data_model_.OMPCancellationPointDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPCancellationPointDirective); 
    } break;
    case clang::Stmt::OMPTargetTeamsDistributeDirectiveClass: {
assert(context_.data_model_.OMPTargetTeamsDistributeDirective != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.OMPTargetTeamsDistributeDirective); 
    } break;
    case clang::Stmt::SubstNonTypeTemplateParmPackExprClass: {
assert(context_.data_model_.SubstNonTypeTemplateParmPackExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.SubstNonTypeTemplateParmPackExpr); 
    } break;
    case clang::Stmt::DeclRefExprClass: {
assert(context_.data_model_.DeclRefExpr != nullptr);
     arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.DeclRefExpr); 
    } break;
    default: break;
  }

  //stripLabelLikeStatements
  {
    const Entity* other = context_.resolve(D->stripLabelLikeStatements());
    arboretum_create_edge(obj, context_.data_model_.stripLabelLikeStatements, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitStmtExpr(clang::StmtExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getSubStmt
  {
    const Entity* other = context_.resolve(D->getSubStmt());
    arboretum_create_edge(obj, context_.data_model_.getSubStmt11, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc154, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc152, other);
  }
  //getLParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getLParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getLParenLoc16, other);
  }
  //getRParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc35, other);
  }
  //getTemplateDepth
  // unsigned int
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitStringLiteral(clang::StringLiteral* D) {
  const Entity* obj = context_.resolve(D);
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
    const Entity* enum_value = context_.data_model_.resolve(D->getKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getKind2, enum_value);
    }
  }
  //isOrdinary
  arboretum_create_edge(obj, context_.data_model_.isOrdinary, context_.data_model_.arboretum_node_for(D->isOrdinary()));
  //isWide
  arboretum_create_edge(obj, context_.data_model_.isWide, context_.data_model_.arboretum_node_for(D->isWide()));
  //isUTF8
  arboretum_create_edge(obj, context_.data_model_.isUTF8, context_.data_model_.arboretum_node_for(D->isUTF8()));
  //isUTF16
  arboretum_create_edge(obj, context_.data_model_.isUTF16, context_.data_model_.arboretum_node_for(D->isUTF16()));
  //isUTF32
  arboretum_create_edge(obj, context_.data_model_.isUTF32, context_.data_model_.arboretum_node_for(D->isUTF32()));
  //isUnevaluated
  arboretum_create_edge(obj, context_.data_model_.isUnevaluated, context_.data_model_.arboretum_node_for(D->isUnevaluated()));
  //isPascal
  arboretum_create_edge(obj, context_.data_model_.isPascal, context_.data_model_.arboretum_node_for(D->isPascal()));
  //containsNonAscii
  arboretum_create_edge(obj, context_.data_model_.containsNonAscii, context_.data_model_.arboretum_node_for(D->containsNonAscii()));
  //containsNonAsciiOrNull
  arboretum_create_edge(obj, context_.data_model_.containsNonAsciiOrNull, context_.data_model_.arboretum_node_for(D->containsNonAsciiOrNull()));
  //getNumConcatenated
  // unsigned int
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc98, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc98, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitSubstNonTypeTemplateParmExpr(clang::SubstNonTypeTemplateParmExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getNameLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getNameLoc());
    arboretum_create_edge(obj, context_.data_model_.getNameLoc6, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc146, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc145, other);
  }
  //getReplacement
  {
    const Entity* other = context_.resolve(D->getReplacement());
    arboretum_create_edge(obj, context_.data_model_.getReplacement1, other);
  }
  //getAssociatedDecl
  {
    const Entity* other = context_.resolve(D->getAssociatedDecl());
    arboretum_create_edge(obj, context_.data_model_.getAssociatedDecl2, other);
  }
  //getIndex
  // unsigned int
  //getPackIndex
  // std::optional<unsigned int>
  //getParameter
  {
    const Entity* other = context_.resolve(D->getParameter());
    arboretum_create_edge(obj, context_.data_model_.getParameter, other);
  }
  //isReferenceParameter
  arboretum_create_edge(obj, context_.data_model_.isReferenceParameter, context_.data_model_.arboretum_node_for(D->isReferenceParameter()));
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitSubstNonTypeTemplateParmPackExpr(clang::SubstNonTypeTemplateParmPackExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getAssociatedDecl
  {
    const Entity* other = context_.resolve(D->getAssociatedDecl());
    arboretum_create_edge(obj, context_.data_model_.getAssociatedDecl1, other);
  }
  //getIndex
  // unsigned int
  //getParameterPack
  {
    const Entity* other = context_.resolve(D->getParameterPack());
    arboretum_create_edge(obj, context_.data_model_.getParameterPack1, other);
  }
  //getParameterPackLocation
  {
    const Entity* other = context_.source_model_.resolve(D->getParameterPackLocation());
    arboretum_create_edge(obj, context_.data_model_.getParameterPackLocation1, other);
  }
  //getArgumentPack
  // TemplateArgument
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc125, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc127, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitSwitchCase(clang::SwitchCase* D) {
  const Entity* obj = context_.resolve(D);
  //getNextSwitchCase
  {
    const Entity* other = context_.resolve(D->getNextSwitchCase());
    arboretum_create_edge(obj, context_.data_model_.getNextSwitchCase, other);
  }
  //getKeywordLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getKeywordLoc());
    arboretum_create_edge(obj, context_.data_model_.getKeywordLoc2, other);
  }
  //getColonLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getColonLoc());
    arboretum_create_edge(obj, context_.data_model_.getColonLoc5, other);
  }
  //getSubStmt
  {
    const Entity* other = context_.resolve(D->getSubStmt());
    arboretum_create_edge(obj, context_.data_model_.getSubStmt13, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc157, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc155, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitSwitchStmt(clang::SwitchStmt* D) {
  const Entity* obj = context_.resolve(D);
  //hasInitStorage
  arboretum_create_edge(obj, context_.data_model_.hasInitStorage, context_.data_model_.arboretum_node_for(D->hasInitStorage()));
  //hasVarStorage
  arboretum_create_edge(obj, context_.data_model_.hasVarStorage1, context_.data_model_.arboretum_node_for(D->hasVarStorage()));
  //getCond
  {
    const Entity* other = context_.resolve(D->getCond());
    arboretum_create_edge(obj, context_.data_model_.getCond5, other);
  }
  //getBody
  {
    const Entity* other = context_.resolve(D->getBody());
    arboretum_create_edge(obj, context_.data_model_.getBody8, other);
  }
  //getInit
  {
    const Entity* other = context_.resolve(D->getInit());
    arboretum_create_edge(obj, context_.data_model_.getInit3, other);
  }
  //getConditionVariable
  {
    const Entity* other = context_.resolve(D->getConditionVariable());
    arboretum_create_edge(obj, context_.data_model_.getConditionVariable4, other);
  }
  //getConditionVariableDeclStmt
  {
    const Entity* other = context_.resolve(D->getConditionVariableDeclStmt());
    arboretum_create_edge(obj, context_.data_model_.getConditionVariableDeclStmt5, other);
  }
  //getSwitchCaseList
  {
    const Entity* other = context_.resolve(D->getSwitchCaseList());
    arboretum_create_edge(obj, context_.data_model_.getSwitchCaseList1, other);
  }
  //getSwitchLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getSwitchLoc());
    arboretum_create_edge(obj, context_.data_model_.getSwitchLoc, other);
  }
  //getLParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getLParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getLParenLoc3, other);
  }
  //getRParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc5, other);
  }
  //isAllEnumCasesCovered
  arboretum_create_edge(obj, context_.data_model_.isAllEnumCasesCovered, context_.data_model_.arboretum_node_for(D->isAllEnumCasesCovered()));
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc29, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc29, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitTypeTraitExpr(clang::TypeTraitExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getTrait
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getTrait());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getTrait1, enum_value);
    }
  }
  //getNumArgs
  // unsigned int
  //getArgs
  // ArrayRef<TypeSourceInfo *>
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc113, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc114, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitTypoExpr(clang::TypoExpr* D) {
  const Entity* obj = context_.resolve(D);
  //children
  // const_child_range
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc26, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc26, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitUnaryExprOrTypeTraitExpr(clang::UnaryExprOrTypeTraitExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getKind1, enum_value);
    }
  }
  //isArgumentType
  arboretum_create_edge(obj, context_.data_model_.isArgumentType, context_.data_model_.arboretum_node_for(D->isArgumentType()));
  //getTypeOfArgument
  {
    const Entity* other = context_.resolve(D->getTypeOfArgument());
    arboretum_create_edge(obj, context_.data_model_.getTypeOfArgument, other);
  }
  //getOperatorLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getOperatorLoc());
    arboretum_create_edge(obj, context_.data_model_.getOperatorLoc6, other);
  }
  //getRParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc19, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc95, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc95, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitUnaryOperator(clang::UnaryOperator* D) {
  const Entity* obj = context_.resolve(D);
  //getOpcode
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getOpcode());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getOpcode2, enum_value);
    }
  }
  //getSubExpr
  {
    const Entity* other = context_.resolve(D->getSubExpr());
    arboretum_create_edge(obj, context_.data_model_.getSubExpr12, other);
  }
  //getOperatorLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getOperatorLoc());
    arboretum_create_edge(obj, context_.data_model_.getOperatorLoc5, other);
  }
  //canOverflow
  arboretum_create_edge(obj, context_.data_model_.canOverflow, context_.data_model_.arboretum_node_for(D->canOverflow()));
  //isPrefix
  arboretum_create_edge(obj, context_.data_model_.isPrefix1, context_.data_model_.arboretum_node_for(D->isPrefix()));
  //isPostfix
  arboretum_create_edge(obj, context_.data_model_.isPostfix1, context_.data_model_.arboretum_node_for(D->isPostfix()));
  //isIncrementOp
  arboretum_create_edge(obj, context_.data_model_.isIncrementOp1, context_.data_model_.arboretum_node_for(D->isIncrementOp()));
  //isDecrementOp
  arboretum_create_edge(obj, context_.data_model_.isDecrementOp1, context_.data_model_.arboretum_node_for(D->isDecrementOp()));
  //isIncrementDecrementOp
  arboretum_create_edge(obj, context_.data_model_.isIncrementDecrementOp1, context_.data_model_.arboretum_node_for(D->isIncrementDecrementOp()));
  //isArithmeticOp
  arboretum_create_edge(obj, context_.data_model_.isArithmeticOp1, context_.data_model_.arboretum_node_for(D->isArithmeticOp()));
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc90, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc90, other);
  }
  //getExprLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getExprLoc());
    arboretum_create_edge(obj, context_.data_model_.getExprLoc12, other);
  }
  //children
  // const_child_range
  //hasStoredFPFeatures
  arboretum_create_edge(obj, context_.data_model_.hasStoredFPFeatures2, context_.data_model_.arboretum_node_for(D->hasStoredFPFeatures()));
  //getStoredFPFeatures
  // FPOptionsOverride
  //getFPOptionsOverride
  // FPOptionsOverride
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedLookupExpr(clang::UnresolvedLookupExpr* D) {
  const Entity* obj = context_.resolve(D);
  //requiresADL
  arboretum_create_edge(obj, context_.data_model_.requiresADL, context_.data_model_.arboretum_node_for(D->requiresADL()));
  //isOverloaded
  arboretum_create_edge(obj, context_.data_model_.isOverloaded, context_.data_model_.arboretum_node_for(D->isOverloaded()));
  //getNamingClass
  {
    const Entity* other = context_.resolve(D->getNamingClass());
    arboretum_create_edge(obj, context_.data_model_.getNamingClass1, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc48, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc48, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedMemberExpr(clang::UnresolvedMemberExpr* D) {
  const Entity* obj = context_.resolve(D);
  //isImplicitAccess
  arboretum_create_edge(obj, context_.data_model_.isImplicitAccess2, context_.data_model_.arboretum_node_for(D->isImplicitAccess()));
  //getBaseType
  {
    const Entity* other = context_.resolve(D->getBaseType());
    arboretum_create_edge(obj, context_.data_model_.getBaseType1, other);
  }
  //hasUnresolvedUsing
  arboretum_create_edge(obj, context_.data_model_.hasUnresolvedUsing, context_.data_model_.arboretum_node_for(D->hasUnresolvedUsing()));
  //isArrow
  arboretum_create_edge(obj, context_.data_model_.isArrow4, context_.data_model_.arboretum_node_for(D->isArrow()));
  //getOperatorLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getOperatorLoc());
    arboretum_create_edge(obj, context_.data_model_.getOperatorLoc2, other);
  }
  //getNamingClass
  {
    const Entity* other = context_.resolve(D->getNamingClass());
    arboretum_create_edge(obj, context_.data_model_.getNamingClass5, other);
  }
  //getMemberNameInfo
  // const DeclarationNameInfo &
  //getMemberName
  // DeclarationName
  //getMemberLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getMemberLoc());
    arboretum_create_edge(obj, context_.data_model_.getMemberLoc2, other);
  }
  //getExprLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getExprLoc());
    arboretum_create_edge(obj, context_.data_model_.getExprLoc5, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc64, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc64, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitUserDefinedLiteral(clang::UserDefinedLiteral* D) {
  const Entity* obj = context_.resolve(D);
  //getLiteralOperatorKind
  {
    const Entity* enum_value = context_.data_model_.resolve(D->getLiteralOperatorKind());
    if (enum_value != nullptr) {
      arboretum_create_edge(obj, context_.data_model_.getLiteralOperatorKind, enum_value);
    }
  }
  //getCookedLiteral
  {
    const Entity* other = context_.resolve(D->getCookedLiteral());
    arboretum_create_edge(obj, context_.data_model_.getCookedLiteral1, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc43, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc43, other);
  }
  //getUDSuffixLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getUDSuffixLoc());
    arboretum_create_edge(obj, context_.data_model_.getUDSuffixLoc, other);
  }
  //getUDSuffix
  return true;
}

bool ArboretumASTVisitor::VisitVAArgExpr(clang::VAArgExpr* D) {
  const Entity* obj = context_.resolve(D);
  //getSubExpr
  {
    const Entity* other = context_.resolve(D->getSubExpr());
    arboretum_create_edge(obj, context_.data_model_.getSubExpr17, other);
  }
  //isMicrosoftABI
  arboretum_create_edge(obj, context_.data_model_.isMicrosoftABI, context_.data_model_.arboretum_node_for(D->isMicrosoftABI()));
  //getWrittenTypeInfo
  //getBuiltinLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBuiltinLoc());
    arboretum_create_edge(obj, context_.data_model_.getBuiltinLoc4, other);
  }
  //getRParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc23, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc114, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc115, other);
  }
  //children
  // const_child_range
  return true;
}

bool ArboretumASTVisitor::VisitValueStmt(clang::ValueStmt* D) {
  const Entity* obj = context_.resolve(D);
  //getExprStmt
  {
    const Entity* other = context_.resolve(D->getExprStmt());
    arboretum_create_edge(obj, context_.data_model_.getExprStmt, other);
  }
  return true;
}

bool ArboretumASTVisitor::VisitWhileStmt(clang::WhileStmt* D) {
  const Entity* obj = context_.resolve(D);
  //hasVarStorage
  arboretum_create_edge(obj, context_.data_model_.hasVarStorage, context_.data_model_.arboretum_node_for(D->hasVarStorage()));
  //getCond
  {
    const Entity* other = context_.resolve(D->getCond());
    arboretum_create_edge(obj, context_.data_model_.getCond3, other);
  }
  //getBody
  {
    const Entity* other = context_.resolve(D->getBody());
    arboretum_create_edge(obj, context_.data_model_.getBody6, other);
  }
  //getConditionVariable
  {
    const Entity* other = context_.resolve(D->getConditionVariable());
    arboretum_create_edge(obj, context_.data_model_.getConditionVariable2, other);
  }
  //getConditionVariableDeclStmt
  {
    const Entity* other = context_.resolve(D->getConditionVariableDeclStmt());
    arboretum_create_edge(obj, context_.data_model_.getConditionVariableDeclStmt3, other);
  }
  //getWhileLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getWhileLoc());
    arboretum_create_edge(obj, context_.data_model_.getWhileLoc, other);
  }
  //getLParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getLParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getLParenLoc1, other);
  }
  //getRParenLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getRParenLoc());
    arboretum_create_edge(obj, context_.data_model_.getRParenLoc3, other);
  }
  //getBeginLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getBeginLoc());
    arboretum_create_edge(obj, context_.data_model_.getBeginLoc24, other);
  }
  //getEndLoc
  {
    const Entity* other = context_.source_model_.resolve(D->getEndLoc());
    arboretum_create_edge(obj, context_.data_model_.getEndLoc24, other);
  }
  //children
  // const_child_range
  return true;
}

////   END ARBORETUM GENERATED CODE ////
} // namespace arboretum