#include "model.h"

#include <queue>
#include <sstream>
#include <stack>

#include "get_usr.h"

namespace arboretum {

Model::Model(clang::ASTContext &ctx_, Index &index_)
    : ast_ctx(ctx_), index(index_) {}

void Model::PopulateMetaTables() {
  tables.push_back(std::make_shared<Table>(
      "file", std::vector<Field>{Field("id", DataType::U64),
                                 Field("filename", DataType::STRING),
                                 Field("content", DataType::STRING)}));

  tables.push_back(std::make_shared<Table>(
      "source_loc", std::vector<Field>{Field("id", DataType::U64),
                                       Field("file_id", DataType::U64),
                                       Field("line", DataType::U64),
                                       Field("col", DataType::U64),
                                       Field("expansion_loc", DataType::U64),
                                       Field("spelling_loc", DataType::U64)}));

  tables.push_back(std::make_shared<Table>(
      "source_range", std::vector<Field>{Field("id", DataType::U64),
                                         Field("begin", DataType::U64),
                                         Field("end", DataType::U64)}));

  tables.push_back(
      std::make_shared<Table>("enum", std::vector<Field>{
                                          Field("id", DataType::U64),
                                          Field("name", DataType::STRING),
                                      }));

  tables.push_back(
      std::make_shared<Table>("enum_value", std::vector<Field>{
                                                Field("id", DataType::U64),
                                                Field("enum_id", DataType::U64),
                                                Field("name", DataType::STRING),
                                            }));

  tables.push_back(std::make_shared<Table>(
      "QualType", std::vector<Field>{
                      Field("id", DataType::U64),
                      Field("Type_id", DataType::U64),
                      Field("is_const", DataType::BOOL),
                      Field("is_volatile", DataType::BOOL),
                      Field("is_restrict", DataType::BOOL),
                  }));

  tables.push_back(
      std::make_shared<Table>("CFG", std::vector<Field>{
                                         Field("id", DataType::U64),
                                         Field("entry_block_id", DataType::U64),
                                         Field("exit_block_id", DataType::U64),
                                         Field("is_linear", DataType::BOOL),
                                         Field("indirect_goto", DataType::U64),
                                     }));

  tables.push_back(std::make_shared<Table>(
      "CFG_blocks", std::vector<Field>{Field("CFG_id", DataType::U64),
                                       Field("CFGBlock_id", DataType::U64)}));

  tables.push_back(std::make_shared<Table>(
      "CFG_try_blocks",
      std::vector<Field>{Field("CFG_id", DataType::U64),
                         Field("CFGBlock_id", DataType::U64)}));

  tables.push_back(std::make_shared<Table>(
      "CFG_edges", std::vector<Field>{Field("CFGBlock_src", DataType::U64),
                                      Field("CFGBlock_dst", DataType::U64)}));

  tables.push_back(std::make_shared<Table>(
      "CFGBlock", std::vector<Field>{
                      Field("id", DataType::U64),
                      Field("terminator_stmt", DataType::U64),
                      Field("terminator_kind", DataType::U64),
                      Field("terminator_cond", DataType::U64),
                      Field("label_stmt", DataType::U64),
                      Field("loop_target", DataType::U64),
                      Field("has_no_return_element", DataType::BOOL),
                  }));

  tables.push_back(std::make_shared<Table>(
      "CFGBlock_elements",
      std::vector<Field>{Field("CFGBlock_id", DataType::U64),
                         Field("CFGElement_id", DataType::U64)}));

  tables.push_back(std::make_shared<Table>(
      "CFGElement", std::vector<Field>{Field("id", DataType::U64),
                                       Field("kind", DataType::U64)}));

  tables.push_back(std::make_shared<Table>(
      "CFGInitializer", std::vector<Field>{
                            Field("id", DataType::U64),
                            Field("getInitializer", DataType::U64),
                        }));

  tables.push_back(std::make_shared<Table>(
      "CFGScopeBegin", std::vector<Field>{
                           Field("id", DataType::U64),
                           Field("getTriggerStmt", DataType::U64),
                           Field("getVarDecl", DataType::U64),
                       }));

  tables.push_back(std::make_shared<Table>(
      "CFGScopeEnd", std::vector<Field>{
                         Field("id", DataType::U64),
                         Field("getTriggerStmt", DataType::U64),
                         Field("getVarDecl", DataType::U64),
                     }));

  tables.push_back(std::make_shared<Table>(
      "CFGNewAllocator", std::vector<Field>{
                             Field("id", DataType::U64),
                             Field("getAllocatorExpr", DataType::U64),
                         }));

  tables.push_back(std::make_shared<Table>(
      "CFGLifetimeEnds", std::vector<Field>{
                             Field("id", DataType::U64),
                             Field("getTriggerStmt", DataType::U64),
                             Field("getVarDecl", DataType::U64),
                         }));

  tables.push_back(std::make_shared<Table>(
      "CFGLoopExit", std::vector<Field>{
                         Field("id", DataType::U64),
                         Field("getLoopStmt", DataType::U64),
                     }));

  tables.push_back(
      std::make_shared<Table>("CFGStmt", std::vector<Field>{
                                             Field("id", DataType::U64),
                                             Field("getStmt", DataType::U64),
                                         }));

  tables.push_back(std::make_shared<Table>(
      "CFGConstructor", std::vector<Field>{
                            Field("id", DataType::U64),
                            Field("getStmt", DataType::U64),
                            Field("getConstructionContext", DataType::U64),
                        }));

  tables.push_back(std::make_shared<Table>(
      "CFGCXXRecordTypedCall",
      std::vector<Field>{
          Field("id", DataType::U64),
          Field("getStmt", DataType::U64),
          Field("getConstructionContext", DataType::U64),
      }));

  tables.push_back(std::make_shared<Table>(
      "CFGAutomaticObjDtor",
      std::vector<Field>{Field("id", DataType::U64),
                         Field("getDestructorDecl", DataType::U64),
                         Field("getTriggerStmt", DataType::U64),
                         Field("getVarDecl", DataType::U64)}));

  tables.push_back(std::make_shared<Table>(
      "CFGDeleteDtor",
      std::vector<Field>{Field("id", DataType::U64),
                         Field("getDestructorDecl", DataType::U64),
                         Field("getCXXRecordDecl", DataType::U64),
                         Field("getDeleteExpr", DataType::U64)}));

  tables.push_back(std::make_shared<Table>(
      "CFGBaseDtor",
      std::vector<Field>{Field("id", DataType::U64),
                         Field("getDestructorDecl", DataType::U64),
                         Field("getBaseSpecifier", DataType::U64)}));

  tables.push_back(std::make_shared<Table>(
      "CFGMemberDtor",
      std::vector<Field>{Field("id", DataType::U64),
                         Field("getDestructorDecl", DataType::U64),
                         Field("getFieldDecl", DataType::U64)}));

  tables.push_back(std::make_shared<Table>(
      "CFGTemporaryDtor", std::vector<Field>{
                              Field("id", DataType::U64),
                              Field("getDestructorDecl", DataType::U64),
                              Field("getBindTemporaryExpr", DataType::U64),
                          }));

  tables.push_back(std::make_shared<Table>(
      "CFGCleanupFunction", std::vector<Field>{
                                Field("id", DataType::U64),
                                Field("getVarDecl", DataType::U64),
                                Field("getFunctionDecl", DataType::U64),
                            }));
  tables.push_back(std::make_shared<Table>(
      "Decl_usr", std::vector<Field>{Field("id", DataType::U64),
                                     Field("usr", DataType::STRING)}));

  tables.push_back(std::make_shared<Table>(
      "QualType_usr", std::vector<Field>{Field("id", DataType::U64),
                                         Field("usr", DataType::STRING)}));

  tables.push_back(std::make_shared<Table>("FunctionDecl_cfg",
                                           std::vector<Field>{
                                               Field("id", DataType::U64),
                                               Field("cfg", DataType::U64),
                                           }));

  assert(index.clang.cfg_terminator_kind != nullptr);
  MarkEnumUsed(index.clang.cfg_terminator_kind);

  assert(index.clang.cfg_element_kind != nullptr);
  MarkEnumUsed(index.clang.cfg_element_kind);
}

bool Model::MarkEnumUsed(const clang::EnumDecl *decl) {
  std::optional<std::string> enum_usr = getUSR(ast_ctx, decl);
  if (enum_usr.has_value()) {
    auto inserted = enums.insert(std::make_pair(*enum_usr, decl)).second;
    if (inserted) {
      std::unordered_set<int64_t> values;
      for (const auto &enumerator : decl->enumerators()) {
        std::string enumerator_name = enumerator->getNameAsString();

        if (values.insert(enumerator->getInitVal().getExtValue()).second) {
          std::optional<std::string> enum_constant_usr =
              getUSR(ast_ctx, enumerator);
          if (enum_constant_usr.has_value()) {
            enum_constants.insert(
                std::make_pair(*enum_constant_usr, enumerator));
          }
        }
      }
    }
    return true;
  }
  return false;
}

std::unique_ptr<MethodHandler> Model::HandlerForType(
    clang::QualType return_type, const clang::CXXMethodDecl *method_decl) {
  if (return_type->isEnumeralType()) {
    const clang::EnumDecl *decl =
        llvm::dyn_cast<clang::EnumDecl>(return_type->getAsTagDecl());

    const clang::EnumDecl *decl_def = decl->getDefinition();
    if (decl_def == nullptr)
      llvm::errs() << "Missing definition for "
                   << decl->getQualifiedNameAsString() << "!\n";

    if (MarkEnumUsed(decl_def)) {
      return std::make_unique<EnumMethodHandler>(method_decl, decl_def);
    } else {
      llvm::errs() << "Failed to generate USR for "
                   << decl->getQualifiedNameAsString() << "!\n";
    }
  } else if (return_type->isPointerType()) {
    auto pointer_type = return_type->getAs<clang::PointerType>();
    auto pointee_type = pointer_type->getPointeeType();
    if (pointee_type->isRecordType()) {
      auto record_type = pointee_type->getAs<clang::RecordType>();
      auto record_decl =
          llvm::dyn_cast<clang::CXXRecordDecl>(record_type->getDecl());
      if (index.clang.all_decls.contains(record_decl)) {
        return std::make_unique<ClangPointerMethodHandler>(method_decl,
                                                           record_decl);
      }
    }
  } else if (return_type->isBuiltinType()) {
    auto builtin_type = return_type->getAs<clang::BuiltinType>();
    switch (builtin_type->getKind()) {
      case clang::BuiltinType::Bool:
        return std::make_unique<IdentityMethodHandler>(method_decl,
                                                       DataType::BOOL);
      case clang::BuiltinType::UChar:
      case clang::BuiltinType::Char_U:
        return std::make_unique<IdentityMethodHandler>(method_decl,
                                                       DataType::U8);
      case clang::BuiltinType::UShort:
        return std::make_unique<IdentityMethodHandler>(method_decl,
                                                       DataType::U16);
      case clang::BuiltinType::UInt:
        return std::make_unique<IdentityMethodHandler>(method_decl,
                                                       DataType::U32);
      case clang::BuiltinType::ULong:
        return std::make_unique<IdentityMethodHandler>(method_decl,
                                                       DataType::U64);
      case clang::BuiltinType::ULongLong:
        return std::make_unique<IdentityMethodHandler>(method_decl,
                                                       DataType::U64);
      case clang::BuiltinType::SChar:
      case clang::BuiltinType::Char_S:
        return std::make_unique<IdentityMethodHandler>(method_decl,
                                                       DataType::I8);
      case clang::BuiltinType::Short:
        return std::make_unique<IdentityMethodHandler>(method_decl,
                                                       DataType::I16);
      case clang::BuiltinType::Int:
        return std::make_unique<IdentityMethodHandler>(method_decl,
                                                       DataType::I32);
      case clang::BuiltinType::Long:
        return std::make_unique<IdentityMethodHandler>(method_decl,
                                                       DataType::I64);
      case clang::BuiltinType::LongLong:
        return std::make_unique<IdentityMethodHandler>(method_decl,
                                                       DataType::I64);
      case clang::BuiltinType::Float:
        return std::make_unique<IdentityMethodHandler>(method_decl,
                                                       DataType::F32);
      case clang::BuiltinType::Double:
        return std::make_unique<IdentityMethodHandler>(method_decl,
                                                       DataType::F64);
      default:
        break;
    }
  } else if (return_type->isRecordType()) {
    auto record_type =
        return_type.getCanonicalType()->getAs<clang::RecordType>();
    auto record_decl =
        llvm::dyn_cast<clang::CXXRecordDecl>(record_type->getDecl());
    auto fqn = record_decl->getQualifiedNameAsString();
    if (fqn == "clang::QualType") {
      return std::make_unique<QualTypeMethodHandler>(method_decl);
    } else if (fqn == "clang::SourceLocation") {
      return std::make_unique<SourceLocationMethodHandler>(method_decl);
    } else if (fqn == "clang::SourceRange") {
      return std::make_unique<SourceRangeMethodHandler>(method_decl);
    } else if (fqn == "llvm::StringRef") {
      return std::make_unique<StringRefMethodHandler>(method_decl);
    } else if (fqn == "std::basic_string") {
      return std::make_unique<BasicStringMethodHandler>(method_decl);
    }
  }

  return nullptr;
}

std::unique_ptr<MethodHandler> Model::HandlerForMethod(
    const clang::CXXRecordDecl *decl, const clang::CXXMethodDecl *method_decl) {
  auto return_type = method_decl->getReturnType();

  if (return_type->isRecordType()) {
    auto record_type =
        return_type.getCanonicalType()->getAs<clang::RecordType>();
    auto record_decl =
        llvm::dyn_cast<clang::CXXRecordDecl>(record_type->getDecl());
    auto fqn = record_decl->getQualifiedNameAsString();
    if (fqn == "llvm::ArrayRef") {
      auto tmpl_record_decl =
          llvm::dyn_cast_or_null<clang::ClassTemplateSpecializationDecl>(
              record_decl);
      auto element_type = tmpl_record_decl->getTemplateArgs()[0].getAsType();

      auto element_adapter = HandlerForType(element_type, method_decl);
      if (element_adapter != nullptr) {
        std::string table_name =
            decl->getNameAsString() + "_" + method_decl->getNameAsString();
        auto table = std::make_shared<Table>(
            table_name,
            std::vector<Field>{Field("id", DataType::U64),
                               Field("idx", DataType::U64),
                               Field("element", *element_adapter->Datatype())});
        tables.push_back(table);

        return std::make_unique<ArrayRefMethodHandler>(
            method_decl, table, std::move(element_adapter));
      }
    } else if (fqn == "llvm::iterator_range") {
      auto iterator_range_arg =
          return_type->getAs<clang::TemplateSpecializationType>()
              ->template_arguments()[0]
              .getAsType()
              .getCanonicalType();

      if (iterator_range_arg->isPointerType()) {
        auto element_type = iterator_range_arg->getPointeeType();
        auto element_adapter = HandlerForType(element_type, method_decl);
        if (element_adapter != nullptr) {
          std::string table_name =
              decl->getNameAsString() + "_" + method_decl->getNameAsString();
          auto table = std::make_shared<Table>(
              table_name,
              std::vector<Field>{
                  Field("id", DataType::U64), Field("idx", DataType::U64),
                  Field("element", *element_adapter->Datatype())});
          tables.push_back(table);

          return std::make_unique<IteratorRangeMethodHandler>(
              method_decl, table, std::move(element_adapter));
        }
      } else if (iterator_range_arg->isRecordType()) {
        auto iterator_decl = iterator_range_arg->getAsCXXRecordDecl();
        if (iterator_decl != nullptr) {
          clang::DeclarationName name =
              &iterator_decl->getASTContext().Idents.get("value_type");
          auto lookup = iterator_decl->lookup(name);
          if (lookup.isSingleResult()) {
            auto typedef_name_decl =
                llvm::dyn_cast_or_null<clang::TypedefNameDecl>(*lookup.begin());
            if (typedef_name_decl != nullptr) {
              auto element_type =
                  typedef_name_decl->getUnderlyingType().getCanonicalType();
              auto element_adapter = HandlerForType(element_type, method_decl);
              if (element_adapter != nullptr) {
                std::string table_name = decl->getNameAsString() + "_" +
                                         method_decl->getNameAsString();
                auto table = std::make_shared<Table>(
                    table_name,
                    std::vector<Field>{
                        Field("id", DataType::U64), Field("idx", DataType::U64),
                        Field("element", *element_adapter->Datatype())});
                tables.push_back(table);

                return std::make_unique<IteratorRangeMethodHandler>(
                    method_decl, table, std::move(element_adapter));
              }
            }
          }
        }
      }
    }
  }

  return HandlerForType(return_type, method_decl);
}

void Model::PopulateClangTable(
    const std::map<std::string, bool> &allowed_properties,
    const clang::CXXRecordDecl *decl) {
  std::string decl_name = decl->getNameAsString();
  std::vector<Field> fields;
  std::vector<std::unique_ptr<MethodHandler>> data_handlers;
  std::vector<std::unique_ptr<MethodHandler>> assoc_handlers;

  if (decl_name.starts_with("ObjC") || decl_name.starts_with("OMP")) return;

  // Populate fields and handlers
  fields.push_back(Field("id", DataType::U64));
  for (const auto &method_decl : decl->methods()) {
    std::string method_name = method_decl->getNameAsString();
    std::optional<std::string> method_usr = getUSR(ast_ctx, method_decl);
    if (!method_usr.has_value()) continue;

    {
      auto find_itr = allowed_properties.find(*method_usr);
      if (find_itr == allowed_properties.end() || !find_itr->second) continue;
    }

    auto method_handler = HandlerForMethod(decl, method_decl);
    if (method_handler == nullptr) continue;

    std::optional<DataType> datatype = method_handler->Datatype();
    if (datatype.has_value()) {
      fields.push_back(Field(method_name, *datatype));
      data_handlers.push_back(std::move(method_handler));
    } else {
      assoc_handlers.push_back(std::move(method_handler));
    }
  }

  // Record the table and construct the handler.
  auto table = std::make_shared<Table>(decl_name, fields);
  tables.push_back(table);

  handler_by_decl.emplace(decl, DeclHandler(table, std::move(data_handlers),
                                            std::move(assoc_handlers)));
}

void Model::PopulateClangTables(
    const std::map<std::string, bool> &allowed_properties) {
  for (auto decl : index.clang.type_decls) {
    PopulateClangTable(allowed_properties, decl);
  }

  for (auto decl : index.clang.decl_decls) {
    PopulateClangTable(allowed_properties, decl);
  }

  for (auto decl : index.clang.stmt_decls) {
    PopulateClangTable(allowed_properties, decl);
  }
}

std::unordered_map<const clang::EnumDecl *, uint64_t> Model::GetEnumToIndex() {
  std::unordered_map<const clang::EnumDecl *, uint64_t> result;

  uint64_t idx = 0;
  for (const auto &[_, decl] : enums) {
    result[decl] = idx++;
  }

  return result;
}

std::unordered_map<const clang::EnumConstantDecl *, uint64_t>
Model::GetEnumConstantToIndex() {
  std::unordered_map<const clang::EnumConstantDecl *, uint64_t> result;

  uint64_t idx = 0;
  for (const auto &[_, enum_constant] : enum_constants) {
    result[enum_constant] = idx++;
  }

  return result;
}

}  // namespace arboretum
