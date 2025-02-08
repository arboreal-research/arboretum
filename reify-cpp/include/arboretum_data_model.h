#pragma once

#include <clang/AST/RecursiveASTVisitor.h>
#include <clang/Analysis/CFG.h>
#include <clang/Basic/Builtins.h>

#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

#include "arboretum_ffi.h"

namespace arboretum {

struct DataModel {
  DataModel(uint64_t subgraph_id);

  uint64_t next_id();

  //// BEGIN ARBORETUM GENERATED CODE ////
  uint64_t resolve(clang::AccessSpecifier kind);
  uint64_t resolve(clang::ArraySizeModifier kind);
  uint64_t resolve(clang::ArrayTypeTrait kind);
  uint64_t resolve(clang::AutoTypeKeyword kind);
  uint64_t resolve(clang::BinaryOperatorKind kind);
  uint64_t resolve(clang::BuiltinTemplateKind kind);
  uint64_t resolve(clang::CXXConstructionKind kind);
  uint64_t resolve(clang::CXXNewInitializationStyle kind);
  uint64_t resolve(clang::CallingConv kind);
  uint64_t resolve(clang::CanThrowResult kind);
  uint64_t resolve(clang::CapturedRegionKind kind);
  uint64_t resolve(clang::CastKind kind);
  uint64_t resolve(clang::CharacterLiteralKind kind);
  uint64_t resolve(clang::ConstantResultStorageKind kind);
  uint64_t resolve(clang::ConstexprSpecKind kind);
  uint64_t resolve(clang::DeductionCandidate kind);
  uint64_t resolve(clang::ElaboratedTypeKeyword kind);
  uint64_t resolve(clang::ExceptionSpecificationType kind);
  uint64_t resolve(clang::ExprObjectKind kind);
  uint64_t resolve(clang::ExprValueKind kind);
  uint64_t resolve(clang::ExpressionTrait kind);
  uint64_t resolve(clang::IfStatementKind kind);
  uint64_t resolve(clang::ImplicitParamKind kind);
  uint64_t resolve(clang::InClassInitStyle kind);
  uint64_t resolve(clang::LambdaCaptureDefault kind);
  uint64_t resolve(clang::LanguageLinkage kind);
  uint64_t resolve(clang::Linkage kind);
  uint64_t resolve(clang::LinkageSpecLanguageIDs kind);
  uint64_t resolve(clang::MSVtorDispMode kind);
  uint64_t resolve(clang::MultiVersionKind kind);
  uint64_t resolve(clang::NonOdrUseReason kind);
  uint64_t resolve(clang::ObjCStringFormatFamily kind);
  uint64_t resolve(clang::OverloadedOperatorKind kind);
  uint64_t resolve(clang::PragmaMSCommentKind kind);
  uint64_t resolve(clang::PredefinedIdentKind kind);
  uint64_t resolve(clang::RecordArgPassingKind kind);
  uint64_t resolve(clang::RefQualifierKind kind);
  uint64_t resolve(clang::SourceLocIdentKind kind);
  uint64_t resolve(clang::StorageClass kind);
  uint64_t resolve(clang::StorageDuration kind);
  uint64_t resolve(clang::StringLiteralKind kind);
  uint64_t resolve(clang::TagTypeKind kind);
  uint64_t resolve(clang::TemplateSpecializationKind kind);
  uint64_t resolve(clang::ThreadStorageClassSpecifier kind);
  uint64_t resolve(clang::TypeOfKind kind);
  uint64_t resolve(clang::TypeTrait kind);
  uint64_t resolve(clang::UnaryExprOrTypeTrait kind);
  uint64_t resolve(clang::UnaryOperatorKind kind);
  uint64_t resolve(clang::VectorKind kind);
  uint64_t resolve(clang::Visibility kind);
  uint64_t resolve(clang::attr::Kind kind);
  uint64_t resolve(clang::APValue::ValueKind kind);
  uint64_t resolve(clang::AtomicExpr::AtomicOp kind);
  uint64_t resolve(clang::BuiltinType::Kind kind);
  uint64_t resolve(clang::CFGElement::Kind kind);
  uint64_t resolve(clang::CFGTerminator::Kind kind);
  uint64_t resolve(clang::CallExpr::ADLCallKind kind);
  uint64_t resolve(clang::Decl::FriendObjectKind kind);
  uint64_t resolve(clang::Decl::ModuleOwnershipKind kind);
  uint64_t resolve(clang::Decl::ObjCDeclQualifier kind);
  uint64_t resolve(clang::ExprDependenceScope::ExprDependence kind);
  uint64_t resolve(clang::FunctionDecl::TemplatedKind kind);
  uint64_t resolve(clang::Qualifiers::ObjCLifetime kind);
  uint64_t resolve(clang::TypeDependenceScope::TypeDependence kind);
  uint64_t resolve(clang::UnaryTransformType::UTTKind kind);
  uint64_t resolve(clang::UserDefinedLiteral::LiteralOperatorKind kind);
  uint64_t resolve(clang::VarDecl::DefinitionKind kind);
  uint64_t resolve(clang::VarDecl::InitializationStyle kind);
  uint64_t resolve(clang::VarDecl::TLSKind kind);
  uint64_t resolve(llvm::APFloatBase::Semantics kind);
  ////   END ARBORETUM GENERATED CODE ////

 private:
  uint64_t next_id_;
};

}  // namespace arboretum