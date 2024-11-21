use juniper::*;


#[derive(GraphQLObject)]
pub struct AbstractConditionalOperator {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ConditionalOperator {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct BinaryConditionalOperator {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum AbstractConditionalOperatorNode {
  AbstractConditionalOperator(AbstractConditionalOperator),
  BinaryConditionalOperator(BinaryConditionalOperator),
  ConditionalOperator(ConditionalOperator),
}

