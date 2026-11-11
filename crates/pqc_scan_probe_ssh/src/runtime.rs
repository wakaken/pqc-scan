use crate::{
    encode_kexinit, encode_packet, parse_identification, parse_kexinit, parse_packet,
    parser::{frame_length, Cursor},
    SshObservation, SshProbe, MAX_BANNER_BYTES, MAX_EARLY_MESSAGES, MAX_PRE_IDENTIFICATION_LINES,
};
use pqc_scan_core::{
    probe::{ProbeDiagnostic, ProbeOutcome, ProbeReport, ProbeTarget},
    probe_transport::BoundedTcpStream,
    target_policy::{AuthorizedTarget, ProbeBudget, ProbeError},
};
use std::io::{self, Read, Write};

impl SshProbe {
    pub fn run(self, target: AuthorizedTarget, budget: ProbeBudget) -> ProbeReport<SshObservation> {
        let mut report = self.failed(
            ProbeTarget::authorized(&target),
            budget.limits(),
            ProbeDiagnostic::InspectionNotCompleted,
        );
        let mut stream = match BoundedTcpStream::connect(target, budget) {
            Ok(s) => s,
            Err(e) => {
                report.set_outcome(ProbeOutcome::Failed, vec![e.into()]);
                return report;
            }
        };
        report.target.connected_peer = report.target.pinned_address;
        let result = self.collect(&mut stream, &mut report.details);
        report.set_traffic(stream.budget());
        stream.close();
        match result {
            Ok(()) => report.set_outcome(ProbeOutcome::Completed, vec![]),
            Err(code) => report.set_outcome(
                if matches!(
                    code,
                    ProbeDiagnostic::LocalResourceLimit
                        | ProbeDiagnostic::TrafficLimit
                        | ProbeDiagnostic::DeadlineExceeded
                        | ProbeDiagnostic::Cancelled
                ) {
                    ProbeOutcome::Partial
                } else {
                    ProbeOutcome::Failed
                },
                vec![code],
            ),
        }
        report
    }
    fn collect(
        &self,
        stream: &mut BoundedTcpStream,
        details: &mut SshObservation,
    ) -> Result<(), ProbeDiagnostic> {
        stream
            .write_all(format!("{}\r\n", self.profile.identification).as_bytes())
            .map_err(io_diagnostic)?;
        details.server_identification = Some(read_identification(
            stream,
            &mut details.pre_identification_lines,
        )?);
        details.assurance = "peer_claim_only";
        let mut cookie = [0; 16];
        getrandom::fill(&mut cookie).map_err(|_| ProbeDiagnostic::UnsupportedLocal)?;
        let payload = encode_kexinit(cookie);
        let mut padding_len = 8 - ((5 + payload.len()) % 8);
        if padding_len < 4 {
            padding_len += 8;
        }
        let mut padding = vec![0; padding_len];
        getrandom::fill(&mut padding).map_err(|_| ProbeDiagnostic::UnsupportedLocal)?;
        stream
            .write_all(&encode_packet(&payload, &padding)?)
            .map_err(io_diagnostic)?;
        loop {
            let frame = read_frame(stream)?;
            let payload = parse_packet(&frame)?;
            match payload[0] {
                20 => {
                    let (offers, follows) = parse_kexinit(payload)?;
                    details.extension_markers = offers.extension_markers();
                    details.kex_candidates = offers.kex_candidates();
                    details.host_key_candidates = offers.host_key_candidates();
                    details.offers = Some(offers);
                    details.first_kex_packet_follows = Some(follows);
                    details.assurance = "unauthenticated_advertisement";
                    return Ok(());
                }
                1 => {
                    validate_early(payload)?;
                    return Err(ProbeDiagnostic::PeerDisconnected);
                }
                2 | 4 => {
                    if details.early_messages == MAX_EARLY_MESSAGES {
                        return Err(ProbeDiagnostic::LocalResourceLimit);
                    }
                    validate_early(payload)?;
                    details.early_messages += 1;
                }
                _ => return Err(ProbeDiagnostic::ProtocolMismatch),
            }
        }
    }
}
fn io_diagnostic(error: io::Error) -> ProbeDiagnostic {
    if let Some(code) = error.get_ref().and_then(|e| e.downcast_ref::<ProbeError>()) {
        return (*code).into();
    }
    if error.kind() == io::ErrorKind::UnexpectedEof {
        ProbeDiagnostic::MalformedPeerData
    } else {
        ProbeDiagnostic::TransportFailed
    }
}
fn read_exact(stream: &mut BoundedTcpStream, out: &mut [u8]) -> Result<(), ProbeDiagnostic> {
    let mut offset = 0;
    while offset < out.len() {
        let n = stream.read(&mut out[offset..]).map_err(io_diagnostic)?;
        if n == 0 {
            return Err(if offset == 0 {
                ProbeDiagnostic::PeerDisconnected
            } else {
                ProbeDiagnostic::MalformedPeerData
            });
        }
        offset += n;
    }
    Ok(())
}
fn read_identification(
    stream: &mut BoundedTcpStream,
    prelines: &mut usize,
) -> Result<String, ProbeDiagnostic> {
    let mut received = 0;
    let mut line = Vec::new();
    loop {
        if received == MAX_BANNER_BYTES {
            return Err(ProbeDiagnostic::LocalResourceLimit);
        }
        let mut byte = [0];
        read_exact(stream, &mut byte)?;
        received += 1;
        line.push(byte[0]);
        if line.starts_with(b"SSH-") && line.len() > 255 {
            return Err(ProbeDiagnostic::LocalResourceLimit);
        }
        if byte[0] == b'\n' {
            if line.starts_with(b"SSH-") {
                return parse_identification(&line);
            }
            if *prelines == MAX_PRE_IDENTIFICATION_LINES {
                return Err(ProbeDiagnostic::LocalResourceLimit);
            }
            *prelines += 1;
            line.clear();
        }
    }
}
fn read_frame(stream: &mut BoundedTcpStream) -> Result<Vec<u8>, ProbeDiagnostic> {
    let mut header = [0; 4];
    read_exact(stream, &mut header)?;
    let total = frame_length(header)?;
    let mut frame = Vec::with_capacity(total);
    frame.extend_from_slice(&header);
    frame.resize(total, 0);
    read_exact(stream, &mut frame[4..]).map_err(|code| {
        if code == ProbeDiagnostic::PeerDisconnected {
            ProbeDiagnostic::MalformedPeerData
        } else {
            code
        }
    })?;
    Ok(frame)
}
fn validate_early(payload: &[u8]) -> Result<(), ProbeDiagnostic> {
    let mut c = Cursor::new(payload);
    let kind = c.bytes(1)?[0];
    match kind {
        1 => {
            c.u32()?;
            c.string()?;
            c.string()?;
        }
        2 => {
            c.string()?;
        }
        4 => {
            c.bytes(1)?;
            c.string()?;
            c.string()?;
        }
        _ => return Err(ProbeDiagnostic::ProtocolMismatch),
    }
    if !c.finished() {
        return Err(ProbeDiagnostic::MalformedPeerData);
    }
    Ok(())
}
