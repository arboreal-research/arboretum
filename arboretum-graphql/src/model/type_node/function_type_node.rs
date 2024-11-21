use juniper::*;


#[derive(GraphQLObject)]
pub struct FunctionType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct FunctionNoProtoType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct FunctionProtoType {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum FunctionTypeNode {
  FunctionType(FunctionType),
  FunctionNoProtoType(FunctionNoProtoType),
  FunctionProtoType(FunctionProtoType),
}

