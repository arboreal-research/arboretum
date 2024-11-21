use juniper::*;

pub mod tag_decl_node;
pub mod typedef_name_decl_node;

#[derive(GraphQLObject)]
pub struct TypeDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct UnresolvedUsingTypenameDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct TemplateTypeParmDecl {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum TypeDeclNode {
  TypeDecl(TypeDecl),
  CXXRecordDecl(tag_decl_node::record_decl_node::cxx_record_decl_node::CXXRecordDecl),
  ClassTemplatePartialSpecializationDecl(tag_decl_node::record_decl_node::cxx_record_decl_node::class_template_specialization_decl_node::ClassTemplatePartialSpecializationDecl),
  ClassTemplateSpecializationDecl(tag_decl_node::record_decl_node::cxx_record_decl_node::class_template_specialization_decl_node::ClassTemplateSpecializationDecl),
  EnumDecl(tag_decl_node::EnumDecl),
  ObjCTypeParamDecl(typedef_name_decl_node::ObjCTypeParamDecl),
  RecordDecl(tag_decl_node::record_decl_node::RecordDecl),
  TagDecl(tag_decl_node::TagDecl),
  TemplateTypeParmDecl(TemplateTypeParmDecl),
  TypeAliasDecl(typedef_name_decl_node::TypeAliasDecl),
  TypedefDecl(typedef_name_decl_node::TypedefDecl),
  TypedefNameDecl(typedef_name_decl_node::TypedefNameDecl),
  UnresolvedUsingTypenameDecl(UnresolvedUsingTypenameDecl),
}

