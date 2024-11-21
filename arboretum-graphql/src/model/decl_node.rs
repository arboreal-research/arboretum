use juniper::*;

pub mod named_decl_node;

#[derive(GraphQLObject)]
pub struct Decl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct TranslationUnitDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct PragmaCommentDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct PragmaDetectMismatchDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ExternCContextDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct FileScopeAsmDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct TopLevelStmtDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct BlockDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CapturedDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ImportDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ExportDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct EmptyDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AccessSpecDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct RequiresExprBodyDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct LinkageSpecDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct LifetimeExtendedTemporaryDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct StaticAssertDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct FriendTemplateDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ImplicitConceptSpecializationDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct FriendDecl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCPropertyImplDecl {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum DeclNode {
  Decl(Decl),
  AccessSpecDecl(AccessSpecDecl),
  BaseUsingDecl(named_decl_node::base_using_decl_node::BaseUsingDecl),
  BindingDecl(named_decl_node::value_decl_node::BindingDecl),
  BlockDecl(BlockDecl),
  BuiltinTemplateDecl(named_decl_node::template_decl_node::BuiltinTemplateDecl),
  CXXConstructorDecl(named_decl_node::value_decl_node::declarator_decl_node::function_decl_node::cxx_method_decl_node::CXXConstructorDecl),
  CXXConversionDecl(named_decl_node::value_decl_node::declarator_decl_node::function_decl_node::cxx_method_decl_node::CXXConversionDecl),
  CXXDeductionGuideDecl(named_decl_node::value_decl_node::declarator_decl_node::function_decl_node::CXXDeductionGuideDecl),
  CXXDestructorDecl(named_decl_node::value_decl_node::declarator_decl_node::function_decl_node::cxx_method_decl_node::CXXDestructorDecl),
  CXXMethodDecl(named_decl_node::value_decl_node::declarator_decl_node::function_decl_node::cxx_method_decl_node::CXXMethodDecl),
  CXXRecordDecl(named_decl_node::type_decl_node::tag_decl_node::record_decl_node::cxx_record_decl_node::CXXRecordDecl),
  CapturedDecl(CapturedDecl),
  ClassTemplateDecl(named_decl_node::template_decl_node::redeclarable_template_decl_node::ClassTemplateDecl),
  ClassTemplatePartialSpecializationDecl(named_decl_node::type_decl_node::tag_decl_node::record_decl_node::cxx_record_decl_node::class_template_specialization_decl_node::ClassTemplatePartialSpecializationDecl),
  ClassTemplateSpecializationDecl(named_decl_node::type_decl_node::tag_decl_node::record_decl_node::cxx_record_decl_node::class_template_specialization_decl_node::ClassTemplateSpecializationDecl),
  ConceptDecl(named_decl_node::template_decl_node::ConceptDecl),
  ConstructorUsingShadowDecl(named_decl_node::using_shadow_decl_node::ConstructorUsingShadowDecl),
  DeclaratorDecl(named_decl_node::value_decl_node::declarator_decl_node::DeclaratorDecl),
  DecompositionDecl(named_decl_node::value_decl_node::declarator_decl_node::var_decl_node::DecompositionDecl),
  EmptyDecl(EmptyDecl),
  EnumConstantDecl(named_decl_node::value_decl_node::EnumConstantDecl),
  EnumDecl(named_decl_node::type_decl_node::tag_decl_node::EnumDecl),
  ExportDecl(ExportDecl),
  ExternCContextDecl(ExternCContextDecl),
  FieldDecl(named_decl_node::value_decl_node::declarator_decl_node::field_decl_node::FieldDecl),
  FileScopeAsmDecl(FileScopeAsmDecl),
  FriendDecl(FriendDecl),
  FriendTemplateDecl(FriendTemplateDecl),
  FunctionDecl(named_decl_node::value_decl_node::declarator_decl_node::function_decl_node::FunctionDecl),
  FunctionTemplateDecl(named_decl_node::template_decl_node::redeclarable_template_decl_node::FunctionTemplateDecl),
  HLSLBufferDecl(named_decl_node::HLSLBufferDecl),
  ImplicitConceptSpecializationDecl(ImplicitConceptSpecializationDecl),
  ImplicitParamDecl(named_decl_node::value_decl_node::declarator_decl_node::var_decl_node::ImplicitParamDecl),
  ImportDecl(ImportDecl),
  IndirectFieldDecl(named_decl_node::value_decl_node::IndirectFieldDecl),
  LabelDecl(named_decl_node::LabelDecl),
  LifetimeExtendedTemporaryDecl(LifetimeExtendedTemporaryDecl),
  LinkageSpecDecl(LinkageSpecDecl),
  MSGuidDecl(named_decl_node::value_decl_node::MSGuidDecl),
  MSPropertyDecl(named_decl_node::value_decl_node::declarator_decl_node::MSPropertyDecl),
  NamedDecl(named_decl_node::NamedDecl),
  NamespaceAliasDecl(named_decl_node::NamespaceAliasDecl),
  NamespaceDecl(named_decl_node::NamespaceDecl),
  NonTypeTemplateParmDecl(named_decl_node::value_decl_node::declarator_decl_node::NonTypeTemplateParmDecl),
  ObjCAtDefsFieldDecl(named_decl_node::value_decl_node::declarator_decl_node::field_decl_node::ObjCAtDefsFieldDecl),
  ObjCCategoryDecl(named_decl_node::obj_c_container_decl_node::ObjCCategoryDecl),
  ObjCCategoryImplDecl(named_decl_node::obj_c_container_decl_node::obj_c_impl_decl_node::ObjCCategoryImplDecl),
  ObjCCompatibleAliasDecl(named_decl_node::ObjCCompatibleAliasDecl),
  ObjCContainerDecl(named_decl_node::obj_c_container_decl_node::ObjCContainerDecl),
  ObjCImplDecl(named_decl_node::obj_c_container_decl_node::obj_c_impl_decl_node::ObjCImplDecl),
  ObjCImplementationDecl(named_decl_node::obj_c_container_decl_node::obj_c_impl_decl_node::ObjCImplementationDecl),
  ObjCInterfaceDecl(named_decl_node::obj_c_container_decl_node::ObjCInterfaceDecl),
  ObjCIvarDecl(named_decl_node::value_decl_node::declarator_decl_node::field_decl_node::ObjCIvarDecl),
  ObjCMethodDecl(named_decl_node::ObjCMethodDecl),
  ObjCPropertyDecl(named_decl_node::ObjCPropertyDecl),
  ObjCPropertyImplDecl(ObjCPropertyImplDecl),
  ObjCProtocolDecl(named_decl_node::obj_c_container_decl_node::ObjCProtocolDecl),
  ObjCTypeParamDecl(named_decl_node::type_decl_node::typedef_name_decl_node::ObjCTypeParamDecl),
  ParmVarDecl(named_decl_node::value_decl_node::declarator_decl_node::var_decl_node::ParmVarDecl),
  PragmaCommentDecl(PragmaCommentDecl),
  PragmaDetectMismatchDecl(PragmaDetectMismatchDecl),
  RecordDecl(named_decl_node::type_decl_node::tag_decl_node::record_decl_node::RecordDecl),
  RedeclarableTemplateDecl(named_decl_node::template_decl_node::redeclarable_template_decl_node::RedeclarableTemplateDecl),
  RequiresExprBodyDecl(RequiresExprBodyDecl),
  StaticAssertDecl(StaticAssertDecl),
  TagDecl(named_decl_node::type_decl_node::tag_decl_node::TagDecl),
  TemplateDecl(named_decl_node::template_decl_node::TemplateDecl),
  TemplateParamObjectDecl(named_decl_node::value_decl_node::TemplateParamObjectDecl),
  TemplateTemplateParmDecl(named_decl_node::template_decl_node::TemplateTemplateParmDecl),
  TemplateTypeParmDecl(named_decl_node::type_decl_node::TemplateTypeParmDecl),
  TopLevelStmtDecl(TopLevelStmtDecl),
  TranslationUnitDecl(TranslationUnitDecl),
  TypeAliasDecl(named_decl_node::type_decl_node::typedef_name_decl_node::TypeAliasDecl),
  TypeAliasTemplateDecl(named_decl_node::template_decl_node::redeclarable_template_decl_node::TypeAliasTemplateDecl),
  TypeDecl(named_decl_node::type_decl_node::TypeDecl),
  TypedefDecl(named_decl_node::type_decl_node::typedef_name_decl_node::TypedefDecl),
  TypedefNameDecl(named_decl_node::type_decl_node::typedef_name_decl_node::TypedefNameDecl),
  UnnamedGlobalConstantDecl(named_decl_node::value_decl_node::UnnamedGlobalConstantDecl),
  UnresolvedUsingIfExistsDecl(named_decl_node::UnresolvedUsingIfExistsDecl),
  UnresolvedUsingTypenameDecl(named_decl_node::type_decl_node::UnresolvedUsingTypenameDecl),
  UnresolvedUsingValueDecl(named_decl_node::value_decl_node::UnresolvedUsingValueDecl),
  UsingDecl(named_decl_node::base_using_decl_node::UsingDecl),
  UsingDirectiveDecl(named_decl_node::UsingDirectiveDecl),
  UsingEnumDecl(named_decl_node::base_using_decl_node::UsingEnumDecl),
  UsingPackDecl(named_decl_node::UsingPackDecl),
  UsingShadowDecl(named_decl_node::using_shadow_decl_node::UsingShadowDecl),
  ValueDecl(named_decl_node::value_decl_node::ValueDecl),
  VarDecl(named_decl_node::value_decl_node::declarator_decl_node::var_decl_node::VarDecl),
  VarTemplateDecl(named_decl_node::template_decl_node::redeclarable_template_decl_node::VarTemplateDecl),
  VarTemplatePartialSpecializationDecl(named_decl_node::value_decl_node::declarator_decl_node::var_decl_node::var_template_specialization_decl_node::VarTemplatePartialSpecializationDecl),
  VarTemplateSpecializationDecl(named_decl_node::value_decl_node::declarator_decl_node::var_decl_node::var_template_specialization_decl_node::VarTemplateSpecializationDecl),
}

