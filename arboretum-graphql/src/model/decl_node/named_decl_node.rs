use juniper::*;

pub mod base_using_decl_node;
pub mod obj_c_container_decl_node;
pub mod template_decl_node;
pub mod type_decl_node;
pub mod using_shadow_decl_node;
pub mod value_decl_node;

#[derive(GraphQLObject)]
pub struct NamedDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct LabelDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NamespaceDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct HLSLBufferDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct UsingDirectiveDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NamespaceAliasDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct UsingPackDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct UnresolvedUsingIfExistsDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCMethodDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCPropertyDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCCompatibleAliasDecl {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum NamedDeclNode {
  NamedDecl(NamedDecl),
  BaseUsingDecl(base_using_decl_node::BaseUsingDecl),
  BindingDecl(value_decl_node::BindingDecl),
  BuiltinTemplateDecl(template_decl_node::BuiltinTemplateDecl),
  CXXConstructorDecl(value_decl_node::declarator_decl_node::function_decl_node::cxx_method_decl_node::CXXConstructorDecl),
  CXXConversionDecl(value_decl_node::declarator_decl_node::function_decl_node::cxx_method_decl_node::CXXConversionDecl),
  CXXDeductionGuideDecl(value_decl_node::declarator_decl_node::function_decl_node::CXXDeductionGuideDecl),
  CXXDestructorDecl(value_decl_node::declarator_decl_node::function_decl_node::cxx_method_decl_node::CXXDestructorDecl),
  CXXMethodDecl(value_decl_node::declarator_decl_node::function_decl_node::cxx_method_decl_node::CXXMethodDecl),
  CXXRecordDecl(type_decl_node::tag_decl_node::record_decl_node::cxx_record_decl_node::CXXRecordDecl),
  ClassTemplateDecl(template_decl_node::redeclarable_template_decl_node::ClassTemplateDecl),
  ClassTemplatePartialSpecializationDecl(type_decl_node::tag_decl_node::record_decl_node::cxx_record_decl_node::class_template_specialization_decl_node::ClassTemplatePartialSpecializationDecl),
  ClassTemplateSpecializationDecl(type_decl_node::tag_decl_node::record_decl_node::cxx_record_decl_node::class_template_specialization_decl_node::ClassTemplateSpecializationDecl),
  ConceptDecl(template_decl_node::ConceptDecl),
  ConstructorUsingShadowDecl(using_shadow_decl_node::ConstructorUsingShadowDecl),
  DeclaratorDecl(value_decl_node::declarator_decl_node::DeclaratorDecl),
  DecompositionDecl(value_decl_node::declarator_decl_node::var_decl_node::DecompositionDecl),
  EnumConstantDecl(value_decl_node::EnumConstantDecl),
  EnumDecl(type_decl_node::tag_decl_node::EnumDecl),
  FieldDecl(value_decl_node::declarator_decl_node::field_decl_node::FieldDecl),
  FunctionDecl(value_decl_node::declarator_decl_node::function_decl_node::FunctionDecl),
  FunctionTemplateDecl(template_decl_node::redeclarable_template_decl_node::FunctionTemplateDecl),
  HLSLBufferDecl(HLSLBufferDecl),
  ImplicitParamDecl(value_decl_node::declarator_decl_node::var_decl_node::ImplicitParamDecl),
  IndirectFieldDecl(value_decl_node::IndirectFieldDecl),
  LabelDecl(LabelDecl),
  MSGuidDecl(value_decl_node::MSGuidDecl),
  MSPropertyDecl(value_decl_node::declarator_decl_node::MSPropertyDecl),
  NamespaceAliasDecl(NamespaceAliasDecl),
  NamespaceDecl(NamespaceDecl),
  NonTypeTemplateParmDecl(value_decl_node::declarator_decl_node::NonTypeTemplateParmDecl),
  ObjCAtDefsFieldDecl(value_decl_node::declarator_decl_node::field_decl_node::ObjCAtDefsFieldDecl),
  ObjCCategoryDecl(obj_c_container_decl_node::ObjCCategoryDecl),
  ObjCCategoryImplDecl(obj_c_container_decl_node::obj_c_impl_decl_node::ObjCCategoryImplDecl),
  ObjCCompatibleAliasDecl(ObjCCompatibleAliasDecl),
  ObjCContainerDecl(obj_c_container_decl_node::ObjCContainerDecl),
  ObjCImplDecl(obj_c_container_decl_node::obj_c_impl_decl_node::ObjCImplDecl),
  ObjCImplementationDecl(obj_c_container_decl_node::obj_c_impl_decl_node::ObjCImplementationDecl),
  ObjCInterfaceDecl(obj_c_container_decl_node::ObjCInterfaceDecl),
  ObjCIvarDecl(value_decl_node::declarator_decl_node::field_decl_node::ObjCIvarDecl),
  ObjCMethodDecl(ObjCMethodDecl),
  ObjCPropertyDecl(ObjCPropertyDecl),
  ObjCProtocolDecl(obj_c_container_decl_node::ObjCProtocolDecl),
  ObjCTypeParamDecl(type_decl_node::typedef_name_decl_node::ObjCTypeParamDecl),
  ParmVarDecl(value_decl_node::declarator_decl_node::var_decl_node::ParmVarDecl),
  RecordDecl(type_decl_node::tag_decl_node::record_decl_node::RecordDecl),
  RedeclarableTemplateDecl(template_decl_node::redeclarable_template_decl_node::RedeclarableTemplateDecl),
  TagDecl(type_decl_node::tag_decl_node::TagDecl),
  TemplateDecl(template_decl_node::TemplateDecl),
  TemplateParamObjectDecl(value_decl_node::TemplateParamObjectDecl),
  TemplateTemplateParmDecl(template_decl_node::TemplateTemplateParmDecl),
  TemplateTypeParmDecl(type_decl_node::TemplateTypeParmDecl),
  TypeAliasDecl(type_decl_node::typedef_name_decl_node::TypeAliasDecl),
  TypeAliasTemplateDecl(template_decl_node::redeclarable_template_decl_node::TypeAliasTemplateDecl),
  TypeDecl(type_decl_node::TypeDecl),
  TypedefDecl(type_decl_node::typedef_name_decl_node::TypedefDecl),
  TypedefNameDecl(type_decl_node::typedef_name_decl_node::TypedefNameDecl),
  UnnamedGlobalConstantDecl(value_decl_node::UnnamedGlobalConstantDecl),
  UnresolvedUsingIfExistsDecl(UnresolvedUsingIfExistsDecl),
  UnresolvedUsingTypenameDecl(type_decl_node::UnresolvedUsingTypenameDecl),
  UnresolvedUsingValueDecl(value_decl_node::UnresolvedUsingValueDecl),
  UsingDecl(base_using_decl_node::UsingDecl),
  UsingDirectiveDecl(UsingDirectiveDecl),
  UsingEnumDecl(base_using_decl_node::UsingEnumDecl),
  UsingPackDecl(UsingPackDecl),
  UsingShadowDecl(using_shadow_decl_node::UsingShadowDecl),
  ValueDecl(value_decl_node::ValueDecl),
  VarDecl(value_decl_node::declarator_decl_node::var_decl_node::VarDecl),
  VarTemplateDecl(template_decl_node::redeclarable_template_decl_node::VarTemplateDecl),
  VarTemplatePartialSpecializationDecl(value_decl_node::declarator_decl_node::var_decl_node::var_template_specialization_decl_node::VarTemplatePartialSpecializationDecl),
  VarTemplateSpecializationDecl(value_decl_node::declarator_decl_node::var_decl_node::var_template_specialization_decl_node::VarTemplateSpecializationDecl),
}

