# PUBLIC API CATALOG
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: APPROVED PUBLIC API REFERENCE  
**Author**: AG (Chief Software Architect)  

---

## 1. Executive Summary

This catalog documents every public API exposed across all **14 workspace packages** of **Siragugal Film Studio**. All APIs listed herein are frozen and classified as **STABLE**.

---

## 2. Package Catalog

### 1. `@sira/core-types` (TypeScript Package)
- **Exported Symbols**: `SiraTimecode`, `FRAME_RATES`, `SiraErrorCode`, `SiraError`, `SiraResult<T>`, `FeatureFlagManager`, `ProjectId`, `SceneId`, `AssetId`, `CharacterId`, `WorkflowId`, `RenderJobId`.
- **Stability**: STABLE (SemVer `0.1.0-alpha`).

### 2. `sira_types` (Rust Crate)
- **Exported Structs**: `SiraTimecode`, `RationalFrameRate`, `SiraError`, `SiraErrorCode`, `SiraResult<T>`, `FeatureFlagManager`, `ProjectId`, `SceneId`, `AssetId`, `CharacterId`, `WorkflowId`, `RenderJobId`.
- **Stability**: STABLE (SemVer `0.1.0`).

### 3. `sira_config` (Rust Crate)
- **Exported Structs**: `SiraConfig`, `HalConfig`, `RenderConfig`, `LoggingConfig`, `ConfigDiagnostics`, `ConfigObserverBus`.
- **Exported Functions**: `resolve_configuration(path, cli_args)`.
- **Stability**: STABLE.

### 4. `sira_diagnostics` (Rust Crate)
- **Exported Structs**: `TraceContext`, `RedactionEngine`, `SiraLogEvent`, `HealthReport`, `CrashCategory`.
- **Exported Functions**: `log_event()`, `enforce_log_cleanup_policy()`, `generate_support_bundle()`, `register_panic_hook()`.
- **Stability**: STABLE.

### 5. `sira_settings` (Rust Crate)
- **Exported Structs**: `SiraSettings`, `AppearanceSettings`, `AccessibilitySettings`, `AudioSettings`, `SettingsTransaction`, `SettingChangeEvent`, `SettingsObserverBus`, `SettingPolicyLocks`.
- **Exported Methods**: `SettingsStorage::load_from_file()`, `SettingsStorage::save_atomic()`.
- **Stability**: STABLE.

### 6. `sfsp_engine` (Rust Crate)
- **Exported Structs**: `SfspProject`, `SfspManifest`, `SchemaVersions`, `PackageIntegrity`, `ProjectLock`.
- **Exported Methods**: `SfspProject::create()`, `SfspProject::open()`, `compute_sha256()`, `package_sfsp_bundle()`.
- **Stability**: STABLE (Format Backward Compatibility 1.x).

### 7. `asset_db` (Rust Crate)
- **Exported Structs**: `UniversalAssetRecord`, `RelationshipRecord`, `AssetTypeRegistry`, `AssetLifecycleStatus`, `RelationshipOntology`, `AssetMutationEvent`, `FtsQueryFilter`.
- **Exported SQL Constants**: `CREATE_ASSETS_TABLE_SQL`, `CREATE_RELATIONSHIPS_TABLE_SQL`.
- **Stability**: STABLE.

### 8. `sira_hal` (Rust / C++20 Crate)
- **Exported Structs**: `HalDeviceInfo`, `DeviceCapabilities`, `DeviceCapabilityRegistry`, `HalBufferHandle`, `MemoryTier`, `HalQueue`, `QueueType`, `HalTelemetrySnapshot`.
- **Exported Functions**: `run_hal_conformance_suite()`, C ABI `sira_hal_enumerate_devices_native()`.
- **Stability**: STABLE.

### 9. `sira_core` (Rust Crate)
- **Exported Structs**: `SiraJob`, `ResourceContract`, `PriorityPolicy`, `JobState`, `AICapability`, `MultiTierScheduler`, `WorkflowCheckpoint`, `CancellationToken`, `SiraCoreEvent`, `SubEngineManager`, `CoreTelemetrySnapshot`.
- **Stability**: STABLE.

### 10. `sira_ai_provider` (Rust Crate)
- **Exported Structs/Traits**: `AiProvider` (async trait), `ProviderManifest`, `AIModelInfo`, `AIRequest`, `AIResponse`, `AIStreamChunk`, `AIUsage`, `ProviderRegistry`, `ModelRegistry`, `ProviderRouter`, `ProviderSecurityManager`, `ProviderBenchmarkReport`, `MockProvider`.
- **Stability**: STABLE.

### 11. `workflow_engine` (Rust Crate)
- **Exported Structs**: `NodeContract`, `NodeCategory`, `CanonicalDataType`, `WorkflowNode`, `NodePort`, `WorkflowEdge`, `DagValidator`, `ResourceAwareScheduler`, `WorkflowExecutionCheckpoint`, `SfswMarketplacePackage`, `ExecutionTarget`, `WorkflowExecutionSummary`.
- **Stability**: STABLE.

### 12. `plugin_runtime` (Rust Crate)
- **Exported Structs**: `PluginSdkVersionInfo`, `ExpandedPluginManifest`, `PublisherMetadata`, `ResourceQuotas`, `PluginLifecycleState`, `PluginCapabilityRegistry`, `PluginPermissionValidator`, `QuotaEnforcer`, `DependencyResolver`, `DigitalSignatureVerifier`, `HostApiModuleGroup`, `PluginEventBus`, `WasmPluginSandbox`.
- **Stability**: STABLE.

### 13. `resource_manager` (Rust Crate)
- **Exported Structs**: `ResourceReservation`, `ResourceSpec`, `ResourceLease`, `LeaseState`, `VramPool`, `RamPool`, `MemoryPressureLevel`, `CpuThreadPoolAllocator`, `PredictiveResourceEstimator`, `MultiGpuPool`, `ResourceTelemetrySnapshot`, `ResourcePolicies`, `LruEvictionEngine`.
- **Stability**: STABLE.

### 14. `cache_manager` (Rust Crate)
- **Exported Structs**: `CacheCategory`, `CacheMetadataRecord`, `CacheIndexDb`, `EvictionPolicy`, `SmartEvictionEngine`, `CacheMaintenanceService`, `ModelResidencyManager`, `RamCacheTier`, `DiskCacheTier`, `CacheTelemetrySnapshot`, `CacheRecoveryEngine`.
- **Stability**: STABLE.
