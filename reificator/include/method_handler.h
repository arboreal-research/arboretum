#pragma once

#include <memory>

#include "table.h"

namespace clang {
class EnumDecl;
class CXXRecordDecl;
}  // namespace clang

namespace arboretum {

enum class MethodHandlerKind {
  Identity,
  Enum,
  ClangPointer,
  QualType,
  SourceLocation,
  SourceRange,
  StringRef,
  BasicString,
  ArrayRef,
  IteratorRange
};

struct MethodHandler {
  MethodHandler(const clang::CXXMethodDecl *method_decl_)
      : method_decl(method_decl_) {}
  virtual ~MethodHandler() = default;
  virtual MethodHandlerKind Kind() const = 0;
  virtual std::optional<DataType> Datatype() const = 0;
  const clang::CXXMethodDecl *method_decl;
};

struct IdentityMethodHandler : MethodHandler {
  IdentityMethodHandler(const clang::CXXMethodDecl *method_decl_,
                        DataType datatype_)
      : MethodHandler(method_decl_), datatype(datatype_) {}
  MethodHandlerKind Kind() const override {
    return MethodHandlerKind::Identity;
  }
  std::optional<DataType> Datatype() const override { return datatype; }
  DataType datatype;
};

struct EnumMethodHandler : MethodHandler {
  EnumMethodHandler(const clang::CXXMethodDecl *method_decl_,
                    const clang::EnumDecl *enum_decl_)
      : MethodHandler(method_decl_), enum_decl(enum_decl_) {}
  const clang::EnumDecl *enum_decl;
  MethodHandlerKind Kind() const override { return MethodHandlerKind::Enum; }
  std::optional<DataType> Datatype() const override { return DataType::U64; }
};

struct ClangPointerMethodHandler : MethodHandler {
  ClangPointerMethodHandler(const clang::CXXMethodDecl *method_decl_,
                            const clang::CXXRecordDecl *record_decl_)
      : MethodHandler(method_decl_), record_decl(record_decl_) {}
  const clang::CXXRecordDecl *record_decl;
  MethodHandlerKind Kind() const override {
    return MethodHandlerKind::ClangPointer;
  }
  std::optional<DataType> Datatype() const override { return DataType::U64; }
};

struct QualTypeMethodHandler : MethodHandler {
  QualTypeMethodHandler(const clang::CXXMethodDecl *method_decl_)
      : MethodHandler(method_decl_) {}
  MethodHandlerKind Kind() const override {
    return MethodHandlerKind::QualType;
  }
  std::optional<DataType> Datatype() const override { return DataType::U64; }
};

struct SourceLocationMethodHandler : MethodHandler {
  SourceLocationMethodHandler(const clang::CXXMethodDecl *method_decl_)
      : MethodHandler(method_decl_) {}
  MethodHandlerKind Kind() const override {
    return MethodHandlerKind::SourceLocation;
  }
  std::optional<DataType> Datatype() const override { return DataType::U64; }
};

struct SourceRangeMethodHandler : MethodHandler {
  SourceRangeMethodHandler(const clang::CXXMethodDecl *method_decl_)
      : MethodHandler(method_decl_) {}
  MethodHandlerKind Kind() const override {
    return MethodHandlerKind::SourceRange;
  }
  std::optional<DataType> Datatype() const override { return DataType::U64; }
};

struct StringRefMethodHandler : MethodHandler {
  StringRefMethodHandler(const clang::CXXMethodDecl *method_decl_)
      : MethodHandler(method_decl_) {}
  MethodHandlerKind Kind() const override {
    return MethodHandlerKind::StringRef;
  }
  std::optional<DataType> Datatype() const override { return DataType::STRING; }
};

struct BasicStringMethodHandler : MethodHandler {
  BasicStringMethodHandler(const clang::CXXMethodDecl *method_decl_)
      : MethodHandler(method_decl_) {}
  MethodHandlerKind Kind() const override {
    return MethodHandlerKind::BasicString;
  }
  std::optional<DataType> Datatype() const override { return DataType::STRING; }
};

struct ArrayRefMethodHandler : MethodHandler {
  ArrayRefMethodHandler(const clang::CXXMethodDecl *method_decl_,
                        std::shared_ptr<Table> owned_table_,
                        std::unique_ptr<MethodHandler> &&element_adapter_)
      : MethodHandler(method_decl_),
        owned_table(owned_table_),
        element_adapter(std::move(element_adapter_)) {}
  MethodHandlerKind Kind() const override {
    return MethodHandlerKind::ArrayRef;
  }
  std::optional<DataType> Datatype() const override { return std::nullopt; }
  std::shared_ptr<Table> owned_table;
  std::unique_ptr<MethodHandler> element_adapter;
};

struct IteratorRangeMethodHandler : MethodHandler {
  IteratorRangeMethodHandler(const clang::CXXMethodDecl *method_decl_,
                             std::shared_ptr<Table> owned_table_,
                             std::unique_ptr<MethodHandler> &&element_adapter_)
      : MethodHandler(method_decl_),
        owned_table(owned_table_),
        element_adapter(std::move(element_adapter_)) {}
  MethodHandlerKind Kind() const override {
    return MethodHandlerKind::IteratorRange;
  }
  std::optional<DataType> Datatype() const override { return std::nullopt; }
  std::shared_ptr<Table> owned_table;
  std::unique_ptr<MethodHandler> element_adapter;
};

}  // namespace arboretum