//! One bounded public selection setting; detector work budgets are independent.
pub(super) const DEFAULT_MIB: usize = 2;
pub(super) const MAX_MIB: usize = 64;
pub(super) fn parse_mib(value: &str) -> Result<usize, String> {
    if value.is_empty() || !value.bytes().all(|b| b.is_ascii_digit()) {
        return Err("file size must be a whole number of MiB (1..64)".into());
    }
    let mib = value
        .parse::<usize>()
        .map_err(|_| "file size is out of range (1..64 MiB)")?;
    if !(1..=MAX_MIB).contains(&mib) {
        return Err("file size is out of range (1..64 MiB)".into());
    }
    Ok(mib)
}
pub(super) fn bytes(mib: usize) -> anyhow::Result<usize> {
    if !(1..=MAX_MIB).contains(&mib) {
        anyhow::bail!("invalid file size limit");
    }
    mib.checked_mul(1024 * 1024)
        .ok_or_else(|| anyhow::anyhow!("file size limit overflow"))
}
