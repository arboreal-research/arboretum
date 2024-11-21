use juniper::*;


#[derive(GraphQLObject)]
pub struct TypeWithKeyword {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ElaboratedType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DependentNameType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DependentTemplateSpecializationType {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum TypeWithKeywordNode {
  TypeWithKeyword(TypeWithKeyword),
  DependentNameType(DependentNameType),
  DependentTemplateSpecializationType(DependentTemplateSpecializationType),
  ElaboratedType(ElaboratedType),
}

