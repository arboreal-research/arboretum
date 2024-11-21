use juniper::*;


#[derive(GraphQLObject)]
pub struct FieldDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCIvarDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCAtDefsFieldDecl {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum FieldDeclNode {
  FieldDecl(FieldDecl),
  ObjCAtDefsFieldDecl(ObjCAtDefsFieldDecl),
  ObjCIvarDecl(ObjCIvarDecl),
}

