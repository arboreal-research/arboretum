use juniper::*;

pub mod redeclarable_template_decl_node;

#[derive(GraphQLObject)]
pub struct TemplateDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct TemplateTemplateParmDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct BuiltinTemplateDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ConceptDecl {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum TemplateDeclNode {
  TemplateDecl(TemplateDecl),
  BuiltinTemplateDecl(BuiltinTemplateDecl),
  ClassTemplateDecl(redeclarable_template_decl_node::ClassTemplateDecl),
  ConceptDecl(ConceptDecl),
  FunctionTemplateDecl(redeclarable_template_decl_node::FunctionTemplateDecl),
  RedeclarableTemplateDecl(redeclarable_template_decl_node::RedeclarableTemplateDecl),
  TemplateTemplateParmDecl(TemplateTemplateParmDecl),
  TypeAliasTemplateDecl(redeclarable_template_decl_node::TypeAliasTemplateDecl),
  VarTemplateDecl(redeclarable_template_decl_node::VarTemplateDecl),
}

