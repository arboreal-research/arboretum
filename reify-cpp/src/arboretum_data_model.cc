#include "arboretum_data_model.h"

namespace arboretum {

//// BEGIN ARBORETUM GENERATED CODE ////
DataModel EmitDataModel() {
  DataModel data_model;

  // clang::APValue::ValueKind
  data_model.clang_APValue_ValueKind_ = arboretum_node_new_with_id("enum", "clang::APValue::ValueKind", nullptr);
  data_model.clang_APValue_None_ = arboretum_node_new_with_id("enum_value", "clang::APValue::None", nullptr);
  data_model.clang_APValue_Indeterminate_ = arboretum_node_new_with_id("enum_value", "clang::APValue::Indeterminate", nullptr);
  data_model.clang_APValue_Int_ = arboretum_node_new_with_id("enum_value", "clang::APValue::Int", nullptr);
  data_model.clang_APValue_Float_ = arboretum_node_new_with_id("enum_value", "clang::APValue::Float", nullptr);
  data_model.clang_APValue_FixedPoint_ = arboretum_node_new_with_id("enum_value", "clang::APValue::FixedPoint", nullptr);
  data_model.clang_APValue_ComplexInt_ = arboretum_node_new_with_id("enum_value", "clang::APValue::ComplexInt", nullptr);
  data_model.clang_APValue_ComplexFloat_ = arboretum_node_new_with_id("enum_value", "clang::APValue::ComplexFloat", nullptr);
  data_model.clang_APValue_LValue_ = arboretum_node_new_with_id("enum_value", "clang::APValue::LValue", nullptr);
  data_model.clang_APValue_Vector_ = arboretum_node_new_with_id("enum_value", "clang::APValue::Vector", nullptr);
  data_model.clang_APValue_Array_ = arboretum_node_new_with_id("enum_value", "clang::APValue::Array", nullptr);
  data_model.clang_APValue_Struct_ = arboretum_node_new_with_id("enum_value", "clang::APValue::Struct", nullptr);
  data_model.clang_APValue_Union_ = arboretum_node_new_with_id("enum_value", "clang::APValue::Union", nullptr);
  data_model.clang_APValue_MemberPointer_ = arboretum_node_new_with_id("enum_value", "clang::APValue::MemberPointer", nullptr);
  data_model.clang_APValue_AddrLabelDiff_ = arboretum_node_new_with_id("enum_value", "clang::APValue::AddrLabelDiff", nullptr);

  // clang::AccessSpecifier
  data_model.clang_AccessSpecifier_ = arboretum_node_new_with_id("enum", "clang::AccessSpecifier", nullptr);
  data_model.clang_AS_public_ = arboretum_node_new_with_id("enum_value", "clang::AS_public", nullptr);
  data_model.clang_AS_protected_ = arboretum_node_new_with_id("enum_value", "clang::AS_protected", nullptr);
  data_model.clang_AS_private_ = arboretum_node_new_with_id("enum_value", "clang::AS_private", nullptr);
  data_model.clang_AS_none_ = arboretum_node_new_with_id("enum_value", "clang::AS_none", nullptr);

  // clang::ArrayType::ArraySizeModifier
  data_model.clang_ArrayType_ArraySizeModifier_ = arboretum_node_new_with_id("enum", "clang::ArrayType::ArraySizeModifier", nullptr);
  data_model.clang_ArrayType_Normal_ = arboretum_node_new_with_id("enum_value", "clang::ArrayType::Normal", nullptr);
  data_model.clang_ArrayType_Static_ = arboretum_node_new_with_id("enum_value", "clang::ArrayType::Static", nullptr);
  data_model.clang_ArrayType_Star_ = arboretum_node_new_with_id("enum_value", "clang::ArrayType::Star", nullptr);

  // clang::ArrayTypeTrait
  data_model.clang_ArrayTypeTrait_ = arboretum_node_new_with_id("enum", "clang::ArrayTypeTrait", nullptr);
  data_model.clang_ATT_ArrayRank_ = arboretum_node_new_with_id("enum_value", "clang::ATT_ArrayRank", nullptr);
  data_model.clang_ATT_ArrayExtent_ = arboretum_node_new_with_id("enum_value", "clang::ATT_ArrayExtent", nullptr);
  data_model.clang_ATT_Last_ = arboretum_node_new_with_id("enum_value", "clang::ATT_Last", nullptr);

  // clang::AtomicExpr::AtomicOp
  data_model.clang_AtomicExpr_AtomicOp_ = arboretum_node_new_with_id("enum", "clang::AtomicExpr::AtomicOp", nullptr);
  data_model.clang_AtomicExpr_AO__c11_atomic_init_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__c11_atomic_init", nullptr);
  data_model.clang_AtomicExpr_AO__c11_atomic_load_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__c11_atomic_load", nullptr);
  data_model.clang_AtomicExpr_AO__c11_atomic_store_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__c11_atomic_store", nullptr);
  data_model.clang_AtomicExpr_AO__c11_atomic_exchange_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__c11_atomic_exchange", nullptr);
  data_model.clang_AtomicExpr_AO__c11_atomic_compare_exchange_strong_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__c11_atomic_compare_exchange_strong", nullptr);
  data_model.clang_AtomicExpr_AO__c11_atomic_compare_exchange_weak_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__c11_atomic_compare_exchange_weak", nullptr);
  data_model.clang_AtomicExpr_AO__c11_atomic_fetch_add_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__c11_atomic_fetch_add", nullptr);
  data_model.clang_AtomicExpr_AO__c11_atomic_fetch_sub_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__c11_atomic_fetch_sub", nullptr);
  data_model.clang_AtomicExpr_AO__c11_atomic_fetch_and_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__c11_atomic_fetch_and", nullptr);
  data_model.clang_AtomicExpr_AO__c11_atomic_fetch_or_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__c11_atomic_fetch_or", nullptr);
  data_model.clang_AtomicExpr_AO__c11_atomic_fetch_xor_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__c11_atomic_fetch_xor", nullptr);
  data_model.clang_AtomicExpr_AO__c11_atomic_fetch_nand_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__c11_atomic_fetch_nand", nullptr);
  data_model.clang_AtomicExpr_AO__c11_atomic_fetch_max_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__c11_atomic_fetch_max", nullptr);
  data_model.clang_AtomicExpr_AO__c11_atomic_fetch_min_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__c11_atomic_fetch_min", nullptr);
  data_model.clang_AtomicExpr_AO__atomic_load_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__atomic_load", nullptr);
  data_model.clang_AtomicExpr_AO__atomic_load_n_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__atomic_load_n", nullptr);
  data_model.clang_AtomicExpr_AO__atomic_store_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__atomic_store", nullptr);
  data_model.clang_AtomicExpr_AO__atomic_store_n_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__atomic_store_n", nullptr);
  data_model.clang_AtomicExpr_AO__atomic_exchange_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__atomic_exchange", nullptr);
  data_model.clang_AtomicExpr_AO__atomic_exchange_n_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__atomic_exchange_n", nullptr);
  data_model.clang_AtomicExpr_AO__atomic_compare_exchange_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__atomic_compare_exchange", nullptr);
  data_model.clang_AtomicExpr_AO__atomic_compare_exchange_n_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__atomic_compare_exchange_n", nullptr);
  data_model.clang_AtomicExpr_AO__atomic_fetch_add_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__atomic_fetch_add", nullptr);
  data_model.clang_AtomicExpr_AO__atomic_fetch_sub_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__atomic_fetch_sub", nullptr);
  data_model.clang_AtomicExpr_AO__atomic_fetch_and_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__atomic_fetch_and", nullptr);
  data_model.clang_AtomicExpr_AO__atomic_fetch_or_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__atomic_fetch_or", nullptr);
  data_model.clang_AtomicExpr_AO__atomic_fetch_xor_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__atomic_fetch_xor", nullptr);
  data_model.clang_AtomicExpr_AO__atomic_fetch_nand_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__atomic_fetch_nand", nullptr);
  data_model.clang_AtomicExpr_AO__atomic_add_fetch_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__atomic_add_fetch", nullptr);
  data_model.clang_AtomicExpr_AO__atomic_sub_fetch_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__atomic_sub_fetch", nullptr);
  data_model.clang_AtomicExpr_AO__atomic_and_fetch_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__atomic_and_fetch", nullptr);
  data_model.clang_AtomicExpr_AO__atomic_or_fetch_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__atomic_or_fetch", nullptr);
  data_model.clang_AtomicExpr_AO__atomic_xor_fetch_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__atomic_xor_fetch", nullptr);
  data_model.clang_AtomicExpr_AO__atomic_max_fetch_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__atomic_max_fetch", nullptr);
  data_model.clang_AtomicExpr_AO__atomic_min_fetch_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__atomic_min_fetch", nullptr);
  data_model.clang_AtomicExpr_AO__atomic_nand_fetch_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__atomic_nand_fetch", nullptr);
  data_model.clang_AtomicExpr_AO__opencl_atomic_init_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__opencl_atomic_init", nullptr);
  data_model.clang_AtomicExpr_AO__opencl_atomic_load_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__opencl_atomic_load", nullptr);
  data_model.clang_AtomicExpr_AO__opencl_atomic_store_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__opencl_atomic_store", nullptr);
  data_model.clang_AtomicExpr_AO__opencl_atomic_exchange_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__opencl_atomic_exchange", nullptr);
  data_model.clang_AtomicExpr_AO__opencl_atomic_compare_exchange_strong_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__opencl_atomic_compare_exchange_strong", nullptr);
  data_model.clang_AtomicExpr_AO__opencl_atomic_compare_exchange_weak_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__opencl_atomic_compare_exchange_weak", nullptr);
  data_model.clang_AtomicExpr_AO__opencl_atomic_fetch_add_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__opencl_atomic_fetch_add", nullptr);
  data_model.clang_AtomicExpr_AO__opencl_atomic_fetch_sub_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__opencl_atomic_fetch_sub", nullptr);
  data_model.clang_AtomicExpr_AO__opencl_atomic_fetch_and_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__opencl_atomic_fetch_and", nullptr);
  data_model.clang_AtomicExpr_AO__opencl_atomic_fetch_or_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__opencl_atomic_fetch_or", nullptr);
  data_model.clang_AtomicExpr_AO__opencl_atomic_fetch_xor_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__opencl_atomic_fetch_xor", nullptr);
  data_model.clang_AtomicExpr_AO__opencl_atomic_fetch_min_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__opencl_atomic_fetch_min", nullptr);
  data_model.clang_AtomicExpr_AO__opencl_atomic_fetch_max_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__opencl_atomic_fetch_max", nullptr);
  data_model.clang_AtomicExpr_AO__atomic_fetch_min_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__atomic_fetch_min", nullptr);
  data_model.clang_AtomicExpr_AO__atomic_fetch_max_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__atomic_fetch_max", nullptr);
  data_model.clang_AtomicExpr_AO__hip_atomic_load_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__hip_atomic_load", nullptr);
  data_model.clang_AtomicExpr_AO__hip_atomic_store_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__hip_atomic_store", nullptr);
  data_model.clang_AtomicExpr_AO__hip_atomic_compare_exchange_weak_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__hip_atomic_compare_exchange_weak", nullptr);
  data_model.clang_AtomicExpr_AO__hip_atomic_compare_exchange_strong_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__hip_atomic_compare_exchange_strong", nullptr);
  data_model.clang_AtomicExpr_AO__hip_atomic_exchange_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__hip_atomic_exchange", nullptr);
  data_model.clang_AtomicExpr_AO__hip_atomic_fetch_add_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__hip_atomic_fetch_add", nullptr);
  data_model.clang_AtomicExpr_AO__hip_atomic_fetch_sub_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__hip_atomic_fetch_sub", nullptr);
  data_model.clang_AtomicExpr_AO__hip_atomic_fetch_and_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__hip_atomic_fetch_and", nullptr);
  data_model.clang_AtomicExpr_AO__hip_atomic_fetch_or_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__hip_atomic_fetch_or", nullptr);
  data_model.clang_AtomicExpr_AO__hip_atomic_fetch_xor_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__hip_atomic_fetch_xor", nullptr);
  data_model.clang_AtomicExpr_AO__hip_atomic_fetch_min_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__hip_atomic_fetch_min", nullptr);
  data_model.clang_AtomicExpr_AO__hip_atomic_fetch_max_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::AO__hip_atomic_fetch_max", nullptr);
  data_model.clang_AtomicExpr_BI_First_ = arboretum_node_new_with_id("enum_value", "clang::AtomicExpr::BI_First", nullptr);

  // clang::AutoTypeKeyword
  data_model.clang_AutoTypeKeyword_ = arboretum_node_new_with_id("enum", "clang::AutoTypeKeyword", nullptr);
  data_model.clang_AutoTypeKeyword_Auto_ = arboretum_node_new_with_id("enum_value", "clang::AutoTypeKeyword::Auto", nullptr);
  data_model.clang_AutoTypeKeyword_DecltypeAuto_ = arboretum_node_new_with_id("enum_value", "clang::AutoTypeKeyword::DecltypeAuto", nullptr);
  data_model.clang_AutoTypeKeyword_GNUAutoType_ = arboretum_node_new_with_id("enum_value", "clang::AutoTypeKeyword::GNUAutoType", nullptr);

  // clang::BinaryOperatorKind
  data_model.clang_BinaryOperatorKind_ = arboretum_node_new_with_id("enum", "clang::BinaryOperatorKind", nullptr);
  data_model.clang_BO_PtrMemD_ = arboretum_node_new_with_id("enum_value", "clang::BO_PtrMemD", nullptr);
  data_model.clang_BO_PtrMemI_ = arboretum_node_new_with_id("enum_value", "clang::BO_PtrMemI", nullptr);
  data_model.clang_BO_Mul_ = arboretum_node_new_with_id("enum_value", "clang::BO_Mul", nullptr);
  data_model.clang_BO_Div_ = arboretum_node_new_with_id("enum_value", "clang::BO_Div", nullptr);
  data_model.clang_BO_Rem_ = arboretum_node_new_with_id("enum_value", "clang::BO_Rem", nullptr);
  data_model.clang_BO_Add_ = arboretum_node_new_with_id("enum_value", "clang::BO_Add", nullptr);
  data_model.clang_BO_Sub_ = arboretum_node_new_with_id("enum_value", "clang::BO_Sub", nullptr);
  data_model.clang_BO_Shl_ = arboretum_node_new_with_id("enum_value", "clang::BO_Shl", nullptr);
  data_model.clang_BO_Shr_ = arboretum_node_new_with_id("enum_value", "clang::BO_Shr", nullptr);
  data_model.clang_BO_Cmp_ = arboretum_node_new_with_id("enum_value", "clang::BO_Cmp", nullptr);
  data_model.clang_BO_LT_ = arboretum_node_new_with_id("enum_value", "clang::BO_LT", nullptr);
  data_model.clang_BO_GT_ = arboretum_node_new_with_id("enum_value", "clang::BO_GT", nullptr);
  data_model.clang_BO_LE_ = arboretum_node_new_with_id("enum_value", "clang::BO_LE", nullptr);
  data_model.clang_BO_GE_ = arboretum_node_new_with_id("enum_value", "clang::BO_GE", nullptr);
  data_model.clang_BO_EQ_ = arboretum_node_new_with_id("enum_value", "clang::BO_EQ", nullptr);
  data_model.clang_BO_NE_ = arboretum_node_new_with_id("enum_value", "clang::BO_NE", nullptr);
  data_model.clang_BO_And_ = arboretum_node_new_with_id("enum_value", "clang::BO_And", nullptr);
  data_model.clang_BO_Xor_ = arboretum_node_new_with_id("enum_value", "clang::BO_Xor", nullptr);
  data_model.clang_BO_Or_ = arboretum_node_new_with_id("enum_value", "clang::BO_Or", nullptr);
  data_model.clang_BO_LAnd_ = arboretum_node_new_with_id("enum_value", "clang::BO_LAnd", nullptr);
  data_model.clang_BO_LOr_ = arboretum_node_new_with_id("enum_value", "clang::BO_LOr", nullptr);
  data_model.clang_BO_Assign_ = arboretum_node_new_with_id("enum_value", "clang::BO_Assign", nullptr);
  data_model.clang_BO_MulAssign_ = arboretum_node_new_with_id("enum_value", "clang::BO_MulAssign", nullptr);
  data_model.clang_BO_DivAssign_ = arboretum_node_new_with_id("enum_value", "clang::BO_DivAssign", nullptr);
  data_model.clang_BO_RemAssign_ = arboretum_node_new_with_id("enum_value", "clang::BO_RemAssign", nullptr);
  data_model.clang_BO_AddAssign_ = arboretum_node_new_with_id("enum_value", "clang::BO_AddAssign", nullptr);
  data_model.clang_BO_SubAssign_ = arboretum_node_new_with_id("enum_value", "clang::BO_SubAssign", nullptr);
  data_model.clang_BO_ShlAssign_ = arboretum_node_new_with_id("enum_value", "clang::BO_ShlAssign", nullptr);
  data_model.clang_BO_ShrAssign_ = arboretum_node_new_with_id("enum_value", "clang::BO_ShrAssign", nullptr);
  data_model.clang_BO_AndAssign_ = arboretum_node_new_with_id("enum_value", "clang::BO_AndAssign", nullptr);
  data_model.clang_BO_XorAssign_ = arboretum_node_new_with_id("enum_value", "clang::BO_XorAssign", nullptr);
  data_model.clang_BO_OrAssign_ = arboretum_node_new_with_id("enum_value", "clang::BO_OrAssign", nullptr);
  data_model.clang_BO_Comma_ = arboretum_node_new_with_id("enum_value", "clang::BO_Comma", nullptr);

  // clang::BuiltinTemplateKind
  data_model.clang_BuiltinTemplateKind_ = arboretum_node_new_with_id("enum", "clang::BuiltinTemplateKind", nullptr);
  data_model.clang_BTK__make_integer_seq_ = arboretum_node_new_with_id("enum_value", "clang::BTK__make_integer_seq", nullptr);
  data_model.clang_BTK__type_pack_element_ = arboretum_node_new_with_id("enum_value", "clang::BTK__type_pack_element", nullptr);

  // clang::BuiltinType::Kind
  data_model.clang_BuiltinType_Kind_ = arboretum_node_new_with_id("enum", "clang::BuiltinType::Kind", nullptr);
  data_model.clang_BuiltinType_OCLImage1dRO_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage1dRO", nullptr);
  data_model.clang_BuiltinType_OCLImage1dArrayRO_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage1dArrayRO", nullptr);
  data_model.clang_BuiltinType_OCLImage1dBufferRO_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage1dBufferRO", nullptr);
  data_model.clang_BuiltinType_OCLImage2dRO_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage2dRO", nullptr);
  data_model.clang_BuiltinType_OCLImage2dArrayRO_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage2dArrayRO", nullptr);
  data_model.clang_BuiltinType_OCLImage2dDepthRO_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage2dDepthRO", nullptr);
  data_model.clang_BuiltinType_OCLImage2dArrayDepthRO_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage2dArrayDepthRO", nullptr);
  data_model.clang_BuiltinType_OCLImage2dMSAARO_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage2dMSAARO", nullptr);
  data_model.clang_BuiltinType_OCLImage2dArrayMSAARO_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage2dArrayMSAARO", nullptr);
  data_model.clang_BuiltinType_OCLImage2dMSAADepthRO_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage2dMSAADepthRO", nullptr);
  data_model.clang_BuiltinType_OCLImage2dArrayMSAADepthRO_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage2dArrayMSAADepthRO", nullptr);
  data_model.clang_BuiltinType_OCLImage3dRO_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage3dRO", nullptr);
  data_model.clang_BuiltinType_OCLImage1dWO_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage1dWO", nullptr);
  data_model.clang_BuiltinType_OCLImage1dArrayWO_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage1dArrayWO", nullptr);
  data_model.clang_BuiltinType_OCLImage1dBufferWO_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage1dBufferWO", nullptr);
  data_model.clang_BuiltinType_OCLImage2dWO_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage2dWO", nullptr);
  data_model.clang_BuiltinType_OCLImage2dArrayWO_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage2dArrayWO", nullptr);
  data_model.clang_BuiltinType_OCLImage2dDepthWO_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage2dDepthWO", nullptr);
  data_model.clang_BuiltinType_OCLImage2dArrayDepthWO_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage2dArrayDepthWO", nullptr);
  data_model.clang_BuiltinType_OCLImage2dMSAAWO_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage2dMSAAWO", nullptr);
  data_model.clang_BuiltinType_OCLImage2dArrayMSAAWO_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage2dArrayMSAAWO", nullptr);
  data_model.clang_BuiltinType_OCLImage2dMSAADepthWO_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage2dMSAADepthWO", nullptr);
  data_model.clang_BuiltinType_OCLImage2dArrayMSAADepthWO_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage2dArrayMSAADepthWO", nullptr);
  data_model.clang_BuiltinType_OCLImage3dWO_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage3dWO", nullptr);
  data_model.clang_BuiltinType_OCLImage1dRW_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage1dRW", nullptr);
  data_model.clang_BuiltinType_OCLImage1dArrayRW_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage1dArrayRW", nullptr);
  data_model.clang_BuiltinType_OCLImage1dBufferRW_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage1dBufferRW", nullptr);
  data_model.clang_BuiltinType_OCLImage2dRW_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage2dRW", nullptr);
  data_model.clang_BuiltinType_OCLImage2dArrayRW_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage2dArrayRW", nullptr);
  data_model.clang_BuiltinType_OCLImage2dDepthRW_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage2dDepthRW", nullptr);
  data_model.clang_BuiltinType_OCLImage2dArrayDepthRW_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage2dArrayDepthRW", nullptr);
  data_model.clang_BuiltinType_OCLImage2dMSAARW_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage2dMSAARW", nullptr);
  data_model.clang_BuiltinType_OCLImage2dArrayMSAARW_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage2dArrayMSAARW", nullptr);
  data_model.clang_BuiltinType_OCLImage2dMSAADepthRW_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage2dMSAADepthRW", nullptr);
  data_model.clang_BuiltinType_OCLImage2dArrayMSAADepthRW_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage2dArrayMSAADepthRW", nullptr);
  data_model.clang_BuiltinType_OCLImage3dRW_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLImage3dRW", nullptr);
  data_model.clang_BuiltinType_OCLIntelSubgroupAVCMcePayload_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLIntelSubgroupAVCMcePayload", nullptr);
  data_model.clang_BuiltinType_OCLIntelSubgroupAVCImePayload_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLIntelSubgroupAVCImePayload", nullptr);
  data_model.clang_BuiltinType_OCLIntelSubgroupAVCRefPayload_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLIntelSubgroupAVCRefPayload", nullptr);
  data_model.clang_BuiltinType_OCLIntelSubgroupAVCSicPayload_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLIntelSubgroupAVCSicPayload", nullptr);
  data_model.clang_BuiltinType_OCLIntelSubgroupAVCMceResult_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLIntelSubgroupAVCMceResult", nullptr);
  data_model.clang_BuiltinType_OCLIntelSubgroupAVCImeResult_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLIntelSubgroupAVCImeResult", nullptr);
  data_model.clang_BuiltinType_OCLIntelSubgroupAVCRefResult_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLIntelSubgroupAVCRefResult", nullptr);
  data_model.clang_BuiltinType_OCLIntelSubgroupAVCSicResult_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLIntelSubgroupAVCSicResult", nullptr);
  data_model.clang_BuiltinType_OCLIntelSubgroupAVCImeResultSingleReferenceStreamout_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLIntelSubgroupAVCImeResultSingleReferenceStreamout", nullptr);
  data_model.clang_BuiltinType_OCLIntelSubgroupAVCImeResultDualReferenceStreamout_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLIntelSubgroupAVCImeResultDualReferenceStreamout", nullptr);
  data_model.clang_BuiltinType_OCLIntelSubgroupAVCImeSingleReferenceStreamin_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLIntelSubgroupAVCImeSingleReferenceStreamin", nullptr);
  data_model.clang_BuiltinType_OCLIntelSubgroupAVCImeDualReferenceStreamin_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLIntelSubgroupAVCImeDualReferenceStreamin", nullptr);
  data_model.clang_BuiltinType_SveInt8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveInt8", nullptr);
  data_model.clang_BuiltinType_SveInt16_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveInt16", nullptr);
  data_model.clang_BuiltinType_SveInt32_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveInt32", nullptr);
  data_model.clang_BuiltinType_SveInt64_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveInt64", nullptr);
  data_model.clang_BuiltinType_SveUint8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveUint8", nullptr);
  data_model.clang_BuiltinType_SveUint16_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveUint16", nullptr);
  data_model.clang_BuiltinType_SveUint32_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveUint32", nullptr);
  data_model.clang_BuiltinType_SveUint64_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveUint64", nullptr);
  data_model.clang_BuiltinType_SveFloat16_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveFloat16", nullptr);
  data_model.clang_BuiltinType_SveFloat32_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveFloat32", nullptr);
  data_model.clang_BuiltinType_SveFloat64_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveFloat64", nullptr);
  data_model.clang_BuiltinType_SveBFloat16_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveBFloat16", nullptr);
  data_model.clang_BuiltinType_SveInt8x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveInt8x2", nullptr);
  data_model.clang_BuiltinType_SveInt16x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveInt16x2", nullptr);
  data_model.clang_BuiltinType_SveInt32x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveInt32x2", nullptr);
  data_model.clang_BuiltinType_SveInt64x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveInt64x2", nullptr);
  data_model.clang_BuiltinType_SveUint8x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveUint8x2", nullptr);
  data_model.clang_BuiltinType_SveUint16x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveUint16x2", nullptr);
  data_model.clang_BuiltinType_SveUint32x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveUint32x2", nullptr);
  data_model.clang_BuiltinType_SveUint64x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveUint64x2", nullptr);
  data_model.clang_BuiltinType_SveFloat16x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveFloat16x2", nullptr);
  data_model.clang_BuiltinType_SveFloat32x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveFloat32x2", nullptr);
  data_model.clang_BuiltinType_SveFloat64x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveFloat64x2", nullptr);
  data_model.clang_BuiltinType_SveBFloat16x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveBFloat16x2", nullptr);
  data_model.clang_BuiltinType_SveInt8x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveInt8x3", nullptr);
  data_model.clang_BuiltinType_SveInt16x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveInt16x3", nullptr);
  data_model.clang_BuiltinType_SveInt32x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveInt32x3", nullptr);
  data_model.clang_BuiltinType_SveInt64x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveInt64x3", nullptr);
  data_model.clang_BuiltinType_SveUint8x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveUint8x3", nullptr);
  data_model.clang_BuiltinType_SveUint16x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveUint16x3", nullptr);
  data_model.clang_BuiltinType_SveUint32x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveUint32x3", nullptr);
  data_model.clang_BuiltinType_SveUint64x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveUint64x3", nullptr);
  data_model.clang_BuiltinType_SveFloat16x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveFloat16x3", nullptr);
  data_model.clang_BuiltinType_SveFloat32x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveFloat32x3", nullptr);
  data_model.clang_BuiltinType_SveFloat64x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveFloat64x3", nullptr);
  data_model.clang_BuiltinType_SveBFloat16x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveBFloat16x3", nullptr);
  data_model.clang_BuiltinType_SveInt8x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveInt8x4", nullptr);
  data_model.clang_BuiltinType_SveInt16x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveInt16x4", nullptr);
  data_model.clang_BuiltinType_SveInt32x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveInt32x4", nullptr);
  data_model.clang_BuiltinType_SveInt64x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveInt64x4", nullptr);
  data_model.clang_BuiltinType_SveUint8x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveUint8x4", nullptr);
  data_model.clang_BuiltinType_SveUint16x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveUint16x4", nullptr);
  data_model.clang_BuiltinType_SveUint32x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveUint32x4", nullptr);
  data_model.clang_BuiltinType_SveUint64x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveUint64x4", nullptr);
  data_model.clang_BuiltinType_SveFloat16x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveFloat16x4", nullptr);
  data_model.clang_BuiltinType_SveFloat32x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveFloat32x4", nullptr);
  data_model.clang_BuiltinType_SveFloat64x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveFloat64x4", nullptr);
  data_model.clang_BuiltinType_SveBFloat16x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveBFloat16x4", nullptr);
  data_model.clang_BuiltinType_SveBool_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveBool", nullptr);
  data_model.clang_BuiltinType_SveBoolx2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveBoolx2", nullptr);
  data_model.clang_BuiltinType_SveBoolx4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveBoolx4", nullptr);
  data_model.clang_BuiltinType_SveCount_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SveCount", nullptr);
  data_model.clang_BuiltinType_VectorQuad_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::VectorQuad", nullptr);
  data_model.clang_BuiltinType_VectorPair_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::VectorPair", nullptr);
  data_model.clang_BuiltinType_RvvInt8mf8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8mf8", nullptr);
  data_model.clang_BuiltinType_RvvInt8mf4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8mf4", nullptr);
  data_model.clang_BuiltinType_RvvInt8mf2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8mf2", nullptr);
  data_model.clang_BuiltinType_RvvInt8m1_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8m1", nullptr);
  data_model.clang_BuiltinType_RvvInt8m2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8m2", nullptr);
  data_model.clang_BuiltinType_RvvInt8m4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8m4", nullptr);
  data_model.clang_BuiltinType_RvvInt8m8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8m8", nullptr);
  data_model.clang_BuiltinType_RvvUint8mf8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8mf8", nullptr);
  data_model.clang_BuiltinType_RvvUint8mf4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8mf4", nullptr);
  data_model.clang_BuiltinType_RvvUint8mf2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8mf2", nullptr);
  data_model.clang_BuiltinType_RvvUint8m1_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8m1", nullptr);
  data_model.clang_BuiltinType_RvvUint8m2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8m2", nullptr);
  data_model.clang_BuiltinType_RvvUint8m4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8m4", nullptr);
  data_model.clang_BuiltinType_RvvUint8m8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8m8", nullptr);
  data_model.clang_BuiltinType_RvvInt16mf4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16mf4", nullptr);
  data_model.clang_BuiltinType_RvvInt16mf2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16mf2", nullptr);
  data_model.clang_BuiltinType_RvvInt16m1_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16m1", nullptr);
  data_model.clang_BuiltinType_RvvInt16m2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16m2", nullptr);
  data_model.clang_BuiltinType_RvvInt16m4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16m4", nullptr);
  data_model.clang_BuiltinType_RvvInt16m8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16m8", nullptr);
  data_model.clang_BuiltinType_RvvUint16mf4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16mf4", nullptr);
  data_model.clang_BuiltinType_RvvUint16mf2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16mf2", nullptr);
  data_model.clang_BuiltinType_RvvUint16m1_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16m1", nullptr);
  data_model.clang_BuiltinType_RvvUint16m2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16m2", nullptr);
  data_model.clang_BuiltinType_RvvUint16m4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16m4", nullptr);
  data_model.clang_BuiltinType_RvvUint16m8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16m8", nullptr);
  data_model.clang_BuiltinType_RvvInt32mf2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt32mf2", nullptr);
  data_model.clang_BuiltinType_RvvInt32m1_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt32m1", nullptr);
  data_model.clang_BuiltinType_RvvInt32m2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt32m2", nullptr);
  data_model.clang_BuiltinType_RvvInt32m4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt32m4", nullptr);
  data_model.clang_BuiltinType_RvvInt32m8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt32m8", nullptr);
  data_model.clang_BuiltinType_RvvUint32mf2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint32mf2", nullptr);
  data_model.clang_BuiltinType_RvvUint32m1_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint32m1", nullptr);
  data_model.clang_BuiltinType_RvvUint32m2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint32m2", nullptr);
  data_model.clang_BuiltinType_RvvUint32m4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint32m4", nullptr);
  data_model.clang_BuiltinType_RvvUint32m8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint32m8", nullptr);
  data_model.clang_BuiltinType_RvvInt64m1_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt64m1", nullptr);
  data_model.clang_BuiltinType_RvvInt64m2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt64m2", nullptr);
  data_model.clang_BuiltinType_RvvInt64m4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt64m4", nullptr);
  data_model.clang_BuiltinType_RvvInt64m8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt64m8", nullptr);
  data_model.clang_BuiltinType_RvvUint64m1_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint64m1", nullptr);
  data_model.clang_BuiltinType_RvvUint64m2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint64m2", nullptr);
  data_model.clang_BuiltinType_RvvUint64m4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint64m4", nullptr);
  data_model.clang_BuiltinType_RvvUint64m8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint64m8", nullptr);
  data_model.clang_BuiltinType_RvvFloat16mf4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16mf4", nullptr);
  data_model.clang_BuiltinType_RvvFloat16mf2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16mf2", nullptr);
  data_model.clang_BuiltinType_RvvFloat16m1_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16m1", nullptr);
  data_model.clang_BuiltinType_RvvFloat16m2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16m2", nullptr);
  data_model.clang_BuiltinType_RvvFloat16m4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16m4", nullptr);
  data_model.clang_BuiltinType_RvvFloat16m8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16m8", nullptr);
  data_model.clang_BuiltinType_RvvFloat32mf2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat32mf2", nullptr);
  data_model.clang_BuiltinType_RvvFloat32m1_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat32m1", nullptr);
  data_model.clang_BuiltinType_RvvFloat32m2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat32m2", nullptr);
  data_model.clang_BuiltinType_RvvFloat32m4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat32m4", nullptr);
  data_model.clang_BuiltinType_RvvFloat32m8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat32m8", nullptr);
  data_model.clang_BuiltinType_RvvFloat64m1_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat64m1", nullptr);
  data_model.clang_BuiltinType_RvvFloat64m2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat64m2", nullptr);
  data_model.clang_BuiltinType_RvvFloat64m4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat64m4", nullptr);
  data_model.clang_BuiltinType_RvvFloat64m8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat64m8", nullptr);
  data_model.clang_BuiltinType_RvvBool1_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvBool1", nullptr);
  data_model.clang_BuiltinType_RvvBool2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvBool2", nullptr);
  data_model.clang_BuiltinType_RvvBool4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvBool4", nullptr);
  data_model.clang_BuiltinType_RvvBool8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvBool8", nullptr);
  data_model.clang_BuiltinType_RvvBool16_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvBool16", nullptr);
  data_model.clang_BuiltinType_RvvBool32_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvBool32", nullptr);
  data_model.clang_BuiltinType_RvvBool64_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvBool64", nullptr);
  data_model.clang_BuiltinType_RvvInt8mf8x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8mf8x2", nullptr);
  data_model.clang_BuiltinType_RvvInt8mf8x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8mf8x3", nullptr);
  data_model.clang_BuiltinType_RvvInt8mf8x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8mf8x4", nullptr);
  data_model.clang_BuiltinType_RvvInt8mf8x5_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8mf8x5", nullptr);
  data_model.clang_BuiltinType_RvvInt8mf8x6_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8mf8x6", nullptr);
  data_model.clang_BuiltinType_RvvInt8mf8x7_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8mf8x7", nullptr);
  data_model.clang_BuiltinType_RvvInt8mf8x8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8mf8x8", nullptr);
  data_model.clang_BuiltinType_RvvInt8mf4x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8mf4x2", nullptr);
  data_model.clang_BuiltinType_RvvInt8mf4x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8mf4x3", nullptr);
  data_model.clang_BuiltinType_RvvInt8mf4x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8mf4x4", nullptr);
  data_model.clang_BuiltinType_RvvInt8mf4x5_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8mf4x5", nullptr);
  data_model.clang_BuiltinType_RvvInt8mf4x6_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8mf4x6", nullptr);
  data_model.clang_BuiltinType_RvvInt8mf4x7_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8mf4x7", nullptr);
  data_model.clang_BuiltinType_RvvInt8mf4x8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8mf4x8", nullptr);
  data_model.clang_BuiltinType_RvvInt8mf2x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8mf2x2", nullptr);
  data_model.clang_BuiltinType_RvvInt8mf2x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8mf2x3", nullptr);
  data_model.clang_BuiltinType_RvvInt8mf2x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8mf2x4", nullptr);
  data_model.clang_BuiltinType_RvvInt8mf2x5_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8mf2x5", nullptr);
  data_model.clang_BuiltinType_RvvInt8mf2x6_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8mf2x6", nullptr);
  data_model.clang_BuiltinType_RvvInt8mf2x7_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8mf2x7", nullptr);
  data_model.clang_BuiltinType_RvvInt8mf2x8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8mf2x8", nullptr);
  data_model.clang_BuiltinType_RvvInt8m1x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8m1x2", nullptr);
  data_model.clang_BuiltinType_RvvInt8m1x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8m1x3", nullptr);
  data_model.clang_BuiltinType_RvvInt8m1x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8m1x4", nullptr);
  data_model.clang_BuiltinType_RvvInt8m1x5_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8m1x5", nullptr);
  data_model.clang_BuiltinType_RvvInt8m1x6_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8m1x6", nullptr);
  data_model.clang_BuiltinType_RvvInt8m1x7_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8m1x7", nullptr);
  data_model.clang_BuiltinType_RvvInt8m1x8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8m1x8", nullptr);
  data_model.clang_BuiltinType_RvvInt8m2x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8m2x2", nullptr);
  data_model.clang_BuiltinType_RvvInt8m2x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8m2x3", nullptr);
  data_model.clang_BuiltinType_RvvInt8m2x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8m2x4", nullptr);
  data_model.clang_BuiltinType_RvvInt8m4x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt8m4x2", nullptr);
  data_model.clang_BuiltinType_RvvUint8mf8x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8mf8x2", nullptr);
  data_model.clang_BuiltinType_RvvUint8mf8x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8mf8x3", nullptr);
  data_model.clang_BuiltinType_RvvUint8mf8x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8mf8x4", nullptr);
  data_model.clang_BuiltinType_RvvUint8mf8x5_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8mf8x5", nullptr);
  data_model.clang_BuiltinType_RvvUint8mf8x6_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8mf8x6", nullptr);
  data_model.clang_BuiltinType_RvvUint8mf8x7_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8mf8x7", nullptr);
  data_model.clang_BuiltinType_RvvUint8mf8x8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8mf8x8", nullptr);
  data_model.clang_BuiltinType_RvvUint8mf4x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8mf4x2", nullptr);
  data_model.clang_BuiltinType_RvvUint8mf4x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8mf4x3", nullptr);
  data_model.clang_BuiltinType_RvvUint8mf4x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8mf4x4", nullptr);
  data_model.clang_BuiltinType_RvvUint8mf4x5_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8mf4x5", nullptr);
  data_model.clang_BuiltinType_RvvUint8mf4x6_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8mf4x6", nullptr);
  data_model.clang_BuiltinType_RvvUint8mf4x7_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8mf4x7", nullptr);
  data_model.clang_BuiltinType_RvvUint8mf4x8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8mf4x8", nullptr);
  data_model.clang_BuiltinType_RvvUint8mf2x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8mf2x2", nullptr);
  data_model.clang_BuiltinType_RvvUint8mf2x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8mf2x3", nullptr);
  data_model.clang_BuiltinType_RvvUint8mf2x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8mf2x4", nullptr);
  data_model.clang_BuiltinType_RvvUint8mf2x5_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8mf2x5", nullptr);
  data_model.clang_BuiltinType_RvvUint8mf2x6_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8mf2x6", nullptr);
  data_model.clang_BuiltinType_RvvUint8mf2x7_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8mf2x7", nullptr);
  data_model.clang_BuiltinType_RvvUint8mf2x8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8mf2x8", nullptr);
  data_model.clang_BuiltinType_RvvUint8m1x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8m1x2", nullptr);
  data_model.clang_BuiltinType_RvvUint8m1x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8m1x3", nullptr);
  data_model.clang_BuiltinType_RvvUint8m1x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8m1x4", nullptr);
  data_model.clang_BuiltinType_RvvUint8m1x5_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8m1x5", nullptr);
  data_model.clang_BuiltinType_RvvUint8m1x6_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8m1x6", nullptr);
  data_model.clang_BuiltinType_RvvUint8m1x7_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8m1x7", nullptr);
  data_model.clang_BuiltinType_RvvUint8m1x8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8m1x8", nullptr);
  data_model.clang_BuiltinType_RvvUint8m2x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8m2x2", nullptr);
  data_model.clang_BuiltinType_RvvUint8m2x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8m2x3", nullptr);
  data_model.clang_BuiltinType_RvvUint8m2x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8m2x4", nullptr);
  data_model.clang_BuiltinType_RvvUint8m4x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint8m4x2", nullptr);
  data_model.clang_BuiltinType_RvvInt16mf4x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16mf4x2", nullptr);
  data_model.clang_BuiltinType_RvvInt16mf4x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16mf4x3", nullptr);
  data_model.clang_BuiltinType_RvvInt16mf4x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16mf4x4", nullptr);
  data_model.clang_BuiltinType_RvvInt16mf4x5_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16mf4x5", nullptr);
  data_model.clang_BuiltinType_RvvInt16mf4x6_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16mf4x6", nullptr);
  data_model.clang_BuiltinType_RvvInt16mf4x7_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16mf4x7", nullptr);
  data_model.clang_BuiltinType_RvvInt16mf4x8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16mf4x8", nullptr);
  data_model.clang_BuiltinType_RvvInt16mf2x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16mf2x2", nullptr);
  data_model.clang_BuiltinType_RvvInt16mf2x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16mf2x3", nullptr);
  data_model.clang_BuiltinType_RvvInt16mf2x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16mf2x4", nullptr);
  data_model.clang_BuiltinType_RvvInt16mf2x5_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16mf2x5", nullptr);
  data_model.clang_BuiltinType_RvvInt16mf2x6_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16mf2x6", nullptr);
  data_model.clang_BuiltinType_RvvInt16mf2x7_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16mf2x7", nullptr);
  data_model.clang_BuiltinType_RvvInt16mf2x8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16mf2x8", nullptr);
  data_model.clang_BuiltinType_RvvInt16m1x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16m1x2", nullptr);
  data_model.clang_BuiltinType_RvvInt16m1x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16m1x3", nullptr);
  data_model.clang_BuiltinType_RvvInt16m1x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16m1x4", nullptr);
  data_model.clang_BuiltinType_RvvInt16m1x5_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16m1x5", nullptr);
  data_model.clang_BuiltinType_RvvInt16m1x6_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16m1x6", nullptr);
  data_model.clang_BuiltinType_RvvInt16m1x7_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16m1x7", nullptr);
  data_model.clang_BuiltinType_RvvInt16m1x8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16m1x8", nullptr);
  data_model.clang_BuiltinType_RvvInt16m2x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16m2x2", nullptr);
  data_model.clang_BuiltinType_RvvInt16m2x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16m2x3", nullptr);
  data_model.clang_BuiltinType_RvvInt16m2x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16m2x4", nullptr);
  data_model.clang_BuiltinType_RvvInt16m4x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt16m4x2", nullptr);
  data_model.clang_BuiltinType_RvvUint16mf4x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16mf4x2", nullptr);
  data_model.clang_BuiltinType_RvvUint16mf4x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16mf4x3", nullptr);
  data_model.clang_BuiltinType_RvvUint16mf4x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16mf4x4", nullptr);
  data_model.clang_BuiltinType_RvvUint16mf4x5_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16mf4x5", nullptr);
  data_model.clang_BuiltinType_RvvUint16mf4x6_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16mf4x6", nullptr);
  data_model.clang_BuiltinType_RvvUint16mf4x7_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16mf4x7", nullptr);
  data_model.clang_BuiltinType_RvvUint16mf4x8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16mf4x8", nullptr);
  data_model.clang_BuiltinType_RvvUint16mf2x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16mf2x2", nullptr);
  data_model.clang_BuiltinType_RvvUint16mf2x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16mf2x3", nullptr);
  data_model.clang_BuiltinType_RvvUint16mf2x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16mf2x4", nullptr);
  data_model.clang_BuiltinType_RvvUint16mf2x5_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16mf2x5", nullptr);
  data_model.clang_BuiltinType_RvvUint16mf2x6_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16mf2x6", nullptr);
  data_model.clang_BuiltinType_RvvUint16mf2x7_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16mf2x7", nullptr);
  data_model.clang_BuiltinType_RvvUint16mf2x8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16mf2x8", nullptr);
  data_model.clang_BuiltinType_RvvUint16m1x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16m1x2", nullptr);
  data_model.clang_BuiltinType_RvvUint16m1x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16m1x3", nullptr);
  data_model.clang_BuiltinType_RvvUint16m1x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16m1x4", nullptr);
  data_model.clang_BuiltinType_RvvUint16m1x5_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16m1x5", nullptr);
  data_model.clang_BuiltinType_RvvUint16m1x6_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16m1x6", nullptr);
  data_model.clang_BuiltinType_RvvUint16m1x7_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16m1x7", nullptr);
  data_model.clang_BuiltinType_RvvUint16m1x8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16m1x8", nullptr);
  data_model.clang_BuiltinType_RvvUint16m2x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16m2x2", nullptr);
  data_model.clang_BuiltinType_RvvUint16m2x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16m2x3", nullptr);
  data_model.clang_BuiltinType_RvvUint16m2x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16m2x4", nullptr);
  data_model.clang_BuiltinType_RvvUint16m4x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint16m4x2", nullptr);
  data_model.clang_BuiltinType_RvvInt32mf2x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt32mf2x2", nullptr);
  data_model.clang_BuiltinType_RvvInt32mf2x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt32mf2x3", nullptr);
  data_model.clang_BuiltinType_RvvInt32mf2x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt32mf2x4", nullptr);
  data_model.clang_BuiltinType_RvvInt32mf2x5_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt32mf2x5", nullptr);
  data_model.clang_BuiltinType_RvvInt32mf2x6_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt32mf2x6", nullptr);
  data_model.clang_BuiltinType_RvvInt32mf2x7_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt32mf2x7", nullptr);
  data_model.clang_BuiltinType_RvvInt32mf2x8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt32mf2x8", nullptr);
  data_model.clang_BuiltinType_RvvInt32m1x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt32m1x2", nullptr);
  data_model.clang_BuiltinType_RvvInt32m1x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt32m1x3", nullptr);
  data_model.clang_BuiltinType_RvvInt32m1x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt32m1x4", nullptr);
  data_model.clang_BuiltinType_RvvInt32m1x5_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt32m1x5", nullptr);
  data_model.clang_BuiltinType_RvvInt32m1x6_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt32m1x6", nullptr);
  data_model.clang_BuiltinType_RvvInt32m1x7_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt32m1x7", nullptr);
  data_model.clang_BuiltinType_RvvInt32m1x8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt32m1x8", nullptr);
  data_model.clang_BuiltinType_RvvInt32m2x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt32m2x2", nullptr);
  data_model.clang_BuiltinType_RvvInt32m2x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt32m2x3", nullptr);
  data_model.clang_BuiltinType_RvvInt32m2x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt32m2x4", nullptr);
  data_model.clang_BuiltinType_RvvInt32m4x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt32m4x2", nullptr);
  data_model.clang_BuiltinType_RvvUint32mf2x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint32mf2x2", nullptr);
  data_model.clang_BuiltinType_RvvUint32mf2x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint32mf2x3", nullptr);
  data_model.clang_BuiltinType_RvvUint32mf2x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint32mf2x4", nullptr);
  data_model.clang_BuiltinType_RvvUint32mf2x5_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint32mf2x5", nullptr);
  data_model.clang_BuiltinType_RvvUint32mf2x6_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint32mf2x6", nullptr);
  data_model.clang_BuiltinType_RvvUint32mf2x7_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint32mf2x7", nullptr);
  data_model.clang_BuiltinType_RvvUint32mf2x8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint32mf2x8", nullptr);
  data_model.clang_BuiltinType_RvvUint32m1x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint32m1x2", nullptr);
  data_model.clang_BuiltinType_RvvUint32m1x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint32m1x3", nullptr);
  data_model.clang_BuiltinType_RvvUint32m1x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint32m1x4", nullptr);
  data_model.clang_BuiltinType_RvvUint32m1x5_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint32m1x5", nullptr);
  data_model.clang_BuiltinType_RvvUint32m1x6_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint32m1x6", nullptr);
  data_model.clang_BuiltinType_RvvUint32m1x7_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint32m1x7", nullptr);
  data_model.clang_BuiltinType_RvvUint32m1x8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint32m1x8", nullptr);
  data_model.clang_BuiltinType_RvvUint32m2x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint32m2x2", nullptr);
  data_model.clang_BuiltinType_RvvUint32m2x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint32m2x3", nullptr);
  data_model.clang_BuiltinType_RvvUint32m2x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint32m2x4", nullptr);
  data_model.clang_BuiltinType_RvvUint32m4x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint32m4x2", nullptr);
  data_model.clang_BuiltinType_RvvInt64m1x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt64m1x2", nullptr);
  data_model.clang_BuiltinType_RvvInt64m1x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt64m1x3", nullptr);
  data_model.clang_BuiltinType_RvvInt64m1x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt64m1x4", nullptr);
  data_model.clang_BuiltinType_RvvInt64m1x5_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt64m1x5", nullptr);
  data_model.clang_BuiltinType_RvvInt64m1x6_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt64m1x6", nullptr);
  data_model.clang_BuiltinType_RvvInt64m1x7_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt64m1x7", nullptr);
  data_model.clang_BuiltinType_RvvInt64m1x8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt64m1x8", nullptr);
  data_model.clang_BuiltinType_RvvInt64m2x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt64m2x2", nullptr);
  data_model.clang_BuiltinType_RvvInt64m2x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt64m2x3", nullptr);
  data_model.clang_BuiltinType_RvvInt64m2x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt64m2x4", nullptr);
  data_model.clang_BuiltinType_RvvInt64m4x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvInt64m4x2", nullptr);
  data_model.clang_BuiltinType_RvvUint64m1x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint64m1x2", nullptr);
  data_model.clang_BuiltinType_RvvUint64m1x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint64m1x3", nullptr);
  data_model.clang_BuiltinType_RvvUint64m1x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint64m1x4", nullptr);
  data_model.clang_BuiltinType_RvvUint64m1x5_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint64m1x5", nullptr);
  data_model.clang_BuiltinType_RvvUint64m1x6_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint64m1x6", nullptr);
  data_model.clang_BuiltinType_RvvUint64m1x7_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint64m1x7", nullptr);
  data_model.clang_BuiltinType_RvvUint64m1x8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint64m1x8", nullptr);
  data_model.clang_BuiltinType_RvvUint64m2x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint64m2x2", nullptr);
  data_model.clang_BuiltinType_RvvUint64m2x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint64m2x3", nullptr);
  data_model.clang_BuiltinType_RvvUint64m2x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint64m2x4", nullptr);
  data_model.clang_BuiltinType_RvvUint64m4x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvUint64m4x2", nullptr);
  data_model.clang_BuiltinType_RvvFloat16mf4x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16mf4x2", nullptr);
  data_model.clang_BuiltinType_RvvFloat16mf4x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16mf4x3", nullptr);
  data_model.clang_BuiltinType_RvvFloat16mf4x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16mf4x4", nullptr);
  data_model.clang_BuiltinType_RvvFloat16mf4x5_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16mf4x5", nullptr);
  data_model.clang_BuiltinType_RvvFloat16mf4x6_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16mf4x6", nullptr);
  data_model.clang_BuiltinType_RvvFloat16mf4x7_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16mf4x7", nullptr);
  data_model.clang_BuiltinType_RvvFloat16mf4x8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16mf4x8", nullptr);
  data_model.clang_BuiltinType_RvvFloat16mf2x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16mf2x2", nullptr);
  data_model.clang_BuiltinType_RvvFloat16mf2x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16mf2x3", nullptr);
  data_model.clang_BuiltinType_RvvFloat16mf2x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16mf2x4", nullptr);
  data_model.clang_BuiltinType_RvvFloat16mf2x5_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16mf2x5", nullptr);
  data_model.clang_BuiltinType_RvvFloat16mf2x6_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16mf2x6", nullptr);
  data_model.clang_BuiltinType_RvvFloat16mf2x7_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16mf2x7", nullptr);
  data_model.clang_BuiltinType_RvvFloat16mf2x8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16mf2x8", nullptr);
  data_model.clang_BuiltinType_RvvFloat16m1x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16m1x2", nullptr);
  data_model.clang_BuiltinType_RvvFloat16m1x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16m1x3", nullptr);
  data_model.clang_BuiltinType_RvvFloat16m1x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16m1x4", nullptr);
  data_model.clang_BuiltinType_RvvFloat16m1x5_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16m1x5", nullptr);
  data_model.clang_BuiltinType_RvvFloat16m1x6_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16m1x6", nullptr);
  data_model.clang_BuiltinType_RvvFloat16m1x7_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16m1x7", nullptr);
  data_model.clang_BuiltinType_RvvFloat16m1x8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16m1x8", nullptr);
  data_model.clang_BuiltinType_RvvFloat16m2x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16m2x2", nullptr);
  data_model.clang_BuiltinType_RvvFloat16m2x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16m2x3", nullptr);
  data_model.clang_BuiltinType_RvvFloat16m2x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16m2x4", nullptr);
  data_model.clang_BuiltinType_RvvFloat16m4x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat16m4x2", nullptr);
  data_model.clang_BuiltinType_RvvFloat32mf2x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat32mf2x2", nullptr);
  data_model.clang_BuiltinType_RvvFloat32mf2x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat32mf2x3", nullptr);
  data_model.clang_BuiltinType_RvvFloat32mf2x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat32mf2x4", nullptr);
  data_model.clang_BuiltinType_RvvFloat32mf2x5_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat32mf2x5", nullptr);
  data_model.clang_BuiltinType_RvvFloat32mf2x6_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat32mf2x6", nullptr);
  data_model.clang_BuiltinType_RvvFloat32mf2x7_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat32mf2x7", nullptr);
  data_model.clang_BuiltinType_RvvFloat32mf2x8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat32mf2x8", nullptr);
  data_model.clang_BuiltinType_RvvFloat32m1x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat32m1x2", nullptr);
  data_model.clang_BuiltinType_RvvFloat32m1x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat32m1x3", nullptr);
  data_model.clang_BuiltinType_RvvFloat32m1x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat32m1x4", nullptr);
  data_model.clang_BuiltinType_RvvFloat32m1x5_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat32m1x5", nullptr);
  data_model.clang_BuiltinType_RvvFloat32m1x6_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat32m1x6", nullptr);
  data_model.clang_BuiltinType_RvvFloat32m1x7_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat32m1x7", nullptr);
  data_model.clang_BuiltinType_RvvFloat32m1x8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat32m1x8", nullptr);
  data_model.clang_BuiltinType_RvvFloat32m2x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat32m2x2", nullptr);
  data_model.clang_BuiltinType_RvvFloat32m2x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat32m2x3", nullptr);
  data_model.clang_BuiltinType_RvvFloat32m2x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat32m2x4", nullptr);
  data_model.clang_BuiltinType_RvvFloat32m4x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat32m4x2", nullptr);
  data_model.clang_BuiltinType_RvvFloat64m1x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat64m1x2", nullptr);
  data_model.clang_BuiltinType_RvvFloat64m1x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat64m1x3", nullptr);
  data_model.clang_BuiltinType_RvvFloat64m1x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat64m1x4", nullptr);
  data_model.clang_BuiltinType_RvvFloat64m1x5_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat64m1x5", nullptr);
  data_model.clang_BuiltinType_RvvFloat64m1x6_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat64m1x6", nullptr);
  data_model.clang_BuiltinType_RvvFloat64m1x7_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat64m1x7", nullptr);
  data_model.clang_BuiltinType_RvvFloat64m1x8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat64m1x8", nullptr);
  data_model.clang_BuiltinType_RvvFloat64m2x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat64m2x2", nullptr);
  data_model.clang_BuiltinType_RvvFloat64m2x3_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat64m2x3", nullptr);
  data_model.clang_BuiltinType_RvvFloat64m2x4_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat64m2x4", nullptr);
  data_model.clang_BuiltinType_RvvFloat64m4x2_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::RvvFloat64m4x2", nullptr);
  data_model.clang_BuiltinType_WasmExternRef_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::WasmExternRef", nullptr);
  data_model.clang_BuiltinType_Void_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::Void", nullptr);
  data_model.clang_BuiltinType_Bool_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::Bool", nullptr);
  data_model.clang_BuiltinType_Char_U_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::Char_U", nullptr);
  data_model.clang_BuiltinType_UChar_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::UChar", nullptr);
  data_model.clang_BuiltinType_WChar_U_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::WChar_U", nullptr);
  data_model.clang_BuiltinType_Char8_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::Char8", nullptr);
  data_model.clang_BuiltinType_Char16_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::Char16", nullptr);
  data_model.clang_BuiltinType_Char32_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::Char32", nullptr);
  data_model.clang_BuiltinType_UShort_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::UShort", nullptr);
  data_model.clang_BuiltinType_UInt_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::UInt", nullptr);
  data_model.clang_BuiltinType_ULong_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::ULong", nullptr);
  data_model.clang_BuiltinType_ULongLong_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::ULongLong", nullptr);
  data_model.clang_BuiltinType_UInt128_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::UInt128", nullptr);
  data_model.clang_BuiltinType_Char_S_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::Char_S", nullptr);
  data_model.clang_BuiltinType_SChar_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SChar", nullptr);
  data_model.clang_BuiltinType_WChar_S_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::WChar_S", nullptr);
  data_model.clang_BuiltinType_Short_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::Short", nullptr);
  data_model.clang_BuiltinType_Int_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::Int", nullptr);
  data_model.clang_BuiltinType_Long_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::Long", nullptr);
  data_model.clang_BuiltinType_LongLong_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::LongLong", nullptr);
  data_model.clang_BuiltinType_Int128_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::Int128", nullptr);
  data_model.clang_BuiltinType_ShortAccum_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::ShortAccum", nullptr);
  data_model.clang_BuiltinType_Accum_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::Accum", nullptr);
  data_model.clang_BuiltinType_LongAccum_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::LongAccum", nullptr);
  data_model.clang_BuiltinType_UShortAccum_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::UShortAccum", nullptr);
  data_model.clang_BuiltinType_UAccum_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::UAccum", nullptr);
  data_model.clang_BuiltinType_ULongAccum_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::ULongAccum", nullptr);
  data_model.clang_BuiltinType_ShortFract_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::ShortFract", nullptr);
  data_model.clang_BuiltinType_Fract_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::Fract", nullptr);
  data_model.clang_BuiltinType_LongFract_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::LongFract", nullptr);
  data_model.clang_BuiltinType_UShortFract_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::UShortFract", nullptr);
  data_model.clang_BuiltinType_UFract_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::UFract", nullptr);
  data_model.clang_BuiltinType_ULongFract_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::ULongFract", nullptr);
  data_model.clang_BuiltinType_SatShortAccum_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SatShortAccum", nullptr);
  data_model.clang_BuiltinType_SatAccum_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SatAccum", nullptr);
  data_model.clang_BuiltinType_SatLongAccum_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SatLongAccum", nullptr);
  data_model.clang_BuiltinType_SatUShortAccum_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SatUShortAccum", nullptr);
  data_model.clang_BuiltinType_SatUAccum_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SatUAccum", nullptr);
  data_model.clang_BuiltinType_SatULongAccum_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SatULongAccum", nullptr);
  data_model.clang_BuiltinType_SatShortFract_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SatShortFract", nullptr);
  data_model.clang_BuiltinType_SatFract_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SatFract", nullptr);
  data_model.clang_BuiltinType_SatLongFract_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SatLongFract", nullptr);
  data_model.clang_BuiltinType_SatUShortFract_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SatUShortFract", nullptr);
  data_model.clang_BuiltinType_SatUFract_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SatUFract", nullptr);
  data_model.clang_BuiltinType_SatULongFract_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::SatULongFract", nullptr);
  data_model.clang_BuiltinType_Half_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::Half", nullptr);
  data_model.clang_BuiltinType_Float_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::Float", nullptr);
  data_model.clang_BuiltinType_Double_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::Double", nullptr);
  data_model.clang_BuiltinType_LongDouble_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::LongDouble", nullptr);
  data_model.clang_BuiltinType_Float16_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::Float16", nullptr);
  data_model.clang_BuiltinType_BFloat16_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::BFloat16", nullptr);
  data_model.clang_BuiltinType_Float128_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::Float128", nullptr);
  data_model.clang_BuiltinType_Ibm128_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::Ibm128", nullptr);
  data_model.clang_BuiltinType_NullPtr_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::NullPtr", nullptr);
  data_model.clang_BuiltinType_ObjCId_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::ObjCId", nullptr);
  data_model.clang_BuiltinType_ObjCClass_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::ObjCClass", nullptr);
  data_model.clang_BuiltinType_ObjCSel_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::ObjCSel", nullptr);
  data_model.clang_BuiltinType_OCLSampler_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLSampler", nullptr);
  data_model.clang_BuiltinType_OCLEvent_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLEvent", nullptr);
  data_model.clang_BuiltinType_OCLClkEvent_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLClkEvent", nullptr);
  data_model.clang_BuiltinType_OCLQueue_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLQueue", nullptr);
  data_model.clang_BuiltinType_OCLReserveID_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OCLReserveID", nullptr);
  data_model.clang_BuiltinType_Dependent_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::Dependent", nullptr);
  data_model.clang_BuiltinType_Overload_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::Overload", nullptr);
  data_model.clang_BuiltinType_BoundMember_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::BoundMember", nullptr);
  data_model.clang_BuiltinType_PseudoObject_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::PseudoObject", nullptr);
  data_model.clang_BuiltinType_UnknownAny_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::UnknownAny", nullptr);
  data_model.clang_BuiltinType_BuiltinFn_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::BuiltinFn", nullptr);
  data_model.clang_BuiltinType_ARCUnbridgedCast_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::ARCUnbridgedCast", nullptr);
  data_model.clang_BuiltinType_IncompleteMatrixIdx_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::IncompleteMatrixIdx", nullptr);
  data_model.clang_BuiltinType_OMPArraySection_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OMPArraySection", nullptr);
  data_model.clang_BuiltinType_OMPArrayShaping_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OMPArrayShaping", nullptr);
  data_model.clang_BuiltinType_OMPIterator_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::OMPIterator", nullptr);
  data_model.clang_BuiltinType_LastKind_ = arboretum_node_new_with_id("enum_value", "clang::BuiltinType::LastKind", nullptr);

  // clang::CXXConstructExpr::ConstructionKind
  data_model.clang_CXXConstructExpr_ConstructionKind_ = arboretum_node_new_with_id("enum", "clang::CXXConstructExpr::ConstructionKind", nullptr);
  data_model.clang_CXXConstructExpr_CK_Complete_ = arboretum_node_new_with_id("enum_value", "clang::CXXConstructExpr::CK_Complete", nullptr);
  data_model.clang_CXXConstructExpr_CK_NonVirtualBase_ = arboretum_node_new_with_id("enum_value", "clang::CXXConstructExpr::CK_NonVirtualBase", nullptr);
  data_model.clang_CXXConstructExpr_CK_VirtualBase_ = arboretum_node_new_with_id("enum_value", "clang::CXXConstructExpr::CK_VirtualBase", nullptr);
  data_model.clang_CXXConstructExpr_CK_Delegating_ = arboretum_node_new_with_id("enum_value", "clang::CXXConstructExpr::CK_Delegating", nullptr);

  // clang::CXXNewExpr::InitializationStyle
  data_model.clang_CXXNewExpr_InitializationStyle_ = arboretum_node_new_with_id("enum", "clang::CXXNewExpr::InitializationStyle", nullptr);
  data_model.clang_CXXNewExpr_NoInit_ = arboretum_node_new_with_id("enum_value", "clang::CXXNewExpr::NoInit", nullptr);
  data_model.clang_CXXNewExpr_CallInit_ = arboretum_node_new_with_id("enum_value", "clang::CXXNewExpr::CallInit", nullptr);
  data_model.clang_CXXNewExpr_ListInit_ = arboretum_node_new_with_id("enum_value", "clang::CXXNewExpr::ListInit", nullptr);

  // clang::CallExpr::ADLCallKind
  data_model.clang_CallExpr_ADLCallKind_ = arboretum_node_new_with_id("enum", "clang::CallExpr::ADLCallKind", nullptr);
  data_model.clang_CallExpr_ADLCallKind_NotADL_ = arboretum_node_new_with_id("enum_value", "clang::CallExpr::ADLCallKind::NotADL", nullptr);
  data_model.clang_CallExpr_ADLCallKind_UsesADL_ = arboretum_node_new_with_id("enum_value", "clang::CallExpr::ADLCallKind::UsesADL", nullptr);

  // clang::CallingConv
  data_model.clang_CallingConv_ = arboretum_node_new_with_id("enum", "clang::CallingConv", nullptr);
  data_model.clang_CC_C_ = arboretum_node_new_with_id("enum_value", "clang::CC_C", nullptr);
  data_model.clang_CC_X86StdCall_ = arboretum_node_new_with_id("enum_value", "clang::CC_X86StdCall", nullptr);
  data_model.clang_CC_X86FastCall_ = arboretum_node_new_with_id("enum_value", "clang::CC_X86FastCall", nullptr);
  data_model.clang_CC_X86ThisCall_ = arboretum_node_new_with_id("enum_value", "clang::CC_X86ThisCall", nullptr);
  data_model.clang_CC_X86VectorCall_ = arboretum_node_new_with_id("enum_value", "clang::CC_X86VectorCall", nullptr);
  data_model.clang_CC_X86Pascal_ = arboretum_node_new_with_id("enum_value", "clang::CC_X86Pascal", nullptr);
  data_model.clang_CC_Win64_ = arboretum_node_new_with_id("enum_value", "clang::CC_Win64", nullptr);
  data_model.clang_CC_X86_64SysV_ = arboretum_node_new_with_id("enum_value", "clang::CC_X86_64SysV", nullptr);
  data_model.clang_CC_X86RegCall_ = arboretum_node_new_with_id("enum_value", "clang::CC_X86RegCall", nullptr);
  data_model.clang_CC_AAPCS_ = arboretum_node_new_with_id("enum_value", "clang::CC_AAPCS", nullptr);
  data_model.clang_CC_AAPCS_VFP_ = arboretum_node_new_with_id("enum_value", "clang::CC_AAPCS_VFP", nullptr);
  data_model.clang_CC_IntelOclBicc_ = arboretum_node_new_with_id("enum_value", "clang::CC_IntelOclBicc", nullptr);
  data_model.clang_CC_SpirFunction_ = arboretum_node_new_with_id("enum_value", "clang::CC_SpirFunction", nullptr);
  data_model.clang_CC_OpenCLKernel_ = arboretum_node_new_with_id("enum_value", "clang::CC_OpenCLKernel", nullptr);
  data_model.clang_CC_Swift_ = arboretum_node_new_with_id("enum_value", "clang::CC_Swift", nullptr);
  data_model.clang_CC_SwiftAsync_ = arboretum_node_new_with_id("enum_value", "clang::CC_SwiftAsync", nullptr);
  data_model.clang_CC_PreserveMost_ = arboretum_node_new_with_id("enum_value", "clang::CC_PreserveMost", nullptr);
  data_model.clang_CC_PreserveAll_ = arboretum_node_new_with_id("enum_value", "clang::CC_PreserveAll", nullptr);
  data_model.clang_CC_AArch64VectorCall_ = arboretum_node_new_with_id("enum_value", "clang::CC_AArch64VectorCall", nullptr);
  data_model.clang_CC_AArch64SVEPCS_ = arboretum_node_new_with_id("enum_value", "clang::CC_AArch64SVEPCS", nullptr);
  data_model.clang_CC_AMDGPUKernelCall_ = arboretum_node_new_with_id("enum_value", "clang::CC_AMDGPUKernelCall", nullptr);

  // clang::CanThrowResult
  data_model.clang_CanThrowResult_ = arboretum_node_new_with_id("enum", "clang::CanThrowResult", nullptr);
  data_model.clang_CT_Cannot_ = arboretum_node_new_with_id("enum_value", "clang::CT_Cannot", nullptr);
  data_model.clang_CT_Dependent_ = arboretum_node_new_with_id("enum_value", "clang::CT_Dependent", nullptr);
  data_model.clang_CT_Can_ = arboretum_node_new_with_id("enum_value", "clang::CT_Can", nullptr);

  // clang::CapturedRegionKind
  data_model.clang_CapturedRegionKind_ = arboretum_node_new_with_id("enum", "clang::CapturedRegionKind", nullptr);
  data_model.clang_CR_Default_ = arboretum_node_new_with_id("enum_value", "clang::CR_Default", nullptr);
  data_model.clang_CR_ObjCAtFinally_ = arboretum_node_new_with_id("enum_value", "clang::CR_ObjCAtFinally", nullptr);
  data_model.clang_CR_OpenMP_ = arboretum_node_new_with_id("enum_value", "clang::CR_OpenMP", nullptr);

  // clang::CastKind
  data_model.clang_CastKind_ = arboretum_node_new_with_id("enum", "clang::CastKind", nullptr);
  data_model.clang_CK_Dependent_ = arboretum_node_new_with_id("enum_value", "clang::CK_Dependent", nullptr);
  data_model.clang_CK_BitCast_ = arboretum_node_new_with_id("enum_value", "clang::CK_BitCast", nullptr);
  data_model.clang_CK_LValueBitCast_ = arboretum_node_new_with_id("enum_value", "clang::CK_LValueBitCast", nullptr);
  data_model.clang_CK_LValueToRValueBitCast_ = arboretum_node_new_with_id("enum_value", "clang::CK_LValueToRValueBitCast", nullptr);
  data_model.clang_CK_LValueToRValue_ = arboretum_node_new_with_id("enum_value", "clang::CK_LValueToRValue", nullptr);
  data_model.clang_CK_NoOp_ = arboretum_node_new_with_id("enum_value", "clang::CK_NoOp", nullptr);
  data_model.clang_CK_BaseToDerived_ = arboretum_node_new_with_id("enum_value", "clang::CK_BaseToDerived", nullptr);
  data_model.clang_CK_DerivedToBase_ = arboretum_node_new_with_id("enum_value", "clang::CK_DerivedToBase", nullptr);
  data_model.clang_CK_UncheckedDerivedToBase_ = arboretum_node_new_with_id("enum_value", "clang::CK_UncheckedDerivedToBase", nullptr);
  data_model.clang_CK_Dynamic_ = arboretum_node_new_with_id("enum_value", "clang::CK_Dynamic", nullptr);
  data_model.clang_CK_ToUnion_ = arboretum_node_new_with_id("enum_value", "clang::CK_ToUnion", nullptr);
  data_model.clang_CK_ArrayToPointerDecay_ = arboretum_node_new_with_id("enum_value", "clang::CK_ArrayToPointerDecay", nullptr);
  data_model.clang_CK_FunctionToPointerDecay_ = arboretum_node_new_with_id("enum_value", "clang::CK_FunctionToPointerDecay", nullptr);
  data_model.clang_CK_NullToPointer_ = arboretum_node_new_with_id("enum_value", "clang::CK_NullToPointer", nullptr);
  data_model.clang_CK_NullToMemberPointer_ = arboretum_node_new_with_id("enum_value", "clang::CK_NullToMemberPointer", nullptr);
  data_model.clang_CK_BaseToDerivedMemberPointer_ = arboretum_node_new_with_id("enum_value", "clang::CK_BaseToDerivedMemberPointer", nullptr);
  data_model.clang_CK_DerivedToBaseMemberPointer_ = arboretum_node_new_with_id("enum_value", "clang::CK_DerivedToBaseMemberPointer", nullptr);
  data_model.clang_CK_MemberPointerToBoolean_ = arboretum_node_new_with_id("enum_value", "clang::CK_MemberPointerToBoolean", nullptr);
  data_model.clang_CK_ReinterpretMemberPointer_ = arboretum_node_new_with_id("enum_value", "clang::CK_ReinterpretMemberPointer", nullptr);
  data_model.clang_CK_UserDefinedConversion_ = arboretum_node_new_with_id("enum_value", "clang::CK_UserDefinedConversion", nullptr);
  data_model.clang_CK_ConstructorConversion_ = arboretum_node_new_with_id("enum_value", "clang::CK_ConstructorConversion", nullptr);
  data_model.clang_CK_IntegralToPointer_ = arboretum_node_new_with_id("enum_value", "clang::CK_IntegralToPointer", nullptr);
  data_model.clang_CK_PointerToIntegral_ = arboretum_node_new_with_id("enum_value", "clang::CK_PointerToIntegral", nullptr);
  data_model.clang_CK_PointerToBoolean_ = arboretum_node_new_with_id("enum_value", "clang::CK_PointerToBoolean", nullptr);
  data_model.clang_CK_ToVoid_ = arboretum_node_new_with_id("enum_value", "clang::CK_ToVoid", nullptr);
  data_model.clang_CK_MatrixCast_ = arboretum_node_new_with_id("enum_value", "clang::CK_MatrixCast", nullptr);
  data_model.clang_CK_VectorSplat_ = arboretum_node_new_with_id("enum_value", "clang::CK_VectorSplat", nullptr);
  data_model.clang_CK_IntegralCast_ = arboretum_node_new_with_id("enum_value", "clang::CK_IntegralCast", nullptr);
  data_model.clang_CK_IntegralToBoolean_ = arboretum_node_new_with_id("enum_value", "clang::CK_IntegralToBoolean", nullptr);
  data_model.clang_CK_IntegralToFloating_ = arboretum_node_new_with_id("enum_value", "clang::CK_IntegralToFloating", nullptr);
  data_model.clang_CK_FloatingToFixedPoint_ = arboretum_node_new_with_id("enum_value", "clang::CK_FloatingToFixedPoint", nullptr);
  data_model.clang_CK_FixedPointToFloating_ = arboretum_node_new_with_id("enum_value", "clang::CK_FixedPointToFloating", nullptr);
  data_model.clang_CK_FixedPointCast_ = arboretum_node_new_with_id("enum_value", "clang::CK_FixedPointCast", nullptr);
  data_model.clang_CK_FixedPointToIntegral_ = arboretum_node_new_with_id("enum_value", "clang::CK_FixedPointToIntegral", nullptr);
  data_model.clang_CK_IntegralToFixedPoint_ = arboretum_node_new_with_id("enum_value", "clang::CK_IntegralToFixedPoint", nullptr);
  data_model.clang_CK_FixedPointToBoolean_ = arboretum_node_new_with_id("enum_value", "clang::CK_FixedPointToBoolean", nullptr);
  data_model.clang_CK_FloatingToIntegral_ = arboretum_node_new_with_id("enum_value", "clang::CK_FloatingToIntegral", nullptr);
  data_model.clang_CK_FloatingToBoolean_ = arboretum_node_new_with_id("enum_value", "clang::CK_FloatingToBoolean", nullptr);
  data_model.clang_CK_BooleanToSignedIntegral_ = arboretum_node_new_with_id("enum_value", "clang::CK_BooleanToSignedIntegral", nullptr);
  data_model.clang_CK_FloatingCast_ = arboretum_node_new_with_id("enum_value", "clang::CK_FloatingCast", nullptr);
  data_model.clang_CK_CPointerToObjCPointerCast_ = arboretum_node_new_with_id("enum_value", "clang::CK_CPointerToObjCPointerCast", nullptr);
  data_model.clang_CK_BlockPointerToObjCPointerCast_ = arboretum_node_new_with_id("enum_value", "clang::CK_BlockPointerToObjCPointerCast", nullptr);
  data_model.clang_CK_AnyPointerToBlockPointerCast_ = arboretum_node_new_with_id("enum_value", "clang::CK_AnyPointerToBlockPointerCast", nullptr);
  data_model.clang_CK_ObjCObjectLValueCast_ = arboretum_node_new_with_id("enum_value", "clang::CK_ObjCObjectLValueCast", nullptr);
  data_model.clang_CK_FloatingRealToComplex_ = arboretum_node_new_with_id("enum_value", "clang::CK_FloatingRealToComplex", nullptr);
  data_model.clang_CK_FloatingComplexToReal_ = arboretum_node_new_with_id("enum_value", "clang::CK_FloatingComplexToReal", nullptr);
  data_model.clang_CK_FloatingComplexToBoolean_ = arboretum_node_new_with_id("enum_value", "clang::CK_FloatingComplexToBoolean", nullptr);
  data_model.clang_CK_FloatingComplexCast_ = arboretum_node_new_with_id("enum_value", "clang::CK_FloatingComplexCast", nullptr);
  data_model.clang_CK_FloatingComplexToIntegralComplex_ = arboretum_node_new_with_id("enum_value", "clang::CK_FloatingComplexToIntegralComplex", nullptr);
  data_model.clang_CK_IntegralRealToComplex_ = arboretum_node_new_with_id("enum_value", "clang::CK_IntegralRealToComplex", nullptr);
  data_model.clang_CK_IntegralComplexToReal_ = arboretum_node_new_with_id("enum_value", "clang::CK_IntegralComplexToReal", nullptr);
  data_model.clang_CK_IntegralComplexToBoolean_ = arboretum_node_new_with_id("enum_value", "clang::CK_IntegralComplexToBoolean", nullptr);
  data_model.clang_CK_IntegralComplexCast_ = arboretum_node_new_with_id("enum_value", "clang::CK_IntegralComplexCast", nullptr);
  data_model.clang_CK_IntegralComplexToFloatingComplex_ = arboretum_node_new_with_id("enum_value", "clang::CK_IntegralComplexToFloatingComplex", nullptr);
  data_model.clang_CK_ARCProduceObject_ = arboretum_node_new_with_id("enum_value", "clang::CK_ARCProduceObject", nullptr);
  data_model.clang_CK_ARCConsumeObject_ = arboretum_node_new_with_id("enum_value", "clang::CK_ARCConsumeObject", nullptr);
  data_model.clang_CK_ARCReclaimReturnedObject_ = arboretum_node_new_with_id("enum_value", "clang::CK_ARCReclaimReturnedObject", nullptr);
  data_model.clang_CK_ARCExtendBlockObject_ = arboretum_node_new_with_id("enum_value", "clang::CK_ARCExtendBlockObject", nullptr);
  data_model.clang_CK_AtomicToNonAtomic_ = arboretum_node_new_with_id("enum_value", "clang::CK_AtomicToNonAtomic", nullptr);
  data_model.clang_CK_NonAtomicToAtomic_ = arboretum_node_new_with_id("enum_value", "clang::CK_NonAtomicToAtomic", nullptr);
  data_model.clang_CK_CopyAndAutoreleaseBlockObject_ = arboretum_node_new_with_id("enum_value", "clang::CK_CopyAndAutoreleaseBlockObject", nullptr);
  data_model.clang_CK_BuiltinFnToFnPtr_ = arboretum_node_new_with_id("enum_value", "clang::CK_BuiltinFnToFnPtr", nullptr);
  data_model.clang_CK_ZeroToOCLOpaqueType_ = arboretum_node_new_with_id("enum_value", "clang::CK_ZeroToOCLOpaqueType", nullptr);
  data_model.clang_CK_AddressSpaceConversion_ = arboretum_node_new_with_id("enum_value", "clang::CK_AddressSpaceConversion", nullptr);
  data_model.clang_CK_IntToOCLSampler_ = arboretum_node_new_with_id("enum_value", "clang::CK_IntToOCLSampler", nullptr);

  // clang::CharacterLiteral::CharacterKind
  data_model.clang_CharacterLiteral_CharacterKind_ = arboretum_node_new_with_id("enum", "clang::CharacterLiteral::CharacterKind", nullptr);
  data_model.clang_CharacterLiteral_Ascii_ = arboretum_node_new_with_id("enum_value", "clang::CharacterLiteral::Ascii", nullptr);
  data_model.clang_CharacterLiteral_Wide_ = arboretum_node_new_with_id("enum_value", "clang::CharacterLiteral::Wide", nullptr);
  data_model.clang_CharacterLiteral_UTF8_ = arboretum_node_new_with_id("enum_value", "clang::CharacterLiteral::UTF8", nullptr);
  data_model.clang_CharacterLiteral_UTF16_ = arboretum_node_new_with_id("enum_value", "clang::CharacterLiteral::UTF16", nullptr);
  data_model.clang_CharacterLiteral_UTF32_ = arboretum_node_new_with_id("enum_value", "clang::CharacterLiteral::UTF32", nullptr);

  // clang::ConstantExpr::ResultStorageKind
  data_model.clang_ConstantExpr_ResultStorageKind_ = arboretum_node_new_with_id("enum", "clang::ConstantExpr::ResultStorageKind", nullptr);
  data_model.clang_ConstantExpr_RSK_None_ = arboretum_node_new_with_id("enum_value", "clang::ConstantExpr::RSK_None", nullptr);
  data_model.clang_ConstantExpr_RSK_Int64_ = arboretum_node_new_with_id("enum_value", "clang::ConstantExpr::RSK_Int64", nullptr);
  data_model.clang_ConstantExpr_RSK_APValue_ = arboretum_node_new_with_id("enum_value", "clang::ConstantExpr::RSK_APValue", nullptr);

  // clang::ConstexprSpecKind
  data_model.clang_ConstexprSpecKind_ = arboretum_node_new_with_id("enum", "clang::ConstexprSpecKind", nullptr);
  data_model.clang_ConstexprSpecKind_Unspecified_ = arboretum_node_new_with_id("enum_value", "clang::ConstexprSpecKind::Unspecified", nullptr);
  data_model.clang_ConstexprSpecKind_Constexpr_ = arboretum_node_new_with_id("enum_value", "clang::ConstexprSpecKind::Constexpr", nullptr);
  data_model.clang_ConstexprSpecKind_Consteval_ = arboretum_node_new_with_id("enum_value", "clang::ConstexprSpecKind::Consteval", nullptr);
  data_model.clang_ConstexprSpecKind_Constinit_ = arboretum_node_new_with_id("enum_value", "clang::ConstexprSpecKind::Constinit", nullptr);

  // clang::Decl::FriendObjectKind
  data_model.clang_Decl_FriendObjectKind_ = arboretum_node_new_with_id("enum", "clang::Decl::FriendObjectKind", nullptr);
  data_model.clang_Decl_FOK_None_ = arboretum_node_new_with_id("enum_value", "clang::Decl::FOK_None", nullptr);
  data_model.clang_Decl_FOK_Declared_ = arboretum_node_new_with_id("enum_value", "clang::Decl::FOK_Declared", nullptr);
  data_model.clang_Decl_FOK_Undeclared_ = arboretum_node_new_with_id("enum_value", "clang::Decl::FOK_Undeclared", nullptr);

  // clang::Decl::Kind
  data_model.clang_Decl_Kind_ = arboretum_node_new_with_id("enum", "clang::Decl::Kind", nullptr);
  data_model.clang_Decl_AccessSpec_ = arboretum_node_new_with_id("enum_value", "clang::Decl::AccessSpec", nullptr);
  data_model.clang_Decl_Block_ = arboretum_node_new_with_id("enum_value", "clang::Decl::Block", nullptr);
  data_model.clang_Decl_Captured_ = arboretum_node_new_with_id("enum_value", "clang::Decl::Captured", nullptr);
  data_model.clang_Decl_ClassScopeFunctionSpecialization_ = arboretum_node_new_with_id("enum_value", "clang::Decl::ClassScopeFunctionSpecialization", nullptr);
  data_model.clang_Decl_Empty_ = arboretum_node_new_with_id("enum_value", "clang::Decl::Empty", nullptr);
  data_model.clang_Decl_Export_ = arboretum_node_new_with_id("enum_value", "clang::Decl::Export", nullptr);
  data_model.clang_Decl_ExternCContext_ = arboretum_node_new_with_id("enum_value", "clang::Decl::ExternCContext", nullptr);
  data_model.clang_Decl_FileScopeAsm_ = arboretum_node_new_with_id("enum_value", "clang::Decl::FileScopeAsm", nullptr);
  data_model.clang_Decl_Friend_ = arboretum_node_new_with_id("enum_value", "clang::Decl::Friend", nullptr);
  data_model.clang_Decl_FriendTemplate_ = arboretum_node_new_with_id("enum_value", "clang::Decl::FriendTemplate", nullptr);
  data_model.clang_Decl_ImplicitConceptSpecialization_ = arboretum_node_new_with_id("enum_value", "clang::Decl::ImplicitConceptSpecialization", nullptr);
  data_model.clang_Decl_Import_ = arboretum_node_new_with_id("enum_value", "clang::Decl::Import", nullptr);
  data_model.clang_Decl_LifetimeExtendedTemporary_ = arboretum_node_new_with_id("enum_value", "clang::Decl::LifetimeExtendedTemporary", nullptr);
  data_model.clang_Decl_LinkageSpec_ = arboretum_node_new_with_id("enum_value", "clang::Decl::LinkageSpec", nullptr);
  data_model.clang_Decl_Using_ = arboretum_node_new_with_id("enum_value", "clang::Decl::Using", nullptr);
  data_model.clang_Decl_UsingEnum_ = arboretum_node_new_with_id("enum_value", "clang::Decl::UsingEnum", nullptr);
  data_model.clang_Decl_firstBaseUsing_ = arboretum_node_new_with_id("enum_value", "clang::Decl::firstBaseUsing", nullptr);
  data_model.clang_Decl_lastBaseUsing_ = arboretum_node_new_with_id("enum_value", "clang::Decl::lastBaseUsing", nullptr);
  data_model.clang_Decl_HLSLBuffer_ = arboretum_node_new_with_id("enum_value", "clang::Decl::HLSLBuffer", nullptr);
  data_model.clang_Decl_Label_ = arboretum_node_new_with_id("enum_value", "clang::Decl::Label", nullptr);
  data_model.clang_Decl_Namespace_ = arboretum_node_new_with_id("enum_value", "clang::Decl::Namespace", nullptr);
  data_model.clang_Decl_NamespaceAlias_ = arboretum_node_new_with_id("enum_value", "clang::Decl::NamespaceAlias", nullptr);
  data_model.clang_Decl_ObjCCompatibleAlias_ = arboretum_node_new_with_id("enum_value", "clang::Decl::ObjCCompatibleAlias", nullptr);
  data_model.clang_Decl_ObjCCategory_ = arboretum_node_new_with_id("enum_value", "clang::Decl::ObjCCategory", nullptr);
  data_model.clang_Decl_ObjCCategoryImpl_ = arboretum_node_new_with_id("enum_value", "clang::Decl::ObjCCategoryImpl", nullptr);
  data_model.clang_Decl_ObjCImplementation_ = arboretum_node_new_with_id("enum_value", "clang::Decl::ObjCImplementation", nullptr);
  data_model.clang_Decl_firstObjCImpl_ = arboretum_node_new_with_id("enum_value", "clang::Decl::firstObjCImpl", nullptr);
  data_model.clang_Decl_lastObjCImpl_ = arboretum_node_new_with_id("enum_value", "clang::Decl::lastObjCImpl", nullptr);
  data_model.clang_Decl_ObjCInterface_ = arboretum_node_new_with_id("enum_value", "clang::Decl::ObjCInterface", nullptr);
  data_model.clang_Decl_ObjCProtocol_ = arboretum_node_new_with_id("enum_value", "clang::Decl::ObjCProtocol", nullptr);
  data_model.clang_Decl_firstObjCContainer_ = arboretum_node_new_with_id("enum_value", "clang::Decl::firstObjCContainer", nullptr);
  data_model.clang_Decl_lastObjCContainer_ = arboretum_node_new_with_id("enum_value", "clang::Decl::lastObjCContainer", nullptr);
  data_model.clang_Decl_ObjCMethod_ = arboretum_node_new_with_id("enum_value", "clang::Decl::ObjCMethod", nullptr);
  data_model.clang_Decl_ObjCProperty_ = arboretum_node_new_with_id("enum_value", "clang::Decl::ObjCProperty", nullptr);
  data_model.clang_Decl_BuiltinTemplate_ = arboretum_node_new_with_id("enum_value", "clang::Decl::BuiltinTemplate", nullptr);
  data_model.clang_Decl_Concept_ = arboretum_node_new_with_id("enum_value", "clang::Decl::Concept", nullptr);
  data_model.clang_Decl_ClassTemplate_ = arboretum_node_new_with_id("enum_value", "clang::Decl::ClassTemplate", nullptr);
  data_model.clang_Decl_FunctionTemplate_ = arboretum_node_new_with_id("enum_value", "clang::Decl::FunctionTemplate", nullptr);
  data_model.clang_Decl_TypeAliasTemplate_ = arboretum_node_new_with_id("enum_value", "clang::Decl::TypeAliasTemplate", nullptr);
  data_model.clang_Decl_VarTemplate_ = arboretum_node_new_with_id("enum_value", "clang::Decl::VarTemplate", nullptr);
  data_model.clang_Decl_firstRedeclarableTemplate_ = arboretum_node_new_with_id("enum_value", "clang::Decl::firstRedeclarableTemplate", nullptr);
  data_model.clang_Decl_lastRedeclarableTemplate_ = arboretum_node_new_with_id("enum_value", "clang::Decl::lastRedeclarableTemplate", nullptr);
  data_model.clang_Decl_TemplateTemplateParm_ = arboretum_node_new_with_id("enum_value", "clang::Decl::TemplateTemplateParm", nullptr);
  data_model.clang_Decl_firstTemplate_ = arboretum_node_new_with_id("enum_value", "clang::Decl::firstTemplate", nullptr);
  data_model.clang_Decl_lastTemplate_ = arboretum_node_new_with_id("enum_value", "clang::Decl::lastTemplate", nullptr);
  data_model.clang_Decl_Enum_ = arboretum_node_new_with_id("enum_value", "clang::Decl::Enum", nullptr);
  data_model.clang_Decl_Record_ = arboretum_node_new_with_id("enum_value", "clang::Decl::Record", nullptr);
  data_model.clang_Decl_CXXRecord_ = arboretum_node_new_with_id("enum_value", "clang::Decl::CXXRecord", nullptr);
  data_model.clang_Decl_ClassTemplateSpecialization_ = arboretum_node_new_with_id("enum_value", "clang::Decl::ClassTemplateSpecialization", nullptr);
  data_model.clang_Decl_ClassTemplatePartialSpecialization_ = arboretum_node_new_with_id("enum_value", "clang::Decl::ClassTemplatePartialSpecialization", nullptr);
  data_model.clang_Decl_firstClassTemplateSpecialization_ = arboretum_node_new_with_id("enum_value", "clang::Decl::firstClassTemplateSpecialization", nullptr);
  data_model.clang_Decl_lastClassTemplateSpecialization_ = arboretum_node_new_with_id("enum_value", "clang::Decl::lastClassTemplateSpecialization", nullptr);
  data_model.clang_Decl_firstCXXRecord_ = arboretum_node_new_with_id("enum_value", "clang::Decl::firstCXXRecord", nullptr);
  data_model.clang_Decl_lastCXXRecord_ = arboretum_node_new_with_id("enum_value", "clang::Decl::lastCXXRecord", nullptr);
  data_model.clang_Decl_firstRecord_ = arboretum_node_new_with_id("enum_value", "clang::Decl::firstRecord", nullptr);
  data_model.clang_Decl_lastRecord_ = arboretum_node_new_with_id("enum_value", "clang::Decl::lastRecord", nullptr);
  data_model.clang_Decl_firstTag_ = arboretum_node_new_with_id("enum_value", "clang::Decl::firstTag", nullptr);
  data_model.clang_Decl_lastTag_ = arboretum_node_new_with_id("enum_value", "clang::Decl::lastTag", nullptr);
  data_model.clang_Decl_TemplateTypeParm_ = arboretum_node_new_with_id("enum_value", "clang::Decl::TemplateTypeParm", nullptr);
  data_model.clang_Decl_ObjCTypeParam_ = arboretum_node_new_with_id("enum_value", "clang::Decl::ObjCTypeParam", nullptr);
  data_model.clang_Decl_TypeAlias_ = arboretum_node_new_with_id("enum_value", "clang::Decl::TypeAlias", nullptr);
  data_model.clang_Decl_Typedef_ = arboretum_node_new_with_id("enum_value", "clang::Decl::Typedef", nullptr);
  data_model.clang_Decl_firstTypedefName_ = arboretum_node_new_with_id("enum_value", "clang::Decl::firstTypedefName", nullptr);
  data_model.clang_Decl_lastTypedefName_ = arboretum_node_new_with_id("enum_value", "clang::Decl::lastTypedefName", nullptr);
  data_model.clang_Decl_UnresolvedUsingTypename_ = arboretum_node_new_with_id("enum_value", "clang::Decl::UnresolvedUsingTypename", nullptr);
  data_model.clang_Decl_firstType_ = arboretum_node_new_with_id("enum_value", "clang::Decl::firstType", nullptr);
  data_model.clang_Decl_lastType_ = arboretum_node_new_with_id("enum_value", "clang::Decl::lastType", nullptr);
  data_model.clang_Decl_UnresolvedUsingIfExists_ = arboretum_node_new_with_id("enum_value", "clang::Decl::UnresolvedUsingIfExists", nullptr);
  data_model.clang_Decl_UsingDirective_ = arboretum_node_new_with_id("enum_value", "clang::Decl::UsingDirective", nullptr);
  data_model.clang_Decl_UsingPack_ = arboretum_node_new_with_id("enum_value", "clang::Decl::UsingPack", nullptr);
  data_model.clang_Decl_UsingShadow_ = arboretum_node_new_with_id("enum_value", "clang::Decl::UsingShadow", nullptr);
  data_model.clang_Decl_ConstructorUsingShadow_ = arboretum_node_new_with_id("enum_value", "clang::Decl::ConstructorUsingShadow", nullptr);
  data_model.clang_Decl_firstUsingShadow_ = arboretum_node_new_with_id("enum_value", "clang::Decl::firstUsingShadow", nullptr);
  data_model.clang_Decl_lastUsingShadow_ = arboretum_node_new_with_id("enum_value", "clang::Decl::lastUsingShadow", nullptr);
  data_model.clang_Decl_Binding_ = arboretum_node_new_with_id("enum_value", "clang::Decl::Binding", nullptr);
  data_model.clang_Decl_Field_ = arboretum_node_new_with_id("enum_value", "clang::Decl::Field", nullptr);
  data_model.clang_Decl_ObjCAtDefsField_ = arboretum_node_new_with_id("enum_value", "clang::Decl::ObjCAtDefsField", nullptr);
  data_model.clang_Decl_ObjCIvar_ = arboretum_node_new_with_id("enum_value", "clang::Decl::ObjCIvar", nullptr);
  data_model.clang_Decl_firstField_ = arboretum_node_new_with_id("enum_value", "clang::Decl::firstField", nullptr);
  data_model.clang_Decl_lastField_ = arboretum_node_new_with_id("enum_value", "clang::Decl::lastField", nullptr);
  data_model.clang_Decl_Function_ = arboretum_node_new_with_id("enum_value", "clang::Decl::Function", nullptr);
  data_model.clang_Decl_CXXDeductionGuide_ = arboretum_node_new_with_id("enum_value", "clang::Decl::CXXDeductionGuide", nullptr);
  data_model.clang_Decl_CXXMethod_ = arboretum_node_new_with_id("enum_value", "clang::Decl::CXXMethod", nullptr);
  data_model.clang_Decl_CXXConstructor_ = arboretum_node_new_with_id("enum_value", "clang::Decl::CXXConstructor", nullptr);
  data_model.clang_Decl_CXXConversion_ = arboretum_node_new_with_id("enum_value", "clang::Decl::CXXConversion", nullptr);
  data_model.clang_Decl_CXXDestructor_ = arboretum_node_new_with_id("enum_value", "clang::Decl::CXXDestructor", nullptr);
  data_model.clang_Decl_firstCXXMethod_ = arboretum_node_new_with_id("enum_value", "clang::Decl::firstCXXMethod", nullptr);
  data_model.clang_Decl_lastCXXMethod_ = arboretum_node_new_with_id("enum_value", "clang::Decl::lastCXXMethod", nullptr);
  data_model.clang_Decl_firstFunction_ = arboretum_node_new_with_id("enum_value", "clang::Decl::firstFunction", nullptr);
  data_model.clang_Decl_lastFunction_ = arboretum_node_new_with_id("enum_value", "clang::Decl::lastFunction", nullptr);
  data_model.clang_Decl_MSProperty_ = arboretum_node_new_with_id("enum_value", "clang::Decl::MSProperty", nullptr);
  data_model.clang_Decl_NonTypeTemplateParm_ = arboretum_node_new_with_id("enum_value", "clang::Decl::NonTypeTemplateParm", nullptr);
  data_model.clang_Decl_Var_ = arboretum_node_new_with_id("enum_value", "clang::Decl::Var", nullptr);
  data_model.clang_Decl_Decomposition_ = arboretum_node_new_with_id("enum_value", "clang::Decl::Decomposition", nullptr);
  data_model.clang_Decl_ImplicitParam_ = arboretum_node_new_with_id("enum_value", "clang::Decl::ImplicitParam", nullptr);
  data_model.clang_Decl_OMPCapturedExpr_ = arboretum_node_new_with_id("enum_value", "clang::Decl::OMPCapturedExpr", nullptr);
  data_model.clang_Decl_ParmVar_ = arboretum_node_new_with_id("enum_value", "clang::Decl::ParmVar", nullptr);
  data_model.clang_Decl_VarTemplateSpecialization_ = arboretum_node_new_with_id("enum_value", "clang::Decl::VarTemplateSpecialization", nullptr);
  data_model.clang_Decl_VarTemplatePartialSpecialization_ = arboretum_node_new_with_id("enum_value", "clang::Decl::VarTemplatePartialSpecialization", nullptr);
  data_model.clang_Decl_firstVarTemplateSpecialization_ = arboretum_node_new_with_id("enum_value", "clang::Decl::firstVarTemplateSpecialization", nullptr);
  data_model.clang_Decl_lastVarTemplateSpecialization_ = arboretum_node_new_with_id("enum_value", "clang::Decl::lastVarTemplateSpecialization", nullptr);
  data_model.clang_Decl_firstVar_ = arboretum_node_new_with_id("enum_value", "clang::Decl::firstVar", nullptr);
  data_model.clang_Decl_lastVar_ = arboretum_node_new_with_id("enum_value", "clang::Decl::lastVar", nullptr);
  data_model.clang_Decl_firstDeclarator_ = arboretum_node_new_with_id("enum_value", "clang::Decl::firstDeclarator", nullptr);
  data_model.clang_Decl_lastDeclarator_ = arboretum_node_new_with_id("enum_value", "clang::Decl::lastDeclarator", nullptr);
  data_model.clang_Decl_EnumConstant_ = arboretum_node_new_with_id("enum_value", "clang::Decl::EnumConstant", nullptr);
  data_model.clang_Decl_IndirectField_ = arboretum_node_new_with_id("enum_value", "clang::Decl::IndirectField", nullptr);
  data_model.clang_Decl_MSGuid_ = arboretum_node_new_with_id("enum_value", "clang::Decl::MSGuid", nullptr);
  data_model.clang_Decl_OMPDeclareMapper_ = arboretum_node_new_with_id("enum_value", "clang::Decl::OMPDeclareMapper", nullptr);
  data_model.clang_Decl_OMPDeclareReduction_ = arboretum_node_new_with_id("enum_value", "clang::Decl::OMPDeclareReduction", nullptr);
  data_model.clang_Decl_TemplateParamObject_ = arboretum_node_new_with_id("enum_value", "clang::Decl::TemplateParamObject", nullptr);
  data_model.clang_Decl_UnnamedGlobalConstant_ = arboretum_node_new_with_id("enum_value", "clang::Decl::UnnamedGlobalConstant", nullptr);
  data_model.clang_Decl_UnresolvedUsingValue_ = arboretum_node_new_with_id("enum_value", "clang::Decl::UnresolvedUsingValue", nullptr);
  data_model.clang_Decl_firstValue_ = arboretum_node_new_with_id("enum_value", "clang::Decl::firstValue", nullptr);
  data_model.clang_Decl_lastValue_ = arboretum_node_new_with_id("enum_value", "clang::Decl::lastValue", nullptr);
  data_model.clang_Decl_firstNamed_ = arboretum_node_new_with_id("enum_value", "clang::Decl::firstNamed", nullptr);
  data_model.clang_Decl_lastNamed_ = arboretum_node_new_with_id("enum_value", "clang::Decl::lastNamed", nullptr);
  data_model.clang_Decl_OMPAllocate_ = arboretum_node_new_with_id("enum_value", "clang::Decl::OMPAllocate", nullptr);
  data_model.clang_Decl_OMPRequires_ = arboretum_node_new_with_id("enum_value", "clang::Decl::OMPRequires", nullptr);
  data_model.clang_Decl_OMPThreadPrivate_ = arboretum_node_new_with_id("enum_value", "clang::Decl::OMPThreadPrivate", nullptr);
  data_model.clang_Decl_ObjCPropertyImpl_ = arboretum_node_new_with_id("enum_value", "clang::Decl::ObjCPropertyImpl", nullptr);
  data_model.clang_Decl_PragmaComment_ = arboretum_node_new_with_id("enum_value", "clang::Decl::PragmaComment", nullptr);
  data_model.clang_Decl_PragmaDetectMismatch_ = arboretum_node_new_with_id("enum_value", "clang::Decl::PragmaDetectMismatch", nullptr);
  data_model.clang_Decl_RequiresExprBody_ = arboretum_node_new_with_id("enum_value", "clang::Decl::RequiresExprBody", nullptr);
  data_model.clang_Decl_StaticAssert_ = arboretum_node_new_with_id("enum_value", "clang::Decl::StaticAssert", nullptr);
  data_model.clang_Decl_TopLevelStmt_ = arboretum_node_new_with_id("enum_value", "clang::Decl::TopLevelStmt", nullptr);
  data_model.clang_Decl_TranslationUnit_ = arboretum_node_new_with_id("enum_value", "clang::Decl::TranslationUnit", nullptr);
  data_model.clang_Decl_firstDecl_ = arboretum_node_new_with_id("enum_value", "clang::Decl::firstDecl", nullptr);
  data_model.clang_Decl_lastDecl_ = arboretum_node_new_with_id("enum_value", "clang::Decl::lastDecl", nullptr);

  // clang::Decl::ModuleOwnershipKind
  data_model.clang_Decl_ModuleOwnershipKind_ = arboretum_node_new_with_id("enum", "clang::Decl::ModuleOwnershipKind", nullptr);
  data_model.clang_Decl_ModuleOwnershipKind_Unowned_ = arboretum_node_new_with_id("enum_value", "clang::Decl::ModuleOwnershipKind::Unowned", nullptr);
  data_model.clang_Decl_ModuleOwnershipKind_Visible_ = arboretum_node_new_with_id("enum_value", "clang::Decl::ModuleOwnershipKind::Visible", nullptr);
  data_model.clang_Decl_ModuleOwnershipKind_VisibleWhenImported_ = arboretum_node_new_with_id("enum_value", "clang::Decl::ModuleOwnershipKind::VisibleWhenImported", nullptr);
  data_model.clang_Decl_ModuleOwnershipKind_ReachableWhenImported_ = arboretum_node_new_with_id("enum_value", "clang::Decl::ModuleOwnershipKind::ReachableWhenImported", nullptr);
  data_model.clang_Decl_ModuleOwnershipKind_ModulePrivate_ = arboretum_node_new_with_id("enum_value", "clang::Decl::ModuleOwnershipKind::ModulePrivate", nullptr);

  // clang::Decl::ObjCDeclQualifier
  data_model.clang_Decl_ObjCDeclQualifier_ = arboretum_node_new_with_id("enum", "clang::Decl::ObjCDeclQualifier", nullptr);
  data_model.clang_Decl_OBJC_TQ_None_ = arboretum_node_new_with_id("enum_value", "clang::Decl::OBJC_TQ_None", nullptr);
  data_model.clang_Decl_OBJC_TQ_In_ = arboretum_node_new_with_id("enum_value", "clang::Decl::OBJC_TQ_In", nullptr);
  data_model.clang_Decl_OBJC_TQ_Inout_ = arboretum_node_new_with_id("enum_value", "clang::Decl::OBJC_TQ_Inout", nullptr);
  data_model.clang_Decl_OBJC_TQ_Out_ = arboretum_node_new_with_id("enum_value", "clang::Decl::OBJC_TQ_Out", nullptr);
  data_model.clang_Decl_OBJC_TQ_Bycopy_ = arboretum_node_new_with_id("enum_value", "clang::Decl::OBJC_TQ_Bycopy", nullptr);
  data_model.clang_Decl_OBJC_TQ_Byref_ = arboretum_node_new_with_id("enum_value", "clang::Decl::OBJC_TQ_Byref", nullptr);
  data_model.clang_Decl_OBJC_TQ_Oneway_ = arboretum_node_new_with_id("enum_value", "clang::Decl::OBJC_TQ_Oneway", nullptr);
  data_model.clang_Decl_OBJC_TQ_CSNullability_ = arboretum_node_new_with_id("enum_value", "clang::Decl::OBJC_TQ_CSNullability", nullptr);

  // clang::DeductionCandidate
  data_model.clang_DeductionCandidate_ = arboretum_node_new_with_id("enum", "clang::DeductionCandidate", nullptr);
  data_model.clang_DeductionCandidate_Normal_ = arboretum_node_new_with_id("enum_value", "clang::DeductionCandidate::Normal", nullptr);
  data_model.clang_DeductionCandidate_Copy_ = arboretum_node_new_with_id("enum_value", "clang::DeductionCandidate::Copy", nullptr);
  data_model.clang_DeductionCandidate_Aggregate_ = arboretum_node_new_with_id("enum_value", "clang::DeductionCandidate::Aggregate", nullptr);

  // clang::ElaboratedTypeKeyword
  data_model.clang_ElaboratedTypeKeyword_ = arboretum_node_new_with_id("enum", "clang::ElaboratedTypeKeyword", nullptr);
  data_model.clang_ETK_Struct_ = arboretum_node_new_with_id("enum_value", "clang::ETK_Struct", nullptr);
  data_model.clang_ETK_Interface_ = arboretum_node_new_with_id("enum_value", "clang::ETK_Interface", nullptr);
  data_model.clang_ETK_Union_ = arboretum_node_new_with_id("enum_value", "clang::ETK_Union", nullptr);
  data_model.clang_ETK_Class_ = arboretum_node_new_with_id("enum_value", "clang::ETK_Class", nullptr);
  data_model.clang_ETK_Enum_ = arboretum_node_new_with_id("enum_value", "clang::ETK_Enum", nullptr);
  data_model.clang_ETK_Typename_ = arboretum_node_new_with_id("enum_value", "clang::ETK_Typename", nullptr);
  data_model.clang_ETK_None_ = arboretum_node_new_with_id("enum_value", "clang::ETK_None", nullptr);

  // clang::ExceptionSpecificationType
  data_model.clang_ExceptionSpecificationType_ = arboretum_node_new_with_id("enum", "clang::ExceptionSpecificationType", nullptr);
  data_model.clang_EST_None_ = arboretum_node_new_with_id("enum_value", "clang::EST_None", nullptr);
  data_model.clang_EST_DynamicNone_ = arboretum_node_new_with_id("enum_value", "clang::EST_DynamicNone", nullptr);
  data_model.clang_EST_Dynamic_ = arboretum_node_new_with_id("enum_value", "clang::EST_Dynamic", nullptr);
  data_model.clang_EST_MSAny_ = arboretum_node_new_with_id("enum_value", "clang::EST_MSAny", nullptr);
  data_model.clang_EST_NoThrow_ = arboretum_node_new_with_id("enum_value", "clang::EST_NoThrow", nullptr);
  data_model.clang_EST_BasicNoexcept_ = arboretum_node_new_with_id("enum_value", "clang::EST_BasicNoexcept", nullptr);
  data_model.clang_EST_DependentNoexcept_ = arboretum_node_new_with_id("enum_value", "clang::EST_DependentNoexcept", nullptr);
  data_model.clang_EST_NoexceptFalse_ = arboretum_node_new_with_id("enum_value", "clang::EST_NoexceptFalse", nullptr);
  data_model.clang_EST_NoexceptTrue_ = arboretum_node_new_with_id("enum_value", "clang::EST_NoexceptTrue", nullptr);
  data_model.clang_EST_Unevaluated_ = arboretum_node_new_with_id("enum_value", "clang::EST_Unevaluated", nullptr);
  data_model.clang_EST_Uninstantiated_ = arboretum_node_new_with_id("enum_value", "clang::EST_Uninstantiated", nullptr);
  data_model.clang_EST_Unparsed_ = arboretum_node_new_with_id("enum_value", "clang::EST_Unparsed", nullptr);

  // clang::ExprObjectKind
  data_model.clang_ExprObjectKind_ = arboretum_node_new_with_id("enum", "clang::ExprObjectKind", nullptr);
  data_model.clang_OK_Ordinary_ = arboretum_node_new_with_id("enum_value", "clang::OK_Ordinary", nullptr);
  data_model.clang_OK_BitField_ = arboretum_node_new_with_id("enum_value", "clang::OK_BitField", nullptr);
  data_model.clang_OK_VectorComponent_ = arboretum_node_new_with_id("enum_value", "clang::OK_VectorComponent", nullptr);
  data_model.clang_OK_ObjCProperty_ = arboretum_node_new_with_id("enum_value", "clang::OK_ObjCProperty", nullptr);
  data_model.clang_OK_ObjCSubscript_ = arboretum_node_new_with_id("enum_value", "clang::OK_ObjCSubscript", nullptr);
  data_model.clang_OK_MatrixComponent_ = arboretum_node_new_with_id("enum_value", "clang::OK_MatrixComponent", nullptr);

  // clang::ExprValueKind
  data_model.clang_ExprValueKind_ = arboretum_node_new_with_id("enum", "clang::ExprValueKind", nullptr);
  data_model.clang_VK_PRValue_ = arboretum_node_new_with_id("enum_value", "clang::VK_PRValue", nullptr);
  data_model.clang_VK_LValue_ = arboretum_node_new_with_id("enum_value", "clang::VK_LValue", nullptr);
  data_model.clang_VK_XValue_ = arboretum_node_new_with_id("enum_value", "clang::VK_XValue", nullptr);

  // clang::ExpressionTrait
  data_model.clang_ExpressionTrait_ = arboretum_node_new_with_id("enum", "clang::ExpressionTrait", nullptr);
  data_model.clang_ET_IsLValueExpr_ = arboretum_node_new_with_id("enum_value", "clang::ET_IsLValueExpr", nullptr);
  data_model.clang_ET_IsRValueExpr_ = arboretum_node_new_with_id("enum_value", "clang::ET_IsRValueExpr", nullptr);
  data_model.clang_ET_Last_ = arboretum_node_new_with_id("enum_value", "clang::ET_Last", nullptr);

  // clang::FunctionDecl::TemplatedKind
  data_model.clang_FunctionDecl_TemplatedKind_ = arboretum_node_new_with_id("enum", "clang::FunctionDecl::TemplatedKind", nullptr);
  data_model.clang_FunctionDecl_TK_NonTemplate_ = arboretum_node_new_with_id("enum_value", "clang::FunctionDecl::TK_NonTemplate", nullptr);
  data_model.clang_FunctionDecl_TK_FunctionTemplate_ = arboretum_node_new_with_id("enum_value", "clang::FunctionDecl::TK_FunctionTemplate", nullptr);
  data_model.clang_FunctionDecl_TK_MemberSpecialization_ = arboretum_node_new_with_id("enum_value", "clang::FunctionDecl::TK_MemberSpecialization", nullptr);
  data_model.clang_FunctionDecl_TK_FunctionTemplateSpecialization_ = arboretum_node_new_with_id("enum_value", "clang::FunctionDecl::TK_FunctionTemplateSpecialization", nullptr);
  data_model.clang_FunctionDecl_TK_DependentFunctionTemplateSpecialization_ = arboretum_node_new_with_id("enum_value", "clang::FunctionDecl::TK_DependentFunctionTemplateSpecialization", nullptr);
  data_model.clang_FunctionDecl_TK_DependentNonTemplate_ = arboretum_node_new_with_id("enum_value", "clang::FunctionDecl::TK_DependentNonTemplate", nullptr);

  // clang::IfStatementKind
  data_model.clang_IfStatementKind_ = arboretum_node_new_with_id("enum", "clang::IfStatementKind", nullptr);
  data_model.clang_IfStatementKind_Ordinary_ = arboretum_node_new_with_id("enum_value", "clang::IfStatementKind::Ordinary", nullptr);
  data_model.clang_IfStatementKind_Constexpr_ = arboretum_node_new_with_id("enum_value", "clang::IfStatementKind::Constexpr", nullptr);
  data_model.clang_IfStatementKind_ConstevalNonNegated_ = arboretum_node_new_with_id("enum_value", "clang::IfStatementKind::ConstevalNonNegated", nullptr);
  data_model.clang_IfStatementKind_ConstevalNegated_ = arboretum_node_new_with_id("enum_value", "clang::IfStatementKind::ConstevalNegated", nullptr);

  // clang::ImplicitParamDecl::ImplicitParamKind
  data_model.clang_ImplicitParamDecl_ImplicitParamKind_ = arboretum_node_new_with_id("enum", "clang::ImplicitParamDecl::ImplicitParamKind", nullptr);
  data_model.clang_ImplicitParamDecl_ObjCSelf_ = arboretum_node_new_with_id("enum_value", "clang::ImplicitParamDecl::ObjCSelf", nullptr);
  data_model.clang_ImplicitParamDecl_ObjCCmd_ = arboretum_node_new_with_id("enum_value", "clang::ImplicitParamDecl::ObjCCmd", nullptr);
  data_model.clang_ImplicitParamDecl_CXXThis_ = arboretum_node_new_with_id("enum_value", "clang::ImplicitParamDecl::CXXThis", nullptr);
  data_model.clang_ImplicitParamDecl_CXXVTT_ = arboretum_node_new_with_id("enum_value", "clang::ImplicitParamDecl::CXXVTT", nullptr);
  data_model.clang_ImplicitParamDecl_CapturedContext_ = arboretum_node_new_with_id("enum_value", "clang::ImplicitParamDecl::CapturedContext", nullptr);
  data_model.clang_ImplicitParamDecl_ThreadPrivateVar_ = arboretum_node_new_with_id("enum_value", "clang::ImplicitParamDecl::ThreadPrivateVar", nullptr);
  data_model.clang_ImplicitParamDecl_Other_ = arboretum_node_new_with_id("enum_value", "clang::ImplicitParamDecl::Other", nullptr);

  // clang::InClassInitStyle
  data_model.clang_InClassInitStyle_ = arboretum_node_new_with_id("enum", "clang::InClassInitStyle", nullptr);
  data_model.clang_ICIS_NoInit_ = arboretum_node_new_with_id("enum_value", "clang::ICIS_NoInit", nullptr);
  data_model.clang_ICIS_CopyInit_ = arboretum_node_new_with_id("enum_value", "clang::ICIS_CopyInit", nullptr);
  data_model.clang_ICIS_ListInit_ = arboretum_node_new_with_id("enum_value", "clang::ICIS_ListInit", nullptr);

  // clang::LambdaCaptureDefault
  data_model.clang_LambdaCaptureDefault_ = arboretum_node_new_with_id("enum", "clang::LambdaCaptureDefault", nullptr);
  data_model.clang_LCD_None_ = arboretum_node_new_with_id("enum_value", "clang::LCD_None", nullptr);
  data_model.clang_LCD_ByCopy_ = arboretum_node_new_with_id("enum_value", "clang::LCD_ByCopy", nullptr);
  data_model.clang_LCD_ByRef_ = arboretum_node_new_with_id("enum_value", "clang::LCD_ByRef", nullptr);

  // clang::LanguageLinkage
  data_model.clang_LanguageLinkage_ = arboretum_node_new_with_id("enum", "clang::LanguageLinkage", nullptr);
  data_model.clang_CLanguageLinkage_ = arboretum_node_new_with_id("enum_value", "clang::CLanguageLinkage", nullptr);
  data_model.clang_CXXLanguageLinkage_ = arboretum_node_new_with_id("enum_value", "clang::CXXLanguageLinkage", nullptr);
  data_model.clang_NoLanguageLinkage_ = arboretum_node_new_with_id("enum_value", "clang::NoLanguageLinkage", nullptr);

  // clang::Linkage
  data_model.clang_Linkage_ = arboretum_node_new_with_id("enum", "clang::Linkage", nullptr);
  data_model.clang_NoLinkage_ = arboretum_node_new_with_id("enum_value", "clang::NoLinkage", nullptr);
  data_model.clang_InternalLinkage_ = arboretum_node_new_with_id("enum_value", "clang::InternalLinkage", nullptr);
  data_model.clang_UniqueExternalLinkage_ = arboretum_node_new_with_id("enum_value", "clang::UniqueExternalLinkage", nullptr);
  data_model.clang_VisibleNoLinkage_ = arboretum_node_new_with_id("enum_value", "clang::VisibleNoLinkage", nullptr);
  data_model.clang_ModuleLinkage_ = arboretum_node_new_with_id("enum_value", "clang::ModuleLinkage", nullptr);
  data_model.clang_ExternalLinkage_ = arboretum_node_new_with_id("enum_value", "clang::ExternalLinkage", nullptr);

  // clang::LinkageSpecDecl::LanguageIDs
  data_model.clang_LinkageSpecDecl_LanguageIDs_ = arboretum_node_new_with_id("enum", "clang::LinkageSpecDecl::LanguageIDs", nullptr);
  data_model.clang_LinkageSpecDecl_lang_c_ = arboretum_node_new_with_id("enum_value", "clang::LinkageSpecDecl::lang_c", nullptr);
  data_model.clang_LinkageSpecDecl_lang_cxx_ = arboretum_node_new_with_id("enum_value", "clang::LinkageSpecDecl::lang_cxx", nullptr);

  // clang::MSVtorDispMode
  data_model.clang_MSVtorDispMode_ = arboretum_node_new_with_id("enum", "clang::MSVtorDispMode", nullptr);
  data_model.clang_MSVtorDispMode_Never_ = arboretum_node_new_with_id("enum_value", "clang::MSVtorDispMode::Never", nullptr);
  data_model.clang_MSVtorDispMode_ForVBaseOverride_ = arboretum_node_new_with_id("enum_value", "clang::MSVtorDispMode::ForVBaseOverride", nullptr);
  data_model.clang_MSVtorDispMode_ForVFTable_ = arboretum_node_new_with_id("enum_value", "clang::MSVtorDispMode::ForVFTable", nullptr);

  // clang::MultiVersionKind
  data_model.clang_MultiVersionKind_ = arboretum_node_new_with_id("enum", "clang::MultiVersionKind", nullptr);
  data_model.clang_MultiVersionKind_None_ = arboretum_node_new_with_id("enum_value", "clang::MultiVersionKind::None", nullptr);
  data_model.clang_MultiVersionKind_Target_ = arboretum_node_new_with_id("enum_value", "clang::MultiVersionKind::Target", nullptr);
  data_model.clang_MultiVersionKind_CPUSpecific_ = arboretum_node_new_with_id("enum_value", "clang::MultiVersionKind::CPUSpecific", nullptr);
  data_model.clang_MultiVersionKind_CPUDispatch_ = arboretum_node_new_with_id("enum_value", "clang::MultiVersionKind::CPUDispatch", nullptr);
  data_model.clang_MultiVersionKind_TargetClones_ = arboretum_node_new_with_id("enum_value", "clang::MultiVersionKind::TargetClones", nullptr);
  data_model.clang_MultiVersionKind_TargetVersion_ = arboretum_node_new_with_id("enum_value", "clang::MultiVersionKind::TargetVersion", nullptr);

  // clang::NonOdrUseReason
  data_model.clang_NonOdrUseReason_ = arboretum_node_new_with_id("enum", "clang::NonOdrUseReason", nullptr);
  data_model.clang_NOUR_None_ = arboretum_node_new_with_id("enum_value", "clang::NOUR_None", nullptr);
  data_model.clang_NOUR_Unevaluated_ = arboretum_node_new_with_id("enum_value", "clang::NOUR_Unevaluated", nullptr);
  data_model.clang_NOUR_Constant_ = arboretum_node_new_with_id("enum_value", "clang::NOUR_Constant", nullptr);
  data_model.clang_NOUR_Discarded_ = arboretum_node_new_with_id("enum_value", "clang::NOUR_Discarded", nullptr);

  // clang::ObjCStringFormatFamily
  data_model.clang_ObjCStringFormatFamily_ = arboretum_node_new_with_id("enum", "clang::ObjCStringFormatFamily", nullptr);
  data_model.clang_SFF_None_ = arboretum_node_new_with_id("enum_value", "clang::SFF_None", nullptr);
  data_model.clang_SFF_NSString_ = arboretum_node_new_with_id("enum_value", "clang::SFF_NSString", nullptr);
  data_model.clang_SFF_CFString_ = arboretum_node_new_with_id("enum_value", "clang::SFF_CFString", nullptr);

  // clang::OverloadedOperatorKind
  data_model.clang_OverloadedOperatorKind_ = arboretum_node_new_with_id("enum", "clang::OverloadedOperatorKind", nullptr);
  data_model.clang_OO_None_ = arboretum_node_new_with_id("enum_value", "clang::OO_None", nullptr);
  data_model.clang_OO_New_ = arboretum_node_new_with_id("enum_value", "clang::OO_New", nullptr);
  data_model.clang_OO_Delete_ = arboretum_node_new_with_id("enum_value", "clang::OO_Delete", nullptr);
  data_model.clang_OO_Array_New_ = arboretum_node_new_with_id("enum_value", "clang::OO_Array_New", nullptr);
  data_model.clang_OO_Array_Delete_ = arboretum_node_new_with_id("enum_value", "clang::OO_Array_Delete", nullptr);
  data_model.clang_OO_Plus_ = arboretum_node_new_with_id("enum_value", "clang::OO_Plus", nullptr);
  data_model.clang_OO_Minus_ = arboretum_node_new_with_id("enum_value", "clang::OO_Minus", nullptr);
  data_model.clang_OO_Star_ = arboretum_node_new_with_id("enum_value", "clang::OO_Star", nullptr);
  data_model.clang_OO_Slash_ = arboretum_node_new_with_id("enum_value", "clang::OO_Slash", nullptr);
  data_model.clang_OO_Percent_ = arboretum_node_new_with_id("enum_value", "clang::OO_Percent", nullptr);
  data_model.clang_OO_Caret_ = arboretum_node_new_with_id("enum_value", "clang::OO_Caret", nullptr);
  data_model.clang_OO_Amp_ = arboretum_node_new_with_id("enum_value", "clang::OO_Amp", nullptr);
  data_model.clang_OO_Pipe_ = arboretum_node_new_with_id("enum_value", "clang::OO_Pipe", nullptr);
  data_model.clang_OO_Tilde_ = arboretum_node_new_with_id("enum_value", "clang::OO_Tilde", nullptr);
  data_model.clang_OO_Exclaim_ = arboretum_node_new_with_id("enum_value", "clang::OO_Exclaim", nullptr);
  data_model.clang_OO_Equal_ = arboretum_node_new_with_id("enum_value", "clang::OO_Equal", nullptr);
  data_model.clang_OO_Less_ = arboretum_node_new_with_id("enum_value", "clang::OO_Less", nullptr);
  data_model.clang_OO_Greater_ = arboretum_node_new_with_id("enum_value", "clang::OO_Greater", nullptr);
  data_model.clang_OO_PlusEqual_ = arboretum_node_new_with_id("enum_value", "clang::OO_PlusEqual", nullptr);
  data_model.clang_OO_MinusEqual_ = arboretum_node_new_with_id("enum_value", "clang::OO_MinusEqual", nullptr);
  data_model.clang_OO_StarEqual_ = arboretum_node_new_with_id("enum_value", "clang::OO_StarEqual", nullptr);
  data_model.clang_OO_SlashEqual_ = arboretum_node_new_with_id("enum_value", "clang::OO_SlashEqual", nullptr);
  data_model.clang_OO_PercentEqual_ = arboretum_node_new_with_id("enum_value", "clang::OO_PercentEqual", nullptr);
  data_model.clang_OO_CaretEqual_ = arboretum_node_new_with_id("enum_value", "clang::OO_CaretEqual", nullptr);
  data_model.clang_OO_AmpEqual_ = arboretum_node_new_with_id("enum_value", "clang::OO_AmpEqual", nullptr);
  data_model.clang_OO_PipeEqual_ = arboretum_node_new_with_id("enum_value", "clang::OO_PipeEqual", nullptr);
  data_model.clang_OO_LessLess_ = arboretum_node_new_with_id("enum_value", "clang::OO_LessLess", nullptr);
  data_model.clang_OO_GreaterGreater_ = arboretum_node_new_with_id("enum_value", "clang::OO_GreaterGreater", nullptr);
  data_model.clang_OO_LessLessEqual_ = arboretum_node_new_with_id("enum_value", "clang::OO_LessLessEqual", nullptr);
  data_model.clang_OO_GreaterGreaterEqual_ = arboretum_node_new_with_id("enum_value", "clang::OO_GreaterGreaterEqual", nullptr);
  data_model.clang_OO_EqualEqual_ = arboretum_node_new_with_id("enum_value", "clang::OO_EqualEqual", nullptr);
  data_model.clang_OO_ExclaimEqual_ = arboretum_node_new_with_id("enum_value", "clang::OO_ExclaimEqual", nullptr);
  data_model.clang_OO_LessEqual_ = arboretum_node_new_with_id("enum_value", "clang::OO_LessEqual", nullptr);
  data_model.clang_OO_GreaterEqual_ = arboretum_node_new_with_id("enum_value", "clang::OO_GreaterEqual", nullptr);
  data_model.clang_OO_Spaceship_ = arboretum_node_new_with_id("enum_value", "clang::OO_Spaceship", nullptr);
  data_model.clang_OO_AmpAmp_ = arboretum_node_new_with_id("enum_value", "clang::OO_AmpAmp", nullptr);
  data_model.clang_OO_PipePipe_ = arboretum_node_new_with_id("enum_value", "clang::OO_PipePipe", nullptr);
  data_model.clang_OO_PlusPlus_ = arboretum_node_new_with_id("enum_value", "clang::OO_PlusPlus", nullptr);
  data_model.clang_OO_MinusMinus_ = arboretum_node_new_with_id("enum_value", "clang::OO_MinusMinus", nullptr);
  data_model.clang_OO_Comma_ = arboretum_node_new_with_id("enum_value", "clang::OO_Comma", nullptr);
  data_model.clang_OO_ArrowStar_ = arboretum_node_new_with_id("enum_value", "clang::OO_ArrowStar", nullptr);
  data_model.clang_OO_Arrow_ = arboretum_node_new_with_id("enum_value", "clang::OO_Arrow", nullptr);
  data_model.clang_OO_Call_ = arboretum_node_new_with_id("enum_value", "clang::OO_Call", nullptr);
  data_model.clang_OO_Subscript_ = arboretum_node_new_with_id("enum_value", "clang::OO_Subscript", nullptr);
  data_model.clang_OO_Conditional_ = arboretum_node_new_with_id("enum_value", "clang::OO_Conditional", nullptr);
  data_model.clang_OO_Coawait_ = arboretum_node_new_with_id("enum_value", "clang::OO_Coawait", nullptr);
  data_model.clang_NUM_OVERLOADED_OPERATORS_ = arboretum_node_new_with_id("enum_value", "clang::NUM_OVERLOADED_OPERATORS", nullptr);

  // clang::PragmaMSCommentKind
  data_model.clang_PragmaMSCommentKind_ = arboretum_node_new_with_id("enum", "clang::PragmaMSCommentKind", nullptr);
  data_model.clang_PCK_Unknown_ = arboretum_node_new_with_id("enum_value", "clang::PCK_Unknown", nullptr);
  data_model.clang_PCK_Linker_ = arboretum_node_new_with_id("enum_value", "clang::PCK_Linker", nullptr);
  data_model.clang_PCK_Lib_ = arboretum_node_new_with_id("enum_value", "clang::PCK_Lib", nullptr);
  data_model.clang_PCK_Compiler_ = arboretum_node_new_with_id("enum_value", "clang::PCK_Compiler", nullptr);
  data_model.clang_PCK_ExeStr_ = arboretum_node_new_with_id("enum_value", "clang::PCK_ExeStr", nullptr);
  data_model.clang_PCK_User_ = arboretum_node_new_with_id("enum_value", "clang::PCK_User", nullptr);

  // clang::PredefinedExpr::IdentKind
  data_model.clang_PredefinedExpr_IdentKind_ = arboretum_node_new_with_id("enum", "clang::PredefinedExpr::IdentKind", nullptr);
  data_model.clang_PredefinedExpr_Func_ = arboretum_node_new_with_id("enum_value", "clang::PredefinedExpr::Func", nullptr);
  data_model.clang_PredefinedExpr_Function_ = arboretum_node_new_with_id("enum_value", "clang::PredefinedExpr::Function", nullptr);
  data_model.clang_PredefinedExpr_LFunction_ = arboretum_node_new_with_id("enum_value", "clang::PredefinedExpr::LFunction", nullptr);
  data_model.clang_PredefinedExpr_FuncDName_ = arboretum_node_new_with_id("enum_value", "clang::PredefinedExpr::FuncDName", nullptr);
  data_model.clang_PredefinedExpr_FuncSig_ = arboretum_node_new_with_id("enum_value", "clang::PredefinedExpr::FuncSig", nullptr);
  data_model.clang_PredefinedExpr_LFuncSig_ = arboretum_node_new_with_id("enum_value", "clang::PredefinedExpr::LFuncSig", nullptr);
  data_model.clang_PredefinedExpr_PrettyFunction_ = arboretum_node_new_with_id("enum_value", "clang::PredefinedExpr::PrettyFunction", nullptr);
  data_model.clang_PredefinedExpr_PrettyFunctionNoVirtual_ = arboretum_node_new_with_id("enum_value", "clang::PredefinedExpr::PrettyFunctionNoVirtual", nullptr);

  // clang::Qualifiers::ObjCLifetime
  data_model.clang_Qualifiers_ObjCLifetime_ = arboretum_node_new_with_id("enum", "clang::Qualifiers::ObjCLifetime", nullptr);
  data_model.clang_Qualifiers_OCL_None_ = arboretum_node_new_with_id("enum_value", "clang::Qualifiers::OCL_None", nullptr);
  data_model.clang_Qualifiers_OCL_ExplicitNone_ = arboretum_node_new_with_id("enum_value", "clang::Qualifiers::OCL_ExplicitNone", nullptr);
  data_model.clang_Qualifiers_OCL_Strong_ = arboretum_node_new_with_id("enum_value", "clang::Qualifiers::OCL_Strong", nullptr);
  data_model.clang_Qualifiers_OCL_Weak_ = arboretum_node_new_with_id("enum_value", "clang::Qualifiers::OCL_Weak", nullptr);
  data_model.clang_Qualifiers_OCL_Autoreleasing_ = arboretum_node_new_with_id("enum_value", "clang::Qualifiers::OCL_Autoreleasing", nullptr);

  // clang::RecordDecl::ArgPassingKind
  data_model.clang_RecordDecl_ArgPassingKind_ = arboretum_node_new_with_id("enum", "clang::RecordDecl::ArgPassingKind", nullptr);
  data_model.clang_RecordDecl_APK_CanPassInRegs_ = arboretum_node_new_with_id("enum_value", "clang::RecordDecl::APK_CanPassInRegs", nullptr);
  data_model.clang_RecordDecl_APK_CannotPassInRegs_ = arboretum_node_new_with_id("enum_value", "clang::RecordDecl::APK_CannotPassInRegs", nullptr);
  data_model.clang_RecordDecl_APK_CanNeverPassInRegs_ = arboretum_node_new_with_id("enum_value", "clang::RecordDecl::APK_CanNeverPassInRegs", nullptr);

  // clang::RefQualifierKind
  data_model.clang_RefQualifierKind_ = arboretum_node_new_with_id("enum", "clang::RefQualifierKind", nullptr);
  data_model.clang_RQ_None_ = arboretum_node_new_with_id("enum_value", "clang::RQ_None", nullptr);
  data_model.clang_RQ_LValue_ = arboretum_node_new_with_id("enum_value", "clang::RQ_LValue", nullptr);
  data_model.clang_RQ_RValue_ = arboretum_node_new_with_id("enum_value", "clang::RQ_RValue", nullptr);

  // clang::SourceLocExpr::IdentKind
  data_model.clang_SourceLocExpr_IdentKind_ = arboretum_node_new_with_id("enum", "clang::SourceLocExpr::IdentKind", nullptr);
  data_model.clang_SourceLocExpr_Function_ = arboretum_node_new_with_id("enum_value", "clang::SourceLocExpr::Function", nullptr);
  data_model.clang_SourceLocExpr_FuncSig_ = arboretum_node_new_with_id("enum_value", "clang::SourceLocExpr::FuncSig", nullptr);
  data_model.clang_SourceLocExpr_File_ = arboretum_node_new_with_id("enum_value", "clang::SourceLocExpr::File", nullptr);
  data_model.clang_SourceLocExpr_FileName_ = arboretum_node_new_with_id("enum_value", "clang::SourceLocExpr::FileName", nullptr);
  data_model.clang_SourceLocExpr_Line_ = arboretum_node_new_with_id("enum_value", "clang::SourceLocExpr::Line", nullptr);
  data_model.clang_SourceLocExpr_Column_ = arboretum_node_new_with_id("enum_value", "clang::SourceLocExpr::Column", nullptr);
  data_model.clang_SourceLocExpr_SourceLocStruct_ = arboretum_node_new_with_id("enum_value", "clang::SourceLocExpr::SourceLocStruct", nullptr);

  // clang::Stmt::StmtClass
  data_model.clang_Stmt_StmtClass_ = arboretum_node_new_with_id("enum", "clang::Stmt::StmtClass", nullptr);
  data_model.clang_Stmt_NoStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::NoStmtClass", nullptr);
  data_model.clang_Stmt_GCCAsmStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::GCCAsmStmtClass", nullptr);
  data_model.clang_Stmt_MSAsmStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::MSAsmStmtClass", nullptr);
  data_model.clang_Stmt_firstAsmStmtConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::firstAsmStmtConstant", nullptr);
  data_model.clang_Stmt_lastAsmStmtConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::lastAsmStmtConstant", nullptr);
  data_model.clang_Stmt_BreakStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::BreakStmtClass", nullptr);
  data_model.clang_Stmt_CXXCatchStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXCatchStmtClass", nullptr);
  data_model.clang_Stmt_CXXForRangeStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXForRangeStmtClass", nullptr);
  data_model.clang_Stmt_CXXTryStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXTryStmtClass", nullptr);
  data_model.clang_Stmt_CapturedStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CapturedStmtClass", nullptr);
  data_model.clang_Stmt_CompoundStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CompoundStmtClass", nullptr);
  data_model.clang_Stmt_ContinueStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ContinueStmtClass", nullptr);
  data_model.clang_Stmt_CoreturnStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CoreturnStmtClass", nullptr);
  data_model.clang_Stmt_CoroutineBodyStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CoroutineBodyStmtClass", nullptr);
  data_model.clang_Stmt_DeclStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::DeclStmtClass", nullptr);
  data_model.clang_Stmt_DoStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::DoStmtClass", nullptr);
  data_model.clang_Stmt_ForStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ForStmtClass", nullptr);
  data_model.clang_Stmt_GotoStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::GotoStmtClass", nullptr);
  data_model.clang_Stmt_IfStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::IfStmtClass", nullptr);
  data_model.clang_Stmt_IndirectGotoStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::IndirectGotoStmtClass", nullptr);
  data_model.clang_Stmt_MSDependentExistsStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::MSDependentExistsStmtClass", nullptr);
  data_model.clang_Stmt_NullStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::NullStmtClass", nullptr);
  data_model.clang_Stmt_OMPCanonicalLoopClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPCanonicalLoopClass", nullptr);
  data_model.clang_Stmt_OMPAtomicDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPAtomicDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPBarrierDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPBarrierDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPCancelDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPCancelDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPCancellationPointDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPCancellationPointDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPCriticalDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPCriticalDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPDepobjDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPDepobjDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPDispatchDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPDispatchDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPErrorDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPErrorDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPFlushDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPFlushDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPInteropDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPInteropDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPDistributeDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPDistributeDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPDistributeParallelForDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPDistributeParallelForDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPDistributeParallelForSimdDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPDistributeParallelForSimdDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPDistributeSimdDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPDistributeSimdDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPForDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPForDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPForSimdDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPForSimdDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPGenericLoopDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPGenericLoopDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPMaskedTaskLoopDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPMaskedTaskLoopDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPMaskedTaskLoopSimdDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPMaskedTaskLoopSimdDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPMasterTaskLoopDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPMasterTaskLoopDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPMasterTaskLoopSimdDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPMasterTaskLoopSimdDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPParallelForDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPParallelForDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPParallelForSimdDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPParallelForSimdDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPParallelGenericLoopDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPParallelGenericLoopDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPParallelMaskedTaskLoopDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPParallelMaskedTaskLoopDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPParallelMaskedTaskLoopSimdDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPParallelMaskedTaskLoopSimdDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPParallelMasterTaskLoopDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPParallelMasterTaskLoopDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPParallelMasterTaskLoopSimdDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPParallelMasterTaskLoopSimdDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPSimdDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPSimdDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPTargetParallelForSimdDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTargetParallelForSimdDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPTargetParallelGenericLoopDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTargetParallelGenericLoopDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPTargetSimdDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTargetSimdDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPTargetTeamsDistributeDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTargetTeamsDistributeDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPTargetTeamsDistributeParallelForDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTargetTeamsDistributeParallelForDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPTargetTeamsDistributeParallelForSimdDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTargetTeamsDistributeParallelForSimdDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPTargetTeamsDistributeSimdDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTargetTeamsDistributeSimdDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPTargetTeamsGenericLoopDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTargetTeamsGenericLoopDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPTaskLoopDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTaskLoopDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPTaskLoopSimdDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTaskLoopSimdDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPTeamsDistributeDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTeamsDistributeDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPTeamsDistributeParallelForDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTeamsDistributeParallelForDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPTeamsDistributeParallelForSimdDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTeamsDistributeParallelForSimdDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPTeamsDistributeSimdDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTeamsDistributeSimdDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPTeamsGenericLoopDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTeamsGenericLoopDirectiveClass", nullptr);
  data_model.clang_Stmt_firstOMPLoopDirectiveConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::firstOMPLoopDirectiveConstant", nullptr);
  data_model.clang_Stmt_lastOMPLoopDirectiveConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::lastOMPLoopDirectiveConstant", nullptr);
  data_model.clang_Stmt_OMPTileDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTileDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPUnrollDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPUnrollDirectiveClass", nullptr);
  data_model.clang_Stmt_firstOMPLoopTransformationDirectiveConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::firstOMPLoopTransformationDirectiveConstant", nullptr);
  data_model.clang_Stmt_lastOMPLoopTransformationDirectiveConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::lastOMPLoopTransformationDirectiveConstant", nullptr);
  data_model.clang_Stmt_firstOMPLoopBasedDirectiveConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::firstOMPLoopBasedDirectiveConstant", nullptr);
  data_model.clang_Stmt_lastOMPLoopBasedDirectiveConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::lastOMPLoopBasedDirectiveConstant", nullptr);
  data_model.clang_Stmt_OMPMaskedDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPMaskedDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPMasterDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPMasterDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPMetaDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPMetaDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPOrderedDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPOrderedDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPParallelDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPParallelDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPParallelMaskedDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPParallelMaskedDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPParallelMasterDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPParallelMasterDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPParallelSectionsDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPParallelSectionsDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPScanDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPScanDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPSectionDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPSectionDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPSectionsDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPSectionsDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPSingleDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPSingleDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPTargetDataDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTargetDataDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPTargetDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTargetDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPTargetEnterDataDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTargetEnterDataDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPTargetExitDataDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTargetExitDataDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPTargetParallelDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTargetParallelDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPTargetParallelForDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTargetParallelForDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPTargetTeamsDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTargetTeamsDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPTargetUpdateDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTargetUpdateDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPTaskDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTaskDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPTaskgroupDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTaskgroupDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPTaskwaitDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTaskwaitDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPTaskyieldDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTaskyieldDirectiveClass", nullptr);
  data_model.clang_Stmt_OMPTeamsDirectiveClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPTeamsDirectiveClass", nullptr);
  data_model.clang_Stmt_firstOMPExecutableDirectiveConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::firstOMPExecutableDirectiveConstant", nullptr);
  data_model.clang_Stmt_lastOMPExecutableDirectiveConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::lastOMPExecutableDirectiveConstant", nullptr);
  data_model.clang_Stmt_ObjCAtCatchStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ObjCAtCatchStmtClass", nullptr);
  data_model.clang_Stmt_ObjCAtFinallyStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ObjCAtFinallyStmtClass", nullptr);
  data_model.clang_Stmt_ObjCAtSynchronizedStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ObjCAtSynchronizedStmtClass", nullptr);
  data_model.clang_Stmt_ObjCAtThrowStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ObjCAtThrowStmtClass", nullptr);
  data_model.clang_Stmt_ObjCAtTryStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ObjCAtTryStmtClass", nullptr);
  data_model.clang_Stmt_ObjCAutoreleasePoolStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ObjCAutoreleasePoolStmtClass", nullptr);
  data_model.clang_Stmt_ObjCForCollectionStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ObjCForCollectionStmtClass", nullptr);
  data_model.clang_Stmt_ReturnStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ReturnStmtClass", nullptr);
  data_model.clang_Stmt_SEHExceptStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::SEHExceptStmtClass", nullptr);
  data_model.clang_Stmt_SEHFinallyStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::SEHFinallyStmtClass", nullptr);
  data_model.clang_Stmt_SEHLeaveStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::SEHLeaveStmtClass", nullptr);
  data_model.clang_Stmt_SEHTryStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::SEHTryStmtClass", nullptr);
  data_model.clang_Stmt_CaseStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CaseStmtClass", nullptr);
  data_model.clang_Stmt_DefaultStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::DefaultStmtClass", nullptr);
  data_model.clang_Stmt_firstSwitchCaseConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::firstSwitchCaseConstant", nullptr);
  data_model.clang_Stmt_lastSwitchCaseConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::lastSwitchCaseConstant", nullptr);
  data_model.clang_Stmt_SwitchStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::SwitchStmtClass", nullptr);
  data_model.clang_Stmt_AttributedStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::AttributedStmtClass", nullptr);
  data_model.clang_Stmt_BinaryConditionalOperatorClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::BinaryConditionalOperatorClass", nullptr);
  data_model.clang_Stmt_ConditionalOperatorClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ConditionalOperatorClass", nullptr);
  data_model.clang_Stmt_firstAbstractConditionalOperatorConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::firstAbstractConditionalOperatorConstant", nullptr);
  data_model.clang_Stmt_lastAbstractConditionalOperatorConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::lastAbstractConditionalOperatorConstant", nullptr);
  data_model.clang_Stmt_AddrLabelExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::AddrLabelExprClass", nullptr);
  data_model.clang_Stmt_ArrayInitIndexExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ArrayInitIndexExprClass", nullptr);
  data_model.clang_Stmt_ArrayInitLoopExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ArrayInitLoopExprClass", nullptr);
  data_model.clang_Stmt_ArraySubscriptExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ArraySubscriptExprClass", nullptr);
  data_model.clang_Stmt_ArrayTypeTraitExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ArrayTypeTraitExprClass", nullptr);
  data_model.clang_Stmt_AsTypeExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::AsTypeExprClass", nullptr);
  data_model.clang_Stmt_AtomicExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::AtomicExprClass", nullptr);
  data_model.clang_Stmt_BinaryOperatorClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::BinaryOperatorClass", nullptr);
  data_model.clang_Stmt_CompoundAssignOperatorClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CompoundAssignOperatorClass", nullptr);
  data_model.clang_Stmt_firstBinaryOperatorConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::firstBinaryOperatorConstant", nullptr);
  data_model.clang_Stmt_lastBinaryOperatorConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::lastBinaryOperatorConstant", nullptr);
  data_model.clang_Stmt_BlockExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::BlockExprClass", nullptr);
  data_model.clang_Stmt_CXXBindTemporaryExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXBindTemporaryExprClass", nullptr);
  data_model.clang_Stmt_CXXBoolLiteralExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXBoolLiteralExprClass", nullptr);
  data_model.clang_Stmt_CXXConstructExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXConstructExprClass", nullptr);
  data_model.clang_Stmt_CXXTemporaryObjectExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXTemporaryObjectExprClass", nullptr);
  data_model.clang_Stmt_firstCXXConstructExprConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::firstCXXConstructExprConstant", nullptr);
  data_model.clang_Stmt_lastCXXConstructExprConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::lastCXXConstructExprConstant", nullptr);
  data_model.clang_Stmt_CXXDefaultArgExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXDefaultArgExprClass", nullptr);
  data_model.clang_Stmt_CXXDefaultInitExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXDefaultInitExprClass", nullptr);
  data_model.clang_Stmt_CXXDeleteExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXDeleteExprClass", nullptr);
  data_model.clang_Stmt_CXXDependentScopeMemberExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXDependentScopeMemberExprClass", nullptr);
  data_model.clang_Stmt_CXXFoldExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXFoldExprClass", nullptr);
  data_model.clang_Stmt_CXXInheritedCtorInitExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXInheritedCtorInitExprClass", nullptr);
  data_model.clang_Stmt_CXXNewExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXNewExprClass", nullptr);
  data_model.clang_Stmt_CXXNoexceptExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXNoexceptExprClass", nullptr);
  data_model.clang_Stmt_CXXNullPtrLiteralExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXNullPtrLiteralExprClass", nullptr);
  data_model.clang_Stmt_CXXParenListInitExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXParenListInitExprClass", nullptr);
  data_model.clang_Stmt_CXXPseudoDestructorExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXPseudoDestructorExprClass", nullptr);
  data_model.clang_Stmt_CXXRewrittenBinaryOperatorClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXRewrittenBinaryOperatorClass", nullptr);
  data_model.clang_Stmt_CXXScalarValueInitExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXScalarValueInitExprClass", nullptr);
  data_model.clang_Stmt_CXXStdInitializerListExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXStdInitializerListExprClass", nullptr);
  data_model.clang_Stmt_CXXThisExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXThisExprClass", nullptr);
  data_model.clang_Stmt_CXXThrowExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXThrowExprClass", nullptr);
  data_model.clang_Stmt_CXXTypeidExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXTypeidExprClass", nullptr);
  data_model.clang_Stmt_CXXUnresolvedConstructExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXUnresolvedConstructExprClass", nullptr);
  data_model.clang_Stmt_CXXUuidofExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXUuidofExprClass", nullptr);
  data_model.clang_Stmt_CallExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CallExprClass", nullptr);
  data_model.clang_Stmt_CUDAKernelCallExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CUDAKernelCallExprClass", nullptr);
  data_model.clang_Stmt_CXXMemberCallExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXMemberCallExprClass", nullptr);
  data_model.clang_Stmt_CXXOperatorCallExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXOperatorCallExprClass", nullptr);
  data_model.clang_Stmt_UserDefinedLiteralClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::UserDefinedLiteralClass", nullptr);
  data_model.clang_Stmt_firstCallExprConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::firstCallExprConstant", nullptr);
  data_model.clang_Stmt_lastCallExprConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::lastCallExprConstant", nullptr);
  data_model.clang_Stmt_BuiltinBitCastExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::BuiltinBitCastExprClass", nullptr);
  data_model.clang_Stmt_CStyleCastExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CStyleCastExprClass", nullptr);
  data_model.clang_Stmt_CXXFunctionalCastExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXFunctionalCastExprClass", nullptr);
  data_model.clang_Stmt_CXXAddrspaceCastExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXAddrspaceCastExprClass", nullptr);
  data_model.clang_Stmt_CXXConstCastExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXConstCastExprClass", nullptr);
  data_model.clang_Stmt_CXXDynamicCastExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXDynamicCastExprClass", nullptr);
  data_model.clang_Stmt_CXXReinterpretCastExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXReinterpretCastExprClass", nullptr);
  data_model.clang_Stmt_CXXStaticCastExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CXXStaticCastExprClass", nullptr);
  data_model.clang_Stmt_firstCXXNamedCastExprConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::firstCXXNamedCastExprConstant", nullptr);
  data_model.clang_Stmt_lastCXXNamedCastExprConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::lastCXXNamedCastExprConstant", nullptr);
  data_model.clang_Stmt_ObjCBridgedCastExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ObjCBridgedCastExprClass", nullptr);
  data_model.clang_Stmt_firstExplicitCastExprConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::firstExplicitCastExprConstant", nullptr);
  data_model.clang_Stmt_lastExplicitCastExprConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::lastExplicitCastExprConstant", nullptr);
  data_model.clang_Stmt_ImplicitCastExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ImplicitCastExprClass", nullptr);
  data_model.clang_Stmt_firstCastExprConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::firstCastExprConstant", nullptr);
  data_model.clang_Stmt_lastCastExprConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::lastCastExprConstant", nullptr);
  data_model.clang_Stmt_CharacterLiteralClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CharacterLiteralClass", nullptr);
  data_model.clang_Stmt_ChooseExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ChooseExprClass", nullptr);
  data_model.clang_Stmt_CompoundLiteralExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CompoundLiteralExprClass", nullptr);
  data_model.clang_Stmt_ConceptSpecializationExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ConceptSpecializationExprClass", nullptr);
  data_model.clang_Stmt_ConvertVectorExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ConvertVectorExprClass", nullptr);
  data_model.clang_Stmt_CoawaitExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CoawaitExprClass", nullptr);
  data_model.clang_Stmt_CoyieldExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::CoyieldExprClass", nullptr);
  data_model.clang_Stmt_firstCoroutineSuspendExprConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::firstCoroutineSuspendExprConstant", nullptr);
  data_model.clang_Stmt_lastCoroutineSuspendExprConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::lastCoroutineSuspendExprConstant", nullptr);
  data_model.clang_Stmt_DeclRefExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::DeclRefExprClass", nullptr);
  data_model.clang_Stmt_DependentCoawaitExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::DependentCoawaitExprClass", nullptr);
  data_model.clang_Stmt_DependentScopeDeclRefExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::DependentScopeDeclRefExprClass", nullptr);
  data_model.clang_Stmt_DesignatedInitExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::DesignatedInitExprClass", nullptr);
  data_model.clang_Stmt_DesignatedInitUpdateExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::DesignatedInitUpdateExprClass", nullptr);
  data_model.clang_Stmt_ExpressionTraitExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ExpressionTraitExprClass", nullptr);
  data_model.clang_Stmt_ExtVectorElementExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ExtVectorElementExprClass", nullptr);
  data_model.clang_Stmt_FixedPointLiteralClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::FixedPointLiteralClass", nullptr);
  data_model.clang_Stmt_FloatingLiteralClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::FloatingLiteralClass", nullptr);
  data_model.clang_Stmt_ConstantExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ConstantExprClass", nullptr);
  data_model.clang_Stmt_ExprWithCleanupsClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ExprWithCleanupsClass", nullptr);
  data_model.clang_Stmt_firstFullExprConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::firstFullExprConstant", nullptr);
  data_model.clang_Stmt_lastFullExprConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::lastFullExprConstant", nullptr);
  data_model.clang_Stmt_FunctionParmPackExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::FunctionParmPackExprClass", nullptr);
  data_model.clang_Stmt_GNUNullExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::GNUNullExprClass", nullptr);
  data_model.clang_Stmt_GenericSelectionExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::GenericSelectionExprClass", nullptr);
  data_model.clang_Stmt_ImaginaryLiteralClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ImaginaryLiteralClass", nullptr);
  data_model.clang_Stmt_ImplicitValueInitExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ImplicitValueInitExprClass", nullptr);
  data_model.clang_Stmt_InitListExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::InitListExprClass", nullptr);
  data_model.clang_Stmt_IntegerLiteralClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::IntegerLiteralClass", nullptr);
  data_model.clang_Stmt_LambdaExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::LambdaExprClass", nullptr);
  data_model.clang_Stmt_MSPropertyRefExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::MSPropertyRefExprClass", nullptr);
  data_model.clang_Stmt_MSPropertySubscriptExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::MSPropertySubscriptExprClass", nullptr);
  data_model.clang_Stmt_MaterializeTemporaryExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::MaterializeTemporaryExprClass", nullptr);
  data_model.clang_Stmt_MatrixSubscriptExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::MatrixSubscriptExprClass", nullptr);
  data_model.clang_Stmt_MemberExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::MemberExprClass", nullptr);
  data_model.clang_Stmt_NoInitExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::NoInitExprClass", nullptr);
  data_model.clang_Stmt_OMPArraySectionExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPArraySectionExprClass", nullptr);
  data_model.clang_Stmt_OMPArrayShapingExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPArrayShapingExprClass", nullptr);
  data_model.clang_Stmt_OMPIteratorExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OMPIteratorExprClass", nullptr);
  data_model.clang_Stmt_ObjCArrayLiteralClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ObjCArrayLiteralClass", nullptr);
  data_model.clang_Stmt_ObjCAvailabilityCheckExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ObjCAvailabilityCheckExprClass", nullptr);
  data_model.clang_Stmt_ObjCBoolLiteralExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ObjCBoolLiteralExprClass", nullptr);
  data_model.clang_Stmt_ObjCBoxedExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ObjCBoxedExprClass", nullptr);
  data_model.clang_Stmt_ObjCDictionaryLiteralClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ObjCDictionaryLiteralClass", nullptr);
  data_model.clang_Stmt_ObjCEncodeExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ObjCEncodeExprClass", nullptr);
  data_model.clang_Stmt_ObjCIndirectCopyRestoreExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ObjCIndirectCopyRestoreExprClass", nullptr);
  data_model.clang_Stmt_ObjCIsaExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ObjCIsaExprClass", nullptr);
  data_model.clang_Stmt_ObjCIvarRefExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ObjCIvarRefExprClass", nullptr);
  data_model.clang_Stmt_ObjCMessageExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ObjCMessageExprClass", nullptr);
  data_model.clang_Stmt_ObjCPropertyRefExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ObjCPropertyRefExprClass", nullptr);
  data_model.clang_Stmt_ObjCProtocolExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ObjCProtocolExprClass", nullptr);
  data_model.clang_Stmt_ObjCSelectorExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ObjCSelectorExprClass", nullptr);
  data_model.clang_Stmt_ObjCStringLiteralClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ObjCStringLiteralClass", nullptr);
  data_model.clang_Stmt_ObjCSubscriptRefExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ObjCSubscriptRefExprClass", nullptr);
  data_model.clang_Stmt_OffsetOfExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OffsetOfExprClass", nullptr);
  data_model.clang_Stmt_OpaqueValueExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::OpaqueValueExprClass", nullptr);
  data_model.clang_Stmt_UnresolvedLookupExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::UnresolvedLookupExprClass", nullptr);
  data_model.clang_Stmt_UnresolvedMemberExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::UnresolvedMemberExprClass", nullptr);
  data_model.clang_Stmt_firstOverloadExprConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::firstOverloadExprConstant", nullptr);
  data_model.clang_Stmt_lastOverloadExprConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::lastOverloadExprConstant", nullptr);
  data_model.clang_Stmt_PackExpansionExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::PackExpansionExprClass", nullptr);
  data_model.clang_Stmt_ParenExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ParenExprClass", nullptr);
  data_model.clang_Stmt_ParenListExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ParenListExprClass", nullptr);
  data_model.clang_Stmt_PredefinedExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::PredefinedExprClass", nullptr);
  data_model.clang_Stmt_PseudoObjectExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::PseudoObjectExprClass", nullptr);
  data_model.clang_Stmt_RecoveryExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::RecoveryExprClass", nullptr);
  data_model.clang_Stmt_RequiresExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::RequiresExprClass", nullptr);
  data_model.clang_Stmt_SYCLUniqueStableNameExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::SYCLUniqueStableNameExprClass", nullptr);
  data_model.clang_Stmt_ShuffleVectorExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::ShuffleVectorExprClass", nullptr);
  data_model.clang_Stmt_SizeOfPackExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::SizeOfPackExprClass", nullptr);
  data_model.clang_Stmt_SourceLocExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::SourceLocExprClass", nullptr);
  data_model.clang_Stmt_StmtExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::StmtExprClass", nullptr);
  data_model.clang_Stmt_StringLiteralClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::StringLiteralClass", nullptr);
  data_model.clang_Stmt_SubstNonTypeTemplateParmExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::SubstNonTypeTemplateParmExprClass", nullptr);
  data_model.clang_Stmt_SubstNonTypeTemplateParmPackExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::SubstNonTypeTemplateParmPackExprClass", nullptr);
  data_model.clang_Stmt_TypeTraitExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::TypeTraitExprClass", nullptr);
  data_model.clang_Stmt_TypoExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::TypoExprClass", nullptr);
  data_model.clang_Stmt_UnaryExprOrTypeTraitExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::UnaryExprOrTypeTraitExprClass", nullptr);
  data_model.clang_Stmt_UnaryOperatorClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::UnaryOperatorClass", nullptr);
  data_model.clang_Stmt_VAArgExprClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::VAArgExprClass", nullptr);
  data_model.clang_Stmt_firstExprConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::firstExprConstant", nullptr);
  data_model.clang_Stmt_lastExprConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::lastExprConstant", nullptr);
  data_model.clang_Stmt_LabelStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::LabelStmtClass", nullptr);
  data_model.clang_Stmt_firstValueStmtConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::firstValueStmtConstant", nullptr);
  data_model.clang_Stmt_lastValueStmtConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::lastValueStmtConstant", nullptr);
  data_model.clang_Stmt_WhileStmtClass_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::WhileStmtClass", nullptr);
  data_model.clang_Stmt_firstStmtConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::firstStmtConstant", nullptr);
  data_model.clang_Stmt_lastStmtConstant_ = arboretum_node_new_with_id("enum_value", "clang::Stmt::lastStmtConstant", nullptr);

  // clang::StorageClass
  data_model.clang_StorageClass_ = arboretum_node_new_with_id("enum", "clang::StorageClass", nullptr);
  data_model.clang_SC_None_ = arboretum_node_new_with_id("enum_value", "clang::SC_None", nullptr);
  data_model.clang_SC_Extern_ = arboretum_node_new_with_id("enum_value", "clang::SC_Extern", nullptr);
  data_model.clang_SC_Static_ = arboretum_node_new_with_id("enum_value", "clang::SC_Static", nullptr);
  data_model.clang_SC_PrivateExtern_ = arboretum_node_new_with_id("enum_value", "clang::SC_PrivateExtern", nullptr);
  data_model.clang_SC_Auto_ = arboretum_node_new_with_id("enum_value", "clang::SC_Auto", nullptr);
  data_model.clang_SC_Register_ = arboretum_node_new_with_id("enum_value", "clang::SC_Register", nullptr);

  // clang::StorageDuration
  data_model.clang_StorageDuration_ = arboretum_node_new_with_id("enum", "clang::StorageDuration", nullptr);
  data_model.clang_SD_FullExpression_ = arboretum_node_new_with_id("enum_value", "clang::SD_FullExpression", nullptr);
  data_model.clang_SD_Automatic_ = arboretum_node_new_with_id("enum_value", "clang::SD_Automatic", nullptr);
  data_model.clang_SD_Thread_ = arboretum_node_new_with_id("enum_value", "clang::SD_Thread", nullptr);
  data_model.clang_SD_Static_ = arboretum_node_new_with_id("enum_value", "clang::SD_Static", nullptr);
  data_model.clang_SD_Dynamic_ = arboretum_node_new_with_id("enum_value", "clang::SD_Dynamic", nullptr);

  // clang::StringLiteral::StringKind
  data_model.clang_StringLiteral_StringKind_ = arboretum_node_new_with_id("enum", "clang::StringLiteral::StringKind", nullptr);
  data_model.clang_StringLiteral_Ordinary_ = arboretum_node_new_with_id("enum_value", "clang::StringLiteral::Ordinary", nullptr);
  data_model.clang_StringLiteral_Wide_ = arboretum_node_new_with_id("enum_value", "clang::StringLiteral::Wide", nullptr);
  data_model.clang_StringLiteral_UTF8_ = arboretum_node_new_with_id("enum_value", "clang::StringLiteral::UTF8", nullptr);
  data_model.clang_StringLiteral_UTF16_ = arboretum_node_new_with_id("enum_value", "clang::StringLiteral::UTF16", nullptr);
  data_model.clang_StringLiteral_UTF32_ = arboretum_node_new_with_id("enum_value", "clang::StringLiteral::UTF32", nullptr);
  data_model.clang_StringLiteral_Unevaluated_ = arboretum_node_new_with_id("enum_value", "clang::StringLiteral::Unevaluated", nullptr);

  // clang::TagTypeKind
  data_model.clang_TagTypeKind_ = arboretum_node_new_with_id("enum", "clang::TagTypeKind", nullptr);
  data_model.clang_TTK_Struct_ = arboretum_node_new_with_id("enum_value", "clang::TTK_Struct", nullptr);
  data_model.clang_TTK_Interface_ = arboretum_node_new_with_id("enum_value", "clang::TTK_Interface", nullptr);
  data_model.clang_TTK_Union_ = arboretum_node_new_with_id("enum_value", "clang::TTK_Union", nullptr);
  data_model.clang_TTK_Class_ = arboretum_node_new_with_id("enum_value", "clang::TTK_Class", nullptr);
  data_model.clang_TTK_Enum_ = arboretum_node_new_with_id("enum_value", "clang::TTK_Enum", nullptr);

  // clang::TemplateSpecializationKind
  data_model.clang_TemplateSpecializationKind_ = arboretum_node_new_with_id("enum", "clang::TemplateSpecializationKind", nullptr);
  data_model.clang_TSK_Undeclared_ = arboretum_node_new_with_id("enum_value", "clang::TSK_Undeclared", nullptr);
  data_model.clang_TSK_ImplicitInstantiation_ = arboretum_node_new_with_id("enum_value", "clang::TSK_ImplicitInstantiation", nullptr);
  data_model.clang_TSK_ExplicitSpecialization_ = arboretum_node_new_with_id("enum_value", "clang::TSK_ExplicitSpecialization", nullptr);
  data_model.clang_TSK_ExplicitInstantiationDeclaration_ = arboretum_node_new_with_id("enum_value", "clang::TSK_ExplicitInstantiationDeclaration", nullptr);
  data_model.clang_TSK_ExplicitInstantiationDefinition_ = arboretum_node_new_with_id("enum_value", "clang::TSK_ExplicitInstantiationDefinition", nullptr);

  // clang::ThreadStorageClassSpecifier
  data_model.clang_ThreadStorageClassSpecifier_ = arboretum_node_new_with_id("enum", "clang::ThreadStorageClassSpecifier", nullptr);
  data_model.clang_TSCS_unspecified_ = arboretum_node_new_with_id("enum_value", "clang::TSCS_unspecified", nullptr);
  data_model.clang_TSCS___thread_ = arboretum_node_new_with_id("enum_value", "clang::TSCS___thread", nullptr);
  data_model.clang_TSCS_thread_local_ = arboretum_node_new_with_id("enum_value", "clang::TSCS_thread_local", nullptr);
  data_model.clang_TSCS__Thread_local_ = arboretum_node_new_with_id("enum_value", "clang::TSCS__Thread_local", nullptr);

  // clang::Type::ScalarTypeKind
  data_model.clang_Type_ScalarTypeKind_ = arboretum_node_new_with_id("enum", "clang::Type::ScalarTypeKind", nullptr);
  data_model.clang_Type_STK_CPointer_ = arboretum_node_new_with_id("enum_value", "clang::Type::STK_CPointer", nullptr);
  data_model.clang_Type_STK_BlockPointer_ = arboretum_node_new_with_id("enum_value", "clang::Type::STK_BlockPointer", nullptr);
  data_model.clang_Type_STK_ObjCObjectPointer_ = arboretum_node_new_with_id("enum_value", "clang::Type::STK_ObjCObjectPointer", nullptr);
  data_model.clang_Type_STK_MemberPointer_ = arboretum_node_new_with_id("enum_value", "clang::Type::STK_MemberPointer", nullptr);
  data_model.clang_Type_STK_Bool_ = arboretum_node_new_with_id("enum_value", "clang::Type::STK_Bool", nullptr);
  data_model.clang_Type_STK_Integral_ = arboretum_node_new_with_id("enum_value", "clang::Type::STK_Integral", nullptr);
  data_model.clang_Type_STK_Floating_ = arboretum_node_new_with_id("enum_value", "clang::Type::STK_Floating", nullptr);
  data_model.clang_Type_STK_IntegralComplex_ = arboretum_node_new_with_id("enum_value", "clang::Type::STK_IntegralComplex", nullptr);
  data_model.clang_Type_STK_FloatingComplex_ = arboretum_node_new_with_id("enum_value", "clang::Type::STK_FloatingComplex", nullptr);
  data_model.clang_Type_STK_FixedPoint_ = arboretum_node_new_with_id("enum_value", "clang::Type::STK_FixedPoint", nullptr);

  // clang::Type::TypeClass
  data_model.clang_Type_TypeClass_ = arboretum_node_new_with_id("enum", "clang::Type::TypeClass", nullptr);
  data_model.clang_Type_Adjusted_ = arboretum_node_new_with_id("enum_value", "clang::Type::Adjusted", nullptr);
  data_model.clang_Type_Decayed_ = arboretum_node_new_with_id("enum_value", "clang::Type::Decayed", nullptr);
  data_model.clang_Type_ConstantArray_ = arboretum_node_new_with_id("enum_value", "clang::Type::ConstantArray", nullptr);
  data_model.clang_Type_DependentSizedArray_ = arboretum_node_new_with_id("enum_value", "clang::Type::DependentSizedArray", nullptr);
  data_model.clang_Type_IncompleteArray_ = arboretum_node_new_with_id("enum_value", "clang::Type::IncompleteArray", nullptr);
  data_model.clang_Type_VariableArray_ = arboretum_node_new_with_id("enum_value", "clang::Type::VariableArray", nullptr);
  data_model.clang_Type_Atomic_ = arboretum_node_new_with_id("enum_value", "clang::Type::Atomic", nullptr);
  data_model.clang_Type_Attributed_ = arboretum_node_new_with_id("enum_value", "clang::Type::Attributed", nullptr);
  data_model.clang_Type_BTFTagAttributed_ = arboretum_node_new_with_id("enum_value", "clang::Type::BTFTagAttributed", nullptr);
  data_model.clang_Type_BitInt_ = arboretum_node_new_with_id("enum_value", "clang::Type::BitInt", nullptr);
  data_model.clang_Type_BlockPointer_ = arboretum_node_new_with_id("enum_value", "clang::Type::BlockPointer", nullptr);
  data_model.clang_Type_Builtin_ = arboretum_node_new_with_id("enum_value", "clang::Type::Builtin", nullptr);
  data_model.clang_Type_Complex_ = arboretum_node_new_with_id("enum_value", "clang::Type::Complex", nullptr);
  data_model.clang_Type_Decltype_ = arboretum_node_new_with_id("enum_value", "clang::Type::Decltype", nullptr);
  data_model.clang_Type_Auto_ = arboretum_node_new_with_id("enum_value", "clang::Type::Auto", nullptr);
  data_model.clang_Type_DeducedTemplateSpecialization_ = arboretum_node_new_with_id("enum_value", "clang::Type::DeducedTemplateSpecialization", nullptr);
  data_model.clang_Type_DependentAddressSpace_ = arboretum_node_new_with_id("enum_value", "clang::Type::DependentAddressSpace", nullptr);
  data_model.clang_Type_DependentBitInt_ = arboretum_node_new_with_id("enum_value", "clang::Type::DependentBitInt", nullptr);
  data_model.clang_Type_DependentName_ = arboretum_node_new_with_id("enum_value", "clang::Type::DependentName", nullptr);
  data_model.clang_Type_DependentSizedExtVector_ = arboretum_node_new_with_id("enum_value", "clang::Type::DependentSizedExtVector", nullptr);
  data_model.clang_Type_DependentTemplateSpecialization_ = arboretum_node_new_with_id("enum_value", "clang::Type::DependentTemplateSpecialization", nullptr);
  data_model.clang_Type_DependentVector_ = arboretum_node_new_with_id("enum_value", "clang::Type::DependentVector", nullptr);
  data_model.clang_Type_Elaborated_ = arboretum_node_new_with_id("enum_value", "clang::Type::Elaborated", nullptr);
  data_model.clang_Type_FunctionNoProto_ = arboretum_node_new_with_id("enum_value", "clang::Type::FunctionNoProto", nullptr);
  data_model.clang_Type_FunctionProto_ = arboretum_node_new_with_id("enum_value", "clang::Type::FunctionProto", nullptr);
  data_model.clang_Type_InjectedClassName_ = arboretum_node_new_with_id("enum_value", "clang::Type::InjectedClassName", nullptr);
  data_model.clang_Type_MacroQualified_ = arboretum_node_new_with_id("enum_value", "clang::Type::MacroQualified", nullptr);
  data_model.clang_Type_ConstantMatrix_ = arboretum_node_new_with_id("enum_value", "clang::Type::ConstantMatrix", nullptr);
  data_model.clang_Type_DependentSizedMatrix_ = arboretum_node_new_with_id("enum_value", "clang::Type::DependentSizedMatrix", nullptr);
  data_model.clang_Type_MemberPointer_ = arboretum_node_new_with_id("enum_value", "clang::Type::MemberPointer", nullptr);
  data_model.clang_Type_ObjCObjectPointer_ = arboretum_node_new_with_id("enum_value", "clang::Type::ObjCObjectPointer", nullptr);
  data_model.clang_Type_ObjCObject_ = arboretum_node_new_with_id("enum_value", "clang::Type::ObjCObject", nullptr);
  data_model.clang_Type_ObjCInterface_ = arboretum_node_new_with_id("enum_value", "clang::Type::ObjCInterface", nullptr);
  data_model.clang_Type_ObjCTypeParam_ = arboretum_node_new_with_id("enum_value", "clang::Type::ObjCTypeParam", nullptr);
  data_model.clang_Type_PackExpansion_ = arboretum_node_new_with_id("enum_value", "clang::Type::PackExpansion", nullptr);
  data_model.clang_Type_Paren_ = arboretum_node_new_with_id("enum_value", "clang::Type::Paren", nullptr);
  data_model.clang_Type_Pipe_ = arboretum_node_new_with_id("enum_value", "clang::Type::Pipe", nullptr);
  data_model.clang_Type_Pointer_ = arboretum_node_new_with_id("enum_value", "clang::Type::Pointer", nullptr);
  data_model.clang_Type_LValueReference_ = arboretum_node_new_with_id("enum_value", "clang::Type::LValueReference", nullptr);
  data_model.clang_Type_RValueReference_ = arboretum_node_new_with_id("enum_value", "clang::Type::RValueReference", nullptr);
  data_model.clang_Type_SubstTemplateTypeParmPack_ = arboretum_node_new_with_id("enum_value", "clang::Type::SubstTemplateTypeParmPack", nullptr);
  data_model.clang_Type_SubstTemplateTypeParm_ = arboretum_node_new_with_id("enum_value", "clang::Type::SubstTemplateTypeParm", nullptr);
  data_model.clang_Type_Enum_ = arboretum_node_new_with_id("enum_value", "clang::Type::Enum", nullptr);
  data_model.clang_Type_Record_ = arboretum_node_new_with_id("enum_value", "clang::Type::Record", nullptr);
  data_model.clang_Type_TemplateSpecialization_ = arboretum_node_new_with_id("enum_value", "clang::Type::TemplateSpecialization", nullptr);
  data_model.clang_Type_TemplateTypeParm_ = arboretum_node_new_with_id("enum_value", "clang::Type::TemplateTypeParm", nullptr);
  data_model.clang_Type_TypeOfExpr_ = arboretum_node_new_with_id("enum_value", "clang::Type::TypeOfExpr", nullptr);
  data_model.clang_Type_TypeOf_ = arboretum_node_new_with_id("enum_value", "clang::Type::TypeOf", nullptr);
  data_model.clang_Type_Typedef_ = arboretum_node_new_with_id("enum_value", "clang::Type::Typedef", nullptr);
  data_model.clang_Type_UnaryTransform_ = arboretum_node_new_with_id("enum_value", "clang::Type::UnaryTransform", nullptr);
  data_model.clang_Type_UnresolvedUsing_ = arboretum_node_new_with_id("enum_value", "clang::Type::UnresolvedUsing", nullptr);
  data_model.clang_Type_Using_ = arboretum_node_new_with_id("enum_value", "clang::Type::Using", nullptr);
  data_model.clang_Type_Vector_ = arboretum_node_new_with_id("enum_value", "clang::Type::Vector", nullptr);
  data_model.clang_Type_ExtVector_ = arboretum_node_new_with_id("enum_value", "clang::Type::ExtVector", nullptr);
  data_model.clang_Type_TypeLast_ = arboretum_node_new_with_id("enum_value", "clang::Type::TypeLast", nullptr);

  // clang::TypeOfKind
  data_model.clang_TypeOfKind_ = arboretum_node_new_with_id("enum", "clang::TypeOfKind", nullptr);
  data_model.clang_TypeOfKind_Qualified_ = arboretum_node_new_with_id("enum_value", "clang::TypeOfKind::Qualified", nullptr);
  data_model.clang_TypeOfKind_Unqualified_ = arboretum_node_new_with_id("enum_value", "clang::TypeOfKind::Unqualified", nullptr);

  // clang::TypeTrait
  data_model.clang_TypeTrait_ = arboretum_node_new_with_id("enum", "clang::TypeTrait", nullptr);
  data_model.clang_UTT_IsInterfaceClass_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsInterfaceClass", nullptr);
  data_model.clang_UTT_IsSealed_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsSealed", nullptr);
  data_model.clang_UTT_IsDestructible_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsDestructible", nullptr);
  data_model.clang_UTT_IsTriviallyDestructible_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsTriviallyDestructible", nullptr);
  data_model.clang_UTT_IsNothrowDestructible_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsNothrowDestructible", nullptr);
  data_model.clang_UTT_HasNothrowMoveAssign_ = arboretum_node_new_with_id("enum_value", "clang::UTT_HasNothrowMoveAssign", nullptr);
  data_model.clang_UTT_HasTrivialMoveAssign_ = arboretum_node_new_with_id("enum_value", "clang::UTT_HasTrivialMoveAssign", nullptr);
  data_model.clang_UTT_HasTrivialMoveConstructor_ = arboretum_node_new_with_id("enum_value", "clang::UTT_HasTrivialMoveConstructor", nullptr);
  data_model.clang_UTT_HasNothrowAssign_ = arboretum_node_new_with_id("enum_value", "clang::UTT_HasNothrowAssign", nullptr);
  data_model.clang_UTT_HasNothrowCopy_ = arboretum_node_new_with_id("enum_value", "clang::UTT_HasNothrowCopy", nullptr);
  data_model.clang_UTT_HasNothrowConstructor_ = arboretum_node_new_with_id("enum_value", "clang::UTT_HasNothrowConstructor", nullptr);
  data_model.clang_UTT_HasTrivialAssign_ = arboretum_node_new_with_id("enum_value", "clang::UTT_HasTrivialAssign", nullptr);
  data_model.clang_UTT_HasTrivialCopy_ = arboretum_node_new_with_id("enum_value", "clang::UTT_HasTrivialCopy", nullptr);
  data_model.clang_UTT_HasTrivialDefaultConstructor_ = arboretum_node_new_with_id("enum_value", "clang::UTT_HasTrivialDefaultConstructor", nullptr);
  data_model.clang_UTT_HasTrivialDestructor_ = arboretum_node_new_with_id("enum_value", "clang::UTT_HasTrivialDestructor", nullptr);
  data_model.clang_UTT_HasVirtualDestructor_ = arboretum_node_new_with_id("enum_value", "clang::UTT_HasVirtualDestructor", nullptr);
  data_model.clang_UTT_IsAbstract_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsAbstract", nullptr);
  data_model.clang_UTT_IsAggregate_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsAggregate", nullptr);
  data_model.clang_UTT_IsClass_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsClass", nullptr);
  data_model.clang_UTT_IsEmpty_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsEmpty", nullptr);
  data_model.clang_UTT_IsEnum_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsEnum", nullptr);
  data_model.clang_UTT_IsFinal_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsFinal", nullptr);
  data_model.clang_UTT_IsLiteral_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsLiteral", nullptr);
  data_model.clang_UTT_IsPOD_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsPOD", nullptr);
  data_model.clang_UTT_IsPolymorphic_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsPolymorphic", nullptr);
  data_model.clang_UTT_IsStandardLayout_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsStandardLayout", nullptr);
  data_model.clang_UTT_IsTrivial_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsTrivial", nullptr);
  data_model.clang_UTT_IsTriviallyCopyable_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsTriviallyCopyable", nullptr);
  data_model.clang_UTT_IsUnion_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsUnion", nullptr);
  data_model.clang_UTT_HasUniqueObjectRepresentations_ = arboretum_node_new_with_id("enum_value", "clang::UTT_HasUniqueObjectRepresentations", nullptr);
  data_model.clang_UTT_IsTriviallyRelocatable_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsTriviallyRelocatable", nullptr);
  data_model.clang_UTT_IsTriviallyEqualityComparable_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsTriviallyEqualityComparable", nullptr);
  data_model.clang_UTT_IsBoundedArray_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsBoundedArray", nullptr);
  data_model.clang_UTT_IsUnboundedArray_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsUnboundedArray", nullptr);
  data_model.clang_UTT_IsNullPointer_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsNullPointer", nullptr);
  data_model.clang_UTT_IsScopedEnum_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsScopedEnum", nullptr);
  data_model.clang_UTT_IsReferenceable_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsReferenceable", nullptr);
  data_model.clang_UTT_CanPassInRegs_ = arboretum_node_new_with_id("enum_value", "clang::UTT_CanPassInRegs", nullptr);
  data_model.clang_UTT_IsArithmetic_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsArithmetic", nullptr);
  data_model.clang_UTT_IsFloatingPoint_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsFloatingPoint", nullptr);
  data_model.clang_UTT_IsIntegral_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsIntegral", nullptr);
  data_model.clang_UTT_IsCompleteType_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsCompleteType", nullptr);
  data_model.clang_UTT_IsVoid_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsVoid", nullptr);
  data_model.clang_UTT_IsArray_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsArray", nullptr);
  data_model.clang_UTT_IsFunction_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsFunction", nullptr);
  data_model.clang_UTT_IsReference_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsReference", nullptr);
  data_model.clang_UTT_IsLvalueReference_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsLvalueReference", nullptr);
  data_model.clang_UTT_IsRvalueReference_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsRvalueReference", nullptr);
  data_model.clang_UTT_IsFundamental_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsFundamental", nullptr);
  data_model.clang_UTT_IsObject_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsObject", nullptr);
  data_model.clang_UTT_IsScalar_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsScalar", nullptr);
  data_model.clang_UTT_IsCompound_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsCompound", nullptr);
  data_model.clang_UTT_IsPointer_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsPointer", nullptr);
  data_model.clang_UTT_IsMemberObjectPointer_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsMemberObjectPointer", nullptr);
  data_model.clang_UTT_IsMemberFunctionPointer_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsMemberFunctionPointer", nullptr);
  data_model.clang_UTT_IsMemberPointer_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsMemberPointer", nullptr);
  data_model.clang_UTT_IsConst_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsConst", nullptr);
  data_model.clang_UTT_IsVolatile_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsVolatile", nullptr);
  data_model.clang_UTT_IsSigned_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsSigned", nullptr);
  data_model.clang_UTT_IsUnsigned_ = arboretum_node_new_with_id("enum_value", "clang::UTT_IsUnsigned", nullptr);
  data_model.clang_UTT_Last_ = arboretum_node_new_with_id("enum_value", "clang::UTT_Last", nullptr);
  data_model.clang_BTT_TypeCompatible_ = arboretum_node_new_with_id("enum_value", "clang::BTT_TypeCompatible", nullptr);
  data_model.clang_BTT_IsNothrowAssignable_ = arboretum_node_new_with_id("enum_value", "clang::BTT_IsNothrowAssignable", nullptr);
  data_model.clang_BTT_IsAssignable_ = arboretum_node_new_with_id("enum_value", "clang::BTT_IsAssignable", nullptr);
  data_model.clang_BTT_IsBaseOf_ = arboretum_node_new_with_id("enum_value", "clang::BTT_IsBaseOf", nullptr);
  data_model.clang_BTT_IsConvertibleTo_ = arboretum_node_new_with_id("enum_value", "clang::BTT_IsConvertibleTo", nullptr);
  data_model.clang_BTT_IsTriviallyAssignable_ = arboretum_node_new_with_id("enum_value", "clang::BTT_IsTriviallyAssignable", nullptr);
  data_model.clang_BTT_ReferenceBindsToTemporary_ = arboretum_node_new_with_id("enum_value", "clang::BTT_ReferenceBindsToTemporary", nullptr);
  data_model.clang_BTT_IsSame_ = arboretum_node_new_with_id("enum_value", "clang::BTT_IsSame", nullptr);
  data_model.clang_BTT_IsConvertible_ = arboretum_node_new_with_id("enum_value", "clang::BTT_IsConvertible", nullptr);
  data_model.clang_BTT_Last_ = arboretum_node_new_with_id("enum_value", "clang::BTT_Last", nullptr);
  data_model.clang_TT_IsConstructible_ = arboretum_node_new_with_id("enum_value", "clang::TT_IsConstructible", nullptr);
  data_model.clang_TT_IsNothrowConstructible_ = arboretum_node_new_with_id("enum_value", "clang::TT_IsNothrowConstructible", nullptr);
  data_model.clang_TT_IsTriviallyConstructible_ = arboretum_node_new_with_id("enum_value", "clang::TT_IsTriviallyConstructible", nullptr);
  data_model.clang_TT_Last_ = arboretum_node_new_with_id("enum_value", "clang::TT_Last", nullptr);

  // clang::UnaryExprOrTypeTrait
  data_model.clang_UnaryExprOrTypeTrait_ = arboretum_node_new_with_id("enum", "clang::UnaryExprOrTypeTrait", nullptr);
  data_model.clang_UETT_SizeOf_ = arboretum_node_new_with_id("enum_value", "clang::UETT_SizeOf", nullptr);
  data_model.clang_UETT_AlignOf_ = arboretum_node_new_with_id("enum_value", "clang::UETT_AlignOf", nullptr);
  data_model.clang_UETT_PreferredAlignOf_ = arboretum_node_new_with_id("enum_value", "clang::UETT_PreferredAlignOf", nullptr);
  data_model.clang_UETT_VecStep_ = arboretum_node_new_with_id("enum_value", "clang::UETT_VecStep", nullptr);
  data_model.clang_UETT_OpenMPRequiredSimdAlign_ = arboretum_node_new_with_id("enum_value", "clang::UETT_OpenMPRequiredSimdAlign", nullptr);
  data_model.clang_UETT_Last_ = arboretum_node_new_with_id("enum_value", "clang::UETT_Last", nullptr);

  // clang::UnaryOperatorKind
  data_model.clang_UnaryOperatorKind_ = arboretum_node_new_with_id("enum", "clang::UnaryOperatorKind", nullptr);
  data_model.clang_UO_PostInc_ = arboretum_node_new_with_id("enum_value", "clang::UO_PostInc", nullptr);
  data_model.clang_UO_PostDec_ = arboretum_node_new_with_id("enum_value", "clang::UO_PostDec", nullptr);
  data_model.clang_UO_PreInc_ = arboretum_node_new_with_id("enum_value", "clang::UO_PreInc", nullptr);
  data_model.clang_UO_PreDec_ = arboretum_node_new_with_id("enum_value", "clang::UO_PreDec", nullptr);
  data_model.clang_UO_AddrOf_ = arboretum_node_new_with_id("enum_value", "clang::UO_AddrOf", nullptr);
  data_model.clang_UO_Deref_ = arboretum_node_new_with_id("enum_value", "clang::UO_Deref", nullptr);
  data_model.clang_UO_Plus_ = arboretum_node_new_with_id("enum_value", "clang::UO_Plus", nullptr);
  data_model.clang_UO_Minus_ = arboretum_node_new_with_id("enum_value", "clang::UO_Minus", nullptr);
  data_model.clang_UO_Not_ = arboretum_node_new_with_id("enum_value", "clang::UO_Not", nullptr);
  data_model.clang_UO_LNot_ = arboretum_node_new_with_id("enum_value", "clang::UO_LNot", nullptr);
  data_model.clang_UO_Real_ = arboretum_node_new_with_id("enum_value", "clang::UO_Real", nullptr);
  data_model.clang_UO_Imag_ = arboretum_node_new_with_id("enum_value", "clang::UO_Imag", nullptr);
  data_model.clang_UO_Extension_ = arboretum_node_new_with_id("enum_value", "clang::UO_Extension", nullptr);
  data_model.clang_UO_Coawait_ = arboretum_node_new_with_id("enum_value", "clang::UO_Coawait", nullptr);

  // clang::UnaryTransformType::UTTKind
  data_model.clang_UnaryTransformType_UTTKind_ = arboretum_node_new_with_id("enum", "clang::UnaryTransformType::UTTKind", nullptr);
  data_model.clang_UnaryTransformType_AddLvalueReference_ = arboretum_node_new_with_id("enum_value", "clang::UnaryTransformType::AddLvalueReference", nullptr);
  data_model.clang_UnaryTransformType_AddPointer_ = arboretum_node_new_with_id("enum_value", "clang::UnaryTransformType::AddPointer", nullptr);
  data_model.clang_UnaryTransformType_AddRvalueReference_ = arboretum_node_new_with_id("enum_value", "clang::UnaryTransformType::AddRvalueReference", nullptr);
  data_model.clang_UnaryTransformType_Decay_ = arboretum_node_new_with_id("enum_value", "clang::UnaryTransformType::Decay", nullptr);
  data_model.clang_UnaryTransformType_MakeSigned_ = arboretum_node_new_with_id("enum_value", "clang::UnaryTransformType::MakeSigned", nullptr);
  data_model.clang_UnaryTransformType_MakeUnsigned_ = arboretum_node_new_with_id("enum_value", "clang::UnaryTransformType::MakeUnsigned", nullptr);
  data_model.clang_UnaryTransformType_RemoveAllExtents_ = arboretum_node_new_with_id("enum_value", "clang::UnaryTransformType::RemoveAllExtents", nullptr);
  data_model.clang_UnaryTransformType_RemoveConst_ = arboretum_node_new_with_id("enum_value", "clang::UnaryTransformType::RemoveConst", nullptr);
  data_model.clang_UnaryTransformType_RemoveCV_ = arboretum_node_new_with_id("enum_value", "clang::UnaryTransformType::RemoveCV", nullptr);
  data_model.clang_UnaryTransformType_RemoveCVRef_ = arboretum_node_new_with_id("enum_value", "clang::UnaryTransformType::RemoveCVRef", nullptr);
  data_model.clang_UnaryTransformType_RemoveExtent_ = arboretum_node_new_with_id("enum_value", "clang::UnaryTransformType::RemoveExtent", nullptr);
  data_model.clang_UnaryTransformType_RemovePointer_ = arboretum_node_new_with_id("enum_value", "clang::UnaryTransformType::RemovePointer", nullptr);
  data_model.clang_UnaryTransformType_RemoveReference_ = arboretum_node_new_with_id("enum_value", "clang::UnaryTransformType::RemoveReference", nullptr);
  data_model.clang_UnaryTransformType_RemoveRestrict_ = arboretum_node_new_with_id("enum_value", "clang::UnaryTransformType::RemoveRestrict", nullptr);
  data_model.clang_UnaryTransformType_RemoveVolatile_ = arboretum_node_new_with_id("enum_value", "clang::UnaryTransformType::RemoveVolatile", nullptr);
  data_model.clang_UnaryTransformType_EnumUnderlyingType_ = arboretum_node_new_with_id("enum_value", "clang::UnaryTransformType::EnumUnderlyingType", nullptr);

  // clang::UserDefinedLiteral::LiteralOperatorKind
  data_model.clang_UserDefinedLiteral_LiteralOperatorKind_ = arboretum_node_new_with_id("enum", "clang::UserDefinedLiteral::LiteralOperatorKind", nullptr);
  data_model.clang_UserDefinedLiteral_LOK_Raw_ = arboretum_node_new_with_id("enum_value", "clang::UserDefinedLiteral::LOK_Raw", nullptr);
  data_model.clang_UserDefinedLiteral_LOK_Template_ = arboretum_node_new_with_id("enum_value", "clang::UserDefinedLiteral::LOK_Template", nullptr);
  data_model.clang_UserDefinedLiteral_LOK_Integer_ = arboretum_node_new_with_id("enum_value", "clang::UserDefinedLiteral::LOK_Integer", nullptr);
  data_model.clang_UserDefinedLiteral_LOK_Floating_ = arboretum_node_new_with_id("enum_value", "clang::UserDefinedLiteral::LOK_Floating", nullptr);
  data_model.clang_UserDefinedLiteral_LOK_String_ = arboretum_node_new_with_id("enum_value", "clang::UserDefinedLiteral::LOK_String", nullptr);
  data_model.clang_UserDefinedLiteral_LOK_Character_ = arboretum_node_new_with_id("enum_value", "clang::UserDefinedLiteral::LOK_Character", nullptr);

  // clang::VarDecl::DefinitionKind
  data_model.clang_VarDecl_DefinitionKind_ = arboretum_node_new_with_id("enum", "clang::VarDecl::DefinitionKind", nullptr);
  data_model.clang_VarDecl_DeclarationOnly_ = arboretum_node_new_with_id("enum_value", "clang::VarDecl::DeclarationOnly", nullptr);
  data_model.clang_VarDecl_TentativeDefinition_ = arboretum_node_new_with_id("enum_value", "clang::VarDecl::TentativeDefinition", nullptr);
  data_model.clang_VarDecl_Definition_ = arboretum_node_new_with_id("enum_value", "clang::VarDecl::Definition", nullptr);

  // clang::VarDecl::InitializationStyle
  data_model.clang_VarDecl_InitializationStyle_ = arboretum_node_new_with_id("enum", "clang::VarDecl::InitializationStyle", nullptr);
  data_model.clang_VarDecl_CInit_ = arboretum_node_new_with_id("enum_value", "clang::VarDecl::CInit", nullptr);
  data_model.clang_VarDecl_CallInit_ = arboretum_node_new_with_id("enum_value", "clang::VarDecl::CallInit", nullptr);
  data_model.clang_VarDecl_ListInit_ = arboretum_node_new_with_id("enum_value", "clang::VarDecl::ListInit", nullptr);
  data_model.clang_VarDecl_ParenListInit_ = arboretum_node_new_with_id("enum_value", "clang::VarDecl::ParenListInit", nullptr);

  // clang::VarDecl::TLSKind
  data_model.clang_VarDecl_TLSKind_ = arboretum_node_new_with_id("enum", "clang::VarDecl::TLSKind", nullptr);
  data_model.clang_VarDecl_TLS_None_ = arboretum_node_new_with_id("enum_value", "clang::VarDecl::TLS_None", nullptr);
  data_model.clang_VarDecl_TLS_Static_ = arboretum_node_new_with_id("enum_value", "clang::VarDecl::TLS_Static", nullptr);
  data_model.clang_VarDecl_TLS_Dynamic_ = arboretum_node_new_with_id("enum_value", "clang::VarDecl::TLS_Dynamic", nullptr);

  // clang::VectorType::VectorKind
  data_model.clang_VectorType_VectorKind_ = arboretum_node_new_with_id("enum", "clang::VectorType::VectorKind", nullptr);
  data_model.clang_VectorType_GenericVector_ = arboretum_node_new_with_id("enum_value", "clang::VectorType::GenericVector", nullptr);
  data_model.clang_VectorType_AltiVecVector_ = arboretum_node_new_with_id("enum_value", "clang::VectorType::AltiVecVector", nullptr);
  data_model.clang_VectorType_AltiVecPixel_ = arboretum_node_new_with_id("enum_value", "clang::VectorType::AltiVecPixel", nullptr);
  data_model.clang_VectorType_AltiVecBool_ = arboretum_node_new_with_id("enum_value", "clang::VectorType::AltiVecBool", nullptr);
  data_model.clang_VectorType_NeonVector_ = arboretum_node_new_with_id("enum_value", "clang::VectorType::NeonVector", nullptr);
  data_model.clang_VectorType_NeonPolyVector_ = arboretum_node_new_with_id("enum_value", "clang::VectorType::NeonPolyVector", nullptr);
  data_model.clang_VectorType_SveFixedLengthDataVector_ = arboretum_node_new_with_id("enum_value", "clang::VectorType::SveFixedLengthDataVector", nullptr);
  data_model.clang_VectorType_SveFixedLengthPredicateVector_ = arboretum_node_new_with_id("enum_value", "clang::VectorType::SveFixedLengthPredicateVector", nullptr);
  data_model.clang_VectorType_RVVFixedLengthDataVector_ = arboretum_node_new_with_id("enum_value", "clang::VectorType::RVVFixedLengthDataVector", nullptr);

  // clang::attr::Kind
  data_model.clang_attr_Kind_ = arboretum_node_new_with_id("enum", "clang::attr::Kind", nullptr);
  data_model.clang_attr_AddressSpace_ = arboretum_node_new_with_id("enum_value", "clang::attr::AddressSpace", nullptr);
  data_model.clang_attr_AnnotateType_ = arboretum_node_new_with_id("enum_value", "clang::attr::AnnotateType", nullptr);
  data_model.clang_attr_ArmMveStrictPolymorphism_ = arboretum_node_new_with_id("enum_value", "clang::attr::ArmMveStrictPolymorphism", nullptr);
  data_model.clang_attr_ArmStreaming_ = arboretum_node_new_with_id("enum_value", "clang::attr::ArmStreaming", nullptr);
  data_model.clang_attr_BTFTypeTag_ = arboretum_node_new_with_id("enum_value", "clang::attr::BTFTypeTag", nullptr);
  data_model.clang_attr_CmseNSCall_ = arboretum_node_new_with_id("enum_value", "clang::attr::CmseNSCall", nullptr);
  data_model.clang_attr_HLSLGroupSharedAddressSpace_ = arboretum_node_new_with_id("enum_value", "clang::attr::HLSLGroupSharedAddressSpace", nullptr);
  data_model.clang_attr_NoDeref_ = arboretum_node_new_with_id("enum_value", "clang::attr::NoDeref", nullptr);
  data_model.clang_attr_ObjCGC_ = arboretum_node_new_with_id("enum_value", "clang::attr::ObjCGC", nullptr);
  data_model.clang_attr_ObjCInertUnsafeUnretained_ = arboretum_node_new_with_id("enum_value", "clang::attr::ObjCInertUnsafeUnretained", nullptr);
  data_model.clang_attr_ObjCKindOf_ = arboretum_node_new_with_id("enum_value", "clang::attr::ObjCKindOf", nullptr);
  data_model.clang_attr_OpenCLConstantAddressSpace_ = arboretum_node_new_with_id("enum_value", "clang::attr::OpenCLConstantAddressSpace", nullptr);
  data_model.clang_attr_OpenCLGenericAddressSpace_ = arboretum_node_new_with_id("enum_value", "clang::attr::OpenCLGenericAddressSpace", nullptr);
  data_model.clang_attr_OpenCLGlobalAddressSpace_ = arboretum_node_new_with_id("enum_value", "clang::attr::OpenCLGlobalAddressSpace", nullptr);
  data_model.clang_attr_OpenCLGlobalDeviceAddressSpace_ = arboretum_node_new_with_id("enum_value", "clang::attr::OpenCLGlobalDeviceAddressSpace", nullptr);
  data_model.clang_attr_OpenCLGlobalHostAddressSpace_ = arboretum_node_new_with_id("enum_value", "clang::attr::OpenCLGlobalHostAddressSpace", nullptr);
  data_model.clang_attr_OpenCLLocalAddressSpace_ = arboretum_node_new_with_id("enum_value", "clang::attr::OpenCLLocalAddressSpace", nullptr);
  data_model.clang_attr_OpenCLPrivateAddressSpace_ = arboretum_node_new_with_id("enum_value", "clang::attr::OpenCLPrivateAddressSpace", nullptr);
  data_model.clang_attr_Ptr32_ = arboretum_node_new_with_id("enum_value", "clang::attr::Ptr32", nullptr);
  data_model.clang_attr_Ptr64_ = arboretum_node_new_with_id("enum_value", "clang::attr::Ptr64", nullptr);
  data_model.clang_attr_SPtr_ = arboretum_node_new_with_id("enum_value", "clang::attr::SPtr", nullptr);
  data_model.clang_attr_TypeNonNull_ = arboretum_node_new_with_id("enum_value", "clang::attr::TypeNonNull", nullptr);
  data_model.clang_attr_TypeNullUnspecified_ = arboretum_node_new_with_id("enum_value", "clang::attr::TypeNullUnspecified", nullptr);
  data_model.clang_attr_TypeNullable_ = arboretum_node_new_with_id("enum_value", "clang::attr::TypeNullable", nullptr);
  data_model.clang_attr_TypeNullableResult_ = arboretum_node_new_with_id("enum_value", "clang::attr::TypeNullableResult", nullptr);
  data_model.clang_attr_UPtr_ = arboretum_node_new_with_id("enum_value", "clang::attr::UPtr", nullptr);
  data_model.clang_attr_WebAssemblyFuncref_ = arboretum_node_new_with_id("enum_value", "clang::attr::WebAssemblyFuncref", nullptr);
  data_model.clang_attr_FallThrough_ = arboretum_node_new_with_id("enum_value", "clang::attr::FallThrough", nullptr);
  data_model.clang_attr_Likely_ = arboretum_node_new_with_id("enum_value", "clang::attr::Likely", nullptr);
  data_model.clang_attr_MustTail_ = arboretum_node_new_with_id("enum_value", "clang::attr::MustTail", nullptr);
  data_model.clang_attr_OpenCLUnrollHint_ = arboretum_node_new_with_id("enum_value", "clang::attr::OpenCLUnrollHint", nullptr);
  data_model.clang_attr_Suppress_ = arboretum_node_new_with_id("enum_value", "clang::attr::Suppress", nullptr);
  data_model.clang_attr_Unlikely_ = arboretum_node_new_with_id("enum_value", "clang::attr::Unlikely", nullptr);
  data_model.clang_attr_AlwaysInline_ = arboretum_node_new_with_id("enum_value", "clang::attr::AlwaysInline", nullptr);
  data_model.clang_attr_NoInline_ = arboretum_node_new_with_id("enum_value", "clang::attr::NoInline", nullptr);
  data_model.clang_attr_NoMerge_ = arboretum_node_new_with_id("enum_value", "clang::attr::NoMerge", nullptr);
  data_model.clang_attr_AArch64SVEPcs_ = arboretum_node_new_with_id("enum_value", "clang::attr::AArch64SVEPcs", nullptr);
  data_model.clang_attr_AArch64VectorPcs_ = arboretum_node_new_with_id("enum_value", "clang::attr::AArch64VectorPcs", nullptr);
  data_model.clang_attr_AMDGPUKernelCall_ = arboretum_node_new_with_id("enum_value", "clang::attr::AMDGPUKernelCall", nullptr);
  data_model.clang_attr_AcquireHandle_ = arboretum_node_new_with_id("enum_value", "clang::attr::AcquireHandle", nullptr);
  data_model.clang_attr_AnyX86NoCfCheck_ = arboretum_node_new_with_id("enum_value", "clang::attr::AnyX86NoCfCheck", nullptr);
  data_model.clang_attr_CDecl_ = arboretum_node_new_with_id("enum_value", "clang::attr::CDecl", nullptr);
  data_model.clang_attr_FastCall_ = arboretum_node_new_with_id("enum_value", "clang::attr::FastCall", nullptr);
  data_model.clang_attr_IntelOclBicc_ = arboretum_node_new_with_id("enum_value", "clang::attr::IntelOclBicc", nullptr);
  data_model.clang_attr_LifetimeBound_ = arboretum_node_new_with_id("enum_value", "clang::attr::LifetimeBound", nullptr);
  data_model.clang_attr_MSABI_ = arboretum_node_new_with_id("enum_value", "clang::attr::MSABI", nullptr);
  data_model.clang_attr_NSReturnsRetained_ = arboretum_node_new_with_id("enum_value", "clang::attr::NSReturnsRetained", nullptr);
  data_model.clang_attr_ObjCOwnership_ = arboretum_node_new_with_id("enum_value", "clang::attr::ObjCOwnership", nullptr);
  data_model.clang_attr_Pascal_ = arboretum_node_new_with_id("enum_value", "clang::attr::Pascal", nullptr);
  data_model.clang_attr_Pcs_ = arboretum_node_new_with_id("enum_value", "clang::attr::Pcs", nullptr);
  data_model.clang_attr_PreserveAll_ = arboretum_node_new_with_id("enum_value", "clang::attr::PreserveAll", nullptr);
  data_model.clang_attr_PreserveMost_ = arboretum_node_new_with_id("enum_value", "clang::attr::PreserveMost", nullptr);
  data_model.clang_attr_RegCall_ = arboretum_node_new_with_id("enum_value", "clang::attr::RegCall", nullptr);
  data_model.clang_attr_StdCall_ = arboretum_node_new_with_id("enum_value", "clang::attr::StdCall", nullptr);
  data_model.clang_attr_SwiftAsyncCall_ = arboretum_node_new_with_id("enum_value", "clang::attr::SwiftAsyncCall", nullptr);
  data_model.clang_attr_SwiftCall_ = arboretum_node_new_with_id("enum_value", "clang::attr::SwiftCall", nullptr);
  data_model.clang_attr_SysVABI_ = arboretum_node_new_with_id("enum_value", "clang::attr::SysVABI", nullptr);
  data_model.clang_attr_ThisCall_ = arboretum_node_new_with_id("enum_value", "clang::attr::ThisCall", nullptr);
  data_model.clang_attr_VectorCall_ = arboretum_node_new_with_id("enum_value", "clang::attr::VectorCall", nullptr);
  data_model.clang_attr_SwiftAsyncContext_ = arboretum_node_new_with_id("enum_value", "clang::attr::SwiftAsyncContext", nullptr);
  data_model.clang_attr_SwiftContext_ = arboretum_node_new_with_id("enum_value", "clang::attr::SwiftContext", nullptr);
  data_model.clang_attr_SwiftErrorResult_ = arboretum_node_new_with_id("enum_value", "clang::attr::SwiftErrorResult", nullptr);
  data_model.clang_attr_SwiftIndirectResult_ = arboretum_node_new_with_id("enum_value", "clang::attr::SwiftIndirectResult", nullptr);
  data_model.clang_attr_Annotate_ = arboretum_node_new_with_id("enum_value", "clang::attr::Annotate", nullptr);
  data_model.clang_attr_CFConsumed_ = arboretum_node_new_with_id("enum_value", "clang::attr::CFConsumed", nullptr);
  data_model.clang_attr_CarriesDependency_ = arboretum_node_new_with_id("enum_value", "clang::attr::CarriesDependency", nullptr);
  data_model.clang_attr_NSConsumed_ = arboretum_node_new_with_id("enum_value", "clang::attr::NSConsumed", nullptr);
  data_model.clang_attr_NonNull_ = arboretum_node_new_with_id("enum_value", "clang::attr::NonNull", nullptr);
  data_model.clang_attr_OSConsumed_ = arboretum_node_new_with_id("enum_value", "clang::attr::OSConsumed", nullptr);
  data_model.clang_attr_PassObjectSize_ = arboretum_node_new_with_id("enum_value", "clang::attr::PassObjectSize", nullptr);
  data_model.clang_attr_ReleaseHandle_ = arboretum_node_new_with_id("enum_value", "clang::attr::ReleaseHandle", nullptr);
  data_model.clang_attr_UseHandle_ = arboretum_node_new_with_id("enum_value", "clang::attr::UseHandle", nullptr);
  data_model.clang_attr_HLSLSV_DispatchThreadID_ = arboretum_node_new_with_id("enum_value", "clang::attr::HLSLSV_DispatchThreadID", nullptr);
  data_model.clang_attr_HLSLSV_GroupIndex_ = arboretum_node_new_with_id("enum_value", "clang::attr::HLSLSV_GroupIndex", nullptr);
  data_model.clang_attr_AMDGPUFlatWorkGroupSize_ = arboretum_node_new_with_id("enum_value", "clang::attr::AMDGPUFlatWorkGroupSize", nullptr);
  data_model.clang_attr_AMDGPUNumSGPR_ = arboretum_node_new_with_id("enum_value", "clang::attr::AMDGPUNumSGPR", nullptr);
  data_model.clang_attr_AMDGPUNumVGPR_ = arboretum_node_new_with_id("enum_value", "clang::attr::AMDGPUNumVGPR", nullptr);
  data_model.clang_attr_AMDGPUWavesPerEU_ = arboretum_node_new_with_id("enum_value", "clang::attr::AMDGPUWavesPerEU", nullptr);
  data_model.clang_attr_ARMInterrupt_ = arboretum_node_new_with_id("enum_value", "clang::attr::ARMInterrupt", nullptr);
  data_model.clang_attr_AVRInterrupt_ = arboretum_node_new_with_id("enum_value", "clang::attr::AVRInterrupt", nullptr);
  data_model.clang_attr_AVRSignal_ = arboretum_node_new_with_id("enum_value", "clang::attr::AVRSignal", nullptr);
  data_model.clang_attr_AcquireCapability_ = arboretum_node_new_with_id("enum_value", "clang::attr::AcquireCapability", nullptr);
  data_model.clang_attr_AcquiredAfter_ = arboretum_node_new_with_id("enum_value", "clang::attr::AcquiredAfter", nullptr);
  data_model.clang_attr_AcquiredBefore_ = arboretum_node_new_with_id("enum_value", "clang::attr::AcquiredBefore", nullptr);
  data_model.clang_attr_AlignMac68k_ = arboretum_node_new_with_id("enum_value", "clang::attr::AlignMac68k", nullptr);
  data_model.clang_attr_AlignNatural_ = arboretum_node_new_with_id("enum_value", "clang::attr::AlignNatural", nullptr);
  data_model.clang_attr_Aligned_ = arboretum_node_new_with_id("enum_value", "clang::attr::Aligned", nullptr);
  data_model.clang_attr_AllocAlign_ = arboretum_node_new_with_id("enum_value", "clang::attr::AllocAlign", nullptr);
  data_model.clang_attr_AllocSize_ = arboretum_node_new_with_id("enum_value", "clang::attr::AllocSize", nullptr);
  data_model.clang_attr_AlwaysDestroy_ = arboretum_node_new_with_id("enum_value", "clang::attr::AlwaysDestroy", nullptr);
  data_model.clang_attr_AnalyzerNoReturn_ = arboretum_node_new_with_id("enum_value", "clang::attr::AnalyzerNoReturn", nullptr);
  data_model.clang_attr_AnyX86Interrupt_ = arboretum_node_new_with_id("enum_value", "clang::attr::AnyX86Interrupt", nullptr);
  data_model.clang_attr_AnyX86NoCallerSavedRegisters_ = arboretum_node_new_with_id("enum_value", "clang::attr::AnyX86NoCallerSavedRegisters", nullptr);
  data_model.clang_attr_ArcWeakrefUnavailable_ = arboretum_node_new_with_id("enum_value", "clang::attr::ArcWeakrefUnavailable", nullptr);
  data_model.clang_attr_ArgumentWithTypeTag_ = arboretum_node_new_with_id("enum_value", "clang::attr::ArgumentWithTypeTag", nullptr);
  data_model.clang_attr_ArmBuiltinAlias_ = arboretum_node_new_with_id("enum_value", "clang::attr::ArmBuiltinAlias", nullptr);
  data_model.clang_attr_Artificial_ = arboretum_node_new_with_id("enum_value", "clang::attr::Artificial", nullptr);
  data_model.clang_attr_AsmLabel_ = arboretum_node_new_with_id("enum_value", "clang::attr::AsmLabel", nullptr);
  data_model.clang_attr_AssertCapability_ = arboretum_node_new_with_id("enum_value", "clang::attr::AssertCapability", nullptr);
  data_model.clang_attr_AssertExclusiveLock_ = arboretum_node_new_with_id("enum_value", "clang::attr::AssertExclusiveLock", nullptr);
  data_model.clang_attr_AssertSharedLock_ = arboretum_node_new_with_id("enum_value", "clang::attr::AssertSharedLock", nullptr);
  data_model.clang_attr_AssumeAligned_ = arboretum_node_new_with_id("enum_value", "clang::attr::AssumeAligned", nullptr);
  data_model.clang_attr_Assumption_ = arboretum_node_new_with_id("enum_value", "clang::attr::Assumption", nullptr);
  data_model.clang_attr_Availability_ = arboretum_node_new_with_id("enum_value", "clang::attr::Availability", nullptr);
  data_model.clang_attr_AvailableOnlyInDefaultEvalMethod_ = arboretum_node_new_with_id("enum_value", "clang::attr::AvailableOnlyInDefaultEvalMethod", nullptr);
  data_model.clang_attr_BPFPreserveAccessIndex_ = arboretum_node_new_with_id("enum_value", "clang::attr::BPFPreserveAccessIndex", nullptr);
  data_model.clang_attr_BTFDeclTag_ = arboretum_node_new_with_id("enum_value", "clang::attr::BTFDeclTag", nullptr);
  data_model.clang_attr_Blocks_ = arboretum_node_new_with_id("enum_value", "clang::attr::Blocks", nullptr);
  data_model.clang_attr_Builtin_ = arboretum_node_new_with_id("enum_value", "clang::attr::Builtin", nullptr);
  data_model.clang_attr_C11NoReturn_ = arboretum_node_new_with_id("enum_value", "clang::attr::C11NoReturn", nullptr);
  data_model.clang_attr_CFAuditedTransfer_ = arboretum_node_new_with_id("enum_value", "clang::attr::CFAuditedTransfer", nullptr);
  data_model.clang_attr_CFGuard_ = arboretum_node_new_with_id("enum_value", "clang::attr::CFGuard", nullptr);
  data_model.clang_attr_CFICanonicalJumpTable_ = arboretum_node_new_with_id("enum_value", "clang::attr::CFICanonicalJumpTable", nullptr);
  data_model.clang_attr_CFReturnsNotRetained_ = arboretum_node_new_with_id("enum_value", "clang::attr::CFReturnsNotRetained", nullptr);
  data_model.clang_attr_CFReturnsRetained_ = arboretum_node_new_with_id("enum_value", "clang::attr::CFReturnsRetained", nullptr);
  data_model.clang_attr_CFUnknownTransfer_ = arboretum_node_new_with_id("enum_value", "clang::attr::CFUnknownTransfer", nullptr);
  data_model.clang_attr_CPUDispatch_ = arboretum_node_new_with_id("enum_value", "clang::attr::CPUDispatch", nullptr);
  data_model.clang_attr_CPUSpecific_ = arboretum_node_new_with_id("enum_value", "clang::attr::CPUSpecific", nullptr);
  data_model.clang_attr_CUDAConstant_ = arboretum_node_new_with_id("enum_value", "clang::attr::CUDAConstant", nullptr);
  data_model.clang_attr_CUDADevice_ = arboretum_node_new_with_id("enum_value", "clang::attr::CUDADevice", nullptr);
  data_model.clang_attr_CUDADeviceBuiltinSurfaceType_ = arboretum_node_new_with_id("enum_value", "clang::attr::CUDADeviceBuiltinSurfaceType", nullptr);
  data_model.clang_attr_CUDADeviceBuiltinTextureType_ = arboretum_node_new_with_id("enum_value", "clang::attr::CUDADeviceBuiltinTextureType", nullptr);
  data_model.clang_attr_CUDAGlobal_ = arboretum_node_new_with_id("enum_value", "clang::attr::CUDAGlobal", nullptr);
  data_model.clang_attr_CUDAHost_ = arboretum_node_new_with_id("enum_value", "clang::attr::CUDAHost", nullptr);
  data_model.clang_attr_CUDAInvalidTarget_ = arboretum_node_new_with_id("enum_value", "clang::attr::CUDAInvalidTarget", nullptr);
  data_model.clang_attr_CUDALaunchBounds_ = arboretum_node_new_with_id("enum_value", "clang::attr::CUDALaunchBounds", nullptr);
  data_model.clang_attr_CUDAShared_ = arboretum_node_new_with_id("enum_value", "clang::attr::CUDAShared", nullptr);
  data_model.clang_attr_CXX11NoReturn_ = arboretum_node_new_with_id("enum_value", "clang::attr::CXX11NoReturn", nullptr);
  data_model.clang_attr_CallableWhen_ = arboretum_node_new_with_id("enum_value", "clang::attr::CallableWhen", nullptr);
  data_model.clang_attr_Callback_ = arboretum_node_new_with_id("enum_value", "clang::attr::Callback", nullptr);
  data_model.clang_attr_Capability_ = arboretum_node_new_with_id("enum_value", "clang::attr::Capability", nullptr);
  data_model.clang_attr_CapturedRecord_ = arboretum_node_new_with_id("enum_value", "clang::attr::CapturedRecord", nullptr);
  data_model.clang_attr_Cleanup_ = arboretum_node_new_with_id("enum_value", "clang::attr::Cleanup", nullptr);
  data_model.clang_attr_CmseNSEntry_ = arboretum_node_new_with_id("enum_value", "clang::attr::CmseNSEntry", nullptr);
  data_model.clang_attr_CodeSeg_ = arboretum_node_new_with_id("enum_value", "clang::attr::CodeSeg", nullptr);
  data_model.clang_attr_Cold_ = arboretum_node_new_with_id("enum_value", "clang::attr::Cold", nullptr);
  data_model.clang_attr_Common_ = arboretum_node_new_with_id("enum_value", "clang::attr::Common", nullptr);
  data_model.clang_attr_Const_ = arboretum_node_new_with_id("enum_value", "clang::attr::Const", nullptr);
  data_model.clang_attr_ConstInit_ = arboretum_node_new_with_id("enum_value", "clang::attr::ConstInit", nullptr);
  data_model.clang_attr_Constructor_ = arboretum_node_new_with_id("enum_value", "clang::attr::Constructor", nullptr);
  data_model.clang_attr_Consumable_ = arboretum_node_new_with_id("enum_value", "clang::attr::Consumable", nullptr);
  data_model.clang_attr_ConsumableAutoCast_ = arboretum_node_new_with_id("enum_value", "clang::attr::ConsumableAutoCast", nullptr);
  data_model.clang_attr_ConsumableSetOnRead_ = arboretum_node_new_with_id("enum_value", "clang::attr::ConsumableSetOnRead", nullptr);
  data_model.clang_attr_Convergent_ = arboretum_node_new_with_id("enum_value", "clang::attr::Convergent", nullptr);
  data_model.clang_attr_DLLExport_ = arboretum_node_new_with_id("enum_value", "clang::attr::DLLExport", nullptr);
  data_model.clang_attr_DLLExportStaticLocal_ = arboretum_node_new_with_id("enum_value", "clang::attr::DLLExportStaticLocal", nullptr);
  data_model.clang_attr_DLLImport_ = arboretum_node_new_with_id("enum_value", "clang::attr::DLLImport", nullptr);
  data_model.clang_attr_DLLImportStaticLocal_ = arboretum_node_new_with_id("enum_value", "clang::attr::DLLImportStaticLocal", nullptr);
  data_model.clang_attr_Deprecated_ = arboretum_node_new_with_id("enum_value", "clang::attr::Deprecated", nullptr);
  data_model.clang_attr_Destructor_ = arboretum_node_new_with_id("enum_value", "clang::attr::Destructor", nullptr);
  data_model.clang_attr_DiagnoseAsBuiltin_ = arboretum_node_new_with_id("enum_value", "clang::attr::DiagnoseAsBuiltin", nullptr);
  data_model.clang_attr_DiagnoseIf_ = arboretum_node_new_with_id("enum_value", "clang::attr::DiagnoseIf", nullptr);
  data_model.clang_attr_DisableSanitizerInstrumentation_ = arboretum_node_new_with_id("enum_value", "clang::attr::DisableSanitizerInstrumentation", nullptr);
  data_model.clang_attr_DisableTailCalls_ = arboretum_node_new_with_id("enum_value", "clang::attr::DisableTailCalls", nullptr);
  data_model.clang_attr_EmptyBases_ = arboretum_node_new_with_id("enum_value", "clang::attr::EmptyBases", nullptr);
  data_model.clang_attr_EnableIf_ = arboretum_node_new_with_id("enum_value", "clang::attr::EnableIf", nullptr);
  data_model.clang_attr_EnforceTCB_ = arboretum_node_new_with_id("enum_value", "clang::attr::EnforceTCB", nullptr);
  data_model.clang_attr_EnforceTCBLeaf_ = arboretum_node_new_with_id("enum_value", "clang::attr::EnforceTCBLeaf", nullptr);
  data_model.clang_attr_EnumExtensibility_ = arboretum_node_new_with_id("enum_value", "clang::attr::EnumExtensibility", nullptr);
  data_model.clang_attr_Error_ = arboretum_node_new_with_id("enum_value", "clang::attr::Error", nullptr);
  data_model.clang_attr_ExcludeFromExplicitInstantiation_ = arboretum_node_new_with_id("enum_value", "clang::attr::ExcludeFromExplicitInstantiation", nullptr);
  data_model.clang_attr_ExclusiveTrylockFunction_ = arboretum_node_new_with_id("enum_value", "clang::attr::ExclusiveTrylockFunction", nullptr);
  data_model.clang_attr_ExternalSourceSymbol_ = arboretum_node_new_with_id("enum_value", "clang::attr::ExternalSourceSymbol", nullptr);
  data_model.clang_attr_Final_ = arboretum_node_new_with_id("enum_value", "clang::attr::Final", nullptr);
  data_model.clang_attr_FlagEnum_ = arboretum_node_new_with_id("enum_value", "clang::attr::FlagEnum", nullptr);
  data_model.clang_attr_Flatten_ = arboretum_node_new_with_id("enum_value", "clang::attr::Flatten", nullptr);
  data_model.clang_attr_Format_ = arboretum_node_new_with_id("enum_value", "clang::attr::Format", nullptr);
  data_model.clang_attr_FormatArg_ = arboretum_node_new_with_id("enum_value", "clang::attr::FormatArg", nullptr);
  data_model.clang_attr_FunctionReturnThunks_ = arboretum_node_new_with_id("enum_value", "clang::attr::FunctionReturnThunks", nullptr);
  data_model.clang_attr_GNUInline_ = arboretum_node_new_with_id("enum_value", "clang::attr::GNUInline", nullptr);
  data_model.clang_attr_GuardedBy_ = arboretum_node_new_with_id("enum_value", "clang::attr::GuardedBy", nullptr);
  data_model.clang_attr_GuardedVar_ = arboretum_node_new_with_id("enum_value", "clang::attr::GuardedVar", nullptr);
  data_model.clang_attr_HIPManaged_ = arboretum_node_new_with_id("enum_value", "clang::attr::HIPManaged", nullptr);
  data_model.clang_attr_HLSLNumThreads_ = arboretum_node_new_with_id("enum_value", "clang::attr::HLSLNumThreads", nullptr);
  data_model.clang_attr_HLSLResource_ = arboretum_node_new_with_id("enum_value", "clang::attr::HLSLResource", nullptr);
  data_model.clang_attr_HLSLResourceBinding_ = arboretum_node_new_with_id("enum_value", "clang::attr::HLSLResourceBinding", nullptr);
  data_model.clang_attr_HLSLShader_ = arboretum_node_new_with_id("enum_value", "clang::attr::HLSLShader", nullptr);
  data_model.clang_attr_Hot_ = arboretum_node_new_with_id("enum_value", "clang::attr::Hot", nullptr);
  data_model.clang_attr_IBAction_ = arboretum_node_new_with_id("enum_value", "clang::attr::IBAction", nullptr);
  data_model.clang_attr_IBOutlet_ = arboretum_node_new_with_id("enum_value", "clang::attr::IBOutlet", nullptr);
  data_model.clang_attr_IBOutletCollection_ = arboretum_node_new_with_id("enum_value", "clang::attr::IBOutletCollection", nullptr);
  data_model.clang_attr_InitPriority_ = arboretum_node_new_with_id("enum_value", "clang::attr::InitPriority", nullptr);
  data_model.clang_attr_InternalLinkage_ = arboretum_node_new_with_id("enum_value", "clang::attr::InternalLinkage", nullptr);
  data_model.clang_attr_LTOVisibilityPublic_ = arboretum_node_new_with_id("enum_value", "clang::attr::LTOVisibilityPublic", nullptr);
  data_model.clang_attr_LayoutVersion_ = arboretum_node_new_with_id("enum_value", "clang::attr::LayoutVersion", nullptr);
  data_model.clang_attr_Leaf_ = arboretum_node_new_with_id("enum_value", "clang::attr::Leaf", nullptr);
  data_model.clang_attr_LockReturned_ = arboretum_node_new_with_id("enum_value", "clang::attr::LockReturned", nullptr);
  data_model.clang_attr_LocksExcluded_ = arboretum_node_new_with_id("enum_value", "clang::attr::LocksExcluded", nullptr);
  data_model.clang_attr_M68kInterrupt_ = arboretum_node_new_with_id("enum_value", "clang::attr::M68kInterrupt", nullptr);
  data_model.clang_attr_MIGServerRoutine_ = arboretum_node_new_with_id("enum_value", "clang::attr::MIGServerRoutine", nullptr);
  data_model.clang_attr_MSAllocator_ = arboretum_node_new_with_id("enum_value", "clang::attr::MSAllocator", nullptr);
  data_model.clang_attr_MSInheritance_ = arboretum_node_new_with_id("enum_value", "clang::attr::MSInheritance", nullptr);
  data_model.clang_attr_MSNoVTable_ = arboretum_node_new_with_id("enum_value", "clang::attr::MSNoVTable", nullptr);
  data_model.clang_attr_MSP430Interrupt_ = arboretum_node_new_with_id("enum_value", "clang::attr::MSP430Interrupt", nullptr);
  data_model.clang_attr_MSStruct_ = arboretum_node_new_with_id("enum_value", "clang::attr::MSStruct", nullptr);
  data_model.clang_attr_MSVtorDisp_ = arboretum_node_new_with_id("enum_value", "clang::attr::MSVtorDisp", nullptr);
  data_model.clang_attr_MaxFieldAlignment_ = arboretum_node_new_with_id("enum_value", "clang::attr::MaxFieldAlignment", nullptr);
  data_model.clang_attr_MayAlias_ = arboretum_node_new_with_id("enum_value", "clang::attr::MayAlias", nullptr);
  data_model.clang_attr_MaybeUndef_ = arboretum_node_new_with_id("enum_value", "clang::attr::MaybeUndef", nullptr);
  data_model.clang_attr_MicroMips_ = arboretum_node_new_with_id("enum_value", "clang::attr::MicroMips", nullptr);
  data_model.clang_attr_MinSize_ = arboretum_node_new_with_id("enum_value", "clang::attr::MinSize", nullptr);
  data_model.clang_attr_MinVectorWidth_ = arboretum_node_new_with_id("enum_value", "clang::attr::MinVectorWidth", nullptr);
  data_model.clang_attr_Mips16_ = arboretum_node_new_with_id("enum_value", "clang::attr::Mips16", nullptr);
  data_model.clang_attr_MipsInterrupt_ = arboretum_node_new_with_id("enum_value", "clang::attr::MipsInterrupt", nullptr);
  data_model.clang_attr_MipsLongCall_ = arboretum_node_new_with_id("enum_value", "clang::attr::MipsLongCall", nullptr);
  data_model.clang_attr_MipsShortCall_ = arboretum_node_new_with_id("enum_value", "clang::attr::MipsShortCall", nullptr);
  data_model.clang_attr_NSConsumesSelf_ = arboretum_node_new_with_id("enum_value", "clang::attr::NSConsumesSelf", nullptr);
  data_model.clang_attr_NSErrorDomain_ = arboretum_node_new_with_id("enum_value", "clang::attr::NSErrorDomain", nullptr);
  data_model.clang_attr_NSReturnsAutoreleased_ = arboretum_node_new_with_id("enum_value", "clang::attr::NSReturnsAutoreleased", nullptr);
  data_model.clang_attr_NSReturnsNotRetained_ = arboretum_node_new_with_id("enum_value", "clang::attr::NSReturnsNotRetained", nullptr);
  data_model.clang_attr_NVPTXKernel_ = arboretum_node_new_with_id("enum_value", "clang::attr::NVPTXKernel", nullptr);
  data_model.clang_attr_Naked_ = arboretum_node_new_with_id("enum_value", "clang::attr::Naked", nullptr);
  data_model.clang_attr_NoAlias_ = arboretum_node_new_with_id("enum_value", "clang::attr::NoAlias", nullptr);
  data_model.clang_attr_NoCommon_ = arboretum_node_new_with_id("enum_value", "clang::attr::NoCommon", nullptr);
  data_model.clang_attr_NoDebug_ = arboretum_node_new_with_id("enum_value", "clang::attr::NoDebug", nullptr);
  data_model.clang_attr_NoDestroy_ = arboretum_node_new_with_id("enum_value", "clang::attr::NoDestroy", nullptr);
  data_model.clang_attr_NoDuplicate_ = arboretum_node_new_with_id("enum_value", "clang::attr::NoDuplicate", nullptr);
  data_model.clang_attr_NoInstrumentFunction_ = arboretum_node_new_with_id("enum_value", "clang::attr::NoInstrumentFunction", nullptr);
  data_model.clang_attr_NoMicroMips_ = arboretum_node_new_with_id("enum_value", "clang::attr::NoMicroMips", nullptr);
  data_model.clang_attr_NoMips16_ = arboretum_node_new_with_id("enum_value", "clang::attr::NoMips16", nullptr);
  data_model.clang_attr_NoProfileFunction_ = arboretum_node_new_with_id("enum_value", "clang::attr::NoProfileFunction", nullptr);
  data_model.clang_attr_NoRandomizeLayout_ = arboretum_node_new_with_id("enum_value", "clang::attr::NoRandomizeLayout", nullptr);
  data_model.clang_attr_NoReturn_ = arboretum_node_new_with_id("enum_value", "clang::attr::NoReturn", nullptr);
  data_model.clang_attr_NoSanitize_ = arboretum_node_new_with_id("enum_value", "clang::attr::NoSanitize", nullptr);
  data_model.clang_attr_NoSpeculativeLoadHardening_ = arboretum_node_new_with_id("enum_value", "clang::attr::NoSpeculativeLoadHardening", nullptr);
  data_model.clang_attr_NoSplitStack_ = arboretum_node_new_with_id("enum_value", "clang::attr::NoSplitStack", nullptr);
  data_model.clang_attr_NoStackProtector_ = arboretum_node_new_with_id("enum_value", "clang::attr::NoStackProtector", nullptr);
  data_model.clang_attr_NoThreadSafetyAnalysis_ = arboretum_node_new_with_id("enum_value", "clang::attr::NoThreadSafetyAnalysis", nullptr);
  data_model.clang_attr_NoThrow_ = arboretum_node_new_with_id("enum_value", "clang::attr::NoThrow", nullptr);
  data_model.clang_attr_NoUniqueAddress_ = arboretum_node_new_with_id("enum_value", "clang::attr::NoUniqueAddress", nullptr);
  data_model.clang_attr_NoUwtable_ = arboretum_node_new_with_id("enum_value", "clang::attr::NoUwtable", nullptr);
  data_model.clang_attr_NotTailCalled_ = arboretum_node_new_with_id("enum_value", "clang::attr::NotTailCalled", nullptr);
  data_model.clang_attr_OMPAllocateDecl_ = arboretum_node_new_with_id("enum_value", "clang::attr::OMPAllocateDecl", nullptr);
  data_model.clang_attr_OMPCaptureNoInit_ = arboretum_node_new_with_id("enum_value", "clang::attr::OMPCaptureNoInit", nullptr);
  data_model.clang_attr_OMPDeclareTargetDecl_ = arboretum_node_new_with_id("enum_value", "clang::attr::OMPDeclareTargetDecl", nullptr);
  data_model.clang_attr_OMPDeclareVariant_ = arboretum_node_new_with_id("enum_value", "clang::attr::OMPDeclareVariant", nullptr);
  data_model.clang_attr_OMPThreadPrivateDecl_ = arboretum_node_new_with_id("enum_value", "clang::attr::OMPThreadPrivateDecl", nullptr);
  data_model.clang_attr_OSConsumesThis_ = arboretum_node_new_with_id("enum_value", "clang::attr::OSConsumesThis", nullptr);
  data_model.clang_attr_OSReturnsNotRetained_ = arboretum_node_new_with_id("enum_value", "clang::attr::OSReturnsNotRetained", nullptr);
  data_model.clang_attr_OSReturnsRetained_ = arboretum_node_new_with_id("enum_value", "clang::attr::OSReturnsRetained", nullptr);
  data_model.clang_attr_OSReturnsRetainedOnNonZero_ = arboretum_node_new_with_id("enum_value", "clang::attr::OSReturnsRetainedOnNonZero", nullptr);
  data_model.clang_attr_OSReturnsRetainedOnZero_ = arboretum_node_new_with_id("enum_value", "clang::attr::OSReturnsRetainedOnZero", nullptr);
  data_model.clang_attr_ObjCBridge_ = arboretum_node_new_with_id("enum_value", "clang::attr::ObjCBridge", nullptr);
  data_model.clang_attr_ObjCBridgeMutable_ = arboretum_node_new_with_id("enum_value", "clang::attr::ObjCBridgeMutable", nullptr);
  data_model.clang_attr_ObjCBridgeRelated_ = arboretum_node_new_with_id("enum_value", "clang::attr::ObjCBridgeRelated", nullptr);
  data_model.clang_attr_ObjCException_ = arboretum_node_new_with_id("enum_value", "clang::attr::ObjCException", nullptr);
  data_model.clang_attr_ObjCExplicitProtocolImpl_ = arboretum_node_new_with_id("enum_value", "clang::attr::ObjCExplicitProtocolImpl", nullptr);
  data_model.clang_attr_ObjCExternallyRetained_ = arboretum_node_new_with_id("enum_value", "clang::attr::ObjCExternallyRetained", nullptr);
  data_model.clang_attr_ObjCIndependentClass_ = arboretum_node_new_with_id("enum_value", "clang::attr::ObjCIndependentClass", nullptr);
  data_model.clang_attr_ObjCMethodFamily_ = arboretum_node_new_with_id("enum_value", "clang::attr::ObjCMethodFamily", nullptr);
  data_model.clang_attr_ObjCNSObject_ = arboretum_node_new_with_id("enum_value", "clang::attr::ObjCNSObject", nullptr);
  data_model.clang_attr_ObjCPreciseLifetime_ = arboretum_node_new_with_id("enum_value", "clang::attr::ObjCPreciseLifetime", nullptr);
  data_model.clang_attr_ObjCRequiresPropertyDefs_ = arboretum_node_new_with_id("enum_value", "clang::attr::ObjCRequiresPropertyDefs", nullptr);
  data_model.clang_attr_ObjCRequiresSuper_ = arboretum_node_new_with_id("enum_value", "clang::attr::ObjCRequiresSuper", nullptr);
  data_model.clang_attr_ObjCReturnsInnerPointer_ = arboretum_node_new_with_id("enum_value", "clang::attr::ObjCReturnsInnerPointer", nullptr);
  data_model.clang_attr_ObjCRootClass_ = arboretum_node_new_with_id("enum_value", "clang::attr::ObjCRootClass", nullptr);
  data_model.clang_attr_ObjCSubclassingRestricted_ = arboretum_node_new_with_id("enum_value", "clang::attr::ObjCSubclassingRestricted", nullptr);
  data_model.clang_attr_OpenCLIntelReqdSubGroupSize_ = arboretum_node_new_with_id("enum_value", "clang::attr::OpenCLIntelReqdSubGroupSize", nullptr);
  data_model.clang_attr_OpenCLKernel_ = arboretum_node_new_with_id("enum_value", "clang::attr::OpenCLKernel", nullptr);
  data_model.clang_attr_OptimizeNone_ = arboretum_node_new_with_id("enum_value", "clang::attr::OptimizeNone", nullptr);
  data_model.clang_attr_Override_ = arboretum_node_new_with_id("enum_value", "clang::attr::Override", nullptr);
  data_model.clang_attr_Owner_ = arboretum_node_new_with_id("enum_value", "clang::attr::Owner", nullptr);
  data_model.clang_attr_Ownership_ = arboretum_node_new_with_id("enum_value", "clang::attr::Ownership", nullptr);
  data_model.clang_attr_Packed_ = arboretum_node_new_with_id("enum_value", "clang::attr::Packed", nullptr);
  data_model.clang_attr_ParamTypestate_ = arboretum_node_new_with_id("enum_value", "clang::attr::ParamTypestate", nullptr);
  data_model.clang_attr_PatchableFunctionEntry_ = arboretum_node_new_with_id("enum_value", "clang::attr::PatchableFunctionEntry", nullptr);
  data_model.clang_attr_Pointer_ = arboretum_node_new_with_id("enum_value", "clang::attr::Pointer", nullptr);
  data_model.clang_attr_PragmaClangBSSSection_ = arboretum_node_new_with_id("enum_value", "clang::attr::PragmaClangBSSSection", nullptr);
  data_model.clang_attr_PragmaClangDataSection_ = arboretum_node_new_with_id("enum_value", "clang::attr::PragmaClangDataSection", nullptr);
  data_model.clang_attr_PragmaClangRelroSection_ = arboretum_node_new_with_id("enum_value", "clang::attr::PragmaClangRelroSection", nullptr);
  data_model.clang_attr_PragmaClangRodataSection_ = arboretum_node_new_with_id("enum_value", "clang::attr::PragmaClangRodataSection", nullptr);
  data_model.clang_attr_PragmaClangTextSection_ = arboretum_node_new_with_id("enum_value", "clang::attr::PragmaClangTextSection", nullptr);
  data_model.clang_attr_PreferredName_ = arboretum_node_new_with_id("enum_value", "clang::attr::PreferredName", nullptr);
  data_model.clang_attr_PtGuardedBy_ = arboretum_node_new_with_id("enum_value", "clang::attr::PtGuardedBy", nullptr);
  data_model.clang_attr_PtGuardedVar_ = arboretum_node_new_with_id("enum_value", "clang::attr::PtGuardedVar", nullptr);
  data_model.clang_attr_Pure_ = arboretum_node_new_with_id("enum_value", "clang::attr::Pure", nullptr);
  data_model.clang_attr_RISCVInterrupt_ = arboretum_node_new_with_id("enum_value", "clang::attr::RISCVInterrupt", nullptr);
  data_model.clang_attr_RandomizeLayout_ = arboretum_node_new_with_id("enum_value", "clang::attr::RandomizeLayout", nullptr);
  data_model.clang_attr_ReadOnlyPlacement_ = arboretum_node_new_with_id("enum_value", "clang::attr::ReadOnlyPlacement", nullptr);
  data_model.clang_attr_Reinitializes_ = arboretum_node_new_with_id("enum_value", "clang::attr::Reinitializes", nullptr);
  data_model.clang_attr_ReleaseCapability_ = arboretum_node_new_with_id("enum_value", "clang::attr::ReleaseCapability", nullptr);
  data_model.clang_attr_ReqdWorkGroupSize_ = arboretum_node_new_with_id("enum_value", "clang::attr::ReqdWorkGroupSize", nullptr);
  data_model.clang_attr_RequiresCapability_ = arboretum_node_new_with_id("enum_value", "clang::attr::RequiresCapability", nullptr);
  data_model.clang_attr_Restrict_ = arboretum_node_new_with_id("enum_value", "clang::attr::Restrict", nullptr);
  data_model.clang_attr_Retain_ = arboretum_node_new_with_id("enum_value", "clang::attr::Retain", nullptr);
  data_model.clang_attr_ReturnTypestate_ = arboretum_node_new_with_id("enum_value", "clang::attr::ReturnTypestate", nullptr);
  data_model.clang_attr_ReturnsNonNull_ = arboretum_node_new_with_id("enum_value", "clang::attr::ReturnsNonNull", nullptr);
  data_model.clang_attr_ReturnsTwice_ = arboretum_node_new_with_id("enum_value", "clang::attr::ReturnsTwice", nullptr);
  data_model.clang_attr_SYCLKernel_ = arboretum_node_new_with_id("enum_value", "clang::attr::SYCLKernel", nullptr);
  data_model.clang_attr_SYCLSpecialClass_ = arboretum_node_new_with_id("enum_value", "clang::attr::SYCLSpecialClass", nullptr);
  data_model.clang_attr_ScopedLockable_ = arboretum_node_new_with_id("enum_value", "clang::attr::ScopedLockable", nullptr);
  data_model.clang_attr_Section_ = arboretum_node_new_with_id("enum_value", "clang::attr::Section", nullptr);
  data_model.clang_attr_SelectAny_ = arboretum_node_new_with_id("enum_value", "clang::attr::SelectAny", nullptr);
  data_model.clang_attr_Sentinel_ = arboretum_node_new_with_id("enum_value", "clang::attr::Sentinel", nullptr);
  data_model.clang_attr_SetTypestate_ = arboretum_node_new_with_id("enum_value", "clang::attr::SetTypestate", nullptr);
  data_model.clang_attr_SharedTrylockFunction_ = arboretum_node_new_with_id("enum_value", "clang::attr::SharedTrylockFunction", nullptr);
  data_model.clang_attr_SpeculativeLoadHardening_ = arboretum_node_new_with_id("enum_value", "clang::attr::SpeculativeLoadHardening", nullptr);
  data_model.clang_attr_StandaloneDebug_ = arboretum_node_new_with_id("enum_value", "clang::attr::StandaloneDebug", nullptr);
  data_model.clang_attr_StrictFP_ = arboretum_node_new_with_id("enum_value", "clang::attr::StrictFP", nullptr);
  data_model.clang_attr_StrictGuardStackCheck_ = arboretum_node_new_with_id("enum_value", "clang::attr::StrictGuardStackCheck", nullptr);
  data_model.clang_attr_SwiftAsync_ = arboretum_node_new_with_id("enum_value", "clang::attr::SwiftAsync", nullptr);
  data_model.clang_attr_SwiftAsyncError_ = arboretum_node_new_with_id("enum_value", "clang::attr::SwiftAsyncError", nullptr);
  data_model.clang_attr_SwiftAsyncName_ = arboretum_node_new_with_id("enum_value", "clang::attr::SwiftAsyncName", nullptr);
  data_model.clang_attr_SwiftAttr_ = arboretum_node_new_with_id("enum_value", "clang::attr::SwiftAttr", nullptr);
  data_model.clang_attr_SwiftBridge_ = arboretum_node_new_with_id("enum_value", "clang::attr::SwiftBridge", nullptr);
  data_model.clang_attr_SwiftBridgedTypedef_ = arboretum_node_new_with_id("enum_value", "clang::attr::SwiftBridgedTypedef", nullptr);
  data_model.clang_attr_SwiftError_ = arboretum_node_new_with_id("enum_value", "clang::attr::SwiftError", nullptr);
  data_model.clang_attr_SwiftName_ = arboretum_node_new_with_id("enum_value", "clang::attr::SwiftName", nullptr);
  data_model.clang_attr_SwiftNewType_ = arboretum_node_new_with_id("enum_value", "clang::attr::SwiftNewType", nullptr);
  data_model.clang_attr_SwiftPrivate_ = arboretum_node_new_with_id("enum_value", "clang::attr::SwiftPrivate", nullptr);
  data_model.clang_attr_TLSModel_ = arboretum_node_new_with_id("enum_value", "clang::attr::TLSModel", nullptr);
  data_model.clang_attr_Target_ = arboretum_node_new_with_id("enum_value", "clang::attr::Target", nullptr);
  data_model.clang_attr_TargetClones_ = arboretum_node_new_with_id("enum_value", "clang::attr::TargetClones", nullptr);
  data_model.clang_attr_TargetVersion_ = arboretum_node_new_with_id("enum_value", "clang::attr::TargetVersion", nullptr);
  data_model.clang_attr_TestTypestate_ = arboretum_node_new_with_id("enum_value", "clang::attr::TestTypestate", nullptr);
  data_model.clang_attr_TransparentUnion_ = arboretum_node_new_with_id("enum_value", "clang::attr::TransparentUnion", nullptr);
  data_model.clang_attr_TrivialABI_ = arboretum_node_new_with_id("enum_value", "clang::attr::TrivialABI", nullptr);
  data_model.clang_attr_TryAcquireCapability_ = arboretum_node_new_with_id("enum_value", "clang::attr::TryAcquireCapability", nullptr);
  data_model.clang_attr_TypeTagForDatatype_ = arboretum_node_new_with_id("enum_value", "clang::attr::TypeTagForDatatype", nullptr);
  data_model.clang_attr_TypeVisibility_ = arboretum_node_new_with_id("enum_value", "clang::attr::TypeVisibility", nullptr);
  data_model.clang_attr_Unavailable_ = arboretum_node_new_with_id("enum_value", "clang::attr::Unavailable", nullptr);
  data_model.clang_attr_Uninitialized_ = arboretum_node_new_with_id("enum_value", "clang::attr::Uninitialized", nullptr);
  data_model.clang_attr_UnsafeBufferUsage_ = arboretum_node_new_with_id("enum_value", "clang::attr::UnsafeBufferUsage", nullptr);
  data_model.clang_attr_Unused_ = arboretum_node_new_with_id("enum_value", "clang::attr::Unused", nullptr);
  data_model.clang_attr_Used_ = arboretum_node_new_with_id("enum_value", "clang::attr::Used", nullptr);
  data_model.clang_attr_UsingIfExists_ = arboretum_node_new_with_id("enum_value", "clang::attr::UsingIfExists", nullptr);
  data_model.clang_attr_Uuid_ = arboretum_node_new_with_id("enum_value", "clang::attr::Uuid", nullptr);
  data_model.clang_attr_VecReturn_ = arboretum_node_new_with_id("enum_value", "clang::attr::VecReturn", nullptr);
  data_model.clang_attr_VecTypeHint_ = arboretum_node_new_with_id("enum_value", "clang::attr::VecTypeHint", nullptr);
  data_model.clang_attr_Visibility_ = arboretum_node_new_with_id("enum_value", "clang::attr::Visibility", nullptr);
  data_model.clang_attr_WarnUnused_ = arboretum_node_new_with_id("enum_value", "clang::attr::WarnUnused", nullptr);
  data_model.clang_attr_WarnUnusedResult_ = arboretum_node_new_with_id("enum_value", "clang::attr::WarnUnusedResult", nullptr);
  data_model.clang_attr_Weak_ = arboretum_node_new_with_id("enum_value", "clang::attr::Weak", nullptr);
  data_model.clang_attr_WeakImport_ = arboretum_node_new_with_id("enum_value", "clang::attr::WeakImport", nullptr);
  data_model.clang_attr_WeakRef_ = arboretum_node_new_with_id("enum_value", "clang::attr::WeakRef", nullptr);
  data_model.clang_attr_WebAssemblyExportName_ = arboretum_node_new_with_id("enum_value", "clang::attr::WebAssemblyExportName", nullptr);
  data_model.clang_attr_WebAssemblyImportModule_ = arboretum_node_new_with_id("enum_value", "clang::attr::WebAssemblyImportModule", nullptr);
  data_model.clang_attr_WebAssemblyImportName_ = arboretum_node_new_with_id("enum_value", "clang::attr::WebAssemblyImportName", nullptr);
  data_model.clang_attr_WorkGroupSizeHint_ = arboretum_node_new_with_id("enum_value", "clang::attr::WorkGroupSizeHint", nullptr);
  data_model.clang_attr_X86ForceAlignArgPointer_ = arboretum_node_new_with_id("enum_value", "clang::attr::X86ForceAlignArgPointer", nullptr);
  data_model.clang_attr_XRayInstrument_ = arboretum_node_new_with_id("enum_value", "clang::attr::XRayInstrument", nullptr);
  data_model.clang_attr_XRayLogArgs_ = arboretum_node_new_with_id("enum_value", "clang::attr::XRayLogArgs", nullptr);
  data_model.clang_attr_ZeroCallUsedRegs_ = arboretum_node_new_with_id("enum_value", "clang::attr::ZeroCallUsedRegs", nullptr);
  data_model.clang_attr_AbiTag_ = arboretum_node_new_with_id("enum_value", "clang::attr::AbiTag", nullptr);
  data_model.clang_attr_Alias_ = arboretum_node_new_with_id("enum_value", "clang::attr::Alias", nullptr);
  data_model.clang_attr_AlignValue_ = arboretum_node_new_with_id("enum_value", "clang::attr::AlignValue", nullptr);
  data_model.clang_attr_BuiltinAlias_ = arboretum_node_new_with_id("enum_value", "clang::attr::BuiltinAlias", nullptr);
  data_model.clang_attr_CalledOnce_ = arboretum_node_new_with_id("enum_value", "clang::attr::CalledOnce", nullptr);
  data_model.clang_attr_IFunc_ = arboretum_node_new_with_id("enum_value", "clang::attr::IFunc", nullptr);
  data_model.clang_attr_InitSeg_ = arboretum_node_new_with_id("enum_value", "clang::attr::InitSeg", nullptr);
  data_model.clang_attr_LoaderUninitialized_ = arboretum_node_new_with_id("enum_value", "clang::attr::LoaderUninitialized", nullptr);
  data_model.clang_attr_LoopHint_ = arboretum_node_new_with_id("enum_value", "clang::attr::LoopHint", nullptr);
  data_model.clang_attr_Mode_ = arboretum_node_new_with_id("enum_value", "clang::attr::Mode", nullptr);
  data_model.clang_attr_NoBuiltin_ = arboretum_node_new_with_id("enum_value", "clang::attr::NoBuiltin", nullptr);
  data_model.clang_attr_NoEscape_ = arboretum_node_new_with_id("enum_value", "clang::attr::NoEscape", nullptr);
  data_model.clang_attr_OMPCaptureKind_ = arboretum_node_new_with_id("enum_value", "clang::attr::OMPCaptureKind", nullptr);
  data_model.clang_attr_OMPDeclareSimdDecl_ = arboretum_node_new_with_id("enum_value", "clang::attr::OMPDeclareSimdDecl", nullptr);
  data_model.clang_attr_OMPReferencedVar_ = arboretum_node_new_with_id("enum_value", "clang::attr::OMPReferencedVar", nullptr);
  data_model.clang_attr_ObjCBoxable_ = arboretum_node_new_with_id("enum_value", "clang::attr::ObjCBoxable", nullptr);
  data_model.clang_attr_ObjCClassStub_ = arboretum_node_new_with_id("enum_value", "clang::attr::ObjCClassStub", nullptr);
  data_model.clang_attr_ObjCDesignatedInitializer_ = arboretum_node_new_with_id("enum_value", "clang::attr::ObjCDesignatedInitializer", nullptr);
  data_model.clang_attr_ObjCDirect_ = arboretum_node_new_with_id("enum_value", "clang::attr::ObjCDirect", nullptr);
  data_model.clang_attr_ObjCDirectMembers_ = arboretum_node_new_with_id("enum_value", "clang::attr::ObjCDirectMembers", nullptr);
  data_model.clang_attr_ObjCNonLazyClass_ = arboretum_node_new_with_id("enum_value", "clang::attr::ObjCNonLazyClass", nullptr);
  data_model.clang_attr_ObjCNonRuntimeProtocol_ = arboretum_node_new_with_id("enum_value", "clang::attr::ObjCNonRuntimeProtocol", nullptr);
  data_model.clang_attr_ObjCRuntimeName_ = arboretum_node_new_with_id("enum_value", "clang::attr::ObjCRuntimeName", nullptr);
  data_model.clang_attr_ObjCRuntimeVisible_ = arboretum_node_new_with_id("enum_value", "clang::attr::ObjCRuntimeVisible", nullptr);
  data_model.clang_attr_OpenCLAccess_ = arboretum_node_new_with_id("enum_value", "clang::attr::OpenCLAccess", nullptr);
  data_model.clang_attr_Overloadable_ = arboretum_node_new_with_id("enum_value", "clang::attr::Overloadable", nullptr);
  data_model.clang_attr_RenderScriptKernel_ = arboretum_node_new_with_id("enum_value", "clang::attr::RenderScriptKernel", nullptr);
  data_model.clang_attr_SwiftObjCMembers_ = arboretum_node_new_with_id("enum_value", "clang::attr::SwiftObjCMembers", nullptr);
  data_model.clang_attr_Thread_ = arboretum_node_new_with_id("enum_value", "clang::attr::Thread", nullptr);
  data_model.clang_attr_FirstAttr_ = arboretum_node_new_with_id("enum_value", "clang::attr::FirstAttr", nullptr);
  data_model.clang_attr_LastAttr_ = arboretum_node_new_with_id("enum_value", "clang::attr::LastAttr", nullptr);
  data_model.clang_attr_FirstTypeAttr_ = arboretum_node_new_with_id("enum_value", "clang::attr::FirstTypeAttr", nullptr);
  data_model.clang_attr_LastTypeAttr_ = arboretum_node_new_with_id("enum_value", "clang::attr::LastTypeAttr", nullptr);
  data_model.clang_attr_FirstStmtAttr_ = arboretum_node_new_with_id("enum_value", "clang::attr::FirstStmtAttr", nullptr);
  data_model.clang_attr_LastStmtAttr_ = arboretum_node_new_with_id("enum_value", "clang::attr::LastStmtAttr", nullptr);
  data_model.clang_attr_FirstDeclOrStmtAttr_ = arboretum_node_new_with_id("enum_value", "clang::attr::FirstDeclOrStmtAttr", nullptr);
  data_model.clang_attr_LastDeclOrStmtAttr_ = arboretum_node_new_with_id("enum_value", "clang::attr::LastDeclOrStmtAttr", nullptr);
  data_model.clang_attr_FirstInheritableAttr_ = arboretum_node_new_with_id("enum_value", "clang::attr::FirstInheritableAttr", nullptr);
  data_model.clang_attr_LastInheritableAttr_ = arboretum_node_new_with_id("enum_value", "clang::attr::LastInheritableAttr", nullptr);
  data_model.clang_attr_FirstDeclOrTypeAttr_ = arboretum_node_new_with_id("enum_value", "clang::attr::FirstDeclOrTypeAttr", nullptr);
  data_model.clang_attr_LastDeclOrTypeAttr_ = arboretum_node_new_with_id("enum_value", "clang::attr::LastDeclOrTypeAttr", nullptr);
  data_model.clang_attr_FirstInheritableParamAttr_ = arboretum_node_new_with_id("enum_value", "clang::attr::FirstInheritableParamAttr", nullptr);
  data_model.clang_attr_LastInheritableParamAttr_ = arboretum_node_new_with_id("enum_value", "clang::attr::LastInheritableParamAttr", nullptr);
  data_model.clang_attr_FirstParameterABIAttr_ = arboretum_node_new_with_id("enum_value", "clang::attr::FirstParameterABIAttr", nullptr);
  data_model.clang_attr_LastParameterABIAttr_ = arboretum_node_new_with_id("enum_value", "clang::attr::LastParameterABIAttr", nullptr);
  data_model.clang_attr_FirstHLSLAnnotationAttr_ = arboretum_node_new_with_id("enum_value", "clang::attr::FirstHLSLAnnotationAttr", nullptr);
  data_model.clang_attr_LastHLSLAnnotationAttr_ = arboretum_node_new_with_id("enum_value", "clang::attr::LastHLSLAnnotationAttr", nullptr);

  // llvm::APFloatBase::Semantics
  data_model.llvm_APFloatBase_Semantics_ = arboretum_node_new_with_id("enum", "llvm::APFloatBase::Semantics", nullptr);
  data_model.llvm_APFloatBase_S_IEEEhalf_ = arboretum_node_new_with_id("enum_value", "llvm::APFloatBase::S_IEEEhalf", nullptr);
  data_model.llvm_APFloatBase_S_BFloat_ = arboretum_node_new_with_id("enum_value", "llvm::APFloatBase::S_BFloat", nullptr);
  data_model.llvm_APFloatBase_S_IEEEsingle_ = arboretum_node_new_with_id("enum_value", "llvm::APFloatBase::S_IEEEsingle", nullptr);
  data_model.llvm_APFloatBase_S_IEEEdouble_ = arboretum_node_new_with_id("enum_value", "llvm::APFloatBase::S_IEEEdouble", nullptr);
  data_model.llvm_APFloatBase_S_IEEEquad_ = arboretum_node_new_with_id("enum_value", "llvm::APFloatBase::S_IEEEquad", nullptr);
  data_model.llvm_APFloatBase_S_PPCDoubleDouble_ = arboretum_node_new_with_id("enum_value", "llvm::APFloatBase::S_PPCDoubleDouble", nullptr);
  data_model.llvm_APFloatBase_S_Float8E5M2_ = arboretum_node_new_with_id("enum_value", "llvm::APFloatBase::S_Float8E5M2", nullptr);
  data_model.llvm_APFloatBase_S_Float8E5M2FNUZ_ = arboretum_node_new_with_id("enum_value", "llvm::APFloatBase::S_Float8E5M2FNUZ", nullptr);
  data_model.llvm_APFloatBase_S_Float8E4M3FN_ = arboretum_node_new_with_id("enum_value", "llvm::APFloatBase::S_Float8E4M3FN", nullptr);
  data_model.llvm_APFloatBase_S_Float8E4M3FNUZ_ = arboretum_node_new_with_id("enum_value", "llvm::APFloatBase::S_Float8E4M3FNUZ", nullptr);
  data_model.llvm_APFloatBase_S_Float8E4M3B11FNUZ_ = arboretum_node_new_with_id("enum_value", "llvm::APFloatBase::S_Float8E4M3B11FNUZ", nullptr);
  data_model.llvm_APFloatBase_S_FloatTF32_ = arboretum_node_new_with_id("enum_value", "llvm::APFloatBase::S_FloatTF32", nullptr);
  data_model.llvm_APFloatBase_S_x87DoubleExtended_ = arboretum_node_new_with_id("enum_value", "llvm::APFloatBase::S_x87DoubleExtended", nullptr);
  data_model.llvm_APFloatBase_S_MaxSemantics_ = arboretum_node_new_with_id("enum_value", "llvm::APFloatBase::S_MaxSemantics", nullptr);

  return data_model;
}

Thing* DataModel::resolve(clang::APValue::ValueKind e) {
  switch(e) {
    case clang::APValue::None: return clang_APValue_None_;
    case clang::APValue::Indeterminate: return clang_APValue_Indeterminate_;
    case clang::APValue::Int: return clang_APValue_Int_;
    case clang::APValue::Float: return clang_APValue_Float_;
    case clang::APValue::FixedPoint: return clang_APValue_FixedPoint_;
    case clang::APValue::ComplexInt: return clang_APValue_ComplexInt_;
    case clang::APValue::ComplexFloat: return clang_APValue_ComplexFloat_;
    case clang::APValue::LValue: return clang_APValue_LValue_;
    case clang::APValue::Vector: return clang_APValue_Vector_;
    case clang::APValue::Array: return clang_APValue_Array_;
    case clang::APValue::Struct: return clang_APValue_Struct_;
    case clang::APValue::Union: return clang_APValue_Union_;
    case clang::APValue::MemberPointer: return clang_APValue_MemberPointer_;
    case clang::APValue::AddrLabelDiff: return clang_APValue_AddrLabelDiff_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::AccessSpecifier e) {
  switch(e) {
    case clang::AS_public: return clang_AS_public_;
    case clang::AS_protected: return clang_AS_protected_;
    case clang::AS_private: return clang_AS_private_;
    case clang::AS_none: return clang_AS_none_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::ArrayType::ArraySizeModifier e) {
  switch(e) {
    case clang::ArrayType::Normal: return clang_ArrayType_Normal_;
    case clang::ArrayType::Static: return clang_ArrayType_Static_;
    case clang::ArrayType::Star: return clang_ArrayType_Star_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::ArrayTypeTrait e) {
  switch(e) {
    case clang::ATT_ArrayRank: return clang_ATT_ArrayRank_;
    case clang::ATT_ArrayExtent: return clang_ATT_ArrayExtent_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::AtomicExpr::AtomicOp e) {
  switch(e) {
    case clang::AtomicExpr::AO__c11_atomic_init: return clang_AtomicExpr_AO__c11_atomic_init_;
    case clang::AtomicExpr::AO__c11_atomic_load: return clang_AtomicExpr_AO__c11_atomic_load_;
    case clang::AtomicExpr::AO__c11_atomic_store: return clang_AtomicExpr_AO__c11_atomic_store_;
    case clang::AtomicExpr::AO__c11_atomic_exchange: return clang_AtomicExpr_AO__c11_atomic_exchange_;
    case clang::AtomicExpr::AO__c11_atomic_compare_exchange_strong: return clang_AtomicExpr_AO__c11_atomic_compare_exchange_strong_;
    case clang::AtomicExpr::AO__c11_atomic_compare_exchange_weak: return clang_AtomicExpr_AO__c11_atomic_compare_exchange_weak_;
    case clang::AtomicExpr::AO__c11_atomic_fetch_add: return clang_AtomicExpr_AO__c11_atomic_fetch_add_;
    case clang::AtomicExpr::AO__c11_atomic_fetch_sub: return clang_AtomicExpr_AO__c11_atomic_fetch_sub_;
    case clang::AtomicExpr::AO__c11_atomic_fetch_and: return clang_AtomicExpr_AO__c11_atomic_fetch_and_;
    case clang::AtomicExpr::AO__c11_atomic_fetch_or: return clang_AtomicExpr_AO__c11_atomic_fetch_or_;
    case clang::AtomicExpr::AO__c11_atomic_fetch_xor: return clang_AtomicExpr_AO__c11_atomic_fetch_xor_;
    case clang::AtomicExpr::AO__c11_atomic_fetch_nand: return clang_AtomicExpr_AO__c11_atomic_fetch_nand_;
    case clang::AtomicExpr::AO__c11_atomic_fetch_max: return clang_AtomicExpr_AO__c11_atomic_fetch_max_;
    case clang::AtomicExpr::AO__c11_atomic_fetch_min: return clang_AtomicExpr_AO__c11_atomic_fetch_min_;
    case clang::AtomicExpr::AO__atomic_load: return clang_AtomicExpr_AO__atomic_load_;
    case clang::AtomicExpr::AO__atomic_load_n: return clang_AtomicExpr_AO__atomic_load_n_;
    case clang::AtomicExpr::AO__atomic_store: return clang_AtomicExpr_AO__atomic_store_;
    case clang::AtomicExpr::AO__atomic_store_n: return clang_AtomicExpr_AO__atomic_store_n_;
    case clang::AtomicExpr::AO__atomic_exchange: return clang_AtomicExpr_AO__atomic_exchange_;
    case clang::AtomicExpr::AO__atomic_exchange_n: return clang_AtomicExpr_AO__atomic_exchange_n_;
    case clang::AtomicExpr::AO__atomic_compare_exchange: return clang_AtomicExpr_AO__atomic_compare_exchange_;
    case clang::AtomicExpr::AO__atomic_compare_exchange_n: return clang_AtomicExpr_AO__atomic_compare_exchange_n_;
    case clang::AtomicExpr::AO__atomic_fetch_add: return clang_AtomicExpr_AO__atomic_fetch_add_;
    case clang::AtomicExpr::AO__atomic_fetch_sub: return clang_AtomicExpr_AO__atomic_fetch_sub_;
    case clang::AtomicExpr::AO__atomic_fetch_and: return clang_AtomicExpr_AO__atomic_fetch_and_;
    case clang::AtomicExpr::AO__atomic_fetch_or: return clang_AtomicExpr_AO__atomic_fetch_or_;
    case clang::AtomicExpr::AO__atomic_fetch_xor: return clang_AtomicExpr_AO__atomic_fetch_xor_;
    case clang::AtomicExpr::AO__atomic_fetch_nand: return clang_AtomicExpr_AO__atomic_fetch_nand_;
    case clang::AtomicExpr::AO__atomic_add_fetch: return clang_AtomicExpr_AO__atomic_add_fetch_;
    case clang::AtomicExpr::AO__atomic_sub_fetch: return clang_AtomicExpr_AO__atomic_sub_fetch_;
    case clang::AtomicExpr::AO__atomic_and_fetch: return clang_AtomicExpr_AO__atomic_and_fetch_;
    case clang::AtomicExpr::AO__atomic_or_fetch: return clang_AtomicExpr_AO__atomic_or_fetch_;
    case clang::AtomicExpr::AO__atomic_xor_fetch: return clang_AtomicExpr_AO__atomic_xor_fetch_;
    case clang::AtomicExpr::AO__atomic_max_fetch: return clang_AtomicExpr_AO__atomic_max_fetch_;
    case clang::AtomicExpr::AO__atomic_min_fetch: return clang_AtomicExpr_AO__atomic_min_fetch_;
    case clang::AtomicExpr::AO__atomic_nand_fetch: return clang_AtomicExpr_AO__atomic_nand_fetch_;
    case clang::AtomicExpr::AO__opencl_atomic_init: return clang_AtomicExpr_AO__opencl_atomic_init_;
    case clang::AtomicExpr::AO__opencl_atomic_load: return clang_AtomicExpr_AO__opencl_atomic_load_;
    case clang::AtomicExpr::AO__opencl_atomic_store: return clang_AtomicExpr_AO__opencl_atomic_store_;
    case clang::AtomicExpr::AO__opencl_atomic_exchange: return clang_AtomicExpr_AO__opencl_atomic_exchange_;
    case clang::AtomicExpr::AO__opencl_atomic_compare_exchange_strong: return clang_AtomicExpr_AO__opencl_atomic_compare_exchange_strong_;
    case clang::AtomicExpr::AO__opencl_atomic_compare_exchange_weak: return clang_AtomicExpr_AO__opencl_atomic_compare_exchange_weak_;
    case clang::AtomicExpr::AO__opencl_atomic_fetch_add: return clang_AtomicExpr_AO__opencl_atomic_fetch_add_;
    case clang::AtomicExpr::AO__opencl_atomic_fetch_sub: return clang_AtomicExpr_AO__opencl_atomic_fetch_sub_;
    case clang::AtomicExpr::AO__opencl_atomic_fetch_and: return clang_AtomicExpr_AO__opencl_atomic_fetch_and_;
    case clang::AtomicExpr::AO__opencl_atomic_fetch_or: return clang_AtomicExpr_AO__opencl_atomic_fetch_or_;
    case clang::AtomicExpr::AO__opencl_atomic_fetch_xor: return clang_AtomicExpr_AO__opencl_atomic_fetch_xor_;
    case clang::AtomicExpr::AO__opencl_atomic_fetch_min: return clang_AtomicExpr_AO__opencl_atomic_fetch_min_;
    case clang::AtomicExpr::AO__opencl_atomic_fetch_max: return clang_AtomicExpr_AO__opencl_atomic_fetch_max_;
    case clang::AtomicExpr::AO__atomic_fetch_min: return clang_AtomicExpr_AO__atomic_fetch_min_;
    case clang::AtomicExpr::AO__atomic_fetch_max: return clang_AtomicExpr_AO__atomic_fetch_max_;
    case clang::AtomicExpr::AO__hip_atomic_load: return clang_AtomicExpr_AO__hip_atomic_load_;
    case clang::AtomicExpr::AO__hip_atomic_store: return clang_AtomicExpr_AO__hip_atomic_store_;
    case clang::AtomicExpr::AO__hip_atomic_compare_exchange_weak: return clang_AtomicExpr_AO__hip_atomic_compare_exchange_weak_;
    case clang::AtomicExpr::AO__hip_atomic_compare_exchange_strong: return clang_AtomicExpr_AO__hip_atomic_compare_exchange_strong_;
    case clang::AtomicExpr::AO__hip_atomic_exchange: return clang_AtomicExpr_AO__hip_atomic_exchange_;
    case clang::AtomicExpr::AO__hip_atomic_fetch_add: return clang_AtomicExpr_AO__hip_atomic_fetch_add_;
    case clang::AtomicExpr::AO__hip_atomic_fetch_sub: return clang_AtomicExpr_AO__hip_atomic_fetch_sub_;
    case clang::AtomicExpr::AO__hip_atomic_fetch_and: return clang_AtomicExpr_AO__hip_atomic_fetch_and_;
    case clang::AtomicExpr::AO__hip_atomic_fetch_or: return clang_AtomicExpr_AO__hip_atomic_fetch_or_;
    case clang::AtomicExpr::AO__hip_atomic_fetch_xor: return clang_AtomicExpr_AO__hip_atomic_fetch_xor_;
    case clang::AtomicExpr::AO__hip_atomic_fetch_min: return clang_AtomicExpr_AO__hip_atomic_fetch_min_;
    case clang::AtomicExpr::AO__hip_atomic_fetch_max: return clang_AtomicExpr_AO__hip_atomic_fetch_max_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::AutoTypeKeyword e) {
  switch(e) {
    case clang::AutoTypeKeyword::Auto: return clang_AutoTypeKeyword_Auto_;
    case clang::AutoTypeKeyword::DecltypeAuto: return clang_AutoTypeKeyword_DecltypeAuto_;
    case clang::AutoTypeKeyword::GNUAutoType: return clang_AutoTypeKeyword_GNUAutoType_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::BinaryOperatorKind e) {
  switch(e) {
    case clang::BO_PtrMemD: return clang_BO_PtrMemD_;
    case clang::BO_PtrMemI: return clang_BO_PtrMemI_;
    case clang::BO_Mul: return clang_BO_Mul_;
    case clang::BO_Div: return clang_BO_Div_;
    case clang::BO_Rem: return clang_BO_Rem_;
    case clang::BO_Add: return clang_BO_Add_;
    case clang::BO_Sub: return clang_BO_Sub_;
    case clang::BO_Shl: return clang_BO_Shl_;
    case clang::BO_Shr: return clang_BO_Shr_;
    case clang::BO_Cmp: return clang_BO_Cmp_;
    case clang::BO_LT: return clang_BO_LT_;
    case clang::BO_GT: return clang_BO_GT_;
    case clang::BO_LE: return clang_BO_LE_;
    case clang::BO_GE: return clang_BO_GE_;
    case clang::BO_EQ: return clang_BO_EQ_;
    case clang::BO_NE: return clang_BO_NE_;
    case clang::BO_And: return clang_BO_And_;
    case clang::BO_Xor: return clang_BO_Xor_;
    case clang::BO_Or: return clang_BO_Or_;
    case clang::BO_LAnd: return clang_BO_LAnd_;
    case clang::BO_LOr: return clang_BO_LOr_;
    case clang::BO_Assign: return clang_BO_Assign_;
    case clang::BO_MulAssign: return clang_BO_MulAssign_;
    case clang::BO_DivAssign: return clang_BO_DivAssign_;
    case clang::BO_RemAssign: return clang_BO_RemAssign_;
    case clang::BO_AddAssign: return clang_BO_AddAssign_;
    case clang::BO_SubAssign: return clang_BO_SubAssign_;
    case clang::BO_ShlAssign: return clang_BO_ShlAssign_;
    case clang::BO_ShrAssign: return clang_BO_ShrAssign_;
    case clang::BO_AndAssign: return clang_BO_AndAssign_;
    case clang::BO_XorAssign: return clang_BO_XorAssign_;
    case clang::BO_OrAssign: return clang_BO_OrAssign_;
    case clang::BO_Comma: return clang_BO_Comma_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::BuiltinTemplateKind e) {
  switch(e) {
    case clang::BTK__make_integer_seq: return clang_BTK__make_integer_seq_;
    case clang::BTK__type_pack_element: return clang_BTK__type_pack_element_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::BuiltinType::Kind e) {
  switch(e) {
    case clang::BuiltinType::OCLImage1dRO: return clang_BuiltinType_OCLImage1dRO_;
    case clang::BuiltinType::OCLImage1dArrayRO: return clang_BuiltinType_OCLImage1dArrayRO_;
    case clang::BuiltinType::OCLImage1dBufferRO: return clang_BuiltinType_OCLImage1dBufferRO_;
    case clang::BuiltinType::OCLImage2dRO: return clang_BuiltinType_OCLImage2dRO_;
    case clang::BuiltinType::OCLImage2dArrayRO: return clang_BuiltinType_OCLImage2dArrayRO_;
    case clang::BuiltinType::OCLImage2dDepthRO: return clang_BuiltinType_OCLImage2dDepthRO_;
    case clang::BuiltinType::OCLImage2dArrayDepthRO: return clang_BuiltinType_OCLImage2dArrayDepthRO_;
    case clang::BuiltinType::OCLImage2dMSAARO: return clang_BuiltinType_OCLImage2dMSAARO_;
    case clang::BuiltinType::OCLImage2dArrayMSAARO: return clang_BuiltinType_OCLImage2dArrayMSAARO_;
    case clang::BuiltinType::OCLImage2dMSAADepthRO: return clang_BuiltinType_OCLImage2dMSAADepthRO_;
    case clang::BuiltinType::OCLImage2dArrayMSAADepthRO: return clang_BuiltinType_OCLImage2dArrayMSAADepthRO_;
    case clang::BuiltinType::OCLImage3dRO: return clang_BuiltinType_OCLImage3dRO_;
    case clang::BuiltinType::OCLImage1dWO: return clang_BuiltinType_OCLImage1dWO_;
    case clang::BuiltinType::OCLImage1dArrayWO: return clang_BuiltinType_OCLImage1dArrayWO_;
    case clang::BuiltinType::OCLImage1dBufferWO: return clang_BuiltinType_OCLImage1dBufferWO_;
    case clang::BuiltinType::OCLImage2dWO: return clang_BuiltinType_OCLImage2dWO_;
    case clang::BuiltinType::OCLImage2dArrayWO: return clang_BuiltinType_OCLImage2dArrayWO_;
    case clang::BuiltinType::OCLImage2dDepthWO: return clang_BuiltinType_OCLImage2dDepthWO_;
    case clang::BuiltinType::OCLImage2dArrayDepthWO: return clang_BuiltinType_OCLImage2dArrayDepthWO_;
    case clang::BuiltinType::OCLImage2dMSAAWO: return clang_BuiltinType_OCLImage2dMSAAWO_;
    case clang::BuiltinType::OCLImage2dArrayMSAAWO: return clang_BuiltinType_OCLImage2dArrayMSAAWO_;
    case clang::BuiltinType::OCLImage2dMSAADepthWO: return clang_BuiltinType_OCLImage2dMSAADepthWO_;
    case clang::BuiltinType::OCLImage2dArrayMSAADepthWO: return clang_BuiltinType_OCLImage2dArrayMSAADepthWO_;
    case clang::BuiltinType::OCLImage3dWO: return clang_BuiltinType_OCLImage3dWO_;
    case clang::BuiltinType::OCLImage1dRW: return clang_BuiltinType_OCLImage1dRW_;
    case clang::BuiltinType::OCLImage1dArrayRW: return clang_BuiltinType_OCLImage1dArrayRW_;
    case clang::BuiltinType::OCLImage1dBufferRW: return clang_BuiltinType_OCLImage1dBufferRW_;
    case clang::BuiltinType::OCLImage2dRW: return clang_BuiltinType_OCLImage2dRW_;
    case clang::BuiltinType::OCLImage2dArrayRW: return clang_BuiltinType_OCLImage2dArrayRW_;
    case clang::BuiltinType::OCLImage2dDepthRW: return clang_BuiltinType_OCLImage2dDepthRW_;
    case clang::BuiltinType::OCLImage2dArrayDepthRW: return clang_BuiltinType_OCLImage2dArrayDepthRW_;
    case clang::BuiltinType::OCLImage2dMSAARW: return clang_BuiltinType_OCLImage2dMSAARW_;
    case clang::BuiltinType::OCLImage2dArrayMSAARW: return clang_BuiltinType_OCLImage2dArrayMSAARW_;
    case clang::BuiltinType::OCLImage2dMSAADepthRW: return clang_BuiltinType_OCLImage2dMSAADepthRW_;
    case clang::BuiltinType::OCLImage2dArrayMSAADepthRW: return clang_BuiltinType_OCLImage2dArrayMSAADepthRW_;
    case clang::BuiltinType::OCLImage3dRW: return clang_BuiltinType_OCLImage3dRW_;
    case clang::BuiltinType::OCLIntelSubgroupAVCMcePayload: return clang_BuiltinType_OCLIntelSubgroupAVCMcePayload_;
    case clang::BuiltinType::OCLIntelSubgroupAVCImePayload: return clang_BuiltinType_OCLIntelSubgroupAVCImePayload_;
    case clang::BuiltinType::OCLIntelSubgroupAVCRefPayload: return clang_BuiltinType_OCLIntelSubgroupAVCRefPayload_;
    case clang::BuiltinType::OCLIntelSubgroupAVCSicPayload: return clang_BuiltinType_OCLIntelSubgroupAVCSicPayload_;
    case clang::BuiltinType::OCLIntelSubgroupAVCMceResult: return clang_BuiltinType_OCLIntelSubgroupAVCMceResult_;
    case clang::BuiltinType::OCLIntelSubgroupAVCImeResult: return clang_BuiltinType_OCLIntelSubgroupAVCImeResult_;
    case clang::BuiltinType::OCLIntelSubgroupAVCRefResult: return clang_BuiltinType_OCLIntelSubgroupAVCRefResult_;
    case clang::BuiltinType::OCLIntelSubgroupAVCSicResult: return clang_BuiltinType_OCLIntelSubgroupAVCSicResult_;
    case clang::BuiltinType::OCLIntelSubgroupAVCImeResultSingleReferenceStreamout: return clang_BuiltinType_OCLIntelSubgroupAVCImeResultSingleReferenceStreamout_;
    case clang::BuiltinType::OCLIntelSubgroupAVCImeResultDualReferenceStreamout: return clang_BuiltinType_OCLIntelSubgroupAVCImeResultDualReferenceStreamout_;
    case clang::BuiltinType::OCLIntelSubgroupAVCImeSingleReferenceStreamin: return clang_BuiltinType_OCLIntelSubgroupAVCImeSingleReferenceStreamin_;
    case clang::BuiltinType::OCLIntelSubgroupAVCImeDualReferenceStreamin: return clang_BuiltinType_OCLIntelSubgroupAVCImeDualReferenceStreamin_;
    case clang::BuiltinType::SveInt8: return clang_BuiltinType_SveInt8_;
    case clang::BuiltinType::SveInt16: return clang_BuiltinType_SveInt16_;
    case clang::BuiltinType::SveInt32: return clang_BuiltinType_SveInt32_;
    case clang::BuiltinType::SveInt64: return clang_BuiltinType_SveInt64_;
    case clang::BuiltinType::SveUint8: return clang_BuiltinType_SveUint8_;
    case clang::BuiltinType::SveUint16: return clang_BuiltinType_SveUint16_;
    case clang::BuiltinType::SveUint32: return clang_BuiltinType_SveUint32_;
    case clang::BuiltinType::SveUint64: return clang_BuiltinType_SveUint64_;
    case clang::BuiltinType::SveFloat16: return clang_BuiltinType_SveFloat16_;
    case clang::BuiltinType::SveFloat32: return clang_BuiltinType_SveFloat32_;
    case clang::BuiltinType::SveFloat64: return clang_BuiltinType_SveFloat64_;
    case clang::BuiltinType::SveBFloat16: return clang_BuiltinType_SveBFloat16_;
    case clang::BuiltinType::SveInt8x2: return clang_BuiltinType_SveInt8x2_;
    case clang::BuiltinType::SveInt16x2: return clang_BuiltinType_SveInt16x2_;
    case clang::BuiltinType::SveInt32x2: return clang_BuiltinType_SveInt32x2_;
    case clang::BuiltinType::SveInt64x2: return clang_BuiltinType_SveInt64x2_;
    case clang::BuiltinType::SveUint8x2: return clang_BuiltinType_SveUint8x2_;
    case clang::BuiltinType::SveUint16x2: return clang_BuiltinType_SveUint16x2_;
    case clang::BuiltinType::SveUint32x2: return clang_BuiltinType_SveUint32x2_;
    case clang::BuiltinType::SveUint64x2: return clang_BuiltinType_SveUint64x2_;
    case clang::BuiltinType::SveFloat16x2: return clang_BuiltinType_SveFloat16x2_;
    case clang::BuiltinType::SveFloat32x2: return clang_BuiltinType_SveFloat32x2_;
    case clang::BuiltinType::SveFloat64x2: return clang_BuiltinType_SveFloat64x2_;
    case clang::BuiltinType::SveBFloat16x2: return clang_BuiltinType_SveBFloat16x2_;
    case clang::BuiltinType::SveInt8x3: return clang_BuiltinType_SveInt8x3_;
    case clang::BuiltinType::SveInt16x3: return clang_BuiltinType_SveInt16x3_;
    case clang::BuiltinType::SveInt32x3: return clang_BuiltinType_SveInt32x3_;
    case clang::BuiltinType::SveInt64x3: return clang_BuiltinType_SveInt64x3_;
    case clang::BuiltinType::SveUint8x3: return clang_BuiltinType_SveUint8x3_;
    case clang::BuiltinType::SveUint16x3: return clang_BuiltinType_SveUint16x3_;
    case clang::BuiltinType::SveUint32x3: return clang_BuiltinType_SveUint32x3_;
    case clang::BuiltinType::SveUint64x3: return clang_BuiltinType_SveUint64x3_;
    case clang::BuiltinType::SveFloat16x3: return clang_BuiltinType_SveFloat16x3_;
    case clang::BuiltinType::SveFloat32x3: return clang_BuiltinType_SveFloat32x3_;
    case clang::BuiltinType::SveFloat64x3: return clang_BuiltinType_SveFloat64x3_;
    case clang::BuiltinType::SveBFloat16x3: return clang_BuiltinType_SveBFloat16x3_;
    case clang::BuiltinType::SveInt8x4: return clang_BuiltinType_SveInt8x4_;
    case clang::BuiltinType::SveInt16x4: return clang_BuiltinType_SveInt16x4_;
    case clang::BuiltinType::SveInt32x4: return clang_BuiltinType_SveInt32x4_;
    case clang::BuiltinType::SveInt64x4: return clang_BuiltinType_SveInt64x4_;
    case clang::BuiltinType::SveUint8x4: return clang_BuiltinType_SveUint8x4_;
    case clang::BuiltinType::SveUint16x4: return clang_BuiltinType_SveUint16x4_;
    case clang::BuiltinType::SveUint32x4: return clang_BuiltinType_SveUint32x4_;
    case clang::BuiltinType::SveUint64x4: return clang_BuiltinType_SveUint64x4_;
    case clang::BuiltinType::SveFloat16x4: return clang_BuiltinType_SveFloat16x4_;
    case clang::BuiltinType::SveFloat32x4: return clang_BuiltinType_SveFloat32x4_;
    case clang::BuiltinType::SveFloat64x4: return clang_BuiltinType_SveFloat64x4_;
    case clang::BuiltinType::SveBFloat16x4: return clang_BuiltinType_SveBFloat16x4_;
    case clang::BuiltinType::SveBool: return clang_BuiltinType_SveBool_;
    case clang::BuiltinType::SveBoolx2: return clang_BuiltinType_SveBoolx2_;
    case clang::BuiltinType::SveBoolx4: return clang_BuiltinType_SveBoolx4_;
    case clang::BuiltinType::SveCount: return clang_BuiltinType_SveCount_;
    case clang::BuiltinType::VectorQuad: return clang_BuiltinType_VectorQuad_;
    case clang::BuiltinType::VectorPair: return clang_BuiltinType_VectorPair_;
    case clang::BuiltinType::RvvInt8mf8: return clang_BuiltinType_RvvInt8mf8_;
    case clang::BuiltinType::RvvInt8mf4: return clang_BuiltinType_RvvInt8mf4_;
    case clang::BuiltinType::RvvInt8mf2: return clang_BuiltinType_RvvInt8mf2_;
    case clang::BuiltinType::RvvInt8m1: return clang_BuiltinType_RvvInt8m1_;
    case clang::BuiltinType::RvvInt8m2: return clang_BuiltinType_RvvInt8m2_;
    case clang::BuiltinType::RvvInt8m4: return clang_BuiltinType_RvvInt8m4_;
    case clang::BuiltinType::RvvInt8m8: return clang_BuiltinType_RvvInt8m8_;
    case clang::BuiltinType::RvvUint8mf8: return clang_BuiltinType_RvvUint8mf8_;
    case clang::BuiltinType::RvvUint8mf4: return clang_BuiltinType_RvvUint8mf4_;
    case clang::BuiltinType::RvvUint8mf2: return clang_BuiltinType_RvvUint8mf2_;
    case clang::BuiltinType::RvvUint8m1: return clang_BuiltinType_RvvUint8m1_;
    case clang::BuiltinType::RvvUint8m2: return clang_BuiltinType_RvvUint8m2_;
    case clang::BuiltinType::RvvUint8m4: return clang_BuiltinType_RvvUint8m4_;
    case clang::BuiltinType::RvvUint8m8: return clang_BuiltinType_RvvUint8m8_;
    case clang::BuiltinType::RvvInt16mf4: return clang_BuiltinType_RvvInt16mf4_;
    case clang::BuiltinType::RvvInt16mf2: return clang_BuiltinType_RvvInt16mf2_;
    case clang::BuiltinType::RvvInt16m1: return clang_BuiltinType_RvvInt16m1_;
    case clang::BuiltinType::RvvInt16m2: return clang_BuiltinType_RvvInt16m2_;
    case clang::BuiltinType::RvvInt16m4: return clang_BuiltinType_RvvInt16m4_;
    case clang::BuiltinType::RvvInt16m8: return clang_BuiltinType_RvvInt16m8_;
    case clang::BuiltinType::RvvUint16mf4: return clang_BuiltinType_RvvUint16mf4_;
    case clang::BuiltinType::RvvUint16mf2: return clang_BuiltinType_RvvUint16mf2_;
    case clang::BuiltinType::RvvUint16m1: return clang_BuiltinType_RvvUint16m1_;
    case clang::BuiltinType::RvvUint16m2: return clang_BuiltinType_RvvUint16m2_;
    case clang::BuiltinType::RvvUint16m4: return clang_BuiltinType_RvvUint16m4_;
    case clang::BuiltinType::RvvUint16m8: return clang_BuiltinType_RvvUint16m8_;
    case clang::BuiltinType::RvvInt32mf2: return clang_BuiltinType_RvvInt32mf2_;
    case clang::BuiltinType::RvvInt32m1: return clang_BuiltinType_RvvInt32m1_;
    case clang::BuiltinType::RvvInt32m2: return clang_BuiltinType_RvvInt32m2_;
    case clang::BuiltinType::RvvInt32m4: return clang_BuiltinType_RvvInt32m4_;
    case clang::BuiltinType::RvvInt32m8: return clang_BuiltinType_RvvInt32m8_;
    case clang::BuiltinType::RvvUint32mf2: return clang_BuiltinType_RvvUint32mf2_;
    case clang::BuiltinType::RvvUint32m1: return clang_BuiltinType_RvvUint32m1_;
    case clang::BuiltinType::RvvUint32m2: return clang_BuiltinType_RvvUint32m2_;
    case clang::BuiltinType::RvvUint32m4: return clang_BuiltinType_RvvUint32m4_;
    case clang::BuiltinType::RvvUint32m8: return clang_BuiltinType_RvvUint32m8_;
    case clang::BuiltinType::RvvInt64m1: return clang_BuiltinType_RvvInt64m1_;
    case clang::BuiltinType::RvvInt64m2: return clang_BuiltinType_RvvInt64m2_;
    case clang::BuiltinType::RvvInt64m4: return clang_BuiltinType_RvvInt64m4_;
    case clang::BuiltinType::RvvInt64m8: return clang_BuiltinType_RvvInt64m8_;
    case clang::BuiltinType::RvvUint64m1: return clang_BuiltinType_RvvUint64m1_;
    case clang::BuiltinType::RvvUint64m2: return clang_BuiltinType_RvvUint64m2_;
    case clang::BuiltinType::RvvUint64m4: return clang_BuiltinType_RvvUint64m4_;
    case clang::BuiltinType::RvvUint64m8: return clang_BuiltinType_RvvUint64m8_;
    case clang::BuiltinType::RvvFloat16mf4: return clang_BuiltinType_RvvFloat16mf4_;
    case clang::BuiltinType::RvvFloat16mf2: return clang_BuiltinType_RvvFloat16mf2_;
    case clang::BuiltinType::RvvFloat16m1: return clang_BuiltinType_RvvFloat16m1_;
    case clang::BuiltinType::RvvFloat16m2: return clang_BuiltinType_RvvFloat16m2_;
    case clang::BuiltinType::RvvFloat16m4: return clang_BuiltinType_RvvFloat16m4_;
    case clang::BuiltinType::RvvFloat16m8: return clang_BuiltinType_RvvFloat16m8_;
    case clang::BuiltinType::RvvFloat32mf2: return clang_BuiltinType_RvvFloat32mf2_;
    case clang::BuiltinType::RvvFloat32m1: return clang_BuiltinType_RvvFloat32m1_;
    case clang::BuiltinType::RvvFloat32m2: return clang_BuiltinType_RvvFloat32m2_;
    case clang::BuiltinType::RvvFloat32m4: return clang_BuiltinType_RvvFloat32m4_;
    case clang::BuiltinType::RvvFloat32m8: return clang_BuiltinType_RvvFloat32m8_;
    case clang::BuiltinType::RvvFloat64m1: return clang_BuiltinType_RvvFloat64m1_;
    case clang::BuiltinType::RvvFloat64m2: return clang_BuiltinType_RvvFloat64m2_;
    case clang::BuiltinType::RvvFloat64m4: return clang_BuiltinType_RvvFloat64m4_;
    case clang::BuiltinType::RvvFloat64m8: return clang_BuiltinType_RvvFloat64m8_;
    case clang::BuiltinType::RvvBool1: return clang_BuiltinType_RvvBool1_;
    case clang::BuiltinType::RvvBool2: return clang_BuiltinType_RvvBool2_;
    case clang::BuiltinType::RvvBool4: return clang_BuiltinType_RvvBool4_;
    case clang::BuiltinType::RvvBool8: return clang_BuiltinType_RvvBool8_;
    case clang::BuiltinType::RvvBool16: return clang_BuiltinType_RvvBool16_;
    case clang::BuiltinType::RvvBool32: return clang_BuiltinType_RvvBool32_;
    case clang::BuiltinType::RvvBool64: return clang_BuiltinType_RvvBool64_;
    case clang::BuiltinType::RvvInt8mf8x2: return clang_BuiltinType_RvvInt8mf8x2_;
    case clang::BuiltinType::RvvInt8mf8x3: return clang_BuiltinType_RvvInt8mf8x3_;
    case clang::BuiltinType::RvvInt8mf8x4: return clang_BuiltinType_RvvInt8mf8x4_;
    case clang::BuiltinType::RvvInt8mf8x5: return clang_BuiltinType_RvvInt8mf8x5_;
    case clang::BuiltinType::RvvInt8mf8x6: return clang_BuiltinType_RvvInt8mf8x6_;
    case clang::BuiltinType::RvvInt8mf8x7: return clang_BuiltinType_RvvInt8mf8x7_;
    case clang::BuiltinType::RvvInt8mf8x8: return clang_BuiltinType_RvvInt8mf8x8_;
    case clang::BuiltinType::RvvInt8mf4x2: return clang_BuiltinType_RvvInt8mf4x2_;
    case clang::BuiltinType::RvvInt8mf4x3: return clang_BuiltinType_RvvInt8mf4x3_;
    case clang::BuiltinType::RvvInt8mf4x4: return clang_BuiltinType_RvvInt8mf4x4_;
    case clang::BuiltinType::RvvInt8mf4x5: return clang_BuiltinType_RvvInt8mf4x5_;
    case clang::BuiltinType::RvvInt8mf4x6: return clang_BuiltinType_RvvInt8mf4x6_;
    case clang::BuiltinType::RvvInt8mf4x7: return clang_BuiltinType_RvvInt8mf4x7_;
    case clang::BuiltinType::RvvInt8mf4x8: return clang_BuiltinType_RvvInt8mf4x8_;
    case clang::BuiltinType::RvvInt8mf2x2: return clang_BuiltinType_RvvInt8mf2x2_;
    case clang::BuiltinType::RvvInt8mf2x3: return clang_BuiltinType_RvvInt8mf2x3_;
    case clang::BuiltinType::RvvInt8mf2x4: return clang_BuiltinType_RvvInt8mf2x4_;
    case clang::BuiltinType::RvvInt8mf2x5: return clang_BuiltinType_RvvInt8mf2x5_;
    case clang::BuiltinType::RvvInt8mf2x6: return clang_BuiltinType_RvvInt8mf2x6_;
    case clang::BuiltinType::RvvInt8mf2x7: return clang_BuiltinType_RvvInt8mf2x7_;
    case clang::BuiltinType::RvvInt8mf2x8: return clang_BuiltinType_RvvInt8mf2x8_;
    case clang::BuiltinType::RvvInt8m1x2: return clang_BuiltinType_RvvInt8m1x2_;
    case clang::BuiltinType::RvvInt8m1x3: return clang_BuiltinType_RvvInt8m1x3_;
    case clang::BuiltinType::RvvInt8m1x4: return clang_BuiltinType_RvvInt8m1x4_;
    case clang::BuiltinType::RvvInt8m1x5: return clang_BuiltinType_RvvInt8m1x5_;
    case clang::BuiltinType::RvvInt8m1x6: return clang_BuiltinType_RvvInt8m1x6_;
    case clang::BuiltinType::RvvInt8m1x7: return clang_BuiltinType_RvvInt8m1x7_;
    case clang::BuiltinType::RvvInt8m1x8: return clang_BuiltinType_RvvInt8m1x8_;
    case clang::BuiltinType::RvvInt8m2x2: return clang_BuiltinType_RvvInt8m2x2_;
    case clang::BuiltinType::RvvInt8m2x3: return clang_BuiltinType_RvvInt8m2x3_;
    case clang::BuiltinType::RvvInt8m2x4: return clang_BuiltinType_RvvInt8m2x4_;
    case clang::BuiltinType::RvvInt8m4x2: return clang_BuiltinType_RvvInt8m4x2_;
    case clang::BuiltinType::RvvUint8mf8x2: return clang_BuiltinType_RvvUint8mf8x2_;
    case clang::BuiltinType::RvvUint8mf8x3: return clang_BuiltinType_RvvUint8mf8x3_;
    case clang::BuiltinType::RvvUint8mf8x4: return clang_BuiltinType_RvvUint8mf8x4_;
    case clang::BuiltinType::RvvUint8mf8x5: return clang_BuiltinType_RvvUint8mf8x5_;
    case clang::BuiltinType::RvvUint8mf8x6: return clang_BuiltinType_RvvUint8mf8x6_;
    case clang::BuiltinType::RvvUint8mf8x7: return clang_BuiltinType_RvvUint8mf8x7_;
    case clang::BuiltinType::RvvUint8mf8x8: return clang_BuiltinType_RvvUint8mf8x8_;
    case clang::BuiltinType::RvvUint8mf4x2: return clang_BuiltinType_RvvUint8mf4x2_;
    case clang::BuiltinType::RvvUint8mf4x3: return clang_BuiltinType_RvvUint8mf4x3_;
    case clang::BuiltinType::RvvUint8mf4x4: return clang_BuiltinType_RvvUint8mf4x4_;
    case clang::BuiltinType::RvvUint8mf4x5: return clang_BuiltinType_RvvUint8mf4x5_;
    case clang::BuiltinType::RvvUint8mf4x6: return clang_BuiltinType_RvvUint8mf4x6_;
    case clang::BuiltinType::RvvUint8mf4x7: return clang_BuiltinType_RvvUint8mf4x7_;
    case clang::BuiltinType::RvvUint8mf4x8: return clang_BuiltinType_RvvUint8mf4x8_;
    case clang::BuiltinType::RvvUint8mf2x2: return clang_BuiltinType_RvvUint8mf2x2_;
    case clang::BuiltinType::RvvUint8mf2x3: return clang_BuiltinType_RvvUint8mf2x3_;
    case clang::BuiltinType::RvvUint8mf2x4: return clang_BuiltinType_RvvUint8mf2x4_;
    case clang::BuiltinType::RvvUint8mf2x5: return clang_BuiltinType_RvvUint8mf2x5_;
    case clang::BuiltinType::RvvUint8mf2x6: return clang_BuiltinType_RvvUint8mf2x6_;
    case clang::BuiltinType::RvvUint8mf2x7: return clang_BuiltinType_RvvUint8mf2x7_;
    case clang::BuiltinType::RvvUint8mf2x8: return clang_BuiltinType_RvvUint8mf2x8_;
    case clang::BuiltinType::RvvUint8m1x2: return clang_BuiltinType_RvvUint8m1x2_;
    case clang::BuiltinType::RvvUint8m1x3: return clang_BuiltinType_RvvUint8m1x3_;
    case clang::BuiltinType::RvvUint8m1x4: return clang_BuiltinType_RvvUint8m1x4_;
    case clang::BuiltinType::RvvUint8m1x5: return clang_BuiltinType_RvvUint8m1x5_;
    case clang::BuiltinType::RvvUint8m1x6: return clang_BuiltinType_RvvUint8m1x6_;
    case clang::BuiltinType::RvvUint8m1x7: return clang_BuiltinType_RvvUint8m1x7_;
    case clang::BuiltinType::RvvUint8m1x8: return clang_BuiltinType_RvvUint8m1x8_;
    case clang::BuiltinType::RvvUint8m2x2: return clang_BuiltinType_RvvUint8m2x2_;
    case clang::BuiltinType::RvvUint8m2x3: return clang_BuiltinType_RvvUint8m2x3_;
    case clang::BuiltinType::RvvUint8m2x4: return clang_BuiltinType_RvvUint8m2x4_;
    case clang::BuiltinType::RvvUint8m4x2: return clang_BuiltinType_RvvUint8m4x2_;
    case clang::BuiltinType::RvvInt16mf4x2: return clang_BuiltinType_RvvInt16mf4x2_;
    case clang::BuiltinType::RvvInt16mf4x3: return clang_BuiltinType_RvvInt16mf4x3_;
    case clang::BuiltinType::RvvInt16mf4x4: return clang_BuiltinType_RvvInt16mf4x4_;
    case clang::BuiltinType::RvvInt16mf4x5: return clang_BuiltinType_RvvInt16mf4x5_;
    case clang::BuiltinType::RvvInt16mf4x6: return clang_BuiltinType_RvvInt16mf4x6_;
    case clang::BuiltinType::RvvInt16mf4x7: return clang_BuiltinType_RvvInt16mf4x7_;
    case clang::BuiltinType::RvvInt16mf4x8: return clang_BuiltinType_RvvInt16mf4x8_;
    case clang::BuiltinType::RvvInt16mf2x2: return clang_BuiltinType_RvvInt16mf2x2_;
    case clang::BuiltinType::RvvInt16mf2x3: return clang_BuiltinType_RvvInt16mf2x3_;
    case clang::BuiltinType::RvvInt16mf2x4: return clang_BuiltinType_RvvInt16mf2x4_;
    case clang::BuiltinType::RvvInt16mf2x5: return clang_BuiltinType_RvvInt16mf2x5_;
    case clang::BuiltinType::RvvInt16mf2x6: return clang_BuiltinType_RvvInt16mf2x6_;
    case clang::BuiltinType::RvvInt16mf2x7: return clang_BuiltinType_RvvInt16mf2x7_;
    case clang::BuiltinType::RvvInt16mf2x8: return clang_BuiltinType_RvvInt16mf2x8_;
    case clang::BuiltinType::RvvInt16m1x2: return clang_BuiltinType_RvvInt16m1x2_;
    case clang::BuiltinType::RvvInt16m1x3: return clang_BuiltinType_RvvInt16m1x3_;
    case clang::BuiltinType::RvvInt16m1x4: return clang_BuiltinType_RvvInt16m1x4_;
    case clang::BuiltinType::RvvInt16m1x5: return clang_BuiltinType_RvvInt16m1x5_;
    case clang::BuiltinType::RvvInt16m1x6: return clang_BuiltinType_RvvInt16m1x6_;
    case clang::BuiltinType::RvvInt16m1x7: return clang_BuiltinType_RvvInt16m1x7_;
    case clang::BuiltinType::RvvInt16m1x8: return clang_BuiltinType_RvvInt16m1x8_;
    case clang::BuiltinType::RvvInt16m2x2: return clang_BuiltinType_RvvInt16m2x2_;
    case clang::BuiltinType::RvvInt16m2x3: return clang_BuiltinType_RvvInt16m2x3_;
    case clang::BuiltinType::RvvInt16m2x4: return clang_BuiltinType_RvvInt16m2x4_;
    case clang::BuiltinType::RvvInt16m4x2: return clang_BuiltinType_RvvInt16m4x2_;
    case clang::BuiltinType::RvvUint16mf4x2: return clang_BuiltinType_RvvUint16mf4x2_;
    case clang::BuiltinType::RvvUint16mf4x3: return clang_BuiltinType_RvvUint16mf4x3_;
    case clang::BuiltinType::RvvUint16mf4x4: return clang_BuiltinType_RvvUint16mf4x4_;
    case clang::BuiltinType::RvvUint16mf4x5: return clang_BuiltinType_RvvUint16mf4x5_;
    case clang::BuiltinType::RvvUint16mf4x6: return clang_BuiltinType_RvvUint16mf4x6_;
    case clang::BuiltinType::RvvUint16mf4x7: return clang_BuiltinType_RvvUint16mf4x7_;
    case clang::BuiltinType::RvvUint16mf4x8: return clang_BuiltinType_RvvUint16mf4x8_;
    case clang::BuiltinType::RvvUint16mf2x2: return clang_BuiltinType_RvvUint16mf2x2_;
    case clang::BuiltinType::RvvUint16mf2x3: return clang_BuiltinType_RvvUint16mf2x3_;
    case clang::BuiltinType::RvvUint16mf2x4: return clang_BuiltinType_RvvUint16mf2x4_;
    case clang::BuiltinType::RvvUint16mf2x5: return clang_BuiltinType_RvvUint16mf2x5_;
    case clang::BuiltinType::RvvUint16mf2x6: return clang_BuiltinType_RvvUint16mf2x6_;
    case clang::BuiltinType::RvvUint16mf2x7: return clang_BuiltinType_RvvUint16mf2x7_;
    case clang::BuiltinType::RvvUint16mf2x8: return clang_BuiltinType_RvvUint16mf2x8_;
    case clang::BuiltinType::RvvUint16m1x2: return clang_BuiltinType_RvvUint16m1x2_;
    case clang::BuiltinType::RvvUint16m1x3: return clang_BuiltinType_RvvUint16m1x3_;
    case clang::BuiltinType::RvvUint16m1x4: return clang_BuiltinType_RvvUint16m1x4_;
    case clang::BuiltinType::RvvUint16m1x5: return clang_BuiltinType_RvvUint16m1x5_;
    case clang::BuiltinType::RvvUint16m1x6: return clang_BuiltinType_RvvUint16m1x6_;
    case clang::BuiltinType::RvvUint16m1x7: return clang_BuiltinType_RvvUint16m1x7_;
    case clang::BuiltinType::RvvUint16m1x8: return clang_BuiltinType_RvvUint16m1x8_;
    case clang::BuiltinType::RvvUint16m2x2: return clang_BuiltinType_RvvUint16m2x2_;
    case clang::BuiltinType::RvvUint16m2x3: return clang_BuiltinType_RvvUint16m2x3_;
    case clang::BuiltinType::RvvUint16m2x4: return clang_BuiltinType_RvvUint16m2x4_;
    case clang::BuiltinType::RvvUint16m4x2: return clang_BuiltinType_RvvUint16m4x2_;
    case clang::BuiltinType::RvvInt32mf2x2: return clang_BuiltinType_RvvInt32mf2x2_;
    case clang::BuiltinType::RvvInt32mf2x3: return clang_BuiltinType_RvvInt32mf2x3_;
    case clang::BuiltinType::RvvInt32mf2x4: return clang_BuiltinType_RvvInt32mf2x4_;
    case clang::BuiltinType::RvvInt32mf2x5: return clang_BuiltinType_RvvInt32mf2x5_;
    case clang::BuiltinType::RvvInt32mf2x6: return clang_BuiltinType_RvvInt32mf2x6_;
    case clang::BuiltinType::RvvInt32mf2x7: return clang_BuiltinType_RvvInt32mf2x7_;
    case clang::BuiltinType::RvvInt32mf2x8: return clang_BuiltinType_RvvInt32mf2x8_;
    case clang::BuiltinType::RvvInt32m1x2: return clang_BuiltinType_RvvInt32m1x2_;
    case clang::BuiltinType::RvvInt32m1x3: return clang_BuiltinType_RvvInt32m1x3_;
    case clang::BuiltinType::RvvInt32m1x4: return clang_BuiltinType_RvvInt32m1x4_;
    case clang::BuiltinType::RvvInt32m1x5: return clang_BuiltinType_RvvInt32m1x5_;
    case clang::BuiltinType::RvvInt32m1x6: return clang_BuiltinType_RvvInt32m1x6_;
    case clang::BuiltinType::RvvInt32m1x7: return clang_BuiltinType_RvvInt32m1x7_;
    case clang::BuiltinType::RvvInt32m1x8: return clang_BuiltinType_RvvInt32m1x8_;
    case clang::BuiltinType::RvvInt32m2x2: return clang_BuiltinType_RvvInt32m2x2_;
    case clang::BuiltinType::RvvInt32m2x3: return clang_BuiltinType_RvvInt32m2x3_;
    case clang::BuiltinType::RvvInt32m2x4: return clang_BuiltinType_RvvInt32m2x4_;
    case clang::BuiltinType::RvvInt32m4x2: return clang_BuiltinType_RvvInt32m4x2_;
    case clang::BuiltinType::RvvUint32mf2x2: return clang_BuiltinType_RvvUint32mf2x2_;
    case clang::BuiltinType::RvvUint32mf2x3: return clang_BuiltinType_RvvUint32mf2x3_;
    case clang::BuiltinType::RvvUint32mf2x4: return clang_BuiltinType_RvvUint32mf2x4_;
    case clang::BuiltinType::RvvUint32mf2x5: return clang_BuiltinType_RvvUint32mf2x5_;
    case clang::BuiltinType::RvvUint32mf2x6: return clang_BuiltinType_RvvUint32mf2x6_;
    case clang::BuiltinType::RvvUint32mf2x7: return clang_BuiltinType_RvvUint32mf2x7_;
    case clang::BuiltinType::RvvUint32mf2x8: return clang_BuiltinType_RvvUint32mf2x8_;
    case clang::BuiltinType::RvvUint32m1x2: return clang_BuiltinType_RvvUint32m1x2_;
    case clang::BuiltinType::RvvUint32m1x3: return clang_BuiltinType_RvvUint32m1x3_;
    case clang::BuiltinType::RvvUint32m1x4: return clang_BuiltinType_RvvUint32m1x4_;
    case clang::BuiltinType::RvvUint32m1x5: return clang_BuiltinType_RvvUint32m1x5_;
    case clang::BuiltinType::RvvUint32m1x6: return clang_BuiltinType_RvvUint32m1x6_;
    case clang::BuiltinType::RvvUint32m1x7: return clang_BuiltinType_RvvUint32m1x7_;
    case clang::BuiltinType::RvvUint32m1x8: return clang_BuiltinType_RvvUint32m1x8_;
    case clang::BuiltinType::RvvUint32m2x2: return clang_BuiltinType_RvvUint32m2x2_;
    case clang::BuiltinType::RvvUint32m2x3: return clang_BuiltinType_RvvUint32m2x3_;
    case clang::BuiltinType::RvvUint32m2x4: return clang_BuiltinType_RvvUint32m2x4_;
    case clang::BuiltinType::RvvUint32m4x2: return clang_BuiltinType_RvvUint32m4x2_;
    case clang::BuiltinType::RvvInt64m1x2: return clang_BuiltinType_RvvInt64m1x2_;
    case clang::BuiltinType::RvvInt64m1x3: return clang_BuiltinType_RvvInt64m1x3_;
    case clang::BuiltinType::RvvInt64m1x4: return clang_BuiltinType_RvvInt64m1x4_;
    case clang::BuiltinType::RvvInt64m1x5: return clang_BuiltinType_RvvInt64m1x5_;
    case clang::BuiltinType::RvvInt64m1x6: return clang_BuiltinType_RvvInt64m1x6_;
    case clang::BuiltinType::RvvInt64m1x7: return clang_BuiltinType_RvvInt64m1x7_;
    case clang::BuiltinType::RvvInt64m1x8: return clang_BuiltinType_RvvInt64m1x8_;
    case clang::BuiltinType::RvvInt64m2x2: return clang_BuiltinType_RvvInt64m2x2_;
    case clang::BuiltinType::RvvInt64m2x3: return clang_BuiltinType_RvvInt64m2x3_;
    case clang::BuiltinType::RvvInt64m2x4: return clang_BuiltinType_RvvInt64m2x4_;
    case clang::BuiltinType::RvvInt64m4x2: return clang_BuiltinType_RvvInt64m4x2_;
    case clang::BuiltinType::RvvUint64m1x2: return clang_BuiltinType_RvvUint64m1x2_;
    case clang::BuiltinType::RvvUint64m1x3: return clang_BuiltinType_RvvUint64m1x3_;
    case clang::BuiltinType::RvvUint64m1x4: return clang_BuiltinType_RvvUint64m1x4_;
    case clang::BuiltinType::RvvUint64m1x5: return clang_BuiltinType_RvvUint64m1x5_;
    case clang::BuiltinType::RvvUint64m1x6: return clang_BuiltinType_RvvUint64m1x6_;
    case clang::BuiltinType::RvvUint64m1x7: return clang_BuiltinType_RvvUint64m1x7_;
    case clang::BuiltinType::RvvUint64m1x8: return clang_BuiltinType_RvvUint64m1x8_;
    case clang::BuiltinType::RvvUint64m2x2: return clang_BuiltinType_RvvUint64m2x2_;
    case clang::BuiltinType::RvvUint64m2x3: return clang_BuiltinType_RvvUint64m2x3_;
    case clang::BuiltinType::RvvUint64m2x4: return clang_BuiltinType_RvvUint64m2x4_;
    case clang::BuiltinType::RvvUint64m4x2: return clang_BuiltinType_RvvUint64m4x2_;
    case clang::BuiltinType::RvvFloat16mf4x2: return clang_BuiltinType_RvvFloat16mf4x2_;
    case clang::BuiltinType::RvvFloat16mf4x3: return clang_BuiltinType_RvvFloat16mf4x3_;
    case clang::BuiltinType::RvvFloat16mf4x4: return clang_BuiltinType_RvvFloat16mf4x4_;
    case clang::BuiltinType::RvvFloat16mf4x5: return clang_BuiltinType_RvvFloat16mf4x5_;
    case clang::BuiltinType::RvvFloat16mf4x6: return clang_BuiltinType_RvvFloat16mf4x6_;
    case clang::BuiltinType::RvvFloat16mf4x7: return clang_BuiltinType_RvvFloat16mf4x7_;
    case clang::BuiltinType::RvvFloat16mf4x8: return clang_BuiltinType_RvvFloat16mf4x8_;
    case clang::BuiltinType::RvvFloat16mf2x2: return clang_BuiltinType_RvvFloat16mf2x2_;
    case clang::BuiltinType::RvvFloat16mf2x3: return clang_BuiltinType_RvvFloat16mf2x3_;
    case clang::BuiltinType::RvvFloat16mf2x4: return clang_BuiltinType_RvvFloat16mf2x4_;
    case clang::BuiltinType::RvvFloat16mf2x5: return clang_BuiltinType_RvvFloat16mf2x5_;
    case clang::BuiltinType::RvvFloat16mf2x6: return clang_BuiltinType_RvvFloat16mf2x6_;
    case clang::BuiltinType::RvvFloat16mf2x7: return clang_BuiltinType_RvvFloat16mf2x7_;
    case clang::BuiltinType::RvvFloat16mf2x8: return clang_BuiltinType_RvvFloat16mf2x8_;
    case clang::BuiltinType::RvvFloat16m1x2: return clang_BuiltinType_RvvFloat16m1x2_;
    case clang::BuiltinType::RvvFloat16m1x3: return clang_BuiltinType_RvvFloat16m1x3_;
    case clang::BuiltinType::RvvFloat16m1x4: return clang_BuiltinType_RvvFloat16m1x4_;
    case clang::BuiltinType::RvvFloat16m1x5: return clang_BuiltinType_RvvFloat16m1x5_;
    case clang::BuiltinType::RvvFloat16m1x6: return clang_BuiltinType_RvvFloat16m1x6_;
    case clang::BuiltinType::RvvFloat16m1x7: return clang_BuiltinType_RvvFloat16m1x7_;
    case clang::BuiltinType::RvvFloat16m1x8: return clang_BuiltinType_RvvFloat16m1x8_;
    case clang::BuiltinType::RvvFloat16m2x2: return clang_BuiltinType_RvvFloat16m2x2_;
    case clang::BuiltinType::RvvFloat16m2x3: return clang_BuiltinType_RvvFloat16m2x3_;
    case clang::BuiltinType::RvvFloat16m2x4: return clang_BuiltinType_RvvFloat16m2x4_;
    case clang::BuiltinType::RvvFloat16m4x2: return clang_BuiltinType_RvvFloat16m4x2_;
    case clang::BuiltinType::RvvFloat32mf2x2: return clang_BuiltinType_RvvFloat32mf2x2_;
    case clang::BuiltinType::RvvFloat32mf2x3: return clang_BuiltinType_RvvFloat32mf2x3_;
    case clang::BuiltinType::RvvFloat32mf2x4: return clang_BuiltinType_RvvFloat32mf2x4_;
    case clang::BuiltinType::RvvFloat32mf2x5: return clang_BuiltinType_RvvFloat32mf2x5_;
    case clang::BuiltinType::RvvFloat32mf2x6: return clang_BuiltinType_RvvFloat32mf2x6_;
    case clang::BuiltinType::RvvFloat32mf2x7: return clang_BuiltinType_RvvFloat32mf2x7_;
    case clang::BuiltinType::RvvFloat32mf2x8: return clang_BuiltinType_RvvFloat32mf2x8_;
    case clang::BuiltinType::RvvFloat32m1x2: return clang_BuiltinType_RvvFloat32m1x2_;
    case clang::BuiltinType::RvvFloat32m1x3: return clang_BuiltinType_RvvFloat32m1x3_;
    case clang::BuiltinType::RvvFloat32m1x4: return clang_BuiltinType_RvvFloat32m1x4_;
    case clang::BuiltinType::RvvFloat32m1x5: return clang_BuiltinType_RvvFloat32m1x5_;
    case clang::BuiltinType::RvvFloat32m1x6: return clang_BuiltinType_RvvFloat32m1x6_;
    case clang::BuiltinType::RvvFloat32m1x7: return clang_BuiltinType_RvvFloat32m1x7_;
    case clang::BuiltinType::RvvFloat32m1x8: return clang_BuiltinType_RvvFloat32m1x8_;
    case clang::BuiltinType::RvvFloat32m2x2: return clang_BuiltinType_RvvFloat32m2x2_;
    case clang::BuiltinType::RvvFloat32m2x3: return clang_BuiltinType_RvvFloat32m2x3_;
    case clang::BuiltinType::RvvFloat32m2x4: return clang_BuiltinType_RvvFloat32m2x4_;
    case clang::BuiltinType::RvvFloat32m4x2: return clang_BuiltinType_RvvFloat32m4x2_;
    case clang::BuiltinType::RvvFloat64m1x2: return clang_BuiltinType_RvvFloat64m1x2_;
    case clang::BuiltinType::RvvFloat64m1x3: return clang_BuiltinType_RvvFloat64m1x3_;
    case clang::BuiltinType::RvvFloat64m1x4: return clang_BuiltinType_RvvFloat64m1x4_;
    case clang::BuiltinType::RvvFloat64m1x5: return clang_BuiltinType_RvvFloat64m1x5_;
    case clang::BuiltinType::RvvFloat64m1x6: return clang_BuiltinType_RvvFloat64m1x6_;
    case clang::BuiltinType::RvvFloat64m1x7: return clang_BuiltinType_RvvFloat64m1x7_;
    case clang::BuiltinType::RvvFloat64m1x8: return clang_BuiltinType_RvvFloat64m1x8_;
    case clang::BuiltinType::RvvFloat64m2x2: return clang_BuiltinType_RvvFloat64m2x2_;
    case clang::BuiltinType::RvvFloat64m2x3: return clang_BuiltinType_RvvFloat64m2x3_;
    case clang::BuiltinType::RvvFloat64m2x4: return clang_BuiltinType_RvvFloat64m2x4_;
    case clang::BuiltinType::RvvFloat64m4x2: return clang_BuiltinType_RvvFloat64m4x2_;
    case clang::BuiltinType::WasmExternRef: return clang_BuiltinType_WasmExternRef_;
    case clang::BuiltinType::Void: return clang_BuiltinType_Void_;
    case clang::BuiltinType::Bool: return clang_BuiltinType_Bool_;
    case clang::BuiltinType::Char_U: return clang_BuiltinType_Char_U_;
    case clang::BuiltinType::UChar: return clang_BuiltinType_UChar_;
    case clang::BuiltinType::WChar_U: return clang_BuiltinType_WChar_U_;
    case clang::BuiltinType::Char8: return clang_BuiltinType_Char8_;
    case clang::BuiltinType::Char16: return clang_BuiltinType_Char16_;
    case clang::BuiltinType::Char32: return clang_BuiltinType_Char32_;
    case clang::BuiltinType::UShort: return clang_BuiltinType_UShort_;
    case clang::BuiltinType::UInt: return clang_BuiltinType_UInt_;
    case clang::BuiltinType::ULong: return clang_BuiltinType_ULong_;
    case clang::BuiltinType::ULongLong: return clang_BuiltinType_ULongLong_;
    case clang::BuiltinType::UInt128: return clang_BuiltinType_UInt128_;
    case clang::BuiltinType::Char_S: return clang_BuiltinType_Char_S_;
    case clang::BuiltinType::SChar: return clang_BuiltinType_SChar_;
    case clang::BuiltinType::WChar_S: return clang_BuiltinType_WChar_S_;
    case clang::BuiltinType::Short: return clang_BuiltinType_Short_;
    case clang::BuiltinType::Int: return clang_BuiltinType_Int_;
    case clang::BuiltinType::Long: return clang_BuiltinType_Long_;
    case clang::BuiltinType::LongLong: return clang_BuiltinType_LongLong_;
    case clang::BuiltinType::Int128: return clang_BuiltinType_Int128_;
    case clang::BuiltinType::ShortAccum: return clang_BuiltinType_ShortAccum_;
    case clang::BuiltinType::Accum: return clang_BuiltinType_Accum_;
    case clang::BuiltinType::LongAccum: return clang_BuiltinType_LongAccum_;
    case clang::BuiltinType::UShortAccum: return clang_BuiltinType_UShortAccum_;
    case clang::BuiltinType::UAccum: return clang_BuiltinType_UAccum_;
    case clang::BuiltinType::ULongAccum: return clang_BuiltinType_ULongAccum_;
    case clang::BuiltinType::ShortFract: return clang_BuiltinType_ShortFract_;
    case clang::BuiltinType::Fract: return clang_BuiltinType_Fract_;
    case clang::BuiltinType::LongFract: return clang_BuiltinType_LongFract_;
    case clang::BuiltinType::UShortFract: return clang_BuiltinType_UShortFract_;
    case clang::BuiltinType::UFract: return clang_BuiltinType_UFract_;
    case clang::BuiltinType::ULongFract: return clang_BuiltinType_ULongFract_;
    case clang::BuiltinType::SatShortAccum: return clang_BuiltinType_SatShortAccum_;
    case clang::BuiltinType::SatAccum: return clang_BuiltinType_SatAccum_;
    case clang::BuiltinType::SatLongAccum: return clang_BuiltinType_SatLongAccum_;
    case clang::BuiltinType::SatUShortAccum: return clang_BuiltinType_SatUShortAccum_;
    case clang::BuiltinType::SatUAccum: return clang_BuiltinType_SatUAccum_;
    case clang::BuiltinType::SatULongAccum: return clang_BuiltinType_SatULongAccum_;
    case clang::BuiltinType::SatShortFract: return clang_BuiltinType_SatShortFract_;
    case clang::BuiltinType::SatFract: return clang_BuiltinType_SatFract_;
    case clang::BuiltinType::SatLongFract: return clang_BuiltinType_SatLongFract_;
    case clang::BuiltinType::SatUShortFract: return clang_BuiltinType_SatUShortFract_;
    case clang::BuiltinType::SatUFract: return clang_BuiltinType_SatUFract_;
    case clang::BuiltinType::SatULongFract: return clang_BuiltinType_SatULongFract_;
    case clang::BuiltinType::Half: return clang_BuiltinType_Half_;
    case clang::BuiltinType::Float: return clang_BuiltinType_Float_;
    case clang::BuiltinType::Double: return clang_BuiltinType_Double_;
    case clang::BuiltinType::LongDouble: return clang_BuiltinType_LongDouble_;
    case clang::BuiltinType::Float16: return clang_BuiltinType_Float16_;
    case clang::BuiltinType::BFloat16: return clang_BuiltinType_BFloat16_;
    case clang::BuiltinType::Float128: return clang_BuiltinType_Float128_;
    case clang::BuiltinType::Ibm128: return clang_BuiltinType_Ibm128_;
    case clang::BuiltinType::NullPtr: return clang_BuiltinType_NullPtr_;
    case clang::BuiltinType::ObjCId: return clang_BuiltinType_ObjCId_;
    case clang::BuiltinType::ObjCClass: return clang_BuiltinType_ObjCClass_;
    case clang::BuiltinType::ObjCSel: return clang_BuiltinType_ObjCSel_;
    case clang::BuiltinType::OCLSampler: return clang_BuiltinType_OCLSampler_;
    case clang::BuiltinType::OCLEvent: return clang_BuiltinType_OCLEvent_;
    case clang::BuiltinType::OCLClkEvent: return clang_BuiltinType_OCLClkEvent_;
    case clang::BuiltinType::OCLQueue: return clang_BuiltinType_OCLQueue_;
    case clang::BuiltinType::OCLReserveID: return clang_BuiltinType_OCLReserveID_;
    case clang::BuiltinType::Dependent: return clang_BuiltinType_Dependent_;
    case clang::BuiltinType::Overload: return clang_BuiltinType_Overload_;
    case clang::BuiltinType::BoundMember: return clang_BuiltinType_BoundMember_;
    case clang::BuiltinType::PseudoObject: return clang_BuiltinType_PseudoObject_;
    case clang::BuiltinType::UnknownAny: return clang_BuiltinType_UnknownAny_;
    case clang::BuiltinType::BuiltinFn: return clang_BuiltinType_BuiltinFn_;
    case clang::BuiltinType::ARCUnbridgedCast: return clang_BuiltinType_ARCUnbridgedCast_;
    case clang::BuiltinType::IncompleteMatrixIdx: return clang_BuiltinType_IncompleteMatrixIdx_;
    case clang::BuiltinType::OMPArraySection: return clang_BuiltinType_OMPArraySection_;
    case clang::BuiltinType::OMPArrayShaping: return clang_BuiltinType_OMPArrayShaping_;
    case clang::BuiltinType::OMPIterator: return clang_BuiltinType_OMPIterator_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::CXXConstructExpr::ConstructionKind e) {
  switch(e) {
    case clang::CXXConstructExpr::CK_Complete: return clang_CXXConstructExpr_CK_Complete_;
    case clang::CXXConstructExpr::CK_NonVirtualBase: return clang_CXXConstructExpr_CK_NonVirtualBase_;
    case clang::CXXConstructExpr::CK_VirtualBase: return clang_CXXConstructExpr_CK_VirtualBase_;
    case clang::CXXConstructExpr::CK_Delegating: return clang_CXXConstructExpr_CK_Delegating_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::CXXNewExpr::InitializationStyle e) {
  switch(e) {
    case clang::CXXNewExpr::NoInit: return clang_CXXNewExpr_NoInit_;
    case clang::CXXNewExpr::CallInit: return clang_CXXNewExpr_CallInit_;
    case clang::CXXNewExpr::ListInit: return clang_CXXNewExpr_ListInit_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::CallExpr::ADLCallKind e) {
  switch(e) {
    case clang::CallExpr::ADLCallKind::NotADL: return clang_CallExpr_ADLCallKind_NotADL_;
    case clang::CallExpr::ADLCallKind::UsesADL: return clang_CallExpr_ADLCallKind_UsesADL_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<bool>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::CallingConv e) {
  switch(e) {
    case clang::CC_C: return clang_CC_C_;
    case clang::CC_X86StdCall: return clang_CC_X86StdCall_;
    case clang::CC_X86FastCall: return clang_CC_X86FastCall_;
    case clang::CC_X86ThisCall: return clang_CC_X86ThisCall_;
    case clang::CC_X86VectorCall: return clang_CC_X86VectorCall_;
    case clang::CC_X86Pascal: return clang_CC_X86Pascal_;
    case clang::CC_Win64: return clang_CC_Win64_;
    case clang::CC_X86_64SysV: return clang_CC_X86_64SysV_;
    case clang::CC_X86RegCall: return clang_CC_X86RegCall_;
    case clang::CC_AAPCS: return clang_CC_AAPCS_;
    case clang::CC_AAPCS_VFP: return clang_CC_AAPCS_VFP_;
    case clang::CC_IntelOclBicc: return clang_CC_IntelOclBicc_;
    case clang::CC_SpirFunction: return clang_CC_SpirFunction_;
    case clang::CC_OpenCLKernel: return clang_CC_OpenCLKernel_;
    case clang::CC_Swift: return clang_CC_Swift_;
    case clang::CC_SwiftAsync: return clang_CC_SwiftAsync_;
    case clang::CC_PreserveMost: return clang_CC_PreserveMost_;
    case clang::CC_PreserveAll: return clang_CC_PreserveAll_;
    case clang::CC_AArch64VectorCall: return clang_CC_AArch64VectorCall_;
    case clang::CC_AArch64SVEPCS: return clang_CC_AArch64SVEPCS_;
    case clang::CC_AMDGPUKernelCall: return clang_CC_AMDGPUKernelCall_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::CanThrowResult e) {
  switch(e) {
    case clang::CT_Cannot: return clang_CT_Cannot_;
    case clang::CT_Dependent: return clang_CT_Dependent_;
    case clang::CT_Can: return clang_CT_Can_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::CapturedRegionKind e) {
  switch(e) {
    case clang::CR_Default: return clang_CR_Default_;
    case clang::CR_ObjCAtFinally: return clang_CR_ObjCAtFinally_;
    case clang::CR_OpenMP: return clang_CR_OpenMP_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::CastKind e) {
  switch(e) {
    case clang::CK_Dependent: return clang_CK_Dependent_;
    case clang::CK_BitCast: return clang_CK_BitCast_;
    case clang::CK_LValueBitCast: return clang_CK_LValueBitCast_;
    case clang::CK_LValueToRValueBitCast: return clang_CK_LValueToRValueBitCast_;
    case clang::CK_LValueToRValue: return clang_CK_LValueToRValue_;
    case clang::CK_NoOp: return clang_CK_NoOp_;
    case clang::CK_BaseToDerived: return clang_CK_BaseToDerived_;
    case clang::CK_DerivedToBase: return clang_CK_DerivedToBase_;
    case clang::CK_UncheckedDerivedToBase: return clang_CK_UncheckedDerivedToBase_;
    case clang::CK_Dynamic: return clang_CK_Dynamic_;
    case clang::CK_ToUnion: return clang_CK_ToUnion_;
    case clang::CK_ArrayToPointerDecay: return clang_CK_ArrayToPointerDecay_;
    case clang::CK_FunctionToPointerDecay: return clang_CK_FunctionToPointerDecay_;
    case clang::CK_NullToPointer: return clang_CK_NullToPointer_;
    case clang::CK_NullToMemberPointer: return clang_CK_NullToMemberPointer_;
    case clang::CK_BaseToDerivedMemberPointer: return clang_CK_BaseToDerivedMemberPointer_;
    case clang::CK_DerivedToBaseMemberPointer: return clang_CK_DerivedToBaseMemberPointer_;
    case clang::CK_MemberPointerToBoolean: return clang_CK_MemberPointerToBoolean_;
    case clang::CK_ReinterpretMemberPointer: return clang_CK_ReinterpretMemberPointer_;
    case clang::CK_UserDefinedConversion: return clang_CK_UserDefinedConversion_;
    case clang::CK_ConstructorConversion: return clang_CK_ConstructorConversion_;
    case clang::CK_IntegralToPointer: return clang_CK_IntegralToPointer_;
    case clang::CK_PointerToIntegral: return clang_CK_PointerToIntegral_;
    case clang::CK_PointerToBoolean: return clang_CK_PointerToBoolean_;
    case clang::CK_ToVoid: return clang_CK_ToVoid_;
    case clang::CK_MatrixCast: return clang_CK_MatrixCast_;
    case clang::CK_VectorSplat: return clang_CK_VectorSplat_;
    case clang::CK_IntegralCast: return clang_CK_IntegralCast_;
    case clang::CK_IntegralToBoolean: return clang_CK_IntegralToBoolean_;
    case clang::CK_IntegralToFloating: return clang_CK_IntegralToFloating_;
    case clang::CK_FloatingToFixedPoint: return clang_CK_FloatingToFixedPoint_;
    case clang::CK_FixedPointToFloating: return clang_CK_FixedPointToFloating_;
    case clang::CK_FixedPointCast: return clang_CK_FixedPointCast_;
    case clang::CK_FixedPointToIntegral: return clang_CK_FixedPointToIntegral_;
    case clang::CK_IntegralToFixedPoint: return clang_CK_IntegralToFixedPoint_;
    case clang::CK_FixedPointToBoolean: return clang_CK_FixedPointToBoolean_;
    case clang::CK_FloatingToIntegral: return clang_CK_FloatingToIntegral_;
    case clang::CK_FloatingToBoolean: return clang_CK_FloatingToBoolean_;
    case clang::CK_BooleanToSignedIntegral: return clang_CK_BooleanToSignedIntegral_;
    case clang::CK_FloatingCast: return clang_CK_FloatingCast_;
    case clang::CK_CPointerToObjCPointerCast: return clang_CK_CPointerToObjCPointerCast_;
    case clang::CK_BlockPointerToObjCPointerCast: return clang_CK_BlockPointerToObjCPointerCast_;
    case clang::CK_AnyPointerToBlockPointerCast: return clang_CK_AnyPointerToBlockPointerCast_;
    case clang::CK_ObjCObjectLValueCast: return clang_CK_ObjCObjectLValueCast_;
    case clang::CK_FloatingRealToComplex: return clang_CK_FloatingRealToComplex_;
    case clang::CK_FloatingComplexToReal: return clang_CK_FloatingComplexToReal_;
    case clang::CK_FloatingComplexToBoolean: return clang_CK_FloatingComplexToBoolean_;
    case clang::CK_FloatingComplexCast: return clang_CK_FloatingComplexCast_;
    case clang::CK_FloatingComplexToIntegralComplex: return clang_CK_FloatingComplexToIntegralComplex_;
    case clang::CK_IntegralRealToComplex: return clang_CK_IntegralRealToComplex_;
    case clang::CK_IntegralComplexToReal: return clang_CK_IntegralComplexToReal_;
    case clang::CK_IntegralComplexToBoolean: return clang_CK_IntegralComplexToBoolean_;
    case clang::CK_IntegralComplexCast: return clang_CK_IntegralComplexCast_;
    case clang::CK_IntegralComplexToFloatingComplex: return clang_CK_IntegralComplexToFloatingComplex_;
    case clang::CK_ARCProduceObject: return clang_CK_ARCProduceObject_;
    case clang::CK_ARCConsumeObject: return clang_CK_ARCConsumeObject_;
    case clang::CK_ARCReclaimReturnedObject: return clang_CK_ARCReclaimReturnedObject_;
    case clang::CK_ARCExtendBlockObject: return clang_CK_ARCExtendBlockObject_;
    case clang::CK_AtomicToNonAtomic: return clang_CK_AtomicToNonAtomic_;
    case clang::CK_NonAtomicToAtomic: return clang_CK_NonAtomicToAtomic_;
    case clang::CK_CopyAndAutoreleaseBlockObject: return clang_CK_CopyAndAutoreleaseBlockObject_;
    case clang::CK_BuiltinFnToFnPtr: return clang_CK_BuiltinFnToFnPtr_;
    case clang::CK_ZeroToOCLOpaqueType: return clang_CK_ZeroToOCLOpaqueType_;
    case clang::CK_AddressSpaceConversion: return clang_CK_AddressSpaceConversion_;
    case clang::CK_IntToOCLSampler: return clang_CK_IntToOCLSampler_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::CharacterLiteral::CharacterKind e) {
  switch(e) {
    case clang::CharacterLiteral::Ascii: return clang_CharacterLiteral_Ascii_;
    case clang::CharacterLiteral::Wide: return clang_CharacterLiteral_Wide_;
    case clang::CharacterLiteral::UTF8: return clang_CharacterLiteral_UTF8_;
    case clang::CharacterLiteral::UTF16: return clang_CharacterLiteral_UTF16_;
    case clang::CharacterLiteral::UTF32: return clang_CharacterLiteral_UTF32_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::ConstantExpr::ResultStorageKind e) {
  switch(e) {
    case clang::ConstantExpr::RSK_None: return clang_ConstantExpr_RSK_None_;
    case clang::ConstantExpr::RSK_Int64: return clang_ConstantExpr_RSK_Int64_;
    case clang::ConstantExpr::RSK_APValue: return clang_ConstantExpr_RSK_APValue_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::ConstexprSpecKind e) {
  switch(e) {
    case clang::ConstexprSpecKind::Unspecified: return clang_ConstexprSpecKind_Unspecified_;
    case clang::ConstexprSpecKind::Constexpr: return clang_ConstexprSpecKind_Constexpr_;
    case clang::ConstexprSpecKind::Consteval: return clang_ConstexprSpecKind_Consteval_;
    case clang::ConstexprSpecKind::Constinit: return clang_ConstexprSpecKind_Constinit_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::Decl::FriendObjectKind e) {
  switch(e) {
    case clang::Decl::FOK_None: return clang_Decl_FOK_None_;
    case clang::Decl::FOK_Declared: return clang_Decl_FOK_Declared_;
    case clang::Decl::FOK_Undeclared: return clang_Decl_FOK_Undeclared_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::Decl::Kind e) {
  switch(e) {
    case clang::Decl::AccessSpec: return clang_Decl_AccessSpec_;
    case clang::Decl::Block: return clang_Decl_Block_;
    case clang::Decl::Captured: return clang_Decl_Captured_;
    case clang::Decl::ClassScopeFunctionSpecialization: return clang_Decl_ClassScopeFunctionSpecialization_;
    case clang::Decl::Empty: return clang_Decl_Empty_;
    case clang::Decl::Export: return clang_Decl_Export_;
    case clang::Decl::ExternCContext: return clang_Decl_ExternCContext_;
    case clang::Decl::FileScopeAsm: return clang_Decl_FileScopeAsm_;
    case clang::Decl::Friend: return clang_Decl_Friend_;
    case clang::Decl::FriendTemplate: return clang_Decl_FriendTemplate_;
    case clang::Decl::ImplicitConceptSpecialization: return clang_Decl_ImplicitConceptSpecialization_;
    case clang::Decl::Import: return clang_Decl_Import_;
    case clang::Decl::LifetimeExtendedTemporary: return clang_Decl_LifetimeExtendedTemporary_;
    case clang::Decl::LinkageSpec: return clang_Decl_LinkageSpec_;
    case clang::Decl::Using: return clang_Decl_Using_;
    case clang::Decl::UsingEnum: return clang_Decl_UsingEnum_;
    case clang::Decl::HLSLBuffer: return clang_Decl_HLSLBuffer_;
    case clang::Decl::Label: return clang_Decl_Label_;
    case clang::Decl::Namespace: return clang_Decl_Namespace_;
    case clang::Decl::NamespaceAlias: return clang_Decl_NamespaceAlias_;
    case clang::Decl::ObjCCompatibleAlias: return clang_Decl_ObjCCompatibleAlias_;
    case clang::Decl::ObjCCategory: return clang_Decl_ObjCCategory_;
    case clang::Decl::ObjCCategoryImpl: return clang_Decl_ObjCCategoryImpl_;
    case clang::Decl::ObjCImplementation: return clang_Decl_ObjCImplementation_;
    case clang::Decl::ObjCInterface: return clang_Decl_ObjCInterface_;
    case clang::Decl::ObjCProtocol: return clang_Decl_ObjCProtocol_;
    case clang::Decl::ObjCMethod: return clang_Decl_ObjCMethod_;
    case clang::Decl::ObjCProperty: return clang_Decl_ObjCProperty_;
    case clang::Decl::BuiltinTemplate: return clang_Decl_BuiltinTemplate_;
    case clang::Decl::Concept: return clang_Decl_Concept_;
    case clang::Decl::ClassTemplate: return clang_Decl_ClassTemplate_;
    case clang::Decl::FunctionTemplate: return clang_Decl_FunctionTemplate_;
    case clang::Decl::TypeAliasTemplate: return clang_Decl_TypeAliasTemplate_;
    case clang::Decl::VarTemplate: return clang_Decl_VarTemplate_;
    case clang::Decl::TemplateTemplateParm: return clang_Decl_TemplateTemplateParm_;
    case clang::Decl::Enum: return clang_Decl_Enum_;
    case clang::Decl::Record: return clang_Decl_Record_;
    case clang::Decl::CXXRecord: return clang_Decl_CXXRecord_;
    case clang::Decl::ClassTemplateSpecialization: return clang_Decl_ClassTemplateSpecialization_;
    case clang::Decl::ClassTemplatePartialSpecialization: return clang_Decl_ClassTemplatePartialSpecialization_;
    case clang::Decl::TemplateTypeParm: return clang_Decl_TemplateTypeParm_;
    case clang::Decl::ObjCTypeParam: return clang_Decl_ObjCTypeParam_;
    case clang::Decl::TypeAlias: return clang_Decl_TypeAlias_;
    case clang::Decl::Typedef: return clang_Decl_Typedef_;
    case clang::Decl::UnresolvedUsingTypename: return clang_Decl_UnresolvedUsingTypename_;
    case clang::Decl::UnresolvedUsingIfExists: return clang_Decl_UnresolvedUsingIfExists_;
    case clang::Decl::UsingDirective: return clang_Decl_UsingDirective_;
    case clang::Decl::UsingPack: return clang_Decl_UsingPack_;
    case clang::Decl::UsingShadow: return clang_Decl_UsingShadow_;
    case clang::Decl::ConstructorUsingShadow: return clang_Decl_ConstructorUsingShadow_;
    case clang::Decl::Binding: return clang_Decl_Binding_;
    case clang::Decl::Field: return clang_Decl_Field_;
    case clang::Decl::ObjCAtDefsField: return clang_Decl_ObjCAtDefsField_;
    case clang::Decl::ObjCIvar: return clang_Decl_ObjCIvar_;
    case clang::Decl::Function: return clang_Decl_Function_;
    case clang::Decl::CXXDeductionGuide: return clang_Decl_CXXDeductionGuide_;
    case clang::Decl::CXXMethod: return clang_Decl_CXXMethod_;
    case clang::Decl::CXXConstructor: return clang_Decl_CXXConstructor_;
    case clang::Decl::CXXConversion: return clang_Decl_CXXConversion_;
    case clang::Decl::CXXDestructor: return clang_Decl_CXXDestructor_;
    case clang::Decl::MSProperty: return clang_Decl_MSProperty_;
    case clang::Decl::NonTypeTemplateParm: return clang_Decl_NonTypeTemplateParm_;
    case clang::Decl::Var: return clang_Decl_Var_;
    case clang::Decl::Decomposition: return clang_Decl_Decomposition_;
    case clang::Decl::ImplicitParam: return clang_Decl_ImplicitParam_;
    case clang::Decl::OMPCapturedExpr: return clang_Decl_OMPCapturedExpr_;
    case clang::Decl::ParmVar: return clang_Decl_ParmVar_;
    case clang::Decl::VarTemplateSpecialization: return clang_Decl_VarTemplateSpecialization_;
    case clang::Decl::VarTemplatePartialSpecialization: return clang_Decl_VarTemplatePartialSpecialization_;
    case clang::Decl::EnumConstant: return clang_Decl_EnumConstant_;
    case clang::Decl::IndirectField: return clang_Decl_IndirectField_;
    case clang::Decl::MSGuid: return clang_Decl_MSGuid_;
    case clang::Decl::OMPDeclareMapper: return clang_Decl_OMPDeclareMapper_;
    case clang::Decl::OMPDeclareReduction: return clang_Decl_OMPDeclareReduction_;
    case clang::Decl::TemplateParamObject: return clang_Decl_TemplateParamObject_;
    case clang::Decl::UnnamedGlobalConstant: return clang_Decl_UnnamedGlobalConstant_;
    case clang::Decl::UnresolvedUsingValue: return clang_Decl_UnresolvedUsingValue_;
    case clang::Decl::OMPAllocate: return clang_Decl_OMPAllocate_;
    case clang::Decl::OMPRequires: return clang_Decl_OMPRequires_;
    case clang::Decl::OMPThreadPrivate: return clang_Decl_OMPThreadPrivate_;
    case clang::Decl::ObjCPropertyImpl: return clang_Decl_ObjCPropertyImpl_;
    case clang::Decl::PragmaComment: return clang_Decl_PragmaComment_;
    case clang::Decl::PragmaDetectMismatch: return clang_Decl_PragmaDetectMismatch_;
    case clang::Decl::RequiresExprBody: return clang_Decl_RequiresExprBody_;
    case clang::Decl::StaticAssert: return clang_Decl_StaticAssert_;
    case clang::Decl::TopLevelStmt: return clang_Decl_TopLevelStmt_;
    case clang::Decl::TranslationUnit: return clang_Decl_TranslationUnit_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::Decl::ModuleOwnershipKind e) {
  switch(e) {
    case clang::Decl::ModuleOwnershipKind::Unowned: return clang_Decl_ModuleOwnershipKind_Unowned_;
    case clang::Decl::ModuleOwnershipKind::Visible: return clang_Decl_ModuleOwnershipKind_Visible_;
    case clang::Decl::ModuleOwnershipKind::VisibleWhenImported: return clang_Decl_ModuleOwnershipKind_VisibleWhenImported_;
    case clang::Decl::ModuleOwnershipKind::ReachableWhenImported: return clang_Decl_ModuleOwnershipKind_ReachableWhenImported_;
    case clang::Decl::ModuleOwnershipKind::ModulePrivate: return clang_Decl_ModuleOwnershipKind_ModulePrivate_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::Decl::ObjCDeclQualifier e) {
  switch(e) {
    case clang::Decl::OBJC_TQ_None: return clang_Decl_OBJC_TQ_None_;
    case clang::Decl::OBJC_TQ_In: return clang_Decl_OBJC_TQ_In_;
    case clang::Decl::OBJC_TQ_Inout: return clang_Decl_OBJC_TQ_Inout_;
    case clang::Decl::OBJC_TQ_Out: return clang_Decl_OBJC_TQ_Out_;
    case clang::Decl::OBJC_TQ_Bycopy: return clang_Decl_OBJC_TQ_Bycopy_;
    case clang::Decl::OBJC_TQ_Byref: return clang_Decl_OBJC_TQ_Byref_;
    case clang::Decl::OBJC_TQ_Oneway: return clang_Decl_OBJC_TQ_Oneway_;
    case clang::Decl::OBJC_TQ_CSNullability: return clang_Decl_OBJC_TQ_CSNullability_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::DeductionCandidate e) {
  switch(e) {
    case clang::DeductionCandidate::Normal: return clang_DeductionCandidate_Normal_;
    case clang::DeductionCandidate::Copy: return clang_DeductionCandidate_Copy_;
    case clang::DeductionCandidate::Aggregate: return clang_DeductionCandidate_Aggregate_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned char>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::ElaboratedTypeKeyword e) {
  switch(e) {
    case clang::ETK_Struct: return clang_ETK_Struct_;
    case clang::ETK_Interface: return clang_ETK_Interface_;
    case clang::ETK_Union: return clang_ETK_Union_;
    case clang::ETK_Class: return clang_ETK_Class_;
    case clang::ETK_Enum: return clang_ETK_Enum_;
    case clang::ETK_Typename: return clang_ETK_Typename_;
    case clang::ETK_None: return clang_ETK_None_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::ExceptionSpecificationType e) {
  switch(e) {
    case clang::EST_None: return clang_EST_None_;
    case clang::EST_DynamicNone: return clang_EST_DynamicNone_;
    case clang::EST_Dynamic: return clang_EST_Dynamic_;
    case clang::EST_MSAny: return clang_EST_MSAny_;
    case clang::EST_NoThrow: return clang_EST_NoThrow_;
    case clang::EST_BasicNoexcept: return clang_EST_BasicNoexcept_;
    case clang::EST_DependentNoexcept: return clang_EST_DependentNoexcept_;
    case clang::EST_NoexceptFalse: return clang_EST_NoexceptFalse_;
    case clang::EST_NoexceptTrue: return clang_EST_NoexceptTrue_;
    case clang::EST_Unevaluated: return clang_EST_Unevaluated_;
    case clang::EST_Uninstantiated: return clang_EST_Uninstantiated_;
    case clang::EST_Unparsed: return clang_EST_Unparsed_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::ExprObjectKind e) {
  switch(e) {
    case clang::OK_Ordinary: return clang_OK_Ordinary_;
    case clang::OK_BitField: return clang_OK_BitField_;
    case clang::OK_VectorComponent: return clang_OK_VectorComponent_;
    case clang::OK_ObjCProperty: return clang_OK_ObjCProperty_;
    case clang::OK_ObjCSubscript: return clang_OK_ObjCSubscript_;
    case clang::OK_MatrixComponent: return clang_OK_MatrixComponent_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::ExprValueKind e) {
  switch(e) {
    case clang::VK_PRValue: return clang_VK_PRValue_;
    case clang::VK_LValue: return clang_VK_LValue_;
    case clang::VK_XValue: return clang_VK_XValue_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::ExpressionTrait e) {
  switch(e) {
    case clang::ET_IsLValueExpr: return clang_ET_IsLValueExpr_;
    case clang::ET_IsRValueExpr: return clang_ET_IsRValueExpr_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::FunctionDecl::TemplatedKind e) {
  switch(e) {
    case clang::FunctionDecl::TK_NonTemplate: return clang_FunctionDecl_TK_NonTemplate_;
    case clang::FunctionDecl::TK_FunctionTemplate: return clang_FunctionDecl_TK_FunctionTemplate_;
    case clang::FunctionDecl::TK_MemberSpecialization: return clang_FunctionDecl_TK_MemberSpecialization_;
    case clang::FunctionDecl::TK_FunctionTemplateSpecialization: return clang_FunctionDecl_TK_FunctionTemplateSpecialization_;
    case clang::FunctionDecl::TK_DependentFunctionTemplateSpecialization: return clang_FunctionDecl_TK_DependentFunctionTemplateSpecialization_;
    case clang::FunctionDecl::TK_DependentNonTemplate: return clang_FunctionDecl_TK_DependentNonTemplate_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::IfStatementKind e) {
  switch(e) {
    case clang::IfStatementKind::Ordinary: return clang_IfStatementKind_Ordinary_;
    case clang::IfStatementKind::Constexpr: return clang_IfStatementKind_Constexpr_;
    case clang::IfStatementKind::ConstevalNonNegated: return clang_IfStatementKind_ConstevalNonNegated_;
    case clang::IfStatementKind::ConstevalNegated: return clang_IfStatementKind_ConstevalNegated_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::ImplicitParamDecl::ImplicitParamKind e) {
  switch(e) {
    case clang::ImplicitParamDecl::ObjCSelf: return clang_ImplicitParamDecl_ObjCSelf_;
    case clang::ImplicitParamDecl::ObjCCmd: return clang_ImplicitParamDecl_ObjCCmd_;
    case clang::ImplicitParamDecl::CXXThis: return clang_ImplicitParamDecl_CXXThis_;
    case clang::ImplicitParamDecl::CXXVTT: return clang_ImplicitParamDecl_CXXVTT_;
    case clang::ImplicitParamDecl::CapturedContext: return clang_ImplicitParamDecl_CapturedContext_;
    case clang::ImplicitParamDecl::ThreadPrivateVar: return clang_ImplicitParamDecl_ThreadPrivateVar_;
    case clang::ImplicitParamDecl::Other: return clang_ImplicitParamDecl_Other_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::InClassInitStyle e) {
  switch(e) {
    case clang::ICIS_NoInit: return clang_ICIS_NoInit_;
    case clang::ICIS_CopyInit: return clang_ICIS_CopyInit_;
    case clang::ICIS_ListInit: return clang_ICIS_ListInit_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::LambdaCaptureDefault e) {
  switch(e) {
    case clang::LCD_None: return clang_LCD_None_;
    case clang::LCD_ByCopy: return clang_LCD_ByCopy_;
    case clang::LCD_ByRef: return clang_LCD_ByRef_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::LanguageLinkage e) {
  switch(e) {
    case clang::CLanguageLinkage: return clang_CLanguageLinkage_;
    case clang::CXXLanguageLinkage: return clang_CXXLanguageLinkage_;
    case clang::NoLanguageLinkage: return clang_NoLanguageLinkage_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::Linkage e) {
  switch(e) {
    case clang::NoLinkage: return clang_NoLinkage_;
    case clang::InternalLinkage: return clang_InternalLinkage_;
    case clang::UniqueExternalLinkage: return clang_UniqueExternalLinkage_;
    case clang::VisibleNoLinkage: return clang_VisibleNoLinkage_;
    case clang::ModuleLinkage: return clang_ModuleLinkage_;
    case clang::ExternalLinkage: return clang_ExternalLinkage_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned char>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::LinkageSpecDecl::LanguageIDs e) {
  switch(e) {
    case clang::LinkageSpecDecl::lang_c: return clang_LinkageSpecDecl_lang_c_;
    case clang::LinkageSpecDecl::lang_cxx: return clang_LinkageSpecDecl_lang_cxx_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::MSVtorDispMode e) {
  switch(e) {
    case clang::MSVtorDispMode::Never: return clang_MSVtorDispMode_Never_;
    case clang::MSVtorDispMode::ForVBaseOverride: return clang_MSVtorDispMode_ForVBaseOverride_;
    case clang::MSVtorDispMode::ForVFTable: return clang_MSVtorDispMode_ForVFTable_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::MultiVersionKind e) {
  switch(e) {
    case clang::MultiVersionKind::None: return clang_MultiVersionKind_None_;
    case clang::MultiVersionKind::Target: return clang_MultiVersionKind_Target_;
    case clang::MultiVersionKind::CPUSpecific: return clang_MultiVersionKind_CPUSpecific_;
    case clang::MultiVersionKind::CPUDispatch: return clang_MultiVersionKind_CPUDispatch_;
    case clang::MultiVersionKind::TargetClones: return clang_MultiVersionKind_TargetClones_;
    case clang::MultiVersionKind::TargetVersion: return clang_MultiVersionKind_TargetVersion_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::NonOdrUseReason e) {
  switch(e) {
    case clang::NOUR_None: return clang_NOUR_None_;
    case clang::NOUR_Unevaluated: return clang_NOUR_Unevaluated_;
    case clang::NOUR_Constant: return clang_NOUR_Constant_;
    case clang::NOUR_Discarded: return clang_NOUR_Discarded_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::ObjCStringFormatFamily e) {
  switch(e) {
    case clang::SFF_None: return clang_SFF_None_;
    case clang::SFF_NSString: return clang_SFF_NSString_;
    case clang::SFF_CFString: return clang_SFF_CFString_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::OverloadedOperatorKind e) {
  switch(e) {
    case clang::OO_None: return clang_OO_None_;
    case clang::OO_New: return clang_OO_New_;
    case clang::OO_Delete: return clang_OO_Delete_;
    case clang::OO_Array_New: return clang_OO_Array_New_;
    case clang::OO_Array_Delete: return clang_OO_Array_Delete_;
    case clang::OO_Plus: return clang_OO_Plus_;
    case clang::OO_Minus: return clang_OO_Minus_;
    case clang::OO_Star: return clang_OO_Star_;
    case clang::OO_Slash: return clang_OO_Slash_;
    case clang::OO_Percent: return clang_OO_Percent_;
    case clang::OO_Caret: return clang_OO_Caret_;
    case clang::OO_Amp: return clang_OO_Amp_;
    case clang::OO_Pipe: return clang_OO_Pipe_;
    case clang::OO_Tilde: return clang_OO_Tilde_;
    case clang::OO_Exclaim: return clang_OO_Exclaim_;
    case clang::OO_Equal: return clang_OO_Equal_;
    case clang::OO_Less: return clang_OO_Less_;
    case clang::OO_Greater: return clang_OO_Greater_;
    case clang::OO_PlusEqual: return clang_OO_PlusEqual_;
    case clang::OO_MinusEqual: return clang_OO_MinusEqual_;
    case clang::OO_StarEqual: return clang_OO_StarEqual_;
    case clang::OO_SlashEqual: return clang_OO_SlashEqual_;
    case clang::OO_PercentEqual: return clang_OO_PercentEqual_;
    case clang::OO_CaretEqual: return clang_OO_CaretEqual_;
    case clang::OO_AmpEqual: return clang_OO_AmpEqual_;
    case clang::OO_PipeEqual: return clang_OO_PipeEqual_;
    case clang::OO_LessLess: return clang_OO_LessLess_;
    case clang::OO_GreaterGreater: return clang_OO_GreaterGreater_;
    case clang::OO_LessLessEqual: return clang_OO_LessLessEqual_;
    case clang::OO_GreaterGreaterEqual: return clang_OO_GreaterGreaterEqual_;
    case clang::OO_EqualEqual: return clang_OO_EqualEqual_;
    case clang::OO_ExclaimEqual: return clang_OO_ExclaimEqual_;
    case clang::OO_LessEqual: return clang_OO_LessEqual_;
    case clang::OO_GreaterEqual: return clang_OO_GreaterEqual_;
    case clang::OO_Spaceship: return clang_OO_Spaceship_;
    case clang::OO_AmpAmp: return clang_OO_AmpAmp_;
    case clang::OO_PipePipe: return clang_OO_PipePipe_;
    case clang::OO_PlusPlus: return clang_OO_PlusPlus_;
    case clang::OO_MinusMinus: return clang_OO_MinusMinus_;
    case clang::OO_Comma: return clang_OO_Comma_;
    case clang::OO_ArrowStar: return clang_OO_ArrowStar_;
    case clang::OO_Arrow: return clang_OO_Arrow_;
    case clang::OO_Call: return clang_OO_Call_;
    case clang::OO_Subscript: return clang_OO_Subscript_;
    case clang::OO_Conditional: return clang_OO_Conditional_;
    case clang::OO_Coawait: return clang_OO_Coawait_;
    case clang::NUM_OVERLOADED_OPERATORS: return clang_NUM_OVERLOADED_OPERATORS_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::PragmaMSCommentKind e) {
  switch(e) {
    case clang::PCK_Unknown: return clang_PCK_Unknown_;
    case clang::PCK_Linker: return clang_PCK_Linker_;
    case clang::PCK_Lib: return clang_PCK_Lib_;
    case clang::PCK_Compiler: return clang_PCK_Compiler_;
    case clang::PCK_ExeStr: return clang_PCK_ExeStr_;
    case clang::PCK_User: return clang_PCK_User_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::PredefinedExpr::IdentKind e) {
  switch(e) {
    case clang::PredefinedExpr::Func: return clang_PredefinedExpr_Func_;
    case clang::PredefinedExpr::Function: return clang_PredefinedExpr_Function_;
    case clang::PredefinedExpr::LFunction: return clang_PredefinedExpr_LFunction_;
    case clang::PredefinedExpr::FuncDName: return clang_PredefinedExpr_FuncDName_;
    case clang::PredefinedExpr::FuncSig: return clang_PredefinedExpr_FuncSig_;
    case clang::PredefinedExpr::LFuncSig: return clang_PredefinedExpr_LFuncSig_;
    case clang::PredefinedExpr::PrettyFunction: return clang_PredefinedExpr_PrettyFunction_;
    case clang::PredefinedExpr::PrettyFunctionNoVirtual: return clang_PredefinedExpr_PrettyFunctionNoVirtual_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::Qualifiers::ObjCLifetime e) {
  switch(e) {
    case clang::Qualifiers::OCL_None: return clang_Qualifiers_OCL_None_;
    case clang::Qualifiers::OCL_ExplicitNone: return clang_Qualifiers_OCL_ExplicitNone_;
    case clang::Qualifiers::OCL_Strong: return clang_Qualifiers_OCL_Strong_;
    case clang::Qualifiers::OCL_Weak: return clang_Qualifiers_OCL_Weak_;
    case clang::Qualifiers::OCL_Autoreleasing: return clang_Qualifiers_OCL_Autoreleasing_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::RecordDecl::ArgPassingKind e) {
  switch(e) {
    case clang::RecordDecl::APK_CanPassInRegs: return clang_RecordDecl_APK_CanPassInRegs_;
    case clang::RecordDecl::APK_CannotPassInRegs: return clang_RecordDecl_APK_CannotPassInRegs_;
    case clang::RecordDecl::APK_CanNeverPassInRegs: return clang_RecordDecl_APK_CanNeverPassInRegs_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::RefQualifierKind e) {
  switch(e) {
    case clang::RQ_None: return clang_RQ_None_;
    case clang::RQ_LValue: return clang_RQ_LValue_;
    case clang::RQ_RValue: return clang_RQ_RValue_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::SourceLocExpr::IdentKind e) {
  switch(e) {
    case clang::SourceLocExpr::Function: return clang_SourceLocExpr_Function_;
    case clang::SourceLocExpr::FuncSig: return clang_SourceLocExpr_FuncSig_;
    case clang::SourceLocExpr::File: return clang_SourceLocExpr_File_;
    case clang::SourceLocExpr::FileName: return clang_SourceLocExpr_FileName_;
    case clang::SourceLocExpr::Line: return clang_SourceLocExpr_Line_;
    case clang::SourceLocExpr::Column: return clang_SourceLocExpr_Column_;
    case clang::SourceLocExpr::SourceLocStruct: return clang_SourceLocExpr_SourceLocStruct_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::Stmt::StmtClass e) {
  switch(e) {
    case clang::Stmt::NoStmtClass: return clang_Stmt_NoStmtClass_;
    case clang::Stmt::GCCAsmStmtClass: return clang_Stmt_GCCAsmStmtClass_;
    case clang::Stmt::MSAsmStmtClass: return clang_Stmt_MSAsmStmtClass_;
    case clang::Stmt::BreakStmtClass: return clang_Stmt_BreakStmtClass_;
    case clang::Stmt::CXXCatchStmtClass: return clang_Stmt_CXXCatchStmtClass_;
    case clang::Stmt::CXXForRangeStmtClass: return clang_Stmt_CXXForRangeStmtClass_;
    case clang::Stmt::CXXTryStmtClass: return clang_Stmt_CXXTryStmtClass_;
    case clang::Stmt::CapturedStmtClass: return clang_Stmt_CapturedStmtClass_;
    case clang::Stmt::CompoundStmtClass: return clang_Stmt_CompoundStmtClass_;
    case clang::Stmt::ContinueStmtClass: return clang_Stmt_ContinueStmtClass_;
    case clang::Stmt::CoreturnStmtClass: return clang_Stmt_CoreturnStmtClass_;
    case clang::Stmt::CoroutineBodyStmtClass: return clang_Stmt_CoroutineBodyStmtClass_;
    case clang::Stmt::DeclStmtClass: return clang_Stmt_DeclStmtClass_;
    case clang::Stmt::DoStmtClass: return clang_Stmt_DoStmtClass_;
    case clang::Stmt::ForStmtClass: return clang_Stmt_ForStmtClass_;
    case clang::Stmt::GotoStmtClass: return clang_Stmt_GotoStmtClass_;
    case clang::Stmt::IfStmtClass: return clang_Stmt_IfStmtClass_;
    case clang::Stmt::IndirectGotoStmtClass: return clang_Stmt_IndirectGotoStmtClass_;
    case clang::Stmt::MSDependentExistsStmtClass: return clang_Stmt_MSDependentExistsStmtClass_;
    case clang::Stmt::NullStmtClass: return clang_Stmt_NullStmtClass_;
    case clang::Stmt::OMPCanonicalLoopClass: return clang_Stmt_OMPCanonicalLoopClass_;
    case clang::Stmt::OMPAtomicDirectiveClass: return clang_Stmt_OMPAtomicDirectiveClass_;
    case clang::Stmt::OMPBarrierDirectiveClass: return clang_Stmt_OMPBarrierDirectiveClass_;
    case clang::Stmt::OMPCancelDirectiveClass: return clang_Stmt_OMPCancelDirectiveClass_;
    case clang::Stmt::OMPCancellationPointDirectiveClass: return clang_Stmt_OMPCancellationPointDirectiveClass_;
    case clang::Stmt::OMPCriticalDirectiveClass: return clang_Stmt_OMPCriticalDirectiveClass_;
    case clang::Stmt::OMPDepobjDirectiveClass: return clang_Stmt_OMPDepobjDirectiveClass_;
    case clang::Stmt::OMPDispatchDirectiveClass: return clang_Stmt_OMPDispatchDirectiveClass_;
    case clang::Stmt::OMPErrorDirectiveClass: return clang_Stmt_OMPErrorDirectiveClass_;
    case clang::Stmt::OMPFlushDirectiveClass: return clang_Stmt_OMPFlushDirectiveClass_;
    case clang::Stmt::OMPInteropDirectiveClass: return clang_Stmt_OMPInteropDirectiveClass_;
    case clang::Stmt::OMPDistributeDirectiveClass: return clang_Stmt_OMPDistributeDirectiveClass_;
    case clang::Stmt::OMPDistributeParallelForDirectiveClass: return clang_Stmt_OMPDistributeParallelForDirectiveClass_;
    case clang::Stmt::OMPDistributeParallelForSimdDirectiveClass: return clang_Stmt_OMPDistributeParallelForSimdDirectiveClass_;
    case clang::Stmt::OMPDistributeSimdDirectiveClass: return clang_Stmt_OMPDistributeSimdDirectiveClass_;
    case clang::Stmt::OMPForDirectiveClass: return clang_Stmt_OMPForDirectiveClass_;
    case clang::Stmt::OMPForSimdDirectiveClass: return clang_Stmt_OMPForSimdDirectiveClass_;
    case clang::Stmt::OMPGenericLoopDirectiveClass: return clang_Stmt_OMPGenericLoopDirectiveClass_;
    case clang::Stmt::OMPMaskedTaskLoopDirectiveClass: return clang_Stmt_OMPMaskedTaskLoopDirectiveClass_;
    case clang::Stmt::OMPMaskedTaskLoopSimdDirectiveClass: return clang_Stmt_OMPMaskedTaskLoopSimdDirectiveClass_;
    case clang::Stmt::OMPMasterTaskLoopDirectiveClass: return clang_Stmt_OMPMasterTaskLoopDirectiveClass_;
    case clang::Stmt::OMPMasterTaskLoopSimdDirectiveClass: return clang_Stmt_OMPMasterTaskLoopSimdDirectiveClass_;
    case clang::Stmt::OMPParallelForDirectiveClass: return clang_Stmt_OMPParallelForDirectiveClass_;
    case clang::Stmt::OMPParallelForSimdDirectiveClass: return clang_Stmt_OMPParallelForSimdDirectiveClass_;
    case clang::Stmt::OMPParallelGenericLoopDirectiveClass: return clang_Stmt_OMPParallelGenericLoopDirectiveClass_;
    case clang::Stmt::OMPParallelMaskedTaskLoopDirectiveClass: return clang_Stmt_OMPParallelMaskedTaskLoopDirectiveClass_;
    case clang::Stmt::OMPParallelMaskedTaskLoopSimdDirectiveClass: return clang_Stmt_OMPParallelMaskedTaskLoopSimdDirectiveClass_;
    case clang::Stmt::OMPParallelMasterTaskLoopDirectiveClass: return clang_Stmt_OMPParallelMasterTaskLoopDirectiveClass_;
    case clang::Stmt::OMPParallelMasterTaskLoopSimdDirectiveClass: return clang_Stmt_OMPParallelMasterTaskLoopSimdDirectiveClass_;
    case clang::Stmt::OMPSimdDirectiveClass: return clang_Stmt_OMPSimdDirectiveClass_;
    case clang::Stmt::OMPTargetParallelForSimdDirectiveClass: return clang_Stmt_OMPTargetParallelForSimdDirectiveClass_;
    case clang::Stmt::OMPTargetParallelGenericLoopDirectiveClass: return clang_Stmt_OMPTargetParallelGenericLoopDirectiveClass_;
    case clang::Stmt::OMPTargetSimdDirectiveClass: return clang_Stmt_OMPTargetSimdDirectiveClass_;
    case clang::Stmt::OMPTargetTeamsDistributeDirectiveClass: return clang_Stmt_OMPTargetTeamsDistributeDirectiveClass_;
    case clang::Stmt::OMPTargetTeamsDistributeParallelForDirectiveClass: return clang_Stmt_OMPTargetTeamsDistributeParallelForDirectiveClass_;
    case clang::Stmt::OMPTargetTeamsDistributeParallelForSimdDirectiveClass: return clang_Stmt_OMPTargetTeamsDistributeParallelForSimdDirectiveClass_;
    case clang::Stmt::OMPTargetTeamsDistributeSimdDirectiveClass: return clang_Stmt_OMPTargetTeamsDistributeSimdDirectiveClass_;
    case clang::Stmt::OMPTargetTeamsGenericLoopDirectiveClass: return clang_Stmt_OMPTargetTeamsGenericLoopDirectiveClass_;
    case clang::Stmt::OMPTaskLoopDirectiveClass: return clang_Stmt_OMPTaskLoopDirectiveClass_;
    case clang::Stmt::OMPTaskLoopSimdDirectiveClass: return clang_Stmt_OMPTaskLoopSimdDirectiveClass_;
    case clang::Stmt::OMPTeamsDistributeDirectiveClass: return clang_Stmt_OMPTeamsDistributeDirectiveClass_;
    case clang::Stmt::OMPTeamsDistributeParallelForDirectiveClass: return clang_Stmt_OMPTeamsDistributeParallelForDirectiveClass_;
    case clang::Stmt::OMPTeamsDistributeParallelForSimdDirectiveClass: return clang_Stmt_OMPTeamsDistributeParallelForSimdDirectiveClass_;
    case clang::Stmt::OMPTeamsDistributeSimdDirectiveClass: return clang_Stmt_OMPTeamsDistributeSimdDirectiveClass_;
    case clang::Stmt::OMPTeamsGenericLoopDirectiveClass: return clang_Stmt_OMPTeamsGenericLoopDirectiveClass_;
    case clang::Stmt::OMPTileDirectiveClass: return clang_Stmt_OMPTileDirectiveClass_;
    case clang::Stmt::OMPUnrollDirectiveClass: return clang_Stmt_OMPUnrollDirectiveClass_;
    case clang::Stmt::OMPMaskedDirectiveClass: return clang_Stmt_OMPMaskedDirectiveClass_;
    case clang::Stmt::OMPMasterDirectiveClass: return clang_Stmt_OMPMasterDirectiveClass_;
    case clang::Stmt::OMPMetaDirectiveClass: return clang_Stmt_OMPMetaDirectiveClass_;
    case clang::Stmt::OMPOrderedDirectiveClass: return clang_Stmt_OMPOrderedDirectiveClass_;
    case clang::Stmt::OMPParallelDirectiveClass: return clang_Stmt_OMPParallelDirectiveClass_;
    case clang::Stmt::OMPParallelMaskedDirectiveClass: return clang_Stmt_OMPParallelMaskedDirectiveClass_;
    case clang::Stmt::OMPParallelMasterDirectiveClass: return clang_Stmt_OMPParallelMasterDirectiveClass_;
    case clang::Stmt::OMPParallelSectionsDirectiveClass: return clang_Stmt_OMPParallelSectionsDirectiveClass_;
    case clang::Stmt::OMPScanDirectiveClass: return clang_Stmt_OMPScanDirectiveClass_;
    case clang::Stmt::OMPSectionDirectiveClass: return clang_Stmt_OMPSectionDirectiveClass_;
    case clang::Stmt::OMPSectionsDirectiveClass: return clang_Stmt_OMPSectionsDirectiveClass_;
    case clang::Stmt::OMPSingleDirectiveClass: return clang_Stmt_OMPSingleDirectiveClass_;
    case clang::Stmt::OMPTargetDataDirectiveClass: return clang_Stmt_OMPTargetDataDirectiveClass_;
    case clang::Stmt::OMPTargetDirectiveClass: return clang_Stmt_OMPTargetDirectiveClass_;
    case clang::Stmt::OMPTargetEnterDataDirectiveClass: return clang_Stmt_OMPTargetEnterDataDirectiveClass_;
    case clang::Stmt::OMPTargetExitDataDirectiveClass: return clang_Stmt_OMPTargetExitDataDirectiveClass_;
    case clang::Stmt::OMPTargetParallelDirectiveClass: return clang_Stmt_OMPTargetParallelDirectiveClass_;
    case clang::Stmt::OMPTargetParallelForDirectiveClass: return clang_Stmt_OMPTargetParallelForDirectiveClass_;
    case clang::Stmt::OMPTargetTeamsDirectiveClass: return clang_Stmt_OMPTargetTeamsDirectiveClass_;
    case clang::Stmt::OMPTargetUpdateDirectiveClass: return clang_Stmt_OMPTargetUpdateDirectiveClass_;
    case clang::Stmt::OMPTaskDirectiveClass: return clang_Stmt_OMPTaskDirectiveClass_;
    case clang::Stmt::OMPTaskgroupDirectiveClass: return clang_Stmt_OMPTaskgroupDirectiveClass_;
    case clang::Stmt::OMPTaskwaitDirectiveClass: return clang_Stmt_OMPTaskwaitDirectiveClass_;
    case clang::Stmt::OMPTaskyieldDirectiveClass: return clang_Stmt_OMPTaskyieldDirectiveClass_;
    case clang::Stmt::OMPTeamsDirectiveClass: return clang_Stmt_OMPTeamsDirectiveClass_;
    case clang::Stmt::ObjCAtCatchStmtClass: return clang_Stmt_ObjCAtCatchStmtClass_;
    case clang::Stmt::ObjCAtFinallyStmtClass: return clang_Stmt_ObjCAtFinallyStmtClass_;
    case clang::Stmt::ObjCAtSynchronizedStmtClass: return clang_Stmt_ObjCAtSynchronizedStmtClass_;
    case clang::Stmt::ObjCAtThrowStmtClass: return clang_Stmt_ObjCAtThrowStmtClass_;
    case clang::Stmt::ObjCAtTryStmtClass: return clang_Stmt_ObjCAtTryStmtClass_;
    case clang::Stmt::ObjCAutoreleasePoolStmtClass: return clang_Stmt_ObjCAutoreleasePoolStmtClass_;
    case clang::Stmt::ObjCForCollectionStmtClass: return clang_Stmt_ObjCForCollectionStmtClass_;
    case clang::Stmt::ReturnStmtClass: return clang_Stmt_ReturnStmtClass_;
    case clang::Stmt::SEHExceptStmtClass: return clang_Stmt_SEHExceptStmtClass_;
    case clang::Stmt::SEHFinallyStmtClass: return clang_Stmt_SEHFinallyStmtClass_;
    case clang::Stmt::SEHLeaveStmtClass: return clang_Stmt_SEHLeaveStmtClass_;
    case clang::Stmt::SEHTryStmtClass: return clang_Stmt_SEHTryStmtClass_;
    case clang::Stmt::CaseStmtClass: return clang_Stmt_CaseStmtClass_;
    case clang::Stmt::DefaultStmtClass: return clang_Stmt_DefaultStmtClass_;
    case clang::Stmt::SwitchStmtClass: return clang_Stmt_SwitchStmtClass_;
    case clang::Stmt::AttributedStmtClass: return clang_Stmt_AttributedStmtClass_;
    case clang::Stmt::BinaryConditionalOperatorClass: return clang_Stmt_BinaryConditionalOperatorClass_;
    case clang::Stmt::ConditionalOperatorClass: return clang_Stmt_ConditionalOperatorClass_;
    case clang::Stmt::AddrLabelExprClass: return clang_Stmt_AddrLabelExprClass_;
    case clang::Stmt::ArrayInitIndexExprClass: return clang_Stmt_ArrayInitIndexExprClass_;
    case clang::Stmt::ArrayInitLoopExprClass: return clang_Stmt_ArrayInitLoopExprClass_;
    case clang::Stmt::ArraySubscriptExprClass: return clang_Stmt_ArraySubscriptExprClass_;
    case clang::Stmt::ArrayTypeTraitExprClass: return clang_Stmt_ArrayTypeTraitExprClass_;
    case clang::Stmt::AsTypeExprClass: return clang_Stmt_AsTypeExprClass_;
    case clang::Stmt::AtomicExprClass: return clang_Stmt_AtomicExprClass_;
    case clang::Stmt::BinaryOperatorClass: return clang_Stmt_BinaryOperatorClass_;
    case clang::Stmt::CompoundAssignOperatorClass: return clang_Stmt_CompoundAssignOperatorClass_;
    case clang::Stmt::BlockExprClass: return clang_Stmt_BlockExprClass_;
    case clang::Stmt::CXXBindTemporaryExprClass: return clang_Stmt_CXXBindTemporaryExprClass_;
    case clang::Stmt::CXXBoolLiteralExprClass: return clang_Stmt_CXXBoolLiteralExprClass_;
    case clang::Stmt::CXXConstructExprClass: return clang_Stmt_CXXConstructExprClass_;
    case clang::Stmt::CXXTemporaryObjectExprClass: return clang_Stmt_CXXTemporaryObjectExprClass_;
    case clang::Stmt::CXXDefaultArgExprClass: return clang_Stmt_CXXDefaultArgExprClass_;
    case clang::Stmt::CXXDefaultInitExprClass: return clang_Stmt_CXXDefaultInitExprClass_;
    case clang::Stmt::CXXDeleteExprClass: return clang_Stmt_CXXDeleteExprClass_;
    case clang::Stmt::CXXDependentScopeMemberExprClass: return clang_Stmt_CXXDependentScopeMemberExprClass_;
    case clang::Stmt::CXXFoldExprClass: return clang_Stmt_CXXFoldExprClass_;
    case clang::Stmt::CXXInheritedCtorInitExprClass: return clang_Stmt_CXXInheritedCtorInitExprClass_;
    case clang::Stmt::CXXNewExprClass: return clang_Stmt_CXXNewExprClass_;
    case clang::Stmt::CXXNoexceptExprClass: return clang_Stmt_CXXNoexceptExprClass_;
    case clang::Stmt::CXXNullPtrLiteralExprClass: return clang_Stmt_CXXNullPtrLiteralExprClass_;
    case clang::Stmt::CXXParenListInitExprClass: return clang_Stmt_CXXParenListInitExprClass_;
    case clang::Stmt::CXXPseudoDestructorExprClass: return clang_Stmt_CXXPseudoDestructorExprClass_;
    case clang::Stmt::CXXRewrittenBinaryOperatorClass: return clang_Stmt_CXXRewrittenBinaryOperatorClass_;
    case clang::Stmt::CXXScalarValueInitExprClass: return clang_Stmt_CXXScalarValueInitExprClass_;
    case clang::Stmt::CXXStdInitializerListExprClass: return clang_Stmt_CXXStdInitializerListExprClass_;
    case clang::Stmt::CXXThisExprClass: return clang_Stmt_CXXThisExprClass_;
    case clang::Stmt::CXXThrowExprClass: return clang_Stmt_CXXThrowExprClass_;
    case clang::Stmt::CXXTypeidExprClass: return clang_Stmt_CXXTypeidExprClass_;
    case clang::Stmt::CXXUnresolvedConstructExprClass: return clang_Stmt_CXXUnresolvedConstructExprClass_;
    case clang::Stmt::CXXUuidofExprClass: return clang_Stmt_CXXUuidofExprClass_;
    case clang::Stmt::CallExprClass: return clang_Stmt_CallExprClass_;
    case clang::Stmt::CUDAKernelCallExprClass: return clang_Stmt_CUDAKernelCallExprClass_;
    case clang::Stmt::CXXMemberCallExprClass: return clang_Stmt_CXXMemberCallExprClass_;
    case clang::Stmt::CXXOperatorCallExprClass: return clang_Stmt_CXXOperatorCallExprClass_;
    case clang::Stmt::UserDefinedLiteralClass: return clang_Stmt_UserDefinedLiteralClass_;
    case clang::Stmt::BuiltinBitCastExprClass: return clang_Stmt_BuiltinBitCastExprClass_;
    case clang::Stmt::CStyleCastExprClass: return clang_Stmt_CStyleCastExprClass_;
    case clang::Stmt::CXXFunctionalCastExprClass: return clang_Stmt_CXXFunctionalCastExprClass_;
    case clang::Stmt::CXXAddrspaceCastExprClass: return clang_Stmt_CXXAddrspaceCastExprClass_;
    case clang::Stmt::CXXConstCastExprClass: return clang_Stmt_CXXConstCastExprClass_;
    case clang::Stmt::CXXDynamicCastExprClass: return clang_Stmt_CXXDynamicCastExprClass_;
    case clang::Stmt::CXXReinterpretCastExprClass: return clang_Stmt_CXXReinterpretCastExprClass_;
    case clang::Stmt::CXXStaticCastExprClass: return clang_Stmt_CXXStaticCastExprClass_;
    case clang::Stmt::ObjCBridgedCastExprClass: return clang_Stmt_ObjCBridgedCastExprClass_;
    case clang::Stmt::ImplicitCastExprClass: return clang_Stmt_ImplicitCastExprClass_;
    case clang::Stmt::CharacterLiteralClass: return clang_Stmt_CharacterLiteralClass_;
    case clang::Stmt::ChooseExprClass: return clang_Stmt_ChooseExprClass_;
    case clang::Stmt::CompoundLiteralExprClass: return clang_Stmt_CompoundLiteralExprClass_;
    case clang::Stmt::ConceptSpecializationExprClass: return clang_Stmt_ConceptSpecializationExprClass_;
    case clang::Stmt::ConvertVectorExprClass: return clang_Stmt_ConvertVectorExprClass_;
    case clang::Stmt::CoawaitExprClass: return clang_Stmt_CoawaitExprClass_;
    case clang::Stmt::CoyieldExprClass: return clang_Stmt_CoyieldExprClass_;
    case clang::Stmt::DeclRefExprClass: return clang_Stmt_DeclRefExprClass_;
    case clang::Stmt::DependentCoawaitExprClass: return clang_Stmt_DependentCoawaitExprClass_;
    case clang::Stmt::DependentScopeDeclRefExprClass: return clang_Stmt_DependentScopeDeclRefExprClass_;
    case clang::Stmt::DesignatedInitExprClass: return clang_Stmt_DesignatedInitExprClass_;
    case clang::Stmt::DesignatedInitUpdateExprClass: return clang_Stmt_DesignatedInitUpdateExprClass_;
    case clang::Stmt::ExpressionTraitExprClass: return clang_Stmt_ExpressionTraitExprClass_;
    case clang::Stmt::ExtVectorElementExprClass: return clang_Stmt_ExtVectorElementExprClass_;
    case clang::Stmt::FixedPointLiteralClass: return clang_Stmt_FixedPointLiteralClass_;
    case clang::Stmt::FloatingLiteralClass: return clang_Stmt_FloatingLiteralClass_;
    case clang::Stmt::ConstantExprClass: return clang_Stmt_ConstantExprClass_;
    case clang::Stmt::ExprWithCleanupsClass: return clang_Stmt_ExprWithCleanupsClass_;
    case clang::Stmt::FunctionParmPackExprClass: return clang_Stmt_FunctionParmPackExprClass_;
    case clang::Stmt::GNUNullExprClass: return clang_Stmt_GNUNullExprClass_;
    case clang::Stmt::GenericSelectionExprClass: return clang_Stmt_GenericSelectionExprClass_;
    case clang::Stmt::ImaginaryLiteralClass: return clang_Stmt_ImaginaryLiteralClass_;
    case clang::Stmt::ImplicitValueInitExprClass: return clang_Stmt_ImplicitValueInitExprClass_;
    case clang::Stmt::InitListExprClass: return clang_Stmt_InitListExprClass_;
    case clang::Stmt::IntegerLiteralClass: return clang_Stmt_IntegerLiteralClass_;
    case clang::Stmt::LambdaExprClass: return clang_Stmt_LambdaExprClass_;
    case clang::Stmt::MSPropertyRefExprClass: return clang_Stmt_MSPropertyRefExprClass_;
    case clang::Stmt::MSPropertySubscriptExprClass: return clang_Stmt_MSPropertySubscriptExprClass_;
    case clang::Stmt::MaterializeTemporaryExprClass: return clang_Stmt_MaterializeTemporaryExprClass_;
    case clang::Stmt::MatrixSubscriptExprClass: return clang_Stmt_MatrixSubscriptExprClass_;
    case clang::Stmt::MemberExprClass: return clang_Stmt_MemberExprClass_;
    case clang::Stmt::NoInitExprClass: return clang_Stmt_NoInitExprClass_;
    case clang::Stmt::OMPArraySectionExprClass: return clang_Stmt_OMPArraySectionExprClass_;
    case clang::Stmt::OMPArrayShapingExprClass: return clang_Stmt_OMPArrayShapingExprClass_;
    case clang::Stmt::OMPIteratorExprClass: return clang_Stmt_OMPIteratorExprClass_;
    case clang::Stmt::ObjCArrayLiteralClass: return clang_Stmt_ObjCArrayLiteralClass_;
    case clang::Stmt::ObjCAvailabilityCheckExprClass: return clang_Stmt_ObjCAvailabilityCheckExprClass_;
    case clang::Stmt::ObjCBoolLiteralExprClass: return clang_Stmt_ObjCBoolLiteralExprClass_;
    case clang::Stmt::ObjCBoxedExprClass: return clang_Stmt_ObjCBoxedExprClass_;
    case clang::Stmt::ObjCDictionaryLiteralClass: return clang_Stmt_ObjCDictionaryLiteralClass_;
    case clang::Stmt::ObjCEncodeExprClass: return clang_Stmt_ObjCEncodeExprClass_;
    case clang::Stmt::ObjCIndirectCopyRestoreExprClass: return clang_Stmt_ObjCIndirectCopyRestoreExprClass_;
    case clang::Stmt::ObjCIsaExprClass: return clang_Stmt_ObjCIsaExprClass_;
    case clang::Stmt::ObjCIvarRefExprClass: return clang_Stmt_ObjCIvarRefExprClass_;
    case clang::Stmt::ObjCMessageExprClass: return clang_Stmt_ObjCMessageExprClass_;
    case clang::Stmt::ObjCPropertyRefExprClass: return clang_Stmt_ObjCPropertyRefExprClass_;
    case clang::Stmt::ObjCProtocolExprClass: return clang_Stmt_ObjCProtocolExprClass_;
    case clang::Stmt::ObjCSelectorExprClass: return clang_Stmt_ObjCSelectorExprClass_;
    case clang::Stmt::ObjCStringLiteralClass: return clang_Stmt_ObjCStringLiteralClass_;
    case clang::Stmt::ObjCSubscriptRefExprClass: return clang_Stmt_ObjCSubscriptRefExprClass_;
    case clang::Stmt::OffsetOfExprClass: return clang_Stmt_OffsetOfExprClass_;
    case clang::Stmt::OpaqueValueExprClass: return clang_Stmt_OpaqueValueExprClass_;
    case clang::Stmt::UnresolvedLookupExprClass: return clang_Stmt_UnresolvedLookupExprClass_;
    case clang::Stmt::UnresolvedMemberExprClass: return clang_Stmt_UnresolvedMemberExprClass_;
    case clang::Stmt::PackExpansionExprClass: return clang_Stmt_PackExpansionExprClass_;
    case clang::Stmt::ParenExprClass: return clang_Stmt_ParenExprClass_;
    case clang::Stmt::ParenListExprClass: return clang_Stmt_ParenListExprClass_;
    case clang::Stmt::PredefinedExprClass: return clang_Stmt_PredefinedExprClass_;
    case clang::Stmt::PseudoObjectExprClass: return clang_Stmt_PseudoObjectExprClass_;
    case clang::Stmt::RecoveryExprClass: return clang_Stmt_RecoveryExprClass_;
    case clang::Stmt::RequiresExprClass: return clang_Stmt_RequiresExprClass_;
    case clang::Stmt::SYCLUniqueStableNameExprClass: return clang_Stmt_SYCLUniqueStableNameExprClass_;
    case clang::Stmt::ShuffleVectorExprClass: return clang_Stmt_ShuffleVectorExprClass_;
    case clang::Stmt::SizeOfPackExprClass: return clang_Stmt_SizeOfPackExprClass_;
    case clang::Stmt::SourceLocExprClass: return clang_Stmt_SourceLocExprClass_;
    case clang::Stmt::StmtExprClass: return clang_Stmt_StmtExprClass_;
    case clang::Stmt::StringLiteralClass: return clang_Stmt_StringLiteralClass_;
    case clang::Stmt::SubstNonTypeTemplateParmExprClass: return clang_Stmt_SubstNonTypeTemplateParmExprClass_;
    case clang::Stmt::SubstNonTypeTemplateParmPackExprClass: return clang_Stmt_SubstNonTypeTemplateParmPackExprClass_;
    case clang::Stmt::TypeTraitExprClass: return clang_Stmt_TypeTraitExprClass_;
    case clang::Stmt::TypoExprClass: return clang_Stmt_TypoExprClass_;
    case clang::Stmt::UnaryExprOrTypeTraitExprClass: return clang_Stmt_UnaryExprOrTypeTraitExprClass_;
    case clang::Stmt::UnaryOperatorClass: return clang_Stmt_UnaryOperatorClass_;
    case clang::Stmt::VAArgExprClass: return clang_Stmt_VAArgExprClass_;
    case clang::Stmt::LabelStmtClass: return clang_Stmt_LabelStmtClass_;
    case clang::Stmt::WhileStmtClass: return clang_Stmt_WhileStmtClass_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::StorageClass e) {
  switch(e) {
    case clang::SC_None: return clang_SC_None_;
    case clang::SC_Extern: return clang_SC_Extern_;
    case clang::SC_Static: return clang_SC_Static_;
    case clang::SC_PrivateExtern: return clang_SC_PrivateExtern_;
    case clang::SC_Auto: return clang_SC_Auto_;
    case clang::SC_Register: return clang_SC_Register_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::StorageDuration e) {
  switch(e) {
    case clang::SD_FullExpression: return clang_SD_FullExpression_;
    case clang::SD_Automatic: return clang_SD_Automatic_;
    case clang::SD_Thread: return clang_SD_Thread_;
    case clang::SD_Static: return clang_SD_Static_;
    case clang::SD_Dynamic: return clang_SD_Dynamic_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::StringLiteral::StringKind e) {
  switch(e) {
    case clang::StringLiteral::Ordinary: return clang_StringLiteral_Ordinary_;
    case clang::StringLiteral::Wide: return clang_StringLiteral_Wide_;
    case clang::StringLiteral::UTF8: return clang_StringLiteral_UTF8_;
    case clang::StringLiteral::UTF16: return clang_StringLiteral_UTF16_;
    case clang::StringLiteral::UTF32: return clang_StringLiteral_UTF32_;
    case clang::StringLiteral::Unevaluated: return clang_StringLiteral_Unevaluated_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::TagTypeKind e) {
  switch(e) {
    case clang::TTK_Struct: return clang_TTK_Struct_;
    case clang::TTK_Interface: return clang_TTK_Interface_;
    case clang::TTK_Union: return clang_TTK_Union_;
    case clang::TTK_Class: return clang_TTK_Class_;
    case clang::TTK_Enum: return clang_TTK_Enum_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::TemplateSpecializationKind e) {
  switch(e) {
    case clang::TSK_Undeclared: return clang_TSK_Undeclared_;
    case clang::TSK_ImplicitInstantiation: return clang_TSK_ImplicitInstantiation_;
    case clang::TSK_ExplicitSpecialization: return clang_TSK_ExplicitSpecialization_;
    case clang::TSK_ExplicitInstantiationDeclaration: return clang_TSK_ExplicitInstantiationDeclaration_;
    case clang::TSK_ExplicitInstantiationDefinition: return clang_TSK_ExplicitInstantiationDefinition_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::ThreadStorageClassSpecifier e) {
  switch(e) {
    case clang::TSCS_unspecified: return clang_TSCS_unspecified_;
    case clang::TSCS___thread: return clang_TSCS___thread_;
    case clang::TSCS_thread_local: return clang_TSCS_thread_local_;
    case clang::TSCS__Thread_local: return clang_TSCS__Thread_local_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::Type::ScalarTypeKind e) {
  switch(e) {
    case clang::Type::STK_CPointer: return clang_Type_STK_CPointer_;
    case clang::Type::STK_BlockPointer: return clang_Type_STK_BlockPointer_;
    case clang::Type::STK_ObjCObjectPointer: return clang_Type_STK_ObjCObjectPointer_;
    case clang::Type::STK_MemberPointer: return clang_Type_STK_MemberPointer_;
    case clang::Type::STK_Bool: return clang_Type_STK_Bool_;
    case clang::Type::STK_Integral: return clang_Type_STK_Integral_;
    case clang::Type::STK_Floating: return clang_Type_STK_Floating_;
    case clang::Type::STK_IntegralComplex: return clang_Type_STK_IntegralComplex_;
    case clang::Type::STK_FloatingComplex: return clang_Type_STK_FloatingComplex_;
    case clang::Type::STK_FixedPoint: return clang_Type_STK_FixedPoint_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::Type::TypeClass e) {
  switch(e) {
    case clang::Type::Adjusted: return clang_Type_Adjusted_;
    case clang::Type::Decayed: return clang_Type_Decayed_;
    case clang::Type::ConstantArray: return clang_Type_ConstantArray_;
    case clang::Type::DependentSizedArray: return clang_Type_DependentSizedArray_;
    case clang::Type::IncompleteArray: return clang_Type_IncompleteArray_;
    case clang::Type::VariableArray: return clang_Type_VariableArray_;
    case clang::Type::Atomic: return clang_Type_Atomic_;
    case clang::Type::Attributed: return clang_Type_Attributed_;
    case clang::Type::BTFTagAttributed: return clang_Type_BTFTagAttributed_;
    case clang::Type::BitInt: return clang_Type_BitInt_;
    case clang::Type::BlockPointer: return clang_Type_BlockPointer_;
    case clang::Type::Builtin: return clang_Type_Builtin_;
    case clang::Type::Complex: return clang_Type_Complex_;
    case clang::Type::Decltype: return clang_Type_Decltype_;
    case clang::Type::Auto: return clang_Type_Auto_;
    case clang::Type::DeducedTemplateSpecialization: return clang_Type_DeducedTemplateSpecialization_;
    case clang::Type::DependentAddressSpace: return clang_Type_DependentAddressSpace_;
    case clang::Type::DependentBitInt: return clang_Type_DependentBitInt_;
    case clang::Type::DependentName: return clang_Type_DependentName_;
    case clang::Type::DependentSizedExtVector: return clang_Type_DependentSizedExtVector_;
    case clang::Type::DependentTemplateSpecialization: return clang_Type_DependentTemplateSpecialization_;
    case clang::Type::DependentVector: return clang_Type_DependentVector_;
    case clang::Type::Elaborated: return clang_Type_Elaborated_;
    case clang::Type::FunctionNoProto: return clang_Type_FunctionNoProto_;
    case clang::Type::FunctionProto: return clang_Type_FunctionProto_;
    case clang::Type::InjectedClassName: return clang_Type_InjectedClassName_;
    case clang::Type::MacroQualified: return clang_Type_MacroQualified_;
    case clang::Type::ConstantMatrix: return clang_Type_ConstantMatrix_;
    case clang::Type::DependentSizedMatrix: return clang_Type_DependentSizedMatrix_;
    case clang::Type::MemberPointer: return clang_Type_MemberPointer_;
    case clang::Type::ObjCObjectPointer: return clang_Type_ObjCObjectPointer_;
    case clang::Type::ObjCObject: return clang_Type_ObjCObject_;
    case clang::Type::ObjCInterface: return clang_Type_ObjCInterface_;
    case clang::Type::ObjCTypeParam: return clang_Type_ObjCTypeParam_;
    case clang::Type::PackExpansion: return clang_Type_PackExpansion_;
    case clang::Type::Paren: return clang_Type_Paren_;
    case clang::Type::Pipe: return clang_Type_Pipe_;
    case clang::Type::Pointer: return clang_Type_Pointer_;
    case clang::Type::LValueReference: return clang_Type_LValueReference_;
    case clang::Type::RValueReference: return clang_Type_RValueReference_;
    case clang::Type::SubstTemplateTypeParmPack: return clang_Type_SubstTemplateTypeParmPack_;
    case clang::Type::SubstTemplateTypeParm: return clang_Type_SubstTemplateTypeParm_;
    case clang::Type::Enum: return clang_Type_Enum_;
    case clang::Type::Record: return clang_Type_Record_;
    case clang::Type::TemplateSpecialization: return clang_Type_TemplateSpecialization_;
    case clang::Type::TemplateTypeParm: return clang_Type_TemplateTypeParm_;
    case clang::Type::TypeOfExpr: return clang_Type_TypeOfExpr_;
    case clang::Type::TypeOf: return clang_Type_TypeOf_;
    case clang::Type::Typedef: return clang_Type_Typedef_;
    case clang::Type::UnaryTransform: return clang_Type_UnaryTransform_;
    case clang::Type::UnresolvedUsing: return clang_Type_UnresolvedUsing_;
    case clang::Type::Using: return clang_Type_Using_;
    case clang::Type::Vector: return clang_Type_Vector_;
    case clang::Type::ExtVector: return clang_Type_ExtVector_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::TypeOfKind e) {
  switch(e) {
    case clang::TypeOfKind::Qualified: return clang_TypeOfKind_Qualified_;
    case clang::TypeOfKind::Unqualified: return clang_TypeOfKind_Unqualified_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<uint8_t>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::TypeTrait e) {
  switch(e) {
    case clang::UTT_IsInterfaceClass: return clang_UTT_IsInterfaceClass_;
    case clang::UTT_IsSealed: return clang_UTT_IsSealed_;
    case clang::UTT_IsDestructible: return clang_UTT_IsDestructible_;
    case clang::UTT_IsTriviallyDestructible: return clang_UTT_IsTriviallyDestructible_;
    case clang::UTT_IsNothrowDestructible: return clang_UTT_IsNothrowDestructible_;
    case clang::UTT_HasNothrowMoveAssign: return clang_UTT_HasNothrowMoveAssign_;
    case clang::UTT_HasTrivialMoveAssign: return clang_UTT_HasTrivialMoveAssign_;
    case clang::UTT_HasTrivialMoveConstructor: return clang_UTT_HasTrivialMoveConstructor_;
    case clang::UTT_HasNothrowAssign: return clang_UTT_HasNothrowAssign_;
    case clang::UTT_HasNothrowCopy: return clang_UTT_HasNothrowCopy_;
    case clang::UTT_HasNothrowConstructor: return clang_UTT_HasNothrowConstructor_;
    case clang::UTT_HasTrivialAssign: return clang_UTT_HasTrivialAssign_;
    case clang::UTT_HasTrivialCopy: return clang_UTT_HasTrivialCopy_;
    case clang::UTT_HasTrivialDefaultConstructor: return clang_UTT_HasTrivialDefaultConstructor_;
    case clang::UTT_HasTrivialDestructor: return clang_UTT_HasTrivialDestructor_;
    case clang::UTT_HasVirtualDestructor: return clang_UTT_HasVirtualDestructor_;
    case clang::UTT_IsAbstract: return clang_UTT_IsAbstract_;
    case clang::UTT_IsAggregate: return clang_UTT_IsAggregate_;
    case clang::UTT_IsClass: return clang_UTT_IsClass_;
    case clang::UTT_IsEmpty: return clang_UTT_IsEmpty_;
    case clang::UTT_IsEnum: return clang_UTT_IsEnum_;
    case clang::UTT_IsFinal: return clang_UTT_IsFinal_;
    case clang::UTT_IsLiteral: return clang_UTT_IsLiteral_;
    case clang::UTT_IsPOD: return clang_UTT_IsPOD_;
    case clang::UTT_IsPolymorphic: return clang_UTT_IsPolymorphic_;
    case clang::UTT_IsStandardLayout: return clang_UTT_IsStandardLayout_;
    case clang::UTT_IsTrivial: return clang_UTT_IsTrivial_;
    case clang::UTT_IsTriviallyCopyable: return clang_UTT_IsTriviallyCopyable_;
    case clang::UTT_IsUnion: return clang_UTT_IsUnion_;
    case clang::UTT_HasUniqueObjectRepresentations: return clang_UTT_HasUniqueObjectRepresentations_;
    case clang::UTT_IsTriviallyRelocatable: return clang_UTT_IsTriviallyRelocatable_;
    case clang::UTT_IsTriviallyEqualityComparable: return clang_UTT_IsTriviallyEqualityComparable_;
    case clang::UTT_IsBoundedArray: return clang_UTT_IsBoundedArray_;
    case clang::UTT_IsUnboundedArray: return clang_UTT_IsUnboundedArray_;
    case clang::UTT_IsNullPointer: return clang_UTT_IsNullPointer_;
    case clang::UTT_IsScopedEnum: return clang_UTT_IsScopedEnum_;
    case clang::UTT_IsReferenceable: return clang_UTT_IsReferenceable_;
    case clang::UTT_CanPassInRegs: return clang_UTT_CanPassInRegs_;
    case clang::UTT_IsArithmetic: return clang_UTT_IsArithmetic_;
    case clang::UTT_IsFloatingPoint: return clang_UTT_IsFloatingPoint_;
    case clang::UTT_IsIntegral: return clang_UTT_IsIntegral_;
    case clang::UTT_IsCompleteType: return clang_UTT_IsCompleteType_;
    case clang::UTT_IsVoid: return clang_UTT_IsVoid_;
    case clang::UTT_IsArray: return clang_UTT_IsArray_;
    case clang::UTT_IsFunction: return clang_UTT_IsFunction_;
    case clang::UTT_IsReference: return clang_UTT_IsReference_;
    case clang::UTT_IsLvalueReference: return clang_UTT_IsLvalueReference_;
    case clang::UTT_IsRvalueReference: return clang_UTT_IsRvalueReference_;
    case clang::UTT_IsFundamental: return clang_UTT_IsFundamental_;
    case clang::UTT_IsObject: return clang_UTT_IsObject_;
    case clang::UTT_IsScalar: return clang_UTT_IsScalar_;
    case clang::UTT_IsCompound: return clang_UTT_IsCompound_;
    case clang::UTT_IsPointer: return clang_UTT_IsPointer_;
    case clang::UTT_IsMemberObjectPointer: return clang_UTT_IsMemberObjectPointer_;
    case clang::UTT_IsMemberFunctionPointer: return clang_UTT_IsMemberFunctionPointer_;
    case clang::UTT_IsMemberPointer: return clang_UTT_IsMemberPointer_;
    case clang::UTT_IsConst: return clang_UTT_IsConst_;
    case clang::UTT_IsVolatile: return clang_UTT_IsVolatile_;
    case clang::UTT_IsSigned: return clang_UTT_IsSigned_;
    case clang::UTT_IsUnsigned: return clang_UTT_IsUnsigned_;
    case clang::BTT_TypeCompatible: return clang_BTT_TypeCompatible_;
    case clang::BTT_IsNothrowAssignable: return clang_BTT_IsNothrowAssignable_;
    case clang::BTT_IsAssignable: return clang_BTT_IsAssignable_;
    case clang::BTT_IsBaseOf: return clang_BTT_IsBaseOf_;
    case clang::BTT_IsConvertibleTo: return clang_BTT_IsConvertibleTo_;
    case clang::BTT_IsTriviallyAssignable: return clang_BTT_IsTriviallyAssignable_;
    case clang::BTT_ReferenceBindsToTemporary: return clang_BTT_ReferenceBindsToTemporary_;
    case clang::BTT_IsSame: return clang_BTT_IsSame_;
    case clang::BTT_IsConvertible: return clang_BTT_IsConvertible_;
    case clang::TT_IsConstructible: return clang_TT_IsConstructible_;
    case clang::TT_IsNothrowConstructible: return clang_TT_IsNothrowConstructible_;
    case clang::TT_IsTriviallyConstructible: return clang_TT_IsTriviallyConstructible_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::UnaryExprOrTypeTrait e) {
  switch(e) {
    case clang::UETT_SizeOf: return clang_UETT_SizeOf_;
    case clang::UETT_AlignOf: return clang_UETT_AlignOf_;
    case clang::UETT_PreferredAlignOf: return clang_UETT_PreferredAlignOf_;
    case clang::UETT_VecStep: return clang_UETT_VecStep_;
    case clang::UETT_OpenMPRequiredSimdAlign: return clang_UETT_OpenMPRequiredSimdAlign_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::UnaryOperatorKind e) {
  switch(e) {
    case clang::UO_PostInc: return clang_UO_PostInc_;
    case clang::UO_PostDec: return clang_UO_PostDec_;
    case clang::UO_PreInc: return clang_UO_PreInc_;
    case clang::UO_PreDec: return clang_UO_PreDec_;
    case clang::UO_AddrOf: return clang_UO_AddrOf_;
    case clang::UO_Deref: return clang_UO_Deref_;
    case clang::UO_Plus: return clang_UO_Plus_;
    case clang::UO_Minus: return clang_UO_Minus_;
    case clang::UO_Not: return clang_UO_Not_;
    case clang::UO_LNot: return clang_UO_LNot_;
    case clang::UO_Real: return clang_UO_Real_;
    case clang::UO_Imag: return clang_UO_Imag_;
    case clang::UO_Extension: return clang_UO_Extension_;
    case clang::UO_Coawait: return clang_UO_Coawait_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::UnaryTransformType::UTTKind e) {
  switch(e) {
    case clang::UnaryTransformType::AddLvalueReference: return clang_UnaryTransformType_AddLvalueReference_;
    case clang::UnaryTransformType::AddPointer: return clang_UnaryTransformType_AddPointer_;
    case clang::UnaryTransformType::AddRvalueReference: return clang_UnaryTransformType_AddRvalueReference_;
    case clang::UnaryTransformType::Decay: return clang_UnaryTransformType_Decay_;
    case clang::UnaryTransformType::MakeSigned: return clang_UnaryTransformType_MakeSigned_;
    case clang::UnaryTransformType::MakeUnsigned: return clang_UnaryTransformType_MakeUnsigned_;
    case clang::UnaryTransformType::RemoveAllExtents: return clang_UnaryTransformType_RemoveAllExtents_;
    case clang::UnaryTransformType::RemoveConst: return clang_UnaryTransformType_RemoveConst_;
    case clang::UnaryTransformType::RemoveCV: return clang_UnaryTransformType_RemoveCV_;
    case clang::UnaryTransformType::RemoveCVRef: return clang_UnaryTransformType_RemoveCVRef_;
    case clang::UnaryTransformType::RemoveExtent: return clang_UnaryTransformType_RemoveExtent_;
    case clang::UnaryTransformType::RemovePointer: return clang_UnaryTransformType_RemovePointer_;
    case clang::UnaryTransformType::RemoveReference: return clang_UnaryTransformType_RemoveReference_;
    case clang::UnaryTransformType::RemoveRestrict: return clang_UnaryTransformType_RemoveRestrict_;
    case clang::UnaryTransformType::RemoveVolatile: return clang_UnaryTransformType_RemoveVolatile_;
    case clang::UnaryTransformType::EnumUnderlyingType: return clang_UnaryTransformType_EnumUnderlyingType_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::UserDefinedLiteral::LiteralOperatorKind e) {
  switch(e) {
    case clang::UserDefinedLiteral::LOK_Raw: return clang_UserDefinedLiteral_LOK_Raw_;
    case clang::UserDefinedLiteral::LOK_Template: return clang_UserDefinedLiteral_LOK_Template_;
    case clang::UserDefinedLiteral::LOK_Integer: return clang_UserDefinedLiteral_LOK_Integer_;
    case clang::UserDefinedLiteral::LOK_Floating: return clang_UserDefinedLiteral_LOK_Floating_;
    case clang::UserDefinedLiteral::LOK_String: return clang_UserDefinedLiteral_LOK_String_;
    case clang::UserDefinedLiteral::LOK_Character: return clang_UserDefinedLiteral_LOK_Character_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::VarDecl::DefinitionKind e) {
  switch(e) {
    case clang::VarDecl::DeclarationOnly: return clang_VarDecl_DeclarationOnly_;
    case clang::VarDecl::TentativeDefinition: return clang_VarDecl_TentativeDefinition_;
    case clang::VarDecl::Definition: return clang_VarDecl_Definition_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::VarDecl::InitializationStyle e) {
  switch(e) {
    case clang::VarDecl::CInit: return clang_VarDecl_CInit_;
    case clang::VarDecl::CallInit: return clang_VarDecl_CallInit_;
    case clang::VarDecl::ListInit: return clang_VarDecl_ListInit_;
    case clang::VarDecl::ParenListInit: return clang_VarDecl_ParenListInit_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::VarDecl::TLSKind e) {
  switch(e) {
    case clang::VarDecl::TLS_None: return clang_VarDecl_TLS_None_;
    case clang::VarDecl::TLS_Static: return clang_VarDecl_TLS_Static_;
    case clang::VarDecl::TLS_Dynamic: return clang_VarDecl_TLS_Dynamic_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::VectorType::VectorKind e) {
  switch(e) {
    case clang::VectorType::GenericVector: return clang_VectorType_GenericVector_;
    case clang::VectorType::AltiVecVector: return clang_VectorType_AltiVecVector_;
    case clang::VectorType::AltiVecPixel: return clang_VectorType_AltiVecPixel_;
    case clang::VectorType::AltiVecBool: return clang_VectorType_AltiVecBool_;
    case clang::VectorType::NeonVector: return clang_VectorType_NeonVector_;
    case clang::VectorType::NeonPolyVector: return clang_VectorType_NeonPolyVector_;
    case clang::VectorType::SveFixedLengthDataVector: return clang_VectorType_SveFixedLengthDataVector_;
    case clang::VectorType::SveFixedLengthPredicateVector: return clang_VectorType_SveFixedLengthPredicateVector_;
    case clang::VectorType::RVVFixedLengthDataVector: return clang_VectorType_RVVFixedLengthDataVector_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(clang::attr::Kind e) {
  switch(e) {
    case clang::attr::AddressSpace: return clang_attr_AddressSpace_;
    case clang::attr::AnnotateType: return clang_attr_AnnotateType_;
    case clang::attr::ArmMveStrictPolymorphism: return clang_attr_ArmMveStrictPolymorphism_;
    case clang::attr::ArmStreaming: return clang_attr_ArmStreaming_;
    case clang::attr::BTFTypeTag: return clang_attr_BTFTypeTag_;
    case clang::attr::CmseNSCall: return clang_attr_CmseNSCall_;
    case clang::attr::HLSLGroupSharedAddressSpace: return clang_attr_HLSLGroupSharedAddressSpace_;
    case clang::attr::NoDeref: return clang_attr_NoDeref_;
    case clang::attr::ObjCGC: return clang_attr_ObjCGC_;
    case clang::attr::ObjCInertUnsafeUnretained: return clang_attr_ObjCInertUnsafeUnretained_;
    case clang::attr::ObjCKindOf: return clang_attr_ObjCKindOf_;
    case clang::attr::OpenCLConstantAddressSpace: return clang_attr_OpenCLConstantAddressSpace_;
    case clang::attr::OpenCLGenericAddressSpace: return clang_attr_OpenCLGenericAddressSpace_;
    case clang::attr::OpenCLGlobalAddressSpace: return clang_attr_OpenCLGlobalAddressSpace_;
    case clang::attr::OpenCLGlobalDeviceAddressSpace: return clang_attr_OpenCLGlobalDeviceAddressSpace_;
    case clang::attr::OpenCLGlobalHostAddressSpace: return clang_attr_OpenCLGlobalHostAddressSpace_;
    case clang::attr::OpenCLLocalAddressSpace: return clang_attr_OpenCLLocalAddressSpace_;
    case clang::attr::OpenCLPrivateAddressSpace: return clang_attr_OpenCLPrivateAddressSpace_;
    case clang::attr::Ptr32: return clang_attr_Ptr32_;
    case clang::attr::Ptr64: return clang_attr_Ptr64_;
    case clang::attr::SPtr: return clang_attr_SPtr_;
    case clang::attr::TypeNonNull: return clang_attr_TypeNonNull_;
    case clang::attr::TypeNullUnspecified: return clang_attr_TypeNullUnspecified_;
    case clang::attr::TypeNullable: return clang_attr_TypeNullable_;
    case clang::attr::TypeNullableResult: return clang_attr_TypeNullableResult_;
    case clang::attr::UPtr: return clang_attr_UPtr_;
    case clang::attr::WebAssemblyFuncref: return clang_attr_WebAssemblyFuncref_;
    case clang::attr::FallThrough: return clang_attr_FallThrough_;
    case clang::attr::Likely: return clang_attr_Likely_;
    case clang::attr::MustTail: return clang_attr_MustTail_;
    case clang::attr::OpenCLUnrollHint: return clang_attr_OpenCLUnrollHint_;
    case clang::attr::Suppress: return clang_attr_Suppress_;
    case clang::attr::Unlikely: return clang_attr_Unlikely_;
    case clang::attr::AlwaysInline: return clang_attr_AlwaysInline_;
    case clang::attr::NoInline: return clang_attr_NoInline_;
    case clang::attr::NoMerge: return clang_attr_NoMerge_;
    case clang::attr::AArch64SVEPcs: return clang_attr_AArch64SVEPcs_;
    case clang::attr::AArch64VectorPcs: return clang_attr_AArch64VectorPcs_;
    case clang::attr::AMDGPUKernelCall: return clang_attr_AMDGPUKernelCall_;
    case clang::attr::AcquireHandle: return clang_attr_AcquireHandle_;
    case clang::attr::AnyX86NoCfCheck: return clang_attr_AnyX86NoCfCheck_;
    case clang::attr::CDecl: return clang_attr_CDecl_;
    case clang::attr::FastCall: return clang_attr_FastCall_;
    case clang::attr::IntelOclBicc: return clang_attr_IntelOclBicc_;
    case clang::attr::LifetimeBound: return clang_attr_LifetimeBound_;
    case clang::attr::MSABI: return clang_attr_MSABI_;
    case clang::attr::NSReturnsRetained: return clang_attr_NSReturnsRetained_;
    case clang::attr::ObjCOwnership: return clang_attr_ObjCOwnership_;
    case clang::attr::Pascal: return clang_attr_Pascal_;
    case clang::attr::Pcs: return clang_attr_Pcs_;
    case clang::attr::PreserveAll: return clang_attr_PreserveAll_;
    case clang::attr::PreserveMost: return clang_attr_PreserveMost_;
    case clang::attr::RegCall: return clang_attr_RegCall_;
    case clang::attr::StdCall: return clang_attr_StdCall_;
    case clang::attr::SwiftAsyncCall: return clang_attr_SwiftAsyncCall_;
    case clang::attr::SwiftCall: return clang_attr_SwiftCall_;
    case clang::attr::SysVABI: return clang_attr_SysVABI_;
    case clang::attr::ThisCall: return clang_attr_ThisCall_;
    case clang::attr::VectorCall: return clang_attr_VectorCall_;
    case clang::attr::SwiftAsyncContext: return clang_attr_SwiftAsyncContext_;
    case clang::attr::SwiftContext: return clang_attr_SwiftContext_;
    case clang::attr::SwiftErrorResult: return clang_attr_SwiftErrorResult_;
    case clang::attr::SwiftIndirectResult: return clang_attr_SwiftIndirectResult_;
    case clang::attr::Annotate: return clang_attr_Annotate_;
    case clang::attr::CFConsumed: return clang_attr_CFConsumed_;
    case clang::attr::CarriesDependency: return clang_attr_CarriesDependency_;
    case clang::attr::NSConsumed: return clang_attr_NSConsumed_;
    case clang::attr::NonNull: return clang_attr_NonNull_;
    case clang::attr::OSConsumed: return clang_attr_OSConsumed_;
    case clang::attr::PassObjectSize: return clang_attr_PassObjectSize_;
    case clang::attr::ReleaseHandle: return clang_attr_ReleaseHandle_;
    case clang::attr::UseHandle: return clang_attr_UseHandle_;
    case clang::attr::HLSLSV_DispatchThreadID: return clang_attr_HLSLSV_DispatchThreadID_;
    case clang::attr::HLSLSV_GroupIndex: return clang_attr_HLSLSV_GroupIndex_;
    case clang::attr::AMDGPUFlatWorkGroupSize: return clang_attr_AMDGPUFlatWorkGroupSize_;
    case clang::attr::AMDGPUNumSGPR: return clang_attr_AMDGPUNumSGPR_;
    case clang::attr::AMDGPUNumVGPR: return clang_attr_AMDGPUNumVGPR_;
    case clang::attr::AMDGPUWavesPerEU: return clang_attr_AMDGPUWavesPerEU_;
    case clang::attr::ARMInterrupt: return clang_attr_ARMInterrupt_;
    case clang::attr::AVRInterrupt: return clang_attr_AVRInterrupt_;
    case clang::attr::AVRSignal: return clang_attr_AVRSignal_;
    case clang::attr::AcquireCapability: return clang_attr_AcquireCapability_;
    case clang::attr::AcquiredAfter: return clang_attr_AcquiredAfter_;
    case clang::attr::AcquiredBefore: return clang_attr_AcquiredBefore_;
    case clang::attr::AlignMac68k: return clang_attr_AlignMac68k_;
    case clang::attr::AlignNatural: return clang_attr_AlignNatural_;
    case clang::attr::Aligned: return clang_attr_Aligned_;
    case clang::attr::AllocAlign: return clang_attr_AllocAlign_;
    case clang::attr::AllocSize: return clang_attr_AllocSize_;
    case clang::attr::AlwaysDestroy: return clang_attr_AlwaysDestroy_;
    case clang::attr::AnalyzerNoReturn: return clang_attr_AnalyzerNoReturn_;
    case clang::attr::AnyX86Interrupt: return clang_attr_AnyX86Interrupt_;
    case clang::attr::AnyX86NoCallerSavedRegisters: return clang_attr_AnyX86NoCallerSavedRegisters_;
    case clang::attr::ArcWeakrefUnavailable: return clang_attr_ArcWeakrefUnavailable_;
    case clang::attr::ArgumentWithTypeTag: return clang_attr_ArgumentWithTypeTag_;
    case clang::attr::ArmBuiltinAlias: return clang_attr_ArmBuiltinAlias_;
    case clang::attr::Artificial: return clang_attr_Artificial_;
    case clang::attr::AsmLabel: return clang_attr_AsmLabel_;
    case clang::attr::AssertCapability: return clang_attr_AssertCapability_;
    case clang::attr::AssertExclusiveLock: return clang_attr_AssertExclusiveLock_;
    case clang::attr::AssertSharedLock: return clang_attr_AssertSharedLock_;
    case clang::attr::AssumeAligned: return clang_attr_AssumeAligned_;
    case clang::attr::Assumption: return clang_attr_Assumption_;
    case clang::attr::Availability: return clang_attr_Availability_;
    case clang::attr::AvailableOnlyInDefaultEvalMethod: return clang_attr_AvailableOnlyInDefaultEvalMethod_;
    case clang::attr::BPFPreserveAccessIndex: return clang_attr_BPFPreserveAccessIndex_;
    case clang::attr::BTFDeclTag: return clang_attr_BTFDeclTag_;
    case clang::attr::Blocks: return clang_attr_Blocks_;
    case clang::attr::Builtin: return clang_attr_Builtin_;
    case clang::attr::C11NoReturn: return clang_attr_C11NoReturn_;
    case clang::attr::CFAuditedTransfer: return clang_attr_CFAuditedTransfer_;
    case clang::attr::CFGuard: return clang_attr_CFGuard_;
    case clang::attr::CFICanonicalJumpTable: return clang_attr_CFICanonicalJumpTable_;
    case clang::attr::CFReturnsNotRetained: return clang_attr_CFReturnsNotRetained_;
    case clang::attr::CFReturnsRetained: return clang_attr_CFReturnsRetained_;
    case clang::attr::CFUnknownTransfer: return clang_attr_CFUnknownTransfer_;
    case clang::attr::CPUDispatch: return clang_attr_CPUDispatch_;
    case clang::attr::CPUSpecific: return clang_attr_CPUSpecific_;
    case clang::attr::CUDAConstant: return clang_attr_CUDAConstant_;
    case clang::attr::CUDADevice: return clang_attr_CUDADevice_;
    case clang::attr::CUDADeviceBuiltinSurfaceType: return clang_attr_CUDADeviceBuiltinSurfaceType_;
    case clang::attr::CUDADeviceBuiltinTextureType: return clang_attr_CUDADeviceBuiltinTextureType_;
    case clang::attr::CUDAGlobal: return clang_attr_CUDAGlobal_;
    case clang::attr::CUDAHost: return clang_attr_CUDAHost_;
    case clang::attr::CUDAInvalidTarget: return clang_attr_CUDAInvalidTarget_;
    case clang::attr::CUDALaunchBounds: return clang_attr_CUDALaunchBounds_;
    case clang::attr::CUDAShared: return clang_attr_CUDAShared_;
    case clang::attr::CXX11NoReturn: return clang_attr_CXX11NoReturn_;
    case clang::attr::CallableWhen: return clang_attr_CallableWhen_;
    case clang::attr::Callback: return clang_attr_Callback_;
    case clang::attr::Capability: return clang_attr_Capability_;
    case clang::attr::CapturedRecord: return clang_attr_CapturedRecord_;
    case clang::attr::Cleanup: return clang_attr_Cleanup_;
    case clang::attr::CmseNSEntry: return clang_attr_CmseNSEntry_;
    case clang::attr::CodeSeg: return clang_attr_CodeSeg_;
    case clang::attr::Cold: return clang_attr_Cold_;
    case clang::attr::Common: return clang_attr_Common_;
    case clang::attr::Const: return clang_attr_Const_;
    case clang::attr::ConstInit: return clang_attr_ConstInit_;
    case clang::attr::Constructor: return clang_attr_Constructor_;
    case clang::attr::Consumable: return clang_attr_Consumable_;
    case clang::attr::ConsumableAutoCast: return clang_attr_ConsumableAutoCast_;
    case clang::attr::ConsumableSetOnRead: return clang_attr_ConsumableSetOnRead_;
    case clang::attr::Convergent: return clang_attr_Convergent_;
    case clang::attr::DLLExport: return clang_attr_DLLExport_;
    case clang::attr::DLLExportStaticLocal: return clang_attr_DLLExportStaticLocal_;
    case clang::attr::DLLImport: return clang_attr_DLLImport_;
    case clang::attr::DLLImportStaticLocal: return clang_attr_DLLImportStaticLocal_;
    case clang::attr::Deprecated: return clang_attr_Deprecated_;
    case clang::attr::Destructor: return clang_attr_Destructor_;
    case clang::attr::DiagnoseAsBuiltin: return clang_attr_DiagnoseAsBuiltin_;
    case clang::attr::DiagnoseIf: return clang_attr_DiagnoseIf_;
    case clang::attr::DisableSanitizerInstrumentation: return clang_attr_DisableSanitizerInstrumentation_;
    case clang::attr::DisableTailCalls: return clang_attr_DisableTailCalls_;
    case clang::attr::EmptyBases: return clang_attr_EmptyBases_;
    case clang::attr::EnableIf: return clang_attr_EnableIf_;
    case clang::attr::EnforceTCB: return clang_attr_EnforceTCB_;
    case clang::attr::EnforceTCBLeaf: return clang_attr_EnforceTCBLeaf_;
    case clang::attr::EnumExtensibility: return clang_attr_EnumExtensibility_;
    case clang::attr::Error: return clang_attr_Error_;
    case clang::attr::ExcludeFromExplicitInstantiation: return clang_attr_ExcludeFromExplicitInstantiation_;
    case clang::attr::ExclusiveTrylockFunction: return clang_attr_ExclusiveTrylockFunction_;
    case clang::attr::ExternalSourceSymbol: return clang_attr_ExternalSourceSymbol_;
    case clang::attr::Final: return clang_attr_Final_;
    case clang::attr::FlagEnum: return clang_attr_FlagEnum_;
    case clang::attr::Flatten: return clang_attr_Flatten_;
    case clang::attr::Format: return clang_attr_Format_;
    case clang::attr::FormatArg: return clang_attr_FormatArg_;
    case clang::attr::FunctionReturnThunks: return clang_attr_FunctionReturnThunks_;
    case clang::attr::GNUInline: return clang_attr_GNUInline_;
    case clang::attr::GuardedBy: return clang_attr_GuardedBy_;
    case clang::attr::GuardedVar: return clang_attr_GuardedVar_;
    case clang::attr::HIPManaged: return clang_attr_HIPManaged_;
    case clang::attr::HLSLNumThreads: return clang_attr_HLSLNumThreads_;
    case clang::attr::HLSLResource: return clang_attr_HLSLResource_;
    case clang::attr::HLSLResourceBinding: return clang_attr_HLSLResourceBinding_;
    case clang::attr::HLSLShader: return clang_attr_HLSLShader_;
    case clang::attr::Hot: return clang_attr_Hot_;
    case clang::attr::IBAction: return clang_attr_IBAction_;
    case clang::attr::IBOutlet: return clang_attr_IBOutlet_;
    case clang::attr::IBOutletCollection: return clang_attr_IBOutletCollection_;
    case clang::attr::InitPriority: return clang_attr_InitPriority_;
    case clang::attr::InternalLinkage: return clang_attr_InternalLinkage_;
    case clang::attr::LTOVisibilityPublic: return clang_attr_LTOVisibilityPublic_;
    case clang::attr::LayoutVersion: return clang_attr_LayoutVersion_;
    case clang::attr::Leaf: return clang_attr_Leaf_;
    case clang::attr::LockReturned: return clang_attr_LockReturned_;
    case clang::attr::LocksExcluded: return clang_attr_LocksExcluded_;
    case clang::attr::M68kInterrupt: return clang_attr_M68kInterrupt_;
    case clang::attr::MIGServerRoutine: return clang_attr_MIGServerRoutine_;
    case clang::attr::MSAllocator: return clang_attr_MSAllocator_;
    case clang::attr::MSInheritance: return clang_attr_MSInheritance_;
    case clang::attr::MSNoVTable: return clang_attr_MSNoVTable_;
    case clang::attr::MSP430Interrupt: return clang_attr_MSP430Interrupt_;
    case clang::attr::MSStruct: return clang_attr_MSStruct_;
    case clang::attr::MSVtorDisp: return clang_attr_MSVtorDisp_;
    case clang::attr::MaxFieldAlignment: return clang_attr_MaxFieldAlignment_;
    case clang::attr::MayAlias: return clang_attr_MayAlias_;
    case clang::attr::MaybeUndef: return clang_attr_MaybeUndef_;
    case clang::attr::MicroMips: return clang_attr_MicroMips_;
    case clang::attr::MinSize: return clang_attr_MinSize_;
    case clang::attr::MinVectorWidth: return clang_attr_MinVectorWidth_;
    case clang::attr::Mips16: return clang_attr_Mips16_;
    case clang::attr::MipsInterrupt: return clang_attr_MipsInterrupt_;
    case clang::attr::MipsLongCall: return clang_attr_MipsLongCall_;
    case clang::attr::MipsShortCall: return clang_attr_MipsShortCall_;
    case clang::attr::NSConsumesSelf: return clang_attr_NSConsumesSelf_;
    case clang::attr::NSErrorDomain: return clang_attr_NSErrorDomain_;
    case clang::attr::NSReturnsAutoreleased: return clang_attr_NSReturnsAutoreleased_;
    case clang::attr::NSReturnsNotRetained: return clang_attr_NSReturnsNotRetained_;
    case clang::attr::NVPTXKernel: return clang_attr_NVPTXKernel_;
    case clang::attr::Naked: return clang_attr_Naked_;
    case clang::attr::NoAlias: return clang_attr_NoAlias_;
    case clang::attr::NoCommon: return clang_attr_NoCommon_;
    case clang::attr::NoDebug: return clang_attr_NoDebug_;
    case clang::attr::NoDestroy: return clang_attr_NoDestroy_;
    case clang::attr::NoDuplicate: return clang_attr_NoDuplicate_;
    case clang::attr::NoInstrumentFunction: return clang_attr_NoInstrumentFunction_;
    case clang::attr::NoMicroMips: return clang_attr_NoMicroMips_;
    case clang::attr::NoMips16: return clang_attr_NoMips16_;
    case clang::attr::NoProfileFunction: return clang_attr_NoProfileFunction_;
    case clang::attr::NoRandomizeLayout: return clang_attr_NoRandomizeLayout_;
    case clang::attr::NoReturn: return clang_attr_NoReturn_;
    case clang::attr::NoSanitize: return clang_attr_NoSanitize_;
    case clang::attr::NoSpeculativeLoadHardening: return clang_attr_NoSpeculativeLoadHardening_;
    case clang::attr::NoSplitStack: return clang_attr_NoSplitStack_;
    case clang::attr::NoStackProtector: return clang_attr_NoStackProtector_;
    case clang::attr::NoThreadSafetyAnalysis: return clang_attr_NoThreadSafetyAnalysis_;
    case clang::attr::NoThrow: return clang_attr_NoThrow_;
    case clang::attr::NoUniqueAddress: return clang_attr_NoUniqueAddress_;
    case clang::attr::NoUwtable: return clang_attr_NoUwtable_;
    case clang::attr::NotTailCalled: return clang_attr_NotTailCalled_;
    case clang::attr::OMPAllocateDecl: return clang_attr_OMPAllocateDecl_;
    case clang::attr::OMPCaptureNoInit: return clang_attr_OMPCaptureNoInit_;
    case clang::attr::OMPDeclareTargetDecl: return clang_attr_OMPDeclareTargetDecl_;
    case clang::attr::OMPDeclareVariant: return clang_attr_OMPDeclareVariant_;
    case clang::attr::OMPThreadPrivateDecl: return clang_attr_OMPThreadPrivateDecl_;
    case clang::attr::OSConsumesThis: return clang_attr_OSConsumesThis_;
    case clang::attr::OSReturnsNotRetained: return clang_attr_OSReturnsNotRetained_;
    case clang::attr::OSReturnsRetained: return clang_attr_OSReturnsRetained_;
    case clang::attr::OSReturnsRetainedOnNonZero: return clang_attr_OSReturnsRetainedOnNonZero_;
    case clang::attr::OSReturnsRetainedOnZero: return clang_attr_OSReturnsRetainedOnZero_;
    case clang::attr::ObjCBridge: return clang_attr_ObjCBridge_;
    case clang::attr::ObjCBridgeMutable: return clang_attr_ObjCBridgeMutable_;
    case clang::attr::ObjCBridgeRelated: return clang_attr_ObjCBridgeRelated_;
    case clang::attr::ObjCException: return clang_attr_ObjCException_;
    case clang::attr::ObjCExplicitProtocolImpl: return clang_attr_ObjCExplicitProtocolImpl_;
    case clang::attr::ObjCExternallyRetained: return clang_attr_ObjCExternallyRetained_;
    case clang::attr::ObjCIndependentClass: return clang_attr_ObjCIndependentClass_;
    case clang::attr::ObjCMethodFamily: return clang_attr_ObjCMethodFamily_;
    case clang::attr::ObjCNSObject: return clang_attr_ObjCNSObject_;
    case clang::attr::ObjCPreciseLifetime: return clang_attr_ObjCPreciseLifetime_;
    case clang::attr::ObjCRequiresPropertyDefs: return clang_attr_ObjCRequiresPropertyDefs_;
    case clang::attr::ObjCRequiresSuper: return clang_attr_ObjCRequiresSuper_;
    case clang::attr::ObjCReturnsInnerPointer: return clang_attr_ObjCReturnsInnerPointer_;
    case clang::attr::ObjCRootClass: return clang_attr_ObjCRootClass_;
    case clang::attr::ObjCSubclassingRestricted: return clang_attr_ObjCSubclassingRestricted_;
    case clang::attr::OpenCLIntelReqdSubGroupSize: return clang_attr_OpenCLIntelReqdSubGroupSize_;
    case clang::attr::OpenCLKernel: return clang_attr_OpenCLKernel_;
    case clang::attr::OptimizeNone: return clang_attr_OptimizeNone_;
    case clang::attr::Override: return clang_attr_Override_;
    case clang::attr::Owner: return clang_attr_Owner_;
    case clang::attr::Ownership: return clang_attr_Ownership_;
    case clang::attr::Packed: return clang_attr_Packed_;
    case clang::attr::ParamTypestate: return clang_attr_ParamTypestate_;
    case clang::attr::PatchableFunctionEntry: return clang_attr_PatchableFunctionEntry_;
    case clang::attr::Pointer: return clang_attr_Pointer_;
    case clang::attr::PragmaClangBSSSection: return clang_attr_PragmaClangBSSSection_;
    case clang::attr::PragmaClangDataSection: return clang_attr_PragmaClangDataSection_;
    case clang::attr::PragmaClangRelroSection: return clang_attr_PragmaClangRelroSection_;
    case clang::attr::PragmaClangRodataSection: return clang_attr_PragmaClangRodataSection_;
    case clang::attr::PragmaClangTextSection: return clang_attr_PragmaClangTextSection_;
    case clang::attr::PreferredName: return clang_attr_PreferredName_;
    case clang::attr::PtGuardedBy: return clang_attr_PtGuardedBy_;
    case clang::attr::PtGuardedVar: return clang_attr_PtGuardedVar_;
    case clang::attr::Pure: return clang_attr_Pure_;
    case clang::attr::RISCVInterrupt: return clang_attr_RISCVInterrupt_;
    case clang::attr::RandomizeLayout: return clang_attr_RandomizeLayout_;
    case clang::attr::ReadOnlyPlacement: return clang_attr_ReadOnlyPlacement_;
    case clang::attr::Reinitializes: return clang_attr_Reinitializes_;
    case clang::attr::ReleaseCapability: return clang_attr_ReleaseCapability_;
    case clang::attr::ReqdWorkGroupSize: return clang_attr_ReqdWorkGroupSize_;
    case clang::attr::RequiresCapability: return clang_attr_RequiresCapability_;
    case clang::attr::Restrict: return clang_attr_Restrict_;
    case clang::attr::Retain: return clang_attr_Retain_;
    case clang::attr::ReturnTypestate: return clang_attr_ReturnTypestate_;
    case clang::attr::ReturnsNonNull: return clang_attr_ReturnsNonNull_;
    case clang::attr::ReturnsTwice: return clang_attr_ReturnsTwice_;
    case clang::attr::SYCLKernel: return clang_attr_SYCLKernel_;
    case clang::attr::SYCLSpecialClass: return clang_attr_SYCLSpecialClass_;
    case clang::attr::ScopedLockable: return clang_attr_ScopedLockable_;
    case clang::attr::Section: return clang_attr_Section_;
    case clang::attr::SelectAny: return clang_attr_SelectAny_;
    case clang::attr::Sentinel: return clang_attr_Sentinel_;
    case clang::attr::SetTypestate: return clang_attr_SetTypestate_;
    case clang::attr::SharedTrylockFunction: return clang_attr_SharedTrylockFunction_;
    case clang::attr::SpeculativeLoadHardening: return clang_attr_SpeculativeLoadHardening_;
    case clang::attr::StandaloneDebug: return clang_attr_StandaloneDebug_;
    case clang::attr::StrictFP: return clang_attr_StrictFP_;
    case clang::attr::StrictGuardStackCheck: return clang_attr_StrictGuardStackCheck_;
    case clang::attr::SwiftAsync: return clang_attr_SwiftAsync_;
    case clang::attr::SwiftAsyncError: return clang_attr_SwiftAsyncError_;
    case clang::attr::SwiftAsyncName: return clang_attr_SwiftAsyncName_;
    case clang::attr::SwiftAttr: return clang_attr_SwiftAttr_;
    case clang::attr::SwiftBridge: return clang_attr_SwiftBridge_;
    case clang::attr::SwiftBridgedTypedef: return clang_attr_SwiftBridgedTypedef_;
    case clang::attr::SwiftError: return clang_attr_SwiftError_;
    case clang::attr::SwiftName: return clang_attr_SwiftName_;
    case clang::attr::SwiftNewType: return clang_attr_SwiftNewType_;
    case clang::attr::SwiftPrivate: return clang_attr_SwiftPrivate_;
    case clang::attr::TLSModel: return clang_attr_TLSModel_;
    case clang::attr::Target: return clang_attr_Target_;
    case clang::attr::TargetClones: return clang_attr_TargetClones_;
    case clang::attr::TargetVersion: return clang_attr_TargetVersion_;
    case clang::attr::TestTypestate: return clang_attr_TestTypestate_;
    case clang::attr::TransparentUnion: return clang_attr_TransparentUnion_;
    case clang::attr::TrivialABI: return clang_attr_TrivialABI_;
    case clang::attr::TryAcquireCapability: return clang_attr_TryAcquireCapability_;
    case clang::attr::TypeTagForDatatype: return clang_attr_TypeTagForDatatype_;
    case clang::attr::TypeVisibility: return clang_attr_TypeVisibility_;
    case clang::attr::Unavailable: return clang_attr_Unavailable_;
    case clang::attr::Uninitialized: return clang_attr_Uninitialized_;
    case clang::attr::UnsafeBufferUsage: return clang_attr_UnsafeBufferUsage_;
    case clang::attr::Unused: return clang_attr_Unused_;
    case clang::attr::Used: return clang_attr_Used_;
    case clang::attr::UsingIfExists: return clang_attr_UsingIfExists_;
    case clang::attr::Uuid: return clang_attr_Uuid_;
    case clang::attr::VecReturn: return clang_attr_VecReturn_;
    case clang::attr::VecTypeHint: return clang_attr_VecTypeHint_;
    case clang::attr::Visibility: return clang_attr_Visibility_;
    case clang::attr::WarnUnused: return clang_attr_WarnUnused_;
    case clang::attr::WarnUnusedResult: return clang_attr_WarnUnusedResult_;
    case clang::attr::Weak: return clang_attr_Weak_;
    case clang::attr::WeakImport: return clang_attr_WeakImport_;
    case clang::attr::WeakRef: return clang_attr_WeakRef_;
    case clang::attr::WebAssemblyExportName: return clang_attr_WebAssemblyExportName_;
    case clang::attr::WebAssemblyImportModule: return clang_attr_WebAssemblyImportModule_;
    case clang::attr::WebAssemblyImportName: return clang_attr_WebAssemblyImportName_;
    case clang::attr::WorkGroupSizeHint: return clang_attr_WorkGroupSizeHint_;
    case clang::attr::X86ForceAlignArgPointer: return clang_attr_X86ForceAlignArgPointer_;
    case clang::attr::XRayInstrument: return clang_attr_XRayInstrument_;
    case clang::attr::XRayLogArgs: return clang_attr_XRayLogArgs_;
    case clang::attr::ZeroCallUsedRegs: return clang_attr_ZeroCallUsedRegs_;
    case clang::attr::AbiTag: return clang_attr_AbiTag_;
    case clang::attr::Alias: return clang_attr_Alias_;
    case clang::attr::AlignValue: return clang_attr_AlignValue_;
    case clang::attr::BuiltinAlias: return clang_attr_BuiltinAlias_;
    case clang::attr::CalledOnce: return clang_attr_CalledOnce_;
    case clang::attr::IFunc: return clang_attr_IFunc_;
    case clang::attr::InitSeg: return clang_attr_InitSeg_;
    case clang::attr::LoaderUninitialized: return clang_attr_LoaderUninitialized_;
    case clang::attr::LoopHint: return clang_attr_LoopHint_;
    case clang::attr::Mode: return clang_attr_Mode_;
    case clang::attr::NoBuiltin: return clang_attr_NoBuiltin_;
    case clang::attr::NoEscape: return clang_attr_NoEscape_;
    case clang::attr::OMPCaptureKind: return clang_attr_OMPCaptureKind_;
    case clang::attr::OMPDeclareSimdDecl: return clang_attr_OMPDeclareSimdDecl_;
    case clang::attr::OMPReferencedVar: return clang_attr_OMPReferencedVar_;
    case clang::attr::ObjCBoxable: return clang_attr_ObjCBoxable_;
    case clang::attr::ObjCClassStub: return clang_attr_ObjCClassStub_;
    case clang::attr::ObjCDesignatedInitializer: return clang_attr_ObjCDesignatedInitializer_;
    case clang::attr::ObjCDirect: return clang_attr_ObjCDirect_;
    case clang::attr::ObjCDirectMembers: return clang_attr_ObjCDirectMembers_;
    case clang::attr::ObjCNonLazyClass: return clang_attr_ObjCNonLazyClass_;
    case clang::attr::ObjCNonRuntimeProtocol: return clang_attr_ObjCNonRuntimeProtocol_;
    case clang::attr::ObjCRuntimeName: return clang_attr_ObjCRuntimeName_;
    case clang::attr::ObjCRuntimeVisible: return clang_attr_ObjCRuntimeVisible_;
    case clang::attr::OpenCLAccess: return clang_attr_OpenCLAccess_;
    case clang::attr::Overloadable: return clang_attr_Overloadable_;
    case clang::attr::RenderScriptKernel: return clang_attr_RenderScriptKernel_;
    case clang::attr::SwiftObjCMembers: return clang_attr_SwiftObjCMembers_;
    case clang::attr::Thread: return clang_attr_Thread_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

Thing* DataModel::resolve(llvm::APFloatBase::Semantics e) {
  switch(e) {
    case llvm::APFloatBase::S_IEEEhalf: return llvm_APFloatBase_S_IEEEhalf_;
    case llvm::APFloatBase::S_BFloat: return llvm_APFloatBase_S_BFloat_;
    case llvm::APFloatBase::S_IEEEsingle: return llvm_APFloatBase_S_IEEEsingle_;
    case llvm::APFloatBase::S_IEEEdouble: return llvm_APFloatBase_S_IEEEdouble_;
    case llvm::APFloatBase::S_IEEEquad: return llvm_APFloatBase_S_IEEEquad_;
    case llvm::APFloatBase::S_PPCDoubleDouble: return llvm_APFloatBase_S_PPCDoubleDouble_;
    case llvm::APFloatBase::S_Float8E5M2: return llvm_APFloatBase_S_Float8E5M2_;
    case llvm::APFloatBase::S_Float8E5M2FNUZ: return llvm_APFloatBase_S_Float8E5M2FNUZ_;
    case llvm::APFloatBase::S_Float8E4M3FN: return llvm_APFloatBase_S_Float8E4M3FN_;
    case llvm::APFloatBase::S_Float8E4M3FNUZ: return llvm_APFloatBase_S_Float8E4M3FNUZ_;
    case llvm::APFloatBase::S_Float8E4M3B11FNUZ: return llvm_APFloatBase_S_Float8E4M3B11FNUZ_;
    case llvm::APFloatBase::S_FloatTF32: return llvm_APFloatBase_S_FloatTF32_;
    case llvm::APFloatBase::S_x87DoubleExtended: return llvm_APFloatBase_S_x87DoubleExtended_;
    default: llvm::errs() << "Unexpected enum value: " << static_cast<unsigned int>(e) << "\n";
  }
  return nullptr;
}

////   END ARBORETUM GENERATED CODE ////
}  // namespace arboretum
