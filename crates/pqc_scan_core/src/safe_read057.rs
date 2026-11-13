//! Real link boundaries. Hard links intentionally demonstrate the documented limit.
use super::*;
use std::{
    fs,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, Barrier,
    },
};
fn fixture(tag: &str) -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!("pqc057-{}-{tag}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    root
}
#[test]
fn parent_link_replacement_never_reads_the_outside_control() {
    use std::os::unix::fs::symlink;
    let base = fixture("race");
    let root = base.join("root");
    let outside = base.join("outside");
    fs::create_dir_all(root.join("parent")).unwrap();
    fs::create_dir(&outside).unwrap();
    fs::write(root.join("parent/file"), b"INSIDE").unwrap();
    fs::write(outside.join("file"), b"OUTSIDE").unwrap();
    fs::rename(root.join("parent"), root.join("held")).unwrap();
    symlink(&outside, root.join("parent")).unwrap();
    assert_eq!(fs::read(root.join("parent/file")).unwrap(), b"OUTSIDE");
    assert!(read(&root, &root.join("parent/file"), 32).is_err());
    fs::remove_file(root.join("parent")).unwrap();
    fs::rename(root.join("held"), root.join("parent")).unwrap();
    let ready = Arc::new(Barrier::new(2));
    let writer_ready = ready.clone();
    let swaps = Arc::new(AtomicUsize::new(0));
    let writer_swaps = swaps.clone();
    let stop = Arc::new(AtomicBool::new(false));
    let signal = stop.clone();
    let moved = root.clone();
    let target = outside.clone();
    let writer = std::thread::spawn(move || {
        let mut first = true;
        while !signal.load(Ordering::Relaxed) {
            fs::rename(moved.join("parent"), moved.join("held")).unwrap();
            symlink(&target, moved.join("parent")).unwrap();
            writer_swaps.fetch_add(1, Ordering::Relaxed);
            if first {
                writer_ready.wait();
                first = false;
            }
            std::thread::yield_now();
            fs::remove_file(moved.join("parent")).unwrap();
            fs::rename(moved.join("held"), moved.join("parent")).unwrap();
        }
    });
    ready.wait();
    let mut bad = false;
    let mut successes = 0;
    for _ in 0..2000 {
        if let Ok(bytes) = read(&root, &root.join("parent/file"), 32) {
            successes += 1;
            bad |= bytes != b"INSIDE";
        }
    }
    stop.store(true, Ordering::Relaxed);
    writer.join().unwrap();
    assert!(swaps.load(Ordering::Relaxed) > 0);
    assert!(!bad);
    assert_eq!(
        read(&root, &root.join("parent/file"), 32).unwrap(),
        b"INSIDE"
    );
    if let Some(dir) = std::env::var_os("PQC_QUALITY_EVIDENCE_DIR") {
        fs::create_dir_all(&dir).unwrap();
        fs::write(std::path::Path::new(&dir).join("filesystem057.json"),serde_json::to_vec(&serde_json::json!({"raceAttempts":2000,"linkSwaps":swaps.load(Ordering::Relaxed),"outsideControlReadableByOrdinaryRead":true,"deterministicLinkRejected":true,"successfulInsideReads":successes,"outsideReads":0})).unwrap()).unwrap();
    }
    fs::remove_dir_all(base).unwrap();
}
#[test]
fn hard_links_are_bounded_regular_files_not_proof_of_origin() {
    let base = fixture("hardlink");
    let root = base.join("root");
    fs::create_dir(&root).unwrap();
    fs::write(base.join("public-control"), b"PUBLIC_HARDLINK_CONTROL").unwrap();
    fs::hard_link(base.join("public-control"), root.join("linked")).unwrap();
    assert_eq!(
        read(&root, &root.join("linked"), 64).unwrap(),
        b"PUBLIC_HARDLINK_CONTROL"
    );
    assert!(read(&root, &root.join("linked"), 4).is_err());
    fs::remove_dir_all(base).unwrap();
}
