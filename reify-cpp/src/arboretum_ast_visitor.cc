#include "arboretum_ast_visitor.h"

#include "arboretum_ffi.h"

namespace arboretum {
//// BEGIN ARBORETUM GENERATED CODE ////
// Types
bool ArboretumASTVisitor::VisitAdjustedType(clang::AdjustedType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitArrayType(clang::ArrayType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getSizeModifier());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getSizeModifier", enum_value)
  }
  return true;
}

bool ArboretumASTVisitor::VisitAtomicType(clang::AtomicType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitAttributedType(clang::AttributedType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getAttrKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getAttrKind", enum_value)
  }
  return true;
}

bool ArboretumASTVisitor::VisitAutoType(clang::AutoType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getTypeConstraintConcept());
    arboretum_create_relation(obj, "getTypeConstraintConcept", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getKeyword());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getKeyword", enum_value)
  }
  return true;
}

bool ArboretumASTVisitor::VisitBTFTagAttributedType(clang::BTFTagAttributedType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getAttr());
    arboretum_create_relation(obj, "getAttr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitBitIntType(clang::BitIntType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitBlockPointerType(clang::BlockPointerType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitBuiltinType(clang::BuiltinType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getKind", enum_value)
  }
  return true;
}

bool ArboretumASTVisitor::VisitComplexType(clang::ComplexType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitConstantArrayType(clang::ConstantArrayType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getSizeExpr());
    arboretum_create_relation(obj, "getSizeExpr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitConstantMatrixType(clang::ConstantMatrixType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitDecayedType(clang::DecayedType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitDecltypeType(clang::DecltypeType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getUnderlyingExpr());
    arboretum_create_relation(obj, "getUnderlyingExpr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitDeducedTemplateSpecializationType(clang::DeducedTemplateSpecializationType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitDeducedType(clang::DeducedType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitDependentAddressSpaceType(clang::DependentAddressSpaceType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getAddrSpaceExpr());
    arboretum_create_relation(obj, "getAddrSpaceExpr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitDependentBitIntType(clang::DependentBitIntType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getNumBitsExpr());
    arboretum_create_relation(obj, "getNumBitsExpr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitDependentDecltypeType(clang::DependentDecltypeType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitDependentNameType(clang::DependentNameType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitDependentSizedArrayType(clang::DependentSizedArrayType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getSizeExpr());
    arboretum_create_relation(obj, "getSizeExpr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitDependentSizedExtVectorType(clang::DependentSizedExtVectorType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getSizeExpr());
    arboretum_create_relation(obj, "getSizeExpr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitDependentSizedMatrixType(clang::DependentSizedMatrixType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getRowExpr());
    arboretum_create_relation(obj, "getRowExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getColumnExpr());
    arboretum_create_relation(obj, "getColumnExpr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitDependentTemplateSpecializationType(clang::DependentTemplateSpecializationType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitDependentTypeOfExprType(clang::DependentTypeOfExprType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitDependentUnaryTransformType(clang::DependentUnaryTransformType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitDependentVectorType(clang::DependentVectorType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getSizeExpr());
    arboretum_create_relation(obj, "getSizeExpr", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getVectorKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getVectorKind", enum_value)
  }
  return true;
}

bool ArboretumASTVisitor::VisitElaboratedType(clang::ElaboratedType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getOwnedTagDecl());
    arboretum_create_relation(obj, "getOwnedTagDecl", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitEnumType(clang::EnumType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getDecl());
    arboretum_create_relation(obj, "getDecl", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitExtVectorType(clang::ExtVectorType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitFunctionNoProtoType(clang::FunctionNoProtoType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitFunctionProtoType(clang::FunctionProtoType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getExceptionSpecType());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getExceptionSpecType", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getNoexceptExpr());
    arboretum_create_relation(obj, "getNoexceptExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getExceptionSpecDecl());
    arboretum_create_relation(obj, "getExceptionSpecDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getExceptionSpecTemplate());
    arboretum_create_relation(obj, "getExceptionSpecTemplate", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->canThrow());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "canThrow", enum_value)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getRefQualifier());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getRefQualifier", enum_value)
  }
  return true;
}

bool ArboretumASTVisitor::VisitFunctionType(clang::FunctionType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getCallConv());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getCallConv", enum_value)
  }
  return true;
}

bool ArboretumASTVisitor::VisitIncompleteArrayType(clang::IncompleteArrayType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitInjectedClassNameType(clang::InjectedClassNameType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getInjectedTST());
    arboretum_create_relation(obj, "getInjectedTST", other)
  }
  {
    const Thing* other = context_.resolve(D->getDecl());
    arboretum_create_relation(obj, "getDecl", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitLValueReferenceType(clang::LValueReferenceType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitMacroQualifiedType(clang::MacroQualifiedType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitMatrixType(clang::MatrixType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitMemberPointerType(clang::MemberPointerType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getClass());
    arboretum_create_relation(obj, "getClass", other)
  }
  {
    const Thing* other = context_.resolve(D->getMostRecentCXXRecordDecl());
    arboretum_create_relation(obj, "getMostRecentCXXRecordDecl", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitObjCInterfaceType(clang::ObjCInterfaceType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCObjectPointerType(clang::ObjCObjectPointerType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCObjectType(clang::ObjCObjectType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCObjectTypeImpl(clang::ObjCObjectTypeImpl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCTypeParamType(clang::ObjCTypeParamType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitPackExpansionType(clang::PackExpansionType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitParenType(clang::ParenType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitPipeType(clang::PipeType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitPointerType(clang::PointerType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitRValueReferenceType(clang::RValueReferenceType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitRecordType(clang::RecordType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getDecl());
    arboretum_create_relation(obj, "getDecl", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitReferenceType(clang::ReferenceType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitSubstTemplateTypeParmPackType(clang::SubstTemplateTypeParmPackType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getAssociatedDecl());
    arboretum_create_relation(obj, "getAssociatedDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getReplacedParameter());
    arboretum_create_relation(obj, "getReplacedParameter", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitSubstTemplateTypeParmType(clang::SubstTemplateTypeParmType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getAssociatedDecl());
    arboretum_create_relation(obj, "getAssociatedDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getReplacedParameter());
    arboretum_create_relation(obj, "getReplacedParameter", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitTagType(clang::TagType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getDecl());
    arboretum_create_relation(obj, "getDecl", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitTemplateSpecializationType(clang::TemplateSpecializationType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitTemplateTypeParmType(clang::TemplateTypeParmType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getDecl());
    arboretum_create_relation(obj, "getDecl", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitType(clang::Type* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getTypeClass());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getTypeClass", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getAsPlaceholderType());
    arboretum_create_relation(obj, "getAsPlaceholderType", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getObjCARCImplicitLifetime());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getObjCARCImplicitLifetime", enum_value)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getScalarTypeKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getScalarTypeKind", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getAsStructureType());
    arboretum_create_relation(obj, "getAsStructureType", other)
  }
  {
    const Thing* other = context_.resolve(D->getAsUnionType());
    arboretum_create_relation(obj, "getAsUnionType", other)
  }
  {
    const Thing* other = context_.resolve(D->getAsComplexIntegerType());
    arboretum_create_relation(obj, "getAsComplexIntegerType", other)
  }
  {
    const Thing* other = context_.resolve(D->getAsObjCInterfaceType());
    arboretum_create_relation(obj, "getAsObjCInterfaceType", other)
  }
  {
    const Thing* other = context_.resolve(D->getAsObjCInterfacePointerType());
    arboretum_create_relation(obj, "getAsObjCInterfacePointerType", other)
  }
  {
    const Thing* other = context_.resolve(D->getAsObjCQualifiedIdType());
    arboretum_create_relation(obj, "getAsObjCQualifiedIdType", other)
  }
  {
    const Thing* other = context_.resolve(D->getAsObjCQualifiedClassType());
    arboretum_create_relation(obj, "getAsObjCQualifiedClassType", other)
  }
  {
    const Thing* other = context_.resolve(D->getAsObjCQualifiedInterfaceType());
    arboretum_create_relation(obj, "getAsObjCQualifiedInterfaceType", other)
  }
  {
    const Thing* other = context_.resolve(D->getAsCXXRecordDecl());
    arboretum_create_relation(obj, "getAsCXXRecordDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getAsRecordDecl());
    arboretum_create_relation(obj, "getAsRecordDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getAsTagDecl());
    arboretum_create_relation(obj, "getAsTagDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getPointeeCXXRecordDecl());
    arboretum_create_relation(obj, "getPointeeCXXRecordDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getContainedDeducedType());
    arboretum_create_relation(obj, "getContainedDeducedType", other)
  }
  {
    const Thing* other = context_.resolve(D->getContainedAutoType());
    arboretum_create_relation(obj, "getContainedAutoType", other)
  }
  {
    const Thing* other = context_.resolve(D->getAsArrayTypeUnsafe());
    arboretum_create_relation(obj, "getAsArrayTypeUnsafe", other)
  }
  {
    const Thing* other = context_.resolve(D->castAsArrayTypeUnsafe());
    arboretum_create_relation(obj, "castAsArrayTypeUnsafe", other)
  }
  {
    const Thing* other = context_.resolve(D->getBaseElementTypeUnsafe());
    arboretum_create_relation(obj, "getBaseElementTypeUnsafe", other)
  }
  {
    const Thing* other = context_.resolve(D->getArrayElementTypeNoTypeQual());
    arboretum_create_relation(obj, "getArrayElementTypeNoTypeQual", other)
  }
  {
    const Thing* other = context_.resolve(D->getPointeeOrArrayElementType());
    arboretum_create_relation(obj, "getPointeeOrArrayElementType", other)
  }
  {
    const Thing* other = context_.resolve(D->getUnqualifiedDesugaredType());
    arboretum_create_relation(obj, "getUnqualifiedDesugaredType", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getLinkage());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getLinkage", enum_value)
  }
  return true;
}

bool ArboretumASTVisitor::VisitTypeOfExprType(clang::TypeOfExprType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getUnderlyingExpr());
    arboretum_create_relation(obj, "getUnderlyingExpr", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getKind", enum_value)
  }
  return true;
}

bool ArboretumASTVisitor::VisitTypeOfType(clang::TypeOfType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getKind", enum_value)
  }
  return true;
}

bool ArboretumASTVisitor::VisitTypeWithKeyword(clang::TypeWithKeyword* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getKeyword());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getKeyword", enum_value)
  }
  return true;
}

bool ArboretumASTVisitor::VisitTypedefType(clang::TypedefType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getDecl());
    arboretum_create_relation(obj, "getDecl", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitUnaryTransformType(clang::UnaryTransformType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getUTTKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getUTTKind", enum_value)
  }
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedUsingType(clang::UnresolvedUsingType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getDecl());
    arboretum_create_relation(obj, "getDecl", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitUsingType(clang::UsingType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getFoundDecl());
    arboretum_create_relation(obj, "getFoundDecl", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitVariableArrayType(clang::VariableArrayType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getSizeExpr());
    arboretum_create_relation(obj, "getSizeExpr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitVectorType(clang::VectorType* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getVectorKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getVectorKind", enum_value)
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
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitBaseUsingDecl(clang::BaseUsingDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitBindingDecl(clang::BindingDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getBinding());
    arboretum_create_relation(obj, "getBinding", other)
  }
  {
    const Thing* other = context_.resolve(D->getDecomposedDecl());
    arboretum_create_relation(obj, "getDecomposedDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getHoldingVar());
    arboretum_create_relation(obj, "getHoldingVar", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitBlockDecl(clang::BlockDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getCompoundBody());
    arboretum_create_relation(obj, "getCompoundBody", other)
  }
  {
    const Thing* other = context_.resolve(D->getBody());
    arboretum_create_relation(obj, "getBody", other)
  }
  {
    const Thing* other = context_.resolve(D->getBlockManglingContextDecl());
    arboretum_create_relation(obj, "getBlockManglingContextDecl", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitBuiltinTemplateDecl(clang::BuiltinTemplateDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getBuiltinTemplateKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getBuiltinTemplateKind", enum_value)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitCXXConstructorDecl(clang::CXXConstructorDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getTargetConstructor());
    arboretum_create_relation(obj, "getTargetConstructor", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitCXXConversionDecl(clang::CXXConversionDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitCXXDeductionGuideDecl(clang::CXXDeductionGuideDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getDeducedTemplate());
    arboretum_create_relation(obj, "getDeducedTemplate", other)
  }
  {
    const Thing* other = context_.resolve(D->getCorrespondingConstructor());
    arboretum_create_relation(obj, "getCorrespondingConstructor", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getDeductionCandidateKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getDeductionCandidateKind", enum_value)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitCXXDestructorDecl(clang::CXXDestructorDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getOperatorDelete());
    arboretum_create_relation(obj, "getOperatorDelete", other)
  }
  {
    const Thing* other = context_.resolve(D->getOperatorDeleteThisArg());
    arboretum_create_relation(obj, "getOperatorDeleteThisArg", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitCXXMethodDecl(clang::CXXMethodDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_relation(obj, "getMostRecentDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_relation(obj, "getMostRecentDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getParent());
    arboretum_create_relation(obj, "getParent", other)
  }
  {
    const Thing* other = context_.resolve(D->getParent());
    arboretum_create_relation(obj, "getParent", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getRefQualifier());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getRefQualifier", enum_value)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitCXXRecordDecl(clang::CXXRecordDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_relation(obj, "getPreviousDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_relation(obj, "getPreviousDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_relation(obj, "getMostRecentDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_relation(obj, "getMostRecentDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getMostRecentNonInjectedDecl());
    arboretum_create_relation(obj, "getMostRecentNonInjectedDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getMostRecentNonInjectedDecl());
    arboretum_create_relation(obj, "getMostRecentNonInjectedDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getDefinition());
    arboretum_create_relation(obj, "getDefinition", other)
  }
  {
    const Thing* other = context_.resolve(D->getLambdaCallOperator());
    arboretum_create_relation(obj, "getLambdaCallOperator", other)
  }
  {
    const Thing* other = context_.resolve(D->getDependentLambdaCallOperator());
    arboretum_create_relation(obj, "getDependentLambdaCallOperator", other)
  }
  {
    const Thing* other = context_.resolve(D->getLambdaStaticInvoker());
    arboretum_create_relation(obj, "getLambdaStaticInvoker", other)
  }
  {
    const Thing* other = context_.resolve(D->getLambdaStaticInvoker());
    arboretum_create_relation(obj, "getLambdaStaticInvoker", other)
  }
  {
    const Thing* other = context_.resolve(D->getInstantiatedFromMemberClass());
    arboretum_create_relation(obj, "getInstantiatedFromMemberClass", other)
  }
  {
    const Thing* other = context_.resolve(D->getDescribedClassTemplate());
    arboretum_create_relation(obj, "getDescribedClassTemplate", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getTemplateSpecializationKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getTemplateSpecializationKind", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getTemplateInstantiationPattern());
    arboretum_create_relation(obj, "getTemplateInstantiationPattern", other)
  }
  {
    const Thing* other = context_.resolve(D->getTemplateInstantiationPattern());
    arboretum_create_relation(obj, "getTemplateInstantiationPattern", other)
  }
  {
    const Thing* other = context_.resolve(D->getDestructor());
    arboretum_create_relation(obj, "getDestructor", other)
  }
  {
    const Thing* other = context_.resolve(D->isLocalClass());
    arboretum_create_relation(obj, "isLocalClass", other)
  }
  {
    const Thing* other = context_.resolve(D->isLocalClass());
    arboretum_create_relation(obj, "isLocalClass", other)
  }
  {
    const Thing* other = context_.resolve(D->getLambdaContextDecl());
    arboretum_create_relation(obj, "getLambdaContextDecl", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getMSVtorDispMode());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getMSVtorDispMode", enum_value)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitCapturedDecl(clang::CapturedDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getBody());
    arboretum_create_relation(obj, "getBody", other)
  }
  {
    const Thing* other = context_.resolve(D->getContextParam());
    arboretum_create_relation(obj, "getContextParam", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitClassScopeFunctionSpecializationDecl(clang::ClassScopeFunctionSpecializationDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getSpecialization());
    arboretum_create_relation(obj, "getSpecialization", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitClassTemplateDecl(clang::ClassTemplateDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getTemplatedDecl());
    arboretum_create_relation(obj, "getTemplatedDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_relation(obj, "getPreviousDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_relation(obj, "getPreviousDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_relation(obj, "getMostRecentDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_relation(obj, "getMostRecentDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getInstantiatedFromMemberTemplate());
    arboretum_create_relation(obj, "getInstantiatedFromMemberTemplate", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitClassTemplatePartialSpecializationDecl(clang::ClassTemplatePartialSpecializationDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getInstantiatedFromMember());
    arboretum_create_relation(obj, "getInstantiatedFromMember", other)
  }
  {
    const Thing* other = context_.resolve(D->getInstantiatedFromMemberTemplate());
    arboretum_create_relation(obj, "getInstantiatedFromMemberTemplate", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitClassTemplateSpecializationDecl(clang::ClassTemplateSpecializationDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getSpecializedTemplate());
    arboretum_create_relation(obj, "getSpecializedTemplate", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getSpecializationKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getSpecializationKind", enum_value)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitConceptDecl(clang::ConceptDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getConstraintExpr());
    arboretum_create_relation(obj, "getConstraintExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitConstructorUsingShadowDecl(clang::ConstructorUsingShadowDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getIntroducer());
    arboretum_create_relation(obj, "getIntroducer", other)
  }
  {
    const Thing* other = context_.resolve(D->getParent());
    arboretum_create_relation(obj, "getParent", other)
  }
  {
    const Thing* other = context_.resolve(D->getParent());
    arboretum_create_relation(obj, "getParent", other)
  }
  {
    const Thing* other = context_.resolve(D->getNominatedBaseClassShadowDecl());
    arboretum_create_relation(obj, "getNominatedBaseClassShadowDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getConstructedBaseClassShadowDecl());
    arboretum_create_relation(obj, "getConstructedBaseClassShadowDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getNominatedBaseClass());
    arboretum_create_relation(obj, "getNominatedBaseClass", other)
  }
  {
    const Thing* other = context_.resolve(D->getConstructedBaseClass());
    arboretum_create_relation(obj, "getConstructedBaseClass", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitDecl(clang::Decl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getKind", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getNextDeclInContext());
    arboretum_create_relation(obj, "getNextDeclInContext", other)
  }
  {
    const Thing* other = context_.resolve(D->getNextDeclInContext());
    arboretum_create_relation(obj, "getNextDeclInContext", other)
  }
  {
    const Thing* other = context_.resolve(D->getNonClosureContext());
    arboretum_create_relation(obj, "getNonClosureContext", other)
  }
  {
    const Thing* other = context_.resolve(D->getNonClosureContext());
    arboretum_create_relation(obj, "getNonClosureContext", other)
  }
  {
    const Thing* other = context_.resolve(D->getTranslationUnitDecl());
    arboretum_create_relation(obj, "getTranslationUnitDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getTranslationUnitDecl());
    arboretum_create_relation(obj, "getTranslationUnitDecl", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getAccess());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getAccess", enum_value)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getAccessUnsafe());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getAccessUnsafe", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getExternalSourceSymbolAttr());
    arboretum_create_relation(obj, "getExternalSourceSymbolAttr", other)
  }
  {
    const Thing* other = context_.resolve(D->getDefiningAttr());
    arboretum_create_relation(obj, "getDefiningAttr", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getModuleOwnershipKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getModuleOwnershipKind", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_relation(obj, "getPreviousDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_relation(obj, "getPreviousDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_relation(obj, "getMostRecentDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_relation(obj, "getMostRecentDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getBody());
    arboretum_create_relation(obj, "getBody", other)
  }
  {
    const Thing* other = context_.resolve(D->getDescribedTemplate());
    arboretum_create_relation(obj, "getDescribedTemplate", other)
  }
  {
    const Thing* other = context_.resolve(D->getAsFunction());
    arboretum_create_relation(obj, "getAsFunction", other)
  }
  {
    const Thing* other = context_.resolve(D->getAsFunction());
    arboretum_create_relation(obj, "getAsFunction", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getFriendObjectKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getFriendObjectKind", enum_value)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitDeclaratorDecl(clang::DeclaratorDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getTrailingRequiresClause());
    arboretum_create_relation(obj, "getTrailingRequiresClause", other)
  }
  {
    const Thing* other = context_.resolve(D->getTrailingRequiresClause());
    arboretum_create_relation(obj, "getTrailingRequiresClause", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitDecompositionDecl(clang::DecompositionDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitEmptyDecl(clang::EmptyDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitEnumConstantDecl(clang::EnumConstantDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getInitExpr());
    arboretum_create_relation(obj, "getInitExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getInitExpr());
    arboretum_create_relation(obj, "getInitExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitEnumDecl(clang::EnumDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_relation(obj, "getPreviousDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_relation(obj, "getPreviousDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_relation(obj, "getMostRecentDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_relation(obj, "getMostRecentDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getDefinition());
    arboretum_create_relation(obj, "getDefinition", other)
  }
  {
    const Thing* other = context_.resolve(D->getTemplateInstantiationPattern());
    arboretum_create_relation(obj, "getTemplateInstantiationPattern", other)
  }
  {
    const Thing* other = context_.resolve(D->getInstantiatedFromMemberEnum());
    arboretum_create_relation(obj, "getInstantiatedFromMemberEnum", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getTemplateSpecializationKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getTemplateSpecializationKind", enum_value)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitExportDecl(clang::ExportDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitExternCContextDecl(clang::ExternCContextDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitFieldDecl(clang::FieldDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getBitWidth());
    arboretum_create_relation(obj, "getBitWidth", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getInClassInitStyle());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getInClassInitStyle", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getInClassInitializer());
    arboretum_create_relation(obj, "getInClassInitializer", other)
  }
  {
    const Thing* other = context_.resolve(D->getCapturedVLAType());
    arboretum_create_relation(obj, "getCapturedVLAType", other)
  }
  {
    const Thing* other = context_.resolve(D->getParent());
    arboretum_create_relation(obj, "getParent", other)
  }
  {
    const Thing* other = context_.resolve(D->getParent());
    arboretum_create_relation(obj, "getParent", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitFileScopeAsmDecl(clang::FileScopeAsmDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getAsmString());
    arboretum_create_relation(obj, "getAsmString", other)
  }
  {
    const Thing* other = context_.resolve(D->getAsmString());
    arboretum_create_relation(obj, "getAsmString", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitFriendDecl(clang::FriendDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getFriendDecl());
    arboretum_create_relation(obj, "getFriendDecl", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitFriendTemplateDecl(clang::FriendTemplateDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getFriendDecl());
    arboretum_create_relation(obj, "getFriendDecl", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitFunctionDecl(clang::FunctionDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getDefinition());
    arboretum_create_relation(obj, "getDefinition", other)
  }
  {
    const Thing* other = context_.resolve(D->getDefinition());
    arboretum_create_relation(obj, "getDefinition", other)
  }
  {
    const Thing* other = context_.resolve(D->getBody());
    arboretum_create_relation(obj, "getBody", other)
  }
  {
    const Thing* other = context_.resolve(D->getBody());
    arboretum_create_relation(obj, "getBody", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getConstexprKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getConstexprKind", enum_value)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getLanguageLinkage());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getLanguageLinkage", enum_value)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getMultiVersionKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getMultiVersionKind", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getExceptionSpecType());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getExceptionSpecType", enum_value)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getStorageClass());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getStorageClass", enum_value)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getOverloadedOperator());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getOverloadedOperator", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getInstantiatedFromMemberFunction());
    arboretum_create_relation(obj, "getInstantiatedFromMemberFunction", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getTemplatedKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getTemplatedKind", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getInstantiatedFromDecl());
    arboretum_create_relation(obj, "getInstantiatedFromDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getDescribedFunctionTemplate());
    arboretum_create_relation(obj, "getDescribedFunctionTemplate", other)
  }
  {
    const Thing* other = context_.resolve(D->getPrimaryTemplate());
    arboretum_create_relation(obj, "getPrimaryTemplate", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getTemplateSpecializationKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getTemplateSpecializationKind", enum_value)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getTemplateSpecializationKindForInstantiation());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getTemplateSpecializationKindForInstantiation", enum_value)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitFunctionTemplateDecl(clang::FunctionTemplateDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getTemplatedDecl());
    arboretum_create_relation(obj, "getTemplatedDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_relation(obj, "getPreviousDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_relation(obj, "getPreviousDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_relation(obj, "getMostRecentDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_relation(obj, "getMostRecentDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getInstantiatedFromMemberTemplate());
    arboretum_create_relation(obj, "getInstantiatedFromMemberTemplate", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitHLSLBufferDecl(clang::HLSLBufferDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitImplicitConceptSpecializationDecl(clang::ImplicitConceptSpecializationDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitImplicitParamDecl(clang::ImplicitParamDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getParameterKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getParameterKind", enum_value)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitImportDecl(clang::ImportDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitIndirectFieldDecl(clang::IndirectFieldDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getAnonField());
    arboretum_create_relation(obj, "getAnonField", other)
  }
  {
    const Thing* other = context_.resolve(D->getVarDecl());
    arboretum_create_relation(obj, "getVarDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitLabelDecl(clang::LabelDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getStmt());
    arboretum_create_relation(obj, "getStmt", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitLifetimeExtendedTemporaryDecl(clang::LifetimeExtendedTemporaryDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getExtendingDecl());
    arboretum_create_relation(obj, "getExtendingDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getExtendingDecl());
    arboretum_create_relation(obj, "getExtendingDecl", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getStorageDuration());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getStorageDuration", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getTemporaryExpr());
    arboretum_create_relation(obj, "getTemporaryExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getTemporaryExpr());
    arboretum_create_relation(obj, "getTemporaryExpr", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitLinkageSpecDecl(clang::LinkageSpecDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getLanguage());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getLanguage", enum_value)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitMSGuidDecl(clang::MSGuidDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitMSPropertyDecl(clang::MSPropertyDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitNamedDecl(clang::NamedDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getLinkageInternal());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getLinkageInternal", enum_value)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getFormalLinkage());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getFormalLinkage", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getUnderlyingDecl());
    arboretum_create_relation(obj, "getUnderlyingDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getUnderlyingDecl());
    arboretum_create_relation(obj, "getUnderlyingDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_relation(obj, "getMostRecentDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_relation(obj, "getMostRecentDecl", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getObjCFStringFormattingFamily());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getObjCFStringFormattingFamily", enum_value)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitNamespaceAliasDecl(clang::NamespaceAliasDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getNamespace());
    arboretum_create_relation(obj, "getNamespace", other)
  }
  {
    const Thing* other = context_.resolve(D->getNamespace());
    arboretum_create_relation(obj, "getNamespace", other)
  }
  {
    const Thing* other = context_.resolve(D->getAliasedNamespace());
    arboretum_create_relation(obj, "getAliasedNamespace", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitNamespaceDecl(clang::NamespaceDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getOriginalNamespace());
    arboretum_create_relation(obj, "getOriginalNamespace", other)
  }
  {
    const Thing* other = context_.resolve(D->getOriginalNamespace());
    arboretum_create_relation(obj, "getOriginalNamespace", other)
  }
  {
    const Thing* other = context_.resolve(D->getAnonymousNamespace());
    arboretum_create_relation(obj, "getAnonymousNamespace", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitNonTypeTemplateParmDecl(clang::NonTypeTemplateParmDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getDefaultArgument());
    arboretum_create_relation(obj, "getDefaultArgument", other)
  }
  {
    const Thing* other = context_.resolve(D->getPlaceholderTypeConstraint());
    arboretum_create_relation(obj, "getPlaceholderTypeConstraint", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitOMPAllocateDecl(clang::OMPAllocateDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitOMPCapturedExprDecl(clang::OMPCapturedExprDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitOMPDeclareMapperDecl(clang::OMPDeclareMapperDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitOMPDeclareReductionDecl(clang::OMPDeclareReductionDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitOMPRequiresDecl(clang::OMPRequiresDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitOMPThreadPrivateDecl(clang::OMPThreadPrivateDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitObjCAtDefsFieldDecl(clang::ObjCAtDefsFieldDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitObjCCategoryDecl(clang::ObjCCategoryDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitObjCCategoryImplDecl(clang::ObjCCategoryImplDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitObjCCompatibleAliasDecl(clang::ObjCCompatibleAliasDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitObjCContainerDecl(clang::ObjCContainerDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitObjCImplDecl(clang::ObjCImplDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitObjCImplementationDecl(clang::ObjCImplementationDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitObjCInterfaceDecl(clang::ObjCInterfaceDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitObjCIvarDecl(clang::ObjCIvarDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitObjCMethodDecl(clang::ObjCMethodDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitObjCPropertyDecl(clang::ObjCPropertyDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitObjCPropertyImplDecl(clang::ObjCPropertyImplDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitObjCProtocolDecl(clang::ObjCProtocolDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitObjCTypeParamDecl(clang::ObjCTypeParamDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitParmVarDecl(clang::ParmVarDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getObjCDeclQualifier());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getObjCDeclQualifier", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getDefaultArg());
    arboretum_create_relation(obj, "getDefaultArg", other)
  }
  {
    const Thing* other = context_.resolve(D->getDefaultArg());
    arboretum_create_relation(obj, "getDefaultArg", other)
  }
  {
    const Thing* other = context_.resolve(D->getUninstantiatedDefaultArg());
    arboretum_create_relation(obj, "getUninstantiatedDefaultArg", other)
  }
  {
    const Thing* other = context_.resolve(D->getUninstantiatedDefaultArg());
    arboretum_create_relation(obj, "getUninstantiatedDefaultArg", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitPragmaCommentDecl(clang::PragmaCommentDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getCommentKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getCommentKind", enum_value)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitPragmaDetectMismatchDecl(clang::PragmaDetectMismatchDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitRecordDecl(clang::RecordDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_relation(obj, "getPreviousDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_relation(obj, "getPreviousDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_relation(obj, "getMostRecentDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_relation(obj, "getMostRecentDecl", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getArgPassingRestrictions());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getArgPassingRestrictions", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getDefinition());
    arboretum_create_relation(obj, "getDefinition", other)
  }
  {
    const Thing* other = context_.resolve(D->findFirstNamedDataMember());
    arboretum_create_relation(obj, "findFirstNamedDataMember", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitRedeclarableTemplateDecl(clang::RedeclarableTemplateDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getInstantiatedFromMemberTemplate());
    arboretum_create_relation(obj, "getInstantiatedFromMemberTemplate", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitRequiresExprBodyDecl(clang::RequiresExprBodyDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitStaticAssertDecl(clang::StaticAssertDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getAssertExpr());
    arboretum_create_relation(obj, "getAssertExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getAssertExpr());
    arboretum_create_relation(obj, "getAssertExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getMessage());
    arboretum_create_relation(obj, "getMessage", other)
  }
  {
    const Thing* other = context_.resolve(D->getMessage());
    arboretum_create_relation(obj, "getMessage", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitTagDecl(clang::TagDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getDefinition());
    arboretum_create_relation(obj, "getDefinition", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getTagKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getTagKind", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getTypedefNameForAnonDecl());
    arboretum_create_relation(obj, "getTypedefNameForAnonDecl", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitTemplateDecl(clang::TemplateDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getTemplatedDecl());
    arboretum_create_relation(obj, "getTemplatedDecl", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitTemplateParamObjectDecl(clang::TemplateParamObjectDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitTemplateTemplateParmDecl(clang::TemplateTemplateParmDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitTemplateTypeParmDecl(clang::TemplateTypeParmDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitTopLevelStmtDecl(clang::TopLevelStmtDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getStmt());
    arboretum_create_relation(obj, "getStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getStmt());
    arboretum_create_relation(obj, "getStmt", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitTranslationUnitDecl(clang::TranslationUnitDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getAnonymousNamespace());
    arboretum_create_relation(obj, "getAnonymousNamespace", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitTypeAliasDecl(clang::TypeAliasDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getDescribedAliasTemplate());
    arboretum_create_relation(obj, "getDescribedAliasTemplate", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitTypeAliasTemplateDecl(clang::TypeAliasTemplateDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getTemplatedDecl());
    arboretum_create_relation(obj, "getTemplatedDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_relation(obj, "getPreviousDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_relation(obj, "getPreviousDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getInstantiatedFromMemberTemplate());
    arboretum_create_relation(obj, "getInstantiatedFromMemberTemplate", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitTypeDecl(clang::TypeDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getTypeForDecl());
    arboretum_create_relation(obj, "getTypeForDecl", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitTypedefDecl(clang::TypedefDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitTypedefNameDecl(clang::TypedefNameDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitUnnamedGlobalConstantDecl(clang::UnnamedGlobalConstantDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedUsingIfExistsDecl(clang::UnresolvedUsingIfExistsDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedUsingTypenameDecl(clang::UnresolvedUsingTypenameDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedUsingValueDecl(clang::UnresolvedUsingValueDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitUsingDecl(clang::UsingDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitUsingDirectiveDecl(clang::UsingDirectiveDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getNominatedNamespaceAsWritten());
    arboretum_create_relation(obj, "getNominatedNamespaceAsWritten", other)
  }
  {
    const Thing* other = context_.resolve(D->getNominatedNamespaceAsWritten());
    arboretum_create_relation(obj, "getNominatedNamespaceAsWritten", other)
  }
  {
    const Thing* other = context_.resolve(D->getNominatedNamespace());
    arboretum_create_relation(obj, "getNominatedNamespace", other)
  }
  {
    const Thing* other = context_.resolve(D->getNominatedNamespace());
    arboretum_create_relation(obj, "getNominatedNamespace", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitUsingEnumDecl(clang::UsingEnumDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getEnumDecl());
    arboretum_create_relation(obj, "getEnumDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitUsingPackDecl(clang::UsingPackDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getInstantiatedFromUsingDecl());
    arboretum_create_relation(obj, "getInstantiatedFromUsingDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitUsingShadowDecl(clang::UsingShadowDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getTargetDecl());
    arboretum_create_relation(obj, "getTargetDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getIntroducer());
    arboretum_create_relation(obj, "getIntroducer", other)
  }
  {
    const Thing* other = context_.resolve(D->getNextUsingShadowDecl());
    arboretum_create_relation(obj, "getNextUsingShadowDecl", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitValueDecl(clang::ValueDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getPotentiallyDecomposedVarDecl());
    arboretum_create_relation(obj, "getPotentiallyDecomposedVarDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getPotentiallyDecomposedVarDecl());
    arboretum_create_relation(obj, "getPotentiallyDecomposedVarDecl", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitVarDecl(clang::VarDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getStorageClass());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getStorageClass", enum_value)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getTSCSpec());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getTSCSpec", enum_value)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getTLSKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getTLSKind", enum_value)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getStorageDuration());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getStorageDuration", enum_value)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getLanguageLinkage());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getLanguageLinkage", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->isThisDeclarationADefinition());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "isThisDeclarationADefinition", enum_value)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->isThisDeclarationADefinition());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "isThisDeclarationADefinition", enum_value)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->hasDefinition());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "hasDefinition", enum_value)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->hasDefinition());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "hasDefinition", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getActingDefinition());
    arboretum_create_relation(obj, "getActingDefinition", other)
  }
  {
    const Thing* other = context_.resolve(D->getActingDefinition());
    arboretum_create_relation(obj, "getActingDefinition", other)
  }
  {
    const Thing* other = context_.resolve(D->getDefinition());
    arboretum_create_relation(obj, "getDefinition", other)
  }
  {
    const Thing* other = context_.resolve(D->getDefinition());
    arboretum_create_relation(obj, "getDefinition", other)
  }
  {
    const Thing* other = context_.resolve(D->getDefinition());
    arboretum_create_relation(obj, "getDefinition", other)
  }
  {
    const Thing* other = context_.resolve(D->getDefinition());
    arboretum_create_relation(obj, "getDefinition", other)
  }
  {
    const Thing* other = context_.resolve(D->getAnyInitializer());
    arboretum_create_relation(obj, "getAnyInitializer", other)
  }
  {
    const Thing* other = context_.resolve(D->getAnyInitializer());
    arboretum_create_relation(obj, "getAnyInitializer", other)
  }
  {
    const Thing* other = context_.resolve(D->getInit());
    arboretum_create_relation(obj, "getInit", other)
  }
  {
    const Thing* other = context_.resolve(D->getInit());
    arboretum_create_relation(obj, "getInit", other)
  }
  {
    const Thing* other = context_.resolve(D->getInitializingDeclaration());
    arboretum_create_relation(obj, "getInitializingDeclaration", other)
  }
  {
    const Thing* other = context_.resolve(D->getInitializingDeclaration());
    arboretum_create_relation(obj, "getInitializingDeclaration", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getInitStyle());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getInitStyle", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getTemplateInstantiationPattern());
    arboretum_create_relation(obj, "getTemplateInstantiationPattern", other)
  }
  {
    const Thing* other = context_.resolve(D->getInstantiatedFromStaticDataMember());
    arboretum_create_relation(obj, "getInstantiatedFromStaticDataMember", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getTemplateSpecializationKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getTemplateSpecializationKind", enum_value)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getTemplateSpecializationKindForInstantiation());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getTemplateSpecializationKindForInstantiation", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getDescribedVarTemplate());
    arboretum_create_relation(obj, "getDescribedVarTemplate", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitVarTemplateDecl(clang::VarTemplateDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getTemplatedDecl());
    arboretum_create_relation(obj, "getTemplatedDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCanonicalDecl());
    arboretum_create_relation(obj, "getCanonicalDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_relation(obj, "getPreviousDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getPreviousDecl());
    arboretum_create_relation(obj, "getPreviousDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_relation(obj, "getMostRecentDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getMostRecentDecl());
    arboretum_create_relation(obj, "getMostRecentDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getInstantiatedFromMemberTemplate());
    arboretum_create_relation(obj, "getInstantiatedFromMemberTemplate", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitVarTemplatePartialSpecializationDecl(clang::VarTemplatePartialSpecializationDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getInstantiatedFromMember());
    arboretum_create_relation(obj, "getInstantiatedFromMember", other)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}

bool ArboretumASTVisitor::VisitVarTemplateSpecializationDecl(clang::VarTemplateSpecializationDecl* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getSpecializedTemplate());
    arboretum_create_relation(obj, "getSpecializedTemplate", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getSpecializationKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getSpecializationKind", enum_value)
  }
  aboretum_merge_fields(obj, builder);
  return true;
}


// Stmts
bool ArboretumASTVisitor::VisitAbstractConditionalOperator(clang::AbstractConditionalOperator* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getCond());
    arboretum_create_relation(obj, "getCond", other)
  }
  {
    const Thing* other = context_.resolve(D->getTrueExpr());
    arboretum_create_relation(obj, "getTrueExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getFalseExpr());
    arboretum_create_relation(obj, "getFalseExpr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitAddrLabelExpr(clang::AddrLabelExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getLabel());
    arboretum_create_relation(obj, "getLabel", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitArrayInitIndexExpr(clang::ArrayInitIndexExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitArrayInitLoopExpr(clang::ArrayInitLoopExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getCommonExpr());
    arboretum_create_relation(obj, "getCommonExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getSubExpr());
    arboretum_create_relation(obj, "getSubExpr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitArraySubscriptExpr(clang::ArraySubscriptExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getLHS());
    arboretum_create_relation(obj, "getLHS", other)
  }
  {
    const Thing* other = context_.resolve(D->getLHS());
    arboretum_create_relation(obj, "getLHS", other)
  }
  {
    const Thing* other = context_.resolve(D->getRHS());
    arboretum_create_relation(obj, "getRHS", other)
  }
  {
    const Thing* other = context_.resolve(D->getRHS());
    arboretum_create_relation(obj, "getRHS", other)
  }
  {
    const Thing* other = context_.resolve(D->getBase());
    arboretum_create_relation(obj, "getBase", other)
  }
  {
    const Thing* other = context_.resolve(D->getBase());
    arboretum_create_relation(obj, "getBase", other)
  }
  {
    const Thing* other = context_.resolve(D->getIdx());
    arboretum_create_relation(obj, "getIdx", other)
  }
  {
    const Thing* other = context_.resolve(D->getIdx());
    arboretum_create_relation(obj, "getIdx", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitArrayTypeTraitExpr(clang::ArrayTypeTraitExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getTrait());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getTrait", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getDimensionExpression());
    arboretum_create_relation(obj, "getDimensionExpression", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitAsTypeExpr(clang::AsTypeExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getSrcExpr());
    arboretum_create_relation(obj, "getSrcExpr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitAsmStmt(clang::AsmStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitAtomicExpr(clang::AtomicExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getPtr());
    arboretum_create_relation(obj, "getPtr", other)
  }
  {
    const Thing* other = context_.resolve(D->getOrder());
    arboretum_create_relation(obj, "getOrder", other)
  }
  {
    const Thing* other = context_.resolve(D->getScope());
    arboretum_create_relation(obj, "getScope", other)
  }
  {
    const Thing* other = context_.resolve(D->getVal1());
    arboretum_create_relation(obj, "getVal1", other)
  }
  {
    const Thing* other = context_.resolve(D->getOrderFail());
    arboretum_create_relation(obj, "getOrderFail", other)
  }
  {
    const Thing* other = context_.resolve(D->getVal2());
    arboretum_create_relation(obj, "getVal2", other)
  }
  {
    const Thing* other = context_.resolve(D->getWeak());
    arboretum_create_relation(obj, "getWeak", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getOp());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getOp", enum_value)
  }
  return true;
}

bool ArboretumASTVisitor::VisitAttributedStmt(clang::AttributedStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getSubStmt());
    arboretum_create_relation(obj, "getSubStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getSubStmt());
    arboretum_create_relation(obj, "getSubStmt", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitBinaryConditionalOperator(clang::BinaryConditionalOperator* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getCommon());
    arboretum_create_relation(obj, "getCommon", other)
  }
  {
    const Thing* other = context_.resolve(D->getOpaqueValue());
    arboretum_create_relation(obj, "getOpaqueValue", other)
  }
  {
    const Thing* other = context_.resolve(D->getCond());
    arboretum_create_relation(obj, "getCond", other)
  }
  {
    const Thing* other = context_.resolve(D->getTrueExpr());
    arboretum_create_relation(obj, "getTrueExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getFalseExpr());
    arboretum_create_relation(obj, "getFalseExpr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitBinaryOperator(clang::BinaryOperator* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getOpcode());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getOpcode", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getLHS());
    arboretum_create_relation(obj, "getLHS", other)
  }
  {
    const Thing* other = context_.resolve(D->getRHS());
    arboretum_create_relation(obj, "getRHS", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitBlockExpr(clang::BlockExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getBlockDecl());
    arboretum_create_relation(obj, "getBlockDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getBlockDecl());
    arboretum_create_relation(obj, "getBlockDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getBody());
    arboretum_create_relation(obj, "getBody", other)
  }
  {
    const Thing* other = context_.resolve(D->getBody());
    arboretum_create_relation(obj, "getBody", other)
  }
  {
    const Thing* other = context_.resolve(D->getFunctionType());
    arboretum_create_relation(obj, "getFunctionType", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitBreakStmt(clang::BreakStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitBuiltinBitCastExpr(clang::BuiltinBitCastExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCStyleCastExpr(clang::CStyleCastExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCUDAKernelCallExpr(clang::CUDAKernelCallExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getConfig());
    arboretum_create_relation(obj, "getConfig", other)
  }
  {
    const Thing* other = context_.resolve(D->getConfig());
    arboretum_create_relation(obj, "getConfig", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXAddrspaceCastExpr(clang::CXXAddrspaceCastExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCXXBindTemporaryExpr(clang::CXXBindTemporaryExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getSubExpr());
    arboretum_create_relation(obj, "getSubExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getSubExpr());
    arboretum_create_relation(obj, "getSubExpr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXBoolLiteralExpr(clang::CXXBoolLiteralExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCXXCatchStmt(clang::CXXCatchStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getExceptionDecl());
    arboretum_create_relation(obj, "getExceptionDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getHandlerBlock());
    arboretum_create_relation(obj, "getHandlerBlock", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXConstCastExpr(clang::CXXConstCastExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCXXConstructExpr(clang::CXXConstructExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getConstructor());
    arboretum_create_relation(obj, "getConstructor", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getConstructionKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getConstructionKind", enum_value)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXDefaultArgExpr(clang::CXXDefaultArgExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getParam());
    arboretum_create_relation(obj, "getParam", other)
  }
  {
    const Thing* other = context_.resolve(D->getParam());
    arboretum_create_relation(obj, "getParam", other)
  }
  {
    const Thing* other = context_.resolve(D->getExpr());
    arboretum_create_relation(obj, "getExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getExpr());
    arboretum_create_relation(obj, "getExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getRewrittenExpr());
    arboretum_create_relation(obj, "getRewrittenExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getRewrittenExpr());
    arboretum_create_relation(obj, "getRewrittenExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getAdjustedRewrittenExpr());
    arboretum_create_relation(obj, "getAdjustedRewrittenExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getAdjustedRewrittenExpr());
    arboretum_create_relation(obj, "getAdjustedRewrittenExpr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXDefaultInitExpr(clang::CXXDefaultInitExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getField());
    arboretum_create_relation(obj, "getField", other)
  }
  {
    const Thing* other = context_.resolve(D->getField());
    arboretum_create_relation(obj, "getField", other)
  }
  {
    const Thing* other = context_.resolve(D->getExpr());
    arboretum_create_relation(obj, "getExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getExpr());
    arboretum_create_relation(obj, "getExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getRewrittenExpr());
    arboretum_create_relation(obj, "getRewrittenExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getRewrittenExpr());
    arboretum_create_relation(obj, "getRewrittenExpr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXDeleteExpr(clang::CXXDeleteExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getOperatorDelete());
    arboretum_create_relation(obj, "getOperatorDelete", other)
  }
  {
    const Thing* other = context_.resolve(D->getArgument());
    arboretum_create_relation(obj, "getArgument", other)
  }
  {
    const Thing* other = context_.resolve(D->getArgument());
    arboretum_create_relation(obj, "getArgument", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXDependentScopeMemberExpr(clang::CXXDependentScopeMemberExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getBase());
    arboretum_create_relation(obj, "getBase", other)
  }
  {
    const Thing* other = context_.resolve(D->getFirstQualifierFoundInScope());
    arboretum_create_relation(obj, "getFirstQualifierFoundInScope", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXDynamicCastExpr(clang::CXXDynamicCastExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCXXFoldExpr(clang::CXXFoldExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getCallee());
    arboretum_create_relation(obj, "getCallee", other)
  }
  {
    const Thing* other = context_.resolve(D->getLHS());
    arboretum_create_relation(obj, "getLHS", other)
  }
  {
    const Thing* other = context_.resolve(D->getRHS());
    arboretum_create_relation(obj, "getRHS", other)
  }
  {
    const Thing* other = context_.resolve(D->getPattern());
    arboretum_create_relation(obj, "getPattern", other)
  }
  {
    const Thing* other = context_.resolve(D->getInit());
    arboretum_create_relation(obj, "getInit", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getOperator());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getOperator", enum_value)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXForRangeStmt(clang::CXXForRangeStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getInit());
    arboretum_create_relation(obj, "getInit", other)
  }
  {
    const Thing* other = context_.resolve(D->getLoopVariable());
    arboretum_create_relation(obj, "getLoopVariable", other)
  }
  {
    const Thing* other = context_.resolve(D->getRangeInit());
    arboretum_create_relation(obj, "getRangeInit", other)
  }
  {
    const Thing* other = context_.resolve(D->getInit());
    arboretum_create_relation(obj, "getInit", other)
  }
  {
    const Thing* other = context_.resolve(D->getLoopVariable());
    arboretum_create_relation(obj, "getLoopVariable", other)
  }
  {
    const Thing* other = context_.resolve(D->getRangeInit());
    arboretum_create_relation(obj, "getRangeInit", other)
  }
  {
    const Thing* other = context_.resolve(D->getRangeStmt());
    arboretum_create_relation(obj, "getRangeStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getBeginStmt());
    arboretum_create_relation(obj, "getBeginStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getEndStmt());
    arboretum_create_relation(obj, "getEndStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getCond());
    arboretum_create_relation(obj, "getCond", other)
  }
  {
    const Thing* other = context_.resolve(D->getInc());
    arboretum_create_relation(obj, "getInc", other)
  }
  {
    const Thing* other = context_.resolve(D->getLoopVarStmt());
    arboretum_create_relation(obj, "getLoopVarStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getBody());
    arboretum_create_relation(obj, "getBody", other)
  }
  {
    const Thing* other = context_.resolve(D->getRangeStmt());
    arboretum_create_relation(obj, "getRangeStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getBeginStmt());
    arboretum_create_relation(obj, "getBeginStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getEndStmt());
    arboretum_create_relation(obj, "getEndStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getCond());
    arboretum_create_relation(obj, "getCond", other)
  }
  {
    const Thing* other = context_.resolve(D->getInc());
    arboretum_create_relation(obj, "getInc", other)
  }
  {
    const Thing* other = context_.resolve(D->getLoopVarStmt());
    arboretum_create_relation(obj, "getLoopVarStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getBody());
    arboretum_create_relation(obj, "getBody", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXFunctionalCastExpr(clang::CXXFunctionalCastExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCXXInheritedCtorInitExpr(clang::CXXInheritedCtorInitExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getConstructor());
    arboretum_create_relation(obj, "getConstructor", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getConstructionKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getConstructionKind", enum_value)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXMemberCallExpr(clang::CXXMemberCallExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getImplicitObjectArgument());
    arboretum_create_relation(obj, "getImplicitObjectArgument", other)
  }
  {
    const Thing* other = context_.resolve(D->getMethodDecl());
    arboretum_create_relation(obj, "getMethodDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getRecordDecl());
    arboretum_create_relation(obj, "getRecordDecl", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXNamedCastExpr(clang::CXXNamedCastExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCXXNewExpr(clang::CXXNewExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getOperatorNew());
    arboretum_create_relation(obj, "getOperatorNew", other)
  }
  {
    const Thing* other = context_.resolve(D->getOperatorDelete());
    arboretum_create_relation(obj, "getOperatorDelete", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getInitializationStyle());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getInitializationStyle", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getInitializer());
    arboretum_create_relation(obj, "getInitializer", other)
  }
  {
    const Thing* other = context_.resolve(D->getInitializer());
    arboretum_create_relation(obj, "getInitializer", other)
  }
  {
    const Thing* other = context_.resolve(D->getConstructExpr());
    arboretum_create_relation(obj, "getConstructExpr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXNoexceptExpr(clang::CXXNoexceptExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getOperand());
    arboretum_create_relation(obj, "getOperand", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXNullPtrLiteralExpr(clang::CXXNullPtrLiteralExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCXXOperatorCallExpr(clang::CXXOperatorCallExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getOperator());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getOperator", enum_value)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXParenListInitExpr(clang::CXXParenListInitExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getArrayFiller());
    arboretum_create_relation(obj, "getArrayFiller", other)
  }
  {
    const Thing* other = context_.resolve(D->getArrayFiller());
    arboretum_create_relation(obj, "getArrayFiller", other)
  }
  {
    const Thing* other = context_.resolve(D->getInitializedFieldInUnion());
    arboretum_create_relation(obj, "getInitializedFieldInUnion", other)
  }
  {
    const Thing* other = context_.resolve(D->getInitializedFieldInUnion());
    arboretum_create_relation(obj, "getInitializedFieldInUnion", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXPseudoDestructorExpr(clang::CXXPseudoDestructorExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getBase());
    arboretum_create_relation(obj, "getBase", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXReinterpretCastExpr(clang::CXXReinterpretCastExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCXXRewrittenBinaryOperator(clang::CXXRewrittenBinaryOperator* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getSemanticForm());
    arboretum_create_relation(obj, "getSemanticForm", other)
  }
  {
    const Thing* other = context_.resolve(D->getSemanticForm());
    arboretum_create_relation(obj, "getSemanticForm", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getOperator());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getOperator", enum_value)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getOpcode());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getOpcode", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getLHS());
    arboretum_create_relation(obj, "getLHS", other)
  }
  {
    const Thing* other = context_.resolve(D->getRHS());
    arboretum_create_relation(obj, "getRHS", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXScalarValueInitExpr(clang::CXXScalarValueInitExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCXXStaticCastExpr(clang::CXXStaticCastExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCXXStdInitializerListExpr(clang::CXXStdInitializerListExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getSubExpr());
    arboretum_create_relation(obj, "getSubExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getSubExpr());
    arboretum_create_relation(obj, "getSubExpr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXTemporaryObjectExpr(clang::CXXTemporaryObjectExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCXXThisExpr(clang::CXXThisExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCXXThrowExpr(clang::CXXThrowExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getSubExpr());
    arboretum_create_relation(obj, "getSubExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getSubExpr());
    arboretum_create_relation(obj, "getSubExpr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXTryStmt(clang::CXXTryStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getTryBlock());
    arboretum_create_relation(obj, "getTryBlock", other)
  }
  {
    const Thing* other = context_.resolve(D->getTryBlock());
    arboretum_create_relation(obj, "getTryBlock", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXTypeidExpr(clang::CXXTypeidExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getExprOperand());
    arboretum_create_relation(obj, "getExprOperand", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCXXUnresolvedConstructExpr(clang::CXXUnresolvedConstructExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCXXUuidofExpr(clang::CXXUuidofExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getExprOperand());
    arboretum_create_relation(obj, "getExprOperand", other)
  }
  {
    const Thing* other = context_.resolve(D->getGuidDecl());
    arboretum_create_relation(obj, "getGuidDecl", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCallExpr(clang::CallExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getCallee());
    arboretum_create_relation(obj, "getCallee", other)
  }
  {
    const Thing* other = context_.resolve(D->getCallee());
    arboretum_create_relation(obj, "getCallee", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getADLCallKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getADLCallKind", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getCalleeDecl());
    arboretum_create_relation(obj, "getCalleeDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCalleeDecl());
    arboretum_create_relation(obj, "getCalleeDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getDirectCallee());
    arboretum_create_relation(obj, "getDirectCallee", other)
  }
  {
    const Thing* other = context_.resolve(D->getDirectCallee());
    arboretum_create_relation(obj, "getDirectCallee", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCapturedStmt(clang::CapturedStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getCapturedStmt());
    arboretum_create_relation(obj, "getCapturedStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getCapturedStmt());
    arboretum_create_relation(obj, "getCapturedStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getCapturedDecl());
    arboretum_create_relation(obj, "getCapturedDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getCapturedDecl());
    arboretum_create_relation(obj, "getCapturedDecl", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getCapturedRegionKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getCapturedRegionKind", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getCapturedRecordDecl());
    arboretum_create_relation(obj, "getCapturedRecordDecl", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCaseStmt(clang::CaseStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getLHS());
    arboretum_create_relation(obj, "getLHS", other)
  }
  {
    const Thing* other = context_.resolve(D->getLHS());
    arboretum_create_relation(obj, "getLHS", other)
  }
  {
    const Thing* other = context_.resolve(D->getRHS());
    arboretum_create_relation(obj, "getRHS", other)
  }
  {
    const Thing* other = context_.resolve(D->getRHS());
    arboretum_create_relation(obj, "getRHS", other)
  }
  {
    const Thing* other = context_.resolve(D->getSubStmt());
    arboretum_create_relation(obj, "getSubStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getSubStmt());
    arboretum_create_relation(obj, "getSubStmt", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCastExpr(clang::CastExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getCastKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getCastKind", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getSubExpr());
    arboretum_create_relation(obj, "getSubExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getSubExpr());
    arboretum_create_relation(obj, "getSubExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getSubExprAsWritten());
    arboretum_create_relation(obj, "getSubExprAsWritten", other)
  }
  {
    const Thing* other = context_.resolve(D->getSubExprAsWritten());
    arboretum_create_relation(obj, "getSubExprAsWritten", other)
  }
  {
    const Thing* other = context_.resolve(D->getConversionFunction());
    arboretum_create_relation(obj, "getConversionFunction", other)
  }
  {
    const Thing* other = context_.resolve(D->getTargetUnionField());
    arboretum_create_relation(obj, "getTargetUnionField", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCharacterLiteral(clang::CharacterLiteral* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getKind", enum_value)
  }
  return true;
}

bool ArboretumASTVisitor::VisitChooseExpr(clang::ChooseExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getChosenSubExpr());
    arboretum_create_relation(obj, "getChosenSubExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getCond());
    arboretum_create_relation(obj, "getCond", other)
  }
  {
    const Thing* other = context_.resolve(D->getLHS());
    arboretum_create_relation(obj, "getLHS", other)
  }
  {
    const Thing* other = context_.resolve(D->getRHS());
    arboretum_create_relation(obj, "getRHS", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCoawaitExpr(clang::CoawaitExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCompoundAssignOperator(clang::CompoundAssignOperator* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitCompoundLiteralExpr(clang::CompoundLiteralExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getInitializer());
    arboretum_create_relation(obj, "getInitializer", other)
  }
  {
    const Thing* other = context_.resolve(D->getInitializer());
    arboretum_create_relation(obj, "getInitializer", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCompoundStmt(clang::CompoundStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->body_front());
    arboretum_create_relation(obj, "body_front", other)
  }
  {
    const Thing* other = context_.resolve(D->body_back());
    arboretum_create_relation(obj, "body_back", other)
  }
  {
    const Thing* other = context_.resolve(D->body_front());
    arboretum_create_relation(obj, "body_front", other)
  }
  {
    const Thing* other = context_.resolve(D->body_back());
    arboretum_create_relation(obj, "body_back", other)
  }
  {
    const Thing* other = context_.resolve(D->getStmtExprResult());
    arboretum_create_relation(obj, "getStmtExprResult", other)
  }
  {
    const Thing* other = context_.resolve(D->getStmtExprResult());
    arboretum_create_relation(obj, "getStmtExprResult", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitConceptSpecializationExpr(clang::ConceptSpecializationExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getSpecializationDecl());
    arboretum_create_relation(obj, "getSpecializationDecl", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitConditionalOperator(clang::ConditionalOperator* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getCond());
    arboretum_create_relation(obj, "getCond", other)
  }
  {
    const Thing* other = context_.resolve(D->getTrueExpr());
    arboretum_create_relation(obj, "getTrueExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getFalseExpr());
    arboretum_create_relation(obj, "getFalseExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getLHS());
    arboretum_create_relation(obj, "getLHS", other)
  }
  {
    const Thing* other = context_.resolve(D->getRHS());
    arboretum_create_relation(obj, "getRHS", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitConstantExpr(clang::ConstantExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getResultAPValueKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getResultAPValueKind", enum_value)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getResultStorageKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getResultStorageKind", enum_value)
  }
  return true;
}

bool ArboretumASTVisitor::VisitContinueStmt(clang::ContinueStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitConvertVectorExpr(clang::ConvertVectorExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getSrcExpr());
    arboretum_create_relation(obj, "getSrcExpr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCoreturnStmt(clang::CoreturnStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getOperand());
    arboretum_create_relation(obj, "getOperand", other)
  }
  {
    const Thing* other = context_.resolve(D->getPromiseCall());
    arboretum_create_relation(obj, "getPromiseCall", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCoroutineBodyStmt(clang::CoroutineBodyStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getBody());
    arboretum_create_relation(obj, "getBody", other)
  }
  {
    const Thing* other = context_.resolve(D->getPromiseDeclStmt());
    arboretum_create_relation(obj, "getPromiseDeclStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getPromiseDecl());
    arboretum_create_relation(obj, "getPromiseDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getInitSuspendStmt());
    arboretum_create_relation(obj, "getInitSuspendStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getFinalSuspendStmt());
    arboretum_create_relation(obj, "getFinalSuspendStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getExceptionHandler());
    arboretum_create_relation(obj, "getExceptionHandler", other)
  }
  {
    const Thing* other = context_.resolve(D->getFallthroughHandler());
    arboretum_create_relation(obj, "getFallthroughHandler", other)
  }
  {
    const Thing* other = context_.resolve(D->getAllocate());
    arboretum_create_relation(obj, "getAllocate", other)
  }
  {
    const Thing* other = context_.resolve(D->getDeallocate());
    arboretum_create_relation(obj, "getDeallocate", other)
  }
  {
    const Thing* other = context_.resolve(D->getResultDecl());
    arboretum_create_relation(obj, "getResultDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getReturnValueInit());
    arboretum_create_relation(obj, "getReturnValueInit", other)
  }
  {
    const Thing* other = context_.resolve(D->getReturnValue());
    arboretum_create_relation(obj, "getReturnValue", other)
  }
  {
    const Thing* other = context_.resolve(D->getReturnStmt());
    arboretum_create_relation(obj, "getReturnStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getReturnStmtOnAllocFailure());
    arboretum_create_relation(obj, "getReturnStmtOnAllocFailure", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCoroutineSuspendExpr(clang::CoroutineSuspendExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getCommonExpr());
    arboretum_create_relation(obj, "getCommonExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getOpaqueValue());
    arboretum_create_relation(obj, "getOpaqueValue", other)
  }
  {
    const Thing* other = context_.resolve(D->getReadyExpr());
    arboretum_create_relation(obj, "getReadyExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getSuspendExpr());
    arboretum_create_relation(obj, "getSuspendExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getResumeExpr());
    arboretum_create_relation(obj, "getResumeExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getOperand());
    arboretum_create_relation(obj, "getOperand", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitCoyieldExpr(clang::CoyieldExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitDeclRefExpr(clang::DeclRefExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getDecl());
    arboretum_create_relation(obj, "getDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getDecl());
    arboretum_create_relation(obj, "getDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getFoundDecl());
    arboretum_create_relation(obj, "getFoundDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getFoundDecl());
    arboretum_create_relation(obj, "getFoundDecl", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->isNonOdrUse());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "isNonOdrUse", enum_value)
  }
  return true;
}

bool ArboretumASTVisitor::VisitDeclStmt(clang::DeclStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getSingleDecl());
    arboretum_create_relation(obj, "getSingleDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getSingleDecl());
    arboretum_create_relation(obj, "getSingleDecl", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitDefaultStmt(clang::DefaultStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getSubStmt());
    arboretum_create_relation(obj, "getSubStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getSubStmt());
    arboretum_create_relation(obj, "getSubStmt", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitDependentCoawaitExpr(clang::DependentCoawaitExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getOperand());
    arboretum_create_relation(obj, "getOperand", other)
  }
  {
    const Thing* other = context_.resolve(D->getOperatorCoawaitLookup());
    arboretum_create_relation(obj, "getOperatorCoawaitLookup", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitDependentScopeDeclRefExpr(clang::DependentScopeDeclRefExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitDesignatedInitExpr(clang::DesignatedInitExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getInit());
    arboretum_create_relation(obj, "getInit", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitDesignatedInitUpdateExpr(clang::DesignatedInitUpdateExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getBase());
    arboretum_create_relation(obj, "getBase", other)
  }
  {
    const Thing* other = context_.resolve(D->getUpdater());
    arboretum_create_relation(obj, "getUpdater", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitDoStmt(clang::DoStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getCond());
    arboretum_create_relation(obj, "getCond", other)
  }
  {
    const Thing* other = context_.resolve(D->getCond());
    arboretum_create_relation(obj, "getCond", other)
  }
  {
    const Thing* other = context_.resolve(D->getBody());
    arboretum_create_relation(obj, "getBody", other)
  }
  {
    const Thing* other = context_.resolve(D->getBody());
    arboretum_create_relation(obj, "getBody", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitExplicitCastExpr(clang::ExplicitCastExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitExpr(clang::Expr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getValueKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getValueKind", enum_value)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getObjectKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getObjectKind", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getSourceBitField());
    arboretum_create_relation(obj, "getSourceBitField", other)
  }
  {
    const Thing* other = context_.resolve(D->getSourceBitField());
    arboretum_create_relation(obj, "getSourceBitField", other)
  }
  {
    const Thing* other = context_.resolve(D->getReferencedDeclOfCallee());
    arboretum_create_relation(obj, "getReferencedDeclOfCallee", other)
  }
  {
    const Thing* other = context_.resolve(D->getReferencedDeclOfCallee());
    arboretum_create_relation(obj, "getReferencedDeclOfCallee", other)
  }
  {
    const Thing* other = context_.resolve(D->getObjCProperty());
    arboretum_create_relation(obj, "getObjCProperty", other)
  }
  {
    const Thing* other = context_.resolve(D->IgnoreUnlessSpelledInSource());
    arboretum_create_relation(obj, "IgnoreUnlessSpelledInSource", other)
  }
  {
    const Thing* other = context_.resolve(D->IgnoreUnlessSpelledInSource());
    arboretum_create_relation(obj, "IgnoreUnlessSpelledInSource", other)
  }
  {
    const Thing* other = context_.resolve(D->IgnoreImpCasts());
    arboretum_create_relation(obj, "IgnoreImpCasts", other)
  }
  {
    const Thing* other = context_.resolve(D->IgnoreImpCasts());
    arboretum_create_relation(obj, "IgnoreImpCasts", other)
  }
  {
    const Thing* other = context_.resolve(D->IgnoreCasts());
    arboretum_create_relation(obj, "IgnoreCasts", other)
  }
  {
    const Thing* other = context_.resolve(D->IgnoreCasts());
    arboretum_create_relation(obj, "IgnoreCasts", other)
  }
  {
    const Thing* other = context_.resolve(D->IgnoreImplicit());
    arboretum_create_relation(obj, "IgnoreImplicit", other)
  }
  {
    const Thing* other = context_.resolve(D->IgnoreImplicit());
    arboretum_create_relation(obj, "IgnoreImplicit", other)
  }
  {
    const Thing* other = context_.resolve(D->IgnoreImplicitAsWritten());
    arboretum_create_relation(obj, "IgnoreImplicitAsWritten", other)
  }
  {
    const Thing* other = context_.resolve(D->IgnoreImplicitAsWritten());
    arboretum_create_relation(obj, "IgnoreImplicitAsWritten", other)
  }
  {
    const Thing* other = context_.resolve(D->IgnoreParens());
    arboretum_create_relation(obj, "IgnoreParens", other)
  }
  {
    const Thing* other = context_.resolve(D->IgnoreParens());
    arboretum_create_relation(obj, "IgnoreParens", other)
  }
  {
    const Thing* other = context_.resolve(D->IgnoreParenImpCasts());
    arboretum_create_relation(obj, "IgnoreParenImpCasts", other)
  }
  {
    const Thing* other = context_.resolve(D->IgnoreParenImpCasts());
    arboretum_create_relation(obj, "IgnoreParenImpCasts", other)
  }
  {
    const Thing* other = context_.resolve(D->IgnoreParenCasts());
    arboretum_create_relation(obj, "IgnoreParenCasts", other)
  }
  {
    const Thing* other = context_.resolve(D->IgnoreParenCasts());
    arboretum_create_relation(obj, "IgnoreParenCasts", other)
  }
  {
    const Thing* other = context_.resolve(D->IgnoreConversionOperatorSingleStep());
    arboretum_create_relation(obj, "IgnoreConversionOperatorSingleStep", other)
  }
  {
    const Thing* other = context_.resolve(D->IgnoreConversionOperatorSingleStep());
    arboretum_create_relation(obj, "IgnoreConversionOperatorSingleStep", other)
  }
  {
    const Thing* other = context_.resolve(D->IgnoreParenLValueCasts());
    arboretum_create_relation(obj, "IgnoreParenLValueCasts", other)
  }
  {
    const Thing* other = context_.resolve(D->IgnoreParenLValueCasts());
    arboretum_create_relation(obj, "IgnoreParenLValueCasts", other)
  }
  {
    const Thing* other = context_.resolve(D->IgnoreParenBaseCasts());
    arboretum_create_relation(obj, "IgnoreParenBaseCasts", other)
  }
  {
    const Thing* other = context_.resolve(D->IgnoreParenBaseCasts());
    arboretum_create_relation(obj, "IgnoreParenBaseCasts", other)
  }
  {
    const Thing* other = context_.resolve(D->getBestDynamicClassType());
    arboretum_create_relation(obj, "getBestDynamicClassType", other)
  }
  {
    const Thing* other = context_.resolve(D->getBestDynamicClassTypeExpr());
    arboretum_create_relation(obj, "getBestDynamicClassTypeExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->skipRValueSubobjectAdjustments());
    arboretum_create_relation(obj, "skipRValueSubobjectAdjustments", other)
  }
  {
    const Thing* other = context_.resolve(D->skipRValueSubobjectAdjustments());
    arboretum_create_relation(obj, "skipRValueSubobjectAdjustments", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitExprWithCleanups(clang::ExprWithCleanups* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitExpressionTraitExpr(clang::ExpressionTraitExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getTrait());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getTrait", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getQueriedExpression());
    arboretum_create_relation(obj, "getQueriedExpression", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitExtVectorElementExpr(clang::ExtVectorElementExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getBase());
    arboretum_create_relation(obj, "getBase", other)
  }
  {
    const Thing* other = context_.resolve(D->getBase());
    arboretum_create_relation(obj, "getBase", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitFixedPointLiteral(clang::FixedPointLiteral* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitFloatingLiteral(clang::FloatingLiteral* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getRawSemantics());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getRawSemantics", enum_value)
  }
  return true;
}

bool ArboretumASTVisitor::VisitForStmt(clang::ForStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getInit());
    arboretum_create_relation(obj, "getInit", other)
  }
  {
    const Thing* other = context_.resolve(D->getConditionVariable());
    arboretum_create_relation(obj, "getConditionVariable", other)
  }
  {
    const Thing* other = context_.resolve(D->getConditionVariableDeclStmt());
    arboretum_create_relation(obj, "getConditionVariableDeclStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getConditionVariableDeclStmt());
    arboretum_create_relation(obj, "getConditionVariableDeclStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getCond());
    arboretum_create_relation(obj, "getCond", other)
  }
  {
    const Thing* other = context_.resolve(D->getInc());
    arboretum_create_relation(obj, "getInc", other)
  }
  {
    const Thing* other = context_.resolve(D->getBody());
    arboretum_create_relation(obj, "getBody", other)
  }
  {
    const Thing* other = context_.resolve(D->getInit());
    arboretum_create_relation(obj, "getInit", other)
  }
  {
    const Thing* other = context_.resolve(D->getCond());
    arboretum_create_relation(obj, "getCond", other)
  }
  {
    const Thing* other = context_.resolve(D->getInc());
    arboretum_create_relation(obj, "getInc", other)
  }
  {
    const Thing* other = context_.resolve(D->getBody());
    arboretum_create_relation(obj, "getBody", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitFullExpr(clang::FullExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getSubExpr());
    arboretum_create_relation(obj, "getSubExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getSubExpr());
    arboretum_create_relation(obj, "getSubExpr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitFunctionParmPackExpr(clang::FunctionParmPackExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getParameterPack());
    arboretum_create_relation(obj, "getParameterPack", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitGCCAsmStmt(clang::GCCAsmStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getAsmString());
    arboretum_create_relation(obj, "getAsmString", other)
  }
  {
    const Thing* other = context_.resolve(D->getAsmString());
    arboretum_create_relation(obj, "getAsmString", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitGNUNullExpr(clang::GNUNullExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitGenericSelectionExpr(clang::GenericSelectionExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getControllingExpr());
    arboretum_create_relation(obj, "getControllingExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getControllingExpr());
    arboretum_create_relation(obj, "getControllingExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getResultExpr());
    arboretum_create_relation(obj, "getResultExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getResultExpr());
    arboretum_create_relation(obj, "getResultExpr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitGotoStmt(clang::GotoStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getLabel());
    arboretum_create_relation(obj, "getLabel", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitIfStmt(clang::IfStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getCond());
    arboretum_create_relation(obj, "getCond", other)
  }
  {
    const Thing* other = context_.resolve(D->getCond());
    arboretum_create_relation(obj, "getCond", other)
  }
  {
    const Thing* other = context_.resolve(D->getThen());
    arboretum_create_relation(obj, "getThen", other)
  }
  {
    const Thing* other = context_.resolve(D->getThen());
    arboretum_create_relation(obj, "getThen", other)
  }
  {
    const Thing* other = context_.resolve(D->getElse());
    arboretum_create_relation(obj, "getElse", other)
  }
  {
    const Thing* other = context_.resolve(D->getElse());
    arboretum_create_relation(obj, "getElse", other)
  }
  {
    const Thing* other = context_.resolve(D->getConditionVariable());
    arboretum_create_relation(obj, "getConditionVariable", other)
  }
  {
    const Thing* other = context_.resolve(D->getConditionVariable());
    arboretum_create_relation(obj, "getConditionVariable", other)
  }
  {
    const Thing* other = context_.resolve(D->getConditionVariableDeclStmt());
    arboretum_create_relation(obj, "getConditionVariableDeclStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getConditionVariableDeclStmt());
    arboretum_create_relation(obj, "getConditionVariableDeclStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getInit());
    arboretum_create_relation(obj, "getInit", other)
  }
  {
    const Thing* other = context_.resolve(D->getInit());
    arboretum_create_relation(obj, "getInit", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getStatementKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getStatementKind", enum_value)
  }
  return true;
}

bool ArboretumASTVisitor::VisitImaginaryLiteral(clang::ImaginaryLiteral* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getSubExpr());
    arboretum_create_relation(obj, "getSubExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getSubExpr());
    arboretum_create_relation(obj, "getSubExpr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitImplicitCastExpr(clang::ImplicitCastExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitImplicitValueInitExpr(clang::ImplicitValueInitExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitIndirectGotoStmt(clang::IndirectGotoStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getTarget());
    arboretum_create_relation(obj, "getTarget", other)
  }
  {
    const Thing* other = context_.resolve(D->getTarget());
    arboretum_create_relation(obj, "getTarget", other)
  }
  {
    const Thing* other = context_.resolve(D->getConstantTarget());
    arboretum_create_relation(obj, "getConstantTarget", other)
  }
  {
    const Thing* other = context_.resolve(D->getConstantTarget());
    arboretum_create_relation(obj, "getConstantTarget", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitInitListExpr(clang::InitListExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getArrayFiller());
    arboretum_create_relation(obj, "getArrayFiller", other)
  }
  {
    const Thing* other = context_.resolve(D->getArrayFiller());
    arboretum_create_relation(obj, "getArrayFiller", other)
  }
  {
    const Thing* other = context_.resolve(D->getInitializedFieldInUnion());
    arboretum_create_relation(obj, "getInitializedFieldInUnion", other)
  }
  {
    const Thing* other = context_.resolve(D->getInitializedFieldInUnion());
    arboretum_create_relation(obj, "getInitializedFieldInUnion", other)
  }
  {
    const Thing* other = context_.resolve(D->getSemanticForm());
    arboretum_create_relation(obj, "getSemanticForm", other)
  }
  {
    const Thing* other = context_.resolve(D->getSyntacticForm());
    arboretum_create_relation(obj, "getSyntacticForm", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitIntegerLiteral(clang::IntegerLiteral* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitLabelStmt(clang::LabelStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getDecl());
    arboretum_create_relation(obj, "getDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getSubStmt());
    arboretum_create_relation(obj, "getSubStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getSubStmt());
    arboretum_create_relation(obj, "getSubStmt", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitLambdaExpr(clang::LambdaExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getCaptureDefault());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getCaptureDefault", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getLambdaClass());
    arboretum_create_relation(obj, "getLambdaClass", other)
  }
  {
    const Thing* other = context_.resolve(D->getCallOperator());
    arboretum_create_relation(obj, "getCallOperator", other)
  }
  {
    const Thing* other = context_.resolve(D->getDependentCallOperator());
    arboretum_create_relation(obj, "getDependentCallOperator", other)
  }
  {
    const Thing* other = context_.resolve(D->getTrailingRequiresClause());
    arboretum_create_relation(obj, "getTrailingRequiresClause", other)
  }
  {
    const Thing* other = context_.resolve(D->getBody());
    arboretum_create_relation(obj, "getBody", other)
  }
  {
    const Thing* other = context_.resolve(D->getCompoundStmtBody());
    arboretum_create_relation(obj, "getCompoundStmtBody", other)
  }
  {
    const Thing* other = context_.resolve(D->getCompoundStmtBody());
    arboretum_create_relation(obj, "getCompoundStmtBody", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitMSAsmStmt(clang::MSAsmStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitMSDependentExistsStmt(clang::MSDependentExistsStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getSubStmt());
    arboretum_create_relation(obj, "getSubStmt", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitMSPropertyRefExpr(clang::MSPropertyRefExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getBaseExpr());
    arboretum_create_relation(obj, "getBaseExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getPropertyDecl());
    arboretum_create_relation(obj, "getPropertyDecl", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitMSPropertySubscriptExpr(clang::MSPropertySubscriptExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getBase());
    arboretum_create_relation(obj, "getBase", other)
  }
  {
    const Thing* other = context_.resolve(D->getBase());
    arboretum_create_relation(obj, "getBase", other)
  }
  {
    const Thing* other = context_.resolve(D->getIdx());
    arboretum_create_relation(obj, "getIdx", other)
  }
  {
    const Thing* other = context_.resolve(D->getIdx());
    arboretum_create_relation(obj, "getIdx", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitMaterializeTemporaryExpr(clang::MaterializeTemporaryExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getSubExpr());
    arboretum_create_relation(obj, "getSubExpr", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getStorageDuration());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getStorageDuration", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getLifetimeExtendedTemporaryDecl());
    arboretum_create_relation(obj, "getLifetimeExtendedTemporaryDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getLifetimeExtendedTemporaryDecl());
    arboretum_create_relation(obj, "getLifetimeExtendedTemporaryDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getExtendingDecl());
    arboretum_create_relation(obj, "getExtendingDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getExtendingDecl());
    arboretum_create_relation(obj, "getExtendingDecl", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitMatrixSubscriptExpr(clang::MatrixSubscriptExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getBase());
    arboretum_create_relation(obj, "getBase", other)
  }
  {
    const Thing* other = context_.resolve(D->getBase());
    arboretum_create_relation(obj, "getBase", other)
  }
  {
    const Thing* other = context_.resolve(D->getRowIdx());
    arboretum_create_relation(obj, "getRowIdx", other)
  }
  {
    const Thing* other = context_.resolve(D->getRowIdx());
    arboretum_create_relation(obj, "getRowIdx", other)
  }
  {
    const Thing* other = context_.resolve(D->getColumnIdx());
    arboretum_create_relation(obj, "getColumnIdx", other)
  }
  {
    const Thing* other = context_.resolve(D->getColumnIdx());
    arboretum_create_relation(obj, "getColumnIdx", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitMemberExpr(clang::MemberExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getBase());
    arboretum_create_relation(obj, "getBase", other)
  }
  {
    const Thing* other = context_.resolve(D->getMemberDecl());
    arboretum_create_relation(obj, "getMemberDecl", other)
  }
  {
    const Thing* enum_value = context_.data_model_.resolve(D->isNonOdrUse());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "isNonOdrUse", enum_value)
  }
  return true;
}

bool ArboretumASTVisitor::VisitNoInitExpr(clang::NoInitExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitNullStmt(clang::NullStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPArraySectionExpr(clang::OMPArraySectionExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPArrayShapingExpr(clang::OMPArrayShapingExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPAtomicDirective(clang::OMPAtomicDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPBarrierDirective(clang::OMPBarrierDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPCancelDirective(clang::OMPCancelDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPCancellationPointDirective(clang::OMPCancellationPointDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPCanonicalLoop(clang::OMPCanonicalLoop* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPCriticalDirective(clang::OMPCriticalDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPDepobjDirective(clang::OMPDepobjDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPDispatchDirective(clang::OMPDispatchDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPDistributeDirective(clang::OMPDistributeDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPDistributeParallelForDirective(clang::OMPDistributeParallelForDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPDistributeParallelForSimdDirective(clang::OMPDistributeParallelForSimdDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPDistributeSimdDirective(clang::OMPDistributeSimdDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPErrorDirective(clang::OMPErrorDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPExecutableDirective(clang::OMPExecutableDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPFlushDirective(clang::OMPFlushDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPForDirective(clang::OMPForDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPForSimdDirective(clang::OMPForSimdDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPGenericLoopDirective(clang::OMPGenericLoopDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPInteropDirective(clang::OMPInteropDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPIteratorExpr(clang::OMPIteratorExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPLoopBasedDirective(clang::OMPLoopBasedDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPLoopDirective(clang::OMPLoopDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPLoopTransformationDirective(clang::OMPLoopTransformationDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPMaskedDirective(clang::OMPMaskedDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPMaskedTaskLoopDirective(clang::OMPMaskedTaskLoopDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPMaskedTaskLoopSimdDirective(clang::OMPMaskedTaskLoopSimdDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPMasterDirective(clang::OMPMasterDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPMasterTaskLoopDirective(clang::OMPMasterTaskLoopDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPMasterTaskLoopSimdDirective(clang::OMPMasterTaskLoopSimdDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPMetaDirective(clang::OMPMetaDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPOrderedDirective(clang::OMPOrderedDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelDirective(clang::OMPParallelDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelForDirective(clang::OMPParallelForDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelForSimdDirective(clang::OMPParallelForSimdDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelGenericLoopDirective(clang::OMPParallelGenericLoopDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelMaskedDirective(clang::OMPParallelMaskedDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelMaskedTaskLoopDirective(clang::OMPParallelMaskedTaskLoopDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelMaskedTaskLoopSimdDirective(clang::OMPParallelMaskedTaskLoopSimdDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelMasterDirective(clang::OMPParallelMasterDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelMasterTaskLoopDirective(clang::OMPParallelMasterTaskLoopDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelMasterTaskLoopSimdDirective(clang::OMPParallelMasterTaskLoopSimdDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPParallelSectionsDirective(clang::OMPParallelSectionsDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPScanDirective(clang::OMPScanDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPSectionDirective(clang::OMPSectionDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPSectionsDirective(clang::OMPSectionsDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPSimdDirective(clang::OMPSimdDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPSingleDirective(clang::OMPSingleDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetDataDirective(clang::OMPTargetDataDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetDirective(clang::OMPTargetDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetEnterDataDirective(clang::OMPTargetEnterDataDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetExitDataDirective(clang::OMPTargetExitDataDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetParallelDirective(clang::OMPTargetParallelDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetParallelForDirective(clang::OMPTargetParallelForDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetParallelForSimdDirective(clang::OMPTargetParallelForSimdDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetParallelGenericLoopDirective(clang::OMPTargetParallelGenericLoopDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetSimdDirective(clang::OMPTargetSimdDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetTeamsDirective(clang::OMPTargetTeamsDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetTeamsDistributeDirective(clang::OMPTargetTeamsDistributeDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetTeamsDistributeParallelForDirective(clang::OMPTargetTeamsDistributeParallelForDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetTeamsDistributeParallelForSimdDirective(clang::OMPTargetTeamsDistributeParallelForSimdDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetTeamsDistributeSimdDirective(clang::OMPTargetTeamsDistributeSimdDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetTeamsGenericLoopDirective(clang::OMPTargetTeamsGenericLoopDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTargetUpdateDirective(clang::OMPTargetUpdateDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTaskDirective(clang::OMPTaskDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTaskLoopDirective(clang::OMPTaskLoopDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTaskLoopSimdDirective(clang::OMPTaskLoopSimdDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTaskgroupDirective(clang::OMPTaskgroupDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTaskwaitDirective(clang::OMPTaskwaitDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTaskyieldDirective(clang::OMPTaskyieldDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTeamsDirective(clang::OMPTeamsDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTeamsDistributeDirective(clang::OMPTeamsDistributeDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTeamsDistributeParallelForDirective(clang::OMPTeamsDistributeParallelForDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTeamsDistributeParallelForSimdDirective(clang::OMPTeamsDistributeParallelForSimdDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTeamsDistributeSimdDirective(clang::OMPTeamsDistributeSimdDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTeamsGenericLoopDirective(clang::OMPTeamsGenericLoopDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPTileDirective(clang::OMPTileDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOMPUnrollDirective(clang::OMPUnrollDirective* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCArrayLiteral(clang::ObjCArrayLiteral* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCAtCatchStmt(clang::ObjCAtCatchStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCAtFinallyStmt(clang::ObjCAtFinallyStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCAtSynchronizedStmt(clang::ObjCAtSynchronizedStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCAtThrowStmt(clang::ObjCAtThrowStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCAtTryStmt(clang::ObjCAtTryStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCAutoreleasePoolStmt(clang::ObjCAutoreleasePoolStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCAvailabilityCheckExpr(clang::ObjCAvailabilityCheckExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCBoolLiteralExpr(clang::ObjCBoolLiteralExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCBoxedExpr(clang::ObjCBoxedExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCBridgedCastExpr(clang::ObjCBridgedCastExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCDictionaryLiteral(clang::ObjCDictionaryLiteral* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCEncodeExpr(clang::ObjCEncodeExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCForCollectionStmt(clang::ObjCForCollectionStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCIndirectCopyRestoreExpr(clang::ObjCIndirectCopyRestoreExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCIsaExpr(clang::ObjCIsaExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCIvarRefExpr(clang::ObjCIvarRefExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCMessageExpr(clang::ObjCMessageExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCPropertyRefExpr(clang::ObjCPropertyRefExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCProtocolExpr(clang::ObjCProtocolExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCSelectorExpr(clang::ObjCSelectorExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCStringLiteral(clang::ObjCStringLiteral* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitObjCSubscriptRefExpr(clang::ObjCSubscriptRefExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOffsetOfExpr(clang::OffsetOfExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitOpaqueValueExpr(clang::OpaqueValueExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getSourceExpr());
    arboretum_create_relation(obj, "getSourceExpr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitOverloadExpr(clang::OverloadExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getNamingClass());
    arboretum_create_relation(obj, "getNamingClass", other)
  }
  {
    const Thing* other = context_.resolve(D->getNamingClass());
    arboretum_create_relation(obj, "getNamingClass", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitPackExpansionExpr(clang::PackExpansionExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getPattern());
    arboretum_create_relation(obj, "getPattern", other)
  }
  {
    const Thing* other = context_.resolve(D->getPattern());
    arboretum_create_relation(obj, "getPattern", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitParenExpr(clang::ParenExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getSubExpr());
    arboretum_create_relation(obj, "getSubExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getSubExpr());
    arboretum_create_relation(obj, "getSubExpr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitParenListExpr(clang::ParenListExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitPredefinedExpr(clang::PredefinedExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getIdentKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getIdentKind", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getFunctionName());
    arboretum_create_relation(obj, "getFunctionName", other)
  }
  {
    const Thing* other = context_.resolve(D->getFunctionName());
    arboretum_create_relation(obj, "getFunctionName", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitPseudoObjectExpr(clang::PseudoObjectExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getSyntacticForm());
    arboretum_create_relation(obj, "getSyntacticForm", other)
  }
  {
    const Thing* other = context_.resolve(D->getSyntacticForm());
    arboretum_create_relation(obj, "getSyntacticForm", other)
  }
  {
    const Thing* other = context_.resolve(D->getResultExpr());
    arboretum_create_relation(obj, "getResultExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getResultExpr());
    arboretum_create_relation(obj, "getResultExpr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitRecoveryExpr(clang::RecoveryExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitRequiresExpr(clang::RequiresExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getBody());
    arboretum_create_relation(obj, "getBody", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitReturnStmt(clang::ReturnStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getRetValue());
    arboretum_create_relation(obj, "getRetValue", other)
  }
  {
    const Thing* other = context_.resolve(D->getRetValue());
    arboretum_create_relation(obj, "getRetValue", other)
  }
  {
    const Thing* other = context_.resolve(D->getNRVOCandidate());
    arboretum_create_relation(obj, "getNRVOCandidate", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitSEHExceptStmt(clang::SEHExceptStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getFilterExpr());
    arboretum_create_relation(obj, "getFilterExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getBlock());
    arboretum_create_relation(obj, "getBlock", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitSEHFinallyStmt(clang::SEHFinallyStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getBlock());
    arboretum_create_relation(obj, "getBlock", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitSEHLeaveStmt(clang::SEHLeaveStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitSEHTryStmt(clang::SEHTryStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getTryBlock());
    arboretum_create_relation(obj, "getTryBlock", other)
  }
  {
    const Thing* other = context_.resolve(D->getHandler());
    arboretum_create_relation(obj, "getHandler", other)
  }
  {
    const Thing* other = context_.resolve(D->getExceptHandler());
    arboretum_create_relation(obj, "getExceptHandler", other)
  }
  {
    const Thing* other = context_.resolve(D->getFinallyHandler());
    arboretum_create_relation(obj, "getFinallyHandler", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitSYCLUniqueStableNameExpr(clang::SYCLUniqueStableNameExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitShuffleVectorExpr(clang::ShuffleVectorExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitSizeOfPackExpr(clang::SizeOfPackExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getPack());
    arboretum_create_relation(obj, "getPack", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitSourceLocExpr(clang::SourceLocExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getIdentKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getIdentKind", enum_value)
  }
  return true;
}

bool ArboretumASTVisitor::VisitStmt(clang::Stmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getStmtClass());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getStmtClass", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->stripLabelLikeStatements());
    arboretum_create_relation(obj, "stripLabelLikeStatements", other)
  }
  {
    const Thing* other = context_.resolve(D->stripLabelLikeStatements());
    arboretum_create_relation(obj, "stripLabelLikeStatements", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitStmtExpr(clang::StmtExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getSubStmt());
    arboretum_create_relation(obj, "getSubStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getSubStmt());
    arboretum_create_relation(obj, "getSubStmt", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitStringLiteral(clang::StringLiteral* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getKind", enum_value)
  }
  return true;
}

bool ArboretumASTVisitor::VisitSubstNonTypeTemplateParmExpr(clang::SubstNonTypeTemplateParmExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getReplacement());
    arboretum_create_relation(obj, "getReplacement", other)
  }
  {
    const Thing* other = context_.resolve(D->getAssociatedDecl());
    arboretum_create_relation(obj, "getAssociatedDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getParameter());
    arboretum_create_relation(obj, "getParameter", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitSubstNonTypeTemplateParmPackExpr(clang::SubstNonTypeTemplateParmPackExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getAssociatedDecl());
    arboretum_create_relation(obj, "getAssociatedDecl", other)
  }
  {
    const Thing* other = context_.resolve(D->getParameterPack());
    arboretum_create_relation(obj, "getParameterPack", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitSwitchCase(clang::SwitchCase* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getNextSwitchCase());
    arboretum_create_relation(obj, "getNextSwitchCase", other)
  }
  {
    const Thing* other = context_.resolve(D->getNextSwitchCase());
    arboretum_create_relation(obj, "getNextSwitchCase", other)
  }
  {
    const Thing* other = context_.resolve(D->getSubStmt());
    arboretum_create_relation(obj, "getSubStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getSubStmt());
    arboretum_create_relation(obj, "getSubStmt", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitSwitchStmt(clang::SwitchStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getCond());
    arboretum_create_relation(obj, "getCond", other)
  }
  {
    const Thing* other = context_.resolve(D->getCond());
    arboretum_create_relation(obj, "getCond", other)
  }
  {
    const Thing* other = context_.resolve(D->getBody());
    arboretum_create_relation(obj, "getBody", other)
  }
  {
    const Thing* other = context_.resolve(D->getBody());
    arboretum_create_relation(obj, "getBody", other)
  }
  {
    const Thing* other = context_.resolve(D->getInit());
    arboretum_create_relation(obj, "getInit", other)
  }
  {
    const Thing* other = context_.resolve(D->getInit());
    arboretum_create_relation(obj, "getInit", other)
  }
  {
    const Thing* other = context_.resolve(D->getConditionVariable());
    arboretum_create_relation(obj, "getConditionVariable", other)
  }
  {
    const Thing* other = context_.resolve(D->getConditionVariable());
    arboretum_create_relation(obj, "getConditionVariable", other)
  }
  {
    const Thing* other = context_.resolve(D->getConditionVariableDeclStmt());
    arboretum_create_relation(obj, "getConditionVariableDeclStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getConditionVariableDeclStmt());
    arboretum_create_relation(obj, "getConditionVariableDeclStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getSwitchCaseList());
    arboretum_create_relation(obj, "getSwitchCaseList", other)
  }
  {
    const Thing* other = context_.resolve(D->getSwitchCaseList());
    arboretum_create_relation(obj, "getSwitchCaseList", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitTypeTraitExpr(clang::TypeTraitExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getTrait());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getTrait", enum_value)
  }
  return true;
}

bool ArboretumASTVisitor::VisitTypoExpr(clang::TypoExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  return true;
}

bool ArboretumASTVisitor::VisitUnaryExprOrTypeTraitExpr(clang::UnaryExprOrTypeTraitExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getKind", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getArgumentExpr());
    arboretum_create_relation(obj, "getArgumentExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getArgumentExpr());
    arboretum_create_relation(obj, "getArgumentExpr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitUnaryOperator(clang::UnaryOperator* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getOpcode());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getOpcode", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getSubExpr());
    arboretum_create_relation(obj, "getSubExpr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedLookupExpr(clang::UnresolvedLookupExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getNamingClass());
    arboretum_create_relation(obj, "getNamingClass", other)
  }
  {
    const Thing* other = context_.resolve(D->getNamingClass());
    arboretum_create_relation(obj, "getNamingClass", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitUnresolvedMemberExpr(clang::UnresolvedMemberExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getBase());
    arboretum_create_relation(obj, "getBase", other)
  }
  {
    const Thing* other = context_.resolve(D->getBase());
    arboretum_create_relation(obj, "getBase", other)
  }
  {
    const Thing* other = context_.resolve(D->getNamingClass());
    arboretum_create_relation(obj, "getNamingClass", other)
  }
  {
    const Thing* other = context_.resolve(D->getNamingClass());
    arboretum_create_relation(obj, "getNamingClass", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitUserDefinedLiteral(clang::UserDefinedLiteral* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* enum_value = context_.data_model_.resolve(D->getLiteralOperatorKind());
    assert(enum_value != nullptr);
    arboretum_create_relation(obj, "getLiteralOperatorKind", enum_value)
  }
  {
    const Thing* other = context_.resolve(D->getCookedLiteral());
    arboretum_create_relation(obj, "getCookedLiteral", other)
  }
  {
    const Thing* other = context_.resolve(D->getCookedLiteral());
    arboretum_create_relation(obj, "getCookedLiteral", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitVAArgExpr(clang::VAArgExpr* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getSubExpr());
    arboretum_create_relation(obj, "getSubExpr", other)
  }
  {
    const Thing* other = context_.resolve(D->getSubExpr());
    arboretum_create_relation(obj, "getSubExpr", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitValueStmt(clang::ValueStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getExprStmt());
    arboretum_create_relation(obj, "getExprStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getExprStmt());
    arboretum_create_relation(obj, "getExprStmt", other)
  }
  return true;
}

bool ArboretumASTVisitor::VisitWhileStmt(clang::WhileStmt* D) {
  ObjectBuilder* builder = arboretum_object_builder_new();
  const Thing* obj = context_.resolve(D);
  {
    const Thing* other = context_.resolve(D->getCond());
    arboretum_create_relation(obj, "getCond", other)
  }
  {
    const Thing* other = context_.resolve(D->getCond());
    arboretum_create_relation(obj, "getCond", other)
  }
  {
    const Thing* other = context_.resolve(D->getBody());
    arboretum_create_relation(obj, "getBody", other)
  }
  {
    const Thing* other = context_.resolve(D->getBody());
    arboretum_create_relation(obj, "getBody", other)
  }
  {
    const Thing* other = context_.resolve(D->getConditionVariable());
    arboretum_create_relation(obj, "getConditionVariable", other)
  }
  {
    const Thing* other = context_.resolve(D->getConditionVariable());
    arboretum_create_relation(obj, "getConditionVariable", other)
  }
  {
    const Thing* other = context_.resolve(D->getConditionVariableDeclStmt());
    arboretum_create_relation(obj, "getConditionVariableDeclStmt", other)
  }
  {
    const Thing* other = context_.resolve(D->getConditionVariableDeclStmt());
    arboretum_create_relation(obj, "getConditionVariableDeclStmt", other)
  }
  return true;
}

////   END ARBORETUM GENERATED CODE ////
}  // namespace arboretum