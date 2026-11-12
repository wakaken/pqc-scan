//! Parse-only, byte-preserving compatibility for specific grammar limitations.
//! Matching and evidence always read the original source. This is not evaluation.
mod javascript;
mod javascript_lexer;
mod jsx_attributes;
mod typescript_types;
use tree_sitter::Tree;
macro_rules! re {
    ($pattern:literal) => {{
        static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
        RE.get_or_init(|| regex::Regex::new($pattern).expect("static syntax pattern"))
    }};
}

pub(super) fn normalize(
    text: &str,
    tree: &Tree,
    language: &str,
    work_budget: usize,
    module: bool,
) -> (Vec<u8>, bool) {
    let mut remaining = work_budget;
    macro_rules! spend {
        ($n:expr) => {{
            let n = $n;
            if n > remaining {
                return (text.as_bytes().to_vec(), true);
            }
            remaining -= n;
        }};
    }
    spend!(text.len().saturating_mul(32));
    let mut bytes = text.as_bytes().to_vec();
    let mut protected = vec![false; bytes.len()];
    let mut jsx = Vec::new();
    let mut stack = vec![tree.root_node()];
    while let Some(node) = stack.pop() {
        spend!(1usize);
        let kind = node.kind();
        if kind.contains("string")
            || kind == "comment"
            || kind == "char_literal"
            || kind.contains("heredoc")
        {
            spend!(node.end_byte() - node.start_byte());
            protected[node.byte_range()].fill(true);
        }
        if kind.starts_with("jsx_") {
            jsx.push((node.start_byte(), node.end_byte(), kind.to_string()));
        }
        if kind == "ERROR" && node.parent().is_some_and(|p| p.kind() == "jsx_element") {
            jsx.push((node.start_byte(), node.end_byte(), "jsx_text".into()));
        }
        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            stack.push(child);
        }
    }
    if language == "javascript"
        && !javascript::normalize(text, tree, &mut bytes, &mut remaining, module)
    {
        return (text.as_bytes().to_vec(), true);
    }
    if language == "go" {
        // Error recovery can pair unrelated quotes around new(value).
        // Lexically valid string/comment boundaries do not depend on that tree.
        let protected = go_lexical_protection(text.as_bytes());
        let re = re!(r"\bnew\s*\(");
        for m in re.find_iter(text) {
            if !protected[m.start()] {
                bytes[m.start()..m.start() + 3].copy_from_slice(b"n_w");
            }
        }
    }
    if matches!(language, "javascript" | "typescript") {
        if language == "typescript" {
            // V8 intrinsics are prefix calls. A preceding value would instead be %.
            let re = re!(r"%[ \t]*[A-Za-z_$][A-Za-z0-9_$]*\s*\(");
            for m in re.find_iter(text) {
                if protected[m.start()] {
                    continue;
                }
                let before = text[..m.start()].trim_end();
                let previous = before.chars().next_back();
                let prefix = previous.is_none_or(|c| "=([{,:;!?&|+-*/%^~<>".contains(c))
                    || before.ends_with(['\n', '\r'])
                    || ["return", "throw", "yield", "else"]
                        .iter()
                        .any(|w| before.ends_with(w));
                // In V8, a standalone intrinsic after an if/loop condition is valid.
                let line = text[..m.start()].rsplit('\n').next().unwrap_or("").trim();
                if prefix || line.is_empty() || line.starts_with("if (") && line.ends_with(')') {
                    bytes[m.start()] = b' ';
                }
            }
        }
        let re = re!(r#"\bimport\s+source\s+([A-Za-z_$][A-Za-z0-9_$]*)\s+from\s*['"]"#);
        for c in re.captures_iter(text) {
            let m = c.get(0).unwrap();
            if !protected[m.start()] {
                let start = m.start() + text[m.range()].find("source").unwrap();
                bytes[start..start + 6].fill(b' ');
            }
        }
        let re = re!(r"\bimport\.source\s*\(");
        for m in re.find_iter(text) {
            if !protected[m.start()] {
                bytes[m.start() + 6..m.start() + 13].fill(b' ');
            }
        }
        // Preserve quoted text and its positions while avoiding a JSX entity lexer bug.
        for (start, end, kind) in &jsx {
            spend!(end - start);
            if kind == "jsx_text" {
                for byte in &mut bytes[*start..*end] {
                    if *byte == b'&' {
                        *byte = b'x';
                    }
                }
            }
        }

        // Reserved JSX attribute names may prevent the opening element from
        // forming at all. Recognize only an unquoted attribute outside {...}.
        for c in re!(r"\b(class|default|for)\s*=").captures_iter(text) {
            let m = c.get(1).unwrap();
            let p = m.start();
            if protected[p] {
                continue;
            }
            spend!(p);
            if let Some(open) = text[..p].rfind('<') {
                let fragment = &text[open + 1..p];
                if fragment
                    .as_bytes()
                    .first()
                    .is_some_and(u8::is_ascii_alphabetic)
                    && !fragment.contains(['<', '>'])
                {
                    let mut braces = 0isize;
                    for i in open + 1..p {
                        if !protected[i] {
                            if bytes[i] == b'{' {
                                braces += 1;
                            }
                            if bytes[i] == b'}' {
                                braces -= 1;
                            }
                        }
                    }
                    if braces == 0 {
                        bytes[p + 1] = b'_';
                    }
                }
            }
        }
        let re = re!(r"\b(using|as|abstract|type|delete)\b");
        for c in re.captures_iter(text) {
            let m = c.get(1).unwrap();
            if protected[m.start()] {
                continue;
            }
            let after = text[m.end()..].trim_start();
            let before = text[..m.start()].trim_end();
            let identifier_role = match m.as_str() {
                "using" => after.starts_with([',', ')', '}', ':', '=', ';', '.']),
                "as" => after.starts_with(['=', ',', ';', ':', '.']),
                "abstract" => after.starts_with(':') || after.starts_with("?:"),
                "type" => {
                    (before.ends_with("typeof") && after.starts_with([')', ']', '.']))
                        || after.starts_with([',', '}'])
                            && before
                                .rsplit_once("export")
                                .is_some_and(|(_, s)| s.contains('{') && !s.contains('}'))
                }
                "delete" => {
                    before.ends_with("as")
                        && after.starts_with([',', '}'])
                        && before
                            .rsplit_once("export")
                            .is_some_and(|(_, s)| s.contains('{') && !s.contains('}'))
                }
                _ => false,
            };
            if identifier_role {
                bytes[m.start() + 1] = b'_';
            }
        }
        let import_word = re!(r"\bimport\b");
        for block in re!(r"\b(import|export)\s*\{[^{}]*\}").captures_iter(text) {
            let m = block.get(0).unwrap();
            if protected[m.start()] {
                continue;
            }
            let kind = &block[1];
            for word in import_word.find_iter(&text[m.start() + kind.len()..m.end()]) {
                let start = m.start() + kind.len() + word.start();
                let end = m.start() + kind.len() + word.end();
                if protected[start] {
                    continue;
                }
                let after = text[end..m.end()].trim_start();
                let before = text[m.start()..start].trim_end();
                if kind == "import" && after.starts_with("as ")
                    || kind == "export" && before.ends_with("as")
                {
                    bytes[start + 1] = b'_';
                }
            }
        }
        let mut pending = vec![tree.root_node()];
        while let Some(n) = pending.pop() {
            spend!(1usize);
            if language == "typescript" && n.kind() == "type_alias_declaration" {
                if let Some(params) = n.child_by_field_name("type_parameters") {
                    let a = params.start_byte();
                    let b = params.end_byte();
                    spend!(b - a);
                    if b >= a + 2
                        && text.as_bytes()[a] == b'<'
                        && text.as_bytes()[b - 1] == b'>'
                        && trivia_only(&text[a + 1..b - 1])
                    {
                        bytes[a] = b' ';
                        bytes[b - 1] = b' ';
                    }
                }
            }
            if n.kind() == "arrow_function"
                && text.as_bytes().get(n.end_byte().saturating_sub(1)) == Some(&b'}')
            {
                let suffix = &text[n.end_byte()..];
                let gap = suffix.len() - suffix.trim_start().len();
                if suffix[..gap].contains('\n') && suffix[gap..].starts_with('[') {
                    if let Some(p) = (n.end_byte()..n.end_byte() + gap)
                        .rev()
                        .find(|p| matches!(bytes[*p], b' ' | b'\t'))
                    {
                        bytes[p] = b';';
                    }
                }
            }
            let mut c = n.walk();
            for child in n.named_children(&mut c) {
                pending.push(child);
            }
        }
        // Type arguments on tagged templates are parsed as a validated type list.
        if language == "typescript" {
            for c in re!(r"([A-Za-z_$][A-Za-z0-9_.$]*)<([^<>]+)>[ \t]*`").captures_iter(text) {
                let whole = c.get(0).unwrap();
                let args = c.get(2).unwrap();
                if protected[whole.start()] || args.len() > 4096 {
                    continue;
                }
                let mut p = tree_sitter::Parser::new();
                p.set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
                    .unwrap();
                let check = format!("type Compatibility = Type<{}>;", args.as_str());
                if p.parse(&check, None)
                    .is_some_and(|t| !t.root_node().has_error())
                {
                    for b in &mut bytes[args.start() - 1..args.end() + 1] {
                        if !matches!(*b, b'\n' | b'\r') {
                            *b = b' ';
                        }
                    }
                }
            }
        }
        for c in re!(r"(?m)^([ \t]*)#[A-Za-z_$][A-Za-z0-9_$]*([ \t]*\r?\n(?:[ \t]*\r?\n)*[ \t]+)\[")
            .captures_iter(text)
        {
            let m = c.get(0).unwrap();
            let hash = m.start() + c[1].len();
            if protected[hash] {
                continue;
            }
            let mut n = tree.root_node().descendant_for_byte_range(hash, hash + 1);
            let mut in_class = false;
            while let Some(node) = n {
                spend!(1usize);
                if node.kind() == "class_body" {
                    in_class = true;
                    break;
                }
                n = node.parent();
            }
            if in_class {
                let gap = c.get(2).unwrap();
                if let Some(p) = gap
                    .range()
                    .rev()
                    .find(|p| matches!(bytes[*p], b' ' | b'\t'))
                {
                    bytes[p] = b';';
                }
            }
        }
        if !module_attributes(text, &protected, &mut bytes, &mut remaining) {
            return (text.as_bytes().to_vec(), true);
        }
        let resource =
            re!(r"\b(await[ \t]+)?using([ \t]+)([A-Za-z_$][A-Za-z0-9_$]*)[ \t]*(?:=|of\b)");
        for c in resource.captures_iter(text) {
            let m = c.get(0).unwrap();
            if protected[m.start()] {
                continue;
            }
            let start = if let Some(a) = c.get(1) {
                for b in &mut bytes[a.range()] {
                    *b = b' ';
                }
                a.end()
            } else {
                m.start()
            };
            bytes[start..start + 5].copy_from_slice(b"const");
        }
        // Decorator auto-accessors are class fields; erase the modifier only.
        for m in re!(r"\baccessor[ \t]*(?:[A-Za-z_$#]|\[)").find_iter(text) {
            if protected[m.start()] {
                continue;
            }
            let mut n = tree
                .root_node()
                .descendant_for_byte_range(m.start(), m.end());
            let mut class_body = false;
            while let Some(node) = n {
                spend!(1usize);
                if node.kind() == "class_body" {
                    class_body = true;
                    break;
                }
                n = node.parent();
            }
            if class_body {
                bytes[m.start()..m.start() + 8].fill(b' ');
            }
        }
        // Babel/V8 do-expressions are parse-only closures. Require an expression
        // position and spare whitespace for the opening brace; no bytes move.
        for m in re!(r"\bdo[ \t]+\{").find_iter(text) {
            if protected[m.start()] {
                continue;
            }
            let prefix = text[..m.start()].trim_end();
            if !(prefix.ends_with(['=', '(', ':', '?'])
                || prefix.ends_with("return")
                || prefix.ends_with("throw"))
            {
                continue;
            }
            if m.len() != 4 {
                continue;
            }
            let next = text.as_bytes()[m.end()..]
                .iter()
                .position(|b| !b.is_ascii_whitespace())
                .map(|n| m.end() + n)
                .unwrap_or(text.len());
            if let Some(space) = (m.end()..next).find(|i| matches!(bytes[*i], b' ' | b'\t')) {
                bytes[m.start()..m.end()].copy_from_slice(b"()=>");
                bytes[space] = b'{';
            }
        }
        // Anonymous ambient default function signatures need a name in the
        // pinned grammar. Consume existing indentation only, preserving spans.
        if text.contains("declare module") {
            for m in
                re!(r"export[ \t]+default[ \t]+function[ \t]*\([ \t]*\r?\n[ \t]+").find_iter(text)
            {
                if protected[m.start()] {
                    continue;
                }
                let open = m.start() + text[m.range()].find('(').unwrap();
                let space = m.end() - 1;
                bytes[open] = b'f';
                bytes[space] = b'(';
            }
        }
        // NUL is permitted within JS/TS quoted/template text, not in identifiers.
        for (i, b) in text.bytes().enumerate() {
            if b == 0 && protected[i] {
                bytes[i] = b' ';
            }
        }
    }
    if matches!(language, "javascript" | "typescript")
        && !jsx_attributes::normalize(text, &protected, &mut bytes, &mut remaining)
    {
        return (text.as_bytes().to_vec(), true);
    }
    if language == "typescript"
        && !typescript_types::normalize(text, tree, &protected, &mut bytes, &mut remaining)
    {
        return (text.as_bytes().to_vec(), true);
    }
    if language == "rust" {
        // Syn validates the complete Rust syntax before any attribute/type
        // compatibility erasure. It never expands macros or runs source code.
        // A conservative delimiter cap avoids recursive parser exhaustion.
        let mut depth = 0usize;
        let mut safe = true;
        for (i, c) in text.bytes().enumerate() {
            if protected[i] {
                continue;
            }
            if b"([{ ".contains(&c) && c != b' ' {
                depth += 1;
                if depth > 128 {
                    safe = false;
                    break;
                }
            } else if b")] }".contains(&c) && c != b' ' {
                depth = depth.saturating_sub(1);
            }
        }
        if safe {
            let mut pending = vec![(tree.root_node(), 0usize)];
            while let Some((node, level)) = pending.pop() {
                spend!(1usize);
                if level > 128 {
                    safe = false;
                    break;
                }
                let mut c = node.walk();
                for child in node.named_children(&mut c) {
                    pending.push((child, level + 1));
                }
            }
            let mut angle = 0usize;
            for (i, c) in text.bytes().enumerate() {
                if protected[i] {
                    continue;
                }
                match c {
                    b'<' => {
                        angle += 1;
                        if angle > 128 {
                            safe = false;
                            break;
                        }
                    }
                    b'>' => angle = angle.saturating_sub(1),
                    b';' | b'{' | b'}' => angle = 0,
                    _ => {}
                }
            }
        }
        if safe && text.len() <= 2 * 1024 * 1024 && syn::parse_file(text).is_ok() {
            let raw = text.as_bytes();
            let mut i = 0;
            while i + 1 < raw.len() {
                if raw[i] == b'#'
                    && !protected[i]
                    && (raw[i + 1] == b'[' || raw.get(i + 1..i + 3) == Some(b"!["))
                {
                    let begin = i;
                    let open = if raw[i + 1] == b'[' { i + 1 } else { i + 2 };
                    let mut level = 1usize;
                    i = open + 1;
                    while i < raw.len() && level > 0 {
                        if !protected[i] {
                            if raw[i] == b'[' {
                                level += 1;
                            } else if raw[i] == b']' {
                                level -= 1;
                            }
                        }
                        i += 1;
                    }
                    if level == 0 {
                        for b in &mut bytes[begin..i] {
                            if !matches!(*b, b'\n' | b'\r') {
                                *b = b' ';
                            }
                        }
                    }
                    continue;
                }
                i += 1;
            }
            for m in re!(r"\$(?:[ \t]*[\]\)},;])").find_iter(text) {
                if !protected[m.start()] {
                    bytes[m.start()] = b'+';
                }
            }
            for c in re!(r":[ \t]*(mut)[ \t]+[A-Za-z_]").captures_iter(text) {
                let m = c.get(1).unwrap();
                if !protected[m.start()] {
                    bytes[m.range()].fill(b' ');
                }
            }
            let mut stack = vec![tree.root_node()];
            while let Some(n) = stack.pop() {
                spend!(1usize);
                if matches!(n.kind(), "attribute_item" | "inner_attribute_item") {
                    for i in n.byte_range() {
                        if !matches!(bytes[i], b'\n' | b'\r') {
                            bytes[i] = b' ';
                        }
                    }
                }
                if n.kind() == "macro_definition" {
                    for i in n.byte_range() {
                        if !protected[i] && bytes[i] == b'~' {
                            bytes[i] = b'+';
                        }
                    }
                }
                let mut cursor = n.walk();
                for child in n.named_children(&mut cursor) {
                    stack.push(child);
                }
            }
            for c in re!(r"\$([ \t]+)([A-Za-z_][A-Za-z0-9_]*)").captures_iter(text) {
                let m = c.get(0).unwrap();
                if !protected[m.start()] {
                    let name = c.get(2).unwrap();
                    bytes[m.start()] = b' ';
                    bytes[name.start() - 1] = b'$';
                }
            }
            for m in re!(r#"\bunsafe([ \t]+)extern\s*""#).find_iter(text) {
                if !protected[m.start()] {
                    bytes[m.start()..m.start() + 6].fill(b' ');
                }
            }
            for m in
                re!(r"\bfor\s*<\s*'[A-Za-z_][A-Za-z0-9_]*(?:\s*,\s*'[A-Za-z_][A-Za-z0-9_]*)*\s*>")
                    .find_iter(text)
            {
                if !protected[m.start()] {
                    for b in &mut bytes[m.range()] {
                        if !matches!(*b, b'\n' | b'\r') {
                            *b = b' ';
                        }
                    }
                }
            }
            for m in re!(r"::[ \t]*<[^<>\n]*>").find_iter(text) {
                if !protected[m.start()] {
                    bytes[m.range()].fill(b' ');
                }
            }
            for c in
                re!(r"\bstruct[ \t]+[A-Za-z_][A-Za-z0-9_]*(\s+where[^;{}]+);").captures_iter(text)
            {
                let whole = c.get(0).unwrap();
                if !protected[whole.start()] {
                    for b in &mut bytes[c.get(1).unwrap().range()] {
                        if !matches!(*b, b'\n' | b'\r') {
                            *b = b' ';
                        }
                    }
                }
            }
        }
        for m in re!(r"&raw\b").find_iter(text) {
            let next = text[m.end()..].trim_start();
            if !protected[m.start()] && !next.starts_with("const ") && !next.starts_with("mut ") {
                bytes[m.start() + 2] = b'_';
            }
        }
        for m in re!(r"\btry!").find_iter(text) {
            if !protected[m.start()] {
                bytes[m.start() + 2] = b'_';
            }
        }
    }
    if language == "ruby" {
        let quoted_raise = re!(r#"(?m)^[ \t]*def[ \t]+[^\n=]+[ \t]*=[ \t]*raise([ \t]+)['"]"#);
        for c in quoted_raise.captures_iter(text) {
            let m = c.get(0).unwrap();
            if !protected[m.start()] {
                bytes[c.get(1).unwrap().start()] = b'+';
            }
        }
        let re = re!(r"(?m)^\s*def\s+[^\n=]+\s*=\s*raise([ \t]+)([A-Z][A-Za-z0-9_:]*)(,)?");
        for c in re.captures_iter(text) {
            let m = c.get(0).unwrap();
            if protected[m.start()] {
                continue;
            }
            let space = c.get(1).unwrap();
            bytes[space.start()] = b'+';
            if let Some(comma) = c.get(3) {
                bytes[comma.start()] = b'+';
            }
        }
    }
    (bytes, false)
}

fn trivia_only(text: &str) -> bool {
    static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
    RE.get_or_init(|| regex::Regex::new(r"(?s)^(?:\s|/\*.*?\*/|//[^\r\n]*(?:\r?\n|$))*$").unwrap())
        .is_match(text)
}

fn module_attributes(
    text: &str,
    protected: &[bool],
    bytes: &mut [u8],
    remaining: &mut usize,
) -> bool {
    let attributes = re!(
        r#"\b(?:with|assert)\s*\{\s*(?:(?:[A-Za-z_$][A-Za-z0-9_$]*|'[A-Za-z0-9_$-]+'|"[A-Za-z0-9_$-]+")\s*:\s*(?:'[A-Za-z0-9_./:-]+'|"[A-Za-z0-9_./:-]+")\s*,?\s*)+\}"#
    );
    let declaration =
        re!(r#"(?s)\b(?:import|export)\b[^;{}]*\bfrom\s*(?:'[^'\r\n]*'|"[^"\r\n]*")\s*$"#);

    let ambient = re!(r#"\bdeclare\s+module\s*(?:'[^'\r\n]*'|"[^"\r\n]*")\s*$"#);
    let field = re!(
        r#"([A-Za-z_$][A-Za-z0-9_$]*|'[A-Za-z0-9_$-]+'|"[A-Za-z0-9_$-]+")\s*:\s*(?:'[A-Za-z0-9_./:-]+'|"[A-Za-z0-9_./:-]+")"#
    );
    for m in attributes.find_iter(text) {
        let Some(rest) = remaining.checked_sub(m.start() + m.len()) else {
            return false;
        };
        *remaining = rest;
        if protected[m.start()] {
            continue;
        }
        let Some(d) = declaration
            .find(&text[..m.start()])
            .or_else(|| ambient.find(&text[..m.start()]))
        else {
            continue;
        };
        if protected[d.start()] {
            continue;
        }
        let mut seen = std::collections::BTreeSet::new();
        let body = &m.as_str()[m.as_str().find('{').unwrap() + 1..m.as_str().len() - 1];

        let mut cursor = 0;
        let mut valid = true;
        for (index, c) in field.captures_iter(body).enumerate() {
            let f = c.get(0).unwrap();
            if body[cursor..f.start()].trim() != if index == 0 { "" } else { "," }
                || !seen.insert(c[1].trim_matches(['\'', '"']).to_string())
            {
                valid = false;
                break;
            }
            cursor = f.end();
        }
        if !valid || seen.is_empty() || !["", ","].contains(&body[cursor..].trim()) {
            continue;
        }
        for b in &mut bytes[m.range()] {
            if !matches!(*b, b'\n' | b'\r') {
                *b = b' ';
            }
        }
    }
    true
}

// One linear pass; no parsing or evaluation of Go expressions.
fn go_lexical_protection(source: &[u8]) -> Vec<bool> {
    let mut protected = vec![false; source.len()];
    let mut i = 0;
    while i < source.len() {
        let start = i;
        if source[i..].starts_with(b"//") {
            i += 2;
            while i < source.len() && source[i] != b'\n' {
                i += 1;
            }
        } else if source[i..].starts_with(b"/*") {
            i += 2;
            while i < source.len() && !source[i..].starts_with(b"*/") {
                i += 1;
            }
            i = (i + 2).min(source.len());
        } else if matches!(source[i], b'"' | b'\'' | b'`') {
            let quote = source[i];
            i += 1;
            while i < source.len() {
                let b = source[i];
                i += 1;
                if quote != b'`' && b == b'\\' {
                    i = (i + 1).min(source.len());
                } else if b == quote {
                    break;
                }
            }
        } else {
            i += 1;
            continue;
        }
        protected[start..i].fill(true);
    }
    protected
}

#[cfg(test)]
mod tests {
    #[test]
    fn go_lexical_boundaries_protect_literals_and_comments_beside_expression_new() {
        let source = "var raw = `new(1)`; var quoted = \"new(2)\\\"still quoted\"; var rune = '('; // new(3)\n/* new(4) */ var live = new(\"a\")";
        let mask = super::go_lexical_protection(source.as_bytes());
        for literal in [
            "new(1)",
            "new(2)",
            "new(3)",
            "new(4)",
            "still quoted",
            "'('",
        ] {
            let start = source.find(literal).unwrap();
            assert!(
                mask[start..start + literal.len()].iter().all(|v| *v),
                "{literal}"
            );
        }
        let live = source.find("new(\"a\")").unwrap();
        assert!(mask[live..live + 4].iter().all(|v| !*v));
        assert!(mask[live + 4..live + 7].iter().all(|v| *v));
    }
}
