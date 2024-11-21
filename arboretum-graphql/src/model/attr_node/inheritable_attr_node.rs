use juniper::*;

pub mod decl_or_stmt_attr_node;
pub mod hlsl_annotation_attr_node;
pub mod inheritable_param_attr_node;

#[derive(GraphQLObject)]
pub struct InheritableAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AArch64SVEPcsAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AArch64VectorPcsAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AMDGPUFlatWorkGroupSizeAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AMDGPUKernelCallAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AMDGPUNumSGPRAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AMDGPUNumVGPRAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AMDGPUWavesPerEUAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ARMInterruptAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AVRInterruptAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AVRSignalAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AcquireCapabilityAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AcquireHandleAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AcquiredAfterAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AcquiredBeforeAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AlignMac68kAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AlignNaturalAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AlignedAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AllocAlignAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AllocSizeAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AlwaysDestroyAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AnalyzerNoReturnAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AnyX86InterruptAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AnyX86NoCallerSavedRegistersAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AnyX86NoCfCheckAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ArcWeakrefUnavailableAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ArgumentWithTypeTagAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ArmBuiltinAliasAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ArmLocallyStreamingAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ArmNewAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ArtificialAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AsmLabelAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AssertCapabilityAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AssertExclusiveLockAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AssertSharedLockAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AssumeAlignedAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AssumptionAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AvailabilityAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct AvailableOnlyInDefaultEvalMethodAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct BPFPreserveAccessIndexAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct BPFPreserveStaticOffsetAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct BTFDeclTagAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct BlocksAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct BuiltinAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct C11NoReturnAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CDeclAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CFAuditedTransferAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CFGuardAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CFICanonicalJumpTableAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CFReturnsNotRetainedAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CFReturnsRetainedAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CFUnknownTransferAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CPUDispatchAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CPUSpecificAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CUDAConstantAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CUDADeviceAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CUDADeviceBuiltinSurfaceTypeAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CUDADeviceBuiltinTextureTypeAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CUDAGlobalAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CUDAHostAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CUDAInvalidTargetAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CUDALaunchBoundsAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CUDASharedAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CXX11NoReturnAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CallableWhenAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CallbackAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CapabilityAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CapturedRecordAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CleanupAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CmseNSEntryAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CodeModelAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CodeSegAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ColdAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CommonAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ConstAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ConstInitAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ConstructorAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ConsumableAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ConsumableAutoCastAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ConsumableSetOnReadAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ConvergentAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CoroDisableLifetimeBoundAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CoroLifetimeBoundAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CoroOnlyDestroyWhenCompleteAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CoroReturnTypeAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CoroWrapperAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct CountedByAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DLLExportAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DLLExportStaticLocalAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DLLImportAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DLLImportStaticLocalAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DeprecatedAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DestructorAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DiagnoseAsBuiltinAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DiagnoseIfAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DisableSanitizerInstrumentationAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct DisableTailCallsAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct EmptyBasesAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct EnableIfAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct EnforceTCBAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct EnforceTCBLeafAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct EnumExtensibilityAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ErrorAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ExcludeFromExplicitInstantiationAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ExclusiveTrylockFunctionAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ExternalSourceSymbolAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct FastCallAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct FinalAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct FlagEnumAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct FlattenAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct FormatAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct FormatArgAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct FunctionReturnThunksAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct GNUInlineAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct GuardedByAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct GuardedVarAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct HIPManagedAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct HLSLNumThreadsAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct HLSLResourceAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct HLSLResourceBindingAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct HLSLShaderAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct HotAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct IBActionAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct IBOutletAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct IBOutletCollectionAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct InitPriorityAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct IntelOclBiccAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct InternalLinkageAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct LTOVisibilityPublicAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct LayoutVersionAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct LeafAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct LifetimeBoundAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct LockReturnedAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct LocksExcludedAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct M68kInterruptAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct M68kRTDAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MIGServerRoutineAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MSABIAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MSAllocatorAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MSConstexprAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MSInheritanceAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MSNoVTableAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MSP430InterruptAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MSStructAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MSVtorDispAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MaxFieldAlignmentAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MayAliasAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MaybeUndefAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MicroMipsAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MinSizeAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MinVectorWidthAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct Mips16Attr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MipsInterruptAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MipsLongCallAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct MipsShortCallAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NSConsumesSelfAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NSErrorDomainAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NSReturnsAutoreleasedAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NSReturnsNotRetainedAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NSReturnsRetainedAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NVPTXKernelAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NakedAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NoAliasAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NoCommonAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NoDebugAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NoDestroyAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NoDuplicateAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NoInstrumentFunctionAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NoMicroMipsAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NoMips16Attr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NoProfileFunctionAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NoRandomizeLayoutAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NoReturnAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NoSanitizeAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NoSpeculativeLoadHardeningAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NoSplitStackAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NoStackProtectorAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NoThreadSafetyAnalysisAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NoThrowAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NoUniqueAddressAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NoUwtableAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct NotTailCalledAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct OSConsumesThisAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct OSReturnsNotRetainedAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct OSReturnsRetainedAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct OSReturnsRetainedOnNonZeroAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct OSReturnsRetainedOnZeroAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCBridgeAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCBridgeMutableAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCBridgeRelatedAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCExceptionAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCExplicitProtocolImplAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCExternallyRetainedAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCIndependentClassAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCMethodFamilyAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCNSObjectAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCOwnershipAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCPreciseLifetimeAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCRequiresPropertyDefsAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCRequiresSuperAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCReturnsInnerPointerAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCRootClassAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ObjCSubclassingRestrictedAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct OpenCLIntelReqdSubGroupSizeAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct OpenCLKernelAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct OptimizeNoneAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct OverrideAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct OwnerAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct OwnershipAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct PackedAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ParamTypestateAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct PascalAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct PatchableFunctionEntryAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct PcsAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct PointerAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct PragmaClangBSSSectionAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct PragmaClangDataSectionAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct PragmaClangRelroSectionAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct PragmaClangRodataSectionAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct PragmaClangTextSectionAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct PreferredNameAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct PreferredTypeAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct PreserveAllAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct PreserveMostAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct PtGuardedByAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct PtGuardedVarAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct PureAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct RISCVInterruptAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct RandomizeLayoutAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ReadOnlyPlacementAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct RegCallAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ReinitializesAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ReleaseCapabilityAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ReqdWorkGroupSizeAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct RequiresCapabilityAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct RestrictAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct RetainAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ReturnTypestateAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ReturnsNonNullAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ReturnsTwiceAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SYCLKernelAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SYCLSpecialClassAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ScopedLockableAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SectionAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SelectAnyAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SentinelAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SetTypestateAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SharedTrylockFunctionAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SpeculativeLoadHardeningAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct StandaloneDebugAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct StdCallAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct StrictFPAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct StrictGuardStackCheckAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SwiftAsyncAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SwiftAsyncCallAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SwiftAsyncErrorAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SwiftAsyncNameAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SwiftAttrAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SwiftBridgeAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SwiftBridgedTypedefAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SwiftCallAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SwiftErrorAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SwiftImportAsNonGenericAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SwiftImportPropertyAsAccessorsAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SwiftNameAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SwiftNewTypeAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SwiftPrivateAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct SysVABIAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct TLSModelAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct TargetAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct TargetClonesAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct TargetVersionAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct TestTypestateAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ThisCallAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct TransparentUnionAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct TrivialABIAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct TryAcquireCapabilityAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct TypeTagForDatatypeAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct TypeVisibilityAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct UnavailableAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct UninitializedAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct UnsafeBufferUsageAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct UnusedAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct UsedAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct UsingIfExistsAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct UuidAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct VecReturnAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct VecTypeHintAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct VectorCallAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct VisibilityAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct WarnUnusedAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct WarnUnusedResultAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct WeakAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct WeakImportAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct WeakRefAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct WebAssemblyExportNameAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct WebAssemblyImportModuleAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct WebAssemblyImportNameAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct WorkGroupSizeHintAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct X86ForceAlignArgPointerAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct XRayInstrumentAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct XRayLogArgsAttr {
  id: juniper::ID,
}

#[derive(GraphQLObject)]
pub struct ZeroCallUsedRegsAttr {
  id: juniper::ID,
}

#[derive(GraphQLUnion)]
pub enum InheritableAttrNode {
  InheritableAttr(InheritableAttr),
  AArch64SVEPcsAttr(AArch64SVEPcsAttr),
  AArch64VectorPcsAttr(AArch64VectorPcsAttr),
  AMDGPUFlatWorkGroupSizeAttr(AMDGPUFlatWorkGroupSizeAttr),
  AMDGPUKernelCallAttr(AMDGPUKernelCallAttr),
  AMDGPUNumSGPRAttr(AMDGPUNumSGPRAttr),
  AMDGPUNumVGPRAttr(AMDGPUNumVGPRAttr),
  AMDGPUWavesPerEUAttr(AMDGPUWavesPerEUAttr),
  ARMInterruptAttr(ARMInterruptAttr),
  AVRInterruptAttr(AVRInterruptAttr),
  AVRSignalAttr(AVRSignalAttr),
  AcquireCapabilityAttr(AcquireCapabilityAttr),
  AcquireHandleAttr(AcquireHandleAttr),
  AcquiredAfterAttr(AcquiredAfterAttr),
  AcquiredBeforeAttr(AcquiredBeforeAttr),
  AlignMac68kAttr(AlignMac68kAttr),
  AlignNaturalAttr(AlignNaturalAttr),
  AlignedAttr(AlignedAttr),
  AllocAlignAttr(AllocAlignAttr),
  AllocSizeAttr(AllocSizeAttr),
  AlwaysDestroyAttr(AlwaysDestroyAttr),
  AlwaysInlineAttr(decl_or_stmt_attr_node::AlwaysInlineAttr),
  AnalyzerNoReturnAttr(AnalyzerNoReturnAttr),
  AnnotateAttr(inheritable_param_attr_node::AnnotateAttr),
  AnyX86InterruptAttr(AnyX86InterruptAttr),
  AnyX86NoCallerSavedRegistersAttr(AnyX86NoCallerSavedRegistersAttr),
  AnyX86NoCfCheckAttr(AnyX86NoCfCheckAttr),
  ArcWeakrefUnavailableAttr(ArcWeakrefUnavailableAttr),
  ArgumentWithTypeTagAttr(ArgumentWithTypeTagAttr),
  ArmBuiltinAliasAttr(ArmBuiltinAliasAttr),
  ArmLocallyStreamingAttr(ArmLocallyStreamingAttr),
  ArmNewAttr(ArmNewAttr),
  ArtificialAttr(ArtificialAttr),
  AsmLabelAttr(AsmLabelAttr),
  AssertCapabilityAttr(AssertCapabilityAttr),
  AssertExclusiveLockAttr(AssertExclusiveLockAttr),
  AssertSharedLockAttr(AssertSharedLockAttr),
  AssumeAlignedAttr(AssumeAlignedAttr),
  AssumptionAttr(AssumptionAttr),
  AvailabilityAttr(AvailabilityAttr),
  AvailableOnlyInDefaultEvalMethodAttr(AvailableOnlyInDefaultEvalMethodAttr),
  BPFPreserveAccessIndexAttr(BPFPreserveAccessIndexAttr),
  BPFPreserveStaticOffsetAttr(BPFPreserveStaticOffsetAttr),
  BTFDeclTagAttr(BTFDeclTagAttr),
  BlocksAttr(BlocksAttr),
  BuiltinAttr(BuiltinAttr),
  C11NoReturnAttr(C11NoReturnAttr),
  CDeclAttr(CDeclAttr),
  CFAuditedTransferAttr(CFAuditedTransferAttr),
  CFConsumedAttr(inheritable_param_attr_node::CFConsumedAttr),
  CFGuardAttr(CFGuardAttr),
  CFICanonicalJumpTableAttr(CFICanonicalJumpTableAttr),
  CFReturnsNotRetainedAttr(CFReturnsNotRetainedAttr),
  CFReturnsRetainedAttr(CFReturnsRetainedAttr),
  CFUnknownTransferAttr(CFUnknownTransferAttr),
  CPUDispatchAttr(CPUDispatchAttr),
  CPUSpecificAttr(CPUSpecificAttr),
  CUDAConstantAttr(CUDAConstantAttr),
  CUDADeviceAttr(CUDADeviceAttr),
  CUDADeviceBuiltinSurfaceTypeAttr(CUDADeviceBuiltinSurfaceTypeAttr),
  CUDADeviceBuiltinTextureTypeAttr(CUDADeviceBuiltinTextureTypeAttr),
  CUDAGlobalAttr(CUDAGlobalAttr),
  CUDAHostAttr(CUDAHostAttr),
  CUDAInvalidTargetAttr(CUDAInvalidTargetAttr),
  CUDALaunchBoundsAttr(CUDALaunchBoundsAttr),
  CUDASharedAttr(CUDASharedAttr),
  CXX11NoReturnAttr(CXX11NoReturnAttr),
  CallableWhenAttr(CallableWhenAttr),
  CallbackAttr(CallbackAttr),
  CapabilityAttr(CapabilityAttr),
  CapturedRecordAttr(CapturedRecordAttr),
  CarriesDependencyAttr(inheritable_param_attr_node::CarriesDependencyAttr),
  CleanupAttr(CleanupAttr),
  CmseNSEntryAttr(CmseNSEntryAttr),
  CodeModelAttr(CodeModelAttr),
  CodeSegAttr(CodeSegAttr),
  ColdAttr(ColdAttr),
  CommonAttr(CommonAttr),
  ConstAttr(ConstAttr),
  ConstInitAttr(ConstInitAttr),
  ConstructorAttr(ConstructorAttr),
  ConsumableAttr(ConsumableAttr),
  ConsumableAutoCastAttr(ConsumableAutoCastAttr),
  ConsumableSetOnReadAttr(ConsumableSetOnReadAttr),
  ConvergentAttr(ConvergentAttr),
  CoroDisableLifetimeBoundAttr(CoroDisableLifetimeBoundAttr),
  CoroLifetimeBoundAttr(CoroLifetimeBoundAttr),
  CoroOnlyDestroyWhenCompleteAttr(CoroOnlyDestroyWhenCompleteAttr),
  CoroReturnTypeAttr(CoroReturnTypeAttr),
  CoroWrapperAttr(CoroWrapperAttr),
  CountedByAttr(CountedByAttr),
  DLLExportAttr(DLLExportAttr),
  DLLExportStaticLocalAttr(DLLExportStaticLocalAttr),
  DLLImportAttr(DLLImportAttr),
  DLLImportStaticLocalAttr(DLLImportStaticLocalAttr),
  DeclOrStmtAttr(decl_or_stmt_attr_node::DeclOrStmtAttr),
  DeprecatedAttr(DeprecatedAttr),
  DestructorAttr(DestructorAttr),
  DiagnoseAsBuiltinAttr(DiagnoseAsBuiltinAttr),
  DiagnoseIfAttr(DiagnoseIfAttr),
  DisableSanitizerInstrumentationAttr(DisableSanitizerInstrumentationAttr),
  DisableTailCallsAttr(DisableTailCallsAttr),
  EmptyBasesAttr(EmptyBasesAttr),
  EnableIfAttr(EnableIfAttr),
  EnforceTCBAttr(EnforceTCBAttr),
  EnforceTCBLeafAttr(EnforceTCBLeafAttr),
  EnumExtensibilityAttr(EnumExtensibilityAttr),
  ErrorAttr(ErrorAttr),
  ExcludeFromExplicitInstantiationAttr(ExcludeFromExplicitInstantiationAttr),
  ExclusiveTrylockFunctionAttr(ExclusiveTrylockFunctionAttr),
  ExternalSourceSymbolAttr(ExternalSourceSymbolAttr),
  FastCallAttr(FastCallAttr),
  FinalAttr(FinalAttr),
  FlagEnumAttr(FlagEnumAttr),
  FlattenAttr(FlattenAttr),
  FormatArgAttr(FormatArgAttr),
  FormatAttr(FormatAttr),
  FunctionReturnThunksAttr(FunctionReturnThunksAttr),
  GNUInlineAttr(GNUInlineAttr),
  GuardedByAttr(GuardedByAttr),
  GuardedVarAttr(GuardedVarAttr),
  HIPManagedAttr(HIPManagedAttr),
  HLSLAnnotationAttr(hlsl_annotation_attr_node::HLSLAnnotationAttr),
  HLSLNumThreadsAttr(HLSLNumThreadsAttr),
  HLSLResourceAttr(HLSLResourceAttr),
  HLSLResourceBindingAttr(HLSLResourceBindingAttr),
  HLSLSVDispatchThreadIDAttr(hlsl_annotation_attr_node::HLSLSVDispatchThreadIDAttr),
  HLSLSVGroupIndexAttr(hlsl_annotation_attr_node::HLSLSVGroupIndexAttr),
  HLSLShaderAttr(HLSLShaderAttr),
  HotAttr(HotAttr),
  IBActionAttr(IBActionAttr),
  IBOutletAttr(IBOutletAttr),
  IBOutletCollectionAttr(IBOutletCollectionAttr),
  InheritableParamAttr(inheritable_param_attr_node::InheritableParamAttr),
  InitPriorityAttr(InitPriorityAttr),
  IntelOclBiccAttr(IntelOclBiccAttr),
  InternalLinkageAttr(InternalLinkageAttr),
  LTOVisibilityPublicAttr(LTOVisibilityPublicAttr),
  LayoutVersionAttr(LayoutVersionAttr),
  LeafAttr(LeafAttr),
  LifetimeBoundAttr(LifetimeBoundAttr),
  LockReturnedAttr(LockReturnedAttr),
  LocksExcludedAttr(LocksExcludedAttr),
  M68kInterruptAttr(M68kInterruptAttr),
  M68kRTDAttr(M68kRTDAttr),
  MIGServerRoutineAttr(MIGServerRoutineAttr),
  MSABIAttr(MSABIAttr),
  MSAllocatorAttr(MSAllocatorAttr),
  MSConstexprAttr(MSConstexprAttr),
  MSInheritanceAttr(MSInheritanceAttr),
  MSNoVTableAttr(MSNoVTableAttr),
  MSP430InterruptAttr(MSP430InterruptAttr),
  MSStructAttr(MSStructAttr),
  MSVtorDispAttr(MSVtorDispAttr),
  MaxFieldAlignmentAttr(MaxFieldAlignmentAttr),
  MayAliasAttr(MayAliasAttr),
  MaybeUndefAttr(MaybeUndefAttr),
  MicroMipsAttr(MicroMipsAttr),
  MinSizeAttr(MinSizeAttr),
  MinVectorWidthAttr(MinVectorWidthAttr),
  Mips16Attr(Mips16Attr),
  MipsInterruptAttr(MipsInterruptAttr),
  MipsLongCallAttr(MipsLongCallAttr),
  MipsShortCallAttr(MipsShortCallAttr),
  NSConsumedAttr(inheritable_param_attr_node::NSConsumedAttr),
  NSConsumesSelfAttr(NSConsumesSelfAttr),
  NSErrorDomainAttr(NSErrorDomainAttr),
  NSReturnsAutoreleasedAttr(NSReturnsAutoreleasedAttr),
  NSReturnsNotRetainedAttr(NSReturnsNotRetainedAttr),
  NSReturnsRetainedAttr(NSReturnsRetainedAttr),
  NVPTXKernelAttr(NVPTXKernelAttr),
  NakedAttr(NakedAttr),
  NoAliasAttr(NoAliasAttr),
  NoCommonAttr(NoCommonAttr),
  NoDebugAttr(NoDebugAttr),
  NoDestroyAttr(NoDestroyAttr),
  NoDuplicateAttr(NoDuplicateAttr),
  NoInlineAttr(decl_or_stmt_attr_node::NoInlineAttr),
  NoInstrumentFunctionAttr(NoInstrumentFunctionAttr),
  NoMergeAttr(decl_or_stmt_attr_node::NoMergeAttr),
  NoMicroMipsAttr(NoMicroMipsAttr),
  NoMips16Attr(NoMips16Attr),
  NoProfileFunctionAttr(NoProfileFunctionAttr),
  NoRandomizeLayoutAttr(NoRandomizeLayoutAttr),
  NoReturnAttr(NoReturnAttr),
  NoSanitizeAttr(NoSanitizeAttr),
  NoSpeculativeLoadHardeningAttr(NoSpeculativeLoadHardeningAttr),
  NoSplitStackAttr(NoSplitStackAttr),
  NoStackProtectorAttr(NoStackProtectorAttr),
  NoThreadSafetyAnalysisAttr(NoThreadSafetyAnalysisAttr),
  NoThrowAttr(NoThrowAttr),
  NoUniqueAddressAttr(NoUniqueAddressAttr),
  NoUwtableAttr(NoUwtableAttr),
  NonNullAttr(inheritable_param_attr_node::NonNullAttr),
  NotTailCalledAttr(NotTailCalledAttr),
  OSConsumedAttr(inheritable_param_attr_node::OSConsumedAttr),
  OSConsumesThisAttr(OSConsumesThisAttr),
  OSReturnsNotRetainedAttr(OSReturnsNotRetainedAttr),
  OSReturnsRetainedAttr(OSReturnsRetainedAttr),
  OSReturnsRetainedOnNonZeroAttr(OSReturnsRetainedOnNonZeroAttr),
  OSReturnsRetainedOnZeroAttr(OSReturnsRetainedOnZeroAttr),
  ObjCBridgeAttr(ObjCBridgeAttr),
  ObjCBridgeMutableAttr(ObjCBridgeMutableAttr),
  ObjCBridgeRelatedAttr(ObjCBridgeRelatedAttr),
  ObjCExceptionAttr(ObjCExceptionAttr),
  ObjCExplicitProtocolImplAttr(ObjCExplicitProtocolImplAttr),
  ObjCExternallyRetainedAttr(ObjCExternallyRetainedAttr),
  ObjCIndependentClassAttr(ObjCIndependentClassAttr),
  ObjCMethodFamilyAttr(ObjCMethodFamilyAttr),
  ObjCNSObjectAttr(ObjCNSObjectAttr),
  ObjCOwnershipAttr(ObjCOwnershipAttr),
  ObjCPreciseLifetimeAttr(ObjCPreciseLifetimeAttr),
  ObjCRequiresPropertyDefsAttr(ObjCRequiresPropertyDefsAttr),
  ObjCRequiresSuperAttr(ObjCRequiresSuperAttr),
  ObjCReturnsInnerPointerAttr(ObjCReturnsInnerPointerAttr),
  ObjCRootClassAttr(ObjCRootClassAttr),
  ObjCSubclassingRestrictedAttr(ObjCSubclassingRestrictedAttr),
  OpenCLIntelReqdSubGroupSizeAttr(OpenCLIntelReqdSubGroupSizeAttr),
  OpenCLKernelAttr(OpenCLKernelAttr),
  OptimizeNoneAttr(OptimizeNoneAttr),
  OverrideAttr(OverrideAttr),
  OwnerAttr(OwnerAttr),
  OwnershipAttr(OwnershipAttr),
  PackedAttr(PackedAttr),
  ParamTypestateAttr(ParamTypestateAttr),
  ParameterABIAttr(inheritable_param_attr_node::parameter_abi_attr_node::ParameterABIAttr),
  PascalAttr(PascalAttr),
  PassObjectSizeAttr(inheritable_param_attr_node::PassObjectSizeAttr),
  PatchableFunctionEntryAttr(PatchableFunctionEntryAttr),
  PcsAttr(PcsAttr),
  PointerAttr(PointerAttr),
  PragmaClangBSSSectionAttr(PragmaClangBSSSectionAttr),
  PragmaClangDataSectionAttr(PragmaClangDataSectionAttr),
  PragmaClangRelroSectionAttr(PragmaClangRelroSectionAttr),
  PragmaClangRodataSectionAttr(PragmaClangRodataSectionAttr),
  PragmaClangTextSectionAttr(PragmaClangTextSectionAttr),
  PreferredNameAttr(PreferredNameAttr),
  PreferredTypeAttr(PreferredTypeAttr),
  PreserveAllAttr(PreserveAllAttr),
  PreserveMostAttr(PreserveMostAttr),
  PtGuardedByAttr(PtGuardedByAttr),
  PtGuardedVarAttr(PtGuardedVarAttr),
  PureAttr(PureAttr),
  RISCVInterruptAttr(RISCVInterruptAttr),
  RandomizeLayoutAttr(RandomizeLayoutAttr),
  ReadOnlyPlacementAttr(ReadOnlyPlacementAttr),
  RegCallAttr(RegCallAttr),
  ReinitializesAttr(ReinitializesAttr),
  ReleaseCapabilityAttr(ReleaseCapabilityAttr),
  ReleaseHandleAttr(inheritable_param_attr_node::ReleaseHandleAttr),
  ReqdWorkGroupSizeAttr(ReqdWorkGroupSizeAttr),
  RequiresCapabilityAttr(RequiresCapabilityAttr),
  RestrictAttr(RestrictAttr),
  RetainAttr(RetainAttr),
  ReturnTypestateAttr(ReturnTypestateAttr),
  ReturnsNonNullAttr(ReturnsNonNullAttr),
  ReturnsTwiceAttr(ReturnsTwiceAttr),
  SYCLKernelAttr(SYCLKernelAttr),
  SYCLSpecialClassAttr(SYCLSpecialClassAttr),
  ScopedLockableAttr(ScopedLockableAttr),
  SectionAttr(SectionAttr),
  SelectAnyAttr(SelectAnyAttr),
  SentinelAttr(SentinelAttr),
  SetTypestateAttr(SetTypestateAttr),
  SharedTrylockFunctionAttr(SharedTrylockFunctionAttr),
  SpeculativeLoadHardeningAttr(SpeculativeLoadHardeningAttr),
  StandaloneDebugAttr(StandaloneDebugAttr),
  StdCallAttr(StdCallAttr),
  StrictFPAttr(StrictFPAttr),
  StrictGuardStackCheckAttr(StrictGuardStackCheckAttr),
  SuppressAttr(decl_or_stmt_attr_node::SuppressAttr),
  SwiftAsyncAttr(SwiftAsyncAttr),
  SwiftAsyncCallAttr(SwiftAsyncCallAttr),
  SwiftAsyncContextAttr(inheritable_param_attr_node::parameter_abi_attr_node::SwiftAsyncContextAttr),
  SwiftAsyncErrorAttr(SwiftAsyncErrorAttr),
  SwiftAsyncNameAttr(SwiftAsyncNameAttr),
  SwiftAttrAttr(SwiftAttrAttr),
  SwiftBridgeAttr(SwiftBridgeAttr),
  SwiftBridgedTypedefAttr(SwiftBridgedTypedefAttr),
  SwiftCallAttr(SwiftCallAttr),
  SwiftContextAttr(inheritable_param_attr_node::parameter_abi_attr_node::SwiftContextAttr),
  SwiftErrorAttr(SwiftErrorAttr),
  SwiftErrorResultAttr(inheritable_param_attr_node::parameter_abi_attr_node::SwiftErrorResultAttr),
  SwiftImportAsNonGenericAttr(SwiftImportAsNonGenericAttr),
  SwiftImportPropertyAsAccessorsAttr(SwiftImportPropertyAsAccessorsAttr),
  SwiftIndirectResultAttr(inheritable_param_attr_node::parameter_abi_attr_node::SwiftIndirectResultAttr),
  SwiftNameAttr(SwiftNameAttr),
  SwiftNewTypeAttr(SwiftNewTypeAttr),
  SwiftPrivateAttr(SwiftPrivateAttr),
  SysVABIAttr(SysVABIAttr),
  TLSModelAttr(TLSModelAttr),
  TargetAttr(TargetAttr),
  TargetClonesAttr(TargetClonesAttr),
  TargetVersionAttr(TargetVersionAttr),
  TestTypestateAttr(TestTypestateAttr),
  ThisCallAttr(ThisCallAttr),
  TransparentUnionAttr(TransparentUnionAttr),
  TrivialABIAttr(TrivialABIAttr),
  TryAcquireCapabilityAttr(TryAcquireCapabilityAttr),
  TypeTagForDatatypeAttr(TypeTagForDatatypeAttr),
  TypeVisibilityAttr(TypeVisibilityAttr),
  UnavailableAttr(UnavailableAttr),
  UninitializedAttr(UninitializedAttr),
  UnsafeBufferUsageAttr(UnsafeBufferUsageAttr),
  UnusedAttr(UnusedAttr),
  UseHandleAttr(inheritable_param_attr_node::UseHandleAttr),
  UsedAttr(UsedAttr),
  UsingIfExistsAttr(UsingIfExistsAttr),
  UuidAttr(UuidAttr),
  VecReturnAttr(VecReturnAttr),
  VecTypeHintAttr(VecTypeHintAttr),
  VectorCallAttr(VectorCallAttr),
  VisibilityAttr(VisibilityAttr),
  WarnUnusedAttr(WarnUnusedAttr),
  WarnUnusedResultAttr(WarnUnusedResultAttr),
  WeakAttr(WeakAttr),
  WeakImportAttr(WeakImportAttr),
  WeakRefAttr(WeakRefAttr),
  WebAssemblyExportNameAttr(WebAssemblyExportNameAttr),
  WebAssemblyImportModuleAttr(WebAssemblyImportModuleAttr),
  WebAssemblyImportNameAttr(WebAssemblyImportNameAttr),
  WorkGroupSizeHintAttr(WorkGroupSizeHintAttr),
  X86ForceAlignArgPointerAttr(X86ForceAlignArgPointerAttr),
  XRayInstrumentAttr(XRayInstrumentAttr),
  XRayLogArgsAttr(XRayLogArgsAttr),
  ZeroCallUsedRegsAttr(ZeroCallUsedRegsAttr),
}

