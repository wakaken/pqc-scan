//! Optional single-connection transport. It never accepts a hostname, retries,
//! reads proxy configuration, or exposes an unmetered socket to a protocol.
use crate::target_policy::{AuthorizedTarget, Cancellation, ProbeBudget, ProbeError};
use socket2::{Domain, Protocol, Socket, Type};
use std::{
    io::{self, Read, Write},
    net::{Shutdown, TcpStream},
    sync::{Arc, Condvar, Mutex, Weak},
    thread,
    time::{Duration, Instant},
};
const POLL_INTERVAL: Duration = Duration::from_millis(5);

#[derive(Debug)]
pub struct BoundedTcpStream {
    socket: Arc<TcpStream>,
    budget: ProbeBudget,
    protocol_deadline: Instant,
    closed: bool,
    watchdog: DeadlineWatchdog,
}
impl BoundedTcpStream {
    pub fn connect(target: AuthorizedTarget, budget: ProbeBudget) -> Result<Self, ProbeError> {
        target.check_budget(&budget)?;
        budget.begin_attempt()?;
        let deadline = budget.phase_deadline(budget.limits().tcp_timeout);
        budget.check(deadline)?;
        let socket = Socket::new(
            Domain::for_address(target.socket_addr()),
            Type::STREAM,
            Some(Protocol::TCP),
        )
        .map_err(|_| ProbeError::Io)?;
        socket.set_nonblocking(true).map_err(|_| ProbeError::Io)?;
        if let Err(error) = socket.connect(&target.socket_addr().into()) {
            if !pending_connect(&error) {
                return Err(ProbeError::Io);
            }
        }
        loop {
            budget.check(deadline)?;
            if socket.take_error().map_err(|_| ProbeError::Io)?.is_some() {
                return Err(ProbeError::Io);
            }
            if let Ok(peer) = socket.peer_addr() {
                target.check_peer(peer.as_socket().ok_or(ProbeError::PeerMismatch)?)?;
                break;
            }
            pause(&budget, deadline)?;
        }
        budget.check(deadline)?;
        let protocol_deadline = budget.phase_deadline(budget.limits().protocol_timeout);
        let socket = Arc::new(TcpStream::from(socket));
        budget.cancellation().register(&socket)?;
        let watchdog = DeadlineWatchdog::start(
            Arc::downgrade(&socket),
            protocol_deadline,
            budget.cancellation().clone(),
        )?;
        budget.check(protocol_deadline)?;
        Ok(Self {
            socket,
            budget,
            protocol_deadline,
            closed: false,
            watchdog,
        })
    }
    pub fn budget(&self) -> &ProbeBudget {
        &self.budget
    }
    pub fn read_probe(&mut self, buffer: &mut [u8]) -> Result<usize, ProbeError> {
        let result = self.read_inner(buffer);
        if result.is_err() {
            self.close();
        }
        result
    }
    fn read_inner(&mut self, buffer: &mut [u8]) -> Result<usize, ProbeError> {
        self.check()?;
        if buffer.is_empty() {
            return Ok(0);
        }
        let len = buffer.len().min(self.budget.read_remaining() as usize);
        if len == 0 {
            return Err(ProbeError::TrafficLimit);
        }
        loop {
            self.check()?;
            match (&*self.socket).read(&mut buffer[..len]) {
                Ok(n) => {
                    self.budget.debit_read(n);
                    self.check()?;
                    return Ok(n);
                }
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    pause(&self.budget, self.protocol_deadline)?
                }
                Err(_) => {
                    self.check()?;
                    return Err(ProbeError::Io);
                }
            }
        }
    }
    pub fn write_probe(&mut self, buffer: &[u8]) -> Result<usize, ProbeError> {
        let result = self.write_inner(buffer);
        if result.is_err() {
            self.close();
        }
        result
    }
    fn write_inner(&mut self, buffer: &[u8]) -> Result<usize, ProbeError> {
        self.check()?;
        if buffer.is_empty() {
            return Ok(0);
        }
        let len = buffer.len().min(self.budget.write_remaining() as usize);
        if len == 0 {
            return Err(ProbeError::TrafficLimit);
        }
        loop {
            self.check()?;
            match (&*self.socket).write(&buffer[..len]) {
                Ok(n) => {
                    self.budget.debit_write(n);
                    self.check()?;
                    return Ok(n);
                }
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    pause(&self.budget, self.protocol_deadline)?
                }
                Err(_) => {
                    self.check()?;
                    return Err(ProbeError::Io);
                }
            }
        }
    }
    fn check(&self) -> Result<(), ProbeError> {
        self.budget.check(self.protocol_deadline)?;
        if self.closed {
            Err(ProbeError::Io)
        } else {
            Ok(())
        }
    }
    pub fn close(&mut self) {
        if !self.closed {
            let _ = self.socket.shutdown(Shutdown::Both);
            self.closed = true;
        }
        self.watchdog.stop();
    }
}
fn pause(budget: &ProbeBudget, deadline: Instant) -> Result<(), ProbeError> {
    thread::sleep(POLL_INTERVAL.min(budget.remaining(deadline)?));
    budget.check(deadline)
}
fn pending_connect(error: &io::Error) -> bool {
    if error.kind() == io::ErrorKind::WouldBlock {
        return true;
    }
    #[cfg(unix)]
    if error.raw_os_error() == Some(libc::EINPROGRESS) {
        return true;
    }
    false
}
fn io_error(error: ProbeError) -> io::Error {
    let kind = match error {
        ProbeError::Cancelled => io::ErrorKind::ConnectionAborted,
        ProbeError::DeadlineExceeded => io::ErrorKind::TimedOut,
        ProbeError::TrafficLimit => io::ErrorKind::Other,
        _ => io::ErrorKind::Other,
    };
    io::Error::new(kind, error)
}
impl Read for BoundedTcpStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.read_probe(buf).map_err(io_error)
    }
}
impl Write for BoundedTcpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.write_probe(buf).map_err(io_error)
    }
    fn flush(&mut self) -> io::Result<()> {
        let result = self.check();
        if result.is_err() {
            self.close();
        }
        result.map_err(io_error)
    }
}
impl Drop for BoundedTcpStream {
    fn drop(&mut self) {
        self.close();
    }
}

/// One bounded, joined watchdog per connected socket. Unlike a blocked DNS
/// worker, it only waits on a condition variable and never performs unbounded I/O.
#[derive(Debug)]
struct DeadlineWatchdog {
    stop: Arc<(Mutex<bool>, Condvar)>,
    thread: Option<thread::JoinHandle<()>>,
}
impl DeadlineWatchdog {
    fn start(
        socket: Weak<TcpStream>,
        deadline: Instant,
        cancellation: Cancellation,
    ) -> Result<Self, ProbeError> {
        let stop = Arc::new((Mutex::new(false), Condvar::new()));
        let worker_stop = Arc::clone(&stop);
        let thread = thread::Builder::new()
            .name("pqc-probe-deadline".into())
            .spawn(move || {
                let (lock, wake) = &*worker_stop;
                let mut stopped = lock.lock().unwrap_or_else(|error| error.into_inner());
                loop {
                    if *stopped || cancellation.is_cancelled() {
                        return;
                    }
                    let remaining = deadline.saturating_duration_since(Instant::now());
                    if remaining.is_zero() {
                        if let Some(socket) = socket.upgrade() {
                            let _ = socket.shutdown(Shutdown::Both);
                        }
                        return;
                    }
                    let waited = wake
                        .wait_timeout(stopped, remaining.min(POLL_INTERVAL))
                        .unwrap_or_else(|error| error.into_inner());
                    stopped = waited.0;
                }
            })
            .map_err(|_| ProbeError::Io)?;
        Ok(Self {
            stop,
            thread: Some(thread),
        })
    }
    fn stop(&mut self) {
        let (lock, wake) = &*self.stop;
        {
            let mut stopped = lock.lock().unwrap_or_else(|error| error.into_inner());
            *stopped = true;
            wake.notify_one();
        }
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}
impl Drop for DeadlineWatchdog {
    fn drop(&mut self) {
        self.stop();
    }
}
