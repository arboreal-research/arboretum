#include "arboretum_data_model.h"

#include <iomanip>
#include <iostream>

#include "arboretum_ffi.h"

namespace arboretum {

DataModel::DataModel(uint64_t subgraph_id) : next_id_(subgraph_id + 1) {}

//// BEGIN ARBORETUM GENERATED CODE ////
  uint64_t resolve(clang::AccessSpecifier kind) {
    switch (kind) {
      case clang::AS_public: return 3;
      case clang::AS_protected: return 2;
      case clang::AS_private: return 1;
      case clang::AS_none: return 0;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::ArraySizeModifier kind) {
    switch (kind) {
      case clang::ArraySizeModifier::Normal: return 4;
      case clang::ArraySizeModifier::Static: return 6;
      case clang::ArraySizeModifier::Star: return 5;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::ArrayTypeTrait kind) {
    switch (kind) {
      case clang::ATT_ArrayRank: return 8;
      case clang::ATT_ArrayExtent: return 7;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::AutoTypeKeyword kind) {
    switch (kind) {
      case clang::AutoTypeKeyword::Auto: return 9;
      case clang::AutoTypeKeyword::DecltypeAuto: return 10;
      case clang::AutoTypeKeyword::GNUAutoType: return 11;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::BinaryOperatorKind kind) {
    switch (kind) {
      case clang::BO_PtrMemD: return 33;
      case clang::BO_PtrMemI: return 34;
      case clang::BO_Mul: return 28;
      case clang::BO_Div: return 19;
      case clang::BO_Rem: return 35;
      case clang::BO_Add: return 12;
      case clang::BO_Sub: return 41;
      case clang::BO_Shl: return 37;
      case clang::BO_Shr: return 39;
      case clang::BO_Cmp: return 17;
      case clang::BO_LT: return 27;
      case clang::BO_GT: return 23;
      case clang::BO_LE: return 25;
      case clang::BO_GE: return 22;
      case clang::BO_EQ: return 21;
      case clang::BO_NE: return 30;
      case clang::BO_And: return 14;
      case clang::BO_Xor: return 43;
      case clang::BO_Or: return 31;
      case clang::BO_LAnd: return 24;
      case clang::BO_LOr: return 26;
      case clang::BO_Assign: return 16;
      case clang::BO_MulAssign: return 29;
      case clang::BO_DivAssign: return 20;
      case clang::BO_RemAssign: return 36;
      case clang::BO_AddAssign: return 13;
      case clang::BO_SubAssign: return 42;
      case clang::BO_ShlAssign: return 38;
      case clang::BO_ShrAssign: return 40;
      case clang::BO_AndAssign: return 15;
      case clang::BO_XorAssign: return 44;
      case clang::BO_OrAssign: return 32;
      case clang::BO_Comma: return 18;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::BuiltinTemplateKind kind) {
    switch (kind) {
      case clang::BTK__make_integer_seq: return 45;
      case clang::BTK__type_pack_element: return 46;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::CXXConstructionKind kind) {
    switch (kind) {
      case clang::CXXConstructionKind::Complete: return 47;
      case clang::CXXConstructionKind::NonVirtualBase: return 49;
      case clang::CXXConstructionKind::VirtualBase: return 50;
      case clang::CXXConstructionKind::Delegating: return 48;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::CXXNewInitializationStyle kind) {
    switch (kind) {
      case clang::CXXNewInitializationStyle::None: return 52;
      case clang::CXXNewInitializationStyle::Parens: return 53;
      case clang::CXXNewInitializationStyle::Braces: return 51;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::CallingConv kind) {
    switch (kind) {
      case clang::CC_C: return 59;
      case clang::CC_X86StdCall: return 72;
      case clang::CC_X86FastCall: return 69;
      case clang::CC_X86ThisCall: return 73;
      case clang::CC_X86VectorCall: return 74;
      case clang::CC_X86Pascal: return 70;
      case clang::CC_Win64: return 68;
      case clang::CC_X86_64SysV: return 75;
      case clang::CC_X86RegCall: return 71;
      case clang::CC_AAPCS: return 54;
      case clang::CC_AAPCS_VFP: return 55;
      case clang::CC_IntelOclBicc: return 60;
      case clang::CC_SpirFunction: return 65;
      case clang::CC_OpenCLKernel: return 62;
      case clang::CC_Swift: return 66;
      case clang::CC_SwiftAsync: return 67;
      case clang::CC_PreserveMost: return 64;
      case clang::CC_PreserveAll: return 63;
      case clang::CC_AArch64VectorCall: return 57;
      case clang::CC_AArch64SVEPCS: return 56;
      case clang::CC_AMDGPUKernelCall: return 58;
      case clang::CC_M68kRTD: return 61;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::CanThrowResult kind) {
    switch (kind) {
      case clang::CT_Cannot: return 77;
      case clang::CT_Dependent: return 78;
      case clang::CT_Can: return 76;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::CapturedRegionKind kind) {
    switch (kind) {
      case clang::CR_Default: return 79;
      case clang::CR_ObjCAtFinally: return 80;
      case clang::CR_OpenMP: return 81;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::CastKind kind) {
    switch (kind) {
      case clang::CK_Dependent: return 99;
      case clang::CK_BitCast: return 92;
      case clang::CK_LValueBitCast: return 128;
      case clang::CK_LValueToRValueBitCast: return 130;
      case clang::CK_LValueToRValue: return 129;
      case clang::CK_NoOp: return 133;
      case clang::CK_BaseToDerived: return 90;
      case clang::CK_DerivedToBase: return 100;
      case clang::CK_UncheckedDerivedToBase: return 143;
      case clang::CK_Dynamic: return 102;
      case clang::CK_ToUnion: return 141;
      case clang::CK_ArrayToPointerDecay: return 88;
      case clang::CK_FunctionToPointerDecay: return 116;
      case clang::CK_NullToPointer: return 136;
      case clang::CK_NullToMemberPointer: return 135;
      case clang::CK_BaseToDerivedMemberPointer: return 91;
      case clang::CK_DerivedToBaseMemberPointer: return 101;
      case clang::CK_MemberPointerToBoolean: return 132;
      case clang::CK_ReinterpretMemberPointer: return 140;
      case clang::CK_UserDefinedConversion: return 144;
      case clang::CK_ConstructorConversion: return 97;
      case clang::CK_IntegralToPointer: return 127;
      case clang::CK_PointerToIntegral: return 139;
      case clang::CK_PointerToBoolean: return 138;
      case clang::CK_ToVoid: return 142;
      case clang::CK_MatrixCast: return 131;
      case clang::CK_VectorSplat: return 145;
      case clang::CK_IntegralCast: return 118;
      case clang::CK_IntegralToBoolean: return 124;
      case clang::CK_IntegralToFloating: return 126;
      case clang::CK_FloatingToFixedPoint: return 114;
      case clang::CK_FixedPointToFloating: return 105;
      case clang::CK_FixedPointCast: return 103;
      case clang::CK_FixedPointToIntegral: return 106;
      case clang::CK_IntegralToFixedPoint: return 125;
      case clang::CK_FixedPointToBoolean: return 104;
      case clang::CK_FloatingToIntegral: return 115;
      case clang::CK_FloatingToBoolean: return 113;
      case clang::CK_BooleanToSignedIntegral: return 94;
      case clang::CK_FloatingCast: return 107;
      case clang::CK_CPointerToObjCPointerCast: return 96;
      case clang::CK_BlockPointerToObjCPointerCast: return 93;
      case clang::CK_AnyPointerToBlockPointerCast: return 87;
      case clang::CK_ObjCObjectLValueCast: return 137;
      case clang::CK_FloatingRealToComplex: return 112;
      case clang::CK_FloatingComplexToReal: return 111;
      case clang::CK_FloatingComplexToBoolean: return 109;
      case clang::CK_FloatingComplexCast: return 108;
      case clang::CK_FloatingComplexToIntegralComplex: return 110;
      case clang::CK_IntegralRealToComplex: return 123;
      case clang::CK_IntegralComplexToReal: return 122;
      case clang::CK_IntegralComplexToBoolean: return 120;
      case clang::CK_IntegralComplexCast: return 119;
      case clang::CK_IntegralComplexToFloatingComplex: return 121;
      case clang::CK_ARCProduceObject: return 84;
      case clang::CK_ARCConsumeObject: return 82;
      case clang::CK_ARCReclaimReturnedObject: return 85;
      case clang::CK_ARCExtendBlockObject: return 83;
      case clang::CK_AtomicToNonAtomic: return 89;
      case clang::CK_NonAtomicToAtomic: return 134;
      case clang::CK_CopyAndAutoreleaseBlockObject: return 98;
      case clang::CK_BuiltinFnToFnPtr: return 95;
      case clang::CK_ZeroToOCLOpaqueType: return 146;
      case clang::CK_AddressSpaceConversion: return 86;
      case clang::CK_IntToOCLSampler: return 117;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::CharacterLiteralKind kind) {
    switch (kind) {
      case clang::CharacterLiteralKind::Ascii: return 147;
      case clang::CharacterLiteralKind::Wide: return 151;
      case clang::CharacterLiteralKind::UTF8: return 150;
      case clang::CharacterLiteralKind::UTF16: return 148;
      case clang::CharacterLiteralKind::UTF32: return 149;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::ConstantResultStorageKind kind) {
    switch (kind) {
      case clang::ConstantResultStorageKind::None: return 154;
      case clang::ConstantResultStorageKind::Int64: return 153;
      case clang::ConstantResultStorageKind::APValue: return 152;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::ConstexprSpecKind kind) {
    switch (kind) {
      case clang::ConstexprSpecKind::Unspecified: return 158;
      case clang::ConstexprSpecKind::Constexpr: return 156;
      case clang::ConstexprSpecKind::Consteval: return 155;
      case clang::ConstexprSpecKind::Constinit: return 157;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::DeductionCandidate kind) {
    switch (kind) {
      case clang::DeductionCandidate::Normal: return 161;
      case clang::DeductionCandidate::Copy: return 160;
      case clang::DeductionCandidate::Aggregate: return 159;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::ElaboratedTypeKeyword kind) {
    switch (kind) {
      case clang::ElaboratedTypeKeyword::Struct: return 166;
      case clang::ElaboratedTypeKeyword::Interface: return 164;
      case clang::ElaboratedTypeKeyword::Union: return 168;
      case clang::ElaboratedTypeKeyword::Class: return 162;
      case clang::ElaboratedTypeKeyword::Enum: return 163;
      case clang::ElaboratedTypeKeyword::Typename: return 167;
      case clang::ElaboratedTypeKeyword::None: return 165;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::ExceptionSpecificationType kind) {
    switch (kind) {
      case clang::EST_None: return 177;
      case clang::EST_DynamicNone: return 172;
      case clang::EST_Dynamic: return 171;
      case clang::EST_MSAny: return 173;
      case clang::EST_NoThrow: return 174;
      case clang::EST_BasicNoexcept: return 169;
      case clang::EST_DependentNoexcept: return 170;
      case clang::EST_NoexceptFalse: return 175;
      case clang::EST_NoexceptTrue: return 176;
      case clang::EST_Unevaluated: return 178;
      case clang::EST_Uninstantiated: return 179;
      case clang::EST_Unparsed: return 180;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::ExprObjectKind kind) {
    switch (kind) {
      case clang::OK_Ordinary: return 185;
      case clang::OK_BitField: return 181;
      case clang::OK_VectorComponent: return 186;
      case clang::OK_ObjCProperty: return 183;
      case clang::OK_ObjCSubscript: return 184;
      case clang::OK_MatrixComponent: return 182;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::ExprValueKind kind) {
    switch (kind) {
      case clang::VK_PRValue: return 188;
      case clang::VK_LValue: return 187;
      case clang::VK_XValue: return 189;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::ExpressionTrait kind) {
    switch (kind) {
      case clang::ET_IsLValueExpr: return 190;
      case clang::ET_IsRValueExpr: return 191;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::IfStatementKind kind) {
    switch (kind) {
      case clang::IfStatementKind::Ordinary: return 195;
      case clang::IfStatementKind::Constexpr: return 194;
      case clang::IfStatementKind::ConstevalNonNegated: return 193;
      case clang::IfStatementKind::ConstevalNegated: return 192;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::ImplicitParamKind kind) {
    switch (kind) {
      case clang::ImplicitParamKind::ObjCSelf: return 200;
      case clang::ImplicitParamKind::ObjCCmd: return 199;
      case clang::ImplicitParamKind::CXXThis: return 196;
      case clang::ImplicitParamKind::CXXVTT: return 197;
      case clang::ImplicitParamKind::CapturedContext: return 198;
      case clang::ImplicitParamKind::ThreadPrivateVar: return 202;
      case clang::ImplicitParamKind::Other: return 201;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::InClassInitStyle kind) {
    switch (kind) {
      case clang::ICIS_NoInit: return 205;
      case clang::ICIS_CopyInit: return 203;
      case clang::ICIS_ListInit: return 204;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::LambdaCaptureDefault kind) {
    switch (kind) {
      case clang::LCD_None: return 208;
      case clang::LCD_ByCopy: return 206;
      case clang::LCD_ByRef: return 207;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::LanguageLinkage kind) {
    switch (kind) {
      case clang::CLanguageLinkage: return 209;
      case clang::CXXLanguageLinkage: return 210;
      case clang::NoLanguageLinkage: return 211;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::Linkage kind) {
    switch (kind) {
      case clang::Linkage::Invalid: return 214;
      case clang::Linkage::None: return 216;
      case clang::Linkage::Internal: return 213;
      case clang::Linkage::UniqueExternal: return 217;
      case clang::Linkage::VisibleNone: return 218;
      case clang::Linkage::Module: return 215;
      case clang::Linkage::External: return 212;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::LinkageSpecLanguageIDs kind) {
    switch (kind) {
      case clang::LinkageSpecLanguageIDs::C: return 219;
      case clang::LinkageSpecLanguageIDs::CXX: return 220;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::MSVtorDispMode kind) {
    switch (kind) {
      case clang::MSVtorDispMode::Never: return 223;
      case clang::MSVtorDispMode::ForVBaseOverride: return 221;
      case clang::MSVtorDispMode::ForVFTable: return 222;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::MultiVersionKind kind) {
    switch (kind) {
      case clang::MultiVersionKind::None: return 226;
      case clang::MultiVersionKind::Target: return 227;
      case clang::MultiVersionKind::CPUSpecific: return 225;
      case clang::MultiVersionKind::CPUDispatch: return 224;
      case clang::MultiVersionKind::TargetClones: return 228;
      case clang::MultiVersionKind::TargetVersion: return 229;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::NonOdrUseReason kind) {
    switch (kind) {
      case clang::NOUR_None: return 232;
      case clang::NOUR_Unevaluated: return 233;
      case clang::NOUR_Constant: return 230;
      case clang::NOUR_Discarded: return 231;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::ObjCStringFormatFamily kind) {
    switch (kind) {
      case clang::SFF_None: return 236;
      case clang::SFF_NSString: return 235;
      case clang::SFF_CFString: return 234;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::OverloadedOperatorKind kind) {
    switch (kind) {
      case clang::OO_None: return 268;
      case clang::OO_New: return 267;
      case clang::OO_Delete: return 251;
      case clang::OO_Array_New: return 242;
      case clang::OO_Array_Delete: return 241;
      case clang::OO_Plus: return 274;
      case clang::OO_Minus: return 264;
      case clang::OO_Star: return 280;
      case clang::OO_Slash: return 277;
      case clang::OO_Percent: return 269;
      case clang::OO_Caret: return 246;
      case clang::OO_Amp: return 238;
      case clang::OO_Pipe: return 271;
      case clang::OO_Tilde: return 283;
      case clang::OO_Exclaim: return 254;
      case clang::OO_Equal: return 252;
      case clang::OO_Less: return 260;
      case clang::OO_Greater: return 256;
      case clang::OO_PlusEqual: return 275;
      case clang::OO_MinusEqual: return 265;
      case clang::OO_StarEqual: return 281;
      case clang::OO_SlashEqual: return 278;
      case clang::OO_PercentEqual: return 270;
      case clang::OO_CaretEqual: return 247;
      case clang::OO_AmpEqual: return 240;
      case clang::OO_PipeEqual: return 272;
      case clang::OO_LessLess: return 262;
      case clang::OO_GreaterGreater: return 258;
      case clang::OO_LessLessEqual: return 263;
      case clang::OO_GreaterGreaterEqual: return 259;
      case clang::OO_EqualEqual: return 253;
      case clang::OO_ExclaimEqual: return 255;
      case clang::OO_LessEqual: return 261;
      case clang::OO_GreaterEqual: return 257;
      case clang::OO_Spaceship: return 279;
      case clang::OO_AmpAmp: return 239;
      case clang::OO_PipePipe: return 273;
      case clang::OO_PlusPlus: return 276;
      case clang::OO_MinusMinus: return 266;
      case clang::OO_Comma: return 249;
      case clang::OO_ArrowStar: return 244;
      case clang::OO_Arrow: return 243;
      case clang::OO_Call: return 245;
      case clang::OO_Subscript: return 282;
      case clang::OO_Conditional: return 250;
      case clang::OO_Coawait: return 248;
      case clang::NUM_OVERLOADED_OPERATORS: return 237;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::PragmaMSCommentKind kind) {
    switch (kind) {
      case clang::PCK_Unknown: return 288;
      case clang::PCK_Linker: return 287;
      case clang::PCK_Lib: return 286;
      case clang::PCK_Compiler: return 284;
      case clang::PCK_ExeStr: return 285;
      case clang::PCK_User: return 289;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::PredefinedIdentKind kind) {
    switch (kind) {
      case clang::PredefinedIdentKind::Func: return 290;
      case clang::PredefinedIdentKind::Function: return 293;
      case clang::PredefinedIdentKind::LFunction: return 295;
      case clang::PredefinedIdentKind::FuncDName: return 291;
      case clang::PredefinedIdentKind::FuncSig: return 292;
      case clang::PredefinedIdentKind::LFuncSig: return 294;
      case clang::PredefinedIdentKind::PrettyFunction: return 296;
      case clang::PredefinedIdentKind::PrettyFunctionNoVirtual: return 297;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::RecordArgPassingKind kind) {
    switch (kind) {
      case clang::RecordArgPassingKind::CanPassInRegs: return 299;
      case clang::RecordArgPassingKind::CannotPassInRegs: return 300;
      case clang::RecordArgPassingKind::CanNeverPassInRegs: return 298;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::RefQualifierKind kind) {
    switch (kind) {
      case clang::RQ_None: return 302;
      case clang::RQ_LValue: return 301;
      case clang::RQ_RValue: return 303;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::SourceLocIdentKind kind) {
    switch (kind) {
      case clang::SourceLocIdentKind::Function: return 308;
      case clang::SourceLocIdentKind::FuncSig: return 307;
      case clang::SourceLocIdentKind::File: return 305;
      case clang::SourceLocIdentKind::FileName: return 306;
      case clang::SourceLocIdentKind::Line: return 309;
      case clang::SourceLocIdentKind::Column: return 304;
      case clang::SourceLocIdentKind::SourceLocStruct: return 310;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::StorageClass kind) {
    switch (kind) {
      case clang::SC_None: return 313;
      case clang::SC_Extern: return 312;
      case clang::SC_Static: return 316;
      case clang::SC_PrivateExtern: return 314;
      case clang::SC_Auto: return 311;
      case clang::SC_Register: return 315;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::StorageDuration kind) {
    switch (kind) {
      case clang::SD_FullExpression: return 319;
      case clang::SD_Automatic: return 317;
      case clang::SD_Thread: return 321;
      case clang::SD_Static: return 320;
      case clang::SD_Dynamic: return 318;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::StringLiteralKind kind) {
    switch (kind) {
      case clang::StringLiteralKind::Ordinary: return 322;
      case clang::StringLiteralKind::Wide: return 327;
      case clang::StringLiteralKind::UTF8: return 325;
      case clang::StringLiteralKind::UTF16: return 323;
      case clang::StringLiteralKind::UTF32: return 324;
      case clang::StringLiteralKind::Unevaluated: return 326;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::TagTypeKind kind) {
    switch (kind) {
      case clang::TagTypeKind::Struct: return 331;
      case clang::TagTypeKind::Interface: return 330;
      case clang::TagTypeKind::Union: return 332;
      case clang::TagTypeKind::Class: return 328;
      case clang::TagTypeKind::Enum: return 329;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::TemplateSpecializationKind kind) {
    switch (kind) {
      case clang::TSK_Undeclared: return 337;
      case clang::TSK_ImplicitInstantiation: return 336;
      case clang::TSK_ExplicitSpecialization: return 335;
      case clang::TSK_ExplicitInstantiationDeclaration: return 333;
      case clang::TSK_ExplicitInstantiationDefinition: return 334;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::ThreadStorageClassSpecifier kind) {
    switch (kind) {
      case clang::TSCS_unspecified: return 341;
      case clang::TSCS___thread: return 339;
      case clang::TSCS_thread_local: return 340;
      case clang::TSCS__Thread_local: return 338;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::TypeOfKind kind) {
    switch (kind) {
      case clang::TypeOfKind::Qualified: return 342;
      case clang::TypeOfKind::Unqualified: return 343;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::TypeTrait kind) {
    switch (kind) {
      case clang::UTT_IsInterfaceClass: return 387;
      case clang::UTT_IsSealed: return 404;
      case clang::UTT_IsDestructible: return 379;
      case clang::UTT_IsTriviallyDestructible: return 409;
      case clang::UTT_IsNothrowDestructible: return 393;
      case clang::UTT_HasNothrowMoveAssign: return 361;
      case clang::UTT_HasTrivialMoveAssign: return 366;
      case clang::UTT_HasTrivialMoveConstructor: return 367;
      case clang::UTT_HasNothrowAssign: return 358;
      case clang::UTT_HasNothrowCopy: return 360;
      case clang::UTT_HasNothrowConstructor: return 359;
      case clang::UTT_HasTrivialAssign: return 362;
      case clang::UTT_HasTrivialCopy: return 363;
      case clang::UTT_HasTrivialDefaultConstructor: return 364;
      case clang::UTT_HasTrivialDestructor: return 365;
      case clang::UTT_HasVirtualDestructor: return 369;
      case clang::UTT_IsAbstract: return 370;
      case clang::UTT_IsAggregate: return 371;
      case clang::UTT_IsClass: return 375;
      case clang::UTT_IsEmpty: return 380;
      case clang::UTT_IsEnum: return 381;
      case clang::UTT_IsFinal: return 382;
      case clang::UTT_IsLiteral: return 388;
      case clang::UTT_IsPOD: return 396;
      case clang::UTT_IsPolymorphic: return 398;
      case clang::UTT_IsStandardLayout: return 406;
      case clang::UTT_IsTrivial: return 407;
      case clang::UTT_IsTriviallyCopyable: return 408;
      case clang::UTT_IsUnion: return 413;
      case clang::UTT_HasUniqueObjectRepresentations: return 368;
      case clang::UTT_IsTriviallyRelocatable: return 411;
      case clang::UTT_IsTriviallyEqualityComparable: return 410;
      case clang::UTT_IsBoundedArray: return 374;
      case clang::UTT_IsUnboundedArray: return 412;
      case clang::UTT_IsNullPointer: return 394;
      case clang::UTT_IsScopedEnum: return 403;
      case clang::UTT_IsReferenceable: return 400;
      case clang::UTT_CanPassInRegs: return 357;
      case clang::UTT_IsArithmetic: return 372;
      case clang::UTT_IsFloatingPoint: return 383;
      case clang::UTT_IsIntegral: return 386;
      case clang::UTT_IsCompleteType: return 376;
      case clang::UTT_IsVoid: return 415;
      case clang::UTT_IsArray: return 373;
      case clang::UTT_IsFunction: return 384;
      case clang::UTT_IsReference: return 399;
      case clang::UTT_IsLvalueReference: return 389;
      case clang::UTT_IsRvalueReference: return 401;
      case clang::UTT_IsFundamental: return 385;
      case clang::UTT_IsObject: return 395;
      case clang::UTT_IsScalar: return 402;
      case clang::UTT_IsCompound: return 377;
      case clang::UTT_IsPointer: return 397;
      case clang::UTT_IsMemberObjectPointer: return 391;
      case clang::UTT_IsMemberFunctionPointer: return 390;
      case clang::UTT_IsMemberPointer: return 392;
      case clang::UTT_IsConst: return 378;
      case clang::UTT_IsVolatile: return 416;
      case clang::UTT_IsSigned: return 405;
      case clang::UTT_IsUnsigned: return 414;
      case clang::BTT_TypeCompatible: return 353;
      case clang::BTT_IsNothrowAssignable: return 348;
      case clang::BTT_IsAssignable: return 344;
      case clang::BTT_IsBaseOf: return 345;
      case clang::BTT_IsConvertibleTo: return 347;
      case clang::BTT_IsTriviallyAssignable: return 350;
      case clang::BTT_ReferenceBindsToTemporary: return 351;
      case clang::BTT_ReferenceConstructsFromTemporary: return 352;
      case clang::BTT_IsSame: return 349;
      case clang::BTT_IsConvertible: return 346;
      case clang::TT_IsConstructible: return 354;
      case clang::TT_IsNothrowConstructible: return 355;
      case clang::TT_IsTriviallyConstructible: return 356;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::UnaryExprOrTypeTrait kind) {
    switch (kind) {
      case clang::UETT_SizeOf: return 421;
      case clang::UETT_DataSizeOf: return 418;
      case clang::UETT_AlignOf: return 417;
      case clang::UETT_PreferredAlignOf: return 420;
      case clang::UETT_VecStep: return 422;
      case clang::UETT_OpenMPRequiredSimdAlign: return 419;
      case clang::UETT_VectorElements: return 423;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::UnaryOperatorKind kind) {
    switch (kind) {
      case clang::UO_PostInc: return 434;
      case clang::UO_PostDec: return 433;
      case clang::UO_PreInc: return 436;
      case clang::UO_PreDec: return 435;
      case clang::UO_AddrOf: return 424;
      case clang::UO_Deref: return 426;
      case clang::UO_Plus: return 432;
      case clang::UO_Minus: return 430;
      case clang::UO_Not: return 431;
      case clang::UO_LNot: return 429;
      case clang::UO_Real: return 437;
      case clang::UO_Imag: return 428;
      case clang::UO_Extension: return 427;
      case clang::UO_Coawait: return 425;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::VectorKind kind) {
    switch (kind) {
      case clang::VectorKind::Generic: return 441;
      case clang::VectorKind::AltiVecVector: return 440;
      case clang::VectorKind::AltiVecPixel: return 439;
      case clang::VectorKind::AltiVecBool: return 438;
      case clang::VectorKind::Neon: return 442;
      case clang::VectorKind::NeonPoly: return 443;
      case clang::VectorKind::SveFixedLengthData: return 446;
      case clang::VectorKind::SveFixedLengthPredicate: return 447;
      case clang::VectorKind::RVVFixedLengthData: return 444;
      case clang::VectorKind::RVVFixedLengthMask: return 445;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::Visibility kind) {
    switch (kind) {
      case clang::HiddenVisibility: return 449;
      case clang::ProtectedVisibility: return 450;
      case clang::DefaultVisibility: return 448;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::attr::Kind kind) {
    switch (kind) {
      case clang::attr::AddressSpace: return 466;
      case clang::attr::AnnotateType: return 478;
      case clang::attr::ArmIn: return 485;
      case clang::attr::ArmInOut: return 486;
      case clang::attr::ArmMveStrictPolymorphism: return 488;
      case clang::attr::ArmOut: return 490;
      case clang::attr::ArmPreserves: return 491;
      case clang::attr::ArmStreaming: return 492;
      case clang::attr::ArmStreamingCompatible: return 493;
      case clang::attr::BTFTypeTag: return 506;
      case clang::attr::CmseNSCall: return 538;
      case clang::attr::HLSLGroupSharedAddressSpace: return 589;
      case clang::attr::HLSLParamModifier: return 591;
      case clang::attr::NoDeref: return 650;
      case clang::attr::ObjCGC: return 697;
      case clang::attr::ObjCInertUnsafeUnretained: return 699;
      case clang::attr::ObjCKindOf: return 700;
      case clang::attr::OpenCLConstantAddressSpace: return 715;
      case clang::attr::OpenCLGenericAddressSpace: return 716;
      case clang::attr::OpenCLGlobalAddressSpace: return 717;
      case clang::attr::OpenCLGlobalDeviceAddressSpace: return 718;
      case clang::attr::OpenCLGlobalHostAddressSpace: return 719;
      case clang::attr::OpenCLLocalAddressSpace: return 722;
      case clang::attr::OpenCLPrivateAddressSpace: return 723;
      case clang::attr::Ptr32: return 748;
      case clang::attr::Ptr64: return 749;
      case clang::attr::SPtr: return 766;
      case clang::attr::TypeNonNull: return 813;
      case clang::attr::TypeNullUnspecified: return 814;
      case clang::attr::TypeNullable: return 815;
      case clang::attr::TypeNullableResult: return 816;
      case clang::attr::UPtr: return 819;
      case clang::attr::WebAssemblyFuncref: return 839;
      case clang::attr::CodeAlign: return 540;
      case clang::attr::FallThrough: return 577;
      case clang::attr::Likely: return 610;
      case clang::attr::MustTail: return 637;
      case clang::attr::OpenCLUnrollHint: return 724;
      case clang::attr::Unlikely: return 822;
      case clang::attr::AlwaysInline: return 475;
      case clang::attr::NoInline: return 654;
      case clang::attr::NoMerge: return 656;
      case clang::attr::Suppress: return 780;
      case clang::attr::AArch64SVEPcs: return 451;
      case clang::attr::AArch64VectorPcs: return 452;
      case clang::attr::AMDGPUKernelCall: return 454;
      case clang::attr::AcquireHandle: return 463;
      case clang::attr::AnyX86NoCfCheck: return 481;
      case clang::attr::CDecl: return 511;
      case clang::attr::FastCall: return 578;
      case clang::attr::IntelOclBicc: return 604;
      case clang::attr::LifetimeBound: return 609;
      case clang::attr::M68kRTD: return 616;
      case clang::attr::MSABI: return 618;
      case clang::attr::NSReturnsRetained: return 643;
      case clang::attr::ObjCOwnership: return 705;
      case clang::attr::Pascal: return 732;
      case clang::attr::Pcs: return 735;
      case clang::attr::PreserveAll: return 744;
      case clang::attr::PreserveMost: return 745;
      case clang::attr::RegCall: return 754;
      case clang::attr::StdCall: return 777;
      case clang::attr::SwiftAsyncCall: return 782;
      case clang::attr::SwiftCall: return 789;
      case clang::attr::SysVABI: return 802;
      case clang::attr::ThisCall: return 808;
      case clang::attr::VectorCall: return 831;
      case clang::attr::SwiftAsyncContext: return 783;
      case clang::attr::SwiftContext: return 790;
      case clang::attr::SwiftErrorResult: return 792;
      case clang::attr::SwiftIndirectResult: return 795;
      case clang::attr::Annotate: return 477;
      case clang::attr::CFConsumed: return 513;
      case clang::attr::CarriesDependency: return 536;
      case clang::attr::NSConsumed: return 638;
      case clang::attr::NonNull: return 670;
      case clang::attr::OSConsumed: return 680;
      case clang::attr::PassObjectSize: return 733;
      case clang::attr::ReleaseHandle: return 757;
      case clang::attr::UseHandle: return 825;
      case clang::attr::HLSLSV_DispatchThreadID: return 594;
      case clang::attr::HLSLSV_GroupIndex: return 595;
      case clang::attr::AMDGPUFlatWorkGroupSize: return 453;
      case clang::attr::AMDGPUNumSGPR: return 455;
      case clang::attr::AMDGPUNumVGPR: return 456;
      case clang::attr::AMDGPUWavesPerEU: return 457;
      case clang::attr::ARMInterrupt: return 458;
      case clang::attr::AVRInterrupt: return 459;
      case clang::attr::AVRSignal: return 460;
      case clang::attr::AcquireCapability: return 462;
      case clang::attr::AcquiredAfter: return 464;
      case clang::attr::AcquiredBefore: return 465;
      case clang::attr::AlignMac68k: return 468;
      case clang::attr::AlignNatural: return 469;
      case clang::attr::Aligned: return 471;
      case clang::attr::AllocAlign: return 472;
      case clang::attr::AllocSize: return 473;
      case clang::attr::AlwaysDestroy: return 474;
      case clang::attr::AnalyzerNoReturn: return 476;
      case clang::attr::AnyX86Interrupt: return 479;
      case clang::attr::AnyX86NoCallerSavedRegisters: return 480;
      case clang::attr::ArcWeakrefUnavailable: return 482;
      case clang::attr::ArgumentWithTypeTag: return 483;
      case clang::attr::ArmBuiltinAlias: return 484;
      case clang::attr::ArmLocallyStreaming: return 487;
      case clang::attr::ArmNew: return 489;
      case clang::attr::Artificial: return 494;
      case clang::attr::AsmLabel: return 495;
      case clang::attr::AssertCapability: return 496;
      case clang::attr::AssertExclusiveLock: return 497;
      case clang::attr::AssertSharedLock: return 498;
      case clang::attr::AssumeAligned: return 499;
      case clang::attr::Assumption: return 500;
      case clang::attr::Availability: return 501;
      case clang::attr::AvailableOnlyInDefaultEvalMethod: return 502;
      case clang::attr::BPFPreserveAccessIndex: return 503;
      case clang::attr::BPFPreserveStaticOffset: return 504;
      case clang::attr::BTFDeclTag: return 505;
      case clang::attr::Blocks: return 507;
      case clang::attr::Builtin: return 508;
      case clang::attr::C11NoReturn: return 510;
      case clang::attr::CFAuditedTransfer: return 512;
      case clang::attr::CFGuard: return 514;
      case clang::attr::CFICanonicalJumpTable: return 515;
      case clang::attr::CFReturnsNotRetained: return 516;
      case clang::attr::CFReturnsRetained: return 517;
      case clang::attr::CFUnknownTransfer: return 518;
      case clang::attr::CPUDispatch: return 519;
      case clang::attr::CPUSpecific: return 520;
      case clang::attr::CUDAConstant: return 521;
      case clang::attr::CUDADevice: return 522;
      case clang::attr::CUDADeviceBuiltinSurfaceType: return 523;
      case clang::attr::CUDADeviceBuiltinTextureType: return 524;
      case clang::attr::CUDAGlobal: return 525;
      case clang::attr::CUDAHost: return 526;
      case clang::attr::CUDAInvalidTarget: return 527;
      case clang::attr::CUDALaunchBounds: return 528;
      case clang::attr::CUDAShared: return 529;
      case clang::attr::CXX11NoReturn: return 530;
      case clang::attr::CallableWhen: return 531;
      case clang::attr::Callback: return 532;
      case clang::attr::Capability: return 534;
      case clang::attr::CapturedRecord: return 535;
      case clang::attr::Cleanup: return 537;
      case clang::attr::CmseNSEntry: return 539;
      case clang::attr::CodeModel: return 541;
      case clang::attr::CodeSeg: return 542;
      case clang::attr::Cold: return 543;
      case clang::attr::Common: return 544;
      case clang::attr::Const: return 545;
      case clang::attr::ConstInit: return 546;
      case clang::attr::Constructor: return 547;
      case clang::attr::Consumable: return 548;
      case clang::attr::ConsumableAutoCast: return 549;
      case clang::attr::ConsumableSetOnRead: return 550;
      case clang::attr::Convergent: return 551;
      case clang::attr::CoroDisableLifetimeBound: return 552;
      case clang::attr::CoroLifetimeBound: return 553;
      case clang::attr::CoroOnlyDestroyWhenComplete: return 554;
      case clang::attr::CoroReturnType: return 555;
      case clang::attr::CoroWrapper: return 556;
      case clang::attr::CountedBy: return 557;
      case clang::attr::DLLExport: return 558;
      case clang::attr::DLLExportStaticLocal: return 559;
      case clang::attr::DLLImport: return 560;
      case clang::attr::DLLImportStaticLocal: return 561;
      case clang::attr::Deprecated: return 562;
      case clang::attr::Destructor: return 563;
      case clang::attr::DiagnoseAsBuiltin: return 564;
      case clang::attr::DiagnoseIf: return 565;
      case clang::attr::DisableSanitizerInstrumentation: return 566;
      case clang::attr::DisableTailCalls: return 567;
      case clang::attr::EmptyBases: return 568;
      case clang::attr::EnableIf: return 569;
      case clang::attr::EnforceTCB: return 570;
      case clang::attr::EnforceTCBLeaf: return 571;
      case clang::attr::EnumExtensibility: return 572;
      case clang::attr::Error: return 573;
      case clang::attr::ExcludeFromExplicitInstantiation: return 574;
      case clang::attr::ExclusiveTrylockFunction: return 575;
      case clang::attr::ExternalSourceSymbol: return 576;
      case clang::attr::Final: return 579;
      case clang::attr::FlagEnum: return 580;
      case clang::attr::Flatten: return 581;
      case clang::attr::Format: return 582;
      case clang::attr::FormatArg: return 583;
      case clang::attr::FunctionReturnThunks: return 584;
      case clang::attr::GNUInline: return 585;
      case clang::attr::GuardedBy: return 586;
      case clang::attr::GuardedVar: return 587;
      case clang::attr::HIPManaged: return 588;
      case clang::attr::HLSLNumThreads: return 590;
      case clang::attr::HLSLResource: return 592;
      case clang::attr::HLSLResourceBinding: return 593;
      case clang::attr::HLSLShader: return 596;
      case clang::attr::Hot: return 597;
      case clang::attr::IBAction: return 598;
      case clang::attr::IBOutlet: return 599;
      case clang::attr::IBOutletCollection: return 600;
      case clang::attr::InitPriority: return 602;
      case clang::attr::InternalLinkage: return 605;
      case clang::attr::LTOVisibilityPublic: return 606;
      case clang::attr::LayoutVersion: return 607;
      case clang::attr::Leaf: return 608;
      case clang::attr::LockReturned: return 612;
      case clang::attr::LocksExcluded: return 613;
      case clang::attr::M68kInterrupt: return 615;
      case clang::attr::MIGServerRoutine: return 617;
      case clang::attr::MSAllocator: return 619;
      case clang::attr::MSConstexpr: return 620;
      case clang::attr::MSInheritance: return 621;
      case clang::attr::MSNoVTable: return 622;
      case clang::attr::MSP430Interrupt: return 623;
      case clang::attr::MSStruct: return 624;
      case clang::attr::MSVtorDisp: return 625;
      case clang::attr::MaxFieldAlignment: return 626;
      case clang::attr::MayAlias: return 627;
      case clang::attr::MaybeUndef: return 628;
      case clang::attr::MicroMips: return 629;
      case clang::attr::MinSize: return 630;
      case clang::attr::MinVectorWidth: return 631;
      case clang::attr::Mips16: return 632;
      case clang::attr::MipsInterrupt: return 633;
      case clang::attr::MipsLongCall: return 634;
      case clang::attr::MipsShortCall: return 635;
      case clang::attr::NSConsumesSelf: return 639;
      case clang::attr::NSErrorDomain: return 640;
      case clang::attr::NSReturnsAutoreleased: return 641;
      case clang::attr::NSReturnsNotRetained: return 642;
      case clang::attr::NVPTXKernel: return 644;
      case clang::attr::Naked: return 645;
      case clang::attr::NoAlias: return 646;
      case clang::attr::NoCommon: return 648;
      case clang::attr::NoDebug: return 649;
      case clang::attr::NoDestroy: return 651;
      case clang::attr::NoDuplicate: return 652;
      case clang::attr::NoInstrumentFunction: return 655;
      case clang::attr::NoMicroMips: return 657;
      case clang::attr::NoMips16: return 658;
      case clang::attr::NoProfileFunction: return 659;
      case clang::attr::NoRandomizeLayout: return 660;
      case clang::attr::NoReturn: return 661;
      case clang::attr::NoSanitize: return 662;
      case clang::attr::NoSpeculativeLoadHardening: return 663;
      case clang::attr::NoSplitStack: return 664;
      case clang::attr::NoStackProtector: return 665;
      case clang::attr::NoThreadSafetyAnalysis: return 666;
      case clang::attr::NoThrow: return 667;
      case clang::attr::NoUniqueAddress: return 668;
      case clang::attr::NoUwtable: return 669;
      case clang::attr::NotTailCalled: return 671;
      case clang::attr::OMPAllocateDecl: return 672;
      case clang::attr::OMPCaptureNoInit: return 674;
      case clang::attr::OMPDeclareTargetDecl: return 676;
      case clang::attr::OMPDeclareVariant: return 677;
      case clang::attr::OMPThreadPrivateDecl: return 679;
      case clang::attr::OSConsumesThis: return 681;
      case clang::attr::OSReturnsNotRetained: return 682;
      case clang::attr::OSReturnsRetained: return 683;
      case clang::attr::OSReturnsRetainedOnNonZero: return 684;
      case clang::attr::OSReturnsRetainedOnZero: return 685;
      case clang::attr::ObjCBridge: return 687;
      case clang::attr::ObjCBridgeMutable: return 688;
      case clang::attr::ObjCBridgeRelated: return 689;
      case clang::attr::ObjCException: return 694;
      case clang::attr::ObjCExplicitProtocolImpl: return 695;
      case clang::attr::ObjCExternallyRetained: return 696;
      case clang::attr::ObjCIndependentClass: return 698;
      case clang::attr::ObjCMethodFamily: return 701;
      case clang::attr::ObjCNSObject: return 702;
      case clang::attr::ObjCPreciseLifetime: return 706;
      case clang::attr::ObjCRequiresPropertyDefs: return 707;
      case clang::attr::ObjCRequiresSuper: return 708;
      case clang::attr::ObjCReturnsInnerPointer: return 709;
      case clang::attr::ObjCRootClass: return 710;
      case clang::attr::ObjCSubclassingRestricted: return 713;
      case clang::attr::OpenCLIntelReqdSubGroupSize: return 720;
      case clang::attr::OpenCLKernel: return 721;
      case clang::attr::OptimizeNone: return 725;
      case clang::attr::Override: return 727;
      case clang::attr::Owner: return 728;
      case clang::attr::Ownership: return 729;
      case clang::attr::Packed: return 730;
      case clang::attr::ParamTypestate: return 731;
      case clang::attr::PatchableFunctionEntry: return 734;
      case clang::attr::Pointer: return 736;
      case clang::attr::PragmaClangBSSSection: return 737;
      case clang::attr::PragmaClangDataSection: return 738;
      case clang::attr::PragmaClangRelroSection: return 739;
      case clang::attr::PragmaClangRodataSection: return 740;
      case clang::attr::PragmaClangTextSection: return 741;
      case clang::attr::PreferredName: return 742;
      case clang::attr::PreferredType: return 743;
      case clang::attr::PtGuardedBy: return 746;
      case clang::attr::PtGuardedVar: return 747;
      case clang::attr::Pure: return 750;
      case clang::attr::RISCVInterrupt: return 751;
      case clang::attr::RandomizeLayout: return 752;
      case clang::attr::ReadOnlyPlacement: return 753;
      case clang::attr::Reinitializes: return 755;
      case clang::attr::ReleaseCapability: return 756;
      case clang::attr::ReqdWorkGroupSize: return 759;
      case clang::attr::RequiresCapability: return 760;
      case clang::attr::Restrict: return 761;
      case clang::attr::Retain: return 762;
      case clang::attr::ReturnTypestate: return 763;
      case clang::attr::ReturnsNonNull: return 764;
      case clang::attr::ReturnsTwice: return 765;
      case clang::attr::SYCLKernel: return 767;
      case clang::attr::SYCLSpecialClass: return 768;
      case clang::attr::ScopedLockable: return 769;
      case clang::attr::Section: return 770;
      case clang::attr::SelectAny: return 771;
      case clang::attr::Sentinel: return 772;
      case clang::attr::SetTypestate: return 773;
      case clang::attr::SharedTrylockFunction: return 774;
      case clang::attr::SpeculativeLoadHardening: return 775;
      case clang::attr::StandaloneDebug: return 776;
      case clang::attr::StrictFP: return 778;
      case clang::attr::StrictGuardStackCheck: return 779;
      case clang::attr::SwiftAsync: return 781;
      case clang::attr::SwiftAsyncError: return 784;
      case clang::attr::SwiftAsyncName: return 785;
      case clang::attr::SwiftAttr: return 786;
      case clang::attr::SwiftBridge: return 787;
      case clang::attr::SwiftBridgedTypedef: return 788;
      case clang::attr::SwiftError: return 791;
      case clang::attr::SwiftImportAsNonGeneric: return 793;
      case clang::attr::SwiftImportPropertyAsAccessors: return 794;
      case clang::attr::SwiftName: return 796;
      case clang::attr::SwiftNewType: return 797;
      case clang::attr::SwiftPrivate: return 799;
      case clang::attr::TLSModel: return 803;
      case clang::attr::Target: return 804;
      case clang::attr::TargetClones: return 805;
      case clang::attr::TargetVersion: return 806;
      case clang::attr::TestTypestate: return 807;
      case clang::attr::TransparentUnion: return 810;
      case clang::attr::TrivialABI: return 811;
      case clang::attr::TryAcquireCapability: return 812;
      case clang::attr::TypeTagForDatatype: return 817;
      case clang::attr::TypeVisibility: return 818;
      case clang::attr::Unavailable: return 820;
      case clang::attr::Uninitialized: return 821;
      case clang::attr::UnsafeBufferUsage: return 823;
      case clang::attr::Unused: return 824;
      case clang::attr::Used: return 826;
      case clang::attr::UsingIfExists: return 827;
      case clang::attr::Uuid: return 828;
      case clang::attr::VecReturn: return 829;
      case clang::attr::VecTypeHint: return 830;
      case clang::attr::Visibility: return 832;
      case clang::attr::WarnUnused: return 833;
      case clang::attr::WarnUnusedResult: return 834;
      case clang::attr::Weak: return 835;
      case clang::attr::WeakImport: return 836;
      case clang::attr::WeakRef: return 837;
      case clang::attr::WebAssemblyExportName: return 838;
      case clang::attr::WebAssemblyImportModule: return 840;
      case clang::attr::WebAssemblyImportName: return 841;
      case clang::attr::WorkGroupSizeHint: return 842;
      case clang::attr::X86ForceAlignArgPointer: return 843;
      case clang::attr::XRayInstrument: return 844;
      case clang::attr::XRayLogArgs: return 845;
      case clang::attr::ZeroCallUsedRegs: return 846;
      case clang::attr::AbiTag: return 461;
      case clang::attr::Alias: return 467;
      case clang::attr::AlignValue: return 470;
      case clang::attr::BuiltinAlias: return 509;
      case clang::attr::CalledOnce: return 533;
      case clang::attr::IFunc: return 601;
      case clang::attr::InitSeg: return 603;
      case clang::attr::LoaderUninitialized: return 611;
      case clang::attr::LoopHint: return 614;
      case clang::attr::Mode: return 636;
      case clang::attr::NoBuiltin: return 647;
      case clang::attr::NoEscape: return 653;
      case clang::attr::OMPCaptureKind: return 673;
      case clang::attr::OMPDeclareSimdDecl: return 675;
      case clang::attr::OMPReferencedVar: return 678;
      case clang::attr::ObjCBoxable: return 686;
      case clang::attr::ObjCClassStub: return 690;
      case clang::attr::ObjCDesignatedInitializer: return 691;
      case clang::attr::ObjCDirect: return 692;
      case clang::attr::ObjCDirectMembers: return 693;
      case clang::attr::ObjCNonLazyClass: return 703;
      case clang::attr::ObjCNonRuntimeProtocol: return 704;
      case clang::attr::ObjCRuntimeName: return 711;
      case clang::attr::ObjCRuntimeVisible: return 712;
      case clang::attr::OpenCLAccess: return 714;
      case clang::attr::Overloadable: return 726;
      case clang::attr::RenderScriptKernel: return 758;
      case clang::attr::SwiftObjCMembers: return 798;
      case clang::attr::SwiftVersionedAddition: return 800;
      case clang::attr::SwiftVersionedRemoval: return 801;
      case clang::attr::Thread: return 809;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::APValue::ValueKind kind) {
    switch (kind) {
      case clang::APValue::None: return 857;
      case clang::APValue::Indeterminate: return 853;
      case clang::APValue::Int: return 854;
      case clang::APValue::Float: return 852;
      case clang::APValue::FixedPoint: return 851;
      case clang::APValue::ComplexInt: return 850;
      case clang::APValue::ComplexFloat: return 849;
      case clang::APValue::LValue: return 855;
      case clang::APValue::Vector: return 860;
      case clang::APValue::Array: return 848;
      case clang::APValue::Struct: return 858;
      case clang::APValue::Union: return 859;
      case clang::APValue::MemberPointer: return 856;
      case clang::APValue::AddrLabelDiff: return 847;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::AtomicExpr::AtomicOp kind) {
    switch (kind) {
      case clang::AtomicExpr::AO__c11_atomic_init: return 896;
      case clang::AtomicExpr::AO__c11_atomic_load: return 897;
      case clang::AtomicExpr::AO__c11_atomic_store: return 898;
      case clang::AtomicExpr::AO__c11_atomic_exchange: return 887;
      case clang::AtomicExpr::AO__c11_atomic_compare_exchange_strong: return 885;
      case clang::AtomicExpr::AO__c11_atomic_compare_exchange_weak: return 886;
      case clang::AtomicExpr::AO__c11_atomic_fetch_add: return 888;
      case clang::AtomicExpr::AO__c11_atomic_fetch_sub: return 894;
      case clang::AtomicExpr::AO__c11_atomic_fetch_and: return 889;
      case clang::AtomicExpr::AO__c11_atomic_fetch_or: return 893;
      case clang::AtomicExpr::AO__c11_atomic_fetch_xor: return 895;
      case clang::AtomicExpr::AO__c11_atomic_fetch_nand: return 892;
      case clang::AtomicExpr::AO__c11_atomic_fetch_max: return 890;
      case clang::AtomicExpr::AO__c11_atomic_fetch_min: return 891;
      case clang::AtomicExpr::AO__atomic_load: return 875;
      case clang::AtomicExpr::AO__atomic_load_n: return 876;
      case clang::AtomicExpr::AO__atomic_store: return 881;
      case clang::AtomicExpr::AO__atomic_store_n: return 882;
      case clang::AtomicExpr::AO__atomic_exchange: return 865;
      case clang::AtomicExpr::AO__atomic_exchange_n: return 866;
      case clang::AtomicExpr::AO__atomic_compare_exchange: return 863;
      case clang::AtomicExpr::AO__atomic_compare_exchange_n: return 864;
      case clang::AtomicExpr::AO__atomic_fetch_add: return 867;
      case clang::AtomicExpr::AO__atomic_fetch_sub: return 873;
      case clang::AtomicExpr::AO__atomic_fetch_and: return 868;
      case clang::AtomicExpr::AO__atomic_fetch_or: return 872;
      case clang::AtomicExpr::AO__atomic_fetch_xor: return 874;
      case clang::AtomicExpr::AO__atomic_fetch_nand: return 871;
      case clang::AtomicExpr::AO__atomic_add_fetch: return 861;
      case clang::AtomicExpr::AO__atomic_sub_fetch: return 883;
      case clang::AtomicExpr::AO__atomic_and_fetch: return 862;
      case clang::AtomicExpr::AO__atomic_or_fetch: return 880;
      case clang::AtomicExpr::AO__atomic_xor_fetch: return 884;
      case clang::AtomicExpr::AO__atomic_max_fetch: return 877;
      case clang::AtomicExpr::AO__atomic_min_fetch: return 878;
      case clang::AtomicExpr::AO__atomic_nand_fetch: return 879;
      case clang::AtomicExpr::AO__scoped_atomic_load: return 938;
      case clang::AtomicExpr::AO__scoped_atomic_load_n: return 939;
      case clang::AtomicExpr::AO__scoped_atomic_store: return 944;
      case clang::AtomicExpr::AO__scoped_atomic_store_n: return 945;
      case clang::AtomicExpr::AO__scoped_atomic_exchange: return 928;
      case clang::AtomicExpr::AO__scoped_atomic_exchange_n: return 929;
      case clang::AtomicExpr::AO__scoped_atomic_compare_exchange: return 926;
      case clang::AtomicExpr::AO__scoped_atomic_compare_exchange_n: return 927;
      case clang::AtomicExpr::AO__scoped_atomic_fetch_add: return 930;
      case clang::AtomicExpr::AO__scoped_atomic_fetch_sub: return 936;
      case clang::AtomicExpr::AO__scoped_atomic_fetch_and: return 931;
      case clang::AtomicExpr::AO__scoped_atomic_fetch_or: return 935;
      case clang::AtomicExpr::AO__scoped_atomic_fetch_xor: return 937;
      case clang::AtomicExpr::AO__scoped_atomic_fetch_nand: return 934;
      case clang::AtomicExpr::AO__scoped_atomic_add_fetch: return 924;
      case clang::AtomicExpr::AO__scoped_atomic_sub_fetch: return 946;
      case clang::AtomicExpr::AO__scoped_atomic_and_fetch: return 925;
      case clang::AtomicExpr::AO__scoped_atomic_or_fetch: return 943;
      case clang::AtomicExpr::AO__scoped_atomic_xor_fetch: return 947;
      case clang::AtomicExpr::AO__scoped_atomic_max_fetch: return 940;
      case clang::AtomicExpr::AO__scoped_atomic_min_fetch: return 941;
      case clang::AtomicExpr::AO__scoped_atomic_nand_fetch: return 942;
      case clang::AtomicExpr::AO__scoped_atomic_fetch_min: return 933;
      case clang::AtomicExpr::AO__scoped_atomic_fetch_max: return 932;
      case clang::AtomicExpr::AO__opencl_atomic_init: return 921;
      case clang::AtomicExpr::AO__opencl_atomic_load: return 922;
      case clang::AtomicExpr::AO__opencl_atomic_store: return 923;
      case clang::AtomicExpr::AO__opencl_atomic_exchange: return 913;
      case clang::AtomicExpr::AO__opencl_atomic_compare_exchange_strong: return 911;
      case clang::AtomicExpr::AO__opencl_atomic_compare_exchange_weak: return 912;
      case clang::AtomicExpr::AO__opencl_atomic_fetch_add: return 914;
      case clang::AtomicExpr::AO__opencl_atomic_fetch_sub: return 919;
      case clang::AtomicExpr::AO__opencl_atomic_fetch_and: return 915;
      case clang::AtomicExpr::AO__opencl_atomic_fetch_or: return 918;
      case clang::AtomicExpr::AO__opencl_atomic_fetch_xor: return 920;
      case clang::AtomicExpr::AO__opencl_atomic_fetch_min: return 917;
      case clang::AtomicExpr::AO__opencl_atomic_fetch_max: return 916;
      case clang::AtomicExpr::AO__atomic_fetch_min: return 870;
      case clang::AtomicExpr::AO__atomic_fetch_max: return 869;
      case clang::AtomicExpr::AO__hip_atomic_load: return 909;
      case clang::AtomicExpr::AO__hip_atomic_store: return 910;
      case clang::AtomicExpr::AO__hip_atomic_compare_exchange_weak: return 900;
      case clang::AtomicExpr::AO__hip_atomic_compare_exchange_strong: return 899;
      case clang::AtomicExpr::AO__hip_atomic_exchange: return 901;
      case clang::AtomicExpr::AO__hip_atomic_fetch_add: return 902;
      case clang::AtomicExpr::AO__hip_atomic_fetch_sub: return 907;
      case clang::AtomicExpr::AO__hip_atomic_fetch_and: return 903;
      case clang::AtomicExpr::AO__hip_atomic_fetch_or: return 906;
      case clang::AtomicExpr::AO__hip_atomic_fetch_xor: return 908;
      case clang::AtomicExpr::AO__hip_atomic_fetch_min: return 905;
      case clang::AtomicExpr::AO__hip_atomic_fetch_max: return 904;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::BuiltinType::Kind kind) {
    switch (kind) {
      case clang::BuiltinType::OCLImage1dRO: return 984;
      case clang::BuiltinType::OCLImage1dArrayRO: return 978;
      case clang::BuiltinType::OCLImage1dBufferRO: return 981;
      case clang::BuiltinType::OCLImage2dRO: return 1008;
      case clang::BuiltinType::OCLImage2dArrayRO: return 996;
      case clang::BuiltinType::OCLImage2dDepthRO: return 999;
      case clang::BuiltinType::OCLImage2dArrayDepthRO: return 987;
      case clang::BuiltinType::OCLImage2dMSAARO: return 1005;
      case clang::BuiltinType::OCLImage2dArrayMSAARO: return 993;
      case clang::BuiltinType::OCLImage2dMSAADepthRO: return 1002;
      case clang::BuiltinType::OCLImage2dArrayMSAADepthRO: return 990;
      case clang::BuiltinType::OCLImage3dRO: return 1011;
      case clang::BuiltinType::OCLImage1dWO: return 986;
      case clang::BuiltinType::OCLImage1dArrayWO: return 980;
      case clang::BuiltinType::OCLImage1dBufferWO: return 983;
      case clang::BuiltinType::OCLImage2dWO: return 1010;
      case clang::BuiltinType::OCLImage2dArrayWO: return 998;
      case clang::BuiltinType::OCLImage2dDepthWO: return 1001;
      case clang::BuiltinType::OCLImage2dArrayDepthWO: return 989;
      case clang::BuiltinType::OCLImage2dMSAAWO: return 1007;
      case clang::BuiltinType::OCLImage2dArrayMSAAWO: return 995;
      case clang::BuiltinType::OCLImage2dMSAADepthWO: return 1004;
      case clang::BuiltinType::OCLImage2dArrayMSAADepthWO: return 992;
      case clang::BuiltinType::OCLImage3dWO: return 1013;
      case clang::BuiltinType::OCLImage1dRW: return 985;
      case clang::BuiltinType::OCLImage1dArrayRW: return 979;
      case clang::BuiltinType::OCLImage1dBufferRW: return 982;
      case clang::BuiltinType::OCLImage2dRW: return 1009;
      case clang::BuiltinType::OCLImage2dArrayRW: return 997;
      case clang::BuiltinType::OCLImage2dDepthRW: return 1000;
      case clang::BuiltinType::OCLImage2dArrayDepthRW: return 988;
      case clang::BuiltinType::OCLImage2dMSAARW: return 1006;
      case clang::BuiltinType::OCLImage2dArrayMSAARW: return 994;
      case clang::BuiltinType::OCLImage2dMSAADepthRW: return 1003;
      case clang::BuiltinType::OCLImage2dArrayMSAADepthRW: return 991;
      case clang::BuiltinType::OCLImage3dRW: return 1012;
      case clang::BuiltinType::OCLIntelSubgroupAVCMcePayload: return 1020;
      case clang::BuiltinType::OCLIntelSubgroupAVCImePayload: return 1015;
      case clang::BuiltinType::OCLIntelSubgroupAVCRefPayload: return 1022;
      case clang::BuiltinType::OCLIntelSubgroupAVCSicPayload: return 1024;
      case clang::BuiltinType::OCLIntelSubgroupAVCMceResult: return 1021;
      case clang::BuiltinType::OCLIntelSubgroupAVCImeResult: return 1016;
      case clang::BuiltinType::OCLIntelSubgroupAVCRefResult: return 1023;
      case clang::BuiltinType::OCLIntelSubgroupAVCSicResult: return 1025;
      case clang::BuiltinType::OCLIntelSubgroupAVCImeResultSingleReferenceStreamout: return 1018;
      case clang::BuiltinType::OCLIntelSubgroupAVCImeResultDualReferenceStreamout: return 1017;
      case clang::BuiltinType::OCLIntelSubgroupAVCImeSingleReferenceStreamin: return 1019;
      case clang::BuiltinType::OCLIntelSubgroupAVCImeDualReferenceStreamin: return 1014;
      case clang::BuiltinType::SveInt8: return 1408;
      case clang::BuiltinType::SveInt16: return 1396;
      case clang::BuiltinType::SveInt32: return 1400;
      case clang::BuiltinType::SveInt64: return 1404;
      case clang::BuiltinType::SveUint8: return 1424;
      case clang::BuiltinType::SveUint16: return 1412;
      case clang::BuiltinType::SveUint32: return 1416;
      case clang::BuiltinType::SveUint64: return 1420;
      case clang::BuiltinType::SveFloat16: return 1384;
      case clang::BuiltinType::SveFloat32: return 1388;
      case clang::BuiltinType::SveFloat64: return 1392;
      case clang::BuiltinType::SveBFloat16: return 1376;
      case clang::BuiltinType::SveInt8x2: return 1409;
      case clang::BuiltinType::SveInt16x2: return 1397;
      case clang::BuiltinType::SveInt32x2: return 1401;
      case clang::BuiltinType::SveInt64x2: return 1405;
      case clang::BuiltinType::SveUint8x2: return 1425;
      case clang::BuiltinType::SveUint16x2: return 1413;
      case clang::BuiltinType::SveUint32x2: return 1417;
      case clang::BuiltinType::SveUint64x2: return 1421;
      case clang::BuiltinType::SveFloat16x2: return 1385;
      case clang::BuiltinType::SveFloat32x2: return 1389;
      case clang::BuiltinType::SveFloat64x2: return 1393;
      case clang::BuiltinType::SveBFloat16x2: return 1377;
      case clang::BuiltinType::SveInt8x3: return 1410;
      case clang::BuiltinType::SveInt16x3: return 1398;
      case clang::BuiltinType::SveInt32x3: return 1402;
      case clang::BuiltinType::SveInt64x3: return 1406;
      case clang::BuiltinType::SveUint8x3: return 1426;
      case clang::BuiltinType::SveUint16x3: return 1414;
      case clang::BuiltinType::SveUint32x3: return 1418;
      case clang::BuiltinType::SveUint64x3: return 1422;
      case clang::BuiltinType::SveFloat16x3: return 1386;
      case clang::BuiltinType::SveFloat32x3: return 1390;
      case clang::BuiltinType::SveFloat64x3: return 1394;
      case clang::BuiltinType::SveBFloat16x3: return 1378;
      case clang::BuiltinType::SveInt8x4: return 1411;
      case clang::BuiltinType::SveInt16x4: return 1399;
      case clang::BuiltinType::SveInt32x4: return 1403;
      case clang::BuiltinType::SveInt64x4: return 1407;
      case clang::BuiltinType::SveUint8x4: return 1427;
      case clang::BuiltinType::SveUint16x4: return 1415;
      case clang::BuiltinType::SveUint32x4: return 1419;
      case clang::BuiltinType::SveUint64x4: return 1423;
      case clang::BuiltinType::SveFloat16x4: return 1387;
      case clang::BuiltinType::SveFloat32x4: return 1391;
      case clang::BuiltinType::SveFloat64x4: return 1395;
      case clang::BuiltinType::SveBFloat16x4: return 1379;
      case clang::BuiltinType::SveBool: return 1380;
      case clang::BuiltinType::SveBoolx2: return 1381;
      case clang::BuiltinType::SveBoolx4: return 1382;
      case clang::BuiltinType::SveCount: return 1383;
      case clang::BuiltinType::VectorQuad: return 1442;
      case clang::BuiltinType::VectorPair: return 1441;
      case clang::BuiltinType::RvvInt8mf8: return 1244;
      case clang::BuiltinType::RvvInt8mf4: return 1236;
      case clang::BuiltinType::RvvInt8mf2: return 1228;
      case clang::BuiltinType::RvvInt8m1: return 1213;
      case clang::BuiltinType::RvvInt8m2: return 1221;
      case clang::BuiltinType::RvvInt8m4: return 1225;
      case clang::BuiltinType::RvvInt8m8: return 1227;
      case clang::BuiltinType::RvvUint8mf8: return 1352;
      case clang::BuiltinType::RvvUint8mf4: return 1344;
      case clang::BuiltinType::RvvUint8mf2: return 1336;
      case clang::BuiltinType::RvvUint8m1: return 1321;
      case clang::BuiltinType::RvvUint8m2: return 1329;
      case clang::BuiltinType::RvvUint8m4: return 1333;
      case clang::BuiltinType::RvvUint8m8: return 1335;
      case clang::BuiltinType::RvvInt16mf4: return 1167;
      case clang::BuiltinType::RvvInt16mf2: return 1159;
      case clang::BuiltinType::RvvInt16m1: return 1144;
      case clang::BuiltinType::RvvInt16m2: return 1152;
      case clang::BuiltinType::RvvInt16m4: return 1156;
      case clang::BuiltinType::RvvInt16m8: return 1158;
      case clang::BuiltinType::RvvUint16mf4: return 1275;
      case clang::BuiltinType::RvvUint16mf2: return 1267;
      case clang::BuiltinType::RvvUint16m1: return 1252;
      case clang::BuiltinType::RvvUint16m2: return 1260;
      case clang::BuiltinType::RvvUint16m4: return 1264;
      case clang::BuiltinType::RvvUint16m8: return 1266;
      case clang::BuiltinType::RvvInt32mf2: return 1190;
      case clang::BuiltinType::RvvInt32m1: return 1175;
      case clang::BuiltinType::RvvInt32m2: return 1183;
      case clang::BuiltinType::RvvInt32m4: return 1187;
      case clang::BuiltinType::RvvInt32m8: return 1189;
      case clang::BuiltinType::RvvUint32mf2: return 1298;
      case clang::BuiltinType::RvvUint32m1: return 1283;
      case clang::BuiltinType::RvvUint32m2: return 1291;
      case clang::BuiltinType::RvvUint32m4: return 1295;
      case clang::BuiltinType::RvvUint32m8: return 1297;
      case clang::BuiltinType::RvvInt64m1: return 1198;
      case clang::BuiltinType::RvvInt64m2: return 1206;
      case clang::BuiltinType::RvvInt64m4: return 1210;
      case clang::BuiltinType::RvvInt64m8: return 1212;
      case clang::BuiltinType::RvvUint64m1: return 1306;
      case clang::BuiltinType::RvvUint64m2: return 1314;
      case clang::BuiltinType::RvvUint64m4: return 1318;
      case clang::BuiltinType::RvvUint64m8: return 1320;
      case clang::BuiltinType::RvvFloat16mf4: return 1098;
      case clang::BuiltinType::RvvFloat16mf2: return 1090;
      case clang::BuiltinType::RvvFloat16m1: return 1075;
      case clang::BuiltinType::RvvFloat16m2: return 1083;
      case clang::BuiltinType::RvvFloat16m4: return 1087;
      case clang::BuiltinType::RvvFloat16m8: return 1089;
      case clang::BuiltinType::RvvBFloat16mf4: return 1060;
      case clang::BuiltinType::RvvBFloat16mf2: return 1052;
      case clang::BuiltinType::RvvBFloat16m1: return 1037;
      case clang::BuiltinType::RvvBFloat16m2: return 1045;
      case clang::BuiltinType::RvvBFloat16m4: return 1049;
      case clang::BuiltinType::RvvBFloat16m8: return 1051;
      case clang::BuiltinType::RvvFloat32mf2: return 1121;
      case clang::BuiltinType::RvvFloat32m1: return 1106;
      case clang::BuiltinType::RvvFloat32m2: return 1114;
      case clang::BuiltinType::RvvFloat32m4: return 1118;
      case clang::BuiltinType::RvvFloat32m8: return 1120;
      case clang::BuiltinType::RvvFloat64m1: return 1129;
      case clang::BuiltinType::RvvFloat64m2: return 1137;
      case clang::BuiltinType::RvvFloat64m4: return 1141;
      case clang::BuiltinType::RvvFloat64m8: return 1143;
      case clang::BuiltinType::RvvBool1: return 1068;
      case clang::BuiltinType::RvvBool2: return 1070;
      case clang::BuiltinType::RvvBool4: return 1072;
      case clang::BuiltinType::RvvBool8: return 1074;
      case clang::BuiltinType::RvvBool16: return 1069;
      case clang::BuiltinType::RvvBool32: return 1071;
      case clang::BuiltinType::RvvBool64: return 1073;
      case clang::BuiltinType::RvvInt8mf8x2: return 1245;
      case clang::BuiltinType::RvvInt8mf8x3: return 1246;
      case clang::BuiltinType::RvvInt8mf8x4: return 1247;
      case clang::BuiltinType::RvvInt8mf8x5: return 1248;
      case clang::BuiltinType::RvvInt8mf8x6: return 1249;
      case clang::BuiltinType::RvvInt8mf8x7: return 1250;
      case clang::BuiltinType::RvvInt8mf8x8: return 1251;
      case clang::BuiltinType::RvvInt8mf4x2: return 1237;
      case clang::BuiltinType::RvvInt8mf4x3: return 1238;
      case clang::BuiltinType::RvvInt8mf4x4: return 1239;
      case clang::BuiltinType::RvvInt8mf4x5: return 1240;
      case clang::BuiltinType::RvvInt8mf4x6: return 1241;
      case clang::BuiltinType::RvvInt8mf4x7: return 1242;
      case clang::BuiltinType::RvvInt8mf4x8: return 1243;
      case clang::BuiltinType::RvvInt8mf2x2: return 1229;
      case clang::BuiltinType::RvvInt8mf2x3: return 1230;
      case clang::BuiltinType::RvvInt8mf2x4: return 1231;
      case clang::BuiltinType::RvvInt8mf2x5: return 1232;
      case clang::BuiltinType::RvvInt8mf2x6: return 1233;
      case clang::BuiltinType::RvvInt8mf2x7: return 1234;
      case clang::BuiltinType::RvvInt8mf2x8: return 1235;
      case clang::BuiltinType::RvvInt8m1x2: return 1214;
      case clang::BuiltinType::RvvInt8m1x3: return 1215;
      case clang::BuiltinType::RvvInt8m1x4: return 1216;
      case clang::BuiltinType::RvvInt8m1x5: return 1217;
      case clang::BuiltinType::RvvInt8m1x6: return 1218;
      case clang::BuiltinType::RvvInt8m1x7: return 1219;
      case clang::BuiltinType::RvvInt8m1x8: return 1220;
      case clang::BuiltinType::RvvInt8m2x2: return 1222;
      case clang::BuiltinType::RvvInt8m2x3: return 1223;
      case clang::BuiltinType::RvvInt8m2x4: return 1224;
      case clang::BuiltinType::RvvInt8m4x2: return 1226;
      case clang::BuiltinType::RvvUint8mf8x2: return 1353;
      case clang::BuiltinType::RvvUint8mf8x3: return 1354;
      case clang::BuiltinType::RvvUint8mf8x4: return 1355;
      case clang::BuiltinType::RvvUint8mf8x5: return 1356;
      case clang::BuiltinType::RvvUint8mf8x6: return 1357;
      case clang::BuiltinType::RvvUint8mf8x7: return 1358;
      case clang::BuiltinType::RvvUint8mf8x8: return 1359;
      case clang::BuiltinType::RvvUint8mf4x2: return 1345;
      case clang::BuiltinType::RvvUint8mf4x3: return 1346;
      case clang::BuiltinType::RvvUint8mf4x4: return 1347;
      case clang::BuiltinType::RvvUint8mf4x5: return 1348;
      case clang::BuiltinType::RvvUint8mf4x6: return 1349;
      case clang::BuiltinType::RvvUint8mf4x7: return 1350;
      case clang::BuiltinType::RvvUint8mf4x8: return 1351;
      case clang::BuiltinType::RvvUint8mf2x2: return 1337;
      case clang::BuiltinType::RvvUint8mf2x3: return 1338;
      case clang::BuiltinType::RvvUint8mf2x4: return 1339;
      case clang::BuiltinType::RvvUint8mf2x5: return 1340;
      case clang::BuiltinType::RvvUint8mf2x6: return 1341;
      case clang::BuiltinType::RvvUint8mf2x7: return 1342;
      case clang::BuiltinType::RvvUint8mf2x8: return 1343;
      case clang::BuiltinType::RvvUint8m1x2: return 1322;
      case clang::BuiltinType::RvvUint8m1x3: return 1323;
      case clang::BuiltinType::RvvUint8m1x4: return 1324;
      case clang::BuiltinType::RvvUint8m1x5: return 1325;
      case clang::BuiltinType::RvvUint8m1x6: return 1326;
      case clang::BuiltinType::RvvUint8m1x7: return 1327;
      case clang::BuiltinType::RvvUint8m1x8: return 1328;
      case clang::BuiltinType::RvvUint8m2x2: return 1330;
      case clang::BuiltinType::RvvUint8m2x3: return 1331;
      case clang::BuiltinType::RvvUint8m2x4: return 1332;
      case clang::BuiltinType::RvvUint8m4x2: return 1334;
      case clang::BuiltinType::RvvInt16mf4x2: return 1168;
      case clang::BuiltinType::RvvInt16mf4x3: return 1169;
      case clang::BuiltinType::RvvInt16mf4x4: return 1170;
      case clang::BuiltinType::RvvInt16mf4x5: return 1171;
      case clang::BuiltinType::RvvInt16mf4x6: return 1172;
      case clang::BuiltinType::RvvInt16mf4x7: return 1173;
      case clang::BuiltinType::RvvInt16mf4x8: return 1174;
      case clang::BuiltinType::RvvInt16mf2x2: return 1160;
      case clang::BuiltinType::RvvInt16mf2x3: return 1161;
      case clang::BuiltinType::RvvInt16mf2x4: return 1162;
      case clang::BuiltinType::RvvInt16mf2x5: return 1163;
      case clang::BuiltinType::RvvInt16mf2x6: return 1164;
      case clang::BuiltinType::RvvInt16mf2x7: return 1165;
      case clang::BuiltinType::RvvInt16mf2x8: return 1166;
      case clang::BuiltinType::RvvInt16m1x2: return 1145;
      case clang::BuiltinType::RvvInt16m1x3: return 1146;
      case clang::BuiltinType::RvvInt16m1x4: return 1147;
      case clang::BuiltinType::RvvInt16m1x5: return 1148;
      case clang::BuiltinType::RvvInt16m1x6: return 1149;
      case clang::BuiltinType::RvvInt16m1x7: return 1150;
      case clang::BuiltinType::RvvInt16m1x8: return 1151;
      case clang::BuiltinType::RvvInt16m2x2: return 1153;
      case clang::BuiltinType::RvvInt16m2x3: return 1154;
      case clang::BuiltinType::RvvInt16m2x4: return 1155;
      case clang::BuiltinType::RvvInt16m4x2: return 1157;
      case clang::BuiltinType::RvvUint16mf4x2: return 1276;
      case clang::BuiltinType::RvvUint16mf4x3: return 1277;
      case clang::BuiltinType::RvvUint16mf4x4: return 1278;
      case clang::BuiltinType::RvvUint16mf4x5: return 1279;
      case clang::BuiltinType::RvvUint16mf4x6: return 1280;
      case clang::BuiltinType::RvvUint16mf4x7: return 1281;
      case clang::BuiltinType::RvvUint16mf4x8: return 1282;
      case clang::BuiltinType::RvvUint16mf2x2: return 1268;
      case clang::BuiltinType::RvvUint16mf2x3: return 1269;
      case clang::BuiltinType::RvvUint16mf2x4: return 1270;
      case clang::BuiltinType::RvvUint16mf2x5: return 1271;
      case clang::BuiltinType::RvvUint16mf2x6: return 1272;
      case clang::BuiltinType::RvvUint16mf2x7: return 1273;
      case clang::BuiltinType::RvvUint16mf2x8: return 1274;
      case clang::BuiltinType::RvvUint16m1x2: return 1253;
      case clang::BuiltinType::RvvUint16m1x3: return 1254;
      case clang::BuiltinType::RvvUint16m1x4: return 1255;
      case clang::BuiltinType::RvvUint16m1x5: return 1256;
      case clang::BuiltinType::RvvUint16m1x6: return 1257;
      case clang::BuiltinType::RvvUint16m1x7: return 1258;
      case clang::BuiltinType::RvvUint16m1x8: return 1259;
      case clang::BuiltinType::RvvUint16m2x2: return 1261;
      case clang::BuiltinType::RvvUint16m2x3: return 1262;
      case clang::BuiltinType::RvvUint16m2x4: return 1263;
      case clang::BuiltinType::RvvUint16m4x2: return 1265;
      case clang::BuiltinType::RvvInt32mf2x2: return 1191;
      case clang::BuiltinType::RvvInt32mf2x3: return 1192;
      case clang::BuiltinType::RvvInt32mf2x4: return 1193;
      case clang::BuiltinType::RvvInt32mf2x5: return 1194;
      case clang::BuiltinType::RvvInt32mf2x6: return 1195;
      case clang::BuiltinType::RvvInt32mf2x7: return 1196;
      case clang::BuiltinType::RvvInt32mf2x8: return 1197;
      case clang::BuiltinType::RvvInt32m1x2: return 1176;
      case clang::BuiltinType::RvvInt32m1x3: return 1177;
      case clang::BuiltinType::RvvInt32m1x4: return 1178;
      case clang::BuiltinType::RvvInt32m1x5: return 1179;
      case clang::BuiltinType::RvvInt32m1x6: return 1180;
      case clang::BuiltinType::RvvInt32m1x7: return 1181;
      case clang::BuiltinType::RvvInt32m1x8: return 1182;
      case clang::BuiltinType::RvvInt32m2x2: return 1184;
      case clang::BuiltinType::RvvInt32m2x3: return 1185;
      case clang::BuiltinType::RvvInt32m2x4: return 1186;
      case clang::BuiltinType::RvvInt32m4x2: return 1188;
      case clang::BuiltinType::RvvUint32mf2x2: return 1299;
      case clang::BuiltinType::RvvUint32mf2x3: return 1300;
      case clang::BuiltinType::RvvUint32mf2x4: return 1301;
      case clang::BuiltinType::RvvUint32mf2x5: return 1302;
      case clang::BuiltinType::RvvUint32mf2x6: return 1303;
      case clang::BuiltinType::RvvUint32mf2x7: return 1304;
      case clang::BuiltinType::RvvUint32mf2x8: return 1305;
      case clang::BuiltinType::RvvUint32m1x2: return 1284;
      case clang::BuiltinType::RvvUint32m1x3: return 1285;
      case clang::BuiltinType::RvvUint32m1x4: return 1286;
      case clang::BuiltinType::RvvUint32m1x5: return 1287;
      case clang::BuiltinType::RvvUint32m1x6: return 1288;
      case clang::BuiltinType::RvvUint32m1x7: return 1289;
      case clang::BuiltinType::RvvUint32m1x8: return 1290;
      case clang::BuiltinType::RvvUint32m2x2: return 1292;
      case clang::BuiltinType::RvvUint32m2x3: return 1293;
      case clang::BuiltinType::RvvUint32m2x4: return 1294;
      case clang::BuiltinType::RvvUint32m4x2: return 1296;
      case clang::BuiltinType::RvvInt64m1x2: return 1199;
      case clang::BuiltinType::RvvInt64m1x3: return 1200;
      case clang::BuiltinType::RvvInt64m1x4: return 1201;
      case clang::BuiltinType::RvvInt64m1x5: return 1202;
      case clang::BuiltinType::RvvInt64m1x6: return 1203;
      case clang::BuiltinType::RvvInt64m1x7: return 1204;
      case clang::BuiltinType::RvvInt64m1x8: return 1205;
      case clang::BuiltinType::RvvInt64m2x2: return 1207;
      case clang::BuiltinType::RvvInt64m2x3: return 1208;
      case clang::BuiltinType::RvvInt64m2x4: return 1209;
      case clang::BuiltinType::RvvInt64m4x2: return 1211;
      case clang::BuiltinType::RvvUint64m1x2: return 1307;
      case clang::BuiltinType::RvvUint64m1x3: return 1308;
      case clang::BuiltinType::RvvUint64m1x4: return 1309;
      case clang::BuiltinType::RvvUint64m1x5: return 1310;
      case clang::BuiltinType::RvvUint64m1x6: return 1311;
      case clang::BuiltinType::RvvUint64m1x7: return 1312;
      case clang::BuiltinType::RvvUint64m1x8: return 1313;
      case clang::BuiltinType::RvvUint64m2x2: return 1315;
      case clang::BuiltinType::RvvUint64m2x3: return 1316;
      case clang::BuiltinType::RvvUint64m2x4: return 1317;
      case clang::BuiltinType::RvvUint64m4x2: return 1319;
      case clang::BuiltinType::RvvFloat16mf4x2: return 1099;
      case clang::BuiltinType::RvvFloat16mf4x3: return 1100;
      case clang::BuiltinType::RvvFloat16mf4x4: return 1101;
      case clang::BuiltinType::RvvFloat16mf4x5: return 1102;
      case clang::BuiltinType::RvvFloat16mf4x6: return 1103;
      case clang::BuiltinType::RvvFloat16mf4x7: return 1104;
      case clang::BuiltinType::RvvFloat16mf4x8: return 1105;
      case clang::BuiltinType::RvvFloat16mf2x2: return 1091;
      case clang::BuiltinType::RvvFloat16mf2x3: return 1092;
      case clang::BuiltinType::RvvFloat16mf2x4: return 1093;
      case clang::BuiltinType::RvvFloat16mf2x5: return 1094;
      case clang::BuiltinType::RvvFloat16mf2x6: return 1095;
      case clang::BuiltinType::RvvFloat16mf2x7: return 1096;
      case clang::BuiltinType::RvvFloat16mf2x8: return 1097;
      case clang::BuiltinType::RvvFloat16m1x2: return 1076;
      case clang::BuiltinType::RvvFloat16m1x3: return 1077;
      case clang::BuiltinType::RvvFloat16m1x4: return 1078;
      case clang::BuiltinType::RvvFloat16m1x5: return 1079;
      case clang::BuiltinType::RvvFloat16m1x6: return 1080;
      case clang::BuiltinType::RvvFloat16m1x7: return 1081;
      case clang::BuiltinType::RvvFloat16m1x8: return 1082;
      case clang::BuiltinType::RvvFloat16m2x2: return 1084;
      case clang::BuiltinType::RvvFloat16m2x3: return 1085;
      case clang::BuiltinType::RvvFloat16m2x4: return 1086;
      case clang::BuiltinType::RvvFloat16m4x2: return 1088;
      case clang::BuiltinType::RvvFloat32mf2x2: return 1122;
      case clang::BuiltinType::RvvFloat32mf2x3: return 1123;
      case clang::BuiltinType::RvvFloat32mf2x4: return 1124;
      case clang::BuiltinType::RvvFloat32mf2x5: return 1125;
      case clang::BuiltinType::RvvFloat32mf2x6: return 1126;
      case clang::BuiltinType::RvvFloat32mf2x7: return 1127;
      case clang::BuiltinType::RvvFloat32mf2x8: return 1128;
      case clang::BuiltinType::RvvFloat32m1x2: return 1107;
      case clang::BuiltinType::RvvFloat32m1x3: return 1108;
      case clang::BuiltinType::RvvFloat32m1x4: return 1109;
      case clang::BuiltinType::RvvFloat32m1x5: return 1110;
      case clang::BuiltinType::RvvFloat32m1x6: return 1111;
      case clang::BuiltinType::RvvFloat32m1x7: return 1112;
      case clang::BuiltinType::RvvFloat32m1x8: return 1113;
      case clang::BuiltinType::RvvFloat32m2x2: return 1115;
      case clang::BuiltinType::RvvFloat32m2x3: return 1116;
      case clang::BuiltinType::RvvFloat32m2x4: return 1117;
      case clang::BuiltinType::RvvFloat32m4x2: return 1119;
      case clang::BuiltinType::RvvFloat64m1x2: return 1130;
      case clang::BuiltinType::RvvFloat64m1x3: return 1131;
      case clang::BuiltinType::RvvFloat64m1x4: return 1132;
      case clang::BuiltinType::RvvFloat64m1x5: return 1133;
      case clang::BuiltinType::RvvFloat64m1x6: return 1134;
      case clang::BuiltinType::RvvFloat64m1x7: return 1135;
      case clang::BuiltinType::RvvFloat64m1x8: return 1136;
      case clang::BuiltinType::RvvFloat64m2x2: return 1138;
      case clang::BuiltinType::RvvFloat64m2x3: return 1139;
      case clang::BuiltinType::RvvFloat64m2x4: return 1140;
      case clang::BuiltinType::RvvFloat64m4x2: return 1142;
      case clang::BuiltinType::RvvBFloat16mf4x2: return 1061;
      case clang::BuiltinType::RvvBFloat16mf4x3: return 1062;
      case clang::BuiltinType::RvvBFloat16mf4x4: return 1063;
      case clang::BuiltinType::RvvBFloat16mf4x5: return 1064;
      case clang::BuiltinType::RvvBFloat16mf4x6: return 1065;
      case clang::BuiltinType::RvvBFloat16mf4x7: return 1066;
      case clang::BuiltinType::RvvBFloat16mf4x8: return 1067;
      case clang::BuiltinType::RvvBFloat16mf2x2: return 1053;
      case clang::BuiltinType::RvvBFloat16mf2x3: return 1054;
      case clang::BuiltinType::RvvBFloat16mf2x4: return 1055;
      case clang::BuiltinType::RvvBFloat16mf2x5: return 1056;
      case clang::BuiltinType::RvvBFloat16mf2x6: return 1057;
      case clang::BuiltinType::RvvBFloat16mf2x7: return 1058;
      case clang::BuiltinType::RvvBFloat16mf2x8: return 1059;
      case clang::BuiltinType::RvvBFloat16m1x2: return 1038;
      case clang::BuiltinType::RvvBFloat16m1x3: return 1039;
      case clang::BuiltinType::RvvBFloat16m1x4: return 1040;
      case clang::BuiltinType::RvvBFloat16m1x5: return 1041;
      case clang::BuiltinType::RvvBFloat16m1x6: return 1042;
      case clang::BuiltinType::RvvBFloat16m1x7: return 1043;
      case clang::BuiltinType::RvvBFloat16m1x8: return 1044;
      case clang::BuiltinType::RvvBFloat16m2x2: return 1046;
      case clang::BuiltinType::RvvBFloat16m2x3: return 1047;
      case clang::BuiltinType::RvvBFloat16m2x4: return 1048;
      case clang::BuiltinType::RvvBFloat16m4x2: return 1050;
      case clang::BuiltinType::WasmExternRef: return 1446;
      case clang::BuiltinType::Void: return 1443;
      case clang::BuiltinType::Bool: return 951;
      case clang::BuiltinType::Char_U: return 958;
      case clang::BuiltinType::UChar: return 1429;
      case clang::BuiltinType::WChar_U: return 1445;
      case clang::BuiltinType::Char8: return 956;
      case clang::BuiltinType::Char16: return 954;
      case clang::BuiltinType::Char32: return 955;
      case clang::BuiltinType::UShort: return 1437;
      case clang::BuiltinType::UInt: return 1431;
      case clang::BuiltinType::ULong: return 1433;
      case clang::BuiltinType::ULongLong: return 1436;
      case clang::BuiltinType::UInt128: return 1432;
      case clang::BuiltinType::Char_S: return 957;
      case clang::BuiltinType::SChar: return 1360;
      case clang::BuiltinType::WChar_S: return 1444;
      case clang::BuiltinType::Short: return 1373;
      case clang::BuiltinType::Int: return 968;
      case clang::BuiltinType::Long: return 970;
      case clang::BuiltinType::LongLong: return 974;
      case clang::BuiltinType::Int128: return 969;
      case clang::BuiltinType::ShortAccum: return 1374;
      case clang::BuiltinType::Accum: return 949;
      case clang::BuiltinType::LongAccum: return 971;
      case clang::BuiltinType::UShortAccum: return 1438;
      case clang::BuiltinType::UAccum: return 1428;
      case clang::BuiltinType::ULongAccum: return 1434;
      case clang::BuiltinType::ShortFract: return 1375;
      case clang::BuiltinType::Fract: return 964;
      case clang::BuiltinType::LongFract: return 973;
      case clang::BuiltinType::UShortFract: return 1439;
      case clang::BuiltinType::UFract: return 1430;
      case clang::BuiltinType::ULongFract: return 1435;
      case clang::BuiltinType::SatShortAccum: return 1365;
      case clang::BuiltinType::SatAccum: return 1361;
      case clang::BuiltinType::SatLongAccum: return 1363;
      case clang::BuiltinType::SatUShortAccum: return 1371;
      case clang::BuiltinType::SatUAccum: return 1367;
      case clang::BuiltinType::SatULongAccum: return 1369;
      case clang::BuiltinType::SatShortFract: return 1366;
      case clang::BuiltinType::SatFract: return 1362;
      case clang::BuiltinType::SatLongFract: return 1364;
      case clang::BuiltinType::SatUShortFract: return 1372;
      case clang::BuiltinType::SatUFract: return 1368;
      case clang::BuiltinType::SatULongFract: return 1370;
      case clang::BuiltinType::Half: return 965;
      case clang::BuiltinType::Float: return 961;
      case clang::BuiltinType::Double: return 960;
      case clang::BuiltinType::LongDouble: return 972;
      case clang::BuiltinType::Float16: return 963;
      case clang::BuiltinType::BFloat16: return 950;
      case clang::BuiltinType::Float128: return 962;
      case clang::BuiltinType::Ibm128: return 966;
      case clang::BuiltinType::NullPtr: return 975;
      case clang::BuiltinType::ObjCId: return 1033;
      case clang::BuiltinType::ObjCClass: return 1032;
      case clang::BuiltinType::ObjCSel: return 1034;
      case clang::BuiltinType::OCLSampler: return 1028;
      case clang::BuiltinType::OCLEvent: return 977;
      case clang::BuiltinType::OCLClkEvent: return 976;
      case clang::BuiltinType::OCLQueue: return 1026;
      case clang::BuiltinType::OCLReserveID: return 1027;
      case clang::BuiltinType::Dependent: return 959;
      case clang::BuiltinType::Overload: return 1035;
      case clang::BuiltinType::BoundMember: return 952;
      case clang::BuiltinType::PseudoObject: return 1036;
      case clang::BuiltinType::UnknownAny: return 1440;
      case clang::BuiltinType::BuiltinFn: return 953;
      case clang::BuiltinType::ARCUnbridgedCast: return 948;
      case clang::BuiltinType::IncompleteMatrixIdx: return 967;
      case clang::BuiltinType::OMPArraySection: return 1029;
      case clang::BuiltinType::OMPArrayShaping: return 1030;
      case clang::BuiltinType::OMPIterator: return 1031;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::CFGElement::Kind kind) {
    switch (kind) {
      case clang::CFGElement::Initializer: return 1453;
      case clang::CFGElement::ScopeBegin: return 1458;
      case clang::CFGElement::ScopeEnd: return 1459;
      case clang::CFGElement::NewAllocator: return 1457;
      case clang::CFGElement::LifetimeEnds: return 1454;
      case clang::CFGElement::LoopExit: return 1455;
      case clang::CFGElement::Statement: return 1460;
      case clang::CFGElement::Constructor: return 1451;
      case clang::CFGElement::CXXRecordTypedCall: return 1449;
      case clang::CFGElement::AutomaticObjectDtor: return 1447;
      case clang::CFGElement::DeleteDtor: return 1452;
      case clang::CFGElement::BaseDtor: return 1448;
      case clang::CFGElement::MemberDtor: return 1456;
      case clang::CFGElement::TemporaryDtor: return 1461;
      case clang::CFGElement::CleanupFunction: return 1450;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::CFGTerminator::Kind kind) {
    switch (kind) {
      case clang::CFGTerminator::StmtBranch: return 1462;
      case clang::CFGTerminator::TemporaryDtorsBranch: return 1463;
      case clang::CFGTerminator::VirtualBaseBranch: return 1464;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::CallExpr::ADLCallKind kind) {
    switch (kind) {
      case clang::CallExpr::ADLCallKind::NotADL: return 1465;
      case clang::CallExpr::ADLCallKind::UsesADL: return 1466;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::Decl::FriendObjectKind kind) {
    switch (kind) {
      case clang::Decl::FOK_None: return 1468;
      case clang::Decl::FOK_Declared: return 1467;
      case clang::Decl::FOK_Undeclared: return 1469;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::Decl::ModuleOwnershipKind kind) {
    switch (kind) {
      case clang::Decl::ModuleOwnershipKind::Unowned: return 1472;
      case clang::Decl::ModuleOwnershipKind::Visible: return 1473;
      case clang::Decl::ModuleOwnershipKind::VisibleWhenImported: return 1474;
      case clang::Decl::ModuleOwnershipKind::ReachableWhenImported: return 1471;
      case clang::Decl::ModuleOwnershipKind::ModulePrivate: return 1470;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::Decl::ObjCDeclQualifier kind) {
    switch (kind) {
      case clang::Decl::OBJC_TQ_None: return 1480;
      case clang::Decl::OBJC_TQ_In: return 1478;
      case clang::Decl::OBJC_TQ_Inout: return 1479;
      case clang::Decl::OBJC_TQ_Out: return 1482;
      case clang::Decl::OBJC_TQ_Bycopy: return 1475;
      case clang::Decl::OBJC_TQ_Byref: return 1476;
      case clang::Decl::OBJC_TQ_Oneway: return 1481;
      case clang::Decl::OBJC_TQ_CSNullability: return 1477;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::ExprDependenceScope::ExprDependence kind) {
    switch (kind) {
      case clang::ExprDependenceScope::UnexpandedPack: return 1492;
      case clang::ExprDependenceScope::Instantiation: return 1486;
      case clang::ExprDependenceScope::Type: return 1488;
      case clang::ExprDependenceScope::Value: return 1493;
      case clang::ExprDependenceScope::Error: return 1484;
      case clang::ExprDependenceScope::None: return 1487;
      case clang::ExprDependenceScope::All: return 1483;
      case clang::ExprDependenceScope::TypeValue: return 1490;
      case clang::ExprDependenceScope::TypeInstantiation: return 1489;
      case clang::ExprDependenceScope::ValueInstantiation: return 1494;
      case clang::ExprDependenceScope::TypeValueInstantiation: return 1491;
      case clang::ExprDependenceScope::ErrorDependent: return 1485;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::FunctionDecl::TemplatedKind kind) {
    switch (kind) {
      case clang::FunctionDecl::TK_NonTemplate: return 1500;
      case clang::FunctionDecl::TK_FunctionTemplate: return 1497;
      case clang::FunctionDecl::TK_MemberSpecialization: return 1499;
      case clang::FunctionDecl::TK_FunctionTemplateSpecialization: return 1498;
      case clang::FunctionDecl::TK_DependentFunctionTemplateSpecialization: return 1495;
      case clang::FunctionDecl::TK_DependentNonTemplate: return 1496;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::Qualifiers::ObjCLifetime kind) {
    switch (kind) {
      case clang::Qualifiers::OCL_None: return 1503;
      case clang::Qualifiers::OCL_ExplicitNone: return 1502;
      case clang::Qualifiers::OCL_Strong: return 1504;
      case clang::Qualifiers::OCL_Weak: return 1505;
      case clang::Qualifiers::OCL_Autoreleasing: return 1501;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::TypeDependenceScope::TypeDependence kind) {
    switch (kind) {
      case clang::TypeDependenceScope::UnexpandedPack: return 1512;
      case clang::TypeDependenceScope::Instantiation: return 1510;
      case clang::TypeDependenceScope::Dependent: return 1507;
      case clang::TypeDependenceScope::VariablyModified: return 1513;
      case clang::TypeDependenceScope::Error: return 1509;
      case clang::TypeDependenceScope::None: return 1511;
      case clang::TypeDependenceScope::All: return 1506;
      case clang::TypeDependenceScope::DependentInstantiation: return 1508;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::UnaryTransformType::UTTKind kind) {
    switch (kind) {
      case clang::UnaryTransformType::AddLvalueReference: return 1514;
      case clang::UnaryTransformType::AddPointer: return 1515;
      case clang::UnaryTransformType::AddRvalueReference: return 1516;
      case clang::UnaryTransformType::Decay: return 1517;
      case clang::UnaryTransformType::MakeSigned: return 1519;
      case clang::UnaryTransformType::MakeUnsigned: return 1520;
      case clang::UnaryTransformType::RemoveAllExtents: return 1521;
      case clang::UnaryTransformType::RemoveConst: return 1524;
      case clang::UnaryTransformType::RemoveCV: return 1522;
      case clang::UnaryTransformType::RemoveCVRef: return 1523;
      case clang::UnaryTransformType::RemoveExtent: return 1525;
      case clang::UnaryTransformType::RemovePointer: return 1526;
      case clang::UnaryTransformType::RemoveReference: return 1527;
      case clang::UnaryTransformType::RemoveRestrict: return 1528;
      case clang::UnaryTransformType::RemoveVolatile: return 1529;
      case clang::UnaryTransformType::EnumUnderlyingType: return 1518;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::UserDefinedLiteral::LiteralOperatorKind kind) {
    switch (kind) {
      case clang::UserDefinedLiteral::LOK_Raw: return 1533;
      case clang::UserDefinedLiteral::LOK_Template: return 1535;
      case clang::UserDefinedLiteral::LOK_Integer: return 1532;
      case clang::UserDefinedLiteral::LOK_Floating: return 1531;
      case clang::UserDefinedLiteral::LOK_String: return 1534;
      case clang::UserDefinedLiteral::LOK_Character: return 1530;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::VarDecl::DefinitionKind kind) {
    switch (kind) {
      case clang::VarDecl::DeclarationOnly: return 1536;
      case clang::VarDecl::TentativeDefinition: return 1538;
      case clang::VarDecl::Definition: return 1537;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::VarDecl::InitializationStyle kind) {
    switch (kind) {
      case clang::VarDecl::CInit: return 1539;
      case clang::VarDecl::CallInit: return 1540;
      case clang::VarDecl::ListInit: return 1541;
      case clang::VarDecl::ParenListInit: return 1542;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(clang::VarDecl::TLSKind kind) {
    switch (kind) {
      case clang::VarDecl::TLS_None: return 1544;
      case clang::VarDecl::TLS_Static: return 1545;
      case clang::VarDecl::TLS_Dynamic: return 1543;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
  uint64_t resolve(llvm::APFloatBase::Semantics kind) {
    switch (kind) {
      case llvm::APFloatBase::S_IEEEhalf: return 1554;
      case llvm::APFloatBase::S_BFloat: return 1546;
      case llvm::APFloatBase::S_IEEEsingle: return 1556;
      case llvm::APFloatBase::S_IEEEdouble: return 1553;
      case llvm::APFloatBase::S_IEEEquad: return 1555;
      case llvm::APFloatBase::S_PPCDoubleDouble: return 1557;
      case llvm::APFloatBase::S_Float8E5M2: return 1550;
      case llvm::APFloatBase::S_Float8E5M2FNUZ: return 1551;
      case llvm::APFloatBase::S_Float8E4M3FN: return 1548;
      case llvm::APFloatBase::S_Float8E4M3FNUZ: return 1549;
      case llvm::APFloatBase::S_Float8E4M3B11FNUZ: return 1547;
      case llvm::APFloatBase::S_FloatTF32: return 1552;
      case llvm::APFloatBase::S_x87DoubleExtended: return 1558;
      default: break;
    }
  assert(false && "Invalid enum kind");
  return 0;
  }
////   END ARBORETUM GENERATED CODE ////
}  // namespace arboretum
