use arrow::array::*;
use arrow::datatypes::*;
use std::ffi::c_char;
use std::sync::Arc;

pub mod ffi;
pub mod io;

// Re-export FFI functions
pub use ffi::{queue_record, flush_records};

//// BEGIN ARBORETUM GENERATED CODE ////
// Thread-local storage for enum queues
thread_local!(static ENUM_QUEUE: std::cell::RefCell<Vec<(u64, String)>> = std::cell::RefCell::new(Vec::new()));
thread_local!(static ENUM_VALUE_QUEUE: std::cell::RefCell<Vec<(u64, u64)>> = std::cell::RefCell::new(Vec::new()));

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__VarDecl__TLSKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__VarDecl__InitializationStyle(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__Qualifiers__ObjCLifetime(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__FunctionDecl__TemplatedKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__TypeDependenceScope__TypeDependence(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__ExprDependenceScope__ExprDependence(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__MSVtorDispMode(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__LinkageSpecLanguageIDs(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__LanguageLinkage(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__ExprValueKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__PredefinedIdentKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__BuiltinType__Kind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__InClassInitStyle(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__ArrayTypeTrait(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__ExprObjectKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__CallingConv(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__UserDefinedLiteral__LiteralOperatorKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__LambdaCaptureDefault(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__attr__Kind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__Decl__ModuleOwnershipKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__ConstantResultStorageKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__Linkage(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__ArraySizeModifier(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__AutoTypeKeyword(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__ExceptionSpecificationType(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__CXXConstructionKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__ImplicitParamKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__CapturedRegionKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__CFGElement__Kind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__UnaryOperatorKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__CXXNewInitializationStyle(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__CastKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__ElaboratedTypeKeyword(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__VectorKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__TagTypeKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__CanThrowResult(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__BuiltinTemplateKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__IfStatementKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__CharacterLiteralKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__StorageDuration(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__ExpressionTrait(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__MultiVersionKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__UnaryTransformType__UTTKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__NonOdrUseReason(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__Visibility(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__ObjCStringFormatFamily(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__VarDecl__DefinitionKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__ConstexprSpecKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__CallExpr__ADLCallKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_llvm__APFloatBase__Semantics(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__Decl__ObjCDeclQualifier(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__OverloadedOperatorKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__BinaryOperatorKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__StorageClass(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__RecordArgPassingKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__RefQualifierKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__SourceLocIdentKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__StringLiteralKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__TemplateSpecializationKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__DeductionCandidate(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__ThreadStorageClassSpecifier(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__TypeOfKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__PragmaMSCommentKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__TypeTrait(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__UnaryExprOrTypeTrait(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__APValue__ValueKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__AccessSpecifier(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__AtomicExpr__AtomicOp(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__CFGTerminator__Kind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_clang__Decl__FriendObjectKind(id: u64, name: *const c_char) {
  let name_str = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
  ENUM_QUEUE.with(|q| q.borrow_mut().push((id, name_str)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__VarDecl__TLS_None(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__VarDecl__TLS_Static(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__VarDecl__TLS_Dynamic(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__VarDecl__CInit(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__VarDecl__CallInit(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__VarDecl__ListInit(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__VarDecl__ParenListInit(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__Qualifiers__OCL_None(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__Qualifiers__OCL_ExplicitNone(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__Qualifiers__OCL_Strong(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__Qualifiers__OCL_Weak(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__Qualifiers__OCL_Autoreleasing(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__FunctionDecl__TK_NonTemplate(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__FunctionDecl__TK_FunctionTemplate(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__FunctionDecl__TK_MemberSpecialization(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__FunctionDecl__TK_FunctionTemplateSpecialization(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__FunctionDecl__TK_DependentFunctionTemplateSpecialization(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__FunctionDecl__TK_DependentNonTemplate(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__TypeDependenceScope__UnexpandedPack(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__TypeDependenceScope__Instantiation(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__TypeDependenceScope__Dependent(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__TypeDependenceScope__VariablyModified(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__TypeDependenceScope__Error(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__TypeDependenceScope__None(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__TypeDependenceScope__All(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__TypeDependenceScope__DependentInstantiation(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ExprDependenceScope__UnexpandedPack(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ExprDependenceScope__Instantiation(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ExprDependenceScope__Type(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ExprDependenceScope__Value(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ExprDependenceScope__Error(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ExprDependenceScope__None(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ExprDependenceScope__All(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ExprDependenceScope__TypeValue(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ExprDependenceScope__TypeInstantiation(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ExprDependenceScope__ValueInstantiation(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ExprDependenceScope__TypeValueInstantiation(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ExprDependenceScope__ErrorDependent(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__MSVtorDispMode__Never(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__MSVtorDispMode__ForVBaseOverride(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__MSVtorDispMode__ForVFTable(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__LinkageSpecLanguageIDs__C(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__LinkageSpecLanguageIDs__CXX(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CLanguageLinkage(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CXXLanguageLinkage(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__NoLanguageLinkage(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__VK_PRValue(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__VK_LValue(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__VK_XValue(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__PredefinedIdentKind__Func(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__PredefinedIdentKind__Function(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__PredefinedIdentKind__LFunction(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__PredefinedIdentKind__FuncDName(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__PredefinedIdentKind__FuncSig(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__PredefinedIdentKind__LFuncSig(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__PredefinedIdentKind__PrettyFunction(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__PredefinedIdentKind__PrettyFunctionNoVirtual(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage1dRO(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage1dArrayRO(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage1dBufferRO(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage2dRO(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage2dArrayRO(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage2dDepthRO(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage2dArrayDepthRO(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage2dMSAARO(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage2dArrayMSAARO(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage2dMSAADepthRO(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage2dArrayMSAADepthRO(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage3dRO(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage1dWO(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage1dArrayWO(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage1dBufferWO(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage2dWO(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage2dArrayWO(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage2dDepthWO(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage2dArrayDepthWO(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage2dMSAAWO(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage2dArrayMSAAWO(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage2dMSAADepthWO(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage2dArrayMSAADepthWO(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage3dWO(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage1dRW(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage1dArrayRW(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage1dBufferRW(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage2dRW(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage2dArrayRW(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage2dDepthRW(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage2dArrayDepthRW(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage2dMSAARW(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage2dArrayMSAARW(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage2dMSAADepthRW(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage2dArrayMSAADepthRW(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLImage3dRW(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLIntelSubgroupAVCMcePayload(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLIntelSubgroupAVCImePayload(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLIntelSubgroupAVCRefPayload(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLIntelSubgroupAVCSicPayload(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLIntelSubgroupAVCMceResult(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLIntelSubgroupAVCImeResult(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLIntelSubgroupAVCRefResult(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLIntelSubgroupAVCSicResult(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLIntelSubgroupAVCImeResultSingleReferenceStreamout(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLIntelSubgroupAVCImeResultDualReferenceStreamout(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLIntelSubgroupAVCImeSingleReferenceStreamin(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLIntelSubgroupAVCImeDualReferenceStreamin(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveInt8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveInt16(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveInt32(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveInt64(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveUint8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveUint16(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveUint32(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveUint64(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveFloat16(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveFloat32(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveFloat64(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveBFloat16(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveInt8x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveInt16x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveInt32x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveInt64x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveUint8x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveUint16x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveUint32x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveUint64x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveFloat16x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveFloat32x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveFloat64x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveBFloat16x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveInt8x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveInt16x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveInt32x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveInt64x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveUint8x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveUint16x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveUint32x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveUint64x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveFloat16x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveFloat32x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveFloat64x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveBFloat16x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveInt8x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveInt16x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveInt32x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveInt64x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveUint8x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveUint16x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveUint32x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveUint64x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveFloat16x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveFloat32x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveFloat64x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveBFloat16x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveBool(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveBoolx2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveBoolx4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SveCount(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__VectorQuad(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__VectorPair(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8mf8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8mf4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8mf2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8m1(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8m2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8m4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8m8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8mf8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8mf4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8mf2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8m1(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8m2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8m4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8m8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16mf4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16mf2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16m1(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16m2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16m4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16m8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16mf4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16mf2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16m1(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16m2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16m4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16m8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt32mf2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt32m1(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt32m2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt32m4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt32m8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint32mf2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint32m1(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint32m2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint32m4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint32m8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt64m1(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt64m2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt64m4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt64m8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint64m1(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint64m2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint64m4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint64m8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16mf4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16mf2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16m1(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16m2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16m4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16m8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16mf4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16mf2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16m1(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16m2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16m4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16m8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat32mf2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat32m1(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat32m2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat32m4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat32m8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat64m1(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat64m2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat64m4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat64m8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBool1(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBool2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBool4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBool8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBool16(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBool32(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBool64(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8mf8x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8mf8x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8mf8x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8mf8x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8mf8x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8mf8x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8mf8x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8mf4x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8mf4x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8mf4x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8mf4x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8mf4x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8mf4x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8mf4x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8mf2x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8mf2x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8mf2x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8mf2x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8mf2x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8mf2x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8mf2x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8m1x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8m1x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8m1x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8m1x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8m1x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8m1x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8m1x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8m2x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8m2x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8m2x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt8m4x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8mf8x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8mf8x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8mf8x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8mf8x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8mf8x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8mf8x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8mf8x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8mf4x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8mf4x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8mf4x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8mf4x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8mf4x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8mf4x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8mf4x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8mf2x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8mf2x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8mf2x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8mf2x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8mf2x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8mf2x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8mf2x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8m1x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8m1x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8m1x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8m1x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8m1x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8m1x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8m1x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8m2x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8m2x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8m2x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint8m4x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16mf4x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16mf4x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16mf4x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16mf4x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16mf4x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16mf4x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16mf4x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16mf2x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16mf2x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16mf2x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16mf2x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16mf2x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16mf2x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16mf2x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16m1x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16m1x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16m1x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16m1x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16m1x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16m1x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16m1x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16m2x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16m2x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16m2x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt16m4x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16mf4x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16mf4x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16mf4x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16mf4x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16mf4x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16mf4x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16mf4x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16mf2x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16mf2x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16mf2x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16mf2x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16mf2x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16mf2x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16mf2x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16m1x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16m1x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16m1x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16m1x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16m1x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16m1x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16m1x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16m2x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16m2x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16m2x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint16m4x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt32mf2x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt32mf2x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt32mf2x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt32mf2x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt32mf2x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt32mf2x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt32mf2x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt32m1x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt32m1x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt32m1x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt32m1x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt32m1x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt32m1x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt32m1x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt32m2x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt32m2x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt32m2x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt32m4x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint32mf2x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint32mf2x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint32mf2x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint32mf2x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint32mf2x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint32mf2x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint32mf2x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint32m1x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint32m1x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint32m1x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint32m1x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint32m1x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint32m1x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint32m1x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint32m2x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint32m2x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint32m2x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint32m4x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt64m1x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt64m1x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt64m1x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt64m1x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt64m1x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt64m1x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt64m1x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt64m2x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt64m2x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt64m2x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvInt64m4x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint64m1x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint64m1x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint64m1x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint64m1x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint64m1x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint64m1x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint64m1x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint64m2x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint64m2x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint64m2x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvUint64m4x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16mf4x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16mf4x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16mf4x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16mf4x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16mf4x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16mf4x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16mf4x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16mf2x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16mf2x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16mf2x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16mf2x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16mf2x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16mf2x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16mf2x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16m1x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16m1x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16m1x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16m1x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16m1x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16m1x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16m1x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16m2x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16m2x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16m2x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat16m4x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat32mf2x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat32mf2x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat32mf2x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat32mf2x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat32mf2x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat32mf2x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat32mf2x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat32m1x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat32m1x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat32m1x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat32m1x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat32m1x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat32m1x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat32m1x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat32m2x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat32m2x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat32m2x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat32m4x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat64m1x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat64m1x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat64m1x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat64m1x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat64m1x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat64m1x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat64m1x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat64m2x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat64m2x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat64m2x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvFloat64m4x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16mf4x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16mf4x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16mf4x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16mf4x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16mf4x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16mf4x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16mf4x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16mf2x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16mf2x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16mf2x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16mf2x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16mf2x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16mf2x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16mf2x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16m1x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16m1x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16m1x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16m1x5(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16m1x6(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16m1x7(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16m1x8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16m2x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16m2x3(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16m2x4(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__RvvBFloat16m4x2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__WasmExternRef(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__Void(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__Bool(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__Char_U(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__UChar(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__WChar_U(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__Char8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__Char16(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__Char32(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__UShort(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__UInt(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__ULong(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__ULongLong(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__UInt128(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__Char_S(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SChar(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__WChar_S(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__Short(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__Int(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__Long(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__LongLong(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__Int128(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__ShortAccum(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__Accum(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__LongAccum(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__UShortAccum(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__UAccum(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__ULongAccum(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__ShortFract(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__Fract(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__LongFract(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__UShortFract(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__UFract(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__ULongFract(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SatShortAccum(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SatAccum(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SatLongAccum(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SatUShortAccum(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SatUAccum(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SatULongAccum(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SatShortFract(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SatFract(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SatLongFract(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SatUShortFract(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SatUFract(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__SatULongFract(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__Half(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__Float(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__Double(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__LongDouble(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__Float16(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__BFloat16(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__Float128(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__Ibm128(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__NullPtr(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__ObjCId(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__ObjCClass(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__ObjCSel(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLSampler(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLEvent(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLClkEvent(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLQueue(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OCLReserveID(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__Dependent(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__Overload(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__BoundMember(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__PseudoObject(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__UnknownAny(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__BuiltinFn(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__ARCUnbridgedCast(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__IncompleteMatrixIdx(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OMPArraySection(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OMPArrayShaping(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BuiltinType__OMPIterator(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ICIS_NoInit(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ICIS_CopyInit(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ICIS_ListInit(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ATT_ArrayRank(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ATT_ArrayExtent(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OK_Ordinary(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OK_BitField(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OK_VectorComponent(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OK_ObjCProperty(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OK_ObjCSubscript(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OK_MatrixComponent(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CC_C(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CC_X86StdCall(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CC_X86FastCall(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CC_X86ThisCall(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CC_X86VectorCall(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CC_X86Pascal(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CC_Win64(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CC_X86_64SysV(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CC_X86RegCall(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CC_AAPCS(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CC_AAPCS_VFP(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CC_IntelOclBicc(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CC_SpirFunction(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CC_OpenCLKernel(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CC_Swift(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CC_SwiftAsync(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CC_PreserveMost(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CC_PreserveAll(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CC_AArch64VectorCall(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CC_AArch64SVEPCS(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CC_AMDGPUKernelCall(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CC_M68kRTD(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UserDefinedLiteral__LOK_Raw(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UserDefinedLiteral__LOK_Template(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UserDefinedLiteral__LOK_Integer(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UserDefinedLiteral__LOK_Floating(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UserDefinedLiteral__LOK_String(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UserDefinedLiteral__LOK_Character(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__LCD_None(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__LCD_ByCopy(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__LCD_ByRef(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AddressSpace(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AnnotateType(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ArmIn(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ArmInOut(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ArmMveStrictPolymorphism(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ArmOut(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ArmPreserves(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ArmStreaming(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ArmStreamingCompatible(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__BTFTypeTag(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CmseNSCall(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__HLSLGroupSharedAddressSpace(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__HLSLParamModifier(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NoDeref(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ObjCGC(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ObjCInertUnsafeUnretained(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ObjCKindOf(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__OpenCLConstantAddressSpace(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__OpenCLGenericAddressSpace(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__OpenCLGlobalAddressSpace(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__OpenCLGlobalDeviceAddressSpace(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__OpenCLGlobalHostAddressSpace(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__OpenCLLocalAddressSpace(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__OpenCLPrivateAddressSpace(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Ptr32(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Ptr64(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SPtr(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__TypeNonNull(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__TypeNullUnspecified(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__TypeNullable(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__TypeNullableResult(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__UPtr(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__WebAssemblyFuncref(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CodeAlign(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__FallThrough(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Likely(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__MustTail(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__OpenCLUnrollHint(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Unlikely(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AlwaysInline(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NoInline(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NoMerge(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Suppress(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AArch64SVEPcs(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AArch64VectorPcs(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AMDGPUKernelCall(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AcquireHandle(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AnyX86NoCfCheck(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CDecl(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__FastCall(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__IntelOclBicc(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__LifetimeBound(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__M68kRTD(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__MSABI(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NSReturnsRetained(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ObjCOwnership(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Pascal(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Pcs(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__PreserveAll(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__PreserveMost(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__RegCall(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__StdCall(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SwiftAsyncCall(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SwiftCall(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SysVABI(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ThisCall(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__VectorCall(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SwiftAsyncContext(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SwiftContext(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SwiftErrorResult(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SwiftIndirectResult(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Annotate(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CFConsumed(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CarriesDependency(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NSConsumed(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NonNull(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__OSConsumed(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__PassObjectSize(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ReleaseHandle(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__UseHandle(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__HLSLSV_DispatchThreadID(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__HLSLSV_GroupIndex(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AMDGPUFlatWorkGroupSize(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AMDGPUNumSGPR(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AMDGPUNumVGPR(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AMDGPUWavesPerEU(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ARMInterrupt(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AVRInterrupt(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AVRSignal(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AcquireCapability(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AcquiredAfter(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AcquiredBefore(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AlignMac68k(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AlignNatural(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Aligned(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AllocAlign(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AllocSize(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AlwaysDestroy(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AnalyzerNoReturn(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AnyX86Interrupt(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AnyX86NoCallerSavedRegisters(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ArcWeakrefUnavailable(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ArgumentWithTypeTag(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ArmBuiltinAlias(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ArmLocallyStreaming(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ArmNew(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Artificial(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AsmLabel(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AssertCapability(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AssertExclusiveLock(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AssertSharedLock(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AssumeAligned(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Assumption(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Availability(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AvailableOnlyInDefaultEvalMethod(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__BPFPreserveAccessIndex(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__BPFPreserveStaticOffset(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__BTFDeclTag(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Blocks(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Builtin(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__C11NoReturn(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CFAuditedTransfer(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CFGuard(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CFICanonicalJumpTable(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CFReturnsNotRetained(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CFReturnsRetained(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CFUnknownTransfer(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CPUDispatch(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CPUSpecific(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CUDAConstant(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CUDADevice(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CUDADeviceBuiltinSurfaceType(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CUDADeviceBuiltinTextureType(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CUDAGlobal(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CUDAHost(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CUDAInvalidTarget(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CUDALaunchBounds(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CUDAShared(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CXX11NoReturn(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CallableWhen(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Callback(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Capability(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CapturedRecord(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Cleanup(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CmseNSEntry(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CodeModel(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CodeSeg(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Cold(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Common(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Const(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ConstInit(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Constructor(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Consumable(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ConsumableAutoCast(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ConsumableSetOnRead(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Convergent(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CoroDisableLifetimeBound(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CoroLifetimeBound(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CoroOnlyDestroyWhenComplete(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CoroReturnType(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CoroWrapper(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CountedBy(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__DLLExport(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__DLLExportStaticLocal(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__DLLImport(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__DLLImportStaticLocal(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Deprecated(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Destructor(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__DiagnoseAsBuiltin(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__DiagnoseIf(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__DisableSanitizerInstrumentation(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__DisableTailCalls(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__EmptyBases(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__EnableIf(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__EnforceTCB(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__EnforceTCBLeaf(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__EnumExtensibility(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Error(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ExcludeFromExplicitInstantiation(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ExclusiveTrylockFunction(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ExternalSourceSymbol(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Final(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__FlagEnum(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Flatten(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Format(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__FormatArg(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__FunctionReturnThunks(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__GNUInline(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__GuardedBy(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__GuardedVar(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__HIPManaged(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__HLSLNumThreads(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__HLSLResource(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__HLSLResourceBinding(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__HLSLShader(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Hot(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__IBAction(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__IBOutlet(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__IBOutletCollection(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__InitPriority(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__InternalLinkage(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__LTOVisibilityPublic(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__LayoutVersion(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Leaf(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__LockReturned(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__LocksExcluded(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__M68kInterrupt(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__MIGServerRoutine(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__MSAllocator(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__MSConstexpr(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__MSInheritance(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__MSNoVTable(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__MSP430Interrupt(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__MSStruct(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__MSVtorDisp(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__MaxFieldAlignment(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__MayAlias(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__MaybeUndef(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__MicroMips(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__MinSize(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__MinVectorWidth(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Mips16(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__MipsInterrupt(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__MipsLongCall(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__MipsShortCall(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NSConsumesSelf(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NSErrorDomain(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NSReturnsAutoreleased(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NSReturnsNotRetained(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NVPTXKernel(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Naked(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NoAlias(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NoCommon(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NoDebug(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NoDestroy(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NoDuplicate(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NoInstrumentFunction(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NoMicroMips(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NoMips16(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NoProfileFunction(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NoRandomizeLayout(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NoReturn(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NoSanitize(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NoSpeculativeLoadHardening(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NoSplitStack(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NoStackProtector(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NoThreadSafetyAnalysis(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NoThrow(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NoUniqueAddress(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NoUwtable(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NotTailCalled(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__OMPAllocateDecl(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__OMPCaptureNoInit(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__OMPDeclareTargetDecl(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__OMPDeclareVariant(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__OMPThreadPrivateDecl(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__OSConsumesThis(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__OSReturnsNotRetained(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__OSReturnsRetained(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__OSReturnsRetainedOnNonZero(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__OSReturnsRetainedOnZero(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ObjCBridge(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ObjCBridgeMutable(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ObjCBridgeRelated(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ObjCException(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ObjCExplicitProtocolImpl(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ObjCExternallyRetained(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ObjCIndependentClass(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ObjCMethodFamily(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ObjCNSObject(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ObjCPreciseLifetime(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ObjCRequiresPropertyDefs(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ObjCRequiresSuper(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ObjCReturnsInnerPointer(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ObjCRootClass(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ObjCSubclassingRestricted(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__OpenCLIntelReqdSubGroupSize(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__OpenCLKernel(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__OptimizeNone(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Override(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Owner(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Ownership(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Packed(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ParamTypestate(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__PatchableFunctionEntry(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Pointer(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__PragmaClangBSSSection(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__PragmaClangDataSection(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__PragmaClangRelroSection(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__PragmaClangRodataSection(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__PragmaClangTextSection(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__PreferredName(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__PreferredType(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__PtGuardedBy(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__PtGuardedVar(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Pure(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__RISCVInterrupt(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__RandomizeLayout(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ReadOnlyPlacement(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Reinitializes(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ReleaseCapability(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ReqdWorkGroupSize(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__RequiresCapability(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Restrict(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Retain(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ReturnTypestate(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ReturnsNonNull(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ReturnsTwice(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SYCLKernel(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SYCLSpecialClass(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ScopedLockable(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Section(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SelectAny(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Sentinel(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SetTypestate(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SharedTrylockFunction(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SpeculativeLoadHardening(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__StandaloneDebug(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__StrictFP(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__StrictGuardStackCheck(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SwiftAsync(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SwiftAsyncError(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SwiftAsyncName(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SwiftAttr(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SwiftBridge(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SwiftBridgedTypedef(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SwiftError(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SwiftImportAsNonGeneric(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SwiftImportPropertyAsAccessors(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SwiftName(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SwiftNewType(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SwiftPrivate(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__TLSModel(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Target(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__TargetClones(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__TargetVersion(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__TestTypestate(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__TransparentUnion(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__TrivialABI(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__TryAcquireCapability(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__TypeTagForDatatype(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__TypeVisibility(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Unavailable(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Uninitialized(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__UnsafeBufferUsage(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Unused(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Used(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__UsingIfExists(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Uuid(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__VecReturn(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__VecTypeHint(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Visibility(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__WarnUnused(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__WarnUnusedResult(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Weak(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__WeakImport(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__WeakRef(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__WebAssemblyExportName(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__WebAssemblyImportModule(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__WebAssemblyImportName(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__WorkGroupSizeHint(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__X86ForceAlignArgPointer(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__XRayInstrument(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__XRayLogArgs(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ZeroCallUsedRegs(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AbiTag(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Alias(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__AlignValue(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__BuiltinAlias(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__CalledOnce(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__IFunc(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__InitSeg(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__LoaderUninitialized(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__LoopHint(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Mode(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NoBuiltin(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__NoEscape(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__OMPCaptureKind(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__OMPDeclareSimdDecl(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__OMPReferencedVar(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ObjCBoxable(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ObjCClassStub(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ObjCDesignatedInitializer(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ObjCDirect(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ObjCDirectMembers(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ObjCNonLazyClass(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ObjCNonRuntimeProtocol(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ObjCRuntimeName(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__ObjCRuntimeVisible(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__OpenCLAccess(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Overloadable(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__RenderScriptKernel(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SwiftObjCMembers(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SwiftVersionedAddition(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__SwiftVersionedRemoval(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__attr__Thread(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__Decl__ModuleOwnershipKind__Unowned(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__Decl__ModuleOwnershipKind__Visible(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__Decl__ModuleOwnershipKind__VisibleWhenImported(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__Decl__ModuleOwnershipKind__ReachableWhenImported(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__Decl__ModuleOwnershipKind__ModulePrivate(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ConstantResultStorageKind__None(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ConstantResultStorageKind__Int64(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ConstantResultStorageKind__APValue(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__Linkage__Invalid(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__Linkage__None(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__Linkage__Internal(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__Linkage__UniqueExternal(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__Linkage__VisibleNone(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__Linkage__Module(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__Linkage__External(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ArraySizeModifier__Normal(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ArraySizeModifier__Static(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ArraySizeModifier__Star(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AutoTypeKeyword__Auto(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AutoTypeKeyword__DecltypeAuto(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AutoTypeKeyword__GNUAutoType(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__EST_None(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__EST_DynamicNone(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__EST_Dynamic(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__EST_MSAny(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__EST_NoThrow(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__EST_BasicNoexcept(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__EST_DependentNoexcept(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__EST_NoexceptFalse(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__EST_NoexceptTrue(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__EST_Unevaluated(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__EST_Uninstantiated(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__EST_Unparsed(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CXXConstructionKind__Complete(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CXXConstructionKind__NonVirtualBase(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CXXConstructionKind__VirtualBase(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CXXConstructionKind__Delegating(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ImplicitParamKind__ObjCSelf(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ImplicitParamKind__ObjCCmd(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ImplicitParamKind__CXXThis(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ImplicitParamKind__CXXVTT(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ImplicitParamKind__CapturedContext(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ImplicitParamKind__ThreadPrivateVar(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ImplicitParamKind__Other(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CR_Default(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CR_ObjCAtFinally(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CR_OpenMP(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CFGElement__Initializer(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CFGElement__ScopeBegin(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CFGElement__ScopeEnd(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CFGElement__NewAllocator(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CFGElement__LifetimeEnds(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CFGElement__LoopExit(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CFGElement__Statement(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CFGElement__Constructor(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CFGElement__CXXRecordTypedCall(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CFGElement__AutomaticObjectDtor(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CFGElement__DeleteDtor(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CFGElement__BaseDtor(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CFGElement__MemberDtor(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CFGElement__TemporaryDtor(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CFGElement__CleanupFunction(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UO_PostInc(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UO_PostDec(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UO_PreInc(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UO_PreDec(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UO_AddrOf(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UO_Deref(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UO_Plus(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UO_Minus(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UO_Not(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UO_LNot(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UO_Real(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UO_Imag(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UO_Extension(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UO_Coawait(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CXXNewInitializationStyle__None(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CXXNewInitializationStyle__Parens(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CXXNewInitializationStyle__Braces(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_Dependent(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_BitCast(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_LValueBitCast(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_LValueToRValueBitCast(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_LValueToRValue(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_NoOp(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_BaseToDerived(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_DerivedToBase(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_UncheckedDerivedToBase(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_Dynamic(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_ToUnion(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_ArrayToPointerDecay(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_FunctionToPointerDecay(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_NullToPointer(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_NullToMemberPointer(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_BaseToDerivedMemberPointer(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_DerivedToBaseMemberPointer(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_MemberPointerToBoolean(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_ReinterpretMemberPointer(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_UserDefinedConversion(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_ConstructorConversion(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_IntegralToPointer(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_PointerToIntegral(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_PointerToBoolean(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_ToVoid(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_MatrixCast(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_VectorSplat(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_IntegralCast(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_IntegralToBoolean(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_IntegralToFloating(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_FloatingToFixedPoint(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_FixedPointToFloating(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_FixedPointCast(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_FixedPointToIntegral(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_IntegralToFixedPoint(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_FixedPointToBoolean(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_FloatingToIntegral(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_FloatingToBoolean(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_BooleanToSignedIntegral(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_FloatingCast(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_CPointerToObjCPointerCast(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_BlockPointerToObjCPointerCast(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_AnyPointerToBlockPointerCast(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_ObjCObjectLValueCast(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_FloatingRealToComplex(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_FloatingComplexToReal(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_FloatingComplexToBoolean(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_FloatingComplexCast(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_FloatingComplexToIntegralComplex(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_IntegralRealToComplex(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_IntegralComplexToReal(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_IntegralComplexToBoolean(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_IntegralComplexCast(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_IntegralComplexToFloatingComplex(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_ARCProduceObject(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_ARCConsumeObject(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_ARCReclaimReturnedObject(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_ARCExtendBlockObject(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_AtomicToNonAtomic(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_NonAtomicToAtomic(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_CopyAndAutoreleaseBlockObject(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_BuiltinFnToFnPtr(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_ZeroToOCLOpaqueType(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_AddressSpaceConversion(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CK_IntToOCLSampler(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ElaboratedTypeKeyword__Struct(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ElaboratedTypeKeyword__Interface(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ElaboratedTypeKeyword__Union(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ElaboratedTypeKeyword__Class(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ElaboratedTypeKeyword__Enum(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ElaboratedTypeKeyword__Typename(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ElaboratedTypeKeyword__None(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__VectorKind__Generic(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__VectorKind__AltiVecVector(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__VectorKind__AltiVecPixel(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__VectorKind__AltiVecBool(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__VectorKind__Neon(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__VectorKind__NeonPoly(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__VectorKind__SveFixedLengthData(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__VectorKind__SveFixedLengthPredicate(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__VectorKind__RVVFixedLengthData(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__VectorKind__RVVFixedLengthMask(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__TagTypeKind__Struct(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__TagTypeKind__Interface(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__TagTypeKind__Union(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__TagTypeKind__Class(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__TagTypeKind__Enum(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CT_Cannot(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CT_Dependent(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CT_Can(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BTK__make_integer_seq(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BTK__type_pack_element(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__IfStatementKind__Ordinary(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__IfStatementKind__Constexpr(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__IfStatementKind__ConstevalNonNegated(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__IfStatementKind__ConstevalNegated(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CharacterLiteralKind__Ascii(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CharacterLiteralKind__Wide(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CharacterLiteralKind__UTF8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CharacterLiteralKind__UTF16(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CharacterLiteralKind__UTF32(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__SD_FullExpression(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__SD_Automatic(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__SD_Thread(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__SD_Static(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__SD_Dynamic(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ET_IsLValueExpr(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ET_IsRValueExpr(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__MultiVersionKind__None(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__MultiVersionKind__Target(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__MultiVersionKind__CPUSpecific(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__MultiVersionKind__CPUDispatch(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__MultiVersionKind__TargetClones(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__MultiVersionKind__TargetVersion(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UnaryTransformType__AddLvalueReference(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UnaryTransformType__AddPointer(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UnaryTransformType__AddRvalueReference(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UnaryTransformType__Decay(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UnaryTransformType__MakeSigned(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UnaryTransformType__MakeUnsigned(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UnaryTransformType__RemoveAllExtents(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UnaryTransformType__RemoveConst(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UnaryTransformType__RemoveCV(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UnaryTransformType__RemoveCVRef(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UnaryTransformType__RemoveExtent(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UnaryTransformType__RemovePointer(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UnaryTransformType__RemoveReference(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UnaryTransformType__RemoveRestrict(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UnaryTransformType__RemoveVolatile(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UnaryTransformType__EnumUnderlyingType(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__NOUR_None(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__NOUR_Unevaluated(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__NOUR_Constant(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__NOUR_Discarded(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__HiddenVisibility(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ProtectedVisibility(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__DefaultVisibility(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__SFF_None(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__SFF_NSString(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__SFF_CFString(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__VarDecl__DeclarationOnly(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__VarDecl__TentativeDefinition(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__VarDecl__Definition(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ConstexprSpecKind__Unspecified(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ConstexprSpecKind__Constexpr(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ConstexprSpecKind__Consteval(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__ConstexprSpecKind__Constinit(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CallExpr__ADLCallKind__NotADL(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CallExpr__ADLCallKind__UsesADL(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_llvm__APFloatBase__S_IEEEhalf(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_llvm__APFloatBase__S_BFloat(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_llvm__APFloatBase__S_IEEEsingle(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_llvm__APFloatBase__S_IEEEdouble(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_llvm__APFloatBase__S_IEEEquad(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_llvm__APFloatBase__S_PPCDoubleDouble(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_llvm__APFloatBase__S_Float8E5M2(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_llvm__APFloatBase__S_Float8E5M2FNUZ(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_llvm__APFloatBase__S_Float8E4M3FN(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_llvm__APFloatBase__S_Float8E4M3FNUZ(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_llvm__APFloatBase__S_Float8E4M3B11FNUZ(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_llvm__APFloatBase__S_FloatTF32(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_llvm__APFloatBase__S_x87DoubleExtended(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__Decl__OBJC_TQ_None(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__Decl__OBJC_TQ_In(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__Decl__OBJC_TQ_Inout(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__Decl__OBJC_TQ_Out(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__Decl__OBJC_TQ_Bycopy(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__Decl__OBJC_TQ_Byref(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__Decl__OBJC_TQ_Oneway(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__Decl__OBJC_TQ_CSNullability(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_None(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_New(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_Delete(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_Array_New(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_Array_Delete(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_Plus(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_Minus(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_Star(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_Slash(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_Percent(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_Caret(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_Amp(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_Pipe(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_Tilde(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_Exclaim(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_Equal(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_Less(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_Greater(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_PlusEqual(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_MinusEqual(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_StarEqual(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_SlashEqual(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_PercentEqual(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_CaretEqual(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_AmpEqual(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_PipeEqual(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_LessLess(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_GreaterGreater(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_LessLessEqual(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_GreaterGreaterEqual(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_EqualEqual(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_ExclaimEqual(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_LessEqual(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_GreaterEqual(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_Spaceship(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_AmpAmp(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_PipePipe(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_PlusPlus(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_MinusMinus(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_Comma(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_ArrowStar(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_Arrow(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_Call(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_Subscript(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_Conditional(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__OO_Coawait(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__NUM_OVERLOADED_OPERATORS(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_PtrMemD(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_PtrMemI(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_Mul(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_Div(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_Rem(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_Add(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_Sub(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_Shl(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_Shr(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_Cmp(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_LT(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_GT(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_LE(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_GE(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_EQ(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_NE(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_And(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_Xor(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_Or(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_LAnd(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_LOr(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_Assign(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_MulAssign(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_DivAssign(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_RemAssign(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_AddAssign(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_SubAssign(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_ShlAssign(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_ShrAssign(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_AndAssign(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_XorAssign(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_OrAssign(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BO_Comma(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__SC_None(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__SC_Extern(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__SC_Static(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__SC_PrivateExtern(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__SC_Auto(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__SC_Register(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__RecordArgPassingKind__CanPassInRegs(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__RecordArgPassingKind__CannotPassInRegs(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__RecordArgPassingKind__CanNeverPassInRegs(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__RQ_None(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__RQ_LValue(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__RQ_RValue(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__SourceLocIdentKind__Function(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__SourceLocIdentKind__FuncSig(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__SourceLocIdentKind__File(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__SourceLocIdentKind__FileName(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__SourceLocIdentKind__Line(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__SourceLocIdentKind__Column(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__SourceLocIdentKind__SourceLocStruct(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__StringLiteralKind__Ordinary(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__StringLiteralKind__Wide(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__StringLiteralKind__UTF8(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__StringLiteralKind__UTF16(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__StringLiteralKind__UTF32(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__StringLiteralKind__Unevaluated(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__TSK_Undeclared(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__TSK_ImplicitInstantiation(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__TSK_ExplicitSpecialization(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__TSK_ExplicitInstantiationDeclaration(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__TSK_ExplicitInstantiationDefinition(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__DeductionCandidate__Normal(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__DeductionCandidate__Copy(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__DeductionCandidate__Aggregate(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__TSCS_unspecified(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__TSCS___thread(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__TSCS_thread_local(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__TSCS__Thread_local(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__TypeOfKind__Qualified(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__TypeOfKind__Unqualified(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__PCK_Unknown(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__PCK_Linker(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__PCK_Lib(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__PCK_Compiler(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__PCK_ExeStr(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__PCK_User(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsInterfaceClass(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsSealed(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsDestructible(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsTriviallyDestructible(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsNothrowDestructible(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_HasNothrowMoveAssign(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_HasTrivialMoveAssign(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_HasTrivialMoveConstructor(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_HasNothrowAssign(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_HasNothrowCopy(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_HasNothrowConstructor(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_HasTrivialAssign(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_HasTrivialCopy(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_HasTrivialDefaultConstructor(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_HasTrivialDestructor(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_HasVirtualDestructor(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsAbstract(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsAggregate(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsClass(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsEmpty(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsEnum(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsFinal(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsLiteral(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsPOD(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsPolymorphic(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsStandardLayout(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsTrivial(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsTriviallyCopyable(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsUnion(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_HasUniqueObjectRepresentations(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsTriviallyRelocatable(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsTriviallyEqualityComparable(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsBoundedArray(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsUnboundedArray(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsNullPointer(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsScopedEnum(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsReferenceable(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_CanPassInRegs(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsArithmetic(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsFloatingPoint(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsIntegral(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsCompleteType(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsVoid(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsArray(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsFunction(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsReference(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsLvalueReference(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsRvalueReference(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsFundamental(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsObject(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsScalar(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsCompound(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsPointer(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsMemberObjectPointer(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsMemberFunctionPointer(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsMemberPointer(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsConst(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsVolatile(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsSigned(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UTT_IsUnsigned(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BTT_TypeCompatible(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BTT_IsNothrowAssignable(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BTT_IsAssignable(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BTT_IsBaseOf(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BTT_IsConvertibleTo(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BTT_IsTriviallyAssignable(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BTT_ReferenceBindsToTemporary(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BTT_ReferenceConstructsFromTemporary(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BTT_IsSame(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__BTT_IsConvertible(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__TT_IsConstructible(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__TT_IsNothrowConstructible(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__TT_IsTriviallyConstructible(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UETT_SizeOf(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UETT_DataSizeOf(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UETT_AlignOf(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UETT_PreferredAlignOf(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UETT_VecStep(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UETT_OpenMPRequiredSimdAlign(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__UETT_VectorElements(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__APValue__None(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__APValue__Indeterminate(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__APValue__Int(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__APValue__Float(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__APValue__FixedPoint(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__APValue__ComplexInt(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__APValue__ComplexFloat(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__APValue__LValue(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__APValue__Vector(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__APValue__Array(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__APValue__Struct(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__APValue__Union(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__APValue__MemberPointer(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__APValue__AddrLabelDiff(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AS_public(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AS_protected(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AS_private(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AS_none(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__c11_atomic_init(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__c11_atomic_load(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__c11_atomic_store(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__c11_atomic_exchange(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__c11_atomic_compare_exchange_strong(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__c11_atomic_compare_exchange_weak(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__c11_atomic_fetch_add(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__c11_atomic_fetch_sub(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__c11_atomic_fetch_and(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__c11_atomic_fetch_or(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__c11_atomic_fetch_xor(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__c11_atomic_fetch_nand(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__c11_atomic_fetch_max(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__c11_atomic_fetch_min(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__atomic_load(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__atomic_load_n(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__atomic_store(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__atomic_store_n(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__atomic_exchange(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__atomic_exchange_n(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__atomic_compare_exchange(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__atomic_compare_exchange_n(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__atomic_fetch_add(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__atomic_fetch_sub(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__atomic_fetch_and(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__atomic_fetch_or(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__atomic_fetch_xor(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__atomic_fetch_nand(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__atomic_add_fetch(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__atomic_sub_fetch(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__atomic_and_fetch(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__atomic_or_fetch(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__atomic_xor_fetch(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__atomic_max_fetch(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__atomic_min_fetch(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__atomic_nand_fetch(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__scoped_atomic_load(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__scoped_atomic_load_n(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__scoped_atomic_store(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__scoped_atomic_store_n(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__scoped_atomic_exchange(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__scoped_atomic_exchange_n(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__scoped_atomic_compare_exchange(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__scoped_atomic_compare_exchange_n(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__scoped_atomic_fetch_add(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__scoped_atomic_fetch_sub(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__scoped_atomic_fetch_and(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__scoped_atomic_fetch_or(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__scoped_atomic_fetch_xor(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__scoped_atomic_fetch_nand(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__scoped_atomic_add_fetch(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__scoped_atomic_sub_fetch(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__scoped_atomic_and_fetch(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__scoped_atomic_or_fetch(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__scoped_atomic_xor_fetch(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__scoped_atomic_max_fetch(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__scoped_atomic_min_fetch(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__scoped_atomic_nand_fetch(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__scoped_atomic_fetch_min(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__scoped_atomic_fetch_max(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__opencl_atomic_init(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__opencl_atomic_load(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__opencl_atomic_store(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__opencl_atomic_exchange(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__opencl_atomic_compare_exchange_strong(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__opencl_atomic_compare_exchange_weak(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__opencl_atomic_fetch_add(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__opencl_atomic_fetch_sub(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__opencl_atomic_fetch_and(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__opencl_atomic_fetch_or(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__opencl_atomic_fetch_xor(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__opencl_atomic_fetch_min(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__opencl_atomic_fetch_max(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__atomic_fetch_min(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__atomic_fetch_max(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__hip_atomic_load(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__hip_atomic_store(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__hip_atomic_compare_exchange_weak(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__hip_atomic_compare_exchange_strong(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__hip_atomic_exchange(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__hip_atomic_fetch_add(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__hip_atomic_fetch_sub(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__hip_atomic_fetch_and(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__hip_atomic_fetch_or(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__hip_atomic_fetch_xor(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__hip_atomic_fetch_min(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__AtomicExpr__AO__hip_atomic_fetch_max(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CFGTerminator__StmtBranch(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CFGTerminator__TemporaryDtorsBranch(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__CFGTerminator__VirtualBaseBranch(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__Decl__FOK_None(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__Decl__FOK_Declared(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

#[no_mangle]
pub extern "C" fn arboretum_emit_enum_value_clang__Decl__FOK_Undeclared(id: u64, enum_id: u64) {
  ENUM_VALUE_QUEUE.with(|q| q.borrow_mut().push((id, enum_id)));
}

////   END ARBORETUM GENERATED CODE ////
