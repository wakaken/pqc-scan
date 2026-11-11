//! Bounded reads of repository files; never follow in-tree symbolic links.
use std::fs::File;
use std::io::{self, Read};
use std::path::{Component, Path};

pub(crate) fn read(root: &Path, path: &Path, max: usize) -> io::Result<Vec<u8>> {
    let relative = path.strip_prefix(root).map_err(|_| invalid())?;
    let parts: Vec<_> = relative.components().collect();
    if parts.iter().any(|c| !matches!(c, Component::Normal(_))) {
        return Err(invalid());
    }
    let file = open(root, &parts)?;
    if !file.metadata()?.is_file() || file.metadata()?.len() > max as u64 {
        return Err(invalid());
    }
    read_bounded(file, max)
}

fn read_bounded(reader: impl Read, max: usize) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    reader
        .take((max as u64).saturating_add(1))
        .read_to_end(&mut bytes)?;
    if bytes.len() > max {
        return Err(invalid());
    }
    Ok(bytes)
}

fn invalid() -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        "repository input is not a bounded regular file",
    )
}

#[cfg(unix)]
fn open(root: &Path, parts: &[Component<'_>]) -> io::Result<File> {
    use std::ffi::CString;
    use std::os::fd::{AsRawFd, FromRawFd};
    use std::os::unix::{ffi::OsStrExt, fs::OpenOptionsExt};
    if parts.is_empty() {
        return std::fs::OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC)
            .open(root);
    }
    // The explicitly selected root may itself be a link (e.g. /tmp on macOS).
    // Subsequent components are resolved relative to held directory handles.
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_DIRECTORY | libc::O_NONBLOCK | libc::O_CLOEXEC)
        .open(root)?;
    for (index, part) in parts.iter().enumerate() {
        let name = CString::new(part.as_os_str().as_bytes()).map_err(|_| invalid())?;
        let directory = if index + 1 < parts.len() {
            libc::O_DIRECTORY
        } else {
            0
        };
        // SAFETY: valid held parent fd and a NUL-terminated single path component.
        let fd = unsafe {
            libc::openat(
                file.as_raw_fd(),
                name.as_ptr(),
                libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC | directory,
            )
        };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: openat returned an owned fd; File closes it exactly once.
        file = unsafe { File::from_raw_fd(fd) };
    }
    Ok(file)
}

#[cfg(windows)]
fn open(root: &Path, parts: &[Component<'_>]) -> io::Result<File> {
    use std::os::windows::ffi::OsStringExt;
    use std::os::windows::{fs::OpenOptionsExt, io::AsRawHandle};
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetFinalPathNameByHandleW(
            handle: *mut std::ffi::c_void,
            buffer: *mut u16,
            size: u32,
            flags: u32,
        ) -> u32;
    }
    let canonical_root = root.canonicalize()?;
    let mut path = root.to_path_buf();
    for part in parts {
        path.push(part);
        if std::fs::symlink_metadata(&path)?.file_type().is_symlink() {
            return Err(invalid());
        }
    }
    let file = std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(0x00200000) // FILE_FLAG_OPEN_REPARSE_POINT
        .open(path)?;
    if !file.metadata()?.is_file() {
        return Err(invalid());
    }
    let mut buffer = vec![0u16; 32768];
    // SAFETY: valid owned file handle and writable buffer of the advertised length.
    let len = unsafe {
        GetFinalPathNameByHandleW(
            file.as_raw_handle(),
            buffer.as_mut_ptr(),
            buffer.len() as u32,
            0,
        )
    };
    if len == 0 || len as usize >= buffer.len() {
        return Err(invalid());
    }
    let actual = std::path::PathBuf::from(std::ffi::OsString::from_wide(&buffer[..len as usize]));
    if !actual.starts_with(canonical_root) {
        return Err(invalid());
    }
    Ok(file)
}

#[cfg(not(any(unix, windows)))]
fn open(_root: &Path, _parts: &[Component<'_>]) -> io::Result<File> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "link-safe repository reads are unsupported on this platform",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn growing_input_is_stopped_at_limit_plus_one() {
        let mut reader = std::io::Cursor::new(b"abcdeMORE".to_vec());
        assert!(read_bounded(&mut reader, 4).is_err());
        assert_eq!(reader.position(), 5);
        assert_eq!(
            read_bounded(std::io::Cursor::new(b"abcd"), 4).unwrap(),
            b"abcd"
        );
        assert!(read_bounded(std::io::Cursor::new(b"x"), 0).is_err());
        assert!(read_bounded(std::io::Cursor::new(b""), 0)
            .unwrap()
            .is_empty());
    }

    #[cfg(unix)]
    #[test]
    fn read_bounds_and_rejects_replaced_files_and_parent_links() {
        use std::os::unix::fs::symlink;
        let root = std::env::temp_dir().join(format!("pqc-safe-read-{}", std::process::id()));
        std::fs::create_dir_all(root.join("nested")).unwrap();
        std::fs::write(root.join("nested/file"), b"abcd").unwrap();
        let path = root.join("nested/file");
        assert_eq!(read(&root, &path, 4).unwrap(), b"abcd");
        assert_eq!(read(&path, &path, 4).unwrap(), b"abcd");
        assert!(read(&root, &path, 3).is_err());
        assert!(read(&root, &root.join("nested/../nested/file"), 4).is_err());
        std::fs::rename(root.join("nested"), root.join("moved")).unwrap();
        symlink(root.join("moved"), root.join("nested")).unwrap();
        assert!(read(&root, &path, 4).is_err());
        std::fs::remove_file(root.join("nested")).unwrap();
        symlink(root.join("moved/file"), root.join("link")).unwrap();
        assert!(read(&root, &root.join("link"), 4).is_err());
        assert!(std::process::Command::new("mkfifo")
            .arg(root.join("pipe"))
            .status()
            .unwrap()
            .success());
        assert!(read(&root, &root.join("pipe"), 4).is_err());
        std::fs::remove_dir_all(root).unwrap();
    }
}
