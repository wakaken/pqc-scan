use std::collections::{BTreeMap, BTreeSet};
use std::ffi::CString;
use std::fs::File;
use std::io::{Read, Write};
use std::os::fd::{AsRawFd, FromRawFd};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::MetadataExt;
use std::path::{Component, Path, PathBuf};
use std::sync::{Arc, Mutex};

use anyhow::{ensure, Result};
use pqc_scan_core::{Detection, Detector, DetectorOutcome, DetectorStatus, ScannableFile};
use pqc_scan_rules::{RuleKind, RuleSet};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const CONFIG_LIMIT: u64 = 64 * 1024;
const OUTPUT_LIMIT: usize = 128 * 1024 * 1024;
const DATA_LIMIT: usize = 120 * 1024 * 1024;
const ROW_LIMIT: usize = 500_000;
const WITNESS_ID: &str = "TEMPORAL_PRIVATE_VALIDITY_WITNESS";
const WITNESS_PATTERN: &str = "^(?:valid|not_yet_valid|expired)$";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Selection {
    include_non_scannable: bool,
    exclude_tests: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Bindings {
    source: String,
    rules: String,
    input: String,
    selection: String,
    tool: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RuleFile {
    path: String,
    sha256: String,
    bytes: u64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    #[serde(default)]
    kind: Option<String>,
    #[serde(default)]
    manifest_projection: Option<ArtifactBinding>,
    #[serde(default)]
    original_diagnostic_projection: Option<ArtifactBinding>,
    #[serde(default)]
    boundary_policy_relation: Option<ArtifactBinding>,
    root: PathBuf,
    max_file_size_bytes: usize,
    threads: usize,
    selection: Selection,
    detector_second: i64,
    rules_dir: PathBuf,
    witness_rule_id: String,
    rule_files: Vec<RuleFile>,
    output: PathBuf,
    bindings: Bindings,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ArtifactBinding {
    path: PathBuf,
    sha256: String,
    bytes: u64,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PinnedLink {
    path: String,
    manifest_entry_sha256: String,
    link_metadata_sha256: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ManifestProjection {
    kind: String,
    manifest: ArtifactBinding,
    entries: Vec<PinnedLink>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DiagnosticIdentity {
    code: String,
    path: Option<String>,
    detector: Option<String>,
    occurrences: usize,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct DiagnosticProjection {
    kind: String,
    phase_id: String,
    original_terminal: ArtifactBinding,
    diagnostics: ArtifactBinding,
    entries: Vec<DiagnosticIdentity>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct BoundaryReceipt {
    path: String,
    reason: &'static str,
    manifest_entry_sha256: String,
    link_metadata_sha256: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Identity {
    device: u64,
    inode: u64,
    mode: u32,
    ctime: i64,
    ctime_nsec: i64,
}

impl Identity {
    fn directory(file: &File) -> Result<Self> {
        let m = file.metadata()?;
        ensure!(m.is_dir(), "E_WITNESS_BOUNDARY_TYPE");
        Ok(Self {
            device: m.dev(),
            inode: m.ino(),
            mode: m.mode(),
            ctime: m.ctime(),
            ctime_nsec: m.ctime_nsec(),
        })
    }
}

struct HeldBoundary {
    pinned: PinnedLink,
    chain: Vec<File>,
    identities: Vec<Identity>,
    leaf: Identity,
}

fn boundary_path(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 4096
        && !value.starts_with('/')
        && !value.chars().any(|c| c.is_control())
        && value
            .split('/')
            .all(|part| !part.is_empty() && part != "." && part != "..")
}

fn link_metadata_hash(path: &str, link: &[u8]) -> Result<String> {
    ensure!(
        boundary_path(path)
            && !link.is_empty()
            && link.len() <= 4096
            && !link.contains(&0)
            && std::str::from_utf8(link).is_ok(),
        "E_WITNESS_BOUNDARY_METADATA"
    );
    let mut digest = Sha256::new();
    for value in [
        b"pqc-temporal-link-v2".as_slice(),
        path.as_bytes(),
        b"120000".as_slice(),
        link,
    ] {
        digest.update((value.len() as u64).to_be_bytes());
        digest.update(value);
    }
    Ok(format!("{:x}", digest.finalize()))
}

fn held_chain(path: &Path) -> Result<Vec<File>> {
    ensure!(path.is_absolute(), "E_WITNESS_PATH");
    let mut chain = vec![directory(Path::new("/"))?];
    for component in path.components() {
        match component {
            Component::RootDir => (),
            Component::Normal(name) => {
                let file = open_at(
                    chain.last().unwrap(),
                    &component_name(Path::new(name))?,
                    libc::O_RDONLY | libc::O_DIRECTORY,
                    0,
                )?;
                chain.push(file);
            }
            _ => anyhow::bail!("E_WITNESS_PATH"),
        }
    }
    Ok(chain)
}

fn leaf_identity(parent: &File, name: &CString) -> Result<Identity> {
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    ensure!(
        unsafe {
            libc::fstatat(
                parent.as_raw_fd(),
                name.as_ptr(),
                &mut stat,
                libc::AT_SYMLINK_NOFOLLOW,
            )
        } == 0
            && (stat.st_mode as u32 & libc::S_IFMT as u32) == libc::S_IFLNK as u32,
        "E_WITNESS_BOUNDARY_TYPE"
    );
    Ok(Identity {
        device: stat.st_dev as u64,
        inode: stat.st_ino as u64,
        mode: stat.st_mode as u32,
        ctime: stat.st_ctime as i64,
        ctime_nsec: stat.st_ctime_nsec as i64,
    })
}

fn observe_link(parent: &File, path: &str) -> Result<(Identity, String)> {
    let leaf = Path::new(path)
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("E_WITNESS_PATH"))?;
    let name = component_name(Path::new(leaf))?;
    let before = leaf_identity(parent, &name)?;
    let mut link = [0u8; 4097];
    let length = unsafe {
        libc::readlinkat(
            parent.as_raw_fd(),
            name.as_ptr(),
            link.as_mut_ptr().cast(),
            link.len(),
        )
    };
    ensure!(length > 0 && length <= 4096, "E_WITNESS_BOUNDARY_METADATA");
    let digest = link_metadata_hash(path, &link[..length as usize])?;
    ensure!(
        before == leaf_identity(parent, &name)?,
        "E_WITNESS_BOUNDARY_CHANGED"
    );
    Ok((before, digest))
}

impl HeldBoundary {
    fn capture(root: &Path, pinned: PinnedLink) -> Result<Self> {
        ensure!(
            boundary_path(&pinned.path)
                && valid_hash(&pinned.manifest_entry_sha256)
                && valid_hash(&pinned.link_metadata_sha256),
            "E_WITNESS_BOUNDARY_PROJECTION"
        );
        let full = root.join(&pinned.path);
        let chain = held_chain(
            full.parent()
                .ok_or_else(|| anyhow::anyhow!("E_WITNESS_PATH"))?,
        )?;
        let identities = chain
            .iter()
            .map(Identity::directory)
            .collect::<Result<Vec<_>>>()?;
        let (leaf, digest) = observe_link(chain.last().unwrap(), &pinned.path)?;
        ensure!(
            digest == pinned.link_metadata_sha256,
            "E_WITNESS_BOUNDARY_METADATA"
        );
        Ok(Self {
            pinned,
            chain,
            identities,
            leaf,
        })
    }

    fn verify(&self, root: &Path) -> Result<()> {
        let full = root.join(&self.pinned.path);
        let current = held_chain(
            full.parent()
                .ok_or_else(|| anyhow::anyhow!("E_WITNESS_PATH"))?,
        )?;
        let identities = current
            .iter()
            .map(Identity::directory)
            .collect::<Result<Vec<_>>>()?;
        ensure!(
            identities == self.identities
                && self
                    .chain
                    .iter()
                    .map(Identity::directory)
                    .collect::<Result<Vec<_>>>()?
                    == self.identities,
            "E_WITNESS_BOUNDARY_CHANGED"
        );
        for parent in [self.chain.last().unwrap(), current.last().unwrap()] {
            let (leaf, digest) = observe_link(parent, &self.pinned.path)?;
            ensure!(
                leaf == self.leaf && digest == self.pinned.link_metadata_sha256,
                "E_WITNESS_BOUNDARY_CHANGED"
            );
        }
        Ok(())
    }

    fn receipt(&self) -> BoundaryReceipt {
        BoundaryReceipt {
            path: self.pinned.path.clone(),
            reason: "nofollow_symlink",
            manifest_entry_sha256: self.pinned.manifest_entry_sha256.clone(),
            link_metadata_sha256: self.pinned.link_metadata_sha256.clone(),
        }
    }
}

fn projection_bytes(binding: &ArtifactBinding) -> Result<Vec<u8>> {
    ensure!(
        valid_hash(&binding.sha256) && binding.bytes <= 64 * 1024 * 1024,
        "E_WITNESS_BOUNDARY_PROJECTION"
    );
    let parent = directory(
        binding
            .path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("E_WITNESS_PATH"))?,
    )?;
    let raw = read_at(
        &parent,
        Path::new(
            binding
                .path
                .file_name()
                .ok_or_else(|| anyhow::anyhow!("E_WITNESS_PATH"))?,
        ),
        64 * 1024 * 1024,
    )?;
    ensure!(
        raw.len() as u64 == binding.bytes && hash(&raw) == binding.sha256,
        "E_WITNESS_BOUNDARY_PROJECTION"
    );
    Ok(raw)
}

fn capture_boundaries(config: &Config) -> Result<Option<BTreeMap<String, HeldBoundary>>> {
    match config.kind.as_deref() {
        None => {
            ensure!(
                config.manifest_projection.is_none()
                    && config.original_diagnostic_projection.is_none()
                    && config.boundary_policy_relation.is_none(),
                "E_WITNESS_CONFIG"
            );
            Ok(None)
        }
        Some("temporal-witness-config-v2") => {
            let manifest: ManifestProjection = serde_json::from_slice(&projection_bytes(
                config
                    .manifest_projection
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("E_WITNESS_CONFIG"))?,
            )?)?;
            let diagnostics: DiagnosticProjection = serde_json::from_slice(&projection_bytes(
                config
                    .original_diagnostic_projection
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("E_WITNESS_CONFIG"))?,
            )?)?;
            projection_bytes(
                config
                    .boundary_policy_relation
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("E_WITNESS_CONFIG"))?,
            )?;
            ensure!(
                manifest.kind == "temporal-manifest-link-projection-v2"
                    && manifest.manifest.sha256 == config.bindings.input
                    && valid_hash(&manifest.manifest.sha256)
                    && manifest.entries.len() <= ROW_LIMIT
                    && diagnostics.entries.len() <= ROW_LIMIT
                    && diagnostics.kind == "temporal-original-diagnostic-projection-v2"
                    && matches!(
                        diagnostics.phase_id.as_str(),
                        "primary-after" | "comparison-after"
                    )
                    && valid_hash(&diagnostics.original_terminal.sha256)
                    && valid_hash(&diagnostics.diagnostics.sha256),
                "E_WITNESS_BOUNDARY_PROJECTION"
            );
            let mut pinned = BTreeMap::new();
            for row in manifest.entries {
                ensure!(
                    boundary_path(&row.path)
                        && valid_hash(&row.manifest_entry_sha256)
                        && valid_hash(&row.link_metadata_sha256)
                        && pinned.insert(row.path.clone(), row).is_none(),
                    "E_WITNESS_BOUNDARY_PROJECTION"
                );
            }
            let mut identities = BTreeSet::new();
            let mut held = BTreeMap::new();
            for row in diagnostics.entries {
                ensure!(
                    row.occurrences > 0
                        && row.path.as_ref().is_none_or(|p| boundary_path(p))
                        && row.detector.as_ref().is_none_or(|d| !d.is_empty()
                            && d.len() <= 256
                            && d.bytes().all(|c| (32..127).contains(&c)))
                        && identities.insert((
                            row.code.clone(),
                            row.path.clone(),
                            row.detector.clone()
                        )),
                    "E_WITNESS_BOUNDARY_PROJECTION"
                );
                if row.code == "read_failed" {
                    ensure!(
                        row.occurrences == 1 && row.detector.is_none(),
                        "E_WITNESS_BOUNDARY_PROJECTION"
                    );
                    let path = row
                        .path
                        .ok_or_else(|| anyhow::anyhow!("E_WITNESS_BOUNDARY_PROJECTION"))?;
                    let entry = pinned
                        .get(&path)
                        .ok_or_else(|| anyhow::anyhow!("E_WITNESS_BOUNDARY_PROJECTION"))?;
                    held.insert(path, HeldBoundary::capture(&config.root, entry.clone())?);
                } else {
                    ensure!(
                        matches!(
                            row.code.as_str(),
                            "detector_unsupported"
                                | "detector_partial"
                                | "excluded_path"
                                | "file_too_large"
                        ) || matches!(
                            row.code.as_str(),
                            "detector_failed" | "detector_truncated" | "detector_not_reported"
                        ) && row
                            .detector
                            .as_ref()
                            .is_some_and(|d| d != "certificate_detector"),
                        "E_WITNESS_BOUNDARY_PROJECTION"
                    );
                }
            }
            Ok(Some(held))
        }
        _ => anyhow::bail!("E_WITNESS_CONFIG"),
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PublicObject {
    path: String,
    source_sha256: String,
    certificate_sha256: String,
    object_index: usize,
    not_before_unix: i64,
    not_after_unix: i64,
    validity_state: String,
    matching_default_validity_rule_ids: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FileRecord {
    path: String,
    source_sha256: String,
    status: &'static str,
    objects: usize,
}

#[derive(Default)]
struct Records {
    objects: Vec<PublicObject>,
    files: Vec<FileRecord>,
    encoded_bytes: usize,
    failure: bool,
}

struct Witness<D> {
    detector: D,
    root: PathBuf,
    records: Arc<Mutex<Records>>,
}

fn hash(raw: &[u8]) -> String {
    format!("{:x}", Sha256::digest(raw))
}

fn valid_hash(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

fn relative_path(root: &Path, file: &Path) -> Result<String> {
    let relative = file
        .strip_prefix(root)
        .map_err(|_| anyhow::anyhow!("E_WITNESS_PATH"))?;
    ensure!(
        relative
            .components()
            .all(|c| matches!(c, Component::Normal(_))),
        "E_WITNESS_PATH"
    );
    let text = relative
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("E_WITNESS_PATH"))?;
    ensure!(
        !text.is_empty() && text.len() <= 4096 && !text.chars().any(char::is_control),
        "E_WITNESS_PATH"
    );
    Ok(text.to_owned())
}

fn status_name(status: DetectorStatus) -> &'static str {
    match status {
        DetectorStatus::Complete => "complete",
        DetectorStatus::NotApplicable => "not_applicable",
        DetectorStatus::Unsupported => "unsupported",
        DetectorStatus::Partial => "partial",
        DetectorStatus::Truncated => "truncated",
        DetectorStatus::NotReported => "not_reported",
    }
}

fn metadata<'a>(detection: &'a Detection, key: &str) -> Result<&'a str> {
    detection
        .evidence
        .metadata
        .get(key)
        .map(String::as_str)
        .ok_or_else(|| anyhow::anyhow!("E_WITNESS_METADATA"))
}

fn integer(detection: &Detection, key: &str) -> Result<i64> {
    let raw = metadata(detection, key)?;
    let value: i64 = raw
        .parse()
        .map_err(|_| anyhow::anyhow!("E_WITNESS_METADATA"))?;
    ensure!(value.to_string() == raw, "E_WITNESS_METADATA");
    Ok(value)
}

fn capture(
    file: &ScannableFile,
    root: &Path,
    rules: &RuleSet,
    outcome: &DetectorOutcome,
) -> Result<(FileRecord, Vec<PublicObject>)> {
    ensure!(
        outcome.status() != DetectorStatus::Truncated
            && outcome.status() != DetectorStatus::NotReported,
        "E_WITNESS_PRODUCER_LOSS"
    );
    let path = relative_path(root, &file.path)?;
    let source_sha256 = hash(&file.bytes);
    let real_validity: BTreeSet<&str> = rules
        .by_kind(RuleKind::Certificate)
        .filter(|r| r.scope == "certificate_validity" && r.id != WITNESS_ID)
        .map(|r| r.id.as_str())
        .collect();
    let mut objects = BTreeMap::new();
    for detection in outcome
        .detections()
        .iter()
        .filter(|d| d.rule_id == WITNESS_ID)
    {
        ensure!(
            detection.location.file == file.path.to_string_lossy(),
            "E_WITNESS_PATH"
        );
        let certificate_sha256 = metadata(detection, "certificate_sha256")?.to_owned();
        ensure!(valid_hash(&certificate_sha256), "E_WITNESS_METADATA");
        let index = integer(detection, "object_index")?;
        ensure!(index > 0, "E_WITNESS_METADATA");
        let object_index =
            usize::try_from(index).map_err(|_| anyhow::anyhow!("E_WITNESS_METADATA"))?;
        let not_before_unix = integer(detection, "not_before_unix")?;
        let not_after_unix = integer(detection, "not_after_unix")?;
        let second = integer(detection, "assessment_time_unix")?;
        ensure!(not_before_unix <= not_after_unix, "E_WITNESS_METADATA");
        let expected = if second < not_before_unix {
            "not_yet_valid"
        } else if second > not_after_unix {
            "expired"
        } else {
            "valid"
        };
        ensure!(
            metadata(detection, "validity_status")? == expected,
            "E_WITNESS_METADATA"
        );
        ensure!(
            objects
                .insert(
                    object_index,
                    PublicObject {
                        path: path.clone(),
                        source_sha256: source_sha256.clone(),
                        certificate_sha256,
                        object_index,
                        not_before_unix,
                        not_after_unix,
                        validity_state: expected.into(),
                        matching_default_validity_rule_ids: Vec::new(),
                    }
                )
                .is_none(),
            "E_WITNESS_DUPLICATE_OBJECT"
        );
    }
    for detection in outcome
        .detections()
        .iter()
        .filter(|d| real_validity.contains(d.rule_id.as_str()))
    {
        let index = usize::try_from(integer(detection, "object_index")?)
            .map_err(|_| anyhow::anyhow!("E_WITNESS_METADATA"))?;
        let object = objects
            .get_mut(&index)
            .ok_or_else(|| anyhow::anyhow!("E_WITNESS_PRODUCER_LOSS"))?;
        ensure!(
            metadata(detection, "certificate_sha256")? == object.certificate_sha256
                && integer(detection, "not_before_unix")? == object.not_before_unix
                && integer(detection, "not_after_unix")? == object.not_after_unix
                && metadata(detection, "validity_status")? == object.validity_state,
            "E_WITNESS_METADATA"
        );
        object
            .matching_default_validity_rule_ids
            .push(detection.rule_id.clone());
    }
    let objects: Vec<_> = objects
        .into_values()
        .map(|mut object| {
            object.matching_default_validity_rule_ids.sort();
            object.matching_default_validity_rule_ids.dedup();
            object
        })
        .collect();
    Ok((
        FileRecord {
            path,
            source_sha256,
            status: status_name(outcome.status()),
            objects: objects.len(),
        },
        objects,
    ))
}

impl<D: Detector> Detector for Witness<D> {
    fn name(&self) -> &'static str {
        "certificate_detector"
    }

    fn detect(&self, file: &ScannableFile, rules: &RuleSet) -> Result<Vec<Detection>> {
        Ok(self.detect_detailed(file, rules)?.into_detections())
    }

    fn detect_detailed(&self, file: &ScannableFile, rules: &RuleSet) -> Result<DetectorOutcome> {
        let captured = self
            .detector
            .detect_detailed(file, rules)
            .and_then(|outcome| {
                capture(file, &self.root, rules, &outcome).map(|rows| (outcome.status(), rows))
            });
        let mut records = self
            .records
            .lock()
            .map_err(|_| anyhow::anyhow!("E_WITNESS_STATE"))?;
        let Ok((status, (record, objects))) = captured else {
            records.failure = true;
            anyhow::bail!("E_WITNESS_PRODUCER");
        };
        let bytes = serde_json::to_vec(&record)?.len() + serde_json::to_vec(&objects)?.len();
        if records.files.len() >= ROW_LIMIT
            || records.objects.len() + objects.len() > ROW_LIMIT
            || records
                .encoded_bytes
                .checked_add(bytes)
                .is_none_or(|n| n > DATA_LIMIT)
        {
            records.failure = true;
            anyhow::bail!("E_WITNESS_QUOTA");
        }
        records.encoded_bytes += bytes;
        records.files.push(record);
        records.objects.extend(objects);
        Ok(DetectorOutcome::new(Vec::new(), status))
    }
}

fn component_name(path: &Path) -> Result<CString> {
    CString::new(path.as_os_str().as_bytes()).map_err(|_| anyhow::anyhow!("E_WITNESS_PATH"))
}

fn open_at(parent: &File, name: &CString, flags: i32, mode: libc::mode_t) -> Result<File> {
    let fd = unsafe {
        libc::openat(
            parent.as_raw_fd(),
            name.as_ptr(),
            flags | libc::O_CLOEXEC | libc::O_NOFOLLOW | libc::O_NONBLOCK,
            mode as libc::c_uint,
        )
    };
    ensure!(fd >= 0, "E_WITNESS_IO");
    Ok(unsafe { File::from_raw_fd(fd) })
}

fn directory(path: &Path) -> Result<File> {
    ensure!(path.is_absolute(), "E_WITNESS_PATH");
    let root = CString::new("/")?;
    let fd = unsafe {
        libc::open(
            root.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
        )
    };
    ensure!(fd >= 0, "E_WITNESS_IO");
    let mut parent = unsafe { File::from_raw_fd(fd) };
    for component in path.components() {
        match component {
            Component::RootDir => (),
            Component::Normal(value) => {
                parent = open_at(
                    &parent,
                    &component_name(Path::new(value))?,
                    libc::O_RDONLY | libc::O_DIRECTORY,
                    0,
                )?
            }
            _ => anyhow::bail!("E_WITNESS_PATH"),
        }
    }
    Ok(parent)
}

fn read_at(parent: &File, name: &Path, limit: u64) -> Result<Vec<u8>> {
    ensure!(
        name.components().count() == 1
            && matches!(name.components().next(), Some(Component::Normal(_))),
        "E_WITNESS_PATH"
    );
    let mut file = open_at(parent, &component_name(name)?, libc::O_RDONLY, 0)?;
    let before = file.metadata()?;
    ensure!(
        before.is_file() && before.len() <= limit,
        "E_WITNESS_INPUT_LIMIT"
    );
    let mut raw = Vec::new();
    (&mut file).take(limit + 1).read_to_end(&mut raw)?;
    let after = file.metadata()?;
    ensure!(
        raw.len() as u64 == before.len()
            && before.len() == after.len()
            && before.mtime() == after.mtime()
            && before.mtime_nsec() == after.mtime_nsec()
            && before.ctime() == after.ctime()
            && before.ctime_nsec() == after.ctime_nsec(),
        "E_WITNESS_INPUT_CHANGED"
    );
    Ok(raw)
}

fn validate_rules(config: &Config) -> Result<RuleSet> {
    ensure!(
        !config.rule_files.is_empty() && config.rule_files.len() <= 64,
        "E_WITNESS_RULES"
    );
    let parent = directory(&config.rules_dir)?;
    ensure!(
        parent.metadata()?.mode() & 0o777 == 0o700,
        "E_WITNESS_PRIVATE_DIRECTORY"
    );
    let mut expected = BTreeSet::new();
    for binding in &config.rule_files {
        ensure!(
            valid_hash(&binding.sha256)
                && binding.bytes <= CONFIG_LIMIT
                && expected.insert(binding.path.clone()),
            "E_WITNESS_RULES"
        );
        let raw = read_at(&parent, Path::new(&binding.path), CONFIG_LIMIT)?;
        ensure!(
            raw.len() as u64 == binding.bytes && hash(&raw) == binding.sha256,
            "E_WITNESS_RULES"
        );
    }
    let mut actual = BTreeSet::new();
    for entry in std::fs::read_dir(&config.rules_dir)? {
        let entry = entry?;
        ensure!(entry.file_type()?.is_file(), "E_WITNESS_RULES");
        actual.insert(
            entry
                .file_name()
                .into_string()
                .map_err(|_| anyhow::anyhow!("E_WITNESS_RULES"))?,
        );
    }
    ensure!(actual == expected, "E_WITNESS_RULES");
    let rules = RuleSet::load_from_dir(&config.rules_dir)
        .map_err(|_| anyhow::anyhow!("E_WITNESS_RULES"))?;
    ensure!(
        rules.len() == 140 && config.witness_rule_id == WITNESS_ID,
        "E_WITNESS_RULES"
    );
    let witness = rules
        .get(WITNESS_ID)
        .ok_or_else(|| anyhow::anyhow!("E_WITNESS_RULES"))?;
    ensure!(
        witness.kind == RuleKind::Certificate
            && witness.scope == "certificate_validity"
            && witness.pattern == WITNESS_PATTERN,
        "E_WITNESS_RULES"
    );
    Ok(rules)
}

fn run(config_path: &Path) -> Result<()> {
    let parent_path = config_path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("E_WITNESS_PATH"))?;
    let parent = directory(parent_path)?;
    ensure!(
        parent.metadata()?.mode() & 0o777 == 0o700,
        "E_WITNESS_PRIVATE_DIRECTORY"
    );
    let raw = read_at(
        &parent,
        Path::new(
            config_path
                .file_name()
                .ok_or_else(|| anyhow::anyhow!("E_WITNESS_PATH"))?,
        ),
        CONFIG_LIMIT,
    )?;
    let envelope: serde_json::Value = serde_json::from_slice(&raw)?;
    let fields = envelope
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("E_WITNESS_CONFIG"))?;
    let mut expected = BTreeSet::from([
        "root",
        "max_file_size_bytes",
        "threads",
        "selection",
        "detector_second",
        "rules_dir",
        "witness_rule_id",
        "rule_files",
        "output",
        "bindings",
    ]);
    if fields.contains_key("kind") {
        ensure!(
            fields["kind"] == "temporal-witness-config-v2",
            "E_WITNESS_CONFIG"
        );
        expected.extend([
            "kind",
            "manifest_projection",
            "original_diagnostic_projection",
            "boundary_policy_relation",
        ]);
    }
    ensure!(
        fields.keys().map(String::as_str).collect::<BTreeSet<_>>() == expected,
        "E_WITNESS_CONFIG"
    );
    let config: Config =
        serde_json::from_slice(&raw).map_err(|_| anyhow::anyhow!("E_WITNESS_CONFIG"))?;
    ensure!(
        config.max_file_size_bytes == 2 * 1024 * 1024
            && config.threads == 1
            && config.detector_second >= 0
            && config.output.parent() == Some(parent_path)
            && config.output != config_path
            && config.rules_dir.parent() == Some(parent_path)
            && !parent_path.starts_with(&config.root)
            && [
                &config.bindings.source,
                &config.bindings.rules,
                &config.bindings.input,
                &config.bindings.selection,
                &config.bindings.tool
            ]
            .into_iter()
            .all(|s| valid_hash(s)),
        "E_WITNESS_CONFIG"
    );
    let root_handle = directory(&config.root)?;
    ensure!(root_handle.metadata()?.is_dir(), "E_WITNESS_PATH");
    let boundaries = capture_boundaries(&config)?;
    let rules = validate_rules(&config)?;
    let records = Arc::new(Mutex::new(Records::default()));
    let detector: Arc<dyn Detector> = Arc::new(Witness {
        detector: pqc_scan_detectors::certificate_detector::CertificateDetector::at(
            config.detector_second,
        ),
        root: config.root.clone(),
        records: records.clone(),
    });
    let selection = pqc_scan_core::ScanSelection {
        files: pqc_scan_core::FileSelection {
            include_non_scannable: config.selection.include_non_scannable,
        },
        exclude_tests: config.selection.exclude_tests,
    };
    let scan = pqc_scan_core::pipeline::scan_repository_with_scope(
        &pqc_scan_core::ScanConfig {
            root: config.root.clone(),
            max_file_size_bytes: config.max_file_size_bytes,
            threads: config.threads,
        },
        &rules,
        &[detector],
        selection,
        None,
        None,
    )
    .map_err(|_| anyhow::anyhow!("E_WITNESS_SCAN"))?;
    let mut records = records
        .lock()
        .map_err(|_| anyhow::anyhow!("E_WITNESS_STATE"))?;
    let coverage = scan.coverage();
    ensure!(
        !records.failure
            && coverage.read_failed_files() == 0
            && coverage.analysis_failed_files() == 0
            && coverage.unknown_files() == 0
            && coverage.enumeration_failed_entries()
                == boundaries.as_ref().map_or(0, |b| b.len() as u64)
            && records.files.len() as u64 == coverage.read_ok_files()
            && coverage.selected_files() == coverage.read_ok_files(),
        "E_WITNESS_PRODUCER_LOSS"
    );
    records.files.sort_by(|a, b| a.path.cmp(&b.path));
    records
        .objects
        .sort_by(|a, b| (&a.path, a.object_index).cmp(&(&b.path, b.object_index)));
    ensure!(
        records.files.windows(2).all(|w| w[0].path != w[1].path),
        "E_WITNESS_DUPLICATE_FILE"
    );
    let codes: BTreeSet<String> = scan
        .diagnostics()
        .iter()
        .map(|d| {
            serde_json::to_value(d.code())
                .ok()
                .and_then(|v| v.as_str().map(str::to_owned))
                .ok_or_else(|| anyhow::anyhow!("E_WITNESS_STATUS"))
        })
        .collect::<Result<_>>()?;
    let mut failed_paths = BTreeSet::new();
    for diagnostic in scan.diagnostics() {
        match diagnostic.code() {
            pqc_scan_core::DiagnosticCode::ReadFailed => {
                let path = diagnostic
                    .file()
                    .ok_or_else(|| anyhow::anyhow!("E_WITNESS_PRODUCER_LOSS"))?;
                ensure!(
                    diagnostic.detector().is_none()
                        && boundary_path(path)
                        && boundaries.as_ref().is_some_and(|b| b.contains_key(path))
                        && failed_paths.insert(path.to_owned()),
                    "E_WITNESS_PRODUCER_LOSS"
                );
            }
            pqc_scan_core::DiagnosticCode::DetectorUnsupported
            | pqc_scan_core::DiagnosticCode::DetectorPartial
            | pqc_scan_core::DiagnosticCode::ExcludedPath
            | pqc_scan_core::DiagnosticCode::FileTooLarge => (),
            _ => anyhow::bail!("E_WITNESS_PRODUCER_LOSS"),
        }
    }
    let mut boundary_receipts = Vec::new();
    if let Some(held) = &boundaries {
        ensure!(
            failed_paths == held.keys().cloned().collect()
                && (held.is_empty() || coverage.status().as_str() == "partial"),
            "E_WITNESS_PRODUCER_LOSS"
        );
        for boundary in held.values() {
            boundary.verify(&config.root)?;
            ensure!(
                !records
                    .files
                    .iter()
                    .any(|row| row.path == boundary.pinned.path)
                    && !records
                        .objects
                        .iter()
                        .any(|row| row.path == boundary.pinned.path),
                "E_WITNESS_PRODUCER_LOSS"
            );
            boundary_receipts.push(boundary.receipt());
        }
    }
    let boundary_bytes = serde_json::to_vec(&boundary_receipts)?.len();
    ensure!(
        records
            .files
            .len()
            .checked_add(boundary_receipts.len())
            .is_some_and(|n| n <= ROW_LIMIT)
            && records
                .encoded_bytes
                .checked_add(boundary_bytes)
                .is_some_and(|n| n <= DATA_LIMIT),
        "E_WITNESS_QUOTA"
    );
    let executable = std::env::current_exe()?;
    let binary_parent = directory(
        executable
            .parent()
            .ok_or_else(|| anyhow::anyhow!("E_WITNESS_PATH"))?,
    )?;
    let binary = read_at(
        &binary_parent,
        Path::new(
            executable
                .file_name()
                .ok_or_else(|| anyhow::anyhow!("E_WITNESS_PATH"))?,
        ),
        OUTPUT_LIMIT as u64,
    )?;
    let mut output = serde_json::json!({
        "kind": "temporal-producer-witness-v1",
        "configurationSha256": hash(&raw), "binarySha256": hash(&binary),
        "detectorSecond": config.detector_second, "ruleCount": rules.len(),
        "status": "producer_recorded_requires_admission", "bindings": config.bindings,
        "objects": records.objects, "files": records.files,
        "diagnosticCodes": codes,
        "coverage": {
            "selectedFiles": coverage.selected_files(), "readOkFiles": coverage.read_ok_files(),
            "readFailedFiles": coverage.read_failed_files(), "analyzedFiles": coverage.analyzed_files(),
            "partialFiles": coverage.partial_files(), "analysisFailedFiles": coverage.analysis_failed_files(),
            "unsupportedFiles": coverage.unsupported_files(), "notApplicableFiles": coverage.not_applicable_files(),
            "unknownFiles": coverage.unknown_files(), "excludedEnumeratedFiles": coverage.excluded_enumerated_files(),
            "enumerationFailedEntries": coverage.enumeration_failed_entries(),
        },
    });
    if boundaries.is_some() {
        output["kind"] = serde_json::json!("temporal-producer-witness-v2");
        output["boundaryFailures"] = serde_json::to_value(boundary_receipts)?;
        output["inspectionStatus"] = serde_json::json!(coverage.status().as_str());
    }
    let encoded = serde_json::to_vec(&output)?;
    ensure!(encoded.len() <= OUTPUT_LIMIT, "E_WITNESS_QUOTA");
    let filename = Path::new(
        config
            .output
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("E_WITNESS_PATH"))?,
    );
    let mut destination = open_at(
        &parent,
        &component_name(filename)?,
        libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL,
        0o600,
    )?;
    destination.write_all(&encoded)?;
    destination.sync_all()?;
    parent.sync_all()?;
    Ok(())
}

fn main() {
    std::panic::set_hook(Box::new(|_| eprintln!("E_WITNESS_PANIC")));
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 2 || run(Path::new(&args[1])).is_err() {
        eprintln!("E_WITNESS_REJECTED");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static SEQUENCE: AtomicU64 = AtomicU64::new(0);

    fn rules() -> (PathBuf, RuleSet) {
        let root = std::env::temp_dir().join(format!(
            "temporal-public-witness-{}-{}",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::SeqCst)
        ));
        std::fs::create_dir(&root).unwrap();
        std::fs::write(root.join("witness.yml"), b"- id: TEMPORAL_PRIVATE_VALIDITY_WITNESS\n  kind: certificate\n  category: proof\n  severity: info\n  risk: non-quantum-risk\n  confidence: 1.0\n  migration_hint: Proof only.\n  pattern: '^(?:valid|not_yet_valid|expired)$'\n  scope: certificate_validity\n").unwrap();
        let rules = RuleSet::load_from_dir(&root).unwrap();
        (root, rules)
    }

    fn public_file() -> ScannableFile {
        ScannableFile::from_bytes(
            PathBuf::from("/synthetic/public.der"),
            include_bytes!(concat!(
                env!("PQC_TEMPORAL_PRODUCT_SOURCE"),
                "/crates/pqc_scan_detectors/tests/fixtures/certificates/rsa2048.der"
            ))
            .to_vec(),
        )
    }

    #[test]
    fn exact_product_parser_witnesses_each_closed_validity_boundary() {
        let (temporary, rules) = rules();
        let file = public_file();
        let first = pqc_scan_detectors::certificate_detector::CertificateDetector::at(0)
            .detect_detailed(&file, &rules)
            .unwrap();
        let (_, objects) = capture(&file, Path::new("/synthetic"), &rules, &first).unwrap();
        assert_eq!(objects.len(), 1);
        let object = &objects[0];
        for (second, expected) in [
            (object.not_before_unix - 1, "not_yet_valid"),
            (object.not_before_unix, "valid"),
            (object.not_after_unix, "valid"),
            (object.not_after_unix + 1, "expired"),
        ] {
            let outcome = pqc_scan_detectors::certificate_detector::CertificateDetector::at(second)
                .detect_detailed(&file, &rules)
                .unwrap();
            let (record, rows) = capture(&file, Path::new("/synthetic"), &rules, &outcome).unwrap();
            assert_eq!(record.objects, 1);
            assert_eq!(rows[0].validity_state, expected);
            assert_eq!(rows[0].certificate_sha256, object.certificate_sha256);
        }
        std::fs::remove_dir_all(temporary).unwrap();
    }

    #[test]
    fn raw_capture_survives_typed_fact_rejection_and_excludes_free_values() {
        let (temporary, rules) = rules();
        let file = public_file();
        let outcome = pqc_scan_detectors::certificate_detector::CertificateDetector::at(0)
            .detect_detailed(&file, &rules)
            .unwrap();
        let mut detections = outcome.detections().to_vec();
        detections[0]
            .evidence
            .metadata
            .insert("signature_oid".into(), "1.".repeat(130));
        detections[0]
            .evidence
            .metadata
            .insert("private_scalar".into(), "PRIVATE_SENTINEL".into());
        detections[0].evidence.snippet_preview = "PRIVATE_SENTINEL".into();
        assert!(pqc_scan_core::PublicCertificateFacts::from_parser_metadata(
            &detections[0].evidence.metadata
        )
        .is_none());
        let (_, objects) = capture(
            &file,
            Path::new("/synthetic"),
            &rules,
            &DetectorOutcome::new(detections, DetectorStatus::Partial),
        )
        .unwrap();
        assert_eq!(objects.len(), 1);
        let value = serde_json::to_value(&objects[0]).unwrap();
        let keys: BTreeSet<_> = value
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(
            keys,
            BTreeSet::from([
                "path",
                "sourceSha256",
                "certificateSha256",
                "objectIndex",
                "notBeforeUnix",
                "notAfterUnix",
                "validityState",
                "matchingDefaultValidityRuleIds"
            ])
        );
        assert!(!serde_json::to_string(&objects)
            .unwrap()
            .contains("PRIVATE_SENTINEL"));
        std::fs::remove_dir_all(temporary).unwrap();
    }

    #[test]
    fn duplicate_missing_metadata_and_truncated_producer_reject() {
        let (temporary, rules) = rules();
        let file = public_file();
        let outcome = pqc_scan_detectors::certificate_detector::CertificateDetector::at(0)
            .detect_detailed(&file, &rules)
            .unwrap();
        let original = outcome.detections().to_vec();
        let mut duplicate = original.clone();
        duplicate.extend(original.clone());
        assert!(capture(
            &file,
            Path::new("/synthetic"),
            &rules,
            &DetectorOutcome::new(duplicate, DetectorStatus::Complete)
        )
        .is_err());
        let mut missing = original.clone();
        missing[0].evidence.metadata.remove("not_before_unix");
        assert!(capture(
            &file,
            Path::new("/synthetic"),
            &rules,
            &DetectorOutcome::new(missing, DetectorStatus::Complete)
        )
        .is_err());
        assert!(capture(
            &file,
            Path::new("/synthetic"),
            &rules,
            &DetectorOutcome::new(original, DetectorStatus::Truncated)
        )
        .is_err());
        std::fs::remove_dir_all(temporary).unwrap();
    }

    #[test]
    fn unknown_and_partial_statuses_are_preserved_without_a_success_claim() {
        let (temporary, rules) = rules();
        let file = public_file();
        for (status, label) in [
            (DetectorStatus::Partial, "partial"),
            (DetectorStatus::Unsupported, "unsupported"),
            (DetectorStatus::NotApplicable, "not_applicable"),
        ] {
            let (record, objects) = capture(
                &file,
                Path::new("/synthetic"),
                &rules,
                &DetectorOutcome::new(Vec::new(), status),
            )
            .unwrap();
            assert_eq!(record.status, label);
            assert!(objects.is_empty());
        }
        std::fs::remove_dir_all(temporary).unwrap();
    }

    #[test]
    fn no_follow_regular_file_size_and_exclusive_creation_boundaries() {
        use std::os::unix::fs::symlink;
        let (temporary, _) = rules();
        let temporary = temporary.canonicalize().unwrap();
        std::fs::write(temporary.join("public"), b"public synthetic").unwrap();
        symlink(temporary.join("public"), temporary.join("alias")).unwrap();
        let parent = directory(&temporary).unwrap();
        assert!(read_at(&parent, Path::new("alias"), CONFIG_LIMIT).is_err());
        assert!(read_at(&parent, Path::new("../public"), CONFIG_LIMIT).is_err());
        assert!(read_at(&parent, Path::new("public"), 2).is_err());
        assert!(open_at(
            &parent,
            &component_name(Path::new("public")).unwrap(),
            libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL,
            0o600
        )
        .is_err());
        assert_eq!(
            std::fs::read(temporary.join("public")).unwrap(),
            b"public synthetic"
        );
        symlink(&temporary, temporary.join("directory-alias")).unwrap();
        assert!(directory(&temporary.join("directory-alias")).is_err());
        std::fs::remove_dir_all(temporary).unwrap();
    }

    #[test]
    fn output_quota_failure_cannot_be_mistaken_for_an_empty_witness() {
        let (temporary, rules) = rules();
        let records = Arc::new(Mutex::new(Records {
            encoded_bytes: DATA_LIMIT,
            ..Records::default()
        }));
        let detector = Witness {
            detector: pqc_scan_detectors::certificate_detector::CertificateDetector::at(0),
            root: PathBuf::from("/synthetic"),
            records: records.clone(),
        };
        assert!(detector.detect_detailed(&public_file(), &rules).is_err());
        assert!(records.lock().unwrap().failure);
        std::fs::remove_dir_all(temporary).unwrap();
    }
    fn pinned(path: &str, target: &str) -> PinnedLink {
        PinnedLink {
            path: path.into(),
            manifest_entry_sha256: "a".repeat(64),
            link_metadata_sha256: link_metadata_hash(path, target.as_bytes()).unwrap(),
        }
    }

    #[test]
    fn framed_link_hash_matches_python_utf8_path_fixture() {
        assert_eq!(
            link_metadata_hash("dir/公.der", "../公開.der".as_bytes()).unwrap(),
            "61f91a35dd32535dcc21fb45346b31714f55333ab79e292023aafbaf9e9884d1"
        );
        for path in ["", "/absolute", "a/../b", "a//b", "./a", "a\n"] {
            assert!(link_metadata_hash(path, b"target").is_err());
        }
        assert!(link_metadata_hash("alias", b"\xff").is_err());
        assert!(link_metadata_hash("alias", &vec![b'a'; 4097]).is_err());
    }

    #[test]
    fn held_boundary_rejects_leaf_target_and_parent_replacement_without_following() {
        use std::os::unix::fs::symlink;
        for mutation in [
            "leaf",
            "target",
            "parent",
            "parent_symlink",
            "regular",
            "fifo",
        ] {
            let (temporary, _) = rules();
            let temporary = temporary.canonicalize().unwrap();
            let root = temporary.join("input");
            std::fs::create_dir_all(root.join("dir")).unwrap();
            symlink("../../never-opened", root.join("dir/alias")).unwrap();
            let held =
                HeldBoundary::capture(&root, pinned("dir/alias", "../../never-opened")).unwrap();
            held.verify(&root).unwrap();
            match mutation {
                "parent" | "parent_symlink" => {
                    std::fs::rename(root.join("dir"), root.join("old")).unwrap();
                    if mutation == "parent" {
                        std::fs::create_dir(root.join("dir")).unwrap();
                        symlink("../../never-opened", root.join("dir/alias")).unwrap();
                    } else {
                        symlink("old", root.join("dir")).unwrap();
                    }
                }
                _ => {
                    std::fs::remove_file(root.join("dir/alias")).unwrap();
                    match mutation {
                        "regular" => std::fs::write(root.join("dir/alias"), b"public").unwrap(),
                        "fifo" => {
                            let name = component_name(&root.join("dir/alias")).unwrap();
                            assert_eq!(unsafe { libc::mkfifo(name.as_ptr(), 0o600) }, 0);
                        }
                        "target" => symlink("../../different", root.join("dir/alias")).unwrap(),
                        _ => symlink("../../never-opened", root.join("dir/alias")).unwrap(),
                    }
                }
            }
            assert!(held.verify(&root).is_err(), "{mutation}");
            std::fs::remove_dir_all(temporary).unwrap();
        }
    }

    fn synthetic_binding(path: &Path) -> serde_json::Value {
        let raw = std::fs::read(path).unwrap();
        serde_json::json!({"path":path,"sha256":hash(&raw),"bytes":raw.len()})
    }

    fn synthetic_config(v2: bool) -> PathBuf {
        use std::os::unix::fs::{symlink, PermissionsExt};
        let (temporary, _) = rules();
        let temporary = temporary.canonicalize().unwrap();
        std::fs::set_permissions(&temporary, std::fs::Permissions::from_mode(0o700)).unwrap();
        let root = temporary.join("input");
        std::fs::create_dir_all(root.join(".git")).unwrap();
        std::fs::write(root.join("public.der"), &public_file().bytes).unwrap();
        symlink("outside-never-opened", root.join("alias.der")).unwrap();
        symlink("outside-excluded", root.join(".git/alias.der")).unwrap();
        let proof = temporary.join("proof-rules");
        std::fs::create_dir(&proof).unwrap();
        std::fs::set_permissions(&proof, std::fs::Permissions::from_mode(0o700)).unwrap();
        for entry in
            std::fs::read_dir(Path::new(env!("PQC_TEMPORAL_PRODUCT_SOURCE")).join("rules/default"))
                .unwrap()
        {
            let entry = entry.unwrap();
            std::fs::copy(entry.path(), proof.join(entry.file_name())).unwrap();
        }
        std::fs::copy(
            temporary.join("witness.yml"),
            proof.join("temporal-private-witness.yml"),
        )
        .unwrap();
        let mut rule_files = Vec::new();
        for entry in std::fs::read_dir(&proof).unwrap() {
            let path = entry.unwrap().path();
            let raw = std::fs::read(&path).unwrap();
            rule_files.push(serde_json::json!({"path":path.file_name().unwrap().to_str().unwrap(),"sha256":hash(&raw),"bytes":raw.len()}));
        }
        let mut config = serde_json::json!({"root":root,"max_file_size_bytes":2097152,"threads":1,
            "selection":{"include_non_scannable":false,"exclude_tests":false},"detector_second":1793491200i64,
            "rules_dir":proof,"witness_rule_id":WITNESS_ID,"rule_files":rule_files,"output":temporary.join("output.json"),
            "bindings":{"source":"a".repeat(64),"rules":"b".repeat(64),"input":"c".repeat(64),"selection":"d".repeat(64),"tool":"e".repeat(64)}});
        if v2 {
            let manifest = temporary.join("manifest-projection.json");
            let rows = [
                pinned("alias.der", "outside-never-opened"),
                pinned(".git/alias.der", "outside-excluded"),
            ];
            std::fs::write(&manifest, serde_json::to_vec(&serde_json::json!({"kind":"temporal-manifest-link-projection-v2",
                "manifest":{"path":"/synthetic/original-manifest.json","sha256":"c".repeat(64),"bytes":1},"entries":rows})).unwrap()).unwrap();
            let diagnostic = temporary.join("diagnostic-projection.json");
            let reference = serde_json::json!({"path":"/synthetic/original-diagnostics.json","sha256":"f".repeat(64),"bytes":1});
            std::fs::write(&diagnostic, serde_json::to_vec(&serde_json::json!({"kind":"temporal-original-diagnostic-projection-v2",
                "phaseId":"primary-after","originalTerminal":reference,"diagnostics":reference,
                "entries":[{"code":"read_failed","path":"alias.der","detector":null,"occurrences":1}]})).unwrap()).unwrap();
            let policy = temporary.join("policy.json");
            std::fs::write(
                &policy,
                b"public synthetic policy; admission remains independent",
            )
            .unwrap();
            config["kind"] = serde_json::json!("temporal-witness-config-v2");
            config["manifest_projection"] = synthetic_binding(&manifest);
            config["original_diagnostic_projection"] = synthetic_binding(&diagnostic);
            config["boundary_policy_relation"] = synthetic_binding(&policy);
        }
        let path = temporary.join("config.json");
        std::fs::write(&path, serde_json::to_vec(&config).unwrap()).unwrap();
        path
    }

    #[test]
    fn v2_product_pipeline_preserves_partial_and_regular_objects_but_v1_rejects_links() {
        for v2 in [false, true] {
            let config = synthetic_config(v2);
            let parent = config.parent().unwrap();
            let result = run(&config);
            if v2 {
                result.unwrap();
                let output: serde_json::Value =
                    serde_json::from_slice(&std::fs::read(parent.join("output.json")).unwrap())
                        .unwrap();
                assert_eq!(output["kind"], "temporal-producer-witness-v2");
                assert_eq!(output["inspectionStatus"], "partial");
                assert_eq!(output["coverage"]["enumerationFailedEntries"], 1);
                assert_eq!(output["coverage"]["readFailedFiles"], 0);
                assert_eq!(output["coverage"]["selectedFiles"], 1);
                assert_eq!(output["boundaryFailures"].as_array().unwrap().len(), 1);
                assert_eq!(output["boundaryFailures"][0]["path"], "alias.der");
                assert_eq!(output["files"].as_array().unwrap().len(), 1);
                assert_eq!(output["objects"].as_array().unwrap().len(), 1);
            } else {
                assert!(result.is_err());
                assert!(!parent.join("output.json").exists());
            }
            std::fs::remove_dir_all(parent).unwrap();
        }
    }

    #[test]
    fn v2_projection_path_detector_multiplicity_hash_and_nonregular_fail_closed() {
        use std::os::unix::fs::symlink;
        for mutation in [
            "path",
            "detector",
            "duplicate",
            "bool",
            "hash",
            "regular",
            "regular_read_failure",
            "parent_symlink",
            "extra",
            "kind",
        ] {
            let config = synthetic_config(true);
            let parent = config.parent().unwrap();
            let mut value: serde_json::Value =
                serde_json::from_slice(&std::fs::read(&config).unwrap()).unwrap();
            if matches!(mutation, "path" | "detector" | "duplicate" | "bool") {
                let path = parent.join("diagnostic-projection.json");
                let mut diagnostic: serde_json::Value =
                    serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
                match mutation {
                    "path" => {
                        diagnostic["entries"][0]["path"] = serde_json::json!(".git/alias.der")
                    }
                    "detector" => {
                        diagnostic["entries"][0]["detector"] =
                            serde_json::json!("certificate_detector")
                    }
                    "bool" => diagnostic["entries"][0]["occurrences"] = serde_json::json!(true),
                    _ => {
                        let row = diagnostic["entries"][0].clone();
                        diagnostic["entries"].as_array_mut().unwrap().push(row);
                    }
                }
                std::fs::write(&path, serde_json::to_vec(&diagnostic).unwrap()).unwrap();
                value["original_diagnostic_projection"] = synthetic_binding(&path);
            } else if mutation == "hash" {
                value["manifest_projection"]["sha256"] = serde_json::json!("0".repeat(64));
            } else if mutation == "regular" {
                std::fs::remove_file(parent.join("input/alias.der")).unwrap();
                std::fs::write(parent.join("input/alias.der"), b"public synthetic").unwrap();
            } else if mutation == "regular_read_failure" {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(
                    parent.join("input/public.der"),
                    std::fs::Permissions::from_mode(0),
                )
                .unwrap();
            } else if mutation == "parent_symlink" {
                std::fs::rename(parent.join("input"), parent.join("old-input")).unwrap();
                symlink("old-input", parent.join("input")).unwrap();
            } else if mutation == "extra" {
                value["skip_boundary"] = serde_json::json!(true);
            } else {
                value["kind"] = serde_json::json!("temporal-witness-config-v1");
            }
            std::fs::write(&config, serde_json::to_vec(&value).unwrap()).unwrap();
            assert!(run(&config).is_err(), "{mutation}");
            assert!(!parent.join("output.json").exists());
            std::fs::remove_dir_all(parent).unwrap();
        }
    }
}
