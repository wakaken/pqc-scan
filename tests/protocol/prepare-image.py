#!/usr/bin/env python3
"""Network-capable tool/cache preparation; never build or run repository target code."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import shutil
import subprocess
import tempfile
import urllib.request

ROOT = Path(__file__).resolve().parents[2]
URL = 'https://cdn.openbsd.org/pub/OpenBSD/OpenSSH/portable/openssh-10.2p1.tar.gz'
SHA = 'ccc42c0419937959263fa1dbd16dafc18c56b984c03562d2937ce56a60f798b2'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out-dir', type=Path, required=True)
    parser.add_argument('--tool-image', help='reuse an existing immutable tool image, without rebuilding OpenSSH')
    args = parser.parse_args(); args.out_dir.mkdir(parents=True, exist_ok=False)
    with tempfile.TemporaryDirectory(prefix='pqc-tool-image-') as temp:
        context = Path(temp); manifests = context/'manifests'; manifests.mkdir()
        for name in ['Cargo.toml','Cargo.lock']:
            shutil.copyfile(ROOT/name, manifests/name)
        for manifest in sorted((ROOT/'crates').glob('*/Cargo.toml')):
            folder = manifests/'crates'/manifest.parent.name; (folder/'src').mkdir(parents=True)
            shutil.copyfile(manifest, folder/'Cargo.toml')
            (folder/'src/lib.rs').write_text('')
            (folder/'src/main.rs').write_text('fn main() {}\n')
        if args.tool_image:
            if not re.fullmatch(r'sha256:[0-9a-f]{64}',args.tool_image):
                raise ValueError('immutable local tool image ID required')
            tag = 'pqc-protocol-tools:'+args.tool_image.split(':')[1]
            subprocess.run(['docker','tag',args.tool_image,tag],check=True)
            prefix = 'FROM '+tag+'\n'
        else:
            data = urllib.request.urlopen(URL,timeout=30).read(4*1024**2+1)
            if hashlib.sha256(data).hexdigest()!=SHA:
                raise ValueError('OpenSSH tool archive digest mismatch')
            (context/'openssh-10.2p1.tar.gz').write_bytes(data)
            prefix = (ROOT/'tests/feasibility/ssh-kex/Dockerfile').read_text().split('WORKDIR /work')[0]
        # Only Cargo manifests and empty source stubs enter this preparation layer.
        # cargo fetch resolves/downloads the locked cache; no build.rs or tests run.
        (context/'Dockerfile').write_text(prefix+'\nRUN rustup component add --toolchain 1.93.1 clippy rustfmt\nCOPY manifests /tool-manifests\nRUN cargo fetch --manifest-path /tool-manifests/Cargo.toml --locked\nWORKDIR /source\n')
        with (args.out_dir/'prepare.log').open('w') as log:
            subprocess.run(['docker','build','--pull=false','--iidfile',str(args.out_dir.resolve()/'image-id'),str(context)],
                           check=True,stdout=log,stderr=subprocess.STDOUT,timeout=900)
    image = (args.out_dir/'image-id').read_text().strip()
    (args.out_dir/'IMAGE.json').write_text(json.dumps({'image':image,'preparation':'network-capable trusted tools and Cargo fetch only; no repository source execution',
        'opensshSourceSha256':SHA,'cargoLockSha256':hashlib.sha256((ROOT/'Cargo.lock').read_bytes()).hexdigest()},indent=2)+'\n')
    print('Prepared immutable tool/cache image:',image)


if __name__ == '__main__':
    main()
