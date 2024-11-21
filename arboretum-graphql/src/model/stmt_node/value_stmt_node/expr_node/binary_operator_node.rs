use juniper::*;


#[derive(GraphQLObject)]
pub struct BinaryOperator {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CompoundAssignOperator {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum BinaryOperatorNode {
  BinaryOperator(BinaryOperator),
  CompoundAssignOperator(CompoundAssignOperator),
}

