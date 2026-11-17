//! Link-safe report output. All path traversal uses held directory handles;
//! existing files are replaced, never opened for truncation (including hard links).
use std::{io, path::Path};

/// Create an output directory without following checkout-controlled links.
pub fn create_dir_all(path: &Path) -> io::Result<()> {
    platform::directory(path).map(|_| ())
}

/// Publish one complete file. This is not a transaction across a report set.
pub fn write(path: &Path, contents: impl AsRef<[u8]>) -> io::Result<()> {
    platform::write(path, contents.as_ref())
}

#[cfg(unix)]
mod platform {
    use std::{
        ffi::{CString, OsStr},
        fs::File,
        io::{self, Write},
        os::{
            fd::{AsRawFd, FromRawFd},
            unix::{ffi::OsStrExt, fs::OpenOptionsExt},
        },
        path::{Component, Path, PathBuf},
        sync::atomic::{AtomicU64, Ordering},
        time::{SystemTime, UNIX_EPOCH},
    };

    fn name(value: &OsStr) -> io::Result<CString> {
        CString::new(value.as_bytes())
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid output path"))
    }

    fn open_directory(parent: &File, component: &CString) -> io::Result<File> {
        // SAFETY: the held parent fd is live and component is NUL-terminated.
        let fd = unsafe {
            libc::openat(
                parent.as_raw_fd(),
                component.as_ptr(),
                libc::O_RDONLY
                    | libc::O_DIRECTORY
                    | libc::O_NOFOLLOW
                    | libc::O_NONBLOCK
                    | libc::O_CLOEXEC,
            )
        };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: openat transferred ownership of this fd exactly once.
        Ok(unsafe { File::from_raw_fd(fd) })
    }

    fn physical_system_alias(path: &Path) -> PathBuf {
        // Darwin's root-controlled OS aliases are ordinary absolute CLI paths.
        // Rewrite only these exact system prefixes, not arbitrary output links.
        #[cfg(target_os = "macos")]
        for (alias, physical) in [
            ("/tmp", "/private/tmp"),
            ("/var", "/private/var"),
            ("/etc", "/private/etc"),
        ] {
            if let Ok(rest) = path.strip_prefix(alias) {
                if std::fs::read_link(alias)
                    .ok()
                    .map(|target| Path::new("/").join(target))
                    .as_deref()
                    == Some(Path::new(physical))
                {
                    return Path::new(physical).join(rest);
                }
            }
        }
        path.to_owned()
    }

    pub(super) fn directory(path: &Path) -> io::Result<File> {
        directory_with_hook(path, |_| {})
    }

    fn directory_with_hook(
        path: &Path,
        mut after_component: impl FnMut(&[u8]),
    ) -> io::Result<File> {
        let path = physical_system_alias(path);
        let origin = std::fs::OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC)
            .open("/")?;
        let mut directories = vec![origin];
        if !path.is_absolute() {
            use std::os::unix::fs::MetadataExt;
            // Acquire cwd's ancestry downward from /, too. Chasing live `..`
            // upward could itself race an ancestor's reparenting. Verify that
            // this chain reaches the actual cwd before using relative outputs.
            let cwd = File::open(".")?.metadata()?;
            for component in physical_system_alias(&std::env::current_dir()?).components() {
                if let Component::Normal(part) = component {
                    let child = open_directory(directories.last().unwrap(), &name(part)?)?;
                    directories.push(child);
                }
            }
            let reached = directories.last().unwrap().metadata()?;
            if (cwd.dev(), cwd.ino()) != (reached.dev(), reached.ino()) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "output working directory changed",
                ));
            }
        }
        for part in path.components() {
            let part = match part {
                Component::RootDir | Component::CurDir => continue,
                Component::Normal(value) => name(value)?,
                Component::ParentDir => {
                    if directories.len() > 1 {
                        directories.pop();
                    }
                    continue;
                }
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "invalid output directory",
                    ))
                }
            };
            let parent = directories.last().unwrap();
            let child = match open_directory(parent, &part) {
                Ok(file) => file,
                Err(error) if error.kind() == io::ErrorKind::NotFound => {
                    // SAFETY: valid live parent and a single NUL-terminated component.
                    if unsafe { libc::mkdirat(parent.as_raw_fd(), part.as_ptr(), 0o777) } != 0 {
                        let error = io::Error::last_os_error();
                        if error.kind() != io::ErrorKind::AlreadyExists {
                            return Err(error);
                        }
                    }
                    // Reopen without following links even if another process won mkdirat.
                    open_directory(parent, &part)?
                }
                Err(error) => return Err(error),
            };
            directories.push(child);
            after_component(part.as_bytes());
        }
        Ok(directories.pop().unwrap())
    }

    fn check_destination(parent: &File, destination: &CString) -> io::Result<()> {
        let mut metadata = std::mem::MaybeUninit::<libc::stat>::uninit();
        // SAFETY: valid parent, NUL-terminated name and writable stat buffer.
        if unsafe {
            libc::fstatat(
                parent.as_raw_fd(),
                destination.as_ptr(),
                metadata.as_mut_ptr(),
                libc::AT_SYMLINK_NOFOLLOW,
            )
        } != 0
        {
            let error = io::Error::last_os_error();
            return if error.kind() == io::ErrorKind::NotFound {
                Ok(())
            } else {
                Err(error)
            };
        }
        // SAFETY: successful fstatat initialized the complete buffer.
        let metadata = unsafe { metadata.assume_init() };
        if metadata.st_mode & libc::S_IFMT != libc::S_IFREG {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "output destination is not a regular file",
            ));
        }
        Ok(())
    }

    struct Staging<'a> {
        parent: &'a File,
        name: CString,
        armed: bool,
    }
    impl Drop for Staging<'_> {
        fn drop(&mut self) {
            if !self.armed {
                return;
            }
            // SAFETY: live borrowed parent and NUL-terminated single component.
            unsafe {
                libc::unlinkat(self.parent.as_raw_fd(), self.name.as_ptr(), 0);
            }
        }
    }

    pub(super) fn write(path: &Path, contents: &[u8]) -> io::Result<()> {
        let destination = match path.components().next_back() {
            Some(Component::Normal(value)) => name(value)?,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "invalid output filename",
                ))
            }
        };
        let parent = directory(path.parent().unwrap_or_else(|| Path::new(".")))?;
        publish(&parent, &destination, contents, || {})
    }

    fn publish(
        parent: &File,
        destination: &CString,
        contents: &[u8],
        before_rename: impl FnOnce(),
    ) -> io::Result<()> {
        check_destination(parent, destination)?;
        static SEQUENCE: AtomicU64 = AtomicU64::new(0);
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(io::Error::other)?
            .as_nanos();
        let temporary = name(OsStr::new(&format!(
            ".pqc-output-{}-{nonce}-{}",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        )))?;
        // SAFETY: live parent and valid name. Exclusive creation cannot follow or
        // truncate an attacker-provided inode. Private mode protects partial data.
        let fd = unsafe {
            libc::openat(
                parent.as_raw_fd(),
                temporary.as_ptr(),
                libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL | libc::O_NOFOLLOW | libc::O_CLOEXEC,
                0o600,
            )
        };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        let mut staging = Staging {
            parent,
            name: temporary,
            armed: true,
        };
        // SAFETY: exclusive openat returned this owned fd.
        let mut file = unsafe { File::from_raw_fd(fd) };
        file.write_all(contents)?;
        check_destination(parent, destination)?;
        before_rename();
        // SAFETY: both names are resolved relative to the same live directory.
        // renameat replaces a raced final symlink itself; it never follows it.
        if unsafe {
            libc::renameat(
                parent.as_raw_fd(),
                staging.name.as_ptr(),
                parent.as_raw_fd(),
                destination.as_ptr(),
            )
        } != 0
        {
            return Err(io::Error::last_os_error());
        }
        staging.armed = false;
        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::os::unix::fs::symlink;

        struct Scratch(PathBuf);
        impl Scratch {
            fn new() -> Self {
                static NEXT: AtomicU64 = AtomicU64::new(0);
                let path = std::env::temp_dir().canonicalize().unwrap().join(format!(
                    "pqc-safe-output-{}-{}",
                    std::process::id(),
                    NEXT.fetch_add(1, Ordering::Relaxed)
                ));
                std::fs::create_dir(&path).unwrap();
                Self(path)
            }
        }
        impl Drop for Scratch {
            fn drop(&mut self) {
                let _ = std::fs::remove_dir_all(&self.0);
            }
        }

        #[test]
        fn regular_output_is_replaced_without_truncating_hard_link_targets() {
            let root = Scratch::new();
            let target = root.0.join("outside");
            std::fs::write(&target, b"CANARY").unwrap();
            let output = root.0.join("new/nested/report.json");
            super::write(&output, b"first").unwrap();
            super::write(&output, b"second").unwrap();
            assert_eq!(std::fs::read(&output).unwrap(), b"second");
            std::fs::remove_file(&output).unwrap();
            std::fs::hard_link(&target, &output).unwrap();
            super::write(&output, b"replacement").unwrap();
            assert_eq!(std::fs::read(target).unwrap(), b"CANARY");
            assert_eq!(std::fs::read(&output).unwrap(), b"replacement");
            assert_eq!(
                std::fs::read_dir(output.parent().unwrap()).unwrap().count(),
                1
            );
        }

        #[test]
        fn final_and_parent_links_are_rejected_including_dangling_links() {
            let root = Scratch::new();
            let target = root.0.join("outside");
            std::fs::write(&target, b"CANARY").unwrap();
            let output = root.0.join("report.json");
            for link in [&target, &root.0.join("absent")] {
                symlink(link, &output).unwrap();
                assert!(super::write(&output, b"ATTACK").is_err());
                std::fs::remove_file(&output).unwrap();
            }
            symlink(&root.0, root.0.join("alias")).unwrap();
            assert!(super::write(&root.0.join("alias/nested/report.json"), b"ATTACK").is_err());
            assert!(!root.0.join("nested").exists());
            assert_eq!(std::fs::read(target).unwrap(), b"CANARY");
            assert!(!root.0.join("absent").exists());
        }

        #[test]
        fn fifo_and_directory_destinations_fail_without_opening_them() {
            let root = Scratch::new();
            let fifo = root.0.join("pipe");
            let cpath = name(fifo.as_os_str()).unwrap();
            // SAFETY: valid test-local NUL-terminated path.
            assert_eq!(unsafe { libc::mkfifo(cpath.as_ptr(), 0o600) }, 0);
            assert!(super::write(&fifo, b"ATTACK").is_err());
            assert!(super::write(&root.0, b"ATTACK").is_err());
            assert_eq!(std::fs::read_dir(&root.0).unwrap().count(), 1);
        }

        #[test]
        fn held_parent_survives_rename_and_replacement_with_link() {
            let root = Scratch::new();
            let output = root.0.join("out");
            let moved = root.0.join("moved");
            let outside = root.0.join("outside");
            std::fs::create_dir(&outside).unwrap();
            std::fs::write(outside.join("report.json"), b"CANARY").unwrap();
            let parent = directory(&output).unwrap();
            publish(
                &parent,
                &name(OsStr::new("report.json")).unwrap(),
                b"REPORT",
                || {
                    std::fs::rename(&output, &moved).unwrap();
                    symlink(&outside, &output).unwrap();
                },
            )
            .unwrap();
            assert_eq!(
                std::fs::read(outside.join("report.json")).unwrap(),
                b"CANARY"
            );
            assert_eq!(std::fs::read(moved.join("report.json")).unwrap(), b"REPORT");
        }

        #[test]
        fn destination_link_inserted_after_final_check_is_replaced_not_followed() {
            let root = Scratch::new();
            let outside = root.0.join("outside");
            std::fs::write(&outside, b"CANARY").unwrap();
            let parent = directory(&root.0).unwrap();
            publish(
                &parent,
                &name(OsStr::new("report.json")).unwrap(),
                b"REPORT",
                || {
                    symlink(&outside, root.0.join("report.json")).unwrap();
                },
            )
            .unwrap();
            assert_eq!(std::fs::read(outside).unwrap(), b"CANARY");
            assert_eq!(
                std::fs::read(root.0.join("report.json")).unwrap(),
                b"REPORT"
            );
            assert!(!std::fs::symlink_metadata(root.0.join("report.json"))
                .unwrap()
                .file_type()
                .is_symlink());
            assert_eq!(std::fs::read_dir(&root.0).unwrap().count(), 2);
        }

        #[test]
        fn failed_publish_removes_staging_without_replacing_a_directory() {
            let root = Scratch::new();
            let parent = directory(&root.0).unwrap();
            assert!(publish(
                &parent,
                &name(OsStr::new("report.json")).unwrap(),
                b"REPORT",
                || {
                    std::fs::create_dir(root.0.join("report.json")).unwrap();
                }
            )
            .is_err());
            assert!(root.0.join("report.json").is_dir());
            assert_eq!(std::fs::read_dir(&root.0).unwrap().count(), 1);
        }

        #[test]
        fn parent_components_return_to_the_original_held_ancestors() {
            let root = Scratch::new();
            let selected = root.0.join("selected");
            std::fs::create_dir_all(selected.join("a/b")).unwrap();
            std::fs::write(root.0.join("report.json"), b"CANARY").unwrap();
            let parent = directory_with_hook(&selected.join("a/b/../.."), |component| {
                if component == b"b" {
                    std::fs::rename(selected.join("a/b"), selected.join("moved")).unwrap();
                }
            })
            .unwrap();
            publish(
                &parent,
                &name(OsStr::new("report.json")).unwrap(),
                b"REPORT",
                || {},
            )
            .unwrap();
            assert_eq!(
                std::fs::read(root.0.join("report.json")).unwrap(),
                b"CANARY"
            );
            assert_eq!(
                std::fs::read(selected.join("report.json")).unwrap(),
                b"REPORT"
            );
        }

        #[test]
        fn leading_relative_parent_components_keep_their_normal_meaning() {
            use std::os::unix::fs::MetadataExt;
            let cwd = std::env::current_dir().unwrap();
            let expected = std::fs::metadata(cwd.parent().unwrap()).unwrap();
            let actual = directory(Path::new("..")).unwrap().metadata().unwrap();
            assert_eq!(
                (actual.dev(), actual.ino()),
                (expected.dev(), expected.ino())
            );
            let roundtrip = directory(&Path::new("..").join(cwd.file_name().unwrap()))
                .unwrap()
                .metadata()
                .unwrap();
            let expected = std::fs::metadata(&cwd).unwrap();
            assert_eq!(
                (roundtrip.dev(), roundtrip.ino()),
                (expected.dev(), expected.ino())
            );
        }

        #[cfg(target_os = "macos")]
        #[test]
        fn darwin_system_alias_still_works_but_nested_links_do_not() {
            let root = Scratch::new();
            let physical = root.0.strip_prefix("/private/var").unwrap();
            let alias = Path::new("/var").join(physical);
            super::write(&alias.join("report.json"), b"REPORT").unwrap();
            assert_eq!(
                std::fs::read(root.0.join("report.json")).unwrap(),
                b"REPORT"
            );
            symlink(&root.0, root.0.join("nested-link")).unwrap();
            assert!(super::write(&alias.join("nested-link/report.json"), b"ATTACK").is_err());
            assert_eq!(
                std::fs::read(root.0.join("report.json")).unwrap(),
                b"REPORT"
            );
        }
    }
}

#[cfg(not(unix))]
mod platform {
    use std::{io, path::Path};
    pub(super) fn directory(_: &Path) -> io::Result<()> {
        Err(unsupported())
    }
    pub(super) fn write(_: &Path, _: &[u8]) -> io::Result<()> {
        Err(unsupported())
    }
    fn unsupported() -> io::Error {
        io::Error::new(
            io::ErrorKind::Unsupported,
            "link-safe report output is unavailable on this platform",
        )
    }
}
