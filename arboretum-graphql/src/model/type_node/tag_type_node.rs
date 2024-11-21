use juniper::*;


#[derive(GraphQLObject)]
pub struct TagType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct RecordType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct EnumType {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum TagTypeNode {
  TagType(TagType),
  EnumType(EnumType),
  RecordType(RecordType),
}

