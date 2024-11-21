use juniper::*;


#[derive(GraphQLObject)]
pub struct UsingShadowDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ConstructorUsingShadowDecl {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum UsingShadowDeclNode {
  UsingShadowDecl(UsingShadowDecl),
  ConstructorUsingShadowDecl(ConstructorUsingShadowDecl),
}

