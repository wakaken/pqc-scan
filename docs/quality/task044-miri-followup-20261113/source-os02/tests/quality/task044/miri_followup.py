"""Reproduce and diagnose Miri using original crate inputs in strict confinement."""
import hashlib
import json
import os
from pathlib import Path
import sys
import traceback

sys.path.insert(0, '/source/tests/protocol')
sys.path.insert(0, '/source/tests/quality')
from container_entry import boundary, EVIDENCE, SCRATCH, write
from task044.entry import run, NIGHTLY


def main():
    write('checks.json', boundary())
    (SCRATCH/'test-target').mkdir()
    cargo = SCRATCH/'cargo'
    cargo.mkdir()
    for name in ('registry', 'git'):
        cache = Path('/usr/local/cargo')/name
        if cache.exists():
            (cargo/name).symlink_to(cache, target_is_directory=True)
    env = dict(os.environ, CARGO_TARGET_DIR='/scratch/miri-original',
               MIRI_SYSROOT='/opt/miri-sysroot', RUSTUP_TOOLCHAIN=NIGHTLY)
    for label, command in [('rustc',['rustc','+'+NIGHTLY,'-Vv']),
                           ('cargo',['cargo','+'+NIGHTLY,'-Vv']),
                           ('miri',['cargo','+'+NIGHTLY,'miri','--version'])]:
        run(command,label+'.log',env=env)
    result = run(['cargo','+'+NIGHTLY,'miri','test','-vv','-p','pqc_scan_core',
                  '--lib','identity::tests','--locked','--offline'],
                 'original-core.log',env=env,timeout=180,required=False)
    files = {}
    for path in (SCRATCH/'miri-original').rglob('build-script-build'):
        data = path.read_bytes()
        files[str(path)]={'length':len(data),'magicHex':data[:16].hex(),
                         'sha256':hashlib.sha256(data).hexdigest()}
    write('failed-build-files.json',files)
    results={'originalCore':result}
    for has_build in (False, True):
        label='minimal-build' if has_build else 'minimal-plain'
        folder=SCRATCH/label
        (folder/'src').mkdir(parents=True)
        (folder/'Cargo.toml').write_text('[package]\nname="'+label+'"\nversion="0.0.0"\nedition="2021"\n')
        (folder/'src/lib.rs').write_text('pub fn sum(a:u32,b:u32)->u32{a+b}\n#[test] fn check(){assert_eq!(sum(2,3),5);}\n')
        if has_build:
            (folder/'build.rs').write_text('fn main(){println!("cargo:rerun-if-changed=build.rs");}\n')
        mode_env=dict(env,CARGO_TARGET_DIR='/scratch/'+label+'-target')
        results[label]=run(['cargo','+'+NIGHTLY,'miri','test','-vv','--manifest-path',
                           str(folder/'Cargo.toml'),'--offline'],label+'.log',
                          env=mode_env,timeout=90,required=False)
    # Hypothesis: Cargo's new host runner path also matches Miri's cfg(all())
    # target runner. Select an explicit native host runner, keeping interpreted
    # test target execution under cargo-miri.
    manifest = str(SCRATCH/'minimal-build/Cargo.toml')
    for label, flags in [('host-config-only',['-Zhost-config','-Ztarget-applies-to-host']),
                         ('native-host',['-Zhost-config','-Ztarget-applies-to-host','--config','host.runner=["/usr/bin/env"]'])]:
        mode_env = dict(env,CARGO_TARGET_DIR='/scratch/'+label+'-target')
        results[label] = run(['cargo','+'+NIGHTLY,'miri','test','-vv',*flags,
                              '--manifest-path',manifest,'--offline'],
                             label+'.log',env=mode_env,timeout=90,required=False)
    write('diagnosis.json',results)
    if results['native-host'] != 0 or results['host-config-only'] == 0:
        raise RuntimeError('host-runner hypothesis did not match controls')
    fixed_env = dict(env,CARGO_TARGET_DIR='/scratch/core-fixed')
    results['originalCoreFixed'] = run(['cargo','+'+NIGHTLY,'miri','test','-vv',
                      '-Zhost-config','-Ztarget-applies-to-host','--config','host.runner=["/usr/bin/env"]',
                      '-p','pqc_scan_core','--lib','identity::tests','--locked','--offline'],
                     'core-fixed.log',env=fixed_env,timeout=300,required=False)
    write('diagnosis.json',results)
    identities={}
    for target in ['native-host-target','core-fixed']:
        for path in (SCRATCH/target).rglob('build-script-build'):
            data=path.read_bytes();identities[str(path)]={'length':len(data),'magicHex':data[:16].hex()}
    write('fixed-build-files.json',identities)


if __name__ == '__main__':
    try:
        main()
    except BaseException:
        traceback.print_exc()
        raise
