use juniper::*;

pub mod adjusted_type_node;
pub mod array_type_node;
pub mod decltype_type_node;
pub mod deduced_type_node;
pub mod function_type_node;
pub mod matrix_type_node;
pub mod obj_c_object_type_node;
pub mod reference_type_node;
pub mod tag_type_node;
pub mod type_of_expr_type_node;
pub mod type_with_keyword_node;
pub mod unary_transform_type_node;
pub mod vector_type_node;

#[derive(GraphQLObject)]
pub struct Type {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct BuiltinType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ComplexType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ParenType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct PointerType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct BlockPointerType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MemberPointerType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DependentAddressSpaceType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DependentSizedExtVectorType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DependentVectorType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct UnresolvedUsingType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct UsingType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct TypedefType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MacroQualifiedType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct TypeOfType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AttributedType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct BTFTagAttributedType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct TemplateTypeParmType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SubstTemplateTypeParmType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SubstTemplateTypeParmPackType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct TemplateSpecializationType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct InjectedClassNameType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct PackExpansionType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCTypeParamType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCObjectPointerType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AtomicType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct PipeType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct BitIntType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DependentBitIntType {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum TypeNode {
  Type(Type),
  AdjustedType(adjusted_type_node::AdjustedType),
  ArrayType(array_type_node::ArrayType),
  AtomicType(AtomicType),
  AttributedType(AttributedType),
  AutoType(deduced_type_node::AutoType),
  BTFTagAttributedType(BTFTagAttributedType),
  BitIntType(BitIntType),
  BlockPointerType(BlockPointerType),
  BuiltinType(BuiltinType),
  ComplexType(ComplexType),
  ConstantArrayType(array_type_node::ConstantArrayType),
  ConstantMatrixType(matrix_type_node::ConstantMatrixType),
  DecayedType(adjusted_type_node::DecayedType),
  DecltypeType(decltype_type_node::DecltypeType),
  DeducedTemplateSpecializationType(deduced_type_node::DeducedTemplateSpecializationType),
  DeducedType(deduced_type_node::DeducedType),
  DependentAddressSpaceType(DependentAddressSpaceType),
  DependentBitIntType(DependentBitIntType),
  DependentDecltypeType(decltype_type_node::DependentDecltypeType),
  DependentNameType(type_with_keyword_node::DependentNameType),
  DependentSizedArrayType(array_type_node::DependentSizedArrayType),
  DependentSizedExtVectorType(DependentSizedExtVectorType),
  DependentSizedMatrixType(matrix_type_node::DependentSizedMatrixType),
  DependentTemplateSpecializationType(type_with_keyword_node::DependentTemplateSpecializationType),
  DependentTypeOfExprType(type_of_expr_type_node::DependentTypeOfExprType),
  DependentUnaryTransformType(unary_transform_type_node::DependentUnaryTransformType),
  DependentVectorType(DependentVectorType),
  ElaboratedType(type_with_keyword_node::ElaboratedType),
  EnumType(tag_type_node::EnumType),
  ExtVectorType(vector_type_node::ExtVectorType),
  FunctionNoProtoType(function_type_node::FunctionNoProtoType),
  FunctionProtoType(function_type_node::FunctionProtoType),
  FunctionType(function_type_node::FunctionType),
  IncompleteArrayType(array_type_node::IncompleteArrayType),
  InjectedClassNameType(InjectedClassNameType),
  LValueReferenceType(reference_type_node::LValueReferenceType),
  MacroQualifiedType(MacroQualifiedType),
  MatrixType(matrix_type_node::MatrixType),
  MemberPointerType(MemberPointerType),
  ObjCInterfaceType(obj_c_object_type_node::ObjCInterfaceType),
  ObjCObjectPointerType(ObjCObjectPointerType),
  ObjCObjectType(obj_c_object_type_node::ObjCObjectType),
  ObjCObjectTypeImpl(obj_c_object_type_node::ObjCObjectTypeImpl),
  ObjCTypeParamType(ObjCTypeParamType),
  PackExpansionType(PackExpansionType),
  ParenType(ParenType),
  PipeType(PipeType),
  PointerType(PointerType),
  RValueReferenceType(reference_type_node::RValueReferenceType),
  RecordType(tag_type_node::RecordType),
  ReferenceType(reference_type_node::ReferenceType),
  SubstTemplateTypeParmPackType(SubstTemplateTypeParmPackType),
  SubstTemplateTypeParmType(SubstTemplateTypeParmType),
  TagType(tag_type_node::TagType),
  TemplateSpecializationType(TemplateSpecializationType),
  TemplateTypeParmType(TemplateTypeParmType),
  TypeOfExprType(type_of_expr_type_node::TypeOfExprType),
  TypeOfType(TypeOfType),
  TypeWithKeyword(type_with_keyword_node::TypeWithKeyword),
  TypedefType(TypedefType),
  UnaryTransformType(unary_transform_type_node::UnaryTransformType),
  UnresolvedUsingType(UnresolvedUsingType),
  UsingType(UsingType),
  VariableArrayType(array_type_node::VariableArrayType),
  VectorType(vector_type_node::VectorType),
}

