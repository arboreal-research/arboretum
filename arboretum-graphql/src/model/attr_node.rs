use juniper::*;

pub mod inheritable_attr_node;
pub mod stmt_attr_node;
pub mod type_attr_node;

#[derive(GraphQLObject)]
pub struct Attr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AbiTagAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AliasAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AlignValueAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct BuiltinAliasAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CalledOnceAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct IFuncAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct InitSegAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct LoaderUninitializedAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct LoopHintAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ModeAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NoBuiltinAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NoEscapeAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCBoxableAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCClassStubAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCDesignatedInitializerAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCDirectAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCDirectMembersAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCNonLazyClassAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCNonRuntimeProtocolAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCRuntimeNameAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCRuntimeVisibleAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct OpenCLAccessAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct OverloadableAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct RenderScriptKernelAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SwiftObjCMembersAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SwiftVersionedAdditionAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SwiftVersionedRemovalAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ThreadAttr {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum AttrNode {
  Attr(Attr),
  AArch64SVEPcsAttr(inheritable_attr_node::AArch64SVEPcsAttr),
  AArch64VectorPcsAttr(inheritable_attr_node::AArch64VectorPcsAttr),
  AMDGPUFlatWorkGroupSizeAttr(inheritable_attr_node::AMDGPUFlatWorkGroupSizeAttr),
  AMDGPUKernelCallAttr(inheritable_attr_node::AMDGPUKernelCallAttr),
  AMDGPUNumSGPRAttr(inheritable_attr_node::AMDGPUNumSGPRAttr),
  AMDGPUNumVGPRAttr(inheritable_attr_node::AMDGPUNumVGPRAttr),
  AMDGPUWavesPerEUAttr(inheritable_attr_node::AMDGPUWavesPerEUAttr),
  ARMInterruptAttr(inheritable_attr_node::ARMInterruptAttr),
  AVRInterruptAttr(inheritable_attr_node::AVRInterruptAttr),
  AVRSignalAttr(inheritable_attr_node::AVRSignalAttr),
  AbiTagAttr(AbiTagAttr),
  AcquireCapabilityAttr(inheritable_attr_node::AcquireCapabilityAttr),
  AcquireHandleAttr(inheritable_attr_node::AcquireHandleAttr),
  AcquiredAfterAttr(inheritable_attr_node::AcquiredAfterAttr),
  AcquiredBeforeAttr(inheritable_attr_node::AcquiredBeforeAttr),
  AddressSpaceAttr(type_attr_node::AddressSpaceAttr),
  AliasAttr(AliasAttr),
  AlignMac68kAttr(inheritable_attr_node::AlignMac68kAttr),
  AlignNaturalAttr(inheritable_attr_node::AlignNaturalAttr),
  AlignValueAttr(AlignValueAttr),
  AlignedAttr(inheritable_attr_node::AlignedAttr),
  AllocAlignAttr(inheritable_attr_node::AllocAlignAttr),
  AllocSizeAttr(inheritable_attr_node::AllocSizeAttr),
  AlwaysDestroyAttr(inheritable_attr_node::AlwaysDestroyAttr),
  AlwaysInlineAttr(inheritable_attr_node::decl_or_stmt_attr_node::AlwaysInlineAttr),
  AnalyzerNoReturnAttr(inheritable_attr_node::AnalyzerNoReturnAttr),
  AnnotateAttr(inheritable_attr_node::inheritable_param_attr_node::AnnotateAttr),
  AnnotateTypeAttr(type_attr_node::AnnotateTypeAttr),
  AnyX86InterruptAttr(inheritable_attr_node::AnyX86InterruptAttr),
  AnyX86NoCallerSavedRegistersAttr(inheritable_attr_node::AnyX86NoCallerSavedRegistersAttr),
  AnyX86NoCfCheckAttr(inheritable_attr_node::AnyX86NoCfCheckAttr),
  ArcWeakrefUnavailableAttr(inheritable_attr_node::ArcWeakrefUnavailableAttr),
  ArgumentWithTypeTagAttr(inheritable_attr_node::ArgumentWithTypeTagAttr),
  ArmBuiltinAliasAttr(inheritable_attr_node::ArmBuiltinAliasAttr),
  ArmInAttr(type_attr_node::ArmInAttr),
  ArmInOutAttr(type_attr_node::ArmInOutAttr),
  ArmLocallyStreamingAttr(inheritable_attr_node::ArmLocallyStreamingAttr),
  ArmMveStrictPolymorphismAttr(type_attr_node::ArmMveStrictPolymorphismAttr),
  ArmNewAttr(inheritable_attr_node::ArmNewAttr),
  ArmOutAttr(type_attr_node::ArmOutAttr),
  ArmPreservesAttr(type_attr_node::ArmPreservesAttr),
  ArmStreamingAttr(type_attr_node::ArmStreamingAttr),
  ArmStreamingCompatibleAttr(type_attr_node::ArmStreamingCompatibleAttr),
  ArtificialAttr(inheritable_attr_node::ArtificialAttr),
  AsmLabelAttr(inheritable_attr_node::AsmLabelAttr),
  AssertCapabilityAttr(inheritable_attr_node::AssertCapabilityAttr),
  AssertExclusiveLockAttr(inheritable_attr_node::AssertExclusiveLockAttr),
  AssertSharedLockAttr(inheritable_attr_node::AssertSharedLockAttr),
  AssumeAlignedAttr(inheritable_attr_node::AssumeAlignedAttr),
  AssumptionAttr(inheritable_attr_node::AssumptionAttr),
  AvailabilityAttr(inheritable_attr_node::AvailabilityAttr),
  AvailableOnlyInDefaultEvalMethodAttr(inheritable_attr_node::AvailableOnlyInDefaultEvalMethodAttr),
  BPFPreserveAccessIndexAttr(inheritable_attr_node::BPFPreserveAccessIndexAttr),
  BPFPreserveStaticOffsetAttr(inheritable_attr_node::BPFPreserveStaticOffsetAttr),
  BTFDeclTagAttr(inheritable_attr_node::BTFDeclTagAttr),
  BTFTypeTagAttr(type_attr_node::BTFTypeTagAttr),
  BlocksAttr(inheritable_attr_node::BlocksAttr),
  BuiltinAliasAttr(BuiltinAliasAttr),
  BuiltinAttr(inheritable_attr_node::BuiltinAttr),
  C11NoReturnAttr(inheritable_attr_node::C11NoReturnAttr),
  CDeclAttr(inheritable_attr_node::CDeclAttr),
  CFAuditedTransferAttr(inheritable_attr_node::CFAuditedTransferAttr),
  CFConsumedAttr(inheritable_attr_node::inheritable_param_attr_node::CFConsumedAttr),
  CFGuardAttr(inheritable_attr_node::CFGuardAttr),
  CFICanonicalJumpTableAttr(inheritable_attr_node::CFICanonicalJumpTableAttr),
  CFReturnsNotRetainedAttr(inheritable_attr_node::CFReturnsNotRetainedAttr),
  CFReturnsRetainedAttr(inheritable_attr_node::CFReturnsRetainedAttr),
  CFUnknownTransferAttr(inheritable_attr_node::CFUnknownTransferAttr),
  CPUDispatchAttr(inheritable_attr_node::CPUDispatchAttr),
  CPUSpecificAttr(inheritable_attr_node::CPUSpecificAttr),
  CUDAConstantAttr(inheritable_attr_node::CUDAConstantAttr),
  CUDADeviceAttr(inheritable_attr_node::CUDADeviceAttr),
  CUDADeviceBuiltinSurfaceTypeAttr(inheritable_attr_node::CUDADeviceBuiltinSurfaceTypeAttr),
  CUDADeviceBuiltinTextureTypeAttr(inheritable_attr_node::CUDADeviceBuiltinTextureTypeAttr),
  CUDAGlobalAttr(inheritable_attr_node::CUDAGlobalAttr),
  CUDAHostAttr(inheritable_attr_node::CUDAHostAttr),
  CUDAInvalidTargetAttr(inheritable_attr_node::CUDAInvalidTargetAttr),
  CUDALaunchBoundsAttr(inheritable_attr_node::CUDALaunchBoundsAttr),
  CUDASharedAttr(inheritable_attr_node::CUDASharedAttr),
  CXX11NoReturnAttr(inheritable_attr_node::CXX11NoReturnAttr),
  CallableWhenAttr(inheritable_attr_node::CallableWhenAttr),
  CallbackAttr(inheritable_attr_node::CallbackAttr),
  CalledOnceAttr(CalledOnceAttr),
  CapabilityAttr(inheritable_attr_node::CapabilityAttr),
  CapturedRecordAttr(inheritable_attr_node::CapturedRecordAttr),
  CarriesDependencyAttr(inheritable_attr_node::inheritable_param_attr_node::CarriesDependencyAttr),
  CleanupAttr(inheritable_attr_node::CleanupAttr),
  CmseNSCallAttr(type_attr_node::CmseNSCallAttr),
  CmseNSEntryAttr(inheritable_attr_node::CmseNSEntryAttr),
  CodeAlignAttr(stmt_attr_node::CodeAlignAttr),
  CodeModelAttr(inheritable_attr_node::CodeModelAttr),
  CodeSegAttr(inheritable_attr_node::CodeSegAttr),
  ColdAttr(inheritable_attr_node::ColdAttr),
  CommonAttr(inheritable_attr_node::CommonAttr),
  ConstAttr(inheritable_attr_node::ConstAttr),
  ConstInitAttr(inheritable_attr_node::ConstInitAttr),
  ConstructorAttr(inheritable_attr_node::ConstructorAttr),
  ConsumableAttr(inheritable_attr_node::ConsumableAttr),
  ConsumableAutoCastAttr(inheritable_attr_node::ConsumableAutoCastAttr),
  ConsumableSetOnReadAttr(inheritable_attr_node::ConsumableSetOnReadAttr),
  ConvergentAttr(inheritable_attr_node::ConvergentAttr),
  CoroDisableLifetimeBoundAttr(inheritable_attr_node::CoroDisableLifetimeBoundAttr),
  CoroLifetimeBoundAttr(inheritable_attr_node::CoroLifetimeBoundAttr),
  CoroOnlyDestroyWhenCompleteAttr(inheritable_attr_node::CoroOnlyDestroyWhenCompleteAttr),
  CoroReturnTypeAttr(inheritable_attr_node::CoroReturnTypeAttr),
  CoroWrapperAttr(inheritable_attr_node::CoroWrapperAttr),
  CountedByAttr(inheritable_attr_node::CountedByAttr),
  DLLExportAttr(inheritable_attr_node::DLLExportAttr),
  DLLExportStaticLocalAttr(inheritable_attr_node::DLLExportStaticLocalAttr),
  DLLImportAttr(inheritable_attr_node::DLLImportAttr),
  DLLImportStaticLocalAttr(inheritable_attr_node::DLLImportStaticLocalAttr),
  DeclOrStmtAttr(inheritable_attr_node::decl_or_stmt_attr_node::DeclOrStmtAttr),
  DeprecatedAttr(inheritable_attr_node::DeprecatedAttr),
  DestructorAttr(inheritable_attr_node::DestructorAttr),
  DiagnoseAsBuiltinAttr(inheritable_attr_node::DiagnoseAsBuiltinAttr),
  DiagnoseIfAttr(inheritable_attr_node::DiagnoseIfAttr),
  DisableSanitizerInstrumentationAttr(inheritable_attr_node::DisableSanitizerInstrumentationAttr),
  DisableTailCallsAttr(inheritable_attr_node::DisableTailCallsAttr),
  EmptyBasesAttr(inheritable_attr_node::EmptyBasesAttr),
  EnableIfAttr(inheritable_attr_node::EnableIfAttr),
  EnforceTCBAttr(inheritable_attr_node::EnforceTCBAttr),
  EnforceTCBLeafAttr(inheritable_attr_node::EnforceTCBLeafAttr),
  EnumExtensibilityAttr(inheritable_attr_node::EnumExtensibilityAttr),
  ErrorAttr(inheritable_attr_node::ErrorAttr),
  ExcludeFromExplicitInstantiationAttr(inheritable_attr_node::ExcludeFromExplicitInstantiationAttr),
  ExclusiveTrylockFunctionAttr(inheritable_attr_node::ExclusiveTrylockFunctionAttr),
  ExternalSourceSymbolAttr(inheritable_attr_node::ExternalSourceSymbolAttr),
  FallThroughAttr(stmt_attr_node::FallThroughAttr),
  FastCallAttr(inheritable_attr_node::FastCallAttr),
  FinalAttr(inheritable_attr_node::FinalAttr),
  FlagEnumAttr(inheritable_attr_node::FlagEnumAttr),
  FlattenAttr(inheritable_attr_node::FlattenAttr),
  FormatArgAttr(inheritable_attr_node::FormatArgAttr),
  FormatAttr(inheritable_attr_node::FormatAttr),
  FunctionReturnThunksAttr(inheritable_attr_node::FunctionReturnThunksAttr),
  GNUInlineAttr(inheritable_attr_node::GNUInlineAttr),
  GuardedByAttr(inheritable_attr_node::GuardedByAttr),
  GuardedVarAttr(inheritable_attr_node::GuardedVarAttr),
  HIPManagedAttr(inheritable_attr_node::HIPManagedAttr),
  HLSLAnnotationAttr(inheritable_attr_node::hlsl_annotation_attr_node::HLSLAnnotationAttr),
  HLSLGroupSharedAddressSpaceAttr(type_attr_node::HLSLGroupSharedAddressSpaceAttr),
  HLSLNumThreadsAttr(inheritable_attr_node::HLSLNumThreadsAttr),
  HLSLParamModifierAttr(type_attr_node::HLSLParamModifierAttr),
  HLSLResourceAttr(inheritable_attr_node::HLSLResourceAttr),
  HLSLResourceBindingAttr(inheritable_attr_node::HLSLResourceBindingAttr),
  HLSLSVDispatchThreadIDAttr(inheritable_attr_node::hlsl_annotation_attr_node::HLSLSVDispatchThreadIDAttr),
  HLSLSVGroupIndexAttr(inheritable_attr_node::hlsl_annotation_attr_node::HLSLSVGroupIndexAttr),
  HLSLShaderAttr(inheritable_attr_node::HLSLShaderAttr),
  HotAttr(inheritable_attr_node::HotAttr),
  IBActionAttr(inheritable_attr_node::IBActionAttr),
  IBOutletAttr(inheritable_attr_node::IBOutletAttr),
  IBOutletCollectionAttr(inheritable_attr_node::IBOutletCollectionAttr),
  IFuncAttr(IFuncAttr),
  InheritableAttr(inheritable_attr_node::InheritableAttr),
  InheritableParamAttr(inheritable_attr_node::inheritable_param_attr_node::InheritableParamAttr),
  InitPriorityAttr(inheritable_attr_node::InitPriorityAttr),
  InitSegAttr(InitSegAttr),
  IntelOclBiccAttr(inheritable_attr_node::IntelOclBiccAttr),
  InternalLinkageAttr(inheritable_attr_node::InternalLinkageAttr),
  LTOVisibilityPublicAttr(inheritable_attr_node::LTOVisibilityPublicAttr),
  LayoutVersionAttr(inheritable_attr_node::LayoutVersionAttr),
  LeafAttr(inheritable_attr_node::LeafAttr),
  LifetimeBoundAttr(inheritable_attr_node::LifetimeBoundAttr),
  LikelyAttr(stmt_attr_node::LikelyAttr),
  LoaderUninitializedAttr(LoaderUninitializedAttr),
  LockReturnedAttr(inheritable_attr_node::LockReturnedAttr),
  LocksExcludedAttr(inheritable_attr_node::LocksExcludedAttr),
  LoopHintAttr(LoopHintAttr),
  M68kInterruptAttr(inheritable_attr_node::M68kInterruptAttr),
  M68kRTDAttr(inheritable_attr_node::M68kRTDAttr),
  MIGServerRoutineAttr(inheritable_attr_node::MIGServerRoutineAttr),
  MSABIAttr(inheritable_attr_node::MSABIAttr),
  MSAllocatorAttr(inheritable_attr_node::MSAllocatorAttr),
  MSConstexprAttr(inheritable_attr_node::MSConstexprAttr),
  MSInheritanceAttr(inheritable_attr_node::MSInheritanceAttr),
  MSNoVTableAttr(inheritable_attr_node::MSNoVTableAttr),
  MSP430InterruptAttr(inheritable_attr_node::MSP430InterruptAttr),
  MSStructAttr(inheritable_attr_node::MSStructAttr),
  MSVtorDispAttr(inheritable_attr_node::MSVtorDispAttr),
  MaxFieldAlignmentAttr(inheritable_attr_node::MaxFieldAlignmentAttr),
  MayAliasAttr(inheritable_attr_node::MayAliasAttr),
  MaybeUndefAttr(inheritable_attr_node::MaybeUndefAttr),
  MicroMipsAttr(inheritable_attr_node::MicroMipsAttr),
  MinSizeAttr(inheritable_attr_node::MinSizeAttr),
  MinVectorWidthAttr(inheritable_attr_node::MinVectorWidthAttr),
  Mips16Attr(inheritable_attr_node::Mips16Attr),
  MipsInterruptAttr(inheritable_attr_node::MipsInterruptAttr),
  MipsLongCallAttr(inheritable_attr_node::MipsLongCallAttr),
  MipsShortCallAttr(inheritable_attr_node::MipsShortCallAttr),
  ModeAttr(ModeAttr),
  MustTailAttr(stmt_attr_node::MustTailAttr),
  NSConsumedAttr(inheritable_attr_node::inheritable_param_attr_node::NSConsumedAttr),
  NSConsumesSelfAttr(inheritable_attr_node::NSConsumesSelfAttr),
  NSErrorDomainAttr(inheritable_attr_node::NSErrorDomainAttr),
  NSReturnsAutoreleasedAttr(inheritable_attr_node::NSReturnsAutoreleasedAttr),
  NSReturnsNotRetainedAttr(inheritable_attr_node::NSReturnsNotRetainedAttr),
  NSReturnsRetainedAttr(inheritable_attr_node::NSReturnsRetainedAttr),
  NVPTXKernelAttr(inheritable_attr_node::NVPTXKernelAttr),
  NakedAttr(inheritable_attr_node::NakedAttr),
  NoAliasAttr(inheritable_attr_node::NoAliasAttr),
  NoBuiltinAttr(NoBuiltinAttr),
  NoCommonAttr(inheritable_attr_node::NoCommonAttr),
  NoDebugAttr(inheritable_attr_node::NoDebugAttr),
  NoDerefAttr(type_attr_node::NoDerefAttr),
  NoDestroyAttr(inheritable_attr_node::NoDestroyAttr),
  NoDuplicateAttr(inheritable_attr_node::NoDuplicateAttr),
  NoEscapeAttr(NoEscapeAttr),
  NoInlineAttr(inheritable_attr_node::decl_or_stmt_attr_node::NoInlineAttr),
  NoInstrumentFunctionAttr(inheritable_attr_node::NoInstrumentFunctionAttr),
  NoMergeAttr(inheritable_attr_node::decl_or_stmt_attr_node::NoMergeAttr),
  NoMicroMipsAttr(inheritable_attr_node::NoMicroMipsAttr),
  NoMips16Attr(inheritable_attr_node::NoMips16Attr),
  NoProfileFunctionAttr(inheritable_attr_node::NoProfileFunctionAttr),
  NoRandomizeLayoutAttr(inheritable_attr_node::NoRandomizeLayoutAttr),
  NoReturnAttr(inheritable_attr_node::NoReturnAttr),
  NoSanitizeAttr(inheritable_attr_node::NoSanitizeAttr),
  NoSpeculativeLoadHardeningAttr(inheritable_attr_node::NoSpeculativeLoadHardeningAttr),
  NoSplitStackAttr(inheritable_attr_node::NoSplitStackAttr),
  NoStackProtectorAttr(inheritable_attr_node::NoStackProtectorAttr),
  NoThreadSafetyAnalysisAttr(inheritable_attr_node::NoThreadSafetyAnalysisAttr),
  NoThrowAttr(inheritable_attr_node::NoThrowAttr),
  NoUniqueAddressAttr(inheritable_attr_node::NoUniqueAddressAttr),
  NoUwtableAttr(inheritable_attr_node::NoUwtableAttr),
  NonNullAttr(inheritable_attr_node::inheritable_param_attr_node::NonNullAttr),
  NotTailCalledAttr(inheritable_attr_node::NotTailCalledAttr),
  OSConsumedAttr(inheritable_attr_node::inheritable_param_attr_node::OSConsumedAttr),
  OSConsumesThisAttr(inheritable_attr_node::OSConsumesThisAttr),
  OSReturnsNotRetainedAttr(inheritable_attr_node::OSReturnsNotRetainedAttr),
  OSReturnsRetainedAttr(inheritable_attr_node::OSReturnsRetainedAttr),
  OSReturnsRetainedOnNonZeroAttr(inheritable_attr_node::OSReturnsRetainedOnNonZeroAttr),
  OSReturnsRetainedOnZeroAttr(inheritable_attr_node::OSReturnsRetainedOnZeroAttr),
  ObjCBoxableAttr(ObjCBoxableAttr),
  ObjCBridgeAttr(inheritable_attr_node::ObjCBridgeAttr),
  ObjCBridgeMutableAttr(inheritable_attr_node::ObjCBridgeMutableAttr),
  ObjCBridgeRelatedAttr(inheritable_attr_node::ObjCBridgeRelatedAttr),
  ObjCClassStubAttr(ObjCClassStubAttr),
  ObjCDesignatedInitializerAttr(ObjCDesignatedInitializerAttr),
  ObjCDirectAttr(ObjCDirectAttr),
  ObjCDirectMembersAttr(ObjCDirectMembersAttr),
  ObjCExceptionAttr(inheritable_attr_node::ObjCExceptionAttr),
  ObjCExplicitProtocolImplAttr(inheritable_attr_node::ObjCExplicitProtocolImplAttr),
  ObjCExternallyRetainedAttr(inheritable_attr_node::ObjCExternallyRetainedAttr),
  ObjCGCAttr(type_attr_node::ObjCGCAttr),
  ObjCIndependentClassAttr(inheritable_attr_node::ObjCIndependentClassAttr),
  ObjCInertUnsafeUnretainedAttr(type_attr_node::ObjCInertUnsafeUnretainedAttr),
  ObjCKindOfAttr(type_attr_node::ObjCKindOfAttr),
  ObjCMethodFamilyAttr(inheritable_attr_node::ObjCMethodFamilyAttr),
  ObjCNSObjectAttr(inheritable_attr_node::ObjCNSObjectAttr),
  ObjCNonLazyClassAttr(ObjCNonLazyClassAttr),
  ObjCNonRuntimeProtocolAttr(ObjCNonRuntimeProtocolAttr),
  ObjCOwnershipAttr(inheritable_attr_node::ObjCOwnershipAttr),
  ObjCPreciseLifetimeAttr(inheritable_attr_node::ObjCPreciseLifetimeAttr),
  ObjCRequiresPropertyDefsAttr(inheritable_attr_node::ObjCRequiresPropertyDefsAttr),
  ObjCRequiresSuperAttr(inheritable_attr_node::ObjCRequiresSuperAttr),
  ObjCReturnsInnerPointerAttr(inheritable_attr_node::ObjCReturnsInnerPointerAttr),
  ObjCRootClassAttr(inheritable_attr_node::ObjCRootClassAttr),
  ObjCRuntimeNameAttr(ObjCRuntimeNameAttr),
  ObjCRuntimeVisibleAttr(ObjCRuntimeVisibleAttr),
  ObjCSubclassingRestrictedAttr(inheritable_attr_node::ObjCSubclassingRestrictedAttr),
  OpenCLAccessAttr(OpenCLAccessAttr),
  OpenCLConstantAddressSpaceAttr(type_attr_node::OpenCLConstantAddressSpaceAttr),
  OpenCLGenericAddressSpaceAttr(type_attr_node::OpenCLGenericAddressSpaceAttr),
  OpenCLGlobalAddressSpaceAttr(type_attr_node::OpenCLGlobalAddressSpaceAttr),
  OpenCLGlobalDeviceAddressSpaceAttr(type_attr_node::OpenCLGlobalDeviceAddressSpaceAttr),
  OpenCLGlobalHostAddressSpaceAttr(type_attr_node::OpenCLGlobalHostAddressSpaceAttr),
  OpenCLIntelReqdSubGroupSizeAttr(inheritable_attr_node::OpenCLIntelReqdSubGroupSizeAttr),
  OpenCLKernelAttr(inheritable_attr_node::OpenCLKernelAttr),
  OpenCLLocalAddressSpaceAttr(type_attr_node::OpenCLLocalAddressSpaceAttr),
  OpenCLPrivateAddressSpaceAttr(type_attr_node::OpenCLPrivateAddressSpaceAttr),
  OpenCLUnrollHintAttr(stmt_attr_node::OpenCLUnrollHintAttr),
  OptimizeNoneAttr(inheritable_attr_node::OptimizeNoneAttr),
  OverloadableAttr(OverloadableAttr),
  OverrideAttr(inheritable_attr_node::OverrideAttr),
  OwnerAttr(inheritable_attr_node::OwnerAttr),
  OwnershipAttr(inheritable_attr_node::OwnershipAttr),
  PackedAttr(inheritable_attr_node::PackedAttr),
  ParamTypestateAttr(inheritable_attr_node::ParamTypestateAttr),
  ParameterABIAttr(inheritable_attr_node::inheritable_param_attr_node::parameter_abi_attr_node::ParameterABIAttr),
  PascalAttr(inheritable_attr_node::PascalAttr),
  PassObjectSizeAttr(inheritable_attr_node::inheritable_param_attr_node::PassObjectSizeAttr),
  PatchableFunctionEntryAttr(inheritable_attr_node::PatchableFunctionEntryAttr),
  PcsAttr(inheritable_attr_node::PcsAttr),
  PointerAttr(inheritable_attr_node::PointerAttr),
  PragmaClangBSSSectionAttr(inheritable_attr_node::PragmaClangBSSSectionAttr),
  PragmaClangDataSectionAttr(inheritable_attr_node::PragmaClangDataSectionAttr),
  PragmaClangRelroSectionAttr(inheritable_attr_node::PragmaClangRelroSectionAttr),
  PragmaClangRodataSectionAttr(inheritable_attr_node::PragmaClangRodataSectionAttr),
  PragmaClangTextSectionAttr(inheritable_attr_node::PragmaClangTextSectionAttr),
  PreferredNameAttr(inheritable_attr_node::PreferredNameAttr),
  PreferredTypeAttr(inheritable_attr_node::PreferredTypeAttr),
  PreserveAllAttr(inheritable_attr_node::PreserveAllAttr),
  PreserveMostAttr(inheritable_attr_node::PreserveMostAttr),
  PtGuardedByAttr(inheritable_attr_node::PtGuardedByAttr),
  PtGuardedVarAttr(inheritable_attr_node::PtGuardedVarAttr),
  Ptr32Attr(type_attr_node::Ptr32Attr),
  Ptr64Attr(type_attr_node::Ptr64Attr),
  PureAttr(inheritable_attr_node::PureAttr),
  RISCVInterruptAttr(inheritable_attr_node::RISCVInterruptAttr),
  RandomizeLayoutAttr(inheritable_attr_node::RandomizeLayoutAttr),
  ReadOnlyPlacementAttr(inheritable_attr_node::ReadOnlyPlacementAttr),
  RegCallAttr(inheritable_attr_node::RegCallAttr),
  ReinitializesAttr(inheritable_attr_node::ReinitializesAttr),
  ReleaseCapabilityAttr(inheritable_attr_node::ReleaseCapabilityAttr),
  ReleaseHandleAttr(inheritable_attr_node::inheritable_param_attr_node::ReleaseHandleAttr),
  RenderScriptKernelAttr(RenderScriptKernelAttr),
  ReqdWorkGroupSizeAttr(inheritable_attr_node::ReqdWorkGroupSizeAttr),
  RequiresCapabilityAttr(inheritable_attr_node::RequiresCapabilityAttr),
  RestrictAttr(inheritable_attr_node::RestrictAttr),
  RetainAttr(inheritable_attr_node::RetainAttr),
  ReturnTypestateAttr(inheritable_attr_node::ReturnTypestateAttr),
  ReturnsNonNullAttr(inheritable_attr_node::ReturnsNonNullAttr),
  ReturnsTwiceAttr(inheritable_attr_node::ReturnsTwiceAttr),
  SPtrAttr(type_attr_node::SPtrAttr),
  SYCLKernelAttr(inheritable_attr_node::SYCLKernelAttr),
  SYCLSpecialClassAttr(inheritable_attr_node::SYCLSpecialClassAttr),
  ScopedLockableAttr(inheritable_attr_node::ScopedLockableAttr),
  SectionAttr(inheritable_attr_node::SectionAttr),
  SelectAnyAttr(inheritable_attr_node::SelectAnyAttr),
  SentinelAttr(inheritable_attr_node::SentinelAttr),
  SetTypestateAttr(inheritable_attr_node::SetTypestateAttr),
  SharedTrylockFunctionAttr(inheritable_attr_node::SharedTrylockFunctionAttr),
  SpeculativeLoadHardeningAttr(inheritable_attr_node::SpeculativeLoadHardeningAttr),
  StandaloneDebugAttr(inheritable_attr_node::StandaloneDebugAttr),
  StdCallAttr(inheritable_attr_node::StdCallAttr),
  StmtAttr(stmt_attr_node::StmtAttr),
  StrictFPAttr(inheritable_attr_node::StrictFPAttr),
  StrictGuardStackCheckAttr(inheritable_attr_node::StrictGuardStackCheckAttr),
  SuppressAttr(inheritable_attr_node::decl_or_stmt_attr_node::SuppressAttr),
  SwiftAsyncAttr(inheritable_attr_node::SwiftAsyncAttr),
  SwiftAsyncCallAttr(inheritable_attr_node::SwiftAsyncCallAttr),
  SwiftAsyncContextAttr(inheritable_attr_node::inheritable_param_attr_node::parameter_abi_attr_node::SwiftAsyncContextAttr),
  SwiftAsyncErrorAttr(inheritable_attr_node::SwiftAsyncErrorAttr),
  SwiftAsyncNameAttr(inheritable_attr_node::SwiftAsyncNameAttr),
  SwiftAttrAttr(inheritable_attr_node::SwiftAttrAttr),
  SwiftBridgeAttr(inheritable_attr_node::SwiftBridgeAttr),
  SwiftBridgedTypedefAttr(inheritable_attr_node::SwiftBridgedTypedefAttr),
  SwiftCallAttr(inheritable_attr_node::SwiftCallAttr),
  SwiftContextAttr(inheritable_attr_node::inheritable_param_attr_node::parameter_abi_attr_node::SwiftContextAttr),
  SwiftErrorAttr(inheritable_attr_node::SwiftErrorAttr),
  SwiftErrorResultAttr(inheritable_attr_node::inheritable_param_attr_node::parameter_abi_attr_node::SwiftErrorResultAttr),
  SwiftImportAsNonGenericAttr(inheritable_attr_node::SwiftImportAsNonGenericAttr),
  SwiftImportPropertyAsAccessorsAttr(inheritable_attr_node::SwiftImportPropertyAsAccessorsAttr),
  SwiftIndirectResultAttr(inheritable_attr_node::inheritable_param_attr_node::parameter_abi_attr_node::SwiftIndirectResultAttr),
  SwiftNameAttr(inheritable_attr_node::SwiftNameAttr),
  SwiftNewTypeAttr(inheritable_attr_node::SwiftNewTypeAttr),
  SwiftObjCMembersAttr(SwiftObjCMembersAttr),
  SwiftPrivateAttr(inheritable_attr_node::SwiftPrivateAttr),
  SwiftVersionedAdditionAttr(SwiftVersionedAdditionAttr),
  SwiftVersionedRemovalAttr(SwiftVersionedRemovalAttr),
  SysVABIAttr(inheritable_attr_node::SysVABIAttr),
  TLSModelAttr(inheritable_attr_node::TLSModelAttr),
  TargetAttr(inheritable_attr_node::TargetAttr),
  TargetClonesAttr(inheritable_attr_node::TargetClonesAttr),
  TargetVersionAttr(inheritable_attr_node::TargetVersionAttr),
  TestTypestateAttr(inheritable_attr_node::TestTypestateAttr),
  ThisCallAttr(inheritable_attr_node::ThisCallAttr),
  ThreadAttr(ThreadAttr),
  TransparentUnionAttr(inheritable_attr_node::TransparentUnionAttr),
  TrivialABIAttr(inheritable_attr_node::TrivialABIAttr),
  TryAcquireCapabilityAttr(inheritable_attr_node::TryAcquireCapabilityAttr),
  TypeAttr(type_attr_node::TypeAttr),
  TypeNonNullAttr(type_attr_node::TypeNonNullAttr),
  TypeNullUnspecifiedAttr(type_attr_node::TypeNullUnspecifiedAttr),
  TypeNullableAttr(type_attr_node::TypeNullableAttr),
  TypeNullableResultAttr(type_attr_node::TypeNullableResultAttr),
  TypeTagForDatatypeAttr(inheritable_attr_node::TypeTagForDatatypeAttr),
  TypeVisibilityAttr(inheritable_attr_node::TypeVisibilityAttr),
  UPtrAttr(type_attr_node::UPtrAttr),
  UnavailableAttr(inheritable_attr_node::UnavailableAttr),
  UninitializedAttr(inheritable_attr_node::UninitializedAttr),
  UnlikelyAttr(stmt_attr_node::UnlikelyAttr),
  UnsafeBufferUsageAttr(inheritable_attr_node::UnsafeBufferUsageAttr),
  UnusedAttr(inheritable_attr_node::UnusedAttr),
  UseHandleAttr(inheritable_attr_node::inheritable_param_attr_node::UseHandleAttr),
  UsedAttr(inheritable_attr_node::UsedAttr),
  UsingIfExistsAttr(inheritable_attr_node::UsingIfExistsAttr),
  UuidAttr(inheritable_attr_node::UuidAttr),
  VecReturnAttr(inheritable_attr_node::VecReturnAttr),
  VecTypeHintAttr(inheritable_attr_node::VecTypeHintAttr),
  VectorCallAttr(inheritable_attr_node::VectorCallAttr),
  VisibilityAttr(inheritable_attr_node::VisibilityAttr),
  WarnUnusedAttr(inheritable_attr_node::WarnUnusedAttr),
  WarnUnusedResultAttr(inheritable_attr_node::WarnUnusedResultAttr),
  WeakAttr(inheritable_attr_node::WeakAttr),
  WeakImportAttr(inheritable_attr_node::WeakImportAttr),
  WeakRefAttr(inheritable_attr_node::WeakRefAttr),
  WebAssemblyExportNameAttr(inheritable_attr_node::WebAssemblyExportNameAttr),
  WebAssemblyFuncrefAttr(type_attr_node::WebAssemblyFuncrefAttr),
  WebAssemblyImportModuleAttr(inheritable_attr_node::WebAssemblyImportModuleAttr),
  WebAssemblyImportNameAttr(inheritable_attr_node::WebAssemblyImportNameAttr),
  WorkGroupSizeHintAttr(inheritable_attr_node::WorkGroupSizeHintAttr),
  X86ForceAlignArgPointerAttr(inheritable_attr_node::X86ForceAlignArgPointerAttr),
  XRayInstrumentAttr(inheritable_attr_node::XRayInstrumentAttr),
  XRayLogArgsAttr(inheritable_attr_node::XRayLogArgsAttr),
  ZeroCallUsedRegsAttr(inheritable_attr_node::ZeroCallUsedRegsAttr),
}

