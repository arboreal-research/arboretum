use juniper::*;


#[derive(GraphQLObject)]
pub struct ObjCObjectType {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCObjectTypeImpl {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCInterfaceType {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum ObjCObjectTypeNode {
  ObjCObjectType(ObjCObjectType),
  ObjCInterfaceType(ObjCInterfaceType),
  ObjCObjectTypeImpl(ObjCObjectTypeImpl),
}

