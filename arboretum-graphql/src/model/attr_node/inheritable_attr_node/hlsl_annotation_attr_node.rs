use juniper::*;


#[derive(GraphQLObject)]
pub struct HLSLAnnotationAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct HLSLSVDispatchThreadIDAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct HLSLSVGroupIndexAttr {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum HLSLAnnotationAttrNode {
  HLSLAnnotationAttr(HLSLAnnotationAttr),
  HLSLSVDispatchThreadIDAttr(HLSLSVDispatchThreadIDAttr),
  HLSLSVGroupIndexAttr(HLSLSVGroupIndexAttr),
}

