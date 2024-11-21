use juniper::*;

pub mod field_decl_node;
pub mod function_decl_node;
pub mod var_decl_node;

#[derive(GraphQLObject)]
pub struct DeclaratorDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MSPropertyDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NonTypeTemplateParmDecl {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum DeclaratorDeclNode {
  DeclaratorDecl(DeclaratorDecl),
  CXXConstructorDecl(function_decl_node::cxx_method_decl_node::CXXConstructorDecl),
  CXXConversionDecl(function_decl_node::cxx_method_decl_node::CXXConversionDecl),
  CXXDeductionGuideDecl(function_decl_node::CXXDeductionGuideDecl),
  CXXDestructorDecl(function_decl_node::cxx_method_decl_node::CXXDestructorDecl),
  CXXMethodDecl(function_decl_node::cxx_method_decl_node::CXXMethodDecl),
  DecompositionDecl(var_decl_node::DecompositionDecl),
  FieldDecl(field_decl_node::FieldDecl),
  FunctionDecl(function_decl_node::FunctionDecl),
  ImplicitParamDecl(var_decl_node::ImplicitParamDecl),
  MSPropertyDecl(MSPropertyDecl),
  NonTypeTemplateParmDecl(NonTypeTemplateParmDecl),
  ObjCAtDefsFieldDecl(field_decl_node::ObjCAtDefsFieldDecl),
  ObjCIvarDecl(field_decl_node::ObjCIvarDecl),
  ParmVarDecl(var_decl_node::ParmVarDecl),
  VarDecl(var_decl_node::VarDecl),
  VarTemplatePartialSpecializationDecl(var_decl_node::var_template_specialization_decl_node::VarTemplatePartialSpecializationDecl),
  VarTemplateSpecializationDecl(var_decl_node::var_template_specialization_decl_node::VarTemplateSpecializationDecl),
}

