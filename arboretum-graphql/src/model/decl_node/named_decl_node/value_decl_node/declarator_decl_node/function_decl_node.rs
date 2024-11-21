use juniper::*;

pub mod cxx_method_decl_node;

#[derive(GraphQLObject)]
pub struct FunctionDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CXXDeductionGuideDecl {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum FunctionDeclNode {
  FunctionDecl(FunctionDecl),
  CXXConstructorDecl(cxx_method_decl_node::CXXConstructorDecl),
  CXXConversionDecl(cxx_method_decl_node::CXXConversionDecl),
  CXXDeductionGuideDecl(CXXDeductionGuideDecl),
  CXXDestructorDecl(cxx_method_decl_node::CXXDestructorDecl),
  CXXMethodDecl(cxx_method_decl_node::CXXMethodDecl),
}

