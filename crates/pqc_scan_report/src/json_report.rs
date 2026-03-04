use std::path::Path;

use anyhow::{Context, Result};

use pqc_scan_core::ScanResult;

pub fn write_json_report(result: &ScanResult, path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(result)?;
    std::fs::write(path, json).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

pub fn write_cbom(result: &ScanResult, path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(&result.cbom)?;
    std::fs::write(path, json).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

pub fn write_dependency_sbom(result: &ScanResult, path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(&result.dependency_sbom)?;
    std::fs::write(path, json).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}
