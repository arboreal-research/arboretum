use juniper::*;


#[derive(GraphQLObject)]
pub struct VectorType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ExtVectorType {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum VectorTypeNode {
  VectorType(VectorType),
  ExtVectorType(ExtVectorType),
}

