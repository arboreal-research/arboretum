use juniper::*;


#[derive(GraphQLObject)]
pub struct ReferenceType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct LValueReferenceType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct RValueReferenceType {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum ReferenceTypeNode {
  ReferenceType(ReferenceType),
  LValueReferenceType(LValueReferenceType),
  RValueReferenceType(RValueReferenceType),
}

