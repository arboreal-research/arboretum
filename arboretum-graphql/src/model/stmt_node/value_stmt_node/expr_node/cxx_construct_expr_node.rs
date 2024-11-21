use juniper::*;


#[derive(GraphQLObject)]
pub struct CXXConstructExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CXXTemporaryObjectExpr {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum CXXConstructExprNode {
  CXXConstructExpr(CXXConstructExpr),
  CXXTemporaryObjectExpr(CXXTemporaryObjectExpr),
}

