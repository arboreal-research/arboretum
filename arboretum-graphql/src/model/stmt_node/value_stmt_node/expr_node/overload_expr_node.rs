use juniper::*;


#[derive(GraphQLObject)]
pub struct OverloadExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct UnresolvedLookupExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct UnresolvedMemberExpr {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum OverloadExprNode {
  OverloadExpr(OverloadExpr),
  UnresolvedLookupExpr(UnresolvedLookupExpr),
  UnresolvedMemberExpr(UnresolvedMemberExpr),
}

