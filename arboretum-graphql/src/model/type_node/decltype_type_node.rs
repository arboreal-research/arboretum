use juniper::*;


#[derive(GraphQLObject)]
pub struct DecltypeType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DependentDecltypeType {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum DecltypeTypeNode {
  DecltypeType(DecltypeType),
  DependentDecltypeType(DependentDecltypeType),
}

