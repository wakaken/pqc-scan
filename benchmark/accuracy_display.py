"""Bounded display-only masking for known private syntax and opaque literals.

This finite policy is not a universal secret detector. Identity fields are kept
outside the text boundary. Callers must mask whole source before taking excerpts.
"""
import html
import json
import re
from bisect import bisect_left, bisect_right
from typing import NamedTuple

DISPLAY_LIMIT = 2 * 1024 * 1024
_ASSIGNMENT = re.compile(r'''(?<![\w.$-])(?P<key>[^\W\d][\w.$-]{0,255})["']?\s*(?:=>|:(?!=)|=(?!=))\s*''', re.UNICODE)
_OPAQUE = re.compile(r'[A-Za-z0-9_+/-]{20,}={0,2}')
_ESCAPES = re.compile(r'\\(?:u([0-9a-fA-F]{4})|([\\"\'/nr]))')
_HTML_ESCAPE = re.compile(r'&(?:quot|apos|amp|lt|gt|#[0-9]{1,7}|#x[0-9a-fA-F]{1,6});')
_SHORT_PRIVATE = {'d', 'p', 'q', 'dp', 'dq', 'qi', 'k'}
_PRIVATE = {'password', 'passwd', 'secret', 'token', 'apikey', 'apitoken',
            'accesstoken', 'authtoken', 'clientsecret', 'privatekey', 'private',
            'keymaterial', 'authorization', 'secretkey', 'secretaccesskey',
            'credential', 'shared', 'sharedsecret', 'seed', 'scalar', 'privatescalar',
            'privateexponent', 'privatekeyjwk', 'privatekeypkcs8', 'privatekeybytes', 'privatekeypem', 'privatescalarbytes', 'seedbytes'}
_HASH_FIELDS = {'sha256', 'sha1', 'sha512', 'md5', 'source_sha256', 'sourceSha256', 'findingSha256'}


def private_name(name):
    normalized = re.sub(r'([a-z0-9])([A-Z])',r'\1_\2',name).lower()
    pieces = re.split(r'[_.$-]+',normalized)
    if pieces[-1] in _SHORT_PRIVATE:
        return True
    return any(''.join(pieces[-count:]) in _PRIVATE for count in range(1,min(len(pieces),5)+1))


def _quote_end(text, start, limit=None):
    limit = len(text) if limit is None else limit
    quote = text[start]
    delimiter = quote * 3 if text.startswith(quote * 3, start) else quote
    index = start + len(delimiter)
    while index < limit:
        if text[index] == '\\':
            index += 2
        elif text.startswith(delimiter, index):
            return min(index + len(delimiter), limit)
        else:
            index += 1
    return limit


class _Literal(NamedTuple):
    start: int
    end: int
    body_start: int
    body_end: int


def _literal(text, start, limit):
    raw = None
    if start == 0 or not (text[start-1].isalnum() or text[start-1] == '_'):
        raw = re.match(r'(?:br|cr|r)(#{0,255})"', text[start: min(limit, start+259)])
        cpp = re.match(r'R"([^\s()\\]{0,16})\(', text[start: min(limit, start+20)])
        if raw and text.startswith('"""', start+raw.end()-1):
            raw = None
        if cpp:
            delimiter = ')' + cpp[1] + '"'
            body_start = start + cpp.end()
        elif raw:
            delimiter = '"' + raw[1]
            body_start = start + raw.end()
        if cpp or raw:
            closing = text.find(delimiter, body_start, limit)
            return _Literal(start, limit if closing < 0 else closing+len(delimiter),
                            body_start, limit if closing < 0 else closing)
    if text[start] not in '\"\'`':
        return None
    width = 3 if text.startswith(text[start]*3, start) else 1
    end = _quote_end(text, start, limit)
    closed = end >= start+width*2 and text[end-width:end] == text[start:start+width]
    return _Literal(start, end, start+width, end-width if closed else end)


def _lexemes(text, start=0, limit=None, comments=True):
    limit = len(text) if limit is None else limit
    index = start
    while index < limit:
        begin = index
        literal = _literal(text, index, limit)
        if comments and text.startswith('/*', index):
            depth = 1
            index += 2
            while index < limit and depth:
                if text.startswith('/*', index):
                    depth += 1
                    index += 2
                elif text.startswith('*/', index):
                    depth -= 1
                    index += 2
                else:
                    index += 1
            kind = 'comment'
        elif comments and (text.startswith('//', index) or text[index] == '#'):
            while index < limit and text[index] not in '\r\n':
                index += 1
            kind = 'comment'
        elif literal is not None:
            index = literal.end
            kind = 'quote'
        elif text[index] == '█':
            while index < limit:
                if text[index] in '█\r\n':
                    index += 1
                elif text[index] == '\\' and index + 1 < limit and text[index+1] in 'nr':
                    index += 2
                else:
                    break
            kind = 'mask'
        else:
            index += 1
            kind = 'char'
        yield kind, begin, index


class _TextSyntax:
    def __init__(self, text):
        self.text = text
        regions = list(_lexemes(text))
        self.comments = [(start, end) for kind, start, end in regions if kind == 'comment']
        self.quotes = []
        for kind, start, end in regions:
            if kind != 'quote':
                continue
            literal = _literal(text, start, end)
            self.quotes.append((start, literal.body_end))
        self.quote_starts = [start for start, _ in self.quotes]
        self.starts = [start for start, _ in self.comments]
        self.limits = [end - 2 if text.startswith('/*', start) and text[end-2:end] == '*/' else end
                       for start, end in self.comments]
        for index in range(len(self.comments) - 2, -1, -1):
            gap = text[self.comments[index][1]:self.comments[index+1][0]]
            if not gap or gap.isspace():
                self.limits[index] = self.limits[index+1]

    def tokens(self, start, origin):
        index = bisect_right(self.starts, origin) - 1
        limit = len(self.text)
        if index >= 0 and self.comments[index][0] <= origin < self.comments[index][1]:
            limit = self.limits[index]
        else:
            index = bisect_right(self.starts, start) - 1
            if index >= 0 and self.comments[index][0] < start < self.comments[index][1]:
                limit = self.limits[index]
        index = bisect_right(self.quote_starts, start) - 1
        if index >= 0 and self.quotes[index][0] < origin <= start <= self.quotes[index][1]:
            limit = min(limit, self.quotes[index][1])
        return _lexemes(self.text, start, limit), limit



def _rsa_oth_spans(text):
    scopes = [(text, [(i, i+1) for i in range(len(text))], 0)]
    spans = []
    while scopes:
        document, positions, depth = scopes.pop()
        stack, objects = [], []
        previous = None
        for kind, begin, end in _lexemes(document):
            if kind in {'comment', 'quote'}:
                if kind == 'quote':
                    literal = _literal(document, begin, end)
                    inside, stop = literal.body_start, literal.body_end
                else:
                    inside = begin + (2 if document.startswith(('/*', '//'), begin) else 1)
                    stop = end - (2 if document.startswith('/*', begin) and document[end-2:end] == '*/' else 0)
                if depth < 3 and inside < stop and '{' in document[inside:stop]:
                    content, mapped = _unescape(document[inside:stop], positions[inside:stop])
                    content, mapped = _unescape_html(content, mapped)
                    scopes.append((content, mapped, depth+1))
                if kind == 'comment':
                    continue
            if kind == 'char' and document[begin].isspace():
                continue
            key_start = _literal(document, begin, end).body_start if kind == 'quote' else begin
            if stack and stack[-1][0] == '{' and previous in {'{', ','}:
                match = _ASSIGNMENT.match(document, key_start)
                if match and (kind != 'quote' or match.end() >= end):
                    stack[-1][1].append(match)
            if kind == 'char':
                char = document[begin]
                if char in '{[(':
                    fields = []
                    stack.append((char, fields))
                    if char == '{':
                        objects.append(fields)
                elif char in '}])':
                    if stack and stack[-1][0] == {'}':'{', ']':'[', ')':'('}[char]:
                        stack.pop()
                    else:
                        stack.clear()
                previous = char
            else:
                previous = kind
        syntax = None
        for fields in objects:
            rsa = False
            for match in fields:
                if match['key'] != 'kty':
                    continue
                value_start = match.end()
                if value_start < len(document) and document[value_start] in '\"\'`':
                    value_end = _quote_end(document, value_start)
                    rsa = rsa or document[value_start:value_end] in {'"RSA"', "'RSA'", '`RSA`'}
            if not rsa:
                continue
            for match in fields:
                if match['key'] != 'oth':
                    continue
                syntax = syntax or _TextSyntax(document)
                end = _value_end(document, match.end(), syntax, match.start())
                if end > match.end():
                    spans.append((positions[match.end()][0], positions[end-1][1]))
    return spans


def _value_end(text, start, syntax, origin):
    tokens, limit = syntax.tokens(start, origin)
    first = None
    for token in tokens:
        kind, begin, end = token
        if kind != 'comment' and not (kind == 'char' and text[begin].isspace()):
            first = token
            break
    if first is None:
        return limit
    kind, begin, end = first
    if kind == 'mask':
        return end
    if kind == 'quote':
        last_quote_end = end
        for kind, begin, end in tokens:
            if kind == 'quote':
                last_quote_end = end
            elif kind == 'comment' or kind == 'char' and (text[begin].isspace() or text[begin] in '+\\'):
                continue
            else:
                break
        return last_quote_end
    if text[begin] in '[{(':
        stack = [text[begin]]
        pairs = {']':'[', '}':'{', ')':'('}
        for kind, begin, end in tokens:
            if kind != 'char':
                continue
            char = text[begin]
            if char in '[{(':
                stack.append(char)
            elif char in ']})':
                if not stack or stack.pop() != pairs[char]:
                    return limit
                if not stack:
                    return end
        return limit
    if kind == 'char' and text[begin] in '\r\n,;}':
        return begin
    for kind, begin, end in tokens:
        if kind == 'char' and text[begin] in '\r\n,;}':
            return begin
    return limit


def _public_literals(text):
    def unique(pairs):
        result = {}
        for key,value in pairs:
            if key in result:
                raise ValueError('duplicate_json_key')
            result[key] = value
        return result
    document = text
    try:
        for _ in range(4):
            projected = ''.join('null' if kind == 'mask' else document[start:end]
                                for kind, start, end in _lexemes(document)) if '█' in document else document
            value = json.loads(projected, object_pairs_hook=unique)
            if not isinstance(value, str):
                break
            document = value
        else:
            return set()
        pending = [value]
    except (ValueError,RecursionError):
        return set()
    public = set()
    while pending:
        value = pending.pop()
        if isinstance(value,dict):
            if isinstance(value.get('kty'),str) and value['kty'] in {'EC','RSA','OKP'}:
                public.update(value[key] for key in ('x','y','n','e') if isinstance(value.get(key),str))
            pending.extend(value.values())
        elif isinstance(value,list):
            pending.extend(value)
    return public


def _spans(text, opaque, public_values):
    spans = _rsa_oth_spans(text)
    public_ranges = []
    pem_ranges = []
    ends = {}
    for marker in re.finditer(r'-----END ([A-Z0-9 ]{1,80})-----',text):
        ends.setdefault(marker[1],[]).append(marker.start())
    def find_end(label,start):
        positions = ends.get(label,[])
        index = bisect_left(positions,start)
        return positions[index] if index < len(positions) else -1
    index = 0
    while True:
        start = text.find('-----BEGIN ', index)
        if start < 0:
            break
        header_end = text.find('-----', start + 11)
        if header_end < 0:
            break
        label = text[start + 11:header_end]
        index = header_end + 5
        if label.endswith('PRIVATE KEY'):
            ending = '-----END ' + label + '-----'
            end = find_end(label,index)
            body_end = len(text) if end < 0 else end
            spans.append((index, body_end))
            pem_ranges.append((index, body_end))
            index = body_end if end < 0 else end + len(ending)
        elif label in {'CERTIFICATE','X509 CERTIFICATE','PUBLIC KEY','RSA PUBLIC KEY','EC PUBLIC KEY'}:
            ending = '-----END ' + label + '-----'
            end = find_end(label,index)
            if end >= 0:
                public_ranges.append((index,end))
                pem_ranges.append((index,end))
    opaque_ranges = [match.span() for match in _OPAQUE.finditer(text) if match[0] in public_values]
    data_ranges = sorted(pem_ranges + opaque_ranges)
    merged_data = []
    for start, end in data_ranges:
        if merged_data and start <= merged_data[-1][1]:
            merged_data[-1] = (merged_data[-1][0], max(end, merged_data[-1][1]))
        else:
            merged_data.append((start, end))
    data_cursor = 0
    syntax = _TextSyntax(text)
    covered_end = 0
    for match in _ASSIGNMENT.finditer(text):
        while data_cursor < len(merged_data) and merged_data[data_cursor][1] <= match.start():
            data_cursor += 1
        if data_cursor < len(merged_data) and merged_data[data_cursor][0] <= match.start() < merged_data[data_cursor][1]:
            continue
        if match.start() < covered_end or not private_name(match['key']):
            continue
        end = _value_end(text, match.end(), syntax, match.start())
        if end > match.end():
            spans.append((match.end(), end))
            covered_end = end
    public_cursor = 0
    for match in (_OPAQUE.finditer(text) if opaque else ()):
        while public_cursor < len(public_ranges) and public_ranges[public_cursor][1] <= match.start():
            public_cursor += 1
        if public_cursor < len(public_ranges) and public_ranges[public_cursor][0] <= match.start() < public_ranges[public_cursor][1]:
            continue
        value = match[0]
        if value in public_values:
            continue
        literal = ((match.start() == 0 or text[match.start()-1] in '\"\'`')
                   and (match.end() == len(text) or text[match.end()] in '\"\'`'))
        if (literal and len(value) >= 32 or len(value) >= 32 and all(c in '0123456789abcdefABCDEF' for c in value)
                or value.isdecimal() and len(value) >= 20
                or len(value) >= 48 and any(c.isdigit() for c in value)
                and any(c.isalpha() for c in value)):
            spans.append(match.span())
    return spans


def _unescape(text, positions):
    chars, mapped = [], []
    cursor = 0
    for match in _ESCAPES.finditer(text):
        chars.extend(text[cursor:match.start()])
        mapped.extend(positions[cursor:match.start()])
        escaped = match[2]
        char = chr(int(match[1], 16)) if match[1] else {'n':'\n','r':'\r'}.get(escaped, escaped)
        chars.append(char)
        mapped.append((positions[match.start()][0], positions[match.end()-1][1]))
        cursor = match.end()
    chars.extend(text[cursor:])
    mapped.extend(positions[cursor:])
    return ''.join(chars), mapped


def _unescape_html(text, positions):
    chars, mapped = [], []
    cursor = 0
    for match in _HTML_ESCAPE.finditer(text):
        chars.extend(text[cursor:match.start()])
        mapped.extend(positions[cursor:match.start()])
        decoded = html.unescape(match[0])
        chars.extend(decoded)
        mapped.extend([(positions[match.start()][0], positions[match.end()-1][1])] * len(decoded))
        cursor = match.end()
    chars.extend(text[cursor:])
    mapped.extend(positions[cursor:])
    return ''.join(chars), mapped


class _DumpScope(NamedTuple):
    document: str
    positions: list
    owner: str
    depth: int


def _private_dump_spans(text):
    scopes = [_DumpScope(text, [(i, i+1) for i in range(len(text))], 'source', 0)]
    spans = []
    while scopes:
        document, positions, owner, depth = scopes.pop()
        if depth:
            document, positions = _unescape(document, positions)
            document, positions = _unescape_html(document, positions)
        header = re.search(r'(?i)(?<![\w-])Private-Key[ \t]*:', document) if owner != 'source' else None
        if header and re.search(r'(?i)[\r\n][ \t]*(?:(?://|#|\*)[ \t]*)?(?:priv|privateExponent|prime1|prime2|exponent1|exponent2|coefficient)[ \t]*:', document[header.end():]):
            if positions:
                spans.append((positions[0][0], positions[-1][1]))
            continue
        tokens = list(_lexemes(document, comments=owner == 'source'))
        visible = list(document)
        cursor = 0
        while cursor < len(tokens):
            kind, begin, end = tokens[cursor]
            cursor += 1
            if kind not in {'quote', 'comment'}:
                continue
            if kind == 'quote':
                literal = _literal(document, begin, end)
                indexes = list(range(literal.body_start, literal.body_end))
            elif document.startswith('/*', begin):
                stop = end-2 if document[end-2:end] == '*/' else end
                indexes = list(range(begin+2, stop))
                body = document[begin+2:stop]
                removed = set()
                for match in re.finditer(r'(?:^|(?<=[\r\n]))[ \t]*\*(?!/)[ \t]?', body):
                    removed.update(range(begin+2+match.start(), begin+2+match.end()))
                indexes = [index for index in indexes if index not in removed]
            else:
                marker = '//' if document.startswith('//', begin) else '#'
                indexes = list(range(begin+len(marker), end))
                while cursor < len(tokens):
                    next_cursor = cursor
                    while next_cursor < len(tokens) and tokens[next_cursor][0] == 'char' and document[tokens[next_cursor][1]].isspace():
                        next_cursor += 1
                    if next_cursor == len(tokens):
                        break
                    next_kind, next_begin, next_end = tokens[next_cursor]
                    gap = document[end:next_begin]
                    if (next_kind != 'comment' or not document.startswith(marker, next_begin)
                            or not re.fullmatch(r'[ \t]*(?:\r\n|\r|\n)[ \t]*', gap)):
                        break
                    indexes.extend(range(end, next_begin))
                    indexes.extend(range(next_begin+len(marker), next_end))
                    end = next_end
                    cursor = next_cursor+1
            for index in range(begin, end):
                if visible[index] not in '\r\n':
                    visible[index] = ' '
            if indexes:
                if depth < 4:
                    scopes.append(_DumpScope(''.join(document[index] for index in indexes),
                                   [positions[index] for index in indexes], kind, depth+1))
                else:
                    spans.append((positions[indexes[0]][0], positions[indexes[-1]][1]))
        plain = ''.join(visible)
        for marker in re.finditer(r'(?<![\w-])Private-Key[ \t]*:[^\r\n]*[\r\n]+[ \t]*priv[ \t]*:', plain):
            if marker.end() < len(positions):
                spans.append((positions[marker.end()][0], positions[-1][1]))
    return spans


def _mask_ranges(text, ranges):
    chars = list(text)
    newlines = {index for match in re.finditer(r'\\+[nr]|[\r\n]', text)
                for index in range(match.start(), match.end())}
    for start, end in ranges:
        for index in range(start, end):
            if index not in newlines:
                chars[index] = '█'
    return ''.join(chars)


def mask_text(text, opaque=True):
    """Mask known content while retaining Unicode columns and newline spelling."""
    if len(text) > DISPLAY_LIMIT or len(text.encode('utf-8')) > DISPLAY_LIMIT:
        raise ValueError('display_resource_limit')
    ranges = _private_dump_spans(text)
    projected = text
    positions = [(i, i+1) for i in range(len(text))]
    public_values = set()
    for _ in range(3):
        if opaque:
            public_values.update(_public_literals(projected))
        for start, end in _spans(projected, opaque, public_values):
            if end > start:
                ranges.append((positions[start][0], positions[end-1][1]))
        decoded, mapping = _unescape(projected, positions)
        decoded, mapping = _unescape_html(decoded, mapping)
        if decoded == projected:
            break
        projected, positions = decoded, mapping
    else:
        if _ESCAPES.search(projected) or _HTML_ESCAPE.search(projected):
            ranges.append((0,len(text)))
    ranges.sort()
    merged = []
    for start,end in ranges:
        if merged and start <= merged[-1][1]:
            merged[-1] = (merged[-1][0], max(end, merged[-1][1]))
        else:
            merged.append((start,end))
    safe = _mask_ranges(text, merged)
    return safe, safe != text


def _withhold(value):
    if isinstance(value, str):
        return _mask_ranges(value, [(0,len(value))])
    if isinstance(value, list):
        return [_withhold(item) for item in value]
    if isinstance(value, dict):
        return {key:_withhold(item) for key,item in value.items()}
    if value is None or isinstance(value,bool):
        return value
    return '█' * len(str(value))


def safe_content(value):
    if isinstance(value,str):
        return mask_text(value, opaque=False)[0]
    if isinstance(value,list):
        return [safe_content(item) for item in value]
    if isinstance(value,dict):
        return {key: (_withhold(item) if private_name(key) or key == 'oth' and value.get('kty') == 'RSA' else item if key in _HASH_FIELDS and isinstance(item,str) and re.fullmatch(r'[0-9a-fA-F]{32,128}',item) else safe_content(item))
                for key,item in value.items()}
    return value


def safe_native(finding):
    """Keep native identity/location stable, scrub all display payloads together."""
    identity = {'finding_id','rule_id','category','risk','priority','severity','confidence','location'}
    safe = {key: value if key in identity else safe_content(value) for key,value in finding.items()}
    snippet = finding.get('source_snippet',{}).get('lines',[])
    if snippet:
        joined = '\n'.join(row['text'] for row in snippet)
        masked,_ = mask_text(joined, opaque=False)
        for row,text in zip(safe['source_snippet']['lines'],masked.split('\n')):
            row['text'] = text
    return safe


def main(argv=None):
    import argparse
    import accuracy_sampling
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest='command', required=True)
    view = commands.add_parser('view')
    view.add_argument('--packet', required=True)
    view.add_argument('--sha256', required=True)
    view.add_argument('--output', required=True)
    excerpt = commands.add_parser('excerpt')
    excerpt.add_argument('--path', required=True)
    excerpt.add_argument('--sha256', required=True)
    excerpt.add_argument('--start', type=int, default=1)
    excerpt.add_argument('--end', type=int)
    args = parser.parse_args(argv)
    try:
        if args.command == 'view':
            result = accuracy_sampling.display_view(args.packet,args.sha256,args.output)
        else:
            result = accuracy_sampling.display_excerpt(args.path,args.sha256,args.start,args.end)
    except (ValueError, OSError, UnicodeError):
        print(json.dumps({'status':'withheld','reason':'display_boundary_rejected_input'}))
        return 2
    print(json.dumps(result,ensure_ascii=False))
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
