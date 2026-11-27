import gzip
import hashlib
import os
from pathlib import Path
import zlib

import boundary as b


def compressed_read(item, *, collect=True, limit=b.MAX):
    expected = dict(path=item['compressedPath'], bytes=item['compressedBytes'], sha256=item['compressedSha256'])
    b.read(expected, limit)
    b.require(type(item['bytes']) is int and 0 <= item['bytes'] <= limit, 'E_GZIP_SIZE')
    fd = b.approved._open(expected['path'])
    decoder = zlib.decompressobj(31)
    compressed_hash, plain_hash = hashlib.sha256(), hashlib.sha256()
    total, compressed_total, chunks = 0, 0, []
    try:
        while True:
            chunk = os.read(fd, 1024 * 1024)
            if not chunk:
                break
            compressed_total += len(chunk)
            b.require(compressed_total <= limit and not decoder.eof, 'E_GZIP_TRAILING')
            compressed_hash.update(chunk)
            try:
                plain = decoder.decompress(chunk, item['bytes'] - total + 1)
            except zlib.error:
                raise b.Rejected('E_GZIP_CORRUPT') from None
            total += len(plain)
            b.require(total <= item['bytes'] and not decoder.unconsumed_tail and not decoder.unused_data, 'E_GZIP_SIZE_OR_MEMBERS')
            plain_hash.update(plain)
            if collect:
                chunks.append(plain)
        b.require(decoder.eof and total == item['bytes'] and plain_hash.hexdigest() == item['sha256'] and
                  compressed_total == expected['bytes'] and compressed_hash.hexdigest() == expected['sha256'], 'E_GZIP_BINDING')
    finally:
        os.close(fd)
    return b''.join(chunks) if collect else None


def compress(source, destination, expected, *, limit=b.MAX):
    b.approved.checked_file(source, expected, limit=limit)
    fd = b.approved._open(source)
    try:
        with b.create(destination) as output:
            with gzip.GzipFile(filename='', mode='wb', fileobj=output, mtime=0) as zipped:
                total = 0
                while True:
                    chunk = os.read(fd, 1024 * 1024)
                    if not chunk:
                        break
                    total += len(chunk)
                    b.require(total <= limit, 'E_COMPRESS_SIZE')
                    zipped.write(chunk)
            output.flush()
            os.fsync(output.fileno())
    finally:
        os.close(fd)
    compressed = b.ref(destination, limit)
    item = dict(expected, compressedPath=compressed['path'], compressedBytes=compressed['bytes'],
                compressedSha256=compressed['sha256'])
    compressed_read(item, collect=False, limit=limit)
    b.approved.checked_file(source, expected, limit=limit)
    return item


def archive_outputs(output, compressed, expected):
    output, compressed = b.private_directory(output), b.private_directory(compressed)
    b.require(set(expected) == set(b.FORMATS), 'E_FORMAT_MATRIX')
    b.require(set(os.listdir(output)) == set(expected), 'E_UNEXPECTED_OUTPUT')
    b.require(sum(item['bytes'] for item in expected.values()) <= b.TOTAL, 'E_OUTPUT_TOTAL')
    archived = {name: compress(output / name, compressed / (name + '.gz'), expected[name]) for name in b.FORMATS}
    for name, item in archived.items():
        compressed_read(item, collect=False)
        b.approved.checked_file(output / name, expected[name], limit=b.MAX)
    fd = b.approved._open(output, directory=True)
    try:
        b.require(set(os.listdir(fd)) == set(expected), 'E_OUTPUT_CHANGED')
        for name in b.FORMATS:
            os.unlink(name, dir_fd=fd)
    finally:
        os.close(fd)
    os.rmdir(output)
    return archived


def validate_matrix(items):
    b.require(isinstance(items, dict) and set(items) == set(b.FORMATS), 'E_FORMAT_MATRIX')
    b.require(sum(item['bytes'] for item in items.values()) <= b.TOTAL, 'E_OUTPUT_TOTAL')
    parents = set()
    for name, item in items.items():
        path = Path(item['compressedPath'])
        b.require(path.name == name + '.gz', 'E_COMPRESSED_NAME')
        parents.add(path.parent)
        compressed_read(item, collect=False)
    b.require(len(parents) == 1, 'E_COMPRESSED_PARENT')
    parent = b.private_directory(parents.pop())
    b.require(set(os.listdir(parent)) == {name + '.gz' for name in b.FORMATS}, 'E_COMPRESSED_MATRIX')


def read_document(items, name):
    b.require(name in {'report.json', 'scan.inventory.v1.json'}, 'E_ARTIFACT_READER_SCOPE')
    validate_matrix(items)
    return b.approved.decode(compressed_read(items[name]))
