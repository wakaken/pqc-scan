use crate::{SshOffers, SshProbe, MAX_FRAME_BYTES, MAX_NAMES_PER_LIST, MAX_NAME_BYTES};
use pqc_scan_core::probe::ProbeDiagnostic;

pub fn parse_identification(line: &[u8]) -> Result<String, ProbeDiagnostic> {
    if line.len() > 255 {
        return Err(ProbeDiagnostic::LocalResourceLimit);
    }
    let body = line
        .strip_suffix(b"\r\n")
        .or_else(|| {
            if line.starts_with(b"SSH-1.99-") {
                line.strip_suffix(b"\n")
            } else {
                None
            }
        })
        .ok_or(ProbeDiagnostic::ProtocolMismatch)?;
    let text = std::str::from_utf8(body).map_err(|_| ProbeDiagnostic::ProtocolMismatch)?;
    if text.chars().any(char::is_control) {
        return Err(ProbeDiagnostic::ProtocolMismatch);
    }
    let rest = text
        .strip_prefix("SSH-2.0-")
        .or_else(|| text.strip_prefix("SSH-1.99-"))
        .ok_or(ProbeDiagnostic::ProtocolMismatch)?;
    let software = rest.split(' ').next().unwrap_or("");
    if software.is_empty() || !software.bytes().all(|b| b.is_ascii_graphic() && b != b'-') {
        return Err(ProbeDiagnostic::ProtocolMismatch);
    }
    Ok(text.into())
}
pub fn parse_packet(frame: &[u8]) -> Result<&[u8], ProbeDiagnostic> {
    if frame.len() < 4 {
        return Err(ProbeDiagnostic::MalformedPeerData);
    }
    let total = frame_length(frame[..4].try_into().expect("four bytes"))?;
    if frame.len() != total || total % 8 != 0 || total < 16 {
        return Err(ProbeDiagnostic::MalformedPeerData);
    }
    let padding = usize::from(frame[4]);
    if padding < 4 || padding + 5 >= total {
        return Err(ProbeDiagnostic::MalformedPeerData);
    }
    Ok(&frame[5..total - padding])
}
pub(crate) fn frame_length(header: [u8; 4]) -> Result<usize, ProbeDiagnostic> {
    let n = usize::try_from(u32::from_be_bytes(header))
        .map_err(|_| ProbeDiagnostic::LocalResourceLimit)?;
    let total = n
        .checked_add(4)
        .ok_or(ProbeDiagnostic::LocalResourceLimit)?;
    if total > MAX_FRAME_BYTES {
        return Err(ProbeDiagnostic::LocalResourceLimit);
    }
    if total < 16 || total % 8 != 0 {
        return Err(ProbeDiagnostic::MalformedPeerData);
    }
    Ok(total)
}
pub(crate) struct Cursor<'a> {
    pub input: &'a [u8],
    pub offset: usize,
}
impl<'a> Cursor<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self { input, offset: 0 }
    }
    pub fn bytes(&mut self, n: usize) -> Result<&'a [u8], ProbeDiagnostic> {
        let end = self
            .offset
            .checked_add(n)
            .ok_or(ProbeDiagnostic::MalformedPeerData)?;
        let bytes = self
            .input
            .get(self.offset..end)
            .ok_or(ProbeDiagnostic::MalformedPeerData)?;
        self.offset = end;
        Ok(bytes)
    }
    pub fn u32(&mut self) -> Result<u32, ProbeDiagnostic> {
        Ok(u32::from_be_bytes(
            self.bytes(4)?.try_into().expect("four bytes"),
        ))
    }
    pub fn string(&mut self) -> Result<&'a [u8], ProbeDiagnostic> {
        let n = usize::try_from(self.u32()?).map_err(|_| ProbeDiagnostic::MalformedPeerData)?;
        self.bytes(n)
    }
    pub fn finished(&self) -> bool {
        self.offset == self.input.len()
    }
}
fn names(cursor: &mut Cursor<'_>, empty_allowed: bool) -> Result<Vec<String>, ProbeDiagnostic> {
    let value = cursor.string()?;
    if value.len() > MAX_NAMES_PER_LIST * (MAX_NAME_BYTES + 1) - 1 {
        return Err(ProbeDiagnostic::LocalResourceLimit);
    }
    if value.is_empty() {
        return if empty_allowed {
            Ok(vec![])
        } else {
            Err(ProbeDiagnostic::MalformedPeerData)
        };
    }
    let mut result = Vec::new();
    for bytes in value.split(|b| *b == b',') {
        if result.len() == MAX_NAMES_PER_LIST || bytes.len() > MAX_NAME_BYTES {
            return Err(ProbeDiagnostic::LocalResourceLimit);
        }
        if bytes.is_empty() || !bytes.iter().all(u8::is_ascii_graphic) {
            return Err(ProbeDiagnostic::MalformedPeerData);
        }
        let name = std::str::from_utf8(bytes).map_err(|_| ProbeDiagnostic::MalformedPeerData)?;
        if name.matches('@').count() > 1 || name.starts_with('@') || name.ends_with('@') {
            return Err(ProbeDiagnostic::MalformedPeerData);
        }
        result.push(name.into());
    }
    Ok(result)
}
pub fn parse_kexinit(payload: &[u8]) -> Result<(SshOffers, bool), ProbeDiagnostic> {
    if payload.len() > MAX_FRAME_BYTES {
        return Err(ProbeDiagnostic::LocalResourceLimit);
    }
    let mut c = Cursor::new(payload);
    if c.bytes(1)? != [20] {
        return Err(ProbeDiagnostic::MalformedPeerData);
    }
    c.bytes(16)?;
    let offers = SshOffers {
        kex_algorithms: names(&mut c, false)?,
        server_host_key_algorithms: names(&mut c, false)?,
        encryption_algorithms_client_to_server: names(&mut c, false)?,
        encryption_algorithms_server_to_client: names(&mut c, false)?,
        mac_algorithms_client_to_server: names(&mut c, false)?,
        mac_algorithms_server_to_client: names(&mut c, false)?,
        compression_algorithms_client_to_server: names(&mut c, false)?,
        compression_algorithms_server_to_client: names(&mut c, false)?,
        languages_client_to_server: names(&mut c, true)?,
        languages_server_to_client: names(&mut c, true)?,
    };
    let follows = c.bytes(1)?[0] != 0;
    if c.u32()? != 0 || !c.finished() {
        return Err(ProbeDiagnostic::MalformedPeerData);
    }
    Ok((offers, follows))
}
pub fn encode_kexinit(cookie: [u8; 16]) -> Vec<u8> {
    let profile = SshProbe::new();
    let mut payload = vec![20];
    payload.extend_from_slice(&cookie);
    for list in profile.profile().offers.lists() {
        let value = list.join(",");
        payload.extend_from_slice(&(value.len() as u32).to_be_bytes());
        payload.extend_from_slice(value.as_bytes());
    }
    payload.push(0);
    payload.extend_from_slice(&0u32.to_be_bytes());
    payload
}
pub fn encode_packet(payload: &[u8], padding: &[u8]) -> Result<Vec<u8>, ProbeDiagnostic> {
    let total = 5usize
        .checked_add(payload.len())
        .and_then(|n| n.checked_add(padding.len()))
        .ok_or(ProbeDiagnostic::LocalResourceLimit)?;
    if total > MAX_FRAME_BYTES {
        return Err(ProbeDiagnostic::LocalResourceLimit);
    }
    if payload.is_empty() || !(4..=255).contains(&padding.len()) || total % 8 != 0 {
        return Err(ProbeDiagnostic::MalformedPeerData);
    }
    let mut frame = Vec::with_capacity(total);
    frame.extend_from_slice(&((total - 4) as u32).to_be_bytes());
    frame.push(padding.len() as u8);
    frame.extend_from_slice(payload);
    frame.extend_from_slice(padding);
    Ok(frame)
}
