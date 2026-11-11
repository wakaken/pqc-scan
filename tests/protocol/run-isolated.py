#!/usr/bin/env python3
"""Build/test and optionally render synthetic previews in the strict container."""
import argparse
from pathlib import Path
import container_runner


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out-dir', type=Path, default=Path('target/protocol-matrix'))
    parser.add_argument('--image-file', type=Path, default=Path('target/protocol-image/IMAGE.json'))
    parser.add_argument('--previews', action='store_true')
    parser.add_argument('--ssh-kex', action='store_true')
    args = parser.parse_args(argv)
    container_runner.execute(container_runner.image_id(args.image_file), args.out_dir,
                             previews=args.previews, ssh_kex=args.ssh_kex)
    print('Confined execution passed; container removed and bounded evidence preserved.')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
