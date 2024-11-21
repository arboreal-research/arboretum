use juniper::*;


#[derive(GraphQLObject)]
pub struct MatrixType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ConstantMatrixType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DependentSizedMatrixType {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum MatrixTypeNode {
  MatrixType(MatrixType),
  ConstantMatrixType(ConstantMatrixType),
  DependentSizedMatrixType(DependentSizedMatrixType),
}

