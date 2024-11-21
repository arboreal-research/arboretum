use juniper::*;


#[derive(GraphQLObject)]
pub struct CoroutineSuspendExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CoawaitExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CoyieldExpr {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum CoroutineSuspendExprNode {
  CoroutineSuspendExpr(CoroutineSuspendExpr),
  CoawaitExpr(CoawaitExpr),
  CoyieldExpr(CoyieldExpr),
}

