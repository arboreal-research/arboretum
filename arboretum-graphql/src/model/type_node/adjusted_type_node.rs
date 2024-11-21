use juniper::*;


#[derive(GraphQLObject)]
pub struct AdjustedType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DecayedType {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum AdjustedTypeNode {
  AdjustedType(AdjustedType),
  DecayedType(DecayedType),
}

