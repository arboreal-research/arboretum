use juniper::*;


#[derive(GraphQLObject)]
pub struct TypeOfExprType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DependentTypeOfExprType {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum TypeOfExprTypeNode {
  TypeOfExprType(TypeOfExprType),
  DependentTypeOfExprType(DependentTypeOfExprType),
}

