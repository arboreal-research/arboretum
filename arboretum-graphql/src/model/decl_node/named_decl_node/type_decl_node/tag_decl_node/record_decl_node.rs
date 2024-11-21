use juniper::*;

pub mod cxx_record_decl_node;

#[derive(GraphQLObject)]
pub struct RecordDecl {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum RecordDeclNode {
  RecordDecl(RecordDecl),
  CXXRecordDecl(cxx_record_decl_node::CXXRecordDecl),
  ClassTemplatePartialSpecializationDecl(cxx_record_decl_node::class_template_specialization_decl_node::ClassTemplatePartialSpecializationDecl),
  ClassTemplateSpecializationDecl(cxx_record_decl_node::class_template_specialization_decl_node::ClassTemplateSpecializationDecl),
}

