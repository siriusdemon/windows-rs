#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DisplayAdapter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayAdapter {}
impl ::core::clone::Clone for DisplayAdapter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayBitsPerChannel(pub u32);
impl DisplayBitsPerChannel {
    pub const None: Self = Self(0u32);
    pub const Bpc6: Self = Self(1u32);
    pub const Bpc8: Self = Self(2u32);
    pub const Bpc10: Self = Self(4u32);
    pub const Bpc12: Self = Self(8u32);
    pub const Bpc14: Self = Self(16u32);
    pub const Bpc16: Self = Self(32u32);
}
impl ::core::marker::Copy for DisplayBitsPerChannel {}
impl ::core::clone::Clone for DisplayBitsPerChannel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayDevice {}
impl ::core::clone::Clone for DisplayDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayDeviceCapability(pub i32);
impl DisplayDeviceCapability {
    pub const FlipOverride: Self = Self(0i32);
}
impl ::core::marker::Copy for DisplayDeviceCapability {}
impl ::core::clone::Clone for DisplayDeviceCapability {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayFence(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayFence {}
impl ::core::clone::Clone for DisplayFence {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayManager {}
impl ::core::clone::Clone for DisplayManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayManagerChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayManagerChangedEventArgs {}
impl ::core::clone::Clone for DisplayManagerChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayManagerDisabledEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayManagerDisabledEventArgs {}
impl ::core::clone::Clone for DisplayManagerDisabledEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayManagerEnabledEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayManagerEnabledEventArgs {}
impl ::core::clone::Clone for DisplayManagerEnabledEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayManagerOptions(pub u32);
impl DisplayManagerOptions {
    pub const None: Self = Self(0u32);
    pub const EnforceSourceOwnership: Self = Self(1u32);
    pub const VirtualRefreshRateAware: Self = Self(2u32);
}
impl ::core::marker::Copy for DisplayManagerOptions {}
impl ::core::clone::Clone for DisplayManagerOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayManagerPathsFailedOrInvalidatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayManagerPathsFailedOrInvalidatedEventArgs {}
impl ::core::clone::Clone for DisplayManagerPathsFailedOrInvalidatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayManagerResult(pub i32);
impl DisplayManagerResult {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const TargetAccessDenied: Self = Self(2i32);
    pub const TargetStale: Self = Self(3i32);
    pub const RemoteSessionNotSupported: Self = Self(4i32);
}
impl ::core::marker::Copy for DisplayManagerResult {}
impl ::core::clone::Clone for DisplayManagerResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayManagerResultWithState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayManagerResultWithState {}
impl ::core::clone::Clone for DisplayManagerResultWithState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayModeInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayModeInfo {}
impl ::core::clone::Clone for DisplayModeInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayModeQueryOptions(pub u32);
impl DisplayModeQueryOptions {
    pub const None: Self = Self(0u32);
    pub const OnlyPreferredResolution: Self = Self(1u32);
}
impl ::core::marker::Copy for DisplayModeQueryOptions {}
impl ::core::clone::Clone for DisplayModeQueryOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayPath(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayPath {}
impl ::core::clone::Clone for DisplayPath {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayPathScaling(pub i32);
impl DisplayPathScaling {
    pub const Identity: Self = Self(0i32);
    pub const Centered: Self = Self(1i32);
    pub const Stretched: Self = Self(2i32);
    pub const AspectRatioStretched: Self = Self(3i32);
    pub const Custom: Self = Self(4i32);
    pub const DriverPreferred: Self = Self(5i32);
}
impl ::core::marker::Copy for DisplayPathScaling {}
impl ::core::clone::Clone for DisplayPathScaling {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayPathStatus(pub i32);
impl DisplayPathStatus {
    pub const Unknown: Self = Self(0i32);
    pub const Succeeded: Self = Self(1i32);
    pub const Pending: Self = Self(2i32);
    pub const Failed: Self = Self(3i32);
    pub const FailedAsync: Self = Self(4i32);
    pub const InvalidatedAsync: Self = Self(5i32);
}
impl ::core::marker::Copy for DisplayPathStatus {}
impl ::core::clone::Clone for DisplayPathStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayPresentStatus(pub i32);
impl DisplayPresentStatus {
    pub const Success: Self = Self(0i32);
    pub const SourceStatusPreventedPresent: Self = Self(1i32);
    pub const ScanoutInvalid: Self = Self(2i32);
    pub const SourceInvalid: Self = Self(3i32);
    pub const DeviceInvalid: Self = Self(4i32);
    pub const UnknownFailure: Self = Self(5i32);
}
impl ::core::marker::Copy for DisplayPresentStatus {}
impl ::core::clone::Clone for DisplayPresentStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
pub struct DisplayPresentationRate {
    pub VerticalSyncRate: super::super::super::Foundation::Numerics::Rational,
    pub VerticalSyncsPerPresentation: i32,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for DisplayPresentationRate {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for DisplayPresentationRate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayPrimaryDescription(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayPrimaryDescription {}
impl ::core::clone::Clone for DisplayPrimaryDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayRotation(pub i32);
impl DisplayRotation {
    pub const None: Self = Self(0i32);
    pub const Clockwise90Degrees: Self = Self(1i32);
    pub const Clockwise180Degrees: Self = Self(2i32);
    pub const Clockwise270Degrees: Self = Self(3i32);
}
impl ::core::marker::Copy for DisplayRotation {}
impl ::core::clone::Clone for DisplayRotation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayScanout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayScanout {}
impl ::core::clone::Clone for DisplayScanout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayScanoutOptions(pub u32);
impl DisplayScanoutOptions {
    pub const None: Self = Self(0u32);
    pub const AllowTearing: Self = Self(2u32);
}
impl ::core::marker::Copy for DisplayScanoutOptions {}
impl ::core::clone::Clone for DisplayScanoutOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplaySource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplaySource {}
impl ::core::clone::Clone for DisplaySource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplaySourceStatus(pub i32);
impl DisplaySourceStatus {
    pub const Active: Self = Self(0i32);
    pub const PoweredOff: Self = Self(1i32);
    pub const Invalid: Self = Self(2i32);
    pub const OwnedByAnotherDevice: Self = Self(3i32);
    pub const Unowned: Self = Self(4i32);
}
impl ::core::marker::Copy for DisplaySourceStatus {}
impl ::core::clone::Clone for DisplaySourceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayState {}
impl ::core::clone::Clone for DisplayState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayStateApplyOptions(pub u32);
impl DisplayStateApplyOptions {
    pub const None: Self = Self(0u32);
    pub const FailIfStateChanged: Self = Self(1u32);
    pub const ForceReapply: Self = Self(2u32);
    pub const ForceModeEnumeration: Self = Self(4u32);
}
impl ::core::marker::Copy for DisplayStateApplyOptions {}
impl ::core::clone::Clone for DisplayStateApplyOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayStateFunctionalizeOptions(pub u32);
impl DisplayStateFunctionalizeOptions {
    pub const None: Self = Self(0u32);
    pub const FailIfStateChanged: Self = Self(1u32);
    pub const ValidateTopologyOnly: Self = Self(2u32);
}
impl ::core::marker::Copy for DisplayStateFunctionalizeOptions {}
impl ::core::clone::Clone for DisplayStateFunctionalizeOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayStateOperationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayStateOperationResult {}
impl ::core::clone::Clone for DisplayStateOperationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayStateOperationStatus(pub i32);
impl DisplayStateOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const PartialFailure: Self = Self(1i32);
    pub const UnknownFailure: Self = Self(2i32);
    pub const TargetOwnershipLost: Self = Self(3i32);
    pub const SystemStateChanged: Self = Self(4i32);
    pub const TooManyPathsForAdapter: Self = Self(5i32);
    pub const ModesNotSupported: Self = Self(6i32);
    pub const RemoteSessionNotSupported: Self = Self(7i32);
}
impl ::core::marker::Copy for DisplayStateOperationStatus {}
impl ::core::clone::Clone for DisplayStateOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplaySurface(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplaySurface {}
impl ::core::clone::Clone for DisplaySurface {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayTarget(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayTarget {}
impl ::core::clone::Clone for DisplayTarget {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayTargetPersistence(pub i32);
impl DisplayTargetPersistence {
    pub const None: Self = Self(0i32);
    pub const BootPersisted: Self = Self(1i32);
    pub const TemporaryPersisted: Self = Self(2i32);
    pub const PathPersisted: Self = Self(3i32);
}
impl ::core::marker::Copy for DisplayTargetPersistence {}
impl ::core::clone::Clone for DisplayTargetPersistence {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayTask(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayTask {}
impl ::core::clone::Clone for DisplayTask {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayTaskPool(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayTaskPool {}
impl ::core::clone::Clone for DisplayTaskPool {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayTaskResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayTaskResult {}
impl ::core::clone::Clone for DisplayTaskResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayTaskSignalKind(pub i32);
impl DisplayTaskSignalKind {
    pub const OnPresentFlipAway: Self = Self(0i32);
    pub const OnPresentFlipTo: Self = Self(1i32);
}
impl ::core::marker::Copy for DisplayTaskSignalKind {}
impl ::core::clone::Clone for DisplayTaskSignalKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayView {}
impl ::core::clone::Clone for DisplayView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayWireFormat(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayWireFormat {}
impl ::core::clone::Clone for DisplayWireFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayWireFormatColorSpace(pub i32);
impl DisplayWireFormatColorSpace {
    pub const BT709: Self = Self(0i32);
    pub const BT2020: Self = Self(1i32);
    pub const ProfileDefinedWideColorGamut: Self = Self(2i32);
}
impl ::core::marker::Copy for DisplayWireFormatColorSpace {}
impl ::core::clone::Clone for DisplayWireFormatColorSpace {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayWireFormatEotf(pub i32);
impl DisplayWireFormatEotf {
    pub const Sdr: Self = Self(0i32);
    pub const HdrSmpte2084: Self = Self(1i32);
}
impl ::core::marker::Copy for DisplayWireFormatEotf {}
impl ::core::clone::Clone for DisplayWireFormatEotf {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayWireFormatHdrMetadata(pub i32);
impl DisplayWireFormatHdrMetadata {
    pub const None: Self = Self(0i32);
    pub const Hdr10: Self = Self(1i32);
    pub const Hdr10Plus: Self = Self(2i32);
    pub const DolbyVisionLowLatency: Self = Self(3i32);
}
impl ::core::marker::Copy for DisplayWireFormatHdrMetadata {}
impl ::core::clone::Clone for DisplayWireFormatHdrMetadata {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayWireFormatPixelEncoding(pub i32);
impl DisplayWireFormatPixelEncoding {
    pub const Rgb444: Self = Self(0i32);
    pub const Ycc444: Self = Self(1i32);
    pub const Ycc422: Self = Self(2i32);
    pub const Ycc420: Self = Self(3i32);
    pub const Intensity: Self = Self(4i32);
}
impl ::core::marker::Copy for DisplayWireFormatPixelEncoding {}
impl ::core::clone::Clone for DisplayWireFormatPixelEncoding {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayAdapter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayAdapter {}
impl ::core::clone::Clone for IDisplayAdapter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayAdapterStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayAdapterStatics {}
impl ::core::clone::Clone for IDisplayAdapterStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayDevice {}
impl ::core::clone::Clone for IDisplayDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayDevice2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayDevice2 {}
impl ::core::clone::Clone for IDisplayDevice2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayFence(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayFence {}
impl ::core::clone::Clone for IDisplayFence {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayManager {}
impl ::core::clone::Clone for IDisplayManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayManagerChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayManagerChangedEventArgs {}
impl ::core::clone::Clone for IDisplayManagerChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayManagerDisabledEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayManagerDisabledEventArgs {}
impl ::core::clone::Clone for IDisplayManagerDisabledEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayManagerEnabledEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayManagerEnabledEventArgs {}
impl ::core::clone::Clone for IDisplayManagerEnabledEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayManagerPathsFailedOrInvalidatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayManagerPathsFailedOrInvalidatedEventArgs {}
impl ::core::clone::Clone for IDisplayManagerPathsFailedOrInvalidatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayManagerResultWithState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayManagerResultWithState {}
impl ::core::clone::Clone for IDisplayManagerResultWithState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayManagerStatics {}
impl ::core::clone::Clone for IDisplayManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayModeInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayModeInfo {}
impl ::core::clone::Clone for IDisplayModeInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayModeInfo2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayModeInfo2 {}
impl ::core::clone::Clone for IDisplayModeInfo2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayPath(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayPath {}
impl ::core::clone::Clone for IDisplayPath {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayPath2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayPath2 {}
impl ::core::clone::Clone for IDisplayPath2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayPrimaryDescription(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayPrimaryDescription {}
impl ::core::clone::Clone for IDisplayPrimaryDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayPrimaryDescriptionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayPrimaryDescriptionFactory {}
impl ::core::clone::Clone for IDisplayPrimaryDescriptionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayPrimaryDescriptionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayPrimaryDescriptionStatics {}
impl ::core::clone::Clone for IDisplayPrimaryDescriptionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayScanout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayScanout {}
impl ::core::clone::Clone for IDisplayScanout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplaySource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplaySource {}
impl ::core::clone::Clone for IDisplaySource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplaySource2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplaySource2 {}
impl ::core::clone::Clone for IDisplaySource2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayState {}
impl ::core::clone::Clone for IDisplayState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayStateOperationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayStateOperationResult {}
impl ::core::clone::Clone for IDisplayStateOperationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplaySurface(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplaySurface {}
impl ::core::clone::Clone for IDisplaySurface {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayTarget(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayTarget {}
impl ::core::clone::Clone for IDisplayTarget {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayTask(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayTask {}
impl ::core::clone::Clone for IDisplayTask {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayTask2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayTask2 {}
impl ::core::clone::Clone for IDisplayTask2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayTaskPool(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayTaskPool {}
impl ::core::clone::Clone for IDisplayTaskPool {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayTaskPool2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayTaskPool2 {}
impl ::core::clone::Clone for IDisplayTaskPool2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayTaskResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayTaskResult {}
impl ::core::clone::Clone for IDisplayTaskResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayView {}
impl ::core::clone::Clone for IDisplayView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayWireFormat(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayWireFormat {}
impl ::core::clone::Clone for IDisplayWireFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayWireFormatFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayWireFormatFactory {}
impl ::core::clone::Clone for IDisplayWireFormatFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayWireFormatStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayWireFormatStatics {}
impl ::core::clone::Clone for IDisplayWireFormatStatics {
    fn clone(&self) -> Self {
        *self
    }
}