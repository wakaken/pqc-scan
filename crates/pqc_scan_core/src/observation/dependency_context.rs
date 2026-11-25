//! Finite parser assertions, not metadata-driven upgrades or new stored fields.
use super::ObservationDraft;

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum DependencyContext {
    PlatformRequirement,
    PlatformDevelopment,
    PlatformEmulated,
    PlatformDisabled,
    SdkDeclared,
    FrameworkDeclared,
    FrameworkLockReported,
    BaselineDeclared,
    FeatureDeclared,
    OverrideDeclared,
    Conditional,
    Development,
    BuildRequirement,
    LexicalReference,
    LegacyDeclaration,
    CentralInherited,
}
impl ObservationDraft {
    pub fn with_dependency_context(mut self, context: DependencyContext) -> Self {
        if !self.typed_dependency || self.assurance != "parsed" {
            return self;
        }
        let Some(dep) = &mut self.dependency else {
            return self;
        };
        use DependencyContext::*;
        let provenance = match context {
            PlatformRequirement => "platform_required",
            PlatformDevelopment => "platform_dev_required",
            PlatformEmulated => "platform_emulated",
            PlatformDisabled => "platform_emulation_disabled",
            SdkDeclared => "sdk_declared",
            FrameworkDeclared => "target_framework_declared",
            FrameworkLockReported => "lock_framework_reported",
            BaselineDeclared => "registry_baseline_declared",
            FeatureDeclared => "feature_declared",
            OverrideDeclared => "override_declared",
            Conditional => "conditional_declaration_unresolved",
            BuildRequirement => match dep.version_provenance.as_str() {
                "lock_record_unverified" => "build_lock_record_unverified",
                "source_unresolved" => "build_source_unresolved",
                _ => "build_requirement_unresolved",
            },
            LexicalReference => "lexical_build_reference",
            LegacyDeclaration => "legacy_package_declared",
            CentralInherited => "central_literal_inherited_not_effective",
            Development => match dep.version_provenance.as_str() {
                "path_dependency_unresolved" => "dev_path_dependency_unresolved",
                "git_dependency_unresolved" => "dev_git_dependency_unresolved",
                "archive_dependency_unresolved" => "dev_archive_dependency_unresolved",
                "lock_record_unverified" => "dev_lock_record_unverified",
                "source_unresolved" => "dev_source_unresolved",
                _ => "manifest_dev_declared",
            },
        };
        dep.version_provenance = provenance.into();
        // Context never confers resolution, installed software or runtime assurance.
        dep.resolved_version = None;
        self.evidence.capture_status = match context {
            Conditional => "conditional_declaration_not_effective",
            CentralInherited => "central_literal_not_effective",
            LexicalReference => "lexical_build_reference_not_evaluated",
            _ => "dependency_context_recorded_not_installed",
        }
        .into();
        self
    }
}
