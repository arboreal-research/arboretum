use juniper::*;


#[derive(GraphQLObject)]
pub struct AsmStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct GCCAsmStmt {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MSAsmStmt {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum AsmStmtNode {
  AsmStmt(AsmStmt),
  GCCAsmStmt(GCCAsmStmt),
  MSAsmStmt(MSAsmStmt),
}

