use juniper::*;

pub mod obj_c_impl_decl_node;

#[derive(GraphQLObject)]
pub struct ObjCContainerDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCInterfaceDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCProtocolDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCCategoryDecl {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum ObjCContainerDeclNode {
  ObjCContainerDecl(ObjCContainerDecl),
  ObjCCategoryDecl(ObjCCategoryDecl),
  ObjCCategoryImplDecl(obj_c_impl_decl_node::ObjCCategoryImplDecl),
  ObjCImplDecl(obj_c_impl_decl_node::ObjCImplDecl),
  ObjCImplementationDecl(obj_c_impl_decl_node::ObjCImplementationDecl),
  ObjCInterfaceDecl(ObjCInterfaceDecl),
  ObjCProtocolDecl(ObjCProtocolDecl),
}

