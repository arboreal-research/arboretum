use juniper::*;


#[derive(GraphQLObject)]
pub struct BaseUsingDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct UsingDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct UsingEnumDecl {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum BaseUsingDeclNode {
  BaseUsingDecl(BaseUsingDecl),
  UsingDecl(UsingDecl),
  UsingEnumDecl(UsingEnumDecl),
}

