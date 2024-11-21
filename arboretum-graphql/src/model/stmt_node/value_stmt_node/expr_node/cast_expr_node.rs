use juniper::*;

pub mod explicit_cast_expr_node;

#[derive(GraphQLObject)]
pub struct CastExpr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ImplicitCastExpr {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum CastExprNode {
  CastExpr(CastExpr),
  BuiltinBitCastExpr(explicit_cast_expr_node::BuiltinBitCastExpr),
  CStyleCastExpr(explicit_cast_expr_node::CStyleCastExpr),
  CXXAddrspaceCastExpr(explicit_cast_expr_node::cxx_named_cast_expr_node::CXXAddrspaceCastExpr),
  CXXConstCastExpr(explicit_cast_expr_node::cxx_named_cast_expr_node::CXXConstCastExpr),
  CXXDynamicCastExpr(explicit_cast_expr_node::cxx_named_cast_expr_node::CXXDynamicCastExpr),
  CXXFunctionalCastExpr(explicit_cast_expr_node::CXXFunctionalCastExpr),
  CXXNamedCastExpr(explicit_cast_expr_node::cxx_named_cast_expr_node::CXXNamedCastExpr),
  CXXReinterpretCastExpr(explicit_cast_expr_node::cxx_named_cast_expr_node::CXXReinterpretCastExpr),
  CXXStaticCastExpr(explicit_cast_expr_node::cxx_named_cast_expr_node::CXXStaticCastExpr),
  ExplicitCastExpr(explicit_cast_expr_node::ExplicitCastExpr),
  ImplicitCastExpr(ImplicitCastExpr),
  ObjCBridgedCastExpr(explicit_cast_expr_node::ObjCBridgedCastExpr),
}

