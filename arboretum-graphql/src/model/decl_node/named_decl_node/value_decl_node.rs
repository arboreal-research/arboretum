use juniper::*;

pub mod declarator_decl_node;

#[derive(GraphQLObject)]
pub struct ValueDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct EnumConstantDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct IndirectFieldDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct UnresolvedUsingValueDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct BindingDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MSGuidDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct UnnamedGlobalConstantDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct TemplateParamObjectDecl {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum ValueDeclNode {
  ValueDecl(ValueDecl),
  BindingDecl(BindingDecl),
  CXXConstructorDecl(declarator_decl_node::function_decl_node::cxx_method_decl_node::CXXConstructorDecl),
  CXXConversionDecl(declarator_decl_node::function_decl_node::cxx_method_decl_node::CXXConversionDecl),
  CXXDeductionGuideDecl(declarator_decl_node::function_decl_node::CXXDeductionGuideDecl),
  CXXDestructorDecl(declarator_decl_node::function_decl_node::cxx_method_decl_node::CXXDestructorDecl),
  CXXMethodDecl(declarator_decl_node::function_decl_node::cxx_method_decl_node::CXXMethodDecl),
  DeclaratorDecl(declarator_decl_node::DeclaratorDecl),
  DecompositionDecl(declarator_decl_node::var_decl_node::DecompositionDecl),
  EnumConstantDecl(EnumConstantDecl),
  FieldDecl(declarator_decl_node::field_decl_node::FieldDecl),
  FunctionDecl(declarator_decl_node::function_decl_node::FunctionDecl),
  ImplicitParamDecl(declarator_decl_node::var_decl_node::ImplicitParamDecl),
  IndirectFieldDecl(IndirectFieldDecl),
  MSGuidDecl(MSGuidDecl),
  MSPropertyDecl(declarator_decl_node::MSPropertyDecl),
  NonTypeTemplateParmDecl(declarator_decl_node::NonTypeTemplateParmDecl),
  ObjCAtDefsFieldDecl(declarator_decl_node::field_decl_node::ObjCAtDefsFieldDecl),
  ObjCIvarDecl(declarator_decl_node::field_decl_node::ObjCIvarDecl),
  ParmVarDecl(declarator_decl_node::var_decl_node::ParmVarDecl),
  TemplateParamObjectDecl(TemplateParamObjectDecl),
  UnnamedGlobalConstantDecl(UnnamedGlobalConstantDecl),
  UnresolvedUsingValueDecl(UnresolvedUsingValueDecl),
  VarDecl(declarator_decl_node::var_decl_node::VarDecl),
  VarTemplatePartialSpecializationDecl(declarator_decl_node::var_decl_node::var_template_specialization_decl_node::VarTemplatePartialSpecializationDecl),
  VarTemplateSpecializationDecl(declarator_decl_node::var_decl_node::var_template_specialization_decl_node::VarTemplateSpecializationDecl),
}

