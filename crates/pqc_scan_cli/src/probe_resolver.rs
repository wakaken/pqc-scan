//! System DNS runs only in a killable child of the optional probe executable.
// Task 030 prepares this adapter; protocol consumers are gated on task 031.
#![allow(dead_code)]
use pqc_scan_core::target_policy::{Cancellation, ProbeError, ProbeHost, Resolver, TargetRequest};
use std::{
    io::{Read, Write},
    net::{IpAddr, ToSocketAddrs},
    process::{Child, Command, Stdio},
    time::{Duration, Instant},
};

const OUTPUT_LIMIT: usize = 512;

// Constructed by the protocol commands introduced after the task-031 gate.
pub(crate) struct ProcessResolver {
    executable: std::path::PathBuf,
}
impl ProcessResolver {
    pub(crate) fn new() -> Result<Self, ProbeError> {
        Ok(Self {
            executable: std::env::current_exe().map_err(|_| ProbeError::DnsFailed)?,
        })
    }
    #[cfg(test)]
    pub(crate) fn for_executable(executable: std::path::PathBuf) -> Self {
        Self { executable }
    }
}
impl Resolver for ProcessResolver {
    fn resolve(
        &mut self,
        host: &str,
        deadline: Instant,
        cancellation: &Cancellation,
    ) -> Result<Vec<IpAddr>, ProbeError> {
        check(deadline, cancellation)?;
        let mut command = Command::new(&self.executable);
        command
            .args(["__resolve-host", host])
            .env_clear()
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());
        let child = command.spawn().map_err(|_| ProbeError::DnsFailed)?;
        parse_answers(&collect_child(child, deadline, cancellation)?)
    }
}

/// Child-only: emits at most nine IP lines (under pipe capacity). The ninth
/// answer is an explicit overflow marker, never a silently truncated success.
pub(crate) fn resolve_helper(host: &str, output: &mut impl Write) -> Result<(), ProbeError> {
    let request = TargetRequest::new(host, 1)?;
    let ProbeHost::Dns(name) = request.host() else {
        return Err(ProbeError::InvalidTarget);
    };
    let answers = (name.as_str(), 1)
        .to_socket_addrs()
        .map_err(|_| ProbeError::DnsFailed)?;
    for address in answers.take(9) {
        writeln!(output, "{}", address.ip()).map_err(|_| ProbeError::DnsFailed)?;
    }
    Ok(())
}

fn check(deadline: Instant, cancellation: &Cancellation) -> Result<(), ProbeError> {
    if cancellation.is_cancelled() {
        Err(ProbeError::Cancelled)
    } else if Instant::now() >= deadline {
        Err(ProbeError::DeadlineExceeded)
    } else {
        Ok(())
    }
}

/// Always reap the resolver, including when interrupted between polling steps.
struct ChildGuard(Child);
impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
fn collect_child(
    child: Child,
    deadline: Instant,
    cancellation: &Cancellation,
) -> Result<Vec<u8>, ProbeError> {
    let mut child = ChildGuard(child);
    loop {
        check(deadline, cancellation)?;
        if let Some(status) = child.0.try_wait().map_err(|_| ProbeError::DnsFailed)? {
            if !status.success() {
                return Err(ProbeError::DnsFailed);
            }
            let mut output = Vec::new();
            child
                .0
                .stdout
                .take()
                .ok_or(ProbeError::DnsFailed)?
                .take((OUTPUT_LIMIT + 1) as u64)
                .read_to_end(&mut output)
                .map_err(|_| ProbeError::DnsFailed)?;
            check(deadline, cancellation)?;
            return Ok(output);
        }
        std::thread::sleep(
            Duration::from_millis(5).min(deadline.saturating_duration_since(Instant::now())),
        );
    }
}
fn parse_answers(output: &[u8]) -> Result<Vec<IpAddr>, ProbeError> {
    if output.len() > OUTPUT_LIMIT {
        return Err(ProbeError::TooManyAnswers);
    }
    let input = std::str::from_utf8(output).map_err(|_| ProbeError::DnsFailed)?;
    let mut result = Vec::new();
    for line in input.lines() {
        if result.len() == 8 {
            return Err(ProbeError::TooManyAnswers);
        }
        result.push(line.parse().map_err(|_| ProbeError::DnsFailed)?);
    }
    if result.is_empty() {
        return Err(ProbeError::DnsFailed);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pqc_scan_core::target_policy::{Cancellation, ProbeError};
    use std::{
        process::{Command, Stdio},
        time::{Duration, Instant},
    };

    #[test]
    fn resolver_output_is_bounded_and_typed() {
        assert_eq!(parse_answers(b"127.0.0.1\n::1\n").unwrap().len(), 2);
        assert_eq!(parse_answers(b"garbage\n"), Err(ProbeError::DnsFailed));
        assert_eq!(
            parse_answers(&b"127.0.0.1\n".repeat(9)),
            Err(ProbeError::TooManyAnswers)
        );
        assert_eq!(parse_answers(&[b'a'; 513]), Err(ProbeError::TooManyAnswers));
    }

    #[cfg(unix)]
    #[test]
    fn slow_resolver_process_is_killed_and_reaped_on_timeout_and_cancellation() {
        for cancelled in [false, true] {
            let cancellation = Cancellation::new();
            let mut command = Command::new("/bin/sleep");
            command
                .arg("30")
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::null());
            if cancelled {
                cancellation.cancel();
            }
            let start = Instant::now();
            let child = command.spawn().unwrap();
            let pid = child.id() as libc::pid_t;
            let result = collect_child(child, start + Duration::from_millis(30), &cancellation);
            assert_eq!(
                result,
                Err(if cancelled {
                    ProbeError::Cancelled
                } else {
                    ProbeError::DeadlineExceeded
                })
            );
            assert!(start.elapsed() < Duration::from_secs(1));
            let mut status = 0;
            // SAFETY: pid came from our own spawned child; status is a valid out pointer.
            assert_eq!(
                unsafe { libc::waitpid(pid, &mut status, libc::WNOHANG) },
                -1
            );
            assert_eq!(
                std::io::Error::last_os_error().raw_os_error(),
                Some(libc::ECHILD)
            );
        }
    }
}
