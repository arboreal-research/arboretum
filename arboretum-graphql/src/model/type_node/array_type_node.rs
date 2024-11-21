use juniper::*;


#[derive(GraphQLObject)]
pub struct ArrayType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ConstantArrayType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct IncompleteArrayType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct VariableArrayType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DependentSizedArrayType {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum ArrayTypeNode {
  ArrayType(ArrayType),
  ConstantArrayType(ConstantArrayType),
  DependentSizedArrayType(DependentSizedArrayType),
  IncompleteArrayType(IncompleteArrayType),
  VariableArrayType(VariableArrayType),
}

