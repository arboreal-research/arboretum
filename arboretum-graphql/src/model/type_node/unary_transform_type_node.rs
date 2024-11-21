use juniper::*;


#[derive(GraphQLObject)]
pub struct UnaryTransformType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DependentUnaryTransformType {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum UnaryTransformTypeNode {
  UnaryTransformType(UnaryTransformType),
  DependentUnaryTransformType(DependentUnaryTransformType),
}

