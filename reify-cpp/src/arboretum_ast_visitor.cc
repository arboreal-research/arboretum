#include "arboretum_ast_visitor.h"

#include "arboretum_ffi.h"

namespace arboretum {

//// BEGIN ARBORETUM GENERATED CODE ////
// Types
bool ArboretumASTVisitor::VisitAdjustedType(clang::AdjustedType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getOriginalType_, context_.resolve(D->getOriginalType()));
  arboretum_create_edge(obj, context_.data_model_.method_getAdjustedType_, context_.resolve(D->getAdjustedType()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitArrayType(clang::ArrayType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getElementType_, context_.resolve(D->getElementType()));
  arboretum_create_edge(obj, context_.data_model_.method_getSizeModifier_, context_.data_model_.resolve(D->getSizeModifier()));
  // getIndexTypeQualifiers ( class clang::Qualifiers )
  arboretum_create_edge(obj, context_.data_model_.method_getIndexTypeCVRQualifiers_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getIndexTypeCVRQualifiers())));
  return true;
}

bool ArboretumASTVisitor::VisitAtomicType(clang::AtomicType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getValueType_, context_.resolve(D->getValueType()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_1_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_1_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitAttributedType(clang::AttributedType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getAttrKind_, context_.data_model_.resolve(D->getAttrKind()));
  arboretum_create_edge(obj, context_.data_model_.method_getModifiedType_, context_.resolve(D->getModifiedType()));
  arboretum_create_edge(obj, context_.data_model_.method_getEquivalentType_, context_.resolve(D->getEquivalentType()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_2_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_2_, context_.resolve(D->desugar()));
  arboretum_create_edge(obj, context_.data_model_.method_isQualifier_, context_.data_model_.arboretum_node_for(D->isQualifier()));
  arboretum_create_edge(obj, context_.data_model_.method_isMSTypeSpec_, context_.data_model_.arboretum_node_for(D->isMSTypeSpec()));
  arboretum_create_edge(obj, context_.data_model_.method_isWebAssemblyFuncrefSpec_, context_.data_model_.arboretum_node_for(D->isWebAssemblyFuncrefSpec()));
  arboretum_create_edge(obj, context_.data_model_.method_isCallingConv_, context_.data_model_.arboretum_node_for(D->isCallingConv()));
  // getImmediateNullability ( class std::optional<enum clang::NullabilityKind> )
  return true;
}

bool ArboretumASTVisitor::VisitAutoType(clang::AutoType* D) {
  const Id* obj = context_.resolve(D);
  // getTypeConstraintArguments ( class llvm::ArrayRef<class clang::TemplateArgument> )
  arboretum_create_edge(obj, context_.data_model_.method_getTypeConstraintConcept_, context_.resolve(D->getTypeConstraintConcept()));
  arboretum_create_edge(obj, context_.data_model_.method_isConstrained_, context_.data_model_.arboretum_node_for(D->isConstrained()));
  arboretum_create_edge(obj, context_.data_model_.method_isDecltypeAuto_, context_.data_model_.arboretum_node_for(D->isDecltypeAuto()));
  arboretum_create_edge(obj, context_.data_model_.method_isGNUAutoType_, context_.data_model_.arboretum_node_for(D->isGNUAutoType()));
  arboretum_create_edge(obj, context_.data_model_.method_getKeyword_, context_.data_model_.resolve(D->getKeyword()));
  return true;
}

bool ArboretumASTVisitor::VisitBTFTagAttributedType(clang::BTFTagAttributedType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getWrappedType_, context_.resolve(D->getWrappedType()));
  arboretum_create_edge(obj, context_.data_model_.method_getAttr_, context_.resolve(D->getAttr()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_3_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_3_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitBitIntType(clang::BitIntType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_isUnsigned_, context_.data_model_.arboretum_node_for(D->isUnsigned()));
  arboretum_create_edge(obj, context_.data_model_.method_isSigned_, context_.data_model_.arboretum_node_for(D->isSigned()));
  arboretum_create_edge(obj, context_.data_model_.method_getNumBits_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumBits())));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_4_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_4_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitBlockPointerType(clang::BlockPointerType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getPointeeType_, context_.resolve(D->getPointeeType()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_5_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_5_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitBuiltinType(clang::BuiltinType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getKind_, context_.data_model_.resolve(D->getKind()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_6_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_6_, context_.resolve(D->desugar()));
  arboretum_create_edge(obj, context_.data_model_.method_isInteger_, context_.data_model_.arboretum_node_for(D->isInteger()));
  arboretum_create_edge(obj, context_.data_model_.method_isSignedInteger_, context_.data_model_.arboretum_node_for(D->isSignedInteger()));
  arboretum_create_edge(obj, context_.data_model_.method_isUnsignedInteger_, context_.data_model_.arboretum_node_for(D->isUnsignedInteger()));
  arboretum_create_edge(obj, context_.data_model_.method_isFloatingPoint_, context_.data_model_.arboretum_node_for(D->isFloatingPoint()));
  arboretum_create_edge(obj, context_.data_model_.method_isSVEBool_, context_.data_model_.arboretum_node_for(D->isSVEBool()));
  arboretum_create_edge(obj, context_.data_model_.method_isSVECount_, context_.data_model_.arboretum_node_for(D->isSVECount()));
  arboretum_create_edge(obj, context_.data_model_.method_isPlaceholderType_, context_.data_model_.arboretum_node_for(D->isPlaceholderType()));
  arboretum_create_edge(obj, context_.data_model_.method_isNonOverloadPlaceholderType_, context_.data_model_.arboretum_node_for(D->isNonOverloadPlaceholderType()));
  return true;
}

bool ArboretumASTVisitor::VisitComplexType(clang::ComplexType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getElementType_1_, context_.resolve(D->getElementType()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_7_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_7_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitConstantArrayType(clang::ConstantArrayType* D) {
  const Id* obj = context_.resolve(D);
  // getSize ( const class llvm::APInt & )
  arboretum_create_edge(obj, context_.data_model_.method_getSizeExpr_, context_.resolve(D->getSizeExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_8_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_8_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitConstantMatrixType(clang::ConstantMatrixType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getNumRows_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumRows())));
  arboretum_create_edge(obj, context_.data_model_.method_getNumColumns_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumColumns())));
  arboretum_create_edge(obj, context_.data_model_.method_getNumElementsFlattened_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumElementsFlattened())));
  return true;
}

bool ArboretumASTVisitor::VisitDecayedType(clang::DecayedType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getDecayedType_, context_.resolve(D->getDecayedType()));
  arboretum_create_edge(obj, context_.data_model_.method_getPointeeType_1_, context_.resolve(D->getPointeeType()));
  return true;
}

bool ArboretumASTVisitor::VisitDecltypeType(clang::DecltypeType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getUnderlyingExpr_, context_.resolve(D->getUnderlyingExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getUnderlyingType_, context_.resolve(D->getUnderlyingType()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_9_, context_.resolve(D->desugar()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_9_, context_.data_model_.arboretum_node_for(D->isSugared()));
  return true;
}

bool ArboretumASTVisitor::VisitDeducedTemplateSpecializationType(clang::DeducedTemplateSpecializationType* D) {
  const Id* obj = context_.resolve(D);
  // getTemplateName ( class clang::TemplateName )
  return true;
}

bool ArboretumASTVisitor::VisitDeducedType(clang::DeducedType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_10_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_10_, context_.resolve(D->desugar()));
  arboretum_create_edge(obj, context_.data_model_.method_getDeducedType_, context_.resolve(D->getDeducedType()));
  arboretum_create_edge(obj, context_.data_model_.method_isDeduced_, context_.data_model_.arboretum_node_for(D->isDeduced()));
  return true;
}

bool ArboretumASTVisitor::VisitDependentAddressSpaceType(clang::DependentAddressSpaceType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getAddrSpaceExpr_, context_.resolve(D->getAddrSpaceExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getPointeeType_2_, context_.resolve(D->getPointeeType()));
  arboretum_create_edge(obj, context_.data_model_.method_getAttributeLoc_, context_.source_model_.resolve(D->getAttributeLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_11_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_11_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitDependentBitIntType(clang::DependentBitIntType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_isUnsigned_1_, context_.data_model_.arboretum_node_for(D->isUnsigned()));
  arboretum_create_edge(obj, context_.data_model_.method_isSigned_1_, context_.data_model_.arboretum_node_for(D->isSigned()));
  arboretum_create_edge(obj, context_.data_model_.method_getNumBitsExpr_, context_.resolve(D->getNumBitsExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_12_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_12_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitDependentDecltypeType(clang::DependentDecltypeType* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitDependentNameType(clang::DependentNameType* D) {
  const Id* obj = context_.resolve(D);
  // getQualifier ( class clang::NestedNameSpecifier * )
  // getIdentifier ( const class clang::IdentifierInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_13_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_13_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitDependentSizedArrayType(clang::DependentSizedArrayType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getSizeExpr_1_, context_.resolve(D->getSizeExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getBracketsRange_, context_.source_model_.resolve(D->getBracketsRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getLBracketLoc_, context_.source_model_.resolve(D->getLBracketLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRBracketLoc_, context_.source_model_.resolve(D->getRBracketLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_14_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_14_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitDependentSizedExtVectorType(clang::DependentSizedExtVectorType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getSizeExpr_2_, context_.resolve(D->getSizeExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getElementType_2_, context_.resolve(D->getElementType()));
  arboretum_create_edge(obj, context_.data_model_.method_getAttributeLoc_1_, context_.source_model_.resolve(D->getAttributeLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_15_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_15_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitDependentSizedMatrixType(clang::DependentSizedMatrixType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getRowExpr_, context_.resolve(D->getRowExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getColumnExpr_, context_.resolve(D->getColumnExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getAttributeLoc_2_, context_.source_model_.resolve(D->getAttributeLoc()));
  return true;
}

bool ArboretumASTVisitor::VisitDependentTemplateSpecializationType(clang::DependentTemplateSpecializationType* D) {
  const Id* obj = context_.resolve(D);
  // getQualifier ( class clang::NestedNameSpecifier * )
  // getIdentifier ( const class clang::IdentifierInfo * )
  // template_arguments ( class llvm::ArrayRef<class clang::TemplateArgument> )
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_16_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_16_, context_.resolve(D->desugar()));
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
  arboretum_create_edge(obj, context_.data_model_.method_getSizeExpr_3_, context_.resolve(D->getSizeExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getElementType_3_, context_.resolve(D->getElementType()));
  arboretum_create_edge(obj, context_.data_model_.method_getAttributeLoc_3_, context_.source_model_.resolve(D->getAttributeLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getVectorKind_, context_.data_model_.resolve(D->getVectorKind()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_17_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_17_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitElaboratedType(clang::ElaboratedType* D) {
  const Id* obj = context_.resolve(D);
  // getQualifier ( class clang::NestedNameSpecifier * )
  arboretum_create_edge(obj, context_.data_model_.method_getNamedType_, context_.resolve(D->getNamedType()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_18_, context_.resolve(D->desugar()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_18_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_getOwnedTagDecl_, context_.resolve(D->getOwnedTagDecl()));
  return true;
}

bool ArboretumASTVisitor::VisitEnumType(clang::EnumType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getDecl_, context_.resolve(D->getDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_19_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_19_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitExtVectorType(clang::ExtVectorType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_20_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_20_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitFunctionNoProtoType(clang::FunctionNoProtoType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_21_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_21_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitFunctionProtoType(clang::FunctionProtoType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getNumParams_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumParams())));
  {
    std::vector<Id*> element_ids;
    auto range = D->getParamTypes();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_getParamTypes_, context_.data_model_.arboretum_node_for(context_.data_model_.qualtype_class_, element_ids));
  }
  // getExtProtoInfo ( struct clang::FunctionProtoType::ExtProtoInfo )
  arboretum_create_edge(obj, context_.data_model_.method_getExceptionSpecType_, context_.data_model_.resolve(D->getExceptionSpecType()));
  arboretum_create_edge(obj, context_.data_model_.method_hasExceptionSpec_, context_.data_model_.arboretum_node_for(D->hasExceptionSpec()));
  arboretum_create_edge(obj, context_.data_model_.method_hasDynamicExceptionSpec_, context_.data_model_.arboretum_node_for(D->hasDynamicExceptionSpec()));
  arboretum_create_edge(obj, context_.data_model_.method_hasNoexceptExceptionSpec_, context_.data_model_.arboretum_node_for(D->hasNoexceptExceptionSpec()));
  arboretum_create_edge(obj, context_.data_model_.method_hasDependentExceptionSpec_, context_.data_model_.arboretum_node_for(D->hasDependentExceptionSpec()));
  arboretum_create_edge(obj, context_.data_model_.method_hasInstantiationDependentExceptionSpec_, context_.data_model_.arboretum_node_for(D->hasInstantiationDependentExceptionSpec()));
  // getExceptionSpecInfo ( struct clang::FunctionProtoType::ExceptionSpecInfo )
  arboretum_create_edge(obj, context_.data_model_.method_getNumExceptions_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumExceptions())));
  arboretum_create_edge(obj, context_.data_model_.method_getNoexceptExpr_, context_.resolve(D->getNoexceptExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getExceptionSpecDecl_, context_.resolve(D->getExceptionSpecDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getExceptionSpecTemplate_, context_.resolve(D->getExceptionSpecTemplate()));
  arboretum_create_edge(obj, context_.data_model_.method_canThrow_, context_.data_model_.resolve(D->canThrow()));
  arboretum_create_edge(obj, context_.data_model_.method_isVariadic_, context_.data_model_.arboretum_node_for(D->isVariadic()));
  arboretum_create_edge(obj, context_.data_model_.method_getEllipsisLoc_, context_.source_model_.resolve(D->getEllipsisLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_isTemplateVariadic_, context_.data_model_.arboretum_node_for(D->isTemplateVariadic()));
  arboretum_create_edge(obj, context_.data_model_.method_hasTrailingReturn_, context_.data_model_.arboretum_node_for(D->hasTrailingReturn()));
  // getMethodQuals ( class clang::Qualifiers )
  arboretum_create_edge(obj, context_.data_model_.method_getRefQualifier_, context_.data_model_.resolve(D->getRefQualifier()));
  {
    std::vector<Id*> element_ids;
    auto range = D->param_types();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_param_types_, context_.data_model_.arboretum_node_for(context_.data_model_.qualtype_class_, element_ids));
  }
  {
    std::vector<Id*> element_ids;
    auto range = D->exceptions();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_exceptions_, context_.data_model_.arboretum_node_for(context_.data_model_.qualtype_class_, element_ids));
  }
  arboretum_create_edge(obj, context_.data_model_.method_hasExtParameterInfos_, context_.data_model_.arboretum_node_for(D->hasExtParameterInfos()));
  // getExtParameterInfos ( class llvm::ArrayRef<class clang::FunctionType::ExtParameterInfo> )
  // getExtParameterInfosOrNull ( const class clang::FunctionType::ExtParameterInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getAArch64SMEAttributes_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getAArch64SMEAttributes())));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_22_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_22_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitFunctionType(clang::FunctionType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getReturnType_, context_.resolve(D->getReturnType()));
  arboretum_create_edge(obj, context_.data_model_.method_getHasRegParm_, context_.data_model_.arboretum_node_for(D->getHasRegParm()));
  arboretum_create_edge(obj, context_.data_model_.method_getRegParmType_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getRegParmType())));
  arboretum_create_edge(obj, context_.data_model_.method_getNoReturnAttr_, context_.data_model_.arboretum_node_for(D->getNoReturnAttr()));
  arboretum_create_edge(obj, context_.data_model_.method_getCmseNSCallAttr_, context_.data_model_.arboretum_node_for(D->getCmseNSCallAttr()));
  arboretum_create_edge(obj, context_.data_model_.method_getCallConv_, context_.data_model_.resolve(D->getCallConv()));
  // getExtInfo ( class clang::FunctionType::ExtInfo )
  arboretum_create_edge(obj, context_.data_model_.method_isConst_, context_.data_model_.arboretum_node_for(D->isConst()));
  arboretum_create_edge(obj, context_.data_model_.method_isVolatile_, context_.data_model_.arboretum_node_for(D->isVolatile()));
  arboretum_create_edge(obj, context_.data_model_.method_isRestrict_, context_.data_model_.arboretum_node_for(D->isRestrict()));
  return true;
}

bool ArboretumASTVisitor::VisitIncompleteArrayType(clang::IncompleteArrayType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_23_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_23_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitInjectedClassNameType(clang::InjectedClassNameType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getInjectedSpecializationType_, context_.resolve(D->getInjectedSpecializationType()));
  arboretum_create_edge(obj, context_.data_model_.method_getInjectedTST_, context_.resolve(D->getInjectedTST()));
  // getTemplateName ( class clang::TemplateName )
  arboretum_create_edge(obj, context_.data_model_.method_getDecl_1_, context_.resolve(D->getDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_24_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_24_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitLValueReferenceType(clang::LValueReferenceType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_25_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_25_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitMacroQualifiedType(clang::MacroQualifiedType* D) {
  const Id* obj = context_.resolve(D);
  // getMacroIdentifier ( const class clang::IdentifierInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getUnderlyingType_1_, context_.resolve(D->getUnderlyingType()));
  arboretum_create_edge(obj, context_.data_model_.method_getModifiedType_1_, context_.resolve(D->getModifiedType()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_26_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_26_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitMatrixType(clang::MatrixType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getElementType_4_, context_.resolve(D->getElementType()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_27_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_27_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitMemberPointerType(clang::MemberPointerType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getPointeeType_3_, context_.resolve(D->getPointeeType()));
  arboretum_create_edge(obj, context_.data_model_.method_isMemberFunctionPointer_, context_.data_model_.arboretum_node_for(D->isMemberFunctionPointer()));
  arboretum_create_edge(obj, context_.data_model_.method_isMemberDataPointer_, context_.data_model_.arboretum_node_for(D->isMemberDataPointer()));
  arboretum_create_edge(obj, context_.data_model_.method_getClass_, context_.resolve(D->getClass()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_28_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_28_, context_.resolve(D->desugar()));
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
  arboretum_create_edge(obj, context_.data_model_.method_getPattern_, context_.resolve(D->getPattern()));
  // getNumExpansions ( class std::optional<unsigned int> )
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_29_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_29_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitParenType(clang::ParenType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getInnerType_, context_.resolve(D->getInnerType()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_30_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_30_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitPipeType(clang::PipeType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getElementType_5_, context_.resolve(D->getElementType()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_31_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_31_, context_.resolve(D->desugar()));
  arboretum_create_edge(obj, context_.data_model_.method_isReadOnly_, context_.data_model_.arboretum_node_for(D->isReadOnly()));
  return true;
}

bool ArboretumASTVisitor::VisitPointerType(clang::PointerType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getPointeeType_4_, context_.resolve(D->getPointeeType()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_32_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_32_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitRValueReferenceType(clang::RValueReferenceType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_33_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_33_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitRecordType(clang::RecordType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getDecl_2_, context_.resolve(D->getDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_hasConstFields_, context_.data_model_.arboretum_node_for(D->hasConstFields()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_34_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_34_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitReferenceType(clang::ReferenceType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_isSpelledAsLValue_, context_.data_model_.arboretum_node_for(D->isSpelledAsLValue()));
  arboretum_create_edge(obj, context_.data_model_.method_isInnerRef_, context_.data_model_.arboretum_node_for(D->isInnerRef()));
  arboretum_create_edge(obj, context_.data_model_.method_getPointeeTypeAsWritten_, context_.resolve(D->getPointeeTypeAsWritten()));
  arboretum_create_edge(obj, context_.data_model_.method_getPointeeType_5_, context_.resolve(D->getPointeeType()));
  return true;
}

bool ArboretumASTVisitor::VisitSubstTemplateTypeParmPackType(clang::SubstTemplateTypeParmPackType* D) {
  const Id* obj = context_.resolve(D);
  // getIdentifier ( class clang::IdentifierInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getAssociatedDecl_, context_.resolve(D->getAssociatedDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getReplacedParameter_, context_.resolve(D->getReplacedParameter()));
  arboretum_create_edge(obj, context_.data_model_.method_getIndex_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getIndex())));
  arboretum_create_edge(obj, context_.data_model_.method_getFinal_, context_.data_model_.arboretum_node_for(D->getFinal()));
  arboretum_create_edge(obj, context_.data_model_.method_getNumArgs_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumArgs())));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_35_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_35_, context_.resolve(D->desugar()));
  // getArgumentPack ( class clang::TemplateArgument )
  return true;
}

bool ArboretumASTVisitor::VisitSubstTemplateTypeParmType(clang::SubstTemplateTypeParmType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getReplacementType_, context_.resolve(D->getReplacementType()));
  arboretum_create_edge(obj, context_.data_model_.method_getAssociatedDecl_1_, context_.resolve(D->getAssociatedDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getReplacedParameter_1_, context_.resolve(D->getReplacedParameter()));
  arboretum_create_edge(obj, context_.data_model_.method_getIndex_1_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getIndex())));
  // getPackIndex ( class std::optional<unsigned int> )
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_36_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_36_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitTagType(clang::TagType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getDecl_3_, context_.resolve(D->getDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_isBeingDefined_, context_.data_model_.arboretum_node_for(D->isBeingDefined()));
  return true;
}

bool ArboretumASTVisitor::VisitTemplateSpecializationType(clang::TemplateSpecializationType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_isCurrentInstantiation_, context_.data_model_.arboretum_node_for(D->isCurrentInstantiation()));
  arboretum_create_edge(obj, context_.data_model_.method_isTypeAlias_, context_.data_model_.arboretum_node_for(D->isTypeAlias()));
  // getTemplateName ( class clang::TemplateName )
  // template_arguments ( class llvm::ArrayRef<class clang::TemplateArgument> )
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_37_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_37_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitTemplateTypeParmType(clang::TemplateTypeParmType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getDepth_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getDepth())));
  arboretum_create_edge(obj, context_.data_model_.method_getIndex_2_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getIndex())));
  arboretum_create_edge(obj, context_.data_model_.method_isParameterPack_, context_.data_model_.arboretum_node_for(D->isParameterPack()));
  arboretum_create_edge(obj, context_.data_model_.method_getDecl_4_, context_.resolve(D->getDecl()));
  // getIdentifier ( class clang::IdentifierInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_38_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_38_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitType(clang::Type* D) {
  const Id* obj = context_.resolve(D);
  switch(D->getTypeClass()) {
    case clang::Type::LValueReference: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_LValueReferenceType_);
      break; 
    case clang::Type::MemberPointer: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_MemberPointerType_);
      break; 
    case clang::Type::ConstantArray: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ConstantArrayType_);
      break; 
    case clang::Type::IncompleteArray: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_IncompleteArrayType_);
      break; 
    case clang::Type::VariableArray: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_VariableArrayType_);
      break; 
    case clang::Type::DependentSizedExtVector: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DependentSizedExtVectorType_);
      break; 
    case clang::Type::ExtVector: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ExtVectorType_);
      break; 
    case clang::Type::DependentVector: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DependentVectorType_);
      break; 
    case clang::Type::Enum: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_EnumType_);
      break; 
    case clang::Type::Vector: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_VectorType_);
      break; 
    case clang::Type::DependentSizedMatrix: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DependentSizedMatrixType_);
      break; 
    case clang::Type::FunctionNoProto: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_FunctionNoProtoType_);
      break; 
    case clang::Type::FunctionProto: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_FunctionProtoType_);
      break; 
    case clang::Type::Adjusted: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_AdjustedType_);
      break; 
    case clang::Type::Pipe: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_PipeType_);
      break; 
    case clang::Type::UnresolvedUsing: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UnresolvedUsingType_);
      break; 
    case clang::Type::ObjCInterface: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCInterfaceType_);
      break; 
    case clang::Type::Using: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UsingType_);
      break; 
    case clang::Type::RValueReference: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_RValueReferenceType_);
      break; 
    case clang::Type::Typedef: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_TypedefType_);
      break; 
    case clang::Type::DependentAddressSpace: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DependentAddressSpaceType_);
      break; 
    case clang::Type::TypeOfExpr: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_TypeOfExprType_);
      break; 
    case clang::Type::Record: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_RecordType_);
      break; 
    case clang::Type::DependentSizedArray: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DependentSizedArrayType_);
      break; 
    case clang::Type::Builtin: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_BuiltinType_);
      break; 
    case clang::Type::DependentBitInt: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DependentBitIntType_);
      break; 
    case clang::Type::ConstantMatrix: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ConstantMatrixType_);
      break; 
    case clang::Type::Decayed: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DecayedType_);
      break; 
    case clang::Type::SubstTemplateTypeParm: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_SubstTemplateTypeParmType_);
      break; 
    case clang::Type::BitInt: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_BitIntType_);
      break; 
    case clang::Type::DependentName: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DependentNameType_);
      break; 
    case clang::Type::DependentTemplateSpecialization: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DependentTemplateSpecializationType_);
      break; 
    case clang::Type::TemplateTypeParm: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_TemplateTypeParmType_);
      break; 
    case clang::Type::DeducedTemplateSpecialization: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DeducedTemplateSpecializationType_);
      break; 
    case clang::Type::SubstTemplateTypeParmPack: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_SubstTemplateTypeParmPackType_);
      break; 
    case clang::Type::Pointer: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_PointerType_);
      break; 
    case clang::Type::TemplateSpecialization: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_TemplateSpecializationType_);
      break; 
    case clang::Type::BlockPointer: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_BlockPointerType_);
      break; 
    case clang::Type::Complex: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ComplexType_);
      break; 
    case clang::Type::ObjCObjectPointer: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCObjectPointerType_);
      break; 
    case clang::Type::Attributed: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_AttributedType_);
      break; 
    case clang::Type::Paren: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ParenType_);
      break; 
    case clang::Type::Elaborated: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ElaboratedType_);
      break; 
    case clang::Type::InjectedClassName: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_InjectedClassNameType_);
      break; 
    case clang::Type::ObjCObject: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCObjectType_);
      break; 
    case clang::Type::BTFTagAttributed: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_BTFTagAttributedType_);
      break; 
    case clang::Type::Atomic: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_AtomicType_);
      break; 
    case clang::Type::Auto: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_AutoType_);
      break; 
    case clang::Type::PackExpansion: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_PackExpansionType_);
      break; 
    case clang::Type::Decltype: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DecltypeType_);
      break; 
    case clang::Type::TypeOf: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_TypeOfType_);
      break; 
    case clang::Type::MacroQualified: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_MacroQualifiedType_);
      break; 
    case clang::Type::UnaryTransform: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UnaryTransformType_);
      break; 
    case clang::Type::ObjCTypeParam: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCTypeParamType_);
      break; 
    default: break;
  }

  arboretum_create_edge(obj, context_.data_model_.method_containsUnexpandedParameterPack_, context_.data_model_.arboretum_node_for(D->containsUnexpandedParameterPack()));
  arboretum_create_edge(obj, context_.data_model_.method_getLocallyUnqualifiedSingleStepDesugaredType_, context_.resolve(D->getLocallyUnqualifiedSingleStepDesugaredType()));
  arboretum_create_edge(obj, context_.data_model_.method_getAsPlaceholderType_, context_.resolve(D->getAsPlaceholderType()));
  arboretum_create_edge(obj, context_.data_model_.method_getObjCARCImplicitLifetime_, context_.data_model_.resolve(D->getObjCARCImplicitLifetime()));
  arboretum_create_edge(obj, context_.data_model_.method_getDependence_, context_.data_model_.resolve(D->getDependence()));
  arboretum_create_edge(obj, context_.data_model_.method_containsErrors_, context_.data_model_.arboretum_node_for(D->containsErrors()));
  arboretum_create_edge(obj, context_.data_model_.method_hasSizedVLAType_, context_.data_model_.arboretum_node_for(D->hasSizedVLAType()));
  arboretum_create_edge(obj, context_.data_model_.method_hasUnnamedOrLocalType_, context_.data_model_.arboretum_node_for(D->hasUnnamedOrLocalType()));
  arboretum_create_edge(obj, context_.data_model_.method_canDecayToPointerType_, context_.data_model_.arboretum_node_for(D->canDecayToPointerType()));
  arboretum_create_edge(obj, context_.data_model_.method_hasPointerRepresentation_, context_.data_model_.arboretum_node_for(D->hasPointerRepresentation()));
  arboretum_create_edge(obj, context_.data_model_.method_hasObjCPointerRepresentation_, context_.data_model_.arboretum_node_for(D->hasObjCPointerRepresentation()));
  arboretum_create_edge(obj, context_.data_model_.method_hasIntegerRepresentation_, context_.data_model_.arboretum_node_for(D->hasIntegerRepresentation()));
  arboretum_create_edge(obj, context_.data_model_.method_hasSignedIntegerRepresentation_, context_.data_model_.arboretum_node_for(D->hasSignedIntegerRepresentation()));
  arboretum_create_edge(obj, context_.data_model_.method_hasUnsignedIntegerRepresentation_, context_.data_model_.arboretum_node_for(D->hasUnsignedIntegerRepresentation()));
  arboretum_create_edge(obj, context_.data_model_.method_hasFloatingRepresentation_, context_.data_model_.arboretum_node_for(D->hasFloatingRepresentation()));
  arboretum_create_edge(obj, context_.data_model_.method_getAsStructureType_, context_.resolve(D->getAsStructureType()));
  arboretum_create_edge(obj, context_.data_model_.method_getAsUnionType_, context_.resolve(D->getAsUnionType()));
  arboretum_create_edge(obj, context_.data_model_.method_getAsComplexIntegerType_, context_.resolve(D->getAsComplexIntegerType()));
  arboretum_create_edge(obj, context_.data_model_.method_getAsObjCInterfaceType_, context_.resolve(D->getAsObjCInterfaceType()));
  arboretum_create_edge(obj, context_.data_model_.method_getAsObjCInterfacePointerType_, context_.resolve(D->getAsObjCInterfacePointerType()));
  arboretum_create_edge(obj, context_.data_model_.method_getAsObjCQualifiedIdType_, context_.resolve(D->getAsObjCQualifiedIdType()));
  arboretum_create_edge(obj, context_.data_model_.method_getAsObjCQualifiedClassType_, context_.resolve(D->getAsObjCQualifiedClassType()));
  arboretum_create_edge(obj, context_.data_model_.method_getAsObjCQualifiedInterfaceType_, context_.resolve(D->getAsObjCQualifiedInterfaceType()));
  arboretum_create_edge(obj, context_.data_model_.method_getAsCXXRecordDecl_, context_.resolve(D->getAsCXXRecordDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getAsRecordDecl_, context_.resolve(D->getAsRecordDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getAsTagDecl_, context_.resolve(D->getAsTagDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getPointeeCXXRecordDecl_, context_.resolve(D->getPointeeCXXRecordDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getBaseElementTypeUnsafe_, context_.resolve(D->getBaseElementTypeUnsafe()));
  arboretum_create_edge(obj, context_.data_model_.method_getArrayElementTypeNoTypeQual_, context_.resolve(D->getArrayElementTypeNoTypeQual()));
  arboretum_create_edge(obj, context_.data_model_.method_getPointeeOrArrayElementType_, context_.resolve(D->getPointeeOrArrayElementType()));
  arboretum_create_edge(obj, context_.data_model_.method_getLinkage_, context_.data_model_.resolve(D->getLinkage()));
  arboretum_create_edge(obj, context_.data_model_.method_getVisibility_, context_.data_model_.resolve(D->getVisibility()));
  // getLinkageAndVisibility ( class clang::LinkageInfo )
  // getNullability ( class std::optional<enum clang::NullabilityKind> )
  arboretum_create_edge(obj, context_.data_model_.method_acceptsObjCTypeParams_, context_.data_model_.arboretum_node_for(D->acceptsObjCTypeParams()));
  // getTypeClassName ( const char * )
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalTypeInternal_, context_.resolve(D->getCanonicalTypeInternal()));
  return true;
}

bool ArboretumASTVisitor::VisitTypeOfExprType(clang::TypeOfExprType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getUnderlyingExpr_1_, context_.resolve(D->getUnderlyingExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getKind_1_, context_.data_model_.resolve(D->getKind()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_39_, context_.resolve(D->desugar()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_39_, context_.data_model_.arboretum_node_for(D->isSugared()));
  return true;
}

bool ArboretumASTVisitor::VisitTypeOfType(clang::TypeOfType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getUnmodifiedType_, context_.resolve(D->getUnmodifiedType()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_40_, context_.resolve(D->desugar()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_40_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_getKind_2_, context_.data_model_.resolve(D->getKind()));
  return true;
}

bool ArboretumASTVisitor::VisitTypeWithKeyword(clang::TypeWithKeyword* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getKeyword_1_, context_.data_model_.resolve(D->getKeyword()));
  return true;
}

bool ArboretumASTVisitor::VisitTypedefType(clang::TypedefType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getDecl_5_, context_.resolve(D->getDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_41_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_41_, context_.resolve(D->desugar()));
  arboretum_create_edge(obj, context_.data_model_.method_typeMatchesDecl_, context_.data_model_.arboretum_node_for(D->typeMatchesDecl()));
  return true;
}

bool ArboretumASTVisitor::VisitUnaryTransformType(clang::UnaryTransformType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_42_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_42_, context_.resolve(D->desugar()));
  arboretum_create_edge(obj, context_.data_model_.method_getUnderlyingType_2_, context_.resolve(D->getUnderlyingType()));
  arboretum_create_edge(obj, context_.data_model_.method_getBaseType_, context_.resolve(D->getBaseType()));
  arboretum_create_edge(obj, context_.data_model_.method_getUTTKind_, context_.data_model_.resolve(D->getUTTKind()));
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedUsingType(clang::UnresolvedUsingType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getDecl_6_, context_.resolve(D->getDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_43_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_43_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitUsingType(clang::UsingType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getFoundDecl_, context_.resolve(D->getFoundDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getUnderlyingType_3_, context_.resolve(D->getUnderlyingType()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_44_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_44_, context_.resolve(D->desugar()));
  arboretum_create_edge(obj, context_.data_model_.method_typeMatchesDecl_1_, context_.data_model_.arboretum_node_for(D->typeMatchesDecl()));
  return true;
}

bool ArboretumASTVisitor::VisitVariableArrayType(clang::VariableArrayType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getSizeExpr_4_, context_.resolve(D->getSizeExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getBracketsRange_1_, context_.source_model_.resolve(D->getBracketsRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getLBracketLoc_1_, context_.source_model_.resolve(D->getLBracketLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRBracketLoc_1_, context_.source_model_.resolve(D->getRBracketLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_45_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_45_, context_.resolve(D->desugar()));
  return true;
}

bool ArboretumASTVisitor::VisitVectorType(clang::VectorType* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getElementType_6_, context_.resolve(D->getElementType()));
  arboretum_create_edge(obj, context_.data_model_.method_getNumElements_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumElements())));
  arboretum_create_edge(obj, context_.data_model_.method_isSugared_46_, context_.data_model_.arboretum_node_for(D->isSugared()));
  arboretum_create_edge(obj, context_.data_model_.method_desugar_46_, context_.resolve(D->desugar()));
  arboretum_create_edge(obj, context_.data_model_.method_getVectorKind_1_, context_.data_model_.resolve(D->getVectorKind()));
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
  arboretum_create_edge(obj, context_.data_model_.method_getAccessSpecifierLoc_, context_.source_model_.resolve(D->getAccessSpecifierLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getColonLoc_, context_.source_model_.resolve(D->getColonLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_, context_.source_model_.resolve(D->getSourceRange()));
  return true;
}

bool ArboretumASTVisitor::VisitBaseUsingDecl(clang::BaseUsingDecl* D) {
  const Id* obj = context_.resolve(D);
  {
    std::vector<Id*> element_ids;
    auto range = D->shadows();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_shadows_, context_.data_model_.arboretum_node_for(context_.data_model_.class_UsingShadowDecl_, element_ids));
  }
  arboretum_create_edge(obj, context_.data_model_.method_shadow_size_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->shadow_size())));
  return true;
}

bool ArboretumASTVisitor::VisitBindingDecl(clang::BindingDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getBinding_, context_.resolve(D->getBinding()));
  arboretum_create_edge(obj, context_.data_model_.method_getDecomposedDecl_, context_.resolve(D->getDecomposedDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getHoldingVar_, context_.resolve(D->getHoldingVar()));
  return true;
}

bool ArboretumASTVisitor::VisitBlockDecl(clang::BlockDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getCaretLocation_, context_.source_model_.resolve(D->getCaretLocation()));
  arboretum_create_edge(obj, context_.data_model_.method_isVariadic_1_, context_.data_model_.arboretum_node_for(D->isVariadic()));
  arboretum_create_edge(obj, context_.data_model_.method_getCompoundBody_, context_.resolve(D->getCompoundBody()));
  arboretum_create_edge(obj, context_.data_model_.method_getBody_, context_.resolve(D->getBody()));
  // getSignatureAsWritten ( class clang::TypeSourceInfo * )
  {
    std::vector<Id*> element_ids;
    auto range = D->parameters();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_parameters_, context_.data_model_.arboretum_node_for(context_.data_model_.class_ParmVarDecl_, element_ids));
  }
  arboretum_create_edge(obj, context_.data_model_.method_param_empty_, context_.data_model_.arboretum_node_for(D->param_empty()));
  arboretum_create_edge(obj, context_.data_model_.method_param_size_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->param_size())));
  arboretum_create_edge(obj, context_.data_model_.method_getNumParams_1_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumParams())));
  arboretum_create_edge(obj, context_.data_model_.method_hasCaptures_, context_.data_model_.arboretum_node_for(D->hasCaptures()));
  arboretum_create_edge(obj, context_.data_model_.method_getNumCaptures_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumCaptures())));
  // captures ( class llvm::ArrayRef<class clang::BlockDecl::Capture> )
  arboretum_create_edge(obj, context_.data_model_.method_capturesCXXThis_, context_.data_model_.arboretum_node_for(D->capturesCXXThis()));
  arboretum_create_edge(obj, context_.data_model_.method_blockMissingReturnType_, context_.data_model_.arboretum_node_for(D->blockMissingReturnType()));
  arboretum_create_edge(obj, context_.data_model_.method_isConversionFromLambda_, context_.data_model_.arboretum_node_for(D->isConversionFromLambda()));
  arboretum_create_edge(obj, context_.data_model_.method_doesNotEscape_, context_.data_model_.arboretum_node_for(D->doesNotEscape()));
  arboretum_create_edge(obj, context_.data_model_.method_canAvoidCopyToHeap_, context_.data_model_.arboretum_node_for(D->canAvoidCopyToHeap()));
  arboretum_create_edge(obj, context_.data_model_.method_getBlockManglingNumber_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getBlockManglingNumber())));
  arboretum_create_edge(obj, context_.data_model_.method_getBlockManglingContextDecl_, context_.resolve(D->getBlockManglingContextDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_1_, context_.source_model_.resolve(D->getSourceRange()));
  return true;
}

bool ArboretumASTVisitor::VisitBuiltinTemplateDecl(clang::BuiltinTemplateDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_2_, context_.source_model_.resolve(D->getSourceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getBuiltinTemplateKind_, context_.data_model_.resolve(D->getBuiltinTemplateKind()));
  return true;
}

bool ArboretumASTVisitor::VisitCXXConstructorDecl(clang::CXXConstructorDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Id* obj = context_.resolve(D);
  // getExplicitSpecifier ( const class clang::ExplicitSpecifier )
  arboretum_create_edge(obj, context_.data_model_.method_isExplicit_, context_.data_model_.arboretum_node_for(D->isExplicit()));
  // inits ( class llvm::iterator_range<class clang::CXXCtorInitializer *const *> )
  arboretum_create_edge(obj, context_.data_model_.method_getNumCtorInitializers_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumCtorInitializers())));
  arboretum_create_edge(obj, context_.data_model_.method_isDelegatingConstructor_, context_.data_model_.arboretum_node_for(D->isDelegatingConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_isDefaultConstructor_, context_.data_model_.arboretum_node_for(D->isDefaultConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_isCopyConstructor_, context_.data_model_.arboretum_node_for(D->isCopyConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_isMoveConstructor_, context_.data_model_.arboretum_node_for(D->isMoveConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_isCopyOrMoveConstructor_, context_.data_model_.arboretum_node_for(D->isCopyOrMoveConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_isSpecializationCopyingObject_, context_.data_model_.arboretum_node_for(D->isSpecializationCopyingObject()));
  arboretum_create_edge(obj, context_.data_model_.method_isInheritingConstructor_, context_.data_model_.arboretum_node_for(D->isInheritingConstructor()));
  // getInheritedConstructor ( class clang::InheritedConstructor )
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_, context_.resolve(D->getCanonicalDecl()));
  return true;
}

bool ArboretumASTVisitor::VisitCXXConversionDecl(clang::CXXConversionDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Id* obj = context_.resolve(D);
  // getExplicitSpecifier ( const class clang::ExplicitSpecifier )
  arboretum_create_edge(obj, context_.data_model_.method_isExplicit_1_, context_.data_model_.arboretum_node_for(D->isExplicit()));
  arboretum_create_edge(obj, context_.data_model_.method_getConversionType_, context_.resolve(D->getConversionType()));
  arboretum_create_edge(obj, context_.data_model_.method_isLambdaToBlockPointerConversion_, context_.data_model_.arboretum_node_for(D->isLambdaToBlockPointerConversion()));
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_1_, context_.resolve(D->getCanonicalDecl()));
  return true;
}

bool ArboretumASTVisitor::VisitCXXDeductionGuideDecl(clang::CXXDeductionGuideDecl* D) {
  const Id* obj = context_.resolve(D);
  // getExplicitSpecifier ( const class clang::ExplicitSpecifier )
  arboretum_create_edge(obj, context_.data_model_.method_isExplicit_2_, context_.data_model_.arboretum_node_for(D->isExplicit()));
  arboretum_create_edge(obj, context_.data_model_.method_getDeducedTemplate_, context_.resolve(D->getDeducedTemplate()));
  arboretum_create_edge(obj, context_.data_model_.method_getCorrespondingConstructor_, context_.resolve(D->getCorrespondingConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_getDeductionCandidateKind_, context_.data_model_.resolve(D->getDeductionCandidateKind()));
  return true;
}

bool ArboretumASTVisitor::VisitCXXDestructorDecl(clang::CXXDestructorDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getOperatorDelete_, context_.resolve(D->getOperatorDelete()));
  arboretum_create_edge(obj, context_.data_model_.method_getOperatorDeleteThisArg_, context_.resolve(D->getOperatorDeleteThisArg()));
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_2_, context_.resolve(D->getCanonicalDecl()));
  return true;
}

bool ArboretumASTVisitor::VisitCXXMethodDecl(clang::CXXMethodDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_isStatic_, context_.data_model_.arboretum_node_for(D->isStatic()));
  arboretum_create_edge(obj, context_.data_model_.method_isInstance_, context_.data_model_.arboretum_node_for(D->isInstance()));
  arboretum_create_edge(obj, context_.data_model_.method_isExplicitObjectMemberFunction_, context_.data_model_.arboretum_node_for(D->isExplicitObjectMemberFunction()));
  arboretum_create_edge(obj, context_.data_model_.method_isImplicitObjectMemberFunction_, context_.data_model_.arboretum_node_for(D->isImplicitObjectMemberFunction()));
  arboretum_create_edge(obj, context_.data_model_.method_isConst_1_, context_.data_model_.arboretum_node_for(D->isConst()));
  arboretum_create_edge(obj, context_.data_model_.method_isVolatile_1_, context_.data_model_.arboretum_node_for(D->isVolatile()));
  arboretum_create_edge(obj, context_.data_model_.method_isVirtual_, context_.data_model_.arboretum_node_for(D->isVirtual()));
  arboretum_create_edge(obj, context_.data_model_.method_isCopyAssignmentOperator_, context_.data_model_.arboretum_node_for(D->isCopyAssignmentOperator()));
  arboretum_create_edge(obj, context_.data_model_.method_isMoveAssignmentOperator_, context_.data_model_.arboretum_node_for(D->isMoveAssignmentOperator()));
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_3_, context_.resolve(D->getCanonicalDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getMostRecentDecl_, context_.resolve(D->getMostRecentDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_size_overridden_methods_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->size_overridden_methods())));
  {
    std::vector<Id*> element_ids;
    auto range = D->overridden_methods();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_overridden_methods_, context_.data_model_.arboretum_node_for(context_.data_model_.class_CXXMethodDecl_, element_ids));
  }
  arboretum_create_edge(obj, context_.data_model_.method_getParent_, context_.resolve(D->getParent()));
  arboretum_create_edge(obj, context_.data_model_.method_getThisType_, context_.resolve(D->getThisType()));
  arboretum_create_edge(obj, context_.data_model_.method_getFunctionObjectParameterReferenceType_, context_.resolve(D->getFunctionObjectParameterReferenceType()));
  arboretum_create_edge(obj, context_.data_model_.method_getFunctionObjectParameterType_, context_.resolve(D->getFunctionObjectParameterType()));
  arboretum_create_edge(obj, context_.data_model_.method_getNumExplicitParams_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumExplicitParams())));
  // getMethodQualifiers ( class clang::Qualifiers )
  arboretum_create_edge(obj, context_.data_model_.method_getRefQualifier_1_, context_.data_model_.resolve(D->getRefQualifier()));
  arboretum_create_edge(obj, context_.data_model_.method_hasInlineBody_, context_.data_model_.arboretum_node_for(D->hasInlineBody()));
  arboretum_create_edge(obj, context_.data_model_.method_isLambdaStaticInvoker_, context_.data_model_.arboretum_node_for(D->isLambdaStaticInvoker()));
  return true;
}

bool ArboretumASTVisitor::VisitCXXRecordDecl(clang::CXXRecordDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_4_, context_.resolve(D->getCanonicalDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getPreviousDecl_, context_.resolve(D->getPreviousDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getMostRecentDecl_1_, context_.resolve(D->getMostRecentDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getDefinition_, context_.resolve(D->getDefinition()));
  arboretum_create_edge(obj, context_.data_model_.method_hasDefinition_, context_.data_model_.arboretum_node_for(D->hasDefinition()));
  arboretum_create_edge(obj, context_.data_model_.method_isDynamicClass_, context_.data_model_.arboretum_node_for(D->isDynamicClass()));
  arboretum_create_edge(obj, context_.data_model_.method_mayBeDynamicClass_, context_.data_model_.arboretum_node_for(D->mayBeDynamicClass()));
  arboretum_create_edge(obj, context_.data_model_.method_mayBeNonDynamicClass_, context_.data_model_.arboretum_node_for(D->mayBeNonDynamicClass()));
  arboretum_create_edge(obj, context_.data_model_.method_isParsingBaseSpecifiers_, context_.data_model_.arboretum_node_for(D->isParsingBaseSpecifiers()));
  arboretum_create_edge(obj, context_.data_model_.method_getODRHash_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getODRHash())));
  arboretum_create_edge(obj, context_.data_model_.method_getNumBases_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumBases())));
  // bases ( class llvm::iterator_range<const class clang::CXXBaseSpecifier *> )
  arboretum_create_edge(obj, context_.data_model_.method_getNumVBases_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumVBases())));
  // vbases ( class llvm::iterator_range<const class clang::CXXBaseSpecifier *> )
  arboretum_create_edge(obj, context_.data_model_.method_hasAnyDependentBases_, context_.data_model_.arboretum_node_for(D->hasAnyDependentBases()));
  {
    std::vector<Id*> element_ids;
    auto range = D->methods();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_methods_, context_.data_model_.arboretum_node_for(context_.data_model_.class_CXXMethodDecl_, element_ids));
  }
  {
    std::vector<Id*> element_ids;
    auto range = D->ctors();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_ctors_, context_.data_model_.arboretum_node_for(context_.data_model_.class_CXXConstructorDecl_, element_ids));
  }
  {
    std::vector<Id*> element_ids;
    auto range = D->friends();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_friends_, context_.data_model_.arboretum_node_for(context_.data_model_.class_FriendDecl_, element_ids));
  }
  arboretum_create_edge(obj, context_.data_model_.method_hasFriends_, context_.data_model_.arboretum_node_for(D->hasFriends()));
  arboretum_create_edge(obj, context_.data_model_.method_hasSimpleCopyConstructor_, context_.data_model_.arboretum_node_for(D->hasSimpleCopyConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_hasSimpleMoveConstructor_, context_.data_model_.arboretum_node_for(D->hasSimpleMoveConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_hasSimpleCopyAssignment_, context_.data_model_.arboretum_node_for(D->hasSimpleCopyAssignment()));
  arboretum_create_edge(obj, context_.data_model_.method_hasSimpleMoveAssignment_, context_.data_model_.arboretum_node_for(D->hasSimpleMoveAssignment()));
  arboretum_create_edge(obj, context_.data_model_.method_hasSimpleDestructor_, context_.data_model_.arboretum_node_for(D->hasSimpleDestructor()));
  arboretum_create_edge(obj, context_.data_model_.method_hasDefaultConstructor_, context_.data_model_.arboretum_node_for(D->hasDefaultConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_needsImplicitDefaultConstructor_, context_.data_model_.arboretum_node_for(D->needsImplicitDefaultConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_hasUserDeclaredConstructor_, context_.data_model_.arboretum_node_for(D->hasUserDeclaredConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_hasUserProvidedDefaultConstructor_, context_.data_model_.arboretum_node_for(D->hasUserProvidedDefaultConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_hasUserDeclaredCopyConstructor_, context_.data_model_.arboretum_node_for(D->hasUserDeclaredCopyConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_needsImplicitCopyConstructor_, context_.data_model_.arboretum_node_for(D->needsImplicitCopyConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_needsOverloadResolutionForCopyConstructor_, context_.data_model_.arboretum_node_for(D->needsOverloadResolutionForCopyConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_implicitCopyConstructorHasConstParam_, context_.data_model_.arboretum_node_for(D->implicitCopyConstructorHasConstParam()));
  arboretum_create_edge(obj, context_.data_model_.method_hasCopyConstructorWithConstParam_, context_.data_model_.arboretum_node_for(D->hasCopyConstructorWithConstParam()));
  arboretum_create_edge(obj, context_.data_model_.method_hasUserDeclaredMoveOperation_, context_.data_model_.arboretum_node_for(D->hasUserDeclaredMoveOperation()));
  arboretum_create_edge(obj, context_.data_model_.method_hasUserDeclaredMoveConstructor_, context_.data_model_.arboretum_node_for(D->hasUserDeclaredMoveConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_hasMoveConstructor_, context_.data_model_.arboretum_node_for(D->hasMoveConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_needsImplicitMoveConstructor_, context_.data_model_.arboretum_node_for(D->needsImplicitMoveConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_needsOverloadResolutionForMoveConstructor_, context_.data_model_.arboretum_node_for(D->needsOverloadResolutionForMoveConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_hasUserDeclaredCopyAssignment_, context_.data_model_.arboretum_node_for(D->hasUserDeclaredCopyAssignment()));
  arboretum_create_edge(obj, context_.data_model_.method_needsImplicitCopyAssignment_, context_.data_model_.arboretum_node_for(D->needsImplicitCopyAssignment()));
  arboretum_create_edge(obj, context_.data_model_.method_needsOverloadResolutionForCopyAssignment_, context_.data_model_.arboretum_node_for(D->needsOverloadResolutionForCopyAssignment()));
  arboretum_create_edge(obj, context_.data_model_.method_implicitCopyAssignmentHasConstParam_, context_.data_model_.arboretum_node_for(D->implicitCopyAssignmentHasConstParam()));
  arboretum_create_edge(obj, context_.data_model_.method_hasCopyAssignmentWithConstParam_, context_.data_model_.arboretum_node_for(D->hasCopyAssignmentWithConstParam()));
  arboretum_create_edge(obj, context_.data_model_.method_hasUserDeclaredMoveAssignment_, context_.data_model_.arboretum_node_for(D->hasUserDeclaredMoveAssignment()));
  arboretum_create_edge(obj, context_.data_model_.method_hasMoveAssignment_, context_.data_model_.arboretum_node_for(D->hasMoveAssignment()));
  arboretum_create_edge(obj, context_.data_model_.method_needsImplicitMoveAssignment_, context_.data_model_.arboretum_node_for(D->needsImplicitMoveAssignment()));
  arboretum_create_edge(obj, context_.data_model_.method_needsOverloadResolutionForMoveAssignment_, context_.data_model_.arboretum_node_for(D->needsOverloadResolutionForMoveAssignment()));
  arboretum_create_edge(obj, context_.data_model_.method_hasUserDeclaredDestructor_, context_.data_model_.arboretum_node_for(D->hasUserDeclaredDestructor()));
  arboretum_create_edge(obj, context_.data_model_.method_needsImplicitDestructor_, context_.data_model_.arboretum_node_for(D->needsImplicitDestructor()));
  arboretum_create_edge(obj, context_.data_model_.method_needsOverloadResolutionForDestructor_, context_.data_model_.arboretum_node_for(D->needsOverloadResolutionForDestructor()));
  arboretum_create_edge(obj, context_.data_model_.method_isLambda_, context_.data_model_.arboretum_node_for(D->isLambda()));
  arboretum_create_edge(obj, context_.data_model_.method_isGenericLambda_, context_.data_model_.arboretum_node_for(D->isGenericLambda()));
  arboretum_create_edge(obj, context_.data_model_.method_lambdaIsDefaultConstructibleAndAssignable_, context_.data_model_.arboretum_node_for(D->lambdaIsDefaultConstructibleAndAssignable()));
  arboretum_create_edge(obj, context_.data_model_.method_getLambdaCallOperator_, context_.resolve(D->getLambdaCallOperator()));
  arboretum_create_edge(obj, context_.data_model_.method_getDependentLambdaCallOperator_, context_.resolve(D->getDependentLambdaCallOperator()));
  // getGenericLambdaTemplateParameterList ( class clang::TemplateParameterList * )
  {
    std::vector<Id*> element_ids;
    auto range = D->getLambdaExplicitTemplateParameters();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_getLambdaExplicitTemplateParameters_, context_.data_model_.arboretum_node_for(context_.data_model_.class_NamedDecl_, element_ids));
  }
  arboretum_create_edge(obj, context_.data_model_.method_isCapturelessLambda_, context_.data_model_.arboretum_node_for(D->isCapturelessLambda()));
  // captures ( class llvm::iterator_range<const class clang::LambdaCapture *> )
  // getVisibleConversionFunctions ( class llvm::iterator_range<class clang::UnresolvedSetIterator> )
  arboretum_create_edge(obj, context_.data_model_.method_isAggregate_, context_.data_model_.arboretum_node_for(D->isAggregate()));
  arboretum_create_edge(obj, context_.data_model_.method_hasInClassInitializer_, context_.data_model_.arboretum_node_for(D->hasInClassInitializer()));
  arboretum_create_edge(obj, context_.data_model_.method_hasUninitializedReferenceMember_, context_.data_model_.arboretum_node_for(D->hasUninitializedReferenceMember()));
  arboretum_create_edge(obj, context_.data_model_.method_isPOD_, context_.data_model_.arboretum_node_for(D->isPOD()));
  arboretum_create_edge(obj, context_.data_model_.method_isCLike_, context_.data_model_.arboretum_node_for(D->isCLike()));
  arboretum_create_edge(obj, context_.data_model_.method_isEmpty_, context_.data_model_.arboretum_node_for(D->isEmpty()));
  arboretum_create_edge(obj, context_.data_model_.method_hasInitMethod_, context_.data_model_.arboretum_node_for(D->hasInitMethod()));
  arboretum_create_edge(obj, context_.data_model_.method_hasPrivateFields_, context_.data_model_.arboretum_node_for(D->hasPrivateFields()));
  arboretum_create_edge(obj, context_.data_model_.method_hasProtectedFields_, context_.data_model_.arboretum_node_for(D->hasProtectedFields()));
  arboretum_create_edge(obj, context_.data_model_.method_hasDirectFields_, context_.data_model_.arboretum_node_for(D->hasDirectFields()));
  arboretum_create_edge(obj, context_.data_model_.method_isPolymorphic_, context_.data_model_.arboretum_node_for(D->isPolymorphic()));
  arboretum_create_edge(obj, context_.data_model_.method_isAbstract_, context_.data_model_.arboretum_node_for(D->isAbstract()));
  arboretum_create_edge(obj, context_.data_model_.method_isStandardLayout_, context_.data_model_.arboretum_node_for(D->isStandardLayout()));
  arboretum_create_edge(obj, context_.data_model_.method_isCXX11StandardLayout_, context_.data_model_.arboretum_node_for(D->isCXX11StandardLayout()));
  arboretum_create_edge(obj, context_.data_model_.method_hasMutableFields_, context_.data_model_.arboretum_node_for(D->hasMutableFields()));
  arboretum_create_edge(obj, context_.data_model_.method_hasVariantMembers_, context_.data_model_.arboretum_node_for(D->hasVariantMembers()));
  arboretum_create_edge(obj, context_.data_model_.method_hasTrivialDefaultConstructor_, context_.data_model_.arboretum_node_for(D->hasTrivialDefaultConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_hasNonTrivialDefaultConstructor_, context_.data_model_.arboretum_node_for(D->hasNonTrivialDefaultConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_hasConstexprNonCopyMoveConstructor_, context_.data_model_.arboretum_node_for(D->hasConstexprNonCopyMoveConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_defaultedDefaultConstructorIsConstexpr_, context_.data_model_.arboretum_node_for(D->defaultedDefaultConstructorIsConstexpr()));
  arboretum_create_edge(obj, context_.data_model_.method_hasConstexprDefaultConstructor_, context_.data_model_.arboretum_node_for(D->hasConstexprDefaultConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_hasTrivialCopyConstructor_, context_.data_model_.arboretum_node_for(D->hasTrivialCopyConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_hasTrivialCopyConstructorForCall_, context_.data_model_.arboretum_node_for(D->hasTrivialCopyConstructorForCall()));
  arboretum_create_edge(obj, context_.data_model_.method_hasNonTrivialCopyConstructor_, context_.data_model_.arboretum_node_for(D->hasNonTrivialCopyConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_hasNonTrivialCopyConstructorForCall_, context_.data_model_.arboretum_node_for(D->hasNonTrivialCopyConstructorForCall()));
  arboretum_create_edge(obj, context_.data_model_.method_hasTrivialMoveConstructor_, context_.data_model_.arboretum_node_for(D->hasTrivialMoveConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_hasTrivialMoveConstructorForCall_, context_.data_model_.arboretum_node_for(D->hasTrivialMoveConstructorForCall()));
  arboretum_create_edge(obj, context_.data_model_.method_hasNonTrivialMoveConstructor_, context_.data_model_.arboretum_node_for(D->hasNonTrivialMoveConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_hasNonTrivialMoveConstructorForCall_, context_.data_model_.arboretum_node_for(D->hasNonTrivialMoveConstructorForCall()));
  arboretum_create_edge(obj, context_.data_model_.method_hasTrivialCopyAssignment_, context_.data_model_.arboretum_node_for(D->hasTrivialCopyAssignment()));
  arboretum_create_edge(obj, context_.data_model_.method_hasNonTrivialCopyAssignment_, context_.data_model_.arboretum_node_for(D->hasNonTrivialCopyAssignment()));
  arboretum_create_edge(obj, context_.data_model_.method_hasTrivialMoveAssignment_, context_.data_model_.arboretum_node_for(D->hasTrivialMoveAssignment()));
  arboretum_create_edge(obj, context_.data_model_.method_hasNonTrivialMoveAssignment_, context_.data_model_.arboretum_node_for(D->hasNonTrivialMoveAssignment()));
  arboretum_create_edge(obj, context_.data_model_.method_defaultedDestructorIsConstexpr_, context_.data_model_.arboretum_node_for(D->defaultedDestructorIsConstexpr()));
  arboretum_create_edge(obj, context_.data_model_.method_hasConstexprDestructor_, context_.data_model_.arboretum_node_for(D->hasConstexprDestructor()));
  arboretum_create_edge(obj, context_.data_model_.method_hasTrivialDestructor_, context_.data_model_.arboretum_node_for(D->hasTrivialDestructor()));
  arboretum_create_edge(obj, context_.data_model_.method_hasTrivialDestructorForCall_, context_.data_model_.arboretum_node_for(D->hasTrivialDestructorForCall()));
  arboretum_create_edge(obj, context_.data_model_.method_hasNonTrivialDestructor_, context_.data_model_.arboretum_node_for(D->hasNonTrivialDestructor()));
  arboretum_create_edge(obj, context_.data_model_.method_hasNonTrivialDestructorForCall_, context_.data_model_.arboretum_node_for(D->hasNonTrivialDestructorForCall()));
  arboretum_create_edge(obj, context_.data_model_.method_allowConstDefaultInit_, context_.data_model_.arboretum_node_for(D->allowConstDefaultInit()));
  arboretum_create_edge(obj, context_.data_model_.method_hasIrrelevantDestructor_, context_.data_model_.arboretum_node_for(D->hasIrrelevantDestructor()));
  arboretum_create_edge(obj, context_.data_model_.method_hasNonLiteralTypeFieldsOrBases_, context_.data_model_.arboretum_node_for(D->hasNonLiteralTypeFieldsOrBases()));
  arboretum_create_edge(obj, context_.data_model_.method_hasInheritedConstructor_, context_.data_model_.arboretum_node_for(D->hasInheritedConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_hasInheritedAssignment_, context_.data_model_.arboretum_node_for(D->hasInheritedAssignment()));
  arboretum_create_edge(obj, context_.data_model_.method_isTriviallyCopyable_, context_.data_model_.arboretum_node_for(D->isTriviallyCopyable()));
  arboretum_create_edge(obj, context_.data_model_.method_isTriviallyCopyConstructible_, context_.data_model_.arboretum_node_for(D->isTriviallyCopyConstructible()));
  arboretum_create_edge(obj, context_.data_model_.method_isTrivial_, context_.data_model_.arboretum_node_for(D->isTrivial()));
  arboretum_create_edge(obj, context_.data_model_.method_isLiteral_, context_.data_model_.arboretum_node_for(D->isLiteral()));
  arboretum_create_edge(obj, context_.data_model_.method_isStructural_, context_.data_model_.arboretum_node_for(D->isStructural()));
  arboretum_create_edge(obj, context_.data_model_.method_getInstantiatedFromMemberClass_, context_.resolve(D->getInstantiatedFromMemberClass()));
  // getMemberSpecializationInfo ( class clang::MemberSpecializationInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getDescribedClassTemplate_, context_.resolve(D->getDescribedClassTemplate()));
  arboretum_create_edge(obj, context_.data_model_.method_getTemplateSpecializationKind_, context_.data_model_.resolve(D->getTemplateSpecializationKind()));
  arboretum_create_edge(obj, context_.data_model_.method_getTemplateInstantiationPattern_, context_.resolve(D->getTemplateInstantiationPattern()));
  arboretum_create_edge(obj, context_.data_model_.method_getDestructor_, context_.resolve(D->getDestructor()));
  arboretum_create_edge(obj, context_.data_model_.method_isAnyDestructorNoReturn_, context_.data_model_.arboretum_node_for(D->isAnyDestructorNoReturn()));
  arboretum_create_edge(obj, context_.data_model_.method_isLocalClass_, context_.resolve(D->isLocalClass()));
  arboretum_create_edge(obj, context_.data_model_.method_mayBeAbstract_, context_.data_model_.arboretum_node_for(D->mayBeAbstract()));
  arboretum_create_edge(obj, context_.data_model_.method_isEffectivelyFinal_, context_.data_model_.arboretum_node_for(D->isEffectivelyFinal()));
  // getLambdaNumbering ( struct clang::CXXRecordDecl::LambdaNumbering )
  arboretum_create_edge(obj, context_.data_model_.method_getDeviceLambdaManglingNumber_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getDeviceLambdaManglingNumber())));
  arboretum_create_edge(obj, context_.data_model_.method_getMSVtorDispMode_, context_.data_model_.resolve(D->getMSVtorDispMode()));
  arboretum_create_edge(obj, context_.data_model_.method_isDependentLambda_, context_.data_model_.arboretum_node_for(D->isDependentLambda()));
  arboretum_create_edge(obj, context_.data_model_.method_isNeverDependentLambda_, context_.data_model_.arboretum_node_for(D->isNeverDependentLambda()));
  arboretum_create_edge(obj, context_.data_model_.method_getLambdaDependencyKind_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getLambdaDependencyKind())));
  // getLambdaTypeInfo ( class clang::TypeSourceInfo * )
  return true;
}

bool ArboretumASTVisitor::VisitCapturedDecl(clang::CapturedDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getBody_1_, context_.resolve(D->getBody()));
  arboretum_create_edge(obj, context_.data_model_.method_isNothrow_, context_.data_model_.arboretum_node_for(D->isNothrow()));
  arboretum_create_edge(obj, context_.data_model_.method_getNumParams_2_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumParams())));
  {
    std::vector<Id*> element_ids;
    auto range = D->parameters();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_parameters_1_, context_.data_model_.arboretum_node_for(context_.data_model_.class_ImplicitParamDecl_, element_ids));
  }
  arboretum_create_edge(obj, context_.data_model_.method_getContextParam_, context_.resolve(D->getContextParam()));
  arboretum_create_edge(obj, context_.data_model_.method_getContextParamPosition_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getContextParamPosition())));
  return true;
}

bool ArboretumASTVisitor::VisitClassTemplateDecl(clang::ClassTemplateDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getTemplatedDecl_, context_.resolve(D->getTemplatedDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_isThisDeclarationADefinition_, context_.data_model_.arboretum_node_for(D->isThisDeclarationADefinition()));
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_5_, context_.resolve(D->getCanonicalDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getPreviousDecl_1_, context_.resolve(D->getPreviousDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getMostRecentDecl_2_, context_.resolve(D->getMostRecentDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getInstantiatedFromMemberTemplate_, context_.resolve(D->getInstantiatedFromMemberTemplate()));
  // specializations ( class llvm::iterator_range<struct clang::RedeclarableTemplateDecl::SpecIterator<class clang::ClassTemplateSpecializationDecl> > )
  return true;
}

bool ArboretumASTVisitor::VisitClassTemplatePartialSpecializationDecl(clang::ClassTemplatePartialSpecializationDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Id* obj = context_.resolve(D);
  // getTemplateParameters ( class clang::TemplateParameterList * )
  arboretum_create_edge(obj, context_.data_model_.method_hasAssociatedConstraints_, context_.data_model_.arboretum_node_for(D->hasAssociatedConstraints()));
  // getTemplateArgsAsWritten ( const struct clang::ASTTemplateArgumentListInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getInstantiatedFromMember_, context_.resolve(D->getInstantiatedFromMember()));
  arboretum_create_edge(obj, context_.data_model_.method_getInstantiatedFromMemberTemplate_1_, context_.resolve(D->getInstantiatedFromMemberTemplate()));
  arboretum_create_edge(obj, context_.data_model_.method_getInjectedSpecializationType_1_, context_.resolve(D->getInjectedSpecializationType()));
  return true;
}

bool ArboretumASTVisitor::VisitClassTemplateSpecializationDecl(clang::ClassTemplateSpecializationDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getSpecializedTemplate_, context_.resolve(D->getSpecializedTemplate()));
  // getTemplateArgs ( const class clang::TemplateArgumentList & )
  arboretum_create_edge(obj, context_.data_model_.method_getSpecializationKind_, context_.data_model_.resolve(D->getSpecializationKind()));
  arboretum_create_edge(obj, context_.data_model_.method_isExplicitSpecialization_, context_.data_model_.arboretum_node_for(D->isExplicitSpecialization()));
  arboretum_create_edge(obj, context_.data_model_.method_isClassScopeExplicitSpecialization_, context_.data_model_.arboretum_node_for(D->isClassScopeExplicitSpecialization()));
  arboretum_create_edge(obj, context_.data_model_.method_isExplicitInstantiationOrSpecialization_, context_.data_model_.arboretum_node_for(D->isExplicitInstantiationOrSpecialization()));
  arboretum_create_edge(obj, context_.data_model_.method_getPointOfInstantiation_, context_.source_model_.resolve(D->getPointOfInstantiation()));
  // getInstantiatedFrom ( class llvm::PointerUnion<class clang::ClassTemplateDecl *, class clang::ClassTemplatePartialSpecializationDecl *> )
  // getSpecializedTemplateOrPartial ( class llvm::PointerUnion<class clang::ClassTemplateDecl *, class clang::ClassTemplatePartialSpecializationDecl *> )
  // getTemplateInstantiationArgs ( const class clang::TemplateArgumentList & )
  // getTypeAsWritten ( class clang::TypeSourceInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getExternLoc_, context_.source_model_.resolve(D->getExternLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getTemplateKeywordLoc_, context_.source_model_.resolve(D->getTemplateKeywordLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_3_, context_.source_model_.resolve(D->getSourceRange()));
  return true;
}

bool ArboretumASTVisitor::VisitConceptDecl(clang::ConceptDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getConstraintExpr_, context_.resolve(D->getConstraintExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_4_, context_.source_model_.resolve(D->getSourceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_isTypeConcept_, context_.data_model_.arboretum_node_for(D->isTypeConcept()));
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_6_, context_.resolve(D->getCanonicalDecl()));
  return true;
}

bool ArboretumASTVisitor::VisitConstructorUsingShadowDecl(clang::ConstructorUsingShadowDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getIntroducer_, context_.resolve(D->getIntroducer()));
  arboretum_create_edge(obj, context_.data_model_.method_getParent_1_, context_.resolve(D->getParent()));
  arboretum_create_edge(obj, context_.data_model_.method_getNominatedBaseClassShadowDecl_, context_.resolve(D->getNominatedBaseClassShadowDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getConstructedBaseClassShadowDecl_, context_.resolve(D->getConstructedBaseClassShadowDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getNominatedBaseClass_, context_.resolve(D->getNominatedBaseClass()));
  arboretum_create_edge(obj, context_.data_model_.method_getConstructedBaseClass_, context_.resolve(D->getConstructedBaseClass()));
  arboretum_create_edge(obj, context_.data_model_.method_constructsVirtualBase_, context_.data_model_.arboretum_node_for(D->constructsVirtualBase()));
  return true;
}

bool ArboretumASTVisitor::VisitDecl(clang::Decl* D) {
  const Id* obj = context_.resolve(D);
  switch(D->getKind()) {
    case clang::Decl::Record: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_RecordDecl_);
      break; 
    case clang::Decl::HLSLBuffer: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_HLSLBufferDecl_);
      break; 
    case clang::Decl::ConstructorUsingShadow: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ConstructorUsingShadowDecl_);
      break; 
    case clang::Decl::UsingShadow: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UsingShadowDecl_);
      break; 
    case clang::Decl::Label: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_LabelDecl_);
      break; 
    case clang::Decl::Using: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UsingDecl_);
      break; 
    case clang::Decl::CXXRecord: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXRecordDecl_);
      break; 
    case clang::Decl::TopLevelStmt: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_TopLevelStmtDecl_);
      break; 
    case clang::Decl::UsingEnum: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UsingEnumDecl_);
      break; 
    case clang::Decl::ClassTemplate: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ClassTemplateDecl_);
      break; 
    case clang::Decl::ObjCTypeParam: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCTypeParamDecl_);
      break; 
    case clang::Decl::TemplateTemplateParm: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_TemplateTemplateParmDecl_);
      break; 
    case clang::Decl::Concept: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ConceptDecl_);
      break; 
    case clang::Decl::ObjCMethod: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCMethodDecl_);
      break; 
    case clang::Decl::ObjCProperty: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCPropertyDecl_);
      break; 
    case clang::Decl::Block: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_BlockDecl_);
      break; 
    case clang::Decl::UnresolvedUsingIfExists: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UnresolvedUsingIfExistsDecl_);
      break; 
    case clang::Decl::ObjCProtocol: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCProtocolDecl_);
      break; 
    case clang::Decl::OMPAllocate: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPAllocateDecl_);
      break; 
    case clang::Decl::ObjCInterface: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCInterfaceDecl_);
      break; 
    case clang::Decl::StaticAssert: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_StaticAssertDecl_);
      break; 
    case clang::Decl::ObjCPropertyImpl: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCPropertyImplDecl_);
      break; 
    case clang::Decl::CXXMethod: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXMethodDecl_);
      break; 
    case clang::Decl::Empty: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_EmptyDecl_);
      break; 
    case clang::Decl::Import: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ImportDecl_);
      break; 
    case clang::Decl::TranslationUnit: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_TranslationUnitDecl_);
      break; 
    case clang::Decl::CXXDeductionGuide: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXDeductionGuideDecl_);
      break; 
    case clang::Decl::ObjCCategoryImpl: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCCategoryImplDecl_);
      break; 
    case clang::Decl::BuiltinTemplate: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_BuiltinTemplateDecl_);
      break; 
    case clang::Decl::Typedef: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_TypedefDecl_);
      break; 
    case clang::Decl::OMPDeclareReduction: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPDeclareReductionDecl_);
      break; 
    case clang::Decl::UsingPack: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UsingPackDecl_);
      break; 
    case clang::Decl::TypeAliasTemplate: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_TypeAliasTemplateDecl_);
      break; 
    case clang::Decl::UnresolvedUsingTypename: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UnresolvedUsingTypenameDecl_);
      break; 
    case clang::Decl::ObjCImplementation: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCImplementationDecl_);
      break; 
    case clang::Decl::OMPDeclareMapper: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPDeclareMapperDecl_);
      break; 
    case clang::Decl::Namespace: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_NamespaceDecl_);
      break; 
    case clang::Decl::UnnamedGlobalConstant: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UnnamedGlobalConstantDecl_);
      break; 
    case clang::Decl::ExternCContext: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ExternCContextDecl_);
      break; 
    case clang::Decl::Enum: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_EnumDecl_);
      break; 
    case clang::Decl::MSGuid: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_MSGuidDecl_);
      break; 
    case clang::Decl::Friend: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_FriendDecl_);
      break; 
    case clang::Decl::RequiresExprBody: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_RequiresExprBodyDecl_);
      break; 
    case clang::Decl::UnresolvedUsingValue: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UnresolvedUsingValueDecl_);
      break; 
    case clang::Decl::CXXConstructor: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXConstructorDecl_);
      break; 
    case clang::Decl::NonTypeTemplateParm: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_NonTypeTemplateParmDecl_);
      break; 
    case clang::Decl::ObjCIvar: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCIvarDecl_);
      break; 
    case clang::Decl::VarTemplatePartialSpecialization: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_VarTemplatePartialSpecializationDecl_);
      break; 
    case clang::Decl::Field: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_FieldDecl_);
      break; 
    case clang::Decl::Captured: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CapturedDecl_);
      break; 
    case clang::Decl::VarTemplateSpecialization: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_VarTemplateSpecializationDecl_);
      break; 
    case clang::Decl::ImplicitConceptSpecialization: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ImplicitConceptSpecializationDecl_);
      break; 
    case clang::Decl::PragmaDetectMismatch: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_PragmaDetectMismatchDecl_);
      break; 
    case clang::Decl::TypeAlias: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_TypeAliasDecl_);
      break; 
    case clang::Decl::Decomposition: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DecompositionDecl_);
      break; 
    case clang::Decl::OMPCapturedExpr: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPCapturedExprDecl_);
      break; 
    case clang::Decl::ClassTemplatePartialSpecialization: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ClassTemplatePartialSpecializationDecl_);
      break; 
    case clang::Decl::ImplicitParam: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ImplicitParamDecl_);
      break; 
    case clang::Decl::ParmVar: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ParmVarDecl_);
      break; 
    case clang::Decl::FriendTemplate: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_FriendTemplateDecl_);
      break; 
    case clang::Decl::CXXDestructor: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXDestructorDecl_);
      break; 
    case clang::Decl::CXXConversion: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXConversionDecl_);
      break; 
    case clang::Decl::IndirectField: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_IndirectFieldDecl_);
      break; 
    case clang::Decl::ObjCCompatibleAlias: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCCompatibleAliasDecl_);
      break; 
    case clang::Decl::EnumConstant: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_EnumConstantDecl_);
      break; 
    case clang::Decl::PragmaComment: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_PragmaCommentDecl_);
      break; 
    case clang::Decl::ObjCAtDefsField: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCAtDefsFieldDecl_);
      break; 
    case clang::Decl::Function: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_FunctionDecl_);
      break; 
    case clang::Decl::MSProperty: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_MSPropertyDecl_);
      break; 
    case clang::Decl::TemplateParamObject: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_TemplateParamObjectDecl_);
      break; 
    case clang::Decl::NamespaceAlias: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_NamespaceAliasDecl_);
      break; 
    case clang::Decl::ClassTemplateSpecialization: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ClassTemplateSpecializationDecl_);
      break; 
    case clang::Decl::VarTemplate: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_VarTemplateDecl_);
      break; 
    case clang::Decl::UsingDirective: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UsingDirectiveDecl_);
      break; 
    case clang::Decl::ObjCCategory: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCCategoryDecl_);
      break; 
    case clang::Decl::Var: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_VarDecl_);
      break; 
    case clang::Decl::FileScopeAsm: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_FileScopeAsmDecl_);
      break; 
    case clang::Decl::OMPRequires: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPRequiresDecl_);
      break; 
    case clang::Decl::Export: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ExportDecl_);
      break; 
    case clang::Decl::LifetimeExtendedTemporary: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_LifetimeExtendedTemporaryDecl_);
      break; 
    case clang::Decl::AccessSpec: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_AccessSpecDecl_);
      break; 
    case clang::Decl::FunctionTemplate: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_FunctionTemplateDecl_);
      break; 
    case clang::Decl::TemplateTypeParm: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_TemplateTypeParmDecl_);
      break; 
    case clang::Decl::OMPThreadPrivate: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPThreadPrivateDecl_);
      break; 
    case clang::Decl::Binding: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_BindingDecl_);
      break; 
    case clang::Decl::LinkageSpec: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_LinkageSpecDecl_);
      break; 
    default: break;
  }

  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_5_, context_.source_model_.resolve(D->getSourceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getLocation_, context_.source_model_.resolve(D->getLocation()));
  arboretum_create_edge(obj, context_.data_model_.method_getNextDeclInContext_, context_.resolve(D->getNextDeclInContext()));
  // getDeclContext ( const class clang::DeclContext * )
  // getNonTransparentDeclContext ( const class clang::DeclContext * )
  arboretum_create_edge(obj, context_.data_model_.method_getNonClosureContext_, context_.resolve(D->getNonClosureContext()));
  arboretum_create_edge(obj, context_.data_model_.method_getTranslationUnitDecl_, context_.resolve(D->getTranslationUnitDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_isInAnonymousNamespace_, context_.data_model_.arboretum_node_for(D->isInAnonymousNamespace()));
  arboretum_create_edge(obj, context_.data_model_.method_isInStdNamespace_, context_.data_model_.arboretum_node_for(D->isInStdNamespace()));
  arboretum_create_edge(obj, context_.data_model_.method_isFileContextDecl_, context_.data_model_.arboretum_node_for(D->isFileContextDecl()));
  // getLangOpts ( const class clang::LangOptions & )
  arboretum_create_edge(obj, context_.data_model_.method_getAccess_, context_.data_model_.resolve(D->getAccess()));
  arboretum_create_edge(obj, context_.data_model_.method_getAccessUnsafe_, context_.data_model_.resolve(D->getAccessUnsafe()));
  arboretum_create_edge(obj, context_.data_model_.method_hasAttrs_, context_.data_model_.arboretum_node_for(D->hasAttrs()));
  // getAttrs ( const class llvm::SmallVector<class clang::Attr *, 4> & )
  {
    std::vector<Id*> element_ids;
    auto range = D->attrs();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_attrs_, context_.data_model_.arboretum_node_for(context_.data_model_.class_Attr_, element_ids));
  }
  arboretum_create_edge(obj, context_.data_model_.method_isInvalidDecl_, context_.data_model_.arboretum_node_for(D->isInvalidDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_isImplicit_, context_.data_model_.arboretum_node_for(D->isImplicit()));
  arboretum_create_edge(obj, context_.data_model_.method_isReferenced_, context_.data_model_.arboretum_node_for(D->isReferenced()));
  arboretum_create_edge(obj, context_.data_model_.method_isThisDeclarationReferenced_, context_.data_model_.arboretum_node_for(D->isThisDeclarationReferenced()));
  arboretum_create_edge(obj, context_.data_model_.method_isTopLevelDeclInObjCContainer_, context_.data_model_.arboretum_node_for(D->isTopLevelDeclInObjCContainer()));
  arboretum_create_edge(obj, context_.data_model_.method_isModulePrivate_, context_.data_model_.arboretum_node_for(D->isModulePrivate()));
  arboretum_create_edge(obj, context_.data_model_.method_isInExportDeclContext_, context_.data_model_.arboretum_node_for(D->isInExportDeclContext()));
  arboretum_create_edge(obj, context_.data_model_.method_isInvisibleOutsideTheOwningModule_, context_.data_model_.arboretum_node_for(D->isInvisibleOutsideTheOwningModule()));
  arboretum_create_edge(obj, context_.data_model_.method_isInAnotherModuleUnit_, context_.data_model_.arboretum_node_for(D->isInAnotherModuleUnit()));
  arboretum_create_edge(obj, context_.data_model_.method_isDiscardedInGlobalModuleFragment_, context_.data_model_.arboretum_node_for(D->isDiscardedInGlobalModuleFragment()));
  arboretum_create_edge(obj, context_.data_model_.method_shouldSkipCheckingODR_, context_.data_model_.arboretum_node_for(D->shouldSkipCheckingODR()));
  arboretum_create_edge(obj, context_.data_model_.method_hasDefiningAttr_, context_.data_model_.arboretum_node_for(D->hasDefiningAttr()));
  arboretum_create_edge(obj, context_.data_model_.method_getDefiningAttr_, context_.resolve(D->getDefiningAttr()));
  // getVersionIntroduced ( class llvm::VersionTuple )
  arboretum_create_edge(obj, context_.data_model_.method_isWeakImported_, context_.data_model_.arboretum_node_for(D->isWeakImported()));
  arboretum_create_edge(obj, context_.data_model_.method_isFromASTFile_, context_.data_model_.arboretum_node_for(D->isFromASTFile()));
  arboretum_create_edge(obj, context_.data_model_.method_getGlobalID_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getGlobalID())));
  arboretum_create_edge(obj, context_.data_model_.method_getOwningModuleID_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getOwningModuleID())));
  // getImportedOwningModule ( class clang::Module * )
  // getLocalOwningModule ( class clang::Module * )
  arboretum_create_edge(obj, context_.data_model_.method_hasOwningModule_, context_.data_model_.arboretum_node_for(D->hasOwningModule()));
  // getOwningModule ( class clang::Module * )
  arboretum_create_edge(obj, context_.data_model_.method_isUnconditionallyVisible_, context_.data_model_.arboretum_node_for(D->isUnconditionallyVisible()));
  arboretum_create_edge(obj, context_.data_model_.method_isReachable_, context_.data_model_.arboretum_node_for(D->isReachable()));
  arboretum_create_edge(obj, context_.data_model_.method_getModuleOwnershipKind_, context_.data_model_.resolve(D->getModuleOwnershipKind()));
  arboretum_create_edge(obj, context_.data_model_.method_getIdentifierNamespace_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getIdentifierNamespace())));
  arboretum_create_edge(obj, context_.data_model_.method_hasTagIdentifierNamespace_, context_.data_model_.arboretum_node_for(D->hasTagIdentifierNamespace()));
  // getLexicalDeclContext ( const class clang::DeclContext * )
  arboretum_create_edge(obj, context_.data_model_.method_isOutOfLine_, context_.data_model_.arboretum_node_for(D->isOutOfLine()));
  arboretum_create_edge(obj, context_.data_model_.method_isTemplated_, context_.data_model_.arboretum_node_for(D->isTemplated()));
  arboretum_create_edge(obj, context_.data_model_.method_getTemplateDepth_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getTemplateDepth())));
  arboretum_create_edge(obj, context_.data_model_.method_isDefinedOutsideFunctionOrMethod_, context_.data_model_.arboretum_node_for(D->isDefinedOutsideFunctionOrMethod()));
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_7_, context_.resolve(D->getCanonicalDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_isCanonicalDecl_, context_.data_model_.arboretum_node_for(D->isCanonicalDecl()));
  {
    std::vector<Id*> element_ids;
    auto range = D->redecls();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_redecls_, context_.data_model_.arboretum_node_for(context_.data_model_.class_Decl_, element_ids));
  }
  arboretum_create_edge(obj, context_.data_model_.method_getPreviousDecl_2_, context_.resolve(D->getPreviousDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_isFirstDecl_, context_.data_model_.arboretum_node_for(D->isFirstDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getMostRecentDecl_3_, context_.resolve(D->getMostRecentDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getBody_2_, context_.resolve(D->getBody()));
  arboretum_create_edge(obj, context_.data_model_.method_hasBody_, context_.data_model_.arboretum_node_for(D->hasBody()));
  arboretum_create_edge(obj, context_.data_model_.method_getBodyRBrace_, context_.source_model_.resolve(D->getBodyRBrace()));
  arboretum_create_edge(obj, context_.data_model_.method_isTemplateParameter_, context_.data_model_.arboretum_node_for(D->isTemplateParameter()));
  arboretum_create_edge(obj, context_.data_model_.method_isTemplateParameterPack_, context_.data_model_.arboretum_node_for(D->isTemplateParameterPack()));
  arboretum_create_edge(obj, context_.data_model_.method_isParameterPack_1_, context_.data_model_.arboretum_node_for(D->isParameterPack()));
  arboretum_create_edge(obj, context_.data_model_.method_isTemplateDecl_, context_.data_model_.arboretum_node_for(D->isTemplateDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_isFunctionOrFunctionTemplate_, context_.data_model_.arboretum_node_for(D->isFunctionOrFunctionTemplate()));
  arboretum_create_edge(obj, context_.data_model_.method_getDescribedTemplate_, context_.resolve(D->getDescribedTemplate()));
  // getDescribedTemplateParams ( const class clang::TemplateParameterList * )
  arboretum_create_edge(obj, context_.data_model_.method_getAsFunction_, context_.resolve(D->getAsFunction()));
  arboretum_create_edge(obj, context_.data_model_.method_isLocalExternDecl_, context_.data_model_.arboretum_node_for(D->isLocalExternDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getFriendObjectKind_, context_.data_model_.resolve(D->getFriendObjectKind()));
  arboretum_create_edge(obj, context_.data_model_.method_getID_, context_.data_model_.arboretum_node_for(static_cast<int64_t>(D->getID())));
  arboretum_create_edge(obj, context_.data_model_.method_isFunctionPointerType_, context_.data_model_.arboretum_node_for(D->isFunctionPointerType()));
  return true;
}

bool ArboretumASTVisitor::VisitDeclaratorDecl(clang::DeclaratorDecl* D) {
  const Id* obj = context_.resolve(D);
  // getTypeSourceInfo ( class clang::TypeSourceInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getInnerLocStart_, context_.source_model_.resolve(D->getInnerLocStart()));
  arboretum_create_edge(obj, context_.data_model_.method_getOuterLocStart_, context_.source_model_.resolve(D->getOuterLocStart()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_6_, context_.source_model_.resolve(D->getSourceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_1_, context_.source_model_.resolve(D->getBeginLoc()));
  // getQualifier ( class clang::NestedNameSpecifier * )
  // getQualifierLoc ( class clang::NestedNameSpecifierLoc )
  arboretum_create_edge(obj, context_.data_model_.method_getTrailingRequiresClause_, context_.resolve(D->getTrailingRequiresClause()));
  arboretum_create_edge(obj, context_.data_model_.method_getNumTemplateParameterLists_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumTemplateParameterLists())));
  arboretum_create_edge(obj, context_.data_model_.method_getTypeSpecStartLoc_, context_.source_model_.resolve(D->getTypeSpecStartLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getTypeSpecEndLoc_, context_.source_model_.resolve(D->getTypeSpecEndLoc()));
  return true;
}

bool ArboretumASTVisitor::VisitDecompositionDecl(clang::DecompositionDecl* D) {
  const Id* obj = context_.resolve(D);
  {
    std::vector<Id*> element_ids;
    auto range = D->bindings();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_bindings_, context_.data_model_.arboretum_node_for(context_.data_model_.class_BindingDecl_, element_ids));
  }
  return true;
}

bool ArboretumASTVisitor::VisitEmptyDecl(clang::EmptyDecl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitEnumConstantDecl(clang::EnumConstantDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getInitExpr_, context_.resolve(D->getInitExpr()));
  // getInitVal ( class llvm::APSInt )
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_7_, context_.source_model_.resolve(D->getSourceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_8_, context_.resolve(D->getCanonicalDecl()));
  return true;
}

bool ArboretumASTVisitor::VisitEnumDecl(clang::EnumDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_9_, context_.resolve(D->getCanonicalDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getPreviousDecl_3_, context_.resolve(D->getPreviousDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getMostRecentDecl_4_, context_.resolve(D->getMostRecentDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getDefinition_1_, context_.resolve(D->getDefinition()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_8_, context_.source_model_.resolve(D->getSourceRange()));
  {
    std::vector<Id*> element_ids;
    auto range = D->enumerators();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_enumerators_, context_.data_model_.arboretum_node_for(context_.data_model_.class_EnumConstantDecl_, element_ids));
  }
  arboretum_create_edge(obj, context_.data_model_.method_getPromotionType_, context_.resolve(D->getPromotionType()));
  arboretum_create_edge(obj, context_.data_model_.method_getIntegerType_, context_.resolve(D->getIntegerType()));
  // getIntegerTypeSourceInfo ( class clang::TypeSourceInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getIntegerTypeRange_, context_.source_model_.resolve(D->getIntegerTypeRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getNumPositiveBits_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumPositiveBits())));
  arboretum_create_edge(obj, context_.data_model_.method_getNumNegativeBits_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumNegativeBits())));
  arboretum_create_edge(obj, context_.data_model_.method_isScoped_, context_.data_model_.arboretum_node_for(D->isScoped()));
  arboretum_create_edge(obj, context_.data_model_.method_isScopedUsingClassTag_, context_.data_model_.arboretum_node_for(D->isScopedUsingClassTag()));
  arboretum_create_edge(obj, context_.data_model_.method_isFixed_, context_.data_model_.arboretum_node_for(D->isFixed()));
  arboretum_create_edge(obj, context_.data_model_.method_isComplete_, context_.data_model_.arboretum_node_for(D->isComplete()));
  arboretum_create_edge(obj, context_.data_model_.method_isClosed_, context_.data_model_.arboretum_node_for(D->isClosed()));
  arboretum_create_edge(obj, context_.data_model_.method_isClosedFlag_, context_.data_model_.arboretum_node_for(D->isClosedFlag()));
  arboretum_create_edge(obj, context_.data_model_.method_isClosedNonFlag_, context_.data_model_.arboretum_node_for(D->isClosedNonFlag()));
  arboretum_create_edge(obj, context_.data_model_.method_getTemplateInstantiationPattern_1_, context_.resolve(D->getTemplateInstantiationPattern()));
  arboretum_create_edge(obj, context_.data_model_.method_getInstantiatedFromMemberEnum_, context_.resolve(D->getInstantiatedFromMemberEnum()));
  arboretum_create_edge(obj, context_.data_model_.method_getTemplateSpecializationKind_1_, context_.data_model_.resolve(D->getTemplateSpecializationKind()));
  // getMemberSpecializationInfo ( class clang::MemberSpecializationInfo * )
  return true;
}

bool ArboretumASTVisitor::VisitExportDecl(clang::ExportDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getExportLoc_, context_.source_model_.resolve(D->getExportLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRBraceLoc_, context_.source_model_.resolve(D->getRBraceLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_hasBraces_, context_.data_model_.arboretum_node_for(D->hasBraces()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_1_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_9_, context_.source_model_.resolve(D->getSourceRange()));
  return true;
}

bool ArboretumASTVisitor::VisitExternCContextDecl(clang::ExternCContextDecl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitFieldDecl(clang::FieldDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getFieldIndex_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getFieldIndex())));
  arboretum_create_edge(obj, context_.data_model_.method_isMutable_, context_.data_model_.arboretum_node_for(D->isMutable()));
  arboretum_create_edge(obj, context_.data_model_.method_isBitField_, context_.data_model_.arboretum_node_for(D->isBitField()));
  arboretum_create_edge(obj, context_.data_model_.method_isUnnamedBitfield_, context_.data_model_.arboretum_node_for(D->isUnnamedBitfield()));
  arboretum_create_edge(obj, context_.data_model_.method_isAnonymousStructOrUnion_, context_.data_model_.arboretum_node_for(D->isAnonymousStructOrUnion()));
  arboretum_create_edge(obj, context_.data_model_.method_getBitWidth_, context_.resolve(D->getBitWidth()));
  arboretum_create_edge(obj, context_.data_model_.method_isPotentiallyOverlapping_, context_.data_model_.arboretum_node_for(D->isPotentiallyOverlapping()));
  arboretum_create_edge(obj, context_.data_model_.method_getInClassInitStyle_, context_.data_model_.resolve(D->getInClassInitStyle()));
  arboretum_create_edge(obj, context_.data_model_.method_hasInClassInitializer_1_, context_.data_model_.arboretum_node_for(D->hasInClassInitializer()));
  arboretum_create_edge(obj, context_.data_model_.method_hasNonNullInClassInitializer_, context_.data_model_.arboretum_node_for(D->hasNonNullInClassInitializer()));
  arboretum_create_edge(obj, context_.data_model_.method_getInClassInitializer_, context_.resolve(D->getInClassInitializer()));
  arboretum_create_edge(obj, context_.data_model_.method_hasCapturedVLAType_, context_.data_model_.arboretum_node_for(D->hasCapturedVLAType()));
  arboretum_create_edge(obj, context_.data_model_.method_getCapturedVLAType_, context_.resolve(D->getCapturedVLAType()));
  arboretum_create_edge(obj, context_.data_model_.method_getParent_2_, context_.resolve(D->getParent()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_10_, context_.source_model_.resolve(D->getSourceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_10_, context_.resolve(D->getCanonicalDecl()));
  return true;
}

bool ArboretumASTVisitor::VisitFileScopeAsmDecl(clang::FileScopeAsmDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getAsmLoc_, context_.source_model_.resolve(D->getAsmLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_, context_.source_model_.resolve(D->getRParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_11_, context_.source_model_.resolve(D->getSourceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getAsmString_, context_.resolve(D->getAsmString()));
  return true;
}

bool ArboretumASTVisitor::VisitFriendDecl(clang::FriendDecl* D) {
  const Id* obj = context_.resolve(D);
  // getFriendType ( class clang::TypeSourceInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getFriendTypeNumTemplateParameterLists_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getFriendTypeNumTemplateParameterLists())));
  arboretum_create_edge(obj, context_.data_model_.method_getFriendDecl_, context_.resolve(D->getFriendDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getFriendLoc_, context_.source_model_.resolve(D->getFriendLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_12_, context_.source_model_.resolve(D->getSourceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_isUnsupportedFriend_, context_.data_model_.arboretum_node_for(D->isUnsupportedFriend()));
  return true;
}

bool ArboretumASTVisitor::VisitFriendTemplateDecl(clang::FriendTemplateDecl* D) {
  const Id* obj = context_.resolve(D);
  // getFriendType ( class clang::TypeSourceInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getFriendDecl_1_, context_.resolve(D->getFriendDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getFriendLoc_1_, context_.source_model_.resolve(D->getFriendLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getNumTemplateParameters_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumTemplateParameters())));
  return true;
}

bool ArboretumASTVisitor::VisitFunctionDecl(clang::FunctionDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Id* obj = context_.resolve(D);
  // getNameInfo ( struct clang::DeclarationNameInfo )
  arboretum_create_edge(obj, context_.data_model_.method_getEllipsisLoc_1_, context_.source_model_.resolve(D->getEllipsisLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_13_, context_.source_model_.resolve(D->getSourceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_hasBody_1_, context_.data_model_.arboretum_node_for(D->hasBody()));
  arboretum_create_edge(obj, context_.data_model_.method_hasTrivialBody_, context_.data_model_.arboretum_node_for(D->hasTrivialBody()));
  arboretum_create_edge(obj, context_.data_model_.method_isDefined_, context_.data_model_.arboretum_node_for(D->isDefined()));
  arboretum_create_edge(obj, context_.data_model_.method_getDefinition_2_, context_.resolve(D->getDefinition()));
  arboretum_create_edge(obj, context_.data_model_.method_getBody_3_, context_.resolve(D->getBody()));
  arboretum_create_edge(obj, context_.data_model_.method_isThisDeclarationADefinition_1_, context_.data_model_.arboretum_node_for(D->isThisDeclarationADefinition()));
  arboretum_create_edge(obj, context_.data_model_.method_isThisDeclarationInstantiatedFromAFriendDefinition_, context_.data_model_.arboretum_node_for(D->isThisDeclarationInstantiatedFromAFriendDefinition()));
  arboretum_create_edge(obj, context_.data_model_.method_doesThisDeclarationHaveABody_, context_.data_model_.arboretum_node_for(D->doesThisDeclarationHaveABody()));
  // getDefaultedFunctionInfo ( class clang::FunctionDecl::DefaultedFunctionInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_isVariadic_2_, context_.data_model_.arboretum_node_for(D->isVariadic()));
  arboretum_create_edge(obj, context_.data_model_.method_isVirtualAsWritten_, context_.data_model_.arboretum_node_for(D->isVirtualAsWritten()));
  arboretum_create_edge(obj, context_.data_model_.method_isPureVirtual_, context_.data_model_.arboretum_node_for(D->isPureVirtual()));
  arboretum_create_edge(obj, context_.data_model_.method_isLateTemplateParsed_, context_.data_model_.arboretum_node_for(D->isLateTemplateParsed()));
  arboretum_create_edge(obj, context_.data_model_.method_isTrivial_1_, context_.data_model_.arboretum_node_for(D->isTrivial()));
  arboretum_create_edge(obj, context_.data_model_.method_isTrivialForCall_, context_.data_model_.arboretum_node_for(D->isTrivialForCall()));
  arboretum_create_edge(obj, context_.data_model_.method_isDefaulted_, context_.data_model_.arboretum_node_for(D->isDefaulted()));
  arboretum_create_edge(obj, context_.data_model_.method_isExplicitlyDefaulted_, context_.data_model_.arboretum_node_for(D->isExplicitlyDefaulted()));
  arboretum_create_edge(obj, context_.data_model_.method_getDefaultLoc_, context_.source_model_.resolve(D->getDefaultLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_isUserProvided_, context_.data_model_.arboretum_node_for(D->isUserProvided()));
  arboretum_create_edge(obj, context_.data_model_.method_isIneligibleOrNotSelected_, context_.data_model_.arboretum_node_for(D->isIneligibleOrNotSelected()));
  arboretum_create_edge(obj, context_.data_model_.method_hasImplicitReturnZero_, context_.data_model_.arboretum_node_for(D->hasImplicitReturnZero()));
  arboretum_create_edge(obj, context_.data_model_.method_hasPrototype_, context_.data_model_.arboretum_node_for(D->hasPrototype()));
  arboretum_create_edge(obj, context_.data_model_.method_hasWrittenPrototype_, context_.data_model_.arboretum_node_for(D->hasWrittenPrototype()));
  arboretum_create_edge(obj, context_.data_model_.method_hasInheritedPrototype_, context_.data_model_.arboretum_node_for(D->hasInheritedPrototype()));
  arboretum_create_edge(obj, context_.data_model_.method_isConstexpr_, context_.data_model_.arboretum_node_for(D->isConstexpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getConstexprKind_, context_.data_model_.resolve(D->getConstexprKind()));
  arboretum_create_edge(obj, context_.data_model_.method_isConstexprSpecified_, context_.data_model_.arboretum_node_for(D->isConstexprSpecified()));
  arboretum_create_edge(obj, context_.data_model_.method_isConsteval_, context_.data_model_.arboretum_node_for(D->isConsteval()));
  arboretum_create_edge(obj, context_.data_model_.method_BodyContainsImmediateEscalatingExpressions_, context_.data_model_.arboretum_node_for(D->BodyContainsImmediateEscalatingExpressions()));
  arboretum_create_edge(obj, context_.data_model_.method_isImmediateEscalating_, context_.data_model_.arboretum_node_for(D->isImmediateEscalating()));
  arboretum_create_edge(obj, context_.data_model_.method_isImmediateFunction_, context_.data_model_.arboretum_node_for(D->isImmediateFunction()));
  arboretum_create_edge(obj, context_.data_model_.method_instantiationIsPending_, context_.data_model_.arboretum_node_for(D->instantiationIsPending()));
  arboretum_create_edge(obj, context_.data_model_.method_usesSEHTry_, context_.data_model_.arboretum_node_for(D->usesSEHTry()));
  arboretum_create_edge(obj, context_.data_model_.method_isDeleted_, context_.data_model_.arboretum_node_for(D->isDeleted()));
  arboretum_create_edge(obj, context_.data_model_.method_isDeletedAsWritten_, context_.data_model_.arboretum_node_for(D->isDeletedAsWritten()));
  arboretum_create_edge(obj, context_.data_model_.method_isMain_, context_.data_model_.arboretum_node_for(D->isMain()));
  arboretum_create_edge(obj, context_.data_model_.method_isMSVCRTEntryPoint_, context_.data_model_.arboretum_node_for(D->isMSVCRTEntryPoint()));
  arboretum_create_edge(obj, context_.data_model_.method_isReservedGlobalPlacementOperator_, context_.data_model_.arboretum_node_for(D->isReservedGlobalPlacementOperator()));
  arboretum_create_edge(obj, context_.data_model_.method_isInlineBuiltinDeclaration_, context_.data_model_.arboretum_node_for(D->isInlineBuiltinDeclaration()));
  arboretum_create_edge(obj, context_.data_model_.method_isDestroyingOperatorDelete_, context_.data_model_.arboretum_node_for(D->isDestroyingOperatorDelete()));
  arboretum_create_edge(obj, context_.data_model_.method_getLanguageLinkage_, context_.data_model_.resolve(D->getLanguageLinkage()));
  arboretum_create_edge(obj, context_.data_model_.method_isExternC_, context_.data_model_.arboretum_node_for(D->isExternC()));
  arboretum_create_edge(obj, context_.data_model_.method_isInExternCContext_, context_.data_model_.arboretum_node_for(D->isInExternCContext()));
  arboretum_create_edge(obj, context_.data_model_.method_isInExternCXXContext_, context_.data_model_.arboretum_node_for(D->isInExternCXXContext()));
  arboretum_create_edge(obj, context_.data_model_.method_isGlobal_, context_.data_model_.arboretum_node_for(D->isGlobal()));
  arboretum_create_edge(obj, context_.data_model_.method_isNoReturn_, context_.data_model_.arboretum_node_for(D->isNoReturn()));
  arboretum_create_edge(obj, context_.data_model_.method_hasSkippedBody_, context_.data_model_.arboretum_node_for(D->hasSkippedBody()));
  arboretum_create_edge(obj, context_.data_model_.method_willHaveBody_, context_.data_model_.arboretum_node_for(D->willHaveBody()));
  arboretum_create_edge(obj, context_.data_model_.method_isMultiVersion_, context_.data_model_.arboretum_node_for(D->isMultiVersion()));
  arboretum_create_edge(obj, context_.data_model_.method_FriendConstraintRefersToEnclosingTemplate_, context_.data_model_.arboretum_node_for(D->FriendConstraintRefersToEnclosingTemplate()));
  arboretum_create_edge(obj, context_.data_model_.method_isMemberLikeConstrainedFriend_, context_.data_model_.arboretum_node_for(D->isMemberLikeConstrainedFriend()));
  arboretum_create_edge(obj, context_.data_model_.method_getMultiVersionKind_, context_.data_model_.resolve(D->getMultiVersionKind()));
  arboretum_create_edge(obj, context_.data_model_.method_isCPUDispatchMultiVersion_, context_.data_model_.arboretum_node_for(D->isCPUDispatchMultiVersion()));
  arboretum_create_edge(obj, context_.data_model_.method_isCPUSpecificMultiVersion_, context_.data_model_.arboretum_node_for(D->isCPUSpecificMultiVersion()));
  arboretum_create_edge(obj, context_.data_model_.method_isTargetMultiVersion_, context_.data_model_.arboretum_node_for(D->isTargetMultiVersion()));
  arboretum_create_edge(obj, context_.data_model_.method_isTargetClonesMultiVersion_, context_.data_model_.arboretum_node_for(D->isTargetClonesMultiVersion()));
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_11_, context_.resolve(D->getCanonicalDecl()));
  {
    std::vector<Id*> element_ids;
    auto range = D->parameters();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_parameters_2_, context_.data_model_.arboretum_node_for(context_.data_model_.class_ParmVarDecl_, element_ids));
  }
  arboretum_create_edge(obj, context_.data_model_.method_param_empty_1_, context_.data_model_.arboretum_node_for(D->param_empty()));
  arboretum_create_edge(obj, context_.data_model_.method_param_size_1_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->param_size())));
  arboretum_create_edge(obj, context_.data_model_.method_getNumParams_3_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumParams())));
  arboretum_create_edge(obj, context_.data_model_.method_getMinRequiredArguments_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getMinRequiredArguments())));
  arboretum_create_edge(obj, context_.data_model_.method_getMinRequiredExplicitArguments_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getMinRequiredExplicitArguments())));
  arboretum_create_edge(obj, context_.data_model_.method_hasCXXExplicitFunctionObjectParameter_, context_.data_model_.arboretum_node_for(D->hasCXXExplicitFunctionObjectParameter()));
  arboretum_create_edge(obj, context_.data_model_.method_getNumNonObjectParams_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumNonObjectParams())));
  arboretum_create_edge(obj, context_.data_model_.method_hasOneParamOrDefaultArgs_, context_.data_model_.arboretum_node_for(D->hasOneParamOrDefaultArgs()));
  // getFunctionTypeLoc ( class clang::FunctionTypeLoc )
  arboretum_create_edge(obj, context_.data_model_.method_getReturnType_1_, context_.resolve(D->getReturnType()));
  arboretum_create_edge(obj, context_.data_model_.method_getReturnTypeSourceRange_, context_.source_model_.resolve(D->getReturnTypeSourceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getParametersSourceRange_, context_.source_model_.resolve(D->getParametersSourceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getDeclaredReturnType_, context_.resolve(D->getDeclaredReturnType()));
  arboretum_create_edge(obj, context_.data_model_.method_getExceptionSpecType_1_, context_.data_model_.resolve(D->getExceptionSpecType()));
  arboretum_create_edge(obj, context_.data_model_.method_getExceptionSpecSourceRange_, context_.source_model_.resolve(D->getExceptionSpecSourceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getCallResultType_, context_.resolve(D->getCallResultType()));
  arboretum_create_edge(obj, context_.data_model_.method_getStorageClass_, context_.data_model_.resolve(D->getStorageClass()));
  arboretum_create_edge(obj, context_.data_model_.method_isInlineSpecified_, context_.data_model_.arboretum_node_for(D->isInlineSpecified()));
  arboretum_create_edge(obj, context_.data_model_.method_UsesFPIntrin_, context_.data_model_.arboretum_node_for(D->UsesFPIntrin()));
  arboretum_create_edge(obj, context_.data_model_.method_isInlined_, context_.data_model_.arboretum_node_for(D->isInlined()));
  arboretum_create_edge(obj, context_.data_model_.method_isInlineDefinitionExternallyVisible_, context_.data_model_.arboretum_node_for(D->isInlineDefinitionExternallyVisible()));
  arboretum_create_edge(obj, context_.data_model_.method_isMSExternInline_, context_.data_model_.arboretum_node_for(D->isMSExternInline()));
  arboretum_create_edge(obj, context_.data_model_.method_doesDeclarationForceExternallyVisibleDefinition_, context_.data_model_.arboretum_node_for(D->doesDeclarationForceExternallyVisibleDefinition()));
  arboretum_create_edge(obj, context_.data_model_.method_isStatic_1_, context_.data_model_.arboretum_node_for(D->isStatic()));
  arboretum_create_edge(obj, context_.data_model_.method_isOverloadedOperator_, context_.data_model_.arboretum_node_for(D->isOverloadedOperator()));
  arboretum_create_edge(obj, context_.data_model_.method_getOverloadedOperator_, context_.data_model_.resolve(D->getOverloadedOperator()));
  // getLiteralIdentifier ( const class clang::IdentifierInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getInstantiatedFromMemberFunction_, context_.resolve(D->getInstantiatedFromMemberFunction()));
  arboretum_create_edge(obj, context_.data_model_.method_getTemplatedKind_, context_.data_model_.resolve(D->getTemplatedKind()));
  // getMemberSpecializationInfo ( class clang::MemberSpecializationInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getInstantiatedFromDecl_, context_.resolve(D->getInstantiatedFromDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getDescribedFunctionTemplate_, context_.resolve(D->getDescribedFunctionTemplate()));
  arboretum_create_edge(obj, context_.data_model_.method_isFunctionTemplateSpecialization_, context_.data_model_.arboretum_node_for(D->isFunctionTemplateSpecialization()));
  // getTemplateSpecializationInfo ( class clang::FunctionTemplateSpecializationInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_isImplicitlyInstantiable_, context_.data_model_.arboretum_node_for(D->isImplicitlyInstantiable()));
  arboretum_create_edge(obj, context_.data_model_.method_isTemplateInstantiation_, context_.data_model_.arboretum_node_for(D->isTemplateInstantiation()));
  arboretum_create_edge(obj, context_.data_model_.method_getPrimaryTemplate_, context_.resolve(D->getPrimaryTemplate()));
  // getTemplateSpecializationArgs ( const class clang::TemplateArgumentList * )
  // getTemplateSpecializationArgsAsWritten ( const struct clang::ASTTemplateArgumentListInfo * )
  // getDependentSpecializationInfo ( class clang::DependentFunctionTemplateSpecializationInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getTemplateSpecializationKind_2_, context_.data_model_.resolve(D->getTemplateSpecializationKind()));
  arboretum_create_edge(obj, context_.data_model_.method_getTemplateSpecializationKindForInstantiation_, context_.data_model_.resolve(D->getTemplateSpecializationKindForInstantiation()));
  arboretum_create_edge(obj, context_.data_model_.method_getPointOfInstantiation_1_, context_.source_model_.resolve(D->getPointOfInstantiation()));
  arboretum_create_edge(obj, context_.data_model_.method_isOutOfLine_1_, context_.data_model_.arboretum_node_for(D->isOutOfLine()));
  arboretum_create_edge(obj, context_.data_model_.method_getMemoryFunctionKind_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getMemoryFunctionKind())));
  arboretum_create_edge(obj, context_.data_model_.method_getODRHash_1_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getODRHash())));
  return true;
}

bool ArboretumASTVisitor::VisitFunctionTemplateDecl(clang::FunctionTemplateDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getTemplatedDecl_1_, context_.resolve(D->getTemplatedDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_isThisDeclarationADefinition_2_, context_.data_model_.arboretum_node_for(D->isThisDeclarationADefinition()));
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_12_, context_.resolve(D->getCanonicalDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getPreviousDecl_4_, context_.resolve(D->getPreviousDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getMostRecentDecl_5_, context_.resolve(D->getMostRecentDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getInstantiatedFromMemberTemplate_2_, context_.resolve(D->getInstantiatedFromMemberTemplate()));
  // specializations ( class llvm::iterator_range<struct clang::RedeclarableTemplateDecl::SpecIterator<class clang::FunctionTemplateSpecializationInfo> > )
  arboretum_create_edge(obj, context_.data_model_.method_isAbbreviated_, context_.data_model_.arboretum_node_for(D->isAbbreviated()));
  return true;
}

bool ArboretumASTVisitor::VisitHLSLBufferDecl(clang::HLSLBufferDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_14_, context_.source_model_.resolve(D->getSourceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getLocStart_, context_.source_model_.resolve(D->getLocStart()));
  arboretum_create_edge(obj, context_.data_model_.method_getLBraceLoc_, context_.source_model_.resolve(D->getLBraceLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRBraceLoc_1_, context_.source_model_.resolve(D->getRBraceLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_isCBuffer_, context_.data_model_.arboretum_node_for(D->isCBuffer()));
  return true;
}

bool ArboretumASTVisitor::VisitImplicitConceptSpecializationDecl(clang::ImplicitConceptSpecializationDecl* D) {
  const Id* obj = context_.resolve(D);
  // getTemplateArguments ( class llvm::ArrayRef<class clang::TemplateArgument> )
  return true;
}

bool ArboretumASTVisitor::VisitImplicitParamDecl(clang::ImplicitParamDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getParameterKind_, context_.data_model_.resolve(D->getParameterKind()));
  return true;
}

bool ArboretumASTVisitor::VisitImportDecl(clang::ImportDecl* D) {
  const Id* obj = context_.resolve(D);
  // getImportedModule ( class clang::Module * )
  {
    std::vector<Id*> element_ids;
    auto range = D->getIdentifierLocs();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.source_model_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_getIdentifierLocs_, context_.data_model_.arboretum_node_for(context_.data_model_.source_location_class_, element_ids));
  }
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_15_, context_.source_model_.resolve(D->getSourceRange()));
  return true;
}

bool ArboretumASTVisitor::VisitIndirectFieldDecl(clang::IndirectFieldDecl* D) {
  const Id* obj = context_.resolve(D);
  {
    std::vector<Id*> element_ids;
    auto range = D->chain();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_chain_, context_.data_model_.arboretum_node_for(context_.data_model_.class_NamedDecl_, element_ids));
  }
  arboretum_create_edge(obj, context_.data_model_.method_getChainingSize_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getChainingSize())));
  arboretum_create_edge(obj, context_.data_model_.method_getAnonField_, context_.resolve(D->getAnonField()));
  arboretum_create_edge(obj, context_.data_model_.method_getVarDecl_, context_.resolve(D->getVarDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_13_, context_.resolve(D->getCanonicalDecl()));
  return true;
}

bool ArboretumASTVisitor::VisitLabelDecl(clang::LabelDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getStmt_, context_.resolve(D->getStmt()));
  arboretum_create_edge(obj, context_.data_model_.method_isGnuLocal_, context_.data_model_.arboretum_node_for(D->isGnuLocal()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_16_, context_.source_model_.resolve(D->getSourceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_isMSAsmLabel_, context_.data_model_.arboretum_node_for(D->isMSAsmLabel()));
  arboretum_create_edge(obj, context_.data_model_.method_isResolvedMSAsmLabel_, context_.data_model_.arboretum_node_for(D->isResolvedMSAsmLabel()));
  arboretum_create_edge(obj, context_.data_model_.method_getMSAsmLabel_, context_.data_model_.arboretum_node_for(D->getMSAsmLabel().str()));
  return true;
}

bool ArboretumASTVisitor::VisitLifetimeExtendedTemporaryDecl(clang::LifetimeExtendedTemporaryDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getExtendingDecl_, context_.resolve(D->getExtendingDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getStorageDuration_, context_.data_model_.resolve(D->getStorageDuration()));
  arboretum_create_edge(obj, context_.data_model_.method_getTemporaryExpr_, context_.resolve(D->getTemporaryExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getManglingNumber_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getManglingNumber())));
  // getValue ( class clang::APValue * )
  // childrenExpr ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitLinkageSpecDecl(clang::LinkageSpecDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getLanguage_, context_.data_model_.resolve(D->getLanguage()));
  arboretum_create_edge(obj, context_.data_model_.method_hasBraces_1_, context_.data_model_.arboretum_node_for(D->hasBraces()));
  arboretum_create_edge(obj, context_.data_model_.method_getExternLoc_1_, context_.source_model_.resolve(D->getExternLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRBraceLoc_2_, context_.source_model_.resolve(D->getRBraceLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_2_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_17_, context_.source_model_.resolve(D->getSourceRange()));
  return true;
}

bool ArboretumASTVisitor::VisitMSGuidDecl(clang::MSGuidDecl* D) {
  const Id* obj = context_.resolve(D);
  // getParts ( struct clang::MSGuidDeclParts )
  // getAsAPValue ( class clang::APValue & )
  return true;
}

bool ArboretumASTVisitor::VisitMSPropertyDecl(clang::MSPropertyDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_hasGetter_, context_.data_model_.arboretum_node_for(D->hasGetter()));
  // getGetterId ( class clang::IdentifierInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_hasSetter_, context_.data_model_.arboretum_node_for(D->hasSetter()));
  // getSetterId ( class clang::IdentifierInfo * )
  return true;
}

bool ArboretumASTVisitor::VisitNamedDecl(clang::NamedDecl* D) {
  const Id* obj = context_.resolve(D);
  // getIdentifier ( class clang::IdentifierInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getNameAsString_, context_.data_model_.arboretum_node_for(D->getNameAsString()));
  // getDeclName ( class clang::DeclarationName )
  arboretum_create_edge(obj, context_.data_model_.method_getQualifiedNameAsString_, context_.data_model_.arboretum_node_for(D->getQualifiedNameAsString()));
  arboretum_create_edge(obj, context_.data_model_.method_hasLinkage_, context_.data_model_.arboretum_node_for(D->hasLinkage()));
  arboretum_create_edge(obj, context_.data_model_.method_isCXXClassMember_, context_.data_model_.arboretum_node_for(D->isCXXClassMember()));
  arboretum_create_edge(obj, context_.data_model_.method_isCXXInstanceMember_, context_.data_model_.arboretum_node_for(D->isCXXInstanceMember()));
  arboretum_create_edge(obj, context_.data_model_.method_getLinkageInternal_, context_.data_model_.resolve(D->getLinkageInternal()));
  arboretum_create_edge(obj, context_.data_model_.method_getFormalLinkage_, context_.data_model_.resolve(D->getFormalLinkage()));
  arboretum_create_edge(obj, context_.data_model_.method_hasExternalFormalLinkage_, context_.data_model_.arboretum_node_for(D->hasExternalFormalLinkage()));
  arboretum_create_edge(obj, context_.data_model_.method_isExternallyVisible_, context_.data_model_.arboretum_node_for(D->isExternallyVisible()));
  arboretum_create_edge(obj, context_.data_model_.method_isExternallyDeclarable_, context_.data_model_.arboretum_node_for(D->isExternallyDeclarable()));
  arboretum_create_edge(obj, context_.data_model_.method_isLinkageValid_, context_.data_model_.arboretum_node_for(D->isLinkageValid()));
  arboretum_create_edge(obj, context_.data_model_.method_hasLinkageBeenComputed_, context_.data_model_.arboretum_node_for(D->hasLinkageBeenComputed()));
  arboretum_create_edge(obj, context_.data_model_.method_getUnderlyingDecl_, context_.resolve(D->getUnderlyingDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getMostRecentDecl_6_, context_.resolve(D->getMostRecentDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getObjCFStringFormattingFamily_, context_.data_model_.resolve(D->getObjCFStringFormattingFamily()));
  return true;
}

bool ArboretumASTVisitor::VisitNamespaceAliasDecl(clang::NamespaceAliasDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_14_, context_.resolve(D->getCanonicalDecl()));
  // getQualifierLoc ( class clang::NestedNameSpecifierLoc )
  // getQualifier ( class clang::NestedNameSpecifier * )
  arboretum_create_edge(obj, context_.data_model_.method_getNamespace_, context_.resolve(D->getNamespace()));
  arboretum_create_edge(obj, context_.data_model_.method_getAliasLoc_, context_.source_model_.resolve(D->getAliasLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getNamespaceLoc_, context_.source_model_.resolve(D->getNamespaceLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getTargetNameLoc_, context_.source_model_.resolve(D->getTargetNameLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getAliasedNamespace_, context_.resolve(D->getAliasedNamespace()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_18_, context_.source_model_.resolve(D->getSourceRange()));
  return true;
}

bool ArboretumASTVisitor::VisitNamespaceDecl(clang::NamespaceDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_isAnonymousNamespace_, context_.data_model_.arboretum_node_for(D->isAnonymousNamespace()));
  arboretum_create_edge(obj, context_.data_model_.method_isInline_, context_.data_model_.arboretum_node_for(D->isInline()));
  arboretum_create_edge(obj, context_.data_model_.method_isNested_, context_.data_model_.arboretum_node_for(D->isNested()));
  arboretum_create_edge(obj, context_.data_model_.method_getOriginalNamespace_, context_.resolve(D->getOriginalNamespace()));
  arboretum_create_edge(obj, context_.data_model_.method_isOriginalNamespace_, context_.data_model_.arboretum_node_for(D->isOriginalNamespace()));
  arboretum_create_edge(obj, context_.data_model_.method_getAnonymousNamespace_, context_.resolve(D->getAnonymousNamespace()));
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_15_, context_.resolve(D->getCanonicalDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_19_, context_.source_model_.resolve(D->getSourceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_2_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRBraceLoc_3_, context_.source_model_.resolve(D->getRBraceLoc()));
  return true;
}

bool ArboretumASTVisitor::VisitNonTypeTemplateParmDecl(clang::NonTypeTemplateParmDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_20_, context_.source_model_.resolve(D->getSourceRange()));
  // getDefaultArgStorage ( const class clang::DefaultArgStorage<class clang::NonTypeTemplateParmDecl, class clang::Expr *> & )
  arboretum_create_edge(obj, context_.data_model_.method_hasDefaultArgument_, context_.data_model_.arboretum_node_for(D->hasDefaultArgument()));
  arboretum_create_edge(obj, context_.data_model_.method_getDefaultArgument_, context_.resolve(D->getDefaultArgument()));
  arboretum_create_edge(obj, context_.data_model_.method_getDefaultArgumentLoc_, context_.source_model_.resolve(D->getDefaultArgumentLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_defaultArgumentWasInherited_, context_.data_model_.arboretum_node_for(D->defaultArgumentWasInherited()));
  arboretum_create_edge(obj, context_.data_model_.method_isParameterPack_2_, context_.data_model_.arboretum_node_for(D->isParameterPack()));
  arboretum_create_edge(obj, context_.data_model_.method_isPackExpansion_, context_.data_model_.arboretum_node_for(D->isPackExpansion()));
  arboretum_create_edge(obj, context_.data_model_.method_isExpandedParameterPack_, context_.data_model_.arboretum_node_for(D->isExpandedParameterPack()));
  arboretum_create_edge(obj, context_.data_model_.method_getPlaceholderTypeConstraint_, context_.resolve(D->getPlaceholderTypeConstraint()));
  arboretum_create_edge(obj, context_.data_model_.method_hasPlaceholderTypeConstraint_, context_.data_model_.arboretum_node_for(D->hasPlaceholderTypeConstraint()));
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
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_21_, context_.source_model_.resolve(D->getSourceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_isObjCMethodParameter_, context_.data_model_.arboretum_node_for(D->isObjCMethodParameter()));
  arboretum_create_edge(obj, context_.data_model_.method_isDestroyedInCallee_, context_.data_model_.arboretum_node_for(D->isDestroyedInCallee()));
  arboretum_create_edge(obj, context_.data_model_.method_getFunctionScopeDepth_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getFunctionScopeDepth())));
  arboretum_create_edge(obj, context_.data_model_.method_getFunctionScopeIndex_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getFunctionScopeIndex())));
  arboretum_create_edge(obj, context_.data_model_.method_getObjCDeclQualifier_, context_.data_model_.resolve(D->getObjCDeclQualifier()));
  arboretum_create_edge(obj, context_.data_model_.method_isKNRPromoted_, context_.data_model_.arboretum_node_for(D->isKNRPromoted()));
  arboretum_create_edge(obj, context_.data_model_.method_isExplicitObjectParameter_, context_.data_model_.arboretum_node_for(D->isExplicitObjectParameter()));
  arboretum_create_edge(obj, context_.data_model_.method_getExplicitObjectParamThisLoc_, context_.source_model_.resolve(D->getExplicitObjectParamThisLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getDefaultArg_, context_.resolve(D->getDefaultArg()));
  arboretum_create_edge(obj, context_.data_model_.method_getDefaultArgRange_, context_.source_model_.resolve(D->getDefaultArgRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getUninstantiatedDefaultArg_, context_.resolve(D->getUninstantiatedDefaultArg()));
  arboretum_create_edge(obj, context_.data_model_.method_hasDefaultArg_, context_.data_model_.arboretum_node_for(D->hasDefaultArg()));
  arboretum_create_edge(obj, context_.data_model_.method_hasUnparsedDefaultArg_, context_.data_model_.arboretum_node_for(D->hasUnparsedDefaultArg()));
  arboretum_create_edge(obj, context_.data_model_.method_hasUninstantiatedDefaultArg_, context_.data_model_.arboretum_node_for(D->hasUninstantiatedDefaultArg()));
  arboretum_create_edge(obj, context_.data_model_.method_hasInheritedDefaultArg_, context_.data_model_.arboretum_node_for(D->hasInheritedDefaultArg()));
  arboretum_create_edge(obj, context_.data_model_.method_getOriginalType_1_, context_.resolve(D->getOriginalType()));
  return true;
}

bool ArboretumASTVisitor::VisitPragmaCommentDecl(clang::PragmaCommentDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getCommentKind_, context_.data_model_.resolve(D->getCommentKind()));
  arboretum_create_edge(obj, context_.data_model_.method_getArg_, context_.data_model_.arboretum_node_for(D->getArg().str()));
  return true;
}

bool ArboretumASTVisitor::VisitPragmaDetectMismatchDecl(clang::PragmaDetectMismatchDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getName_, context_.data_model_.arboretum_node_for(D->getName().str()));
  arboretum_create_edge(obj, context_.data_model_.method_getValue_1_, context_.data_model_.arboretum_node_for(D->getValue().str()));
  return true;
}

bool ArboretumASTVisitor::VisitRecordDecl(clang::RecordDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getPreviousDecl_5_, context_.resolve(D->getPreviousDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getMostRecentDecl_7_, context_.resolve(D->getMostRecentDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_hasFlexibleArrayMember_, context_.data_model_.arboretum_node_for(D->hasFlexibleArrayMember()));
  arboretum_create_edge(obj, context_.data_model_.method_isAnonymousStructOrUnion_1_, context_.data_model_.arboretum_node_for(D->isAnonymousStructOrUnion()));
  arboretum_create_edge(obj, context_.data_model_.method_hasObjectMember_, context_.data_model_.arboretum_node_for(D->hasObjectMember()));
  arboretum_create_edge(obj, context_.data_model_.method_hasVolatileMember_, context_.data_model_.arboretum_node_for(D->hasVolatileMember()));
  arboretum_create_edge(obj, context_.data_model_.method_hasLoadedFieldsFromExternalStorage_, context_.data_model_.arboretum_node_for(D->hasLoadedFieldsFromExternalStorage()));
  arboretum_create_edge(obj, context_.data_model_.method_isNonTrivialToPrimitiveDefaultInitialize_, context_.data_model_.arboretum_node_for(D->isNonTrivialToPrimitiveDefaultInitialize()));
  arboretum_create_edge(obj, context_.data_model_.method_isNonTrivialToPrimitiveCopy_, context_.data_model_.arboretum_node_for(D->isNonTrivialToPrimitiveCopy()));
  arboretum_create_edge(obj, context_.data_model_.method_isNonTrivialToPrimitiveDestroy_, context_.data_model_.arboretum_node_for(D->isNonTrivialToPrimitiveDestroy()));
  arboretum_create_edge(obj, context_.data_model_.method_hasNonTrivialToPrimitiveDefaultInitializeCUnion_, context_.data_model_.arboretum_node_for(D->hasNonTrivialToPrimitiveDefaultInitializeCUnion()));
  arboretum_create_edge(obj, context_.data_model_.method_hasNonTrivialToPrimitiveDestructCUnion_, context_.data_model_.arboretum_node_for(D->hasNonTrivialToPrimitiveDestructCUnion()));
  arboretum_create_edge(obj, context_.data_model_.method_hasNonTrivialToPrimitiveCopyCUnion_, context_.data_model_.arboretum_node_for(D->hasNonTrivialToPrimitiveCopyCUnion()));
  arboretum_create_edge(obj, context_.data_model_.method_canPassInRegisters_, context_.data_model_.arboretum_node_for(D->canPassInRegisters()));
  arboretum_create_edge(obj, context_.data_model_.method_getArgPassingRestrictions_, context_.data_model_.resolve(D->getArgPassingRestrictions()));
  arboretum_create_edge(obj, context_.data_model_.method_isParamDestroyedInCallee_, context_.data_model_.arboretum_node_for(D->isParamDestroyedInCallee()));
  arboretum_create_edge(obj, context_.data_model_.method_isRandomized_, context_.data_model_.arboretum_node_for(D->isRandomized()));
  arboretum_create_edge(obj, context_.data_model_.method_isInjectedClassName_, context_.data_model_.arboretum_node_for(D->isInjectedClassName()));
  arboretum_create_edge(obj, context_.data_model_.method_isLambda_1_, context_.data_model_.arboretum_node_for(D->isLambda()));
  arboretum_create_edge(obj, context_.data_model_.method_isCapturedRecord_, context_.data_model_.arboretum_node_for(D->isCapturedRecord()));
  arboretum_create_edge(obj, context_.data_model_.method_getDefinition_3_, context_.resolve(D->getDefinition()));
  arboretum_create_edge(obj, context_.data_model_.method_isOrContainsUnion_, context_.data_model_.arboretum_node_for(D->isOrContainsUnion()));
  {
    std::vector<Id*> element_ids;
    auto range = D->fields();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_fields_, context_.data_model_.arboretum_node_for(context_.data_model_.class_FieldDecl_, element_ids));
  }
  arboretum_create_edge(obj, context_.data_model_.method_field_empty_, context_.data_model_.arboretum_node_for(D->field_empty()));
  arboretum_create_edge(obj, context_.data_model_.method_findFirstNamedDataMember_, context_.resolve(D->findFirstNamedDataMember()));
  return true;
}

bool ArboretumASTVisitor::VisitRedeclarableTemplateDecl(clang::RedeclarableTemplateDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_16_, context_.resolve(D->getCanonicalDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_isMemberSpecialization_, context_.data_model_.arboretum_node_for(D->isMemberSpecialization()));
  arboretum_create_edge(obj, context_.data_model_.method_getInstantiatedFromMemberTemplate_3_, context_.resolve(D->getInstantiatedFromMemberTemplate()));
  return true;
}

bool ArboretumASTVisitor::VisitRequiresExprBodyDecl(clang::RequiresExprBodyDecl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitStaticAssertDecl(clang::StaticAssertDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getAssertExpr_, context_.resolve(D->getAssertExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getMessage_, context_.resolve(D->getMessage()));
  arboretum_create_edge(obj, context_.data_model_.method_isFailed_, context_.data_model_.arboretum_node_for(D->isFailed()));
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_1_, context_.source_model_.resolve(D->getRParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_22_, context_.source_model_.resolve(D->getSourceRange()));
  return true;
}

bool ArboretumASTVisitor::VisitTagDecl(clang::TagDecl* D) {
  if (!D->isThisDeclarationADefinition()) return true;

  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getBraceRange_, context_.source_model_.resolve(D->getBraceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getInnerLocStart_1_, context_.source_model_.resolve(D->getInnerLocStart()));
  arboretum_create_edge(obj, context_.data_model_.method_getOuterLocStart_1_, context_.source_model_.resolve(D->getOuterLocStart()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_23_, context_.source_model_.resolve(D->getSourceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_17_, context_.resolve(D->getCanonicalDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_isThisDeclarationADefinition_3_, context_.data_model_.arboretum_node_for(D->isThisDeclarationADefinition()));
  arboretum_create_edge(obj, context_.data_model_.method_isCompleteDefinition_, context_.data_model_.arboretum_node_for(D->isCompleteDefinition()));
  arboretum_create_edge(obj, context_.data_model_.method_isCompleteDefinitionRequired_, context_.data_model_.arboretum_node_for(D->isCompleteDefinitionRequired()));
  arboretum_create_edge(obj, context_.data_model_.method_isBeingDefined_1_, context_.data_model_.arboretum_node_for(D->isBeingDefined()));
  arboretum_create_edge(obj, context_.data_model_.method_isEmbeddedInDeclarator_, context_.data_model_.arboretum_node_for(D->isEmbeddedInDeclarator()));
  arboretum_create_edge(obj, context_.data_model_.method_isFreeStanding_, context_.data_model_.arboretum_node_for(D->isFreeStanding()));
  arboretum_create_edge(obj, context_.data_model_.method_mayHaveOutOfDateDef_, context_.data_model_.arboretum_node_for(D->mayHaveOutOfDateDef()));
  arboretum_create_edge(obj, context_.data_model_.method_isDependentType_, context_.data_model_.arboretum_node_for(D->isDependentType()));
  arboretum_create_edge(obj, context_.data_model_.method_isThisDeclarationADemotedDefinition_, context_.data_model_.arboretum_node_for(D->isThisDeclarationADemotedDefinition()));
  arboretum_create_edge(obj, context_.data_model_.method_getDefinition_4_, context_.resolve(D->getDefinition()));
  arboretum_create_edge(obj, context_.data_model_.method_getKindName_, context_.data_model_.arboretum_node_for(D->getKindName().str()));
  arboretum_create_edge(obj, context_.data_model_.method_getTagKind_, context_.data_model_.resolve(D->getTagKind()));
  arboretum_create_edge(obj, context_.data_model_.method_isStruct_, context_.data_model_.arboretum_node_for(D->isStruct()));
  arboretum_create_edge(obj, context_.data_model_.method_isInterface_, context_.data_model_.arboretum_node_for(D->isInterface()));
  arboretum_create_edge(obj, context_.data_model_.method_isClass_, context_.data_model_.arboretum_node_for(D->isClass()));
  arboretum_create_edge(obj, context_.data_model_.method_isUnion_, context_.data_model_.arboretum_node_for(D->isUnion()));
  arboretum_create_edge(obj, context_.data_model_.method_isEnum_, context_.data_model_.arboretum_node_for(D->isEnum()));
  arboretum_create_edge(obj, context_.data_model_.method_hasNameForLinkage_, context_.data_model_.arboretum_node_for(D->hasNameForLinkage()));
  arboretum_create_edge(obj, context_.data_model_.method_getTypedefNameForAnonDecl_, context_.resolve(D->getTypedefNameForAnonDecl()));
  // getQualifier ( class clang::NestedNameSpecifier * )
  // getQualifierLoc ( class clang::NestedNameSpecifierLoc )
  arboretum_create_edge(obj, context_.data_model_.method_getNumTemplateParameterLists_1_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumTemplateParameterLists())));
  return true;
}

bool ArboretumASTVisitor::VisitTemplateDecl(clang::TemplateDecl* D) {
  const Id* obj = context_.resolve(D);
  // getTemplateParameters ( class clang::TemplateParameterList * )
  arboretum_create_edge(obj, context_.data_model_.method_hasAssociatedConstraints_1_, context_.data_model_.arboretum_node_for(D->hasAssociatedConstraints()));
  arboretum_create_edge(obj, context_.data_model_.method_getTemplatedDecl_2_, context_.resolve(D->getTemplatedDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_isTypeAlias_1_, context_.data_model_.arboretum_node_for(D->isTypeAlias()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_24_, context_.source_model_.resolve(D->getSourceRange()));
  return true;
}

bool ArboretumASTVisitor::VisitTemplateParamObjectDecl(clang::TemplateParamObjectDecl* D) {
  const Id* obj = context_.resolve(D);
  // getValue ( const class clang::APValue & )
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_18_, context_.resolve(D->getCanonicalDecl()));
  return true;
}

bool ArboretumASTVisitor::VisitTemplateTemplateParmDecl(clang::TemplateTemplateParmDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_isParameterPack_3_, context_.data_model_.arboretum_node_for(D->isParameterPack()));
  arboretum_create_edge(obj, context_.data_model_.method_isPackExpansion_1_, context_.data_model_.arboretum_node_for(D->isPackExpansion()));
  arboretum_create_edge(obj, context_.data_model_.method_isExpandedParameterPack_1_, context_.data_model_.arboretum_node_for(D->isExpandedParameterPack()));
  // getDefaultArgStorage ( const class clang::DefaultArgStorage<class clang::TemplateTemplateParmDecl, class clang::TemplateArgumentLoc *> & )
  arboretum_create_edge(obj, context_.data_model_.method_hasDefaultArgument_1_, context_.data_model_.arboretum_node_for(D->hasDefaultArgument()));
  // getDefaultArgument ( const class clang::TemplateArgumentLoc & )
  arboretum_create_edge(obj, context_.data_model_.method_getDefaultArgumentLoc_1_, context_.source_model_.resolve(D->getDefaultArgumentLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_defaultArgumentWasInherited_1_, context_.data_model_.arboretum_node_for(D->defaultArgumentWasInherited()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_25_, context_.source_model_.resolve(D->getSourceRange()));
  return true;
}

bool ArboretumASTVisitor::VisitTemplateTypeParmDecl(clang::TemplateTypeParmDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_wasDeclaredWithTypename_, context_.data_model_.arboretum_node_for(D->wasDeclaredWithTypename()));
  // getDefaultArgStorage ( const class clang::DefaultArgStorage<class clang::TemplateTypeParmDecl, class clang::TypeSourceInfo *> & )
  arboretum_create_edge(obj, context_.data_model_.method_hasDefaultArgument_2_, context_.data_model_.arboretum_node_for(D->hasDefaultArgument()));
  // getDefaultArgumentInfo ( class clang::TypeSourceInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getDefaultArgumentLoc_2_, context_.source_model_.resolve(D->getDefaultArgumentLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_defaultArgumentWasInherited_2_, context_.data_model_.arboretum_node_for(D->defaultArgumentWasInherited()));
  arboretum_create_edge(obj, context_.data_model_.method_getDepth_1_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getDepth())));
  arboretum_create_edge(obj, context_.data_model_.method_getIndex_3_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getIndex())));
  arboretum_create_edge(obj, context_.data_model_.method_isParameterPack_4_, context_.data_model_.arboretum_node_for(D->isParameterPack()));
  arboretum_create_edge(obj, context_.data_model_.method_isPackExpansion_2_, context_.data_model_.arboretum_node_for(D->isPackExpansion()));
  arboretum_create_edge(obj, context_.data_model_.method_isExpandedParameterPack_2_, context_.data_model_.arboretum_node_for(D->isExpandedParameterPack()));
  // getTypeConstraint ( const class clang::TypeConstraint * )
  arboretum_create_edge(obj, context_.data_model_.method_hasTypeConstraint_, context_.data_model_.arboretum_node_for(D->hasTypeConstraint()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_26_, context_.source_model_.resolve(D->getSourceRange()));
  return true;
}

bool ArboretumASTVisitor::VisitTopLevelStmtDecl(clang::TopLevelStmtDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_27_, context_.source_model_.resolve(D->getSourceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getStmt_1_, context_.resolve(D->getStmt()));
  arboretum_create_edge(obj, context_.data_model_.method_isSemiMissing_, context_.data_model_.arboretum_node_for(D->isSemiMissing()));
  return true;
}

bool ArboretumASTVisitor::VisitTranslationUnitDecl(clang::TranslationUnitDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getAnonymousNamespace_1_, context_.resolve(D->getAnonymousNamespace()));
  return true;
}

bool ArboretumASTVisitor::VisitTypeAliasDecl(clang::TypeAliasDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_28_, context_.source_model_.resolve(D->getSourceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getDescribedAliasTemplate_, context_.resolve(D->getDescribedAliasTemplate()));
  return true;
}

bool ArboretumASTVisitor::VisitTypeAliasTemplateDecl(clang::TypeAliasTemplateDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getTemplatedDecl_3_, context_.resolve(D->getTemplatedDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_19_, context_.resolve(D->getCanonicalDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getPreviousDecl_6_, context_.resolve(D->getPreviousDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getInstantiatedFromMemberTemplate_4_, context_.resolve(D->getInstantiatedFromMemberTemplate()));
  return true;
}

bool ArboretumASTVisitor::VisitTypeDecl(clang::TypeDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getTypeForDecl_, context_.resolve(D->getTypeForDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_3_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_29_, context_.source_model_.resolve(D->getSourceRange()));
  return true;
}

bool ArboretumASTVisitor::VisitTypedefDecl(clang::TypedefDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_30_, context_.source_model_.resolve(D->getSourceRange()));
  return true;
}

bool ArboretumASTVisitor::VisitTypedefNameDecl(clang::TypedefNameDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_isModed_, context_.data_model_.arboretum_node_for(D->isModed()));
  // getTypeSourceInfo ( class clang::TypeSourceInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getUnderlyingType_4_, context_.resolve(D->getUnderlyingType()));
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_20_, context_.resolve(D->getCanonicalDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_isTransparentTag_, context_.data_model_.arboretum_node_for(D->isTransparentTag()));
  return true;
}

bool ArboretumASTVisitor::VisitUnnamedGlobalConstantDecl(clang::UnnamedGlobalConstantDecl* D) {
  const Id* obj = context_.resolve(D);
  // getValue ( const class clang::APValue & )
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedUsingIfExistsDecl(clang::UnresolvedUsingIfExistsDecl* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedUsingTypenameDecl(clang::UnresolvedUsingTypenameDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getUsingLoc_, context_.source_model_.resolve(D->getUsingLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getTypenameLoc_, context_.source_model_.resolve(D->getTypenameLoc()));
  // getQualifierLoc ( class clang::NestedNameSpecifierLoc )
  // getQualifier ( class clang::NestedNameSpecifier * )
  // getNameInfo ( struct clang::DeclarationNameInfo )
  arboretum_create_edge(obj, context_.data_model_.method_isPackExpansion_3_, context_.data_model_.arboretum_node_for(D->isPackExpansion()));
  arboretum_create_edge(obj, context_.data_model_.method_getEllipsisLoc_2_, context_.source_model_.resolve(D->getEllipsisLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_21_, context_.resolve(D->getCanonicalDecl()));
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedUsingValueDecl(clang::UnresolvedUsingValueDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getUsingLoc_1_, context_.source_model_.resolve(D->getUsingLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_isAccessDeclaration_, context_.data_model_.arboretum_node_for(D->isAccessDeclaration()));
  // getQualifierLoc ( class clang::NestedNameSpecifierLoc )
  // getQualifier ( class clang::NestedNameSpecifier * )
  // getNameInfo ( struct clang::DeclarationNameInfo )
  arboretum_create_edge(obj, context_.data_model_.method_isPackExpansion_4_, context_.data_model_.arboretum_node_for(D->isPackExpansion()));
  arboretum_create_edge(obj, context_.data_model_.method_getEllipsisLoc_3_, context_.source_model_.resolve(D->getEllipsisLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_31_, context_.source_model_.resolve(D->getSourceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_22_, context_.resolve(D->getCanonicalDecl()));
  return true;
}

bool ArboretumASTVisitor::VisitUsingDecl(clang::UsingDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getUsingLoc_2_, context_.source_model_.resolve(D->getUsingLoc()));
  // getQualifierLoc ( class clang::NestedNameSpecifierLoc )
  // getQualifier ( class clang::NestedNameSpecifier * )
  // getNameInfo ( struct clang::DeclarationNameInfo )
  arboretum_create_edge(obj, context_.data_model_.method_isAccessDeclaration_1_, context_.data_model_.arboretum_node_for(D->isAccessDeclaration()));
  arboretum_create_edge(obj, context_.data_model_.method_hasTypename_, context_.data_model_.arboretum_node_for(D->hasTypename()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_32_, context_.source_model_.resolve(D->getSourceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_23_, context_.resolve(D->getCanonicalDecl()));
  return true;
}

bool ArboretumASTVisitor::VisitUsingDirectiveDecl(clang::UsingDirectiveDecl* D) {
  const Id* obj = context_.resolve(D);
  // getQualifierLoc ( class clang::NestedNameSpecifierLoc )
  // getQualifier ( class clang::NestedNameSpecifier * )
  arboretum_create_edge(obj, context_.data_model_.method_getNominatedNamespaceAsWritten_, context_.resolve(D->getNominatedNamespaceAsWritten()));
  arboretum_create_edge(obj, context_.data_model_.method_getNominatedNamespace_, context_.resolve(D->getNominatedNamespace()));
  // getCommonAncestor ( const class clang::DeclContext * )
  arboretum_create_edge(obj, context_.data_model_.method_getUsingLoc_3_, context_.source_model_.resolve(D->getUsingLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getNamespaceKeyLocation_, context_.source_model_.resolve(D->getNamespaceKeyLocation()));
  arboretum_create_edge(obj, context_.data_model_.method_getIdentLocation_, context_.source_model_.resolve(D->getIdentLocation()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_33_, context_.source_model_.resolve(D->getSourceRange()));
  return true;
}

bool ArboretumASTVisitor::VisitUsingEnumDecl(clang::UsingEnumDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getUsingLoc_4_, context_.source_model_.resolve(D->getUsingLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEnumLoc_, context_.source_model_.resolve(D->getEnumLoc()));
  // getQualifier ( class clang::NestedNameSpecifier * )
  // getQualifierLoc ( class clang::NestedNameSpecifierLoc )
  // getEnumTypeLoc ( class clang::TypeLoc )
  // getEnumType ( class clang::TypeSourceInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getEnumDecl_, context_.resolve(D->getEnumDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_34_, context_.source_model_.resolve(D->getSourceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_24_, context_.resolve(D->getCanonicalDecl()));
  return true;
}

bool ArboretumASTVisitor::VisitUsingPackDecl(clang::UsingPackDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getInstantiatedFromUsingDecl_, context_.resolve(D->getInstantiatedFromUsingDecl()));
  {
    std::vector<Id*> element_ids;
    auto range = D->expansions();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_expansions_, context_.data_model_.arboretum_node_for(context_.data_model_.class_NamedDecl_, element_ids));
  }
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_35_, context_.source_model_.resolve(D->getSourceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_25_, context_.resolve(D->getCanonicalDecl()));
  return true;
}

bool ArboretumASTVisitor::VisitUsingShadowDecl(clang::UsingShadowDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_26_, context_.resolve(D->getCanonicalDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getTargetDecl_, context_.resolve(D->getTargetDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getIntroducer_1_, context_.resolve(D->getIntroducer()));
  arboretum_create_edge(obj, context_.data_model_.method_getNextUsingShadowDecl_, context_.resolve(D->getNextUsingShadowDecl()));
  return true;
}

bool ArboretumASTVisitor::VisitValueDecl(clang::ValueDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getType_, context_.resolve(D->getType()));
  arboretum_create_edge(obj, context_.data_model_.method_isWeak_, context_.data_model_.arboretum_node_for(D->isWeak()));
  arboretum_create_edge(obj, context_.data_model_.method_isInitCapture_, context_.data_model_.arboretum_node_for(D->isInitCapture()));
  arboretum_create_edge(obj, context_.data_model_.method_getPotentiallyDecomposedVarDecl_, context_.resolve(D->getPotentiallyDecomposedVarDecl()));
  return true;
}

bool ArboretumASTVisitor::VisitVarDecl(clang::VarDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_36_, context_.source_model_.resolve(D->getSourceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getStorageClass_1_, context_.data_model_.resolve(D->getStorageClass()));
  arboretum_create_edge(obj, context_.data_model_.method_getTSCSpec_, context_.data_model_.resolve(D->getTSCSpec()));
  arboretum_create_edge(obj, context_.data_model_.method_getTLSKind_, context_.data_model_.resolve(D->getTLSKind()));
  arboretum_create_edge(obj, context_.data_model_.method_hasLocalStorage_, context_.data_model_.arboretum_node_for(D->hasLocalStorage()));
  arboretum_create_edge(obj, context_.data_model_.method_isStaticLocal_, context_.data_model_.arboretum_node_for(D->isStaticLocal()));
  arboretum_create_edge(obj, context_.data_model_.method_hasExternalStorage_, context_.data_model_.arboretum_node_for(D->hasExternalStorage()));
  arboretum_create_edge(obj, context_.data_model_.method_hasGlobalStorage_, context_.data_model_.arboretum_node_for(D->hasGlobalStorage()));
  arboretum_create_edge(obj, context_.data_model_.method_getStorageDuration_1_, context_.data_model_.resolve(D->getStorageDuration()));
  arboretum_create_edge(obj, context_.data_model_.method_getLanguageLinkage_1_, context_.data_model_.resolve(D->getLanguageLinkage()));
  arboretum_create_edge(obj, context_.data_model_.method_isExternC_1_, context_.data_model_.arboretum_node_for(D->isExternC()));
  arboretum_create_edge(obj, context_.data_model_.method_isInExternCContext_1_, context_.data_model_.arboretum_node_for(D->isInExternCContext()));
  arboretum_create_edge(obj, context_.data_model_.method_isInExternCXXContext_1_, context_.data_model_.arboretum_node_for(D->isInExternCXXContext()));
  arboretum_create_edge(obj, context_.data_model_.method_isLocalVarDecl_, context_.data_model_.arboretum_node_for(D->isLocalVarDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_isLocalVarDeclOrParm_, context_.data_model_.arboretum_node_for(D->isLocalVarDeclOrParm()));
  arboretum_create_edge(obj, context_.data_model_.method_isFunctionOrMethodVarDecl_, context_.data_model_.arboretum_node_for(D->isFunctionOrMethodVarDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_isStaticDataMember_, context_.data_model_.arboretum_node_for(D->isStaticDataMember()));
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_27_, context_.resolve(D->getCanonicalDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_isThisDeclarationADefinition_4_, context_.data_model_.resolve(D->isThisDeclarationADefinition()));
  arboretum_create_edge(obj, context_.data_model_.method_hasDefinition_1_, context_.data_model_.resolve(D->hasDefinition()));
  arboretum_create_edge(obj, context_.data_model_.method_getActingDefinition_, context_.resolve(D->getActingDefinition()));
  arboretum_create_edge(obj, context_.data_model_.method_getDefinition_5_, context_.resolve(D->getDefinition()));
  arboretum_create_edge(obj, context_.data_model_.method_isOutOfLine_2_, context_.data_model_.arboretum_node_for(D->isOutOfLine()));
  arboretum_create_edge(obj, context_.data_model_.method_isFileVarDecl_, context_.data_model_.arboretum_node_for(D->isFileVarDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getAnyInitializer_, context_.resolve(D->getAnyInitializer()));
  arboretum_create_edge(obj, context_.data_model_.method_hasInit_, context_.data_model_.arboretum_node_for(D->hasInit()));
  arboretum_create_edge(obj, context_.data_model_.method_getInit_, context_.resolve(D->getInit()));
  arboretum_create_edge(obj, context_.data_model_.method_getInitializingDeclaration_, context_.resolve(D->getInitializingDeclaration()));
  // ensureEvaluatedStmt ( struct clang::EvaluatedStmt * )
  // getEvaluatedStmt ( struct clang::EvaluatedStmt * )
  // evaluateValue ( class clang::APValue * )
  // getEvaluatedValue ( class clang::APValue * )
  arboretum_create_edge(obj, context_.data_model_.method_hasConstantInitialization_, context_.data_model_.arboretum_node_for(D->hasConstantInitialization()));
  arboretum_create_edge(obj, context_.data_model_.method_getInitStyle_, context_.data_model_.resolve(D->getInitStyle()));
  arboretum_create_edge(obj, context_.data_model_.method_isDirectInit_, context_.data_model_.arboretum_node_for(D->isDirectInit()));
  arboretum_create_edge(obj, context_.data_model_.method_isThisDeclarationADemotedDefinition_1_, context_.data_model_.arboretum_node_for(D->isThisDeclarationADemotedDefinition()));
  arboretum_create_edge(obj, context_.data_model_.method_isExceptionVariable_, context_.data_model_.arboretum_node_for(D->isExceptionVariable()));
  arboretum_create_edge(obj, context_.data_model_.method_isNRVOVariable_, context_.data_model_.arboretum_node_for(D->isNRVOVariable()));
  arboretum_create_edge(obj, context_.data_model_.method_isCXXForRangeDecl_, context_.data_model_.arboretum_node_for(D->isCXXForRangeDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_isObjCForDecl_, context_.data_model_.arboretum_node_for(D->isObjCForDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_isARCPseudoStrong_, context_.data_model_.arboretum_node_for(D->isARCPseudoStrong()));
  arboretum_create_edge(obj, context_.data_model_.method_isInline_1_, context_.data_model_.arboretum_node_for(D->isInline()));
  arboretum_create_edge(obj, context_.data_model_.method_isInlineSpecified_1_, context_.data_model_.arboretum_node_for(D->isInlineSpecified()));
  arboretum_create_edge(obj, context_.data_model_.method_isConstexpr_1_, context_.data_model_.arboretum_node_for(D->isConstexpr()));
  arboretum_create_edge(obj, context_.data_model_.method_isInitCapture_1_, context_.data_model_.arboretum_node_for(D->isInitCapture()));
  arboretum_create_edge(obj, context_.data_model_.method_isParameterPack_5_, context_.data_model_.arboretum_node_for(D->isParameterPack()));
  arboretum_create_edge(obj, context_.data_model_.method_isPreviousDeclInSameBlockScope_, context_.data_model_.arboretum_node_for(D->isPreviousDeclInSameBlockScope()));
  arboretum_create_edge(obj, context_.data_model_.method_isEscapingByref_, context_.data_model_.arboretum_node_for(D->isEscapingByref()));
  arboretum_create_edge(obj, context_.data_model_.method_isNonEscapingByref_, context_.data_model_.arboretum_node_for(D->isNonEscapingByref()));
  arboretum_create_edge(obj, context_.data_model_.method_hasDependentAlignment_, context_.data_model_.arboretum_node_for(D->hasDependentAlignment()));
  arboretum_create_edge(obj, context_.data_model_.method_getTemplateInstantiationPattern_2_, context_.resolve(D->getTemplateInstantiationPattern()));
  arboretum_create_edge(obj, context_.data_model_.method_getInstantiatedFromStaticDataMember_, context_.resolve(D->getInstantiatedFromStaticDataMember()));
  arboretum_create_edge(obj, context_.data_model_.method_getTemplateSpecializationKind_3_, context_.data_model_.resolve(D->getTemplateSpecializationKind()));
  arboretum_create_edge(obj, context_.data_model_.method_getTemplateSpecializationKindForInstantiation_1_, context_.data_model_.resolve(D->getTemplateSpecializationKindForInstantiation()));
  arboretum_create_edge(obj, context_.data_model_.method_getPointOfInstantiation_2_, context_.source_model_.resolve(D->getPointOfInstantiation()));
  // getMemberSpecializationInfo ( class clang::MemberSpecializationInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getDescribedVarTemplate_, context_.resolve(D->getDescribedVarTemplate()));
  arboretum_create_edge(obj, context_.data_model_.method_isKnownToBeDefined_, context_.data_model_.arboretum_node_for(D->isKnownToBeDefined()));
  return true;
}

bool ArboretumASTVisitor::VisitVarTemplateDecl(clang::VarTemplateDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getTemplatedDecl_4_, context_.resolve(D->getTemplatedDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_isThisDeclarationADefinition_5_, context_.data_model_.arboretum_node_for(D->isThisDeclarationADefinition()));
  arboretum_create_edge(obj, context_.data_model_.method_getCanonicalDecl_28_, context_.resolve(D->getCanonicalDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getPreviousDecl_7_, context_.resolve(D->getPreviousDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getMostRecentDecl_8_, context_.resolve(D->getMostRecentDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getInstantiatedFromMemberTemplate_5_, context_.resolve(D->getInstantiatedFromMemberTemplate()));
  // specializations ( class llvm::iterator_range<struct clang::RedeclarableTemplateDecl::SpecIterator<class clang::VarTemplateSpecializationDecl> > )
  return true;
}

bool ArboretumASTVisitor::VisitVarTemplatePartialSpecializationDecl(clang::VarTemplatePartialSpecializationDecl* D) {
  const Id* obj = context_.resolve(D);
  // getTemplateParameters ( class clang::TemplateParameterList * )
  // getTemplateArgsAsWritten ( const struct clang::ASTTemplateArgumentListInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_hasAssociatedConstraints_2_, context_.data_model_.arboretum_node_for(D->hasAssociatedConstraints()));
  arboretum_create_edge(obj, context_.data_model_.method_getInstantiatedFromMember_1_, context_.resolve(D->getInstantiatedFromMember()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_37_, context_.source_model_.resolve(D->getSourceRange()));
  return true;
}

bool ArboretumASTVisitor::VisitVarTemplateSpecializationDecl(clang::VarTemplateSpecializationDecl* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getSpecializedTemplate_1_, context_.resolve(D->getSpecializedTemplate()));
  // getTemplateArgs ( const class clang::TemplateArgumentList & )
  // getTemplateArgsInfo ( const struct clang::ASTTemplateArgumentListInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getSpecializationKind_1_, context_.data_model_.resolve(D->getSpecializationKind()));
  arboretum_create_edge(obj, context_.data_model_.method_isExplicitSpecialization_1_, context_.data_model_.arboretum_node_for(D->isExplicitSpecialization()));
  arboretum_create_edge(obj, context_.data_model_.method_isClassScopeExplicitSpecialization_1_, context_.data_model_.arboretum_node_for(D->isClassScopeExplicitSpecialization()));
  arboretum_create_edge(obj, context_.data_model_.method_isExplicitInstantiationOrSpecialization_1_, context_.data_model_.arboretum_node_for(D->isExplicitInstantiationOrSpecialization()));
  arboretum_create_edge(obj, context_.data_model_.method_getPointOfInstantiation_3_, context_.source_model_.resolve(D->getPointOfInstantiation()));
  // getInstantiatedFrom ( class llvm::PointerUnion<class clang::VarTemplateDecl *, class clang::VarTemplatePartialSpecializationDecl *> )
  // getSpecializedTemplateOrPartial ( class llvm::PointerUnion<class clang::VarTemplateDecl *, class clang::VarTemplatePartialSpecializationDecl *> )
  // getTemplateInstantiationArgs ( const class clang::TemplateArgumentList & )
  // getTypeAsWritten ( class clang::TypeSourceInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getExternLoc_2_, context_.source_model_.resolve(D->getExternLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getTemplateKeywordLoc_1_, context_.source_model_.resolve(D->getTemplateKeywordLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_38_, context_.source_model_.resolve(D->getSourceRange()));
  return true;
}


// Stmts
bool ArboretumASTVisitor::VisitAbstractConditionalOperator(clang::AbstractConditionalOperator* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getCond_, context_.resolve(D->getCond()));
  arboretum_create_edge(obj, context_.data_model_.method_getTrueExpr_, context_.resolve(D->getTrueExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getFalseExpr_, context_.resolve(D->getFalseExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getQuestionLoc_, context_.source_model_.resolve(D->getQuestionLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getColonLoc_1_, context_.source_model_.resolve(D->getColonLoc()));
  return true;
}

bool ArboretumASTVisitor::VisitAddrLabelExpr(clang::AddrLabelExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getAmpAmpLoc_, context_.source_model_.resolve(D->getAmpAmpLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getLabelLoc_, context_.source_model_.resolve(D->getLabelLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_4_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_3_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getLabel_, context_.resolve(D->getLabel()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitArrayInitIndexExpr(clang::ArrayInitIndexExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_5_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_4_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitArrayInitLoopExpr(clang::ArrayInitLoopExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getCommonExpr_, context_.resolve(D->getCommonExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getSubExpr_, context_.resolve(D->getSubExpr()));
  // getArraySize ( class llvm::APInt )
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_6_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_5_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitArraySubscriptExpr(clang::ArraySubscriptExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getLHS_, context_.resolve(D->getLHS()));
  arboretum_create_edge(obj, context_.data_model_.method_getRHS_, context_.resolve(D->getRHS()));
  arboretum_create_edge(obj, context_.data_model_.method_getBase_, context_.resolve(D->getBase()));
  arboretum_create_edge(obj, context_.data_model_.method_getIdx_, context_.resolve(D->getIdx()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_7_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_6_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRBracketLoc_2_, context_.source_model_.resolve(D->getRBracketLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getExprLoc_, context_.source_model_.resolve(D->getExprLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitArrayTypeTraitExpr(clang::ArrayTypeTraitExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_8_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_7_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getTrait_, context_.data_model_.resolve(D->getTrait()));
  arboretum_create_edge(obj, context_.data_model_.method_getQueriedType_, context_.resolve(D->getQueriedType()));
  // getQueriedTypeSourceInfo ( class clang::TypeSourceInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getDimensionExpression_, context_.resolve(D->getDimensionExpression()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitAsTypeExpr(clang::AsTypeExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getSrcExpr_, context_.resolve(D->getSrcExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getBuiltinLoc_, context_.source_model_.resolve(D->getBuiltinLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_2_, context_.source_model_.resolve(D->getRParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_9_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_8_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitAsmStmt(clang::AsmStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getAsmLoc_1_, context_.source_model_.resolve(D->getAsmLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_isSimple_, context_.data_model_.arboretum_node_for(D->isSimple()));
  arboretum_create_edge(obj, context_.data_model_.method_isVolatile_2_, context_.data_model_.arboretum_node_for(D->isVolatile()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_10_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_9_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getNumOutputs_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumOutputs())));
  arboretum_create_edge(obj, context_.data_model_.method_getNumPlusOperands_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumPlusOperands())));
  arboretum_create_edge(obj, context_.data_model_.method_getNumInputs_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumInputs())));
  arboretum_create_edge(obj, context_.data_model_.method_getNumClobbers_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumClobbers())));
  // inputs ( class llvm::iterator_range<struct clang::Stmt::CastIterator<class clang::Expr, const class clang::Expr *const, const class clang::Stmt *const> > )
  // outputs ( class llvm::iterator_range<struct clang::Stmt::CastIterator<class clang::Expr, const class clang::Expr *const, const class clang::Stmt *const> > )
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitAtomicExpr(clang::AtomicExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getPtr_, context_.resolve(D->getPtr()));
  arboretum_create_edge(obj, context_.data_model_.method_getOrder_, context_.resolve(D->getOrder()));
  arboretum_create_edge(obj, context_.data_model_.method_getScope_, context_.resolve(D->getScope()));
  arboretum_create_edge(obj, context_.data_model_.method_getVal1_, context_.resolve(D->getVal1()));
  arboretum_create_edge(obj, context_.data_model_.method_getOrderFail_, context_.resolve(D->getOrderFail()));
  arboretum_create_edge(obj, context_.data_model_.method_getVal2_, context_.resolve(D->getVal2()));
  arboretum_create_edge(obj, context_.data_model_.method_getWeak_, context_.resolve(D->getWeak()));
  arboretum_create_edge(obj, context_.data_model_.method_getValueType_1_, context_.resolve(D->getValueType()));
  arboretum_create_edge(obj, context_.data_model_.method_getOp_, context_.data_model_.resolve(D->getOp()));
  arboretum_create_edge(obj, context_.data_model_.method_getOpAsString_, context_.data_model_.arboretum_node_for(D->getOpAsString().str()));
  arboretum_create_edge(obj, context_.data_model_.method_getNumSubExprs_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumSubExprs())));
  // getSubExprs ( const class clang::Expr *const * )
  arboretum_create_edge(obj, context_.data_model_.method_isVolatile_3_, context_.data_model_.arboretum_node_for(D->isVolatile()));
  arboretum_create_edge(obj, context_.data_model_.method_isCmpXChg_, context_.data_model_.arboretum_node_for(D->isCmpXChg()));
  arboretum_create_edge(obj, context_.data_model_.method_isOpenCL_, context_.data_model_.arboretum_node_for(D->isOpenCL()));
  arboretum_create_edge(obj, context_.data_model_.method_getBuiltinLoc_1_, context_.source_model_.resolve(D->getBuiltinLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_3_, context_.source_model_.resolve(D->getRParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_11_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_10_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  // getScopeModel ( class std::unique_ptr<class clang::AtomicScopeModel> )
  return true;
}

bool ArboretumASTVisitor::VisitAttributedStmt(clang::AttributedStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getAttrLoc_, context_.source_model_.resolve(D->getAttrLoc()));
  {
    std::vector<Id*> element_ids;
    auto range = D->getAttrs();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_getAttrs_1_, context_.data_model_.arboretum_node_for(context_.data_model_.class_Attr_, element_ids));
  }
  arboretum_create_edge(obj, context_.data_model_.method_getSubStmt_, context_.resolve(D->getSubStmt()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_12_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_11_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitBinaryConditionalOperator(clang::BinaryConditionalOperator* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getCommon_, context_.resolve(D->getCommon()));
  arboretum_create_edge(obj, context_.data_model_.method_getOpaqueValue_, context_.resolve(D->getOpaqueValue()));
  arboretum_create_edge(obj, context_.data_model_.method_getCond_1_, context_.resolve(D->getCond()));
  arboretum_create_edge(obj, context_.data_model_.method_getTrueExpr_1_, context_.resolve(D->getTrueExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getFalseExpr_1_, context_.resolve(D->getFalseExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_13_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_12_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitBinaryOperator(clang::BinaryOperator* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getExprLoc_1_, context_.source_model_.resolve(D->getExprLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getOperatorLoc_, context_.source_model_.resolve(D->getOperatorLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getOpcode_, context_.data_model_.resolve(D->getOpcode()));
  arboretum_create_edge(obj, context_.data_model_.method_getLHS_1_, context_.resolve(D->getLHS()));
  arboretum_create_edge(obj, context_.data_model_.method_getRHS_1_, context_.resolve(D->getRHS()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_14_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_13_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getOpcodeStr_, context_.data_model_.arboretum_node_for(D->getOpcodeStr().str()));
  arboretum_create_edge(obj, context_.data_model_.method_isPtrMemOp_, context_.data_model_.arboretum_node_for(D->isPtrMemOp()));
  arboretum_create_edge(obj, context_.data_model_.method_isMultiplicativeOp_, context_.data_model_.arboretum_node_for(D->isMultiplicativeOp()));
  arboretum_create_edge(obj, context_.data_model_.method_isAdditiveOp_, context_.data_model_.arboretum_node_for(D->isAdditiveOp()));
  arboretum_create_edge(obj, context_.data_model_.method_isShiftOp_, context_.data_model_.arboretum_node_for(D->isShiftOp()));
  arboretum_create_edge(obj, context_.data_model_.method_isBitwiseOp_, context_.data_model_.arboretum_node_for(D->isBitwiseOp()));
  arboretum_create_edge(obj, context_.data_model_.method_isRelationalOp_, context_.data_model_.arboretum_node_for(D->isRelationalOp()));
  arboretum_create_edge(obj, context_.data_model_.method_isEqualityOp_, context_.data_model_.arboretum_node_for(D->isEqualityOp()));
  arboretum_create_edge(obj, context_.data_model_.method_isComparisonOp_, context_.data_model_.arboretum_node_for(D->isComparisonOp()));
  arboretum_create_edge(obj, context_.data_model_.method_isCommaOp_, context_.data_model_.arboretum_node_for(D->isCommaOp()));
  arboretum_create_edge(obj, context_.data_model_.method_isLogicalOp_, context_.data_model_.arboretum_node_for(D->isLogicalOp()));
  arboretum_create_edge(obj, context_.data_model_.method_isAssignmentOp_, context_.data_model_.arboretum_node_for(D->isAssignmentOp()));
  arboretum_create_edge(obj, context_.data_model_.method_isCompoundAssignmentOp_, context_.data_model_.arboretum_node_for(D->isCompoundAssignmentOp()));
  arboretum_create_edge(obj, context_.data_model_.method_isShiftAssignOp_, context_.data_model_.arboretum_node_for(D->isShiftAssignOp()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  arboretum_create_edge(obj, context_.data_model_.method_hasStoredFPFeatures_, context_.data_model_.arboretum_node_for(D->hasStoredFPFeatures()));
  // getStoredFPFeatures ( class clang::FPOptionsOverride )
  // getFPFeatures ( class clang::FPOptionsOverride )
  return true;
}

bool ArboretumASTVisitor::VisitBlockExpr(clang::BlockExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getBlockDecl_, context_.resolve(D->getBlockDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getCaretLocation_1_, context_.source_model_.resolve(D->getCaretLocation()));
  arboretum_create_edge(obj, context_.data_model_.method_getBody_4_, context_.resolve(D->getBody()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_15_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_14_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getFunctionType_, context_.resolve(D->getFunctionType()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitBreakStmt(clang::BreakStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getBreakLoc_, context_.source_model_.resolve(D->getBreakLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_16_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_15_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitBuiltinBitCastExpr(clang::BuiltinBitCastExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_17_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_16_, context_.source_model_.resolve(D->getEndLoc()));
  return true;
}

bool ArboretumASTVisitor::VisitCStyleCastExpr(clang::CStyleCastExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getLParenLoc_, context_.source_model_.resolve(D->getLParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_4_, context_.source_model_.resolve(D->getRParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_18_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_17_, context_.source_model_.resolve(D->getEndLoc()));
  return true;
}

bool ArboretumASTVisitor::VisitCUDAKernelCallExpr(clang::CUDAKernelCallExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getConfig_, context_.resolve(D->getConfig()));
  return true;
}

bool ArboretumASTVisitor::VisitCXXAddrspaceCastExpr(clang::CXXAddrspaceCastExpr* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCXXBindTemporaryExpr(clang::CXXBindTemporaryExpr* D) {
  const Id* obj = context_.resolve(D);
  // getTemporary ( const class clang::CXXTemporary * )
  arboretum_create_edge(obj, context_.data_model_.method_getSubExpr_1_, context_.resolve(D->getSubExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_19_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_18_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCXXBoolLiteralExpr(clang::CXXBoolLiteralExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getValue_4_, context_.data_model_.arboretum_node_for(D->getValue()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_20_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_19_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getLocation_1_, context_.source_model_.resolve(D->getLocation()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCXXCatchStmt(clang::CXXCatchStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_21_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_20_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getCatchLoc_, context_.source_model_.resolve(D->getCatchLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getExceptionDecl_, context_.resolve(D->getExceptionDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getCaughtType_, context_.resolve(D->getCaughtType()));
  arboretum_create_edge(obj, context_.data_model_.method_getHandlerBlock_, context_.resolve(D->getHandlerBlock()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCXXConstCastExpr(clang::CXXConstCastExpr* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCXXConstructExpr(clang::CXXConstructExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getConstructor_, context_.resolve(D->getConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_getLocation_2_, context_.source_model_.resolve(D->getLocation()));
  arboretum_create_edge(obj, context_.data_model_.method_isElidable_, context_.data_model_.arboretum_node_for(D->isElidable()));
  arboretum_create_edge(obj, context_.data_model_.method_hadMultipleCandidates_, context_.data_model_.arboretum_node_for(D->hadMultipleCandidates()));
  arboretum_create_edge(obj, context_.data_model_.method_isListInitialization_, context_.data_model_.arboretum_node_for(D->isListInitialization()));
  arboretum_create_edge(obj, context_.data_model_.method_isStdInitListInitialization_, context_.data_model_.arboretum_node_for(D->isStdInitListInitialization()));
  arboretum_create_edge(obj, context_.data_model_.method_requiresZeroInitialization_, context_.data_model_.arboretum_node_for(D->requiresZeroInitialization()));
  arboretum_create_edge(obj, context_.data_model_.method_getConstructionKind_, context_.data_model_.resolve(D->getConstructionKind()));
  // arguments ( class llvm::iterator_range<struct clang::Stmt::CastIterator<class clang::Expr, const class clang::Expr *const, const class clang::Stmt *const> > )
  // getArgs ( const class clang::Expr *const * )
  arboretum_create_edge(obj, context_.data_model_.method_getNumArgs_1_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumArgs())));
  arboretum_create_edge(obj, context_.data_model_.method_isImmediateEscalating_1_, context_.data_model_.arboretum_node_for(D->isImmediateEscalating()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_22_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_21_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getParenOrBraceRange_, context_.source_model_.resolve(D->getParenOrBraceRange()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCXXDefaultArgExpr(clang::CXXDefaultArgExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getParam_, context_.resolve(D->getParam()));
  arboretum_create_edge(obj, context_.data_model_.method_hasRewrittenInit_, context_.data_model_.arboretum_node_for(D->hasRewrittenInit()));
  arboretum_create_edge(obj, context_.data_model_.method_getExpr_, context_.resolve(D->getExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getRewrittenExpr_, context_.resolve(D->getRewrittenExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getAdjustedRewrittenExpr_, context_.resolve(D->getAdjustedRewrittenExpr()));
  // getUsedContext ( const class clang::DeclContext * )
  arboretum_create_edge(obj, context_.data_model_.method_getUsedLocation_, context_.source_model_.resolve(D->getUsedLocation()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_23_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_22_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getExprLoc_2_, context_.source_model_.resolve(D->getExprLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCXXDefaultInitExpr(clang::CXXDefaultInitExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_hasRewrittenInit_1_, context_.data_model_.arboretum_node_for(D->hasRewrittenInit()));
  arboretum_create_edge(obj, context_.data_model_.method_getField_, context_.resolve(D->getField()));
  arboretum_create_edge(obj, context_.data_model_.method_getExpr_1_, context_.resolve(D->getExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getRewrittenExpr_1_, context_.resolve(D->getRewrittenExpr()));
  // getUsedContext ( const class clang::DeclContext * )
  arboretum_create_edge(obj, context_.data_model_.method_getUsedLocation_1_, context_.source_model_.resolve(D->getUsedLocation()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_24_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_23_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCXXDeleteExpr(clang::CXXDeleteExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_isGlobalDelete_, context_.data_model_.arboretum_node_for(D->isGlobalDelete()));
  arboretum_create_edge(obj, context_.data_model_.method_isArrayForm_, context_.data_model_.arboretum_node_for(D->isArrayForm()));
  arboretum_create_edge(obj, context_.data_model_.method_isArrayFormAsWritten_, context_.data_model_.arboretum_node_for(D->isArrayFormAsWritten()));
  arboretum_create_edge(obj, context_.data_model_.method_doesUsualArrayDeleteWantSize_, context_.data_model_.arboretum_node_for(D->doesUsualArrayDeleteWantSize()));
  arboretum_create_edge(obj, context_.data_model_.method_getOperatorDelete_1_, context_.resolve(D->getOperatorDelete()));
  arboretum_create_edge(obj, context_.data_model_.method_getArgument_, context_.resolve(D->getArgument()));
  arboretum_create_edge(obj, context_.data_model_.method_getDestroyedType_, context_.resolve(D->getDestroyedType()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_25_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_24_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCXXDependentScopeMemberExpr(clang::CXXDependentScopeMemberExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_isImplicitAccess_, context_.data_model_.arboretum_node_for(D->isImplicitAccess()));
  arboretum_create_edge(obj, context_.data_model_.method_getBaseType_1_, context_.resolve(D->getBaseType()));
  arboretum_create_edge(obj, context_.data_model_.method_isArrow_, context_.data_model_.arboretum_node_for(D->isArrow()));
  arboretum_create_edge(obj, context_.data_model_.method_getOperatorLoc_1_, context_.source_model_.resolve(D->getOperatorLoc()));
  // getQualifier ( class clang::NestedNameSpecifier * )
  // getQualifierLoc ( class clang::NestedNameSpecifierLoc )
  arboretum_create_edge(obj, context_.data_model_.method_getFirstQualifierFoundInScope_, context_.resolve(D->getFirstQualifierFoundInScope()));
  // getMemberNameInfo ( const struct clang::DeclarationNameInfo & )
  // getMember ( class clang::DeclarationName )
  arboretum_create_edge(obj, context_.data_model_.method_getMemberLoc_, context_.source_model_.resolve(D->getMemberLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getTemplateKeywordLoc_2_, context_.source_model_.resolve(D->getTemplateKeywordLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getLAngleLoc_, context_.source_model_.resolve(D->getLAngleLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRAngleLoc_, context_.source_model_.resolve(D->getRAngleLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_hasTemplateKeyword_, context_.data_model_.arboretum_node_for(D->hasTemplateKeyword()));
  arboretum_create_edge(obj, context_.data_model_.method_hasExplicitTemplateArgs_, context_.data_model_.arboretum_node_for(D->hasExplicitTemplateArgs()));
  // getTemplateArgs ( const class clang::TemplateArgumentLoc * )
  arboretum_create_edge(obj, context_.data_model_.method_getNumTemplateArgs_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumTemplateArgs())));
  // template_arguments ( class llvm::ArrayRef<class clang::TemplateArgumentLoc> )
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_26_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_25_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCXXDynamicCastExpr(clang::CXXDynamicCastExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_isAlwaysNull_, context_.data_model_.arboretum_node_for(D->isAlwaysNull()));
  return true;
}

bool ArboretumASTVisitor::VisitCXXFoldExpr(clang::CXXFoldExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getCallee_, context_.resolve(D->getCallee()));
  arboretum_create_edge(obj, context_.data_model_.method_getLHS_2_, context_.resolve(D->getLHS()));
  arboretum_create_edge(obj, context_.data_model_.method_getRHS_2_, context_.resolve(D->getRHS()));
  arboretum_create_edge(obj, context_.data_model_.method_isRightFold_, context_.data_model_.arboretum_node_for(D->isRightFold()));
  arboretum_create_edge(obj, context_.data_model_.method_isLeftFold_, context_.data_model_.arboretum_node_for(D->isLeftFold()));
  arboretum_create_edge(obj, context_.data_model_.method_getPattern_1_, context_.resolve(D->getPattern()));
  arboretum_create_edge(obj, context_.data_model_.method_getInit_1_, context_.resolve(D->getInit()));
  arboretum_create_edge(obj, context_.data_model_.method_getLParenLoc_1_, context_.source_model_.resolve(D->getLParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_5_, context_.source_model_.resolve(D->getRParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEllipsisLoc_4_, context_.source_model_.resolve(D->getEllipsisLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getOperator_, context_.data_model_.resolve(D->getOperator()));
  // getNumExpansions ( class std::optional<unsigned int> )
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_27_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_26_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCXXForRangeStmt(clang::CXXForRangeStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getInit_2_, context_.resolve(D->getInit()));
  arboretum_create_edge(obj, context_.data_model_.method_getLoopVariable_, context_.resolve(D->getLoopVariable()));
  arboretum_create_edge(obj, context_.data_model_.method_getRangeInit_, context_.resolve(D->getRangeInit()));
  arboretum_create_edge(obj, context_.data_model_.method_getRangeStmt_, context_.resolve(D->getRangeStmt()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginStmt_, context_.resolve(D->getBeginStmt()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndStmt_, context_.resolve(D->getEndStmt()));
  arboretum_create_edge(obj, context_.data_model_.method_getCond_2_, context_.resolve(D->getCond()));
  arboretum_create_edge(obj, context_.data_model_.method_getInc_, context_.resolve(D->getInc()));
  arboretum_create_edge(obj, context_.data_model_.method_getLoopVarStmt_, context_.resolve(D->getLoopVarStmt()));
  arboretum_create_edge(obj, context_.data_model_.method_getBody_5_, context_.resolve(D->getBody()));
  arboretum_create_edge(obj, context_.data_model_.method_getForLoc_, context_.source_model_.resolve(D->getForLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getCoawaitLoc_, context_.source_model_.resolve(D->getCoawaitLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getColonLoc_2_, context_.source_model_.resolve(D->getColonLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_6_, context_.source_model_.resolve(D->getRParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_28_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_27_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCXXFunctionalCastExpr(clang::CXXFunctionalCastExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getLParenLoc_2_, context_.source_model_.resolve(D->getLParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_7_, context_.source_model_.resolve(D->getRParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_isListInitialization_1_, context_.data_model_.arboretum_node_for(D->isListInitialization()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_29_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_28_, context_.source_model_.resolve(D->getEndLoc()));
  return true;
}

bool ArboretumASTVisitor::VisitCXXInheritedCtorInitExpr(clang::CXXInheritedCtorInitExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getConstructor_1_, context_.resolve(D->getConstructor()));
  arboretum_create_edge(obj, context_.data_model_.method_constructsVBase_, context_.data_model_.arboretum_node_for(D->constructsVBase()));
  arboretum_create_edge(obj, context_.data_model_.method_getConstructionKind_1_, context_.data_model_.resolve(D->getConstructionKind()));
  arboretum_create_edge(obj, context_.data_model_.method_inheritedFromVBase_, context_.data_model_.arboretum_node_for(D->inheritedFromVBase()));
  arboretum_create_edge(obj, context_.data_model_.method_getLocation_3_, context_.source_model_.resolve(D->getLocation()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_30_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_29_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCXXMemberCallExpr(clang::CXXMemberCallExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getImplicitObjectArgument_, context_.resolve(D->getImplicitObjectArgument()));
  arboretum_create_edge(obj, context_.data_model_.method_getObjectType_, context_.resolve(D->getObjectType()));
  arboretum_create_edge(obj, context_.data_model_.method_getMethodDecl_, context_.resolve(D->getMethodDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getRecordDecl_, context_.resolve(D->getRecordDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getExprLoc_3_, context_.source_model_.resolve(D->getExprLoc()));
  return true;
}

bool ArboretumASTVisitor::VisitCXXNamedCastExpr(clang::CXXNamedCastExpr* D) {
  const Id* obj = context_.resolve(D);
  // getCastName ( const char * )
  arboretum_create_edge(obj, context_.data_model_.method_getOperatorLoc_2_, context_.source_model_.resolve(D->getOperatorLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_8_, context_.source_model_.resolve(D->getRParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_31_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_30_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getAngleBrackets_, context_.source_model_.resolve(D->getAngleBrackets()));
  return true;
}

bool ArboretumASTVisitor::VisitCXXNewExpr(clang::CXXNewExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getAllocatedType_, context_.resolve(D->getAllocatedType()));
  // getAllocatedTypeSourceInfo ( class clang::TypeSourceInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getOperatorNew_, context_.resolve(D->getOperatorNew()));
  arboretum_create_edge(obj, context_.data_model_.method_getOperatorDelete_2_, context_.resolve(D->getOperatorDelete()));
  arboretum_create_edge(obj, context_.data_model_.method_isArray_, context_.data_model_.arboretum_node_for(D->isArray()));
  // getArraySize ( class std::optional<const class clang::Expr *> )
  arboretum_create_edge(obj, context_.data_model_.method_getNumPlacementArgs_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumPlacementArgs())));
  arboretum_create_edge(obj, context_.data_model_.method_isParenTypeId_, context_.data_model_.arboretum_node_for(D->isParenTypeId()));
  arboretum_create_edge(obj, context_.data_model_.method_getTypeIdParens_, context_.source_model_.resolve(D->getTypeIdParens()));
  arboretum_create_edge(obj, context_.data_model_.method_isGlobalNew_, context_.data_model_.arboretum_node_for(D->isGlobalNew()));
  arboretum_create_edge(obj, context_.data_model_.method_hasInitializer_, context_.data_model_.arboretum_node_for(D->hasInitializer()));
  arboretum_create_edge(obj, context_.data_model_.method_getInitializationStyle_, context_.data_model_.resolve(D->getInitializationStyle()));
  arboretum_create_edge(obj, context_.data_model_.method_getInitializer_, context_.resolve(D->getInitializer()));
  arboretum_create_edge(obj, context_.data_model_.method_getConstructExpr_, context_.resolve(D->getConstructExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_passAlignment_, context_.data_model_.arboretum_node_for(D->passAlignment()));
  arboretum_create_edge(obj, context_.data_model_.method_doesUsualArrayDeleteWantSize_1_, context_.data_model_.arboretum_node_for(D->doesUsualArrayDeleteWantSize()));
  // placement_arguments ( class llvm::iterator_range<struct clang::Stmt::CastIterator<class clang::Expr, const class clang::Expr *const, const class clang::Stmt *const> > )
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_32_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_31_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getDirectInitRange_, context_.source_model_.resolve(D->getDirectInitRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_39_, context_.source_model_.resolve(D->getSourceRange()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCXXNoexceptExpr(clang::CXXNoexceptExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getOperand_, context_.resolve(D->getOperand()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_33_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_32_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_40_, context_.source_model_.resolve(D->getSourceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getValue_5_, context_.data_model_.arboretum_node_for(D->getValue()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCXXNullPtrLiteralExpr(clang::CXXNullPtrLiteralExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_34_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_33_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getLocation_4_, context_.source_model_.resolve(D->getLocation()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCXXOperatorCallExpr(clang::CXXOperatorCallExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getOperator_1_, context_.data_model_.resolve(D->getOperator()));
  arboretum_create_edge(obj, context_.data_model_.method_isAssignmentOp_1_, context_.data_model_.arboretum_node_for(D->isAssignmentOp()));
  arboretum_create_edge(obj, context_.data_model_.method_isComparisonOp_1_, context_.data_model_.arboretum_node_for(D->isComparisonOp()));
  arboretum_create_edge(obj, context_.data_model_.method_isInfixBinaryOp_, context_.data_model_.arboretum_node_for(D->isInfixBinaryOp()));
  arboretum_create_edge(obj, context_.data_model_.method_getOperatorLoc_3_, context_.source_model_.resolve(D->getOperatorLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getExprLoc_4_, context_.source_model_.resolve(D->getExprLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_35_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_34_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_41_, context_.source_model_.resolve(D->getSourceRange()));
  return true;
}

bool ArboretumASTVisitor::VisitCXXParenListInitExpr(clang::CXXParenListInitExpr* D) {
  const Id* obj = context_.resolve(D);
  {
    std::vector<Id*> element_ids;
    auto range = D->getInitExprs();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_getInitExprs_, context_.data_model_.arboretum_node_for(context_.data_model_.class_Expr_, element_ids));
  }
  {
    std::vector<Id*> element_ids;
    auto range = D->getUserSpecifiedInitExprs();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_getUserSpecifiedInitExprs_, context_.data_model_.arboretum_node_for(context_.data_model_.class_Expr_, element_ids));
  }
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_36_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_35_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getInitLoc_, context_.source_model_.resolve(D->getInitLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_42_, context_.source_model_.resolve(D->getSourceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getArrayFiller_, context_.resolve(D->getArrayFiller()));
  arboretum_create_edge(obj, context_.data_model_.method_getInitializedFieldInUnion_, context_.resolve(D->getInitializedFieldInUnion()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCXXPseudoDestructorExpr(clang::CXXPseudoDestructorExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getBase_1_, context_.resolve(D->getBase()));
  arboretum_create_edge(obj, context_.data_model_.method_hasQualifier_, context_.data_model_.arboretum_node_for(D->hasQualifier()));
  // getQualifierLoc ( class clang::NestedNameSpecifierLoc )
  // getQualifier ( class clang::NestedNameSpecifier * )
  arboretum_create_edge(obj, context_.data_model_.method_isArrow_1_, context_.data_model_.arboretum_node_for(D->isArrow()));
  arboretum_create_edge(obj, context_.data_model_.method_getOperatorLoc_4_, context_.source_model_.resolve(D->getOperatorLoc()));
  // getScopeTypeInfo ( class clang::TypeSourceInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getColonColonLoc_, context_.source_model_.resolve(D->getColonColonLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getTildeLoc_, context_.source_model_.resolve(D->getTildeLoc()));
  // getDestroyedTypeInfo ( class clang::TypeSourceInfo * )
  // getDestroyedTypeIdentifier ( class clang::IdentifierInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getDestroyedType_1_, context_.resolve(D->getDestroyedType()));
  arboretum_create_edge(obj, context_.data_model_.method_getDestroyedTypeLoc_, context_.source_model_.resolve(D->getDestroyedTypeLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_37_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_36_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCXXReinterpretCastExpr(clang::CXXReinterpretCastExpr* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCXXRewrittenBinaryOperator(clang::CXXRewrittenBinaryOperator* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getSemanticForm_, context_.resolve(D->getSemanticForm()));
  // getDecomposedForm ( struct clang::CXXRewrittenBinaryOperator::DecomposedForm )
  arboretum_create_edge(obj, context_.data_model_.method_isReversed_, context_.data_model_.arboretum_node_for(D->isReversed()));
  arboretum_create_edge(obj, context_.data_model_.method_getOperator_2_, context_.data_model_.resolve(D->getOperator()));
  arboretum_create_edge(obj, context_.data_model_.method_getOpcode_1_, context_.data_model_.resolve(D->getOpcode()));
  arboretum_create_edge(obj, context_.data_model_.method_getOpcodeStr_1_, context_.data_model_.arboretum_node_for(D->getOpcodeStr().str()));
  arboretum_create_edge(obj, context_.data_model_.method_isComparisonOp_2_, context_.data_model_.arboretum_node_for(D->isComparisonOp()));
  arboretum_create_edge(obj, context_.data_model_.method_isAssignmentOp_2_, context_.data_model_.arboretum_node_for(D->isAssignmentOp()));
  arboretum_create_edge(obj, context_.data_model_.method_getLHS_3_, context_.resolve(D->getLHS()));
  arboretum_create_edge(obj, context_.data_model_.method_getRHS_3_, context_.resolve(D->getRHS()));
  arboretum_create_edge(obj, context_.data_model_.method_getOperatorLoc_5_, context_.source_model_.resolve(D->getOperatorLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getExprLoc_5_, context_.source_model_.resolve(D->getExprLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_38_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_37_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_43_, context_.source_model_.resolve(D->getSourceRange()));
  return true;
}

bool ArboretumASTVisitor::VisitCXXScalarValueInitExpr(clang::CXXScalarValueInitExpr* D) {
  const Id* obj = context_.resolve(D);
  // getTypeSourceInfo ( class clang::TypeSourceInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_9_, context_.source_model_.resolve(D->getRParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_39_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_38_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCXXStaticCastExpr(clang::CXXStaticCastExpr* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCXXStdInitializerListExpr(clang::CXXStdInitializerListExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getSubExpr_2_, context_.resolve(D->getSubExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_40_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_39_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_44_, context_.source_model_.resolve(D->getSourceRange()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCXXTemporaryObjectExpr(clang::CXXTemporaryObjectExpr* D) {
  const Id* obj = context_.resolve(D);
  // getTypeSourceInfo ( class clang::TypeSourceInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_41_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_40_, context_.source_model_.resolve(D->getEndLoc()));
  return true;
}

bool ArboretumASTVisitor::VisitCXXThisExpr(clang::CXXThisExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getLocation_5_, context_.source_model_.resolve(D->getLocation()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_42_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_41_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_isImplicit_1_, context_.data_model_.arboretum_node_for(D->isImplicit()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCXXThrowExpr(clang::CXXThrowExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getSubExpr_3_, context_.resolve(D->getSubExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getThrowLoc_, context_.source_model_.resolve(D->getThrowLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_isThrownVariableInScope_, context_.data_model_.arboretum_node_for(D->isThrownVariableInScope()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_43_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_42_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCXXTryStmt(clang::CXXTryStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_44_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getTryLoc_, context_.source_model_.resolve(D->getTryLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_43_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getTryBlock_, context_.resolve(D->getTryBlock()));
  arboretum_create_edge(obj, context_.data_model_.method_getNumHandlers_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumHandlers())));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCXXTypeidExpr(clang::CXXTypeidExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_isPotentiallyEvaluated_, context_.data_model_.arboretum_node_for(D->isPotentiallyEvaluated()));
  arboretum_create_edge(obj, context_.data_model_.method_isTypeOperand_, context_.data_model_.arboretum_node_for(D->isTypeOperand()));
  // getTypeOperandSourceInfo ( class clang::TypeSourceInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getExprOperand_, context_.resolve(D->getExprOperand()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_45_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_44_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_45_, context_.source_model_.resolve(D->getSourceRange()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCXXUnresolvedConstructExpr(clang::CXXUnresolvedConstructExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getTypeAsWritten_2_, context_.resolve(D->getTypeAsWritten()));
  // getTypeSourceInfo ( class clang::TypeSourceInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getLParenLoc_3_, context_.source_model_.resolve(D->getLParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_10_, context_.source_model_.resolve(D->getRParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_isListInitialization_2_, context_.data_model_.arboretum_node_for(D->isListInitialization()));
  arboretum_create_edge(obj, context_.data_model_.method_getNumArgs_2_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumArgs())));
  {
    std::vector<Id*> element_ids;
    auto range = D->arguments();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_arguments_1_, context_.data_model_.arboretum_node_for(context_.data_model_.class_Expr_, element_ids));
  }
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_46_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_45_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCXXUuidofExpr(clang::CXXUuidofExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_isTypeOperand_1_, context_.data_model_.arboretum_node_for(D->isTypeOperand()));
  // getTypeOperandSourceInfo ( class clang::TypeSourceInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getExprOperand_1_, context_.resolve(D->getExprOperand()));
  arboretum_create_edge(obj, context_.data_model_.method_getGuidDecl_, context_.resolve(D->getGuidDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_47_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_46_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_46_, context_.source_model_.resolve(D->getSourceRange()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCallExpr(clang::CallExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getCallee_1_, context_.resolve(D->getCallee()));
  arboretum_create_edge(obj, context_.data_model_.method_getADLCallKind_, context_.data_model_.resolve(D->getADLCallKind()));
  arboretum_create_edge(obj, context_.data_model_.method_usesADL_, context_.data_model_.arboretum_node_for(D->usesADL()));
  arboretum_create_edge(obj, context_.data_model_.method_hasStoredFPFeatures_1_, context_.data_model_.arboretum_node_for(D->hasStoredFPFeatures()));
  arboretum_create_edge(obj, context_.data_model_.method_getCalleeDecl_, context_.resolve(D->getCalleeDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getDirectCallee_, context_.resolve(D->getDirectCallee()));
  arboretum_create_edge(obj, context_.data_model_.method_getNumArgs_3_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumArgs())));
  // getArgs ( const class clang::Expr *const * )
  // arguments ( class llvm::iterator_range<struct clang::Stmt::CastIterator<class clang::Expr, const class clang::Expr *const, const class clang::Stmt *const> > )
  // getStoredFPFeatures ( class clang::FPOptionsOverride )
  // getFPFeatures ( class clang::FPOptionsOverride )
  arboretum_create_edge(obj, context_.data_model_.method_getBuiltinCallee_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getBuiltinCallee())));
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_11_, context_.source_model_.resolve(D->getRParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_48_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_47_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_isCallToStdMove_, context_.data_model_.arboretum_node_for(D->isCallToStdMove()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCapturedStmt(clang::CapturedStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getCapturedStmt_, context_.resolve(D->getCapturedStmt()));
  arboretum_create_edge(obj, context_.data_model_.method_getCapturedDecl_, context_.resolve(D->getCapturedDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getCapturedRegionKind_, context_.data_model_.resolve(D->getCapturedRegionKind()));
  arboretum_create_edge(obj, context_.data_model_.method_getCapturedRecordDecl_, context_.resolve(D->getCapturedRecordDecl()));
  // captures ( class llvm::iterator_range<const class clang::CapturedStmt::Capture *> )
  arboretum_create_edge(obj, context_.data_model_.method_capture_size_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->capture_size())));
  {
    std::vector<Id*> element_ids;
    auto range = D->capture_inits();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_capture_inits_, context_.data_model_.arboretum_node_for(context_.data_model_.class_Expr_, element_ids));
  }
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_49_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_48_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_47_, context_.source_model_.resolve(D->getSourceRange()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCaseStmt(clang::CaseStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_caseStmtIsGNURange_, context_.data_model_.arboretum_node_for(D->caseStmtIsGNURange()));
  arboretum_create_edge(obj, context_.data_model_.method_getCaseLoc_, context_.source_model_.resolve(D->getCaseLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEllipsisLoc_5_, context_.source_model_.resolve(D->getEllipsisLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getLHS_4_, context_.resolve(D->getLHS()));
  arboretum_create_edge(obj, context_.data_model_.method_getRHS_4_, context_.resolve(D->getRHS()));
  arboretum_create_edge(obj, context_.data_model_.method_getSubStmt_1_, context_.resolve(D->getSubStmt()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_50_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_49_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCastExpr(clang::CastExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getCastKind_, context_.data_model_.resolve(D->getCastKind()));
  // getCastKindName ( const char * )
  arboretum_create_edge(obj, context_.data_model_.method_getSubExpr_4_, context_.resolve(D->getSubExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getSubExprAsWritten_, context_.resolve(D->getSubExprAsWritten()));
  arboretum_create_edge(obj, context_.data_model_.method_getConversionFunction_, context_.resolve(D->getConversionFunction()));
  arboretum_create_edge(obj, context_.data_model_.method_path_empty_, context_.data_model_.arboretum_node_for(D->path_empty()));
  arboretum_create_edge(obj, context_.data_model_.method_path_size_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->path_size())));
  // path ( class llvm::iterator_range<const class clang::CXXBaseSpecifier *const *> )
  arboretum_create_edge(obj, context_.data_model_.method_hasStoredFPFeatures_2_, context_.data_model_.arboretum_node_for(D->hasStoredFPFeatures()));
  // getStoredFPFeatures ( class clang::FPOptionsOverride )
  // getFPFeatures ( class clang::FPOptionsOverride )
  arboretum_create_edge(obj, context_.data_model_.method_changesVolatileQualification_, context_.data_model_.arboretum_node_for(D->changesVolatileQualification()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCharacterLiteral(clang::CharacterLiteral* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getLocation_6_, context_.source_model_.resolve(D->getLocation()));
  arboretum_create_edge(obj, context_.data_model_.method_getKind_3_, context_.data_model_.resolve(D->getKind()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_51_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_50_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getValue_6_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getValue())));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitChooseExpr(clang::ChooseExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_isConditionTrue_, context_.data_model_.arboretum_node_for(D->isConditionTrue()));
  arboretum_create_edge(obj, context_.data_model_.method_isConditionDependent_, context_.data_model_.arboretum_node_for(D->isConditionDependent()));
  arboretum_create_edge(obj, context_.data_model_.method_getChosenSubExpr_, context_.resolve(D->getChosenSubExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getCond_3_, context_.resolve(D->getCond()));
  arboretum_create_edge(obj, context_.data_model_.method_getLHS_5_, context_.resolve(D->getLHS()));
  arboretum_create_edge(obj, context_.data_model_.method_getRHS_5_, context_.resolve(D->getRHS()));
  arboretum_create_edge(obj, context_.data_model_.method_getBuiltinLoc_2_, context_.source_model_.resolve(D->getBuiltinLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_12_, context_.source_model_.resolve(D->getRParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_52_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_51_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCoawaitExpr(clang::CoawaitExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_isImplicit_2_, context_.data_model_.arboretum_node_for(D->isImplicit()));
  return true;
}

bool ArboretumASTVisitor::VisitCompoundAssignOperator(clang::CompoundAssignOperator* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getComputationLHSType_, context_.resolve(D->getComputationLHSType()));
  arboretum_create_edge(obj, context_.data_model_.method_getComputationResultType_, context_.resolve(D->getComputationResultType()));
  return true;
}

bool ArboretumASTVisitor::VisitCompoundLiteralExpr(clang::CompoundLiteralExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getInitializer_1_, context_.resolve(D->getInitializer()));
  arboretum_create_edge(obj, context_.data_model_.method_isFileScope_, context_.data_model_.arboretum_node_for(D->isFileScope()));
  arboretum_create_edge(obj, context_.data_model_.method_getLParenLoc_4_, context_.source_model_.resolve(D->getLParenLoc()));
  // getTypeSourceInfo ( class clang::TypeSourceInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_53_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_52_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCompoundStmt(clang::CompoundStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_body_empty_, context_.data_model_.arboretum_node_for(D->body_empty()));
  arboretum_create_edge(obj, context_.data_model_.method_size_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->size())));
  arboretum_create_edge(obj, context_.data_model_.method_hasStoredFPFeatures_3_, context_.data_model_.arboretum_node_for(D->hasStoredFPFeatures()));
  // getStoredFPFeatures ( class clang::FPOptionsOverride )
  {
    std::vector<Id*> element_ids;
    auto range = D->body();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_body_, context_.data_model_.arboretum_node_for(context_.data_model_.class_Stmt_, element_ids));
  }
  arboretum_create_edge(obj, context_.data_model_.method_body_front_, context_.resolve(D->body_front()));
  arboretum_create_edge(obj, context_.data_model_.method_body_back_, context_.resolve(D->body_back()));
  arboretum_create_edge(obj, context_.data_model_.method_getStmtExprResult_, context_.resolve(D->getStmtExprResult()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_54_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_53_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getLBracLoc_, context_.source_model_.resolve(D->getLBracLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRBracLoc_, context_.source_model_.resolve(D->getRBracLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitConceptSpecializationExpr(clang::ConceptSpecializationExpr* D) {
  const Id* obj = context_.resolve(D);
  // getTemplateArguments ( class llvm::ArrayRef<class clang::TemplateArgument> )
  // getConceptReference ( class clang::ConceptReference * )
  arboretum_create_edge(obj, context_.data_model_.method_getNamedConcept_, context_.resolve(D->getNamedConcept()));
  arboretum_create_edge(obj, context_.data_model_.method_hasExplicitTemplateArgs_1_, context_.data_model_.arboretum_node_for(D->hasExplicitTemplateArgs()));
  arboretum_create_edge(obj, context_.data_model_.method_getConceptNameLoc_, context_.source_model_.resolve(D->getConceptNameLoc()));
  // getTemplateArgsAsWritten ( const struct clang::ASTTemplateArgumentListInfo * )
  // getNestedNameSpecifierLoc ( const class clang::NestedNameSpecifierLoc & )
  arboretum_create_edge(obj, context_.data_model_.method_getTemplateKWLoc_, context_.source_model_.resolve(D->getTemplateKWLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getFoundDecl_1_, context_.resolve(D->getFoundDecl()));
  // getConceptNameInfo ( const struct clang::DeclarationNameInfo & )
  arboretum_create_edge(obj, context_.data_model_.method_getSpecializationDecl_, context_.resolve(D->getSpecializationDecl()));
  // getSatisfaction ( const struct clang::ASTConstraintSatisfaction & )
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_55_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_54_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getExprLoc_6_, context_.source_model_.resolve(D->getExprLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitConditionalOperator(clang::ConditionalOperator* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getCond_4_, context_.resolve(D->getCond()));
  arboretum_create_edge(obj, context_.data_model_.method_getTrueExpr_2_, context_.resolve(D->getTrueExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getFalseExpr_2_, context_.resolve(D->getFalseExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getLHS_6_, context_.resolve(D->getLHS()));
  arboretum_create_edge(obj, context_.data_model_.method_getRHS_6_, context_.resolve(D->getRHS()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_56_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_55_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitConstantExpr(clang::ConstantExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_57_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_56_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getResultAPValueKind_, context_.data_model_.resolve(D->getResultAPValueKind()));
  arboretum_create_edge(obj, context_.data_model_.method_getResultStorageKind_, context_.data_model_.resolve(D->getResultStorageKind()));
  arboretum_create_edge(obj, context_.data_model_.method_isImmediateInvocation_, context_.data_model_.arboretum_node_for(D->isImmediateInvocation()));
  arboretum_create_edge(obj, context_.data_model_.method_hasAPValueResult_, context_.data_model_.arboretum_node_for(D->hasAPValueResult()));
  // getAPValueResult ( class clang::APValue )
  // getResultAsAPSInt ( class llvm::APSInt )
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitContinueStmt(clang::ContinueStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getContinueLoc_, context_.source_model_.resolve(D->getContinueLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_58_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_57_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitConvertVectorExpr(clang::ConvertVectorExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getSrcExpr_1_, context_.resolve(D->getSrcExpr()));
  // getTypeSourceInfo ( class clang::TypeSourceInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getBuiltinLoc_3_, context_.source_model_.resolve(D->getBuiltinLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_13_, context_.source_model_.resolve(D->getRParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_59_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_58_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCoreturnStmt(clang::CoreturnStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getKeywordLoc_, context_.source_model_.resolve(D->getKeywordLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getOperand_1_, context_.resolve(D->getOperand()));
  arboretum_create_edge(obj, context_.data_model_.method_getPromiseCall_, context_.resolve(D->getPromiseCall()));
  arboretum_create_edge(obj, context_.data_model_.method_isImplicit_3_, context_.data_model_.arboretum_node_for(D->isImplicit()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_60_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_59_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCoroutineBodyStmt(clang::CoroutineBodyStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_hasDependentPromiseType_, context_.data_model_.arboretum_node_for(D->hasDependentPromiseType()));
  arboretum_create_edge(obj, context_.data_model_.method_getBody_6_, context_.resolve(D->getBody()));
  arboretum_create_edge(obj, context_.data_model_.method_getPromiseDeclStmt_, context_.resolve(D->getPromiseDeclStmt()));
  arboretum_create_edge(obj, context_.data_model_.method_getPromiseDecl_, context_.resolve(D->getPromiseDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getInitSuspendStmt_, context_.resolve(D->getInitSuspendStmt()));
  arboretum_create_edge(obj, context_.data_model_.method_getFinalSuspendStmt_, context_.resolve(D->getFinalSuspendStmt()));
  arboretum_create_edge(obj, context_.data_model_.method_getExceptionHandler_, context_.resolve(D->getExceptionHandler()));
  arboretum_create_edge(obj, context_.data_model_.method_getFallthroughHandler_, context_.resolve(D->getFallthroughHandler()));
  arboretum_create_edge(obj, context_.data_model_.method_getAllocate_, context_.resolve(D->getAllocate()));
  arboretum_create_edge(obj, context_.data_model_.method_getDeallocate_, context_.resolve(D->getDeallocate()));
  arboretum_create_edge(obj, context_.data_model_.method_getResultDecl_, context_.resolve(D->getResultDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getReturnValueInit_, context_.resolve(D->getReturnValueInit()));
  arboretum_create_edge(obj, context_.data_model_.method_getReturnValue_, context_.resolve(D->getReturnValue()));
  arboretum_create_edge(obj, context_.data_model_.method_getReturnStmt_, context_.resolve(D->getReturnStmt()));
  arboretum_create_edge(obj, context_.data_model_.method_getReturnStmtOnAllocFailure_, context_.resolve(D->getReturnStmtOnAllocFailure()));
  {
    std::vector<Id*> element_ids;
    auto range = D->getParamMoves();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_getParamMoves_, context_.data_model_.arboretum_node_for(context_.data_model_.class_Stmt_, element_ids));
  }
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_61_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_60_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  // childrenExclBody ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCoroutineSuspendExpr(clang::CoroutineSuspendExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getCommonExpr_1_, context_.resolve(D->getCommonExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getOpaqueValue_1_, context_.resolve(D->getOpaqueValue()));
  arboretum_create_edge(obj, context_.data_model_.method_getReadyExpr_, context_.resolve(D->getReadyExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getSuspendExpr_, context_.resolve(D->getSuspendExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getResumeExpr_, context_.resolve(D->getResumeExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getOperand_2_, context_.resolve(D->getOperand()));
  arboretum_create_edge(obj, context_.data_model_.method_getKeywordLoc_1_, context_.source_model_.resolve(D->getKeywordLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_62_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_61_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitCoyieldExpr(clang::CoyieldExpr* D) {
  const Id* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitDeclRefExpr(clang::DeclRefExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getDecl_7_, context_.resolve(D->getDecl()));
  // getNameInfo ( struct clang::DeclarationNameInfo )
  arboretum_create_edge(obj, context_.data_model_.method_getLocation_7_, context_.source_model_.resolve(D->getLocation()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_63_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_62_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_hasQualifier_1_, context_.data_model_.arboretum_node_for(D->hasQualifier()));
  // getQualifierLoc ( class clang::NestedNameSpecifierLoc )
  // getQualifier ( class clang::NestedNameSpecifier * )
  arboretum_create_edge(obj, context_.data_model_.method_getFoundDecl_2_, context_.resolve(D->getFoundDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_hasTemplateKWAndArgsInfo_, context_.data_model_.arboretum_node_for(D->hasTemplateKWAndArgsInfo()));
  arboretum_create_edge(obj, context_.data_model_.method_getTemplateKeywordLoc_3_, context_.source_model_.resolve(D->getTemplateKeywordLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getLAngleLoc_1_, context_.source_model_.resolve(D->getLAngleLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRAngleLoc_1_, context_.source_model_.resolve(D->getRAngleLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_hasTemplateKeyword_1_, context_.data_model_.arboretum_node_for(D->hasTemplateKeyword()));
  arboretum_create_edge(obj, context_.data_model_.method_hasExplicitTemplateArgs_2_, context_.data_model_.arboretum_node_for(D->hasExplicitTemplateArgs()));
  // getTemplateArgs ( const class clang::TemplateArgumentLoc * )
  arboretum_create_edge(obj, context_.data_model_.method_getNumTemplateArgs_1_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumTemplateArgs())));
  // template_arguments ( class llvm::ArrayRef<class clang::TemplateArgumentLoc> )
  arboretum_create_edge(obj, context_.data_model_.method_hadMultipleCandidates_1_, context_.data_model_.arboretum_node_for(D->hadMultipleCandidates()));
  arboretum_create_edge(obj, context_.data_model_.method_isNonOdrUse_, context_.data_model_.resolve(D->isNonOdrUse()));
  arboretum_create_edge(obj, context_.data_model_.method_refersToEnclosingVariableOrCapture_, context_.data_model_.arboretum_node_for(D->refersToEnclosingVariableOrCapture()));
  arboretum_create_edge(obj, context_.data_model_.method_isImmediateEscalating_2_, context_.data_model_.arboretum_node_for(D->isImmediateEscalating()));
  arboretum_create_edge(obj, context_.data_model_.method_isCapturedByCopyInLambdaWithExplicitObjectParameter_, context_.data_model_.arboretum_node_for(D->isCapturedByCopyInLambdaWithExplicitObjectParameter()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitDeclStmt(clang::DeclStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_isSingleDecl_, context_.data_model_.arboretum_node_for(D->isSingleDecl()));
  // getDeclGroup ( const class clang::DeclGroupRef )
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_63_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_64_, context_.source_model_.resolve(D->getBeginLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  {
    std::vector<Id*> element_ids;
    auto range = D->decls();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_decls_, context_.data_model_.arboretum_node_for(context_.data_model_.class_Decl_, element_ids));
  }
  return true;
}

bool ArboretumASTVisitor::VisitDefaultStmt(clang::DefaultStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getSubStmt_2_, context_.resolve(D->getSubStmt()));
  arboretum_create_edge(obj, context_.data_model_.method_getDefaultLoc_1_, context_.source_model_.resolve(D->getDefaultLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_65_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_64_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitDependentCoawaitExpr(clang::DependentCoawaitExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getOperand_3_, context_.resolve(D->getOperand()));
  arboretum_create_edge(obj, context_.data_model_.method_getOperatorCoawaitLookup_, context_.resolve(D->getOperatorCoawaitLookup()));
  arboretum_create_edge(obj, context_.data_model_.method_getKeywordLoc_2_, context_.source_model_.resolve(D->getKeywordLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_66_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_65_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitDependentScopeDeclRefExpr(clang::DependentScopeDeclRefExpr* D) {
  const Id* obj = context_.resolve(D);
  // getNameInfo ( const struct clang::DeclarationNameInfo & )
  // getDeclName ( class clang::DeclarationName )
  arboretum_create_edge(obj, context_.data_model_.method_getLocation_8_, context_.source_model_.resolve(D->getLocation()));
  // getQualifierLoc ( class clang::NestedNameSpecifierLoc )
  // getQualifier ( class clang::NestedNameSpecifier * )
  arboretum_create_edge(obj, context_.data_model_.method_getTemplateKeywordLoc_4_, context_.source_model_.resolve(D->getTemplateKeywordLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getLAngleLoc_2_, context_.source_model_.resolve(D->getLAngleLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRAngleLoc_2_, context_.source_model_.resolve(D->getRAngleLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_hasTemplateKeyword_2_, context_.data_model_.arboretum_node_for(D->hasTemplateKeyword()));
  arboretum_create_edge(obj, context_.data_model_.method_hasExplicitTemplateArgs_3_, context_.data_model_.arboretum_node_for(D->hasExplicitTemplateArgs()));
  // getTemplateArgs ( const class clang::TemplateArgumentLoc * )
  arboretum_create_edge(obj, context_.data_model_.method_getNumTemplateArgs_2_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumTemplateArgs())));
  // template_arguments ( class llvm::ArrayRef<class clang::TemplateArgumentLoc> )
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_67_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_66_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitDesignatedInitExpr(clang::DesignatedInitExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_size_1_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->size())));
  // designators ( class llvm::ArrayRef<class clang::DesignatedInitExpr::Designator> )
  arboretum_create_edge(obj, context_.data_model_.method_getEqualOrColonLoc_, context_.source_model_.resolve(D->getEqualOrColonLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_isDirectInit_1_, context_.data_model_.arboretum_node_for(D->isDirectInit()));
  arboretum_create_edge(obj, context_.data_model_.method_usesGNUSyntax_, context_.data_model_.arboretum_node_for(D->usesGNUSyntax()));
  arboretum_create_edge(obj, context_.data_model_.method_getInit_3_, context_.resolve(D->getInit()));
  arboretum_create_edge(obj, context_.data_model_.method_getNumSubExprs_1_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumSubExprs())));
  arboretum_create_edge(obj, context_.data_model_.method_getDesignatorsSourceRange_, context_.source_model_.resolve(D->getDesignatorsSourceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_68_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_67_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitDesignatedInitUpdateExpr(clang::DesignatedInitUpdateExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_69_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_68_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBase_2_, context_.resolve(D->getBase()));
  arboretum_create_edge(obj, context_.data_model_.method_getUpdater_, context_.resolve(D->getUpdater()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitDoStmt(clang::DoStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getCond_5_, context_.resolve(D->getCond()));
  arboretum_create_edge(obj, context_.data_model_.method_getBody_7_, context_.resolve(D->getBody()));
  arboretum_create_edge(obj, context_.data_model_.method_getDoLoc_, context_.source_model_.resolve(D->getDoLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getWhileLoc_, context_.source_model_.resolve(D->getWhileLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_14_, context_.source_model_.resolve(D->getRParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_70_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_69_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitExplicitCastExpr(clang::ExplicitCastExpr* D) {
  const Id* obj = context_.resolve(D);
  // getTypeInfoAsWritten ( class clang::TypeSourceInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getTypeAsWritten_3_, context_.resolve(D->getTypeAsWritten()));
  return true;
}

bool ArboretumASTVisitor::VisitExpr(clang::Expr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getType_1_, context_.resolve(D->getType()));
  arboretum_create_edge(obj, context_.data_model_.method_getDependence_1_, context_.data_model_.resolve(D->getDependence()));
  arboretum_create_edge(obj, context_.data_model_.method_isValueDependent_, context_.data_model_.arboretum_node_for(D->isValueDependent()));
  arboretum_create_edge(obj, context_.data_model_.method_isTypeDependent_, context_.data_model_.arboretum_node_for(D->isTypeDependent()));
  arboretum_create_edge(obj, context_.data_model_.method_isInstantiationDependent_, context_.data_model_.arboretum_node_for(D->isInstantiationDependent()));
  arboretum_create_edge(obj, context_.data_model_.method_containsUnexpandedParameterPack_1_, context_.data_model_.arboretum_node_for(D->containsUnexpandedParameterPack()));
  arboretum_create_edge(obj, context_.data_model_.method_containsErrors_1_, context_.data_model_.arboretum_node_for(D->containsErrors()));
  arboretum_create_edge(obj, context_.data_model_.method_isLValue_, context_.data_model_.arboretum_node_for(D->isLValue()));
  arboretum_create_edge(obj, context_.data_model_.method_isPRValue_, context_.data_model_.arboretum_node_for(D->isPRValue()));
  arboretum_create_edge(obj, context_.data_model_.method_isXValue_, context_.data_model_.arboretum_node_for(D->isXValue()));
  arboretum_create_edge(obj, context_.data_model_.method_isGLValue_, context_.data_model_.arboretum_node_for(D->isGLValue()));
  arboretum_create_edge(obj, context_.data_model_.method_getValueKind_, context_.data_model_.resolve(D->getValueKind()));
  arboretum_create_edge(obj, context_.data_model_.method_getObjectKind_, context_.data_model_.resolve(D->getObjectKind()));
  arboretum_create_edge(obj, context_.data_model_.method_isOrdinaryOrBitFieldObject_, context_.data_model_.arboretum_node_for(D->isOrdinaryOrBitFieldObject()));
  arboretum_create_edge(obj, context_.data_model_.method_refersToBitField_, context_.data_model_.arboretum_node_for(D->refersToBitField()));
  arboretum_create_edge(obj, context_.data_model_.method_getSourceBitField_, context_.resolve(D->getSourceBitField()));
  arboretum_create_edge(obj, context_.data_model_.method_getReferencedDeclOfCallee_, context_.resolve(D->getReferencedDeclOfCallee()));
  arboretum_create_edge(obj, context_.data_model_.method_getObjCProperty_, context_.resolve(D->getObjCProperty()));
  arboretum_create_edge(obj, context_.data_model_.method_isObjCSelfExpr_, context_.data_model_.arboretum_node_for(D->isObjCSelfExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_refersToVectorElement_, context_.data_model_.arboretum_node_for(D->refersToVectorElement()));
  arboretum_create_edge(obj, context_.data_model_.method_refersToMatrixElement_, context_.data_model_.arboretum_node_for(D->refersToMatrixElement()));
  arboretum_create_edge(obj, context_.data_model_.method_refersToGlobalRegisterVar_, context_.data_model_.arboretum_node_for(D->refersToGlobalRegisterVar()));
  arboretum_create_edge(obj, context_.data_model_.method_IgnoreImpCasts_, context_.resolve(D->IgnoreImpCasts()));
  arboretum_create_edge(obj, context_.data_model_.method_IgnoreCasts_, context_.resolve(D->IgnoreCasts()));
  arboretum_create_edge(obj, context_.data_model_.method_IgnoreImplicit_, context_.resolve(D->IgnoreImplicit()));
  arboretum_create_edge(obj, context_.data_model_.method_IgnoreImplicitAsWritten_, context_.resolve(D->IgnoreImplicitAsWritten()));
  arboretum_create_edge(obj, context_.data_model_.method_IgnoreParens_, context_.resolve(D->IgnoreParens()));
  arboretum_create_edge(obj, context_.data_model_.method_IgnoreParenImpCasts_, context_.resolve(D->IgnoreParenImpCasts()));
  arboretum_create_edge(obj, context_.data_model_.method_IgnoreParenCasts_, context_.resolve(D->IgnoreParenCasts()));
  arboretum_create_edge(obj, context_.data_model_.method_IgnoreConversionOperatorSingleStep_, context_.resolve(D->IgnoreConversionOperatorSingleStep()));
  arboretum_create_edge(obj, context_.data_model_.method_IgnoreParenLValueCasts_, context_.resolve(D->IgnoreParenLValueCasts()));
  arboretum_create_edge(obj, context_.data_model_.method_IgnoreParenBaseCasts_, context_.resolve(D->IgnoreParenBaseCasts()));
  arboretum_create_edge(obj, context_.data_model_.method_isDefaultArgument_, context_.data_model_.arboretum_node_for(D->isDefaultArgument()));
  arboretum_create_edge(obj, context_.data_model_.method_isImplicitCXXThis_, context_.data_model_.arboretum_node_for(D->isImplicitCXXThis()));
  arboretum_create_edge(obj, context_.data_model_.method_skipRValueSubobjectAdjustments_, context_.resolve(D->skipRValueSubobjectAdjustments()));
  return true;
}

bool ArboretumASTVisitor::VisitExprWithCleanups(clang::ExprWithCleanups* D) {
  const Id* obj = context_.resolve(D);
  // getObjects ( class llvm::ArrayRef<class llvm::PointerUnion<class clang::BlockDecl *, class clang::CompoundLiteralExpr *> > )
  arboretum_create_edge(obj, context_.data_model_.method_getNumObjects_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumObjects())));
  arboretum_create_edge(obj, context_.data_model_.method_cleanupsHaveSideEffects_, context_.data_model_.arboretum_node_for(D->cleanupsHaveSideEffects()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_71_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_70_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitExpressionTraitExpr(clang::ExpressionTraitExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_72_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_71_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getTrait_1_, context_.data_model_.resolve(D->getTrait()));
  arboretum_create_edge(obj, context_.data_model_.method_getQueriedExpression_, context_.resolve(D->getQueriedExpression()));
  arboretum_create_edge(obj, context_.data_model_.method_getValue_7_, context_.data_model_.arboretum_node_for(D->getValue()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitExtVectorElementExpr(clang::ExtVectorElementExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getBase_3_, context_.resolve(D->getBase()));
  // getAccessor ( class clang::IdentifierInfo & )
  arboretum_create_edge(obj, context_.data_model_.method_getAccessorLoc_, context_.source_model_.resolve(D->getAccessorLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getNumElements_1_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumElements())));
  arboretum_create_edge(obj, context_.data_model_.method_containsDuplicateElements_, context_.data_model_.arboretum_node_for(D->containsDuplicateElements()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_73_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_72_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_isArrow_2_, context_.data_model_.arboretum_node_for(D->isArrow()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitFixedPointLiteral(clang::FixedPointLiteral* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_74_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_73_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getLocation_9_, context_.source_model_.resolve(D->getLocation()));
  arboretum_create_edge(obj, context_.data_model_.method_getScale_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getScale())));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitFloatingLiteral(clang::FloatingLiteral* D) {
  const Id* obj = context_.resolve(D);
  // getValue ( class llvm::APFloat )
  arboretum_create_edge(obj, context_.data_model_.method_getRawSemantics_, context_.data_model_.resolve(D->getRawSemantics()));
  // getSemantics ( const struct llvm::fltSemantics & )
  arboretum_create_edge(obj, context_.data_model_.method_isExact_, context_.data_model_.arboretum_node_for(D->isExact()));
  arboretum_create_edge(obj, context_.data_model_.method_getValueAsApproximateDouble_, context_.data_model_.arboretum_node_for(D->getValueAsApproximateDouble()));
  arboretum_create_edge(obj, context_.data_model_.method_getLocation_10_, context_.source_model_.resolve(D->getLocation()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_75_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_74_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitForStmt(clang::ForStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getConditionVariable_, context_.resolve(D->getConditionVariable()));
  arboretum_create_edge(obj, context_.data_model_.method_getConditionVariableDeclStmt_, context_.resolve(D->getConditionVariableDeclStmt()));
  arboretum_create_edge(obj, context_.data_model_.method_getInit_4_, context_.resolve(D->getInit()));
  arboretum_create_edge(obj, context_.data_model_.method_getCond_6_, context_.resolve(D->getCond()));
  arboretum_create_edge(obj, context_.data_model_.method_getInc_1_, context_.resolve(D->getInc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBody_8_, context_.resolve(D->getBody()));
  arboretum_create_edge(obj, context_.data_model_.method_getForLoc_1_, context_.source_model_.resolve(D->getForLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getLParenLoc_5_, context_.source_model_.resolve(D->getLParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_15_, context_.source_model_.resolve(D->getRParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_76_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_75_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitFullExpr(clang::FullExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getSubExpr_5_, context_.resolve(D->getSubExpr()));
  return true;
}

bool ArboretumASTVisitor::VisitFunctionParmPackExpr(clang::FunctionParmPackExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getParameterPack_, context_.resolve(D->getParameterPack()));
  arboretum_create_edge(obj, context_.data_model_.method_getParameterPackLocation_, context_.source_model_.resolve(D->getParameterPackLocation()));
  arboretum_create_edge(obj, context_.data_model_.method_getNumExpansions_2_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumExpansions())));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_77_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_76_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitGCCAsmStmt(clang::GCCAsmStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_16_, context_.source_model_.resolve(D->getRParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getAsmString_1_, context_.resolve(D->getAsmString()));
  arboretum_create_edge(obj, context_.data_model_.method_isAsmGoto_, context_.data_model_.arboretum_node_for(D->isAsmGoto()));
  arboretum_create_edge(obj, context_.data_model_.method_getNumLabels_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumLabels())));
  // labels ( class llvm::iterator_range<struct clang::Stmt::CastIterator<class clang::AddrLabelExpr, const class clang::AddrLabelExpr *const, const class clang::Stmt *const> > )
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_78_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_77_, context_.source_model_.resolve(D->getEndLoc()));
  return true;
}

bool ArboretumASTVisitor::VisitGNUNullExpr(clang::GNUNullExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getTokenLocation_, context_.source_model_.resolve(D->getTokenLocation()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_79_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_78_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitGenericSelectionExpr(clang::GenericSelectionExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getNumAssocs_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumAssocs())));
  arboretum_create_edge(obj, context_.data_model_.method_getResultIndex_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getResultIndex())));
  arboretum_create_edge(obj, context_.data_model_.method_isResultDependent_, context_.data_model_.arboretum_node_for(D->isResultDependent()));
  arboretum_create_edge(obj, context_.data_model_.method_isExprPredicate_, context_.data_model_.arboretum_node_for(D->isExprPredicate()));
  arboretum_create_edge(obj, context_.data_model_.method_isTypePredicate_, context_.data_model_.arboretum_node_for(D->isTypePredicate()));
  arboretum_create_edge(obj, context_.data_model_.method_getControllingExpr_, context_.resolve(D->getControllingExpr()));
  // getControllingType ( const class clang::TypeSourceInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getResultExpr_, context_.resolve(D->getResultExpr()));
  {
    std::vector<Id*> element_ids;
    auto range = D->getAssocExprs();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_getAssocExprs_, context_.data_model_.arboretum_node_for(context_.data_model_.class_Expr_, element_ids));
  }
  // getAssocTypeSourceInfos ( class llvm::ArrayRef<class clang::TypeSourceInfo *> )
  // associations ( class llvm::iterator_range<class clang::GenericSelectionExpr::AssociationIteratorTy<true> > )
  arboretum_create_edge(obj, context_.data_model_.method_getGenericLoc_, context_.source_model_.resolve(D->getGenericLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getDefaultLoc_2_, context_.source_model_.resolve(D->getDefaultLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_17_, context_.source_model_.resolve(D->getRParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_80_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_79_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitGotoStmt(clang::GotoStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getLabel_1_, context_.resolve(D->getLabel()));
  arboretum_create_edge(obj, context_.data_model_.method_getGotoLoc_, context_.source_model_.resolve(D->getGotoLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getLabelLoc_1_, context_.source_model_.resolve(D->getLabelLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_81_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_80_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitIfStmt(clang::IfStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_hasInitStorage_, context_.data_model_.arboretum_node_for(D->hasInitStorage()));
  arboretum_create_edge(obj, context_.data_model_.method_hasVarStorage_, context_.data_model_.arboretum_node_for(D->hasVarStorage()));
  arboretum_create_edge(obj, context_.data_model_.method_hasElseStorage_, context_.data_model_.arboretum_node_for(D->hasElseStorage()));
  arboretum_create_edge(obj, context_.data_model_.method_getCond_7_, context_.resolve(D->getCond()));
  arboretum_create_edge(obj, context_.data_model_.method_getThen_, context_.resolve(D->getThen()));
  arboretum_create_edge(obj, context_.data_model_.method_getElse_, context_.resolve(D->getElse()));
  arboretum_create_edge(obj, context_.data_model_.method_getConditionVariable_1_, context_.resolve(D->getConditionVariable()));
  arboretum_create_edge(obj, context_.data_model_.method_getConditionVariableDeclStmt_1_, context_.resolve(D->getConditionVariableDeclStmt()));
  arboretum_create_edge(obj, context_.data_model_.method_getInit_5_, context_.resolve(D->getInit()));
  arboretum_create_edge(obj, context_.data_model_.method_getIfLoc_, context_.source_model_.resolve(D->getIfLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getElseLoc_, context_.source_model_.resolve(D->getElseLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_isConsteval_1_, context_.data_model_.arboretum_node_for(D->isConsteval()));
  arboretum_create_edge(obj, context_.data_model_.method_isNonNegatedConsteval_, context_.data_model_.arboretum_node_for(D->isNonNegatedConsteval()));
  arboretum_create_edge(obj, context_.data_model_.method_isNegatedConsteval_, context_.data_model_.arboretum_node_for(D->isNegatedConsteval()));
  arboretum_create_edge(obj, context_.data_model_.method_isConstexpr_2_, context_.data_model_.arboretum_node_for(D->isConstexpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getStatementKind_, context_.data_model_.resolve(D->getStatementKind()));
  arboretum_create_edge(obj, context_.data_model_.method_isObjCAvailabilityCheck_, context_.data_model_.arboretum_node_for(D->isObjCAvailabilityCheck()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_82_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_81_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getLParenLoc_6_, context_.source_model_.resolve(D->getLParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_18_, context_.source_model_.resolve(D->getRParenLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitImaginaryLiteral(clang::ImaginaryLiteral* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getSubExpr_6_, context_.resolve(D->getSubExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_83_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_82_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitImplicitCastExpr(clang::ImplicitCastExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_isPartOfExplicitCast_, context_.data_model_.arboretum_node_for(D->isPartOfExplicitCast()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_84_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_83_, context_.source_model_.resolve(D->getEndLoc()));
  return true;
}

bool ArboretumASTVisitor::VisitImplicitValueInitExpr(clang::ImplicitValueInitExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_85_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_84_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitIndirectGotoStmt(clang::IndirectGotoStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getGotoLoc_1_, context_.source_model_.resolve(D->getGotoLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getStarLoc_, context_.source_model_.resolve(D->getStarLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getTarget_, context_.resolve(D->getTarget()));
  arboretum_create_edge(obj, context_.data_model_.method_getConstantTarget_, context_.resolve(D->getConstantTarget()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_86_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_85_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitInitListExpr(clang::InitListExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getNumInits_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumInits())));
  {
    std::vector<Id*> element_ids;
    auto range = D->inits();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_inits_1_, context_.data_model_.arboretum_node_for(context_.data_model_.class_Expr_, element_ids));
  }
  arboretum_create_edge(obj, context_.data_model_.method_getArrayFiller_1_, context_.resolve(D->getArrayFiller()));
  arboretum_create_edge(obj, context_.data_model_.method_hasArrayFiller_, context_.data_model_.arboretum_node_for(D->hasArrayFiller()));
  arboretum_create_edge(obj, context_.data_model_.method_hasDesignatedInit_, context_.data_model_.arboretum_node_for(D->hasDesignatedInit()));
  arboretum_create_edge(obj, context_.data_model_.method_getInitializedFieldInUnion_1_, context_.resolve(D->getInitializedFieldInUnion()));
  arboretum_create_edge(obj, context_.data_model_.method_isExplicit_3_, context_.data_model_.arboretum_node_for(D->isExplicit()));
  arboretum_create_edge(obj, context_.data_model_.method_isStringLiteralInit_, context_.data_model_.arboretum_node_for(D->isStringLiteralInit()));
  arboretum_create_edge(obj, context_.data_model_.method_isTransparent_, context_.data_model_.arboretum_node_for(D->isTransparent()));
  arboretum_create_edge(obj, context_.data_model_.method_getLBraceLoc_1_, context_.source_model_.resolve(D->getLBraceLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRBraceLoc_4_, context_.source_model_.resolve(D->getRBraceLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_isSemanticForm_, context_.data_model_.arboretum_node_for(D->isSemanticForm()));
  arboretum_create_edge(obj, context_.data_model_.method_getSemanticForm_1_, context_.resolve(D->getSemanticForm()));
  arboretum_create_edge(obj, context_.data_model_.method_isSyntacticForm_, context_.data_model_.arboretum_node_for(D->isSyntacticForm()));
  arboretum_create_edge(obj, context_.data_model_.method_getSyntacticForm_, context_.resolve(D->getSyntacticForm()));
  arboretum_create_edge(obj, context_.data_model_.method_hadArrayRangeDesignator_, context_.data_model_.arboretum_node_for(D->hadArrayRangeDesignator()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_87_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_86_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitIntegerLiteral(clang::IntegerLiteral* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_88_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_87_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getLocation_11_, context_.source_model_.resolve(D->getLocation()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitLabelStmt(clang::LabelStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getIdentLoc_, context_.source_model_.resolve(D->getIdentLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getDecl_8_, context_.resolve(D->getDecl()));
  // getName ( const char * )
  arboretum_create_edge(obj, context_.data_model_.method_getSubStmt_3_, context_.resolve(D->getSubStmt()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_89_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_88_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  arboretum_create_edge(obj, context_.data_model_.method_isSideEntry_, context_.data_model_.arboretum_node_for(D->isSideEntry()));
  return true;
}

bool ArboretumASTVisitor::VisitLambdaExpr(clang::LambdaExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getCaptureDefault_, context_.data_model_.resolve(D->getCaptureDefault()));
  arboretum_create_edge(obj, context_.data_model_.method_getCaptureDefaultLoc_, context_.source_model_.resolve(D->getCaptureDefaultLoc()));
  // captures ( class llvm::iterator_range<const class clang::LambdaCapture *> )
  arboretum_create_edge(obj, context_.data_model_.method_capture_size_1_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->capture_size())));
  // explicit_captures ( class llvm::iterator_range<const class clang::LambdaCapture *> )
  // implicit_captures ( class llvm::iterator_range<const class clang::LambdaCapture *> )
  {
    std::vector<Id*> element_ids;
    auto range = D->capture_inits();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_capture_inits_1_, context_.data_model_.arboretum_node_for(context_.data_model_.class_Expr_, element_ids));
  }
  arboretum_create_edge(obj, context_.data_model_.method_getIntroducerRange_, context_.source_model_.resolve(D->getIntroducerRange()));
  arboretum_create_edge(obj, context_.data_model_.method_getLambdaClass_, context_.resolve(D->getLambdaClass()));
  arboretum_create_edge(obj, context_.data_model_.method_getCallOperator_, context_.resolve(D->getCallOperator()));
  arboretum_create_edge(obj, context_.data_model_.method_getDependentCallOperator_, context_.resolve(D->getDependentCallOperator()));
  // getTemplateParameterList ( class clang::TemplateParameterList * )
  {
    std::vector<Id*> element_ids;
    auto range = D->getExplicitTemplateParameters();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_getExplicitTemplateParameters_, context_.data_model_.arboretum_node_for(context_.data_model_.class_NamedDecl_, element_ids));
  }
  arboretum_create_edge(obj, context_.data_model_.method_getTrailingRequiresClause_1_, context_.resolve(D->getTrailingRequiresClause()));
  arboretum_create_edge(obj, context_.data_model_.method_isGenericLambda_1_, context_.data_model_.arboretum_node_for(D->isGenericLambda()));
  arboretum_create_edge(obj, context_.data_model_.method_getBody_9_, context_.resolve(D->getBody()));
  arboretum_create_edge(obj, context_.data_model_.method_getCompoundStmtBody_, context_.resolve(D->getCompoundStmtBody()));
  arboretum_create_edge(obj, context_.data_model_.method_isMutable_1_, context_.data_model_.arboretum_node_for(D->isMutable()));
  arboretum_create_edge(obj, context_.data_model_.method_hasExplicitParameters_, context_.data_model_.arboretum_node_for(D->hasExplicitParameters()));
  arboretum_create_edge(obj, context_.data_model_.method_hasExplicitResultType_, context_.data_model_.arboretum_node_for(D->hasExplicitResultType()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_90_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_89_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitMSAsmStmt(clang::MSAsmStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getLBraceLoc_2_, context_.source_model_.resolve(D->getLBraceLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_90_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_hasBraces_2_, context_.data_model_.arboretum_node_for(D->hasBraces()));
  arboretum_create_edge(obj, context_.data_model_.method_getAsmString_2_, context_.data_model_.arboretum_node_for(D->getAsmString().str()));
  {
    std::vector<Id*> element_ids;
    auto range = D->getAllConstraints();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.data_model_.arboretum_node_for((*itr).str()));
    }
    arboretum_create_edge(obj, context_.data_model_.method_getAllConstraints_, context_.data_model_.arboretum_node_for(context_.data_model_.builtin_string_class_, element_ids));
  }
  {
    std::vector<Id*> element_ids;
    auto range = D->getClobbers();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.data_model_.arboretum_node_for((*itr).str()));
    }
    arboretum_create_edge(obj, context_.data_model_.method_getClobbers_, context_.data_model_.arboretum_node_for(context_.data_model_.builtin_string_class_, element_ids));
  }
  {
    std::vector<Id*> element_ids;
    auto range = D->getAllExprs();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_getAllExprs_, context_.data_model_.arboretum_node_for(context_.data_model_.class_Expr_, element_ids));
  }
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_91_, context_.source_model_.resolve(D->getBeginLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitMSDependentExistsStmt(clang::MSDependentExistsStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getKeywordLoc_3_, context_.source_model_.resolve(D->getKeywordLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_isIfExists_, context_.data_model_.arboretum_node_for(D->isIfExists()));
  arboretum_create_edge(obj, context_.data_model_.method_isIfNotExists_, context_.data_model_.arboretum_node_for(D->isIfNotExists()));
  // getQualifierLoc ( class clang::NestedNameSpecifierLoc )
  // getNameInfo ( struct clang::DeclarationNameInfo )
  arboretum_create_edge(obj, context_.data_model_.method_getSubStmt_4_, context_.resolve(D->getSubStmt()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_92_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_91_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitMSPropertyRefExpr(clang::MSPropertyRefExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getSourceRange_48_, context_.source_model_.resolve(D->getSourceRange()));
  arboretum_create_edge(obj, context_.data_model_.method_isImplicitAccess_1_, context_.data_model_.arboretum_node_for(D->isImplicitAccess()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_93_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_92_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  arboretum_create_edge(obj, context_.data_model_.method_getBaseExpr_, context_.resolve(D->getBaseExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getPropertyDecl_, context_.resolve(D->getPropertyDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_isArrow_3_, context_.data_model_.arboretum_node_for(D->isArrow()));
  arboretum_create_edge(obj, context_.data_model_.method_getMemberLoc_1_, context_.source_model_.resolve(D->getMemberLoc()));
  // getQualifierLoc ( class clang::NestedNameSpecifierLoc )
  return true;
}

bool ArboretumASTVisitor::VisitMSPropertySubscriptExpr(clang::MSPropertySubscriptExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getBase_4_, context_.resolve(D->getBase()));
  arboretum_create_edge(obj, context_.data_model_.method_getIdx_1_, context_.resolve(D->getIdx()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_94_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_93_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRBracketLoc_3_, context_.source_model_.resolve(D->getRBracketLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getExprLoc_7_, context_.source_model_.resolve(D->getExprLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitMaterializeTemporaryExpr(clang::MaterializeTemporaryExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getSubExpr_7_, context_.resolve(D->getSubExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getStorageDuration_2_, context_.data_model_.resolve(D->getStorageDuration()));
  arboretum_create_edge(obj, context_.data_model_.method_getLifetimeExtendedTemporaryDecl_, context_.resolve(D->getLifetimeExtendedTemporaryDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getExtendingDecl_1_, context_.resolve(D->getExtendingDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getManglingNumber_1_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getManglingNumber())));
  arboretum_create_edge(obj, context_.data_model_.method_isBoundToLvalueReference_, context_.data_model_.arboretum_node_for(D->isBoundToLvalueReference()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_95_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_94_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitMatrixSubscriptExpr(clang::MatrixSubscriptExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_isIncomplete_, context_.data_model_.arboretum_node_for(D->isIncomplete()));
  arboretum_create_edge(obj, context_.data_model_.method_getBase_5_, context_.resolve(D->getBase()));
  arboretum_create_edge(obj, context_.data_model_.method_getRowIdx_, context_.resolve(D->getRowIdx()));
  arboretum_create_edge(obj, context_.data_model_.method_getColumnIdx_, context_.resolve(D->getColumnIdx()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_96_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_95_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getExprLoc_8_, context_.source_model_.resolve(D->getExprLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRBracketLoc_4_, context_.source_model_.resolve(D->getRBracketLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitMemberExpr(clang::MemberExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getBase_6_, context_.resolve(D->getBase()));
  arboretum_create_edge(obj, context_.data_model_.method_getMemberDecl_, context_.resolve(D->getMemberDecl()));
  // getFoundDecl ( class clang::DeclAccessPair )
  arboretum_create_edge(obj, context_.data_model_.method_hasQualifier_2_, context_.data_model_.arboretum_node_for(D->hasQualifier()));
  // getQualifierLoc ( class clang::NestedNameSpecifierLoc )
  // getQualifier ( class clang::NestedNameSpecifier * )
  arboretum_create_edge(obj, context_.data_model_.method_getTemplateKeywordLoc_5_, context_.source_model_.resolve(D->getTemplateKeywordLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getLAngleLoc_3_, context_.source_model_.resolve(D->getLAngleLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRAngleLoc_3_, context_.source_model_.resolve(D->getRAngleLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_hasTemplateKeyword_3_, context_.data_model_.arboretum_node_for(D->hasTemplateKeyword()));
  arboretum_create_edge(obj, context_.data_model_.method_hasExplicitTemplateArgs_4_, context_.data_model_.arboretum_node_for(D->hasExplicitTemplateArgs()));
  // getTemplateArgs ( const class clang::TemplateArgumentLoc * )
  arboretum_create_edge(obj, context_.data_model_.method_getNumTemplateArgs_3_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumTemplateArgs())));
  // template_arguments ( class llvm::ArrayRef<class clang::TemplateArgumentLoc> )
  // getMemberNameInfo ( struct clang::DeclarationNameInfo )
  arboretum_create_edge(obj, context_.data_model_.method_getOperatorLoc_6_, context_.source_model_.resolve(D->getOperatorLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_isArrow_4_, context_.data_model_.arboretum_node_for(D->isArrow()));
  arboretum_create_edge(obj, context_.data_model_.method_getMemberLoc_2_, context_.source_model_.resolve(D->getMemberLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_97_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_96_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getExprLoc_9_, context_.source_model_.resolve(D->getExprLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_isImplicitAccess_2_, context_.data_model_.arboretum_node_for(D->isImplicitAccess()));
  arboretum_create_edge(obj, context_.data_model_.method_hadMultipleCandidates_2_, context_.data_model_.arboretum_node_for(D->hadMultipleCandidates()));
  arboretum_create_edge(obj, context_.data_model_.method_isNonOdrUse_1_, context_.data_model_.resolve(D->isNonOdrUse()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitNoInitExpr(clang::NoInitExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_98_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_97_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitNullStmt(clang::NullStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getSemiLoc_, context_.source_model_.resolve(D->getSemiLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_hasLeadingEmptyMacro_, context_.data_model_.arboretum_node_for(D->hasLeadingEmptyMacro()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_99_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_98_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
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
  arboretum_create_edge(obj, context_.data_model_.method_getOperatorLoc_7_, context_.source_model_.resolve(D->getOperatorLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_19_, context_.source_model_.resolve(D->getRParenLoc()));
  // getTypeSourceInfo ( class clang::TypeSourceInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getNumComponents_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumComponents())));
  arboretum_create_edge(obj, context_.data_model_.method_getNumExpressions_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumExpressions())));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_100_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_99_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitOpaqueValueExpr(clang::OpaqueValueExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getLocation_12_, context_.source_model_.resolve(D->getLocation()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_101_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_100_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getExprLoc_10_, context_.source_model_.resolve(D->getExprLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  arboretum_create_edge(obj, context_.data_model_.method_getSourceExpr_, context_.resolve(D->getSourceExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_isUnique_, context_.data_model_.arboretum_node_for(D->isUnique()));
  return true;
}

bool ArboretumASTVisitor::VisitOverloadExpr(clang::OverloadExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getNamingClass_, context_.resolve(D->getNamingClass()));
  // decls ( class llvm::iterator_range<class clang::UnresolvedSetIterator> )
  arboretum_create_edge(obj, context_.data_model_.method_getNumDecls_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumDecls())));
  // getNameInfo ( const struct clang::DeclarationNameInfo & )
  // getName ( class clang::DeclarationName )
  arboretum_create_edge(obj, context_.data_model_.method_getNameLoc_, context_.source_model_.resolve(D->getNameLoc()));
  // getQualifier ( class clang::NestedNameSpecifier * )
  // getQualifierLoc ( class clang::NestedNameSpecifierLoc )
  arboretum_create_edge(obj, context_.data_model_.method_getTemplateKeywordLoc_6_, context_.source_model_.resolve(D->getTemplateKeywordLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getLAngleLoc_4_, context_.source_model_.resolve(D->getLAngleLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRAngleLoc_4_, context_.source_model_.resolve(D->getRAngleLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_hasTemplateKeyword_4_, context_.data_model_.arboretum_node_for(D->hasTemplateKeyword()));
  arboretum_create_edge(obj, context_.data_model_.method_hasExplicitTemplateArgs_5_, context_.data_model_.arboretum_node_for(D->hasExplicitTemplateArgs()));
  // getTemplateArgs ( const class clang::TemplateArgumentLoc * )
  arboretum_create_edge(obj, context_.data_model_.method_getNumTemplateArgs_4_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumTemplateArgs())));
  // template_arguments ( class llvm::ArrayRef<class clang::TemplateArgumentLoc> )
  return true;
}

bool ArboretumASTVisitor::VisitPackExpansionExpr(clang::PackExpansionExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getPattern_2_, context_.resolve(D->getPattern()));
  arboretum_create_edge(obj, context_.data_model_.method_getEllipsisLoc_6_, context_.source_model_.resolve(D->getEllipsisLoc()));
  // getNumExpansions ( class std::optional<unsigned int> )
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_102_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_101_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitParenExpr(clang::ParenExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getSubExpr_8_, context_.resolve(D->getSubExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_103_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_102_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getLParen_, context_.source_model_.resolve(D->getLParen()));
  arboretum_create_edge(obj, context_.data_model_.method_getRParen_, context_.source_model_.resolve(D->getRParen()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitParenListExpr(clang::ParenListExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getNumExprs_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumExprs())));
  arboretum_create_edge(obj, context_.data_model_.method_getLParenLoc_7_, context_.source_model_.resolve(D->getLParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_20_, context_.source_model_.resolve(D->getRParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_104_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_103_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitPredefinedExpr(clang::PredefinedExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getIdentKind_, context_.data_model_.resolve(D->getIdentKind()));
  arboretum_create_edge(obj, context_.data_model_.method_isTransparent_1_, context_.data_model_.arboretum_node_for(D->isTransparent()));
  arboretum_create_edge(obj, context_.data_model_.method_getLocation_13_, context_.source_model_.resolve(D->getLocation()));
  arboretum_create_edge(obj, context_.data_model_.method_getFunctionName_, context_.resolve(D->getFunctionName()));
  arboretum_create_edge(obj, context_.data_model_.method_getIdentKindName_, context_.data_model_.arboretum_node_for(D->getIdentKindName().str()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_105_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_104_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitPseudoObjectExpr(clang::PseudoObjectExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getSyntacticForm_1_, context_.resolve(D->getSyntacticForm()));
  arboretum_create_edge(obj, context_.data_model_.method_getResultExprIndex_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getResultExprIndex())));
  arboretum_create_edge(obj, context_.data_model_.method_getResultExpr_1_, context_.resolve(D->getResultExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getNumSemanticExprs_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumSemanticExprs())));
  {
    std::vector<Id*> element_ids;
    auto range = D->semantics();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_semantics_, context_.data_model_.arboretum_node_for(context_.data_model_.class_Expr_, element_ids));
  }
  arboretum_create_edge(obj, context_.data_model_.method_getExprLoc_11_, context_.source_model_.resolve(D->getExprLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_106_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_105_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitRecoveryExpr(clang::RecoveryExpr* D) {
  const Id* obj = context_.resolve(D);
  {
    std::vector<Id*> element_ids;
    auto range = D->subExpressions();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_subExpressions_, context_.data_model_.arboretum_node_for(context_.data_model_.class_Expr_, element_ids));
  }
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_107_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_106_, context_.source_model_.resolve(D->getEndLoc()));
  return true;
}

bool ArboretumASTVisitor::VisitRequiresExpr(clang::RequiresExpr* D) {
  const Id* obj = context_.resolve(D);
  {
    std::vector<Id*> element_ids;
    auto range = D->getLocalParameters();
    for (auto itr=range.begin(); itr != range.end(); ++itr) {
      element_ids.push_back(context_.resolve((*itr)));
    }
    arboretum_create_edge(obj, context_.data_model_.method_getLocalParameters_, context_.data_model_.arboretum_node_for(context_.data_model_.class_ParmVarDecl_, element_ids));
  }
  arboretum_create_edge(obj, context_.data_model_.method_getBody_10_, context_.resolve(D->getBody()));
  // getRequirements ( class llvm::ArrayRef<class clang::concepts::Requirement *> )
  arboretum_create_edge(obj, context_.data_model_.method_getRequiresKWLoc_, context_.source_model_.resolve(D->getRequiresKWLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getLParenLoc_8_, context_.source_model_.resolve(D->getLParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_21_, context_.source_model_.resolve(D->getRParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRBraceLoc_5_, context_.source_model_.resolve(D->getRBraceLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_108_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_107_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitReturnStmt(clang::ReturnStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getRetValue_, context_.resolve(D->getRetValue()));
  arboretum_create_edge(obj, context_.data_model_.method_getNRVOCandidate_, context_.resolve(D->getNRVOCandidate()));
  arboretum_create_edge(obj, context_.data_model_.method_getReturnLoc_, context_.source_model_.resolve(D->getReturnLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_109_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_108_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitSEHExceptStmt(clang::SEHExceptStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_110_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getExceptLoc_, context_.source_model_.resolve(D->getExceptLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_109_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getFilterExpr_, context_.resolve(D->getFilterExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getBlock_, context_.resolve(D->getBlock()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitSEHFinallyStmt(clang::SEHFinallyStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_111_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getFinallyLoc_, context_.source_model_.resolve(D->getFinallyLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_110_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBlock_1_, context_.resolve(D->getBlock()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitSEHLeaveStmt(clang::SEHLeaveStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getLeaveLoc_, context_.source_model_.resolve(D->getLeaveLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_112_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_111_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitSEHTryStmt(clang::SEHTryStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_113_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getTryLoc_1_, context_.source_model_.resolve(D->getTryLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_112_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getIsCXXTry_, context_.data_model_.arboretum_node_for(D->getIsCXXTry()));
  arboretum_create_edge(obj, context_.data_model_.method_getTryBlock_1_, context_.resolve(D->getTryBlock()));
  arboretum_create_edge(obj, context_.data_model_.method_getHandler_, context_.resolve(D->getHandler()));
  arboretum_create_edge(obj, context_.data_model_.method_getExceptHandler_, context_.resolve(D->getExceptHandler()));
  arboretum_create_edge(obj, context_.data_model_.method_getFinallyHandler_, context_.resolve(D->getFinallyHandler()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitSYCLUniqueStableNameExpr(clang::SYCLUniqueStableNameExpr* D) {
  const Id* obj = context_.resolve(D);
  // getTypeSourceInfo ( const class clang::TypeSourceInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_114_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_113_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getLocation_14_, context_.source_model_.resolve(D->getLocation()));
  arboretum_create_edge(obj, context_.data_model_.method_getLParenLocation_, context_.source_model_.resolve(D->getLParenLocation()));
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLocation_, context_.source_model_.resolve(D->getRParenLocation()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitShuffleVectorExpr(clang::ShuffleVectorExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getBuiltinLoc_4_, context_.source_model_.resolve(D->getBuiltinLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_22_, context_.source_model_.resolve(D->getRParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_115_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_114_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getNumSubExprs_2_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumSubExprs())));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitSizeOfPackExpr(clang::SizeOfPackExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getOperatorLoc_8_, context_.source_model_.resolve(D->getOperatorLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getPackLoc_, context_.source_model_.resolve(D->getPackLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_23_, context_.source_model_.resolve(D->getRParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getPack_, context_.resolve(D->getPack()));
  arboretum_create_edge(obj, context_.data_model_.method_isPartiallySubstituted_, context_.data_model_.arboretum_node_for(D->isPartiallySubstituted()));
  // getPartialArguments ( class llvm::ArrayRef<class clang::TemplateArgument> )
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_116_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_115_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitSourceLocExpr(clang::SourceLocExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getBuiltinStr_, context_.data_model_.arboretum_node_for(D->getBuiltinStr().str()));
  arboretum_create_edge(obj, context_.data_model_.method_getIdentKind_1_, context_.data_model_.resolve(D->getIdentKind()));
  arboretum_create_edge(obj, context_.data_model_.method_isIntType_, context_.data_model_.arboretum_node_for(D->isIntType()));
  // getParentContext ( const class clang::DeclContext * )
  arboretum_create_edge(obj, context_.data_model_.method_getLocation_15_, context_.source_model_.resolve(D->getLocation()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_117_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_116_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitStmt(clang::Stmt* D) {
  const Id* obj = context_.resolve(D);
  switch(D->getStmtClass()) {
    case clang::Stmt::ObjCArrayLiteralClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCArrayLiteral_);
      break; 
    case clang::Stmt::CXXStdInitializerListExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXStdInitializerListExpr_);
      break; 
    case clang::Stmt::ImplicitCastExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ImplicitCastExpr_);
      break; 
    case clang::Stmt::ObjCSelectorExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCSelectorExpr_);
      break; 
    case clang::Stmt::ObjCAtTryStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCAtTryStmt_);
      break; 
    case clang::Stmt::ObjCProtocolExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCProtocolExpr_);
      break; 
    case clang::Stmt::CXXTypeidExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXTypeidExpr_);
      break; 
    case clang::Stmt::ObjCPropertyRefExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCPropertyRefExpr_);
      break; 
    case clang::Stmt::OMPTargetTeamsDistributeSimdDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetTeamsDistributeSimdDirective_);
      break; 
    case clang::Stmt::ObjCMessageExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCMessageExpr_);
      break; 
    case clang::Stmt::CXXParenListInitExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXParenListInitExpr_);
      break; 
    case clang::Stmt::ObjCIsaExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCIsaExpr_);
      break; 
    case clang::Stmt::DoStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DoStmt_);
      break; 
    case clang::Stmt::GotoStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_GotoStmt_);
      break; 
    case clang::Stmt::FloatingLiteralClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_FloatingLiteral_);
      break; 
    case clang::Stmt::ContinueStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ContinueStmt_);
      break; 
    case clang::Stmt::OMPSectionsDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPSectionsDirective_);
      break; 
    case clang::Stmt::BreakStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_BreakStmt_);
      break; 
    case clang::Stmt::TypoExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_TypoExpr_);
      break; 
    case clang::Stmt::SwitchStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_SwitchStmt_);
      break; 
    case clang::Stmt::AsTypeExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_AsTypeExpr_);
      break; 
    case clang::Stmt::VAArgExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_VAArgExpr_);
      break; 
    case clang::Stmt::MSAsmStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_MSAsmStmt_);
      break; 
    case clang::Stmt::OMPDistributeParallelForDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPDistributeParallelForDirective_);
      break; 
    case clang::Stmt::SEHTryStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_SEHTryStmt_);
      break; 
    case clang::Stmt::StringLiteralClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_StringLiteral_);
      break; 
    case clang::Stmt::IfStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_IfStmt_);
      break; 
    case clang::Stmt::CapturedStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CapturedStmt_);
      break; 
    case clang::Stmt::PackExpansionExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_PackExpansionExpr_);
      break; 
    case clang::Stmt::CXXThisExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXThisExpr_);
      break; 
    case clang::Stmt::CXXForRangeStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXForRangeStmt_);
      break; 
    case clang::Stmt::ObjCEncodeExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCEncodeExpr_);
      break; 
    case clang::Stmt::SubstNonTypeTemplateParmExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_SubstNonTypeTemplateParmExpr_);
      break; 
    case clang::Stmt::ObjCIndirectCopyRestoreExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCIndirectCopyRestoreExpr_);
      break; 
    case clang::Stmt::CoroutineBodyStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CoroutineBodyStmt_);
      break; 
    case clang::Stmt::CoreturnStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CoreturnStmt_);
      break; 
    case clang::Stmt::ArrayTypeTraitExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ArrayTypeTraitExpr_);
      break; 
    case clang::Stmt::CXXUuidofExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXUuidofExpr_);
      break; 
    case clang::Stmt::ObjCForCollectionStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCForCollectionStmt_);
      break; 
    case clang::Stmt::ObjCAtCatchStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCAtCatchStmt_);
      break; 
    case clang::Stmt::ObjCAtSynchronizedStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCAtSynchronizedStmt_);
      break; 
    case clang::Stmt::LabelStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_LabelStmt_);
      break; 
    case clang::Stmt::ObjCAtThrowStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCAtThrowStmt_);
      break; 
    case clang::Stmt::OMPParallelMasterDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPParallelMasterDirective_);
      break; 
    case clang::Stmt::ImaginaryLiteralClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ImaginaryLiteral_);
      break; 
    case clang::Stmt::ObjCAutoreleasePoolStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCAutoreleasePoolStmt_);
      break; 
    case clang::Stmt::OMPCanonicalLoopClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPCanonicalLoop_);
      break; 
    case clang::Stmt::OMPParallelDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPParallelDirective_);
      break; 
    case clang::Stmt::OMPTileDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTileDirective_);
      break; 
    case clang::Stmt::MSPropertyRefExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_MSPropertyRefExpr_);
      break; 
    case clang::Stmt::ObjCDictionaryLiteralClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCDictionaryLiteral_);
      break; 
    case clang::Stmt::OMPInteropDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPInteropDirective_);
      break; 
    case clang::Stmt::ObjCSubscriptRefExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCSubscriptRefExpr_);
      break; 
    case clang::Stmt::CXXBoolLiteralExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXBoolLiteralExpr_);
      break; 
    case clang::Stmt::SYCLUniqueStableNameExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_SYCLUniqueStableNameExpr_);
      break; 
    case clang::Stmt::OMPSimdDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPSimdDirective_);
      break; 
    case clang::Stmt::OMPForSimdDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPForSimdDirective_);
      break; 
    case clang::Stmt::ObjCBoxedExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCBoxedExpr_);
      break; 
    case clang::Stmt::OMPParallelSectionsDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPParallelSectionsDirective_);
      break; 
    case clang::Stmt::ObjCAvailabilityCheckExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCAvailabilityCheckExpr_);
      break; 
    case clang::Stmt::OMPParallelForDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPParallelForDirective_);
      break; 
    case clang::Stmt::OMPTargetParallelForDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetParallelForDirective_);
      break; 
    case clang::Stmt::OMPErrorDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPErrorDirective_);
      break; 
    case clang::Stmt::OMPTaskLoopSimdDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTaskLoopSimdDirective_);
      break; 
    case clang::Stmt::OMPMaskedTaskLoopDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPMaskedTaskLoopDirective_);
      break; 
    case clang::Stmt::OMPMasterTaskLoopSimdDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPMasterTaskLoopSimdDirective_);
      break; 
    case clang::Stmt::WhileStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_WhileStmt_);
      break; 
    case clang::Stmt::CoawaitExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CoawaitExpr_);
      break; 
    case clang::Stmt::CXXTryStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXTryStmt_);
      break; 
    case clang::Stmt::OMPMaskedTaskLoopSimdDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPMaskedTaskLoopSimdDirective_);
      break; 
    case clang::Stmt::CompoundAssignOperatorClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CompoundAssignOperator_);
      break; 
    case clang::Stmt::ObjCAtFinallyStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCAtFinallyStmt_);
      break; 
    case clang::Stmt::OMPParallelMasterTaskLoopDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPParallelMasterTaskLoopDirective_);
      break; 
    case clang::Stmt::BlockExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_BlockExpr_);
      break; 
    case clang::Stmt::OMPParallelMaskedTaskLoopDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPParallelMaskedTaskLoopDirective_);
      break; 
    case clang::Stmt::OMPMasterDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPMasterDirective_);
      break; 
    case clang::Stmt::CXXNoexceptExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXNoexceptExpr_);
      break; 
    case clang::Stmt::OMPSingleDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPSingleDirective_);
      break; 
    case clang::Stmt::ObjCIvarRefExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCIvarRefExpr_);
      break; 
    case clang::Stmt::CUDAKernelCallExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CUDAKernelCallExpr_);
      break; 
    case clang::Stmt::OMPTaskyieldDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTaskyieldDirective_);
      break; 
    case clang::Stmt::OMPIteratorExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPIteratorExpr_);
      break; 
    case clang::Stmt::MSDependentExistsStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_MSDependentExistsStmt_);
      break; 
    case clang::Stmt::OMPTargetUpdateDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetUpdateDirective_);
      break; 
    case clang::Stmt::OMPCancelDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPCancelDirective_);
      break; 
    case clang::Stmt::RecoveryExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_RecoveryExpr_);
      break; 
    case clang::Stmt::OMPTaskwaitDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTaskwaitDirective_);
      break; 
    case clang::Stmt::OMPTargetParallelDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetParallelDirective_);
      break; 
    case clang::Stmt::SEHLeaveStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_SEHLeaveStmt_);
      break; 
    case clang::Stmt::OMPMaskedDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPMaskedDirective_);
      break; 
    case clang::Stmt::UserDefinedLiteralClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UserDefinedLiteral_);
      break; 
    case clang::Stmt::OMPCriticalDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPCriticalDirective_);
      break; 
    case clang::Stmt::SEHExceptStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_SEHExceptStmt_);
      break; 
    case clang::Stmt::OMPForDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPForDirective_);
      break; 
    case clang::Stmt::OMPTargetSimdDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetSimdDirective_);
      break; 
    case clang::Stmt::OMPDepobjDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPDepobjDirective_);
      break; 
    case clang::Stmt::ConstantExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ConstantExpr_);
      break; 
    case clang::Stmt::SEHFinallyStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_SEHFinallyStmt_);
      break; 
    case clang::Stmt::OMPParallelForSimdDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPParallelForSimdDirective_);
      break; 
    case clang::Stmt::OMPParallelMaskedDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPParallelMaskedDirective_);
      break; 
    case clang::Stmt::InitListExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_InitListExpr_);
      break; 
    case clang::Stmt::OMPTargetTeamsGenericLoopDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetTeamsGenericLoopDirective_);
      break; 
    case clang::Stmt::CXXUnresolvedConstructExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXUnresolvedConstructExpr_);
      break; 
    case clang::Stmt::OMPTargetDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetDirective_);
      break; 
    case clang::Stmt::OMPTargetDataDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetDataDirective_);
      break; 
    case clang::Stmt::CaseStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CaseStmt_);
      break; 
    case clang::Stmt::ExpressionTraitExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ExpressionTraitExpr_);
      break; 
    case clang::Stmt::CXXNullPtrLiteralExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXNullPtrLiteralExpr_);
      break; 
    case clang::Stmt::GenericSelectionExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_GenericSelectionExpr_);
      break; 
    case clang::Stmt::OMPTeamsDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTeamsDirective_);
      break; 
    case clang::Stmt::CXXPseudoDestructorExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXPseudoDestructorExpr_);
      break; 
    case clang::Stmt::CXXDeleteExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXDeleteExpr_);
      break; 
    case clang::Stmt::CXXScalarValueInitExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXScalarValueInitExpr_);
      break; 
    case clang::Stmt::AtomicExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_AtomicExpr_);
      break; 
    case clang::Stmt::CoyieldExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CoyieldExpr_);
      break; 
    case clang::Stmt::CXXBindTemporaryExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXBindTemporaryExpr_);
      break; 
    case clang::Stmt::CXXOperatorCallExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXOperatorCallExpr_);
      break; 
    case clang::Stmt::ArraySubscriptExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ArraySubscriptExpr_);
      break; 
    case clang::Stmt::CXXThrowExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXThrowExpr_);
      break; 
    case clang::Stmt::PseudoObjectExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_PseudoObjectExpr_);
      break; 
    case clang::Stmt::ParenExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ParenExpr_);
      break; 
    case clang::Stmt::ExtVectorElementExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ExtVectorElementExpr_);
      break; 
    case clang::Stmt::OMPUnrollDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPUnrollDirective_);
      break; 
    case clang::Stmt::FunctionParmPackExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_FunctionParmPackExpr_);
      break; 
    case clang::Stmt::TypeTraitExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_TypeTraitExpr_);
      break; 
    case clang::Stmt::ArrayInitIndexExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ArrayInitIndexExpr_);
      break; 
    case clang::Stmt::UnaryOperatorClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UnaryOperator_);
      break; 
    case clang::Stmt::OMPTargetTeamsDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetTeamsDirective_);
      break; 
    case clang::Stmt::CXXMemberCallExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXMemberCallExpr_);
      break; 
    case clang::Stmt::CXXInheritedCtorInitExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXInheritedCtorInitExpr_);
      break; 
    case clang::Stmt::OMPTeamsDistributeParallelForDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTeamsDistributeParallelForDirective_);
      break; 
    case clang::Stmt::IndirectGotoStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_IndirectGotoStmt_);
      break; 
    case clang::Stmt::CompoundStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CompoundStmt_);
      break; 
    case clang::Stmt::OffsetOfExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OffsetOfExpr_);
      break; 
    case clang::Stmt::OMPMasterTaskLoopDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPMasterTaskLoopDirective_);
      break; 
    case clang::Stmt::OMPParallelMasterTaskLoopSimdDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPParallelMasterTaskLoopSimdDirective_);
      break; 
    case clang::Stmt::UnaryExprOrTypeTraitExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UnaryExprOrTypeTraitExpr_);
      break; 
    case clang::Stmt::CXXFunctionalCastExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXFunctionalCastExpr_);
      break; 
    case clang::Stmt::CXXDefaultArgExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXDefaultArgExpr_);
      break; 
    case clang::Stmt::PredefinedExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_PredefinedExpr_);
      break; 
    case clang::Stmt::IntegerLiteralClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_IntegerLiteral_);
      break; 
    case clang::Stmt::CStyleCastExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CStyleCastExpr_);
      break; 
    case clang::Stmt::BinaryOperatorClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_BinaryOperator_);
      break; 
    case clang::Stmt::SourceLocExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_SourceLocExpr_);
      break; 
    case clang::Stmt::CXXNewExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXNewExpr_);
      break; 
    case clang::Stmt::ReturnStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ReturnStmt_);
      break; 
    case clang::Stmt::DesignatedInitExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DesignatedInitExpr_);
      break; 
    case clang::Stmt::MSPropertySubscriptExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_MSPropertySubscriptExpr_);
      break; 
    case clang::Stmt::OpaqueValueExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OpaqueValueExpr_);
      break; 
    case clang::Stmt::OMPTargetParallelGenericLoopDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetParallelGenericLoopDirective_);
      break; 
    case clang::Stmt::UnresolvedMemberExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UnresolvedMemberExpr_);
      break; 
    case clang::Stmt::CompoundLiteralExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CompoundLiteralExpr_);
      break; 
    case clang::Stmt::CXXConstructExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXConstructExpr_);
      break; 
    case clang::Stmt::ExprWithCleanupsClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ExprWithCleanups_);
      break; 
    case clang::Stmt::ObjCBridgedCastExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCBridgedCastExpr_);
      break; 
    case clang::Stmt::NullStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_NullStmt_);
      break; 
    case clang::Stmt::DeclStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DeclStmt_);
      break; 
    case clang::Stmt::ArrayInitLoopExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ArrayInitLoopExpr_);
      break; 
    case clang::Stmt::CXXReinterpretCastExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXReinterpretCastExpr_);
      break; 
    case clang::Stmt::CharacterLiteralClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CharacterLiteral_);
      break; 
    case clang::Stmt::OMPParallelMaskedTaskLoopSimdDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPParallelMaskedTaskLoopSimdDirective_);
      break; 
    case clang::Stmt::CXXStaticCastExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXStaticCastExpr_);
      break; 
    case clang::Stmt::MemberExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_MemberExpr_);
      break; 
    case clang::Stmt::CXXDynamicCastExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXDynamicCastExpr_);
      break; 
    case clang::Stmt::CXXConstCastExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXConstCastExpr_);
      break; 
    case clang::Stmt::BuiltinBitCastExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_BuiltinBitCastExpr_);
      break; 
    case clang::Stmt::ParenListExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ParenListExpr_);
      break; 
    case clang::Stmt::StmtExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_StmtExpr_);
      break; 
    case clang::Stmt::CXXDefaultInitExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXDefaultInitExpr_);
      break; 
    case clang::Stmt::MatrixSubscriptExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_MatrixSubscriptExpr_);
      break; 
    case clang::Stmt::CXXRewrittenBinaryOperatorClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXRewrittenBinaryOperator_);
      break; 
    case clang::Stmt::OMPBarrierDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPBarrierDirective_);
      break; 
    case clang::Stmt::DefaultStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DefaultStmt_);
      break; 
    case clang::Stmt::OMPScanDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPScanDirective_);
      break; 
    case clang::Stmt::ConvertVectorExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ConvertVectorExpr_);
      break; 
    case clang::Stmt::ImplicitValueInitExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ImplicitValueInitExpr_);
      break; 
    case clang::Stmt::NoInitExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_NoInitExpr_);
      break; 
    case clang::Stmt::CallExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CallExpr_);
      break; 
    case clang::Stmt::OMPScopeDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPScopeDirective_);
      break; 
    case clang::Stmt::GCCAsmStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_GCCAsmStmt_);
      break; 
    case clang::Stmt::ConditionalOperatorClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ConditionalOperator_);
      break; 
    case clang::Stmt::DesignatedInitUpdateExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DesignatedInitUpdateExpr_);
      break; 
    case clang::Stmt::DependentScopeDeclRefExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DependentScopeDeclRefExpr_);
      break; 
    case clang::Stmt::CXXDependentScopeMemberExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXDependentScopeMemberExpr_);
      break; 
    case clang::Stmt::MaterializeTemporaryExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_MaterializeTemporaryExpr_);
      break; 
    case clang::Stmt::CXXFoldExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXFoldExpr_);
      break; 
    case clang::Stmt::LambdaExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_LambdaExpr_);
      break; 
    case clang::Stmt::DependentCoawaitExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DependentCoawaitExpr_);
      break; 
    case clang::Stmt::SizeOfPackExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_SizeOfPackExpr_);
      break; 
    case clang::Stmt::RequiresExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_RequiresExpr_);
      break; 
    case clang::Stmt::ObjCStringLiteralClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCStringLiteral_);
      break; 
    case clang::Stmt::ObjCBoolLiteralExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ObjCBoolLiteralExpr_);
      break; 
    case clang::Stmt::GNUNullExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_GNUNullExpr_);
      break; 
    case clang::Stmt::OMPTargetParallelForSimdDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetParallelForSimdDirective_);
      break; 
    case clang::Stmt::AddrLabelExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_AddrLabelExpr_);
      break; 
    case clang::Stmt::ConceptSpecializationExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ConceptSpecializationExpr_);
      break; 
    case clang::Stmt::OMPMetaDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPMetaDirective_);
      break; 
    case clang::Stmt::OMPAtomicDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPAtomicDirective_);
      break; 
    case clang::Stmt::OMPTaskDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTaskDirective_);
      break; 
    case clang::Stmt::ShuffleVectorExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ShuffleVectorExpr_);
      break; 
    case clang::Stmt::BinaryConditionalOperatorClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_BinaryConditionalOperator_);
      break; 
    case clang::Stmt::OMPTeamsDistributeSimdDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTeamsDistributeSimdDirective_);
      break; 
    case clang::Stmt::OMPArrayShapingExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPArrayShapingExpr_);
      break; 
    case clang::Stmt::UnresolvedLookupExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_UnresolvedLookupExpr_);
      break; 
    case clang::Stmt::OMPTeamsDistributeParallelForSimdDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTeamsDistributeParallelForSimdDirective_);
      break; 
    case clang::Stmt::OMPTargetTeamsDistributeDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetTeamsDistributeDirective_);
      break; 
    case clang::Stmt::SubstNonTypeTemplateParmPackExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_SubstNonTypeTemplateParmPackExpr_);
      break; 
    case clang::Stmt::DeclRefExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_DeclRefExpr_);
      break; 
    case clang::Stmt::OMPTargetTeamsDistributeParallelForSimdDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetTeamsDistributeParallelForSimdDirective_);
      break; 
    case clang::Stmt::CXXAddrspaceCastExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXAddrspaceCastExpr_);
      break; 
    case clang::Stmt::OMPGenericLoopDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPGenericLoopDirective_);
      break; 
    case clang::Stmt::AttributedStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_AttributedStmt_);
      break; 
    case clang::Stmt::OMPDistributeParallelForSimdDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPDistributeParallelForSimdDirective_);
      break; 
    case clang::Stmt::OMPDispatchDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPDispatchDirective_);
      break; 
    case clang::Stmt::OMPTeamsGenericLoopDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTeamsGenericLoopDirective_);
      break; 
    case clang::Stmt::OMPArraySectionExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPArraySectionExpr_);
      break; 
    case clang::Stmt::OMPParallelGenericLoopDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPParallelGenericLoopDirective_);
      break; 
    case clang::Stmt::CXXTemporaryObjectExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXTemporaryObjectExpr_);
      break; 
    case clang::Stmt::OMPDistributeSimdDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPDistributeSimdDirective_);
      break; 
    case clang::Stmt::ChooseExprClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ChooseExpr_);
      break; 
    case clang::Stmt::OMPSectionDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPSectionDirective_);
      break; 
    case clang::Stmt::OMPTargetTeamsDistributeParallelForDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetTeamsDistributeParallelForDirective_);
      break; 
    case clang::Stmt::OMPTargetExitDataDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetExitDataDirective_);
      break; 
    case clang::Stmt::CXXCatchStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_CXXCatchStmt_);
      break; 
    case clang::Stmt::FixedPointLiteralClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_FixedPointLiteral_);
      break; 
    case clang::Stmt::OMPCancellationPointDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPCancellationPointDirective_);
      break; 
    case clang::Stmt::OMPDistributeDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPDistributeDirective_);
      break; 
    case clang::Stmt::OMPTaskLoopDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTaskLoopDirective_);
      break; 
    case clang::Stmt::ForStmtClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_ForStmt_);
      break; 
    case clang::Stmt::OMPFlushDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPFlushDirective_);
      break; 
    case clang::Stmt::OMPTaskgroupDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTaskgroupDirective_);
      break; 
    case clang::Stmt::OMPOrderedDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPOrderedDirective_);
      break; 
    case clang::Stmt::OMPTargetEnterDataDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTargetEnterDataDirective_);
      break; 
    case clang::Stmt::OMPTeamsDistributeDirectiveClass: 
      arboretum_create_edge(obj, context_.data_model_.meta_class_, context_.data_model_.class_OMPTeamsDistributeDirective_);
      break; 
    default: break;
  }

  arboretum_create_edge(obj, context_.data_model_.method_stripLabelLikeStatements_, context_.resolve(D->stripLabelLikeStatements()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitStmtExpr(clang::StmtExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getSubStmt_5_, context_.resolve(D->getSubStmt()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_118_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_117_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getLParenLoc_9_, context_.source_model_.resolve(D->getLParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_24_, context_.source_model_.resolve(D->getRParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getTemplateDepth_1_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getTemplateDepth())));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitStringLiteral(clang::StringLiteral* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getString_, context_.data_model_.arboretum_node_for(D->getString().str()));
  arboretum_create_edge(obj, context_.data_model_.method_getBytes_, context_.data_model_.arboretum_node_for(D->getBytes().str()));
  arboretum_create_edge(obj, context_.data_model_.method_getByteLength_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getByteLength())));
  arboretum_create_edge(obj, context_.data_model_.method_getLength_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getLength())));
  arboretum_create_edge(obj, context_.data_model_.method_getCharByteWidth_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getCharByteWidth())));
  arboretum_create_edge(obj, context_.data_model_.method_getKind_4_, context_.data_model_.resolve(D->getKind()));
  arboretum_create_edge(obj, context_.data_model_.method_isOrdinary_, context_.data_model_.arboretum_node_for(D->isOrdinary()));
  arboretum_create_edge(obj, context_.data_model_.method_isWide_, context_.data_model_.arboretum_node_for(D->isWide()));
  arboretum_create_edge(obj, context_.data_model_.method_isUTF8_, context_.data_model_.arboretum_node_for(D->isUTF8()));
  arboretum_create_edge(obj, context_.data_model_.method_isUTF16_, context_.data_model_.arboretum_node_for(D->isUTF16()));
  arboretum_create_edge(obj, context_.data_model_.method_isUTF32_, context_.data_model_.arboretum_node_for(D->isUTF32()));
  arboretum_create_edge(obj, context_.data_model_.method_isUnevaluated_, context_.data_model_.arboretum_node_for(D->isUnevaluated()));
  arboretum_create_edge(obj, context_.data_model_.method_isPascal_, context_.data_model_.arboretum_node_for(D->isPascal()));
  arboretum_create_edge(obj, context_.data_model_.method_containsNonAscii_, context_.data_model_.arboretum_node_for(D->containsNonAscii()));
  arboretum_create_edge(obj, context_.data_model_.method_containsNonAsciiOrNull_, context_.data_model_.arboretum_node_for(D->containsNonAsciiOrNull()));
  arboretum_create_edge(obj, context_.data_model_.method_getNumConcatenated_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumConcatenated())));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_119_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_118_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitSubstNonTypeTemplateParmExpr(clang::SubstNonTypeTemplateParmExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getNameLoc_1_, context_.source_model_.resolve(D->getNameLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_120_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_119_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getReplacement_, context_.resolve(D->getReplacement()));
  arboretum_create_edge(obj, context_.data_model_.method_getAssociatedDecl_2_, context_.resolve(D->getAssociatedDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getIndex_4_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getIndex())));
  // getPackIndex ( class std::optional<unsigned int> )
  arboretum_create_edge(obj, context_.data_model_.method_getParameter_, context_.resolve(D->getParameter()));
  arboretum_create_edge(obj, context_.data_model_.method_isReferenceParameter_, context_.data_model_.arboretum_node_for(D->isReferenceParameter()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitSubstNonTypeTemplateParmPackExpr(clang::SubstNonTypeTemplateParmPackExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getAssociatedDecl_3_, context_.resolve(D->getAssociatedDecl()));
  arboretum_create_edge(obj, context_.data_model_.method_getIndex_5_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getIndex())));
  arboretum_create_edge(obj, context_.data_model_.method_getParameterPack_1_, context_.resolve(D->getParameterPack()));
  arboretum_create_edge(obj, context_.data_model_.method_getParameterPackLocation_1_, context_.source_model_.resolve(D->getParameterPackLocation()));
  // getArgumentPack ( class clang::TemplateArgument )
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_121_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_120_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitSwitchCase(clang::SwitchCase* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getNextSwitchCase_, context_.resolve(D->getNextSwitchCase()));
  arboretum_create_edge(obj, context_.data_model_.method_getKeywordLoc_4_, context_.source_model_.resolve(D->getKeywordLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getColonLoc_3_, context_.source_model_.resolve(D->getColonLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getSubStmt_6_, context_.resolve(D->getSubStmt()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_122_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_121_, context_.source_model_.resolve(D->getEndLoc()));
  return true;
}

bool ArboretumASTVisitor::VisitSwitchStmt(clang::SwitchStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_hasInitStorage_1_, context_.data_model_.arboretum_node_for(D->hasInitStorage()));
  arboretum_create_edge(obj, context_.data_model_.method_hasVarStorage_1_, context_.data_model_.arboretum_node_for(D->hasVarStorage()));
  arboretum_create_edge(obj, context_.data_model_.method_getCond_8_, context_.resolve(D->getCond()));
  arboretum_create_edge(obj, context_.data_model_.method_getBody_11_, context_.resolve(D->getBody()));
  arboretum_create_edge(obj, context_.data_model_.method_getInit_6_, context_.resolve(D->getInit()));
  arboretum_create_edge(obj, context_.data_model_.method_getConditionVariable_2_, context_.resolve(D->getConditionVariable()));
  arboretum_create_edge(obj, context_.data_model_.method_getConditionVariableDeclStmt_2_, context_.resolve(D->getConditionVariableDeclStmt()));
  arboretum_create_edge(obj, context_.data_model_.method_getSwitchCaseList_, context_.resolve(D->getSwitchCaseList()));
  arboretum_create_edge(obj, context_.data_model_.method_getSwitchLoc_, context_.source_model_.resolve(D->getSwitchLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getLParenLoc_10_, context_.source_model_.resolve(D->getLParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_25_, context_.source_model_.resolve(D->getRParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_isAllEnumCasesCovered_, context_.data_model_.arboretum_node_for(D->isAllEnumCasesCovered()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_123_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_122_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitTypeTraitExpr(clang::TypeTraitExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getTrait_2_, context_.data_model_.resolve(D->getTrait()));
  arboretum_create_edge(obj, context_.data_model_.method_getNumArgs_4_, context_.data_model_.arboretum_node_for(static_cast<uint64_t>(D->getNumArgs())));
  // getArgs ( class llvm::ArrayRef<class clang::TypeSourceInfo *> )
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_124_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_123_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitTypoExpr(clang::TypoExpr* D) {
  const Id* obj = context_.resolve(D);
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_125_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_124_, context_.source_model_.resolve(D->getEndLoc()));
  return true;
}

bool ArboretumASTVisitor::VisitUnaryExprOrTypeTraitExpr(clang::UnaryExprOrTypeTraitExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getKind_5_, context_.data_model_.resolve(D->getKind()));
  arboretum_create_edge(obj, context_.data_model_.method_isArgumentType_, context_.data_model_.arboretum_node_for(D->isArgumentType()));
  arboretum_create_edge(obj, context_.data_model_.method_getTypeOfArgument_, context_.resolve(D->getTypeOfArgument()));
  arboretum_create_edge(obj, context_.data_model_.method_getOperatorLoc_9_, context_.source_model_.resolve(D->getOperatorLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_26_, context_.source_model_.resolve(D->getRParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_126_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_125_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitUnaryOperator(clang::UnaryOperator* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getOpcode_2_, context_.data_model_.resolve(D->getOpcode()));
  arboretum_create_edge(obj, context_.data_model_.method_getSubExpr_9_, context_.resolve(D->getSubExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_getOperatorLoc_10_, context_.source_model_.resolve(D->getOperatorLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_canOverflow_, context_.data_model_.arboretum_node_for(D->canOverflow()));
  arboretum_create_edge(obj, context_.data_model_.method_isPrefix_, context_.data_model_.arboretum_node_for(D->isPrefix()));
  arboretum_create_edge(obj, context_.data_model_.method_isPostfix_, context_.data_model_.arboretum_node_for(D->isPostfix()));
  arboretum_create_edge(obj, context_.data_model_.method_isIncrementOp_, context_.data_model_.arboretum_node_for(D->isIncrementOp()));
  arboretum_create_edge(obj, context_.data_model_.method_isDecrementOp_, context_.data_model_.arboretum_node_for(D->isDecrementOp()));
  arboretum_create_edge(obj, context_.data_model_.method_isIncrementDecrementOp_, context_.data_model_.arboretum_node_for(D->isIncrementDecrementOp()));
  arboretum_create_edge(obj, context_.data_model_.method_isArithmeticOp_, context_.data_model_.arboretum_node_for(D->isArithmeticOp()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_127_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_126_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getExprLoc_12_, context_.source_model_.resolve(D->getExprLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  arboretum_create_edge(obj, context_.data_model_.method_hasStoredFPFeatures_4_, context_.data_model_.arboretum_node_for(D->hasStoredFPFeatures()));
  // getStoredFPFeatures ( class clang::FPOptionsOverride )
  // getFPOptionsOverride ( class clang::FPOptionsOverride )
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedLookupExpr(clang::UnresolvedLookupExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_requiresADL_, context_.data_model_.arboretum_node_for(D->requiresADL()));
  arboretum_create_edge(obj, context_.data_model_.method_isOverloaded_, context_.data_model_.arboretum_node_for(D->isOverloaded()));
  arboretum_create_edge(obj, context_.data_model_.method_getNamingClass_1_, context_.resolve(D->getNamingClass()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_128_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_127_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedMemberExpr(clang::UnresolvedMemberExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_isImplicitAccess_3_, context_.data_model_.arboretum_node_for(D->isImplicitAccess()));
  arboretum_create_edge(obj, context_.data_model_.method_getBaseType_2_, context_.resolve(D->getBaseType()));
  arboretum_create_edge(obj, context_.data_model_.method_hasUnresolvedUsing_, context_.data_model_.arboretum_node_for(D->hasUnresolvedUsing()));
  arboretum_create_edge(obj, context_.data_model_.method_isArrow_5_, context_.data_model_.arboretum_node_for(D->isArrow()));
  arboretum_create_edge(obj, context_.data_model_.method_getOperatorLoc_11_, context_.source_model_.resolve(D->getOperatorLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getNamingClass_2_, context_.resolve(D->getNamingClass()));
  // getMemberNameInfo ( const struct clang::DeclarationNameInfo & )
  // getMemberName ( class clang::DeclarationName )
  arboretum_create_edge(obj, context_.data_model_.method_getMemberLoc_3_, context_.source_model_.resolve(D->getMemberLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getExprLoc_13_, context_.source_model_.resolve(D->getExprLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_129_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_128_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitUserDefinedLiteral(clang::UserDefinedLiteral* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getLiteralOperatorKind_, context_.data_model_.resolve(D->getLiteralOperatorKind()));
  arboretum_create_edge(obj, context_.data_model_.method_getCookedLiteral_, context_.resolve(D->getCookedLiteral()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_130_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_129_, context_.source_model_.resolve(D->getEndLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getUDSuffixLoc_, context_.source_model_.resolve(D->getUDSuffixLoc()));
  // getUDSuffix ( const class clang::IdentifierInfo * )
  return true;
}

bool ArboretumASTVisitor::VisitVAArgExpr(clang::VAArgExpr* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getSubExpr_10_, context_.resolve(D->getSubExpr()));
  arboretum_create_edge(obj, context_.data_model_.method_isMicrosoftABI_, context_.data_model_.arboretum_node_for(D->isMicrosoftABI()));
  // getWrittenTypeInfo ( class clang::TypeSourceInfo * )
  arboretum_create_edge(obj, context_.data_model_.method_getBuiltinLoc_5_, context_.source_model_.resolve(D->getBuiltinLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_27_, context_.source_model_.resolve(D->getRParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_131_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_130_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

bool ArboretumASTVisitor::VisitValueStmt(clang::ValueStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_getExprStmt_, context_.resolve(D->getExprStmt()));
  return true;
}

bool ArboretumASTVisitor::VisitWhileStmt(clang::WhileStmt* D) {
  const Id* obj = context_.resolve(D);
  arboretum_create_edge(obj, context_.data_model_.method_hasVarStorage_2_, context_.data_model_.arboretum_node_for(D->hasVarStorage()));
  arboretum_create_edge(obj, context_.data_model_.method_getCond_9_, context_.resolve(D->getCond()));
  arboretum_create_edge(obj, context_.data_model_.method_getBody_12_, context_.resolve(D->getBody()));
  arboretum_create_edge(obj, context_.data_model_.method_getConditionVariable_3_, context_.resolve(D->getConditionVariable()));
  arboretum_create_edge(obj, context_.data_model_.method_getConditionVariableDeclStmt_3_, context_.resolve(D->getConditionVariableDeclStmt()));
  arboretum_create_edge(obj, context_.data_model_.method_getWhileLoc_1_, context_.source_model_.resolve(D->getWhileLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getLParenLoc_11_, context_.source_model_.resolve(D->getLParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getRParenLoc_28_, context_.source_model_.resolve(D->getRParenLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getBeginLoc_132_, context_.source_model_.resolve(D->getBeginLoc()));
  arboretum_create_edge(obj, context_.data_model_.method_getEndLoc_131_, context_.source_model_.resolve(D->getEndLoc()));
  // children ( class llvm::iterator_range<struct clang::ConstStmtIterator> )
  return true;
}

////   END ARBORETUM GENERATED CODE ////
}  // namespace arboretum