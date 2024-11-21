use juniper::*;

pub mod record_decl_node;

#[derive(GraphQLObject)]
pub struct TagDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct EnumDecl {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum TagDeclNode {
  TagDecl(TagDecl),
  CXXRecordDecl(record_decl_node::cxx_record_decl_node::CXXRecordDecl),
  ClassTemplatePartialSpecializationDecl(record_decl_node::cxx_record_decl_node::class_template_specialization_decl_node::ClassTemplatePartialSpecializationDecl),
  ClassTemplateSpecializationDecl(record_decl_node::cxx_record_decl_node::class_template_specialization_decl_node::ClassTemplateSpecializationDecl),
  EnumDecl(EnumDecl),
  RecordDecl(record_decl_node::RecordDecl),
}

