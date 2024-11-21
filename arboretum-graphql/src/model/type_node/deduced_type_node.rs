use juniper::*;


#[derive(GraphQLObject)]
pub struct DeducedType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AutoType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DeducedTemplateSpecializationType {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum DeducedTypeNode {
  DeducedType(DeducedType),
  AutoType(AutoType),
  DeducedTemplateSpecializationType(DeducedTemplateSpecializationType),
}

